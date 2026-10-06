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

/// Extract the numeric arguments from a record's `args_raw` given its `argc`.
///
/// Layout in TYB (per `tyb_parser.rs:70-90`):
/// - argc=0: no args
/// - argc=1: u32 LE in bytes [2..6]
/// - argc=2: u16 LE in [2..4] + u32 LE in [4..8]
/// - argc=3: u16 LE in [2..4], [4..6], [6..8]
fn args_of(rec: &TybRecord) -> Vec<u64> {
    match rec.argc {
        0 => Vec::new(),
        1 => vec![u32::from_le_bytes([
            rec.args_raw[0], rec.args_raw[1], rec.args_raw[2], rec.args_raw[3],
        ]) as u64],
        2 => vec![
            u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]) as u64,
            u32::from_le_bytes([
                rec.args_raw[2], rec.args_raw[3], rec.args_raw[4], rec.args_raw[5],
            ]) as u64,
        ],
        3 => vec![
            u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]) as u64,
            u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]) as u64,
            u16::from_le_bytes([rec.args_raw[4], rec.args_raw[5]]) as u64,
        ],
        _ => Vec::new(),
    }
}

/// mov rax, imm32 → REX.W 0xB8 + imm32 = 6 bytes.
/// (Uses 32-bit immediate encoding — sign-extends to rax on x86-64.)
fn emit_mov_rax_imm32(out: &mut Vec<u8>, imm: u32) {
    out.extend_from_slice(&[0x48, 0xB8]);
    out.extend_from_slice(&imm.to_le_bytes());
}

/// mov [r15 + slot*8], rax (r15-relative store).
/// Uses disp8 for slot*8 <= 127 (slot <= 15), disp32 otherwise.
fn emit_store_state_r15(out: &mut Vec<u8>, slot: u16) {
    let disp = (slot as u32) * 8;
    if disp <= 127 {
        out.extend_from_slice(&[0x4C, 0x89, 0x47, disp as u8]); // 4 bytes
    } else {
        out.extend_from_slice(&[0x4C, 0x89, 0x87]); // 3 bytes + disp32
        out.extend_from_slice(&((disp as i32)).to_le_bytes());
    }
}

/// mov rax, [r15 + slot*8] (r15-relative load).
fn emit_load_state_r15(out: &mut Vec<u8>, slot: u16) {
    let disp = (slot as u32) * 8;
    if disp <= 127 {
        out.extend_from_slice(&[0x4C, 0x8B, 0x47, disp as u8]); // 4 bytes (reg=rax)
    } else {
        out.extend_from_slice(&[0x4C, 0x8B, 0x87]); // 3 bytes + disp32
        out.extend_from_slice(&((disp as i32)).to_le_bytes());
    }
}

/// mov rcx, [r15 + slot*8] (r15-relative load into rcx).
///
/// Same pattern as `emit_load_state_r15` but writes to rcx instead of rax.
/// Required so that operations like `add rax, rcx` don't lose the rax operand.
/// MODRM differs only in the `reg` field: rax=000 (`47`/`87`), rcx=001 (`48`/`88`).
fn emit_load_state_rcx(out: &mut Vec<u8>, slot: u16) {
    let disp = (slot as u32) * 8;
    if disp <= 127 {
        out.extend_from_slice(&[0x4C, 0x8B, 0x48, disp as u8]); // 4 bytes (reg=rcx)
    } else {
        out.extend_from_slice(&[0x4C, 0x8B, 0x88]); // 3 bytes + disp32
        out.extend_from_slice(&((disp as i32)).to_le_bytes());
    }
}

/// add rax, imm32 → 6 bytes (REX.W 0x81 C0 imm32)
fn emit_add_rax_imm32(out: &mut Vec<u8>, imm: u32) {
    out.extend_from_slice(&[0x48, 0x81, 0xC0]);
    out.extend_from_slice(&imm.to_le_bytes());
}

/// sub rax, imm32 → 6 bytes (REX.W 0x81 E0 imm32)
fn emit_sub_rax_imm32(out: &mut Vec<u8>, imm: u32) {
    out.extend_from_slice(&[0x48, 0x81, 0xE0]);
    out.extend_from_slice(&imm.to_le_bytes());
}

