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
/// (imm8 form is STB-only — fixed10 ADD golden pins imm32; do not change this.)
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

/// S3.156/S3.210: STB val base oo — store low byte of state[val] to [state[base]+oo].
/// Small `oo` (1..=127) uses imm8 add (STB-only; fixed10 ADD golden stays imm32).
fn emit_stb(out: &mut Vec<u8>, val: u16, base: u16, oo: u16) {
    emit_load_state(out, val, Reg::Rcx);
    emit_load_state(out, base, Reg::Rax);
    if oo != 0 {
        if oo <= 127 {
            let rex = rex_wrxb(true, false, false, Reg::Rax.rex_bit());
            out.extend_from_slice(&[rex, 0x83, 0xC0 | Reg::Rax.low3(), oo as u8]);
        } else {
            emit_add_imm(out, Reg::Rax, oo as u32);
        }
    }
    out.extend_from_slice(&[0x88, 0x08]); // mov byte [rax], cl
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
            // --- S3.156: STB val base oo ---
            0x81 => {
                if let Some((val, base, oo)) = arg_slot3(rec) {
                    emit_stb(&mut text, val, base, oo);
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
/// S3.7: 256→512. S3.219: 512→1024 so poke+lea+prog (~800B) can empty DATA.
pub const S1_SLIM_MAX_CODE: usize = 1024;
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
/// S3.7: align(13+512)=0x400 → data@0x800. S3.219: align(13+1024)=0x600 → @0xA00.
pub const S1_SLIM_DATA_PAYLOAD_OFF: usize = 0x400 + 0x600;

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
            | 0x81
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
            // S3.156: STB val base oo (same slot rules as LDB).
            0x81 => {
                if rec.argc != 3 {
                    return bad(i, format!("s1 mini STB requires argc=3, got {}", rec.argc));
                }
                let val = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                let base = u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]);
                if !mini_slot_disp8_ok(val) || !mini_slot_disp8_ok(base) {
                    return bad(i, "s1 mini STB slot disp32 unsupported".into());
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
/// S3.21/S3.111: fixed-template 10 ops prefer YOYO table-compose
/// ([`try_compile_tyb_via_yoyo_table`]); otherwise `emit_x86`.
/// Subset proof does not go through `pack_meta_emit_*`. Host firmware keeps
/// one DATA pack (`s3_pack_data`); RAW prologue dropped in S3.112.
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

/// S3.150: RAW `0xA1` pack helper (short lea / PE seeds — still Rust chunk pack).
fn s3_append_raw_a1(recs: &mut Vec<(u8, u8, [u8; 6])>, bytes: &[u8]) {
    recs.extend(s3_host_pack_bytes(0xA1, bytes));
}

/// S3.153/S3.203: argc=1 per byte. Production pack/emit paths no longer call this;
/// only a couple of unit tests still seed DATA this way.
fn s3_pack_data_bytewise(bytes: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    bytes
        .iter()
        .map(|&b| (0x10u8, 1u8, [b, 0, 0, 0, 0, 0]))
        .collect()
}

/// S3.166: flatten one TYB record to 8 wire bytes (op+argc+args).
fn s3_rec_wire(op: u8, argc: u8, a: [u8; 6]) -> [u8; 8] {
    [op, argc, a[0], a[1], a[2], a[3], a[4], a[5]]
}

/// S3.226/S3.279: append SET DST + run-folded STB of `flat` at `out_off`.
/// S3.279: drop SET ZERO — sim state starts zeroed (slot13); runs CMP against it.
/// Shared by one-shot emit and two-phase / loop-staged phase1. Returns next label id.
fn s3_append_stb_flat_at(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    flat: &[u8],
    out_off: u32,
    label_id: u32,
) -> u32 {
    recs.push(s3_rec_set(S3_PACK_PROTO_DST, out_off));
    s3_append_stb_runs(recs, flat, label_id)
}

/// S3.225/S3.226: shared empty-DATA STB emit — write `flat` at `out_off`, then RET.
/// YOYO-target: one emit primitive; Rust still builds the run-folded IR today.
fn s3_yoyo_emit_flat_via_stb(flat: &[u8], out_off: u32) -> (Vec<u8>, u32) {
    let mut recs = Vec::new();
    s3_append_stb_flat_at(&mut recs, flat, out_off, 1);
    recs.push((0xFF, 0, [0; 6]));
    (s3_host_tyb_from_records(&recs), out_off)
}

/// S3.166/S3.200/S3.225: firmware that STB-emits given records to outbuf (empty DATA).
fn s3_yoyo_emit_recs_tyb(records: &[(u8, u8, [u8; 6])]) -> (Vec<u8>, u32) {
    let out_off = 0x20u32;
    let mut flat = Vec::with_capacity(records.len() * RECORD_SIZE);
    for &(op, argc, a) in records {
        flat.extend_from_slice(&s3_rec_wire(op, argc, a));
    }
    s3_yoyo_emit_flat_via_stb(&flat, out_off)
}

/// S3.167/S3.200/S3.225: STB-emit full TYB file via run-folded wire.
fn s3_yoyo_emit_tyb_file_unrolled(records: &[(u8, u8, [u8; 6])]) -> (Vec<u8>, u32) {
    assert!(records.len() <= 255, "S3.167 prototype: rec_cnt fits u8 path");
    let out_off = 0x20u32;
    let mut flat = Vec::with_capacity(HEADER_SIZE + records.len() * RECORD_SIZE);
    flat.extend_from_slice(&TYB_MAGIC);
    flat.extend_from_slice(&0u16.to_le_bytes());
    flat.extend_from_slice(&(records.len() as u16).to_le_bytes());
    for &(op, argc, a) in records {
        flat.extend_from_slice(&s3_rec_wire(op, argc, a));
    }
    s3_yoyo_emit_flat_via_stb(&flat, out_off)
}

/// S3.171/S3.202/S3.226: comparison path. Flat is IR-written at 0 (run-folded), then
/// one MEMCPY to outbuf. Phase1 shares `s3_append_stb_flat_at`.
fn s3_yoyo_emit_tyb_file_loop_staged(records: &[(u8, u8, [u8; 6])]) -> (Vec<u8>, u32) {
    let flat = s3_host_tyb_from_records(records);
    let pay = 0u32;
    let out_off = flat.len() as u32;
    let mut recs = Vec::new();
    s3_append_stb_flat_at(&mut recs, &flat, pay, 10);
    recs.push(s3_rec_set(S3_PACK_LOOP_CUR, pay));
    recs.push(s3_rec_set(S3_PACK_LOOP_END, out_off));
    recs.push(s3_rec_set(S3_PACK_PROTO_DST, out_off));
    recs.push(s3_rec_memcpy(
        S3_PACK_PROTO_DST,
        S3_PACK_LOOP_CUR,
        S3_PACK_LOOP_END,
    ));
    recs.push((0xFF, 0, [0; 6]));
    (s3_host_tyb_from_records(&recs), out_off)
}

/// S3.183: STB `bytes` at DST. Identical bytes of length >= 2 share one SET and a
/// SUB/JE loop. Singletons stay SET+STB+ADD. Returns the next free label id.
/// Rust still chooses the immediates; they are not copied into DATA.
fn s3_append_stb_runs(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    bytes: &[u8],
    mut label_id: u32,
) -> u32 {
    let mut i = 0;
    while i < bytes.len() {
        let b = bytes[i];
        let mut n = 1usize;
        while i + n < bytes.len() && bytes[i + n] == b {
            n += 1;
        }
        if n == 1 {
            recs.push(s3_rec_set(S3_PACK_STB_TMP, b as u32));
            recs.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_PROTO_DST, 0));
            recs.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
        } else {
            let l_loop = label_id;
            let l_done = label_id + 1;
            label_id += 2;
            recs.push(s3_rec_set(S3_PACK_STB_TMP, b as u32));
            recs.push(s3_rec_set(S3_PACK_LOOP_REM, n as u32));
            recs.push(s3_rec_label(l_loop));
            recs.push(s3_rec_cmp(S3_PACK_LOOP_REM, S3_PACK_STB_ZERO));
            recs.push(s3_rec_je(l_done));
            recs.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_PROTO_DST, 0));
            recs.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
            recs.push(s3_rec_sub(S3_PACK_LOOP_REM, 1));
            recs.push(s3_rec_jmp(l_loop));
            recs.push(s3_rec_label(l_done));
        }
        i += n;
    }
    label_id
}

/// S3.222: one SET per distinct nonzero; STB `oo` relative to already-live BASE.
/// Caller owns BASE (SET absolute, or MOVRR from lea slot15). No SHR.
fn s3_append_stb_poke_global_distinct(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    bytes: &[u8],
) {
    let mut by_val: Vec<(u8, Vec<u16>)> = Vec::new();
    for (i, &b) in bytes.iter().enumerate() {
        if b == 0 {
            continue;
        }
        match by_val.iter_mut().find(|(v, _)| *v == b) {
            Some((_, offs)) => offs.push(i as u16),
            None => by_val.push((b, vec![i as u16])),
        }
    }
    for (b, offs) in by_val {
        recs.push(s3_rec_set(S3_PACK_STB_TMP, b as u32));
        for oo in offs {
            recs.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_STB_BASE, oo));
        }
    }
}

/// S3.209–S3.217: IR-seed sparse bytes — poke only nonzero offsets via STB `oo`.
/// Gaps stay zero from empty-image / STB resize. Short spans (≤127) use one SET
/// per distinct nonzero (keeps in-DLL seed under 512). Longer host seeds walk
/// address order so BASE switches once per live window (S3.217). No SHR.
fn s3_append_stb_zero_then_poke(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    base: u32,
    bytes: &[u8],
    label_id: u32,
) -> u32 {
    const WIN: usize = 127;
    if bytes.len() <= WIN {
        // Global distinct: fewest TMP SETs; single BASE window.
        if bytes.iter().any(|&b| b != 0) {
            recs.push(s3_rec_set(S3_PACK_STB_BASE, base));
            s3_append_stb_poke_global_distinct(recs, bytes);
        }
        return label_id;
    }
    // Long span: address order → BASE once per live window.
    let mut cur_win: Option<usize> = None;
    let mut cur_val: Option<u8> = None;
    for (i, &b) in bytes.iter().enumerate() {
        if b == 0 {
            continue;
        }
        let win = i / WIN;
        if cur_win != Some(win) {
            let win_base = base + (win * WIN) as u32;
            recs.push(s3_rec_set(S3_PACK_STB_BASE, win_base));
            cur_win = Some(win);
        }
        if cur_val != Some(b) {
            recs.push(s3_rec_set(S3_PACK_STB_TMP, b as u32));
            cur_val = Some(b);
        }
        let oo = (i % WIN) as u16;
        recs.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_STB_BASE, oo));
    }
    label_id
}

/// S3.181/S3.184: IR writes staging (runs, not SET per byte), then one MEMCPY
/// to outbuf. No DATA records — STB grows the image. No Rust flat preimage.
fn s3_yoyo_emit_tyb_file_two_phase(records: &[(u8, u8, [u8; 6])]) -> (Vec<u8>, u32) {
    assert!(records.len() <= 255);
    let pay = 0x10u32;
    let flat_len = HEADER_SIZE as u32 + records.len() as u32 * RECORD_SIZE as u32;
    let out_off = pay + flat_len;
    let mut flat = Vec::with_capacity(flat_len as usize);
    flat.extend_from_slice(&TYB_MAGIC);
    flat.extend_from_slice(&0u16.to_le_bytes());
    flat.extend_from_slice(&(records.len() as u16).to_le_bytes());
    for &(op, argc, a) in records {
        flat.extend_from_slice(&s3_rec_wire(op, argc, a));
    }
    debug_assert_eq!(flat.len(), flat_len as usize);
    // S3.184/S3.226: no zero DATA; phase1 is the shared STB-flat prefix.
    let mut recs = Vec::new();
    s3_append_stb_flat_at(&mut recs, &flat, pay, 1);
    // Phase 2 (S3.182): one MEMCPY staging → outbuf. Length is state, not an imm.
    recs.push(s3_rec_set(S3_PACK_LOOP_CUR, pay));
    recs.push(s3_rec_set(S3_PACK_LOOP_END, flat_len));
    recs.push(s3_rec_set(S3_PACK_PROTO_DST, out_off));
    recs.push(s3_rec_memcpy(
        S3_PACK_PROTO_DST,
        S3_PACK_LOOP_CUR,
        S3_PACK_LOOP_END,
    ));
    recs.push((0xFF, 0, [0; 6]));
    (s3_host_tyb_from_records(&recs), out_off)
}

/// S3.177/S3.181: small → unrolled; large → two-phase (IR staging, zero DATA image).
const S3_EMIT_UNROLL_MAX_FLAT: usize = 64;

fn s3_yoyo_emit_tyb_file(records: &[(u8, u8, [u8; 6])]) -> (Vec<u8>, u32) {
    let flat_len = HEADER_SIZE + records.len() * RECORD_SIZE;
    if flat_len <= S3_EMIT_UNROLL_MAX_FLAT {
        s3_yoyo_emit_tyb_file_unrolled(records)
    } else {
        s3_yoyo_emit_tyb_file_two_phase(records)
    }
}

/// S3.141: YOYO-target chunk size for DATA `0x10` pack (still Rust today).
const S3_PACK_CHUNK: usize = 3;

/// S3.136/S3.141: YOYO-table firmware DATA pack is always op `0x10` (still Rust).
/// Algorithm (portable to YOYO IR later): walk `bytes` in chunks of `S3_PACK_CHUNK`,
/// each record argc=chunk_len, low bytes at args[0], args[2], args[4].
fn s3_pack_data(bytes: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    debug_assert_eq!(S3_PACK_CHUNK, 3);
    s3_host_pack_bytes(0x10, bytes)
}

/// S3.137: inverse of `s3_pack_data` — recover flat payload from DATA `0x10` records.
fn s3_unpack_data(recs: &[(u8, u8, [u8; 6])]) -> Vec<u8> {
    let mut out = Vec::new();
    for &(op, argc, a) in recs {
        assert_eq!(op, 0x10, "s3_unpack_data: expected DATA 0x10, got {op:#x}");
        for k in 0..argc as usize {
            out.push(a[k * 2]);
        }
    }
    out
}

fn s3_unpack_data_tyb(tyb: &[u8]) -> Vec<u8> {
    let prog = parse_tyb(tyb).expect("unpack tyb");
    let mut out = Vec::new();
    for rec in &prog.records {
        if rec.op == 0x10 {
            out.extend(arg_low_bytes(rec));
        }
    }
    out
}

/// S3.142: flatten `s3_pack_data` records to wire bytes (op+argc+6 args each).
fn s3_pack_data_wire(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for (op, argc, a) in s3_pack_data(bytes) {
        out.push(op);
        out.push(argc);
        out.extend_from_slice(&a);
    }
    out
}

/// S3.144/S3.266/S3.269: pack-only — wrap a raw 8B×N record section as TYB + parse.
/// Expand path emits a full TYB itself (S3.267); this wrap remains for pack wire.
fn s3_pack_wire_section_to_recs(wire: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    assert_eq!(wire.len() % RECORD_SIZE, 0, "pack wire must be 8-byte records");
    let n = wire.len() / RECORD_SIZE;
    assert!(n <= u16::MAX as usize, "wire rec_cnt fits u16");
    let mut tyb = Vec::with_capacity(HEADER_SIZE + wire.len());
    tyb.extend_from_slice(&TYB_MAGIC);
    tyb.extend_from_slice(&[0, 0]);
    tyb.extend_from_slice(&(n as u16).to_le_bytes());
    tyb.extend_from_slice(wire);
    parse_tyb(&tyb)
        .expect("s3_pack_wire_section_to_recs: parse_tyb")
        .records
        .into_iter()
        .map(|r| (r.op, r.argc, r.args_raw))
        .collect()
}

/// S3.144/S3.158: pack via YOYO loop STB IR (sim) → wire → records; empty stays empty.
fn s3_pack_data_via_ir(bytes: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    if bytes.is_empty() {
        return Vec::new();
    }
    let (tyb, out_off) = s3_yoyo_pack_loop_stb_tyb(bytes);
    let wire = s3_sim_prog_loop(&tyb, 0, out_off, 0).expect("s3_pack_data_via_ir: sim pack");
    s3_pack_wire_section_to_recs(&wire)
}

/// S3.142/S3.145 pack-proto DATA layout (no STB — write via 1-byte MEMCPY from consts/payload).
/// S3.186: old payload cursor, kept for the out_off formula check. Live packers no longer use it.
const S3_PACK_PROTO_PAY: u32 = 5;
/// S3.142 fixed out (small unroll only); S3.145 loop uses `s3_pack_proto_out_off`.
const S3_PACK_PROTO_OUT: u32 = 0x80;
const S3_PACK_PROTO_SRC: u16 = 8;
const S3_PACK_PROTO_DST: u16 = 9;
const S3_PACK_PROTO_ONE: u16 = 12;

/// S3.145: outbuf sits just past payload + 2B zero pad (host-sized payloads fit).
fn s3_pack_proto_out_off(payload_len: usize) -> u32 {
    S3_PACK_PROTO_PAY + payload_len as u32 + 2
}

/// S3.142: emit one byte from DATA[`src_off`] to *DST, then DST += 1 (MEMCPY).
fn s3_pack_proto_putc(recs: &mut Vec<(u8, u8, [u8; 6])>, src_off: u32) {
    recs.push(s3_rec_set(S3_PACK_PROTO_SRC, src_off));
    recs.push(s3_rec_memcpy(
        S3_PACK_PROTO_DST,
        S3_PACK_PROTO_SRC,
        S3_PACK_PROTO_ONE,
    ));
    recs.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
}

const S3_PACK_STB_TMP: u16 = 11;
const S3_PACK_STB_ZERO: u16 = 13;
/// S3.210: seed poke base so STB `oo` stays ≤127 (imm8 add in emit_stb).
const S3_PACK_STB_BASE: u16 = 14;

/// S3.191: store an immediate byte (op, argc, or zero pad) — not a DATA const.
fn s3_pack_proto_putc_imm(recs: &mut Vec<(u8, u8, [u8; 6])>, byte: u8) {
    recs.push(s3_rec_set(S3_PACK_STB_TMP, byte as u32));
    recs.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_PROTO_DST, 0));
    recs.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
}

/// S3.155: emit one byte via LDB→STB (sim-only STB); DST += 1.
fn s3_pack_proto_putc_stb(recs: &mut Vec<(u8, u8, [u8; 6])>, src_off: u32) {
    recs.push(s3_rec_ldb(
        S3_PACK_STB_TMP,
        S3_PACK_STB_ZERO,
        src_off as u16,
    ));
    recs.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_PROTO_DST, 0));
    recs.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
}

/// S3.142: one DATA `0x10` record (8 wire bytes) from payload at `base`, argc=`n`.
fn s3_pack_proto_record(recs: &mut Vec<(u8, u8, [u8; 6])>, base: u32, n: u8) {
    debug_assert!((1..=S3_PACK_CHUNK as u8).contains(&n));
    const BASE: u32 = 16;
    s3_pack_proto_putc(recs, BASE);
    s3_pack_proto_putc(recs, BASE + n as u32);
    for k in 0..S3_PACK_CHUNK as u32 {
        if (k as u8) < n {
            s3_pack_proto_putc(recs, base + k);
        } else {
            s3_pack_proto_putc(recs, BASE + 4);
        }
        s3_pack_proto_putc(recs, BASE + 4);
    }
}

/// S3.142: unrolled YOYO IR pack (SET/ADD/MEMCPY/RET) matching `s3_pack_data` wire.
/// Builder is still Rust; the *executed* algorithm is YOYO ops (no STB — 1-byte MEMCPY).
fn s3_yoyo_pack_proto_ir(payload: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    let payload_len = payload.len();
    let mut recs = Vec::new();
    // S3.198/S3.279: payload at 0, consts at 16. Slot13 image-zero; slot9 needs SET
    // (sim preloads slot9 = outbuf).
    recs.push(s3_rec_set(S3_PACK_PROTO_DST, 0));
    s3_append_stb_runs(&mut recs, payload, 10);
    recs.push(s3_rec_set(S3_PACK_PROTO_DST, 16));
    for b in [0x10u8, 1, 2, 3, 0] {
        recs.push(s3_rec_set(S3_PACK_STB_TMP, b as u32));
        recs.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_PROTO_DST, 0));
        recs.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    }
    recs.push(s3_rec_set(S3_PACK_PROTO_ONE, 1));
    recs.push(s3_rec_set(S3_PACK_PROTO_DST, S3_PACK_PROTO_OUT));
    let mut i = 0usize;
    while i < payload_len {
        let n = (payload_len - i).min(S3_PACK_CHUNK) as u8;
        s3_pack_proto_record(&mut recs, i as u32, n);
        i += n as usize;
    }
    recs.push((0xFF, 0, [0; 6]));
    recs
}

/// S3.142/S3.195: firmware = payload DATA + unrolled pack IR.
fn s3_yoyo_pack_proto_tyb(payload: &[u8]) -> Vec<u8> {
    assert!(!payload.is_empty() && payload.len() <= 6);
    // S3.153: bootstrap scratch is bytewise (not chunk-3).
    let recs = s3_yoyo_pack_proto_ir(payload);
    s3_host_tyb_from_records(&recs)
}

/// S3.155: one DATA record via LDB→STB putc.
fn s3_pack_proto_record_stb(recs: &mut Vec<(u8, u8, [u8; 6])>, base: u32, n: u8) {
    debug_assert!((1..=S3_PACK_CHUNK as u8).contains(&n));
    s3_pack_proto_putc_imm(recs, 0x10);
    s3_pack_proto_putc_imm(recs, n);
    for k in 0..S3_PACK_CHUNK as u32 {
        if (k as u8) < n {
            s3_pack_proto_putc_stb(recs, base + k);
        } else {
            s3_pack_proto_putc_imm(recs, 0);
        }
        s3_pack_proto_putc_imm(recs, 0);
    }
}

/// S3.155: unrolled pack IR using LDB→STB (sim-only).
fn s3_yoyo_pack_stb_ir(payload: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    let payload_len = payload.len();
    let mut recs = Vec::new();
    // S3.279: slot13 image-zero; SET DST overrides sim outbuf preload on slot9.
    recs.push(s3_rec_set(S3_PACK_PROTO_DST, 0));
    s3_append_stb_runs(&mut recs, payload, 10);
    recs.push(s3_rec_set(S3_PACK_PROTO_DST, S3_PACK_PROTO_OUT));
    let mut i = 0usize;
    while i < payload_len {
        let n = (payload_len - i).min(S3_PACK_CHUNK) as u8;
        s3_pack_proto_record_stb(&mut recs, i as u32, n);
        i += n as usize;
    }
    recs.push((0xFF, 0, [0; 6]));
    recs
}

fn s3_yoyo_pack_stb_tyb(payload: &[u8]) -> Vec<u8> {
    assert!(!payload.is_empty() && payload.len() <= 6);
    let recs = s3_yoyo_pack_stb_ir(payload);
    s3_host_tyb_from_records(&recs)
}

/// S3.143 loop pack slots (CUR/END/REM/CHUNK/THREE; SRC/DST/ONE shared with 142).
const S3_PACK_LOOP_CUR: u16 = 4;
const S3_PACK_LOOP_END: u16 = 5;
const S3_PACK_LOOP_REM: u16 = 6;
const S3_PACK_LOOP_CHUNK: u16 = 7;
const S3_PACK_LOOP_THREE: u16 = 10;
const S3_PACK_LOOP_L_LOOP: u32 = 1;
const S3_PACK_LOOP_L_USE_REM: u32 = 2;
const S3_PACK_LOOP_L_PACK: u32 = 3;
const S3_PACK_LOOP_L_DONE: u32 = 4;

