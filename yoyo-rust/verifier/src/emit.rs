//! Emit pipeline: TirInst[] → x64 bytes (PROMPT-v3 Part 4.5 / Phase 1).

use crate::assembler::{
    self, call_rel32, emit_add_imm, emit_addv, emit_cmp, emit_dec, emit_get, emit_imul, emit_inc,
    emit_memcpy_data, emit_memcpy_state, emit_orv, emit_set, emit_sub_imm, emit_subv, jcc_rel32,
    jmp_rel32, ret, JCC_TABLE,
};
use crate::fixup::FixupTable;
use crate::platform::{PlatformBackend, PlatformKind, select_platform};
use crate::tir::{instr_branch_kind, opcode_to_u8, BranchKind, TirInst, TirOp};
use crate::types::{IsaError, IsaResult, CODE_BUF_CAP, FixedBuf};

pub struct EmitOutput {
    pub code: Vec<u8>,
    pub data: Vec<u8>,
    pub entry_hh: u16,
    pub labels: FixupTable,
}

fn label_hh(args: &[u64]) -> IsaResult<u16> {
    let raw = *args.first().unwrap_or(&0);
    if raw > 0xffff {
        return Err(IsaError::LabelOutOfRange { hh: 0xffff });
    }
    Ok(raw as u16)
}

pub fn emit(tir: &[TirInst], platform: PlatformKind) -> IsaResult<EmitOutput> {
    let mut backend = select_platform(platform);
    let mut code: Box<FixedBuf<CODE_BUF_CAP>> = FixedBuf::new_boxed();
    let mut data: Vec<u8> = Vec::new();
    let mut labels = FixupTable::new();
    let mut pending_fixups: Vec<(usize, u16, BranchKind)> = Vec::new();
    let mut entry_hh: u16 = 0;
    let mut first_handler = true;

    // Pass 1: emit with placeholder rel32 = 0
    for inst in tir {
        let op = opcode_to_u8(inst.op);
        match instr_branch_kind(inst.op) {
            BranchKind::LabelDef => {
                let hh = label_hh(&inst.args)?;
                labels.define(hh, code.tell() as u32)?;
                if first_handler {
                    entry_hh = hh;
                    first_handler = false;
                }
            }
            BranchKind::Call | BranchKind::Jmp | BranchKind::Jcc { .. } => {
                let hh = label_hh(&inst.args)?;
                let start = code.tell();
                let bytes = match instr_branch_kind(inst.op) {
                    BranchKind::Call => call_rel32(0)?,
                    BranchKind::Jmp => jmp_rel32(0)?,
                    BranchKind::Jcc { index } => jcc_rel32(JCC_TABLE[index as usize], 0)?,
                    _ => unreachable!(),
                };
                code.extend_from_slice(&bytes)?;
                // rel32 starts at offset +1 for call/jmp, +2 for jcc
                let rel_at = match instr_branch_kind(inst.op) {
                    BranchKind::Jcc { .. } => start + 2,
                    _ => start + 1,
                };
                pending_fixups.push((rel_at, hh, instr_branch_kind(inst.op)));
            }
            BranchKind::Ret => {
                code.extend_from_slice(&ret())?;
            }
            BranchKind::None => {
                let bytes = emit_one(inst, &mut *backend, &mut data)?;
                code.extend_from_slice(&bytes)?;
            }
        }
        let _ = op;
    }

    // Pass 2: patch rel32
    let _code_len = code.tell();
    for (rel_at, hh, kind) in pending_fixups {
        let target = labels
            .lookup(hh)
            .ok_or(IsaError::LabelOutOfRange { hh })?;
        let _ = kind;
        let rel = target as i32 - (rel_at as i32 + 4);
        code.patch_u32_le(rel_at, rel as u32)?;
    }

    Ok(EmitOutput {
        code: code.slice().to_vec(),
        data,
        entry_hh,
        labels,
    })
}

fn emit_one(
    inst: &TirInst,
    backend: &mut dyn PlatformBackend,
    data: &mut Vec<u8>,
) -> IsaResult<Vec<u8>> {
    let a = |i: usize| inst.args.get(i).copied().unwrap_or(0);
    match inst.op {
        TirOp::Nop => Ok(vec![0x90]),
        TirOp::Data | TirOp::Str | TirOp::Raw => {
            // Data defs: append payload to data section, emit nothing in code
            for v in &inst.args {
                data.push(*v as u8);
            }
            Ok(vec![])
        }
        TirOp::Alloc => backend.emit_alloc(a(0) as u16, a(1)),
        TirOp::Set => emit_set(a(0) as u16, a(1)),
        TirOp::LoadFile => backend.emit_load_file(a(0) as u16, a(1) as u8),
        TirOp::WriteFile => backend.emit_write_file(a(0) as u16, a(1) as u8, a(2) as u16),
        TirOp::Get => emit_get(a(0) as u16, a(1) as u16),
        TirOp::Sub => emit_sub_imm(a(0) as u16, a(1)),
        TirOp::Add => emit_add_imm(a(0) as u16, a(1)),
        TirOp::Imul => emit_imul(a(0) as u16, a(1) as u16),
        TirOp::Movrr => emit_get(a(0) as u16, a(1) as u16),
        TirOp::Cmp => emit_cmp(a(0) as u16, a(1) as u16),
        TirOp::Inc => emit_inc(a(0) as u16),
        TirOp::Dec => emit_dec(a(0) as u16),
        TirOp::Addv => emit_addv(a(0) as u16, a(1) as u16),
        TirOp::Orv => emit_orv(a(0) as u16, a(1) as u16),
        TirOp::Subv => emit_subv(a(0) as u16, a(1) as u16),
        TirOp::Ldb => emit_ldb(a(0) as u16, a(1) as u16, a(2) as u16),
        TirOp::MemcpyData => emit_memcpy_data(a(1) as u16, a(0) as u16, a(2) as u16),
        TirOp::MemcpyState => emit_memcpy_state(a(1) as u16, a(0) as u16, a(2) as u16),
        TirOp::RawByte => Ok(vec![a(0) as u8]),
        TirOp::RawBytes => Ok(inst.args.iter().map(|v| *v as u8).collect()),
        // Handlers / branches / ret handled in emit()
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
