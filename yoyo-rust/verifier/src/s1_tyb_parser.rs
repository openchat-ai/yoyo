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
pub struct CodegenOutput {
    pub text: Vec<u8>,
}

pub fn emit_x86(prog: &TybProgram) -> CodegenOutput {
    let mut text = Vec::with_capacity(prog.rec_cnt + 1);
    for _ in &prog.records {
        // S1.1.b stub: every record → 1 byte x86 NOP.
        // Real op dispatch: S1.1.c.
        text.push(0x90);
    }
    text.push(0xC3); // ret
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
    fn emit_x86_one_record_one_nop_plus_ret() {
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x30, argc: 0, args_raw: [0; 6] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text, vec![0x90, 0xC3]);
    }

    #[test]
    fn emit_x86_length_equals_records_plus_one() {
        let prog = TybProgram {
            rec_cnt: 50,
            records: (0..50).map(|i| TybRecord {
                op: 0x50,
                argc: 0,
                args_raw: [i as u8, 0, 0, 0, 0, 0],
            }).collect(),
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 51);
        assert_eq!(&out.text[..49], &vec![0x90u8; 49][..]);
        // Actually all 50 NOPS then RET
        for i in 0..50 {
            assert_eq!(out.text[i], 0x90, "byte {i} should be NOP");
        }
        assert_eq!(out.text[50], 0xC3);
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
        assert_eq!(out.text.len(), p.rec_cnt + 1, "length = rec_cnt + 1");
        assert_eq!(*out.text.last().unwrap(), 0xC3, "last byte is ret");
        for b in &out.text[..p.rec_cnt] {
            assert_eq!(*b, 0x90, "each record emits 1 byte NOP");
        }
    }
}
