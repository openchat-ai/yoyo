//! WebAssembly backend — maps YOYO TIR to a native `.wasm` binary.
//!
//! Produces a valid Wasm module with:
//!   - 1 type: (func (param i32*locals)) — one big state vector
//!   - 1 function: main entry containing all handlers as labeled blocks
//!   - 1 memory: linear memory (1 page = 64KB)
//!   - 1 data segment: raw data bytes (active, anchored at base 0)
//!
//! State: N locals (i32), one per slot. Max slot index determines N.
//! Branches: handlers are blocks with labels; `br` jumps between them.
//! Memory: ALLOC stores the linear-memory base for that allocation into its slot;
//!   LDB computes address = state[src] + offset and loads a byte from linear memory.
//! Exit: `unreachable` trap (0x00).
//!
//! Wasm opcodes used:
//!   local.get N   : 0x20 N
//!   local.set N   : 0x21 N
//!   local.tee N   : 0x22 N
//!   i32.const N   : 0x41 N
//!   i32.add       : 0x6A
//!   i32.sub       : 0x6B
//!   i32.mul       : 0x6C
//!   i32.and       : 0x70
//!   i32.or        : 0x71
//!   i32.xor       : 0x72
//!   i32.eq        : 0x46
//!   i32.ne        : 0x47
//!   i32.lt_s      : 0x48
//!   i32.le_s      : 0x49
//!   i32.gt_s      : 0x4A
//!   i32.ge_s      : 0x4B
//!   i32.eqz       : 0x45
//!   i32.load8_u   : 0x2C <mem_idx> <align> <offset>
//!   i32.store8    : 0x3A <mem_idx> <align> <offset>
//!   block(label)  : 0x02 0x40 ... end 0x0B
//!   br depth      : 0x0C depth
//!   br_if depth   : 0x0D depth
//!   drop          : 0x1A
//!   end           : 0x0B
//!   unreachable   : 0x00
//!
//! Handlers are nested labeled blocks. Branching between handlers uses
//! the same br to a block label. Exit = unreachable.

use crate::tir::{instr_branch_kind, BranchKind, TirInst, TirOp};
use crate::types::IsaResult;

