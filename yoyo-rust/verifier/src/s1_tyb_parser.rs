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
    /// Emitted x86-64 machine code (.text).
    pub text: Vec<u8>,
    /// Data/string/raw payload bytes (.data). Filled by DATA/STR/RAW (S1.4).
    pub data: Vec<u8>,
}

/// S1.2 physical register set (4 regs). Standalone — does not depend on
/// `crate::types::Reg` so this module stays portable for the in-DLL port.
///
/// Allocation policy:
/// - `R15` = pinned state base (never a scratch)
/// - `Rax` = primary scratch / result
/// - `Rcx` = secondary operand / memcpy count
/// - `Rdx` = tertiary scratch
/// - `Rsi` / `Rdi` = memcpy src/dst pointers (S1.3 MEMCPY_DATA)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Reg {
    Rax = 0,
    Rcx = 1,
    Rdx = 2,
    Rsi = 6,
    Rdi = 7,
    R15 = 15,
}

impl Reg {
    fn low3(self) -> u8 {
        (self as u8) & 7
    }
    fn rex_bit(self) -> bool {
        (self as u8) >= 8
    }
}

fn rex_wrxb(w: bool, r: bool, x: bool, b: bool) -> u8 {
    0x40 | ((w as u8) << 3) | ((r as u8) << 2) | ((x as u8) << 1) | (b as u8)
}

/// TYB argc=2 layout: `[u16 slot][u32 imm]` (per `tyb_parser.rs` / `make_record`).
/// Used by SET / ADD-imm / SUB-imm.
fn arg_slot_imm(rec: &TybRecord) -> Option<(u16, u32)> {
    if rec.argc != 2 {
        return None;
    }
    let slot = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
    let imm = u32::from_le_bytes([
        rec.args_raw[2], rec.args_raw[3], rec.args_raw[4], rec.args_raw[5],
    ]);
    Some((slot, imm))
}

/// TYB argc=2 layout for slot-slot ops (GET/MOVRR/IMUL/ADDV/ORV/SUBV/CMP).
///
/// Wire format still packs the second field as u32 (same as SET), but the
/// second operand is a **slot** — take the low 16 bits intentionally.
/// This is the S1.2 GET-arg semantics clarification: not an accidental cast.
fn arg_slot_slot(rec: &TybRecord) -> Option<(u16, u16)> {
    if rec.argc != 2 {
        return None;
    }
    let a = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
    let b = u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]);
    Some((a, b))
}

/// TYB argc=1 layout: u32 LE in args_raw[0..4] (LABEL/CALL/JMP/JCC/INC/DEC/RAW_BYTE).
fn arg_u32(rec: &TybRecord) -> Option<u32> {
    if rec.argc != 1 {
        return None;
    }
    Some(u32::from_le_bytes([
        rec.args_raw[0], rec.args_raw[1], rec.args_raw[2], rec.args_raw[3],
    ]))
}

/// TYB argc=3 layout: three u16 LE fields (LDB / MEMCPY_*).
fn arg_slot3(rec: &TybRecord) -> Option<(u16, u16, u16)> {
    if rec.argc != 3 {
        return None;
    }
    Some((
        u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]),
        u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]),
        u16::from_le_bytes([rec.args_raw[4], rec.args_raw[5]]),
    ))
}

/// movabs rax, imm64 (10B). Imm32 is zero-extended into the 8-byte field.
/// Matches `assembler.rs::movabs(Reg::Rax, imm)`.
fn emit_movabs_rax(out: &mut Vec<u8>, imm: u32) {
    out.extend_from_slice(&[0x48, 0xB8]);
    out.extend_from_slice(&(imm as u64).to_le_bytes());
}

/// mov [r15 + slot*8], src — matches `assembler.rs::store_state`.
fn emit_store_state(out: &mut Vec<u8>, slot: u16, src: Reg) {
    let disp = (slot as u32) * 8;
    let rex = rex_wrxb(true, src.rex_bit(), false, true); // B=1: base=r15
    let modrm_reg = src.low3() << 3;
    if disp <= 127 {
        out.extend_from_slice(&[rex, 0x89, modrm_reg | 0x40 | 0x07, disp as u8]);
    } else {
        out.extend_from_slice(&[rex, 0x89, modrm_reg | 0x80 | 0x07]);
        out.extend_from_slice(&(disp as i32).to_le_bytes());
    }
}

/// mov dest, [r15 + slot*8] — matches `assembler.rs::load_state`.
fn emit_load_state(out: &mut Vec<u8>, slot: u16, dest: Reg) {
    let disp = (slot as u32) * 8;
    let rex = rex_wrxb(true, dest.rex_bit(), false, true); // B=1: base=r15
    let modrm_reg = dest.low3() << 3;
    if disp <= 127 {
        out.extend_from_slice(&[rex, 0x8B, modrm_reg | 0x40 | 0x07, disp as u8]);
    } else {
        out.extend_from_slice(&[rex, 0x8B, modrm_reg | 0x80 | 0x07]);
        out.extend_from_slice(&(disp as i32).to_le_bytes());
    }
}

/// add reg, imm32 → REX.W 81 /0.
fn emit_add_imm(out: &mut Vec<u8>, reg: Reg, imm: u32) {
    let rex = rex_wrxb(true, false, false, reg.rex_bit());
    out.extend_from_slice(&[rex, 0x81, 0xC0 | reg.low3()]);
    out.extend_from_slice(&imm.to_le_bytes());
}

/// sub reg, imm32 → REX.W 81 /5. Was wrongly /4 (E0) before S1.2.
fn emit_sub_imm(out: &mut Vec<u8>, reg: Reg, imm: u32) {
    let rex = rex_wrxb(true, false, false, reg.rex_bit());
    out.extend_from_slice(&[rex, 0x81, 0xE8 | reg.low3()]);
    out.extend_from_slice(&imm.to_le_bytes());
}

/// inc reg → REX.W FF /0.
fn emit_inc(out: &mut Vec<u8>, reg: Reg) {
    let rex = rex_wrxb(true, false, false, reg.rex_bit());
    out.extend_from_slice(&[rex, 0xFF, 0xC0 | reg.low3()]);
}

/// dec reg → REX.W FF /1.
fn emit_dec(out: &mut Vec<u8>, reg: Reg) {
    let rex = rex_wrxb(true, false, false, reg.rex_bit());
    out.extend_from_slice(&[rex, 0xFF, 0xC8 | reg.low3()]);
}

/// add dst, src — matches `assembler.rs::add_reg`.
fn emit_add_reg(out: &mut Vec<u8>, dst: Reg, src: Reg) {
    let rex = rex_wrxb(true, src.rex_bit(), false, dst.rex_bit());
    out.extend_from_slice(&[rex, 0x01, 0xC0 | (src.low3() << 3) | dst.low3()]);
}

/// or dst, src — matches `assembler.rs::or_reg`.
fn emit_or_reg(out: &mut Vec<u8>, dst: Reg, src: Reg) {
    let rex = rex_wrxb(true, src.rex_bit(), false, dst.rex_bit());
    out.extend_from_slice(&[rex, 0x09, 0xC0 | (src.low3() << 3) | dst.low3()]);
}

/// sub dst, src — matches `assembler.rs::sub_reg`.
fn emit_sub_reg(out: &mut Vec<u8>, dst: Reg, src: Reg) {
    let rex = rex_wrxb(true, src.rex_bit(), false, dst.rex_bit());
    out.extend_from_slice(&[rex, 0x29, 0xC0 | (src.low3() << 3) | dst.low3()]);
}

/// imul dst, src — matches `assembler.rs::mul_reg` (ModRM reg=dst, r/m=src).
fn emit_imul_reg(out: &mut Vec<u8>, dst: Reg, src: Reg) {
    let rex = rex_wrxb(true, dst.rex_bit(), false, src.rex_bit());
    out.extend_from_slice(&[rex, 0x0F, 0xAF, 0xC0 | (dst.low3() << 3) | src.low3()]);
}

/// cmp a, b — matches `assembler.rs::cmp_reg` (ModRM reg=b, r/m=a).
fn emit_cmp_reg(out: &mut Vec<u8>, a: Reg, b: Reg) {
    let rex = rex_wrxb(true, b.rex_bit(), false, a.rex_bit());
    out.extend_from_slice(&[rex, 0x39, 0xC0 | (b.low3() << 3) | a.low3()]);
}

/// movzx rax, byte [rax] — 4B (`48 0F B6 00`). Used by LDB.
fn emit_movzx_rax_byte_rax(out: &mut Vec<u8>) {
    out.extend_from_slice(&[0x48, 0x0F, 0xB6, 0x00]);
}

/// `rep movsb` — 2B (`F3 A4`).
///
/// Note: `assembler.rs::emit_memcpy_data` currently pushes `0xFC` (CLD) and
/// mislabels it as rep movsb. S1.3 emits the correct encoding.
fn emit_rep_movsb(out: &mut Vec<u8>) {
    out.extend_from_slice(&[0xF3, 0xA4]);
}

/// LDB dd ss oo — load byte from mem[state[ss]+oo] into state[dd] (zero-extend).
/// Matches `emit.rs::emit_ldb` / platform default.
fn emit_ldb(out: &mut Vec<u8>, dd: u16, ss: u16, oo: u16) {
    emit_load_state(out, ss, Reg::Rax);
    if oo != 0 {
        emit_add_imm(out, Reg::Rax, oo as u32);
    }
    emit_movzx_rax_byte_rax(out);
    emit_store_state(out, dd, Reg::Rax);
}

/// MEMCPY_DATA dst src n — pointer-form: RSI=S[src], RDI=S[dst], RCX=S[n], rep movsb.
/// Arg order on the wire is (dst, src, n) per `isa_table` / `emit.rs`.
fn emit_memcpy_data(out: &mut Vec<u8>, dst: u16, src: u16, n: u16) {
    emit_load_state(out, src, Reg::Rsi);
    emit_load_state(out, dst, Reg::Rdi);
    emit_load_state(out, n, Reg::Rcx);
    emit_rep_movsb(out);
}

/// MEMCPY_STATE dst src n — slot-to-slot copy of `n` slots (Win32/simulator semantics).
/// Caps at 64 slots (same as `Win32Platform::emit_memcpy_state`); larger n → no-op.
fn emit_memcpy_state(out: &mut Vec<u8>, dst: u16, src: u16, n: u16) {
    if n > 64 {
        return;
    }
    for i in 0..n {
        emit_load_state(out, src.wrapping_add(i), Reg::Rax);
        emit_store_state(out, dst.wrapping_add(i), Reg::Rax);
    }
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
/// - `0x40` LABEL hh (argc=1): no x86 emitted; position recorded for rel32 patching
/// - `0x41` CALL hh (argc=1): E8 rel32 (5B); rel32 patched post-pass
/// - `0x70` JMP hh (argc=1): E9 rel32 (5B); rel32 patched post-pass
/// - `0x71..=0x7A` JCC hh (argc=1): 0F 8x rel32 (6B); rel32 patched post-pass
///   (second byte per `assembler.rs::JCC_VALID`: 0x71=0x84 je, 0x72=0x85 jne,
///   0x73=0x8C jl, 0x74=0x8D jge, 0x75=0x8E jle, 0x76=0x8F jg,
///   0x77=0x82 jb, 0x78=0x83 jae, 0x79=0x86 jbe, 0x7A=0x87 ja)
/// - `0xFF` RET (argc=0): ret
///
/// Two-pass label resolution (S1.1.f):
/// - Pass 1: emit instructions in order; record each LABEL's byte offset in
///   `labels: HashMap<u32, usize>`. For CALL/JMP/JCC, emit the opcode + a
///   placeholder 4-byte rel32 (zeros), and queue the patch site in
///   `rel32_patches: Vec<(usize, u32)>` (rel32 offset in `text`, target label_id).
/// - Pass 2: for each queued patch, if the label is defined, fill the rel32
///   as `target_ip - next_ip` where `next_ip = patch_pos + 4` (the address of
///   the byte right after the branch imm32). Undefined labels leave rel32=0.
///
/// S1.3 memory ops (this step):
/// - `0x80` LDB dd ss oo (argc=3)
/// - `0x84` MEMCPY_DATA dst src n (argc=3) — pointer-form + `rep movsb` (`F3 A4`)
/// - `0x85` MEMCPY_STATE dst src n (argc=3) — slot-to-slot (Win32/simulator)
/// - `0xA0` RAW_BYTE byte (argc=1) — emit literal byte
///
/// S1.4 data + PE wrap:
/// - `0x10` / `0x12` / `0x13` DATA/STR/RAW → append low-bytes of args to `.data` (no .text)
/// - `0xA1` RAW_BYTES → append low-bytes of args to `.text`
/// - `wrap_pe` packages `.text`+`.data` via `crate::pe_link::link_pe`
///
/// Platform ops (S2):
/// - `0x20` ALLOC slot size (argc=2): **StubPlatform-aligned** — `movabs rax,size`
///   + `store_state slot` (records requested size in state; **not** a real heap
///   allocator). S2.4: mini/in-DLL accept argc=2 stub form.
/// - `0x50` LOAD_FILE slot str_idx (argc=2): Stub — `movabs rax,str_idx` + store
///   (**not** real file I/O). S2.4: mini/in-DLL accept argc=2 stub form.
/// - `0x51` WRITE_FILE slot str_idx [sz]: Stub — `movabs rax,str_idx` + store;
///   host accepts argc=2|3 (`sz` ignored); mini/in-DLL accept **argc=2 only**.
///
/// Register allocation:
/// - r15 = pinned state base
/// - rax = primary scratch / result
/// - rcx = secondary / memcpy count
/// - rdx = tertiary
/// - rsi / rdi = memcpy pointers (MEMCPY_DATA)
pub fn emit_x86(prog: &TybProgram) -> CodegenOutput {
    use std::collections::HashMap;
    let mut text = Vec::with_capacity(prog.rec_cnt * 20 + 1);
    let mut data = Vec::new();
    let mut labels: HashMap<u32, usize> = HashMap::new();
    let mut rel32_patches: Vec<(usize, u32)> = Vec::new();
    for rec in &prog.records {
        match rec.op {
            // --- SET slot, imm ---
            0x30 => {
                if let Some((slot, imm)) = arg_slot_imm(rec) {
                    emit_movabs_rax(&mut text, imm);
                    emit_store_state(&mut text, slot, Reg::Rax);
                } else {
                    text.push(0x90);
                }
            }
            // --- GET / MOVRR dst, src ---
            0x60 | 0x64 => {
                if let Some((dst, src)) = arg_slot_slot(rec) {
                    emit_load_state(&mut text, src, Reg::Rax);
                    emit_store_state(&mut text, dst, Reg::Rax);
                } else {
                    text.push(0x90);
                }
            }
            // --- SUB / ADD slot, imm ---
            0x61 | 0x62 => {
                if let Some((slot, imm)) = arg_slot_imm(rec) {
                    emit_load_state(&mut text, slot, Reg::Rax);
                    if rec.op == 0x61 {
                        emit_sub_imm(&mut text, Reg::Rax, imm);
                    } else {
                        emit_add_imm(&mut text, Reg::Rax, imm);
                    }
                    emit_store_state(&mut text, slot, Reg::Rax);
                } else {
                    text.push(0x90);
                }
            }
            // --- INC / DEC slot ---
            0x66 | 0x67 => {
                if let Some(slot) = arg_u32(rec) {
                    let slot = slot as u16;
                    emit_load_state(&mut text, slot, Reg::Rax);
                    if rec.op == 0x66 {
                        emit_inc(&mut text, Reg::Rax);
                    } else {
                        emit_dec(&mut text, Reg::Rax);
                    }
                    emit_store_state(&mut text, slot, Reg::Rax);
                } else {
                    text.push(0x90);
                }
            }
            // --- IMUL dst, src ---
            0x63 => {
                if let Some((dst, src)) = arg_slot_slot(rec) {
                    emit_load_state(&mut text, dst, Reg::Rax);
                    emit_load_state(&mut text, src, Reg::Rcx);
                    emit_imul_reg(&mut text, Reg::Rax, Reg::Rcx);
                    emit_store_state(&mut text, dst, Reg::Rax);
                } else {
                    text.push(0x90);
                }
            }
            // --- ADDV / ORV / SUBV dst, src ---
            0x68 | 0x69 | 0x6A => {
                if let Some((dst, src)) = arg_slot_slot(rec) {
                    emit_load_state(&mut text, dst, Reg::Rax);
                    emit_load_state(&mut text, src, Reg::Rcx);
                    if rec.op == 0x68 {
                        emit_add_reg(&mut text, Reg::Rax, Reg::Rcx);
                    } else if rec.op == 0x69 {
                        emit_or_reg(&mut text, Reg::Rax, Reg::Rcx);
                    } else {
                        emit_sub_reg(&mut text, Reg::Rax, Reg::Rcx);
                    }
                    emit_store_state(&mut text, dst, Reg::Rax);
                } else {
                    text.push(0x90);
                }
            }
            // --- CMP a, b ---
            0x65 => {
                if let Some((a, b)) = arg_slot_slot(rec) {
                    emit_load_state(&mut text, a, Reg::Rax);
                    emit_load_state(&mut text, b, Reg::Rcx);
                    emit_cmp_reg(&mut text, Reg::Rax, Reg::Rcx);
                } else {
                    text.push(0x90);
                }
            }
            // --- LABEL hh (argc=1) ---
            0x40 => {
                if let Some(label_id) = arg_u32(rec) {
                    labels.insert(label_id, text.len());
                }
            }
            // --- CALL hh (argc=1) → E8 rel32 (5B) ---
            0x41 => {
                if let Some(label_id) = arg_u32(rec) {
                    let patch_pos = text.len() + 1;
                    text.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
                    rel32_patches.push((patch_pos, label_id));
                } else {
                    text.push(0x90);
                }
            }
            // --- JMP hh (argc=1) → E9 rel32 (5B) ---
            0x70 => {
                if let Some(label_id) = arg_u32(rec) {
                    let patch_pos = text.len() + 1;
                    text.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
                    rel32_patches.push((patch_pos, label_id));
                } else {
                    text.push(0x90);
                }
            }
            // --- JCC hh (0x71..=0x7A, argc=1) → 0F 8x rel32 (6B) ---
            0x71 | 0x72 | 0x73 | 0x74 | 0x75 | 0x76 | 0x77 | 0x78 | 0x79 | 0x7A => {
                if let Some(label_id) = arg_u32(rec) {
                    let cc = match rec.op {
                        0x71 => 0x84, // je
                        0x72 => 0x85, // jne
                        0x73 => 0x8C, // jl
                        0x74 => 0x8D, // jge
                        0x75 => 0x8E, // jle
                        0x76 => 0x8F, // jg
                        0x77 => 0x82, // jb
                        0x78 => 0x83, // jae
                        0x79 => 0x86, // jbe
                        0x7A => 0x87, // ja
                        _ => unreachable!(),
                    };
                    let patch_pos = text.len() + 2;
                    text.extend_from_slice(&[0x0F, cc, 0, 0, 0, 0]);
                    rel32_patches.push((patch_pos, label_id));
                } else {
                    text.push(0x90);
                }
            }
            // --- LDB dd ss oo ---
            0x80 => {
                if let Some((dd, ss, oo)) = arg_slot3(rec) {
                    emit_ldb(&mut text, dd, ss, oo);
                } else {
                    text.push(0x90);
                }
            }
            // --- MEMCPY_DATA dst src n ---
            0x84 => {
                if let Some((dst, src, n)) = arg_slot3(rec) {
                    emit_memcpy_data(&mut text, dst, src, n);
                } else {
                    text.push(0x90);
                }
            }
            // --- MEMCPY_STATE dst src n ---
            0x85 => {
                if let Some((dst, src, n)) = arg_slot3(rec) {
                    if n == 0 || n > 64 {
                        // empty / oversized: no bytes (safety); trailing ret still appended
                    } else {
                        emit_memcpy_state(&mut text, dst, src, n);
                    }
                } else {
                    text.push(0x90);
                }
            }
            // --- RAW_BYTE byte ---
            0xA0 => {
                if let Some(b) = arg_u32(rec) {
                    text.push(b as u8);
                } else {
                    text.push(0x90);
                }
            }
            // --- DATA / STR / RAW → .data (no .text) ---
            0x10 | 0x12 | 0x13 => {
                data.extend(arg_low_bytes(rec));
            }
            // --- RAW_BYTES → append literal bytes to .text ---
            0xA1 => {
                text.extend(arg_low_bytes(rec));
            }
            // --- ALLOC slot, size (S3.94: slim VirtualAlloc IAT) ---
            0x20 => {
                if let Some((slot, size)) = arg_slot_imm(rec) {
                    if slot <= 255 {
                        text.extend(emit_slim_alloc(slot, size));
                    } else {
                        text.push(0x90);
                    }
                } else {
                    text.push(0x90);
                }
            }
            // --- LOAD_FILE slot, path_off (S3.94: slim IAT; path at r15+path_off) ---
            0x50 => {
                if let Some((slot, path_off)) = arg_slot_imm(rec) {
                    if slot <= 255 && path_off < S1_SLIM_IAT_BASE {
                        text.extend(emit_slim_load_file(slot, path_off));
                    } else {
                        text.push(0x90);
                    }
                } else {
                    text.push(0x90);
                }
            }
            // --- WRITE_FILE slot, path_off, sz_slot (S3.94: argc=3 slim IAT) ---
            0x51 => {
                if let Some((slot, path_off, sz_slot)) = arg_slot3(rec) {
                    if slot <= 255
                        && sz_slot <= 255
                        && (path_off as u32) < S1_SLIM_IAT_BASE
                    {
                        text.extend(emit_slim_write_file(
                            slot,
                            path_off as u32,
                            sz_slot,
                        ));
                    } else {
                        text.push(0x90);
                    }
                } else if let Some((slot, path_off)) = arg_slot_imm(rec) {
                    // argc=2: sz_slot=0 (legacy wire; size from slot0)
                    if slot <= 255 && path_off < S1_SLIM_IAT_BASE {
                        text.extend(emit_slim_write_file(slot, path_off, 0));
                    } else {
                        text.push(0x90);
                    }
                } else {
                    text.push(0x90);
                }
            }
            // --- NOP (explicit) ---
            0x00 if rec.argc == 0 => {
                text.push(0x90);
            }
            // --- RET ---
            0xFF if rec.argc == 0 => {
                text.push(0xC3);
            }
            // --- unknown: NOP placeholder ---
            _ => {
                text.push(0x90);
            }
        }
    }
    // Pass 2: fill rel32 = target_ip - next_ip, where next_ip = patch_pos + 4.
    for (patch_pos, label_id) in &rel32_patches {
        if let Some(&target) = labels.get(label_id) {
            let next_ip = *patch_pos + 4;
            let rel = target as i32 - next_ip as i32;
            text[*patch_pos..*patch_pos + 4].copy_from_slice(&rel.to_le_bytes());
        }
        // Undefined label: leave rel32 as 0 (no error).
    }
    // Always terminate with ret if we didn't end on one.
    if text.is_empty() || *text.last().unwrap() != 0xC3 {
        text.push(0xC3);
    }
    CodegenOutput { text, data }
}

/// Decode TYB args and take each arg's low byte (matches `emit.rs` DATA/STR/RAW/RAW_BYTES).
fn arg_low_bytes(rec: &TybRecord) -> Vec<u8> {
    match rec.argc {
        0 => Vec::new(),
        1 => {
            // argc=1: u32 LE in args_raw[0..4] → one arg, low byte = args_raw[0]
            vec![rec.args_raw[0]]
        }
        2 => {
            // argc=2: a0=u16@[0..2], a1=u32@[2..6] → low bytes at [0] and [2]
            vec![rec.args_raw[0], rec.args_raw[2]]
        }
        3 => {
            // argc=3: three u16s → low bytes at [0], [2], [4]
            vec![rec.args_raw[0], rec.args_raw[2], rec.args_raw[4]]
        }
        _ => Vec::new(),
    }
}

/// S1.4 — wrap codegen `.text`+`.data` into a PE32+ image via `pe_link::link_pe`.
///
/// Entry setup (R15 → .data) is handled inside `link_pe`. This is the host-side
/// packaging step; in-DLL entry rewrite remains S1.5.
///
/// Note: `link_pe` pads `.data` to `OUTPUT_DATA_NEED` (~230 KiB) for H_00
/// compatibility. Prefer [`wrap_pe_slim`] / [`compile_tyb_to_pe`] when the PE
/// must fit in an in-DLL stack buffer (S1.5).
pub fn wrap_pe(out: &CodegenOutput) -> crate::types::IsaResult<crate::pe_link::PeImage> {
    crate::pe_link::link_pe(&out.text, &out.data)
}

/// Max `.text` payload (startup 13 + user code) for slim PE / in-DLL scratch.
/// S3.7: raised 256→512 (in-DLL scratch + host slim PE keep in sync).
pub const S1_SLIM_MAX_CODE: usize = 512;
/// Slim `.data` virtual size (state base for R15); keeps PE ≪ `link_pe` H_00 pad.
pub const S1_SLIM_DATA_VS: u32 = 0x1000;

/// S3.93: slim IAT blob (VA+CreateFileA+ReadFile+WriteFile+CloseHandle; 16-aligned).
pub const S1_SLIM_IAT_PAD: usize = 0x100;
/// S3.89+: IAT at high end of slim `.data` so r15+0 state slots stay free.
/// ALLOC: `call [r15 + S1_SLIM_IAT_BASE]` (= VirtualAlloc slot0).
pub const S1_SLIM_IAT_BASE: u32 = S1_SLIM_DATA_VS - S1_SLIM_IAT_PAD as u32;
/// IAT slot byte offsets from [`S1_SLIM_IAT_BASE`] (qword stride).
pub const S1_SLIM_IAT_OFF_VIRTUAL_ALLOC: u32 = 0;
pub const S1_SLIM_IAT_OFF_CREATE_FILE_A: u32 = 8;
pub const S1_SLIM_IAT_OFF_READ_FILE: u32 = 16;
pub const S1_SLIM_IAT_OFF_WRITE_FILE: u32 = 24;
pub const S1_SLIM_IAT_OFF_CLOSE_HANDLE: u32 = 32;
/// Slim LOAD reads into a VirtualAlloc'd buffer of this size.
pub const S1_SLIM_READ_CHUNK: u32 = 0x10000;

fn emit_slim_call_iat(out: &mut Vec<u8>, off: u32) {
    out.extend_from_slice(&[0x41, 0xFF, 0x97]);
    out.extend_from_slice(&(S1_SLIM_IAT_BASE + off).to_le_bytes());
}

fn emit_slim_store_disp32(out: &mut Vec<u8>, slot: u16) {
    // mov [r15+slot*8], rax — always disp32 (any-slot≤255)
    let disp = (slot as u32) * 8;
    out.extend_from_slice(&[0x49, 0x89, 0x87]);
    out.extend_from_slice(&disp.to_le_bytes());
}

fn emit_slim_load_disp32(out: &mut Vec<u8>, slot: u16, dest_modrm: u8) {
    // mov dest, [r15+slot*8] — dest_modrm: rax=0x87 rcx=0x8F rdx=0x97 r8=0x87|/r
    let disp = (slot as u32) * 8;
    out.extend_from_slice(&[0x49, 0x8B, dest_modrm]);
    out.extend_from_slice(&disp.to_le_bytes());
}

/// S3.94: ALLOC → VirtualAlloc via slim IAT + store (disp32).
pub fn emit_slim_alloc(slot: u16, size: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(48);
    out.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);
    out.extend_from_slice(&[0x31, 0xC9]); // xor ecx,ecx
    out.extend_from_slice(&[0xBA]);
    out.extend_from_slice(&size.to_le_bytes());
    out.extend_from_slice(&[0x41, 0xB8, 0x00, 0x30, 0x00, 0x00]);
    out.extend_from_slice(&[0x41, 0xB9, 0x04, 0x00, 0x00, 0x00]);
    emit_slim_call_iat(&mut out, S1_SLIM_IAT_OFF_VIRTUAL_ALLOC);
    out.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);
    emit_slim_store_disp32(&mut out, slot);
    out
}

