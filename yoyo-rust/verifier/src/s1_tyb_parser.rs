//! S1.1.a — Minimal .tyb parser / validator for in-DLL codegen route.
//!
//! Standalone module: does not touch `pe_dll_link.rs` or any existing emit
//! path. Provides a Rust-side reference implementation of TYB parsing that
//! S1 later ports to in-DLL x86 machine code.
//!
//! Purpose: give S1 a small, testable first node. The full S1 goal is
//! replacing the 8-entry oracle lookup in `yoyo_runtime_selfhost_main`
//! with a real IR → x86 codegen. That's 1500-2700 lines across S1.1-S1.6.
//! This module is S1.1.a only (~100 lines): parse header + count records +
//! sanity-check lengths. No codegen yet.

use std::path::Path;

/// `TYB\0` magic at bytes 0-3.
pub const TYB_MAGIC: [u8; 4] = *b"TYB\0";

/// Records are 8 bytes: op(1) argc(1) args(6).
pub const RECORD_SIZE: usize = 8;

/// Header is 8 bytes: magic(4) + reserved(2) + rec_cnt(2, LE).
pub const HEADER_SIZE: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    TooShort(usize),
    BadMagic,
    Truncated {
        expected: usize,
        got: usize,
    },
    BadArgc {
        record: usize,
        argc: u8,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::TooShort(n) => write!(f, ".tyb too short: {} bytes", n),
            ParseError::BadMagic => write!(f, "bad .tyb magic: expected TYB\\0"),
            ParseError::Truncated { expected, got } => write!(
                f,
                ".tyb truncated: need {} bytes for records, got {}",
                expected, got
            ),
            ParseError::BadArgc { record, argc } => write!(
                f,
                "record {}: bad argc={}",
                record, argc
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TybRecord {
    pub op: u8,
    pub argc: u8,
    /// 6-byte arg slot raw (little-endian), zero-padded for argc < 3.
    pub args_raw: [u8; 6],
}

#[derive(Debug, Clone)]
pub struct TybProgram {
    pub rec_cnt: usize,
    pub records: Vec<TybRecord>,
}

/// Parse a `.tyb` binary. Mirrors `verifier::tyb_parser::parse_tyb` semantics
/// but standalone — no dependency on SourceLine / Arg / parse pipeline.
///
/// Only structural validation: magic, record count, per-record argc ∈ [0,3].
/// No semantic validation (label resolution, branch targets, etc).
pub fn parse_tyb(data: &[u8]) -> Result<TybProgram, ParseError> {
    if data.len() < HEADER_SIZE {
        return Err(ParseError::TooShort(data.len()));
    }
    if &data[0..4] != &TYB_MAGIC {
        return Err(ParseError::BadMagic);
    }
    let rec_cnt = u16::from_le_bytes([data[6], data[7]]) as usize;
    let expected_min = HEADER_SIZE + rec_cnt * RECORD_SIZE;
    if data.len() < expected_min {
        return Err(ParseError::Truncated {
            expected: expected_min,
            got: data.len(),
        });
    }
    let mut records = Vec::with_capacity(rec_cnt);
    for i in 0..rec_cnt {
        let off = HEADER_SIZE + i * RECORD_SIZE;
        let op = data[off];
        let argc = data[off + 1];
        if argc > 3 {
            return Err(ParseError::BadArgc {
                record: i,
                argc,
            });
        }
        let mut args_raw = [0u8; 6];
        args_raw.copy_from_slice(&data[off + 2..off + 8]);
        records.push(TybRecord {
            op,
            argc,
            args_raw,
        });
    }
    Ok(TybProgram { rec_cnt, records })
}

/// Detect .tyb by magic.
pub fn is_tyb(data: &[u8]) -> bool {
    data.len() >= 4 && &data[0..4] == &TYB_MAGIC
}

/// S1.1.b — Minimal IR → x86 codegen.
///
/// Emits one x86 NOP (0x90) per TYB record, then a final `ret` (0xC3).
/// NOT real codegen (does not map opcode semantics); proves the pipeline
/// structure: TybProgram → Vec<u8>.
///
/// What this DOES:
/// - Output length = rec_cnt + 1 (deterministic, testable)
/// - Every input record produces exactly 1 byte of output
/// - Terminates with `ret`
///
/// What this DOES NOT:
/// - Map H_00 opcode semantics to correct x86 instructions
/// - Emit `mov eax, imm` / load / store / branch / call
/// - Handle labels, branches, or multi-instruction sequences
///
/// Those come in S1.1.c (op dispatch), S1.2 (register allocation), S1.3
/// (state + immediate codegen), S1.4 (PE wrapping), S1.5 (DLL entry rewrite).
/// S1.1.c — opcode dispatch with real x86 output.
///
/// Replaces S1.1.b's stub (all NOPs + RET). Currently handles:
/// - `0x50` with argc==1 and imm32 <= 0xFFFFFFFF: `mov rax, imm32; mov [r15], rax`
///   (approximation: writes to slot 0 via r15-relative disp8)
/// - All other opcodes: x86 NOP (0x90) placeholder
/// - Always appends `ret` (0xC3) at the end
///
/// Still STUBS:
/// - State slot semantics (currently writes to r15+0 = slot 0 for all movs)
/// - All arithmetic (add/sub/or/cmp/inc/dec)
/// - Loads (ldb/get/movrr)
/// - Branches and labels (require two-pass label resolution)
/// - Calls and ret with arg passing
///
/// S1.1.d / S1.2 tackle these. This step proves:
/// 1. Opcode dispatch structure works (match on op)
/// 2. Real x86 bytes are emitted for at least one op
/// 3. Tests validate byte-for-byte against expected hex
///
/// Reference: `verifier/src/assembler.rs` for the real emit functions
/// (S1.3 will replace this hand-written code with assembler.rs calls
/// once we have assembler.rs available in-DLL).
pub struct CodegenOutput {
    pub text: Vec<u8>,
}

/// Emit x86 for a TYB program.
///
/// Confirmed opcodes (from `tyb_parser.rs:159-162` tests + source):
/// - `0x30` SET: slot(u16) imm(u32)
/// - `0x40` LABEL: label_id(u16)
/// - `0xFF` RET: no args
///
/// S1.1.c handles 0x30 (SET) and 0xFF (RET) with correct x86 encoding.
/// S1.1.d will add GET/ADD/SUB/MOV/CMP/branches.
///
/// Register convention (tentative, may change in S1.2):
/// - r15 = state base pointer (already set by H_00 stub prologue)
/// - rax = scratch / immediate holder
/// - rcx = second operand scratch
pub fn emit_x86(prog: &TybProgram) -> CodegenOutput {
    let mut text = Vec::with_capacity(prog.rec_cnt * 10 + 1);
    for rec in &prog.records {
        match rec.op {
            // SET slot, imm  →  mov rax, imm32; mov [r15+slot*8], rax
            // args_raw layout for argc==2: [slot_lo, slot_hi, imm_0..3]
            0x30 if rec.argc == 2 => {
                let slot = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]) as u32;
                let imm = u32::from_le_bytes([
                    rec.args_raw[2],
                    rec.args_raw[3],
                    rec.args_raw[4],
                    rec.args_raw[5],
                ]) as u64;
                // mov rax, imm32  (REX.W + B8, imm32) = 10 bytes
                text.extend_from_slice(&[0x48, 0xB8]);
                text.extend_from_slice(&imm.to_le_bytes());
                // mov [r15 + disp8], rax  (REX.WB + 89, mod=01 reg=0 r/m=7) = 4 bytes
                // slot*8 must fit disp8 (≤ 127) → slot ≤ 15
                if slot * 8 <= 127 {
                    text.extend_from_slice(&[0x4C, 0x89, 0x47, (slot * 8) as u8]);
                } else {
                    // disp32 variant (REX.WB + 89, mod=10, imm32)
                    let disp = (slot * 8) as i32;
                    text.extend_from_slice(&[0x4C, 0x89, 0x87]);
                    text.extend_from_slice(&disp.to_le_bytes());
                }
            }
            // RET: emit x86 ret (1 byte 0xC3). Skip trailing implicit ret if
            // this is the last instruction; we still always emit a final ret
            // for safety (see below).
            0xFF if rec.argc == 0 => {
                text.push(0xC3);
            }
            // LABEL (0x40): emit nothing — labels are resolved at link time
            // (two-pass), which is S1.2 / S1.3 scope.
            0x40 => {
                // no x86 emitted
            }
            _ => {
                // Placeholder for unhandled opcodes: 1 NOP
                text.push(0x90);
            }
        }
    }
    // Always terminate with ret if we didn't end on one (safety net).
    if text.is_empty() || *text.last().unwrap() != 0xC3 {
        text.push(0xC3);
    }
    CodegenOutput { text }
}

