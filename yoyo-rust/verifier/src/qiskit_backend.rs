//! Qiskit/OpenQASM backend — maps YOYO TIR operations to OpenQASM 3.0 source.
//!
//! Produces a `.qasm` text file describing the quantum circuit equivalent
//! of the TIR program. Since TIR is classical, the quantum circuit is a
//! stub — each instruction emits a comment describing the operation.

use crate::tir::{instr_branch_kind, BranchKind, TirInst, TirOp};
use crate::types::IsaResult;

/// Emit OpenQASM 3.0 text from TIR instructions.
pub fn emit_qiskit(tir: &[TirInst]) -> IsaResult<String> {
    let mut qasm = String::new();

    // OpenQASM 3.0 header
    qasm.push_str("OPENQASM 3.0;\n");
    qasm.push_str("include \"stdgates.inc\";\n\n");

    // Declare a single qubit (stub)
    qasm.push_str("qubit[1] q;\n");
    // Declare classical register for state
    qasm.push_str("bit[1] c;\n\n");

    // Build label map from handler definitions
    let mut label_map: std::collections::HashMap<u16, String> = std::collections::HashMap::new();
    for inst in tir {
        if let TirOp::Handler = inst.op {
            let hh = inst.args.first().copied().unwrap_or(0) as u16;
            label_map.insert(hh, format!("H_{:02X}", hh));
        }
    }

    // Emit instructions for each handler
    let mut in_handler = false;

    for inst in tir {
        match instr_branch_kind(inst.op) {
            BranchKind::LabelDef => {
                if in_handler {
                    qasm.push_str("// end handler\n");
                }
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let label = label_map.get(&hh).cloned().unwrap_or_else(|| format!("H_{:02X}", hh));
                qasm.push_str(&format!("\n// -- {} --\n", label));
                in_handler = true;
            }
            BranchKind::Ret => {
                // ret emitted when we close the handler (next HANDLER or end)
            }
            BranchKind::Call => {
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let target = label_map.get(&hh).cloned().unwrap_or_else(|| format!("H_{:02X}", hh));
                qasm.push_str(&format!("// CALL {}\n", target));
            }
            BranchKind::Jmp => {
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let target = label_map.get(&hh).cloned().unwrap_or_else(|| format!("H_{:02X}", hh));
                qasm.push_str(&format!("// JMP {}\n", target));
            }
            BranchKind::Jcc { index: _ } => {
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                let target = label_map.get(&hh).cloned().unwrap_or_else(|| format!("H_{:02X}", hh));
                qasm.push_str(&format!("// JCC {}\n", target));
            }
            BranchKind::None => {
                emit_qiskit_inst(&mut qasm, inst)?;
            }
        }
    }

    if in_handler {
        qasm.push_str("// end handler\n");
    }

    qasm.push_str("\n// end\n");
    Ok(qasm)
}

/// Emit a single non-branch TIR instruction as an OpenQASM comment.
fn emit_qiskit_inst(qasm: &mut String, inst: &TirInst) -> IsaResult<()> {
    let a = |i: usize| inst.args.get(i).copied().unwrap_or(0);

    match inst.op {
        TirOp::Nop => {
            qasm.push_str("// nop\n");
        }
        TirOp::Data | TirOp::Str | TirOp::Raw => {
            // Data section — emit as comment
        }
        TirOp::Alloc => {
            let slot = a(0) as u16;
            let size = a(1);
            qasm.push_str(&format!("// ALLOC state[{}] = malloc({})\n", slot, size));
        }
        TirOp::Set => {
            let slot = a(0) as u16;
            let imm = a(1);
            qasm.push_str(&format!("// SET state[{}] = 0x{:x}\n", slot, imm));
        }
        TirOp::Get | TirOp::Movrr => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            qasm.push_str(&format!("// GET state[{}] = state[{}]\n", dst, src));
        }
        TirOp::Add => {
            let slot = a(0) as u16;
            let imm = a(1);
            qasm.push_str(&format!("// ADD state[{}] += {}\n", slot, imm));
        }
        TirOp::Sub => {
            let slot = a(0) as u16;
            let imm = a(1);
            qasm.push_str(&format!("// SUB state[{}] -= {}\n", slot, imm));
        }
        TirOp::Inc => {
            let slot = a(0) as u16;
            qasm.push_str(&format!("// INC state[{}]++\n", slot));
        }
        TirOp::Dec => {
            let slot = a(0) as u16;
            qasm.push_str(&format!("// DEC state[{}]--\n", slot));
        }
        TirOp::Addv => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            qasm.push_str(&format!("// ADDV state[{}] += state[{}]\n", dst, src));
        }
        TirOp::Orv => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            qasm.push_str(&format!("// ORV state[{}] |= state[{}]\n", dst, src));
        }
        TirOp::Subv => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            qasm.push_str(&format!("// SUBV state[{}] -= state[{}]\n", dst, src));
        }
        TirOp::Imul => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            qasm.push_str(&format!("// IMUL state[{}] *= state[{}]\n", dst, src));
        }
        TirOp::Cmp => {
            let a_slot = a(0) as u16;
            let b_slot = a(1) as u16;
            qasm.push_str(&format!("// CMP state[{}] == state[{}]\n", a_slot, b_slot));
        }
        TirOp::Ldb => {
            let dd = a(0) as u16;
            let ss = a(1) as u16;
            let oo = a(2) as u16;
            qasm.push_str(&format!("// LDB state[{}] = *(u8*)(state[{}] + {})\n", dd, ss, oo));
        }
        TirOp::LoadFile => {
            let slot = a(0) as u16;
            let str_idx = a(1);
            qasm.push_str(&format!("// LOAD_FILE state[{}] = fopen(str{})\n", slot, str_idx));
        }
        TirOp::WriteFile => {
            let slot = a(0) as u16;
            let str_idx = a(1);
            qasm.push_str(&format!("// WRITE_FILE state[{}] -> str{}\n", slot, str_idx));
        }
        TirOp::MemcpyData => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            let n = a(2) as u16;
            qasm.push_str(&format!("// MEMCPY_DATA memcpy(data+{}, state[{}], {})\n", dst, src, n));
        }
        TirOp::MemcpyState => {
            let dst = a(0) as u16;
            let src = a(1) as u16;
            let n = a(2) as u16;
            qasm.push_str(&format!("// MEMCPY_STATE memcpy(state+{}, state+{}, {})\n", dst, src, n));
        }
        TirOp::Handler => {
            // handled in branch dispatch
        }
        _ => {
            let code = a(0) as u8;
            qasm.push_str(&format!("// EXIT code={}\n", code));
            qasm.push_str(&format!("// unknown op {:?}\n", inst.op));
        }
    }
    Ok(())
}