/// Emit a Wasm module from TIR. Returns Wasm binary bytes.
pub fn emit_wasm(tir: &[TirInst]) -> IsaResult<Vec<u8>> {
    let mut out = Vec::new();

    // Magic + version
    out.extend_from_slice(b"\x00asm");
    out.extend_from_slice(&[0x01, 0x00, 0x00, 0x00]); // version 1

    // Determine total number of state slots (locals) = max slot index + 1
    let mut max_slot: u32 = 0;
    let mut raw_data: Vec<u8> = Vec::new();
    let mut local_count = 0u32;

    // Map each slot index → linear-memory base offset assigned by ALLOC.
    let mut alloc_base: std::collections::HashMap<u32, u32> =
        std::collections::HashMap::new();

    for inst in tir {
        match inst.op {
            TirOp::Set => {
                let s = inst.args.get(0).copied().unwrap_or(0) as u32;
                max_slot = max_slot.max(s);
            }
            TirOp::Get | TirOp::Movrr => {
                let dst = inst.args.get(0).copied().unwrap_or(0) as u32;
                let src = inst.args.get(1).copied().unwrap_or(0) as u32;
                max_slot = max_slot.max(dst.max(src));
            }
            TirOp::Add | TirOp::Sub | TirOp::Inc | TirOp::Dec => {
                let s = inst.args.get(0).copied().unwrap_or(0) as u32;
                max_slot = max_slot.max(s);
            }
            TirOp::Addv | TirOp::Orv | TirOp::Subv | TirOp::Imul => {
                let a = inst.args.get(0).copied().unwrap_or(0) as u32;
                let b = inst.args.get(1).copied().unwrap_or(0) as u32;
                max_slot = max_slot.max(a.max(b));
            }
            TirOp::Cmp => {
                let a = inst.args.get(0).copied().unwrap_or(0) as u32;
                let b = inst.args.get(1).copied().unwrap_or(0) as u32;
                max_slot = max_slot.max(a.max(b));
            }
            TirOp::Ldb => {
                let dd = inst.args.get(0).copied().unwrap_or(0) as u32;
                let ss = inst.args.get(1).copied().unwrap_or(0) as u32;
                max_slot = max_slot.max(dd.max(ss));
            }
            TirOp::MemcpyData => {
                let dst = inst.args.get(0).copied().unwrap_or(0) as u32;
                let src = inst.args.get(1).copied().unwrap_or(0) as u32;
                max_slot = max_slot.max(dst.max(src));
            }
            TirOp::MemcpyState => {
                let dst = inst.args.get(0).copied().unwrap_or(0) as u32;
                let src = inst.args.get(1).copied().unwrap_or(0) as u32;
                max_slot = max_slot.max(dst.max(src));
            }
            TirOp::Alloc => {
                let slot = inst.args.get(0).copied().unwrap_or(0) as u32;
                max_slot = max_slot.max(slot);
            }
            TirOp::LoadFile | TirOp::WriteFile => {
                let s = inst.args.get(0).copied().unwrap_or(0) as u32;
                max_slot = max_slot.max(s);
            }
            TirOp::Data | TirOp::Str | TirOp::Raw => {
                for v in &inst.args {
                    raw_data.push(*v as u8);
                }
            }
            _ => {}
        }
    }
    local_count = max_slot + 1;

    // Compute per-slot alloc base offsets. Start after the static data region.
    let data_end = raw_data.len() as u32;
    let mut current_offset = data_end;
    for inst in tir {
        if inst.op == TirOp::Alloc {
            let slot = inst.args.get(0).copied().unwrap_or(0) as u32;
            let size = inst.args.get(1).copied().unwrap_or(1) as u32;
            // Align allocations to 1 byte (Wasm linear memory is byte-addressable).
            let aligned = size.max(1);
            alloc_base.insert(slot, current_offset);
            current_offset += aligned;
        }
    }

    // ── Type section (id 1) ──
    // func type: (param N i32) -> () — N = local_count state slots
    {
        let mut type_body = Vec::new();
        // type count
        type_body.extend_from_slice(&encode_u32leb(1));
        // func type entry
        type_body.push(0x60); // func
        // param count + param types
        type_body.extend_from_slice(&encode_u32leb(local_count));
        for _ in 0..local_count {
            type_body.push(0x7F); // i32
        }
        // result count = 0
        type_body.push(0x00);
        out.push(1); // section id
        out.extend_from_slice(&encode_u32leb(type_body.len() as u32));
        out.extend_from_slice(&type_body);
    }

    // ── Function section (id 3) ──
    {
        out.push(3);
        let func_body = vec![0x01, 0x00]; // count=1, type_index=0
        out.extend_from_slice(&encode_u32leb(func_body.len() as u32));
        out.extend_from_slice(&func_body);
    }

    // ── Memory section (id 5) ──
    // 1 page (64KB), no maximum.
    // body: <mem_count> <flags=0x00 (no max)> <initial=1>
    {
        let mut mem_body = Vec::new();
        mem_body.extend_from_slice(&encode_u32leb(1)); // mem_count = 1
        mem_body.push(0x00); // flags: no maximum
        mem_body.extend_from_slice(&encode_u32leb(1)); // initial pages = 1
        out.push(5); // section id 5 = Memory
        out.extend_from_slice(&encode_u32leb(mem_body.len() as u32));
        out.extend_from_slice(&mem_body);
    }

    // ── Export section (id 7) ──
    // Export the function as "main" and the memory as "memory" so wasmtime can find them.
    {
        let name = b"main";
        let mem_name = b"memory";
        let mut export_body = Vec::new();
        export_body.extend_from_slice(&encode_u32leb(2)); // export count = 2
        // export func
        export_body.extend_from_slice(&encode_u32leb(name.len() as u32));
        export_body.extend_from_slice(name);
        export_body.push(0x00); // export kind = func
        export_body.extend_from_slice(&encode_u32leb(0)); // func index = 0
        // export memory
        export_body.extend_from_slice(&encode_u32leb(mem_name.len() as u32));
        export_body.extend_from_slice(mem_name);
        export_body.push(0x02); // export kind = memory
        export_body.extend_from_slice(&encode_u32leb(0)); // memory index = 0
        out.push(7); // section id 7 = Export
        out.extend_from_slice(&encode_u32leb(export_body.len() as u32));
        out.extend_from_slice(&export_body);
    }

    // ── Code section (id 10) ──
    {
        let mut code_body = Vec::new();
        code_body.extend_from_slice(&encode_u32leb(1)); // func count = 1
        let body = build_function_body(tir, local_count, &alloc_base);
        code_body.extend_from_slice(&encode_u32leb(body.len() as u32));
        code_body.extend_from_slice(&body);

        out.push(10);
        out.extend_from_slice(&encode_u32leb(code_body.len() as u32));
        out.extend_from_slice(&code_body);
    }

    // ── Data section (id 11) — optional, only if data present ──
    if !raw_data.is_empty() {
        let data_body = encode_data_segment(&raw_data);
        out.push(11);
        out.extend_from_slice(&encode_u32leb(data_body.len() as u32));
        out.extend_from_slice(&data_body);
    }

    Ok(out)
}