/// S3.94: LOAD_FILE — path at `[r15+path_off]` (slim .data byte offset, not WIN32 0x10000 table).
/// CreateFileA → VirtualAlloc(READ_CHUNK) → ReadFile → CloseHandle → store buf ptr.
/// Preserves rbx/rsi (Win64 non-volatiles). Frame = push×2 + `sub rsp,0x38` (16-align + 3 stack args).
pub fn emit_slim_load_file(slot: u16, path_off: u32) -> Vec<u8> {
    let mut out = Vec::with_capacity(176);
    out.extend_from_slice(&[0x53, 0x56]); // push rbx, rsi
    out.extend_from_slice(&[0x48, 0x83, 0xEC, 0x38]); // shadow + 3 stack args (keep 16-align)
    // lea rcx, [r15+path_off]
    out.extend_from_slice(&[0x49, 0x8D, 0x8F]);
    out.extend_from_slice(&path_off.to_le_bytes());
    out.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x80]); // GENERIC_READ
    out.extend_from_slice(&[0x41, 0xB8, 0x01, 0x00, 0x00, 0x00]); // FILE_SHARE_READ
    out.extend_from_slice(&[0x45, 0x31, 0xC9]); // xor r9d (sa=NULL)
    out.extend_from_slice(&[0xC7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00]); // OPEN_EXISTING
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00]); // FILE_ATTRIBUTE_NORMAL
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00]); // hTemplate=NULL
    emit_slim_call_iat(&mut out, S1_SLIM_IAT_OFF_CREATE_FILE_A);
    out.extend_from_slice(&[0x48, 0x89, 0xC3]); // mov rbx, rax (handle)
    // VirtualAlloc(NULL, READ_CHUNK, COMMIT|RESERVE, READWRITE)
    out.extend_from_slice(&[0x31, 0xC9]);
    out.extend_from_slice(&[0xBA]);
    out.extend_from_slice(&S1_SLIM_READ_CHUNK.to_le_bytes());
    out.extend_from_slice(&[0x41, 0xB8, 0x00, 0x30, 0x00, 0x00]);
    out.extend_from_slice(&[0x41, 0xB9, 0x04, 0x00, 0x00, 0x00]);
    emit_slim_call_iat(&mut out, S1_SLIM_IAT_OFF_VIRTUAL_ALLOC);
    out.extend_from_slice(&[0x48, 0x89, 0xC6]); // mov rsi, rax (buf)
    // ReadFile(h, buf, READ_CHUNK, &bytes_read, NULL) — clear nRead first (h00 parity)
    out.extend_from_slice(&[0x48, 0x89, 0xD9]); // rcx = handle
    out.extend_from_slice(&[0x48, 0x89, 0xF2]); // rdx = buf
    out.extend_from_slice(&[0x41, 0xB8]);
    out.extend_from_slice(&S1_SLIM_READ_CHUNK.to_le_bytes());
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00]); // nRead=0
    out.extend_from_slice(&[0x4C, 0x8D, 0x4C, 0x24, 0x20]); // lea r9,[rsp+0x20]
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00]); // lpOverlapped=NULL
    emit_slim_call_iat(&mut out, S1_SLIM_IAT_OFF_READ_FILE);
    out.extend_from_slice(&[0x48, 0x89, 0xD9]);
    emit_slim_call_iat(&mut out, S1_SLIM_IAT_OFF_CLOSE_HANDLE);
    out.extend_from_slice(&[0x48, 0x83, 0xC4, 0x38]);
    out.extend_from_slice(&[0x48, 0x89, 0xF0]); // rax = buf
    emit_slim_store_disp32(&mut out, slot);
    out.extend_from_slice(&[0x5E, 0x5B]); // pop rsi, rbx
    out
}

/// S3.94: WRITE_FILE — path at `[r15+path_off]`; buf=`[r15+slot*8]`; size=`[r15+sz_slot*8]`.
/// Preserves rbx/rsi (Win64 non-volatiles). Frame = push×2 + `sub rsp,0x38`.
pub fn emit_slim_write_file(slot: u16, path_off: u32, sz_slot: u16) -> Vec<u8> {
    let mut out = Vec::with_capacity(160);
    out.extend_from_slice(&[0x53, 0x56]); // push rbx, rsi
    out.extend_from_slice(&[0x48, 0x83, 0xEC, 0x38]);
    out.extend_from_slice(&[0x49, 0x8D, 0x8F]);
    out.extend_from_slice(&path_off.to_le_bytes());
    out.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x40]); // GENERIC_WRITE
    out.extend_from_slice(&[0x41, 0xB8, 0x00, 0x00, 0x00, 0x00]); // share=0 (exclusive)
    out.extend_from_slice(&[0x45, 0x31, 0xC9]); // xor r9d (sa=NULL)
    out.extend_from_slice(&[0xC7, 0x44, 0x24, 0x20, 0x02, 0x00, 0x00, 0x00]); // CREATE_ALWAYS
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00]);
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00]);
    emit_slim_call_iat(&mut out, S1_SLIM_IAT_OFF_CREATE_FILE_A);
    out.extend_from_slice(&[0x48, 0x89, 0xC3]);
    emit_slim_load_disp32(&mut out, slot, 0x97); // mov rdx, [r15+slot]
    emit_slim_load_disp32(&mut out, sz_slot, 0x87); // mov rax, [r15+sz]
    out.extend_from_slice(&[0x49, 0x89, 0xC0]); // mov r8, rax
    out.extend_from_slice(&[0x48, 0x89, 0xD9]); // rcx = handle
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00]); // nWritten=0
    out.extend_from_slice(&[0x4C, 0x8D, 0x4C, 0x24, 0x20]); // lea r9,[rsp+0x20]
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00]);
    emit_slim_call_iat(&mut out, S1_SLIM_IAT_OFF_WRITE_FILE);
    out.extend_from_slice(&[0x48, 0x89, 0xD9]);
    emit_slim_call_iat(&mut out, S1_SLIM_IAT_OFF_CLOSE_HANDLE);
    out.extend_from_slice(&[0x48, 0x83, 0xC4, 0x38]);
    out.extend_from_slice(&[0x48, 0x31, 0xC0]); // xor rax,rax
    emit_slim_store_disp32(&mut out, slot);
    out.extend_from_slice(&[0x5E, 0x5B]); // pop rsi, rbx
    out
}

/// Patch sites inside [`emit_slim_load_file`] / [`emit_slim_write_file`] blobs (for in-DLL bake).
pub fn slim_load_path_patch_off() -> usize {
    // push rbx,rsi (2) + sub rsp (4) + lea 49 8D 8F (3) → disp32
    2 + 4 + 3
}
pub fn slim_load_slot_patch_off(blob: &[u8]) -> usize {
    // store disp32 then pop rsi,rbx (2) — disp is 4 bytes before the pops
    blob.len() - 2 - 4
}
pub fn slim_write_path_patch_off() -> usize {
    // push rbx,rsi (2) + sub rsp (4) + lea (3)
    2 + 4 + 3
}
pub fn slim_write_slot_load_patch_off(blob: &[u8]) -> usize {
    // first load_disp32 after mov rbx,rax (3) following CreateFile call
    // Find 49 8B 97 (mov rdx,[r15+…])
    blob.windows(3)
        .position(|w| w == [0x49, 0x8B, 0x97])
        .expect("write tmpl rdx load")
        + 3
}
pub fn slim_write_sz_load_patch_off(blob: &[u8]) -> usize {
    blob.windows(3)
        .position(|w| w == [0x49, 0x8B, 0x87])
        .expect("write tmpl rax/sz load")
        + 3
}
pub fn slim_write_store_patch_off(blob: &[u8]) -> usize {
    // store disp32 then pop rsi,rbx (2)
    blob.len() - 2 - 4
}

/// S1.5 — minimal PE32+ for in-DLL codegen (S3.93: +CloseHandle IAT, tiny `.data`).
///
/// Layout mirrors `pe_link::link_pe_impl` startup (`lea r15,[rip+disp]; jmp code`)
/// but with `.data` VS = [`S1_SLIM_DATA_VS`] so the image fits in ~6–8 KiB.
pub fn wrap_pe_slim(out: &CodegenOutput) -> crate::types::IsaResult<crate::pe_link::PeImage> {
    wrap_pe_slim_bytes(&out.text, &out.data)
}