/// Parse a .tyb file from disk (used by S1 gates / tests).
pub fn parse_tyb_file(path: &Path) -> Result<TybProgram, ParseError> {
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(_) => return Err(ParseError::BadMagic), // simplified; caller should handle IO
    };
    parse_tyb(&data)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(rec_cnt: u16) -> Vec<u8> {
        let mut v = Vec::new();
        v.extend_from_slice(&TYB_MAGIC);
        v.extend_from_slice(&[0u8, 0u8]); // reserved
        v.extend_from_slice(&rec_cnt.to_le_bytes());
        v
    }

    fn record(op: u8, argc: u8, a: [u8; 6]) -> [u8; 8] {
        let mut r = [0u8; 8];
        r[0] = op;
        r[1] = argc;
        r[2..].copy_from_slice(&a);
        r
    }

    #[test]
    fn empty_tyb() {
        let data = header(0);
        let p = parse_tyb(&data).unwrap();
        assert_eq!(p.rec_cnt, 0);
        assert!(p.records.is_empty());
    }

    #[test]
    fn bad_magic() {
        let data = [0x58, 0x59, 0x5A, 0x00, 0, 0, 0, 0];
        assert!(matches!(parse_tyb(&data), Err(ParseError::BadMagic)));
    }

    #[test]
    fn too_short() {
        assert!(matches!(parse_tyb(b"TYB\0"), Err(ParseError::TooShort(4))));
    }

    #[test]
    fn truncated() {
        let mut data = header(3);
        data.extend_from_slice(&record(0x50, 0, [0, 0, 0, 0, 0, 0]));
        // only 1 record but header claims 3
        assert!(matches!(
            parse_tyb(&data),
            Err(ParseError::Truncated { expected, got }) if expected == 32 && got == 16
        ));
    }

    #[test]
    fn one_zero_argc_record() {
        let mut data = header(1);
        data.extend_from_slice(&record(0x50, 0, [0, 0, 0, 0, 0, 0]));
        let p = parse_tyb(&data).unwrap();
        assert_eq!(p.rec_cnt, 1);
        assert_eq!(p.records[0].op, 0x50);
        assert_eq!(p.records[0].argc, 0);
    }

    #[test]
    fn bad_argc() {
        let mut data = header(1);
        data.extend_from_slice(&record(0x50, 4, [0, 0, 0, 0, 0, 0]));
        assert!(matches!(
            parse_tyb(&data),
            Err(ParseError::BadArgc { record: 0, argc: 4 })
        ));
    }

    #[test]
    fn real_yoyo_tyb_parses() {
        // Parse the actual seed source used in gate scripts.
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../yoyo/projects/yoyo.tyb");
        if !path.exists() {
            eprintln!("skip: {} not present", path.display());
            return;
        }
        let p = parse_tyb_file(&path).unwrap();
        assert!(p.rec_cnt > 0, "yoyo.tyb should have records");
        assert!(p.records.iter().all(|r| r.argc <= 3), "argc ∈ [0,3]");
    }

    // --- S1.1.b: emit_x86 codegen ---

    #[test]
    fn emit_x86_empty_program_emits_ret() {
        let prog = TybProgram { rec_cnt: 0, records: Vec::new() };
        let out = emit_x86(&prog);
        assert_eq!(out.text, vec![0xC3]);
    }

    #[test]
    fn emit_x86_set_slot0_imm0() {
        // SET slot=0x0000, imm=0x00000000
        // emit = movabs rax, imm64 (10B) + store_state slot=0 (4B) + ret (1B) = 15B
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x30, argc: 2, args_raw: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        // movabs rax, 0 → 0x48 0xB8 + 8 zero bytes
        // store_state(0, rax) → REX.WB 0x4C, 0x89, MODRM=0x47, disp8=0x00
        // ret → 0xC3
        assert_eq!(
            out.text,
            vec![
                0x48, 0xB8, 0, 0, 0, 0, 0, 0, 0, 0,  // movabs rax, 0
                0x4C, 0x89, 0x47, 0x00,                // mov [r15+0], rax
                0xC3,                                  // ret
            ]
        );
    }

    #[test]
    fn emit_x86_set_slot8_imm_deadbeef() {
        // SET slot=0x0008, imm=0xdeadbeef
        // slot=8 → disp8 = 8*8 = 64 = 0x40
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x30, argc: 2, args_raw: [0x08, 0x00, 0xEF, 0xBE, 0xAD, 0xDE] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(
            out.text,
            vec![
                0x48, 0xB8,                        // movabs rax, imm64
                0xEF, 0xBE, 0xAD, 0xDE, 0, 0, 0, 0, // 0xdeadbeef sign-extended to 8B
                0x4C, 0x89, 0x47, 0x40,             // mov [r15+64], rax
                0xC3,                               // ret
            ]
        );
    }

    #[test]
    fn emit_x86_ret_opcode_0xff() {
        // RET alone → 0xC3 (single byte, no implicit trailing ret since we end on one)
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0xFF, argc: 0, args_raw: [0; 6] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text, vec![0xC3]);
    }

    #[test]
    fn emit_x86_label_is_noop() {
        // LABEL 0x1234 → no x86 emitted, just trailing ret
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x40, argc: 1, args_raw: [0x34, 0x12, 0, 0, 0, 0] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text, vec![0xC3]);
    }

    #[test]
    fn emit_x86_unknown_op_is_nop() {
        // Unknown opcode 0x50 → 1 NOP + trailing ret
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x50, argc: 0, args_raw: [0; 6] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text, vec![0x90, 0xC3]);
    }

    #[test]
    fn emit_x86_sequence_set_then_ret() {
        // SET slot=0, imm=0x01000000 ; RET
        // argc==2 layout: [slot_lo, slot_hi, imm_0, imm_1, imm_2, imm_3]
        // imm = 0x01000000 LE = [0x00, 0x00, 0x00, 0x01]
        // Total: 10 (movabs) + 4 (store) + 1 (ret) = 15 bytes
        let prog = TybProgram {
            rec_cnt: 2,
            records: vec![
                TybRecord { op: 0x30, argc: 2, args_raw: [0x00, 0x00, 0x00, 0x00, 0x00, 0x01] },
                TybRecord { op: 0xFF, argc: 0, args_raw: [0; 6] },
            ],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 15);
        // bytes 0-9: movabs rax, 0x01000000
        assert_eq!(out.text[0], 0x48);
        assert_eq!(out.text[1], 0xB8);
        assert_eq!(&out.text[2..10], &[0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]);
        // bytes 10-13: store_state(0, rax)
        assert_eq!(&out.text[10..14], &[0x4C, 0x89, 0x47, 0x00]);
        // byte 14: ret
        assert_eq!(out.text[14], 0xC3);
    }

    #[test]
    fn emit_x86_real_yoyo_tyb() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../yoyo/projects/yoyo.tyb");
        if !path.exists() {
            eprintln!("skip: {} not present", path.display());
            return;
        }
        let p = parse_tyb_file(&path).unwrap();
        let out = emit_x86(&p);
        assert!(!out.text.is_empty(), "should emit at least ret");
        assert_eq!(*out.text.last().unwrap(), 0xC3, "must end with ret");
    }
}
