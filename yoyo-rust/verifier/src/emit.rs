//! Emit pipeline: TirInst[] → x64 bytes (PROMPT-v3 Part 4.5 / Phase 1).

use crate::assembler::{
    self, JCC_TABLE,
};
use crate::fixup::FixupTable;
use crate::platform::{BranchFixup, PlatformBackend, PlatformKind, select_platform};
use crate::tir::{instr_branch_kind, opcode_to_u8, BranchKind, TirInst, TirOp};
use crate::ty_parser;
use crate::types::{IsaError, IsaResult, CODE_BUF_CAP, FixedBuf};

pub struct EmitOutput {
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    pub entry_hh: u16,
    pub labels: FixupTable,
    /// (hh, offset, length) for each handler — used by --selfhost
    pub handler_offsets: Vec<(u16, u32, u32)>,
}

fn label_hh(args: &[u64]) -> IsaResult<u16> {
    let raw = *args.first().unwrap_or(&0);
    if raw > 0xffff {
        return Err(IsaError::LabelOutOfRange { hh: 0xffff });
    }
    Ok(raw as u16)
}

pub fn emit(tir: &[TirInst], platform: PlatformKind) -> IsaResult<EmitOutput> {
    emit_internal(tir, platform, false)
}

/// Compile from SourceLine entries (used by selfhost path).
pub fn emit_from_lines(lines: &[ty_parser::SourceLine], platform: PlatformKind) -> IsaResult<EmitOutput> {
    let mut tir = Vec::new();
    for line in lines {
        let args = ty_parser::resolve_line(line)?;
        tir.push(crate::tir::lower_op_checked(line.opcode, &args, line.line)?);
    }
    emit_internal(&tir, platform, true)
}

fn emit_internal(tir: &[TirInst], platform: PlatformKind, track_handlers: bool) -> IsaResult<EmitOutput> {
    let mut backend = select_platform(platform);
    let mut code: Box<FixedBuf<CODE_BUF_CAP>> = FixedBuf::new_boxed();
    let mut data: Vec<u8> = Vec::new();
    let mut labels = FixupTable::new();
    // (branch_start, hh, fixup)
    struct PendingFixup {
        branch_start: usize,
        hh: u16,
        fixup: BranchFixup,
    }
    let mut pending_fixups: Vec<PendingFixup> = Vec::new();
    let mut entry_hh: u16 = 0;
    let mut first_handler = true;
    let mut current_hh: u16 = 0xFFFF;
    let mut handler_offsets: Vec<(u16, u32, u32)> = Vec::new();
    let mut handler_start: u32 = 0;

    for inst in tir {
        let op = opcode_to_u8(inst.op);
        match instr_branch_kind(inst.op) {
            BranchKind::LabelDef => {
                if current_hh != 0xFFFF {
                    handler_offsets.push((current_hh, handler_start, code.tell() as u32 - handler_start));
                }
                let hh = label_hh(&inst.args)?;
                labels.define(hh, code.tell() as u32)?;
                if first_handler {
                    entry_hh = hh;
                    first_handler = false;
                }
                current_hh = hh;
                handler_start = code.tell() as u32;
            }
            BranchKind::Call | BranchKind::Jmp | BranchKind::Jcc { .. } => {
                let hh = label_hh(&inst.args)?;
                let branch_start = code.tell();
                let (bytes, fixup) = match instr_branch_kind(inst.op) {
                    BranchKind::Call => backend.emit_call_branch()?,
                    BranchKind::Jmp => backend.emit_jmp_branch()?,
                    BranchKind::Jcc { index } => {
                        backend.emit_jcc_branch(JCC_TABLE[index as usize])?
                    }
                    _ => unreachable!(),
                };
                code.extend_from_slice(&bytes)?;
                pending_fixups.push(PendingFixup { branch_start, hh, fixup });
            }
            BranchKind::Ret => {
                code.extend_from_slice(&backend.emit_ret()?)?;
            }
            BranchKind::None => {
                let bytes = emit_one(inst, &mut *backend, &mut data)?;
                code.extend_from_slice(&bytes)?;
            }
        }
        let _ = op;
    }
    let _code_len = code.tell();
    if current_hh != 0xFFFF {
        handler_offsets.push((current_hh, handler_start, code.tell() as u32 - handler_start));
    }
    // Pass 2: patch branches using per-arch patching
    let mut code_slice = code.slice().to_vec();
    for pf in &pending_fixups {
        let target = labels
            .lookup(pf.hh)
            .ok_or(IsaError::LabelOutOfRange { hh: pf.hh })?;
        backend.patch_branch(&mut code_slice, pf.branch_start, &pf.fixup, target)?;
    }
    code_slice.truncate(code_slice.len()); // ensure correct length

    Ok(EmitOutput {
        code: code_slice,
        data,
        entry_hh,
        labels,
        handler_offsets,
    })
}