/// Build slim PE from raw `.text` / `.data` bytes.
pub fn wrap_pe_slim_bytes(
    code: &[u8],
    data: &[u8],
) -> crate::types::IsaResult<crate::pe_link::PeImage> {
    use crate::types::IsaError;
    if code.len() > S1_SLIM_MAX_CODE {
        return Err(IsaError::PlatformError {
            msg: format!(
                "s1 slim PE: code {} exceeds max {}",
                code.len(),
                S1_SLIM_MAX_CODE
            ),
        });
    }
    if data.len() > S1_SLIM_IAT_BASE as usize {
        return Err(IsaError::PlatformError {
            msg: format!(
                "s1 slim PE: data {} overlaps IAT base 0x{:x} (VS {})",
                data.len(),
                S1_SLIM_IAT_BASE,
                S1_SLIM_DATA_VS
            ),
        });
    }

    const SECTION_ALIGN: u32 = 0x1000;
    const FILE_ALIGN: u32 = 0x200;
    const HEADERS_RAW: u32 = 0x400;
    const PE_STARTUP_LEN: u32 = 13;
    const TEXT_RVA: u32 = SECTION_ALIGN; // 0x1000

    let align_up = |v: u32, a: u32| (v + a - 1) & !(a - 1);
    let align_up_usize = |v: usize, a: usize| (v + a - 1) & !(a - 1);
    let write_u16 = |buf: &mut [u8], off: usize, v: u16| {
        buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
    };
    let write_u32 = |buf: &mut [u8], off: usize, v: u32| {
        buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    };
    let write_u64 = |buf: &mut [u8], off: usize, v: u64| {
        buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
    };
    let write_name = |buf: &mut [u8], off: usize, name: &[u8]| {
        let n = name.len().min(8);
        buf[off..off + n].copy_from_slice(&name[..n]);
    };

    let code_need = code.len() as u32 + PE_STARTUP_LEN;
    // S3.7: always reserve room for S1_SLIM_MAX_CODE so file offsets
    // (TEXT/DATA payload) stay stable for host↔in-DLL parity.
    let code_raw = align_up(
        code_need
            .max(1)
            .max(PE_STARTUP_LEN + S1_SLIM_MAX_CODE as u32),
        FILE_ALIGN,
    );
    let text_vs = align_up(code_need + 0x40, SECTION_ALIGN);
    let data_rva = TEXT_RVA + text_vs;
    let data_vs = S1_SLIM_DATA_VS;
    let data_raw = align_up(data_vs, FILE_ALIGN);
    let size_of_image = align_up(data_rva + data_vs, SECTION_ALIGN);

    let mut img = vec![0u8; (HEADERS_RAW + code_raw + data_raw) as usize];

    img[0] = 0x4D;
    img[1] = 0x5A;
    write_u32(&mut img, 0x3C, 0x80);
    img[0x80] = b'P';
    img[0x81] = b'E';

    write_u16(&mut img, 0x84, 0x8664);
    write_u16(&mut img, 0x86, 2);
    write_u16(&mut img, 0x94, 0xF0);
    write_u16(&mut img, 0x96, 0x22);

    let opt = 0x98usize;
    write_u16(&mut img, opt, 0x20B);
    img[opt + 2] = 1;
    write_u32(&mut img, opt + 16, TEXT_RVA);
    write_u64(&mut img, opt + 24, 0x140000000);
    write_u32(&mut img, opt + 32, SECTION_ALIGN);
    write_u32(&mut img, opt + 36, FILE_ALIGN);
    write_u16(&mut img, opt + 40, 6);
    write_u16(&mut img, opt + 48, 6);
    write_u32(&mut img, opt + 56, size_of_image);
    write_u32(&mut img, opt + 60, HEADERS_RAW);
    write_u16(&mut img, opt + 68, 3);
    write_u16(&mut img, opt + 70, 0x0100);
    write_u64(&mut img, opt + 72, 0x100000);
    write_u64(&mut img, opt + 80, 0x1000);
    write_u64(&mut img, opt + 88, 0x100000);
    write_u64(&mut img, opt + 96, 0x1000);
    write_u32(&mut img, opt + 108, 16);
    write_u32(&mut img, opt + 4, code_raw);
    write_u32(&mut img, opt + 8, data_raw);
    write_u32(&mut img, opt + 20, TEXT_RVA);

    let s1 = 0x98 + 0xF0;
    write_name(&mut img, s1, b".text");
    write_u32(&mut img, s1 + 8, text_vs);
    write_u32(&mut img, s1 + 12, TEXT_RVA);
    write_u32(&mut img, s1 + 16, code_raw);
    write_u32(&mut img, s1 + 20, HEADERS_RAW);
    write_u32(&mut img, s1 + 36, 0x60000020);

    let s2 = s1 + 40;
    write_name(&mut img, s2, b".data");
    write_u32(&mut img, s2 + 8, data_vs);
    write_u32(&mut img, s2 + 12, data_rva);
    write_u32(&mut img, s2 + 16, data_raw);
    write_u32(&mut img, s2 + 20, HEADERS_RAW + code_raw);
    write_u32(&mut img, s2 + 36, 0xC0000040);

    let text_file_off = HEADERS_RAW as usize;
    let lea_disp = data_rva as i32 - (TEXT_RVA as i32 + 7);
    img[text_file_off] = 0x4C;
    img[text_file_off + 1] = 0x8D;
    img[text_file_off + 2] = 0x3D;
    img[text_file_off + 3..text_file_off + 7].copy_from_slice(&lea_disp.to_le_bytes());
    let jmp_rel = (TEXT_RVA + PE_STARTUP_LEN) as i32 - (TEXT_RVA as i32 + 7 + 5);
    img[text_file_off + 7] = 0xE9;
    img[text_file_off + 8..text_file_off + 12].copy_from_slice(&jmp_rel.to_le_bytes());
    img[text_file_off + 12] = 0x90;

    let code_dst = text_file_off + PE_STARTUP_LEN as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // S3.93: 5-import IAT at high end of .data (r15+S1_SLIM_IAT_BASE).
    // Layout: IAT 0x00 (6×8), ILT 0x30, desc 0x60, dll 0x88, hint-names 0x98…
    // VirtualAlloc stays slot0 so ALLOC emit is unchanged.
    let data_file_off = (HEADERS_RAW + code_raw) as usize;
    let iat_base = S1_SLIM_IAT_BASE as usize;
    let iat_off = iat_base;
    let ilt_off = iat_base + 0x30;
    let desc_off = iat_base + 0x60;
    let dll_name_off = iat_base + 0x88;
    let hn0_off = iat_base + 0x98;
    let names: &[&[u8]] = &[
        b"VirtualAlloc\0",
        b"CreateFileA\0",
        b"ReadFile\0",
        b"WriteFile\0",
        b"CloseHandle\0",
    ];
    let mut hn_offs = [0usize; 5];
    let mut hn_cur = hn0_off;
    for (i, name) in names.iter().enumerate() {
        hn_offs[i] = hn_cur;
        img[data_file_off + hn_cur] = 0;
        img[data_file_off + hn_cur + 1] = 0;
        img[data_file_off + hn_cur + 2..data_file_off + hn_cur + 2 + name.len()]
            .copy_from_slice(name);
        hn_cur += 2 + name.len();
        hn_cur = align_up_usize(hn_cur, 2);
    }
    debug_assert!(hn_cur - iat_base <= S1_SLIM_IAT_PAD);
    debug_assert_eq!(S1_SLIM_IAT_PAD, 0x100);
    for i in 0..5 {
        let hn_rva = data_rva + hn_offs[i] as u32;
        write_u64(&mut img, data_file_off + iat_off + i * 8, hn_rva as u64);
        write_u64(&mut img, data_file_off + ilt_off + i * 8, hn_rva as u64);
    }
    // IAT/ILT null terminators already zero-filled.
    write_u32(&mut img, data_file_off + desc_off, data_rva + ilt_off as u32);
    write_u32(
        &mut img,
        data_file_off + desc_off + 12,
        data_rva + dll_name_off as u32,
    );
    write_u32(
        &mut img,
        data_file_off + desc_off + 16,
        data_rva + iat_off as u32,
    );
    img[data_file_off + dll_name_off..data_file_off + dll_name_off + 13]
        .copy_from_slice(b"kernel32.dll\0");
    write_u32(&mut img, opt + 120, data_rva + desc_off as u32);
    write_u32(&mut img, opt + 124, 40);
    let copy_n = data.len().min(iat_base).min(data_raw as usize);
    img[data_file_off..data_file_off + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(crate::pe_link::PeImage { bytes: img })
}

/// File offset of user `.text` (after 13-byte startup) in slim PE images.
pub const S1_SLIM_TEXT_PAYLOAD_OFF: usize = 0x400 + 13;

/// File offset of user `.data` payload — headers 0x400 + code_raw for max code.
/// S3.7: code_raw = align(13+512)=0x400 → data at 0x800 (was 0x600 at 256).
/// S3.89: IAT at high end — user payload offset unchanged.
pub const S1_SLIM_DATA_PAYLOAD_OFF: usize = 0x400 + 0x400;

/// S1.5.b / S1.7.a–f in-DLL mini-ISA (must stay in sync with `pe_dll_link` export body).
///
/// S1.5.b: NOP/SET/LABEL/RAW_BYTE/RET  
/// S1.7.a: +GET/MOVRR/ADD imm/SUB imm/INC/DEC  
/// S1.7.b: +CMP/CALL/JMP/JCC (two-pass label patch, ≤16 labels/patches)  
/// S1.7.c: +IMUL/ADDV/ORV/SUBV  
/// S1.7.d: +LDB/MEMCPY_DATA/MEMCPY_STATE (state n≤4)  
/// S1.7.e: +DATA/STR/RAW → `.data` (≤64B total)  
/// S1.7.f: +RAW_BYTES → `.text` (argc≤3 low-bytes)  
/// S2.4: +ALLOC/LOAD_FILE/WRITE_FILE stub form (argc=2; not real OS I/O)
pub fn is_s1_mini_op(op: u8) -> bool {
    matches!(
        op,
        0x00 | 0x10
            | 0x12
            | 0x13
            | 0x20
            | 0x30
            | 0x40
            | 0x41
            | 0x50
            | 0x51
            | 0x60
            | 0x61
            | 0x62
            | 0x63
            | 0x64
            | 0x65
            | 0x66
            | 0x67
            | 0x68
            | 0x69
            | 0x6A
            | 0x70
            | 0x71
            | 0x72
            | 0x73
            | 0x74
            | 0x75
            | 0x76
            | 0x77
            | 0x78
            | 0x79
            | 0x7A
            | 0x80
            | 0x84
            | 0x85
            | 0xA0
            | 0xA1
            | 0xFF
    )
}

/// In-DLL MEMCPY_STATE unroll cap (8B×n must fit slim 256B code buf with headroom).
pub const S1_MINI_MEMCPY_STATE_MAX_N: u16 = 4;

/// In-DLL / mini host cap on aggregated DATA/STR/RAW bytes.
/// S3.10: 64→128; S3.13: 128→256; S3.25: 256→**384** (STACK 0x2E00).
pub const S1_MINI_DATA_MAX: usize = 384;

fn mini_slot_disp8_ok(slot: u16) -> bool {
    (slot as u32) * 8 < 0x80
}

/// S3.78: SET/platform-stub accept slot≤255 (disp8 or disp32).
fn mini_slot_any_ok(slot: u16) -> bool {
    slot <= 255
}

/// Validate TYB against the in-DLL mini-ISA (argc + slot disp8 limit).
pub fn validate_tyb_mini(prog: &TybProgram) -> crate::types::IsaResult<()> {
    use crate::types::IsaError;
    let bad = |i: usize, msg: String| {
        Err(IsaError::ParseError {
            line: i + 1,
            msg,
        })
    };
    let mut data_len = 0usize;
    for (i, rec) in prog.records.iter().enumerate() {
        match rec.op {
            0x10 | 0x12 | 0x13 => {
                if rec.argc > 3 {
                    return bad(
                        i,
                        format!("s1 mini DATA/STR/RAW argc={} > 3", rec.argc),
                    );
                }
                data_len += rec.argc as usize;
                if data_len > S1_MINI_DATA_MAX {
                    return bad(
                        i,
                        format!(
                            "s1 mini DATA/STR/RAW total {data_len} > max {S1_MINI_DATA_MAX}"
                        ),
                    );
                }
            }
            0x20 | 0x50 => {
                if rec.argc != 2 {
                    return bad(
                        i,
                        format!(
                            "s1 mini ALLOC/LOAD_FILE requires argc=2, got {}",
                            rec.argc
                        ),
                    );
                }
                let slot = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                if !mini_slot_any_ok(slot) {
                    return bad(
                        i,
                        format!("s1 mini ALLOC/LOAD slot {slot} out of range (>255)"),
                    );
                }
                if rec.op == 0x50 {
                    let path_off = u32::from_le_bytes([
                        rec.args_raw[2],
                        rec.args_raw[3],
                        rec.args_raw[4],
                        rec.args_raw[5],
                    ]);
                    if path_off >= S1_SLIM_IAT_BASE {
                        return bad(
                            i,
                            format!("s1 mini LOAD path_off 0x{path_off:x} overlaps IAT"),
                        );
                    }
                }
            }
            0x51 => {
                if rec.argc != 2 && rec.argc != 3 {
                    return bad(
                        i,
                        format!("s1 mini WRITE_FILE requires argc=2|3, got {}", rec.argc),
                    );
                }
                let slot = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                if !mini_slot_any_ok(slot) {
                    return bad(i, format!("s1 mini WRITE slot {slot} out of range"));
                }
                if rec.argc == 3 {
                    let path_off = u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]);
                    let sz_slot = u16::from_le_bytes([rec.args_raw[4], rec.args_raw[5]]);
                    if !mini_slot_any_ok(sz_slot) {
                        return bad(i, format!("s1 mini WRITE sz_slot out of range"));
                    }
                    if (path_off as u32) >= S1_SLIM_IAT_BASE {
                        return bad(i, format!("s1 mini WRITE path_off overlaps IAT"));
                    }
                } else {
                    let path_off = u32::from_le_bytes([
                        rec.args_raw[2],
                        rec.args_raw[3],
                        rec.args_raw[4],
                        rec.args_raw[5],
                    ]);
                    if path_off >= S1_SLIM_IAT_BASE {
                        return bad(i, format!("s1 mini WRITE path_off overlaps IAT"));
                    }
                }
            }
            0x30 => {
                if rec.argc != 2 {
                    return bad(i, format!("s1 mini SET requires argc=2, got {}", rec.argc));
                }
                let slot = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                if !mini_slot_any_ok(slot) {
                    return bad(
                        i,
                        format!("s1 mini SET slot {slot} out of range (>255)"),
                    );
                }
            }
            0x60 | 0x64 => {
                if rec.argc != 2 {
                    return bad(
                        i,
                        format!("s1 mini GET/MOVRR requires argc=2, got {}", rec.argc),
                    );
                }
                let dst = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                let src = u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]);
                if !mini_slot_disp8_ok(dst) || !mini_slot_disp8_ok(src) {
                    return bad(i, format!("s1 mini GET/MOVRR slot disp32 unsupported"));
                }
            }
            0x61 | 0x62 => {
                if rec.argc != 2 {
                    return bad(
                        i,
                        format!("s1 mini ADD/SUB imm requires argc=2, got {}", rec.argc),
                    );
                }
                let slot = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                if !mini_slot_disp8_ok(slot) {
                    return bad(i, format!("s1 mini ADD/SUB slot {slot} disp32 unsupported"));
                }
            }
            0x66 | 0x67 => {
                if rec.argc != 1 {
                    return bad(
                        i,
                        format!("s1 mini INC/DEC requires argc=1, got {}", rec.argc),
                    );
                }
                let slot = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                if !mini_slot_disp8_ok(slot) {
                    return bad(i, format!("s1 mini INC/DEC slot {slot} disp32 unsupported"));
                }
            }
            0x63 | 0x65 | 0x68 | 0x69 | 0x6A => {
                if rec.argc != 2 {
                    return bad(
                        i,
                        format!(
                            "s1 mini IMUL/CMP/ADDV/ORV/SUBV requires argc=2, got {}",
                            rec.argc
                        ),
                    );
                }
                let a = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                let b = u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]);
                if !mini_slot_disp8_ok(a) || !mini_slot_disp8_ok(b) {
                    return bad(i, "s1 mini ALU-reg/CMP slot disp32 unsupported".into());
                }
            }
            0x80 => {
                if rec.argc != 3 {
                    return bad(i, format!("s1 mini LDB requires argc=3, got {}", rec.argc));
                }
                let dd = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                let ss = u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]);
                if !mini_slot_disp8_ok(dd) || !mini_slot_disp8_ok(ss) {
                    return bad(i, "s1 mini LDB slot disp32 unsupported".into());
                }
            }
            0x84 => {
                if rec.argc != 3 {
                    return bad(
                        i,
                        format!("s1 mini MEMCPY_DATA requires argc=3, got {}", rec.argc),
                    );
                }
                let dst = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                let src = u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]);
                let n = u16::from_le_bytes([rec.args_raw[4], rec.args_raw[5]]);
                if !mini_slot_disp8_ok(dst) || !mini_slot_disp8_ok(src) || !mini_slot_disp8_ok(n) {
                    return bad(i, "s1 mini MEMCPY_DATA slot disp32 unsupported".into());
                }
            }
            0x85 => {
                if rec.argc != 3 {
                    return bad(
                        i,
                        format!("s1 mini MEMCPY_STATE requires argc=3, got {}", rec.argc),
                    );
                }
                let dst = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                let src = u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]);
                let n = u16::from_le_bytes([rec.args_raw[4], rec.args_raw[5]]);
                if n > S1_MINI_MEMCPY_STATE_MAX_N {
                    return bad(
                        i,
                        format!(
                            "s1 mini MEMCPY_STATE n={n} > max {}",
                            S1_MINI_MEMCPY_STATE_MAX_N
                        ),
                    );
                }
                if n > 0 {
                    let last_dst = dst.wrapping_add(n - 1);
                    let last_src = src.wrapping_add(n - 1);
                    if !mini_slot_disp8_ok(dst)
                        || !mini_slot_disp8_ok(src)
                        || !mini_slot_disp8_ok(last_dst)
                        || !mini_slot_disp8_ok(last_src)
                    {
                        return bad(i, "s1 mini MEMCPY_STATE slot disp32 unsupported".into());
                    }
                }
            }
            0x40 | 0x41 | 0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 | 0x76 | 0x77 | 0x78
            | 0x79 | 0x7A => {
                if rec.argc != 1 {
                    return bad(
                        i,
                        format!(
                            "s1 mini LABEL/CALL/JMP/JCC requires argc=1, got {}",
                            rec.argc
                        ),
                    );
                }
            }
            0xFF | 0x00 => {
                if rec.argc != 0 {
                    return bad(
                        i,
                        format!("s1 mini op {:02X} requires argc=0, got {}", rec.op, rec.argc),
                    );
                }
            }
            0xA0 => {
                if rec.argc != 1 {
                    return bad(
                        i,
                        format!("s1 mini RAW_BYTE requires argc=1, got {}", rec.argc),
                    );
                }
            }
            0xA1 => {
                if rec.argc > 3 {
                    return bad(
                        i,
                        format!("s1 mini RAW_BYTES argc={} > 3", rec.argc),
                    );
                }
            }
            op => {
                return bad(i, format!("s1 mini unsupported op {op:#04x}"));
            }
        }
    }
    Ok(())
}

/// S1.5 — host-side TYB → slim PE (parse + emit_x86 + wrap_pe_slim).
///
/// Full `emit_x86` (unknown→NOP). Prefer [`compile_tyb_to_pe_mini`] when
/// matching the in-DLL S1.5.b path (unsupported ops fail closed).
pub fn compile_tyb_to_pe(tyb: &[u8]) -> crate::types::IsaResult<crate::pe_link::PeImage> {
    let prog = parse_tyb(tyb).map_err(|e| crate::types::IsaError::ParseError {
        line: 0,
        msg: format!("s1 compile_tyb_to_pe: {e}"),
    })?;
    let out = emit_x86(&prog);
    wrap_pe_slim(&out)
}

/// S1.6 — host path aligned with in-DLL mini-ISA (fail closed on other ops).
///
/// S3.21: fixed-template subset (RET / NOP / SET-slot0-DEADBEEF) prefers
/// YOYO table-compose ([`try_compile_tyb_via_yoyo_table`]); otherwise `emit_x86`.
/// in-DLL dispatcher remains Rust. OW-RT CUT.
pub fn compile_tyb_to_pe_mini(tyb: &[u8]) -> crate::types::IsaResult<crate::pe_link::PeImage> {
    if let Some(pe) = try_compile_tyb_via_yoyo_table(tyb) {
        return Ok(pe);
    }
    let prog = parse_tyb(tyb).map_err(|e| crate::types::IsaError::ParseError {
        line: 0,
        msg: format!("s1 compile_tyb_to_pe_mini: {e}"),
    })?;
    validate_tyb_mini(&prog)?;
    let out = emit_x86(&prog);
    wrap_pe_slim(&out)
}

/// Extract user `.text` (post-startup) of length `text_len` from a slim PE.
pub fn slim_pe_user_text(pe: &[u8], text_len: usize) -> Option<&[u8]> {
    let end = S1_SLIM_TEXT_PAYLOAD_OFF.checked_add(text_len)?;
    if pe.len() < end {
        return None;
    }
    Some(&pe[S1_SLIM_TEXT_PAYLOAD_OFF..end])
}

/// Extract `.data` payload of length `data_len` from a slim PE.
pub fn slim_pe_user_data(pe: &[u8], data_len: usize) -> Option<&[u8]> {
    let end = S1_SLIM_DATA_PAYLOAD_OFF.checked_add(data_len)?;
    if pe.len() < end {
        return None;
    }
    Some(&pe[S1_SLIM_DATA_PAYLOAD_OFF..end])
}

/// Parse a .tyb file from disk (used by S1 gates / tests).
pub fn parse_tyb_file(path: &Path) -> Result<TybProgram, ParseError> {
    let data = match std::fs::read(path) {
        Ok(d) => d,
        Err(_) => return Err(ParseError::BadMagic), // simplified; caller should handle IO
    };
    parse_tyb(&data)
}

// =============================================================================
// S3.20 — host emit path via YOYO table-compose (S3.19 firmware, host sim)
// =============================================================================
//
// Supported: RET/NOP/SET/GET/INC/ADD(+5)/DEC/SUB(-5)/MOVRR/CMP(0,1).
// Does **not** replace in-DLL Rust dispatcher. OW-RT CUT.

/// S3.29: 10×20B rows; prog@0x148; outbuf **0x160** (DATA max 384).
const S3_HOST_OUTBUF_OFF: u32 = 0x160;
const S3_HOST_TABLE_OFF: u32 = 0x80;
const S3_HOST_NROWS: u32 = 10;
const S3_HOST_PROG_OFF: u32 = S3_HOST_TABLE_OFF + S3_HOST_NROWS * 20; // 0x148
const S3_HOST_SET_DEADBEEF: u32 = 0xDEAD_BEEF;
const S3_HOST_ADD_IMM: u32 = 5;
const S3_HOST_SUB_IMM: u32 = 5;

fn s3_host_tyb_from_records(recs: &[(u8, u8, [u8; 6])]) -> Vec<u8> {
    let mut v = Vec::with_capacity(HEADER_SIZE + recs.len() * RECORD_SIZE);
    v.extend_from_slice(&TYB_MAGIC);
    v.extend_from_slice(&[0, 0]);
    v.extend_from_slice(&(recs.len() as u16).to_le_bytes());
    for &(op, argc, a) in recs {
        v.push(op);
        v.push(argc);
        v.extend_from_slice(&a);
    }
    v
}

fn s3_host_pack_bytes(op: u8, bytes: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    let mut recs = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let n = (bytes.len() - i).min(3);
        let mut a = [0u8; 6];
        for k in 0..n {
            a[k * 2] = bytes[i + k];
        }
        recs.push((op, n as u8, a));
        i += n;
    }
    recs
}

fn s3_host_tmpl_row(op: u8, payload: &[u8]) -> [u8; 20] {
    let mut r = [0u8; 20];
    r[0] = op;
    r[1] = payload.len() as u8;
    r[4..4 + payload.len()].copy_from_slice(payload);
    r
}

fn s3_host_set_body() -> [u8; 14] {
    [
        0x48, 0xB8, 0xEF, 0xBE, 0xAD, 0xDE, 0, 0, 0, 0, 0x49, 0x89, 0x47, 0x00,
    ]
}

/// GET dst0←src1 body (no RET): load [r15+8]→rax; store [r15+0].
fn s3_host_get_body() -> [u8; 8] {
    [0x49, 0x8B, 0x47, 0x08, 0x49, 0x89, 0x47, 0x00]
}

/// INC slot0 body (no RET): load; inc rax; store.
fn s3_host_inc_body() -> [u8; 11] {
    [
        0x49, 0x8B, 0x47, 0x00, 0x48, 0xFF, 0xC0, 0x49, 0x89, 0x47, 0x00,
    ]
}

/// ADD slot0,+5 body (no RET): load; add rax,imm32; store.
fn s3_host_add5_body() -> [u8; 15] {
    [
        0x49, 0x8B, 0x47, 0x00, 0x48, 0x81, 0xC0, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47,
        0x00,
    ]
}

/// DEC slot0 body (no RET): load; dec rax; store.
fn s3_host_dec_body() -> [u8; 11] {
    [
        0x49, 0x8B, 0x47, 0x00, 0x48, 0xFF, 0xC8, 0x49, 0x89, 0x47, 0x00,
    ]
}

/// SUB slot0,-5 body (no RET): load; sub rax,imm32; store.
fn s3_host_sub5_body() -> [u8; 15] {
    [
        0x49, 0x8B, 0x47, 0x00, 0x48, 0x81, 0xE8, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47,
        0x00,
    ]
}

/// MOVRR dst1←src0 body (no RET): load [r15+0]; store [r15+8].
fn s3_host_movrr_body() -> [u8; 8] {
    [0x49, 0x8B, 0x47, 0x00, 0x49, 0x89, 0x47, 0x08]
}

/// CMP slot0,slot1 body (no RET): load a→rax; load b→rcx; cmp rax,rcx.
fn s3_host_cmp_body() -> [u8; 11] {
    [
        0x49, 0x8B, 0x47, 0x00, 0x49, 0x8B, 0x4F, 0x08, 0x48, 0x39, 0xC8,
    ]
}