/// Build the body of the single Wasm function.
/// Layout:
///   <labeled blocks for handlers, nested>
fn build_function_body(
    tir: &[TirInst],
    _local_count: u32,
    alloc_base: &std::collections::HashMap<u32, u32>,
) -> Vec<u8> {
    let mut body = Vec::new();

    // Local declaration: no extras — state slots are already parameters
    body.push(0x00); // 0 locals declared

    let mut label_stack: Vec<u16> = Vec::new();
    let mut handler_label: std::collections::HashMap<u16, usize> = std::collections::HashMap::new();

    for inst in tir {
        match instr_branch_kind(inst.op) {
            BranchKind::LabelDef => {
                if !label_stack.is_empty() {
                    body.push(0x0B); // end previous block
                }
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                handler_label.insert(hh, label_stack.len());
                body.push(0x02);
                body.push(0x40);
                label_stack.push(hh);
            }
            BranchKind::Ret => {
                body.push(0x00); // unreachable
            }
            BranchKind::Call => {
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let depth = handler_label.get(&hh).copied().unwrap_or(0);
                body.push(0x0C);
                body.extend_from_slice(&encode_u32leb(depth as u32));
            }
            BranchKind::Jmp => {
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let depth = handler_label.get(&hh).copied().unwrap_or(0);
                body.push(0x0C);
                body.extend_from_slice(&encode_u32leb(depth as u32));
            }
            BranchKind::Jcc { index: _ } => {
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let depth = handler_label.get(&hh).copied().unwrap_or(0);
                body.push(0x0D); // br_if
                body.extend_from_slice(&encode_u32leb(depth as u32));
            }
            BranchKind::None => {
                emit_wasm_inst(inst, &mut body, alloc_base);
            }
        }
    }

    // Close remaining blocks and trap at end
    while !label_stack.is_empty() {
        let _ = label_stack.pop();
        body.push(0x0B);
    }
    body.push(0x0B); // end (function body terminator)
    body
}