/// inc rax → 3 bytes (REX.W 0xFF /0)
fn emit_inc_rax(out: &mut Vec<u8>) {
    out.extend_from_slice(&[0x48, 0xFF, 0xC0]);
}

/// dec rax → 3 bytes (REX.W 0xFF /1)
fn emit_dec_rax(out: &mut Vec<u8>) {
    out.extend_from_slice(&[0x48, 0xFF, 0xC1]);
}

/// add rax, rcx → 3 bytes (REX.W 0x01 /r)
fn emit_add_rax_rcx(out: &mut Vec<u8>) {
    out.extend_from_slice(&[0x48, 0x01, 0xC8]); // rax = 0, rcx = 1 → modrm=0xC8
}

/// or rax, rcx → 3 bytes (REX.W 0x09 /r)
fn emit_or_rax_rcx(out: &mut Vec<u8>) {
    out.extend_from_slice(&[0x48, 0x09, 0xC8]);
}

/// sub rax, rcx → 3 bytes (REX.W 0x29 /r)
fn emit_sub_rax_rcx(out: &mut Vec<u8>) {
    out.extend_from_slice(&[0x48, 0x29, 0xC8]);
}

/// imul rax, rcx → 4 bytes (REX.W 0F AF /r)
fn emit_imul_rax_rcx(out: &mut Vec<u8>) {
    out.extend_from_slice(&[0x48, 0x0F, 0xAF, 0xC8]);
}

/// cmp rcx, rax → 3 bytes (REX.W 0x39 /r). Semantics: compare rax against rcx
/// (assembler.rs cmp_reg(a,b) emits `cmp rax, rcx` per its encoding).
fn emit_cmp_rax_rcx(out: &mut Vec<u8>) {
    out.extend_from_slice(&[0x48, 0x39, 0xC0]);
}