/// S3.143/S3.145: looped YOYO IR pack (CMP/JL/SUBV/ADDV/MEMCPY). Payload must be
/// zero-padded 2B past `payload_len` so chunk=1/2 can still read 3 arg lows.
fn s3_yoyo_pack_loop_ir(
    pay_off: u32,
    out_off: u32,
    payload: &[u8],
    const_base: u32,
) -> Vec<(u8, u8, [u8; 6])> {
    let payload_len = payload.len() as u32;
    let mut r = Vec::new();
    // S3.197/S3.279: payload + overread zeros are stores; slot13 stays image-zero.
    r.push(s3_rec_set(S3_PACK_PROTO_DST, pay_off));
    s3_append_stb_runs(&mut r, payload, 10);
    r.push(s3_rec_stb(S3_PACK_STB_ZERO, S3_PACK_PROTO_DST, 0));
    r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    r.push(s3_rec_stb(S3_PACK_STB_ZERO, S3_PACK_PROTO_DST, 0));
    // S3.194: [0x10, 1, 2, 3, 0] is written below outbuf, not packed as DATA.
    r.push(s3_rec_set(S3_PACK_PROTO_DST, const_base));
    for b in [0x10u8, 1, 2, 3, 0] {
        r.push(s3_rec_set(S3_PACK_STB_TMP, b as u32));
        r.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_PROTO_DST, 0));
        r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    }
    r.push(s3_rec_set(S3_PACK_PROTO_ONE, 1));
    r.push(s3_rec_set(S3_PACK_LOOP_THREE, S3_PACK_CHUNK as u32));
    r.push(s3_rec_set(S3_PACK_PROTO_DST, out_off));
    r.push(s3_rec_set(S3_PACK_LOOP_CUR, pay_off));
    r.push(s3_rec_set(S3_PACK_LOOP_END, pay_off + payload_len));
    r.push(s3_rec_label(S3_PACK_LOOP_L_LOOP));
    r.push(s3_rec_cmp(S3_PACK_LOOP_CUR, S3_PACK_LOOP_END));
    r.push(s3_rec_je(S3_PACK_LOOP_L_DONE));
    r.push(s3_rec_movrr(S3_PACK_LOOP_REM, S3_PACK_LOOP_END));
    r.push(s3_rec_subv(S3_PACK_LOOP_REM, S3_PACK_LOOP_CUR));
    r.push(s3_rec_movrr(S3_PACK_LOOP_CHUNK, S3_PACK_LOOP_THREE));
    r.push(s3_rec_cmp(S3_PACK_LOOP_REM, S3_PACK_LOOP_THREE));
    r.push(s3_rec_jl(S3_PACK_LOOP_L_USE_REM));
    r.push(s3_rec_jmp(S3_PACK_LOOP_L_PACK));
    r.push(s3_rec_label(S3_PACK_LOOP_L_USE_REM));
    r.push(s3_rec_movrr(S3_PACK_LOOP_CHUNK, S3_PACK_LOOP_REM));
    r.push(s3_rec_label(S3_PACK_LOOP_L_PACK));
    // op 0x10 at const_base
    r.push(s3_rec_set(S3_PACK_PROTO_SRC, const_base));
    r.push(s3_rec_memcpy(
        S3_PACK_PROTO_DST,
        S3_PACK_PROTO_SRC,
        S3_PACK_PROTO_ONE,
    ));
    r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    // argc = mem[const_base + CHUNK] (1, 2, or 3)
    r.push(s3_rec_set(S3_PACK_PROTO_SRC, const_base));
    r.push(s3_rec_addv(S3_PACK_PROTO_SRC, S3_PACK_LOOP_CHUNK));
    r.push(s3_rec_memcpy(
        S3_PACK_PROTO_DST,
        S3_PACK_PROTO_SRC,
        S3_PACK_PROTO_ONE,
    ));
    r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    // args: b0,0,b1,0,b2,0 (over-read safe via payload zero pad)
    for k in 0..S3_PACK_CHUNK as u32 {
        r.push(s3_rec_movrr(S3_PACK_PROTO_SRC, S3_PACK_LOOP_CUR));
        if k > 0 {
            r.push(s3_rec_add(S3_PACK_PROTO_SRC, k));
        }
        r.push(s3_rec_memcpy(
            S3_PACK_PROTO_DST,
            S3_PACK_PROTO_SRC,
            S3_PACK_PROTO_ONE,
        ));
        r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
        r.push(s3_rec_set(S3_PACK_PROTO_SRC, const_base + 4));
        r.push(s3_rec_memcpy(
            S3_PACK_PROTO_DST,
            S3_PACK_PROTO_SRC,
            S3_PACK_PROTO_ONE,
        ));
        r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    }
    r.push(s3_rec_addv(S3_PACK_LOOP_CUR, S3_PACK_LOOP_CHUNK));
    r.push(s3_rec_jmp(S3_PACK_LOOP_L_LOOP));
    r.push(s3_rec_label(S3_PACK_LOOP_L_DONE));
    r.push((0xFF, 0, [0; 6]));
    r
}

/// S3.157: putc from address in `src_slot` via LDB→STB.
fn s3_pack_loop_putc_from_slot(recs: &mut Vec<(u8, u8, [u8; 6])>, src_slot: u16) {
    recs.push(s3_rec_ldb(S3_PACK_STB_TMP, src_slot, 0));
    recs.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_PROTO_DST, 0));
    recs.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
}

/// S3.157: looped pack IR using LDB→STB instead of MEMCPY.
fn s3_yoyo_pack_loop_stb_ir(pay_off: u32, out_off: u32, payload: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    let payload_len = payload.len() as u32;
    let mut r = Vec::new();
    // S3.279: slot13 image-zero; no SET ZERO prologue.
    r.push(s3_rec_set(S3_PACK_LOOP_THREE, S3_PACK_CHUNK as u32));
    // S3.196: payload itself is IR stores. Repeated bytes share one SET.
    r.push(s3_rec_set(S3_PACK_PROTO_DST, pay_off));
    s3_append_stb_runs(&mut r, payload, 10);
    r.push(s3_rec_stb(S3_PACK_STB_ZERO, S3_PACK_PROTO_DST, 0));
    r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    r.push(s3_rec_stb(S3_PACK_STB_ZERO, S3_PACK_PROTO_DST, 0));
    r.push(s3_rec_set(S3_PACK_PROTO_DST, out_off));
    r.push(s3_rec_set(S3_PACK_LOOP_CUR, pay_off));
    r.push(s3_rec_set(S3_PACK_LOOP_END, pay_off + payload_len));
    r.push(s3_rec_label(S3_PACK_LOOP_L_LOOP));
    r.push(s3_rec_cmp(S3_PACK_LOOP_CUR, S3_PACK_LOOP_END));
    r.push(s3_rec_je(S3_PACK_LOOP_L_DONE));
    r.push(s3_rec_movrr(S3_PACK_LOOP_REM, S3_PACK_LOOP_END));
    r.push(s3_rec_subv(S3_PACK_LOOP_REM, S3_PACK_LOOP_CUR));
    r.push(s3_rec_movrr(S3_PACK_LOOP_CHUNK, S3_PACK_LOOP_THREE));
    r.push(s3_rec_cmp(S3_PACK_LOOP_REM, S3_PACK_LOOP_THREE));
    r.push(s3_rec_jl(S3_PACK_LOOP_L_USE_REM));
    r.push(s3_rec_jmp(S3_PACK_LOOP_L_PACK));
    r.push(s3_rec_label(S3_PACK_LOOP_L_USE_REM));
    r.push(s3_rec_movrr(S3_PACK_LOOP_CHUNK, S3_PACK_LOOP_REM));
    r.push(s3_rec_label(S3_PACK_LOOP_L_PACK));
    // S3.190: op, argc, and the zero pad come from slots. No const bytes in DATA.
    r.push(s3_rec_set(S3_PACK_STB_TMP, 0x10));
    r.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_PROTO_DST, 0));
    r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    r.push(s3_rec_stb(S3_PACK_LOOP_CHUNK, S3_PACK_PROTO_DST, 0));
    r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    for k in 0..S3_PACK_CHUNK as u32 {
        r.push(s3_rec_movrr(S3_PACK_PROTO_SRC, S3_PACK_LOOP_CUR));
        if k > 0 {
            r.push(s3_rec_add(S3_PACK_PROTO_SRC, k));
        }
        s3_pack_loop_putc_from_slot(&mut r, S3_PACK_PROTO_SRC);
        r.push(s3_rec_stb(S3_PACK_STB_ZERO, S3_PACK_PROTO_DST, 0));
        r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    }
    r.push(s3_rec_addv(S3_PACK_LOOP_CUR, S3_PACK_LOOP_CHUNK));
    r.push(s3_rec_jmp(S3_PACK_LOOP_L_LOOP));
    r.push(s3_rec_label(S3_PACK_LOOP_L_DONE));
    r.push((0xFF, 0, [0; 6]));
    r
}

fn s3_yoyo_pack_loop_stb_tyb(payload: &[u8]) -> (Vec<u8>, u32) {
    assert!(!payload.is_empty());
    // S3.196: no DATA records. IR writes the payload, then the two overread zeros.
    let out_off = payload.len() as u32 + 2;
    let recs = s3_yoyo_pack_loop_stb_ir(0, out_off, payload);
    (s3_host_tyb_from_records(&recs), out_off)
}

/// S3.143–S3.194: MEMCPY loop pack. DATA is payload+2; five consts are IR stores past that.
fn s3_yoyo_pack_loop_tyb(payload: &[u8]) -> (Vec<u8>, u32) {
    assert!(!payload.is_empty());
    let const_base = payload.len() as u32 + 2;
    let out_off = const_base + 5;
    let recs = s3_yoyo_pack_loop_ir(0, out_off, payload, const_base);
    (s3_host_tyb_from_records(&recs), out_off)
}

fn s3_host_tmpl_row(op: u8, payload: &[u8]) -> [u8; 20] {
    let mut r = [0u8; 20];
    r[0] = op;
    r[1] = payload.len() as u8;
    r[4..4 + payload.len()].copy_from_slice(payload);
    r
}

/// S3.122: template body is `emit_x86(op + RET)` without the trailing `C3`.
fn s3_host_body(op: u8, argc: u8, args: [u8; 6]) -> Vec<u8> {
    let text = emit_x86(
        &parse_tyb(&s3_host_tyb_from_records(&[
            (op, argc, args),
            s3_fixed_rec(0xFF),
        ]))
        .expect("body tyb"),
    )
    .text;
    assert!(text.ends_with(&[0xC3]), "template body must end in RET");
    text[..text.len() - 1].to_vec()
}

/// S3.129: canonical argc/args for one fixed10 opcode. Does not tighten `fixed_template_ops`.
fn s3_fixed_rec(op: u8) -> (u8, u8, [u8; 6]) {
    let pack = |slot: u16, imm: u32| {
        let s = slot.to_le_bytes();
        let i = imm.to_le_bytes();
        [s[0], s[1], i[0], i[1], i[2], i[3]]
    };
    match op {
        0xFF | 0x00 => (op, 0, [0; 6]),
        0x30 => (op, 2, pack(0, S3_HOST_SET_DEADBEEF)),
        0x60 => (op, 2, pack(0, 1)),
        0x66 | 0x67 => (op, 1, [0; 6]),
        0x62 => (op, 2, pack(0, S3_HOST_ADD_IMM)),
        0x61 => (op, 2, pack(0, S3_HOST_SUB_IMM)),
        0x64 => (op, 2, pack(1, 0)),
        0x65 => (op, 2, pack(0, 1)),
        _ => panic!("s3_fixed_rec: opcode {op:#x} is outside fixed10"),
    }
}

fn s3_host_body_of(op: u8) -> Vec<u8> {
    let (o, argc, args) = s3_fixed_rec(op);
    s3_host_body(o, argc, args)
}

fn s3_host_set_body() -> Vec<u8> {
    s3_host_body_of(0x30)
}
/// S3.123: RET is the whole emit (do not strip the only `C3`).
fn s3_host_ret_body() -> Vec<u8> {
    let text = emit_x86(
        &parse_tyb(&s3_host_tyb_from_records(&[s3_fixed_rec(0xFF)])).expect("ret tyb"),
    )
    .text;
    assert_eq!(text, vec![0xC3]);
    text
}
/// S3.123: NOP emit is `90 C3`; drop the trailing RET.
fn s3_host_nop_body() -> Vec<u8> {
    let b = s3_host_body_of(0x00);
    assert_eq!(b, vec![0x90]);
    b
}

/// S3.124: one body per fixed-subset opcode (host table order).
const S3_FIXED10_OPS: [u8; 10] = [
    0xFF, 0x00, 0x30, 0x60, 0x66, 0x62, 0x67, 0x61, 0x64, 0x65,
];

fn s3_fixed10_named_cases() -> Vec<(&'static str, Vec<(u8, u8, [u8; 6])>)> {
    const NAMES: [&str; 10] = [
        "RET", "NOP+RET", "SET+RET", "GET+RET", "INC+RET", "ADD+RET", "DEC+RET", "SUB+RET",
        "MOVRR+RET", "CMP+RET",
    ];
    S3_FIXED10_OPS
        .iter()
        .zip(NAMES)
        .map(|(op, name)| {
            let mut recs = vec![s3_fixed_rec(*op)];
            if *op != 0xFF {
                recs.push(s3_fixed_rec(0xFF));
            }
            (name, recs)
        })
        .collect()
}

/// S3.138: one emit_x86 bake per fixed10 op; reused for every firmware build.
fn s3_compute_fixed_body(op: u8) -> Vec<u8> {
    match op {
        0xFF => s3_host_ret_body(),
        0x00 => s3_host_nop_body(),
        o if S3_FIXED10_OPS.contains(&o) => s3_host_body_of(o),
        _ => panic!("s3_compute_fixed_body: opcode {op:#x} is outside fixed10"),
    }
}

fn s3_fixed_bodies() -> &'static [Vec<u8>; 10] {
    use std::sync::OnceLock;
    static BODIES: OnceLock<[Vec<u8>; 10]> = OnceLock::new();
    BODIES.get_or_init(|| S3_FIXED10_OPS.map(s3_compute_fixed_body))
}

fn s3_fixed_row(op: u8) -> Vec<u8> {
    let idx = S3_FIXED10_OPS
        .iter()
        .position(|&o| o == op)
        .unwrap_or_else(|| panic!("s3_fixed_row: opcode {op:#x} is outside fixed10"));
    s3_fixed_bodies()[idx].clone()
}

fn s3_fixed_table(ops: &[u8]) -> Vec<u8> {
    let mut table = Vec::with_capacity(ops.len() * 20);
    for op in ops {
        table.extend_from_slice(&s3_host_tmpl_row(*op, &s3_fixed_row(*op)));
    }
    table
}

/// S3.120: pad to table base, append template rows, then `[n][ops…]`, pad to `outbuf`.
fn s3_fixed_payload(row_ops: &[u8], prog: &[u8], outbuf: usize) -> Vec<u8> {
    let owned: Vec<(u8, Vec<u8>)> = row_ops.iter().map(|op| (*op, s3_fixed_row(*op))).collect();
    let rows: Vec<(u8, &[u8])> = owned.iter().map(|(op, b)| (*op, b.as_slice())).collect();
    s3_prog_data_payload(&rows, prog, outbuf)
}

fn s3_prog_data_payload(rows: &[(u8, &[u8])], ops: &[u8], outbuf: usize) -> Vec<u8> {
    let mut payload = vec![0u8; S3_HOST_TABLE_OFF as usize];
    for (op, body) in rows {
        payload.extend_from_slice(&s3_host_tmpl_row(*op, body));
    }
    payload.push(ops.len() as u8);
    payload.extend_from_slice(ops);
    while payload.len() < outbuf {
        payload.push(0);
    }
    payload
}

/// S3.207/S3.209: IR-write `bytes` at `base`. Bytes before `base` stay zero from
/// the empty image — do not spend IR on the leading pad. Table-shaped payloads
/// are sparse zeros; poke-only (one SET per distinct nonzero) beats run-fold.
fn s3_ir_seed_at(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    base: u32,
    bytes: &[u8],
    label_id: u32,
) -> u32 {
    s3_append_stb_zero_then_poke(recs, base, bytes, label_id)
}

/// Build program-loop firmware TYB for `ops`.
fn s3_host_prog_loop_tyb(ops: &[u8]) -> Vec<u8> {
    let payload = s3_fixed_payload(&S3_FIXED10_OPS, ops, S3_HOST_OUTBUF_OFF as usize);
    debug_assert_eq!(
        S3_HOST_TABLE_OFF as usize + 10 * 20,
        S3_HOST_PROG_OFF as usize
    );
    assert!(
        payload.len() <= S1_MINI_DATA_MAX,
        "host table DATA {} > {}",
        payload.len(),
        S1_MINI_DATA_MAX
    );
    // S3.112: host sim seeds table/outbuf/op cursor and skips 0xA1.
    // S3.207: IR-write table+prog only; [0..TABLE) stays zero. Slot 9 is the
    // outbuf cursor — restore it after the seed so the prog loop lands right.
    let start = S3_HOST_TABLE_OFF as usize;
    let end = S3_HOST_PROG_OFF as usize + 1 + ops.len();
    let mut recs = Vec::new();
    s3_ir_seed_at(&mut recs, S3_HOST_TABLE_OFF, &payload[start..end], 10);
    recs.push(s3_rec_set(S3_PACK_PROTO_DST, S3_HOST_OUTBUF_OFF));
    recs.extend(s3_prog_loop_code_host(ops.len() as u32, S3_HOST_NROWS));
    s3_host_tyb_from_records(&recs)
}

/// S3.224/S3.228: IR record shapes — YOYO-target layouts; Rust still fills bytes.
/// Named `s3_rec_*` wrappers stay for call sites; the op→shape table is the cliff.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum S3RecShape {
    SlotImm = 0,
    SlotSlot = 1,
    U32 = 2,
    Slot3 = 3,
}

/// Fixed10 / pack / emit ops that Rust still builds. YOYO reads this table later.
const S3_REC_OP_SHAPE: &[(u8, S3RecShape)] = &[
    (0x30, S3RecShape::SlotImm),  // SET
    (0x61, S3RecShape::SlotImm),  // SUB imm
    (0x62, S3RecShape::SlotImm),  // ADD imm
    (0x40, S3RecShape::U32),      // LABEL
    (0x70, S3RecShape::U32),      // JMP
    (0x71, S3RecShape::U32),      // JE
    (0x72, S3RecShape::U32),      // JNE
    (0x73, S3RecShape::U32),      // JL
    (0x74, S3RecShape::U32),      // JZ
    (0x64, S3RecShape::SlotSlot), // MOVRR
    (0x65, S3RecShape::SlotSlot), // CMP
    (0x68, S3RecShape::SlotSlot), // ADDV
    (0x6A, S3RecShape::SlotSlot), // SUBV
    (0x80, S3RecShape::Slot3),    // LDB
    (0x81, S3RecShape::Slot3),    // STB
    (0x84, S3RecShape::Slot3),    // MEMCPY
];

fn s3_rec_shape_of(op: u8) -> Option<S3RecShape> {
    S3_REC_OP_SHAPE.iter().find(|(o, _)| *o == op).map(|(_, s)| *s)
}

/// S3.229: op→shape table as a flat byte payload (op, shape_tag)×N — YOYO-readable.
fn s3_rec_op_shape_bytes() -> Vec<u8> {
    let mut out = Vec::with_capacity(S3_REC_OP_SHAPE.len() * 2);
    for &(op, shape) in S3_REC_OP_SHAPE {
        out.push(op);
        out.push(shape as u8);
    }
    out
}

/// S3.237: shape tag → argc. Indexed by the tag YOYO loaded (0..3).
fn s3_shape_argc_bytes() -> [u8; 4] {
    let mut argc = [0u8; 4];
    for &(_, shape) in S3_REC_OP_SHAPE {
        let tag = shape as u8;
        let n = match shape {
            S3RecShape::SlotImm | S3RecShape::SlotSlot => 2,
            S3RecShape::U32 => 1,
            S3RecShape::Slot3 => 3,
        };
        if argc[tag as usize] == 0 {
            argc[tag as usize] = n;
        } else {
            debug_assert_eq!(argc[tag as usize], n);
        }
    }
    argc
}

/// S3.252: argc seed is a frozen flat blob (shape-derived + RET→0). Not rebuilt at call.
const S3_SEED_ARGC_BYTES: &[u8] = &[
    48, 2, 97, 2, 98, 2, 64, 1, 112, 1, 113, 1, 114, 1, 115, 1, 116, 1,
    100, 2, 101, 2, 104, 2, 106, 2, 128, 3, 129, 3, 132, 3, 255, 0,
];
/// S3.286: shape-fill scans `(op,argc)` pairs (same blob as expand), not op→shape.
/// S3.289: +JNE → 34B table (was 32B).
const S3_SHAPE_TABLE: u32 = 0x40;
const S3_SHAPE_TABLE_END: u32 = S3_SHAPE_TABLE + S3_SEED_ARGC_BYTES.len() as u32; // 0x62
/// One job: op byte then 6 arg bytes. Control IR never SETs these.
const S3_SHAPE_JOB: u32 = 0x68;
const S3_SHAPE_OUT: u32 = 0x80;

fn s3_seed_argc_bytes() -> Vec<u8> {
    S3_SEED_ARGC_BYTES.to_vec()
}

fn s3_seed_argc_flat() -> Vec<(u8, u8)> {
    s3_seed_argc_bytes()
        .chunks_exact(2)
        .map(|c| (c[0], c[1]))
        .collect()
}

fn s3_seed_argc(op: u8) -> u8 {
    s3_seed_argc_from(S3_SEED_ARGC_BYTES, op)
}

fn s3_seed_argc_from(argc_flat: &[u8], op: u8) -> u8 {
    argc_flat
        .chunks_exact(2)
        .find(|c| c[0] == op)
        .map(|c| c[1])
        .expect("shape-fill seed op")
}

fn s3_rec_from_wire_seed(row: &[u8; 7]) -> (u8, u8, [u8; 6]) {
    s3_rec_from_wire_seed_with_argc(row, S3_SEED_ARGC_BYTES)
}

fn s3_rec_from_wire_seed_with_argc(row: &[u8; 7], argc_flat: &[u8]) -> (u8, u8, [u8; 6]) {
    (
        row[0],
        s3_seed_argc_from(argc_flat, row[0]),
        [row[1], row[2], row[3], row[4], row[5], row[6]],
    )
}

/// S3.244/S3.253/S3.262/S3.276: Rust expand (bootstrap + test parity). Production
/// fill ctrl uses via_yoyo; sole production bootstrap is `s3_bootstrap_expand_asm`.
fn s3_recs_from_flat_seed(flat: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    s3_recs_from_flat_seed_with_argc(flat, S3_SEED_ARGC_BYTES)
}

/// S3.275/S3.276: sole named bootstrap — init+ctrl asm flat without via_yoyo.
/// Argc table is IR-seeded separately (`s3_yoyo_expand_flat_tyb`).
/// S3.276: dropped `s3_bootstrap_expand_ctrl`; ctrl-only uses `s3_recs_from_flat_seed`.
fn s3_expand_asm_flat() -> Vec<u8> {
    let mut flat = Vec::with_capacity(S3_EXPAND_ASM_ROWS * 7);
    flat.extend_from_slice(S3_EXPAND_INIT_FLAT);
    flat.extend_from_slice(S3_EXPAND_CTRL_FLAT);
    flat
}

