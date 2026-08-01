//! CUDA PTX backend — maps YOYO TIR operations to PTX instructions.
//!
//! Produces a `.ptx` text file from the same TIR instructions that the x64
//! backends consume. The resulting PTX is a GPU kernel that operates on state
//! in global memory.

use crate::tir::{instr_branch_kind, BranchKind, TirInst, TirOp};
use crate::types::IsaResult;

/// Register index allocator for PTX virtual registers.
struct RegAlloc {
    next: u32,
}

impl RegAlloc {
    fn new() -> Self {
        Self { next: 1 } // %rd0 is reserved for the state pointer
    }

    fn alloc(&mut self) -> String {
        let r = self.next;
        self.next += 1;
        format!("%rd{}", r)
    }

    fn reset(&mut self) {
        self.next = 1; // %rd0 is reserved for the state pointer
    }
}

/// Emit PTX text from TIR instructions.
///
/// Returns valid PTX assembly text that can be compiled by `ptxas`.
pub fn emit_cuda(tir: &[TirInst]) -> IsaResult<String> {
    let mut ptx = String::new();

    // PTX header
    ptx.push_str(".version 7.0\n");
    ptx.push_str(".target sm_50\n");
    ptx.push_str(".address_size 64\n\n");

    // Declare kernel entry point with state pointer parameter
    ptx.push_str(".visible .entry kernel_function(\n");
    ptx.push_str("    .param .u64 .ptr .global .u64 *state_ptr\n");
    ptx.push_str(")\n");
    ptx.push_str("{\n");

    // Register declarations
    ptx.push_str("    .reg .u64 %rd<256>;\n");
    ptx.push_str("    .reg .pred %p<16>;\n");
    ptx.push_str("    .reg .u8 %rb<16>;\n\n");

    // Load state pointer from parameter
    ptx.push_str("    ld.param.u64 %rd0, [state_ptr];\n");
    let state_ptr_reg = "%rd0";

    // Collect data bytes for .rodata section
    let mut data_bytes: Vec<u8> = Vec::new();
    for inst in tir {
        match inst.op {
            TirOp::Data | TirOp::Str | TirOp::Raw => {
                for v in &inst.args {
                    data_bytes.push(*v as u8);
                }
            }
            _ => {}
        }
    }

    // Emit .rodata section
    if !data_bytes.is_empty() {
        ptx.push_str("\n.section .rodata\n");
        ptx.push_str(".align 8\n");
        ptx.push_str("data_bytes:\n");
        ptx.push_str("    .b8 ");
        for (i, b) in data_bytes.iter().enumerate() {
            if i > 0 {
                ptx.push_str(", ");
            }
            ptx.push_str(&format!("0x{:02x}", b));
        }
        ptx.push_str("\n\n");
    }

    // Build label map from handler definitions
    let mut label_map: std::collections::HashMap<u16, String> = std::collections::HashMap::new();
    for inst in tir {
        if let TirOp::Handler = inst.op {
            let hh = inst.args.first().copied().unwrap_or(0) as u16;
            label_map.insert(hh, format!("H_{:02X}", hh));
        }
    }

    // Second pass: emit PTX instructions for each handler
    let mut in_handler = false;
    let mut regs = RegAlloc::new();
    regs.reset();

    for inst in tir {
        match instr_branch_kind(inst.op) {
            BranchKind::LabelDef => {
                // Close previous handler body with ret
                if in_handler {
                    ptx.push_str("    ret;\n");
                }
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let label = label_map.get(&hh).cloned().unwrap_or_else(|| format!("H_{:02X}", hh));
                ptx.push_str(&format!("\n{}:\n", label));
                in_handler = true;
                regs.reset();
            }
            BranchKind::Ret => {
                // ret emitted when we close the handler (next HANDLER or end)
            }
            BranchKind::Call => {
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let target = label_map.get(&hh).cloned().unwrap_or_else(|| format!("H_{:02X}", hh));
                ptx.push_str(&format!("    // CALL {}\n", target));
                ptx.push_str(&format!("    bra.uni {};\n", target));
            }
            BranchKind::Jmp => {
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let target = label_map.get(&hh).cloned().unwrap_or_else(|| format!("H_{:02X}", hh));
                ptx.push_str(&format!("    // JMP {}\n", target));
                ptx.push_str(&format!("    bra.uni {};\n", target));
            }
            BranchKind::Jcc { index: _ } => {
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let target = label_map.get(&hh).cloned().unwrap_or_else(|| format!("H_{:02X}", hh));
                ptx.push_str(&format!("    // JCC {}\n", target));
                ptx.push_str(&format!("    @%p0 bra.uni {};\n", target));
            }
            BranchKind::None => {
                let lines = emit_cuda_inst(inst, &mut regs, state_ptr_reg)?;
                for l in lines {
                    ptx.push_str(&format!("    {}\n", l));
                }
            }
        }
    }

    // Close last handler
    if in_handler {
        ptx.push_str("    ret;\n");
    }

    ptx.push_str("}\n");
    Ok(ptx)
}