/// Emit x86 for a TYB program.
///
/// Covers (from `isa_table.txt`):
/// - `0x30` SET slot imm (argc=2): movabs rax imm32 + store_state slot
/// - `0x60` GET dst src (argc=2): load_state src rax + store_state dst rax
/// - `0x64` MOVRR dst src (argc=2): same as GET
/// - `0x61` SUB slot imm (argc=2): load_state slot rax; sub rax imm; store_state slot rax
/// - `0x62` ADD slot imm (argc=2): load_state slot rax; add rax imm; store_state slot rax
/// - `0x66` INC slot (argc=1): load_state slot rax; inc rax; store_state slot rax
/// - `0x67` DEC slot (argc=1): load_state slot rax; dec rax; store_state slot rax
/// - `0x63` IMUL dst src (argc=2): load dst rax; load src rcx; imul rax rcx; store dst rax
/// - `0x68` ADDV dst src (argc=2): load dst rax; load src rcx; add rax rcx; store dst rax
/// - `0x69` ORV dst src (argc=2): load dst rax; load src rcx; or rax rcx; store dst rax
/// - `0x6A` SUBV dst src (argc=2): load dst rax; load src rcx; sub rax rcx; store dst rax
/// - `0x65` CMP a b (argc=2): load a rax; load b rcx; cmp rax rcx
/// - `0xFF` RET (argc=0): ret
///
/// Still STUBS (S1.1.e / S1.2 / S1.3):
/// - `0x40` LABEL / `0x41` CALL / `0x70-0x7A` branches — need two-pass label resolution
/// - `0x80` LDB dd ss oo — memory addressing beyond state slots
/// - `0x84` / `0x85` MEMCPY_* — multi-byte data movement
/// - `0x20` ALLOC / `0x50` LOAD_FILE / `0x51` WRITE_FILE — platform primitives
/// - `0x10` / `0x12` / `0x13` / `0xA0` / `0xA1` — data/string/raw (S1.4 PE wrapping scope)
///
/// Register convention:
/// - r15 = state base pointer
/// - rax = primary scratch
/// - rcx = secondary scratch (for reg-reg ops)
pub fn emit_x86(prog: &TybProgram) -> CodegenOutput {
    let mut text = Vec::with_capacity(prog.rec_cnt * 20 + 1);
    for rec in &prog.records {
        let args = args_of(rec);
        match rec.op {
            // --- SET slot, imm ---
            0x30 if args.len() == 2 => {
                let slot = args[0] as u16;
                let imm = args[1] as u32;
                emit_mov_rax_imm32(&mut text, imm);
                emit_store_state_r15(&mut text, slot);
            }
            // --- GET / MOVRR dst, src ---
            0x60 | 0x64 if args.len() == 2 => {
                let dst = args[0] as u16;
                let src = args[1] as u16;
                emit_load_state_r15(&mut text, src);
                emit_store_state_r15(&mut text, dst);
            }
            // --- SUB / ADD slot, imm ---
            0x61 | 0x62 if args.len() == 2 => {
                let slot = args[0] as u16;
                let imm = args[1] as u32;
                emit_load_state_r15(&mut text, slot);
                if rec.op == 0x61 {
                    emit_sub_rax_imm32(&mut text, imm);
                } else {
                    emit_add_rax_imm32(&mut text, imm);
                }
                emit_store_state_r15(&mut text, slot);
            }
            // --- INC / DEC slot ---
            0x66 | 0x67 if args.len() == 1 => {
                let slot = args[0] as u16;
                emit_load_state_r15(&mut text, slot);
                if rec.op == 0x66 {
                    emit_inc_rax(&mut text);
                } else {
                    emit_dec_rax(&mut text);
                }
                emit_store_state_r15(&mut text, slot);
            }
            // --- IMUL dst, src ---
            // imul rax, rcx → rax = rax * rcx.
            // Load dst (rax) FIRST, then src (rcx) — preserves rax.
            0x63 if args.len() == 2 => {
                let dst = args[0] as u16;
                let src = args[1] as u16;
                emit_load_state_r15(&mut text, dst);
                emit_load_state_rcx(&mut text, src);
                emit_imul_rax_rcx(&mut text);
                emit_store_state_r15(&mut text, dst);
            }
            // --- ADDV / ORV / SUBV dst, src ---
            // add rax, rcx / or rax, rcx / sub rax, rcx
            // Load dst (rax) FIRST, then src (rcx).
            0x68 | 0x69 | 0x6A if args.len() == 2 => {
                let dst = args[0] as u16;
                let src = args[1] as u16;
                emit_load_state_r15(&mut text, dst);
                emit_load_state_rcx(&mut text, src);
                if rec.op == 0x68 {
                    emit_add_rax_rcx(&mut text);
                } else if rec.op == 0x69 {
                    emit_or_rax_rcx(&mut text);
                } else {
                    emit_sub_rax_rcx(&mut text);
                }
                emit_store_state_r15(&mut text, dst);
            }
            // --- CMP a, b ---
            // cmp rax, rcx → a = a - b (flags), no write-back.
            0x65 if args.len() == 2 => {
                let a = args[0] as u16;
                let b = args[1] as u16;
                emit_load_state_r15(&mut text, a);
                emit_load_state_rcx(&mut text, b);
                emit_cmp_rax_rcx(&mut text);
            }
            // --- RET ---
            0xFF if rec.argc == 0 => {
                text.push(0xC3);
            }
            // --- LABEL (0x40): two-pass label resolution in S1.1.e ---
            0x40 => {}
            // --- Everything else: NOP placeholder ---
            _ => {
                text.push(0x90);
            }
        }
    }
    // Always terminate with ret if we didn't end on one.
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
        // SET slot=0, imm=0 → mov rax, imm32 (6B) + mov [r15], rax (4B) + ret (1B) = 11B
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x30, argc: 2, args_raw: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(
            out.text,
            vec![
                0x48, 0xB8, 0, 0, 0, 0,          // mov rax, 0 (imm32)
                0x4C, 0x89, 0x47, 0x00,             // mov [r15+0], rax
                0xC3,                                 // ret
            ]
        );
    }

    #[test]
    fn emit_x86_set_slot8_imm_deadbeef() {
        // SET slot=8, imm=0xdeadbeef → slot*8 = 0x40
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x30, argc: 2, args_raw: [0x08, 0x00, 0xEF, 0xBE, 0xAD, 0xDE] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(
            out.text,
            vec![
                0x48, 0xB8,                        // mov rax, imm32
                0xEF, 0xBE, 0xAD, 0xDE,             // 0xdeadbeef LE
                0x4C, 0x89, 0x47, 0x40,             // mov [r15+64], rax
                0xC3,                                 // ret
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
        // Total: 6 (mov rax, imm32) + 4 (store) + 1 (ret) = 11 bytes
        let prog = TybProgram {
            rec_cnt: 2,
            records: vec![
                TybRecord { op: 0x30, argc: 2, args_raw: [0x00, 0x00, 0x00, 0x00, 0x00, 0x01] },
                TybRecord { op: 0xFF, argc: 0, args_raw: [0; 6] },
            ],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 11);
        // bytes 0-5: mov rax, imm32=0x01000000
        assert_eq!(out.text[0], 0x48);
        assert_eq!(out.text[1], 0xB8);
        assert_eq!(&out.text[2..6], &[0x00, 0x00, 0x00, 0x01]);
        // bytes 6-9: store_state(0, rax)
        assert_eq!(&out.text[6..10], &[0x4C, 0x89, 0x47, 0x00]);
        // byte 10: ret
        assert_eq!(out.text[10], 0xC3);
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

    // S1.1.d smoke tests: each newly-dispatched opcode emits a non-empty
    // sequence ending with the correct tail, so we can catch gross encoding
    // errors before S1.1.e refines them.

    #[test]
    fn emit_x86_get_slot0_from_slot1() {
        // GET dst=0, src=1:
        //   load_state r15+0*8 -> rax   (4B: 4C 8B 47 00)
        //   load_state r15+1*8 -> rax   (4B: 4C 8B 47 08)  ← BUG: clobbers rax
        //   store_state r15+0 = rax     (4B: 4C 89 47 00)
        //   ret                          (1B: C3)
        // Total = 9B (4 + 4 + 1). Known bug: second load clobbers rax.
        // S1.1.e will fix by adding load_state_rcx.
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x60, argc: 2, args_raw: [0x00, 0x00, 0x01, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 9);
        assert_eq!(*out.text.last().unwrap(), 0xC3);
    }

    #[test]
    fn emit_x86_add_imm_slot0_add_5() {
        // ADD slot=0, imm=5 → load r15+0 to rax; add rax 5; store r15+0
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x62, argc: 2, args_raw: [0x00, 0x00, 0x05, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        assert!(out.text.len() >= 12, "load(4) + add(6) + store(4) + ret(1) = 15");
        assert_eq!(*out.text.last().unwrap(), 0xC3);
    }

    #[test]
    fn emit_x86_inc_slot0() {
        // INC slot=0 → load rax from r15+0; inc rax; store back = 4+3+4 = 11B + ret
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x66, argc: 1, args_raw: [0x00, 0x00, 0, 0, 0, 0] }],
        };
        let out = emit_x86(&prog);
        assert!(out.text.len() >= 12, "load(4) + inc(3) + store(4) + ret(1) = 12");
        assert_eq!(*out.text.last().unwrap(), 0xC3);
    }

    #[test]
    fn emit_x86_movrr_slot1_from_slot0() {
        // MOVRR dst=1, src=0 → load rax from r15+0; store to r15+8
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x64, argc: 2, args_raw: [0x01, 0x00, 0x00, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        // load r15+0 (4B) + store r15+8 (4B) + ret (1B) = 9
        assert_eq!(out.text.len(), 9);
        // Verify store uses disp8=8 (offset 0x08)
        assert_eq!(&out.text[4..8], &[0x4C, 0x89, 0x47, 0x08]);
        assert_eq!(out.text[8], 0xC3);
    }

    // S1.1.e tests: real byte verification for ADDV/ORV/SUBV/IMUL/CMP
    // using load_state_rcx (reg=001 → MODRM 0x48/0x88) instead of a second
    // load_state_r15 (reg=000 → MODRM 0x47/0x87).
    //
    // args_raw layout for argc=2: [slot_lo, slot_hi, u32_lo32...]
    // We set args[1] as an imm32 in args_of(), but for reg-reg ops only the
    // low 16 bits are used (cast to u16), so values 0..=0xFFFF work as slots.

    #[test]
    fn emit_x86_addv_slot1_from_slot2() {
        // ADDV dst=1, src=2:
        //   4C 8B 47 08   mov rax, [r15+8]   (dst)
        //   4C 8B 48 10   mov rcx, [r15+16]  (src — uses MODRM 0x48 reg=rcx)
        //   48 01 C8      add rax, rcx       (MODRM 0xC8: reg=rcx, r/m=rax)
        //   4C 89 47 08   mov [r15+8], rax
        //   C3            ret
        // Total = 4+4+3+4+1 = 16
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x68, argc: 2, args_raw: [0x01, 0x00, 0x02, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 16);
        assert_eq!(
            &out.text,
            &[
                0x4C, 0x8B, 0x47, 0x08,   // load_state_r15 slot=1 → rax
                0x4C, 0x8B, 0x48, 0x10,   // load_state_rcx slot=2 → rcx (MODRM 0x48)
                0x48, 0x01, 0xC8,          // add rax, rcx
                0x4C, 0x89, 0x47, 0x08,   // store_state_r15 slot=1 ← rax
                0xC3,                      // ret
            ]
        );
    }

    #[test]
    fn emit_x86_orv_slot0_from_slot1() {
        // ORV dst=0, src=1 → OR 0x09, no store write-back semantics issue
        // bytes: load r15+0 (rax), load r15+8 (rcx), or rax rcx, store r15+0, ret
        //       = 4 + 4 + 3 + 4 + 1 = 16
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x69, argc: 2, args_raw: [0x00, 0x00, 0x01, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 16);
        assert_eq!(
            &out.text,
            &[
                0x4C, 0x8B, 0x47, 0x00,
                0x4C, 0x8B, 0x48, 0x08,   // MODRM 0x48 (rcx), disp=8
                0x48, 0x09, 0xC8,          // or rax, rcx
                0x4C, 0x89, 0x47, 0x00,
                0xC3,
            ]
        );
    }

    #[test]
    fn emit_x86_subv_slot2_from_slot3() {
        // SUBV dst=2, src=3
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x6A, argc: 2, args_raw: [0x02, 0x00, 0x03, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 16);
        assert_eq!(
            &out.text,
            &[
                0x4C, 0x8B, 0x47, 0x10,
                0x4C, 0x8B, 0x48, 0x18,
                0x48, 0x29, 0xC8,          // sub rax, rcx
                0x4C, 0x89, 0x47, 0x10,
                0xC3,
            ]
        );
    }

    #[test]
    fn emit_x86_imul_slot1_from_slot2() {
        // IMUL dst=1, src=2 → 4+4+4+4+1 = 17B (imul is 4B not 3B)
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x63, argc: 2, args_raw: [0x01, 0x00, 0x02, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 17);
        assert_eq!(
            &out.text,
            &[
                0x4C, 0x8B, 0x47, 0x08,
                0x4C, 0x8B, 0x48, 0x10,
                0x48, 0x0F, 0xAF, 0xC8,    // imul rax, rcx
                0x4C, 0x89, 0x47, 0x08,
                0xC3,
            ]
        );
    }

    #[test]
    fn emit_x86_cmp_slot1_vs_slot2() {
        // CMP a=1, b=2 → 4+4+3+1 = 12B (no store, no ret inside emit — but
        // emit_x86 appends ret at end)
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x65, argc: 2, args_raw: [0x01, 0x00, 0x02, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 12);
        assert_eq!(
            &out.text,
            &[
                0x4C, 0x8B, 0x47, 0x08,
                0x4C, 0x8B, 0x48, 0x10,
                0x48, 0x39, 0xC0,          // cmp rax, rcx (MODRM 0xC0: reg=rcx, r/m=rax)
                0xC3,
            ]
        );
    }
}
