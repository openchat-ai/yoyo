//! Three-column render: SOURCE / TIR / X86 (PROMPT-v3 Part 4.5).

use crate::assembler;
use crate::tir::{instr_name, TirInst};

pub fn render_one(inst: &TirInst, x86: &[u8]) -> String {
    let args: Vec<String> = inst.args.iter().map(|a| format!("{a:X}")).collect();
    let src = format!("{:02X} {}", opcode_byte(inst), args.join(" "));
    let tir = format!("{} {}", instr_name(inst.op), args.join(","));
    let x86_hex: Vec<String> = x86.iter().map(|b| format!("{b:02X}")).collect();
    format!("{:<24} | {:<28} | {}", src, tir, x86_hex.join(" "))
}

fn opcode_byte(inst: &TirInst) -> u8 {
    crate::tir::opcode_to_u8(inst.op)
}

pub fn disasm_bytes(bytes: &[u8]) -> String {
    // Minimal hex dump disasm for audit
    let mut out = String::new();
    for (i, chunk) in bytes.chunks(16).enumerate() {
        let hex: Vec<String> = chunk.iter().map(|b| format!("{b:02X}")).collect();
        out.push_str(&format!("{:04X}: {}\n", i * 16, hex.join(" ")));
    }
    out
}

/// Self-check known primitive encodings.
pub fn primitive_smoke() -> bool {
    assembler::ret() == vec![0xC3]
        && assembler::movabs(crate::types::Reg::Rax, 0).unwrap().len() == 10
}