/// PTX load state[slot] into a register, pushing instructions to `out`.
fn ptx_load_slot(out: &mut Vec<String>, regs: &mut RegAlloc, slot: u16, state_ptr: &str) -> String {
    let r = regs.alloc();
    let offset = slot as u64 * 8;
    out.push(format!(
        "ld.global.u64 {}, [{} + {}];",
        r, state_ptr, offset
    ));
    r
}

/// PTX store a register to state[slot], pushing instructions to `out`.
fn ptx_store_slot(out: &mut Vec<String>, slot: u16, reg: &str, state_ptr: &str) {
    let offset = slot as u64 * 8;
    out.push(format!(
        "st.global.u64 [{} + {}], {};",
        state_ptr, offset, reg
    ));
}

/// Emit a single non-branch TIR instruction as PTX text lines.
fn emit_cuda_inst(
    inst: &TirInst,
    regs: &mut RegAlloc,
    state_ptr: &str,
) -> IsaResult<Vec<String>> {
    let a = |i: usize| inst.args.get(i).copied().unwrap_or(0);
    let mut out = Vec::new();

    match inst.op {
        TirOp::Nop => {
            out.push("nop;".to_string());
        }
        TirOp::Data | TirOp::Str | TirOp::Raw => {
            // Data is emitted in the .rodata section; nothing to do in code
        }
        TirOp::Alloc => {
            let slot = a(0) as u16;
            let size = a(1);
            let r = regs.alloc();
            out.push(format!("mov.u64 {}, {};", r, size));
            ptx_store_slot(&mut out, slot, &r, state_ptr);
        }
        TirOp::Set => {
            let slot = a(0) as u16;
            let imm = a(1);
            let r = regs.alloc();
            out.push(format!("mov.u64 {}, {};", r, imm));
            ptx_store_slot(&mut out, slot, &r, state_ptr);
        }
        TirOp::Get | TirOp::Movrr => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            let r = ptx_load_slot(&mut out, regs, src, state_ptr);
            ptx_store_slot(&mut out, dst, &r, state_ptr);
        }
        TirOp::Add => {
            let slot = a(0) as u16;
            let imm = a(1);
            let r = ptx_load_slot(&mut out, regs, slot, state_ptr);
            let r2 = regs.alloc();
            out.push(format!("mov.u64 {}, {};", r2, imm));
            out.push(format!("add.u64 {}, {}, {};", r, r, r2));
            ptx_store_slot(&mut out, slot, &r, state_ptr);
        }
        TirOp::Sub => {
            let slot = a(0) as u16;
            let imm = a(1);
            let r = ptx_load_slot(&mut out, regs, slot, state_ptr);
            let r2 = regs.alloc();
            out.push(format!("mov.u64 {}, {};", r2, imm));
            out.push(format!("sub.u64 {}, {}, {};", r, r, r2));
            ptx_store_slot(&mut out, slot, &r, state_ptr);
        }
        TirOp::Inc => {
            let slot = a(0) as u16;
            let r = ptx_load_slot(&mut out, regs, slot, state_ptr);
            let r1 = regs.alloc();
            out.push(format!("mov.u64 {}, 1;", r1));
            out.push(format!("add.u64 {}, {}, {};", r, r, r1));
            ptx_store_slot(&mut out, slot, &r, state_ptr);
        }
        TirOp::Dec => {
            let slot = a(0) as u16;
            let r = ptx_load_slot(&mut out, regs, slot, state_ptr);
            let r1 = regs.alloc();
            out.push(format!("mov.u64 {}, 1;", r1));
            out.push(format!("sub.u64 {}, {}, {};", r, r, r1));
            ptx_store_slot(&mut out, slot, &r, state_ptr);
        }
        TirOp::Addv => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            let r0 = ptx_load_slot(&mut out, regs, dst, state_ptr);
            let r1 = ptx_load_slot(&mut out, regs, src, state_ptr);
            out.push(format!("add.u64 {}, {}, {};", r0, r0, r1));
            ptx_store_slot(&mut out, dst, &r0, state_ptr);
        }
        TirOp::Orv => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            let r0 = ptx_load_slot(&mut out, regs, dst, state_ptr);
            let r1 = ptx_load_slot(&mut out, regs, src, state_ptr);
            out.push(format!("or.u64 {}, {}, {};", r0, r0, r1));
            ptx_store_slot(&mut out, dst, &r0, state_ptr);
        }
        TirOp::Subv => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            let r0 = ptx_load_slot(&mut out, regs, dst, state_ptr);
            let r1 = ptx_load_slot(&mut out, regs, src, state_ptr);
            out.push(format!("sub.u64 {}, {}, {};", r0, r0, r1));
            ptx_store_slot(&mut out, dst, &r0, state_ptr);
        }
        TirOp::Imul => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            let r0 = ptx_load_slot(&mut out, regs, dst, state_ptr);
            let r1 = ptx_load_slot(&mut out, regs, src, state_ptr);
            out.push(format!("mul.lo.u64 {}, {}, {};", r0, r0, r1));
            ptx_store_slot(&mut out, dst, &r0, state_ptr);
        }
        TirOp::Cmp => {
            let a_slot = a(0) as u16;
            let b_slot = a(1) as u16;
            let r0 = ptx_load_slot(&mut out, regs, a_slot, state_ptr);
            let r1 = ptx_load_slot(&mut out, regs, b_slot, state_ptr);
            out.push(format!("setp.eq.u64 %p0, {}, {};", r0, r1));
        }
        TirOp::Ldb => {
            let dd = a(0) as u16;
            let ss = a(1) as u16;
            let oo = a(2) as u16;
            let r_addr = ptx_load_slot(&mut out, regs, ss, state_ptr);
            if oo != 0 {
                let r_off = regs.alloc();
                out.push(format!("mov.u64 {}, {};", r_off, oo));
                out.push(format!("add.u64 {}, {}, {};", r_addr, r_addr, r_off));
            }
            let r_byte = regs.alloc();
            out.push(format!("ld.global.u8 {}, [{}];", r_byte, r_addr));
            ptx_store_slot(&mut out, dd, &r_byte, state_ptr);
        }
        TirOp::LoadFile => {
            let slot = a(0) as u16;
            let str_idx = a(1);
            let r = regs.alloc();
            out.push(format!("mov.u64 {}, {};", r, str_idx));
            ptx_store_slot(&mut out, slot, &r, state_ptr);
        }
        TirOp::WriteFile => {
            let slot = a(0) as u16;
            let str_idx = a(1);
            let r = regs.alloc();
            out.push(format!("mov.u64 {}, {};", r, str_idx));
            ptx_store_slot(&mut out, slot, &r, state_ptr);
        }
        TirOp::MemcpyData => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            let n = a(2) as u16;
            let r_src = ptx_load_slot(&mut out, regs, src, state_ptr);
            let r_dst = ptx_load_slot(&mut out, regs, dst, state_ptr);
            let r_i = regs.alloc();
            out.push(format!("mov.u64 {}, 0;", r_i));
            let loop_label = format!("memcpy_loop_{}_{}", src, dst);
            out.push(format!("{}:", loop_label));
            let r_byte = regs.alloc();
            out.push(format!("ld.global.u8 {}, [{} + {}];", r_byte, r_src, r_i));
            out.push(format!("st.global.u8 [{} + {}], {};", r_dst, r_i, r_byte));
            out.push(format!("add.u64 {}, {}, 1;", r_i, r_i));
            out.push(format!("setp.lt.u64 %p1, {}, {};", r_i, n));
            out.push(format!("@%p1 bra.uni {};", loop_label));
        }
        TirOp::MemcpyState => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            let n = a(2) as u16;
            let loop_label = format!("memcpy_state_loop_{}_{}", src, dst);
            let r_i = regs.alloc();
            out.push(format!("mov.u64 {}, 0;", r_i));
            out.push(format!("{}:", loop_label));
            let r_idx_src = regs.alloc();
            let r_idx_dst = regs.alloc();
            // Compute byte offsets: (slot + i) * 8
            out.push(format!("mul.u64 {}, {}, 8;", r_idx_src, r_i));
            out.push(format!("mul.u64 {}, {}, 8;", r_idx_dst, r_i));
            // Add base offset for src slot
            let r_src_addr = regs.alloc();
            let r_dst_addr = regs.alloc();
            out.push(format!("mov.u64 {}, {};", r_src_addr, src as u64 * 8));
            out.push(format!("mov.u64 {}, {};", r_dst_addr, dst as u64 * 8));
            out.push(format!("add.u64 {}, {}, {};", r_src_addr, r_src_addr, r_idx_src));
            out.push(format!("add.u64 {}, {}, {};", r_dst_addr, r_dst_addr, r_idx_dst));
            let r_val = regs.alloc();
            out.push(format!("ld.global.u64 {}, [{} + {}];", r_val, state_ptr, r_src_addr));
            out.push(format!("st.global.u64 [{} + {}], {};", state_ptr, r_dst_addr, r_val));
            out.push(format!("add.u64 {}, {}, 1;", r_i, r_i));
            out.push(format!("setp.lt.u64 %p1, {}, {};", r_i, n));
            out.push(format!("@%p1 bra.uni {};", loop_label));
        }
        TirOp::RawByte => {
            out.push(format!("// RAW_BYTE 0x{:02x}", a(0) as u8));
        }
        TirOp::RawBytes => {
            let bytes: Vec<String> = inst.args.iter().map(|v| format!("0x{:02x}", *v as u8)).collect();
            out.push(format!("// RAW_BYTES {}", bytes.join(" ")));
        }
        TirOp::Handler | TirOp::Call | TirOp::Jmp | TirOp::Je | TirOp::Jne
        | TirOp::Jl | TirOp::Jge | TirOp::Jle | TirOp::Jg | TirOp::Jb
        | TirOp::Jae | TirOp::Jbe | TirOp::Ja | TirOp::Ret => {}
    }

    Ok(out)
}