fn s3_bootstrap_expand_asm() -> Vec<(u8, u8, [u8; 6])> {
    s3_recs_from_flat_seed(&s3_expand_asm_flat())
}

fn s3_recs_from_flat_seed_with_argc(
    flat: &[u8],
    argc_flat: &[u8],
) -> Vec<(u8, u8, [u8; 6])> {
    assert_eq!(flat.len() % 7, 0);
    flat.chunks_exact(7)
        .map(|c| {
            let row = [c[0], c[1], c[2], c[3], c[4], c[5], c[6]];
            s3_rec_from_wire_seed_with_argc(&row, argc_flat)
        })
        .collect()
}

const S3_EXPAND_SEED: u32 = 0x40;
const S3_EXPAND_ARGC: u32 = 0x1C0;
const S3_EXPAND_OUT: u32 = 0x200;

/// S3.257: trailing 0xFE sentinel ends the job blob — no job-side SET END.
/// S3.264: argc table is IR-seeded from `S3_SEED_ARGC_BYTES` (not 51 poke rows).
/// S3.267/S3.268: expand emits full TYB; header template IR-seeded; body patches cnt.
/// Bootstrap still Rust-expands the ctrl body (cannot via_yoyo itself).
const S3_EXPAND_SENTINEL: u8 = 0xFE;
const S3_EXPAND_CTRL_ROWS: usize = 27;
/// S3.270: only magic must be IR-seeded; reserved/count stay zero until body patches cnt.
const S3_EXPAND_TYB_MAGIC: &[u8] = b"TYB";
/// S3.274/S3.277: assembler init SETs as a flat seed (same expand path as ctrl).
/// S3.277: drop SET slot13=0 / slot1=0 — sim state starts zeroed.
const S3_EXPAND_INIT_ROWS: usize = 4;
const S3_EXPAND_INIT_FLAT: &[u8] = &[
    // SET 0 = OUT, SET 4 = SEED, SET 5 = sentinel, SET 9 = OUT+hdr
    48, 0, 0, 0, 2, 0, 0, 48, 4, 0, 64, 0, 0, 0,
    48, 5, 0, 254, 0, 0, 0, 48, 9, 0, 8, 2, 0, 0,
];
/// S3.275: assembler body rows = init + ctrl (joined once in `s3_expand_asm_flat`).
const S3_EXPAND_ASM_ROWS: usize = S3_EXPAND_INIT_ROWS + S3_EXPAND_CTRL_ROWS;

/// Expand ctrl body: job/argc loops + emit + rec_cnt patch. Init SETs live in assembler (S3.273/274).
/// S3.280/S3.282: 6-byte arg copy is MEMCPY+ADDV dst (was LDB/STB loop; no src ADDV).
const S3_EXPAND_CTRL_FLAT: &[u8] = &[
    64, 20, 0, 0, 0, 0, 0, 128, 7, 0, 4, 0, 0, 0,
    101, 7, 0, 5, 0, 0, 0, 113, 21, 0, 0, 0, 0, 0,
    48, 2, 0, 192, 1, 0, 0, 48, 3, 0, 226, 1, 0, 0,
    64, 22, 0, 0, 0, 0, 0, 101, 2, 0, 3, 0, 0, 0,
    113, 21, 0, 0, 0, 0, 0, 128, 11, 0, 2, 0, 0, 0,
    // S3.289: speculative argc load + ADD ptr+=2 + JNE scan (drop LABEL/JMP).
    128, 6, 0, 2, 0, 1, 0, 98, 2, 0, 2, 0, 0, 0,
    101, 11, 0, 7, 0, 0, 0, 114, 22, 0, 0, 0, 0, 0,
    // S3.288: STB op@0 / argc@1 then ADD dst+=2 (was two ADD+=1).
    129, 7, 0, 9, 0, 0, 0, 129, 6, 0, 9, 0, 1, 0,
    98, 9, 0, 2, 0, 0, 0, 98, 4, 0, 1, 0, 0, 0,
    48, 10, 0, 6, 0, 0, 0,
    132, 9, 0, 4, 0, 10, 0, 104, 9, 0, 10, 0, 0, 0,
    98, 1, 0, 1, 0, 0, 0, 98, 4, 0, 6, 0, 0, 0,
    112, 20, 0, 0, 0, 0, 0, 64, 21, 0, 0, 0, 0, 0,
    129, 1, 0, 0, 0, 6, 0, 255, 0, 0, 0, 0, 0, 0,
];

/// S3.254/S3.260/S3.264/S3.268: YOYO IR expands a 7×N flat seed into a TYB file.
/// Assembler: job poke + argc IR-seed + TYB header seed + bootstrap body.
fn s3_yoyo_expand_flat_tyb(flat: &[u8]) -> (Vec<u8>, u32) {
    assert!(!flat.is_empty() && flat.len() % 7 == 0);
    assert!(
        flat.chunks_exact(7).all(|r| r[0] != S3_EXPAND_SENTINEL),
        "flat op collides with expand sentinel"
    );
    assert!(flat.len() + 1 <= (S3_EXPAND_ARGC - S3_EXPAND_SEED) as usize);
    // Body SETs argc end = 0x1E2; blob must fill [0x1C0, 0x1E2).
    assert_eq!(S3_SEED_ARGC_BYTES.len() as u32, 0x1E2 - S3_EXPAND_ARGC);
    assert_eq!(S3_EXPAND_TYB_MAGIC, &TYB_MAGIC[..3]);
    let mut job = Vec::with_capacity(flat.len() + 1);
    job.extend_from_slice(flat);
    job.push(S3_EXPAND_SENTINEL);
    let mut recs = Vec::new();
    s3_ir_seed_at(&mut recs, S3_EXPAND_SEED, &job, 30);
    s3_ir_seed_at(&mut recs, S3_EXPAND_ARGC, S3_SEED_ARGC_BYTES, 31);
    // Sparse poke of magic only; high-water composed + zero DATA cover the rest.
    s3_ir_seed_at(&mut recs, S3_EXPAND_OUT, S3_EXPAND_TYB_MAGIC, 32);
    // S3.275: init+ctrl are one assembler flat seed (single Rust expand).
    assert_eq!(S3_EXPAND_INIT_FLAT.len(), S3_EXPAND_INIT_ROWS * 7);
    assert_eq!(S3_EXPAND_CTRL_FLAT.len(), S3_EXPAND_CTRL_ROWS * 7);
    assert_eq!(s3_expand_asm_flat().len(), S3_EXPAND_ASM_ROWS * 7);
    recs.extend(s3_bootstrap_expand_asm());
    (s3_host_tyb_from_records(&recs), S3_EXPAND_OUT)
}

/// S3.254/S3.255/S3.263/S3.267: expand flat via YOYO sim → full TYB → parse_tyb.
fn s3_recs_from_flat_seed_via_yoyo(flat: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    let (asm, out) = s3_yoyo_expand_flat_tyb(flat);
    let emitted = s3_sim_prog_loop(&asm, 0, out, 0).expect("expand flat via yoyo");
    parse_tyb(&emitted)
        .expect("expand emits TYB")
        .records
        .into_iter()
        .map(|r| (r.op, r.argc, r.args_raw))
        .collect()
}

const S3_SHAPE_FILL_ONE_AT: usize = 6;
const S3_SHAPE_FILL_ONE_LEN: usize = 17;
const S3_SHAPE_FILL_MANY_ROWS: usize = 29;

/// S3.241/S3.244: shape-fill many-ctrl seed as a flat 7×N blob. RET is 0xFF.
/// S3.286: `(op,argc)` scan. S3.287: drop SET out — sim preloads slot9 = outbuf.
const S3_SHAPE_FILL_MANY_FLAT: &[u8] = &[
    48, 2, 0, 104, 0, 0, 0, 128, 3, 0, 2, 0, 0, 0,
    98, 2, 0, 1, 0, 0, 0, 64, 7, 0, 0, 0, 0, 0,
    101, 3, 0, 13, 0, 0, 0, 113, 6, 0, 0, 0, 0, 0,
    128, 7, 0, 2, 0, 0, 0, 48, 4, 0, 64, 0, 0, 0,
    48, 5, 0, 98, 0, 0, 0, 64, 1, 0, 0, 0, 0, 0,
    101, 4, 0, 5, 0, 0, 0, 113, 6, 0, 0, 0, 0, 0,
    128, 11, 0, 4, 0, 0, 0,
    // S3.289: speculative argc + ADD+=2 + JNE scan (drop match LABEL/JMP).
    128, 6, 0, 4, 0, 1, 0, 98, 4, 0, 2, 0, 0, 0,
    101, 11, 0, 7, 0, 0, 0, 114, 1, 0, 0, 0, 0, 0,
    129, 7, 0, 9, 0, 0, 0,
    // S3.288: STB op@0 / argc@1 then ADD dst+=2.
    129, 6, 0, 9, 0, 1, 0, 98, 9, 0, 2, 0, 0, 0,
    98, 2, 0, 1, 0, 0, 0, 48, 10, 0, 6, 0, 0, 0,
    132, 9, 0, 2, 0, 10, 0, 104, 9, 0, 10, 0, 0, 0,
    98, 2, 0, 6, 0, 0, 0, 97, 3, 0, 1, 0, 0, 0,
    112, 7, 0, 0, 0, 0, 0, 64, 6, 0, 0, 0, 0, 0,
    255, 0, 0, 0, 0, 0, 0,
];

fn s3_yoyo_shape_fill_one() -> Vec<(u8, u8, [u8; 6])> {
    let start = S3_SHAPE_FILL_ONE_AT * 7;
    let end = start + S3_SHAPE_FILL_ONE_LEN * 7;
    s3_recs_from_flat_seed_via_yoyo(&S3_SHAPE_FILL_MANY_FLAT[start..end])
}

const S3_SHAPE_FILL_ONE_FLAT: &[u8] = &[
    // S3.287: only job ptr SET; out = sim outbuf preload on slot9.
    48, 2, 0, 104, 0, 0, 0, 128, 7, 0, 2, 0, 0, 0,
    48, 4, 0, 64, 0, 0, 0, 48, 5, 0, 98, 0, 0, 0,
    64, 1, 0, 0, 0, 0, 0, 101, 4, 0, 5, 0, 0, 0,
    113, 6, 0, 0, 0, 0, 0, 128, 11, 0, 4, 0, 0, 0,
    // S3.289: speculative argc + ADD+=2 + JNE scan.
    128, 6, 0, 4, 0, 1, 0, 98, 4, 0, 2, 0, 0, 0,
    101, 11, 0, 7, 0, 0, 0, 114, 1, 0, 0, 0, 0, 0,
    129, 7, 0, 9, 0, 0, 0, 129, 6, 0, 9, 0, 1, 0,
    98, 9, 0, 2, 0, 0, 0, 98, 2, 0, 1, 0, 0, 0,
    48, 10, 0, 6, 0, 0, 0, 132, 9, 0, 2, 0, 10, 0,
    64, 6, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0,
];

const S3_SHAPE_FILL_ONE_ROWS: usize = 20;

/// S3.246/S3.255: one-job control expands via YOYO IR (not a Rust chunk loop).
fn s3_yoyo_shape_fill_ctrl() -> Vec<(u8, u8, [u8; 6])> {
    s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_ONE_FLAT)
}

/// S3.239/S3.255: many-ctrl expands via YOYO IR from the flat seed blob.
const S3_SHAPE_MANY_OUT: u32 = 0x300;

fn s3_yoyo_shape_fill_many_ctrl() -> Vec<(u8, u8, [u8; 6])> {
    s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT)
}

fn s3_yoyo_shape_fill_many_tyb(jobs: &[(u8, [u8; 6])]) -> (Vec<u8>, u32) {
    // S3.261: 64→128 (pre-264 argc-poke‖ctrl fit). S3.264 body-only is <<128.
    assert!(!jobs.is_empty() && jobs.len() <= 128);
    let mut blob = Vec::with_capacity(1 + jobs.len() * 7);
    blob.push(jobs.len() as u8);
    for &(op, args) in jobs {
        blob.push(op);
        blob.extend_from_slice(&args);
    }
    let mut recs = Vec::new();
    // S3.286: one `(op,argc)` seed (shared with expand); no op→shape + shape→argc.
    s3_ir_seed_at(&mut recs, S3_SHAPE_TABLE, S3_SEED_ARGC_BYTES, 20);
    s3_ir_seed_at(&mut recs, S3_SHAPE_JOB, &blob, 21);
    recs.extend(s3_yoyo_shape_fill_many_ctrl());
    (s3_host_tyb_from_records(&recs), S3_SHAPE_MANY_OUT)
}

/// S3.236/S3.238/S3.286: poke `(op,argc)` table + one job, then the fixed control stream.
fn s3_yoyo_shape_fill_tyb(op: u8, args: [u8; 6]) -> (Vec<u8>, u32) {
    let mut job = Vec::with_capacity(7);
    job.push(op);
    job.extend_from_slice(&args);
    let mut recs = Vec::new();
    s3_ir_seed_at(&mut recs, S3_SHAPE_TABLE, S3_SEED_ARGC_BYTES, 20);
    s3_ir_seed_at(&mut recs, S3_SHAPE_JOB, &job, 21);
    recs.extend(s3_yoyo_shape_fill_ctrl());
    (s3_host_tyb_from_records(&recs), S3_SHAPE_OUT)
}

/// S3.235: build a record by looking up `op` in `S3_REC_OP_SHAPE`, then filling
/// the matching wire shape. YOYO-target dispatch; Rust still supplies immediates.
fn s3_rec_from_shape(op: u8, a0: u32, a1: u32, a2: u32) -> (u8, u8, [u8; 6]) {
    match s3_rec_shape_of(op).expect("op must be in S3_REC_OP_SHAPE") {
        S3RecShape::SlotImm => s3_rec_slot_imm(op, a0 as u16, a1),
        S3RecShape::SlotSlot => s3_rec_slot_slot(op, a0 as u16, a1 as u16),
        S3RecShape::U32 => s3_rec_u32(op, a0),
        S3RecShape::Slot3 => s3_rec_slot3(op, a0 as u16, a1 as u16, a2 as u16),
    }
}

fn s3_rec_slot_imm(op: u8, slot: u16, imm: u32) -> (u8, u8, [u8; 6]) {
    debug_assert_eq!(s3_rec_shape_of(op), Some(S3RecShape::SlotImm));
    let sb = slot.to_le_bytes();
    let ib = imm.to_le_bytes();
    (op, 2, [sb[0], sb[1], ib[0], ib[1], ib[2], ib[3]])
}
fn s3_rec_slot_slot(op: u8, a: u16, b: u16) -> (u8, u8, [u8; 6]) {
    debug_assert_eq!(s3_rec_shape_of(op), Some(S3RecShape::SlotSlot));
    let aa = a.to_le_bytes();
    let bb = b.to_le_bytes();
    (op, 2, [aa[0], aa[1], bb[0], bb[1], 0, 0])
}
fn s3_rec_u32(op: u8, id: u32) -> (u8, u8, [u8; 6]) {
    debug_assert_eq!(s3_rec_shape_of(op), Some(S3RecShape::U32));
    let b = id.to_le_bytes();
    (op, 1, [b[0], b[1], b[2], b[3], 0, 0])
}
fn s3_rec_slot3(op: u8, a: u16, b: u16, c: u16) -> (u8, u8, [u8; 6]) {
    debug_assert_eq!(s3_rec_shape_of(op), Some(S3RecShape::Slot3));
    let aa = a.to_le_bytes();
    let bb = b.to_le_bytes();
    let cc = c.to_le_bytes();
    (op, 3, [aa[0], aa[1], bb[0], bb[1], cc[0], cc[1]])
}

/// S3.128/S3.133/S3.224/S3.235: named builders dispatch through `s3_rec_from_shape`.
fn s3_rec_set(slot: u16, imm: u32) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x30, slot as u32, imm, 0)
}
fn s3_rec_add(slot: u16, imm: u32) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x62, slot as u32, imm, 0)
}
fn s3_rec_sub(slot: u16, imm: u32) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x61, slot as u32, imm, 0)
}
fn s3_rec_label(id: u32) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x40, id, 0, 0)
}
fn s3_rec_ldb(dst: u16, src: u16, off: u16) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x80, dst as u32, src as u32, off as u32)
}
/// S3.154/S3.156: STB val base off — store low byte of state[val] to data[state[base]+off].
fn s3_rec_stb(val: u16, base: u16, off: u16) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x81, val as u32, base as u32, off as u32)
}
fn s3_rec_cmp(a: u16, b: u16) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x65, a as u32, b as u32, 0)
}
fn s3_rec_movrr(dst: u16, src: u16) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x64, dst as u32, src as u32, 0)
}
fn s3_rec_je(id: u32) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x71, id, 0, 0)
}
fn s3_rec_jne(id: u32) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x72, id, 0, 0)
}
fn s3_rec_jl(id: u32) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x73, id, 0, 0)
}
fn s3_rec_jz(id: u32) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x74, id, 0, 0)
}
fn s3_rec_jmp(id: u32) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x70, id, 0, 0)
}
fn s3_rec_memcpy(dst: u16, src: u16, len: u16) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x84, dst as u32, src as u32, len as u32)
}
fn s3_rec_addv(dst: u16, src: u16) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x68, dst as u32, src as u32, 0)
}
fn s3_rec_subv(dst: u16, src: u16) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x6A, dst as u32, src as u32, 0)
}

/// S3.117: shared scan/compose tail. `nrows` is 10 (host) or 3 (in-DLL).
/// Record stream is golden-pinned for in-DLL — do not edit. Host uses
/// [`s3_prog_loop_code_host`].
fn s3_prog_loop_code(ops_len: u32, nrows: u32) -> Vec<(u8, u8, [u8; 6])> {
    let set = s3_rec_set;
    let add = s3_rec_add;
    vec![
        set(1, ops_len),
        set(0, 0),
        s3_rec_label(5),
        s3_rec_cmp(0, 1),
        s3_rec_jz(6),
        s3_rec_ldb(14, 2, 0),
        s3_rec_movrr(15, 3),
        set(11, 0),
        set(10, nrows),
        s3_rec_label(1),
        s3_rec_ldb(13, 15, 0),
        s3_rec_cmp(13, 14),
        s3_rec_je(2),
        add(15, 20),
        add(11, 1),
        s3_rec_cmp(11, 10),
        s3_rec_jl(1),
        set(12, 0xFE),
        s3_fixed_rec(0xFF),
        s3_rec_label(2),
        s3_rec_ldb(12, 15, 1),
        s3_rec_movrr(8, 15),
        add(8, 4),
        s3_rec_memcpy(9, 8, 12),
        s3_rec_addv(9, 12),
        add(2, 1),
        add(0, 1),
        s3_rec_jmp(5),
        s3_rec_label(6),
        s3_fixed_rec(0xFF),
    ]
}

/// S3.214: host-only prog loop — drop redundant `SET i=0` (fresh state) and
/// reset the scan index with MOVRR from an untouched zero slot. Same semantics
/// as [`s3_prog_loop_code`]; in-DLL golden keeps the older record stream.
fn s3_prog_loop_code_host(ops_len: u32, nrows: u32) -> Vec<(u8, u8, [u8; 6])> {
    let set = s3_rec_set;
    let add = s3_rec_add;
    const ZERO_SLOT: u16 = 4;
    vec![
        set(1, ops_len),
        s3_rec_label(5),
        s3_rec_cmp(0, 1),
        s3_rec_jz(6),
        s3_rec_ldb(14, 2, 0),
        s3_rec_movrr(15, 3),
        s3_rec_movrr(11, ZERO_SLOT),
        set(10, nrows),
        s3_rec_label(1),
        s3_rec_ldb(13, 15, 0),
        s3_rec_cmp(13, 14),
        s3_rec_je(2),
        add(15, 20),
        add(11, 1),
        s3_rec_cmp(11, 10),
        s3_rec_jl(1),
        set(12, 0xFE),
        s3_fixed_rec(0xFF),
        s3_rec_label(2),
        s3_rec_ldb(12, 15, 1),
        s3_rec_movrr(8, 15),
        add(8, 4),
        s3_rec_memcpy(9, 8, 12),
        s3_rec_addv(9, 12),
        add(2, 1),
        add(0, 1),
        s3_rec_jmp(5),
        s3_rec_label(6),
        s3_fixed_rec(0xFF),
    ]
}