/// Build program-loop firmware TYB for `ops`.
fn s3_host_prog_loop_tyb(ops: &[u8]) -> Vec<u8> {
    let mut payload = vec![0u8; S3_HOST_TABLE_OFF as usize];
    payload.extend_from_slice(&s3_host_tmpl_row(0xFF, &[0xC3]));
    payload.extend_from_slice(&s3_host_tmpl_row(0x00, &[0x90]));
    payload.extend_from_slice(&s3_host_tmpl_row(0x30, &s3_host_set_body()));
    payload.extend_from_slice(&s3_host_tmpl_row(0x60, &s3_host_get_body()));
    payload.extend_from_slice(&s3_host_tmpl_row(0x66, &s3_host_inc_body()));
    payload.extend_from_slice(&s3_host_tmpl_row(0x62, &s3_host_add5_body()));
    payload.extend_from_slice(&s3_host_tmpl_row(0x67, &s3_host_dec_body()));
    payload.extend_from_slice(&s3_host_tmpl_row(0x61, &s3_host_sub5_body()));
    payload.extend_from_slice(&s3_host_tmpl_row(0x64, &s3_host_movrr_body()));
    payload.extend_from_slice(&s3_host_tmpl_row(0x65, &s3_host_cmp_body()));
    debug_assert_eq!(payload.len(), S3_HOST_PROG_OFF as usize);
    payload.push(ops.len() as u8);
    payload.extend_from_slice(ops);
    while payload.len() < S3_HOST_OUTBUF_OFF as usize {
        payload.push(0);
    }
    assert!(
        payload.len() <= S1_MINI_DATA_MAX,
        "host table DATA {} > {}",
        payload.len(),
        S1_MINI_DATA_MAX
    );
    let mut recs = s3_host_pack_bytes(0x10, &payload);
    recs.extend(s3_host_pack_bytes(
        0xA1,
        &[
            0x49, 0x8D, 0x87, 0x80, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x18, 0x49, 0x89, 0x47,
            0x78,
        ],
    ));
    // lea rax,[r15+0x160]; store dst slot9
    recs.extend(s3_host_pack_bytes(
        0xA1,
        &[
            0x49, 0x8D, 0x87, 0x60, 0x01, 0x00, 0x00, 0x49, 0x89, 0x47, 0x48,
        ],
    ));
    let op0 = S3_HOST_PROG_OFF + 1;
    recs.extend(s3_host_pack_bytes(
        0xA1,
        &[
            0x49,
            0x8D,
            0x87,
            (op0 & 0xFF) as u8,
            ((op0 >> 8) & 0xFF) as u8,
            ((op0 >> 16) & 0xFF) as u8,
            ((op0 >> 24) & 0xFF) as u8,
            0x49,
            0x89,
            0x47,
            0x10,
        ],
    ));
    let slot = |s: u16| s.to_le_bytes();
    let set = |s: u16, imm: u32| {
        let sb = slot(s);
        let ib = imm.to_le_bytes();
        (0x30u8, 2u8, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
    };
    let add = |s: u16, imm: u32| {
        let sb = slot(s);
        let ib = imm.to_le_bytes();
        (0x62u8, 2u8, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
    };
    let p = slot(15);
    let o = slot(13);
    let n = slot(14);
    let i = slot(11);
    let nr = slot(10);
    let len = slot(12);
    let src = slot(8);
    let dst = slot(9);
    let pi = slot(0);
    let pn = slot(1);
    let pp = slot(2);
    let tb = slot(3);
    recs.push(set(1, ops.len() as u32));
    recs.push(set(0, 0));
    recs.push((0x40, 1, [5, 0, 0, 0, 0, 0]));
    recs.push((0x65, 2, [pi[0], pi[1], pn[0], pn[1], 0, 0]));
    recs.push((0x74, 1, [6, 0, 0, 0, 0, 0]));
    recs.push((0x80, 3, [n[0], n[1], pp[0], pp[1], 0, 0]));
    recs.push((0x64, 2, [p[0], p[1], tb[0], tb[1], 0, 0]));
    recs.push(set(11, 0));
    recs.push(set(10, S3_HOST_NROWS));
    recs.push((0x40, 1, [1, 0, 0, 0, 0, 0]));
    recs.push((0x80, 3, [o[0], o[1], p[0], p[1], 0, 0]));
    recs.push((0x65, 2, [o[0], o[1], n[0], n[1], 0, 0]));
    recs.push((0x71, 1, [2, 0, 0, 0, 0, 0]));
    recs.push(add(15, 20));
    recs.push(add(11, 1));
    recs.push((0x65, 2, [i[0], i[1], nr[0], nr[1], 0, 0]));
    recs.push((0x73, 1, [1, 0, 0, 0, 0, 0]));
    recs.push(set(12, 0xFE));
    recs.push((0xFF, 0, [0; 6]));
    recs.push((0x40, 1, [2, 0, 0, 0, 0, 0]));
    recs.push((0x80, 3, [len[0], len[1], p[0], p[1], 1, 0]));
    recs.push((0x64, 2, [src[0], src[1], p[0], p[1], 0, 0]));
    recs.push(add(8, 4));
    recs.push((0x84, 3, [dst[0], dst[1], src[0], src[1], len[0], len[1]]));
    recs.push((0x68, 2, [dst[0], dst[1], len[0], len[1], 0, 0]));
    recs.push(add(2, 1));
    recs.push(add(0, 1));
    recs.push((0x70, 1, [5, 0, 0, 0, 0, 0]));
    recs.push((0x40, 1, [6, 0, 0, 0, 0, 0]));
    recs.push((0xFF, 0, [0; 6]));
    s3_host_tyb_from_records(&recs)
}

fn s3_host_sim_prog_loop(tyb: &[u8]) -> Result<Vec<u8>, u8> {
    let prog = parse_tyb(tyb).map_err(|_| 0xFDu8)?;
    let mut data = vec![0u8; 512];
    let mut off = 0usize;
    for rec in &prog.records {
        if matches!(rec.op, 0x10 | 0x12 | 0x13) {
            let b = arg_low_bytes(rec);
            data[off..off + b.len()].copy_from_slice(&b);
            off += b.len();
        }
    }
    let mut state = [0u64; 16];
    state[3] = S3_HOST_TABLE_OFF as u64;
    state[15] = S3_HOST_TABLE_OFF as u64;
    state[9] = S3_HOST_OUTBUF_OFF as u64;
    state[2] = (S3_HOST_PROG_OFF + 1) as u64;
    let code: Vec<&TybRecord> = prog
        .records
        .iter()
        .filter(|r| !matches!(r.op, 0x10 | 0x12 | 0x13 | 0xA1 | 0xA0))
        .collect();
    let mut labels = std::collections::HashMap::new();
    for (i, rec) in code.iter().enumerate() {
        if rec.op == 0x40 {
            if let Some(id) = arg_u32(rec) {
                labels.insert(id, i);
            }
        }
    }
    let mut pc = 0usize;
    let mut zf = false;
    let mut sf = false;
    let mut composed = 0usize;
    let mut steps = 0usize;
    while pc < code.len() {
        steps += 1;
        if steps >= 50_000 {
            return Err(0xFC);
        }
        let rec = code[pc];
        match rec.op {
            0x40 => pc += 1,
            0x30 => {
                let (s, imm) = arg_slot_imm(rec).ok_or(0xFB)?;
                state[s as usize] = imm as u64;
                pc += 1;
            }
            0x62 => {
                let (s, imm) = arg_slot_imm(rec).ok_or(0xFB)?;
                state[s as usize] = state[s as usize].wrapping_add(imm as u64);
                pc += 1;
            }
            0x64 => {
                let (d, s) = arg_slot_slot(rec).ok_or(0xFB)?;
                state[d as usize] = state[s as usize];
                pc += 1;
            }
            0x68 => {
                let (d, s) = arg_slot_slot(rec).ok_or(0xFB)?;
                state[d as usize] = state[d as usize].wrapping_add(state[s as usize]);
                pc += 1;
            }
            0x80 => {
                let (dd, ss, oo) = arg_slot3(rec).ok_or(0xFB)?;
                let addr = state[ss as usize] as usize + oo as usize;
                state[dd as usize] = *data.get(addr).ok_or(0xFA)? as u64;
                pc += 1;
            }
            0x84 => {
                let (dst, src, n) = arg_slot3(rec).ok_or(0xFB)?;
                let d = state[dst as usize] as usize;
                let s = state[src as usize] as usize;
                let n = state[n as usize] as usize;
                let tmp = data.get(s..s + n).ok_or(0xFA)?.to_vec();
                data.get_mut(d..d + n).ok_or(0xFA)?.copy_from_slice(&tmp);
                composed += n;
                pc += 1;
            }
            0x65 => {
                let (a, b) = arg_slot_slot(rec).ok_or(0xFB)?;
                let d = (state[a as usize] as i64).wrapping_sub(state[b as usize] as i64);
                zf = d == 0;
                sf = d < 0;
                pc += 1;
            }
            0x70 => {
                pc = *labels.get(&arg_u32(rec).ok_or(0xFB)?).ok_or(0xF9)?;
            }
            0x71 => {
                let id = arg_u32(rec).ok_or(0xFB)?;
                if zf {
                    pc = *labels.get(&id).ok_or(0xF9)?;
                } else {
                    pc += 1;
                }
            }
            0x73 => {
                let id = arg_u32(rec).ok_or(0xFB)?;
                if sf {
                    pc = *labels.get(&id).ok_or(0xF9)?;
                } else {
                    pc += 1;
                }
            }
            0x74 => {
                let id = arg_u32(rec).ok_or(0xFB)?;
                if !sf {
                    pc = *labels.get(&id).ok_or(0xF9)?;
                } else {
                    pc += 1;
                }
            }
            0xFF => {
                if composed == 0 {
                    return Err(state[12] as u8);
                }
                break;
            }
            _ => return Err(0xF8),
        }
    }
    if composed == 0 {
        return Err(0xFE);
    }
    let base = S3_HOST_OUTBUF_OFF as usize;
    Ok(data[base..base + composed].to_vec())
}

/// If `prog` is only fixed-template ops, return that op sequence.
pub fn fixed_template_ops(prog: &TybProgram) -> Option<Vec<u8>> {
    let mut ops = Vec::new();
    for rec in &prog.records {
        match rec.op {
            0xFF | 0x00 => ops.push(rec.op),
            0x30 => {
                let (slot, imm) = arg_slot_imm(rec)?;
                if slot != 0 || imm != S3_HOST_SET_DEADBEEF {
                    return None;
                }
                ops.push(0x30);
            }
            0x60 => {
                let (dst, src) = arg_slot_slot(rec)?;
                if dst != 0 || src != 1 {
                    return None;
                }
                ops.push(0x60);
            }
            0x66 => {
                let slot = arg_u32(rec)? as u16;
                if slot != 0 {
                    return None;
                }
                ops.push(0x66);
            }
            0x62 => {
                let (slot, imm) = arg_slot_imm(rec)?;
                if slot != 0 || imm != S3_HOST_ADD_IMM {
                    return None;
                }
                ops.push(0x62);
            }
            0x67 => {
                let slot = arg_u32(rec)? as u16;
                if slot != 0 {
                    return None;
                }
                ops.push(0x67);
            }
            0x61 => {
                let (slot, imm) = arg_slot_imm(rec)?;
                if slot != 0 || imm != S3_HOST_SUB_IMM {
                    return None;
                }
                ops.push(0x61);
            }
            0x64 => {
                let (dst, src) = arg_slot_slot(rec)?;
                if dst != 1 || src != 0 {
                    return None;
                }
                ops.push(0x64);
            }
            0x65 => {
                let (a, b) = arg_slot_slot(rec)?;
                if a != 0 || b != 1 {
                    return None;
                }
                ops.push(0x65);
            }
            _ => return None,
        }
    }
    if ops.is_empty() {
        None
    } else {
        Some(ops)
    }
}

/// Host-side YOYO table compose → `.text` bytes.
/// `None` if `ops` outside the fixed template subset.
pub fn try_emit_text_via_yoyo_table(ops: &[u8]) -> Option<Vec<u8>> {
    if ops.is_empty() || ops.len() > 8 {
        return None;
    }
    if !ops.iter().all(|o| {
        matches!(
            o,
            0xFF | 0x00 | 0x30 | 0x60 | 0x66 | 0x62 | 0x67 | 0x61 | 0x64 | 0x65
        )
    }) {
        return None;
    }
    s3_host_sim_prog_loop(&s3_host_prog_loop_tyb(ops)).ok()
}

/// Host PE path: fixed-template TYB → slim PE via YOYO table compose (not `emit_x86`).
/// `None` if program is outside the subset (caller should use [`compile_tyb_to_pe_mini`]).
pub fn try_compile_tyb_via_yoyo_table(tyb: &[u8]) -> Option<crate::pe_link::PeImage> {
    let prog = parse_tyb(tyb).ok()?;
    validate_tyb_mini(&prog).ok()?;
    let ops = fixed_template_ops(&prog)?;
    let text = try_emit_text_via_yoyo_table(&ops)?;
    wrap_pe_slim(&CodegenOutput {
        text,
        data: Vec::new(),
    })
    .ok()
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
        // SET slot=0, imm=0 → movabs rax,imm64 (10B) + mov [r15], rax (4B) + ret (1B) = 15B
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x30, argc: 2, args_raw: [0x00, 0x00, 0x00, 0x00, 0x00, 0x00] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(
            out.text,
            vec![
                0x48, 0xB8, 0, 0, 0, 0, 0, 0, 0, 0, // movabs rax, 0
                0x49, 0x89, 0x47, 0x00,             // mov [r15+0], rax
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
                0x48, 0xB8,                                    // movabs rax, imm64
                0xEF, 0xBE, 0xAD, 0xDE, 0, 0, 0, 0,            // 0xdeadbeef zero-extended
                0x49, 0x89, 0x47, 0x40,                        // mov [r15+64], rax
                0xC3,                                          // ret
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
        // Unknown opcode 0xFE → 1 NOP + trailing ret
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0xFE, argc: 0, args_raw: [0; 6] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text, vec![0x90, 0xC3]);
    }

    #[test]
    fn emit_x86_sequence_set_then_ret() {
        // SET slot=0, imm=0x01000000 ; RET
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
        assert_eq!(out.text[0], 0x48);
        assert_eq!(out.text[1], 0xB8);
        assert_eq!(&out.text[2..10], &[0x00, 0x00, 0x00, 0x01, 0, 0, 0, 0]);
        assert_eq!(&out.text[10..14], &[0x49, 0x89, 0x47, 0x00]);
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
        assert_eq!(&out.text[4..8], &[0x49, 0x89, 0x47, 0x08]);
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
                0x49, 0x8B, 0x47, 0x08,   // load_state_r15 slot=1 → rax
                0x49, 0x8B, 0x4F, 0x10,   // load_state_rcx slot=2 → rcx (MODRM 0x48)
                0x48, 0x01, 0xC8,          // add rax, rcx
                0x49, 0x89, 0x47, 0x08,   // store_state_r15 slot=1 ← rax
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
                0x49, 0x8B, 0x47, 0x00,
                0x49, 0x8B, 0x4F, 0x08,   // MODRM 0x48 (rcx), disp=8
                0x48, 0x09, 0xC8,          // or rax, rcx
                0x49, 0x89, 0x47, 0x00,
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
                0x49, 0x8B, 0x47, 0x10,
                0x49, 0x8B, 0x4F, 0x18,
                0x48, 0x29, 0xC8,          // sub rax, rcx
                0x49, 0x89, 0x47, 0x10,
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
                0x49, 0x8B, 0x47, 0x08,
                0x49, 0x8B, 0x4F, 0x10,
                0x48, 0x0F, 0xAF, 0xC1,    // imul rax, rcx
                0x49, 0x89, 0x47, 0x08,
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
                0x49, 0x8B, 0x47, 0x08,
                0x49, 0x8B, 0x4F, 0x10,
                0x48, 0x39, 0xC8,          // cmp rax, rcx (MODRM 0xC0: reg=rcx, r/m=rax)
                0xC3,
            ]
        );
    }

    // ============= S1.1.f: two-pass label resolution tests =============

    #[test]
    fn emit_x86_label_only_noop() {
        // LABEL 0x1234 alone → no x86 emitted, only trailing ret.
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord { op: 0x40, argc: 1, args_raw: [0x34, 0x12, 0, 0, 0, 0] }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text, vec![0xC3]);
    }

    #[test]
    fn emit_x86_jmp_forward_to_ret() {
        // Program: NOP | LABEL 1 | JMP 1
        //   NOP @0..1 (1B)
        //   LABEL 1 → anchored at offset 1 (no bytes)
        //   JMP 1   @1..6 (E9 + rel32)
        //     next_ip = 1+5 = 6; target = 1; rel = 1-6 = -5 = 0xFFFFFFFB
        //   trailing ret @6..7
        // Total = 7B
        let prog = TybProgram {
            rec_cnt: 3,
            records: vec![
                TybRecord { op: 0x00, argc: 0, args_raw: [0; 6] }, // NOP
                TybRecord { op: 0x40, argc: 1, args_raw: [0x01, 0x00, 0, 0, 0, 0] }, // LABEL 1
                TybRecord { op: 0x70, argc: 1, args_raw: [0x01, 0x00, 0, 0, 0, 0] }, // JMP 1
            ],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 7);
        assert_eq!(&out.text, &[
            0x90,                       // NOP
            0xE9,                       // JMP
            0xFB, 0xFF, 0xFF, 0xFF,      // rel32 = -5 LE
            0xC3,                       // trailing ret
        ]);
    }

    #[test]
    fn emit_x86_jmp_backward_to_loop_head() {
        // Program: LABEL 1 | INC slot=0 | JMP 1
        //   LABEL 1 @0 → anchored at 0 (no bytes)
        //   INC slot=0 @0..11 (load 4B + inc 3B + store 4B)
        //   JMP 1 @11..16 (E9 + rel32)
        //     next_ip = 11+5 = 16; target = 0; rel = 0-16 = -16 = 0xFFFFFFF0
        //   trailing ret @16..17
        // Total = 17B
        let prog = TybProgram {
            rec_cnt: 3,
            records: vec![
                TybRecord { op: 0x40, argc: 1, args_raw: [0x01, 0x00, 0, 0, 0, 0] }, // LABEL 1
                TybRecord { op: 0x66, argc: 1, args_raw: [0x00, 0x00, 0, 0, 0, 0] }, // INC slot=0
                TybRecord { op: 0x70, argc: 1, args_raw: [0x01, 0x00, 0, 0, 0, 0] }, // JMP 1
            ],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 17);
        // Verify the JMP instruction starts at byte 11
        assert_eq!(out.text[11], 0xE9);
        // Verify rel32 = -16 (LE)
        assert_eq!(&out.text[12..16], &[0xF0, 0xFF, 0xFF, 0xFF]);
        // Verify trailing ret at 16
        assert_eq!(out.text[16], 0xC3);
    }

    #[test]
    fn emit_x86_jcc_all_ten_conditions_undefined_label() {
        // Emit one JCC hh for each of 0x71..=0x7A with argc=1, label_id=0.
        // No LABEL record → rel32 stays 0 for all (undefined-label default).
        // Each JCC is 6B (0F 8x + rel32). Total = 60B + trailing ret (1B) = 61B.
        let prog = TybProgram {
            rec_cnt: 10,
            records: vec![
                TybRecord { op: 0x71, argc: 1, args_raw: [0; 6] },
                TybRecord { op: 0x72, argc: 1, args_raw: [0; 6] },
                TybRecord { op: 0x73, argc: 1, args_raw: [0; 6] },
                TybRecord { op: 0x74, argc: 1, args_raw: [0; 6] },
                TybRecord { op: 0x75, argc: 1, args_raw: [0; 6] },
                TybRecord { op: 0x76, argc: 1, args_raw: [0; 6] },
                TybRecord { op: 0x77, argc: 1, args_raw: [0; 6] },
                TybRecord { op: 0x78, argc: 1, args_raw: [0; 6] },
                TybRecord { op: 0x79, argc: 1, args_raw: [0; 6] },
                TybRecord { op: 0x7A, argc: 1, args_raw: [0; 6] },
            ],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 61);
        // Verify each JCC's second byte matches JCC_VALID order:
        //   0x71=je=0x84, 0x72=jne=0x85, 0x73=jl=0x8C, 0x74=jge=0x8D, 0x75=jle=0x8E,
        //   0x76=jg=0x8F, 0x77=jb=0x82, 0x78=jae=0x83, 0x79=jbe=0x86, 0x7A=ja=0x87
        let expected_ccs: [u8; 10] = [0x84, 0x85, 0x8C, 0x8D, 0x8E, 0x8F, 0x82, 0x83, 0x86, 0x87];
        for i in 0..10 {
            let base = i * 6;
            assert_eq!(out.text[base], 0x0F, "JCC {} first byte", i);
            assert_eq!(out.text[base + 1], expected_ccs[i], "JCC {} second byte", i);
            // rel32 undefined → 0
            assert_eq!(&out.text[base + 2..base + 6], &[0, 0, 0, 0]);
        }
        assert_eq!(out.text[60], 0xC3);
    }

    #[test]
    fn emit_x86_call_forward_to_label() {
        // Program: LABEL 1 | CALL 1 | RET
        //   LABEL 1 @0 → anchored at 0 (no bytes)
        //   CALL 1 @0..5 (E8 + rel32)
        //     next_ip = 0+5 = 5; target = 0; rel = -5 = 0xFFFFFFFB
        //   RET @5..6
        // Total = 6B
        let prog = TybProgram {
            rec_cnt: 3,
            records: vec![
                TybRecord { op: 0x40, argc: 1, args_raw: [0x01, 0x00, 0, 0, 0, 0] }, // LABEL 1
                TybRecord { op: 0x41, argc: 1, args_raw: [0x01, 0x00, 0, 0, 0, 0] }, // CALL 1
                TybRecord { op: 0xFF, argc: 0, args_raw: [0; 6] }, // RET
            ],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 6);
        assert_eq!(&out.text, &[
            0xE8,                       // CALL
            0xFB, 0xFF, 0xFF, 0xFF,     // rel32 = -5
            0xC3,                       // RET (explicit)
        ]);
    }

    #[test]
    fn emit_x86_jcc_backward_conditional_loop() {
        // Program: LABEL 1 | SUB slot=0 imm=1 | JNE 1 (0x72)
        //   LABEL 1 @0 → anchored at 0 (no bytes)
        //   SUB slot=0 imm=1 @0..15 (load 4B + sub imm 7B + store 4B)
        //     emit_sub_rax_imm32 = REX.W 81 E0 + imm32 = 3+4 = 7B (not 6B)
        //   JNE 1 @15..21 (0F 85 + rel32)
        //     next_ip = 15+6 = 21; target = 0; rel = 0-21 = -21 = 0xFFFFFFEB
        //   trailing ret @21
        // Total = 22B
        let prog = TybProgram {
            rec_cnt: 3,
            records: vec![
                TybRecord { op: 0x40, argc: 1, args_raw: [0x01, 0x00, 0, 0, 0, 0] }, // LABEL 1
                TybRecord { op: 0x61, argc: 2, args_raw: [0x00, 0x00, 0x01, 0x00, 0, 0] }, // SUB slot=0 imm=1
                TybRecord { op: 0x72, argc: 1, args_raw: [0x01, 0x00, 0, 0, 0, 0] }, // JNE 1
            ],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 22);
        // Verify JNE at offset 15
        assert_eq!(out.text[15], 0x0F);
        assert_eq!(out.text[16], 0x85); // jne
        // Verify rel32 = -21 (LE)
        assert_eq!(&out.text[17..21], &[0xEB, 0xFF, 0xFF, 0xFF]);
        // Verify trailing ret
        assert_eq!(out.text[21], 0xC3);
    }

    #[test]
    fn emit_x86_jmp_to_undefined_label_leaves_rel32_zero() {
        // JMP to a label that doesn't exist → rel32 stays 0.
        // Program: JMP 99 (no LABEL 99) | RET
        //   JMP @0..5: E9 00 00 00 00
        //   RET @5..6
        let prog = TybProgram {
            rec_cnt: 2,
            records: vec![
                TybRecord { op: 0x70, argc: 1, args_raw: [0x63, 0x00, 0, 0, 0, 0] }, // JMP 99
                TybRecord { op: 0xFF, argc: 0, args_raw: [0; 6] }, // RET
            ],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text.len(), 6);
        assert_eq!(&out.text, &[
            0xE9,                       // JMP
            0x00, 0x00, 0x00, 0x00,     // rel32 = 0 (undefined label)
            0xC3,                       // RET
        ]);
    }

    // ============= S1.2: register allocation + GET arg semantics =============

    #[test]
    fn arg_slot_slot_reads_low16_of_second_field() {
        // Wire: argc=2 packs second field as u32. High 16 bits of that u32
        // must be ignored for slot-slot ops (GET etc).
        // dst=0x0007, second u32 = 0xABCD0009 → src slot = 0x0009 (low 16)
        let rec = TybRecord {
            op: 0x60,
            argc: 2,
            args_raw: [0x07, 0x00, 0x09, 0x00, 0xCD, 0xAB],
        };
        assert_eq!(arg_slot_slot(&rec), Some((7, 9)));
        // Contrast: arg_slot_imm would see imm = 0xABCD0009
        assert_eq!(arg_slot_imm(&rec), Some((7, 0xABCD0009)));
    }

    #[test]
    fn emit_x86_get_uses_slot_slot_not_imm_truncation() {
        // GET dst=3, src=9 with garbage in high bytes of second field.
        // If we wrongly used the full u32 as slot, disp would be huge / wrong.
        // Correct: src=9 → disp8 = 72 = 0x48
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0x60,
                argc: 2,
                args_raw: [0x03, 0x00, 0x09, 0x00, 0xFF, 0xFF],
            }],
        };
        let out = emit_x86(&prog);
        assert_eq!(
            &out.text,
            &[
                0x49, 0x8B, 0x47, 0x48, // load [r15+72] → rax (src=9)
                0x49, 0x89, 0x47, 0x18, // store [r15+24] ← rax (dst=3)
                0xC3,
            ]
        );
    }

    #[test]
    fn emit_load_state_rdx_encoding() {
        // Tertiary scratch Rdx: rex=0x49 (B=1 for r15), ModRM reg=rdx(010) → 0x57
        // 49 8B 57 <disp8>
        let mut out = Vec::new();
        emit_load_state(&mut out, 1, Reg::Rdx);
        assert_eq!(&out, &[0x49, 0x8B, 0x57, 0x08]);
    }

    #[test]
    fn emit_store_state_rdx_encoding() {
        let mut out = Vec::new();
        emit_store_state(&mut out, 2, Reg::Rdx);
        assert_eq!(&out, &[0x49, 0x89, 0x57, 0x10]);
    }

    #[test]
    fn emit_sub_imm_uses_modrm_slash5() {
        // Was wrongly E0 (/4=AND) before S1.2; must be E8 (/5=SUB).
        let mut out = Vec::new();
        emit_sub_imm(&mut out, Reg::Rax, 1);
        assert_eq!(&out, &[0x48, 0x81, 0xE8, 0x01, 0x00, 0x00, 0x00]);
    }

    // ============= S1.3: LDB / MEMCPY / RAW_BYTE =============

    #[test]
    fn emit_x86_ldb_oo0() {
        // LDB dd=1 ss=2 oo=0:
        //   49 8B 47 10   load [r15+16] → rax
        //   48 0F B6 00   movzx rax, byte [rax]
        //   49 89 47 08   store [r15+8] ← rax
        //   C3
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0x80,
                argc: 3,
                args_raw: [0x01, 0x00, 0x02, 0x00, 0x00, 0x00],
            }],
        };
        let out = emit_x86(&prog);
        assert_eq!(
            &out.text,
            &[
                0x49, 0x8B, 0x47, 0x10,
                0x48, 0x0F, 0xB6, 0x00,
                0x49, 0x89, 0x47, 0x08,
                0xC3,
            ]
        );
    }

    #[test]
    fn emit_x86_ldb_with_offset() {
        // LDB dd=0 ss=1 oo=5 → load + add imm32 5 + movzx + store
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0x80,
                argc: 3,
                args_raw: [0x00, 0x00, 0x01, 0x00, 0x05, 0x00],
            }],
        };
        let out = emit_x86(&prog);
        assert_eq!(
            &out.text,
            &[
                0x49, 0x8B, 0x47, 0x08,                         // load ss=1
                0x48, 0x81, 0xC0, 0x05, 0x00, 0x00, 0x00,       // add rax, 5
                0x48, 0x0F, 0xB6, 0x00,                         // movzx
                0x49, 0x89, 0x47, 0x00,                         // store dd=0
                0xC3,
            ]
        );
    }

    #[test]
    fn emit_x86_memcpy_data_pointer_form() {
        // MEMCPY_DATA dst=1 src=2 n=3:
        //   load src→rsi, dst→rdi, n→rcx, then F3 A4
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0x84,
                argc: 3,
                args_raw: [0x01, 0x00, 0x02, 0x00, 0x03, 0x00],
            }],
        };
        let out = emit_x86(&prog);
        assert_eq!(
            &out.text,
            &[
                0x49, 0x8B, 0x77, 0x10, // mov rsi, [r15+16]  (src=2)
                0x49, 0x8B, 0x7F, 0x08, // mov rdi, [r15+8]   (dst=1)
                0x49, 0x8B, 0x4F, 0x18, // mov rcx, [r15+24]  (n=3)
                0xF3, 0xA4,             // rep movsb (correct; not assembler.rs's 0xFC)
                0xC3,
            ]
        );
    }

    #[test]
    fn emit_x86_memcpy_state_two_slots() {
        // MEMCPY_STATE dst=4 src=1 n=2 → two load/store pairs
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0x85,
                argc: 3,
                args_raw: [0x04, 0x00, 0x01, 0x00, 0x02, 0x00],
            }],
        };
        let out = emit_x86(&prog);
        assert_eq!(
            &out.text,
            &[
                // i=0: src=1 → dst=4
                0x49, 0x8B, 0x47, 0x08,
                0x49, 0x89, 0x47, 0x20,
                // i=1: src=2 → dst=5
                0x49, 0x8B, 0x47, 0x10,
                0x49, 0x89, 0x47, 0x28,
                0xC3,
            ]
        );
    }

    #[test]
    fn emit_x86_memcpy_state_n_over_64_is_empty() {
        // n=65 → no memcpy bytes, only trailing ret
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0x85,
                argc: 3,
                args_raw: [0x00, 0x00, 0x00, 0x00, 0x41, 0x00], // n=65
            }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text, vec![0xC3]);
    }

    #[test]
    fn emit_x86_raw_byte() {
        // RAW_BYTE 0x90 → emit 0x90 + trailing ret (since last != C3)
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0xA0,
                argc: 1,
                args_raw: [0x90, 0x00, 0x00, 0x00, 0, 0],
            }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text, vec![0x90, 0xC3]);
    }

    #[test]
    fn slim_io_blob_sizes_and_patches() {
        let load = emit_slim_load_file(0, 0);
        let write = emit_slim_write_file(0, 0, 0);
        // LOAD: push/pop (+4) + FILE_SHARE_READ (+3) + clear nRead (+9)
        // WRITE: push/pop (+4) + mov r8d,0 vs xor (+3) + clear nWritten (+9)
        assert_eq!(load.len(), 160, "load blob len");
        assert_eq!(write.len(), 139, "write blob len");
        assert_eq!(slim_load_path_patch_off(), 9);
        assert_eq!(slim_load_slot_patch_off(&load), load.len() - 6);
        assert_eq!(slim_write_path_patch_off(), 9);
        assert_eq!(&load[9..13], &[0, 0, 0, 0]);
        assert_eq!(&write[9..13], &[0, 0, 0, 0]);
        assert_eq!(&load[slim_load_slot_patch_off(&load)..][..4], &[0, 0, 0, 0]);
    }

    #[test]
    fn emit_x86_alloc_slim_iat() {
        // S3.94: VirtualAlloc via slim IAT + disp32 store
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0x20,
                argc: 2,
                args_raw: [0x01, 0x00, 0x10, 0x00, 0x00, 0x00],
            }],
        };
        let out = emit_x86(&prog);
        let expect = emit_slim_alloc(1, 0x10);
        assert_eq!(&out.text[..expect.len()], expect.as_slice());
        assert_eq!(*out.text.last().unwrap(), 0xC3);
        assert!(out.data.is_empty());
    }

    #[test]
    fn compile_tyb_to_pe_mini_accepts_alloc_stub() {
        let tyb = tyb_from_records(&[(0x20, 2, [1, 0, 0x10, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(compile_tyb_to_pe_mini(&tyb).is_ok());
    }

    #[test]
    fn emit_x86_load_file_slim_iat() {
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0x50,
                argc: 2,
                args_raw: [0x00, 0x00, 0x08, 0x00, 0x00, 0x00], // path_off=8
            }],
        };
        let out = emit_x86(&prog);
        let expect = emit_slim_load_file(0, 8);
        assert_eq!(&out.text[..expect.len()], expect.as_slice());
        assert_eq!(*out.text.last().unwrap(), 0xC3);
    }

    #[test]
    fn emit_x86_write_file_slim_iat() {
        // argc=3: slot=1 path_off=2 sz_slot=3
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0x51,
                argc: 3,
                args_raw: [0x01, 0x00, 0x02, 0x00, 0x03, 0x00],
            }],
        };
        let out = emit_x86(&prog);
        let expect = emit_slim_write_file(1, 2, 3);
        assert_eq!(&out.text[..expect.len()], expect.as_slice());
        assert_eq!(*out.text.last().unwrap(), 0xC3);
    }

    #[test]
    fn compile_tyb_to_pe_mini_accepts_load_file_stub() {
        let tyb = tyb_from_records(&[(0x50, 2, [0, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(compile_tyb_to_pe_mini(&tyb).is_ok());
    }

    // ============= S1.4: DATA/STR/RAW / RAW_BYTES / wrap_pe =============

    #[test]
    fn emit_x86_data_goes_to_data_section() {
        // DATA argc=3 bytes HI,'i',0 → .data = [0x48, 0x69, 0x00]; .text = ret only
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0x10,
                argc: 3,
                args_raw: [0x48, 0x00, 0x69, 0x00, 0x00, 0x00],
            }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.data, vec![0x48, 0x69, 0x00]);
        assert_eq!(out.text, vec![0xC3]);
    }

    #[test]
    fn emit_x86_str_and_raw_append_data() {
        // STR "AB" (argc=2) + RAW 0xFF (argc=1) → .data = [0x41, 0x42, 0xFF]
        let prog = TybProgram {
            rec_cnt: 2,
            records: vec![
                TybRecord {
                    op: 0x12,
                    argc: 2,
                    args_raw: [0x41, 0x00, 0x42, 0x00, 0x00, 0x00],
                },
                TybRecord {
                    op: 0x13,
                    argc: 1,
                    args_raw: [0xFF, 0x00, 0x00, 0x00, 0, 0],
                },
            ],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.data, vec![0x41, 0x42, 0xFF]);
        assert_eq!(out.text, vec![0xC3]);
    }

    #[test]
    fn emit_x86_raw_bytes_to_text() {
        // RAW_BYTES 0xCC 0xDD (argc=2) → .text = cc dd c3 (matches self_test H_33 pin)
        let prog = TybProgram {
            rec_cnt: 1,
            records: vec![TybRecord {
                op: 0xA1,
                argc: 2,
                args_raw: [0xCC, 0x00, 0xDD, 0x00, 0x00, 0x00],
            }],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.text, vec![0xCC, 0xDD, 0xC3]);
        assert!(out.data.is_empty());
    }

    #[test]
    fn wrap_pe_produces_mz_pe() {
        let prog = TybProgram {
            rec_cnt: 2,
            records: vec![
                TybRecord {
                    op: 0x10,
                    argc: 2,
                    args_raw: [0x59, 0x00, 0x4F, 0x00, 0x00, 0x00], // 'Y','O'
                },
                TybRecord {
                    op: 0xFF,
                    argc: 0,
                    args_raw: [0; 6],
                },
            ],
        };
        let out = emit_x86(&prog);
        assert_eq!(out.data, vec![0x59, 0x4F]);
        assert_eq!(out.text, vec![0xC3]);
        let pe = wrap_pe(&out).expect("link_pe");
        assert!(pe.bytes.len() > 0x200, "PE should be non-trivial");
        assert_eq!(&pe.bytes[0..2], b"MZ");
        // PE signature at e_lfanew
        let e_lfanew = u32::from_le_bytes(pe.bytes[0x3C..0x40].try_into().unwrap()) as usize;
        assert_eq!(&pe.bytes[e_lfanew..e_lfanew + 4], b"PE\0\0");
    }

    // ============= S1.5: slim PE + compile_tyb_to_pe =============

    #[test]
    fn wrap_pe_slim_is_small_and_mz() {
        let out = emit_x86(&TybProgram {
            rec_cnt: 0,
            records: vec![],
        });
        let pe = wrap_pe_slim(&out).expect("slim");
        assert!(
            pe.bytes.len() < 0x3000,
            "slim PE should be ≪ H_00 pad, got {}",
            pe.bytes.len()
        );
        assert_eq!(&pe.bytes[0..2], b"MZ");
        let e_lfanew =
            u32::from_le_bytes(pe.bytes[0x3C..0x40].try_into().unwrap()) as usize;
        assert_eq!(&pe.bytes[e_lfanew..e_lfanew + 4], b"PE\0\0");
        // User code (ret) sits at fixed payload offset.
        assert_eq!(pe.bytes[S1_SLIM_TEXT_PAYLOAD_OFF], 0xC3);
    }

    #[test]
    fn compile_tyb_to_pe_set_ret() {
        // TYB: SET slot0=0xDEADBEEF ; RET
        let mut tyb = Vec::new();
        tyb.extend_from_slice(&TYB_MAGIC);
        tyb.extend_from_slice(&[0, 0]); // reserved
        tyb.extend_from_slice(&2u16.to_le_bytes());
        // SET
        tyb.push(0x30);
        tyb.push(2);
        tyb.extend_from_slice(&[0x00, 0x00, 0xEF, 0xBE, 0xAD, 0xDE]);
        // RET
        tyb.push(0xFF);
        tyb.push(0);
        tyb.extend_from_slice(&[0u8; 6]);

        let pe = compile_tyb_to_pe(&tyb).expect("compile");
        assert_eq!(&pe.bytes[0..2], b"MZ");
        assert!(pe.bytes.len() < 0x3000);
        // movabs rax, imm64 starts with 48 B8
        assert_eq!(&pe.bytes[S1_SLIM_TEXT_PAYLOAD_OFF..S1_SLIM_TEXT_PAYLOAD_OFF + 2], &[
            0x48, 0xB8
        ]);
    }

    #[test]
    fn compile_tyb_to_pe_rejects_bad_magic() {
        match compile_tyb_to_pe(b"XXXX") {
            Ok(_) => panic!("bad magic must fail"),
            Err(err) => {
                let msg = format!("{err}");
                assert!(
                    msg.contains("s1 compile_tyb_to_pe")
                        || msg.contains("magic")
                        || msg.contains("TYB")
                        || msg.contains("short"),
                    "unexpected err: {msg}"
                );
            }
        }
    }

    // ============= S1.6: mini-ISA + parity helpers =============

    fn tyb_from_records(recs: &[(u8, u8, [u8; 6])]) -> Vec<u8> {
        let mut v = header(recs.len() as u16);
        for &(op, argc, a) in recs {
            v.extend_from_slice(&record(op, argc, a));
        }
        v
    }

    #[test]
    fn compile_tyb_to_pe_mini_accepts_imul() {
        let tyb = tyb_from_records(&[(0x63, 2, [1, 0, 2, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(compile_tyb_to_pe_mini(&tyb).is_ok());
    }

    #[test]
    fn compile_tyb_to_pe_mini_accepts_ldb() {
        let tyb = tyb_from_records(&[(0x80, 3, [1, 0, 2, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(compile_tyb_to_pe_mini(&tyb).is_ok());
    }

    #[test]
    fn compile_tyb_to_pe_mini_rejects_write_file_argc3() {
        // Mini stub form is argc=2 only; host emit still accepts argc=3.
        let tyb = tyb_from_records(&[(0x51, 3, [1, 0, 2, 0, 99, 0]), (0xFF, 0, [0; 6])]);
        assert!(compile_tyb_to_pe(&tyb).is_ok());
        assert!(compile_tyb_to_pe_mini(&tyb).is_err());
    }

    #[test]
    fn compile_tyb_to_pe_mini_rejects_memcpy_state_over_cap() {
        let tyb = tyb_from_records(&[(0x85, 3, [0, 0, 0, 0, 5, 0])]); // n=5 > 4
        assert!(compile_tyb_to_pe_mini(&tyb).is_err());
    }

    #[test]
    fn compile_tyb_to_pe_mini_accepts_data_str() {
        let tyb = tyb_from_records(&[
            (0x10, 3, [0x48, 0, 0x69, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("mini data");
        let data = slim_pe_user_data(&pe.bytes, 3).expect("data slice");
        assert_eq!(data, &[0x48, 0x69, 0x00]);
    }

    #[test]
    fn compile_tyb_to_pe_mini_rejects_data_over_cap() {
        // S3.25: 129 × argc=3 = 387 > 384
        let mut recs = vec![(0x10u8, 3u8, [1u8, 0, 2, 0, 3, 0]); 129];
        recs.push((0xFF, 0, [0; 6]));
        let tyb = tyb_from_records(&recs);
        assert!(compile_tyb_to_pe_mini(&tyb).is_err());
        // 128 × 3 = 384 OK under 384
        let mut ok = vec![(0x10u8, 3u8, [1u8, 0, 2, 0, 3, 0]); 128];
        ok.push((0xFF, 0, [0; 6]));
        assert!(compile_tyb_to_pe_mini(&tyb_from_records(&ok)).is_ok());
    }

    #[test]
    fn compile_tyb_to_pe_mini_accepts_raw_bytes() {
        let tyb = tyb_from_records(&[(0xA1, 2, [0xCC, 0, 0xDD, 0, 0, 0])]);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("mini raw_bytes");
        let text = slim_pe_user_text(&pe.bytes, 3).expect("text");
        assert_eq!(text, &[0xCC, 0xDD, 0xC3]);
    }

    #[test]
    fn compile_tyb_to_pe_mini_accepts_get() {
        let tyb = tyb_from_records(&[(0x60, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(compile_tyb_to_pe_mini(&tyb).is_ok());
    }

    #[test]
    fn compile_tyb_to_pe_mini_set_ret_text() {
        let tyb = tyb_from_records(&[
            (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
            (0xFF, 0, [0; 6]),
        ]);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("mini");
        let host = emit_x86(&parse_tyb(&tyb).unwrap());
        let got = slim_pe_user_text(&pe.bytes, host.text.len()).expect("slice");
        assert_eq!(got, host.text.as_slice());
    }

    /// S3.78: mini accepts SET slot≥16 (disp32).
    #[test]
    fn compile_tyb_to_pe_mini_set_slot32_text() {
        let tyb = tyb_from_records(&[
            (0x30, 2, [0x10, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
            (0xFF, 0, [0; 6]),
        ]);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("mini slot16");
        let host = emit_x86(&parse_tyb(&tyb).unwrap());
        let got = slim_pe_user_text(&pe.bytes, host.text.len()).expect("slice");
        assert_eq!(got, host.text.as_slice());
        assert_eq!(
            &host.text[..17],
            &[
                0x48, 0xB8, 0xEF, 0xBE, 0xAD, 0xDE, 0, 0, 0, 0, 0x49, 0x89, 0x87, 0x80, 0, 0, 0
            ]
        );
    }

    #[test]
    fn compile_tyb_to_pe_mini_matrix_text() {
        let cases: Vec<(&str, Vec<(u8, u8, [u8; 6])>)> = vec![
            ("empty", vec![]),
            ("ret", vec![(0xFF, 0, [0; 6])]),
            ("nop_ret", vec![(0x00, 0, [0; 6]), (0xFF, 0, [0; 6])]),
            (
                "label_ret",
                vec![(0x40, 1, [1, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
            (
                "raw_ret",
                vec![(0xA0, 1, [0x90, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
            (
                "set_ret",
                vec![
                    (0x30, 2, [0x08, 0, 0x11, 0x22, 0x33, 0x44]),
                    (0xFF, 0, [0; 6]),
                ],
            ),
            (
                "get_ret",
                vec![(0x60, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
            (
                "add_ret",
                vec![(0x62, 2, [0, 0, 5, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
            (
                "inc_ret",
                vec![(0x66, 1, [0, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
        ];
        for (name, recs) in cases {
            let tyb = tyb_from_records(&recs);
            let pe = compile_tyb_to_pe_mini(&tyb).unwrap_or_else(|e| panic!("{name}: {e}"));
            let host = emit_x86(&parse_tyb(&tyb).unwrap());
            let got = slim_pe_user_text(&pe.bytes, host.text.len()).unwrap();
            assert_eq!(got, host.text.as_slice(), "{name} user .text");
        }
    }

    // ============= S2.5: runtime-effect stub firmware + acceptance =============

    fn s2_runtime_effect_stub_tyb_path() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../yoyo/tests/golden/s2_runtime_effect_stub.tyb")
    }

    #[test]
    fn s2_runtime_effect_firmware_mini_compiles() {
        let path = s2_runtime_effect_stub_tyb_path();
        assert!(path.exists(), "missing firmware {}", path.display());
        let tyb = std::fs::read(&path).expect("read s2 firmware tyb");
        assert!(is_tyb(&tyb), "firmware must be TYB\\0");
        let prog = parse_tyb(&tyb).expect("parse firmware");
        assert_eq!(prog.rec_cnt, 6, "STR×2 + ALLOC + LOAD + WRITE + RET");
        assert_eq!(prog.records[0].op, 0x12);
        assert_eq!(prog.records[2].op, 0x20);
        assert_eq!(prog.records[3].op, 0x50);
        assert_eq!(prog.records[4].op, 0x51);
        assert_eq!(prog.records[5].op, 0xFF);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("mini must accept stub firmware");
        let host = emit_x86(&prog);
        let got = slim_pe_user_text(&pe.bytes, host.text.len()).expect("user text");
        assert_eq!(got, host.text.as_slice(), "mini PE user .text == emit_x86");
        // .data = 'i','n',0,'o','u','t'
        let data = slim_pe_user_data(&pe.bytes, 6).expect("user data");
        assert_eq!(data, b"in\0out");
    }

    #[test]
    fn s2_runtime_effect_firmware_full_host_compiles() {
        let tyb = std::fs::read(s2_runtime_effect_stub_tyb_path()).expect("read");
        assert!(compile_tyb_to_pe(&tyb).is_ok());
    }

    // ============= S3.2: minimal meta-emit SET+RET (RAW_BYTES template) =============

    fn s3_meta_emit_set_ret_tyb_path() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../yoyo/tests/golden/s3_meta_emit_set_ret.tyb")
    }

    #[test]
    fn s3_meta_emit_set_ret_matches_emit_x86() {
        // Canonical SET+RET program (slot0 = 0xDEADBEEF).
        let set_ret = tyb_from_records(&[
            (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
            (0xFF, 0, [0; 6]),
        ]);
        let expect = emit_x86(&parse_tyb(&set_ret).unwrap()).text;

        let path = s3_meta_emit_set_ret_tyb_path();
        assert!(path.exists(), "missing {}", path.display());
        let meta = std::fs::read(&path).expect("read meta-emit tyb");
        let pe = compile_tyb_to_pe_mini(&meta).expect("mini meta-emit");
        let got = slim_pe_user_text(&pe.bytes, expect.len()).expect("user text");
        assert_eq!(
            got, expect.as_slice(),
            "S3.2 meta-emit RAW_BYTES must be byte-identical to emit_x86(SET+RET)"
        );
        // Also: host emit of the meta TYB itself equals the same bytes.
        let meta_host = emit_x86(&parse_tyb(&meta).unwrap());
        assert_eq!(meta_host.text, expect);
    }

    // ============= S3.3: meta-emit matrix + soft-cap raise deferred =============

    #[test]
    fn s3_meta_emit_matrix_matches_emit_x86() {
        // Soft-cap raise deferred: in-DLL scratch is hard-wired to 256B
        // (`pe_dll_link` r13 end = r12+256). Expand template coverage inside caps.
        let golden = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../yoyo/tests/golden");
        let cases: Vec<(&str, Vec<(u8, u8, [u8; 6])>)> = vec![
            ("s3_meta_emit_ret.tyb", vec![(0xFF, 0, [0; 6])]),
            (
                "s3_meta_emit_nop_ret.tyb",
                vec![(0x00, 0, [0; 6]), (0xFF, 0, [0; 6])],
            ),
            (
                "s3_meta_emit_set_ret.tyb",
                vec![
                    (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
                    (0xFF, 0, [0; 6]),
                ],
            ),
        ];
        for (file, semantic) in cases {
            let expect = emit_x86(&parse_tyb(&tyb_from_records(&semantic)).unwrap()).text;
            let meta = std::fs::read(golden.join(file)).unwrap_or_else(|e| panic!("{file}: {e}"));
            let pe = compile_tyb_to_pe_mini(&meta).unwrap_or_else(|e| panic!("{file} mini: {e}"));
            let got = slim_pe_user_text(&pe.bytes, expect.len()).expect("slice");
            assert_eq!(got, expect.as_slice(), "{file} vs emit_x86");
        }
    }

    #[test]
    fn s3_soft_caps_after_s313() {
        // S3.7 code=512; S3.25 DATA=384.
        assert_eq!(S1_SLIM_MAX_CODE, 512);
        assert_eq!(S1_MINI_DATA_MAX, 384);
    }

    // ============= S3.4: auto-pack RAW_BYTES meta-emit from emit_x86 =============

    /// Pack machine bytes as mini-legal RAW_BYTES records (argc≤3).
    fn pack_meta_emit_raw_bytes_tyb(machine: &[u8]) -> Vec<u8> {
        let mut recs: Vec<(u8, u8, [u8; 6])> = Vec::new();
        let mut i = 0;
        while i < machine.len() {
            let n = (machine.len() - i).min(3);
            let mut a = [0u8; 6];
            for k in 0..n {
                a[k * 2] = machine[i + k];
            }
            recs.push((0xA1, n as u8, a));
            i += n;
        }
        tyb_from_records(&recs)
    }

    #[test]
    fn s3_meta_emit_autopack_get_ret() {
        let semantic = tyb_from_records(&[
            (0x60, 2, [0, 0, 1, 0, 0, 0]), // GET dst0←src1
            (0xFF, 0, [0; 6]),
        ]);
        let expect = emit_x86(&parse_tyb(&semantic).unwrap()).text;
        let meta = pack_meta_emit_raw_bytes_tyb(&expect);
        let pe = compile_tyb_to_pe_mini(&meta).expect("autopack mini");
        let got = slim_pe_user_text(&pe.bytes, expect.len()).expect("slice");
        assert_eq!(got, expect.as_slice());
        // Round-trip: emit of packed meta == original semantic emit.
        assert_eq!(emit_x86(&parse_tyb(&meta).unwrap()).text, expect);
    }

    #[test]
    fn s3_meta_emit_autopack_matches_hand_goldens() {
        // Autopack must regenerate S3.2/S3.3 hand goldens byte-for-byte.
        let golden = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../yoyo/tests/golden");
        let cases: Vec<(&str, Vec<(u8, u8, [u8; 6])>)> = vec![
            ("s3_meta_emit_ret.tyb", vec![(0xFF, 0, [0; 6])]),
            (
                "s3_meta_emit_nop_ret.tyb",
                vec![(0x00, 0, [0; 6]), (0xFF, 0, [0; 6])],
            ),
            (
                "s3_meta_emit_set_ret.tyb",
                vec![
                    (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
                    (0xFF, 0, [0; 6]),
                ],
            ),
        ];
        for (file, semantic) in cases {
            let expect = emit_x86(&parse_tyb(&tyb_from_records(&semantic)).unwrap()).text;
            let packed = pack_meta_emit_raw_bytes_tyb(&expect);
            let hand = std::fs::read(golden.join(file)).unwrap();
            assert_eq!(packed, hand, "{file} autopack == hand golden");
        }
    }

    // ============= S3.8: DATA-resident opcode template table =============

    #[derive(Debug, Clone)]
    struct OpcodeTemplateRow {
        op: u8,
        bytes: Vec<u8>,
    }

    /// S3.9: fixed 20-byte rows — op, len, pad_u16, bytes[16].
    const S3_TMPL_ROW: usize = 20;
    const S3_TMPL_BYTES: usize = 16;

    fn parse_opcode_template_table(data: &[u8]) -> Vec<OpcodeTemplateRow> {
        assert_eq!(data.len() % S3_TMPL_ROW, 0, "table must be {S3_TMPL_ROW}-byte rows");
        data.chunks_exact(S3_TMPL_ROW)
            .map(|row| {
                let op = row[0];
                let len = row[1] as usize;
                assert!(len <= S3_TMPL_BYTES, "template len {len} > {S3_TMPL_BYTES}");
                OpcodeTemplateRow {
                    op,
                    bytes: row[4..4 + len].to_vec(),
                }
            })
            .collect()
    }

    fn lookup_template<'a>(table: &'a [OpcodeTemplateRow], op: u8) -> Option<&'a [u8]> {
        table.iter().find(|r| r.op == op).map(|r| r.bytes.as_slice())
    }

    #[test]
    fn s3_opcode_template_table_data_lookup() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../yoyo/tests/golden/s3_opcode_template_table.tyb");
        let tyb = std::fs::read(&path).expect("read table tyb");
        let pe = compile_tyb_to_pe_mini(&tyb).expect("mini table");
        let data = slim_pe_user_data(&pe.bytes, 200).expect("200B table");
        let table = parse_opcode_template_table(data);
        assert_eq!(table.len(), 10);
        assert_eq!(lookup_template(&table, 0xFF), Some(&[0xC3][..]));
        assert_eq!(lookup_template(&table, 0x00), Some(&[0x90][..]));

        let compose = |ops: &[u8]| -> Vec<u8> {
            let mut out = Vec::new();
            for &op in ops {
                out.extend_from_slice(lookup_template(&table, op).unwrap_or_else(|| panic!("missing {op:#x}")));
            }
            out
        };
        let cases: Vec<(&str, Vec<(u8, u8, [u8; 6])>, Vec<u8>)> = vec![
            (
                "nop_ret",
                vec![(0x00, 0, [0; 6]), (0xFF, 0, [0; 6])],
                compose(&[0x00, 0xFF]),
            ),
            (
                "set_ret",
                vec![
                    (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
                    (0xFF, 0, [0; 6]),
                ],
                compose(&[0x30, 0xFF]),
            ),
            (
                "get_ret",
                vec![(0x60, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])],
                compose(&[0x60, 0xFF]),
            ),
            (
                "inc_ret",
                vec![(0x66, 1, [0, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])],
                compose(&[0x66, 0xFF]),
            ),
            (
                "add_ret",
                vec![(0x62, 2, [0, 0, 5, 0, 0, 0]), (0xFF, 0, [0; 6])],
                compose(&[0x62, 0xFF]),
            ),
            (
                "sub_ret",
                vec![(0x61, 2, [0, 0, 5, 0, 0, 0]), (0xFF, 0, [0; 6])],
                compose(&[0x61, 0xFF]),
            ),
            (
                "dec_ret",
                vec![(0x67, 1, [0, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])],
                compose(&[0x67, 0xFF]),
            ),
            (
                "movrr_ret",
                vec![(0x64, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])],
                compose(&[0x64, 0xFF]),
            ),
            (
                "cmp_ret",
                vec![(0x65, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])],
                compose(&[0x65, 0xFF]),
            ),
        ];
        for (name, semantic, from_table) in cases {
            let expect = emit_x86(&parse_tyb(&tyb_from_records(&semantic)).unwrap()).text;
            assert_eq!(from_table, expect, "{name}");
        }
    }

    // ============= S3.16: YOYO table-scan emit (LDB/CMP/JE over DATA) =============

    const S3_SCAN_PTR: u16 = 15; // state[15] = offset into .data (sim) / abs ptr (PE)
    const S3_SCAN_NEEDLE: u16 = 14;
    const S3_SCAN_OP: u16 = 13;
    const S3_SCAN_OUT: u16 = 12;
    const S3_SCAN_I: u16 = 11;
    const S3_SCAN_NROWS: u16 = 10;

    fn pack_data_recs(bytes: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
        let mut recs = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let n = (bytes.len() - i).min(3);
            let mut a = [0u8; 6];
            for k in 0..n {
                a[k * 2] = bytes[i + k];
            }
            recs.push((0x10, n as u8, a));
            i += n;
        }
        recs
    }

    fn tmpl_row(op: u8, payload: &[u8]) -> [u8; 20] {
        let mut r = [0u8; 20];
        r[0] = op;
        r[1] = payload.len() as u8;
        r[4..4 + payload.len()].copy_from_slice(payload);
        r
    }

    /// TYB: 2-row DATA table (RET, NOP) + scan loop → state[OUT]=first template byte.
    fn s3_table_scan_tyb(needle: u8) -> Vec<u8> {
        let mut table = Vec::new();
        table.extend_from_slice(&tmpl_row(0xFF, &[0xC3]));
        table.extend_from_slice(&tmpl_row(0x00, &[0x90]));
        let mut recs = pack_data_recs(&table);
        // lea rax,[r15]; mov [r15+15*8],rax — PE runtime ptr seed (sim seeds offset 0)
        recs.push((0xA1, 3, [0x49, 0, 0x8D, 0, 0x07, 0]));
        recs.push((0xA1, 3, [0x49, 0, 0x89, 0, 0x47, 0]));
        recs.push((0xA1, 1, [0x78, 0, 0, 0, 0, 0])); // disp8=0x78 = slot15*8
        let slot = |s: u16| s.to_le_bytes();
        let set = |s: u16, imm: u32| {
            let sb = slot(s);
            let ib = imm.to_le_bytes();
            (0x30u8, 2u8, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
        };
        let add = |s: u16, imm: u32| {
            let sb = slot(s);
            let ib = imm.to_le_bytes();
            (0x62u8, 2u8, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
        };
        let p = slot(S3_SCAN_PTR);
        let o = slot(S3_SCAN_OP);
        let n = slot(S3_SCAN_NEEDLE);
        let i = slot(S3_SCAN_I);
        let nr = slot(S3_SCAN_NROWS);
        let out = slot(S3_SCAN_OUT);
        recs.push(set(S3_SCAN_NEEDLE, needle as u32));
        recs.push(set(S3_SCAN_I, 0));
        recs.push(set(S3_SCAN_NROWS, 2));
        recs.push((0x40, 1, [1, 0, 0, 0, 0, 0])); // LABEL loop
        recs.push((0x80, 3, [o[0], o[1], p[0], p[1], 0, 0])); // LDB op,[ptr]
        recs.push((0x65, 2, [o[0], o[1], n[0], n[1], 0, 0])); // CMP op,needle
        recs.push((0x71, 1, [2, 0, 0, 0, 0, 0])); // JE found
        recs.push(add(S3_SCAN_PTR, 20));
        recs.push(add(S3_SCAN_I, 1));
        recs.push((0x65, 2, [i[0], i[1], nr[0], nr[1], 0, 0])); // CMP i,nrows
        recs.push((0x73, 1, [1, 0, 0, 0, 0, 0])); // JL loop
        recs.push(set(S3_SCAN_OUT, 0xFE)); // miss
        recs.push((0xFF, 0, [0; 6]));
        recs.push((0x40, 1, [2, 0, 0, 0, 0, 0])); // LABEL found
        recs.push((0x80, 3, [out[0], out[1], p[0], p[1], 4, 0])); // LDB out,[ptr+4]
        recs.push((0xFF, 0, [0; 6]));
        tyb_from_records(&recs)
    }

    /// Host sim: DATA buffer + state[0..16]; ptr slots hold **offsets** into DATA.
    /// RAW_BYTES/A1 ignored (PE lea); sim seeds ptr=0.
    fn s3_sim_table_scan(tyb: &[u8]) -> u64 {
        let prog = parse_tyb(tyb).expect("parse");
        let mut data = Vec::new();
        for rec in &prog.records {
            if matches!(rec.op, 0x10 | 0x12 | 0x13) {
                data.extend(arg_low_bytes(rec));
            }
        }
        let mut state = [0u64; 16];
        state[S3_SCAN_PTR as usize] = 0; // .data base offset
        // Collect code records (skip DATA and RAW_BYTES)
        let code: Vec<&TybRecord> = prog
            .records
            .iter()
            .filter(|r| !matches!(r.op, 0x10 | 0x12 | 0x13 | 0xA1 | 0xA0))
            .collect();
        // Map label_id → index in code
        let mut labels = std::collections::HashMap::new();
        for (i, rec) in code.iter().enumerate() {
            if rec.op == 0x40 {
                if let Some(id) = arg_u32(rec) {
                    labels.insert(id, i);
                }
            }
        }
        let mut pc = 0usize;
        let mut zf = false;
        let mut sf = false;
        let mut steps = 0usize;
        while pc < code.len() {
            steps += 1;
            assert!(steps < 10_000, "sim runaway");
            let rec = code[pc];
            match rec.op {
                0x40 => {
                    pc += 1;
                } // LABEL
                0x30 => {
                    let (s, imm) = arg_slot_imm(rec).unwrap();
                    state[s as usize] = imm as u64;
                    pc += 1;
                }
                0x62 => {
                    let (s, imm) = arg_slot_imm(rec).unwrap();
                    state[s as usize] = state[s as usize].wrapping_add(imm as u64);
                    pc += 1;
                }
                0x80 => {
                    let (dd, ss, oo) = arg_slot3(rec).unwrap();
                    let addr = state[ss as usize] as usize + oo as usize;
                    state[dd as usize] = data[addr] as u64;
                    pc += 1;
                }
                0x65 => {
                    let (a, b) = arg_slot_slot(rec).unwrap();
                    let va = state[a as usize] as i64;
                    let vb = state[b as usize] as i64;
                    let d = va.wrapping_sub(vb);
                    zf = d == 0;
                    sf = d < 0;
                    pc += 1;
                }
                0x71 => {
                    // JE
                    let id = arg_u32(rec).unwrap();
                    if zf {
                        pc = labels[&id];
                    } else {
                        pc += 1;
                    }
                }
                0x73 => {
                    // JL (signed): SF != OF; for our non-neg small ints, SF means <
                    let id = arg_u32(rec).unwrap();
                    if sf {
                        pc = labels[&id];
                    } else {
                        pc += 1;
                    }
                }
                0xFF => break,
                other => panic!("s3 sim unsupported op {other:#x}"),
            }
        }
        state[S3_SCAN_OUT as usize]
    }

    #[test]
    fn s3_yoyo_table_scan_compiles_mini() {
        let tyb = s3_table_scan_tyb(0xFF);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("S3.16 scan firmware mini");
        let host = emit_x86(&parse_tyb(&tyb).unwrap());
        // lea rax,[r15] present
        assert!(
            host.text.windows(3).any(|w| w == [0x49, 0x8D, 0x07]),
            "expected lea rax,[r15] seed"
        );
        let data = slim_pe_user_data(&pe.bytes, 40).expect("2 rows");
        assert_eq!(&data[0..2], &[0xFF, 0x01]);
        assert_eq!(data[4], 0xC3);
    }

    #[test]
    fn s3_yoyo_table_scan_sim_ret_and_nop() {
        // YOYO IR scan produces template first byte ≡ emit_x86 targets.
        assert_eq!(s3_sim_table_scan(&s3_table_scan_tyb(0xFF)), 0xC3);
        assert_eq!(s3_sim_table_scan(&s3_table_scan_tyb(0x00)), 0x90);
        assert_eq!(s3_sim_table_scan(&s3_table_scan_tyb(0xAB)), 0xFE); // miss
        // Match emit_x86(RET) / emit_x86(NOP;RET) first bytes
        let ret_b = emit_x86(&parse_tyb(&tyb_from_records(&[(0xFF, 0, [0; 6])])).unwrap()).text;
        assert_eq!(ret_b, vec![0xC3]);
        let nop_b = emit_x86(&parse_tyb(&tyb_from_records(&[
            (0x00, 0, [0; 6]),
            (0xFF, 0, [0; 6]),
        ]))
        .unwrap())
        .text;
        assert_eq!(nop_b[0], 0x90);
        // Golden fixture (needle=RET) matches builder
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../yoyo/tests/golden/s3_yoyo_table_scan.tyb");
        let golden = std::fs::read(&path).expect("golden scan tyb");
        assert_eq!(golden, s3_table_scan_tyb(0xFF));
        assert_eq!(s3_sim_table_scan(&golden), 0xC3);
    }

    // ============= S3.17: multi-byte compose + Windows PE run =============

    const S3_COMPOSE_SRC: u16 = 8;
    const S3_COMPOSE_DST: u16 = 9;
    /// .data offset for composed template bytes (beyond table + slot area).
    const S3_OUTBUF_OFF: u32 = 0xC8; // 200

    fn set_body_deadbeef() -> [u8; 14] {
        [
            0x48, 0xB8, 0xEF, 0xBE, 0xAD, 0xDE, 0, 0, 0, 0, 0x49, 0x89, 0x47, 0x00,
        ]
    }

    fn pack_raw_recs(bytes: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
        let mut recs = Vec::new();
        let mut i = 0;
        while i < bytes.len() {
            let n = (bytes.len() - i).min(3);
            let mut a = [0u8; 6];
            for k in 0..n {
                a[k * 2] = bytes[i + k];
            }
            recs.push((0xA1, n as u8, a));
            i += n;
        }
        recs
    }

    /// S3.17: 3-row table (RET/NOP/SET) + scan + MEMCPY_DATA → outbuf@.data+0xC8.
    fn s3_table_compose_tyb(needle: u8) -> Vec<u8> {
        let mut table = Vec::new();
        table.extend_from_slice(&tmpl_row(0xFF, &[0xC3]));
        table.extend_from_slice(&tmpl_row(0x00, &[0x90]));
        table.extend_from_slice(&tmpl_row(0x30, &set_body_deadbeef()));
        let mut recs = pack_data_recs(&table);
        // lea rax,[r15]; mov [r15+15*8],rax  — scan ptr
        recs.extend(pack_raw_recs(&[
            0x49, 0x8D, 0x07, 0x49, 0x89, 0x47, 0x78,
        ]));
        // lea rax,[r15+0xC8]; mov [r15+9*8],rax  — compose dst
        recs.extend(pack_raw_recs(&[
            0x49, 0x8D, 0x87, 0xC8, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x48,
        ]));

        let slot = |s: u16| s.to_le_bytes();
        let set = |s: u16, imm: u32| {
            let sb = slot(s);
            let ib = imm.to_le_bytes();
            (0x30u8, 2u8, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
        };
        let add = |s: u16, imm: u32| {
            let sb = slot(s);
            let ib = imm.to_le_bytes();
            (0x62u8, 2u8, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
        };
        let p = slot(S3_SCAN_PTR);
        let o = slot(S3_SCAN_OP);
        let n = slot(S3_SCAN_NEEDLE);
        let i = slot(S3_SCAN_I);
        let nr = slot(S3_SCAN_NROWS);
        let len = slot(S3_SCAN_OUT); // reuse OUT as len after found
        let src = slot(S3_COMPOSE_SRC);
        let dst = slot(S3_COMPOSE_DST);

        recs.push(set(S3_SCAN_NEEDLE, needle as u32));
        recs.push(set(S3_SCAN_I, 0));
        recs.push(set(S3_SCAN_NROWS, 3));
        recs.push((0x40, 1, [1, 0, 0, 0, 0, 0])); // loop
        recs.push((0x80, 3, [o[0], o[1], p[0], p[1], 0, 0]));
        recs.push((0x65, 2, [o[0], o[1], n[0], n[1], 0, 0]));
        recs.push((0x71, 1, [2, 0, 0, 0, 0, 0])); // JE found
        recs.push(add(S3_SCAN_PTR, 20));
        recs.push(add(S3_SCAN_I, 1));
        recs.push((0x65, 2, [i[0], i[1], nr[0], nr[1], 0, 0]));
        recs.push((0x73, 1, [1, 0, 0, 0, 0, 0])); // JL loop
        recs.push(set(S3_SCAN_OUT, 0xFE)); // miss status
        recs.push((0xFF, 0, [0; 6]));
        recs.push((0x40, 1, [2, 0, 0, 0, 0, 0])); // found
        recs.push((0x80, 3, [len[0], len[1], p[0], p[1], 1, 0])); // LDB len
        recs.push((0x64, 2, [src[0], src[1], p[0], p[1], 0, 0])); // MOVRR src,ptr
        recs.push(add(S3_COMPOSE_SRC, 4)); // skip op/len/pad → bytes
        recs.push((0x84, 3, [dst[0], dst[1], src[0], src[1], len[0], len[1]]));
        recs.push((0xFF, 0, [0; 6]));
        tyb_from_records(&recs)
    }

    /// Sim compose: returns Ok(payload) or Err(miss_code).
    fn s3_sim_table_compose(tyb: &[u8]) -> Result<Vec<u8>, u8> {
        let prog = parse_tyb(tyb).expect("parse");
        let mut data = vec![0u8; 512];
        let mut off = 0usize;
        for rec in &prog.records {
            if matches!(rec.op, 0x10 | 0x12 | 0x13) {
                let b = arg_low_bytes(rec);
                data[off..off + b.len()].copy_from_slice(&b);
                off += b.len();
            }
        }
        let mut state = [0u64; 16];
        state[S3_SCAN_PTR as usize] = 0;
        state[S3_COMPOSE_DST as usize] = S3_OUTBUF_OFF as u64;
        let code: Vec<&TybRecord> = prog
            .records
            .iter()
            .filter(|r| !matches!(r.op, 0x10 | 0x12 | 0x13 | 0xA1 | 0xA0))
            .collect();
        let mut labels = std::collections::HashMap::new();
        for (i, rec) in code.iter().enumerate() {
            if rec.op == 0x40 {
                if let Some(id) = arg_u32(rec) {
                    labels.insert(id, i);
                }
            }
        }
        let mut pc = 0usize;
        let mut zf = false;
        let mut sf = false;
        let mut hit = false;
        let mut steps = 0usize;
        while pc < code.len() {
            steps += 1;
            assert!(steps < 10_000, "sim runaway");
            let rec = code[pc];
            match rec.op {
                0x40 => pc += 1,
                0x30 => {
                    let (s, imm) = arg_slot_imm(rec).unwrap();
                    state[s as usize] = imm as u64;
                    pc += 1;
                }
                0x62 => {
                    let (s, imm) = arg_slot_imm(rec).unwrap();
                    state[s as usize] = state[s as usize].wrapping_add(imm as u64);
                    pc += 1;
                }
                0x64 => {
                    let (d, s) = arg_slot_slot(rec).unwrap();
                    state[d as usize] = state[s as usize];
                    pc += 1;
                }
                0x80 => {
                    let (dd, ss, oo) = arg_slot3(rec).unwrap();
                    let addr = state[ss as usize] as usize + oo as usize;
                    state[dd as usize] = data[addr] as u64;
                    pc += 1;
                }
                0x84 => {
                    let (dst, src, n) = arg_slot3(rec).unwrap();
                    let d = state[dst as usize] as usize;
                    let s = state[src as usize] as usize;
                    let n = state[n as usize] as usize;
                    let tmp = data[s..s + n].to_vec();
                    data[d..d + n].copy_from_slice(&tmp);
                    hit = true;
                    pc += 1;
                }
                0x65 => {
                    let (a, b) = arg_slot_slot(rec).unwrap();
                    let d = (state[a as usize] as i64).wrapping_sub(state[b as usize] as i64);
                    zf = d == 0;
                    sf = d < 0;
                    pc += 1;
                }
                0x71 => {
                    let id = arg_u32(rec).unwrap();
                    if zf {
                        pc = labels[&id];
                    } else {
                        pc += 1;
                    }
                }
                0x73 => {
                    let id = arg_u32(rec).unwrap();
                    if sf {
                        pc = labels[&id];
                    } else {
                        pc += 1;
                    }
                }
                0xFF => break,
                other => panic!("compose sim unsupported {other:#x}"),
            }
        }
        if !hit {
            return Err(state[S3_SCAN_OUT as usize] as u8);
        }
        let n = state[S3_SCAN_OUT as usize] as usize;
        let base = S3_OUTBUF_OFF as usize;
        Ok(data[base..base + n].to_vec())
    }

    #[test]
    fn s3_yoyo_table_compose_sim_multibyte() {
        let ret = s3_sim_table_compose(&s3_table_compose_tyb(0xFF)).expect("ret");
        assert_eq!(ret, vec![0xC3]);
        let nop = s3_sim_table_compose(&s3_table_compose_tyb(0x00)).expect("nop");
        assert_eq!(nop, vec![0x90]);
        let set = s3_sim_table_compose(&s3_table_compose_tyb(0x30)).expect("set");
        assert_eq!(set.as_slice(), set_body_deadbeef().as_slice());
        // full SET+RET ≡ emit_x86
        let mut composed = set;
        composed.push(0xC3);
        let expect = emit_x86(&parse_tyb(&tyb_from_records(&[
            (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
            (0xFF, 0, [0; 6]),
        ]))
        .unwrap())
        .text;
        assert_eq!(composed, expect);
        assert_eq!(s3_sim_table_compose(&s3_table_compose_tyb(0xAB)), Err(0xFE));
        assert!(compile_tyb_to_pe_mini(&s3_table_compose_tyb(0x30)).is_ok());
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../yoyo/tests/golden/s3_yoyo_table_compose.tyb");
        let golden = std::fs::read(&path).expect("golden compose tyb");
        assert_eq!(golden, s3_table_compose_tyb(0xFF));
        assert_eq!(s3_sim_table_compose(&golden).unwrap(), vec![0xC3]);
    }

    /// Map slim PE RWX, call entry, read composed outbuf (Windows).
    #[cfg(windows)]
    fn s3_run_slim_pe_outbuf(pe: &[u8], len: usize) -> Vec<u8> {
        use crate::pe_manual_map::{map_pe_sections, parse_pe64_headers};
        #[link(name = "kernel32")]
        extern "system" {
            fn VirtualAlloc(
                lpAddress: *mut std::ffi::c_void,
                dwSize: usize,
                flAllocationType: u32,
                flProtect: u32,
            ) -> *mut std::ffi::c_void;
            fn VirtualFree(lpAddress: *mut std::ffi::c_void, dwSize: usize, dwFreeType: u32)
                -> i32;
        }
        const MEM_COMMIT: u32 = 0x1000;
        const MEM_RESERVE: u32 = 0x2000;
        const MEM_RELEASE: u32 = 0x8000;
        const PAGE_EXECUTE_READWRITE: u32 = 0x40;

        let headers = parse_pe64_headers(pe).expect("headers");
        let staging = map_pe_sections(pe, &headers).expect("map");
        // data RVA = .data VirtualAddress
        let pe_off = u32::from_le_bytes(pe[0x3C..0x40].try_into().unwrap()) as usize;
        let num_sec = u16::from_le_bytes(pe[pe_off + 6..pe_off + 8].try_into().unwrap()) as usize;
        let opt_size = u16::from_le_bytes(pe[pe_off + 20..pe_off + 22].try_into().unwrap()) as usize;
        let mut data_rva = 0u32;
        for i in 0..num_sec {
            let sh = pe_off + 24 + opt_size + i * 40;
            if &pe[sh..sh + 5] == b".data" {
                data_rva = u32::from_le_bytes(pe[sh + 12..sh + 16].try_into().unwrap());
                break;
            }
        }
        assert_ne!(data_rva, 0, ".data section");

        let size = headers.size_of_image as usize;
        let base = unsafe {
            VirtualAlloc(
                std::ptr::null_mut(),
                size,
                MEM_COMMIT | MEM_RESERVE,
                PAGE_EXECUTE_READWRITE,
            )
        };
        assert!(!base.is_null(), "VirtualAlloc");
        unsafe {
            std::ptr::copy_nonoverlapping(staging.as_ptr(), base as *mut u8, staging.len().min(size));
            let entry = (base as usize + headers.entry_rva as usize) as *const ();
            let f: extern "C" fn() = std::mem::transmute(entry);
            f();
            let out = std::slice::from_raw_parts(
                (base as *const u8).add(data_rva as usize + S3_OUTBUF_OFF as usize),
                len,
            )
            .to_vec();
            VirtualFree(base, 0, MEM_RELEASE);
            out
        }
    }

    #[test]
    #[cfg(windows)]
    fn s3_yoyo_table_compose_runs_on_pe() {
        let tyb = s3_table_compose_tyb(0x30);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("mini");
        let got = s3_run_slim_pe_outbuf(&pe.bytes, 14);
        assert_eq!(got.as_slice(), set_body_deadbeef().as_slice());
        let tyb_ret = s3_table_compose_tyb(0xFF);
        let pe_ret = compile_tyb_to_pe_mini(&tyb_ret).expect("mini ret");
        let got_ret = s3_run_slim_pe_outbuf(&pe_ret.bytes, 1);
        assert_eq!(got_ret, vec![0xC3]);
    }

    /// S3.99: slim PE IAT binds via manual_map + host GetProcAddress; ALLOC runs.
    #[test]
    #[cfg(windows)]
    fn s3_slim_pe_manual_map_binds_iat_and_runs_alloc() {
        use crate::pe_manual_map::{
            collect_resolved_imports, manual_map_pe_dll, manual_map_pe_dll_executable,
            parse_pe64_headers,
        };
        use std::ffi::CString;

        #[link(name = "kernel32")]
        extern "system" {
            fn LoadLibraryA(name: *const i8) -> *mut std::ffi::c_void;
            fn GetProcAddress(
                module: *mut std::ffi::c_void,
                name: *const i8,
            ) -> *mut std::ffi::c_void;
            fn VirtualFree(lpAddress: *mut std::ffi::c_void, dwSize: usize, dwFreeType: u32)
                -> i32;
        }
        const MEM_RELEASE: u32 = 0x8000;

        fn host_resolve(dll: &str, name: &str) -> Option<u64> {
            let dll_c = CString::new(dll).ok()?;
            let name_c = CString::new(name).ok()?;
            unsafe {
                let module = LoadLibraryA(dll_c.as_ptr());
                if module.is_null() {
                    return None;
                }
                let proc = GetProcAddress(module, name_c.as_ptr());
                if proc.is_null() {
                    None
                } else {
                    Some(proc as u64)
                }
            }
        }

        let tyb = tyb_from_records(&[
            (0x20, 2, [0x01, 0x00, 0x00, 0x10, 0x00, 0x00]), // ALLOC slot1 0x1000
            (0xFF, 0, [0; 6]),
        ]);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("slim ALLOC+RET");

        // Vec map: prove all five imports resolve to non-null VAs.
        let mapped = manual_map_pe_dll(&pe.bytes, host_resolve).expect("manual_map bind");
        let imports = collect_resolved_imports(&mapped.image, &mapped.headers).expect("collect");
        let want = [
            "VirtualAlloc",
            "CreateFileA",
            "ReadFile",
            "WriteFile",
            "CloseHandle",
        ];
        for name in want {
            let hit = imports.iter().find(|(_, n, _, _)| n == name);
            let Some((_, _, _, va)) = hit else {
                panic!("missing import {name}");
            };
            // Unbound IAT holds hint RVAs (~data RVA); bound holds host VAs.
            assert!(
                *va > 0x1_0000_0000,
                "{name} IAT looks unbound (va={va:#x})"
            );
        }

        // Executable map: run entry; ALLOC must leave non-null ptr in slot1.
        let exe = manual_map_pe_dll_executable(&pe.bytes, host_resolve).expect("exec map");
        let headers = parse_pe64_headers(&pe.bytes).expect("headers");
        let pe_off = u32::from_le_bytes(pe.bytes[0x3C..0x40].try_into().unwrap()) as usize;
        let num_sec =
            u16::from_le_bytes(pe.bytes[pe_off + 6..pe_off + 8].try_into().unwrap()) as usize;
        let opt_size =
            u16::from_le_bytes(pe.bytes[pe_off + 20..pe_off + 22].try_into().unwrap()) as usize;
        let mut data_rva = 0u32;
        for i in 0..num_sec {
            let sh = pe_off + 24 + opt_size + i * 40;
            if &pe.bytes[sh..sh + 5] == b".data" {
                data_rva = u32::from_le_bytes(pe.bytes[sh + 12..sh + 16].try_into().unwrap());
                break;
            }
        }
        assert_ne!(data_rva, 0);
        unsafe {
            let entry = (exe.base as usize + headers.entry_rva as usize) as *const ();
            let f: extern "C" fn() = std::mem::transmute(entry);
            f();
            let slot1 = std::ptr::read_unaligned(
                (exe.base as *const u8).add(data_rva as usize + 8) as *const u64,
            );
            assert_ne!(slot1, 0, "ALLOC via bound IAT must store non-null");
            // Free the VirtualAlloc'd user buffer (best-effort).
            VirtualFree(slot1 as *mut std::ffi::c_void, 0, MEM_RELEASE);
        }
    }

    /// S3.100: slim LOAD end-to-end — real CreateFileA/ReadFile via bound IAT.
    /// StubPlatform stays stub (FlatBinary/H_00); TYB slim path is the real I/O lane.
    #[test]
    #[cfg(windows)]
    fn s3_slim_pe_manual_map_load_file_e2e() {
        use crate::pe_manual_map::{
            collect_resolved_imports, manual_map_pe_dll, manual_map_pe_dll_executable,
            parse_pe64_headers,
        };
        use std::ffi::CString;

        #[link(name = "kernel32")]
        extern "system" {
            fn LoadLibraryA(name: *const i8) -> *mut std::ffi::c_void;
            fn GetProcAddress(
                module: *mut std::ffi::c_void,
                name: *const i8,
            ) -> *mut std::ffi::c_void;
            fn VirtualFree(lpAddress: *mut std::ffi::c_void, dwSize: usize, dwFreeType: u32)
                -> i32;
        }
        const MEM_RELEASE: u32 = 0x8000;

        fn host_resolve(dll: &str, name: &str) -> Option<u64> {
            let dll_c = CString::new(dll).ok()?;
            let name_c = CString::new(name).ok()?;
            unsafe {
                let module = LoadLibraryA(dll_c.as_ptr());
                if module.is_null() {
                    return None;
                }
                let proc = GetProcAddress(module, name_c.as_ptr());
                if proc.is_null() {
                    None
                } else {
                    Some(proc as u64)
                }
            }
        }

        // ASCII-only path under cwd (CreateFileA); avoid non-ANSI temp paths.
        let dir = std::env::current_dir()
            .unwrap_or_else(|_| std::env::temp_dir())
            .join(format!("yoyo-s3100-load-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("payload.bin");
        let payload = [0xDEu8, 0xAD, 0xBE, 0xEF, 0x01, 0x02, 0x03, 0x04];
        std::fs::write(&path, &payload).expect("write payload");
        let path_str = path.to_str().expect("path must be UTF-8/ANSI-safe").to_string();
        assert!(
            path_str.is_ascii(),
            "CreateFileA path must be ASCII, got {path_str}"
        );
        let mut path_bytes = path_str.into_bytes();
        path_bytes.push(0);

        // path lives at .data+0x100 (past low state slots); LOAD slot1.
        const PATH_OFF: u32 = 0x100;
        let prog = TybProgram {
            rec_cnt: 2,
            records: vec![
                TybRecord {
                    op: 0x50,
                    argc: 2,
                    args_raw: [
                        0x01,
                        0x00,
                        PATH_OFF as u8,
                        (PATH_OFF >> 8) as u8,
                        (PATH_OFF >> 16) as u8,
                        (PATH_OFF >> 24) as u8,
                    ],
                },
                TybRecord {
                    op: 0xFF,
                    argc: 0,
                    args_raw: [0; 6],
                },
            ],
        };
        let mut cg = emit_x86(&prog);
        cg.data.resize(PATH_OFF as usize, 0);
        cg.data.extend_from_slice(&path_bytes);
        assert!(
            cg.data.len() < S1_SLIM_IAT_BASE as usize,
            "path must not overlap IAT"
        );
        let pe = wrap_pe_slim(&cg).expect("slim LOAD PE");

        let mapped = manual_map_pe_dll(&pe.bytes, host_resolve).expect("manual_map bind");
        let imports = collect_resolved_imports(&mapped.image, &mapped.headers).expect("collect");
        for name in [
            "VirtualAlloc",
            "CreateFileA",
            "ReadFile",
            "WriteFile",
            "CloseHandle",
        ] {
            let va = imports
                .iter()
                .find(|(_, n, _, _)| n == name)
                .map(|(_, _, _, va)| *va)
                .unwrap_or(0);
            assert!(va > 0x1_0000_0000, "{name} IAT unbound (va={va:#x})");
        }
        // Path bytes must land at r15+PATH_OFF in the mapped image.
        let pe_off = u32::from_le_bytes(pe.bytes[0x3C..0x40].try_into().unwrap()) as usize;
        let num_sec =
            u16::from_le_bytes(pe.bytes[pe_off + 6..pe_off + 8].try_into().unwrap()) as usize;
        let opt_size =
            u16::from_le_bytes(pe.bytes[pe_off + 20..pe_off + 22].try_into().unwrap()) as usize;
        let mut data_rva = 0u32;
        for i in 0..num_sec {
            let sh = pe_off + 24 + opt_size + i * 40;
            if &pe.bytes[sh..sh + 5] == b".data" {
                data_rva = u32::from_le_bytes(pe.bytes[sh + 12..sh + 16].try_into().unwrap());
                break;
            }
        }
        assert_ne!(data_rva, 0);
        let path_in_img = &mapped.image
            [(data_rva as usize + PATH_OFF as usize)..(data_rva as usize + PATH_OFF as usize + path_bytes.len())];
        assert_eq!(path_in_img, path_bytes.as_slice(), "mapped path mismatch");
        drop(mapped);

        let exe = manual_map_pe_dll_executable(&pe.bytes, host_resolve).expect("exec map");
        let headers = parse_pe64_headers(&pe.bytes).expect("headers");
        unsafe {
            let entry = (exe.base as usize + headers.entry_rva as usize) as *const ();
            let f: extern "C" fn() = std::mem::transmute(entry);
            f();
            let buf = std::ptr::read_unaligned(
                (exe.base as *const u8).add(data_rva as usize + 8) as *const u64,
            );
            assert_ne!(buf, 0, "LOAD must store buffer ptr");
            let got = std::slice::from_raw_parts(buf as *const u8, payload.len());
            assert_eq!(got, &payload, "ReadFile payload mismatch");
            VirtualFree(buf as *mut std::ffi::c_void, 0, MEM_RELEASE);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// S3.101: slim WRITE end-to-end — real CreateFileA/WriteFile via bound IAT.
    /// Host fills slot1=buf ptr, slot2=len after map (runtime VA); path at .data+0x100.
    #[test]
    #[cfg(windows)]
    fn s3_slim_pe_manual_map_write_file_e2e() {
        use crate::pe_manual_map::{
            collect_resolved_imports, manual_map_pe_dll, manual_map_pe_dll_executable,
            parse_pe64_headers,
        };
        use std::ffi::CString;

        #[link(name = "kernel32")]
        extern "system" {
            fn LoadLibraryA(name: *const i8) -> *mut std::ffi::c_void;
            fn GetProcAddress(
                module: *mut std::ffi::c_void,
                name: *const i8,
            ) -> *mut std::ffi::c_void;
        }

        fn host_resolve(dll: &str, name: &str) -> Option<u64> {
            let dll_c = CString::new(dll).ok()?;
            let name_c = CString::new(name).ok()?;
            unsafe {
                let module = LoadLibraryA(dll_c.as_ptr());
                if module.is_null() {
                    return None;
                }
                let proc = GetProcAddress(module, name_c.as_ptr());
                if proc.is_null() {
                    None
                } else {
                    Some(proc as u64)
                }
            }
        }

        let dir = std::env::current_dir()
            .unwrap_or_else(|_| std::env::temp_dir())
            .join(format!("yoyo-s3101-write-{}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let path = dir.join("out.bin");
        // File must not exist yet; WRITE creates it.
        let _ = std::fs::remove_file(&path);
        let path_str = path.to_str().expect("path must be UTF-8/ANSI-safe").to_string();
        assert!(path_str.is_ascii(), "CreateFileA path must be ASCII, got {path_str}");
        let mut path_bytes = path_str.into_bytes();
        path_bytes.push(0);

        const PATH_OFF: u32 = 0x100;
        const PAYLOAD_OFF: u32 = 0x180;
        let payload = [0xCAu8, 0xFE, 0xBA, 0xBE, 0x11, 0x22, 0x33, 0x44];

        // WRITE slot1 path_off sz_slot=2
        let prog = TybProgram {
            rec_cnt: 2,
            records: vec![
                TybRecord {
                    op: 0x51,
                    argc: 3,
                    args_raw: [
                        0x01,
                        0x00,
                        PATH_OFF as u8,
                        (PATH_OFF >> 8) as u8,
                        0x02,
                        0x00,
                    ],
                },
                TybRecord {
                    op: 0xFF,
                    argc: 0,
                    args_raw: [0; 6],
                },
            ],
        };
        let mut cg = emit_x86(&prog);
        cg.data.resize(PATH_OFF as usize, 0);
        cg.data.extend_from_slice(&path_bytes);
        assert!(
            cg.data.len() <= PAYLOAD_OFF as usize,
            "path overlaps payload region"
        );
        cg.data.resize(PAYLOAD_OFF as usize, 0);
        cg.data.extend_from_slice(&payload);
        assert!(
            cg.data.len() < S1_SLIM_IAT_BASE as usize,
            "payload must not overlap IAT"
        );
        let pe = wrap_pe_slim(&cg).expect("slim WRITE PE");

        let mapped = manual_map_pe_dll(&pe.bytes, host_resolve).expect("manual_map bind");
        let imports = collect_resolved_imports(&mapped.image, &mapped.headers).expect("collect");
        for name in [
            "VirtualAlloc",
            "CreateFileA",
            "ReadFile",
            "WriteFile",
            "CloseHandle",
        ] {
            let va = imports
                .iter()
                .find(|(_, n, _, _)| n == name)
                .map(|(_, _, _, va)| *va)
                .unwrap_or(0);
            assert!(va > 0x1_0000_0000, "{name} IAT unbound (va={va:#x})");
        }
        drop(mapped);

        let exe = manual_map_pe_dll_executable(&pe.bytes, host_resolve).expect("exec map");
        let headers = parse_pe64_headers(&pe.bytes).expect("headers");
        let pe_off = u32::from_le_bytes(pe.bytes[0x3C..0x40].try_into().unwrap()) as usize;
        let num_sec =
            u16::from_le_bytes(pe.bytes[pe_off + 6..pe_off + 8].try_into().unwrap()) as usize;
        let opt_size =
            u16::from_le_bytes(pe.bytes[pe_off + 20..pe_off + 22].try_into().unwrap()) as usize;
        let mut data_rva = 0u32;
        for i in 0..num_sec {
            let sh = pe_off + 24 + opt_size + i * 40;
            if &pe.bytes[sh..sh + 5] == b".data" {
                data_rva = u32::from_le_bytes(pe.bytes[sh + 12..sh + 16].try_into().unwrap());
                break;
            }
        }
        assert_ne!(data_rva, 0);
        unsafe {
            let data_base = (exe.base as *mut u8).add(data_rva as usize);
            // slot1 = ptr to payload in mapped .data; slot2 = len
            let buf_va = data_base.add(PAYLOAD_OFF as usize) as u64;
            std::ptr::write_unaligned(data_base.add(8) as *mut u64, buf_va);
            std::ptr::write_unaligned(data_base.add(16) as *mut u64, payload.len() as u64);

            let entry = (exe.base as usize + headers.entry_rva as usize) as *const ();
            let f: extern "C" fn() = std::mem::transmute(entry);
            f();
        }
        let got = std::fs::read(&path).expect("read written file");
        assert_eq!(got.as_slice(), &payload, "WriteFile payload mismatch");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // ============= S3.18: full .text compose (multi-op SET+RET) =============

    const S3_LEA_PTR: &[u8] = &[0x49, 0x8D, 0x07, 0x49, 0x89, 0x47, 0x78]; // lea rax,[r15]; mov [r15+0x78],rax
    const S3_LEA_DST: &[u8] = &[
        0x49, 0x8D, 0x87, 0xC8, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x48,
    ];

    /// Append one table-scan + MEMCPY_DATA; optionally ADDV dst+=len afterwards.
    fn append_scan_memcpy(
        recs: &mut Vec<(u8, u8, [u8; 6])>,
        needle: u8,
        loop_id: u32,
        found_id: u32,
        advance_dst: bool,
    ) {
        let slot = |s: u16| s.to_le_bytes();
        let set = |s: u16, imm: u32| {
            let sb = slot(s);
            let ib = imm.to_le_bytes();
            (0x30u8, 2u8, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
        };
        let add = |s: u16, imm: u32| {
            let sb = slot(s);
            let ib = imm.to_le_bytes();
            (0x62u8, 2u8, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
        };
        let p = slot(S3_SCAN_PTR);
        let o = slot(S3_SCAN_OP);
        let n = slot(S3_SCAN_NEEDLE);
        let i = slot(S3_SCAN_I);
        let nr = slot(S3_SCAN_NROWS);
        let len = slot(S3_SCAN_OUT);
        let src = slot(S3_COMPOSE_SRC);
        let dst = slot(S3_COMPOSE_DST);
        let lb = loop_id.to_le_bytes();
        let fb = found_id.to_le_bytes();

        recs.push(set(S3_SCAN_NEEDLE, needle as u32));
        recs.push(set(S3_SCAN_I, 0));
        recs.push(set(S3_SCAN_NROWS, 3));
        recs.push((0x40, 1, [lb[0], lb[1], lb[2], lb[3], 0, 0]));
        recs.push((0x80, 3, [o[0], o[1], p[0], p[1], 0, 0]));
        recs.push((0x65, 2, [o[0], o[1], n[0], n[1], 0, 0]));
        recs.push((0x71, 1, [fb[0], fb[1], fb[2], fb[3], 0, 0]));
        recs.push(add(S3_SCAN_PTR, 20));
        recs.push(add(S3_SCAN_I, 1));
        recs.push((0x65, 2, [i[0], i[1], nr[0], nr[1], 0, 0]));
        recs.push((0x73, 1, [lb[0], lb[1], lb[2], lb[3], 0, 0]));
        recs.push(set(S3_SCAN_OUT, 0xFE));
        recs.push((0xFF, 0, [0; 6]));
        recs.push((0x40, 1, [fb[0], fb[1], fb[2], fb[3], 0, 0]));
        recs.push((0x80, 3, [len[0], len[1], p[0], p[1], 1, 0]));
        recs.push((0x64, 2, [src[0], src[1], p[0], p[1], 0, 0]));
        recs.push(add(S3_COMPOSE_SRC, 4));
        recs.push((0x84, 3, [dst[0], dst[1], src[0], src[1], len[0], len[1]]));
        if advance_dst {
            // ADDV dst, len
            recs.push((0x68, 2, [dst[0], dst[1], len[0], len[1], 0, 0]));
        }
    }

    /// S3.18: scan SET then RET → outbuf full `.text` ≡ emit_x86(SET+RET).
    fn s3_table_fulltext_set_ret_tyb() -> Vec<u8> {
        let mut table = Vec::new();
        table.extend_from_slice(&tmpl_row(0xFF, &[0xC3]));
        table.extend_from_slice(&tmpl_row(0x00, &[0x90]));
        table.extend_from_slice(&tmpl_row(0x30, &set_body_deadbeef()));
        let mut recs = pack_data_recs(&table);
        recs.extend(pack_raw_recs(S3_LEA_PTR));
        recs.extend(pack_raw_recs(S3_LEA_DST));
        append_scan_memcpy(&mut recs, 0x30, 1, 2, true);
        // re-seed table ptr for second scan
        recs.extend(pack_raw_recs(S3_LEA_PTR));
        append_scan_memcpy(&mut recs, 0xFF, 3, 4, false);
        recs.push((0xFF, 0, [0; 6]));
        tyb_from_records(&recs)
    }

    /// Sim fulltext: accumulate every MEMCPY into outbuf; support ADDV.
    fn s3_sim_table_fulltext(tyb: &[u8]) -> Result<Vec<u8>, u8> {
        let prog = parse_tyb(tyb).expect("parse");
        let mut data = vec![0u8; 512];
        let mut off = 0usize;
        for rec in &prog.records {
            if matches!(rec.op, 0x10 | 0x12 | 0x13) {
                let b = arg_low_bytes(rec);
                data[off..off + b.len()].copy_from_slice(&b);
                off += b.len();
            }
        }
        let mut state = [0u64; 16];
        state[S3_SCAN_PTR as usize] = 0;
        state[S3_COMPOSE_DST as usize] = S3_OUTBUF_OFF as u64;
        let code: Vec<&TybRecord> = prog
            .records
            .iter()
            .filter(|r| !matches!(r.op, 0x10 | 0x12 | 0x13 | 0xA1 | 0xA0))
            .collect();
        let mut labels = std::collections::HashMap::new();
        for (i, rec) in code.iter().enumerate() {
            if rec.op == 0x40 {
                if let Some(id) = arg_u32(rec) {
                    labels.insert(id, i);
                }
            }
        }
        let mut pc = 0usize;
        let mut zf = false;
        let mut sf = false;
        let mut composed = 0usize;
        let mut steps = 0usize;
        while pc < code.len() {
            steps += 1;
            assert!(steps < 10_000, "sim runaway");
            let rec = code[pc];
            match rec.op {
                0x40 => pc += 1,
                0x30 => {
                    let (s, imm) = arg_slot_imm(rec).unwrap();
                    state[s as usize] = imm as u64;
                    // PE re-seeds ptr via RAW lea (ignored here); each scan starts with SET needle.
                    if s == S3_SCAN_NEEDLE {
                        state[S3_SCAN_PTR as usize] = 0;
                    }
                    pc += 1;
                }
                0x62 => {
                    let (s, imm) = arg_slot_imm(rec).unwrap();
                    state[s as usize] = state[s as usize].wrapping_add(imm as u64);
                    pc += 1;
                }
                0x64 => {
                    let (d, s) = arg_slot_slot(rec).unwrap();
                    state[d as usize] = state[s as usize];
                    pc += 1;
                }
                0x68 => {
                    let (d, s) = arg_slot_slot(rec).unwrap();
                    state[d as usize] = state[d as usize].wrapping_add(state[s as usize]);
                    pc += 1;
                }
                0x80 => {
                    let (dd, ss, oo) = arg_slot3(rec).unwrap();
                    let addr = state[ss as usize] as usize + oo as usize;
                    state[dd as usize] = data[addr] as u64;
                    pc += 1;
                }
                0x84 => {
                    let (dst, src, n) = arg_slot3(rec).unwrap();
                    let d = state[dst as usize] as usize;
                    let s = state[src as usize] as usize;
                    let n = state[n as usize] as usize;
                    let tmp = data[s..s + n].to_vec();
                    data[d..d + n].copy_from_slice(&tmp);
                    composed += n;
                    pc += 1;
                }
                0x65 => {
                    let (a, b) = arg_slot_slot(rec).unwrap();
                    let d = (state[a as usize] as i64).wrapping_sub(state[b as usize] as i64);
                    zf = d == 0;
                    sf = d < 0;
                    pc += 1;
                }
                0x71 => {
                    let id = arg_u32(rec).unwrap();
                    if zf {
                        pc = labels[&id];
                    } else {
                        pc += 1;
                    }
                }
                0x73 => {
                    let id = arg_u32(rec).unwrap();
                    if sf {
                        pc = labels[&id];
                    } else {
                        pc += 1;
                    }
                }
                0xFF => {
                    // miss path sets OUT=0xFE then RET before any memcpy
                    if composed == 0 {
                        return Err(state[S3_SCAN_OUT as usize] as u8);
                    }
                    break;
                }
                other => panic!("fulltext sim unsupported {other:#x}"),
            }
        }
        if composed == 0 {
            return Err(0xFE);
        }
        let base = S3_OUTBUF_OFF as usize;
        Ok(data[base..base + composed].to_vec())
    }

    #[test]
    fn s3_yoyo_table_fulltext_set_ret_sim() {
        let tyb = s3_table_fulltext_set_ret_tyb();
        if let Ok(out) = std::env::var("YOYO_S318_OUT") {
            std::fs::write(&out, &tyb).expect("dump fulltext tyb");
        }
        let got = s3_sim_table_fulltext(&tyb).expect("fulltext");
        let expect = emit_x86(
            &parse_tyb(&tyb_from_records(&[
                (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
                (0xFF, 0, [0; 6]),
            ]))
            .unwrap(),
        )
        .text;
        assert_eq!(got, expect);
        // also ≡ SET body ‖ RET
        let mut hand = set_body_deadbeef().to_vec();
        hand.push(0xC3);
        assert_eq!(got, hand);
        assert!(compile_tyb_to_pe_mini(&tyb).is_ok());
        let user = emit_x86(&parse_tyb(&tyb).unwrap()).text;
        assert!(
            user.len() <= S1_SLIM_MAX_CODE,
            "fulltext code {} > {}",
            user.len(),
            S1_SLIM_MAX_CODE
        );
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../yoyo/tests/golden/s3_yoyo_table_fulltext.tyb");
        let golden = std::fs::read(&path).expect("golden fulltext tyb");
        assert_eq!(golden, tyb);
        assert_eq!(s3_sim_table_fulltext(&golden).unwrap(), expect);
    }

    #[test]
    #[cfg(windows)]
    fn s3_yoyo_table_fulltext_runs_on_pe() {
        let tyb = s3_table_fulltext_set_ret_tyb();
        let pe = compile_tyb_to_pe_mini(&tyb).expect("mini");
        let expect = emit_x86(
            &parse_tyb(&tyb_from_records(&[
                (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
                (0xFF, 0, [0; 6]),
            ]))
            .unwrap(),
        )
        .text;
        let got = s3_run_slim_pe_outbuf(&pe.bytes, expect.len());
        assert_eq!(got, expect);
    }

    // ============= S3.19: general multi-op program loop =============

    const S3_PROG_I: u16 = 0;
    const S3_PROG_N: u16 = 1;
    const S3_PROG_PTR: u16 = 2;
    const S3_TABLE_BASE: u16 = 3;
    /// Table relocated past slot area [0..0x80).
    const S3_TABLE_OFF: u32 = 0x80;
    /// Program: [n][op…] immediately after 3×20B table.
    const S3_PROG_OFF: u32 = S3_TABLE_OFF + 60; // 0xBC

    /// S3.19: DATA pad+table+program; outer loop loads needles; scan+MEMCPY each.
    fn s3_table_prog_loop_tyb(ops: &[u8]) -> Vec<u8> {
        assert!(!ops.is_empty() && ops.len() <= 8);
        let mut payload = vec![0u8; S3_TABLE_OFF as usize];
        payload.extend_from_slice(&tmpl_row(0xFF, &[0xC3]));
        payload.extend_from_slice(&tmpl_row(0x00, &[0x90]));
        payload.extend_from_slice(&tmpl_row(0x30, &set_body_deadbeef()));
        assert_eq!(payload.len(), S3_PROG_OFF as usize);
        payload.push(ops.len() as u8);
        payload.extend_from_slice(ops);
        // pad to outbuf so layout is stable (zeros)
        while payload.len() < S3_OUTBUF_OFF as usize {
            payload.push(0);
        }
        let mut recs = pack_data_recs(&payload);
        // lea rax,[r15+0x80]; store table_base (slot3) + scan ptr (slot15)
        recs.extend(pack_raw_recs(&[
            0x49, 0x8D, 0x87, 0x80, 0x00, 0x00, 0x00, // lea rax,[r15+0x80]
            0x49, 0x89, 0x47, 0x18, // mov [r15+3*8],rax
            0x49, 0x89, 0x47, 0x78, // mov [r15+15*8],rax
        ]));
        // lea rax,[r15+0xC8]; store dst slot9
        recs.extend(pack_raw_recs(S3_LEA_DST));
        // lea rax,[r15+0xBD]; store prog_ptr slot2  (first op = PROG_OFF+1)
        let op0 = S3_PROG_OFF + 1;
        recs.extend(pack_raw_recs(&[
            0x49, 0x8D, 0x87, // lea rax,[r15+disp32]
            (op0 & 0xFF) as u8,
            ((op0 >> 8) & 0xFF) as u8,
            ((op0 >> 16) & 0xFF) as u8,
            ((op0 >> 24) & 0xFF) as u8,
            0x49, 0x89, 0x47, 0x10, // mov [r15+2*8],rax
        ]));

        let slot = |s: u16| s.to_le_bytes();
        let set = |s: u16, imm: u32| {
            let sb = slot(s);
            let ib = imm.to_le_bytes();
            (0x30u8, 2u8, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
        };
        let add = |s: u16, imm: u32| {
            let sb = slot(s);
            let ib = imm.to_le_bytes();
            (0x62u8, 2u8, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
        };
        let p = slot(S3_SCAN_PTR);
        let o = slot(S3_SCAN_OP);
        let n = slot(S3_SCAN_NEEDLE);
        let i = slot(S3_SCAN_I);
        let nr = slot(S3_SCAN_NROWS);
        let len = slot(S3_SCAN_OUT);
        let src = slot(S3_COMPOSE_SRC);
        let dst = slot(S3_COMPOSE_DST);
        let pi = slot(S3_PROG_I);
        let pn = slot(S3_PROG_N);
        let pp = slot(S3_PROG_PTR);
        let tb = slot(S3_TABLE_BASE);

        // LDB prog_n from absolute/offset PROG_OFF — use ptr trick: SET tmp via
        // lea already done for ops; load n from [r15+0xBC] via LDB with a ptr slot.
        // Seed slot1 from DATA: use LDB with table_base-relative: easier SET from known.
        // Program length is known at build time — SET prog_n = ops.len().
        recs.push(set(S3_PROG_N, ops.len() as u32));
        recs.push(set(S3_PROG_I, 0));

        // LABEL outer=5
        recs.push((0x40, 1, [5, 0, 0, 0, 0, 0]));
        recs.push((0x65, 2, [pi[0], pi[1], pn[0], pn[1], 0, 0])); // CMP i,n
        recs.push((0x74, 1, [6, 0, 0, 0, 0, 0])); // JGE done (0x74)
        // needle = LDB [prog_ptr]
        recs.push((0x80, 3, [n[0], n[1], pp[0], pp[1], 0, 0]));
        // ptr = table_base
        recs.push((0x64, 2, [p[0], p[1], tb[0], tb[1], 0, 0]));
        recs.push(set(S3_SCAN_I, 0));
        recs.push(set(S3_SCAN_NROWS, 3));
        // LABEL scan=1
        recs.push((0x40, 1, [1, 0, 0, 0, 0, 0]));
        recs.push((0x80, 3, [o[0], o[1], p[0], p[1], 0, 0]));
        recs.push((0x65, 2, [o[0], o[1], n[0], n[1], 0, 0]));
        recs.push((0x71, 1, [2, 0, 0, 0, 0, 0])); // JE found
        recs.push(add(S3_SCAN_PTR, 20));
        recs.push(add(S3_SCAN_I, 1));
        recs.push((0x65, 2, [i[0], i[1], nr[0], nr[1], 0, 0]));
        recs.push((0x73, 1, [1, 0, 0, 0, 0, 0])); // JL scan
        recs.push(set(S3_SCAN_OUT, 0xFE));
        recs.push((0xFF, 0, [0; 6]));
        // LABEL found=2
        recs.push((0x40, 1, [2, 0, 0, 0, 0, 0]));
        recs.push((0x80, 3, [len[0], len[1], p[0], p[1], 1, 0]));
        recs.push((0x64, 2, [src[0], src[1], p[0], p[1], 0, 0]));
        recs.push(add(S3_COMPOSE_SRC, 4));
        recs.push((0x84, 3, [dst[0], dst[1], src[0], src[1], len[0], len[1]]));
        recs.push((0x68, 2, [dst[0], dst[1], len[0], len[1], 0, 0])); // ADDV dst,len
        recs.push(add(S3_PROG_PTR, 1));
        recs.push(add(S3_PROG_I, 1));
        recs.push((0x70, 1, [5, 0, 0, 0, 0, 0])); // JMP outer
        // LABEL done=6
        recs.push((0x40, 1, [6, 0, 0, 0, 0, 0]));
        recs.push((0xFF, 0, [0; 6]));
        tyb_from_records(&recs)
    }

    fn s3_sim_table_prog_loop(tyb: &[u8]) -> Result<Vec<u8>, u8> {
        let prog = parse_tyb(tyb).expect("parse");
        let mut data = vec![0u8; 512];
        let mut off = 0usize;
        for rec in &prog.records {
            if matches!(rec.op, 0x10 | 0x12 | 0x13) {
                let b = arg_low_bytes(rec);
                data[off..off + b.len()].copy_from_slice(&b);
                off += b.len();
            }
        }
        let mut state = [0u64; 16];
        // Seed mirrors RAW lea (sim uses offsets into data[])
        state[S3_TABLE_BASE as usize] = S3_TABLE_OFF as u64;
        state[S3_SCAN_PTR as usize] = S3_TABLE_OFF as u64;
        state[S3_COMPOSE_DST as usize] = S3_OUTBUF_OFF as u64;
        state[S3_PROG_PTR as usize] = (S3_PROG_OFF + 1) as u64;
        let code: Vec<&TybRecord> = prog
            .records
            .iter()
            .filter(|r| !matches!(r.op, 0x10 | 0x12 | 0x13 | 0xA1 | 0xA0))
            .collect();
        let mut labels = std::collections::HashMap::new();
        for (i, rec) in code.iter().enumerate() {
            if rec.op == 0x40 {
                if let Some(id) = arg_u32(rec) {
                    labels.insert(id, i);
                }
            }
        }
        let mut pc = 0usize;
        let mut zf = false;
        let mut sf = false;
        let mut composed = 0usize;
        let mut steps = 0usize;
        while pc < code.len() {
            steps += 1;
            assert!(steps < 50_000, "sim runaway");
            let rec = code[pc];
            match rec.op {
                0x40 => pc += 1,
                0x30 => {
                    let (s, imm) = arg_slot_imm(rec).unwrap();
                    state[s as usize] = imm as u64;
                    pc += 1;
                }
                0x62 => {
                    let (s, imm) = arg_slot_imm(rec).unwrap();
                    state[s as usize] = state[s as usize].wrapping_add(imm as u64);
                    pc += 1;
                }
                0x64 => {
                    let (d, s) = arg_slot_slot(rec).unwrap();
                    state[d as usize] = state[s as usize];
                    pc += 1;
                }
                0x68 => {
                    let (d, s) = arg_slot_slot(rec).unwrap();
                    state[d as usize] = state[d as usize].wrapping_add(state[s as usize]);
                    pc += 1;
                }
                0x80 => {
                    let (dd, ss, oo) = arg_slot3(rec).unwrap();
                    let addr = state[ss as usize] as usize + oo as usize;
                    state[dd as usize] = data[addr] as u64;
                    pc += 1;
                }
                0x84 => {
                    let (dst, src, n) = arg_slot3(rec).unwrap();
                    let d = state[dst as usize] as usize;
                    let s = state[src as usize] as usize;
                    let n = state[n as usize] as usize;
                    let tmp = data[s..s + n].to_vec();
                    data[d..d + n].copy_from_slice(&tmp);
                    composed += n;
                    pc += 1;
                }
                0x65 => {
                    let (a, b) = arg_slot_slot(rec).unwrap();
                    let d = (state[a as usize] as i64).wrapping_sub(state[b as usize] as i64);
                    zf = d == 0;
                    sf = d < 0;
                    pc += 1;
                }
                0x70 => {
                    let id = arg_u32(rec).unwrap();
                    pc = labels[&id];
                }
                0x71 => {
                    let id = arg_u32(rec).unwrap();
                    if zf {
                        pc = labels[&id];
                    } else {
                        pc += 1;
                    }
                }
                0x73 => {
                    let id = arg_u32(rec).unwrap();
                    if sf {
                        pc = labels[&id];
                    } else {
                        pc += 1;
                    }
                }
                0x74 => {
                    // JGE: SF==0 (not less) — matches after CMP a,b when a>=b
                    let id = arg_u32(rec).unwrap();
                    if !sf {
                        pc = labels[&id];
                    } else {
                        pc += 1;
                    }
                }
                0xFF => {
                    if composed == 0 {
                        return Err(state[S3_SCAN_OUT as usize] as u8);
                    }
                    break;
                }
                other => panic!("prog-loop sim unsupported {other:#x}"),
            }
        }
        if composed == 0 {
            return Err(0xFE);
        }
        let base = S3_OUTBUF_OFF as usize;
        Ok(data[base..base + composed].to_vec())
    }

    #[test]
    fn s3_yoyo_table_prog_loop_sim() {
        let set_ret = s3_table_prog_loop_tyb(&[0x30, 0xFF]);
        if let Ok(out) = std::env::var("YOYO_S319_OUT") {
            std::fs::write(&out, &set_ret).expect("dump");
        }
        let got = s3_sim_table_prog_loop(&set_ret).expect("set+ret");
        let expect = emit_x86(
            &parse_tyb(&tyb_from_records(&[
                (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
                (0xFF, 0, [0; 6]),
            ]))
            .unwrap(),
        )
        .text;
        assert_eq!(got, expect);

        let nop_ret = s3_sim_table_prog_loop(&s3_table_prog_loop_tyb(&[0x00, 0xFF])).unwrap();
        assert_eq!(
            nop_ret,
            emit_x86(
                &parse_tyb(&tyb_from_records(&[
                    (0x00, 0, [0; 6]),
                    (0xFF, 0, [0; 6]),
                ]))
                .unwrap(),
            )
            .text
        );

        let ret_only = s3_sim_table_prog_loop(&s3_table_prog_loop_tyb(&[0xFF])).unwrap();
        assert_eq!(ret_only, vec![0xC3]);

        assert!(compile_tyb_to_pe_mini(&set_ret).is_ok());
        let user = emit_x86(&parse_tyb(&set_ret).unwrap()).text;
        assert!(user.len() <= S1_SLIM_MAX_CODE, "code {}", user.len());

        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../yoyo/tests/golden/s3_yoyo_table_prog_loop.tyb");
        let golden = std::fs::read(&path).expect("golden prog loop");
        assert_eq!(golden, set_ret);
    }

    #[test]
    #[cfg(windows)]
    fn s3_yoyo_table_prog_loop_runs_on_pe() {
        let tyb = s3_table_prog_loop_tyb(&[0x30, 0xFF]);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("mini");
        let expect = emit_x86(
            &parse_tyb(&tyb_from_records(&[
                (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
                (0xFF, 0, [0; 6]),
            ]))
            .unwrap(),
        )
        .text;
        let got = s3_run_slim_pe_outbuf(&pe.bytes, expect.len());
        assert_eq!(got, expect);
    }

    // ============= S3.20: host emit path via YOYO table =============

    #[test]
    fn s3_host_yoyo_table_emit_matches_emit_x86() {
        let cases: &[(&[u8], &[(u8, u8, [u8; 6])])] = &[
            (&[0xFF], &[(0xFF, 0, [0; 6])]),
            (&[0x00, 0xFF], &[(0x00, 0, [0; 6]), (0xFF, 0, [0; 6])]),
            (
                &[0x30, 0xFF],
                &[
                    (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
                    (0xFF, 0, [0; 6]),
                ],
            ),
            (
                &[0x60, 0xFF],
                &[(0x60, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
            (
                &[0x66, 0xFF],
                &[(0x66, 1, [0, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
            (
                &[0x62, 0xFF],
                &[(0x62, 2, [0, 0, 5, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
            (
                &[0x67, 0xFF],
                &[(0x67, 1, [0, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
            (
                &[0x61, 0xFF],
                &[(0x61, 2, [0, 0, 5, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
            (
                &[0x64, 0xFF],
                &[(0x64, 2, [1, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
            (
                &[0x65, 0xFF],
                &[(0x65, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])],
            ),
        ];
        for (ops, semantic) in cases {
            let via_yoyo = try_emit_text_via_yoyo_table(ops).expect("yoyo emit");
            let via_rust = emit_x86(&parse_tyb(&tyb_from_records(semantic)).unwrap()).text;
            assert_eq!(via_yoyo, via_rust, "ops={ops:?}");
            let tyb = tyb_from_records(semantic);
            let pe_y = try_compile_tyb_via_yoyo_table(&tyb).expect("yoyo pe");
            let pe_r = compile_tyb_to_pe_mini(&tyb).expect("mini pe");
            let t_y = slim_pe_user_text(&pe_y.bytes, via_yoyo.len()).unwrap();
            let t_r = slim_pe_user_text(&pe_r.bytes, via_rust.len()).unwrap();
            assert_eq!(t_y, t_r, "PE user text ops={ops:?}");
            assert_eq!(fixed_template_ops(&parse_tyb(&tyb).unwrap()).unwrap(), *ops);
        }
        // Outside subset → None (IMUL not in table)
        assert!(try_emit_text_via_yoyo_table(&[0x63, 0xFF]).is_none());
        let other = tyb_from_records(&[(0x63, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(try_compile_tyb_via_yoyo_table(&other).is_none());
        assert!(fixed_template_ops(&parse_tyb(&other).unwrap()).is_none());
    }

    // ============= S3.21: mini prefers YOYO table for fixed subset =============

    #[test]
    fn s3_mini_prefers_yoyo_table_for_fixed_subset() {
        let set_ret = tyb_from_records(&[
            (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
            (0xFF, 0, [0; 6]),
        ]);
        let pe_mini = compile_tyb_to_pe_mini(&set_ret).expect("mini");
        let pe_yoyo = try_compile_tyb_via_yoyo_table(&set_ret).expect("yoyo");
        // Same PE bytes → mini took the YOYO-table path (not a divergent emit_x86 wrap).
        assert_eq!(pe_mini.bytes, pe_yoyo.bytes);
        let text = try_emit_text_via_yoyo_table(&[0x30, 0xFF]).unwrap();
        assert_eq!(
            slim_pe_user_text(&pe_mini.bytes, text.len()).unwrap(),
            text.as_slice()
        );
        // Non-subset (IMUL) still compiles via emit_x86 fallback
        let imul_ret = tyb_from_records(&[(0x63, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(try_compile_tyb_via_yoyo_table(&imul_ret).is_none());
        let pe_imul = compile_tyb_to_pe_mini(&imul_ret).expect("imul fallback");
        let expect = emit_x86(&parse_tyb(&imul_ret).unwrap()).text;
        assert_eq!(
            slim_pe_user_text(&pe_imul.bytes, expect.len()).unwrap(),
            expect.as_slice()
        );
    }
}