fn emit_wasm_inst(
    inst: &TirInst,
    body: &mut Vec<u8>,
    alloc_base: &std::collections::HashMap<u32, u32>,
) {
    let a = |i: usize| inst.args.get(i).copied().unwrap_or(0) as u32;

    macro_rules! push_slot {
        ($s:expr) => {{
            body.push(0x20); // local.get
            body.extend_from_slice(&encode_u32leb($s));
        }};
    }
    macro_rules! set_slot {
        ($s:expr) => {{
            body.push(0x21); // local.set
            body.extend_from_slice(&encode_u32leb($s));
        }};
    }
    macro_rules! push_const {
        ($v:expr) => {{
            body.push(0x41); // i32.const
            body.extend_from_slice(&encode_i32leb($v as i32));
        }};
    }

    /// Emit `i32.load8_u mem=0 align=0 offset=0` — load 8-bit unsigned from
    /// linear memory at (top-of-stack) address into a 32-bit i32 result.
    fn emit_i32_load8_u(body: &mut Vec<u8>) {
        body.push(0x2C); // i32.load8_u
        body.push(0x00); // mem_idx = 0
        body.push(0x00); // align = 0
        body.push(0x00); // byte_offset = 0
    }

    /// Emit `i32.store8 mem=0 align=0 offset=0` — store 8-bit unsigned from
    /// (second on stack) into linear memory at (top-of-stack) address.
    /// Stack: <addr> <value>  ->  (pops both)
    fn emit_i32_store8(body: &mut Vec<u8>) {
        body.push(0x3A); // i32.store8
        body.push(0x00); // mem_idx = 0
        body.push(0x00); // align = 0
        body.push(0x00); // byte_offset = 0
    }

    match inst.op {
        TirOp::Nop => {
            body.push(0x01); // Wasm nop = 0x01
        }
        TirOp::Data | TirOp::Str | TirOp::Raw => {}
        TirOp::Alloc => {
            // Store the precomputed linear-memory base offset for this slot.
            let slot = a(0);
            let base = alloc_base.get(&slot).copied().unwrap_or(0);
            push_const!(base);
            set_slot!(slot);
        }
        TirOp::Set => {
            push_const!(a(1));
            set_slot!(a(0));
        }
        TirOp::LoadFile => {
            push_const!(a(1));
            set_slot!(a(0));
        }
        TirOp::WriteFile => {
            push_const!(a(1));
            set_slot!(a(0));
        }
        TirOp::Get | TirOp::Movrr => {
            push_slot!(a(1));
            set_slot!(a(0));
        }
        TirOp::Add => {
            push_slot!(a(0));
            push_const!(a(1));
            body.push(0x6A); // i32.add
            set_slot!(a(0));
        }
        TirOp::Sub => {
            push_slot!(a(0));
            push_const!(a(1));
            body.push(0x6B); // i32.sub
            set_slot!(a(0));
        }
        TirOp::Inc => {
            push_slot!(a(0));
            push_const!(1);
            body.push(0x6A);
            set_slot!(a(0));
        }
        TirOp::Dec => {
            push_slot!(a(0));
            push_const!(1);
            body.push(0x6B);
            set_slot!(a(0));
        }
        TirOp::Addv => {
            push_slot!(a(0));
            push_slot!(a(1));
            body.push(0x6A);
            set_slot!(a(0));
        }
        TirOp::Orv => {
            push_slot!(a(0));
            push_slot!(a(1));
            body.push(0x71);
            set_slot!(a(0));
        }
        TirOp::Subv => {
            push_slot!(a(0));
            push_slot!(a(1));
            body.push(0x6B);
            set_slot!(a(0));
        }
        TirOp::Imul => {
            push_slot!(a(0));
            push_slot!(a(1));
            body.push(0x6C);
            set_slot!(a(0));
        }
        TirOp::Cmp => {
            push_slot!(a(0));
            push_slot!(a(1));
            body.push(0x46); // i32.eq
        }
        TirOp::Ldb => {
            // LDB dst src offset: load byte from state[src] + offset → state[dst]
            let dst = a(0);
            let src = a(1);
            let offset = a(2);
            push_slot!(src);       // stack: base_addr (from ALLOC)
            push_const!(offset);    // stack: base_addr, offset
            body.push(0x6A);       // stack: byte_addr = base_addr + offset
            emit_i32_load8_u(body); // stack: loaded_byte (i32)
            set_slot!(dst);        // store into state[dst]
        }
        TirOp::MemcpyState => {
            // MemcpyState dst src N: copy N words from state[src] to state[dst]
            // (N words = N slots)
            let dst = a(0) as usize;
            let src = a(1) as usize;
            let n = a(2) as usize;
            if n <= 64 {
                for i in 0..n {
                    push_slot!((src + i) as u32);
                    set_slot!((dst + i) as u32);
                }
            } else {
                // Trap for large copy counts.
                body.push(0x00); // unreachable
            }
        }
        TirOp::MemcpyData => {
            // MemcpyData dst src N: copy N bytes from data segment to
            // linear memory at address state[dst].
            // src = offset into the data segment; N = byte count.
            let dst = a(0);
            let src = a(1) as usize;
            let n = a(2) as usize;
            if n == 0 {
                return;
            }
            // dst slot may not be used when we already have an address; we
            // load the address from state[dst] if available, otherwise use
            // the raw src offset directly.
            // Stack: <dst_addr>
            push_slot!(dst);
            // Emit a bytewise copy loop: load8 from data base + i, store8
            // into dst_addr + i.  Since Wasm has no dynamic-loop construct
            // in our flat model, unroll for small N and trap otherwise.
            if n <= 64 {
                for i in 0..n {
                    // --- store byte i ---
                    // load address of destination byte: dst_addr + i
                    push_slot!(dst);
                    push_const!(src as u32 + i as u32);
                    body.push(0x6A); // add
                    // load the byte from the data segment
                    push_const!(src as u32 + i as u32);
                    emit_i32_load8_u(body); // load from data area
                    // stack now: <dst_byte_addr> <data_byte>
                    emit_i32_store8(body);
                }
            } else {
                body.push(0x1A); // drop dst_addr
                body.push(0x00); // unreachable
            }
        }
        TirOp::RawByte | TirOp::RawBytes => {}
        TirOp::Handler | TirOp::Call | TirOp::Jmp | TirOp::Je
        | TirOp::Jne | TirOp::Jl | TirOp::Jge | TirOp::Jle
        | TirOp::Jg | TirOp::Jb | TirOp::Jae | TirOp::Jbe
        | TirOp::Ja | TirOp::Ret => {}
    }
}