/// S3.116: one prog-loop sim. Host (10-row, outbuf 0x160) and in-DLL
/// (3-row, outbuf 0xC8) only differ by these seeds. RAW `0xA1` is skipped.
fn s3_sim_prog_loop(tyb: &[u8], table_off: u64, outbuf: u32, prog_ptr: u64) -> Result<Vec<u8>, u8> {
    let prog = parse_tyb(tyb).map_err(|_| 0xFDu8)?;
    // S3.145: size DATA to packed image (host-sized pack IR needs >512).
    let mut data = Vec::new();
    for rec in &prog.records {
        if matches!(rec.op, 0x10 | 0x12 | 0x13) {
            data.extend_from_slice(&arg_low_bytes(rec));
        }
    }
    if data.len() < 512 {
        data.resize(512, 0);
    }
    let mut state = [0u64; 16];
    state[3] = table_off;
    state[15] = table_off;
    state[9] = outbuf as u64;
    state[2] = prog_ptr;
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
                // S3.118: fulltext re-scans from table base; PE does this with RAW lea.
                if s == 14 {
                    state[15] = table_off;
                }
                pc += 1;
            }
            0x62 => {
                let (s, imm) = arg_slot_imm(rec).ok_or(0xFB)?;
                state[s as usize] = state[s as usize].wrapping_add(imm as u64);
                pc += 1;
            }
            // S3.183: SUB imm — countdown for STB runs (emit_x86 already has 0x61).
            0x61 => {
                let (s, imm) = arg_slot_imm(rec).ok_or(0xFB)?;
                state[s as usize] = state[s as usize].wrapping_sub(imm as u64);
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
            // S3.143: SUBV for loop rem = end - cur in pack IR.
            0x6A => {
                let (d, s) = arg_slot_slot(rec).ok_or(0xFB)?;
                state[d as usize] = state[d as usize].wrapping_sub(state[s as usize]);
                pc += 1;
            }
            0x80 => {
                let (dd, ss, oo) = arg_slot3(rec).ok_or(0xFB)?;
                let addr = state[ss as usize] as usize + oo as usize;
                state[dd as usize] = *data.get(addr).ok_or(0xFA)? as u64;
                pc += 1;
            }
            // S3.154/S3.181/S3.268: STB — high-water mark into [outbuf,…).
            // Staging below outbuf must not inflate; overwrites must not either.
            0x81 => {
                let (val, base, oo) = arg_slot3(rec).ok_or(0xFB)?;
                let addr = state[base as usize] as usize + oo as usize;
                if addr >= data.len() {
                    data.resize(addr + 1, 0);
                }
                data[addr] = state[val as usize] as u8;
                let ob = outbuf as usize;
                if addr >= ob {
                    let end = addr + 1 - ob;
                    if end > composed {
                        composed = end;
                    }
                }
                pc += 1;
            }
            0x84 => {
                let (dst, src, n) = arg_slot3(rec).ok_or(0xFB)?;
                let d = state[dst as usize] as usize;
                let s = state[src as usize] as usize;
                let n = state[n as usize] as usize;
                let tmp = data.get(s..s + n).ok_or(0xFA)?.to_vec();
                // S3.149: grow DATA for outbuf writes beyond packed scratch image.
                if d + n > data.len() {
                    data.resize(d + n, 0);
                }
                data[d..d + n].copy_from_slice(&tmp);
                let ob = outbuf as usize;
                if d + n > ob {
                    let end = d + n - ob;
                    if end > composed {
                        composed = end;
                    }
                }
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
            0x72 => {
                let id = arg_u32(rec).ok_or(0xFB)?;
                if !zf {
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
    let base = outbuf as usize;
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
    if !ops.iter().all(|o| S3_FIXED10_OPS.contains(o)) {
        return None;
    }
    s3_sim_prog_loop(
        &s3_host_prog_loop_tyb(ops),
        S3_HOST_TABLE_OFF as u64,
        S3_HOST_OUTBUF_OFF,
        (S3_HOST_PROG_OFF + 1) as u64,
    )
    .ok()
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
            panic!("skip: {} not present", path.display());
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
        s3_host_tyb_from_records(recs)
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
        assert_eq!(S1_SLIM_MAX_CODE, 1024);
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

    /// S3.109/S3.111: remaining fixed-template ops. Rust `pack_meta_emit` dropped;
    /// YOYO table compose must match `emit_x86` (oracle only).
    #[test]
    fn s3_meta_emit_autopack_fixed_template_rest() {
        let cases: Vec<(&str, Vec<(u8, u8, [u8; 6])>)> = s3_fixed10_named_cases()
            .into_iter()
            .filter(|(name, _)| !matches!(*name, "RET" | "NOP+RET" | "SET+RET" | "GET+RET"))
            .collect();
        for (name, semantic) in cases {
            let tyb = tyb_from_records(&semantic);
            let prog = parse_tyb(&tyb).unwrap();
            assert!(
                fixed_template_ops(&prog).is_some(),
                "{name} must stay inside fixed_template_ops"
            );
            let expect = emit_x86(&prog).text;
            let ops = fixed_template_ops(&prog).unwrap();
            let via = try_emit_text_via_yoyo_table(&ops).unwrap_or_else(|| panic!("{name} yoyo table"));
            assert_eq!(via, expect, "{name} yoyo_table == emit_x86");
            let pe = compile_tyb_to_pe_mini(&tyb).expect("mini");
            let pe_y = try_compile_tyb_via_yoyo_table(&tyb).expect("yoyo pe");
            assert_eq!(pe.bytes, pe_y.bytes, "{name} mini took YOYO path, not emit_x86");
        }
    }

    /// S3.129: each canonical fixed10 record is accepted; acceptance rules stay in `fixed_template_ops`.
    #[test]
    fn s3_fixed_rec_stays_inside_fixed_template_ops() {
        for op in S3_FIXED10_OPS {
            let tyb = s3_host_tyb_from_records(&[s3_fixed_rec(op)]);
            assert_eq!(
                fixed_template_ops(&parse_tyb(&tyb).unwrap()),
                Some(vec![op]),
                "fixed rec {op:#x}"
            );
        }
        let all = s3_fixed10_named_cases();
        assert_eq!(all.len(), 10);
        assert_eq!(
            all.iter().map(|(n, _)| *n).collect::<Vec<_>>(),
            vec![
                "RET", "NOP+RET", "SET+RET", "GET+RET", "INC+RET", "ADD+RET", "DEC+RET",
                "SUB+RET", "MOVRR+RET", "CMP+RET",
            ]
        );
    }

    /// S3.163/S3.176/S3.222/S3.224: IR self-host near-end inventory (not a schedule).
    /// Floors: (1) short lea → `.text` only via A1 (STB/poke can hold the 37B
    /// payload but do not inject `.text`) (2) one SET per distinct emit byte
    /// (no SHR; shared `s3_append_stb_poke_global_distinct`).
    /// Cliff: expand asm is one flat (init+ctrl, S3.275); argc+header IR-seeded; Rust
    /// bootstraps asm; pack still wraps wire→TYB (S3.269). A1 floor.
    #[test]
    fn s3_ir_selfhost_roadmap() {
        // Done: DATA pack algorithm runs as YOYO IR (STB loop) via via_ir.
        let (probe, _) = s3_yoyo_pack_loop_stb_tyb(&[1, 2, 3, 4]);
        assert!(parse_tyb(&probe).unwrap().records.iter().any(|r| r.op == 0x81));
        assert_eq!(s3_pack_data_via_ir(&[9, 8, 7]), s3_pack_data(&[9, 8, 7]));
        // Done: YOYO STB emits full TYB (flat by IR) that mini-PE accepts.
        let body = [s3_fixed_rec(0xFF)];
        let (fw, off) = s3_yoyo_emit_tyb_file(&body);
        assert!(
            !parse_tyb(&fw)
                .unwrap()
                .records
                .iter()
                .any(|r| r.op == 0x84),
            "small RET TYB uses unrolled emit (no MEMCPY)"
        );
        let emitted = s3_sim_prog_loop(&fw, 0, off, 0).unwrap();
        assert_eq!(emitted, s3_host_tyb_from_records(&body));
        assert!(compile_tyb_to_pe_mini(&emitted).is_ok());
        // S3.192/S3.194: both loop packers keep only payload bytes in DATA.
        let (via_fw, via_off) = s3_yoyo_pack_loop_stb_tyb(&[1, 2, 3, 4]);
        assert_eq!(via_off, 6);
        assert!(s3_unpack_data_tyb(&via_fw).is_empty());
        // Floor: in-DLL short lea RAW bytes stay (golden / PE seed).
        assert_eq!(S3_LEA_PTR.len(), 7);
        assert_eq!(S3_LEA_TABLE.len(), 15);
        assert_eq!(S3_LEA_DST.len() + S3_LEA_PROG.len(), 22);
        let dll = parse_tyb(&s3_table_prog_loop_tyb(&[0xFF])).unwrap();
        assert!(dll.records.iter().any(|r| r.op == 0xA1));
        let host = parse_tyb(&s3_host_prog_loop_tyb(&[0xFF])).unwrap();
        assert!(host.records.iter().all(|r| r.op != 0xA1));
        // Floors + O(1) escape hatch (S3.174/S3.175).
        let one = [s3_fixed_rec(0xFF)];
        let (fw_u, _) = s3_yoyo_emit_tyb_file_unrolled(&one);
        assert!(s3_unpack_data_tyb(&fw_u).is_empty());
        // S3.200: runs fold; STB IR count is below flat_len when zeros repeat.
        let stb_u = parse_tyb(&fw_u)
            .unwrap()
            .records
            .iter()
            .filter(|r| r.op == 0x81)
            .count();
        assert!(stb_u < HEADER_SIZE + RECORD_SIZE);
        let (fw_l, _) = s3_yoyo_emit_tyb_file_loop_staged(&one);
        assert!(parse_tyb(&fw_l).unwrap().records.iter().any(|r| r.op == 0x84));
        assert!(s3_unpack_data_tyb(&fw_l).is_empty());
        // S3.181: large emit is two-phase IR staging (zero DATA), not Rust flat.
        let mut big = vec![s3_fixed_rec(0x00); 16];
        big.push(s3_fixed_rec(0xFF));
        let (fw_2, _) = s3_yoyo_emit_tyb_file(&big);
        assert!(parse_tyb(&fw_2).unwrap().records.iter().any(|r| r.op == 0x84));
        assert!(s3_unpack_data_tyb(&fw_2).is_empty());
        // Cliff: `s3_append_stb_runs` / shape fillers remain Rust (S3.224: 4 shapes).
        let _ = s3_rec_stb(0, 1, 0);
        assert_eq!(s3_fixed_row(0xFF), s3_compute_fixed_body(0xFF));
    }

    /// S3.225/S3.226: unrolled + two-phase + loop-staged share STB-flat phase1 prefix.
    #[test]
    fn s3_emit_flat_via_stb_shared() {
        let body = [s3_fixed_rec(0xFF)];
        let (recs_tyb, recs_off) = s3_yoyo_emit_recs_tyb(&body);
        let (file_tyb, file_off) = s3_yoyo_emit_tyb_file_unrolled(&body);
        assert_eq!(recs_off, 0x20);
        assert_eq!(file_off, 0x20);
        let recs_ir = parse_tyb(&recs_tyb).unwrap();
        let file_ir = parse_tyb(&file_tyb).unwrap();
        // S3.279: prologue is SET DST then STB runs (slot13 image-zero; no SET ZERO).
        assert_eq!(recs_ir.records[0].op, 0x30);
        assert!(recs_ir.records.iter().any(|r| r.op == 0x81));
        assert_eq!(recs_ir.records.last().unwrap().op, 0xFF);
        assert_eq!(file_ir.records[0].op, 0x30);
        assert!(file_ir.records.iter().any(|r| r.op == 0x81));
        assert_eq!(file_ir.records.last().unwrap().op, 0xFF);
        assert!(s3_unpack_data_tyb(&recs_tyb).is_empty());
        assert!(s3_unpack_data_tyb(&file_tyb).is_empty());
        // Shared helper matches hand-built path for a tiny flat.
        let flat = s3_rec_wire(0xFF, 0, [0; 6]);
        let (via, off) = s3_yoyo_emit_flat_via_stb(&flat, 0x20);
        assert_eq!(off, 0x20);
        assert_eq!(
            s3_sim_prog_loop(&via, 0, off, 0).unwrap(),
            flat.to_vec()
        );
        // S3.226/S3.279: two-phase / loop-staged open with SET DST (no SET ZERO).
        let mut big = vec![s3_fixed_rec(0x00); 16];
        big.push(s3_fixed_rec(0xFF));
        let (fw2, _) = s3_yoyo_emit_tyb_file_two_phase(&big);
        let (fwl, _) = s3_yoyo_emit_tyb_file_loop_staged(&body);
        for (name, tyb) in [("two_phase", &fw2), ("loop_staged", &fwl)] {
            let ir = parse_tyb(tyb).unwrap();
            assert_eq!(ir.records[0].op, 0x30, "{name} DST");
            assert!(ir.records.iter().any(|r| r.op == 0x81), "{name} STB");
            assert!(ir.records.iter().any(|r| r.op == 0x84), "{name} MEMCPY");
            assert!(s3_unpack_data_tyb(tyb).is_empty(), "{name} empty DATA");
        }
    }

    /// S3.224/S3.228: named builders are thin op tags over 4 wire shapes; op→shape
    /// table is the YOYO-target data plane (16 ops). Rust still fills bytes.
    #[test]
    fn s3_rec_builder_shapes_floor() {
        assert_eq!(s3_rec_set(3, 0x11), s3_rec_slot_imm(0x30, 3, 0x11));
        assert_eq!(s3_rec_add(3, 5), s3_rec_slot_imm(0x62, 3, 5));
        assert_eq!(s3_rec_sub(3, 5), s3_rec_slot_imm(0x61, 3, 5));
        assert_eq!(s3_rec_movrr(1, 2), s3_rec_slot_slot(0x64, 1, 2));
        assert_eq!(s3_rec_cmp(0, 1), s3_rec_slot_slot(0x65, 0, 1));
        assert_eq!(s3_rec_addv(9, 12), s3_rec_slot_slot(0x68, 9, 12));
        assert_eq!(s3_rec_subv(4, 5), s3_rec_slot_slot(0x6A, 4, 5));
        assert_eq!(s3_rec_label(7), s3_rec_u32(0x40, 7));
        assert_eq!(s3_rec_jmp(7), s3_rec_u32(0x70, 7));
        assert_eq!(s3_rec_je(7), s3_rec_u32(0x71, 7));
        assert_eq!(s3_rec_jne(7), s3_rec_u32(0x72, 7));
        assert_eq!(s3_rec_jl(7), s3_rec_u32(0x73, 7));
        assert_eq!(s3_rec_jz(7), s3_rec_u32(0x74, 7));
        assert_eq!(s3_rec_ldb(1, 2, 3), s3_rec_slot3(0x80, 1, 2, 3));
        assert_eq!(s3_rec_stb(11, 14, 5), s3_rec_slot3(0x81, 11, 14, 5));
        assert_eq!(s3_rec_memcpy(9, 10, 11), s3_rec_slot3(0x84, 9, 10, 11));
        // Shape argc pins (YOYO-target): imm/slot=2, u32=1, slot³=3.
        assert_eq!(s3_rec_slot_imm(0x30, 0, 0).1, 2);
        assert_eq!(s3_rec_slot_slot(0x64, 0, 0).1, 2);
        assert_eq!(s3_rec_u32(0x40, 0).1, 1);
        assert_eq!(s3_rec_slot3(0x81, 0, 0, 0).1, 3);
        // S3.228/S3.289: op→shape table covers every named builder op exactly once.
        assert_eq!(S3_REC_OP_SHAPE.len(), 16);
        assert_eq!(s3_rec_shape_of(0x30), Some(S3RecShape::SlotImm));
        assert_eq!(s3_rec_shape_of(0x64), Some(S3RecShape::SlotSlot));
        assert_eq!(s3_rec_shape_of(0x40), Some(S3RecShape::U32));
        assert_eq!(s3_rec_shape_of(0x81), Some(S3RecShape::Slot3));
        assert_eq!(s3_rec_shape_of(0xFF), None, "RET is not a shaped builder");
        let mut ops: Vec<u8> = S3_REC_OP_SHAPE.iter().map(|(o, _)| *o).collect();
        let before = ops.len();
        ops.sort_unstable();
        ops.dedup();
        assert_eq!(ops.len(), before, "op→shape table has unique ops");
        let mut by_shape = [0usize; 4];
        for (_, s) in S3_REC_OP_SHAPE {
            by_shape[*s as usize] += 1;
        }
        assert_eq!(by_shape, [3, 4, 6, 3], "SlotImm/SlotSlot/U32/Slot3 counts");
        // S3.235: table-driven dispatch matches every named builder.
        assert_eq!(s3_rec_from_shape(0x30, 3, 0x11, 0), s3_rec_set(3, 0x11));
        assert_eq!(s3_rec_from_shape(0x62, 3, 5, 0), s3_rec_add(3, 5));
        assert_eq!(s3_rec_from_shape(0x61, 3, 5, 0), s3_rec_sub(3, 5));
        assert_eq!(s3_rec_from_shape(0x64, 1, 2, 0), s3_rec_movrr(1, 2));
        assert_eq!(s3_rec_from_shape(0x65, 0, 1, 0), s3_rec_cmp(0, 1));
        assert_eq!(s3_rec_from_shape(0x68, 9, 12, 0), s3_rec_addv(9, 12));
        assert_eq!(s3_rec_from_shape(0x6A, 4, 5, 0), s3_rec_subv(4, 5));
        assert_eq!(s3_rec_from_shape(0x40, 7, 0, 0), s3_rec_label(7));
        assert_eq!(s3_rec_from_shape(0x70, 7, 0, 0), s3_rec_jmp(7));
        assert_eq!(s3_rec_from_shape(0x71, 7, 0, 0), s3_rec_je(7));
        assert_eq!(s3_rec_from_shape(0x72, 7, 0, 0), s3_rec_jne(7));
        assert_eq!(s3_rec_from_shape(0x73, 7, 0, 0), s3_rec_jl(7));
        assert_eq!(s3_rec_from_shape(0x74, 7, 0, 0), s3_rec_jz(7));
        assert_eq!(s3_rec_from_shape(0x80, 1, 2, 3), s3_rec_ldb(1, 2, 3));
        assert_eq!(s3_rec_from_shape(0x81, 11, 14, 5), s3_rec_stb(11, 14, 5));
        assert_eq!(s3_rec_from_shape(0x84, 9, 10, 11), s3_rec_memcpy(9, 10, 11));
    }

    /// S3.236: YOYO IR looks up each builder op and emits the 8-byte record wire.
    /// Argc comes from the scanned shape tag. Rust still assembles the scan IR.
    #[test]
    fn s3_yoyo_shape_fill_record_matches_rust_wire() {
        let samples: &[(u8, [u8; 6])] = &[
            (0x30, [3, 0, 0x11, 0, 0, 0]),
            (0x61, [3, 0, 5, 0, 0, 0]),
            (0x62, [3, 0, 5, 0, 0, 0]),
            (0x40, [7, 0, 0, 0, 0, 0]),
            (0x70, [7, 0, 0, 0, 0, 0]),
            (0x71, [7, 0, 0, 0, 0, 0]),
            (0x72, [7, 0, 0, 0, 0, 0]),
            (0x73, [7, 0, 0, 0, 0, 0]),
            (0x74, [7, 0, 0, 0, 0, 0]),
            (0x64, [1, 0, 2, 0, 0, 0]),
            (0x65, [0, 0, 1, 0, 0, 0]),
            (0x68, [9, 0, 12, 0, 0, 0]),
            (0x6A, [4, 0, 5, 0, 0, 0]),
            (0x80, [1, 0, 2, 0, 3, 0]),
            (0x81, [11, 0, 14, 0, 5, 0]),
            (0x84, [9, 0, 10, 0, 11, 0]),
        ];
        assert_eq!(samples.len(), S3_REC_OP_SHAPE.len());
        for &(op, args) in samples {
            let (tyb, out) = s3_yoyo_shape_fill_tyb(op, args);
            assert!(s3_unpack_data_tyb(&tyb).is_empty(), "op {op:#x} empty DATA");
            let ir = parse_tyb(&tyb).unwrap();
            assert!(ir.records.iter().any(|r| r.op == 0x80), "op {op:#x} scans via LDB");
            assert!(
                ir.records.iter().all(|r| r.op != 0x10),
                "op {op:#x} no DATA"
            );
            let shape = s3_rec_shape_of(op).unwrap();
            let argc = match shape {
                S3RecShape::SlotImm | S3RecShape::SlotSlot => 2,
                S3RecShape::U32 => 1,
                S3RecShape::Slot3 => 3,
            };
            let expect = s3_rec_wire(op, argc, args);
            assert_eq!(
                s3_sim_prog_loop(&tyb, 0, out, 0).unwrap(),
                expect.to_vec(),
                "op {op:#x} shape {shape:?}"
            );
            // S3.238: op/args live in the job poke. Control SETs neither slot 7 nor slot 12.
            let needle_sets = ir
                .records
                .iter()
                .filter(|r| {
                    r.op == 0x30
                        && u16::from_le_bytes([r.args_raw[0], r.args_raw[1]]) == 7
                })
                .count();
            let byte_sets = ir
                .records
                .iter()
                .filter(|r| {
                    r.op == 0x30
                        && u16::from_le_bytes([r.args_raw[0], r.args_raw[1]]) == 12
                })
                .count();
            assert_eq!(needle_sets, 0, "op {op:#x} needle is a load");
            assert_eq!(byte_sets, 0, "op {op:#x} wire bytes are loads");
            // S3.286: argc from matched `(op,argc)` pair via STB (no shape-tag ADDV).
            assert!(
                ir.records.iter().any(|r| r.op == 0x81),
                "op {op:#x} emits STB for op/argc"
            );
        }
        assert_eq!(s3_shape_argc_bytes(), [2, 2, 1, 3]);
        let (a, _) = s3_yoyo_shape_fill_tyb(0x30, [3, 0, 0x11, 0, 0, 0]);
        let (b, _) = s3_yoyo_shape_fill_tyb(0x81, [11, 0, 14, 0, 5, 0]);
        let ctrl = s3_yoyo_shape_fill_ctrl();
        let tail = |tyb: &[u8]| {
            let recs = parse_tyb(tyb).unwrap().records;
            let start = recs.len() - ctrl.len();
            recs[start..]
                .iter()
                .map(|r| (r.op, r.argc, r.args_raw))
                .collect::<Vec<_>>()
        };
        assert_eq!(tail(&a), tail(&b), "control IR does not depend on the job");
        assert_eq!(tail(&a), ctrl);
    }

    /// S3.239: one fill body, count-prefixed blob, many records in order.
    #[test]
    fn s3_yoyo_shape_fill_many_matches_rust_wire() {
        let jobs: [(u8, [u8; 6]); 4] = [
            (0x30, [3, 0, 0x11, 0, 0, 0]),
            (0x40, [7, 0, 0, 0, 0, 0]),
            (0x64, [1, 0, 2, 0, 0, 0]),
            (0x81, [11, 0, 14, 0, 5, 0]),
        ];
        let (tyb, out) = s3_yoyo_shape_fill_many_tyb(&jobs);
        assert!(s3_unpack_data_tyb(&tyb).is_empty());
        let mut expect = Vec::new();
        for &(op, args) in &jobs {
            let shape = s3_rec_shape_of(op).unwrap();
            let argc = match shape {
                S3RecShape::SlotImm | S3RecShape::SlotSlot => 2,
                S3RecShape::U32 => 1,
                S3RecShape::Slot3 => 3,
            };
            expect.extend_from_slice(&s3_rec_wire(op, argc, args));
        }
        assert_eq!(s3_sim_prog_loop(&tyb, 0, out, 0).unwrap(), expect);
        let ir = parse_tyb(&tyb).unwrap();
        let one = s3_yoyo_shape_fill_one();
        let bodies = ir.records.iter().filter(|r| r.op == 0x40).count();
        // one body has 4 labels; outer adds 2.
        assert_eq!(bodies, one.iter().filter(|r| r.0 == 0x40).count() + 2);
        let ctrl = s3_yoyo_shape_fill_many_ctrl();
        let (other, _) = s3_yoyo_shape_fill_many_tyb(&jobs[..1]);
        let tail = |tyb: &[u8]| {
            let recs = parse_tyb(tyb).unwrap().records;
            let start = recs.len() - ctrl.len();
            recs[start..]
                .iter()
                .map(|r| (r.op, r.argc, r.args_raw))
                .collect::<Vec<_>>()
        };
        assert_eq!(tail(&tyb), tail(&other), "many-ctrl ignores job count");
        assert_eq!(tail(&tyb), ctrl);
    }

    /// S3.241–S3.244: flat seed expands to wire args; argc is a table lookup.
    #[test]
    fn s3_shape_fill_seed_roundtrip() {
        assert_eq!(S3_SHAPE_FILL_MANY_FLAT.len(), S3_SHAPE_FILL_MANY_ROWS * 7);
        let many = s3_yoyo_shape_fill_many_ctrl();
        assert_eq!(many.len(), S3_SHAPE_FILL_MANY_ROWS);
        for (i, rec) in many.iter().enumerate() {
            let row = &S3_SHAPE_FILL_MANY_FLAT[i * 7..i * 7 + 7];
            assert_eq!(rec.0, row[0]);
            assert_eq!(rec.1, s3_seed_argc(row[0]));
            assert_eq!(rec.2, [row[1], row[2], row[3], row[4], row[5], row[6]]);
        }
        let one = s3_yoyo_shape_fill_one();
        assert_eq!(one.len(), S3_SHAPE_FILL_ONE_LEN);
        assert_eq!(
            one,
            many[S3_SHAPE_FILL_ONE_AT..S3_SHAPE_FILL_ONE_AT + S3_SHAPE_FILL_ONE_LEN]
        );
        assert_eq!(many.last().unwrap().0, 0xFF);
        assert_eq!(s3_seed_argc(0xFF), 0);
        assert_eq!(s3_seed_argc(0x30), 2);
        assert_eq!(s3_seed_argc(0x40), 1);
        assert_eq!(s3_seed_argc(0x81), 3);
        let argc_seed = s3_seed_argc_flat();
        assert!(argc_seed.iter().any(|&(o, n)| o == 0xFF && n == 0));
        assert_eq!(argc_seed.len(), S3_REC_OP_SHAPE.len() + 1);
        for &(op, n) in &argc_seed {
            assert_eq!(s3_seed_argc(op), n);
        }
        // Frozen blob stays in lockstep with the shape table + RET.
        let mut derived = Vec::new();
        for &(op, shape) in S3_REC_OP_SHAPE {
            let n = match shape {
                S3RecShape::SlotImm | S3RecShape::SlotSlot => 2,
                S3RecShape::U32 => 1,
                S3RecShape::Slot3 => 3,
            };
            derived.push(op);
            derived.push(n);
        }
        derived.extend_from_slice(&[0xFF, 0]);
        assert_eq!(S3_SEED_ARGC_BYTES, derived.as_slice());
        // Expander does not re-encode through s3_rec_from_shape.
        let row = [0x30u8, 2, 0, 104, 0, 0, 0];
        assert_eq!(
            s3_rec_from_wire_seed(&row),
            (0x30, 2, [2, 0, 104, 0, 0, 0])
        );
        // S3.246/S3.287: one-ctrl = prefix(1)||body||LABEL6||RET of many.
        assert_eq!(S3_SHAPE_FILL_ONE_FLAT.len(), S3_SHAPE_FILL_ONE_ROWS * 7);
        let ctrl = s3_yoyo_shape_fill_ctrl();
        assert_eq!(ctrl.len(), S3_SHAPE_FILL_ONE_ROWS);
        assert_eq!(ctrl, s3_recs_from_flat_seed(S3_SHAPE_FILL_ONE_FLAT));
        let mut rebuilt = Vec::new();
        rebuilt.extend_from_slice(&S3_SHAPE_FILL_ONE_FLAT[..7]);
        let start = S3_SHAPE_FILL_ONE_AT * 7;
        let end = start + S3_SHAPE_FILL_ONE_LEN * 7;
        rebuilt.extend_from_slice(&S3_SHAPE_FILL_MANY_FLAT[start..end]);
        rebuilt.extend_from_slice(&S3_SHAPE_FILL_MANY_FLAT[S3_SHAPE_FILL_MANY_FLAT.len() - 14..]);
        assert_eq!(rebuilt.as_slice(), S3_SHAPE_FILL_ONE_FLAT);
        assert_eq!(
            s3_recs_from_flat_seed_with_argc(S3_SHAPE_FILL_MANY_FLAT, S3_SEED_ARGC_BYTES),
            s3_yoyo_shape_fill_many_ctrl()
        );
    }

    /// S3.245/S3.247: control flat seeds are STB-emittable empty-DATA payloads.
    #[test]
    fn s3_shape_fill_flat_stb_payload() {
        let (tyb, off) = s3_yoyo_emit_flat_via_stb(S3_SHAPE_FILL_MANY_FLAT, 0x20);
        assert!(s3_unpack_data_tyb(&tyb).is_empty());
        assert_eq!(
            s3_sim_prog_loop(&tyb, 0, off, 0).unwrap(),
            S3_SHAPE_FILL_MANY_FLAT
        );
        assert_eq!(
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT),
            s3_yoyo_shape_fill_many_ctrl()
        );
        let (tyb1, off1) = s3_yoyo_emit_flat_via_stb(S3_SHAPE_FILL_ONE_FLAT, 0x20);
        assert!(s3_unpack_data_tyb(&tyb1).is_empty());
        assert_eq!(
            s3_sim_prog_loop(&tyb1, 0, off1, 0).unwrap(),
            S3_SHAPE_FILL_ONE_FLAT
        );
        assert_eq!(
            s3_recs_from_flat_seed(S3_SHAPE_FILL_ONE_FLAT),
            s3_yoyo_shape_fill_ctrl()
        );
        // S3.256/S3.257: expand ctrl flat seed; job ends with 0xFE sentinel (no SET END).
        assert_eq!(S3_EXPAND_CTRL_FLAT.len(), S3_EXPAND_CTRL_ROWS * 7);
        assert_eq!(s3_recs_from_flat_seed(S3_EXPAND_CTRL_FLAT).len(), S3_EXPAND_CTRL_ROWS);
        assert_eq!(s3_recs_from_flat_seed(S3_EXPAND_CTRL_FLAT).last().unwrap().0, 0xFF);
        let (exp_stb, exp_off) = s3_yoyo_emit_flat_via_stb(S3_EXPAND_CTRL_FLAT, 0x20);
        assert!(s3_unpack_data_tyb(&exp_stb).is_empty());
        assert_eq!(
            s3_sim_prog_loop(&exp_stb, 0, exp_off, 0).unwrap(),
            S3_EXPAND_CTRL_FLAT
        );
        // S3.254/S3.259/S3.264: YOYO expand matches Rust; no job SET END; size floor.
        let (exp, off) = s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_ONE_FLAT);
        assert!(s3_unpack_data_tyb(&exp).is_empty());
        let exp_recs = parse_tyb(&exp).unwrap().records;
        // S3.283: pin ONE-flat assembler size (in-place seed ptr; no MOVRR s8).
        assert_eq!(exp_recs.len(), 210);
        assert_eq!(S3_EXPAND_ASM_ROWS, 31);
        assert_eq!(S3_EXPAND_INIT_ROWS, 4);
        assert_eq!(
            s3_recs_from_flat_seed(S3_EXPAND_INIT_FLAT),
            vec![
                s3_rec_set(0, S3_EXPAND_OUT),
                s3_rec_set(4, S3_EXPAND_SEED),
                s3_rec_set(5, S3_EXPAND_SENTINEL as u32),
                s3_rec_set(9, S3_EXPAND_OUT + HEADER_SIZE as u32),
            ]
        );
        assert_eq!(
            s3_bootstrap_expand_asm(),
            [
                s3_recs_from_flat_seed(S3_EXPAND_INIT_FLAT),
                s3_recs_from_flat_seed(S3_EXPAND_CTRL_FLAT)
            ]
            .concat()
        );
        assert_eq!(S3_EXPAND_TYB_MAGIC, b"TYB");
        // Sentinel termination lives in the ctrl body (slot 5 = 0xFE), not a length SET.
        assert!(
            !s3_recs_from_flat_seed(S3_EXPAND_CTRL_FLAT).iter().any(|r| {
                r.0 == 0x30
                    && u16::from_le_bytes([r.2[0], r.2[1]]) == 5
                    && u32::from_le_bytes([r.2[2], r.2[3], r.2[4], r.2[5]])
                        != S3_EXPAND_SENTINEL as u32
            }),
            "expand ctrl must not SET slot5 to a length; sentinel owns termination"
        );
        let mut expect_tyb = Vec::new();
        expect_tyb.extend_from_slice(&TYB_MAGIC);
        expect_tyb.extend_from_slice(&[0, 0]);
        let one_recs = s3_recs_from_flat_seed(S3_SHAPE_FILL_ONE_FLAT);
        expect_tyb.extend_from_slice(&(one_recs.len() as u16).to_le_bytes());
        for r in &one_recs {
            expect_tyb.extend_from_slice(&s3_rec_wire(r.0, r.1, r.2));
        }
        assert_eq!(s3_sim_prog_loop(&exp, 0, off, 0).unwrap(), expect_tyb);
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT),
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT)
        );
        // S3.250: argc seed blob STB-emits and round-trips.
        let argc_bytes = s3_seed_argc_bytes();
        assert_eq!(argc_bytes.len(), (S3_REC_OP_SHAPE.len() + 1) * 2);
        assert_eq!(argc_bytes[argc_bytes.len() - 2..], [0xFF, 0]);
        let (tyb_a, off_a) = s3_yoyo_emit_flat_via_stb(&argc_bytes, 0x20);
        assert!(s3_unpack_data_tyb(&tyb_a).is_empty());
        assert_eq!(s3_sim_prog_loop(&tyb_a, 0, off_a, 0).unwrap(), argc_bytes);
    }

    /// S3.258/S3.261/S3.264: expand ctrl body re-encodes via shape-fill.
    /// S3.264 dropped argc-poke rows; body-only jobs fit well under the 128 cap.
    #[test]
    fn s3_yoyo_expand_ctrl_reencodes_via_shape_fill() {
        assert_eq!(S3_EXPAND_CTRL_FLAT.len(), S3_EXPAND_CTRL_ROWS * 7);
        assert_eq!(S3_EXPAND_CTRL_ROWS, 27);
        let ctrl = s3_recs_from_flat_seed(S3_EXPAND_CTRL_FLAT);
        assert_eq!(ctrl.len(), S3_EXPAND_CTRL_ROWS);
        assert_eq!(ctrl.last().unwrap().0, 0xFF);
        let jobs: Vec<(u8, [u8; 6])> = ctrl
            .iter()
            .filter(|r| r.0 != 0xFF)
            .map(|r| (r.0, r.2))
            .collect();
        assert_eq!(jobs.len(), S3_EXPAND_CTRL_ROWS - 1);
        assert!(jobs.len() < 64 && jobs.len() <= 128);
        let (tyb, out) = s3_yoyo_shape_fill_many_tyb(&jobs);
        let mut expect = Vec::new();
        for &(op, args) in &jobs {
            let argc = match s3_rec_shape_of(op).expect("shaped") {
                S3RecShape::SlotImm | S3RecShape::SlotSlot => 2,
                S3RecShape::U32 => 1,
                S3RecShape::Slot3 => 3,
            };
            expect.extend_from_slice(&s3_rec_wire(op, argc, args));
        }
        assert_eq!(s3_sim_prog_loop(&tyb, 0, out, 0).unwrap(), expect);
    }

    /// S3.240: many-fill re-encodes its own control records. RET is not in the
    /// shape table, so it stays outside the job blob.
    #[test]
    fn s3_yoyo_shape_fill_reencodes_own_ctrl() {
        let ctrl = s3_yoyo_shape_fill_many_ctrl();
        assert!(ctrl.iter().any(|r| r.0 == 0xFF));
        let jobs: Vec<(u8, [u8; 6])> = ctrl
            .iter()
            .filter(|r| r.0 != 0xFF)
            .map(|r| (r.0, r.2))
            .collect();
        assert!(jobs.len() > 15 && jobs.len() <= 128, "jobs {}", jobs.len());
        let (tyb, out) = s3_yoyo_shape_fill_many_tyb(&jobs);
        let mut expect = Vec::new();
        for &(op, args) in &jobs {
            let argc = match s3_rec_shape_of(op).expect("shaped") {
                S3RecShape::SlotImm | S3RecShape::SlotSlot => 2,
                S3RecShape::U32 => 1,
                S3RecShape::Slot3 => 3,
            };
            expect.extend_from_slice(&s3_rec_wire(op, argc, args));
        }
        assert_eq!(s3_sim_prog_loop(&tyb, 0, out, 0).unwrap(), expect);
    }

    /// S3.229: op→shape table packs as empty-DATA STB flat and round-trips.
    #[test]
    fn s3_rec_op_shape_table_stb_payload() {
        let bytes = s3_rec_op_shape_bytes();
        assert_eq!(bytes.len(), 32, "16 ops × (op, shape)");
        assert_eq!(&bytes[..4], &[0x30, 0, 0x61, 0]);
        assert_eq!(&bytes[bytes.len() - 2..], &[0x84, 3]);
        // Empty-DATA emit of the table payload; sim recovers the same bytes.
        let (tyb, off) = s3_yoyo_emit_flat_via_stb(&bytes, 0x20);
        assert!(s3_unpack_data_tyb(&tyb).is_empty());
        assert_eq!(s3_sim_prog_loop(&tyb, 0, off, 0).unwrap(), bytes);
        // Rebuild lookup from the payload bytes alone.
        let mut rebuilt = Vec::new();
        for chunk in bytes.chunks_exact(2) {
            let shape = match chunk[1] {
                0 => S3RecShape::SlotImm,
                1 => S3RecShape::SlotSlot,
                2 => S3RecShape::U32,
                3 => S3RecShape::Slot3,
                o => panic!("bad shape tag {o}"),
            };
            rebuilt.push((chunk[0], shape));
        }
        assert_eq!(rebuilt.as_slice(), S3_REC_OP_SHAPE);
    }

    /// S3.230: op→shape payload also seeds via shared global-distinct poke (empty DATA).
    /// Shape tag 0 stays image-zero (poke skips); nonzero ops/tags get one SET each.
    #[test]
    fn s3_rec_op_shape_table_ir_seed_poke() {
        let bytes = s3_rec_op_shape_bytes();
        let distinct: std::collections::BTreeSet<u8> =
            bytes.iter().copied().filter(|&b| b != 0).collect();
        assert!(distinct.len() < bytes.len(), "shape tag 0 leaves some zeros");
        let mut recs = Vec::new();
        s3_ir_seed_at(&mut recs, 0x200, &bytes, 10);
        recs.push((0xFF, 0, [0; 6]));
        let tyb = s3_host_tyb_from_records(&recs);
        assert!(s3_unpack_data_tyb(&tyb).is_empty());
        let tmp_sets = recs
            .iter()
            .filter(|r| {
                r.0 == 0x30 && u16::from_le_bytes([r.2[0], r.2[1]]) == S3_PACK_STB_TMP
            })
            .count();
        assert_eq!(tmp_sets, distinct.len(), "one SET TMP per distinct nonzero");
        assert!(recs.iter().any(|r| {
            r.0 == 0x30 && u16::from_le_bytes([r.2[0], r.2[1]]) == S3_PACK_STB_BASE
        }));
        // Same poke stream as the shared helper after SET BASE.
        let mut via = Vec::new();
        via.push(s3_rec_set(S3_PACK_STB_BASE, 0x200));
        s3_append_stb_poke_global_distinct(&mut via, &bytes);
        assert_eq!(
            &recs[..recs.len() - 1],
            via.as_slice(),
            "ir_seed_at short path == SET BASE + shared poke"
        );
    }

    /// S3.231: poke-seed op→shape at 0x200, then MEMCPY to outbuf — sim readback.
    #[test]
    fn s3_rec_op_shape_table_poke_memcpy_readback() {
        let bytes = s3_rec_op_shape_bytes();
        let base = 0x200u32;
        let out = 0x300u32;
        let mut recs = Vec::new();
        s3_ir_seed_at(&mut recs, base, &bytes, 10);
        recs.push(s3_rec_set(S3_PACK_LOOP_CUR, base));
        recs.push(s3_rec_set(S3_PACK_LOOP_END, bytes.len() as u32));
        recs.push(s3_rec_set(S3_PACK_PROTO_DST, out));
        recs.push(s3_rec_memcpy(
            S3_PACK_PROTO_DST,
            S3_PACK_LOOP_CUR,
            S3_PACK_LOOP_END,
        ));
        recs.push((0xFF, 0, [0; 6]));
        let tyb = s3_host_tyb_from_records(&recs);
        assert!(s3_unpack_data_tyb(&tyb).is_empty());
        assert!(parse_tyb(&tyb).unwrap().records.iter().any(|r| r.op == 0x84));
        assert_eq!(s3_sim_prog_loop(&tyb, 0, out, 0).unwrap(), bytes);
    }

    /// S3.135/S3.160/S3.223/S3.259: remaining Rust sidecar after pack+STB+expand.
    /// Still Rust: short lea A1 (in-DLL PE seed), `emit_x86` body OnceLock,
    /// expand-asm bootstrap (`s3_bootstrap_expand_asm`, S3.275/276), pack wire section wrap
    /// (`s3_pack_wire_section_to_recs`, S3.269), IR builders / run folders. Not CLOSED.
    #[test]
    fn s3_fixed10_fold_remaining_rust_surface() {
        assert_eq!(S3_LEA_PTR.len(), 7, "short lea must stay 7B (not disp32)");
        assert_eq!(S3_LEA_PTR, &[0x49, 0x8D, 0x07, 0x49, 0x89, 0x47, 0x78]);
        // S3.275: assembler body is one flat seed (init prefix + ctrl); still Rust.
        assert_eq!(s3_expand_asm_flat().len(), S3_EXPAND_ASM_ROWS * 7);
        assert_eq!(S3_EXPAND_ASM_ROWS, 31, "S3.289: init 4 + ctrl 27");
        assert_eq!(S3_EXPAND_CTRL_FLAT.len(), S3_EXPAND_CTRL_ROWS * 7);
        assert_eq!(S3_EXPAND_CTRL_ROWS, 27, "S3.289: JNE fall-through match");
        assert_eq!(S3_EXPAND_INIT_ROWS, 4, "S3.277: drop zero INIT SETs");
        assert_eq!(S3_SHAPE_TABLE_END, 0x62, "S3.289: (op,argc) table is 34B");
        assert_eq!(S3_SHAPE_FILL_ONE_ROWS, 20, "S3.289: JNE fall-through");
        // S3.276: ctrl-only expand is plain flat_seed (no named bootstrap).
        assert_eq!(
            s3_recs_from_flat_seed(S3_EXPAND_CTRL_FLAT),
            s3_recs_from_flat_seed_with_argc(S3_EXPAND_CTRL_FLAT, S3_SEED_ARGC_BYTES)
        );
        assert_eq!(
            s3_bootstrap_expand_asm(),
            s3_recs_from_flat_seed_with_argc(&s3_expand_asm_flat(), S3_SEED_ARGC_BYTES)
        );
        assert_eq!(S3_SEED_ARGC_BYTES.len() as u32, 0x1E2 - S3_EXPAND_ARGC);
        // S3.289: ONE-flat expand assembler size (210; JNE fall-through + shorter job).
        let (exp_pin, _) = s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_ONE_FLAT);
        assert_eq!(parse_tyb(&exp_pin).unwrap().records.len(), 210);
        let one = s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_ONE_FLAT);
        assert_eq!(one, s3_recs_from_flat_seed(S3_SHAPE_FILL_ONE_FLAT));
        // S3.269: pack wire-section wrap; expand already emits TYB (no wrap).
        for r in &one {
            let w = s3_rec_wire(r.0, r.1, r.2);
            assert_eq!(s3_pack_wire_section_to_recs(&w), vec![*r]);
        }
        assert_eq!(S3_EXPAND_SEED, 0x40);
        assert_eq!(S3_EXPAND_ARGC, 0x1C0);
        assert_eq!(S3_EXPAND_OUT, 0x200);
        assert_eq!(S3_EXPAND_SENTINEL, 0xFE);
        assert_eq!(S3_EXPAND_ARGC - S3_EXPAND_SEED, 0x180);
        let (exp, off) = s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_ONE_FLAT);
        let emitted = s3_sim_prog_loop(&exp, 0, off, 0).unwrap();
        let parsed = parse_tyb(&emitted).unwrap();
        assert_eq!(
            parsed
                .records
                .iter()
                .map(|r| (r.op, r.argc, r.args_raw))
                .collect::<Vec<_>>(),
            one
        );
        // S3.223/S3.234: in-DLL three-lea RAW floor size — PE seed cliff.
        let lea_floor = s3_short_lea_raw_bytes().len();
        assert_eq!(lea_floor, 37, "short lea RAW floor stays 37B");
        assert_eq!(
            lea_floor,
            S3_LEA_TABLE.len() + S3_LEA_DST.len() + S3_LEA_PROG.len()
        );
        let host = parse_tyb(&s3_host_prog_loop_tyb(&[0xFF])).unwrap();
        let host_data: Vec<_> = host.records.iter().filter(|r| matches!(r.op, 0x10 | 0x12 | 0x13)).collect();
        assert!(host_data.is_empty(), "S3.205: host firmware has no DATA seed");
        assert_eq!(
            s3_sim_prog_loop(
                &s3_host_prog_loop_tyb(&[0xFF]),
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
            )
            .unwrap(),
            s3_compute_fixed_body(0xFF),
            "host IR-seeded table still emits RET body"
        );
        assert_eq!(
            s3_pack_data(&[0xAA, 0xBB, 0xCC, 0xDD]),
            s3_host_pack_bytes(0x10, &[0xAA, 0xBB, 0xCC, 0xDD])
        );
        assert!(host.records.iter().all(|r| r.op != 0xA1));
        let dll = parse_tyb(&s3_table_prog_loop_tyb(&[0xFF])).unwrap();
        assert!(dll.records.iter().all(|r| r.op != 0x10), "S3.221: default in-DLL empty-DATA");
        assert!(dll.records.iter().any(|r| r.op == 0x81), "S3.222: seed uses STB poke");
        let a1 = dll.records.iter().filter(|r| r.op == 0xA1).count();
        assert!(a1 >= 3, "in-DLL keeps ≥3 RAW lea packs for PE, got {a1}");
        let a1_bytes: usize = dll
            .records
            .iter()
            .filter(|r| r.op == 0xA1)
            .map(|r| r.argc as usize)
            .sum();
        assert_eq!(a1_bytes, lea_floor, "A1 payload bytes == lea floor");
        assert_eq!(
            s3_fixed_payload(&S3_FIXED10_OPS, &[0xFF], S3_HOST_OUTBUF_OFF as usize).len(),
            S3_HOST_OUTBUF_OFF as usize
        );
        // S3.194: MEMCPY loop scratch is payload+2; five consts are IR stores past that.
        let (cmp, out_off) = s3_yoyo_pack_loop_tyb(&[0xAB, 0xCD]);
        assert_eq!(out_off, 2 + 2 + 5);
        assert!(s3_unpack_data_tyb(&cmp).is_empty());
    }

    /// S3.227: default in-DLL `.text` opens with the 37B short-lea RAW (A1) floor.
    /// Host firmware has no lea prefix. Golden via_ir also keeps the same lea head.
    #[test]
    fn s3_in_dll_text_starts_with_short_lea_raw() {
        let lea = s3_short_lea_raw_bytes();
        assert_eq!(lea.len(), 37);
        let dll_text = emit_x86(&parse_tyb(&s3_table_prog_loop_tyb(&[0xFF])).unwrap()).text;
        assert!(
            dll_text.starts_with(&lea),
            "default empty-DATA in-DLL .text must open with short lea RAW"
        );
        let via_text =
            emit_x86(&parse_tyb(&s3_table_prog_loop_tyb_via_ir(&[0xFF])).unwrap()).text;
        assert!(via_text.starts_with(&lea), "via_ir golden also opens with lea");
        let host_text = emit_x86(&parse_tyb(&s3_host_prog_loop_tyb(&[0xFF])).unwrap()).text;
        assert!(
            !host_text.starts_with(&lea),
            "host firmware has no short lea RAW prefix"
        );
    }

    /// S3.232: short-lea RAW bytes are a STB-flat / poke payload (YOYO can materialize
    /// the constant); A1 still injects them into in-DLL `.text` (floor unchanged).
    #[test]
    fn s3_short_lea_raw_stb_and_poke_payload() {
        let lea = s3_short_lea_raw_bytes();
        assert_eq!(lea.len(), 37);
        let (tyb, off) = s3_yoyo_emit_flat_via_stb(&lea, 0x20);
        assert!(s3_unpack_data_tyb(&tyb).is_empty());
        assert_eq!(s3_sim_prog_loop(&tyb, 0, off, 0).unwrap(), lea);
        let distinct: std::collections::BTreeSet<u8> =
            lea.iter().copied().filter(|&b| b != 0).collect();
        let mut poked = Vec::new();
        s3_ir_seed_at(&mut poked, 0x80, &lea, 10);
        let tmp_sets = poked
            .iter()
            .filter(|r| {
                r.0 == 0x30 && u16::from_le_bytes([r.2[0], r.2[1]]) == S3_PACK_STB_TMP
            })
            .count();
        assert_eq!(tmp_sets, distinct.len());
        // Chunked A1 pack (table/dst/prog) still matches the OnceLock prologue.
        let mut a1 = Vec::new();
        s3_append_raw_a1(&mut a1, &S3_LEA_TABLE);
        s3_append_raw_a1(&mut a1, &S3_LEA_DST);
        s3_append_raw_a1(&mut a1, &S3_LEA_PROG);
        assert_eq!(a1.as_slice(), s3_in_dll_lea_prologue_recs());
    }

    /// S3.233: STB-flat / poke can materialize lea bytes, but only A1 injects them
    /// into `.text`. Shaped emit alone does not open `.text` with the lea prefix.
    #[test]
    fn s3_short_lea_text_only_via_a1_floor() {
        let lea = s3_short_lea_raw_bytes();
        let (flat_tyb, _) = s3_yoyo_emit_flat_via_stb(&lea, 0x20);
        let flat_text = emit_x86(&parse_tyb(&flat_tyb).unwrap()).text;
        assert!(
            !flat_text.starts_with(&lea),
            "STB-flat of lea payload must not become .text head"
        );
        let mut poke = Vec::new();
        s3_ir_seed_at(&mut poke, 0x80, &lea, 10);
        poke.push((0xFF, 0, [0; 6]));
        let poke_text = emit_x86(&parse_tyb(&s3_host_tyb_from_records(&poke)).unwrap()).text;
        assert!(
            !poke_text.starts_with(&lea),
            "poke-seed of lea payload must not become .text head"
        );
        // A1 path is the floor that does inject.
        let a1_tyb = tyb_from_records(s3_in_dll_lea_prologue_recs());
        let a1_text = emit_x86(&parse_tyb(&a1_tyb).unwrap()).text;
        assert!(a1_text.starts_with(&lea), "A1 prologue alone opens .text with lea");
        assert!(
            emit_x86(&parse_tyb(&s3_table_prog_loop_tyb(&[0xFF])).unwrap())
                .text
                .starts_with(&lea)
        );
    }

    /// S3.214: host prog loop is a thinner record stream than the golden in-DLL one.
    #[test]
    fn s3_host_prog_loop_thinner_than_in_dll_golden() {
        let host = s3_prog_loop_code_host(1, S3_HOST_NROWS);
        let dll = s3_prog_loop_code(1, 3);
        assert!(
            host.len() < dll.len(),
            "host {} records should be under golden dll {}",
            host.len(),
            dll.len()
        );
        let host_code = emit_x86(&parse_tyb(&s3_host_tyb_from_records(&host)).unwrap())
            .text
            .len();
        let dll_code = emit_x86(&parse_tyb(&s3_host_tyb_from_records(&dll)).unwrap())
            .text
            .len();
        assert!(
            host_code < dll_code,
            "host .text {host_code} should beat golden-style {dll_code}"
        );
        // Semantics: host RET firmware still emits C3 via sim.
        assert_eq!(
            s3_sim_prog_loop(
                &s3_host_prog_loop_tyb(&[0xFF]),
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
            )
            .unwrap(),
            vec![0xC3]
        );
    }

    /// S3.207/S3.221: host IR-writes table. Default in-DLL is empty-DATA; via_ir is golden-only.
    #[test]
    fn s3_host_firmware_data_is_via_ir_product() {
        let host = parse_tyb(&s3_host_prog_loop_tyb(&[0xFF])).unwrap();
        assert!(host.records.iter().all(|r| r.op != 0x10));
        assert!(host.records.iter().any(|r| r.op == 0x81));
        let dll = parse_tyb(&s3_table_prog_loop_tyb(&[0xFF])).unwrap();
        assert!(
            dll.records.iter().all(|r| r.op != 0x10),
            "S3.221: default in-DLL has no DATA"
        );
        assert!(dll.records.iter().any(|r| r.op == 0x81));
        assert!(dll.records.iter().any(|r| r.op == 0xA1), "in-DLL keeps lea A1");
        let via = parse_tyb(&s3_table_prog_loop_tyb_via_ir(&[0xFF])).unwrap();
        assert!(via.records.iter().any(|r| r.op == 0x10), "golden via_ir still DATA");
        compile_tyb_to_pe_mini(&s3_table_prog_loop_tyb(&[0xFF])).expect("in-DLL fits mini");
    }

    /// S3.207: host seed cursor starts at TABLE_OFF, not at 0 (leading pad skipped).
    #[test]
    fn s3_host_ir_seed_skips_leading_zero_pad() {
        let tyb = s3_host_prog_loop_tyb(&[0xFF]);
        let sets: Vec<_> = parse_tyb(&tyb)
            .unwrap()
            .records
            .into_iter()
            .filter(|r| r.op == 0x30)
            .collect();
        // S3.217 long-path: BASE first, one per live window; all STB oo imm8.
        assert!(!sets.is_empty());
        let slot0 = u16::from_le_bytes([sets[0].args_raw[0], sets[0].args_raw[1]]);
        let imm0 = u32::from_le_bytes([
            sets[0].args_raw[2],
            sets[0].args_raw[3],
            sets[0].args_raw[4],
            sets[0].args_raw[5],
        ]);
        assert_eq!(slot0, S3_PACK_STB_BASE);
        assert_eq!(imm0, S3_HOST_TABLE_OFF);
        let stbs: Vec<_> = parse_tyb(&tyb)
            .unwrap()
            .records
            .into_iter()
            .filter(|r| r.op == 0x81)
            .collect();
        assert!(!stbs.is_empty());
        assert!(stbs.iter().all(|r| {
            u16::from_le_bytes([r.args_raw[4], r.args_raw[5]]) < 128
        }));
        let bases: Vec<_> = sets
            .iter()
            .filter(|r| u16::from_le_bytes([r.args_raw[0], r.args_raw[1]]) == S3_PACK_STB_BASE)
            .collect();
        let payload = s3_fixed_payload(&S3_FIXED10_OPS, &[0xFF], S3_HOST_OUTBUF_OFF as usize);
        let start = S3_HOST_TABLE_OFF as usize;
        let end = S3_HOST_PROG_OFF as usize + 1 + 1;
        let slice = &payload[start..end];
        assert!(slice.len() > 127, "host seed must take long poke path");
        let live_wins = {
            let mut w = std::collections::BTreeSet::new();
            for (i, &b) in slice.iter().enumerate() {
                if b != 0 {
                    w.insert(i / 127);
                }
            }
            w.len()
        };
        assert_eq!(bases.len(), live_wins);
        assert!(live_wins >= 2);
    }



    /// S3.209–S3.213: poke seed fits mini alone; seed+prog exceeds 512 (lea ~38B).
    /// S3.213 floors: cannot imm8-ify ADD (fixed10 golden) or edit `s3_prog_loop_code`
    /// records (in-DLL golden). via_ir stays until those cliffs move.
    #[test]
    fn s3_ir_seed_poke_beats_run_fold_still_over_cap() {
        let payload = s3_fixed_payload(&S3_FIXED10_OPS[..3], &[0xFF], S3_OUTBUF_OFF as usize);
        let start = 0x80usize;
        let end = 0xBC + 1 + 1;
        let slice = &payload[start..end];
        let mut poked = Vec::new();
        s3_ir_seed_at(&mut poked, 0x80, slice, 10);
        let poke_code = emit_x86(&parse_tyb(&tyb_from_records(&poked)).unwrap())
            .text
            .len();
        let mut run_recs = Vec::new();
        // S3.279: run-fold compare also skips SET ZERO (image-zero floor).
        run_recs.push(s3_rec_set(S3_PACK_PROTO_DST, 0x80));
        s3_append_stb_runs(&mut run_recs, slice, 10);
        let run_code = emit_x86(&parse_tyb(&tyb_from_records(&run_recs)).unwrap())
            .text
            .len();
        assert!(
            poke_code < run_code,
            "poke-only {poke_code} should beat run-fold {run_code}"
        );
        assert!(
            poke_code <= S1_SLIM_MAX_CODE,
            "S3.210: short poke seed {poke_code} must fit mini cap {}",
            S1_SLIM_MAX_CODE
        );
        let mut seed_ret = poked.clone();
        seed_ret.push(s3_fixed_rec(0xFF));
        compile_tyb_to_pe_mini(&tyb_from_records(&seed_ret))
            .expect("poke seed + RET must fit mini");
        // S3.212: lea A1 .text ≈ raw lea bytes; prog loop is the heavy sibling.
        let lea_code = emit_x86(&parse_tyb(&tyb_from_records(
            &s3_in_dll_lea_prologue_recs().to_vec(),
        ))
        .unwrap())
        .text
        .len();
        let prog_code =
            emit_x86(&parse_tyb(&tyb_from_records(&s3_prog_loop_code(1, 3))).unwrap())
                .text
                .len();
        assert!(
            lea_code < 64,
            "short lea A1 .text {lea_code} should stay near raw 37B"
        );
        assert!(
            prog_code > lea_code * 4,
            "prog loop {prog_code} dwarfs lea {lea_code}"
        );
        let mut seed_prog = poked.clone();
        seed_prog.extend(s3_prog_loop_code(1, 3));
        let seed_prog_code =
            emit_x86(&parse_tyb(&tyb_from_records(&seed_prog)).unwrap())
                .text
                .len();
        // S3.219: raised cap fits seed+prog; lea was never the bulk (~38B).
        assert!(
            seed_prog_code <= S1_SLIM_MAX_CODE,
            "seed+prog {seed_prog_code} must fit raised cap"
        );
        assert!(
            prog_code > lea_code * 4,
            "prog still dwarfs lea after raise"
        );
        let mut full = poked.clone();
        full.extend_from_slice(s3_in_dll_lea_prologue_recs());
        full.extend(s3_prog_loop_code(1, 3));
        compile_tyb_to_pe_mini(&tyb_from_records(&full))
            .expect("S3.219: poke+lea+prog fits raised mini cap");
        let dll_code = emit_x86(&parse_tyb(&s3_table_prog_loop_tyb(&[0xFF])).unwrap())
            .text
            .len();
        assert!(dll_code <= S1_SLIM_MAX_CODE, "default empty-DATA in-DLL code {dll_code}");
        assert!(poked.iter().any(|r| {
            r.0 == 0x30 && u16::from_le_bytes([r.2[0], r.2[1]]) == S3_PACK_STB_BASE
        }));
        assert!(poked.iter().any(|r| {
            r.0 == 0x81
                && u16::from_le_bytes([r.2[2], r.2[3]]) == S3_PACK_STB_BASE
                && u16::from_le_bytes([r.2[4], r.2[5]]) < 128
        }));
        // S3.217 short path: one BASE; one TMP per distinct nonzero (global).
        assert_eq!(slice.len() <= 127, true);
        let base_sets = poked
            .iter()
            .filter(|r| {
                r.0 == 0x30 && u16::from_le_bytes([r.2[0], r.2[1]]) == S3_PACK_STB_BASE
            })
            .count();
        assert_eq!(base_sets, 1);
        let distinct: std::collections::BTreeSet<u8> =
            slice.iter().copied().filter(|&b| b != 0).collect();
        let tmp_sets = poked
            .iter()
            .filter(|r| {
                r.0 == 0x30 && u16::from_le_bytes([r.2[0], r.2[1]]) == S3_PACK_STB_TMP
            })
            .count();
        assert_eq!(tmp_sets, distinct.len());
    }

    /// S3.208/S3.219: slim DATA file offset tracks the code cap (now 1024 → @0xA00).
    #[test]
    fn s3_slim_code_cap_pins_data_payload_off() {
        assert_eq!(S1_SLIM_MAX_CODE, 1024);
        assert_eq!(S1_SLIM_DATA_PAYLOAD_OFF, 0xA00);
        // align(13+1024, 0x200) == 0x600; headers 0x400 → data at 0xA00.
        assert_eq!(0x400 + 0x600, S1_SLIM_DATA_PAYLOAD_OFF);
    }

    /// S3.218/S3.219: poke+lea+prog (~800B) compiles under the raised 1024 cap.
    /// Golden in-DLL firmware still embeds via_ir DATA (TYB bytes pinned).
    #[test]
    fn s3_cap_raise_1024_recipe_and_poke_lea_prog_need() {
        let payload = s3_fixed_payload(&S3_FIXED10_OPS[..3], &[0xFF], S3_OUTBUF_OFF as usize);
        let start = 0x80usize;
        let end = 0xBC + 1 + 1;
        let mut recs = Vec::new();
        s3_ir_seed_at(&mut recs, 0x80, &payload[start..end], 10);
        recs.extend_from_slice(s3_in_dll_lea_prologue_recs());
        recs.extend(s3_prog_loop_code(1, 3));
        let tyb = tyb_from_records(&recs);
        let need = emit_x86(&parse_tyb(&tyb).unwrap()).text.len();
        assert!(
            need <= S1_SLIM_MAX_CODE,
            "poke+lea+prog {need} must fit raised cap {}",
            S1_SLIM_MAX_CODE
        );
        compile_tyb_to_pe_mini(&tyb).expect("S3.219: poke+lea+prog mini PE");
        assert!(s3_unpack_data_tyb(&tyb).is_empty());
        assert!(
            parse_tyb(&s3_table_prog_loop_tyb_via_ir(&[0xFF]))
                .unwrap()
                .records
                .iter()
                .any(|r| r.op == 0x10),
            "golden via_ir helper keeps DATA"
        );
    }

    /// S3.207/S3.219: poke seed + lea + prog now fits the raised mini cap;
    /// golden via_ir path still compiles.
    #[test]
    fn s3_in_dll_ir_seed_exceeds_mini_code_cap() {
        let payload = s3_fixed_payload(&S3_FIXED10_OPS[..3], &[0xFF], S3_OUTBUF_OFF as usize);
        let start = 0x80usize;
        let end = 0xBC + 1 + 1;
        let mut seeded = Vec::new();
        s3_ir_seed_at(&mut seeded, 0x80, &payload[start..end], 10);
        seeded.extend_from_slice(s3_in_dll_lea_prologue_recs());
        seeded.extend(s3_prog_loop_code(1, 3));
        compile_tyb_to_pe_mini(&tyb_from_records(&seeded))
            .expect("S3.219: poke+lea+prog fits 1024 cap");
        compile_tyb_to_pe_mini(&s3_table_prog_loop_tyb_via_ir(&[0xFF]))
            .expect("via_ir golden path fits");
    }

    /// S3.137/S3.141: pack ↔ unpack is the YOYO-target algorithm (still implemented in Rust).
    #[test]
    fn s3_pack_data_roundtrip_and_host_payload() {
        assert_eq!(S3_PACK_CHUNK, 3);
        for n in 0usize..=64 {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 17 + 3) as u8).collect();
            let packed = s3_pack_data(&bytes);
            let expect_recs = if n == 0 { 0 } else { (n + S3_PACK_CHUNK - 1) / S3_PACK_CHUNK };
            assert_eq!(packed.len(), expect_recs, "chunk count len {n}");
            assert_eq!(s3_unpack_data(&packed), bytes, "len {n}");
            let tyb = s3_host_tyb_from_records(&packed);
            assert_eq!(s3_unpack_data_tyb(&tyb), bytes, "tyb len {n}");
        }
        let ops = &[0x30u8, 0xFF];
        let tyb = s3_host_prog_loop_tyb(ops);
        assert!(
            s3_unpack_data_tyb(&tyb).is_empty(),
            "S3.205: host firmware has no DATA seed"
        );
        let got = s3_sim_prog_loop(
            &tyb,
            S3_HOST_TABLE_OFF as u64,
            S3_HOST_OUTBUF_OFF,
            (S3_HOST_PROG_OFF + 1) as u64,
        )
        .unwrap();
        assert_eq!(got, emit_x86(&parse_tyb(&s3_host_tyb_from_records(&[s3_fixed_rec(0x30), s3_fixed_rec(0xFF)])).unwrap()).text);
    }

    /// S3.142: YOYO IR pack prototype (unrolled SET/ADD/MEMCPY) == `s3_pack_data` wire.
    /// No STB; no golden change. Builder still Rust — executed algorithm is YOYO ops.
    #[test]
    fn s3_yoyo_ir_pack_proto_parity() {
        assert!(s3_pack_data_wire(&[]).is_empty());
        for n in 1usize..=6 {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 17 + 3) as u8).collect();
            let expect = s3_pack_data_wire(&bytes);
            assert_eq!(expect.len(), ((n + S3_PACK_CHUNK - 1) / S3_PACK_CHUNK) * 8);
            let tyb = s3_yoyo_pack_proto_tyb(&bytes);
            assert!(
                s3_unpack_data_tyb(&tyb).is_empty(),
                "S3.198: proto pack has no DATA"
            );
            let prog = parse_tyb(&tyb).expect("pack proto tyb");
            assert!(
                prog.records.iter().any(|r| r.op == 0x84),
                "pack proto must use MEMCPY"
            );
            assert!(
                prog.records.iter().all(|r| !matches!(r.op, 0xA1 | 0xA0 | 0x12 | 0x13)),
                "pack proto stays host-style (no RAW/A1)"
            );
            let got = s3_sim_prog_loop(&tyb, 0, S3_PACK_PROTO_OUT, 0)
                .unwrap_or_else(|e| panic!("sim pack len {n}: {e:#x}"));
            assert_eq!(got, expect, "YOYO IR pack != s3_pack_data wire, len {n}");
        }
    }

    /// S3.143: looped YOYO IR pack (SUBV/CMP/JL) == `s3_pack_data` wire for lens 1..=64.
    #[test]
    fn s3_yoyo_ir_pack_loop_parity() {
        for n in 1usize..=64 {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 17 + 3) as u8).collect();
            let expect = s3_pack_data_wire(&bytes);
            let (tyb, out_off) = s3_yoyo_pack_loop_tyb(&bytes);
            let prog = parse_tyb(&tyb).expect("pack loop tyb");
            assert!(
                prog.records.iter().any(|r| r.op == 0x6A),
                "loop pack must use SUBV"
            );
            assert!(
                prog.records.iter().any(|r| r.op == 0x73),
                "loop pack must use JL"
            );
            let got = s3_sim_prog_loop(&tyb, 0, out_off, 0)
                .unwrap_or_else(|e| panic!("sim loop pack len {n}: {e:#x}"));
            assert_eq!(got, expect, "loop IR pack != s3_pack_data wire, len {n}");
        }
    }

    /// S3.193: distinct bytes do not fold. No shift op, so each stays one SET.
    #[test]
    fn s3_emit_unique_bytes_stay_one_set() {
        let bytes: Vec<u8> = (1..9).collect();
        let mut recs = Vec::new();
        s3_append_stb_runs(&mut recs, &bytes, 1);
        let sets = recs.iter().filter(|r| r.0 == 0x30).count();
        assert_eq!(sets, bytes.len(), "unique byte stays one SET");
        assert!(recs.iter().all(|r| r.0 != 0x61 && r.0 != 0x70));
    }

    /// S3.222: short seed + in-DLL empty-DATA share one global-distinct poke helper.
    /// BASE is caller-owned (SET absolute vs MOVRR from lea). Still one SET/distinct; no SHR.
    #[test]
    fn s3_shared_global_distinct_poke_floor() {
        let payload = s3_fixed_payload(&S3_FIXED10_OPS[..3], &[0x30, 0xFF], S3_OUTBUF_OFF as usize);
        let start = S3_TABLE_OFF as usize;
        let end = S3_PROG_OFF as usize + 1 + 2;
        let slice = &payload[start..end];
        assert!(slice.len() <= 127);
        let distinct: std::collections::BTreeSet<u8> =
            slice.iter().copied().filter(|&b| b != 0).collect();

        let mut via_set = Vec::new();
        s3_append_stb_zero_then_poke(&mut via_set, S3_TABLE_OFF, slice, 10);
        let mut via_shared = Vec::new();
        via_shared.push(s3_rec_set(S3_PACK_STB_BASE, S3_TABLE_OFF));
        s3_append_stb_poke_global_distinct(&mut via_shared, slice);
        assert_eq!(via_set, via_shared, "short zero_then_poke == SET BASE + shared poke");

        let mut poke_only = Vec::new();
        s3_append_stb_poke_global_distinct(&mut poke_only, slice);
        let tmp_sets = poke_only
            .iter()
            .filter(|r| {
                r.0 == 0x30 && u16::from_le_bytes([r.2[0], r.2[1]]) == S3_PACK_STB_TMP
            })
            .count();
        assert_eq!(tmp_sets, distinct.len(), "shared poke: one SET TMP per distinct");
        assert!(poke_only.iter().all(|r| r.0 != 0x61 && r.0 != 0x70));

        let dll = parse_tyb(&s3_table_prog_loop_tyb_ir_seed(&[0x30, 0xFF])).unwrap();
        let base_sets = dll
            .records
            .iter()
            .filter(|r| {
                r.op == 0x30
                    && u16::from_le_bytes([r.args_raw[0], r.args_raw[1]]) == S3_PACK_STB_BASE
            })
            .count();
        assert_eq!(base_sets, 0, "in-DLL uses MOVRR BASE←15, not SET BASE");
        assert!(dll.records.iter().any(|r| {
            r.op == 0x64
                && u16::from_le_bytes([r.args_raw[0], r.args_raw[1]]) == S3_PACK_STB_BASE
                && u16::from_le_bytes([r.args_raw[2], r.args_raw[3]]) == 15
        }));
        // Shared poke records appear contiguously after MOVRR BASE←15.
        let dll_tuples: Vec<_> = dll
            .records
            .iter()
            .map(|r| (r.op, r.argc, r.args_raw))
            .collect();
        let mov = dll_tuples
            .iter()
            .position(|r| {
                r.0 == 0x64
                    && u16::from_le_bytes([r.2[0], r.2[1]]) == S3_PACK_STB_BASE
                    && u16::from_le_bytes([r.2[2], r.2[3]]) == 15
            })
            .expect("MOVRR BASE←15");
        assert_eq!(
            &dll_tuples[mov + 1..mov + 1 + poke_only.len()],
            poke_only.as_slice(),
            "in-DLL seed poke is the shared helper stream"
        );
    }

    /// S3.174/S3.200: distinct bytes still need one SET each; repeated zeros fold.
    #[test]
    fn s3_emit_set_stb_expansion_floor() {
        let body = [(0xFFu8, 0u8, [0u8; 6])];
        let flat_len = HEADER_SIZE + RECORD_SIZE;
        let (fw, off) = s3_yoyo_emit_tyb_file_unrolled(&body);
        let ir = parse_tyb(&fw).unwrap();
        assert!(ir.records.iter().any(|r| r.op == 0x61), "zero runs use SUB");
        let sets = ir.records.iter().filter(|r| r.op == 0x30).count();
        assert!(sets < flat_len + 2, "S3.200: runs collapse SET-per-byte");
        assert_eq!(
            s3_sim_prog_loop(&fw, 0, off, 0).unwrap(),
            s3_host_tyb_from_records(&body)
        );
    }

    /// S3.177/S3.178: hybrid picks unrolled under cap, two-phase (no Rust flat) above.
    #[test]
    fn s3_emit_hybrid_picks_path_by_flat_size() {
        let small = [s3_fixed_rec(0xFF)];
        assert!(HEADER_SIZE + RECORD_SIZE <= S3_EMIT_UNROLL_MAX_FLAT);
        let (fw_s, _) = s3_yoyo_emit_tyb_file(&small);
        // S3.200: unrolled may JMP inside zero runs; two-phase alone uses MEMCPY.
        assert!(!parse_tyb(&fw_s).unwrap().records.iter().any(|r| r.op == 0x84));
        let mut big = vec![s3_fixed_rec(0x00); 16];
        big.push(s3_fixed_rec(0xFF));
        assert!(HEADER_SIZE + big.len() * RECORD_SIZE > S3_EMIT_UNROLL_MAX_FLAT);
        let (fw_b, off_b) = s3_yoyo_emit_tyb_file(&big);
        assert!(parse_tyb(&fw_b).unwrap().records.iter().any(|r| r.op == 0x84));
        assert!(s3_unpack_data_tyb(&fw_b).is_empty());
        assert_eq!(
            s3_sim_prog_loop(&fw_b, 0, off_b, 0).unwrap(),
            s3_host_tyb_from_records(&big)
        );
    }

    /// S3.181: large default is two-phase (zero DATA image), not Rust-staged.
    #[test]
    fn s3_emit_large_default_is_two_phase() {
        let mut big = vec![s3_fixed_rec(0x00); 16];
        big.push(s3_fixed_rec(0xFF));
        let (fw, off) = s3_yoyo_emit_tyb_file(&big);
        let (fw_2, _) = s3_yoyo_emit_tyb_file_two_phase(&big);
        assert_eq!(fw, fw_2);
        assert!(s3_unpack_data_tyb(&fw).is_empty());
        assert_eq!(
            s3_sim_prog_loop(&fw, 0, off, 0).unwrap(),
            s3_host_tyb_from_records(&big)
        );
    }

    /// S3.181: two-phase parity — IR builds staging; composed returns only outbuf copy.
    #[test]
    fn s3_emit_two_phase_ir_size_and_parity() {
        let mut big = vec![s3_fixed_rec(0x00); 20];
        big.push(s3_fixed_rec(0xFF));
        let expect = s3_host_tyb_from_records(&big);
        let (fw, out_off) = s3_yoyo_emit_tyb_file_two_phase(&big);
        let ir = parse_tyb(&fw).unwrap();
        assert!(ir.records.iter().any(|r| r.op == 0x84));
        assert!(ir.records.iter().any(|r| r.op == 0x61), "zero run uses SUB");
        let sets = ir.records.iter().filter(|r| r.op == 0x30).count();
        let flat_len = HEADER_SIZE + big.len() * RECORD_SIZE;
        assert!(
            sets * 4 < flat_len,
            "S3.183 runs collapse SET-per-byte: sets={sets} flat={flat_len}"
        );
        assert!(s3_unpack_data_tyb(&fw).is_empty(), "S3.184: no zero DATA pad");
        let got = s3_sim_prog_loop(&fw, 0, out_off, 0).expect("two-phase");
        assert_eq!(got, expect);
        compile_tyb_to_pe_mini(&got).expect("two-phase emitted TYB mini pe");
    }

    /// S3.202: comparison path has empty DATA; flat is IR-written then MEMCPY'd.
    #[test]
    fn s3_loop_staged_data_is_empty() {
        let body = [s3_fixed_rec(0xFF)];
        let (fw, off) = s3_yoyo_emit_tyb_file_loop_staged(&body);
        assert!(s3_unpack_data_tyb(&fw).is_empty());
        assert!(parse_tyb(&fw).unwrap().records.iter().any(|r| r.op == 0x84));
        assert_eq!(
            s3_sim_prog_loop(&fw, 0, off, 0).unwrap(),
            s3_host_tyb_from_records(&body)
        );
    }

    /// S3.175/S3.202: the MEMCPY copy is O(1); staging runs grow with the flat.
    #[test]
    fn s3_emit_loop_staged_ir_size_independent_of_flat() {
        let small = [s3_fixed_rec(0xFF)];
        let mut big = vec![s3_fixed_rec(0x00); 8];
        big.push(s3_fixed_rec(0xFF));
        let memcpy_n = |body: &[(u8, u8, [u8; 6])]| {
            let (fw, _) = s3_yoyo_emit_tyb_file_loop_staged(body);
            parse_tyb(&fw)
                .unwrap()
                .records
                .iter()
                .filter(|r| r.op == 0x84)
                .count()
        };
        assert_eq!(memcpy_n(&small), 1);
        assert_eq!(memcpy_n(&big), 1);
        let (fw_l, off_l) = s3_yoyo_emit_tyb_file_loop_staged(&big);
        assert!(s3_unpack_data_tyb(&fw_l).is_empty());
        assert_eq!(
            s3_sim_prog_loop(&fw_l, 0, off_l, 0).unwrap(),
            s3_host_tyb_from_records(&big)
        );
    }

    /// S3.166: YOYO IR STB-emits TYB record wire (== hand `s3_rec_wire`).
    #[test]
    fn s3_yoyo_emit_tyb_records_via_stb() {
        let ret = (0xFFu8, 0u8, [0u8; 6]);
        let set = s3_rec_set(0, 0xDEAD_BEEF);
        let expect: Vec<u8> = [s3_rec_wire(ret.0, ret.1, ret.2), s3_rec_wire(set.0, set.1, set.2)]
            .into_iter()
            .flatten()
            .collect();
        let (tyb, out_off) = s3_yoyo_emit_recs_tyb(&[ret, set]);
        assert!(s3_unpack_data_tyb(&tyb).is_empty());
        assert!(
            parse_tyb(&tyb).unwrap().records.iter().any(|r| r.op == 0x81),
            "emit path must use STB"
        );
        let got = s3_sim_prog_loop(&tyb, 0, out_off, 0).expect("emit recs");
        assert_eq!(got, expect);
        // Round-trip: wire → records → matches inputs.
        assert_eq!(s3_pack_wire_section_to_recs(&got), vec![ret, set]);
    }

    /// S3.167/S3.171/S3.172: YOYO STB-emits a full TYB file (== hand bytes).
    #[test]
    fn s3_yoyo_emit_full_tyb_file_via_stb() {
        let body = [
            s3_rec_set(0, 0xDEAD_BEEF),
            (0xFFu8, 0u8, [0u8; 6]),
        ];
        let expect = s3_host_tyb_from_records(&body);
        // S3.172 default: IR writes flat (unrolled SET+STB), no Rust preimage.
        let (fw, out_off) = s3_yoyo_emit_tyb_file(&body);
        assert!(parse_tyb(&fw).unwrap().records.iter().any(|r| r.op == 0x81));
        let got = s3_sim_prog_loop(&fw, 0, out_off, 0).expect("emit tyb file");
        assert_eq!(got, expect);
        let parsed = parse_tyb(&got).expect("emitted tyb parses");
        assert_eq!(parsed.rec_cnt, 2);
        assert_eq!(parsed.records[0].op, 0x30);
        assert_eq!(parsed.records[1].op, 0xFF);
        assert_eq!(
            emit_x86(&parsed).text,
            emit_x86(&parse_tyb(&expect).unwrap()).text
        );
        // S3.202: comparison path is IR-written flat + MEMCPY.
        let (fw_l, off_l) = s3_yoyo_emit_tyb_file_loop_staged(&body);
        assert!(parse_tyb(&fw_l).unwrap().records.iter().any(|r| r.op == 0x84));
        assert_eq!(s3_sim_prog_loop(&fw_l, 0, off_l, 0).unwrap(), expect);
    }

    /// S3.168: fixed10 cases via STB-emitted TYB == emit_x86 / yoyo table.
    #[test]
    fn s3_fixed10_via_stb_emitted_tyb_parity() {
        for (name, body) in s3_fixed10_named_cases() {
            let hand = s3_host_tyb_from_records(&body);
            let expect = emit_x86(&parse_tyb(&hand).unwrap()).text;
            let (fw, out_off) = s3_yoyo_emit_tyb_file(&body);
            let emitted = s3_sim_prog_loop(&fw, 0, out_off, 0)
                .unwrap_or_else(|e| panic!("{name}: emit tyb {e:#x}"));
            assert_eq!(emitted, hand, "{name}: STB-emitted TYB bytes");
            let via = emit_x86(&parse_tyb(&emitted).unwrap()).text;
            assert_eq!(via, expect, "{name}: emit_x86 via emitted TYB");
            let ops: Vec<u8> = body.iter().map(|(op, _, _)| *op).collect();
            if let Some(yoyo) = try_emit_text_via_yoyo_table(&ops) {
                assert_eq!(yoyo, expect, "{name}: yoyo table still matches");
            }
        }
    }

    /// S3.169: STB-emitted fixed10 TYB files compile through mini PE.
    #[test]
    fn s3_stb_emitted_tyb_compiles_mini_pe() {
        for (name, body) in s3_fixed10_named_cases() {
            let (fw, out_off) = s3_yoyo_emit_tyb_file(&body);
            let emitted = s3_sim_prog_loop(&fw, 0, out_off, 0)
                .unwrap_or_else(|e| panic!("{name}: emit {e:#x}"));
            compile_tyb_to_pe_mini(&emitted).unwrap_or_else(|e| panic!("{name}: mini pe {e}"));
        }
    }

    /// S3.159: STB copy runs on slim PE (Windows); outbuf gets source bytes.
    #[test]
    #[cfg(windows)]
    fn s3_stb_copy_runs_on_pe() {
        let mut data = vec![0u8; S3_OUTBUF_OFF as usize + 4];
        data[0] = 0xDE;
        data[1] = 0xAD;
        data[2] = 0xBE;
        let mut recs = s3_pack_data_bytewise(&data);
        // slot 15 = .data base; slot 9 = outbuf (same lea seeds as compose).
        recs.extend_from_slice(s3_lea_ptr_a1_recs());
        recs.extend_from_slice(s3_lea_dst_a1_recs());
        const BASE: u16 = 15;
        const DST: u16 = 9;
        const TMP: u16 = 11;
        for k in 0..3u16 {
            recs.push(s3_rec_ldb(TMP, BASE, k));
            recs.push(s3_rec_stb(TMP, DST, 0));
            recs.push(s3_rec_add(DST, 1));
        }
        recs.push((0xFF, 0, [0; 6]));
        let tyb = tyb_from_records(&recs);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("stb mini pe");
        let got = s3_run_slim_pe_outbuf(&pe.bytes, 3);
        assert_eq!(got, vec![0xDE, 0xAD, 0xBE]);
    }

    /// S3.157: looped STB pack IR == `s3_pack_data` wire for lens 1..=64.
    #[test]
    fn s3_yoyo_ir_pack_loop_stb_parity() {
        for n in 1usize..=64 {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 17 + 3) as u8).collect();
            let expect = s3_pack_data_wire(&bytes);
            let (tyb, out_off) = s3_yoyo_pack_loop_stb_tyb(&bytes);
            assert!(
                parse_tyb(&tyb).unwrap().records.iter().any(|r| r.op == 0x81),
                "loop STB pack must use STB"
            );
            let got = s3_sim_prog_loop(&tyb, 0, out_off, 0)
                .unwrap_or_else(|e| panic!("loop stb pack len {n}: {e:#x}"));
            assert_eq!(got, expect, "loop STB IR pack != wire, len {n}");
        }
    }

    /// S3.156: emit_stb produces load-val / load-base / mov [rax], cl.
    #[test]
    fn s3_emit_stb_bytes() {
        let prog = parse_tyb(&s3_host_tyb_from_records(&[
            s3_rec_stb(1, 2, 0),
            (0xFF, 0, [0; 6]),
        ]))
        .unwrap();
        let text = emit_x86(&prog).text;
        // Ends with mov [rax], cl (88 08) then ret C3 — not a lone NOP.
        assert!(text.windows(2).any(|w| w == [0x88, 0x08]), "missing mov [rax],cl");
        assert_eq!(*text.last().unwrap(), 0xC3);
        assert!(!text.iter().all(|&b| b == 0x90 || b == 0xC3));
    }

    /// S3.155: unrolled STB pack IR == `s3_pack_data` wire (lens 1..=6).
    #[test]
    fn s3_yoyo_ir_pack_stb_parity() {
        for n in 1usize..=6 {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 17 + 3) as u8).collect();
            let expect = s3_pack_data_wire(&bytes);
            let tyb = s3_yoyo_pack_stb_tyb(&bytes);
            assert!(
                s3_unpack_data_tyb(&tyb).is_empty(),
                "S3.198: unrolled STB pack has no DATA"
            );
            assert!(
                parse_tyb(&tyb).unwrap().records.iter().any(|r| r.op == 0x81),
                "STB pack must use op 0x81"
            );
            let got = s3_sim_prog_loop(&tyb, 0, S3_PACK_PROTO_OUT, 0)
                .unwrap_or_else(|e| panic!("stb pack len {n}: {e:#x}"));
            assert_eq!(got, expect, "STB IR pack != wire, len {n}");
        }
    }

    /// S3.154: sim STB stores bytes; LDB→STB copy matches MEMCPY of same span.
    #[test]
    fn s3_sim_stb_ldb_roundtrip() {
        const TMP: u16 = 11;
        const ZERO: u16 = 13;
        let mut data = vec![0u8; 0x40];
        data[0] = 0x10;
        data[1] = 1;
        data[2] = 2;
        data[0x20] = 0xAA;
        data[0x21] = 0xBB;
        data[0x22] = 0xCC;
        let mut recs = s3_pack_data_bytewise(&data);
        recs.push(s3_rec_set(ZERO, 0));
        recs.push(s3_rec_set(S3_PACK_PROTO_DST, 0x30));
        recs.push(s3_rec_set(S3_PACK_PROTO_ONE, 1));
        for k in 0..3u32 {
            recs.push(s3_rec_ldb(TMP, ZERO, (0x20 + k) as u16));
            recs.push(s3_rec_stb(TMP, S3_PACK_PROTO_DST, 0));
            recs.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
        }
        recs.push((0xFF, 0, [0; 6]));
        let tyb = s3_host_tyb_from_records(&recs);
        let got = s3_sim_prog_loop(&tyb, 0, 0x30, 0).expect("stb copy");
        assert_eq!(got, vec![0xAA, 0xBB, 0xCC]);
        assert!(
            parse_tyb(&tyb).unwrap().records.iter().any(|r| r.op == 0x81),
            "must use STB"
        );
    }

    /// S3.153/S3.203: production pack/emit TYBs carry no bytewise DATA seed.
    #[test]
    fn s3_pack_bootstrap_scratch_is_bytewise() {
        let bytes = [0xAAu8, 0xBB, 0xCC];
        for (name, tyb) in [
            ("loop_stb", s3_yoyo_pack_loop_stb_tyb(&bytes).0),
            ("loop_memcpy", s3_yoyo_pack_loop_tyb(&bytes).0),
            ("proto", s3_yoyo_pack_proto_tyb(&bytes)),
            ("stb", s3_yoyo_pack_stb_tyb(&bytes)),
            ("emit", s3_yoyo_emit_tyb_file(&[s3_fixed_rec(0xFF)]).0),
        ] {
            let data: Vec<_> = parse_tyb(&tyb)
                .unwrap()
                .records
                .into_iter()
                .filter(|r| matches!(r.op, 0x10 | 0x12 | 0x13))
                .collect();
            assert!(data.is_empty(), "{name}: S3.203 production path has no DATA seed");
        }
        let via = s3_pack_data_via_ir(&bytes);
        assert!(via.iter().any(|(_, argc, _)| *argc == 3));
        assert_eq!(via, s3_pack_data(&bytes));
        // Helper still exists for a couple of unit tests that seed DATA by hand.
        assert_eq!(s3_pack_data_bytewise(&[1, 2]).len(), 2);
    }

    /// S3.150/S3.164: RAW A1 short-lea packs; in-DLL prologue OnceLock-stable.
    #[test]
    fn s3_append_raw_a1_matches_host_pack() {
        let mut a = Vec::new();
        s3_append_raw_a1(&mut a, S3_LEA_PTR);
        assert_eq!(a, s3_host_pack_bytes(0xA1, S3_LEA_PTR));
        assert_eq!(S3_LEA_PTR.len(), 7);
        let dll = parse_tyb(&s3_table_prog_loop_tyb(&[0xFF])).unwrap();
        let a1_bytes: usize = dll
            .records
            .iter()
            .filter(|r| r.op == 0xA1)
            .map(|r| r.argc as usize)
            .sum();
        assert_eq!(
            a1_bytes,
            S3_LEA_TABLE.len() + S3_LEA_DST.len() + S3_LEA_PROG.len(),
            "in-DLL A1 payload bytes == three lea seeds"
        );
        assert!(std::ptr::eq(
            s3_in_dll_lea_prologue_recs(),
            s3_in_dll_lea_prologue_recs()
        ));
        let mut expect = Vec::new();
        s3_append_raw_a1(&mut expect, &S3_LEA_TABLE);
        s3_append_raw_a1(&mut expect, &S3_LEA_DST);
        s3_append_raw_a1(&mut expect, &S3_LEA_PROG);
        assert_eq!(s3_in_dll_lea_prologue_recs(), expect.as_slice());
        assert!(std::ptr::eq(s3_lea_ptr_a1_recs(), s3_lea_ptr_a1_recs()));
        assert!(std::ptr::eq(s3_lea_dst_a1_recs(), s3_lea_dst_a1_recs()));
        assert_eq!(s3_lea_ptr_a1_recs(), s3_host_pack_bytes(0xA1, S3_LEA_PTR).as_slice());
        assert_eq!(s3_lea_dst_a1_recs(), s3_host_pack_bytes(0xA1, &S3_LEA_DST).as_slice());
    }

    /// S3.149/S3.151: packer scratch DATA length == out_off; prefix OnceLock stable.
    #[test]
    fn s3_pack_loop_scratch_excludes_outbuf() {
        let bytes: Vec<u8> = (0..64).map(|i| i as u8).collect();
        let (tyb, out_off) = s3_yoyo_pack_loop_tyb(&bytes);
        assert_eq!(out_off, 64 + 2 + 5);
        assert!(
            s3_unpack_data_tyb(&tyb).is_empty(),
            "S3.197: MEMCPY loop DATA is empty"
        );
    }

    /// S3.188: a short last chunk still reads CUR+1 and CUR+2. Those two zeros stay.
    #[test]
    fn s3_pack_scratch_keeps_overread_pad() {
        let payload = [0x11u8];
        let (tyb, out_off) = s3_yoyo_pack_loop_stb_tyb(&payload);
        let scratch = s3_unpack_data_tyb(&tyb);
        assert_eq!(out_off, payload.len() as u32 + 2);
        assert!(scratch.is_empty(), "S3.196: via_ir pack has no DATA records");
        assert_eq!(s3_pack_data_via_ir(&payload), s3_pack_data(&payload));
    }

    /// S3.148/S3.205: `s3_pack_data` stays reference-only; host/in-DLL no longer embed it.
    #[test]
    fn s3_pack_data_reference_only_after_firmware_fold() {
        assert_eq!(
            s3_pack_data(&[0x10, 0x20, 0x30, 0x40]),
            s3_host_pack_bytes(0x10, &[0x10, 0x20, 0x30, 0x40])
        );
        assert!(s3_unpack_data_tyb(&s3_host_prog_loop_tyb(&[0xFF])).is_empty());
        let dll_payload =
            s3_fixed_payload(&S3_FIXED10_OPS[..3], &[0xFF], S3_OUTBUF_OFF as usize);
        assert!(s3_unpack_data_tyb(&s3_table_prog_loop_tyb(&[0xFF])).is_empty());
        let dll = parse_tyb(&s3_table_prog_loop_tyb_via_ir(&[0xFF])).unwrap();
        let dll_data: Vec<_> = dll
            .records
            .iter()
            .filter(|r| r.op == 0x10)
            .map(|r| (r.op, r.argc, r.args_raw))
            .collect();
        assert_eq!(dll_data, s3_pack_data_via_ir(&dll_payload));
        assert_eq!(
            s3_pack_data_via_ir(&[1, 2, 3, 4]),
            s3_pack_data(&[1, 2, 3, 4])
        );
    }

    /// S3.145: dynamic out_off lets via_ir pack host-sized payloads (0x160).
    #[test]
    fn s3_pack_via_ir_host_sized_payload() {
        let n = S3_HOST_OUTBUF_OFF as usize;
        assert_eq!(s3_pack_proto_out_off(n), S3_PACK_PROTO_PAY + n as u32 + 2);
        let bytes: Vec<u8> = (0..n).map(|i| (i * 13 + 11) as u8).collect();
        let via = s3_pack_data_via_ir(&bytes);
        assert_eq!(via, s3_pack_data(&bytes), "host-sized via_ir != s3_pack_data");
        assert_eq!(s3_unpack_data(&via), bytes);
        // Also a mid size between 64 and host.
        let mid: Vec<u8> = (0..200).map(|i| i as u8).collect();
        assert_eq!(s3_pack_data_via_ir(&mid), s3_pack_data(&mid));
    }

    /// S3.144/S3.161: IR wire→records equals `s3_pack_data`; via_ir firmware uses STB.
    #[test]
    fn s3_pack_data_via_ir_records_parity() {
        assert!(s3_pack_data_via_ir(&[]).is_empty());
        let (probe, _) = s3_yoyo_pack_loop_stb_tyb(&[0x11, 0x22, 0x33]);
        assert!(
            parse_tyb(&probe).unwrap().records.iter().any(|r| r.op == 0x81),
            "S3.161: via_ir path must be STB loop"
        );
        for n in 1usize..=64 {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 19 + 7) as u8).collect();
            assert_eq!(
                s3_pack_data_via_ir(&bytes),
                s3_pack_data(&bytes),
                "via_ir records != s3_pack_data, len {n}"
            );
            assert_eq!(
                s3_unpack_data(&s3_pack_data_via_ir(&bytes)),
                bytes,
                "via_ir unpack, len {n}"
            );
        }
    }

    /// S3.138: cached bodies stay equal to a fresh emit_x86 bake.
    #[test]
    fn s3_fixed_bodies_match_emit_x86_bake() {
        for op in S3_FIXED10_OPS {
            assert_eq!(
                s3_fixed_row(op),
                s3_compute_fixed_body(op),
                "cached body {op:#x}"
            );
        }
        // Second call hits OnceLock; still matches.
        assert_eq!(s3_fixed_row(0x30), s3_host_set_body());
        assert_eq!(s3_fixed_row(0xFF), s3_host_ret_body());
        assert_eq!(s3_fixed_row(0x00), s3_host_nop_body());
    }

    /// S3.139/S3.205: host firmware is IR-only (table seed + loop). No DATA / A1.
    #[test]
    fn s3_host_firmware_last_rust_mile_opcodes() {
        let tyb = s3_host_prog_loop_tyb(&[0x62, 0xFF]);
        let prog = parse_tyb(&tyb).unwrap();
        let mut saw_stb = false;
        let mut saw_ir = false;
        for rec in &prog.records {
            match rec.op {
                0x10 | 0xA1 | 0xA0 | 0x12 | 0x13 => {
                    panic!("host firmware must not carry {:#x}", rec.op)
                }
                0x81 => {
                    saw_stb = true;
                    saw_ir = true;
                }
                0x30 | 0x40 | 0x61 | 0x62 | 0x64 | 0x65 | 0x68 | 0x70 | 0x71 | 0x73 | 0x74
                | 0x80 | 0x84 | 0xFF => saw_ir = true,
                other => panic!("unexpected host firmware op {other:#x}"),
            }
        }
        assert!(saw_stb && saw_ir);
        assert!(
            try_emit_text_via_yoyo_table(&[0x62, 0xFF]).is_some(),
            "host path still YOYO-composes"
        );
    }

    /// S3.140: every RET-terminated sequence of length ≤3 over fixed10 — yoyo == emit_x86.
    #[test]
    fn s3_fixed10_yoyo_exhaust_ret_terminated() {
        let check = |ops: &[u8]| {
            let recs: Vec<_> = ops.iter().map(|o| s3_fixed_rec(*o)).collect();
            let tyb = s3_host_tyb_from_records(&recs);
            let expect = emit_x86(&parse_tyb(&tyb).unwrap()).text;
            let via = try_emit_text_via_yoyo_table(ops)
                .unwrap_or_else(|| panic!("yoyo table missing for {ops:?}"));
            assert_eq!(via, expect, "ops={ops:?}");
        };
        check(&[0xFF]);
        let mut n = 1usize;
        for a in S3_FIXED10_OPS {
            check(&[a, 0xFF]);
            n += 1;
            for b in S3_FIXED10_OPS {
                check(&[a, b, 0xFF]);
                n += 1;
            }
        }
        assert_eq!(n, 1 + 10 + 100, "RET-terminated exhaust count");
    }

    /// S3.110/S3.111: `S3-subset-fixed10` one-shot gate.
    /// All 10 ops: YOYO table compose == `emit_x86`, and mini PE is the YOYO path
    /// (no `pack_meta_emit` driver). Firmware Rust pack remains — not CLOSED.
    #[test]
    fn s3_subset_fixed10_meta_emit_parity() {
        let cases = s3_fixed10_named_cases();
        assert_eq!(cases.len(), 10, "fixed10 contract");
        let fw = parse_tyb(&s3_host_prog_loop_tyb(&[0xFF])).unwrap();
        assert!(
            fw.records.iter().all(|r| r.op != 0xA1),
            "S3.112 host firmware has no RAW prologue"
        );
        // S3.132: in-DLL still needs short-form RAW lea for PE (host sim seeds offsets).
        let dll = parse_tyb(&s3_table_prog_loop_tyb(&[0xFF])).unwrap();
        assert!(
            dll.records.iter().any(|r| r.op == 0xA1),
            "in-DLL prog-loop must keep RAW lea for PE"
        );
        for (name, semantic) in cases {
            let tyb = tyb_from_records(&semantic);
            let prog = parse_tyb(&tyb).unwrap();
            let ops = fixed_template_ops(&prog).unwrap_or_else(|| panic!("{name} outside subset"));
            let expect = emit_x86(&prog).text;
            let via = try_emit_text_via_yoyo_table(&ops).unwrap_or_else(|| panic!("{name} yoyo table"));
            assert_eq!(via, expect, "{name} yoyo_table == emit_x86");
            let pe = compile_tyb_to_pe_mini(&tyb).expect("mini");
            let pe_y = try_compile_tyb_via_yoyo_table(&tyb).expect("yoyo pe");
            assert_eq!(pe.bytes, pe_y.bytes, "{name} mini took YOYO path");
            assert_eq!(
                slim_pe_user_text(&pe.bytes, expect.len()).unwrap(),
                expect.as_slice(),
                "{name} PE .text"
            );
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

    /// S3.134: table scan → OUT = first template byte (no memcpy). `nrows` is 2 for scan.
    fn append_scan_byte_out(
        recs: &mut Vec<(u8, u8, [u8; 6])>,
        needle: u8,
        nrows: u32,
        loop_id: u32,
        found_id: u32,
    ) {
        recs.push(s3_rec_set(S3_SCAN_NEEDLE, needle as u32));
        recs.push(s3_rec_set(S3_SCAN_I, 0));
        recs.push(s3_rec_set(S3_SCAN_NROWS, nrows));
        recs.push(s3_rec_label(loop_id));
        recs.push(s3_rec_ldb(S3_SCAN_OP, S3_SCAN_PTR, 0));
        recs.push(s3_rec_cmp(S3_SCAN_OP, S3_SCAN_NEEDLE));
        recs.push(s3_rec_je(found_id));
        recs.push(s3_rec_add(S3_SCAN_PTR, 20));
        recs.push(s3_rec_add(S3_SCAN_I, 1));
        recs.push(s3_rec_cmp(S3_SCAN_I, S3_SCAN_NROWS));
        recs.push(s3_rec_jl(loop_id));
        recs.push(s3_rec_set(S3_SCAN_OUT, 0xFE));
        recs.push(s3_fixed_rec(0xFF));
        recs.push(s3_rec_label(found_id));
        recs.push(s3_rec_ldb(S3_SCAN_OUT, S3_SCAN_PTR, 4));
        recs.push(s3_fixed_rec(0xFF));
    }

    /// TYB: 2-row DATA table (RET, NOP) + scan loop → state[OUT]=first template byte.
    fn s3_table_scan_tyb(needle: u8) -> Vec<u8> {
        let table = s3_fixed_table(&[0xFF, 0x00]);
        // S3.147: DATA via_ir (Rust pack remains packer-scratch only).
        let mut recs = s3_pack_data_via_ir(&table);
        // S3.131/S3.165: short `lea rax,[r15]` seed from OnceLock (not disp32).
        recs.extend_from_slice(s3_lea_ptr_a1_recs());
        append_scan_byte_out(&mut recs, needle, 2, 1, 2);
        tyb_from_records(&recs)
    }

    /// Host sim: DATA buffer + state[0..16]; ptr slots hold **offsets** into DATA.
    /// RAW_BYTES/A1 ignored (PE lea); sim seeds ptr=0.
    fn s3_sim_table_scan(tyb: &[u8]) -> u64 {
        // No memcpy: shared sim returns the OUT slot as Err.
        match s3_sim_prog_loop(tyb, 0, 0, 0) {
            Err(code) => code as u64,
            Ok(_) => panic!("scan firmware must not memcpy"),
        }
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
        let ret_b = emit_x86(&parse_tyb(&tyb_from_records(&[s3_fixed_rec(0xFF)])).unwrap()).text;
        assert_eq!(ret_b, vec![0xC3]);
        let nop_b = emit_x86(
            &parse_tyb(&tyb_from_records(&[s3_fixed_rec(0x00), s3_fixed_rec(0xFF)])).unwrap(),
        )
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

    /// S3.17/S3.131: 3-row table + shared short lea + `append_scan_memcpy` + RET.
    fn s3_table_compose_tyb(needle: u8) -> Vec<u8> {
        let table = s3_fixed_table(&[0xFF, 0x00, 0x30]);
        let mut recs = s3_pack_data_via_ir(&table);
        recs.extend_from_slice(s3_lea_ptr_a1_recs());
        recs.extend_from_slice(s3_lea_dst_a1_recs());
        append_scan_memcpy(&mut recs, needle, 1, 2, false);
        recs.push(s3_fixed_rec(0xFF));
        tyb_from_records(&recs)
    }

    /// S3.118: compose uses the shared prog-loop sim (table at 0, outbuf 0xC8).
    fn s3_sim_table_compose(tyb: &[u8]) -> Result<Vec<u8>, u8> {
        s3_sim_prog_loop(tyb, 0, S3_OUTBUF_OFF, 0)
    }


    #[test]
    fn s3_yoyo_table_compose_sim_multibyte() {
        let ret = s3_sim_table_compose(&s3_table_compose_tyb(0xFF)).expect("ret");
        assert_eq!(ret, vec![0xC3]);
        let nop = s3_sim_table_compose(&s3_table_compose_tyb(0x00)).expect("nop");
        assert_eq!(nop, vec![0x90]);
        let set = s3_sim_table_compose(&s3_table_compose_tyb(0x30)).expect("set");
        assert_eq!(set.as_slice(), s3_host_set_body().as_slice());
        // full SET+RET ≡ emit_x86
        let mut composed = set;
        composed.push(0xC3);
        let expect = emit_x86(
            &parse_tyb(&tyb_from_records(&[s3_fixed_rec(0x30), s3_fixed_rec(0xFF)])).unwrap(),
        )
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
        assert_eq!(got.as_slice(), s3_host_set_body().as_slice());
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

        // ASCII path under temp_dir (not cwd; cwd is process-global under parallel tests).
        let dir = std::env::temp_dir().join(format!(
            "ys100l-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
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

        // ASCII path under temp_dir (not cwd; cwd is process-global under parallel tests).
        let dir = std::env::temp_dir().join(format!(
            "ys101w-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
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

    /// S3.102: slim LOAD→WRITE roundtrip — read src, write dst, bytes match.
    /// StubPlatform stays stub; this is the TYB slim real-I/O lane only.
    #[test]
    #[cfg(windows)]
    fn s3_slim_pe_manual_map_load_write_roundtrip_e2e() {
        use crate::pe_manual_map::{manual_map_pe_dll_executable, parse_pe64_headers};
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

        // ASCII path under temp_dir (not cwd; cwd is process-global under parallel tests).
        let dir = std::env::temp_dir().join(format!(
            "ys102rw-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).expect("mkdir");
        let src = dir.join("src.bin");
        let dst = dir.join("dst.bin");
        let payload = [0xABu8, 0xCD, 0xEF, 0x01, 0x23, 0x45, 0x67, 0x89];
        std::fs::write(&src, &payload).expect("write src");
        let _ = std::fs::remove_file(&dst);

        let src_str = src.to_str().expect("src ascii").to_string();
        let dst_str = dst.to_str().expect("dst ascii").to_string();
        assert!(src_str.is_ascii() && dst_str.is_ascii());
        let mut src_bytes = src_str.into_bytes();
        src_bytes.push(0);
        let mut dst_bytes = dst_str.into_bytes();
        dst_bytes.push(0);

        // .data layout: src path @0x100, dst path @0x180; LOAD→slot1; WRITE slot1→dst sz=slot2
        const SRC_OFF: u32 = 0x100;
        const DST_OFF: u32 = 0x180;
        // After LOAD, slot1=buf. Size for WRITE: we store payload.len() in slot2 via host before call,
        // and also after LOAD the buffer holds exact payload (ReadFile may read more into the chunk —
        // we only WriteFile `payload.len()` bytes from slot2).
        let prog = TybProgram {
            rec_cnt: 3,
            records: vec![
                TybRecord {
                    op: 0x50,
                    argc: 2,
                    args_raw: [
                        0x01,
                        0x00,
                        SRC_OFF as u8,
                        (SRC_OFF >> 8) as u8,
                        0,
                        0,
                    ],
                },
                TybRecord {
                    op: 0x51,
                    argc: 3,
                    args_raw: [
                        0x01,
                        0x00,
                        DST_OFF as u8,
                        (DST_OFF >> 8) as u8,
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
        cg.data.resize(SRC_OFF as usize, 0);
        cg.data.extend_from_slice(&src_bytes);
        assert!(cg.data.len() <= DST_OFF as usize, "src path overlaps dst");
        cg.data.resize(DST_OFF as usize, 0);
        cg.data.extend_from_slice(&dst_bytes);
        assert!(cg.data.len() < S1_SLIM_IAT_BASE as usize);
        let pe = wrap_pe_slim(&cg).expect("slim RW PE");

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
            // slot2 = write size (LOAD does not set it)
            std::ptr::write_unaligned(data_base.add(16) as *mut u64, payload.len() as u64);
            let entry = (exe.base as usize + headers.entry_rva as usize) as *const ();
            let f: extern "C" fn() = std::mem::transmute(entry);
            f();
            // WRITE zeros slot1 → LOAD's VirtualAlloc buffer leaks once (64KiB); process exit reclaims.
        }
        let got = std::fs::read(&dst).expect("read dst");
        assert_eq!(got.as_slice(), &payload, "roundtrip mismatch");
        let _ = std::fs::remove_dir_all(&dir);
    }

    // ============= S3.18: full .text compose (multi-op SET+RET) =============

    const S3_LEA_PTR: &[u8] = &[0x49, 0x8D, 0x07, 0x49, 0x89, 0x47, 0x78]; // lea rax,[r15]; mov [r15+0x78],rax
    /// S3.127: `lea rax,[r15+disp32]; mov [r15+slot*8], rax`.
    const fn s3_lea_store(disp: u32, slot: u8) -> [u8; 11] {
        let d = disp.to_le_bytes();
        [
            0x49,
            0x8D,
            0x87,
            d[0],
            d[1],
            d[2],
            d[3],
            0x49,
            0x89,
            0x47,
            slot.wrapping_mul(8),
        ]
    }
    /// S3.127: one lea, then two slot stores (table base + scan ptr).
    const fn s3_lea_two(disp: u32, a: u8, b: u8) -> [u8; 15] {
        let d = disp.to_le_bytes();
        [
            0x49,
            0x8D,
            0x87,
            d[0],
            d[1],
            d[2],
            d[3],
            0x49,
            0x89,
            0x47,
            a.wrapping_mul(8),
            0x49,
            0x89,
            0x47,
            b.wrapping_mul(8),
        ]
    }
    const S3_LEA_TABLE: [u8; 15] = s3_lea_two(0x80, 3, 15);
    const S3_LEA_DST: [u8; 11] = s3_lea_store(0xC8, 9);
    /// prog cursor = `S3_PROG_OFF + 1` = 0xBD. Checked where the firmware is built.
    const S3_LEA_PROG: [u8; 11] = s3_lea_store(0xBD, 2);

    /// S3.232/S3.234: single source for in-DLL short-lea RAW (table‖dst‖prog = 37B).
    fn s3_short_lea_raw_bytes() -> Vec<u8> {
        let mut lea = Vec::with_capacity(37);
        lea.extend_from_slice(&S3_LEA_TABLE);
        lea.extend_from_slice(&S3_LEA_DST);
        lea.extend_from_slice(&S3_LEA_PROG);
        lea
    }

    /// S3.164/S3.234: OnceLock of in-DLL three-lea A1 prologue from the shared 37B.
    /// (S3.265: one-stream A1 pack changes TYB framing — golden stays three chunks.)
    fn s3_in_dll_lea_prologue_recs() -> &'static [(u8, u8, [u8; 6])] {
        use std::sync::OnceLock;
        static PROLOGUE: OnceLock<Vec<(u8, u8, [u8; 6])>> = OnceLock::new();
        PROLOGUE
            .get_or_init(|| {
                let lea = s3_short_lea_raw_bytes();
                assert_eq!(lea.len(), 37);
                let mut r = Vec::new();
                s3_append_raw_a1(&mut r, &lea[..15]); // TABLE
                s3_append_raw_a1(&mut r, &lea[15..26]); // DST
                s3_append_raw_a1(&mut r, &lea[26..37]); // PROG
                r
            })
            .as_slice()
    }

    /// S3.165: cached A1 pack of short `lea rax,[r15]` seed.
    fn s3_lea_ptr_a1_recs() -> &'static [(u8, u8, [u8; 6])] {
        use std::sync::OnceLock;
        static R: OnceLock<Vec<(u8, u8, [u8; 6])>> = OnceLock::new();
        R.get_or_init(|| {
            let mut r = Vec::new();
            s3_append_raw_a1(&mut r, S3_LEA_PTR);
            r
        })
        .as_slice()
    }

    /// S3.165: cached A1 pack of outbuf `lea` seed.
    fn s3_lea_dst_a1_recs() -> &'static [(u8, u8, [u8; 6])] {
        use std::sync::OnceLock;
        static R: OnceLock<Vec<(u8, u8, [u8; 6])>> = OnceLock::new();
        R.get_or_init(|| {
            let mut r = Vec::new();
            s3_append_raw_a1(&mut r, &S3_LEA_DST);
            r
        })
        .as_slice()
    }

    /// Append one table-scan + MEMCPY_DATA; optionally ADDV dst+=len afterwards.
    fn append_scan_memcpy(
        recs: &mut Vec<(u8, u8, [u8; 6])>,
        needle: u8,
        loop_id: u32,
        found_id: u32,
        advance_dst: bool,
    ) {
        recs.push(s3_rec_set(S3_SCAN_NEEDLE, needle as u32));
        recs.push(s3_rec_set(S3_SCAN_I, 0));
        recs.push(s3_rec_set(S3_SCAN_NROWS, 3));
        recs.push(s3_rec_label(loop_id));
        recs.push(s3_rec_ldb(S3_SCAN_OP, S3_SCAN_PTR, 0));
        recs.push(s3_rec_cmp(S3_SCAN_OP, S3_SCAN_NEEDLE));
        recs.push(s3_rec_je(found_id));
        recs.push(s3_rec_add(S3_SCAN_PTR, 20));
        recs.push(s3_rec_add(S3_SCAN_I, 1));
        recs.push(s3_rec_cmp(S3_SCAN_I, S3_SCAN_NROWS));
        recs.push(s3_rec_jl(loop_id));
        recs.push(s3_rec_set(S3_SCAN_OUT, 0xFE));
        recs.push(s3_fixed_rec(0xFF));
        recs.push(s3_rec_label(found_id));
        recs.push(s3_rec_ldb(S3_SCAN_OUT, S3_SCAN_PTR, 1));
        recs.push(s3_rec_movrr(S3_COMPOSE_SRC, S3_SCAN_PTR));
        recs.push(s3_rec_add(S3_COMPOSE_SRC, 4));
        recs.push(s3_rec_memcpy(S3_COMPOSE_DST, S3_COMPOSE_SRC, S3_SCAN_OUT));
        if advance_dst {
            recs.push(s3_rec_addv(S3_COMPOSE_DST, S3_SCAN_OUT));
        }
    }

    /// S3.18: scan SET then RET → outbuf full `.text` ≡ emit_x86(SET+RET).
    fn s3_table_fulltext_set_ret_tyb() -> Vec<u8> {
        let table = s3_fixed_table(&[0xFF, 0x00, 0x30]);
        let mut recs = s3_pack_data_via_ir(&table);
        recs.extend_from_slice(s3_lea_ptr_a1_recs());
        recs.extend_from_slice(s3_lea_dst_a1_recs());
        append_scan_memcpy(&mut recs, 0x30, 1, 2, true);
        // re-seed table ptr for second scan
        recs.extend_from_slice(s3_lea_ptr_a1_recs());
        append_scan_memcpy(&mut recs, 0xFF, 3, 4, false);
        recs.push(s3_fixed_rec(0xFF));
        tyb_from_records(&recs)
    }

    /// S3.118: fulltext uses the shared sim; SET needle resets the scan ptr.
    fn s3_sim_table_fulltext(tyb: &[u8]) -> Result<Vec<u8>, u8> {
        s3_sim_prog_loop(tyb, 0, S3_OUTBUF_OFF, 0)
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
        let mut hand = s3_host_set_body();
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

    /// Table relocated past slot area [0..0x80).
    const S3_TABLE_OFF: u32 = 0x80;
    /// Program: [n][op…] immediately after 3×20B table.
    const S3_PROG_OFF: u32 = S3_TABLE_OFF + 60; // 0xBC

    /// S3.19/S3.221: default in-DLL prog loop is empty-DATA IR seed.
    /// Golden fixture stays [`s3_table_prog_loop_tyb_via_ir`].
    fn s3_table_prog_loop_tyb(ops: &[u8]) -> Vec<u8> {
        s3_table_prog_loop_tyb_ir_seed(ops)
    }

    /// S3.19 via_ir DATA path — golden TYB bytes only. Not the default builder.
    fn s3_table_prog_loop_tyb_via_ir(ops: &[u8]) -> Vec<u8> {
        assert!(!ops.is_empty() && ops.len() <= 8);
        let payload = s3_fixed_payload(&S3_FIXED10_OPS[..3], ops, S3_OUTBUF_OFF as usize);
        assert_eq!(S3_TABLE_OFF + 60, S3_PROG_OFF);
        assert_eq!(S3_PROG_OFF + 1, 0xBD);
        let mut recs = s3_pack_data_via_ir(&payload);
        recs.extend_from_slice(s3_in_dll_lea_prologue_recs());
        recs.extend(s3_prog_loop_code(ops.len() as u32, 3));
        tyb_from_records(&recs)
    }

    /// S3.220/S3.222: empty-DATA in-DLL — lea first (slot15 = table VA / sim table_off),
    /// MOVRR BASE←15, then shared global-distinct poke, then prog. Golden unchanged.
    fn s3_table_prog_loop_tyb_ir_seed(ops: &[u8]) -> Vec<u8> {
        assert!(!ops.is_empty() && ops.len() <= 8);
        let payload = s3_fixed_payload(&S3_FIXED10_OPS[..3], ops, S3_OUTBUF_OFF as usize);
        let start = S3_TABLE_OFF as usize;
        let end = S3_PROG_OFF as usize + 1 + ops.len();
        let slice = &payload[start..end];
        assert!(slice.len() <= 127, "in-DLL seed must stay one imm8 window");
        let mut recs = Vec::new();
        // PE: lea writes table VA into slot 15. Sim skips A1 but seeds state[15]=table_off.
        recs.extend_from_slice(s3_in_dll_lea_prologue_recs());
        recs.push(s3_rec_movrr(S3_PACK_STB_BASE, 15));
        // Shared SET×distinct poke; BASE already live (no absolute SET).
        s3_append_stb_poke_global_distinct(&mut recs, slice);
        recs.extend(s3_prog_loop_code(ops.len() as u32, 3));
        tyb_from_records(&recs)
    }

    fn s3_sim_table_prog_loop(tyb: &[u8]) -> Result<Vec<u8>, u8> {
        s3_sim_prog_loop(
            tyb,
            S3_TABLE_OFF as u64,
            S3_OUTBUF_OFF,
            (S3_PROG_OFF + 1) as u64,
        )
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
        // S3.221: fixture stays via_ir; default builder is empty-DATA.
        assert_eq!(golden, s3_table_prog_loop_tyb_via_ir(&[0x30, 0xFF]));
        assert_ne!(golden, set_ret);
    }

    /// S3.220: empty-DATA IR-seed twin matches via_ir sim; fits mini; no DATA.
    #[test]
    fn s3_table_prog_loop_ir_seed_empty_data_parity() {
        for ops in [&[0xFFu8][..], &[0x00, 0xFF][..], &[0x30, 0xFF][..]] {
            let via = s3_table_prog_loop_tyb_via_ir(ops);
            let seeded = s3_table_prog_loop_tyb_ir_seed(ops);
            assert!(s3_unpack_data_tyb(&seeded).is_empty(), "ops {ops:?}");
            assert!(
                parse_tyb(&seeded).unwrap().records.iter().all(|r| r.op != 0x10),
                "no DATA op"
            );
            assert_eq!(
                s3_sim_table_prog_loop(&seeded).unwrap(),
                s3_sim_table_prog_loop(&via).unwrap(),
                "sim parity ops {ops:?}"
            );
            compile_tyb_to_pe_mini(&seeded).expect("ir-seed mini");
            let user = emit_x86(&parse_tyb(&seeded).unwrap()).text;
            assert!(user.len() <= S1_SLIM_MAX_CODE, "code {}", user.len());
        }
        // Golden fixture still equals via_ir builder.
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../yoyo/tests/golden/s3_yoyo_table_prog_loop.tyb");
        let golden = std::fs::read(&path).expect("golden");
        assert_eq!(golden, s3_table_prog_loop_tyb_via_ir(&[0x30, 0xFF]));
        assert_eq!(
            s3_table_prog_loop_tyb(&[0x30, 0xFF]),
            s3_table_prog_loop_tyb_ir_seed(&[0x30, 0xFF]),
            "S3.221: default builder is the empty-DATA seed"
        );
    }

    #[test]
    #[cfg(windows)]
    fn s3_yoyo_table_prog_loop_ir_seed_runs_on_pe() {
        let tyb = s3_table_prog_loop_tyb_ir_seed(&[0x30, 0xFF]);
        let pe = compile_tyb_to_pe_mini(&tyb).expect("mini ir-seed");
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
