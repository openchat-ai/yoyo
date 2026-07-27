//! TIR layer — generated TirOp + hand-written TirInst / lower (PROMPT-v3 Part 4.4–4.5).

use isa_proc::isa;

use crate::types::{IsaError, IsaResult};

isa!(include_str!("isa_table.txt"));

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TirInst {
    pub op: TirOp,
    pub args: Vec<u64>,
    pub line: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchKind {
    None,
    LabelDef,
    Call,
    Jmp,
    Jcc { index: u8 },
    Ret,
}

pub fn instr_branch_kind(op: TirOp) -> BranchKind {
    match opcode_to_u8(op) {
        0x40 => BranchKind::LabelDef,
        0x41 => BranchKind::Call,
        0x70 => BranchKind::Jmp,
        x @ 0x71..=0x7A => BranchKind::Jcc {
            index: x - 0x71,
        },
        0xFF => BranchKind::Ret,
        _ => BranchKind::None,
    }
}

/// Lower raw opcode + numeric args into a typed TirInst.
pub fn lower_op(op: u8, args: &[u64], line: usize) -> IsaResult<TirInst> {
    let tir_op = opcode_from_u8(op).ok_or(IsaError::ArgCountMismatch {
        op,
        expected: 0,
        got: args.len(),
    })?;
    // Unknown opcode uses ArgCountMismatch as stand-in when opcode missing —
    // remapped below for clarity.
    let expected = instr_arity(tir_op);
    // RAW_BYTES / RAW / DATA / STR are variadic — accept any arity ≥ declared.
    let variadic = matches!(
        op,
        0x10 | 0x12 | 0x13 | 0xA1
    );
    if !variadic && args.len() != expected {
        return Err(IsaError::ArgCountMismatch {
            op,
            expected,
            got: args.len(),
        });
    }
    if variadic && args.len() < expected {
        return Err(IsaError::ArgCountMismatch {
            op,
            expected,
            got: args.len(),
        });
    }
    Ok(TirInst {
        op: tir_op,
        args: args.to_vec(),
        line,
    })
}

/// Fix the misleading error when opcode is unknown.
pub fn lower_op_checked(op: u8, args: &[u64], line: usize) -> IsaResult<TirInst> {
    if opcode_from_u8(op).is_none() {
        return Err(IsaError::ParseError {
            line,
            msg: format!("unknown opcode 0x{op:02X}"),
        });
    }
    lower_op(op, args, line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn isa_has_set() {
        assert_eq!(opcode_from_u8(0x30), Some(TirOp::Set));
        assert_eq!(instr_name(TirOp::Set), "SET");
        assert_eq!(instr_arity(TirOp::Set), 2);
    }

    #[test]
    fn isa_jcc_table() {
        assert_eq!(JCC_TABLE[0], 0x84);
        assert_eq!(JCC_MNEMONIC[0], "je");
    }

    #[test]
    fn lower_set() {
        let t = lower_op_checked(0x30, &[0x50, 0], 1).unwrap();
        assert_eq!(t.op, TirOp::Set);
    }

    #[test]
    fn lower_arity_mismatch() {
        assert!(lower_op_checked(0x30, &[1], 1).is_err());
    }
}