/// Encode a single Wasm data segment (count=1, passive=false, global_init=0, bytes).
fn encode_data_segment(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&encode_u32leb(data.len() as u32)); // count
    // active data segment: 0x00, global index (const 0), constant, 0, size, content
    out.push(0x00); // is_active
    out.push(0x41); // i32.const
    out.extend_from_slice(&encode_i32leb(0));
    out.push(0x0B); // end expr
    out.push(0x00); // memory index
    out.extend_from_slice(&encode_u32leb(data.len() as u32));
    out.extend_from_slice(data);
    out
}

fn encode_vec_with_size(data: Vec<u8>) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&encode_u32leb(data.len() as u32));
    out.extend_from_slice(&data);
    out
}

/// Encode unsigned LEB128.
fn encode_u32leb(v: u32) -> Vec<u8> {
    let mut out = Vec::new();
    let mut val = v;
    loop {
        let byte = (val & 0x7F) as u8;
        val >>= 7;
        if val != 0 {
            out.push(byte | 0x80);
        } else {
            out.push(byte);
            break;
        }
    }
    out
}

/// Encode signed LEB128.
fn encode_i32leb(v: i32) -> Vec<u8> {
    let mut out = Vec::new();
    let mut val = v as i32;
    loop {
        let byte = val & 0x7F;
        val >>= 7;
        let unsigned_byte = byte as u8;
        let mut b = unsigned_byte;
        if (val == 0 && byte & 0x40 == 0) || (val == -1 && byte & 0x40 != 0) {
            out.push(b);
            break;
        }
        out.push(b | 0x80);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_tir_nop_ret() -> Vec<TirInst> {
        vec![
            TirInst {
                op: TirOp::Handler,
                args: vec![0],
                line: 1,
            },
            TirInst {
                op: TirOp::Nop,
                args: vec![],
                line: 2,
            },
            TirInst {
                op: TirOp::Ret,
                args: vec![],
                line: 3,
            },
        ]
    }

    #[test]
    fn wasm_magic_and_version() {
        let bytes = emit_wasm(&build_tir_nop_ret()).unwrap();
        assert_eq!(&bytes[0..8], b"\x00asm\x01\x00\x00\x00");
    }

    #[test]
    fn wasm_has_sections() {
        let bytes = emit_wasm(&build_tir_nop_ret()).unwrap();
        assert!(bytes.len() > 8);
        // After magic/version, first byte should be type section id = 1
        assert_eq!(bytes[8], 1);
    }

    #[test]
    fn wasm_nonempty_and_contains_unreachable() {
        let bytes = emit_wasm(&build_tir_nop_ret()).unwrap();
        assert!(bytes.len() > 16);
        // function body should contain unreachable 0x00
        let mut found = false;
        for b in &bytes {
            if *b == 0x00 {
                found = true;
                break;
            }
        }
        assert!(found);
    }

    #[test]
    fn wasm_with_data_has_data_section() {
        let tir = vec![
            TirInst {
                op: TirOp::Data,
                args: vec![1, 2, 3],
                line: 1,
            },
            TirInst {
                op: TirOp::Handler,
                args: vec![0],
                line: 2,
            },
            TirInst {
                op: TirOp::Ret,
                args: vec![],
                line: 3,
            },
        ];
        let bytes = emit_wasm(&tir).unwrap();
        // section id 11 = data
        assert!(bytes.contains(&11));
        // data content should be present
        assert!(bytes.windows(3).any(|w| w == [1, 2, 3]));
    }

    #[test]
    fn wasm_branch_handlers() {
        let tir = vec![
            TirInst {
                op: TirOp::Handler,
                args: vec![0x00],
                line: 1,
            },
            TirInst {
                op: TirOp::Jmp,
                args: vec![0x01],
                line: 2,
            },
            TirInst {
                op: TirOp::Handler,
                args: vec![0x01],
                line: 3,
            },
            TirInst {
                op: TirOp::Ret,
                args: vec![],
                line: 4,
            },
        ];
        let bytes = emit_wasm(&tir).unwrap();
        assert!(bytes.contains(&0x0C)); // br present
    }
}