fn emit_one(
    inst: &TirInst,
    backend: &mut dyn PlatformBackend,
    data: &mut Vec<u8>,
) -> IsaResult<Vec<u8>> {
    let a = |i: usize| inst.args.get(i).copied().unwrap_or(0);
    match inst.op {
        TirOp::Nop => backend.emit_nop(),
        TirOp::Data | TirOp::Str | TirOp::Raw => {
            for v in &inst.args {
                data.push(*v as u8);
            }
            Ok(vec![])
        }
        TirOp::Alloc => backend.emit_alloc(a(0) as u16, a(1)),
        TirOp::Set => backend.emit_set(a(0) as u16, a(1)),
        TirOp::LoadFile => backend.emit_load_file(a(0) as u16, a(1) as u8),
        TirOp::WriteFile => backend.emit_write_file(a(0) as u16, a(1) as u8, a(2) as u16),
        TirOp::Get => backend.emit_get(a(0) as u16, a(1) as u16),
        TirOp::Sub => backend.emit_sub_imm(a(0) as u16, a(1)),
        TirOp::Add => backend.emit_add_imm(a(0) as u16, a(1)),
        TirOp::Imul => backend.emit_imul(a(0) as u16, a(1) as u16),
        TirOp::Movrr => backend.emit_movrr(a(0) as u16, a(1) as u16),
        TirOp::Cmp => backend.emit_cmp(a(0) as u16, a(1) as u16),
        TirOp::Inc => backend.emit_inc(a(0) as u16),
        TirOp::Dec => backend.emit_dec(a(0) as u16),
        TirOp::Addv => backend.emit_addv(a(0) as u16, a(1) as u16),
        TirOp::Orv => backend.emit_orv(a(0) as u16, a(1) as u16),
        TirOp::Subv => backend.emit_subv(a(0) as u16, a(1) as u16),
        TirOp::Ldb => backend.emit_ldb(a(0) as u16, a(1) as u16, a(2) as u16),
        TirOp::MemcpyData => backend.emit_memcpy_data(a(0) as u16, a(1) as u16, a(2) as u16),
        TirOp::MemcpyState => backend.emit_memcpy_state(a(0) as u16, a(1) as u16, a(2) as u16),
        TirOp::RawByte => backend.emit_raw_byte(a(0) as u8),
        TirOp::RawBytes => backend.emit_raw_bytes(inst.args.iter().map(|v| *v as u8).collect()),
        TirOp::Handler | TirOp::Call | TirOp::Jmp | TirOp::Je | TirOp::Jne | TirOp::Jl
        | TirOp::Jge | TirOp::Jle | TirOp::Jg | TirOp::Jb | TirOp::Jae | TirOp::Jbe
        | TirOp::Ja | TirOp::Ret => Ok(vec![]),
    }
}

/// LDB dd ss oo — load byte from mem[state[ss]+oo] into state[dd] (zero-extend).
fn emit_ldb(dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
    use crate::assembler::{load_state, store_state};
    use crate::types::Reg;
    let mut out = load_state(ss, Reg::Rax)?;
    if oo != 0 {
        out.extend(assembler::add_imm(Reg::Rax, oo as u64)?);
    }
    // movzx rax, byte [rax]
    out.extend_from_slice(&[0x48, 0x0F, 0xB6, 0x00]);
    out.extend(store_state(dd, Reg::Rax)?);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tir::lower_op_checked;

    #[test]
    fn emit_set_ret() {
        let tir = vec![
            lower_op_checked(0x40, &[0x20], 1).unwrap(),
            lower_op_checked(0x30, &[0x50, 0], 2).unwrap(),
            lower_op_checked(0xFF, &[], 3).unwrap(),
        ];
        let out = emit(&tir, PlatformKind::Stub).unwrap();
        assert!(!out.code.is_empty());
        assert_eq!(*out.code.last().unwrap(), 0xC3);
    }

    #[test]
    fn emit_branch_patch() {
        let tir = vec![
            lower_op_checked(0x40, &[0x20], 1).unwrap(),
            lower_op_checked(0x70, &[0x21], 2).unwrap(),
            lower_op_checked(0x40, &[0x21], 3).unwrap(),
            lower_op_checked(0xFF, &[], 4).unwrap(),
        ];
        let out = emit(&tir, PlatformKind::Stub).unwrap();
        assert!(out.code.len() >= 6);
    }
}
