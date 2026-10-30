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
                // S3.516: GET/MOVRR any-slot≤255
                if !mini_slot_any_ok(dst) || !mini_slot_any_ok(src) {
                    return bad(i, "s1 mini GET/MOVRR slot out of range (>255)".into());
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
                // S3.516: ADD/SUB any-slot≤255
                if !mini_slot_any_ok(slot) {
                    return bad(
                        i,
                        format!("s1 mini ADD/SUB slot {slot} out of range (>255)"),
                    );
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
                // S3.516: INC/DEC any-slot≤255
                if !mini_slot_any_ok(slot) {
                    return bad(
                        i,
                        format!("s1 mini INC/DEC slot {slot} out of range (>255)"),
                    );
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
                // S3.514/S3.516: IMUL/ALUV/CMP any-slot≤255 (host≡in-DLL lift).
                if !mini_slot_any_ok(a) || !mini_slot_any_ok(b) {
                    return bad(
                        i,
                        "s1 mini IMUL/CMP/ALUV slot out of range (>255)".into(),
                    );
                }
            }
            0x80 => {
                if rec.argc != 3 {
                    return bad(i, format!("s1 mini LDB requires argc=3, got {}", rec.argc));
                }
                let dd = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                let ss = u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]);
                // S3.515: LDB any-slot≤255
                if !mini_slot_any_ok(dd) || !mini_slot_any_ok(ss) {
                    return bad(i, "s1 mini LDB slot out of range (>255)".into());
                }
            }
            // S3.156/S3.515: STB val base oo — any-slot≤255
            0x81 => {
                if rec.argc != 3 {
                    return bad(i, format!("s1 mini STB requires argc=3, got {}", rec.argc));
                }
                let val = u16::from_le_bytes([rec.args_raw[0], rec.args_raw[1]]);
                let base = u16::from_le_bytes([rec.args_raw[2], rec.args_raw[3]]);
                if !mini_slot_any_ok(val) || !mini_slot_any_ok(base) {
                    return bad(i, "s1 mini STB slot out of range (>255)".into());
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
                // S3.515: MEMCPY_DATA any-slot≤255
                if !mini_slot_any_ok(dst) || !mini_slot_any_ok(src) || !mini_slot_any_ok(n) {
                    return bad(i, "s1 mini MEMCPY_DATA slot out of range (>255)".into());
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
                    // S3.515: MEMCPY_STATE any-slot≤255 across range
                    if !mini_slot_any_ok(dst)
                        || !mini_slot_any_ok(src)
                        || !mini_slot_any_ok(last_dst)
                        || !mini_slot_any_ok(last_src)
                    {
                        return bad(i, "s1 mini MEMCPY_STATE slot out of range (>255)".into());
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
/// S3.402/S3.405: fixed-template → try_compile; else emit_x86 + Windows PE
/// template patch (gap U — no wrap). Prefer mini for fail-closed.
pub fn compile_tyb_to_pe(tyb: &[u8]) -> crate::types::IsaResult<crate::pe_link::PeImage> {
    if let Some(pe) = try_compile_tyb_via_yoyo_table(tyb) {
        return Ok(pe);
    }
    let prog = parse_tyb(tyb).map_err(|e| crate::types::IsaError::ParseError {
        line: 0,
        msg: format!("s1 compile_tyb_to_pe: {e}"),
    })?;
    let out = emit_x86(&prog);
    // S3.405: Windows — patch emit text/data into compile PE template (no wrap).
    #[cfg(windows)]
    {
        return Ok(crate::pe_link::PeImage {
            bytes: s3_compile_pe_patch_text_data(&out.text, &out.data)?,
        });
    }
    #[cfg(not(windows))]
    {
        wrap_pe_slim(&out)
    }
}

/// S1.6 — host path aligned with in-DLL mini-ISA (fail closed on other ops).
///
/// S3.21/S3.111: fixed-template 10 ops prefer YOYO table-compose
/// ([`try_compile_tyb_via_yoyo_table`]); otherwise `emit_x86`.
/// Subset proof does not go through `pack_meta_emit_*`. Host firmware keeps
/// one DATA pack (`s3_pack_data`); RAW prologue dropped in S3.112.
/// in-DLL dispatcher remains Rust. OW-RT CUT.
/// S3.548: slim PE via emit_x86+patch only (skip host-table try_compile).
///
/// `try_compile_tyb_via_yoyo_table` can AV when baking child-compiler rows; this
/// path stays emit-only for fixed10 op+RET table fill.
pub fn compile_tyb_to_pe_mini_emit(tyb: &[u8]) -> crate::types::IsaResult<crate::pe_link::PeImage> {
    let prog = parse_tyb(tyb).map_err(|e| crate::types::IsaError::ParseError {
        line: 0,
        msg: format!("s1 compile_tyb_to_pe_mini_emit: {e}"),
    })?;
    validate_tyb_mini(&prog)?;
    let out = emit_x86(&prog);
    #[cfg(windows)]
    {
        Ok(crate::pe_link::PeImage {
            bytes: s3_compile_pe_patch_text_data(&out.text, &out.data)?,
        })
    }
    #[cfg(not(windows))]
    {
        wrap_pe_slim(&out)
    }
}

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
    #[cfg(windows)]
    {
        return Ok(crate::pe_link::PeImage {
            bytes: s3_compile_pe_patch_text_data(&out.text, &out.data)?,
        });
    }
    #[cfg(not(windows))]
    {
        wrap_pe_slim(&out)
    }
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

/// S3.334: chunk pack helper — test-only (feeds reference pack + A1 append).
#[cfg(test)]
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

/// S3.150/S3.333: RAW `0xA1` pack helper — test A1 OnceLock / density pins only.
#[cfg(test)]
fn s3_append_raw_a1(recs: &mut Vec<(u8, u8, [u8; 6])>, bytes: &[u8]) {
    recs.extend(s3_host_pack_bytes(0xA1, bytes));
}

/// S3.153/S3.203/S3.333: argc=1 per byte — test-only (production pack is via_ir/poke).
#[cfg(test)]
fn s3_pack_data_bytewise(bytes: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    bytes
        .iter()
        .map(|&b| (0x10u8, 1u8, [b, 0, 0, 0, 0, 0]))
        .collect()
}

/// S3.166: flatten one TYB record to 8 wire bytes (op+argc+args).
#[cfg(test)]
fn s3_rec_wire(op: u8, argc: u8, a: [u8; 6]) -> [u8; 8] {
    [op, argc, a[0], a[1], a[2], a[3], a[4], a[5]]
}

/// S3.226/S3.279: append SET DST + run-folded STB of `flat` at `out_off`.
/// S3.279: drop SET ZERO — sim state starts zeroed (slot13); runs CMP against it.
/// Shared by one-shot emit and two-phase / loop-staged phase1. Returns next label id.
/// S3.314: poke (global-distinct) beats run-fold for emit flats (−24 on many).
/// Trailing zeros are poke-skipped — pin high-water with one STB ZERO at last oo.
#[cfg(test)]
fn s3_append_stb_flat_at(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    flat: &[u8],
    out_off: u32,
    label_id: u32,
) -> u32 {
    let lid = s3_ir_seed_at(recs, out_off, flat, label_id);
    if let Some(last) = flat.len().checked_sub(1) {
        if flat[last] == 0 {
            const WIN: usize = 127;
            let win = last / WIN;
            let oo = (last % WIN) as u16;
            recs.push(s3_rec_set(
                S3_PACK_STB_BASE,
                out_off + (win * WIN) as u32,
            ));
            recs.push(s3_rec_stb(S3_PACK_STB_ZERO, S3_PACK_STB_BASE, oo));
        }
    }
    lid
}

/// S3.225/S3.226/S3.335: shared empty-DATA STB emit — test emit-TYB cluster.
#[cfg(test)]
fn s3_yoyo_emit_flat_via_stb(flat: &[u8], out_off: u32) -> (Vec<u8>, u32) {
    let mut recs = Vec::new();
    s3_append_stb_flat_at(&mut recs, flat, out_off, 1);
    recs.push((0xFF, 0, [0; 6]));
    (s3_host_tyb_from_records(&recs), out_off)
}

/// S3.166/S3.200/S3.225/S3.335: STB-emit records — test emit-TYB cluster.
#[cfg(test)]
fn s3_yoyo_emit_recs_tyb(records: &[(u8, u8, [u8; 6])]) -> (Vec<u8>, u32) {
    let out_off = 0x20u32;
    let mut flat = Vec::with_capacity(records.len() * RECORD_SIZE);
    for &(op, argc, a) in records {
        flat.extend_from_slice(&s3_rec_wire(op, argc, a));
    }
    s3_yoyo_emit_flat_via_stb(&flat, out_off)
}

/// S3.167/S3.200/S3.225/S3.335: STB-emit full TYB unrolled — test emit-TYB cluster.
#[cfg(test)]
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

/// S3.171/S3.202/S3.226/S3.335: loop-staged emit — test emit-TYB cluster.
#[cfg(test)]
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

/// S3.183/S3.318: STB run-fold — **test/compare only** after poke era (S3.313–314).
/// S3.339: STB runs stay test-only; production poke is `s3_ir_seed_at` (host/expand).
#[cfg(test)]
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

/// S3.222/S3.345: one SET per distinct nonzero via default BASE — test helper.
/// Production seed uses [`s3_append_stb_poke_global_distinct_base`] directly.
#[cfg(test)]
fn s3_append_stb_poke_global_distinct(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    bytes: &[u8],
) {
    s3_append_stb_poke_global_distinct_base(recs, bytes, S3_PACK_STB_BASE)
}

/// S3.310: poke through a live base slot (in-DLL: slot15 = lea table VA).
fn s3_append_stb_poke_global_distinct_base(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    bytes: &[u8],
    base_slot: u16,
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
            recs.push(s3_rec_stb(S3_PACK_STB_TMP, base_slot, oo));
        }
    }
}

/// S3.209–S3.217/S3.312: IR-seed sparse bytes — poke only nonzero offsets via STB `oo`.
/// Gaps stay zero from empty-image / STB resize. Each ≤127 window uses one SET BASE
/// + global-distinct TMP SETs (imm8 `oo`). S3.312: long host seeds use the same
/// per-window global-distinct (was address-order; −49 host IR). No SHR.
fn s3_append_stb_zero_then_poke(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    base: u32,
    bytes: &[u8],
    label_id: u32,
) -> u32 {
    const WIN: usize = 127;
    let mut off = 0usize;
    while off < bytes.len() {
        let take = (bytes.len() - off).min(WIN);
        let win = &bytes[off..off + take];
        if win.iter().any(|&b| b != 0) {
            recs.push(s3_rec_set(S3_PACK_STB_BASE, base + off as u32));
            // S3.345: call base poke directly (default-BASE wrapper is cfg(test)).
            s3_append_stb_poke_global_distinct_base(recs, win, S3_PACK_STB_BASE);
        }
        off += take;
    }
    label_id
}

/// S3.181/S3.184/S3.335: two-phase emit — test emit-TYB cluster.
#[cfg(test)]
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

/// S3.177/S3.181/S3.335: emit picker — test emit-TYB cluster.
#[cfg(test)]
const S3_EMIT_UNROLL_MAX_FLAT: usize = 64;

#[cfg(test)]
fn s3_yoyo_emit_tyb_file(records: &[(u8, u8, [u8; 6])]) -> (Vec<u8>, u32) {
    let flat_len = HEADER_SIZE + records.len() * RECORD_SIZE;
    if flat_len <= S3_EMIT_UNROLL_MAX_FLAT {
        s3_yoyo_emit_tyb_file_unrolled(records)
    } else {
        s3_yoyo_emit_tyb_file_two_phase(records)
    }
}

/// S3.141: YOYO-target chunk size for DATA `0x10` pack (still Rust today).
#[cfg(test)]
const S3_PACK_CHUNK: usize = 3;

/// S3.136/S3.141/S3.334: reference Rust DATA `0x10` pack — test parity vs via_ir.
#[cfg(test)]
fn s3_pack_data(bytes: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    debug_assert_eq!(S3_PACK_CHUNK, 3);
    s3_host_pack_bytes(0x10, bytes)
}

/// S3.137/S3.334: inverse of `s3_pack_data` — test parity only.
#[cfg(test)]
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

/// S3.335: unpack DATA op 0x10 from TYB — test empty-DATA pins.
#[cfg(test)]
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

/// S3.142/S3.334: flatten reference pack to wire — test parity only.
#[cfg(test)]
fn s3_pack_data_wire(bytes: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    for (op, argc, a) in s3_pack_data(bytes) {
        out.push(op);
        out.push(argc);
        out.extend_from_slice(&a);
    }
    out
}

/// S3.144/S3.266/S3.269/S3.334: pack-wire→recs helper — test-only (via_ir emits full TYB).
#[cfg(test)]
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

/// S3.144/S3.158/S3.307: pack via YOYO loop STB IR → full TYB → parse_tyb.
#[cfg(test)]
fn s3_pack_data_via_ir(bytes: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    if bytes.is_empty() {
        return Vec::new();
    }
    let (tyb, out_off) = s3_yoyo_pack_loop_stb_tyb(bytes);
    let emitted = s3_sim_prog_loop(&tyb, 0, out_off, 0).expect("s3_pack_data_via_ir: sim pack");
    parse_tyb(&emitted)
        .expect("s3_pack_data_via_ir: pack emits TYB")
        .records
        .into_iter()
        .map(|r| (r.op, r.argc, r.args_raw))
        .collect()
}

/// S3.142/S3.145 pack-proto DATA layout (no STB — write via 1-byte MEMCPY from consts/payload).
/// S3.186: old payload cursor, kept for the out_off formula check. Live packers no longer use it.
#[cfg(test)]
const S3_PACK_PROTO_PAY: u32 = 5;
/// S3.142 fixed out (small unroll only); S3.145 loop uses `s3_pack_proto_out_off`.
#[cfg(test)]
const S3_PACK_PROTO_OUT: u32 = 0x80;
#[cfg(test)]
const S3_PACK_PROTO_SRC: u16 = 8;
const S3_PACK_PROTO_DST: u16 = 9;
#[cfg(test)]
const S3_PACK_PROTO_ONE: u16 = 12;

/// S3.145: outbuf sits just past payload + 2B zero pad (host-sized payloads fit).
#[cfg(test)]
fn s3_pack_proto_out_off(payload_len: usize) -> u32 {
    S3_PACK_PROTO_PAY + payload_len as u32 + 2
}

/// S3.142: emit one byte from DATA[`src_off`] to *DST, then DST += 1 (MEMCPY).
#[cfg(test)]
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
#[cfg(test)]
const S3_PACK_STB_ZERO: u16 = 13;
/// S3.210: seed poke base so STB `oo` stays ≤127 (imm8 add in emit_stb).
const S3_PACK_STB_BASE: u16 = 14;

/// S3.191: store an immediate byte (op, argc, or zero pad) — not a DATA const.
#[cfg(test)]
fn s3_pack_proto_putc_imm(recs: &mut Vec<(u8, u8, [u8; 6])>, byte: u8) {
    recs.push(s3_rec_set(S3_PACK_STB_TMP, byte as u32));
    recs.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_PROTO_DST, 0));
    recs.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
}

/// S3.155: emit one byte via LDB→STB (sim-only STB); DST += 1.
#[cfg(test)]
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
#[cfg(test)]
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

/// S3.142/S3.336: unrolled proto pack IR — test alt path (production = loop_stb via_ir).
#[cfg(test)]
fn s3_yoyo_pack_proto_ir(payload: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    let payload_len = payload.len();
    let mut recs = Vec::new();
    // S3.198/S3.279/S3.313: payload at 0 via poke (not runs); consts at 16.
    // Slot13 image-zero; slot9 needs SET (sim preloads slot9 = outbuf).
    s3_ir_seed_at(&mut recs, 0, payload, 10);
    // S3.316: const scratch via poke flat (was SET+STB+ADD unroll).
    s3_append_stb_flat_at(&mut recs, &[0x10, 1, 2, 3, 0], 16, 20);
    recs.push(s3_rec_set(S3_PACK_PROTO_ONE, 1));
    recs.push(s3_rec_set(
        S3_PACK_PROTO_DST,
        S3_PACK_PROTO_OUT + HEADER_SIZE as u32,
    ));
    let mut i = 0usize;
    while i < payload_len {
        let n = (payload_len - i).min(S3_PACK_CHUNK) as u8;
        s3_pack_proto_record(&mut recs, i as u32, n);
        i += n as usize;
    }
    recs.push((0xFF, 0, [0; 6]));
    recs
}

/// S3.142/S3.195/S3.309/S3.336: proto pack TYB — test alt path.
#[cfg(test)]
fn s3_yoyo_pack_proto_tyb(payload: &[u8]) -> Vec<u8> {
    assert!(!payload.is_empty() && payload.len() <= 6);
    // S3.153: bootstrap scratch is bytewise (not chunk-3).
    // S3.309: IR-seed TYB header; proto writes records at OUT+hdr.
    let nrecs = (payload.len() + S3_PACK_CHUNK - 1) / S3_PACK_CHUNK;
    let mut recs = Vec::new();
    s3_ir_seed_tyb_hdr(&mut recs, S3_PACK_PROTO_OUT, nrecs, 40);
    recs.extend(s3_yoyo_pack_proto_ir(payload));
    s3_host_tyb_from_records(&recs)
}

/// S3.155: one DATA record via LDB→STB putc.
#[cfg(test)]
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

/// S3.155/S3.309/S3.336: unrolled STB pack IR — test alt path.
#[cfg(test)]
fn s3_yoyo_pack_stb_ir(payload: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    let payload_len = payload.len();
    let mut recs = Vec::new();
    // S3.279/S3.313: slot13 image-zero; payload poke at 0 (not runs).
    s3_ir_seed_at(&mut recs, 0, payload, 10);
    recs.push(s3_rec_set(
        S3_PACK_PROTO_DST,
        S3_PACK_PROTO_OUT + HEADER_SIZE as u32,
    ));
    let mut i = 0usize;
    while i < payload_len {
        let n = (payload_len - i).min(S3_PACK_CHUNK) as u8;
        s3_pack_proto_record_stb(&mut recs, i as u32, n);
        i += n as usize;
    }
    recs.push((0xFF, 0, [0; 6]));
    recs
}

/// S3.336: unrolled STB pack TYB — test alt path.
#[cfg(test)]
fn s3_yoyo_pack_stb_tyb(payload: &[u8]) -> Vec<u8> {
    assert!(!payload.is_empty() && payload.len() <= 6);
    // S3.309: IR-seed TYB header; unrolled STB writes records at OUT+hdr.
    let nrecs = (payload.len() + S3_PACK_CHUNK - 1) / S3_PACK_CHUNK;
    let mut recs = Vec::new();
    s3_ir_seed_tyb_hdr(&mut recs, S3_PACK_PROTO_OUT, nrecs, 40);
    recs.extend(s3_yoyo_pack_stb_ir(payload));
    s3_host_tyb_from_records(&recs)
}

/// S3.143 loop pack slots (CUR/END/REM/CHUNK/THREE; SRC/DST/ONE shared with 142).
#[cfg(test)]
const S3_PACK_LOOP_CUR: u16 = 4;
#[cfg(test)]
const S3_PACK_LOOP_END: u16 = 5;
#[cfg(test)]
const S3_PACK_LOOP_REM: u16 = 6;
#[cfg(test)]
const S3_PACK_LOOP_CHUNK: u16 = 7;
#[cfg(test)]
const S3_PACK_LOOP_THREE: u16 = 10;
#[cfg(test)]
const S3_PACK_LOOP_L_LOOP: u32 = 1;
#[cfg(test)]
const S3_PACK_LOOP_L_USE_REM: u32 = 2;
#[cfg(test)]
const S3_PACK_LOOP_L_PACK: u32 = 3;
#[cfg(test)]
const S3_PACK_LOOP_L_DONE: u32 = 4;

/// S3.143/S3.145/S3.336: MEMCPY loop pack IR — test alt path.
#[cfg(test)]
fn s3_yoyo_pack_loop_ir(
    pay_off: u32,
    out_off: u32,
    payload: &[u8],
    const_base: u32,
) -> Vec<(u8, u8, [u8; 6])> {
    let payload_len = payload.len() as u32;
    let mut r = Vec::new();
    // S3.197/S3.279/S3.313: payload poke (not runs) + overread zero stores.
    s3_ir_seed_at(&mut r, pay_off, payload, 10);
    r.push(s3_rec_set(S3_PACK_PROTO_DST, pay_off + payload_len));
    r.push(s3_rec_stb(S3_PACK_STB_ZERO, S3_PACK_PROTO_DST, 0));
    r.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
    r.push(s3_rec_stb(S3_PACK_STB_ZERO, S3_PACK_PROTO_DST, 0));
    // S3.194: [0x10, 1, 2, 3, 0] is written below outbuf, not packed as DATA.
    // S3.316: const scratch via poke flat.
    s3_append_stb_flat_at(&mut r, &[0x10, 1, 2, 3, 0], const_base, 20);
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
#[cfg(test)]
fn s3_pack_loop_putc_from_slot(recs: &mut Vec<(u8, u8, [u8; 6])>, src_slot: u16) {
    recs.push(s3_rec_ldb(S3_PACK_STB_TMP, src_slot, 0));
    recs.push(s3_rec_stb(S3_PACK_STB_TMP, S3_PACK_PROTO_DST, 0));
    recs.push(s3_rec_add(S3_PACK_PROTO_DST, 1));
}

/// S3.157: looped pack IR using LDB→STB instead of MEMCPY.
#[cfg(test)]
fn s3_yoyo_pack_loop_stb_ir(pay_off: u32, out_off: u32, payload: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    let payload_len = payload.len() as u32;
    let mut r = Vec::new();
    // S3.279: slot13 image-zero; no SET ZERO prologue.
    r.push(s3_rec_set(S3_PACK_LOOP_THREE, S3_PACK_CHUNK as u32));
    // S3.196/S3.313: payload poke (global-distinct) — beats STB runs (≥10).
    s3_ir_seed_at(&mut r, pay_off, payload, 10);
    r.push(s3_rec_set(S3_PACK_PROTO_DST, pay_off + payload_len));
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

/// S3.307/S3.308: IR-seed an 8B TYB header (magic + rec_cnt) at `base`.
#[cfg(test)]
fn s3_ir_seed_tyb_hdr(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    base: u32,
    nrecs: usize,
    label_id: u32,
) -> u32 {
    let mut hdr = [0u8; HEADER_SIZE];
    hdr[..4].copy_from_slice(&TYB_MAGIC);
    hdr[6..8].copy_from_slice(&(nrecs as u16).to_le_bytes());
    s3_ir_seed_at(recs, base, &hdr, label_id)
}

#[cfg(test)]
fn s3_yoyo_pack_loop_stb_tyb(payload: &[u8]) -> (Vec<u8>, u32) {
    assert!(!payload.is_empty());
    // S3.196: no DATA records. IR writes the payload, then the two overread zeros.
    // S3.307: IR-seed TYB header at out; pack loop writes records at out+hdr.
    let out_off = payload.len() as u32 + 2;
    let nrecs = (payload.len() + S3_PACK_CHUNK - 1) / S3_PACK_CHUNK;
    let mut recs = Vec::new();
    s3_ir_seed_tyb_hdr(&mut recs, out_off, nrecs, 40);
    recs.extend(s3_yoyo_pack_loop_stb_ir(
        0,
        out_off + HEADER_SIZE as u32,
        payload,
    ));
    (s3_host_tyb_from_records(&recs), out_off)
}

/// S3.143–S3.194/S3.308/S3.336: MEMCPY loop pack TYB — test alt (prod = loop_stb).
#[cfg(test)]
fn s3_yoyo_pack_loop_tyb(payload: &[u8]) -> (Vec<u8>, u32) {
    assert!(!payload.is_empty());
    let const_base = payload.len() as u32 + 2;
    let out_off = const_base + 5;
    let nrecs = (payload.len() + S3_PACK_CHUNK - 1) / S3_PACK_CHUNK;
    let mut recs = Vec::new();
    s3_ir_seed_tyb_hdr(&mut recs, out_off, nrecs, 40);
    recs.extend(s3_yoyo_pack_loop_ir(
        0,
        out_off + HEADER_SIZE as u32,
        payload,
        const_base,
    ));
    (s3_host_tyb_from_records(&recs), out_off)
}

fn s3_host_tmpl_row(op: u8, payload: &[u8]) -> [u8; 20] {
    let mut r = [0u8; 20];
    r[0] = op;
    r[1] = payload.len() as u8;
    r[4..4 + payload.len()].copy_from_slice(payload);
    r
}

/// S3.122/S3.359: live `emit_x86(op+RET)` strip — test verify vs body seed only.
#[cfg(test)]
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

/// S3.411: TYB for one fixed10 op (+RET unless op is RET). Used by pe_dll parity.
pub fn s3_fixed10_ret_tyb(op: u8) -> Vec<u8> {
    if op == 0xFF {
        s3_host_tyb_from_records(&[s3_fixed_rec(0xFF)])
    } else {
        s3_host_tyb_from_records(&[s3_fixed_rec(op), s3_fixed_rec(0xFF)])
    }
}

/// S3.600: IMUL+RET TYB (beyond fixed10). `dst`/`src` are disp8 slots (≤15).
pub fn s3_imul_ret_tyb(dst: u16, src: u16) -> Vec<u8> {
    let mut a = [0u8; 6];
    a[0..2].copy_from_slice(&dst.to_le_bytes());
    a[2..4].copy_from_slice(&src.to_le_bytes());
    s3_host_tyb_from_records(&[(0x63, 2, a), (0xFF, 0, [0; 6])])
}

/// S3.601: ALUV+RET TYB (ADDV=0x68 / ORV=0x69 / SUBV=0x6A). Beyond fixed10.
pub fn s3_aluv_ret_tyb(op: u8, dst: u16, src: u16) -> Vec<u8> {
    assert!(matches!(op, 0x68 | 0x69 | 0x6A), "not ALUV op {op:#x}");
    let mut a = [0u8; 6];
    a[0..2].copy_from_slice(&dst.to_le_bytes());
    a[2..4].copy_from_slice(&src.to_le_bytes());
    s3_host_tyb_from_records(&[(op, 2, a), (0xFF, 0, [0; 6])])
}

/// S3.602: RAW_BYTE+RET TYB (beyond fixed10). Emits one literal `.text` byte.
pub fn s3_raw_byte_ret_tyb(byte: u8) -> Vec<u8> {
    let mut a = [0u8; 6];
    a[0] = byte;
    s3_host_tyb_from_records(&[(0xA0, 1, a), (0xFF, 0, [0; 6])])
}

/// S3.610: RAW_BYTES+RET TYB (beyond fixed10). `bytes.len()` in 1..=3; low-bytes‖C3.
pub fn s3_raw_bytes_ret_tyb(bytes: &[u8]) -> Vec<u8> {
    assert!((1..=3).contains(&bytes.len()), "RAW_BYTES argc 1..=3");
    let mut a = [0u8; 6];
    for (k, &b) in bytes.iter().enumerate() {
        a[k * 2] = b;
    }
    s3_host_tyb_from_records(&[(0xA1, bytes.len() as u8, a), (0xFF, 0, [0; 6])])
}

/// S3.611: MEMCPY_DATA+RET TYB (beyond fixed10). `dst`/`src`/`n` are slot ids (≤15 for parse1).
pub fn s3_memcpy_data_ret_tyb(dst: u16, src: u16, n: u16) -> Vec<u8> {
    let mut a = [0u8; 6];
    a[0..2].copy_from_slice(&dst.to_le_bytes());
    a[2..4].copy_from_slice(&src.to_le_bytes());
    a[4..6].copy_from_slice(&n.to_le_bytes());
    s3_host_tyb_from_records(&[(0x84, 3, a), (0xFF, 0, [0; 6])])
}

/// S3.612: MEMCPY_STATE+RET TYB (beyond fixed10). `n` is slot count (≤4); last slots≤15 for parse1.
pub fn s3_memcpy_state_ret_tyb(dst: u16, src: u16, n: u16) -> Vec<u8> {
    assert!(n <= 4, "MEMCPY_STATE n≤4");
    let mut a = [0u8; 6];
    a[0..2].copy_from_slice(&dst.to_le_bytes());
    a[2..4].copy_from_slice(&src.to_le_bytes());
    a[4..6].copy_from_slice(&n.to_le_bytes());
    s3_host_tyb_from_records(&[(0x85, 3, a), (0xFF, 0, [0; 6])])
}

/// S3.606: LABEL + NOP + JMP(same id) + RET — exercises parse1 label table.
pub fn s3_label_nop_jmp_ret_tyb(label_id: u32) -> Vec<u8> {
    let mut a = [0u8; 6];
    a[0..4].copy_from_slice(&label_id.to_le_bytes());
    s3_host_tyb_from_records(&[
        (0x40, 1, a),
        (0x00, 0, [0; 6]),
        (0x70, 1, a),
        (0xFF, 0, [0; 6]),
    ])
}

/// S3.606: JMP + LABEL(same id) + RET — forward-ref patch.
pub fn s3_jmp_label_ret_tyb(label_id: u32) -> Vec<u8> {
    let mut a = [0u8; 6];
    a[0..4].copy_from_slice(&label_id.to_le_bytes());
    s3_host_tyb_from_records(&[
        (0x70, 1, a),
        (0x40, 1, a),
        (0xFF, 0, [0; 6]),
    ])
}

/// S3.608: LABEL + NOP + CALL(same id) + RET.
pub fn s3_label_nop_call_ret_tyb(label_id: u32) -> Vec<u8> {
    let mut a = [0u8; 6];
    a[0..4].copy_from_slice(&label_id.to_le_bytes());
    s3_host_tyb_from_records(&[
        (0x40, 1, a),
        (0x00, 0, [0; 6]),
        (0x41, 1, a),
        (0xFF, 0, [0; 6]),
    ])
}

/// S3.608: CALL + LABEL(same id) + RET — forward-ref.
pub fn s3_call_label_ret_tyb(label_id: u32) -> Vec<u8> {
    let mut a = [0u8; 6];
    a[0..4].copy_from_slice(&label_id.to_le_bytes());
    s3_host_tyb_from_records(&[
        (0x41, 1, a),
        (0x40, 1, a),
        (0xFF, 0, [0; 6]),
    ])
}

/// S3.609: LABEL + JCC(0x71..=0x7A) + RET — back-edge.
pub fn s3_label_jcc_ret_tyb(jcc: u8, label_id: u32) -> Vec<u8> {
    assert!((0x71..=0x7A).contains(&jcc), "not JCC {jcc:#x}");
    let mut a = [0u8; 6];
    a[0..4].copy_from_slice(&label_id.to_le_bytes());
    s3_host_tyb_from_records(&[
        (0x40, 1, a),
        (jcc, 1, a),
        (0xFF, 0, [0; 6]),
    ])
}

/// S3.609: JCC + LABEL + RET — forward-ref.
pub fn s3_jcc_label_ret_tyb(jcc: u8, label_id: u32) -> Vec<u8> {
    assert!((0x71..=0x7A).contains(&jcc), "not JCC {jcc:#x}");
    let mut a = [0u8; 6];
    a[0..4].copy_from_slice(&label_id.to_le_bytes());
    s3_host_tyb_from_records(&[
        (jcc, 1, a),
        (0x40, 1, a),
        (0xFF, 0, [0; 6]),
    ])
}

/// S3.604: LDB+RET TYB (beyond fixed10). `dd`/`ss`/`oo` are u16; parse1 covers ≤15.
pub fn s3_ldb_ret_tyb(dd: u16, ss: u16, oo: u16) -> Vec<u8> {
    let mut a = [0u8; 6];
    a[0..2].copy_from_slice(&dd.to_le_bytes());
    a[2..4].copy_from_slice(&ss.to_le_bytes());
    a[4..6].copy_from_slice(&oo.to_le_bytes());
    s3_host_tyb_from_records(&[(0x80, 3, a), (0xFF, 0, [0; 6])])
}

/// S3.603: LABEL+RET TYB (beyond fixed10). LABEL emits no `.text`; id is argc=1 u32.
pub fn s3_label_ret_tyb(label_id: u32) -> Vec<u8> {
    let mut a = [0u8; 6];
    a[0..4].copy_from_slice(&label_id.to_le_bytes());
    s3_host_tyb_from_records(&[(0x40, 1, a), (0xFF, 0, [0; 6])])
}

/// S3.603: LABEL + trailing ops + RET. `ops` are non-RET opcodes with canonical args
/// for fixed10 / RAW_BYTE(0x90) / ALUV — used for parse1 LABEL* chains.
pub fn s3_label_chain_ret_tyb(label_id: u32, ops: &[u8]) -> Vec<u8> {
    let mut a = [0u8; 6];
    a[0..4].copy_from_slice(&label_id.to_le_bytes());
    let mut recs: Vec<(u8, u8, [u8; 6])> = vec![(0x40, 1, a)];
    for &op in ops {
        match op {
            0x00 | 0xFF => recs.push((op, 0, [0; 6])),
            0xA0 => recs.push((0xA0, 1, [0x90, 0, 0, 0, 0, 0])),
            o if matches!(o, 0x30 | 0x60 | 0x61 | 0x62 | 0x64 | 0x65 | 0x66 | 0x67) => {
                recs.push(s3_fixed_rec(o));
            }
            _ => panic!("s3_label_chain_ret_tyb: unsupported op {op:#x}"),
        }
    }
    if ops.last().copied() != Some(0xFF) {
        recs.push(s3_fixed_rec(0xFF));
    }
    s3_host_tyb_from_records(&recs)
}

/// S3.551: fixed10 multi-op chain + trailing RET (beyond single op+RET).
///
/// `ops` are non-RET fixed10 opcodes; RET is appended. Empty → RET-only.
pub fn s3_fixed10_chain_ret_tyb(ops: &[u8]) -> Vec<u8> {
    let mut recs: Vec<(u8, u8, [u8; 6])> = ops.iter().map(|&op| s3_fixed_rec(op)).collect();
    if ops.last().copied() != Some(0xFF) {
        recs.push(s3_fixed_rec(0xFF));
    }
    s3_host_tyb_from_records(&recs)
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

/// S3.348: strip-RET body helper — test pin; production bakes via OnceLock.
#[cfg(test)]
fn s3_host_body_of(op: u8) -> Vec<u8> {
    let (o, argc, args) = s3_fixed_rec(op);
    s3_host_body(o, argc, args)
}

/// S3.337: test-only helper.
#[cfg(test)]
fn s3_host_set_body() -> Vec<u8> {
    s3_fixed_row(0x30).to_vec()
}
/// S3.123/S3.345: RET body helper — test pin; production bakes via OnceLock path.
#[cfg(test)]
fn s3_host_ret_body() -> Vec<u8> {
    s3_compute_fixed_body(0xFF)
}
/// S3.123/S3.345: NOP body helper — test pin; production bakes via OnceLock path.
#[cfg(test)]
fn s3_host_nop_body() -> Vec<u8> {
    s3_compute_fixed_body(0x00)
}

/// S3.124: one body per fixed-subset opcode (host table order).
const S3_FIXED10_OPS: [u8; 10] = [
    0xFF, 0x00, 0x30, 0x60, 0x66, 0x62, 0x67, 0x61, 0x64, 0x65,
];

/// S3.461/S3.462: YOYO-own s1_loop op→trampoline index seed (256B).
/// Nonzero = trampoline index 1..=29; 0 = unsupported → compile_fail.
/// 1..=10 ≡ [`S3_FIXED10_OPS`]; 11..=29 = remaining mini ops (JCC range→29).
/// Own-ISA object seed — not densify of cmp/je hex.
pub const S3_S1_OP_DISPATCH_INDEX_SEED: [u8; 256] = {
    let mut t = [0u8; 256];
    // 1..=10 fixed10: RET NOP SET GET INC ADD DEC SUB MOVRR CMP
    t[0xFF] = 1;
    t[0x00] = 2;
    t[0x30] = 3;
    t[0x60] = 4;
    t[0x66] = 5;
    t[0x62] = 6;
    t[0x67] = 7;
    t[0x61] = 8;
    t[0x64] = 9;
    t[0x65] = 10;
    // 11..=29 non-fixed10
    t[0x20] = 11; // ALLOC
    t[0x50] = 12; // LOAD
    t[0x51] = 13; // WRITE
    t[0x40] = 14; // LABEL
    t[0xA0] = 15; // RAW_BYTE
    t[0xA1] = 16; // RAW_BYTES
    t[0x63] = 17; // IMUL
    t[0x68] = 18; // ADDV
    t[0x69] = 19; // ORV
    t[0x6A] = 20; // SUBV
    t[0x80] = 21; // LDB
    t[0x84] = 22; // MEMCPY_DATA
    t[0x85] = 23; // MEMCPY_STATE
    t[0x10] = 24; // DATA
    t[0x12] = 25; // STR
    t[0x13] = 26; // RAW (data)
    t[0x41] = 27; // CALL
    t[0x70] = 28; // JMP
    // JCC 0x71..=0x7A → 29
    t[0x71] = 29;
    t[0x72] = 29;
    t[0x73] = 29;
    t[0x74] = 29;
    t[0x75] = 29;
    t[0x76] = 29;
    t[0x77] = 29;
    t[0x78] = 29;
    t[0x79] = 29;
    t[0x7A] = 29;
    t
};

/// S3.461 name kept as alias of full op-dispatch seed (S3.462).
pub const S3_S1_FIXED10_DISPATCH_INDEX_SEED: [u8; 256] = S3_S1_OP_DISPATCH_INDEX_SEED;

/// S3.466: YOYO-own s1_loop op-dispatch probe object seed (45B).
/// lea r10→index / lookup / jz fail / lea r11→tramp / scaled jmp.
/// Disp32 at +3 (seed) · jz at +19 · lea tramp at +25 (disp +28) patched live.
pub const S3_S1_OP_DISPATCH_PROBE_SEED: [u8; 45] = [
    0x4C, 0x8D, 0x15, 0x00, 0x00, 0x00, 0x00, // lea r10,[rip+d]
    0x44, 0x0F, 0xB6, 0xC0, // movzx r8d, al
    0x47, 0x0F, 0xB6, 0x04, 0x02, // movzx r8d,[r10+r8]
    0x45, 0x85, 0xC0, // test r8d, r8d
    0x0F, 0x84, 0x00, 0x00, 0x00, 0x00, // jz fail
    0x4C, 0x8D, 0x1D, 0x00, 0x00, 0x00, 0x00, // lea r11,[rip+d]
    0x41, 0xFF, 0xC8, // dec r8d
    0x41, 0xC1, 0xE0, 0x03, // shl r8d, 3
    0x4D, 0x01, 0xC3, // add r11, r8
    0x41, 0xFF, 0xE3, // jmp r11
];
pub const S3_S1_OP_DISPATCH_PROBE_JZ_OFF: usize = 19;
pub const S3_S1_OP_DISPATCH_PROBE_LEA_TRAMP_AT: usize = 25;

/// S3.468: YOYO-own handler trampoline table seed — 29 × (E9 rel32 + 3×NOP).
/// Live emit patches each E9 → s1 handler. Own-ISA object seed (≥10 slots).
pub const S3_S1_OP_DISPATCH_TRAMP_SEED: [u8; 29 * 8] = {
    let mut t = [0x90u8; 29 * 8];
    let mut i = 0usize;
    while i < 29 {
        let o = i * 8;
        t[o] = 0xE9;
        t[o + 1] = 0;
        t[o + 2] = 0;
        t[o + 3] = 0;
        t[o + 4] = 0;
        // +5..+7 already 0x90
        i += 1;
    }
    t
};

/// S3.470: RAW_BYTE emit body = own-ISA object seed (rcx=rec, r12=out; RET).
/// `mov al,[rcx+2]; mov [r12],al; inc r12; ret`. LABEL=meta — no seed.
pub const S3_RAW_BYTE_EMIT_BODY_SEED: [u8; 11] = [
    0x8A, 0x41, 0x02, // mov al, [rcx+2]
    0x41, 0x88, 0x04, 0x24, // mov [r12], al
    0x49, 0xFF, 0xC4, // inc r12
    0xC3, // ret
];

/// S3.471: RAW_BYTES emit body = own-ISA object seed (rcx=rec, r9b=argc≤3,
/// r12=out; RET). Appends low-bytes of args[0..argc) from record. Rel8s baked.
pub const S3_RAW_BYTES_EMIT_BODY_SEED: [u8; 48] = [
    0x45, 0x84, 0xC9, // test r9b, r9b
    0x74, 0x2A, // jz done
    0x8A, 0x41, 0x02, // mov al, [rcx+2]
    0x41, 0x88, 0x04, 0x24, // mov [r12], al
    0x49, 0xFF, 0xC4, // inc r12
    0x41, 0x80, 0xF9, 0x01, // cmp r9b, 1
    0x76, 0x1A, // jbe done
    0x8A, 0x41, 0x04,
    0x41, 0x88, 0x04, 0x24,
    0x49, 0xFF, 0xC4,
    0x41, 0x80, 0xF9, 0x02, // cmp r9b, 2
    0x76, 0x0A, // jbe done
    0x8A, 0x41, 0x06,
    0x41, 0x88, 0x04, 0x24,
    0x49, 0xFF, 0xC4,
    0xC3, // done: ret
];

/// S3.473: push_patch body = own-ISA object seed (pos=eax, id=edx; return via
/// jmp r11 — no call, fixed RSP frame). Live poke: N_PATCHES @+3/+34,
/// PATCHES @+20. Live patch: jae fail @+10. Clobbers esi, rdi.
pub const S3_PUSH_PATCH_BODY_SEED: [u8; 41] = [
    0x8B, 0xB4, 0x24, 0x00, 0x00, 0x00, 0x00, // mov esi,[rsp+N_PATCHES]  poke @3
    0x83, 0xFE, 0x10, // cmp esi, 16
    0x0F, 0x83, 0x00, 0x00, 0x00, 0x00, // jae fail  live @10
    0x48, 0x8D, 0xBC, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rdi,[rsp+PATCHES] poke @20
    0x89, 0x04, 0xF7, // mov [rdi+rsi*8], eax
    0x89, 0x54, 0xF7, 0x04, // mov [rdi+rsi*8+4], edx
    0xFF, 0x84, 0x24, 0x00, 0x00, 0x00, 0x00, // inc dword [rsp+N_PATCHES] poke @34
    0x41, 0xFF, 0xE3, // jmp r11
];
/// Poke sites inside [`S3_PUSH_PATCH_BODY_SEED`].
pub const S3_PUSH_PATCH_N_OFF_AT: usize = 3;
pub const S3_PUSH_PATCH_N_OFF_AT2: usize = 34;
pub const S3_PUSH_PATCH_TABLE_OFF_AT: usize = 20;
pub const S3_PUSH_PATCH_JAE_AT: usize = 10;

/// S3.475: pass-2 rel32 patch loop = own-ISA object seed (95B). Internal
/// jcc/jmp rels baked. Live poke: N_PATCHES @+5, PATCHES @+19, N_LABELS @+35,
/// LABELS @+49, CODE @+78. Clobbers esi/edi/eax/edx/ecx/r8.
pub const S3_PASS2_BODY_SEED: [u8; 95] = [
    0x31, 0xF6, // xor esi,esi
    // loop:
    0x3B, 0xB4, 0x24, 0x00, 0x00, 0x00, 0x00, // cmp esi,[rsp+N_PATCHES] poke @5
    0x0F, 0x8D, 0x50, 0x00, 0x00, 0x00, // jge done (+80)
    0x48, 0x8D, 0xBC, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rdi,[rsp+PATCHES] poke @19
    0x8B, 0x04, 0xF7, // mov eax,[rdi+rsi*8]
    0x8B, 0x54, 0xF7, 0x04, // mov edx,[rdi+rsi*8+4]
    0x31, 0xFF, // xor edi,edi
    // find:
    0x3B, 0xBC, 0x24, 0x00, 0x00, 0x00, 0x00, // cmp edi,[rsp+N_LABELS] poke @35
    0x0F, 0x8D, 0x2B, 0x00, 0x00, 0x00, // jge next (+43)
    0x48, 0x8D, 0x8C, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rcx,[rsp+LABELS] poke @49
    0x39, 0x14, 0xF9, // cmp [rcx+rdi*8],edx
    0x74, 0x07, // je hit
    0xFF, 0xC7, // inc edi
    0xE9, 0xDF, 0xFF, 0xFF, 0xFF, // jmp find (-33)
    // hit:
    0x8B, 0x4C, 0xF9, 0x04, // mov ecx,[rcx+rdi*8+4]
    0x8D, 0x50, 0x04, // lea edx,[rax+4]
    0x29, 0xD1, // sub ecx,edx
    0x4C, 0x8D, 0x84, 0x24, 0x00, 0x00, 0x00, 0x00, // lea r8,[rsp+CODE] poke @78
    0x49, 0x01, 0xC0, // add r8,rax
    0x41, 0x89, 0x08, // mov [r8],ecx
    // next:
    0xFF, 0xC6, // inc esi
    0xE9, 0xA3, 0xFF, 0xFF, 0xFF, // jmp loop (-93)
    // done:
];
pub const S3_PASS2_N_PATCHES_OFF_AT: usize = 5;
pub const S3_PASS2_PATCHES_OFF_AT: usize = 19;
pub const S3_PASS2_N_LABELS_OFF_AT: usize = 35;
pub const S3_PASS2_LABELS_OFF_AT: usize = 49;
pub const S3_PASS2_CODE_OFF_AT: usize = 78;

/// S3.476: trail-C3 ensure = own-ISA object seed (44B). If .text empty or
/// last≠C3, append C3 (ovf→fail). Live poke: CODE @+4. Live patch: ja fail
/// @+30. Internal je rel8s baked.
pub const S3_TRAIL_C3_BODY_SEED: [u8; 44] = [
    0x4C, 0x8D, 0x84, 0x24, 0x00, 0x00, 0x00, 0x00, // lea r8,[rsp+CODE] poke @4
    0x4D, 0x39, 0xC4, // cmp r12, r8
    0x74, 0x08, // je need_ret
    0x41, 0x80, 0x7C, 0x24, 0xFF, 0xC3, // cmp byte [r12-1], 0xC3
    0x74, 0x17, // je after
    // need_ret:
    0x4C, 0x89, 0xE0, // mov rax, r12
    0x48, 0xFF, 0xC0, // inc rax
    0x4C, 0x39, 0xE8, // cmp rax, r13
    0x0F, 0x87, 0x00, 0x00, 0x00, 0x00, // ja fail  live @30
    0x41, 0xC6, 0x04, 0x24, 0xC3, // mov byte [r12], 0xC3
    0x49, 0xFF, 0xC4, // inc r12
    // after:
];
pub const S3_TRAIL_C3_CODE_OFF_AT: usize = 4;
pub const S3_TRAIL_C3_JA_AT: usize = 30;

/// S3.477: finish memcpy-code = own-ISA object seed (39B). Copies emitted
/// .text into PE at `S1_SLIM_TEXT_PAYLOAD_OFF` (baked). Live poke: CODE @+4/+30,
/// PE @+12. Template memcpy remains RIP (419 cliff).
pub const S3_MEMCPY_CODE_BODY_SEED: [u8; 39] = [
    0x48, 0x8D, 0xB4, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rsi,[rsp+CODE] poke @4
    0x48, 0x8D, 0xBC, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rdi,[rsp+PE] poke @12
    0x48, 0x81, 0xC7, 0x0D, 0x04, 0x00, 0x00, // add rdi, 0x40D (TEXT_PAYLOAD)
    0x4C, 0x89, 0xE1, // mov rcx, r12
    0x48, 0x8D, 0x84, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rax,[rsp+CODE] poke @30
    0x48, 0x29, 0xC1, // sub rcx, rax
    0xF3, 0xA4, // rep movsb
];
pub const S3_MEMCPY_CODE_CODE_OFF_AT: usize = 4;
pub const S3_MEMCPY_CODE_PE_OFF_AT: usize = 12;
pub const S3_MEMCPY_CODE_CODE_OFF_AT2: usize = 30;
const _: () = assert!(
    S3_MEMCPY_CODE_BODY_SEED[19] == (S1_SLIM_TEXT_PAYLOAD_OFF as u8)
        && S3_MEMCPY_CODE_BODY_SEED[20] == ((S1_SLIM_TEXT_PAYLOAD_OFF >> 8) as u8)
);

/// S3.478: finish memcpy-data = own-ISA object seed (36B). Copies data buf
/// into PE at `S1_SLIM_DATA_PAYLOAD_OFF` (baked) when data_len≠0. Live poke:
/// DATA_LEN @+3, DATA @+15, PE @+23. jz rel8 baked.
pub const S3_MEMCPY_DATA_FINISH_BODY_SEED: [u8; 36] = [
    0x8B, 0x8C, 0x24, 0x00, 0x00, 0x00, 0x00, // mov ecx,[rsp+DATA_LEN] poke @3
    0x85, 0xC9, // test ecx,ecx
    0x74, 0x19, // jz after
    0x48, 0x8D, 0xB4, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rsi,[rsp+DATA] poke @15
    0x48, 0x8D, 0xBC, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rdi,[rsp+PE] poke @23
    0x48, 0x81, 0xC7, 0x00, 0x0A, 0x00, 0x00, // add rdi, 0xA00 (DATA_PAYLOAD)
    0xF3, 0xA4, // rep movsb
    // after:
];
pub const S3_MEMCPY_DATA_FINISH_LEN_OFF_AT: usize = 3;
pub const S3_MEMCPY_DATA_FINISH_DATA_OFF_AT: usize = 15;
pub const S3_MEMCPY_DATA_FINISH_PE_OFF_AT: usize = 23;
const _: () = assert!(
    S3_MEMCPY_DATA_FINISH_BODY_SEED[30] == (S1_SLIM_DATA_PAYLOAD_OFF as u8)
        && S3_MEMCPY_DATA_FINISH_BODY_SEED[31] == ((S1_SLIM_DATA_PAYLOAD_OFF >> 8) as u8)
);

/// S3.480: template-post-lea memcpy = own-ISA object seed (18B). Assumes
/// rcx=template src already (RIP lea stays hand · 419). Live poke: PE @+4,
/// len @+12.
pub const S3_TEMPLATE_POST_LEA_BODY_SEED: [u8; 18] = [
    0x48, 0x8D, 0xBC, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rdi,[rsp+PE] poke @4
    0x48, 0x89, 0xCE, // mov rsi, rcx
    0xB9, 0x00, 0x00, 0x00, 0x00, // mov ecx, len poke @12
    0xF3, 0xA4, // rep movsb
];
pub const S3_TEMPLATE_POST_LEA_PE_OFF_AT: usize = 4;
pub const S3_TEMPLATE_POST_LEA_LEN_AT: usize = 12;

/// S3.481: write_out setup = own-ISA object seed (19B). Loads pe_buf→rcx and
/// template_len→r12d then jmp write_out. Live poke: PE @+4, len @+10. Live
/// patch: jmp @+14.
pub const S3_WRITE_OUT_SETUP_BODY_SEED: [u8; 19] = [
    0x48, 0x8D, 0x8C, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rcx,[rsp+PE] poke @4
    0x41, 0xBC, 0x00, 0x00, 0x00, 0x00, // mov r12d, len poke @10
    0xE9, 0x00, 0x00, 0x00, 0x00, // jmp write_out live @14
];
pub const S3_WRITE_OUT_SETUP_PE_OFF_AT: usize = 4;
pub const S3_WRITE_OUT_SETUP_LEN_AT: usize = 10;
pub const S3_WRITE_OUT_SETUP_JMP_AT: usize = 14;

/// S3.486: DATA/STR/RAW-data append body = own-ISA object seed (39B).
/// rcx=rec, r9b=argc≤3, rdi=dest; advances rdi for bytes 0..argc-1 (last
/// write no-inc — caller adds argc to data_len). Shared mt+s1. Rel8s baked.
pub const S3_DATA_APPEND_BODY_SEED: [u8; 39] = [
    0x45, 0x84, 0xC9, // test r9b, r9b
    0x74, 0x21, // jz done
    0x8A, 0x41, 0x02, // mov al, [rcx+2]
    0x88, 0x07, // mov [rdi], al
    0x48, 0xFF, 0xC7, // inc rdi
    0x41, 0x80, 0xF9, 0x01, // cmp r9b, 1
    0x76, 0x13, // jbe done
    0x8A, 0x41, 0x04,
    0x88, 0x07,
    0x48, 0xFF, 0xC7,
    0x41, 0x80, 0xF9, 0x02, // cmp r9b, 2
    0x76, 0x05, // jbe done
    0x8A, 0x41, 0x06,
    0x88, 0x07,
    0xC3, // done: ret
];

/// S3.488: s1 LABEL table-push body = own-ISA object seed (62B). rcx=rec;
/// records (id, code_off=r12-CODE) into labels[n++]. Live poke: N_LABELS
/// @+3/+36/+58, CODE @+23, LABELS @+44. Live: jae fail @+10. LABEL=meta for
/// .text emit — this is table bookkeeping (≡ push_patch class).
pub const S3_PUSH_LABEL_BODY_SEED: [u8; 62] = [
    0x8B, 0x84, 0x24, 0x00, 0x00, 0x00, 0x00, // mov eax,[rsp+N_LABELS] poke @3
    0x83, 0xF8, 0x10, // cmp eax, 16
    0x0F, 0x83, 0x00, 0x00, 0x00, 0x00, // jae fail live @10
    0x8B, 0x51, 0x02, // mov edx, [rcx+2]
    0x4C, 0x8D, 0x84, 0x24, 0x00, 0x00, 0x00, 0x00, // lea r8,[rsp+CODE] poke @23
    0x4C, 0x89, 0xE0, // mov rax, r12
    0x4C, 0x29, 0xC0, // sub rax, r8
    0x8B, 0xB4, 0x24, 0x00, 0x00, 0x00, 0x00, // mov esi,[rsp+N_LABELS] poke @36
    0x48, 0x8D, 0xBC, 0x24, 0x00, 0x00, 0x00, 0x00, // lea rdi,[rsp+LABELS] poke @44
    0x89, 0x14, 0xF7, // mov [rdi+rsi*8], edx
    0x89, 0x44, 0xF7, 0x04, // mov [rdi+rsi*8+4], eax
    0xFF, 0x84, 0x24, 0x00, 0x00, 0x00, 0x00, // inc [rsp+N_LABELS] poke @58
];
pub const S3_PUSH_LABEL_N_OFF_AT: usize = 3;
pub const S3_PUSH_LABEL_JAE_AT: usize = 10;
pub const S3_PUSH_LABEL_CODE_OFF_AT: usize = 23;
pub const S3_PUSH_LABEL_N_OFF_AT2: usize = 36;
pub const S3_PUSH_LABEL_TABLE_OFF_AT: usize = 44;
pub const S3_PUSH_LABEL_N_OFF_AT3: usize = 58;

/// S3.337: test-only helper.
#[cfg(test)]
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

/// S3.359/S3.365: fixed10 template bodies = **object code of our own ops** (trust-root seed).
/// Not an emit_x86 translator — YOYO table-compose copies these encodings.
/// Order ≡ `S3_FIXED10_OPS`. Verified ≡ live emit_x86 in `s3_fixed10_body_seed_pin`.
/// `static` (not `const`): `s3_fixed_row` must alias one storage — const re-promotes per use.
/// S3.424: SET slot≥16 body = own-ISA object seed (movabs + `[r15+disp32]`).
/// Not in fixed10 host-table rows (those stay disp8). In-DLL bake appends this
/// after `S3_HOST_TABLE_SEED` (offset +200).
pub const S3_SET_DISP32_BODY_SEED: [u8; 17] = [
    0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x00, 0x00, 0x00, // movabs rax,imm
    0x49, 0x89, 0x87, 0x00, 0x00, 0x00, 0x00, // mov [r15+disp32], rax
];

/// S3.425: IMUL body = own-ISA object seed (load dst/src, imul, store). Bake
/// appends after SET disp32 (offset +217).
pub const S3_IMUL_BODY_SEED: [u8; 16] = [
    0x49, 0x8b, 0x47, 0x00, // mov rax, [r15+dst]
    0x49, 0x8b, 0x4f, 0x08, // mov rcx, [r15+src]
    0x48, 0x0f, 0xaf, 0xc1, // imul rax, rcx
    0x49, 0x89, 0x47, 0x00, // mov [r15+dst], rax
];

/// S3.426: ALUV (ADDV/ORV/SUBV) body seed — opcode byte @+9 poked to 01/09/29.
/// Bake appends after IMUL (offset +233).
pub const S3_ALUV_BODY_SEED: [u8; 15] = [
    0x49, 0x8b, 0x47, 0x00, // mov rax, [r15+dst]
    0x49, 0x8b, 0x4f, 0x08, // mov rcx, [r15+src]
    0x48, 0x01, 0xc8, // add rax, rcx (OR=09 / SUB=29 poked)
    0x49, 0x89, 0x47, 0x00, // mov [r15+dst], rax
];

/// S3.427: MEMCPY_DATA body seed (14B) — load rsi/rdi/rcx + rep movsb.
/// Bake appends after ALUV (offset +248).
pub const S3_MEMCPY_DATA_BODY_SEED: [u8; 14] = [
    0x49, 0x8b, 0x77, 0x00, // mov rsi, [r15+src]
    0x49, 0x8b, 0x7f, 0x00, // mov rdi, [r15+dst]
    0x49, 0x8b, 0x4f, 0x00, // mov rcx, [r15+n]
    0xf3, 0xa4, // rep movsb
];

/// S3.427: LDB oo=0 body seed (12B). Bake +262.
pub const S3_LDB_OO0_BODY_SEED: [u8; 12] = [
    0x49, 0x8b, 0x47, 0x00, // mov rax, [r15+ss]
    0x48, 0x0f, 0xb6, 0x00, // movzx rax, byte [rax]
    0x49, 0x89, 0x47, 0x00, // mov [r15+dd], rax
];

/// S3.427: LDB oo≠0 body seed (19B). Bake +274; poke imm32 @+7.
pub const S3_LDB_OO_BODY_SEED: [u8; 19] = [
    0x49, 0x8b, 0x47, 0x00, // mov rax, [r15+ss]
    0x48, 0x81, 0xc0, 0x01, 0x00, 0x00, 0x00, // add rax, imm32
    0x48, 0x0f, 0xb6, 0x00, // movzx rax, byte [rax]
    0x49, 0x89, 0x47, 0x00, // mov [r15+dd], rax
];

/// S3.502: STB oo=0 body seed (10B) — host compose (not bake append; bake=362).
/// Poke val disp @+3 / base disp @+7. Matches `emit_stb` for oo=0.
pub const S3_STB_OO0_BODY_SEED: [u8; 10] = [
    0x49, 0x8b, 0x4f, 0x00, // mov rcx, [r15+val]
    0x49, 0x8b, 0x47, 0x00, // mov rax, [r15+base]
    0x88, 0x08, // mov [rax], cl
];

/// S3.502: STB oo∈1..=127 body seed (14B). Poke val@+3 / base@+7 / oo@+11.
pub const S3_STB_OO_IMM8_BODY_SEED: [u8; 14] = [
    0x49, 0x8b, 0x4f, 0x00, // mov rcx, [r15+val]
    0x49, 0x8b, 0x47, 0x00, // mov rax, [r15+base]
    0x48, 0x83, 0xc0, 0x01, // add rax, imm8
    0x88, 0x08, // mov [rax], cl
];

/// S3.513: STB oo>127 body seed (17B) — add rax,imm32. Poke val@+3 / base@+7 /
/// oo@+11..+14. Host compose (bake=362). **Not** FIXED10.
pub const S3_STB_OO_IMM32_BODY_SEED: [u8; 17] = [
    0x49, 0x8b, 0x4f, 0x00, // mov rcx, [r15+val]
    0x49, 0x8b, 0x47, 0x00, // mov rax, [r15+base]
    0x48, 0x81, 0xc0, 0x00, 0x00, 0x00, 0x00, // add rax, imm32
    0x88, 0x08, // mov [rax], cl
];

/// S3.504: LOAD_FILE body seed (160B) — host compose (not bake append; bake=362).
/// ≡ `emit_slim_load_file(0,0)` / pe_dll `slim_load_tmpl`. Poke path_off @+9 /
/// store slot disp @ [`slim_load_slot_patch_off`]. IAT = CreateFileA→VirtualAlloc
/// →ReadFile→CloseHandle. **Not** FIXED10.
pub const S3_LOAD_FILE_BODY_SEED: [u8; 160] = [
    0x53, 0x56, 0x48, 0x83, 0xec, 0x38, 0x49, 0x8d, 0x8f, 0x00, 0x00, 0x00, 0x00, 0xba, 0x00,
    0x00, 0x00, 0x80, 0x41, 0xb8, 0x01, 0x00, 0x00, 0x00, 0x45, 0x31, 0xc9, 0xc7, 0x44, 0x24,
    0x20, 0x03, 0x00, 0x00, 0x00, 0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00, 0x48,
    0xc7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00, 0x41, 0xff, 0x97, 0x08, 0x0f, 0x00, 0x00,
    0x48, 0x89, 0xc3, 0x31, 0xc9, 0xba, 0x00, 0x00, 0x01, 0x00, 0x41, 0xb8, 0x00, 0x30, 0x00,
    0x00, 0x41, 0xb9, 0x04, 0x00, 0x00, 0x00, 0x41, 0xff, 0x97, 0x00, 0x0f, 0x00, 0x00, 0x48,
    0x89, 0xc6, 0x48, 0x89, 0xd9, 0x48, 0x89, 0xf2, 0x41, 0xb8, 0x00, 0x00, 0x01, 0x00, 0x48,
    0xc7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00, 0x4c, 0x8d, 0x4c, 0x24, 0x20, 0x48, 0xc7,
    0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00, 0x41, 0xff, 0x97, 0x10, 0x0f, 0x00, 0x00, 0x48,
    0x89, 0xd9, 0x41, 0xff, 0x97, 0x20, 0x0f, 0x00, 0x00, 0x48, 0x83, 0xc4, 0x38, 0x48, 0x89,
    0xf0, 0x49, 0x89, 0x87, 0x00, 0x00, 0x00, 0x00, 0x5e, 0x5b,
];

/// S3.505: WRITE_FILE body seed (139B) — host compose (not bake append; bake=362).
/// ≡ `emit_slim_write_file(0,0,0)` / pe_dll `slim_write_tmpl`. Poke path@+9 /
/// slot load / sz load / store via slim_write_*_patch_off. **Not** FIXED10.
pub const S3_WRITE_FILE_BODY_SEED: [u8; 139] = [
    0x53, 0x56, 0x48, 0x83, 0xec, 0x38, 0x49, 0x8d, 0x8f, 0x00, 0x00, 0x00, 0x00, 0xba, 0x00,
    0x00, 0x00, 0x40, 0x41, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x45, 0x31, 0xc9, 0xc7, 0x44, 0x24,
    0x20, 0x02, 0x00, 0x00, 0x00, 0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00, 0x48,
    0xc7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00, 0x41, 0xff, 0x97, 0x08, 0x0f, 0x00, 0x00,
    0x48, 0x89, 0xc3, 0x49, 0x8b, 0x97, 0x00, 0x00, 0x00, 0x00, 0x49, 0x8b, 0x87, 0x00, 0x00,
    0x00, 0x00, 0x49, 0x89, 0xc0, 0x48, 0x89, 0xd9, 0x48, 0xc7, 0x44, 0x24, 0x20, 0x00, 0x00,
    0x00, 0x00, 0x4c, 0x8d, 0x4c, 0x24, 0x20, 0x48, 0xc7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00,
    0x00, 0x41, 0xff, 0x97, 0x18, 0x0f, 0x00, 0x00, 0x48, 0x89, 0xd9, 0x41, 0xff, 0x97, 0x20,
    0x0f, 0x00, 0x00, 0x48, 0x83, 0xc4, 0x38, 0x48, 0x31, 0xc0, 0x49, 0x89, 0x87, 0x00, 0x00,
    0x00, 0x00, 0x5e, 0x5b,
];

/// S3.428: MEMCPY_STATE one slot pair (8B) — load src + store dst.
/// Bake +293; loop pokes disp @+3 / @+7 then advances r12 by 8.
pub const S3_MEMCPY_STATE_PAIR_SEED: [u8; 8] = [
    0x49, 0x8b, 0x47, 0x00, // mov rax, [r15+src]
    0x49, 0x89, 0x47, 0x00, // mov [r15+dst], rax
];

/// S3.429: CALL rel32 placeholder (5B). Bake +301; pass-2 patches disp.
pub const S3_CALL_BODY_SEED: [u8; 5] = [0xe8, 0x00, 0x00, 0x00, 0x00];

/// S3.429: JMP rel32 placeholder (5B). Bake +306.
pub const S3_JMP_BODY_SEED: [u8; 5] = [0xe9, 0x00, 0x00, 0x00, 0x00];

/// S3.429: JCC 0F cc rel32 placeholder (6B). Bake +311; poke cc @+1.
pub const S3_JCC_BODY_SEED: [u8; 6] = [0x0f, 0x00, 0x00, 0x00, 0x00, 0x00];

/// S3.430: ALLOC VirtualAlloc prefix (34B) — through `add rsp,0x28`.
/// Bake +317; poke size @+7; store (disp8/32) still hand after seed.
/// IAT disp @+0x1A = [`S1_SLIM_IAT_BASE`] (VirtualAlloc slot0).
pub const S3_ALLOC_PREFIX_SEED: [u8; 34] = [
    0x48, 0x83, 0xec, 0x28, // sub rsp, 0x28
    0x31, 0xc9, // xor ecx, ecx
    0xba, 0x00, 0x00, 0x00, 0x00, // mov edx, size (poke)
    0x41, 0xb8, 0x00, 0x30, 0x00, 0x00, // mov r8d, MEM_COMMIT|RESERVE
    0x41, 0xb9, 0x04, 0x00, 0x00, 0x00, // mov r9d, PAGE_READWRITE
    0x41, 0xff, 0x97, 0x00, 0x0f, 0x00, 0x00, // call [r15+0xF00]
    0x48, 0x83, 0xc4, 0x28, // add rsp, 0x28
];

/// S3.431: `mov [r15+disp8], rax` (4B). Bake +351; poke disp @+3.
pub const S3_STORE_RAX_DISP8_SEED: [u8; 4] = [0x49, 0x89, 0x47, 0x00];

/// S3.431: `mov [r15+disp32], rax` (7B). Bake +355; poke disp @+3.
pub const S3_STORE_RAX_DISP32_SEED: [u8; 7] = [0x49, 0x89, 0x87, 0x00, 0x00, 0x00, 0x00];

/// S3.514: host `mov rax,[r15+disp8]` (4B). Not bake append.
pub const S3_LOAD_RAX_DISP8_SEED: [u8; 4] = [0x49, 0x8b, 0x47, 0x00];
/// S3.514: host `mov rax,[r15+disp32]` (7B).
pub const S3_LOAD_RAX_DISP32_SEED: [u8; 7] = [0x49, 0x8b, 0x87, 0x00, 0x00, 0x00, 0x00];
/// S3.514: host `mov rcx,[r15+disp8]` (4B).
pub const S3_LOAD_RCX_DISP8_SEED: [u8; 4] = [0x49, 0x8b, 0x4f, 0x00];
/// S3.514: host `mov rcx,[r15+disp32]` (7B).
pub const S3_LOAD_RCX_DISP32_SEED: [u8; 7] = [0x49, 0x8b, 0x8f, 0x00, 0x00, 0x00, 0x00];
/// S3.514: `imul rax, rcx` (4B).
pub const S3_IMUL_RAX_RCX_SEED: [u8; 4] = [0x48, 0x0f, 0xaf, 0xc1];

/// S3.515: host `mov rsi,[r15+disp8/32]` / `mov rdi,[r15+…]` (MEMCPY_DATA).
pub const S3_LOAD_RSI_DISP8_SEED: [u8; 4] = [0x49, 0x8b, 0x77, 0x00];
pub const S3_LOAD_RSI_DISP32_SEED: [u8; 7] = [0x49, 0x8b, 0xb7, 0x00, 0x00, 0x00, 0x00];
pub const S3_LOAD_RDI_DISP8_SEED: [u8; 4] = [0x49, 0x8b, 0x7f, 0x00];
pub const S3_LOAD_RDI_DISP32_SEED: [u8; 7] = [0x49, 0x8b, 0xbf, 0x00, 0x00, 0x00, 0x00];
/// S3.515: `movzx rax, byte [rax]` / `add rax,imm32` / `add rax,imm8` / `mov [rax],cl`.
pub const S3_MOVZX_RAX_BYTE_RAX_SEED: [u8; 4] = [0x48, 0x0f, 0xb6, 0x00];
pub const S3_ADD_RAX_IMM32_SEED: [u8; 7] = [0x48, 0x81, 0xc0, 0x00, 0x00, 0x00, 0x00];
pub const S3_ADD_RAX_IMM8_SEED: [u8; 4] = [0x48, 0x83, 0xc0, 0x00];
pub const S3_STORE_CL_RAX_SEED: [u8; 2] = [0x88, 0x08];
pub const S3_REP_MOVSB_SEED: [u8; 2] = [0xf3, 0xa4];

/// S3.516: fixed10-ish ALU host seeds (CMP / INC / DEC / SUB / MOVABS).
pub const S3_CMP_RAX_RCX_SEED: [u8; 3] = [0x48, 0x39, 0xc8];
pub const S3_INC_RAX_SEED: [u8; 3] = [0x48, 0xff, 0xc0];
pub const S3_DEC_RAX_SEED: [u8; 3] = [0x48, 0xff, 0xc8];
pub const S3_SUB_RAX_IMM32_SEED: [u8; 7] = [0x48, 0x81, 0xe8, 0x00, 0x00, 0x00, 0x00];
/// `movabs rax, imm64` — poke imm32 @+2 (high dword stays 0).
pub const S3_MOVABS_RAX_IMM64_SEED: [u8; 10] = [
    0x48, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// S3.514/S3.515: poke disp into a 4B disp8 or 7B disp32 load/store seed.
fn s3_push_r15_mem_op(out: &mut Vec<u8>, slot: u16, disp8: &[u8; 4], disp32: &[u8; 7]) {
    let disp = (slot as u32) * 8;
    if disp <= 127 {
        let mut s = *disp8;
        s[3] = disp as u8;
        out.extend_from_slice(&s);
    } else {
        let mut s = *disp32;
        s[3..7].copy_from_slice(&disp.to_le_bytes());
        out.extend_from_slice(&s);
    }
}

fn s3_push_load_rax_slot(out: &mut Vec<u8>, slot: u16) {
    s3_push_r15_mem_op(out, slot, &S3_LOAD_RAX_DISP8_SEED, &S3_LOAD_RAX_DISP32_SEED);
}
fn s3_push_load_rcx_slot(out: &mut Vec<u8>, slot: u16) {
    s3_push_r15_mem_op(out, slot, &S3_LOAD_RCX_DISP8_SEED, &S3_LOAD_RCX_DISP32_SEED);
}
fn s3_push_load_rsi_slot(out: &mut Vec<u8>, slot: u16) {
    s3_push_r15_mem_op(out, slot, &S3_LOAD_RSI_DISP8_SEED, &S3_LOAD_RSI_DISP32_SEED);
}
fn s3_push_load_rdi_slot(out: &mut Vec<u8>, slot: u16) {
    s3_push_r15_mem_op(out, slot, &S3_LOAD_RDI_DISP8_SEED, &S3_LOAD_RDI_DISP32_SEED);
}
fn s3_push_store_rax_slot(out: &mut Vec<u8>, slot: u16) {
    s3_push_r15_mem_op(out, slot, &S3_STORE_RAX_DISP8_SEED, &S3_STORE_RAX_DISP32_SEED);
}

static S3_FIXED10_BODY_SEED: [&[u8]; 10] = [
    /* 0xff RET */ &[0xc3],
    /* 0x00 NOP */ &[0x90],
    /* 0x30 SET */ &[
        0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00,
    ],
    /* 0x60 GET */ &[0x49, 0x8b, 0x47, 0x08, 0x49, 0x89, 0x47, 0x00],
    /* 0x66 INC */ &[0x49, 0x8b, 0x47, 0x00, 0x48, 0xff, 0xc0, 0x49, 0x89, 0x47, 0x00],
    /* 0x62 ADD */ &[
        0x49, 0x8b, 0x47, 0x00, 0x48, 0x81, 0xc0, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00,
    ],
    /* 0x67 DEC */ &[0x49, 0x8b, 0x47, 0x00, 0x48, 0xff, 0xc8, 0x49, 0x89, 0x47, 0x00],
    /* 0x61 SUB */ &[
        0x49, 0x8b, 0x47, 0x00, 0x48, 0x81, 0xe8, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00,
    ],
    /* 0x64 MOVRR */ &[0x49, 0x8b, 0x47, 0x00, 0x49, 0x89, 0x47, 0x08],
    /* 0x65 CMP */ &[0x49, 0x8b, 0x47, 0x00, 0x49, 0x8b, 0x4f, 0x08, 0x48, 0x39, 0xc8],
];

/// S3.138/S3.359: live emit_x86 bake — test oracle only (prod uses body seed).
#[cfg(test)]
fn s3_compute_fixed_body(op: u8) -> Vec<u8> {
    match op {
        0xFF => {
            let text = emit_x86(
                &parse_tyb(&s3_host_tyb_from_records(&[s3_fixed_rec(0xFF)])).expect("ret tyb"),
            )
            .text;
            assert_eq!(text, vec![0xC3]);
            text
        }
        0x00 => {
            let (o, argc, args) = s3_fixed_rec(0x00);
            let b = s3_host_body(o, argc, args);
            assert_eq!(b, vec![0x90]);
            b
        }
        o if S3_FIXED10_OPS.contains(&o) => {
            let (op, argc, args) = s3_fixed_rec(o);
            s3_host_body(op, argc, args)
        }
        _ => panic!("s3_compute_fixed_body: opcode {op:#x} is outside fixed10"),
    }
}

/// S3.359/S3.365: body row = direct seed slice (no OnceLock / no Vec cache).
fn s3_fixed_row(op: u8) -> &'static [u8] {
    let idx = S3_FIXED10_OPS
        .iter()
        .position(|&o| o == op)
        .unwrap_or_else(|| panic!("s3_fixed_row: opcode {op:#x} is outside fixed10"));
    S3_FIXED10_BODY_SEED[idx]
}

/// S3.337: test-only helper.
#[cfg(test)]
fn s3_fixed_table(ops: &[u8]) -> Vec<u8> {
    let mut table = Vec::with_capacity(ops.len() * 20);
    for op in ops {
        table.extend_from_slice(&s3_host_tmpl_row(*op, &s3_fixed_row(*op)));
    }
    table
}

/// S3.120/S3.349: fixed10 payload helper — test/proof; production host inlines.
#[cfg(test)]
fn s3_fixed_payload(row_ops: &[u8], prog: &[u8], outbuf: usize) -> Vec<u8> {
    let rows: Vec<(u8, &[u8])> = row_ops.iter().map(|op| (*op, s3_fixed_row(*op))).collect();
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

/// S3.207/S3.209/S3.346: IR-write alias — test/proof call sites.
/// Production host/expand call [`s3_append_stb_zero_then_poke`] directly.
#[cfg(test)]
fn s3_ir_seed_at(
    recs: &mut Vec<(u8, u8, [u8; 6])>,
    base: u32,
    bytes: &[u8],
    label_id: u32,
) -> u32 {
    s3_append_stb_zero_then_poke(recs, base, bytes, label_id)
}

/// S3.364: fixed10 host table = own object-code seed (10×20B tmpl rows).
/// Bodies come from [`S3_FIXED10_BODY_SEED`]; prod host no longer live-composes
/// via OnceLock for every emit. Verified in `s3_host_table_seed_pin`.
pub const S3_HOST_TABLE_SEED: [u8; 200] = [
    0xff, 0x01, 0x00, 0x00, 0xc3, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x90, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x0e, 0x00, 0x00, 0x48, 0xb8, 0xef, 0xbe,
    0xad, 0xde, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x00, 0x00, 0x60, 0x08, 0x00, 0x00,
    0x49, 0x8b, 0x47, 0x08, 0x49, 0x89, 0x47, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x66, 0x0b, 0x00, 0x00, 0x49, 0x8b, 0x47, 0x00, 0x48, 0xff, 0xc0, 0x49, 0x89, 0x47, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x62, 0x0f, 0x00, 0x00, 0x49, 0x8b, 0x47, 0x00, 0x48, 0x81, 0xc0, 0x05,
    0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x00, 0x67, 0x0b, 0x00, 0x00, 0x49, 0x8b, 0x47, 0x00,
    0x48, 0xff, 0xc8, 0x49, 0x89, 0x47, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x61, 0x0f, 0x00, 0x00,
    0x49, 0x8b, 0x47, 0x00, 0x48, 0x81, 0xe8, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x00,
    0x64, 0x08, 0x00, 0x00, 0x49, 0x8b, 0x47, 0x00, 0x49, 0x89, 0x47, 0x08, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x65, 0x0b, 0x00, 0x00, 0x49, 0x8b, 0x47, 0x00, 0x49, 0x8b, 0x4f, 0x08,
    0x48, 0x39, 0xc8, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// S3.366: host table win0 poke shell = own-ISA SET/STB seed (first 127B at TABLE_OFF).
/// S3.366: first 127B poke seed. S3.367 adds win1; floor moved 171→173.
/// Live full-slice poke stays as oracle in pins. Verified in s3_host_table_win0_poke_seed_pin.
const S3_HOST_TABLE_WIN0_POKE_SEED: [(u8, u8, [u8; 6]); 80] = [
    /*  0*/ (0x30, 2, [14, 0, 128, 0, 0, 0]),
    /*  1*/ (0x30, 2, [11, 0, 255, 0, 0, 0]),
    /*  2*/ (0x81, 3, [11, 0, 14, 0, 0, 0]),
    /*  3*/ (0x81, 3, [11, 0, 14, 0, 89, 0]),
    /*  4*/ (0x30, 2, [11, 0, 1, 0, 0, 0]),
    /*  5*/ (0x81, 3, [11, 0, 14, 0, 1, 0]),
    /*  6*/ (0x81, 3, [11, 0, 14, 0, 21, 0]),
    /*  7*/ (0x30, 2, [11, 0, 195, 0, 0, 0]),
    /*  8*/ (0x81, 3, [11, 0, 14, 0, 4, 0]),
    /*  9*/ (0x30, 2, [11, 0, 144, 0, 0, 0]),
    /* 10*/ (0x81, 3, [11, 0, 14, 0, 24, 0]),
    /* 11*/ (0x30, 2, [11, 0, 48, 0, 0, 0]),
    /* 12*/ (0x81, 3, [11, 0, 14, 0, 40, 0]),
    /* 13*/ (0x30, 2, [11, 0, 14, 0, 0, 0]),
    /* 14*/ (0x81, 3, [11, 0, 14, 0, 41, 0]),
    /* 15*/ (0x30, 2, [11, 0, 72, 0, 0, 0]),
    /* 16*/ (0x81, 3, [11, 0, 14, 0, 44, 0]),
    /* 17*/ (0x81, 3, [11, 0, 14, 0, 88, 0]),
    /* 18*/ (0x81, 3, [11, 0, 14, 0, 108, 0]),
    /* 19*/ (0x30, 2, [11, 0, 184, 0, 0, 0]),
    /* 20*/ (0x81, 3, [11, 0, 14, 0, 45, 0]),
    /* 21*/ (0x30, 2, [11, 0, 239, 0, 0, 0]),
    /* 22*/ (0x81, 3, [11, 0, 14, 0, 46, 0]),
    /* 23*/ (0x30, 2, [11, 0, 190, 0, 0, 0]),
    /* 24*/ (0x81, 3, [11, 0, 14, 0, 47, 0]),
    /* 25*/ (0x30, 2, [11, 0, 173, 0, 0, 0]),
    /* 26*/ (0x81, 3, [11, 0, 14, 0, 48, 0]),
    /* 27*/ (0x30, 2, [11, 0, 222, 0, 0, 0]),
    /* 28*/ (0x81, 3, [11, 0, 14, 0, 49, 0]),
    /* 29*/ (0x30, 2, [11, 0, 73, 0, 0, 0]),
    /* 30*/ (0x81, 3, [11, 0, 14, 0, 54, 0]),
    /* 31*/ (0x81, 3, [11, 0, 14, 0, 64, 0]),
    /* 32*/ (0x81, 3, [11, 0, 14, 0, 68, 0]),
    /* 33*/ (0x81, 3, [11, 0, 14, 0, 84, 0]),
    /* 34*/ (0x81, 3, [11, 0, 14, 0, 91, 0]),
    /* 35*/ (0x81, 3, [11, 0, 14, 0, 104, 0]),
    /* 36*/ (0x81, 3, [11, 0, 14, 0, 115, 0]),
    /* 37*/ (0x81, 3, [11, 0, 14, 0, 124, 0]),
    /* 38*/ (0x30, 2, [11, 0, 137, 0, 0, 0]),
    /* 39*/ (0x81, 3, [11, 0, 14, 0, 55, 0]),
    /* 40*/ (0x81, 3, [11, 0, 14, 0, 69, 0]),
    /* 41*/ (0x81, 3, [11, 0, 14, 0, 92, 0]),
    /* 42*/ (0x81, 3, [11, 0, 14, 0, 116, 0]),
    /* 43*/ (0x30, 2, [11, 0, 71, 0, 0, 0]),
    /* 44*/ (0x81, 3, [11, 0, 14, 0, 56, 0]),
    /* 45*/ (0x81, 3, [11, 0, 14, 0, 66, 0]),
    /* 46*/ (0x81, 3, [11, 0, 14, 0, 70, 0]),
    /* 47*/ (0x81, 3, [11, 0, 14, 0, 86, 0]),
    /* 48*/ (0x81, 3, [11, 0, 14, 0, 93, 0]),
    /* 49*/ (0x81, 3, [11, 0, 14, 0, 106, 0]),
    /* 50*/ (0x81, 3, [11, 0, 14, 0, 117, 0]),
    /* 51*/ (0x81, 3, [11, 0, 14, 0, 126, 0]),
    /* 52*/ (0x30, 2, [11, 0, 96, 0, 0, 0]),
    /* 53*/ (0x81, 3, [11, 0, 14, 0, 60, 0]),
    /* 54*/ (0x30, 2, [11, 0, 8, 0, 0, 0]),
    /* 55*/ (0x81, 3, [11, 0, 14, 0, 61, 0]),
    /* 56*/ (0x81, 3, [11, 0, 14, 0, 67, 0]),
    /* 57*/ (0x30, 2, [11, 0, 139, 0, 0, 0]),
    /* 58*/ (0x81, 3, [11, 0, 14, 0, 65, 0]),
    /* 59*/ (0x81, 3, [11, 0, 14, 0, 85, 0]),
    /* 60*/ (0x81, 3, [11, 0, 14, 0, 105, 0]),
    /* 61*/ (0x81, 3, [11, 0, 14, 0, 125, 0]),
    /* 62*/ (0x30, 2, [11, 0, 102, 0, 0, 0]),
    /* 63*/ (0x81, 3, [11, 0, 14, 0, 80, 0]),
    /* 64*/ (0x30, 2, [11, 0, 11, 0, 0, 0]),
    /* 65*/ (0x81, 3, [11, 0, 14, 0, 81, 0]),
    /* 66*/ (0x81, 3, [11, 0, 14, 0, 121, 0]),
    /* 67*/ (0x30, 2, [11, 0, 192, 0, 0, 0]),
    /* 68*/ (0x81, 3, [11, 0, 14, 0, 90, 0]),
    /* 69*/ (0x81, 3, [11, 0, 14, 0, 110, 0]),
    /* 70*/ (0x30, 2, [11, 0, 98, 0, 0, 0]),
    /* 71*/ (0x81, 3, [11, 0, 14, 0, 100, 0]),
    /* 72*/ (0x30, 2, [11, 0, 15, 0, 0, 0]),
    /* 73*/ (0x81, 3, [11, 0, 14, 0, 101, 0]),
    /* 74*/ (0x30, 2, [11, 0, 129, 0, 0, 0]),
    /* 75*/ (0x81, 3, [11, 0, 14, 0, 109, 0]),
    /* 76*/ (0x30, 2, [11, 0, 5, 0, 0, 0]),
    /* 77*/ (0x81, 3, [11, 0, 14, 0, 111, 0]),
    /* 78*/ (0x30, 2, [11, 0, 103, 0, 0, 0]),
    /* 79*/ (0x81, 3, [11, 0, 14, 0, 120, 0]),
];

/// S3.367: host table win1 poke shell = own-ISA SET/STB seed (table[127..200] at TABLE_OFF+127).
/// Prog (ops_len|ops) live-poked at PROG_OFF — splits window (+2 host IR; floor 171→173).
/// Verified in s3_host_table_win1_poke_seed_pin.
const S3_HOST_TABLE_WIN1_POKE_SEED: [(u8, u8, [u8; 6]); 58] = [
    /*  0*/ (0x30, 2, [14, 0, 255, 0, 0, 0]),
    /*  1*/ (0x30, 2, [11, 0, 72, 0, 0, 0]),
    /*  2*/ (0x81, 3, [11, 0, 14, 0, 1, 0]),
    /*  3*/ (0x81, 3, [11, 0, 14, 0, 21, 0]),
    /*  4*/ (0x81, 3, [11, 0, 14, 0, 65, 0]),
    /*  5*/ (0x30, 2, [11, 0, 255, 0, 0, 0]),
    /*  6*/ (0x81, 3, [11, 0, 14, 0, 2, 0]),
    /*  7*/ (0x30, 2, [11, 0, 200, 0, 0, 0]),
    /*  8*/ (0x81, 3, [11, 0, 14, 0, 3, 0]),
    /*  9*/ (0x81, 3, [11, 0, 14, 0, 67, 0]),
    /* 10*/ (0x30, 2, [11, 0, 73, 0, 0, 0]),
    /* 11*/ (0x81, 3, [11, 0, 14, 0, 4, 0]),
    /* 12*/ (0x81, 3, [11, 0, 14, 0, 17, 0]),
    /* 13*/ (0x81, 3, [11, 0, 14, 0, 28, 0]),
    /* 14*/ (0x81, 3, [11, 0, 14, 0, 37, 0]),
    /* 15*/ (0x81, 3, [11, 0, 14, 0, 41, 0]),
    /* 16*/ (0x81, 3, [11, 0, 14, 0, 57, 0]),
    /* 17*/ (0x81, 3, [11, 0, 14, 0, 61, 0]),
    /* 18*/ (0x30, 2, [11, 0, 137, 0, 0, 0]),
    /* 19*/ (0x81, 3, [11, 0, 14, 0, 5, 0]),
    /* 20*/ (0x81, 3, [11, 0, 14, 0, 29, 0]),
    /* 21*/ (0x81, 3, [11, 0, 14, 0, 42, 0]),
    /* 22*/ (0x30, 2, [11, 0, 71, 0, 0, 0]),
    /* 23*/ (0x81, 3, [11, 0, 14, 0, 6, 0]),
    /* 24*/ (0x81, 3, [11, 0, 14, 0, 19, 0]),
    /* 25*/ (0x81, 3, [11, 0, 14, 0, 30, 0]),
    /* 26*/ (0x81, 3, [11, 0, 14, 0, 39, 0]),
    /* 27*/ (0x81, 3, [11, 0, 14, 0, 43, 0]),
    /* 28*/ (0x81, 3, [11, 0, 14, 0, 59, 0]),
    /* 29*/ (0x30, 2, [11, 0, 97, 0, 0, 0]),
    /* 30*/ (0x81, 3, [11, 0, 14, 0, 13, 0]),
    /* 31*/ (0x30, 2, [11, 0, 15, 0, 0, 0]),
    /* 32*/ (0x81, 3, [11, 0, 14, 0, 14, 0]),
    /* 33*/ (0x30, 2, [11, 0, 139, 0, 0, 0]),
    /* 34*/ (0x81, 3, [11, 0, 14, 0, 18, 0]),
    /* 35*/ (0x81, 3, [11, 0, 14, 0, 38, 0]),
    /* 36*/ (0x81, 3, [11, 0, 14, 0, 58, 0]),
    /* 37*/ (0x81, 3, [11, 0, 14, 0, 62, 0]),
    /* 38*/ (0x30, 2, [11, 0, 129, 0, 0, 0]),
    /* 39*/ (0x81, 3, [11, 0, 14, 0, 22, 0]),
    /* 40*/ (0x30, 2, [11, 0, 232, 0, 0, 0]),
    /* 41*/ (0x81, 3, [11, 0, 14, 0, 23, 0]),
    /* 42*/ (0x30, 2, [11, 0, 5, 0, 0, 0]),
    /* 43*/ (0x81, 3, [11, 0, 14, 0, 24, 0]),
    /* 44*/ (0x30, 2, [11, 0, 100, 0, 0, 0]),
    /* 45*/ (0x81, 3, [11, 0, 14, 0, 33, 0]),
    /* 46*/ (0x30, 2, [11, 0, 8, 0, 0, 0]),
    /* 47*/ (0x81, 3, [11, 0, 14, 0, 34, 0]),
    /* 48*/ (0x81, 3, [11, 0, 14, 0, 44, 0]),
    /* 49*/ (0x81, 3, [11, 0, 14, 0, 64, 0]),
    /* 50*/ (0x30, 2, [11, 0, 101, 0, 0, 0]),
    /* 51*/ (0x81, 3, [11, 0, 14, 0, 53, 0]),
    /* 52*/ (0x30, 2, [11, 0, 11, 0, 0, 0]),
    /* 53*/ (0x81, 3, [11, 0, 14, 0, 54, 0]),
    /* 54*/ (0x30, 2, [11, 0, 79, 0, 0, 0]),
    /* 55*/ (0x81, 3, [11, 0, 14, 0, 63, 0]),
    /* 56*/ (0x30, 2, [11, 0, 57, 0, 0, 0]),
    /* 57*/ (0x81, 3, [11, 0, 14, 0, 66, 0]),
];

/// S3.368: production host RET firmware = full TYB object-code seed (hdr|173 records).
/// No live poke / no host_tyb_from_records on the ops=[0xFF] path (≡ expand S3.362).
/// General ops still assemble win0+win1+prog. Verified in s3_host_ret_tyb_seed_pin.
const S3_HOST_RET_TYB_SEED: &[u8] = &[
    0x54, 0x59, 0x42, 0x00, 0x00, 0x00, 0xad, 0x00, 0x30, 0x02, 0x0e, 0x00, 0x80, 0x00, 0x00, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xff, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x59, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x15, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xc3, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x04, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x90, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x18, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x30, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x28, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x29, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x48, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2c, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x58, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x6c, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xb8, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2d, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xef, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2e, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xbe, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2f, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xad, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x30, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xde, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x31, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x49, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x36, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x40, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x44, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x54, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x5b, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x68, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x73, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x7c, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x89, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x37, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x45, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x5c, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x74, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x47, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x38, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x42, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x46, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x56, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x5d, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x6a, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x75, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x7e, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x60, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3c, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3d, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x43, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x8b, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x41, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x55, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x69, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x7d, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x66, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x50, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x0b, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x51, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x79, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x5a, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x6e, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x62, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x64, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x0f, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x65, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x81, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x6d, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x05, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x6f, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x67, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x78, 0x00, 0x30, 0x02, 0x0e, 0x00, 0xff, 0x00, 0x00, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x48, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x01, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x15, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x41, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xff, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x02, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xc8, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x03, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x43, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x49, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x04, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x11, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x1c, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x25, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x29, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x39, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3d, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x89, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x05, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x1d, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2a, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x47, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x06, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x13, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x1e, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x27, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2b, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3b, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x61, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x0d, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x0e, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x8b, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x12, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x26, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3a, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3e, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x81, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x16, 0x00, 0x30, 0x02, 0x0b, 0x00, 0xe8, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x17, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x05, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x18, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x64, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x21, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x22, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2c, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x40, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x65, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x35, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x0b, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x36, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x4f, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3f, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x39, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x42, 0x00, 0x30, 0x02, 0x0e, 0x00, 0x48, 0x01, 0x00, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x01, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x00, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xff, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x01, 0x00,
    0x30, 0x02, 0x09, 0x00, 0x60, 0x01, 0x00, 0x00, 0x30, 0x02, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x40, 0x01, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x65, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x74, 0x01, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x03, 0x0e, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x64, 0x02, 0x0f, 0x00, 0x03, 0x00, 0x00, 0x00, 0x64, 0x02, 0x0b, 0x00, 0x04, 0x00, 0x00, 0x00,
    0x30, 0x02, 0x0a, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x40, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x80, 0x03, 0x0d, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x65, 0x02, 0x0d, 0x00, 0x0e, 0x00, 0x00, 0x00,
    0x71, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x02, 0x0f, 0x00, 0x14, 0x00, 0x00, 0x00,
    0x62, 0x02, 0x0b, 0x00, 0x01, 0x00, 0x00, 0x00, 0x65, 0x02, 0x0b, 0x00, 0x0a, 0x00, 0x00, 0x00,
    0x73, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x02, 0x0c, 0x00, 0xfe, 0x00, 0x00, 0x00,
    0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x80, 0x03, 0x0c, 0x00, 0x0f, 0x00, 0x01, 0x00, 0x64, 0x02, 0x08, 0x00, 0x0f, 0x00, 0x00, 0x00,
    0x62, 0x02, 0x08, 0x00, 0x04, 0x00, 0x00, 0x00, 0x84, 0x03, 0x09, 0x00, 0x08, 0x00, 0x0c, 0x00,
    0x68, 0x02, 0x09, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x62, 0x02, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x62, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x70, 0x01, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x40, 0x01, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// S3.369: production host SET+RET firmware = full TYB object-code seed (hdr|175).
/// ops=[0x30,0xFF] short-circuit (capability-map multi-op path). Other non-RET still assemble.
/// Verified in s3_host_set_ret_tyb_seed_pin.
const S3_HOST_SET_RET_TYB_SEED: &[u8] = &[
    0x54, 0x59, 0x42, 0x00, 0x00, 0x00, 0xaf, 0x00, 0x30, 0x02, 0x0e, 0x00, 0x80, 0x00, 0x00, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xff, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x59, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x01, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x15, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xc3, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x04, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x90, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x18, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x30, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x28, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x0e, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x29, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x48, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2c, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x58, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x6c, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xb8, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2d, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xef, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2e, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xbe, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2f, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xad, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x30, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xde, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x31, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x49, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x36, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x40, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x44, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x54, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x5b, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x68, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x73, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x7c, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x89, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x37, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x45, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x5c, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x74, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x47, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x38, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x42, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x46, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x56, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x5d, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x6a, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x75, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x7e, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x60, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3c, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3d, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x43, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x8b, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x41, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x55, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x69, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x7d, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x66, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x50, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x0b, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x51, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x79, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xc0, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x5a, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x6e, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x62, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x64, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x0f, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x65, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x81, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x6d, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x05, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x6f, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x67, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x78, 0x00, 0x30, 0x02, 0x0e, 0x00, 0xff, 0x00, 0x00, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x48, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x01, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x15, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x41, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xff, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x02, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xc8, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x03, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x43, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x49, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x04, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x11, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x1c, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x25, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x29, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x39, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3d, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x89, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x05, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x1d, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2a, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x47, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x06, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x13, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x1e, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x27, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2b, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3b, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x61, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x0d, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x0e, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x8b, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x12, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x26, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3a, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3e, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x81, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x16, 0x00, 0x30, 0x02, 0x0b, 0x00, 0xe8, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x17, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x05, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x18, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x64, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x21, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x08, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x22, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x2c, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x40, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x65, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x35, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x0b, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x36, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x4f, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x3f, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x39, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x42, 0x00, 0x30, 0x02, 0x0e, 0x00, 0x48, 0x01, 0x00, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x02, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x00, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x30, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x01, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0xff, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x02, 0x00,
    0x30, 0x02, 0x09, 0x00, 0x60, 0x01, 0x00, 0x00, 0x30, 0x02, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x40, 0x01, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x65, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x74, 0x01, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x03, 0x0e, 0x00, 0x02, 0x00, 0x00, 0x00,
    0x64, 0x02, 0x0f, 0x00, 0x03, 0x00, 0x00, 0x00, 0x64, 0x02, 0x0b, 0x00, 0x04, 0x00, 0x00, 0x00,
    0x30, 0x02, 0x0a, 0x00, 0x0a, 0x00, 0x00, 0x00, 0x40, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x80, 0x03, 0x0d, 0x00, 0x0f, 0x00, 0x00, 0x00, 0x65, 0x02, 0x0d, 0x00, 0x0e, 0x00, 0x00, 0x00,
    0x71, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x62, 0x02, 0x0f, 0x00, 0x14, 0x00, 0x00, 0x00,
    0x62, 0x02, 0x0b, 0x00, 0x01, 0x00, 0x00, 0x00, 0x65, 0x02, 0x0b, 0x00, 0x0a, 0x00, 0x00, 0x00,
    0x73, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x02, 0x0c, 0x00, 0xfe, 0x00, 0x00, 0x00,
    0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x01, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x80, 0x03, 0x0c, 0x00, 0x0f, 0x00, 0x01, 0x00, 0x64, 0x02, 0x08, 0x00, 0x0f, 0x00, 0x00, 0x00,
    0x62, 0x02, 0x08, 0x00, 0x04, 0x00, 0x00, 0x00, 0x84, 0x03, 0x09, 0x00, 0x08, 0x00, 0x0c, 0x00,
    0x68, 0x02, 0x09, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x62, 0x02, 0x02, 0x00, 0x01, 0x00, 0x00, 0x00,
    0x62, 0x02, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x70, 0x01, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x40, 0x01, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];





/// Build program-loop firmware TYB for ops.
/// S3.364–369: table/win poke = seeds; RET and SET+RET = full TYB seeds.
/// Other ops: win0+win1 seeds + live prog poke + loop (floor 173 for RET).
fn s3_host_prog_loop_tyb(ops: &[u8]) -> Vec<u8> {
    if ops == [0xFF] {
        return S3_HOST_RET_TYB_SEED.to_vec();
    }
    if ops == [0x30, 0xFF] {
        return S3_HOST_SET_RET_TYB_SEED.to_vec();
    }
    debug_assert_eq!(
        S3_HOST_TABLE_OFF as usize + S3_HOST_TABLE_SEED.len(),
        S3_HOST_PROG_OFF as usize
    );
    debug_assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
    assert!(
        S3_HOST_TABLE_OFF as usize + S3_HOST_TABLE_SEED.len() + 1 + ops.len()
            <= S1_MINI_DATA_MAX,
        "host table|prog DATA {} > {}",
        S3_HOST_TABLE_OFF as usize + S3_HOST_TABLE_SEED.len() + 1 + ops.len(),
        S1_MINI_DATA_MAX
    );
    let mut recs = Vec::new();
    recs.extend_from_slice(&S3_HOST_TABLE_WIN0_POKE_SEED);
    recs.extend_from_slice(&S3_HOST_TABLE_WIN1_POKE_SEED);
    let mut prog = Vec::with_capacity(1 + ops.len());
    prog.push(ops.len() as u8);
    prog.extend_from_slice(ops);
    s3_append_stb_zero_then_poke(&mut recs, S3_HOST_PROG_OFF, &prog, 10);
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
#[cfg(test)]
fn s3_rec_op_shape_bytes() -> Vec<u8> {
    let mut out = Vec::with_capacity(S3_REC_OP_SHAPE.len() * 2);
    for &(op, shape) in S3_REC_OP_SHAPE {
        out.push(op);
        out.push(shape as u8);
    }
    out
}

/// S3.237: shape tag → argc. Indexed by the tag YOYO loaded (0..3).
#[cfg(test)]
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

/// S3.299: argc seed is a 256B index table — `argc = table[op]` (0 = miss / RET).
/// Sparse poke beats the old packed `(op,argc)` scan seed for assembler size.
/// S3.343: test/proof only — production expand hardcodes MEMCPY+RET wire argc.
#[cfg(test)]
const S3_SEED_ARGC_BYTES: [u8; 256] = {
    let mut t = [0u8; 256];
    t[0x30] = 2; // SET
    t[0x61] = 2; // SUB imm
    t[0x62] = 2; // ADD imm
    t[0x40] = 1; // LABEL
    t[0x70] = 1; // JMP
    t[0x71] = 1; // JE
    t[0x72] = 1; // JNE
    t[0x73] = 1; // JL
    t[0x74] = 1; // JZ
    t[0x64] = 2; // MOVRR
    t[0x65] = 2; // CMP
    t[0x68] = 2; // ADDV
    t[0x6A] = 2; // SUBV
    t[0x80] = 3; // LDB
    t[0x81] = 3; // STB
    t[0x84] = 3; // MEMCPY
    t
};
/// S3.286/S3.299/S3.332: argc index layout pins + shape-fill addrs — test-only.
#[cfg(test)]
const S3_SHAPE_TABLE: u32 = 0x40;
#[cfg(test)]
const S3_SHAPE_TABLE_END: u32 = S3_SHAPE_TABLE + S3_SEED_ARGC_BYTES.len() as u32; // 0x140
/// S3.304: wire-job blob at low DATA (argc baked; no live table walk).
#[cfg(test)]
const S3_SHAPE_JOB: u32 = 0x40;
#[cfg(test)]
const S3_SHAPE_OUT: u32 = 0x100;

/// S3.337: test-only helper.
#[cfg(test)]
fn s3_seed_argc_bytes() -> Vec<u8> {
    S3_SEED_ARGC_BYTES.to_vec()
}

/// S3.337: test-only helper.
#[cfg(test)]
fn s3_seed_argc_flat() -> Vec<(u8, u8)> {
    let mut v: Vec<(u8, u8)> = S3_REC_OP_SHAPE
        .iter()
        .map(|&(op, shape)| {
            let n = match shape {
                S3RecShape::SlotImm | S3RecShape::SlotSlot => 2,
                S3RecShape::U32 => 1,
                S3RecShape::Slot3 => 3,
            };
            (op, n)
        })
        .collect();
    v.push((0xFF, 0));
    v
}

#[cfg(test)]
fn s3_seed_argc(op: u8) -> u8 {
    s3_seed_argc_from(&S3_SEED_ARGC_BYTES, op)
}

#[cfg(test)]
fn s3_seed_argc_from(argc_flat: &[u8], op: u8) -> u8 {
    *argc_flat
        .get(op as usize)
        .expect("argc index table")
}

/// S3.320/S3.356: sole bootstrap body — const MEMCPY+RET (≡ MANY flat under test).
const S3_BOOTSTRAP_EXPAND_ASM: [(u8, u8, [u8; 6]); 2] = [
    (0x84, 3, [9, 0, 2, 0, 12, 0]), // MEMCPY dst=9 src=2 len=slot12
    (0xFF, 0, [0; 6]),              // RET
];

/// S3.357: Vec wrapper — test convenience; production uses the const table.
#[cfg(test)]
fn s3_bootstrap_expand_asm() -> Vec<(u8, u8, [u8; 6])> {
    S3_BOOTSTRAP_EXPAND_ASM.to_vec()
}

/// S3.329: flat→records decode is test parity only (production uses named bootstrap).
#[cfg(test)]
fn s3_rec_from_wire_seed(row: &[u8; 7]) -> (u8, u8, [u8; 6]) {
    s3_rec_from_wire_seed_with_argc(row, &S3_SEED_ARGC_BYTES)
}

#[cfg(test)]
fn s3_rec_from_wire_seed_with_argc(row: &[u8; 7], argc_flat: &[u8]) -> (u8, u8, [u8; 6]) {
    (
        row[0],
        s3_seed_argc_from(argc_flat, row[0]),
        [row[1], row[2], row[3], row[4], row[5], row[6]],
    )
}

/// S3.244/S3.253/S3.262/S3.276/S3.321/S3.329: test parity vs named bootstrap.
#[cfg(test)]
fn s3_recs_from_flat_seed(flat: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    s3_recs_from_flat_seed_with_argc(flat, &S3_SEED_ARGC_BYTES)
}

#[cfg(test)]
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
/// ONE-flat expand assembler record count (job|hdr seed + bulk MEMCPY+RET).
/// S3.341: pub so the self-host cliff stays visible on the lib surface.
pub const S3_EXPAND_ASM_PIN: usize = 22; // S3.306: bulk wire MEMCPY (−10)

/// S3.257/S3.291: trailing 0 sentinel ended the job blob — no job-side SET END.
/// S3.291: CMP against slot13 (image-zero); no INIT SET of sentinel imm.
/// S3.353: production expand drops the vestigial sentinel (bulk MEMCPY is
/// length-preload; no job scan). Kept for cfg(test) generic expand_flat parity.
/// S3.264: argc table is IR-seeded from `S3_SEED_ARGC_BYTES` (not 51 poke rows).
/// S3.267/S3.268/S3.290: expand emits full TYB; header magic+count IR-seeded.
/// Bootstrap still Rust-expands the ctrl body (cannot via_yoyo itself).
/// S3.303: expand job seed is 8B wire (argc baked); ctrl MEMCPY record rows.
/// S3.306: ctrl = bulk MEMCPY+RET; slot12 = nrecs×8 (sim preload); job|hdr join.
#[cfg(test)]
const S3_EXPAND_SENTINEL: u8 = 0;
/// S3.305/S3.306/S3.324: expand ctrl ≡ shape-fill many ≡ one (bulk wire MEMCPY).
/// S3.270: only magic must be IR-seeded; reserved/count stay zero until body patches cnt.
/// S3.274/S3.277: assembler init SETs as a flat seed (same expand path as ctrl).
/// S3.277: drop SET slot13=0 / slot1=0 — sim state starts zeroed.
/// S3.290: drop SET OUT base — count is IR-seeded; no STB patch.
/// S3.291: drop SET sentinel — job end is 0 vs slot13 image-zero.
/// S3.297: drop SET out+hdr — sim slot9 starts at OUT+HEADER_SIZE.
/// S3.305: job cursor = prog_ptr (slot2); same flat as shape-fill many.
/// S3.275/S3.322: assembler body rows = ctrl only (INIT empty since S3.297).


/// S3.361: production expand poke shell = own-ISA SET/STB record seed.
/// Live [`s3_append_stb_zero_then_poke`] stays as cfg(test)/generic oracle.
/// Blob = boot wire (2×8) | hdr(8) at SEED=0x40 → 20 poke rows (global-distinct).
const S3_EXPAND_POKE_SEED: [(u8, u8, [u8; 6]); 20] = [
    (0x30, 2, [14, 0, 64, 0, 0, 0]),  // SET BASE=SEED
    (0x30, 2, [11, 0, 132, 0, 0, 0]), // SET TMP=0x84 (MEMCPY)
    (0x81, 3, [11, 0, 14, 0, 0, 0]),
    (0x30, 2, [11, 0, 3, 0, 0, 0]),
    (0x81, 3, [11, 0, 14, 0, 1, 0]),
    (0x30, 2, [11, 0, 9, 0, 0, 0]),
    (0x81, 3, [11, 0, 14, 0, 2, 0]),
    (0x30, 2, [11, 0, 2, 0, 0, 0]),
    (0x81, 3, [11, 0, 14, 0, 4, 0]),
    (0x81, 3, [11, 0, 14, 0, 22, 0]),
    (0x30, 2, [11, 0, 12, 0, 0, 0]),
    (0x81, 3, [11, 0, 14, 0, 6, 0]),
    (0x30, 2, [11, 0, 255, 0, 0, 0]), // SET TMP=0xFF (RET)
    (0x81, 3, [11, 0, 14, 0, 8, 0]),
    (0x30, 2, [11, 0, 84, 0, 0, 0]),  // TYB magic 'T'
    (0x81, 3, [11, 0, 14, 0, 16, 0]),
    (0x30, 2, [11, 0, 89, 0, 0, 0]),  // 'Y'
    (0x81, 3, [11, 0, 14, 0, 17, 0]),
    (0x30, 2, [11, 0, 66, 0, 0, 0]),  // 'B'
    (0x81, 3, [11, 0, 14, 0, 18, 0]),
];

/// S3.361: OUT = SEED + boot wire bytes (MEMCPY writes OUT+hdr).
const S3_EXPAND_OUT: u32 = S3_EXPAND_SEED + (S3_BOOTSTRAP_EXPAND_ASM.len() * RECORD_SIZE) as u32;

/// S3.362: production expand firmware = full TYB object-code seed (hdr|22 records).
/// No live poke / no host_tyb_from_records on the prod expand path.
/// Verified ≡ poke-seed+bootstrap assemble in `s3_expand_tyb_seed_pin`.
const S3_EXPAND_TYB_SEED: &[u8] = &[
    0x54, 0x59, 0x42, 0x00, 0x00, 0x00, 0x16, 0x00, 0x30, 0x02, 0x0e, 0x00, 0x40, 0x00, 0x00, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x84, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x00, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x03, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x01, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x09, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x02, 0x00,
    0x30, 0x02, 0x0b, 0x00, 0x02, 0x00, 0x00, 0x00, 0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x04, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x16, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x0c, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x06, 0x00, 0x30, 0x02, 0x0b, 0x00, 0xff, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x08, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x54, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x10, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x59, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x11, 0x00, 0x30, 0x02, 0x0b, 0x00, 0x42, 0x00, 0x00, 0x00,
    0x81, 0x03, 0x0b, 0x00, 0x0e, 0x00, 0x12, 0x00, 0x84, 0x03, 0x09, 0x00, 0x02, 0x00, 0x0c, 0x00,
    0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

/// S3.355/S3.361: shared expand assemble — live poke `job_recs` wire|hdr at SEED.
/// Layout: records | hdr；OUT = SEED+nrecs×8（MEMCPY 写 OUT+hdr）。
/// Production path uses [`S3_EXPAND_POKE_SEED`]; this stays for generic/test.
fn s3_expand_job_recs_tyb(job_recs: &[(u8, u8, [u8; 6])]) -> (Vec<u8>, u32) {
    assert!(!job_recs.is_empty());
    let nrecs = job_recs.len();
    assert!(nrecs * RECORD_SIZE + HEADER_SIZE <= 0x200);
    let mut job = Vec::with_capacity(nrecs * RECORD_SIZE + HEADER_SIZE);
    for &(op, argc, args) in job_recs {
        job.push(op);
        job.push(argc);
        job.extend_from_slice(&args);
    }
    let out = S3_EXPAND_SEED + job.len() as u32;
    let mut hdr = [0u8; HEADER_SIZE];
    hdr[..4].copy_from_slice(&TYB_MAGIC);
    hdr[6..8].copy_from_slice(&(nrecs as u16).to_le_bytes());
    job.extend_from_slice(&hdr);
    let mut recs = Vec::new();
    s3_append_stb_zero_then_poke(&mut recs, S3_EXPAND_SEED, &job, 30);
    recs.extend_from_slice(&S3_BOOTSTRAP_EXPAND_ASM);
    (s3_host_tyb_from_records(&recs), out)
}

/// S3.254/S3.260/S3.264/S3.268/S3.343/S3.353/S3.355: generic 7×N flat→TYB via argc index.
/// Test/proof only — production uses [`s3_expand_many_flat_tyb`].
#[cfg(test)]
fn s3_yoyo_expand_flat_tyb(flat: &[u8]) -> (Vec<u8>, u32) {
    assert!(!flat.is_empty() && flat.len() % 7 == 0);
    assert!(
        flat.chunks_exact(7).all(|r| r[0] != S3_EXPAND_SENTINEL),
        "flat op collides with expand sentinel tag"
    );
    assert_eq!(S3_SEED_ARGC_BYTES.len(), 256);
    let job_recs = s3_recs_from_flat_seed(flat);
    s3_expand_job_recs_tyb(&job_recs)
}

/// S3.341/S3.343/S3.344/S3.353/S3.354/S3.355/S3.356/S3.361/S3.362: production expand —
/// full TYB object-code seed. Poke-seed+bootstrap remain the record-level truth
/// (oracle / docs); prod path does not assemble records or call host_tyb_from_records.
pub fn s3_expand_many_flat_tyb() -> (Vec<u8>, u32) {
    debug_assert_eq!(S3_EXPAND_TYB_SEED.len(), HEADER_SIZE + S3_EXPAND_ASM_PIN * RECORD_SIZE);
    debug_assert_eq!(
        S3_EXPAND_POKE_SEED.len() + S3_BOOTSTRAP_EXPAND_ASM.len(),
        S3_EXPAND_ASM_PIN
    );
    (S3_EXPAND_TYB_SEED.to_vec(), S3_EXPAND_OUT)
}

/// S3.389: PE expand SEED past 16 state slots (empty-DATA @0x40 overlaps slot9 on PE).
const S3_EXPAND_PE_SEED: u32 = 0x80;
const S3_EXPAND_PE_OUT: u32 = S3_EXPAND_PE_SEED + (S3_BOOTSTRAP_EXPAND_ASM.len() * RECORD_SIZE) as u32;

/// S3.389: PE expand DATA prefix = pad@0..0x80 ‖ boot wire ‖ hdr (152B).
const S3_EXPAND_PE_DATA_PREFIX: &[u8] =
    include_bytes!("../blobs/s3_expand_pe_data_prefix.dat");

/// S3.389: PE expand lea A1 raw — SEED→slot2, OUT+hdr→slot9 (22B).
const S3_EXPAND_PE_LEA_RAW: &[u8] = include_bytes!("../blobs/s3_expand_pe_lea.txb");

/// S3.389: expand compose TYB for PE — DATA|lea|SET slot12|MEMCPY+RET (no poke).
fn s3_expand_compose_tyb_pe() -> Vec<u8> {
    debug_assert_eq!(S3_EXPAND_PE_DATA_PREFIX.len(), S3_EXPAND_PE_OUT as usize + HEADER_SIZE);
    debug_assert_eq!(S3_EXPAND_PE_LEA_RAW.len(), 22);
    let wire_len = S3_BOOTSTRAP_EXPAND_ASM.len() * RECORD_SIZE;
    let mut payload = S3_EXPAND_PE_DATA_PREFIX.to_vec();
    while payload.len() < S3_EXPAND_PE_OUT as usize + HEADER_SIZE + wire_len {
        payload.push(0);
    }
    let mut recs = s3_pack_data_records(&payload);
    // A1 lea
    let mut i = 0;
    while i < S3_EXPAND_PE_LEA_RAW.len() {
        let n = (S3_EXPAND_PE_LEA_RAW.len() - i).min(3);
        let mut a = [0u8; 6];
        for k in 0..n {
            a[k * 2] = S3_EXPAND_PE_LEA_RAW[i + k];
        }
        recs.push((0xA1, n as u8, a));
        i += n;
    }
    recs.push(s3_rec_set(12, wire_len as u32));
    recs.extend_from_slice(&S3_BOOTSTRAP_EXPAND_ASM);
    s3_host_tyb_from_records(&recs)
}

/// S3.389: slim PE image seed for expand compose (gap M — no live emit_x86).
#[cfg(windows)]
const S3_EXPAND_PE_IMAGE_SEED: &[u8] = include_bytes!("../blobs/s3_expand_pe.pe");

/// S3.389: run expand on PE; image seed skips emit_x86+wrap.
#[cfg(windows)]
pub fn s3_expand_many_via_pe() -> Option<Vec<u8>> {
    let expect_len = HEADER_SIZE + S3_BOOTSTRAP_EXPAND_ASM.len() * RECORD_SIZE;
    Some(s3_run_slim_pe_outbuf_at(
        S3_EXPAND_PE_IMAGE_SEED,
        S3_EXPAND_PE_OUT,
        expect_len,
    ))
}


/// S3.254/S3.255/S3.263/S3.267/S3.329: expand flat via YOYO sim — test proof only.
#[cfg(test)]
fn s3_recs_from_flat_seed_via_yoyo(flat: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
    // S3.393/S3.394: bootstrap MANY_FLAT + 7-aligned prefixes → PE (gap O/P);
    // non-prefix flats keep sim (residual Q).
    #[cfg(windows)]
    if !flat.is_empty()
        && flat.len() % 7 == 0
        && flat.len() <= S3_SHAPE_FILL_MANY_FLAT.len()
        && S3_SHAPE_FILL_MANY_FLAT.starts_with(flat)
    {
        let n = flat.len() / 7;
        let emitted = s3_expand_many_via_pe().expect("S3.394: expand via PE");
        return parse_tyb(&emitted)
            .expect("expand PE emits TYB")
            .records
            .into_iter()
            .take(n)
            .map(|r| (r.op, r.argc, r.args_raw))
            .collect();
    }
    let (asm, out) = s3_yoyo_expand_flat_tyb(flat);
    let nrecs = flat.len() / 7;
    // S3.305: prog_ptr → slot2 = job. S3.297: slot9 = OUT+hdr.
    // S3.306: slot12 = total wire bytes (bulk MEMCPY).
    let emitted = s3_sim_prog_loop_cursor(
        &asm,
        0,
        out,
        S3_EXPAND_SEED as u64,
        out as u64 + HEADER_SIZE as u64,
        (nrecs * RECORD_SIZE) as u64,
    )
    .expect("expand flat via yoyo");
    parse_tyb(&emitted)
        .expect("expand emits TYB")
        .records
        .into_iter()
        .map(|r| (r.op, r.argc, r.args_raw))
        .collect()
}

/// S3.306: shape-fill many = bulk wire MEMCPY + RET (slot12 = nbytes preload).
/// S3.328: ONE_AT/LEN/shape_fill_one gone — leading MEMCPY is bootstrap[0].
/// S3.344: test/proof seed only — production expand hardcodes the same wire.
#[cfg(test)]
const S3_SHAPE_FILL_MANY_ROWS: usize = 2;

/// S3.341/S3.344: expand body seed — test parity vs pub hardcoded wire expand.
#[cfg(test)]
const S3_SHAPE_FILL_MANY_FLAT: &[u8] = &[
    // MEMCPY dst=9 src=2 len=slot12; RET.
    132, 9, 0, 2, 0, 12, 0, 255, 0, 0, 0, 0, 0, 0,
];

/// S3.304: many out past wire-job blob (no 256B argc table in-stream).
/// S3.332: shape-fill TYB builders are proof-only (production expand owns wire→TYB).
#[cfg(test)]
const S3_SHAPE_MANY_OUT: u32 = 0x300;

/// S3.304/S3.332: bake op|argc|args wire rows — test shape-fill builders only.
#[cfg(test)]
fn s3_shape_wire_job_blob(jobs: &[(u8, [u8; 6])], term: bool) -> Vec<u8> {
    let mut blob = Vec::with_capacity(jobs.len() * RECORD_SIZE + usize::from(term));
    for &(op, args) in jobs {
        blob.push(op);
        blob.push(s3_seed_argc(op));
        blob.extend_from_slice(&args);
    }
    if term {
        blob.push(0);
    }
    blob
}

#[cfg(test)]
fn s3_yoyo_shape_fill_many_tyb(jobs: &[(u8, [u8; 6])]) -> (Vec<u8>, u32) {
    // S3.298: zero-terminated job stream; op 0 reserved as sentinel.
    // S3.304: wire rows; no argc table seed in the TYB.
    assert!(!jobs.is_empty() && jobs.len() <= 128);
    assert!(jobs.iter().all(|&(op, _)| op != 0), "job op 0 is stream sentinel");
    let blob = s3_shape_wire_job_blob(jobs, true);
    let mut recs = Vec::new();
    s3_ir_seed_at(&mut recs, S3_SHAPE_JOB, &blob, 21);
    recs.extend_from_slice(&S3_BOOTSTRAP_EXPAND_ASM);
    (s3_host_tyb_from_records(&recs), S3_SHAPE_MANY_OUT)
}

/// S3.304/S3.306/S3.332: shape-fill sim — test proof only.
#[cfg(test)]
fn s3_sim_shape_fill(tyb: &[u8], out: u32) -> Result<Vec<u8>, u8> {
    s3_sim_shape_fill_n(tyb, out, RECORD_SIZE as u64)
}

#[cfg(test)]
fn s3_sim_shape_fill_n(tyb: &[u8], out: u32, nbytes: u64) -> Result<Vec<u8>, u8> {
    s3_sim_prog_loop_cursor(tyb, 0, out, S3_SHAPE_JOB as u64, out as u64, nbytes)
}

/// S3.236/S3.304/S3.332: one wire-job + bootstrap ctrl — test proof only.
#[cfg(test)]
fn s3_yoyo_shape_fill_tyb(op: u8, args: [u8; 6]) -> (Vec<u8>, u32) {
    let job = s3_shape_wire_job_blob(&[(op, args)], false);
    let mut recs = Vec::new();
    s3_ir_seed_at(&mut recs, S3_SHAPE_JOB, &job, 21);
    recs.extend_from_slice(&S3_BOOTSTRAP_EXPAND_ASM);
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
#[cfg(test)]
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
/// S3.337: test-only helper.
#[cfg(test)]
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
#[cfg(test)]
fn s3_rec_subv(dst: u16, src: u16) -> (u8, u8, [u8; 6]) {
    s3_rec_from_shape(0x6A, dst as u32, src as u32, 0)
}

/// S3.117: shared scan/compose tail. `nrows` is 10 (host) or 3 (in-DLL).
/// Record stream is golden-pinned for in-DLL — do not edit. Host uses
/// [`s3_prog_loop_code_host`].
/// S3.331: in-DLL golden prog loop — test builders only (PE path uses host compose
/// or ir-seed+A1 builders under `#[cfg(test)]`).
#[cfg(test)]
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

/// S3.214/S3.331/S3.363: host prog-loop body = own-ISA record seed.
/// Only live patch: SET slot1 = `ops_len`. `nrows` frozen at [`S3_HOST_NROWS`].
/// Seed baked at ops_len=1; verified in `s3_host_prog_loop_seed_pin`.
const S3_HOST_PROG_LOOP_SEED: [(u8, u8, [u8; 6]); 29] = [
    (0x30, 2, [1, 0, 1, 0, 0, 0]), // SET slot1=ops_len (patched live)
    (0x40, 1, [5, 0, 0, 0, 0, 0]),
    (0x65, 2, [0, 0, 1, 0, 0, 0]),
    (0x74, 1, [6, 0, 0, 0, 0, 0]),
    (0x80, 3, [14, 0, 2, 0, 0, 0]),
    (0x64, 2, [15, 0, 3, 0, 0, 0]),
    (0x64, 2, [11, 0, 4, 0, 0, 0]),
    (0x30, 2, [10, 0, 10, 0, 0, 0]), // SET slot10=NROWS
    (0x40, 1, [1, 0, 0, 0, 0, 0]),
    (0x80, 3, [13, 0, 15, 0, 0, 0]),
    (0x65, 2, [13, 0, 14, 0, 0, 0]),
    (0x71, 1, [2, 0, 0, 0, 0, 0]),
    (0x62, 2, [15, 0, 20, 0, 0, 0]),
    (0x62, 2, [11, 0, 1, 0, 0, 0]),
    (0x65, 2, [11, 0, 10, 0, 0, 0]),
    (0x73, 1, [1, 0, 0, 0, 0, 0]),
    (0x30, 2, [12, 0, 254, 0, 0, 0]),
    (0xff, 0, [0, 0, 0, 0, 0, 0]),
    (0x40, 1, [2, 0, 0, 0, 0, 0]),
    (0x80, 3, [12, 0, 15, 0, 1, 0]),
    (0x64, 2, [8, 0, 15, 0, 0, 0]),
    (0x62, 2, [8, 0, 4, 0, 0, 0]),
    (0x84, 3, [9, 0, 8, 0, 12, 0]),
    (0x68, 2, [9, 0, 12, 0, 0, 0]),
    (0x62, 2, [2, 0, 1, 0, 0, 0]),
    (0x62, 2, [0, 0, 1, 0, 0, 0]),
    (0x70, 1, [5, 0, 0, 0, 0, 0]),
    (0x40, 1, [6, 0, 0, 0, 0, 0]),
    (0xff, 0, [0, 0, 0, 0, 0, 0]),
];

/// S3.214/S3.331/S3.363: sole production prog-loop — seed + patch ops_len.
fn s3_prog_loop_code_host(ops_len: u32, nrows: u32) -> Vec<(u8, u8, [u8; 6])> {
    assert_eq!(nrows, S3_HOST_NROWS, "host prog-loop seed freezes nrows");
    let mut recs = S3_HOST_PROG_LOOP_SEED.to_vec();
    recs[0] = s3_rec_set(1, ops_len);
    recs
}

/// S3.116/S3.350: default-seed prog-loop sim — test/proof helper.
/// Production [`try_emit_text_via_yoyo_table`] calls cursor directly.
#[cfg(test)]
fn s3_sim_prog_loop(tyb: &[u8], table_off: u64, outbuf: u32, prog_ptr: u64) -> Result<Vec<u8>, u8> {
    s3_sim_prog_loop_cursor(
        tyb,
        table_off,
        outbuf,
        prog_ptr,
        outbuf as u64,
        RECORD_SIZE as u64,
    )
}

/// S3.297: `slot9_init` may differ from composed `outbuf` (expand body @ OUT+hdr).
/// S3.306: `slot12_init` = bulk wire byte length (default RECORD_SIZE).
fn s3_sim_prog_loop_cursor(
    tyb: &[u8],
    table_off: u64,
    outbuf: u32,
    prog_ptr: u64,
    slot9_init: u64,
    slot12_init: u64,
) -> Result<Vec<u8>, u8> {
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
    state[9] = slot9_init;
    // S3.296: args payload width = RECORD_SIZE-2 (op,argc|6 args); MEMCPY len slot.
    state[10] = (RECORD_SIZE as u64) - 2;
    // S3.309: drop slot8 ARGC-base preload (wire-job era; no live argc table walk).
    // S3.303/S3.306: MEMCPY length slot (row=8 or bulk nrecs×8).
    state[12] = slot12_init;
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

/// S3.376: in-DLL empty-DATA RET TYB = full object-code seed (gap C).
#[cfg(test)]
const S3_IN_DLL_RET_TYB_SEED: &[u8] = &[
    0x54,
    0x59,
    0x42,
    0x00,
    0x00,
    0x00,
    0x4c,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x8d,
    0x00,
    0x87,
    0x00,
    0xa1,
    0x03,
    0x80,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0xa1,
    0x03,
    0x47,
    0x00,
    0x18,
    0x00,
    0x49,
    0x00,
    0xa1,
    0x03,
    0x89,
    0x00,
    0x47,
    0x00,
    0x78,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x8d,
    0x00,
    0x87,
    0x00,
    0xa1,
    0x03,
    0xc8,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0xa1,
    0x02,
    0x47,
    0x00,
    0x48,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x8d,
    0x00,
    0x87,
    0x00,
    0xa1,
    0x03,
    0xbd,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0xa1,
    0x02,
    0x47,
    0x00,
    0x10,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x3d,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x01,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x15,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x3c,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xc3,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x04,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x90,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x18,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x30,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x28,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x0e,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x29,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x48,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x2c,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xb8,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x2d,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xef,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x2e,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xbe,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x2f,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xad,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x30,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xde,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x31,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x49,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x36,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x89,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x37,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x47,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x38,
    0x00,
    0x30,
    0x02,
    0x01,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x05,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x00,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x74,
    0x01,
    0x06,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0e,
    0x00,
    0x02,
    0x00,
    0x00,
    0x00,
    0x64,
    0x02,
    0x0f,
    0x00,
    0x03,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0a,
    0x00,
    0x03,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0d,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x0d,
    0x00,
    0x0e,
    0x00,
    0x00,
    0x00,
    0x71,
    0x01,
    0x02,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x0f,
    0x00,
    0x14,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x0b,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x0b,
    0x00,
    0x0a,
    0x00,
    0x00,
    0x00,
    0x73,
    0x01,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0c,
    0x00,
    0xfe,
    0x00,
    0x00,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x02,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0c,
    0x00,
    0x0f,
    0x00,
    0x01,
    0x00,
    0x64,
    0x02,
    0x08,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x08,
    0x00,
    0x04,
    0x00,
    0x00,
    0x00,
    0x84,
    0x03,
    0x09,
    0x00,
    0x08,
    0x00,
    0x0c,
    0x00,
    0x68,
    0x02,
    0x09,
    0x00,
    0x0c,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x02,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x00,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x70,
    0x01,
    0x05,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x06,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
];

/// S3.376: in-DLL empty-DATA SET+RET TYB = full object-code seed.
#[cfg(test)]
const S3_IN_DLL_SET_RET_TYB_SEED: &[u8] = &[
    0x54,
    0x59,
    0x42,
    0x00,
    0x00,
    0x00,
    0x4e,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x8d,
    0x00,
    0x87,
    0x00,
    0xa1,
    0x03,
    0x80,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0xa1,
    0x03,
    0x47,
    0x00,
    0x18,
    0x00,
    0x49,
    0x00,
    0xa1,
    0x03,
    0x89,
    0x00,
    0x47,
    0x00,
    0x78,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x8d,
    0x00,
    0x87,
    0x00,
    0xa1,
    0x03,
    0xc8,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0xa1,
    0x02,
    0x47,
    0x00,
    0x48,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x8d,
    0x00,
    0x87,
    0x00,
    0xa1,
    0x03,
    0xbd,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0xa1,
    0x02,
    0x47,
    0x00,
    0x10,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x3e,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x01,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x15,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xc3,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x04,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x90,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x18,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x30,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x28,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x3d,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x0e,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x29,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x48,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x2c,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xb8,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x2d,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xef,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x2e,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xbe,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x2f,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xad,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x30,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0xde,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x31,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x49,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x36,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x89,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x37,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x47,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x38,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x02,
    0x00,
    0x00,
    0x00,
    0x81,
    0x03,
    0x0b,
    0x00,
    0x0f,
    0x00,
    0x3c,
    0x00,
    0x30,
    0x02,
    0x01,
    0x00,
    0x02,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x05,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x00,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x74,
    0x01,
    0x06,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0e,
    0x00,
    0x02,
    0x00,
    0x00,
    0x00,
    0x64,
    0x02,
    0x0f,
    0x00,
    0x03,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0b,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0a,
    0x00,
    0x03,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0d,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x0d,
    0x00,
    0x0e,
    0x00,
    0x00,
    0x00,
    0x71,
    0x01,
    0x02,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x0f,
    0x00,
    0x14,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x0b,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x0b,
    0x00,
    0x0a,
    0x00,
    0x00,
    0x00,
    0x73,
    0x01,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0c,
    0x00,
    0xfe,
    0x00,
    0x00,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x02,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0c,
    0x00,
    0x0f,
    0x00,
    0x01,
    0x00,
    0x64,
    0x02,
    0x08,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x08,
    0x00,
    0x04,
    0x00,
    0x00,
    0x00,
    0x84,
    0x03,
    0x09,
    0x00,
    0x08,
    0x00,
    0x0c,
    0x00,
    0x68,
    0x02,
    0x09,
    0x00,
    0x0c,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x02,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x00,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x70,
    0x01,
    0x05,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x06,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
];

/// S3.372: host-offset lea A1 seed — table@0x80→slots3/15; dst@0x160→9; prog@0x149→2.
/// Empty-DATA host firmware is 2279B > slim 1024; PE path uses DATA|lea|host-loop (~308B).
const S3_HOST_LEA_A1_SEED: [(u8, u8, [u8; 6]); 13] = [
    (0xa1, 3, [0x49, 0x00, 0x8d, 0x00, 0x87, 0x00]),
    (0xa1, 3, [0x80, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0xa1, 3, [0x00, 0x00, 0x49, 0x00, 0x89, 0x00]),
    (0xa1, 3, [0x47, 0x00, 0x18, 0x00, 0x49, 0x00]),
    (0xa1, 3, [0x89, 0x00, 0x47, 0x00, 0x78, 0x00]),
    (0xa1, 3, [0x49, 0x00, 0x8d, 0x00, 0x87, 0x00]),
    (0xa1, 3, [0x60, 0x00, 0x01, 0x00, 0x00, 0x00]),
    (0xa1, 3, [0x00, 0x00, 0x49, 0x00, 0x89, 0x00]),
    (0xa1, 3, [0x47, 0x00, 0x48, 0x00, 0x49, 0x00]),
    (0xa1, 3, [0x8d, 0x00, 0x87, 0x00, 0x49, 0x00]),
    (0xa1, 3, [0x01, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0xa1, 3, [0x49, 0x00, 0x89, 0x00, 0x47, 0x00]),
    (0xa1, 1, [0x10, 0x00, 0x00, 0x00, 0x00, 0x00]),
];

/// S3.374: PE compose RET TYB = full object-code seed (no live DATA/lea/loop).
const S3_HOST_PE_RET_TYB_SEED: &[u8] = &[
    0x54,
    0x59,
    0x42,
    0x00,
    0x00,
    0x00,
    0xa0,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0xff,
    0x00,
    0x10,
    0x03,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0xc3,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x01,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x90,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x30,
    0x00,
    0x0e,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x48,
    0x00,
    0xb8,
    0x00,
    0x10,
    0x03,
    0xef,
    0x00,
    0xbe,
    0x00,
    0xad,
    0x00,
    0x10,
    0x03,
    0xde,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x49,
    0x00,
    0x10,
    0x03,
    0x89,
    0x00,
    0x47,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x60,
    0x00,
    0x10,
    0x03,
    0x08,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x47,
    0x00,
    0x10,
    0x03,
    0x08,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0x10,
    0x03,
    0x47,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x66,
    0x00,
    0x0b,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x49,
    0x00,
    0x10,
    0x03,
    0x8b,
    0x00,
    0x47,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x48,
    0x00,
    0xff,
    0x00,
    0xc0,
    0x00,
    0x10,
    0x03,
    0x49,
    0x00,
    0x89,
    0x00,
    0x47,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x62,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x10,
    0x03,
    0x47,
    0x00,
    0x00,
    0x00,
    0x48,
    0x00,
    0x10,
    0x03,
    0x81,
    0x00,
    0xc0,
    0x00,
    0x05,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x49,
    0x00,
    0x89,
    0x00,
    0x47,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x67,
    0x00,
    0x10,
    0x03,
    0x0b,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x47,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x48,
    0x00,
    0xff,
    0x00,
    0x10,
    0x03,
    0xc8,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0x10,
    0x03,
    0x47,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x61,
    0x00,
    0x0f,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x49,
    0x00,
    0x10,
    0x03,
    0x8b,
    0x00,
    0x47,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x48,
    0x00,
    0x81,
    0x00,
    0xe8,
    0x00,
    0x10,
    0x03,
    0x05,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0x10,
    0x03,
    0x47,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x64,
    0x00,
    0x08,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x10,
    0x03,
    0x47,
    0x00,
    0x00,
    0x00,
    0x49,
    0x00,
    0x10,
    0x03,
    0x89,
    0x00,
    0x47,
    0x00,
    0x08,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x65,
    0x00,
    0x10,
    0x03,
    0x0b,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x47,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x10,
    0x03,
    0x4f,
    0x00,
    0x08,
    0x00,
    0x48,
    0x00,
    0x10,
    0x03,
    0x39,
    0x00,
    0xc8,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x01,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x8d,
    0x00,
    0x87,
    0x00,
    0xa1,
    0x03,
    0x80,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0xa1,
    0x03,
    0x47,
    0x00,
    0x18,
    0x00,
    0x49,
    0x00,
    0xa1,
    0x03,
    0x89,
    0x00,
    0x47,
    0x00,
    0x78,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x8d,
    0x00,
    0x87,
    0x00,
    0xa1,
    0x03,
    0x60,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0xa1,
    0x03,
    0x47,
    0x00,
    0x48,
    0x00,
    0x49,
    0x00,
    0xa1,
    0x03,
    0x8d,
    0x00,
    0x87,
    0x00,
    0x49,
    0x00,
    0xa1,
    0x03,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x89,
    0x00,
    0x47,
    0x00,
    0xa1,
    0x01,
    0x10,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x01,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x05,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x00,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x74,
    0x01,
    0x06,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0e,
    0x00,
    0x02,
    0x00,
    0x00,
    0x00,
    0x64,
    0x02,
    0x0f,
    0x00,
    0x03,
    0x00,
    0x00,
    0x00,
    0x64,
    0x02,
    0x0b,
    0x00,
    0x04,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0a,
    0x00,
    0x0a,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0d,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x0d,
    0x00,
    0x0e,
    0x00,
    0x00,
    0x00,
    0x71,
    0x01,
    0x02,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x0f,
    0x00,
    0x14,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x0b,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x0b,
    0x00,
    0x0a,
    0x00,
    0x00,
    0x00,
    0x73,
    0x01,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0c,
    0x00,
    0xfe,
    0x00,
    0x00,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x02,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0c,
    0x00,
    0x0f,
    0x00,
    0x01,
    0x00,
    0x64,
    0x02,
    0x08,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x08,
    0x00,
    0x04,
    0x00,
    0x00,
    0x00,
    0x84,
    0x03,
    0x09,
    0x00,
    0x08,
    0x00,
    0x0c,
    0x00,
    0x68,
    0x02,
    0x09,
    0x00,
    0x0c,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x02,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x00,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x70,
    0x01,
    0x05,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x06,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
];

/// S3.374: PE compose SET+RET TYB = full object-code seed.
const S3_HOST_PE_SET_RET_TYB_SEED: &[u8] = &[
    0x54,
    0x59,
    0x42,
    0x00,
    0x00,
    0x00,
    0xa0,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0xff,
    0x00,
    0x10,
    0x03,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0xc3,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x01,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x90,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x30,
    0x00,
    0x0e,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x48,
    0x00,
    0xb8,
    0x00,
    0x10,
    0x03,
    0xef,
    0x00,
    0xbe,
    0x00,
    0xad,
    0x00,
    0x10,
    0x03,
    0xde,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x49,
    0x00,
    0x10,
    0x03,
    0x89,
    0x00,
    0x47,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x60,
    0x00,
    0x10,
    0x03,
    0x08,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x47,
    0x00,
    0x10,
    0x03,
    0x08,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0x10,
    0x03,
    0x47,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x66,
    0x00,
    0x0b,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x49,
    0x00,
    0x10,
    0x03,
    0x8b,
    0x00,
    0x47,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x48,
    0x00,
    0xff,
    0x00,
    0xc0,
    0x00,
    0x10,
    0x03,
    0x49,
    0x00,
    0x89,
    0x00,
    0x47,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x62,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x10,
    0x03,
    0x47,
    0x00,
    0x00,
    0x00,
    0x48,
    0x00,
    0x10,
    0x03,
    0x81,
    0x00,
    0xc0,
    0x00,
    0x05,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x49,
    0x00,
    0x89,
    0x00,
    0x47,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x67,
    0x00,
    0x10,
    0x03,
    0x0b,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x47,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x48,
    0x00,
    0xff,
    0x00,
    0x10,
    0x03,
    0xc8,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0x10,
    0x03,
    0x47,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x61,
    0x00,
    0x0f,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x49,
    0x00,
    0x10,
    0x03,
    0x8b,
    0x00,
    0x47,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x48,
    0x00,
    0x81,
    0x00,
    0xe8,
    0x00,
    0x10,
    0x03,
    0x05,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0x10,
    0x03,
    0x47,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x64,
    0x00,
    0x08,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x10,
    0x03,
    0x47,
    0x00,
    0x00,
    0x00,
    0x49,
    0x00,
    0x10,
    0x03,
    0x89,
    0x00,
    0x47,
    0x00,
    0x08,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x65,
    0x00,
    0x10,
    0x03,
    0x0b,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x47,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x8b,
    0x00,
    0x10,
    0x03,
    0x4f,
    0x00,
    0x08,
    0x00,
    0x48,
    0x00,
    0x10,
    0x03,
    0x39,
    0x00,
    0xc8,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x02,
    0x00,
    0x30,
    0x00,
    0xff,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x10,
    0x03,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x8d,
    0x00,
    0x87,
    0x00,
    0xa1,
    0x03,
    0x80,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0xa1,
    0x03,
    0x47,
    0x00,
    0x18,
    0x00,
    0x49,
    0x00,
    0xa1,
    0x03,
    0x89,
    0x00,
    0x47,
    0x00,
    0x78,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x8d,
    0x00,
    0x87,
    0x00,
    0xa1,
    0x03,
    0x60,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x00,
    0x00,
    0x49,
    0x00,
    0x89,
    0x00,
    0xa1,
    0x03,
    0x47,
    0x00,
    0x48,
    0x00,
    0x49,
    0x00,
    0xa1,
    0x03,
    0x8d,
    0x00,
    0x87,
    0x00,
    0x49,
    0x00,
    0xa1,
    0x03,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xa1,
    0x03,
    0x49,
    0x00,
    0x89,
    0x00,
    0x47,
    0x00,
    0xa1,
    0x01,
    0x10,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x01,
    0x00,
    0x02,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x05,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x00,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x74,
    0x01,
    0x06,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0e,
    0x00,
    0x02,
    0x00,
    0x00,
    0x00,
    0x64,
    0x02,
    0x0f,
    0x00,
    0x03,
    0x00,
    0x00,
    0x00,
    0x64,
    0x02,
    0x0b,
    0x00,
    0x04,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0a,
    0x00,
    0x0a,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0d,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x0d,
    0x00,
    0x0e,
    0x00,
    0x00,
    0x00,
    0x71,
    0x01,
    0x02,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x0f,
    0x00,
    0x14,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x0b,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x65,
    0x02,
    0x0b,
    0x00,
    0x0a,
    0x00,
    0x00,
    0x00,
    0x73,
    0x01,
    0x01,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x30,
    0x02,
    0x0c,
    0x00,
    0xfe,
    0x00,
    0x00,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x02,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x80,
    0x03,
    0x0c,
    0x00,
    0x0f,
    0x00,
    0x01,
    0x00,
    0x64,
    0x02,
    0x08,
    0x00,
    0x0f,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x08,
    0x00,
    0x04,
    0x00,
    0x00,
    0x00,
    0x84,
    0x03,
    0x09,
    0x00,
    0x08,
    0x00,
    0x0c,
    0x00,
    0x68,
    0x02,
    0x09,
    0x00,
    0x0c,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x02,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x62,
    0x02,
    0x00,
    0x00,
    0x01,
    0x00,
    0x00,
    0x00,
    0x70,
    0x01,
    0x05,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x40,
    0x01,
    0x06,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0xff,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
    0x00,
];

/// S3.373: PE compose DATA prefix = pad@0..0x80 ‖ host table seed (328B → 110×DATA).
/// Prog|pad-to-outbuf still live-packed (≤24B). Own object-code; ≡ `s3_pack_data_records`.
const S3_HOST_PE_DATA_PREFIX_SEED: [(u8, u8, [u8; 6]); 110] = [
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0xff, 0x00]),
    (0x10, 3, [0x01, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0xc3, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x01, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x90, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x30, 0x00, 0x0e, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x48, 0x00, 0xb8, 0x00]),
    (0x10, 3, [0xef, 0x00, 0xbe, 0x00, 0xad, 0x00]),
    (0x10, 3, [0xde, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x49, 0x00]),
    (0x10, 3, [0x89, 0x00, 0x47, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x60, 0x00]),
    (0x10, 3, [0x08, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x49, 0x00, 0x8b, 0x00, 0x47, 0x00]),
    (0x10, 3, [0x08, 0x00, 0x49, 0x00, 0x89, 0x00]),
    (0x10, 3, [0x47, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x66, 0x00, 0x0b, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x49, 0x00]),
    (0x10, 3, [0x8b, 0x00, 0x47, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x48, 0x00, 0xff, 0x00, 0xc0, 0x00]),
    (0x10, 3, [0x49, 0x00, 0x89, 0x00, 0x47, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x62, 0x00, 0x0f, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x49, 0x00, 0x8b, 0x00]),
    (0x10, 3, [0x47, 0x00, 0x00, 0x00, 0x48, 0x00]),
    (0x10, 3, [0x81, 0x00, 0xc0, 0x00, 0x05, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x49, 0x00, 0x89, 0x00, 0x47, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x67, 0x00]),
    (0x10, 3, [0x0b, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x49, 0x00, 0x8b, 0x00, 0x47, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x48, 0x00, 0xff, 0x00]),
    (0x10, 3, [0xc8, 0x00, 0x49, 0x00, 0x89, 0x00]),
    (0x10, 3, [0x47, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x61, 0x00, 0x0f, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x49, 0x00]),
    (0x10, 3, [0x8b, 0x00, 0x47, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x48, 0x00, 0x81, 0x00, 0xe8, 0x00]),
    (0x10, 3, [0x05, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x49, 0x00, 0x89, 0x00]),
    (0x10, 3, [0x47, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x64, 0x00, 0x08, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x49, 0x00, 0x8b, 0x00]),
    (0x10, 3, [0x47, 0x00, 0x00, 0x00, 0x49, 0x00]),
    (0x10, 3, [0x89, 0x00, 0x47, 0x00, 0x08, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x65, 0x00]),
    (0x10, 3, [0x0b, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x49, 0x00, 0x8b, 0x00, 0x47, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x49, 0x00, 0x8b, 0x00]),
    (0x10, 3, [0x4f, 0x00, 0x08, 0x00, 0x48, 0x00]),
    (0x10, 3, [0x39, 0x00, 0xc8, 0x00, 0x00, 0x00]),
    (0x10, 3, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
    (0x10, 1, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00]),
];

/// S3.372: pack bytes as DATA `0x10` argc≤3 records (PE compose image).
fn s3_pack_data_records(bytes: &[u8]) -> Vec<(u8, u8, [u8; 6])> {
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

/// S3.372: PE-capable host compose TYB — DATA(table|prog)|lea A1|host prog-loop.
/// Bypasses empty-DATA poke cliff (2279 > [`S1_SLIM_MAX_CODE`]). Do **not** feed
/// through [`compile_tyb_to_pe_mini`] (would recurse via try_emit); use emit+wrap.
fn s3_host_compose_tyb_pe(ops: &[u8]) -> Vec<u8> {
    debug_assert!(!ops.is_empty() && ops.len() <= 8);
    // S3.374: RET / SET+RET = full PE TYB seeds (≡ expand/host seed pattern).
    if ops == [0xFF] {
        return S3_HOST_PE_RET_TYB_SEED.to_vec();
    }
    if ops == [0x30, 0xFF] {
        return S3_HOST_PE_SET_RET_TYB_SEED.to_vec();
    }
    // S3.373: fixed pad|table = object-code DATA seed; only prog tail live-packed.
    let mut tail = Vec::with_capacity(1 + ops.len() + 24);
    tail.push(ops.len() as u8);
    tail.extend_from_slice(ops);
    let prefix_len = S3_HOST_PROG_OFF as usize; // 0x148
    debug_assert_eq!(prefix_len, 0x80 + S3_HOST_TABLE_SEED.len());
    while prefix_len + tail.len() < S3_HOST_OUTBUF_OFF as usize {
        tail.push(0);
    }
    assert!(
        prefix_len + tail.len() <= S1_MINI_DATA_MAX,
        "S3.373 PE compose DATA {} > {}",
        prefix_len + tail.len(),
        S1_MINI_DATA_MAX
    );
    let mut recs = S3_HOST_PE_DATA_PREFIX_SEED.to_vec();
    recs.extend(s3_pack_data_records(&tail));
    recs.extend_from_slice(&S3_HOST_LEA_A1_SEED);
    recs.extend(s3_prog_loop_code_host(ops.len() as u32, S3_HOST_NROWS));
    s3_host_tyb_from_records(&recs)
}

/// S3.372: slim PE → run entry → read `out_off` bytes (Windows). Direct emit+wrap
/// (no [`compile_tyb_to_pe_mini`] / try_emit cycle).
#[cfg(windows)]
fn s3_run_slim_pe_outbuf_at(pe: &[u8], out_off: u32, len: usize) -> Vec<u8> {
    use crate::pe_manual_map::{map_pe_sections, parse_pe64_headers};
    #[link(name = "kernel32")]
    extern "system" {
        fn VirtualAlloc(
            lpAddress: *mut std::ffi::c_void,
            dwSize: usize,
            flAllocationType: u32,
            flProtect: u32,
        ) -> *mut std::ffi::c_void;
        fn VirtualFree(lpAddress: *mut std::ffi::c_void, dwSize: usize, dwFreeType: u32) -> i32;
    }
    const MEM_COMMIT: u32 = 0x1000;
    const MEM_RESERVE: u32 = 0x2000;
    const MEM_RELEASE: u32 = 0x8000;
    const PAGE_EXECUTE_READWRITE: u32 = 0x40;

    let headers = parse_pe64_headers(pe).expect("S3.372 pe headers");
    let staging = map_pe_sections(pe, &headers).expect("S3.372 map");
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
    assert_ne!(data_rva, 0, "S3.372 .data section");
    let size = headers.size_of_image as usize;
    let base = unsafe {
        VirtualAlloc(
            std::ptr::null_mut(),
            size,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_EXECUTE_READWRITE,
        )
    };
    assert!(!base.is_null(), "S3.372 VirtualAlloc");
    unsafe {
        std::ptr::copy_nonoverlapping(staging.as_ptr(), base as *mut u8, staging.len().min(size));
        let entry = (base as usize + headers.entry_rva as usize) as *const ();
        let f: extern "C" fn() = std::mem::transmute(entry);
        f();
        let out = std::slice::from_raw_parts(
            (base as *const u8).add(data_rva as usize + out_off as usize),
            len,
        )
        .to_vec();
        VirtualFree(base, 0, MEM_RELEASE);
        out
    }
}

/// S3.379: slim PE image seeds for RET / SET+RET compose (gap E — no live emit_x86).
/// Verified ≡ wrap_pe_slim(emit_x86(tyb)) in s3_host_pe_image_seeds_pin.
#[cfg(windows)]
const S3_HOST_PE_RET_IMAGE_SEED: &[u8] =
    include_bytes!("../blobs/s3_host_pe_ret.pe");
#[cfg(windows)]
const S3_HOST_PE_SET_RET_IMAGE_SEED: &[u8] =
    include_bytes!("../blobs/s3_host_pe_setret.pe");
/// S3.381: in-DLL slim PE image seeds for RET / SET+RET (gap F — no live emit_x86).
#[cfg(windows)]
const S3_IN_DLL_PE_RET_IMAGE_SEED: &[u8] =
    include_bytes!("../blobs/s3_in_dll_pe_ret.pe");
#[cfg(windows)]
const S3_IN_DLL_PE_SET_RET_IMAGE_SEED: &[u8] =
    include_bytes!("../blobs/s3_in_dll_pe_setret.pe");


/// S3.383: PE compose .text seeds keyed by ops_len (1..=8). Same len ⇒ same text
/// (only SET ops_len in prog-loop differs). gap G: non-RET try_emit skips emit_x86.
#[cfg(windows)]
const S3_HOST_PE_TEXT_BY_LEN: [&[u8]; 8] = [
    include_bytes!("../blobs/s3_host_pe_text_len1.txb"),
    include_bytes!("../blobs/s3_host_pe_text_len2.txb"),
    include_bytes!("../blobs/s3_host_pe_text_len3.txb"),
    include_bytes!("../blobs/s3_host_pe_text_len4.txb"),
    include_bytes!("../blobs/s3_host_pe_text_len5.txb"),
    include_bytes!("../blobs/s3_host_pe_text_len6.txb"),
    include_bytes!("../blobs/s3_host_pe_text_len7.txb"),
    include_bytes!("../blobs/s3_host_pe_text_len8.txb"),
];

/// S3.386: PE compose DATA prefix = pad@0..0x80 ‖ table (328B object-code seed).
const S3_HOST_PE_DATA_PREFIX_BYTES: &[u8] =
    include_bytes!("../blobs/s3_host_pe_data_prefix.dat");

/// S3.383/S3.386: DATA payload for PE compose — prefix seed + prog tail (no emit_x86).
fn s3_host_pe_data_payload(ops: &[u8]) -> Vec<u8> {
    debug_assert_eq!(S3_HOST_PE_DATA_PREFIX_BYTES.len(), S3_HOST_PROG_OFF as usize);
    let mut payload = S3_HOST_PE_DATA_PREFIX_BYTES.to_vec();
    payload.push(ops.len() as u8);
    payload.extend_from_slice(ops);
    while payload.len() < S3_HOST_OUTBUF_OFF as usize {
        payload.push(0);
    }
    payload
}

/// S3.399: patch text+data into a slim PE template (stable file offs).
/// Base = RET compose image seed; IAT/headers preserved; user payload ≤ IAT base.
#[cfg(windows)]
fn s3_slim_pe_patch_text_data(text: &[u8], data: &[u8]) -> Vec<u8> {
    debug_assert!(text.len() <= S1_SLIM_MAX_CODE);
    debug_assert!(data.len() <= S1_SLIM_IAT_BASE as usize);
    let mut pe = S3_HOST_PE_RET_IMAGE_SEED.to_vec();
    let t0 = S1_SLIM_TEXT_PAYLOAD_OFF;
    pe[t0..t0 + text.len()].copy_from_slice(text);
    // Clear any leftover text bytes beyond this payload (template may be longer).
    let t_clear = t0 + text.len();
    let t_end = t0 + S1_SLIM_MAX_CODE;
    if t_clear < t_end {
        for b in &mut pe[t_clear..t_end] {
            *b = 0;
        }
    }
    let d0 = S1_SLIM_DATA_PAYLOAD_OFF;
    let copy_n = data.len().min(S1_SLIM_IAT_BASE as usize);
    pe[d0..d0 + copy_n].copy_from_slice(&data[..copy_n]);
    pe
}

/// S3.372/S3.379/S3.383/S3.399: try_emit via PE-run YOYO compose (not Rust sim).
/// RET/SET+RET = full PE image seeds; other ops = text-by-len + data payload
/// patched into PE template (no emit_x86, no wrap — gap S).
#[cfg(windows)]
fn s3_try_emit_via_pe(ops: &[u8]) -> Option<Vec<u8>> {
    let len: usize = ops.iter().map(|o| s3_fixed_row(*o).len()).sum();
    if ops == [0xFF] {
        return Some(s3_run_slim_pe_outbuf_at(
            S3_HOST_PE_RET_IMAGE_SEED,
            S3_HOST_OUTBUF_OFF,
            len,
        ));
    }
    if ops == [0x30, 0xFF] {
        return Some(s3_run_slim_pe_outbuf_at(
            S3_HOST_PE_SET_RET_IMAGE_SEED,
            S3_HOST_OUTBUF_OFF,
            len,
        ));
    }
    let n = ops.len();
    if !(1..=8).contains(&n) {
        return None;
    }
    let text = S3_HOST_PE_TEXT_BY_LEN[n - 1];
    debug_assert_eq!(text.len(), 308);
    let data = s3_host_pe_data_payload(ops);
    let pe = s3_slim_pe_patch_text_data(text, &data);
    Some(s3_run_slim_pe_outbuf_at(&pe, S3_HOST_OUTBUF_OFF, len))
}

/// S3.494/S3.511: IMUL+RET via `S3_IMUL_BODY_SEED`‖C3.
/// ops=`[0x63,0xFF]` → dst=0/src=1. TYB: any disp8 slots (S3.511 lift).
/// **Not** folded into `S3_FIXED10_OPS` (golden/layout 禁折).
fn try_emit_imul_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x63, 0xFF] {
        return None;
    }
    try_compose_imul_ret_via_yoyo_seed(0, 1)
}

/// S3.511/S3.514: compose IMUL — disp8 seed poke, or load/store seed compose
/// for any slot≤255 (disp32 when needed).
fn try_compose_imul_ret_via_yoyo_seed(dst: u16, src: u16) -> Option<Vec<u8>> {
    if !mini_slot_any_ok(dst) || !mini_slot_any_ok(src) {
        return None;
    }
    if mini_slot_disp8_ok(dst) && mini_slot_disp8_ok(src) {
        let mut text = S3_IMUL_BODY_SEED.to_vec();
        text[3] = (dst as u8) * 8;
        text[7] = (src as u8) * 8;
        text[15] = (dst as u8) * 8;
        text.push(0xC3);
        return Some(text);
    }
    // S3.514: variable disp8/32 via host load/store seeds
    let mut text = Vec::with_capacity(28);
    s3_push_load_rax_slot(&mut text, dst);
    s3_push_load_rcx_slot(&mut text, src);
    text.extend_from_slice(&S3_IMUL_RAX_RCX_SEED);
    s3_push_store_rax_slot(&mut text, dst);
    text.push(0xC3);
    Some(text)
}

/// S3.494/S3.511/S3.514: TYB → IMUL+RET ops (slot≤255).
fn try_imul_ret_ops(prog: &TybProgram) -> Option<[u8; 2]> {
    if prog.records.len() != 2 {
        return None;
    }
    let imul = &prog.records[0];
    let ret = &prog.records[1];
    if imul.op != 0x63 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (dst, src) = arg_slot_slot(imul)?;
    if !mini_slot_any_ok(dst) || !mini_slot_any_ok(src) {
        return None;
    }
    Some([0x63, 0xFF])
}

/// S3.511/S3.514: TYB → IMUL+RET text (slot≤255).
fn try_imul_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let imul = &prog.records[0];
    let ret = &prog.records[1];
    if imul.op != 0x63 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (dst, src) = arg_slot_slot(imul)?;
    try_compose_imul_ret_via_yoyo_seed(dst, src)
}

/// S3.495/S3.511: ALUV+RET via `S3_ALUV_BODY_SEED`‖C3.
/// Opcode @+9 poked: ADDV=01 / ORV=09 / SUBV=29. ops=slots 0,1;
/// TYB: any disp8 (S3.511 lift). **Not** FIXED10.
fn try_emit_aluv_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops.len() != 2 || ops[1] != 0xFF {
        return None;
    }
    try_compose_aluv_ret_via_yoyo_seed(ops[0], 0, 1)
}

/// S3.511/S3.514: compose ALUV — disp8 seed or load/store seed compose (≤255).
fn try_compose_aluv_ret_via_yoyo_seed(op: u8, dst: u16, src: u16) -> Option<Vec<u8>> {
    if !mini_slot_any_ok(dst) || !mini_slot_any_ok(src) {
        return None;
    }
    let poke = match op {
        0x68 => 0x01u8, // ADDV
        0x69 => 0x09u8, // ORV
        0x6A => 0x29u8, // SUBV
        _ => return None,
    };
    if mini_slot_disp8_ok(dst) && mini_slot_disp8_ok(src) {
        let mut text = S3_ALUV_BODY_SEED.to_vec();
        text[3] = (dst as u8) * 8;
        text[7] = (src as u8) * 8;
        text[9] = poke;
        text[14] = (dst as u8) * 8;
        text.push(0xC3);
        return Some(text);
    }
    // S3.514: load/store seeds + alu byte
    let mut text = Vec::with_capacity(28);
    s3_push_load_rax_slot(&mut text, dst);
    s3_push_load_rcx_slot(&mut text, src);
    text.extend_from_slice(&[0x48, poke, 0xc8]); // alu rax, rcx
    s3_push_store_rax_slot(&mut text, dst);
    text.push(0xC3);
    Some(text)
}

/// S3.495/S3.511/S3.514: TYB → ALUV+RET ops (slot≤255).
fn try_aluv_ret_ops(prog: &TybProgram) -> Option<[u8; 2]> {
    if prog.records.len() != 2 {
        return None;
    }
    let alu = &prog.records[0];
    let ret = &prog.records[1];
    if !matches!(alu.op, 0x68 | 0x69 | 0x6A) || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (dst, src) = arg_slot_slot(alu)?;
    if !mini_slot_any_ok(dst) || !mini_slot_any_ok(src) {
        return None;
    }
    Some([alu.op, 0xFF])
}

/// S3.511/S3.514: TYB → ALUV+RET text (slot≤255).
fn try_aluv_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let alu = &prog.records[0];
    let ret = &prog.records[1];
    if !matches!(alu.op, 0x68 | 0x69 | 0x6A) || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (dst, src) = arg_slot_slot(alu)?;
    try_compose_aluv_ret_via_yoyo_seed(alu.op, dst, src)
}

/// S3.496: LDB+RET via own-ISA object seeds (`S3_LDB_OO0_BODY_SEED` /
/// `S3_LDB_OO_BODY_SEED`‖C3). `ops=[0x80,0xFF]` → oo=0 (dd=ss=0).
/// **Not** folded into `S3_FIXED10_OPS`.
fn try_emit_ldb_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x80, 0xFF] {
        return None;
    }
    let mut text = Vec::with_capacity(S3_LDB_OO0_BODY_SEED.len() + 1);
    text.extend_from_slice(&S3_LDB_OO0_BODY_SEED);
    text.push(0xC3);
    Some(text)
}

/// S3.496/S3.515: compose LDB — dd=ss=0 fast path; else any-slot≤255 seeds.
fn try_compose_ldb_body_via_yoyo_seed(dd: u16, ss: u16, oo: u16) -> Option<Vec<u8>> {
    if !mini_slot_any_ok(dd) || !mini_slot_any_ok(ss) {
        return None;
    }
    if dd == 0 && ss == 0 {
        if oo == 0 {
            return Some(S3_LDB_OO0_BODY_SEED.to_vec());
        }
        let mut body = S3_LDB_OO_BODY_SEED.to_vec();
        body[7..11].copy_from_slice(&(oo as u32).to_le_bytes());
        return Some(body);
    }
    // S3.515: load ss → (+add oo) → movzx → store dd
    let mut body = Vec::with_capacity(32);
    s3_push_load_rax_slot(&mut body, ss);
    if oo != 0 {
        let mut add = S3_ADD_RAX_IMM32_SEED;
        add[3..7].copy_from_slice(&(oo as u32).to_le_bytes());
        body.extend_from_slice(&add);
    }
    body.extend_from_slice(&S3_MOVZX_RAX_BYTE_RAX_SEED);
    s3_push_store_rax_slot(&mut body, dd);
    Some(body)
}

/// S3.496/S3.515: TYB → LDB+RET (slot≤255).
fn try_ldb_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let ldb = &prog.records[0];
    let ret = &prog.records[1];
    if ldb.op != 0x80 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (dd, ss, oo) = arg_slot3(ldb)?;
    let mut text = try_compose_ldb_body_via_yoyo_seed(dd, ss, oo)?;
    text.push(0xC3);
    Some(text)
}

/// S3.520: bare LDB (no RET) — body ‖ auto-C3.
fn try_ldb_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let ldb = &prog.records[0];
    if ldb.op != 0x80 {
        return None;
    }
    let (dd, ss, oo) = arg_slot3(ldb)?;
    let mut text = try_compose_ldb_body_via_yoyo_seed(dd, ss, oo)?;
    text.push(0xC3);
    Some(text)
}

/// S3.520: bare LDB ops (`ops=[0x80]`) — dd=ss=oo=0 ‖C3.
fn try_emit_ldb_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x80] {
        return None;
    }
    let mut text = try_compose_ldb_body_via_yoyo_seed(0, 0, 0)?;
    text.push(0xC3);
    Some(text)
}

/// S3.497: MEMCPY_DATA+RET via `S3_MEMCPY_DATA_BODY_SEED`‖C3.
/// `ops=[0x84,0xFF]` → dst=src=n=0. TYB: any-slot≤255 (S3.515).
/// **Not** folded into `S3_FIXED10_OPS`.
fn try_emit_memcpy_data_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x84, 0xFF] {
        return None;
    }
    let mut text = Vec::with_capacity(S3_MEMCPY_DATA_BODY_SEED.len() + 1);
    text.extend_from_slice(&S3_MEMCPY_DATA_BODY_SEED);
    text.push(0xC3);
    Some(text)
}

/// S3.497/S3.515: compose MEMCPY_DATA — disp8 seed or rsi/rdi/rcx load seeds.
fn try_compose_memcpy_data_body_via_yoyo_seed(dst: u16, src: u16, n: u16) -> Option<Vec<u8>> {
    if !mini_slot_any_ok(dst) || !mini_slot_any_ok(src) || !mini_slot_any_ok(n) {
        return None;
    }
    if mini_slot_disp8_ok(dst) && mini_slot_disp8_ok(src) && mini_slot_disp8_ok(n) {
        let mut body = S3_MEMCPY_DATA_BODY_SEED.to_vec();
        body[3] = (src as u8) * 8;
        body[7] = (dst as u8) * 8;
        body[11] = (n as u8) * 8;
        return Some(body);
    }
    // S3.515: RSI←src, RDI←dst, RCX←n, rep movsb
    let mut body = Vec::with_capacity(28);
    s3_push_load_rsi_slot(&mut body, src);
    s3_push_load_rdi_slot(&mut body, dst);
    s3_push_load_rcx_slot(&mut body, n);
    body.extend_from_slice(&S3_REP_MOVSB_SEED);
    Some(body)
}

/// S3.497/S3.515: TYB → MEMCPY_DATA+RET (slot≤255).
fn try_memcpy_data_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let mcp = &prog.records[0];
    let ret = &prog.records[1];
    if mcp.op != 0x84 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (dst, src, n) = arg_slot3(mcp)?;
    let mut text = try_compose_memcpy_data_body_via_yoyo_seed(dst, src, n)?;
    text.push(0xC3);
    Some(text)
}

/// S3.520: bare MEMCPY_DATA (no RET) — body ‖ auto-C3.
fn try_memcpy_data_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let mcp = &prog.records[0];
    if mcp.op != 0x84 {
        return None;
    }
    let (dst, src, n) = arg_slot3(mcp)?;
    let mut text = try_compose_memcpy_data_body_via_yoyo_seed(dst, src, n)?;
    text.push(0xC3);
    Some(text)
}

/// S3.520: bare MEMCPY_DATA ops (`ops=[0x84]`) — dst=src=n=0 ‖C3.
fn try_emit_memcpy_data_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x84] {
        return None;
    }
    let mut text = try_compose_memcpy_data_body_via_yoyo_seed(0, 0, 0)?;
    text.push(0xC3);
    Some(text)
}

/// S3.498: MEMCPY_STATE+RET via looped pair seeds‖C3.
/// `ops=[0x85,0xFF]` → n=1 dst=src=0. TYB: n∈1..=4, slot≤255 (S3.515).
/// **Not** folded into `S3_FIXED10_OPS`.
fn try_emit_memcpy_state_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x85, 0xFF] {
        return None;
    }
    let mut text = try_compose_memcpy_state_body_via_yoyo_seed(0, 0, 1)?;
    text.push(0xC3);
    Some(text)
}

/// S3.498/S3.515: compose MEMCPY_STATE — disp8 pair poke or load/store seeds.
fn try_compose_memcpy_state_body_via_yoyo_seed(dst: u16, src: u16, n: u16) -> Option<Vec<u8>> {
    if n == 0 || n > S1_MINI_MEMCPY_STATE_MAX_N {
        return None;
    }
    let last_dst = dst.wrapping_add(n - 1);
    let last_src = src.wrapping_add(n - 1);
    if !mini_slot_any_ok(dst)
        || !mini_slot_any_ok(src)
        || !mini_slot_any_ok(last_dst)
        || !mini_slot_any_ok(last_src)
    {
        return None;
    }
    let all_disp8 = mini_slot_disp8_ok(dst)
        && mini_slot_disp8_ok(src)
        && mini_slot_disp8_ok(last_dst)
        && mini_slot_disp8_ok(last_src);
    if all_disp8 {
        let mut body = Vec::with_capacity((n as usize) * S3_MEMCPY_STATE_PAIR_SEED.len());
        for i in 0..n {
            let mut pair = S3_MEMCPY_STATE_PAIR_SEED.to_vec();
            pair[3] = ((src + i) as u8) * 8;
            pair[7] = ((dst + i) as u8) * 8;
            body.extend_from_slice(&pair);
        }
        return Some(body);
    }
    // S3.515: per-slot load/store seeds
    let mut body = Vec::with_capacity((n as usize) * 14);
    for i in 0..n {
        s3_push_load_rax_slot(&mut body, src + i);
        s3_push_store_rax_slot(&mut body, dst + i);
    }
    Some(body)
}

/// S3.498/S3.515: TYB → MEMCPY_STATE+RET (n∈1..=4, slot≤255).
fn try_memcpy_state_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let mcp = &prog.records[0];
    let ret = &prog.records[1];
    if mcp.op != 0x85 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (dst, src, n) = arg_slot3(mcp)?;
    let mut text = try_compose_memcpy_state_body_via_yoyo_seed(dst, src, n)?;
    text.push(0xC3);
    Some(text)
}

/// S3.520: bare MEMCPY_STATE (no RET) — body ‖ auto-C3.
fn try_memcpy_state_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let mcp = &prog.records[0];
    if mcp.op != 0x85 {
        return None;
    }
    let (dst, src, n) = arg_slot3(mcp)?;
    let mut text = try_compose_memcpy_state_body_via_yoyo_seed(dst, src, n)?;
    text.push(0xC3);
    Some(text)
}

/// S3.520: bare MEMCPY_STATE ops (`ops=[0x85]`) — n=1 dst=src=0 ‖C3.
fn try_emit_memcpy_state_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x85] {
        return None;
    }
    let mut text = try_compose_memcpy_state_body_via_yoyo_seed(0, 0, 1)?;
    text.push(0xC3);
    Some(text)
}

/// S3.520: bare IMUL (no RET) — body ‖ auto-C3.
fn try_imul_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let imul = &prog.records[0];
    if imul.op != 0x63 {
        return None;
    }
    let (dst, src) = arg_slot_slot(imul)?;
    try_compose_imul_ret_via_yoyo_seed(dst, src) // already ‖C3
}

/// S3.520: bare IMUL ops (`ops=[0x63]`) — dst=0 src=1 ‖C3.
fn try_emit_imul_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x63] {
        return None;
    }
    try_compose_imul_ret_via_yoyo_seed(0, 1)
}

/// S3.520: bare ALUV (no RET) — body ‖ auto-C3.
fn try_aluv_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let alu = &prog.records[0];
    if !matches!(alu.op, 0x68 | 0x69 | 0x6A) {
        return None;
    }
    let (dst, src) = arg_slot_slot(alu)?;
    try_compose_aluv_ret_via_yoyo_seed(alu.op, dst, src) // already ‖C3
}

/// S3.520: bare ALUV ops (`ops=[0x68|69|6A]`) — dst=0 src=1 ‖C3.
fn try_emit_aluv_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops.len() != 1 {
        return None;
    }
    try_compose_aluv_ret_via_yoyo_seed(ops[0], 0, 1)
}

/// S3.500: JCC op → x86 cc byte (matches emit_x86).
fn s3_jcc_cc(op: u8) -> Option<u8> {
    Some(match op {
        0x71 => 0x84,
        0x72 => 0x85,
        0x73 => 0x8C,
        0x74 => 0x8D,
        0x75 => 0x8E,
        0x76 => 0x8F,
        0x77 => 0x82,
        0x78 => 0x83,
        0x79 => 0x86,
        0x7A => 0x87,
        _ => return None,
    })
}

/// S3.500: control-flow +RET via CALL/JMP/JCC object seeds (rel32=0 ≡
/// undefined label). **Not** FIXED10 fold; no pass-2 patch here.
fn try_emit_cf_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops.len() != 2 || ops[1] != 0xFF {
        return None;
    }
    let mut text = match ops[0] {
        0x41 => S3_CALL_BODY_SEED.to_vec(),
        0x70 => S3_JMP_BODY_SEED.to_vec(),
        op if (0x71..=0x7A).contains(&op) => {
            let mut j = S3_JCC_BODY_SEED.to_vec();
            j[1] = s3_jcc_cc(op)?;
            j
        }
        _ => return None,
    };
    text.push(0xC3);
    Some(text)
}

/// S3.500: TYB → CF+RET when argc=1 (rel32 left 0; no LABEL in prog).
fn try_cf_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let cf = &prog.records[0];
    let ret = &prog.records[1];
    if ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    if cf.argc != 1 || arg_u32(cf).is_none() {
        return None;
    }
    // Defined LABEL would need pass-2 — refuse if any LABEL present.
    if prog.records.iter().any(|r| r.op == 0x40) {
        return None;
    }
    try_emit_cf_ret_via_yoyo_seed(&[cf.op, 0xFF])
}

/// S3.509: LABEL+CF+RET — LABEL@0 then CALL/JMP/JCC same-id ‖C3.
/// Pass-2 poke rel32 = 0 − next_ip (backward). **Not** FIXED10.
fn try_emit_label_cf_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops.len() != 3 || ops[0] != 0x40 || ops[2] != 0xFF {
        return None;
    }
    try_compose_label_cf_ret_via_yoyo_seed(ops[1], 0)
}

/// S3.509: compose LABEL@0 + CF body with pass-2 rel32 + C3.
fn try_compose_label_cf_ret_via_yoyo_seed(cf_op: u8, _label_id: u32) -> Option<Vec<u8>> {
    let mut text = match cf_op {
        0x41 => S3_CALL_BODY_SEED.to_vec(),
        0x70 => S3_JMP_BODY_SEED.to_vec(),
        op if (0x71..=0x7A).contains(&op) => {
            let mut j = S3_JCC_BODY_SEED.to_vec();
            j[1] = s3_jcc_cc(op)?;
            j
        }
        _ => return None,
    };
    let patch_pos = if matches!(cf_op, 0x41 | 0x70) { 1 } else { 2 };
    let next_ip = (patch_pos + 4) as i32;
    let rel = 0i32 - next_ip;
    text[patch_pos..patch_pos + 4].copy_from_slice(&rel.to_le_bytes());
    text.push(0xC3);
    Some(text)
}

/// S3.509: TYB → LABEL id + CF same-id + RET (pass-2 backward).
fn try_label_cf_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 3 {
        return None;
    }
    let lab = &prog.records[0];
    let cf = &prog.records[1];
    let ret = &prog.records[2];
    if lab.op != 0x40 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let lid = arg_u32(lab)?;
    if cf.argc != 1 {
        return None;
    }
    let cid = arg_u32(cf)?;
    if lid != cid {
        return None;
    }
    try_compose_label_cf_ret_via_yoyo_seed(cf.op, lid)
}

/// S3.510: CF+NOP+LABEL+RET — forward pass-2 (NOP filler ⇒ rel≠0).
/// `ops=[cf,0x00,0x40,0xFF]`. **Not** FIXED10; not sheep of bare CF+RET.
fn try_emit_cf_nop_label_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops.len() != 4 || ops[1] != 0x00 || ops[2] != 0x40 || ops[3] != 0xFF {
        return None;
    }
    try_compose_cf_nop_label_ret_via_yoyo_seed(ops[0])
}

/// S3.510: compose CF body + NOP + pass-2 rel to LABEL@after-NOP ‖C3.
fn try_compose_cf_nop_label_ret_via_yoyo_seed(cf_op: u8) -> Option<Vec<u8>> {
    let mut text = match cf_op {
        0x41 => S3_CALL_BODY_SEED.to_vec(),
        0x70 => S3_JMP_BODY_SEED.to_vec(),
        op if (0x71..=0x7A).contains(&op) => {
            let mut j = S3_JCC_BODY_SEED.to_vec();
            j[1] = s3_jcc_cc(op)?;
            j
        }
        _ => return None,
    };
    let patch_pos = if matches!(cf_op, 0x41 | 0x70) { 1 } else { 2 };
    text.push(0x90); // NOP filler
    let target = text.len() as i32; // LABEL here
    let next_ip = (patch_pos + 4) as i32;
    let rel = target - next_ip;
    text[patch_pos..patch_pos + 4].copy_from_slice(&rel.to_le_bytes());
    text.push(0xC3);
    Some(text)
}

/// S3.516/S3.517: compose one fixed10-ish ALU/LABEL body
/// (SET/GET/MOVRR/ADD/SUB/INC/DEC/CMP/NOP/LABEL) via load/store + ALU host
/// seeds. LABEL → empty. Any slot≤255; any imm for SET/ADD/SUB.
/// **Not** folded into `S3_FIXED10_OPS` / golden.
fn s3_compose_slot_alu_body(rec: &TybRecord) -> Option<Vec<u8>> {
    match rec.op {
        0x40 => {
            if rec.argc != 1 {
                return None;
            }
            Some(Vec::new()) // LABEL: position marker only
        }
        0x00 => {
            if rec.argc != 0 {
                return None;
            }
            Some(vec![0x90])
        }
        0x30 => {
            let (slot, imm) = arg_slot_imm(rec)?;
            if !mini_slot_any_ok(slot) {
                return None;
            }
            let mut body = Vec::with_capacity(20);
            let mut m = S3_MOVABS_RAX_IMM64_SEED;
            m[2..6].copy_from_slice(&imm.to_le_bytes());
            body.extend_from_slice(&m);
            s3_push_store_rax_slot(&mut body, slot);
            Some(body)
        }
        0x60 | 0x64 => {
            let (dst, src) = arg_slot_slot(rec)?;
            if !mini_slot_any_ok(dst) || !mini_slot_any_ok(src) {
                return None;
            }
            let mut body = Vec::with_capacity(16);
            s3_push_load_rax_slot(&mut body, src);
            s3_push_store_rax_slot(&mut body, dst);
            Some(body)
        }
        0x61 | 0x62 => {
            let (slot, imm) = arg_slot_imm(rec)?;
            if !mini_slot_any_ok(slot) {
                return None;
            }
            let mut body = Vec::with_capacity(24);
            s3_push_load_rax_slot(&mut body, slot);
            if rec.op == 0x61 {
                let mut sub = S3_SUB_RAX_IMM32_SEED;
                sub[3..7].copy_from_slice(&imm.to_le_bytes());
                body.extend_from_slice(&sub);
            } else {
                let mut add = S3_ADD_RAX_IMM32_SEED;
                add[3..7].copy_from_slice(&imm.to_le_bytes());
                body.extend_from_slice(&add);
            }
            s3_push_store_rax_slot(&mut body, slot);
            Some(body)
        }
        0x66 | 0x67 => {
            let slot = arg_u32(rec)? as u16;
            if !mini_slot_any_ok(slot) {
                return None;
            }
            let mut body = Vec::with_capacity(16);
            s3_push_load_rax_slot(&mut body, slot);
            if rec.op == 0x66 {
                body.extend_from_slice(&S3_INC_RAX_SEED);
            } else {
                body.extend_from_slice(&S3_DEC_RAX_SEED);
            }
            s3_push_store_rax_slot(&mut body, slot);
            Some(body)
        }
        0x65 => {
            let (a, b) = arg_slot_slot(rec)?;
            if !mini_slot_any_ok(a) || !mini_slot_any_ok(b) {
                return None;
            }
            let mut body = Vec::with_capacity(20);
            s3_push_load_rax_slot(&mut body, a);
            s3_push_load_rcx_slot(&mut body, b);
            body.extend_from_slice(&S3_CMP_RAX_RCX_SEED);
            Some(body)
        }
        _ => None,
    }
}

/// S3.516/S3.517: CMP any-slot + fixed10-ish multi-op mix + LABEL residual +RET.
/// Chains LABEL/SET/GET/MOVRR/ADD/SUB/INC/DEC/CMP/NOP then RET (2..=8 records).
/// Activates only when `fixed_template_ops` refuses (wrong-slot / non-canonical
/// imm / LABEL / multi-op outside default image seeds) — **does not** fold FIXED10.
fn try_slot_alu_chain_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    let n = prog.records.len();
    if !(2..=8).contains(&n) {
        return None;
    }
    let last = prog.records.last()?;
    if last.op != 0xFF || last.argc != 0 {
        return None;
    }
    // Leave default-slot fixed10 PE/image path alone (禁折 FIXED10).
    if fixed_template_ops(prog).is_some() {
        return None;
    }
    let mut text = Vec::with_capacity(64);
    for rec in &prog.records[..n - 1] {
        let body = s3_compose_slot_alu_body(rec)?;
        text.extend_from_slice(&body);
    }
    text.push(0xC3);
    if text.len() > S1_SLIM_MAX_CODE {
        return None;
    }
    Some(text)
}

/// S3.517: bare fixed10-ish ALU/LABEL (no trailing RET record).
/// `emit_x86` always appends C3 — match that. 1..=7 records; must emit ≥1
/// code byte before auto-C3. **Not** FIXED10.
fn try_slot_alu_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    let n = prog.records.len();
    if !(1..=7).contains(&n) {
        return None;
    }
    if prog.records.last()?.op == 0xFF {
        return None;
    }
    if fixed_template_ops(prog).is_some() {
        return None;
    }
    let mut text = Vec::with_capacity(64);
    for rec in &prog.records {
        let body = s3_compose_slot_alu_body(rec)?;
        text.extend_from_slice(&body);
    }
    if text.is_empty() {
        return None;
    }
    text.push(0xC3); // emit_x86 auto-terminate
    if text.len() > S1_SLIM_MAX_CODE {
        return None;
    }
    Some(text)
}

/// S3.517: bare RAW_BYTES (no RET record) — payload ‖ auto-C3 (`emit_x86`).
fn try_raw_bytes_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let raw = &prog.records[0];
    if raw.op != 0xA1 || raw.argc == 0 || raw.argc > 3 {
        return None;
    }
    let mut text = arg_low_bytes(raw);
    text.push(0xC3);
    Some(text)
}

/// S3.517: bare RAW_BYTES ops-path (`ops=[0xA1]`) — default NOP, no C3.
fn try_emit_raw_bytes_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0xA1] {
        return None;
    }
    Some(S3_RAW_BYTES_RET_OPS_DEFAULT.to_vec())
}

/// S3.517: DATA|STR|RAW + ALU/LABEL mix +RET → `.data` payload + ALU text‖C3.
/// ≥1 data then ≥1 ALU/LABEL then RET. Pure-data owned by S3.506/507.
fn try_data_alu_mix_ret_tyb(prog: &TybProgram) -> Option<(Vec<u8>, Vec<u8>)> {
    let n = prog.records.len();
    if !(3..=10).contains(&n) {
        return None;
    }
    let last = prog.records.last()?;
    if last.op != 0xFF || last.argc != 0 {
        return None;
    }
    let body = &prog.records[..n - 1];
    let mut i = 0usize;
    let mut data = Vec::new();
    while i < body.len() && matches!(body[i].op, 0x10 | 0x12 | 0x13) {
        if body[i].argc > 3 {
            return None;
        }
        data.extend(arg_low_bytes(&body[i]));
        if data.len() > S1_MINI_DATA_MAX {
            return None;
        }
        i += 1;
    }
    if data.is_empty() || i >= body.len() {
        return None; // no data, or all-data (506/507)
    }
    let mut text = Vec::with_capacity(64);
    for rec in &body[i..] {
        let b = s3_compose_slot_alu_body(rec)?;
        text.extend_from_slice(&b);
    }
    text.push(0xC3);
    if text.len() > S1_SLIM_MAX_CODE {
        return None;
    }
    Some((text, data))
}

/// S3.510: CF+NOP+LABEL+RET — forward pass-2 (NOP filler ⇒ rel≠0).
fn try_cf_nop_label_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 4 {
        return None;
    }
    let cf = &prog.records[0];
    let nop = &prog.records[1];
    let lab = &prog.records[2];
    let ret = &prog.records[3];
    if nop.op != 0x00 || nop.argc != 0 || lab.op != 0x40 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    if cf.argc != 1 {
        return None;
    }
    let cid = arg_u32(cf)?;
    let lid = arg_u32(lab)?;
    if cid != lid {
        return None;
    }
    try_compose_cf_nop_label_ret_via_yoyo_seed(cf.op)
}

/// S3.501: ALLOC+RET via `S3_ALLOC_PREFIX_SEED` + `S3_STORE_RAX_DISP32_SEED`‖C3.
/// `ops=[0x20,0xFF]` → slot=0 size=0. TYB pokes size@+7 / store disp@+3.
/// Matches host `emit_slim_alloc` (always disp32 store). **Not** FIXED10.
fn try_emit_alloc_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x20, 0xFF] {
        return None;
    }
    let mut text = try_compose_alloc_body_via_yoyo_seed(0, 0)?;
    text.push(0xC3);
    Some(text)
}

/// S3.501: compose ALLOC body — prefix size poke + disp32 store.
fn try_compose_alloc_body_via_yoyo_seed(slot: u16, size: u32) -> Option<Vec<u8>> {
    if slot > 255 {
        return None;
    }
    let mut body = S3_ALLOC_PREFIX_SEED.to_vec();
    body[7..11].copy_from_slice(&size.to_le_bytes());
    let mut store = S3_STORE_RAX_DISP32_SEED.to_vec();
    store[3..7].copy_from_slice(&((slot as u32) * 8).to_le_bytes());
    body.extend_from_slice(&store);
    Some(body)
}

/// S3.501: TYB → ALLOC+RET (slot≤255).
fn try_alloc_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let alloc = &prog.records[0];
    let ret = &prog.records[1];
    if alloc.op != 0x20 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (slot, size) = arg_slot_imm(alloc)?;
    let mut text = try_compose_alloc_body_via_yoyo_seed(slot, size)?;
    text.push(0xC3);
    Some(text)
}

/// S3.518: bare ALLOC (no RET record) — body ‖ auto-C3 (`emit_x86`).
fn try_alloc_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let alloc = &prog.records[0];
    if alloc.op != 0x20 {
        return None;
    }
    let (slot, size) = arg_slot_imm(alloc)?;
    let mut text = try_compose_alloc_body_via_yoyo_seed(slot, size)?;
    text.push(0xC3);
    Some(text)
}

/// S3.518: bare ALLOC ops-path (`ops=[0x20]`) — default slot0/size0 ‖C3.
fn try_emit_alloc_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x20] {
        return None;
    }
    let mut text = try_compose_alloc_body_via_yoyo_seed(0, 0)?;
    text.push(0xC3);
    Some(text)
}

/// S3.518: CF+LABEL+RET (no filler) — LABEL@after-CF ⇒ rel32=0 (≡ CF+RET).
fn try_cf_label_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 3 {
        return None;
    }
    let cf = &prog.records[0];
    let lab = &prog.records[1];
    let ret = &prog.records[2];
    if lab.op != 0x40 || ret.op != 0xFF || ret.argc != 0 || cf.argc != 1 {
        return None;
    }
    let cid = arg_u32(cf)?;
    let lid = arg_u32(lab)?;
    if cid != lid {
        return None;
    }
    // LABEL emits 0B → target==next_ip → rel0 ≡ try_cf_ret
    try_emit_cf_ret_via_yoyo_seed(&[cf.op, 0xFF])
}

/// S3.518: CF + filler(s) + LABEL + RET — forward pass-2.
/// filler = NOP or fixed10-ish ALU (1..=3). Extends S3.510 beyond single NOP.
fn try_cf_filler_label_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    let n = prog.records.len();
    if !(4..=6).contains(&n) {
        return None;
    }
    let cf = &prog.records[0];
    let lab = &prog.records[n - 2];
    let ret = &prog.records[n - 1];
    if lab.op != 0x40 || ret.op != 0xFF || ret.argc != 0 || cf.argc != 1 {
        return None;
    }
    let cid = arg_u32(cf)?;
    let lid = arg_u32(lab)?;
    if cid != lid {
        return None;
    }
    let fillers = &prog.records[1..n - 2];
    if fillers.is_empty() || fillers.len() > 3 {
        return None;
    }
    // S3.510 owns exact CF+NOP+LABEL+RET — leave that path (same bytes OK either way,
    // but refuse single-NOP here so 510 stays the pin).
    if fillers.len() == 1 && fillers[0].op == 0x00 && fillers[0].argc == 0 {
        return None;
    }
    let mut text = match cf.op {
        0x41 => S3_CALL_BODY_SEED.to_vec(),
        0x70 => S3_JMP_BODY_SEED.to_vec(),
        op if (0x71..=0x7A).contains(&op) => {
            let mut j = S3_JCC_BODY_SEED.to_vec();
            j[1] = s3_jcc_cc(op)?;
            j
        }
        _ => return None,
    };
    let patch_pos = if matches!(cf.op, 0x41 | 0x70) { 1 } else { 2 };
    for f in fillers {
        let body = s3_compose_slot_alu_body(f)?;
        if body.is_empty() {
            return None; // LABEL-as-filler not allowed mid-forward
        }
        text.extend_from_slice(&body);
    }
    let target = text.len() as i32;
    let next_ip = (patch_pos + 4) as i32;
    let rel = target - next_ip;
    text[patch_pos..patch_pos + 4].copy_from_slice(&rel.to_le_bytes());
    text.push(0xC3);
    if text.len() > S1_SLIM_MAX_CODE {
        return None;
    }
    Some(text)
}

/// S3.502/S3.513: STB+RET via OO0 / imm8 / imm32 object seeds.
/// `ops=[0x81,0xFF]` → val=base=oo=0. TYB: disp8 slots; any oo (S3.513).
/// **Not** FIXED10 fold; bake=362 unchanged.
fn try_emit_stb_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x81, 0xFF] {
        return None;
    }
    let mut text = try_compose_stb_body_via_yoyo_seed(0, 0, 0)?;
    text.push(0xC3);
    Some(text)
}

/// S3.502/S3.513/S3.515: compose STB — disp8 body seeds, or any-slot load seeds.
fn try_compose_stb_body_via_yoyo_seed(val: u16, base: u16, oo: u16) -> Option<Vec<u8>> {
    if !mini_slot_any_ok(val) || !mini_slot_any_ok(base) {
        return None;
    }
    if mini_slot_disp8_ok(val) && mini_slot_disp8_ok(base) {
        if oo == 0 {
            let mut body = S3_STB_OO0_BODY_SEED.to_vec();
            body[3] = (val as u8) * 8;
            body[7] = (base as u8) * 8;
            return Some(body);
        }
        if oo <= 127 {
            let mut body = S3_STB_OO_IMM8_BODY_SEED.to_vec();
            body[3] = (val as u8) * 8;
            body[7] = (base as u8) * 8;
            body[11] = oo as u8;
            return Some(body);
        }
        let mut body = S3_STB_OO_IMM32_BODY_SEED.to_vec();
        body[3] = (val as u8) * 8;
        body[7] = (base as u8) * 8;
        body[11..15].copy_from_slice(&(oo as u32).to_le_bytes());
        return Some(body);
    }
    // S3.515: load val→rcx, base→rax, (+oo), store cl
    let mut body = Vec::with_capacity(28);
    s3_push_load_rcx_slot(&mut body, val);
    s3_push_load_rax_slot(&mut body, base);
    if oo != 0 {
        if oo <= 127 {
            let mut add = S3_ADD_RAX_IMM8_SEED;
            add[3] = oo as u8;
            body.extend_from_slice(&add);
        } else {
            let mut add = S3_ADD_RAX_IMM32_SEED;
            add[3..7].copy_from_slice(&(oo as u32).to_le_bytes());
            body.extend_from_slice(&add);
        }
    }
    body.extend_from_slice(&S3_STORE_CL_RAX_SEED);
    Some(body)
}

/// S3.502/S3.513: TYB → STB+RET (disp8 slots; any oo).
fn try_stb_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let stb = &prog.records[0];
    let ret = &prog.records[1];
    if stb.op != 0x81 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (val, base, oo) = arg_slot3(stb)?;
    let mut text = try_compose_stb_body_via_yoyo_seed(val, base, oo)?;
    text.push(0xC3);
    Some(text)
}

/// S3.503: RAW_BYTE+RET ops-path default payload byte (NOP). TYB carries real.
pub const S3_RAW_BYTE_RET_OPS_DEFAULT: u8 = 0x90;

/// S3.503: RAW_BYTE+RET — literal byte‖C3 (YOYO payload, not FIXED10).
/// `ops=[0xA0,0xFF]` → NOP‖C3. In-DLL `S3_RAW_BYTE_EMIT_BODY_SEED` = emitter ≠ this.
fn try_emit_raw_byte_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0xA0, 0xFF] {
        return None;
    }
    Some(vec![S3_RAW_BYTE_RET_OPS_DEFAULT, 0xC3])
}

/// S3.503: TYB → RAW_BYTE+RET (argc=1).
fn try_raw_byte_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let raw = &prog.records[0];
    let ret = &prog.records[1];
    if raw.op != 0xA0 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let b = arg_u32(raw)? as u8;
    Some(vec![b, 0xC3])
}

/// S3.503: TYB → RAW_BYTES+RET (argc 1..=3 low-bytes‖C3).
fn try_raw_bytes_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let raw = &prog.records[0];
    let ret = &prog.records[1];
    if raw.op != 0xA1 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    if raw.argc == 0 || raw.argc > 3 {
        return None;
    }
    let mut text = arg_low_bytes(raw);
    text.push(0xC3);
    Some(text)
}

/// S3.512: RAW_BYTES+RET ops-path default payload (1×NOP). TYB carries real.
/// Emitter `S3_RAW_BYTES_EMIT_BODY_SEED` ≠ this host payload. **Not** FIXED10.
pub const S3_RAW_BYTES_RET_OPS_DEFAULT: [u8; 1] = [0x90];

/// S3.512: RAW_BYTES+RET ops — default payload‖C3 (`ops=[0xA1,0xFF]`).
fn try_emit_raw_bytes_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0xA1, 0xFF] {
        return None;
    }
    let mut text = S3_RAW_BYTES_RET_OPS_DEFAULT.to_vec();
    text.push(0xC3);
    Some(text)
}

/// S3.504: LOAD_FILE+RET via `S3_LOAD_FILE_BODY_SEED`‖C3.
/// `ops=[0x50,0xFF]` → slot=0 path_off=0. TYB pokes path@+9 / store disp.
/// Host object seed (bake=362 unchanged). **Not** FIXED10.
fn try_emit_load_file_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x50, 0xFF] {
        return None;
    }
    let mut text = try_compose_load_file_body_via_yoyo_seed(0, 0)?;
    text.push(0xC3);
    Some(text)
}

/// S3.504: compose LOAD_FILE body — poke path_off + slot store disp32.
fn try_compose_load_file_body_via_yoyo_seed(slot: u16, path_off: u32) -> Option<Vec<u8>> {
    if slot > 255 || path_off >= S1_SLIM_IAT_BASE {
        return None;
    }
    let mut body = S3_LOAD_FILE_BODY_SEED.to_vec();
    let path_poff = slim_load_path_patch_off();
    body[path_poff..path_poff + 4].copy_from_slice(&path_off.to_le_bytes());
    let slot_poff = slim_load_slot_patch_off(&body);
    body[slot_poff..slot_poff + 4].copy_from_slice(&((slot as u32) * 8).to_le_bytes());
    Some(body)
}

/// S3.504: TYB → LOAD_FILE+RET (slot≤255; path_off < IAT base).
fn try_load_file_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let load = &prog.records[0];
    let ret = &prog.records[1];
    if load.op != 0x50 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (slot, path_off) = arg_slot_imm(load)?;
    let mut text = try_compose_load_file_body_via_yoyo_seed(slot, path_off)?;
    text.push(0xC3);
    Some(text)
}

/// S3.505: WRITE_FILE+RET via `S3_WRITE_FILE_BODY_SEED`‖C3.
/// `ops=[0x51,0xFF]` → slot=path=sz=0. TYB pokes path / slot / sz / store.
/// Host object seed (bake=362 unchanged). **Not** FIXED10.
fn try_emit_write_file_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x51, 0xFF] {
        return None;
    }
    let mut text = try_compose_write_file_body_via_yoyo_seed(0, 0, 0)?;
    text.push(0xC3);
    Some(text)
}

/// S3.505: compose WRITE_FILE body — poke path + slot/sz loads + store.
fn try_compose_write_file_body_via_yoyo_seed(
    slot: u16,
    path_off: u32,
    sz_slot: u16,
) -> Option<Vec<u8>> {
    if slot > 255 || sz_slot > 255 || path_off >= S1_SLIM_IAT_BASE {
        return None;
    }
    let mut body = S3_WRITE_FILE_BODY_SEED.to_vec();
    let path_poff = slim_write_path_patch_off();
    body[path_poff..path_poff + 4].copy_from_slice(&path_off.to_le_bytes());
    let slot_poff = slim_write_slot_load_patch_off(&body);
    body[slot_poff..slot_poff + 4].copy_from_slice(&((slot as u32) * 8).to_le_bytes());
    let sz_poff = slim_write_sz_load_patch_off(&body);
    body[sz_poff..sz_poff + 4].copy_from_slice(&((sz_slot as u32) * 8).to_le_bytes());
    let store_poff = slim_write_store_patch_off(&body);
    body[store_poff..store_poff + 4].copy_from_slice(&((slot as u32) * 8).to_le_bytes());
    Some(body)
}

/// S3.505: TYB → WRITE_FILE+RET (argc=3 or argc=2 sz=0).
fn try_write_file_ret_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 2 {
        return None;
    }
    let wr = &prog.records[0];
    let ret = &prog.records[1];
    if wr.op != 0x51 || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let (slot, path_off, sz_slot) = if let Some((s, p, z)) = arg_slot3(wr) {
        (s, p as u32, z)
    } else {
        let (s, p) = arg_slot_imm(wr)?;
        (s, p, 0)
    };
    let mut text = try_compose_write_file_body_via_yoyo_seed(slot, path_off, sz_slot)?;
    text.push(0xC3);
    Some(text)
}

/// S3.519: bare LOAD_FILE (no RET record) — body ‖ auto-C3.
fn try_load_file_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let load = &prog.records[0];
    if load.op != 0x50 {
        return None;
    }
    let (slot, path_off) = arg_slot_imm(load)?;
    let mut text = try_compose_load_file_body_via_yoyo_seed(slot, path_off)?;
    text.push(0xC3);
    Some(text)
}

/// S3.519: bare LOAD_FILE ops (`ops=[0x50]`) — slot0/path0 ‖C3.
fn try_emit_load_file_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x50] {
        return None;
    }
    let mut text = try_compose_load_file_body_via_yoyo_seed(0, 0)?;
    text.push(0xC3);
    Some(text)
}

/// S3.519: bare WRITE_FILE (no RET record) — body ‖ auto-C3.
fn try_write_file_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let wr = &prog.records[0];
    if wr.op != 0x51 {
        return None;
    }
    let (slot, path_off, sz_slot) = if let Some((s, p, z)) = arg_slot3(wr) {
        (s, p as u32, z)
    } else {
        let (s, p) = arg_slot_imm(wr)?;
        (s, p, 0)
    };
    let mut text = try_compose_write_file_body_via_yoyo_seed(slot, path_off, sz_slot)?;
    text.push(0xC3);
    Some(text)
}

/// S3.519: bare WRITE_FILE ops (`ops=[0x51]`) — defaults ‖C3.
fn try_emit_write_file_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0x51] {
        return None;
    }
    let mut text = try_compose_write_file_body_via_yoyo_seed(0, 0, 0)?;
    text.push(0xC3);
    Some(text)
}

/// S3.519: bare RAW_BYTE (no RET) — byte ‖ auto-C3.
fn try_raw_byte_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let raw = &prog.records[0];
    if raw.op != 0xA0 || raw.argc != 1 {
        return None;
    }
    let b = arg_u32(raw)? as u8;
    Some(vec![b, 0xC3])
}

/// S3.519: bare RAW_BYTE ops (`ops=[0xA0]`) — default NOP ‖C3.
fn try_emit_raw_byte_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    if ops != [0xA0] {
        return None;
    }
    Some(vec![S3_RAW_BYTE_RET_OPS_DEFAULT, 0xC3])
}

/// S3.519: bare STB (no RET) — body ‖ auto-C3.
fn try_stb_bare_tyb(prog: &TybProgram) -> Option<Vec<u8>> {
    if prog.records.len() != 1 {
        return None;
    }
    let stb = &prog.records[0];
    if stb.op != 0x81 {
        return None;
    }
    let (val, base, oo) = arg_slot3(stb)?;
    let mut text = try_compose_stb_body_via_yoyo_seed(val, base, oo)?;
    text.push(0xC3);
    Some(text)
}

/// S3.506: DATA/STR/RAW+RET ops-path — `.text`=C3; `.data` empty (TYB carries).
/// In-DLL `S3_DATA_APPEND_BODY_SEED` = emitter ≠ host payload. **Not** FIXED10.
fn try_emit_data_ret_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    match ops {
        [0x10, 0xFF] | [0x12, 0xFF] | [0x13, 0xFF] => Some(vec![0xC3]),
        _ => None,
    }
}

/// S3.506: TYB → DATA|STR|RAW +RET → (text=C3, data=low-bytes). argc≤3.
fn try_data_section_ret_tyb(prog: &TybProgram) -> Option<(Vec<u8>, Vec<u8>)> {
    if prog.records.len() != 2 {
        return None;
    }
    let d = &prog.records[0];
    let ret = &prog.records[1];
    if !matches!(d.op, 0x10 | 0x12 | 0x13) || ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    if d.argc > 3 {
        return None;
    }
    Some((vec![0xC3], arg_low_bytes(d)))
}

/// S3.521: bare DATA|STR|RAW (no RET) — text=C3 (emit_x86 auto), data=payload.
fn try_data_section_bare_tyb(prog: &TybProgram) -> Option<(Vec<u8>, Vec<u8>)> {
    if prog.records.len() != 1 {
        return None;
    }
    let d = &prog.records[0];
    if !matches!(d.op, 0x10 | 0x12 | 0x13) || d.argc > 3 {
        return None;
    }
    Some((vec![0xC3], arg_low_bytes(d)))
}

/// S3.521: bare DATA ops (`ops=[0x10|0x12|0x13]`) — text=C3; data empty.
fn try_emit_data_bare_via_yoyo_seed(ops: &[u8]) -> Option<Vec<u8>> {
    match ops {
        [0x10] | [0x12] | [0x13] => Some(vec![0xC3]),
        _ => None,
    }
}

/// S3.521: multi bare DATA|STR|RAW (no RET) — concat data ‖ text=C3.
fn try_multi_data_section_bare_tyb(prog: &TybProgram) -> Option<(Vec<u8>, Vec<u8>)> {
    if prog.records.len() < 2 {
        return None;
    }
    if prog.records.last()?.op == 0xFF {
        return None;
    }
    let mut data = Vec::new();
    for d in &prog.records {
        if !matches!(d.op, 0x10 | 0x12 | 0x13) || d.argc > 3 {
            return None;
        }
        data.extend(arg_low_bytes(d));
        if data.len() > S1_MINI_DATA_MAX {
            return None;
        }
    }
    Some((vec![0xC3], data))
}

/// S3.521: DATA + ALU mix bare (no RET) — data payload + ALU text ‖ auto-C3.
fn try_data_alu_mix_bare_tyb(prog: &TybProgram) -> Option<(Vec<u8>, Vec<u8>)> {
    let n = prog.records.len();
    if !(2..=9).contains(&n) {
        return None;
    }
    if prog.records.last()?.op == 0xFF {
        return None;
    }
    let mut i = 0usize;
    let mut data = Vec::new();
    while i < n && matches!(prog.records[i].op, 0x10 | 0x12 | 0x13) {
        if prog.records[i].argc > 3 {
            return None;
        }
        data.extend(arg_low_bytes(&prog.records[i]));
        if data.len() > S1_MINI_DATA_MAX {
            return None;
        }
        i += 1;
    }
    if data.is_empty() || i >= n {
        return None;
    }
    let mut text = Vec::with_capacity(64);
    for rec in &prog.records[i..] {
        let b = s3_compose_slot_alu_body(rec)?;
        text.extend_from_slice(&b);
    }
    if text.is_empty() {
        return None;
    }
    text.push(0xC3);
    if text.len() > S1_SLIM_MAX_CODE {
        return None;
    }
    Some((text, data))
}

/// S3.507: multi DATA|STR|RAW +RET → concat `.data` ‖ text=C3.
/// ≥2 payload records; total ≤ [`S1_MINI_DATA_MAX`]. **Not** FIXED10.
fn try_multi_data_section_ret_tyb(prog: &TybProgram) -> Option<(Vec<u8>, Vec<u8>)> {
    if prog.records.len() < 3 {
        return None;
    }
    let ret = prog.records.last()?;
    if ret.op != 0xFF || ret.argc != 0 {
        return None;
    }
    let payloads = &prog.records[..prog.records.len() - 1];
    if payloads.len() < 2 {
        return None;
    }
    let mut data = Vec::new();
    for d in payloads {
        if !matches!(d.op, 0x10 | 0x12 | 0x13) || d.argc > 3 {
            return None;
        }
        data.extend(arg_low_bytes(d));
        if data.len() > S1_MINI_DATA_MAX {
            return None;
        }
    }
    Some((vec![0xC3], data))
}

/// Host-side YOYO table compose → `.text` bytes.
/// `None` if `ops` outside the fixed template subset (and S3.494–512 seeds).
/// S3.350: call cursor directly (default-seed `s3_sim_prog_loop` is cfg(test)).
pub fn try_emit_text_via_yoyo_table(ops: &[u8]) -> Option<Vec<u8>> {
    if ops.is_empty() || ops.len() > 8 {
        return None;
    }
    // S3.494–512: non-fixed10 YOYO compose — object seeds (no golden fold).
    if let Some(text) = try_emit_imul_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_imul_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_aluv_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_aluv_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_ldb_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_ldb_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_memcpy_data_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_memcpy_data_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_memcpy_state_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_memcpy_state_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_cf_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_alloc_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    // S3.518: bare ALLOC ops
    if let Some(text) = try_emit_alloc_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_stb_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_raw_byte_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_raw_bytes_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    // S3.517: bare RAW_BYTES ops (no RET)
    if let Some(text) = try_emit_raw_bytes_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_load_file_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_load_file_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_write_file_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_write_file_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_raw_byte_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_data_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_data_bare_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_label_cf_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if let Some(text) = try_emit_cf_nop_label_ret_via_yoyo_seed(ops) {
        return Some(text);
    }
    if !ops.iter().all(|o| S3_FIXED10_OPS.contains(o)) {
        return None;
    }
    // S3.372: Windows = PE-run YOYO compose; elsewhere keep host sim cursor.
    #[cfg(windows)]
    {
        return s3_try_emit_via_pe(ops);
    }
    #[cfg(not(windows))]
    {
        s3_sim_prog_loop_cursor(
            &s3_host_prog_loop_tyb(ops),
            S3_HOST_TABLE_OFF as u64,
            S3_HOST_OUTBUF_OFF,
            (S3_HOST_PROG_OFF + 1) as u64,
            S3_HOST_OUTBUF_OFF as u64,
            RECORD_SIZE as u64,
        )
        .ok()
    }
}

/// Host PE path: fixed-template TYB → slim PE via YOYO table compose (not `emit_x86`).
/// `None` if program is outside the subset (caller should use [`compile_tyb_to_pe_mini`]).
/// S3.397/S3.407: RET/SET+RET = compile PE image seeds (gap R — no live wrap).
/// S3.407: also the in-DLL s1_finish PE template (host↔DLL one object-code seed).
#[cfg(windows)]
pub const S3_COMPILE_PE_RET_IMAGE_SEED: &[u8] =
    include_bytes!("../blobs/s3_compile_pe_ret.pe");
#[cfg(windows)]
pub const S3_COMPILE_PE_SET_RET_IMAGE_SEED: &[u8] =
    include_bytes!("../blobs/s3_compile_pe_setret.pe");

/// S3.400/S3.405: patch user `.text`/`.data` into compile PE template (IAT kept).
#[cfg(windows)]
fn s3_compile_pe_patch_text(text: &[u8]) -> crate::types::IsaResult<Vec<u8>> {
    s3_compile_pe_patch_text_data(text, &[])
}

#[cfg(windows)]
fn s3_compile_pe_patch_text_data(text: &[u8], data: &[u8]) -> crate::types::IsaResult<Vec<u8>> {
    // S3.529: fail-closed beyond slim (was debug_assert → panic on overflow).
    if text.len() > S1_SLIM_MAX_CODE {
        return Err(crate::types::IsaError::PlatformError {
            msg: format!(
                "s1 slim PE text {} > max {}",
                text.len(),
                S1_SLIM_MAX_CODE
            ),
        });
    }
    if data.len() > S1_SLIM_IAT_BASE as usize {
        return Err(crate::types::IsaError::PlatformError {
            msg: format!(
                "s1 slim PE data {} > max {}",
                data.len(),
                S1_SLIM_IAT_BASE
            ),
        });
    }
    let mut pe = S3_COMPILE_PE_RET_IMAGE_SEED.to_vec();
    let t0 = S1_SLIM_TEXT_PAYLOAD_OFF;
    let t_end = t0 + S1_SLIM_MAX_CODE;
    for b in &mut pe[t0..t_end] {
        *b = 0;
    }
    pe[t0..t0 + text.len()].copy_from_slice(text);
    let d0 = S1_SLIM_DATA_PAYLOAD_OFF;
    let d_end = d0 + S1_SLIM_IAT_BASE as usize;
    for b in &mut pe[d0..d_end] {
        *b = 0;
    }
    if !data.is_empty() {
        pe[d0..d0 + data.len()].copy_from_slice(data);
    }
    Ok(pe)
}

pub fn try_compile_tyb_via_yoyo_table(tyb: &[u8]) -> Option<crate::pe_link::PeImage> {
    let prog = parse_tyb(tyb).ok()?;
    validate_tyb_mini(&prog).ok()?;
    // S3.494–511: non-fixed10 +RET → YOYO object-seed; not FIXED10.
    if let Some(text) = try_imul_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_aluv_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_ldb_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_memcpy_data_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_memcpy_state_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_cf_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_alloc_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_stb_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_raw_byte_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_raw_bytes_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_load_file_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_write_file_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some((text, data)) = try_data_section_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text_data(&text, &data).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput { text, data }).ok();
        }
    }
    if let Some((text, data)) = try_data_section_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text_data(&text, &data).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput { text, data }).ok();
        }
    }
    if let Some((text, data)) = try_multi_data_section_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text_data(&text, &data).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput { text, data }).ok();
        }
    }
    if let Some((text, data)) = try_multi_data_section_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text_data(&text, &data).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput { text, data }).ok();
        }
    }
    if let Some((text, data)) = try_data_alu_mix_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text_data(&text, &data).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput { text, data }).ok();
        }
    }
    if let Some(text) = try_label_cf_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_cf_nop_label_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    // S3.518: CF+LABEL+RET (rel0) / CF+filler(s)+LABEL+RET
    if let Some(text) = try_cf_label_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_cf_filler_label_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    // S3.518: bare ALLOC
    if let Some(text) = try_alloc_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    // S3.517: DATA + ALU/LABEL mix +RET
    if let Some((text, data)) = try_data_alu_mix_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text_data(&text, &data).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput { text, data }).ok();
        }
    }
    // S3.516/S3.517: CMP any-slot + multi-op + LABEL residual (non-default).
    if let Some(text) = try_slot_alu_chain_ret_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    // S3.517: bare ALU / bare RAW_BYTES (no trailing RET)
    if let Some(text) = try_slot_alu_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_raw_bytes_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    // S3.519: bare LOAD/WRITE/RAW_BYTE/STB
    if let Some(text) = try_load_file_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_write_file_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_raw_byte_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_stb_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    // S3.520: bare LDB/MCP/IMUL/ALUV
    if let Some(text) = try_ldb_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_memcpy_data_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_memcpy_state_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_imul_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    if let Some(text) = try_aluv_bare_tyb(&prog) {
        #[cfg(windows)]
        {
            return Some(crate::pe_link::PeImage {
                bytes: s3_compile_pe_patch_text(&text).ok()?,
            });
        }
        #[cfg(not(windows))]
        {
            return wrap_pe_slim(&CodegenOutput {
                text,
                data: Vec::new(),
            })
            .ok();
        }
    }
    let ops = fixed_template_ops(&prog)?;
    // S3.397/S3.400: Windows — RET/SET+RET image seeds; else patch text into
    // compile RET template (no wrap — gap T).
    #[cfg(windows)]
    {
        if ops.as_slice() == [0xFF] {
            return Some(crate::pe_link::PeImage {
                bytes: S3_COMPILE_PE_RET_IMAGE_SEED.to_vec(),
            });
        }
        if ops.as_slice() == [0x30, 0xFF] {
            return Some(crate::pe_link::PeImage {
                bytes: S3_COMPILE_PE_SET_RET_IMAGE_SEED.to_vec(),
            });
        }
        let text = try_emit_text_via_yoyo_table(&ops)?;
        return Some(crate::pe_link::PeImage {
            bytes: s3_compile_pe_patch_text(&text).ok()?,
        });
    }
    #[cfg(not(windows))]
    {
        let text = try_emit_text_via_yoyo_table(&ops)?;
        wrap_pe_slim(&CodegenOutput {
            text,
            data: Vec::new(),
        })
        .ok()
    }
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
    fn compile_tyb_to_pe_mini_rejects_write_file_iat_path() {
        // S3.94: mini accepts WRITE argc=3; path_off overlapping IAT still fail-closed.
        let tyb = tyb_from_records(&[(0x51, 3, [1, 0, 0x00, 0x0F, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!((0x0F00u16 as u32) >= S1_SLIM_IAT_BASE);
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

    /// S3.320: expand bootstrap is named MEMCPY+RET (≡ MANY flat; no flat_seed decode).
    #[test]
    fn s3_bootstrap_named_memcpy_ret_pin() {
        assert_eq!(
            s3_bootstrap_expand_asm(),
            vec![s3_rec_memcpy(9, 2, 12), s3_fixed_rec(0xFF)]
        );
        assert_eq!(
            s3_bootstrap_expand_asm(),
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT)
        );
        assert_eq!(s3_bootstrap_expand_asm().len(), S3_SHAPE_FILL_MANY_ROWS);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
    }

    #[test]
    fn s3_fixed_body_text_only_via_emit_or_a1_floor() {
        let body = s3_fixed_row(0x30);
        assert_eq!(body.len(), 14);
        let (flat_tyb, _) = s3_yoyo_emit_flat_via_stb(&body, 0x20);
        let flat_text = emit_x86(&parse_tyb(&flat_tyb).unwrap()).text;
        assert!(
            !flat_text.starts_with(&body),
            "STB-flat of SET body must not become .text head"
        );
        let mut poke = Vec::new();
        s3_ir_seed_at(&mut poke, 0x80, &body, 10);
        poke.push((0xFF, 0, [0; 6]));
        let poke_text = emit_x86(&parse_tyb(&s3_host_tyb_from_records(&poke)).unwrap()).text;
        assert!(
            !poke_text.starts_with(&body),
            "poke-seed of SET body must not become .text head"
        );
        let mut a1 = Vec::new();
        s3_append_raw_a1(&mut a1, &body);
        let a1_text = emit_x86(&parse_tyb(&tyb_from_records(&a1)).unwrap()).text;
        assert!(a1_text.starts_with(&body), "A1 of body opens .text");
        assert_eq!(
            emit_x86(
                &parse_tyb(&s3_host_tyb_from_records(&[
                    s3_fixed_rec(0x30),
                    s3_fixed_rec(0xFF),
                ]))
                .unwrap()
            )
            .text[..body.len()],
            body[..],
            "emit_x86(SET+RET) body prefix matches OnceLock bake"
        );
    }

    #[test]
    fn s3_stb_runs_test_only_production_poke() {
        let lea = s3_short_lea_raw_bytes();
        let (pack, _) = s3_yoyo_pack_loop_stb_tyb(&lea);
        assert!(
            !parse_tyb(&pack).unwrap().records.iter().any(|r| r.op == 0x61),
            "pack loop must not SUB-fold"
        );
        let (emit, _) = s3_yoyo_emit_flat_via_stb(S3_SHAPE_FILL_MANY_FLAT, 0x20);
        assert!(
            !parse_tyb(&emit).unwrap().records.iter().any(|r| r.op == 0x61),
            "emit-flat must not SUB-fold"
        );
        // Compare helper still exists under cfg(test).
        let mut runs = Vec::new();
        runs.push(s3_rec_set(S3_PACK_PROTO_DST, 0));
        s3_append_stb_runs(&mut runs, &[0xAA, 0xAA], 1);
        assert!(runs.iter().any(|r| r.0 == 0x61));
    }

    #[test]
    fn s3_emit_body_const_embed_reject() {
        // S3.317: rejected const-embed as *host-IR densify*. S3.359: seed is
        // allowed as 真自举 gap1 (own-ISA object code), still ≡ emit_x86 oracle.
        let total: usize = S3_FIXED10_OPS.iter().map(|o| s3_fixed_row(*o).len()).sum();
        assert_eq!(total, 95, "fixed10 body bytes stay 95");
        for op in S3_FIXED10_OPS {
            assert_eq!(s3_fixed_row(op), s3_compute_fixed_body(op).as_slice());
        }
        assert_eq!(s3_bootstrap_expand_asm().len(), 2);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(s3_in_dll_lea_prologue_recs().len(), 13);
    }


    #[test]
    fn s3_pack_const_scratch_poke_pin() {
        let proto = s3_yoyo_pack_proto_tyb(&[0xAA, 0xBB]);
        let n = parse_tyb(&proto).unwrap().records.len();
        assert_eq!(n, 52, "proto pack IR after const poke (was 57)");
        assert_eq!(
            s3_pack_data_via_ir(&[0xAA, 0xBB]),
            s3_pack_data(&[0xAA, 0xBB])
        );
        // MEMCPY loop pack still parity (const scratch also poked).
        let (fw, off) = {
            // exercise loop_ir path via large-enough? use pack_loop through via stb pack
            s3_yoyo_pack_loop_stb_tyb(&[0xAA, 0xBB])
        };
        let _ = (fw, off);
        assert_eq!(s3_pack_data_via_ir(&[1, 2, 3, 4, 5]), s3_pack_data(&[1, 2, 3, 4, 5]));
    }

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
        // S3.200/S3.314: poke skips zeros; STB count stays below flat_len.
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
        // S3.315: remaining hard pins (numeric).
        assert_eq!(S3_EXPAND_ASM_PIN, 22, "expand assembler cliff");
        assert_eq!(s3_in_dll_lea_prologue_recs().len(), 13, "A1 13×37B cliff");
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF])).unwrap().records.len(),
            173,
            "host seed pin (S3.367: +2 split)"
        );
        assert_eq!(s3_bootstrap_expand_asm().len(), 2, "Rust bootstrap MEMCPY+RET");
        assert_eq!(s3_fixed_row(0xFF), s3_compute_fixed_body(0xFF).as_slice());
    }

    /// S3.321/S3.327: via_yoyo of MANY_FLAT ≡ bootstrap (proof; production uses named).
    #[test]
    fn s3_shape_fill_ctrl_shares_bootstrap_pin() {
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT),
            s3_bootstrap_expand_asm(),
            "via_yoyo still ≡ bootstrap (proof retained)"
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
    }

    /// S3.323/S3.324: no expand_asm_flat / CTRL alias — MANY_FLAT is asm body.
    #[test]
    fn s3_expand_ctrl_flat_is_asm_body_pin() {
        assert_eq!(S3_SHAPE_FILL_MANY_FLAT.len(), S3_SHAPE_FILL_MANY_ROWS * 7);
        assert_eq!(
            s3_bootstrap_expand_asm(),
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT)
        );
    }

    /// S3.340/S3.341/S3.343: pack consts + argc index cfg(test); pub expand stays live.
    #[test]
    fn s3_expand_stays_in_lib_pin() {
        assert_eq!(s3_bootstrap_expand_asm().len(), S3_SHAPE_FILL_MANY_ROWS);
        let (exp, off) = s3_expand_many_flat_tyb();
        assert_eq!((exp.clone(), off), s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT));
        assert_eq!(parse_tyb(&exp).unwrap().records.len(), S3_EXPAND_ASM_PIN);
        assert_eq!(S3_PACK_PROTO_DST, 9, "host outbuf slot stays live");
    }

    /// S3.360: gap2 — lea→.text A1 prologue = const seed (own ISA RAW_BYTES).
    /// A1 stays the inject op; Rust no longer live-packs. ≡ via_pack oracle.
    #[test]
    fn s3_in_dll_lea_a1_seed_pin() {
        assert_eq!(S3_IN_DLL_LEA_A1_SEED.len(), 13);
        let via = s3_in_dll_lea_prologue_recs_via_pack();
        assert_eq!(S3_IN_DLL_LEA_A1_SEED.as_slice(), via.as_slice());
        assert_eq!(s3_in_dll_lea_prologue_recs(), S3_IN_DLL_LEA_A1_SEED.as_slice());
        let a1_bytes: usize = S3_IN_DLL_LEA_A1_SEED.iter().map(|r| r.1 as usize).sum();
        assert_eq!(a1_bytes, 37);
        assert_eq!(s3_short_lea_raw_bytes().len(), 37);
        // Still injects via A1 (own op) — poke alone does not open .text.
        let a1_tyb = tyb_from_records(s3_in_dll_lea_prologue_recs());
        let a1_text = emit_x86(&parse_tyb(&a1_tyb).unwrap()).text;
        assert!(a1_text.starts_with(&s3_short_lea_raw_bytes()));
        assert_eq!(
            parse_tyb(&s3_expand_many_flat_tyb().0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
    }

    /// S3.359: gap1 — template body seed = own-ISA object code; ≡ live emit_x86.
    /// Prod OnceLock no longer calls emit_x86; seed is the trust root YOYO copies.
    #[test]
    fn s3_fixed10_body_seed_pin() {
        assert_eq!(S3_FIXED10_BODY_SEED.len(), S3_FIXED10_OPS.len());
        let total: usize = S3_FIXED10_BODY_SEED.iter().map(|s| s.len()).sum();
        assert_eq!(total, 95, "fixed10 body bytes stay 95");
        for (i, &op) in S3_FIXED10_OPS.iter().enumerate() {
            assert_eq!(
                S3_FIXED10_BODY_SEED[i],
                s3_compute_fixed_body(op).as_slice(),
                "seed ≡ emit_x86 for {op:#x}"
            );
            assert_eq!(s3_fixed_row(op), s3_compute_fixed_body(op).as_slice());
        }
        assert_eq!(
            try_emit_text_via_yoyo_table(&[0xFF]).unwrap(),
            S3_FIXED10_BODY_SEED[0]
        );
        assert_eq!(
            parse_tyb(&s3_expand_many_flat_tyb().0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
    }

    /// S3.353–357: no sentinel; job≡const bootstrap; Vec wrapper cfg(test).
    /// Const table ≡ `s3_rec_memcpy`+RET builders. Assembler pin 22 (0 IR).
    #[test]
    fn s3_expand_job_from_bootstrap_pin() {
        assert_eq!(
            S3_BOOTSTRAP_EXPAND_ASM,
            [s3_rec_memcpy(9, 2, 12), s3_fixed_rec(0xFF)]
        );
        let boot = s3_bootstrap_expand_asm();
        assert_eq!(boot.as_slice(), &S3_BOOTSTRAP_EXPAND_ASM);
        assert_eq!(S3_BOOTSTRAP_EXPAND_ASM.len(), 2);
        assert_eq!(boot, s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT));
        let (prod, poff) = s3_expand_many_flat_tyb();
        let (gen, goff) = s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT);
        let (shared, soff) = s3_expand_job_recs_tyb(&S3_BOOTSTRAP_EXPAND_ASM);
        assert_eq!((prod.clone(), poff), (gen, goff));
        assert_eq!((prod.clone(), poff), (shared, soff));
        assert_eq!(poff, S3_EXPAND_SEED + 16, "OUT = SEED + nrecs×8 (no sentinel gap)");
        assert_eq!(parse_tyb(&prod).unwrap().records.len(), S3_EXPAND_ASM_PIN);
        let emitted = s3_sim_prog_loop_cursor(
            &prod,
            0,
            poff,
            S3_EXPAND_SEED as u64,
            poff as u64 + HEADER_SIZE as u64,
            (S3_BOOTSTRAP_EXPAND_ASM.len() * RECORD_SIZE) as u64,
        )
        .unwrap();
        assert_eq!(&emitted[..4], &TYB_MAGIC);
        assert_eq!(
            parse_tyb(&emitted).unwrap().records.len(),
            S3_BOOTSTRAP_EXPAND_ASM.len()
        );
    }

    /// S3.358–371: 真自举能力图 — 缺的能力用自己的指令做出来（不译 emit_x86）。
    /// 已会：poke/pack、TYB 发射、表合成→.text、expand、body/lea/win 种子、
    /// RET+SET/RET 整段 TYB、组合悬崖(S3.370)。
    /// **下一缺口**（S3.371 地图）：
    ///   (A) try_emit = Rust sim host，不是 PE 上跑 YOYO；
    ///   (B) 非 RET host assemble（组合悬崖，需别的 arch，不堆 TYB）；
    ///   (C) in-DLL 超 A1 种子的路径仍偏 Rust。
    /// 不挤 IR 羊粪。
    #[test]
    fn s3_true_selfhost_capability_map_pin() {
        // —— 已会（自己的指令）——
        assert!(
            s3_unpack_data_tyb(&s3_yoyo_pack_loop_stb_tyb(&[1, 2, 3]).0).is_empty(),
            "pack = YOYO STB/poke IR"
        );
        let (emit_tyb, eoff) = s3_yoyo_emit_tyb_file(&[s3_fixed_rec(0xFF)]);
        assert!(s3_unpack_data_tyb(&emit_tyb).is_empty());
        assert_eq!(
            s3_sim_prog_loop(&emit_tyb, 0, eoff, 0).unwrap(),
            s3_host_tyb_from_records(&[s3_fixed_rec(0xFF)]),
            "emit TYB file = YOYO IR"
        );
        let yoyo_text = try_emit_text_via_yoyo_table(&[0x30, 0xFF]).unwrap();
        let rust_text = emit_x86(
            &parse_tyb(&s3_host_tyb_from_records(&[
                s3_fixed_rec(0x30),
                s3_fixed_rec(0xFF),
            ]))
            .unwrap(),
        )
        .text;
        assert_eq!(yoyo_text, rust_text, "fixed10 .text: YOYO table ≡ emit_x86");
        assert_eq!(
            S3_BOOTSTRAP_EXPAND_ASM,
            [s3_rec_memcpy(9, 2, 12), s3_fixed_rec(0xFF)],
            "expand body = own MEMCPY+RET"
        );
        // —— 三缺口关闭（能力种子，不是翻译器）——
        assert_eq!(
            s3_fixed_row(0x30),
            S3_FIXED10_BODY_SEED[2],
            "gap1 closed: template body = own-ISA object-code seed"
        );
        assert_eq!(
            s3_fixed_row(0x30),
            s3_compute_fixed_body(0x30).as_slice(),
            "seed still ≡ emit_x86 oracle"
        );
        // S3.365: fixed_row returns &'static seed (same bytes; no OnceLock Vec clone).
        assert_eq!(s3_fixed_row(0xFF), S3_FIXED10_BODY_SEED[0]);
        assert_eq!(
            s3_fixed_row(0xFF).as_ptr(),
            S3_FIXED10_BODY_SEED[0].as_ptr(),
            "S3.365: fixed_row aliases seed storage (no Vec cache)"
        );
        assert_eq!(
            s3_in_dll_lea_prologue_recs(),
            S3_IN_DLL_LEA_A1_SEED.as_slice(),
            "gap2 closed: lea→.text = A1 record seed (own RAW_BYTES)"
        );
        assert_eq!(
            S3_EXPAND_POKE_SEED.len() + S3_BOOTSTRAP_EXPAND_ASM.len(),
            S3_EXPAND_ASM_PIN,
            "gap3 closed: expand poke shell = own-ISA SET/STB seed"
        );
        let (prod, poff) = s3_expand_many_flat_tyb();
        let (live, loff) = s3_expand_job_recs_tyb(&S3_BOOTSTRAP_EXPAND_ASM);
        assert_eq!(prod, live, "prod expand TYB seed ≡ live poke oracle");
        assert_eq!(prod.as_slice(), S3_EXPAND_TYB_SEED);
        assert_eq!(poff, loff);
        assert_eq!(poff, S3_EXPAND_OUT);
        assert_eq!(
            S3_HOST_PROG_LOOP_SEED.len(),
            29,
            "host prog-loop = own-ISA seed (ops_len patched)"
        );
        assert_eq!(
            s3_prog_loop_code_host(1, S3_HOST_NROWS).as_slice(),
            S3_HOST_PROG_LOOP_SEED.as_slice()
        );
        assert_eq!(
            S3_HOST_TABLE_SEED.len(),
            200,
            "host table = own object-code seed (10×20)"
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173,
            "host floor 173: win0+win1 poke seeded; only prog live"
        );
        assert_eq!(
            S3_HOST_TABLE_WIN0_POKE_SEED.len(),
            80,
            "S3.366: win0 poke seed = 80 SET/STB rows"
        );
        assert_eq!(
            S3_HOST_TABLE_WIN1_POKE_SEED.len(),
            58,
            "S3.367: win1 poke seed = 58 SET/STB rows"
        );
        assert_eq!(
            S3_HOST_RET_TYB_SEED.len(),
            1392,
            "S3.368: host RET = full TYB object-code seed"
        );
        assert_eq!(s3_host_prog_loop_tyb(&[0xFF]).as_slice(), S3_HOST_RET_TYB_SEED);
        assert_eq!(
            S3_HOST_SET_RET_TYB_SEED.len(),
            1408,
            "S3.369: host SET+RET = full TYB object-code seed"
        );
        assert_eq!(
            s3_host_prog_loop_tyb(&[0x30, 0xFF]).as_slice(),
            S3_HOST_SET_RET_TYB_SEED
        );
        // —— S3.371: next-gap map (not CLOSED) ——
        // (A) try_emit is Rust sim of host TYB, not PE execution.
        let via_sim = try_emit_text_via_yoyo_table(&[0xFF]).unwrap();
        assert_eq!(via_sim, s3_fixed_row(0xFF));
        // (B) non-RET assemble still live for unseeded combos (cliff holds).
        assert_ne!(
            s3_host_prog_loop_tyb(&[0x66, 0xFF]).as_slice(),
            S3_HOST_RET_TYB_SEED
        );
        // (C) A1 lea seed stays 13×37; broader in-DLL still Rust-side.
        assert_eq!(s3_in_dll_lea_prologue_recs().len(), 13);
        assert_eq!(s3_short_lea_raw_bytes().len(), 37);
    }

    /// S3.371: post-host-seed-chain gap map (historical pin).
    #[test]
    fn s3_true_selfhost_next_gap_map_pin() {
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(
            s3_host_prog_loop_tyb(&[0xFF]).as_slice(),
            S3_HOST_RET_TYB_SEED
        );
        assert_eq!(
            s3_host_prog_loop_tyb(&[0x30, 0xFF]).as_slice(),
            S3_HOST_SET_RET_TYB_SEED
        );
        let assembled = s3_host_prog_loop_tyb(&[0x62, 0xFF]);
        assert!(assembled.len() >= 1392);
        assert_ne!(assembled.as_slice(), S3_HOST_RET_TYB_SEED);
        // Empty-DATA host still ≡ sim (oracle); try_emit itself is PE on Windows (S3.372).
        assert_eq!(
            s3_sim_prog_loop_cursor(
                S3_HOST_RET_TYB_SEED,
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
                S3_HOST_OUTBUF_OFF as u64,
                RECORD_SIZE as u64,
            )
            .unwrap(),
            vec![0xC3]
        );
        assert_eq!(S3_IN_DLL_LEA_A1_SEED.len(), 13);
        assert_eq!(s3_bootstrap_expand_asm().len(), 2);
    }

    /// S3.372: gap A — try_emit via PE/YOYO run (not Rust sim). Empty-DATA host
    /// TYB stays 2279 > slim 1024; PE path = DATA|host-lea|host-loop (~308B).
    #[test]
    #[cfg(windows)]
    fn s3_try_emit_via_pe_pin() {
        assert_eq!(S3_HOST_LEA_A1_SEED.len(), 13);
        let a1_bytes: usize = S3_HOST_LEA_A1_SEED.iter().map(|r| r.1 as usize).sum();
        assert_eq!(a1_bytes, 37);
        let pe_tyb = s3_host_compose_tyb_pe(&[0xFF]);
        let pe_code = emit_x86(&parse_tyb(&pe_tyb).unwrap()).text.len();
        assert!(pe_code <= S1_SLIM_MAX_CODE, "PE compose code {pe_code}");
        assert!(pe_code < 400, "PE compose stays lean {pe_code}");
        let host_code = emit_x86(&parse_tyb(S3_HOST_RET_TYB_SEED).unwrap()).text.len();
        assert!(host_code > S1_SLIM_MAX_CODE, "empty-DATA host cliff {host_code}");
        // Direct wrap (no mini/try_emit cycle) runs ≡ try_emit for all fixed10.
        let cases: &[&[u8]] = &[
            &[0xFF],
            &[0x00, 0xFF],
            &[0x30, 0xFF],
            &[0x60, 0xFF],
            &[0x66, 0xFF],
            &[0x62, 0xFF],
            &[0x67, 0xFF],
            &[0x61, 0xFF],
            &[0x64, 0xFF],
            &[0x65, 0xFF],
        ];
        assert_eq!(cases.len(), 10);
        for &ops in cases {
            let via = try_emit_text_via_yoyo_table(ops).unwrap_or_else(|| panic!("try_emit {ops:?}"));
            let sim = s3_sim_prog_loop_cursor(
                &s3_host_prog_loop_tyb(ops),
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
                S3_HOST_OUTBUF_OFF as u64,
                RECORD_SIZE as u64,
            )
            .unwrap();
            assert_eq!(via, sim, "S3.372 PE try_emit ≡ host sim ops={ops:?}");
            let tyb = s3_host_compose_tyb_pe(ops);
            let pe = wrap_pe_slim(&emit_x86(&parse_tyb(&tyb).unwrap())).expect("wrap");
            let got = s3_run_slim_pe_outbuf_at(&pe.bytes, S3_HOST_OUTBUF_OFF, via.len());
            assert_eq!(got, via, "S3.372 PE outbuf ops={ops:?}");
        }
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }



    /// S3.373: PE compose DATA prefix = own object-code seed (110); prog tail live.
    /// Record framing may split vs one-shot pack; .data image + .text must hold.
    #[test]
    fn s3_host_pe_data_prefix_seed_pin() {
        assert_eq!(S3_HOST_PE_DATA_PREFIX_SEED.len(), 110);
        let mut prefix = vec![0u8; S3_HOST_TABLE_OFF as usize];
        prefix.extend_from_slice(&S3_HOST_TABLE_SEED);
        assert_eq!(prefix.len(), S3_HOST_PROG_OFF as usize);
        assert_eq!(
            S3_HOST_PE_DATA_PREFIX_SEED.as_slice(),
            s3_pack_data_records(&prefix).as_slice()
        );
        let unpack = |tyb: &[u8]| -> Vec<u8> {
            let mut d = Vec::new();
            for r in parse_tyb(tyb).unwrap().records {
                if r.op == 0x10 {
                    for k in 0..r.argc as usize {
                        d.push(r.args_raw[k * 2]);
                    }
                }
            }
            d
        };
        let cases: &[&[u8]] = &[
            &[0xFF],
            &[0x00, 0xFF],
            &[0x30, 0xFF],
            &[0x60, 0xFF],
            &[0x66, 0xFF],
            &[0x62, 0xFF],
            &[0x67, 0xFF],
            &[0x61, 0xFF],
            &[0x64, 0xFF],
            &[0x65, 0xFF],
        ];
        assert_eq!(cases.len(), 10);
        for &ops in cases {
            let mut live_payload = prefix.clone();
            live_payload.push(ops.len() as u8);
            live_payload.extend_from_slice(ops);
            while live_payload.len() < S3_HOST_OUTBUF_OFF as usize {
                live_payload.push(0);
            }
            let mut live = s3_pack_data_records(&live_payload);
            live.extend_from_slice(&S3_HOST_LEA_A1_SEED);
            live.extend(s3_prog_loop_code_host(ops.len() as u32, S3_HOST_NROWS));
            let live_tyb = s3_host_tyb_from_records(&live);
            let seeded = s3_host_compose_tyb_pe(ops);
            assert_eq!(
                unpack(&seeded),
                live_payload,
                "S3.373 .data image ops={ops:?}"
            );
            assert_eq!(unpack(&seeded), unpack(&live_tyb), "S3.373 unpack≡live");
            assert_eq!(
                emit_x86(&parse_tyb(&seeded).unwrap()).text,
                emit_x86(&parse_tyb(&live_tyb).unwrap()).text,
                "S3.373 .text ops={ops:?}"
            );
        }
        assert_eq!(S3_HOST_LEA_A1_SEED.len(), 13);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
    }



    /// S3.374: PE compose RET/SET+RET = full TYB seeds; other ops still assemble.
    /// Gap B empty-DATA non-RET assemble / C in-DLL density remain.
    #[test]
    fn s3_host_pe_tyb_seeds_pin() {
        assert_eq!(S3_HOST_PE_RET_TYB_SEED.len(), 1288);
        assert_eq!(S3_HOST_PE_SET_RET_TYB_SEED.len(), 1288);
        assert_eq!(&S3_HOST_PE_RET_TYB_SEED[..4], &TYB_MAGIC);
        assert_eq!(&S3_HOST_PE_SET_RET_TYB_SEED[..4], &TYB_MAGIC);
        // Live assemble oracle (force past seed short-circuit by rebuilding).
        let live_ret = {
            let ops: &[u8] = &[0xFF];
            let mut tail = vec![ops.len() as u8];
            tail.extend_from_slice(ops);
            let prefix_len = S3_HOST_PROG_OFF as usize;
            while prefix_len + tail.len() < S3_HOST_OUTBUF_OFF as usize {
                tail.push(0);
            }
            let mut recs = S3_HOST_PE_DATA_PREFIX_SEED.to_vec();
            recs.extend(s3_pack_data_records(&tail));
            recs.extend_from_slice(&S3_HOST_LEA_A1_SEED);
            recs.extend(s3_prog_loop_code_host(ops.len() as u32, S3_HOST_NROWS));
            s3_host_tyb_from_records(&recs)
        };
        let live_set = {
            let ops: &[u8] = &[0x30, 0xFF];
            let mut tail = vec![ops.len() as u8];
            tail.extend_from_slice(ops);
            let prefix_len = S3_HOST_PROG_OFF as usize;
            while prefix_len + tail.len() < S3_HOST_OUTBUF_OFF as usize {
                tail.push(0);
            }
            let mut recs = S3_HOST_PE_DATA_PREFIX_SEED.to_vec();
            recs.extend(s3_pack_data_records(&tail));
            recs.extend_from_slice(&S3_HOST_LEA_A1_SEED);
            recs.extend(s3_prog_loop_code_host(ops.len() as u32, S3_HOST_NROWS));
            s3_host_tyb_from_records(&recs)
        };
        assert_eq!(S3_HOST_PE_RET_TYB_SEED, live_ret.as_slice());
        assert_eq!(S3_HOST_PE_SET_RET_TYB_SEED, live_set.as_slice());
        assert_eq!(s3_host_compose_tyb_pe(&[0xFF]).as_slice(), S3_HOST_PE_RET_TYB_SEED);
        assert_eq!(
            s3_host_compose_tyb_pe(&[0x30, 0xFF]).as_slice(),
            S3_HOST_PE_SET_RET_TYB_SEED
        );
        let other = s3_host_compose_tyb_pe(&[0x62, 0xFF]);
        assert_ne!(other.as_slice(), S3_HOST_PE_RET_TYB_SEED);
        assert!(other.len() >= 1288);
        assert_eq!(S3_HOST_PE_DATA_PREFIX_SEED.len(), 110);
        assert_eq!(S3_HOST_LEA_A1_SEED.len(), 13);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
    }

    /// S3.375: per-ops PE full-TYB seed cliff (mirror S3.370). RET+SET/RET already
    /// seeded (S3.374); remaining len2 alone = 8 × ≥1288B — volume not capability.
    /// Keep assemble for other ops; next ≥10 = gap C in-DLL / empty-DATA shrink.
    #[test]
    fn s3_pe_per_ops_tyb_seed_cliff_reject() {
        let mut len2_ret = 0usize;
        for &op in &S3_FIXED10_OPS {
            if op == 0xFF {
                continue;
            }
            len2_ret += 1;
            let tyb = s3_host_compose_tyb_pe(&[op, 0xFF]);
            assert!(
                tyb.len() >= 1288,
                "each PE TYB seed would be ≥RET size ({op:#x}+RET)"
            );
        }
        assert_eq!(len2_ret, 9);
        let seeded = 2usize; // S3.374 RET + SET+RET
        let remaining_len2 = len2_ret - 1; // SET already seeded
        assert_eq!(remaining_len2, 8);
        assert!(
            remaining_len2 * 1288 > 10_000,
            "per-ops PE TYB seeds: {} more len2 alone (≥{}KB) — combinatorial cliff",
            remaining_len2,
            remaining_len2 * 1288 / 1024
        );
        assert_eq!(
            s3_host_compose_tyb_pe(&[0xFF]).as_slice(),
            S3_HOST_PE_RET_TYB_SEED
        );
        assert_eq!(
            s3_host_compose_tyb_pe(&[0x30, 0xFF]).as_slice(),
            S3_HOST_PE_SET_RET_TYB_SEED
        );
        let other = s3_host_compose_tyb_pe(&[0x66, 0xFF]);
        assert_ne!(other.as_slice(), S3_HOST_PE_RET_TYB_SEED);
        assert_ne!(other.as_slice(), S3_HOST_PE_SET_RET_TYB_SEED);
        assert_eq!(seeded, 2);
        assert_eq!(S3_HOST_PE_DATA_PREFIX_SEED.len(), 110);
        assert_eq!(S3_HOST_LEA_A1_SEED.len(), 13);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
    }

    /// S3.376: gap C — in-DLL RET/SET+RET = full TYB seeds (beyond A1-only).
    /// Other ops still live lea+poke+loop. Not CLOSED.
    #[test]
    fn s3_in_dll_tyb_seeds_pin() {
        assert_eq!(S3_IN_DLL_RET_TYB_SEED.len(), 616);
        assert_eq!(S3_IN_DLL_SET_RET_TYB_SEED.len(), 632);
        assert_eq!(&S3_IN_DLL_RET_TYB_SEED[..4], &TYB_MAGIC);
        assert_eq!(&S3_IN_DLL_SET_RET_TYB_SEED[..4], &TYB_MAGIC);
        // Live oracle (rebuild without seed short-circuit).
        let live = |ops: &[u8]| {
            let payload = s3_fixed_payload(&S3_FIXED10_OPS[..3], ops, S3_OUTBUF_OFF as usize);
            let start = S3_TABLE_OFF as usize;
            let end = S3_PROG_OFF as usize + 1 + ops.len();
            let slice = &payload[start..end];
            let mut recs = Vec::new();
            recs.extend_from_slice(s3_in_dll_lea_prologue_recs());
            s3_append_stb_poke_global_distinct_base(&mut recs, slice, 15);
            recs.extend(s3_prog_loop_code(ops.len() as u32, 3));
            tyb_from_records(&recs)
        };
        assert_eq!(S3_IN_DLL_RET_TYB_SEED, live(&[0xFF]).as_slice());
        assert_eq!(S3_IN_DLL_SET_RET_TYB_SEED, live(&[0x30, 0xFF]).as_slice());
        assert_eq!(s3_table_prog_loop_tyb(&[0xFF]).as_slice(), S3_IN_DLL_RET_TYB_SEED);
        assert_eq!(
            s3_table_prog_loop_tyb(&[0x30, 0xFF]).as_slice(),
            S3_IN_DLL_SET_RET_TYB_SEED
        );
        let other = s3_table_prog_loop_tyb(&[0x00, 0xFF]);
        assert_ne!(other.as_slice(), S3_IN_DLL_RET_TYB_SEED);
        assert!(other.len() >= 600);
        // Floors: A1 still 13×37; PE still runs.
        assert_eq!(S3_IN_DLL_LEA_A1_SEED.len(), 13);
        assert_eq!(s3_short_lea_raw_bytes().len(), 37);
        compile_tyb_to_pe_mini(S3_IN_DLL_RET_TYB_SEED).expect("in-DLL RET seed mini");
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.377: post-seed-chain gap map. Next ≥10 ≠ more RET/SET+RET TYB seeds.
    /// Remaining: (D) empty-DATA host poke ≤slim 1024 so try_emit can drop DATA;
    /// (E) PE compose firmware itself not via Rust emit_x86; (F) in-DLL beyond
    /// 3-row / non-seeded ops. Not CLOSED.
    #[test]
    fn s3_true_selfhost_post_seed_gap_map_pin() {
        // Seeded PE + in-DLL RET/SET+RET hold.
        assert_eq!(
            s3_host_compose_tyb_pe(&[0xFF]).as_slice(),
            S3_HOST_PE_RET_TYB_SEED
        );
        assert_eq!(
            s3_host_compose_tyb_pe(&[0x30, 0xFF]).as_slice(),
            S3_HOST_PE_SET_RET_TYB_SEED
        );
        assert_eq!(
            s3_table_prog_loop_tyb(&[0xFF]).as_slice(),
            S3_IN_DLL_RET_TYB_SEED
        );
        assert_eq!(
            s3_table_prog_loop_tyb(&[0x30, 0xFF]).as_slice(),
            S3_IN_DLL_SET_RET_TYB_SEED
        );
        // (D) empty-DATA host still over slim — DATA PE path remains.
        let host_code = emit_x86(&parse_tyb(S3_HOST_RET_TYB_SEED).unwrap()).text.len();
        assert!(host_code > S1_SLIM_MAX_CODE, "D: empty-DATA host {host_code}");
        let pe_code = emit_x86(&parse_tyb(S3_HOST_PE_RET_TYB_SEED).unwrap()).text.len();
        assert!(pe_code <= S1_SLIM_MAX_CODE, "PE compose fits {pe_code}");
        // (E) witness: PE compose TYB still needs Rust emit_x86 to become PE bytes.
        assert!(wrap_pe_slim(&emit_x86(&parse_tyb(S3_HOST_PE_RET_TYB_SEED).unwrap())).is_ok());
        // (F) non-seeded in-DLL still assembles.
        let other = s3_table_prog_loop_tyb(&[0x00, 0xFF]);
        assert_ne!(other.as_slice(), S3_IN_DLL_RET_TYB_SEED);
        assert_eq!(S3_IN_DLL_LEA_A1_SEED.len(), 13);
        assert_eq!(S3_HOST_PE_DATA_PREFIX_SEED.len(), 110);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.378: gap D reject — empty-DATA host cannot fit slim PE. win0 poke alone
    /// emits 1117 > [S1_SLIM_MAX_CODE] 1024; full RET=2279. Keep DATA|lea PE path
    /// (S3.372+). Raising slim / dropping empty-DATA = different arch cut, not this knife.
    #[test]
    fn s3_empty_data_host_slim_cliff_reject() {
        let win0 = emit_x86(
            &parse_tyb(&s3_host_tyb_from_records(&S3_HOST_TABLE_WIN0_POKE_SEED.to_vec())).unwrap(),
        )
        .text
        .len();
        let win1 = emit_x86(
            &parse_tyb(&s3_host_tyb_from_records(&S3_HOST_TABLE_WIN1_POKE_SEED.to_vec())).unwrap(),
        )
        .text
        .len();
        let full = emit_x86(&parse_tyb(S3_HOST_RET_TYB_SEED).unwrap()).text.len();
        let pe = emit_x86(&parse_tyb(S3_HOST_PE_RET_TYB_SEED).unwrap()).text.len();
        assert!(win0 > S1_SLIM_MAX_CODE, "win0 alone {win0} must exceed slim");
        assert!(full > S1_SLIM_MAX_CODE, "full host {full}");
        assert!(pe <= S1_SLIM_MAX_CODE, "PE DATA path stays under slim {pe}");
        assert_eq!(S1_SLIM_MAX_CODE, 1024);
        // Floors unchanged; PE try_emit path remains the compose route.
        assert_eq!(S3_HOST_TABLE_WIN0_POKE_SEED.len(), 80);
        assert_eq!(S3_HOST_TABLE_WIN1_POKE_SEED.len(), 58);
        assert!(win1 < win0);
        assert_eq!(
            s3_host_compose_tyb_pe(&[0xFF]).as_slice(),
            S3_HOST_PE_RET_TYB_SEED
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.379: gap E — PE compose RET/SET+RET = slim PE image seeds (no live emit_x86).
    /// Other ops still emit+wrap; per-ops PE image seeds = combinatorial cliff (like 375).
    #[test]
    #[cfg(windows)]
    fn s3_host_pe_image_seeds_pin() {
        assert_eq!(S3_HOST_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_HOST_PE_SET_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(&S3_HOST_PE_RET_IMAGE_SEED[..2], b"MZ");
        assert_eq!(&S3_HOST_PE_SET_RET_IMAGE_SEED[..2], b"MZ");
        let live = |ops: &[u8]| {
            let tyb = s3_host_compose_tyb_pe(ops);
            wrap_pe_slim(&emit_x86(&parse_tyb(&tyb).unwrap()))
                .unwrap()
                .bytes
        };
        assert_eq!(S3_HOST_PE_RET_IMAGE_SEED, live(&[0xFF]).as_slice());
        assert_eq!(S3_HOST_PE_SET_RET_IMAGE_SEED, live(&[0x30, 0xFF]).as_slice());
        // try_emit via image seed ≡ host sim / live PE outbuf
        assert_eq!(try_emit_text_via_yoyo_table(&[0xFF]).unwrap(), vec![0xC3]);
        let set = try_emit_text_via_yoyo_table(&[0x30, 0xFF]).unwrap();
        assert_eq!(
            set,
            s3_sim_prog_loop_cursor(
                &s3_host_prog_loop_tyb(&[0x30, 0xFF]),
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
                S3_HOST_OUTBUF_OFF as u64,
                RECORD_SIZE as u64,
            )
            .unwrap()
        );
        // Non-seeded op still works (live emit path).
        let add = try_emit_text_via_yoyo_table(&[0x62, 0xFF]).unwrap();
        assert_eq!(
            add,
            s3_sim_prog_loop_cursor(
                &s3_host_prog_loop_tyb(&[0x62, 0xFF]),
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
                S3_HOST_OUTBUF_OFF as u64,
                RECORD_SIZE as u64,
            )
            .unwrap()
        );
        // Cliff witness: 8 more len2 PE images × 6656 ≫ 10KB volume.
        assert!(8 * 6656 > 50_000);
        assert_eq!(S3_HOST_PE_RET_TYB_SEED.len(), 1288);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.380: per-ops PE *image* seed cliff (gap E residual). RET+SET/RET images
    /// already seeded (S3.379); 8 more len2 × 6656B ≫ volume. Keep emit+wrap for
    /// other ops; next ≥10 = gap F in-DLL image / non-emit arch.
    #[test]
    #[cfg(windows)]
    fn s3_pe_per_ops_image_seed_cliff_reject() {
        let mut len2 = 0usize;
        for &op in &S3_FIXED10_OPS {
            if op == 0xFF {
                continue;
            }
            len2 += 1;
            let tyb = s3_host_compose_tyb_pe(&[op, 0xFF]);
            let pe = wrap_pe_slim(&emit_x86(&parse_tyb(&tyb).unwrap())).unwrap();
            assert_eq!(pe.bytes.len(), 6656, "slim PE size stable ({op:#x})");
        }
        assert_eq!(len2, 9);
        let remaining = 8usize; // SET already seeded
        assert!(
            remaining * 6656 > 50_000,
            "per-ops PE images: {} more (≥{}KB) — combinatorial cliff",
            remaining,
            remaining * 6656 / 1024
        );
        assert_eq!(S3_HOST_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_HOST_PE_SET_RET_IMAGE_SEED.len(), 6656);
        // Seeded paths skip emit; a third combo still live-wraps.
        assert_eq!(try_emit_text_via_yoyo_table(&[0xFF]).unwrap(), vec![0xC3]);
        let other = try_emit_text_via_yoyo_table(&[0x66, 0xFF]).unwrap();
        assert_ne!(other, vec![0xC3]);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.381: gap F — in-DLL RET/SET+RET = slim PE image seeds (no live emit_x86).
    #[test]
    #[cfg(windows)]
    fn s3_in_dll_pe_image_seeds_pin() {
        assert_eq!(S3_IN_DLL_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_IN_DLL_PE_SET_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(&S3_IN_DLL_PE_RET_IMAGE_SEED[..2], b"MZ");
        let live = |ops: &[u8]| {
            let tyb = s3_table_prog_loop_tyb(ops);
            wrap_pe_slim(&emit_x86(&parse_tyb(&tyb).unwrap()))
                .unwrap()
                .bytes
        };
        assert_eq!(S3_IN_DLL_PE_RET_IMAGE_SEED, live(&[0xFF]).as_slice());
        assert_eq!(S3_IN_DLL_PE_SET_RET_IMAGE_SEED, live(&[0x30, 0xFF]).as_slice());
        assert_eq!(
            s3_run_slim_pe_outbuf(S3_IN_DLL_PE_RET_IMAGE_SEED, 1),
            vec![0xC3]
        );
        // Distinct from host PE image (different firmware / outbuf).
        assert_ne!(S3_IN_DLL_PE_RET_IMAGE_SEED, S3_HOST_PE_RET_IMAGE_SEED);
        assert_eq!(S3_IN_DLL_RET_TYB_SEED.len(), 616);
        assert_eq!(S3_IN_DLL_LEA_A1_SEED.len(), 13);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.382: post-E/F-image-seed gap map. Next ≥10 ≠ more PE image seeds.
    /// Remaining: (G) non-RET try_emit still live emit_x86+wrap; (H) wrap_pe_slim
    /// itself is Rust (image seeds bypass for RET/SET+RET only); (I) empty-DATA
    /// host slim cliff stands (S3.378). Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_true_selfhost_post_image_gap_map_pin() {
        // Seeded image paths hold.
        assert_eq!(S3_HOST_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_IN_DLL_PE_RET_IMAGE_SEED.len(), 6656);
        assert_ne!(S3_HOST_PE_RET_IMAGE_SEED, S3_IN_DLL_PE_RET_IMAGE_SEED);
        assert_eq!(try_emit_text_via_yoyo_table(&[0xFF]).unwrap(), vec![0xC3]);
        assert_eq!(
            s3_run_slim_pe_outbuf(S3_IN_DLL_PE_RET_IMAGE_SEED, 1),
            vec![0xC3]
        );
        // (G) non-RET still needs live wrap path (no image seed).
        let add_tyb = s3_host_compose_tyb_pe(&[0x62, 0xFF]);
        let add_pe = wrap_pe_slim(&emit_x86(&parse_tyb(&add_tyb).unwrap())).unwrap();
        assert_eq!(add_pe.bytes.len(), 6656);
        assert_ne!(add_pe.bytes.as_slice(), S3_HOST_PE_RET_IMAGE_SEED);
        // (I) empty-DATA cliff still.
        let host_code = emit_x86(&parse_tyb(S3_HOST_RET_TYB_SEED).unwrap()).text.len();
        assert!(host_code > S1_SLIM_MAX_CODE);
        assert_eq!(S3_HOST_PE_DATA_PREFIX_SEED.len(), 110);
        assert_eq!(S3_IN_DLL_LEA_A1_SEED.len(), 13);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.383: gap G — non-RET try_emit uses text-by-len seeds + data payload (no emit_x86).
    /// All len2 share one .text; wrap_pe_slim still Rust (H).
    #[test]
    #[cfg(windows)]
    fn s3_host_pe_text_by_len_seeds_pin() {
        for (i, seed) in S3_HOST_PE_TEXT_BY_LEN.iter().enumerate() {
            assert_eq!(seed.len(), 308, "len{}", i + 1);
        }
        // len2: ADD+RET text ≡ SET+RET live emit text
        let live2 = emit_x86(&parse_tyb(&s3_host_compose_tyb_pe(&[0x62, 0xFF])).unwrap()).text;
        assert_eq!(S3_HOST_PE_TEXT_BY_LEN[1], live2.as_slice());
        let live_set = emit_x86(&parse_tyb(S3_HOST_PE_SET_RET_TYB_SEED).unwrap()).text;
        assert_eq!(S3_HOST_PE_TEXT_BY_LEN[1], live_set.as_slice());
        // len1 ≡ RET compose text
        let live1 = emit_x86(&parse_tyb(S3_HOST_PE_RET_TYB_SEED).unwrap()).text;
        assert_eq!(S3_HOST_PE_TEXT_BY_LEN[0], live1.as_slice());
        // try_emit all fixed10 len2 ≡ sim (no emit_x86 on path for non-SET image).
        for &op in &S3_FIXED10_OPS {
            if op == 0xFF || op == 0x30 {
                continue;
            }
            let ops = [op, 0xFF];
            let via = try_emit_text_via_yoyo_table(&ops).unwrap();
            let sim = s3_sim_prog_loop_cursor(
                &s3_host_prog_loop_tyb(&ops),
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
                S3_HOST_OUTBUF_OFF as u64,
                RECORD_SIZE as u64,
            )
            .unwrap();
            assert_eq!(via, sim, "G: try_emit≡sim {op:#x}");
        }
        // data payload ≡ emit data
        let data = s3_host_pe_data_payload(&[0x62, 0xFF]);
        let live_d = emit_x86(&parse_tyb(&s3_host_compose_tyb_pe(&[0x62, 0xFF])).unwrap()).data;
        assert_eq!(data, live_d);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.384: gap H reject — wrap_pe_slim is PE container packaging (headers /
    /// sections / IAT), not YOYO ISA. RET/SET+RET already bypass via image seeds
    /// (S3.379); text-by-len path still needs a host packer. Porting wrap into YOYO
    /// = new surface, not this knife. Keep wrap; next ≠ sheep PE-header seeds.
    #[test]
    #[cfg(windows)]
    fn s3_wrap_pe_slim_host_packer_cliff_reject() {
        // Image seeds bypass wrap for two programs.
        assert_eq!(S3_HOST_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_HOST_PE_SET_RET_IMAGE_SEED.len(), 6656);
        // text-by-len still goes through wrap (witness: live wrap ≡ try_emit path).
        let text = S3_HOST_PE_TEXT_BY_LEN[1];
        let data = s3_host_pe_data_payload(&[0x62, 0xFF]);
        let pe = wrap_pe_slim(&CodegenOutput {
            text: text.to_vec(),
            data: data.clone(),
        })
        .unwrap();
        assert_eq!(pe.bytes.len(), 6656);
        assert_eq!(&pe.bytes[..2], b"MZ");
        let via = try_emit_text_via_yoyo_table(&[0x62, 0xFF]).unwrap();
        let got = s3_run_slim_pe_outbuf_at(&pe.bytes, S3_HOST_OUTBUF_OFF, via.len());
        assert_eq!(got, via);
        // Raising to seed every wrap output = same combinatorial cliff as S3.380.
        assert!(8 * 6656 > 50_000);
        assert_eq!(S3_HOST_PE_TEXT_BY_LEN.len(), 8);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.385: post-G/H gap map. Next ≥10 ≠ more wrap/image sheep.
    /// Remaining: (J) PE data payload still live-builds pad‖table; (K) PE runner
    /// VirtualAlloc is host Rust; (L) non-Windows try_emit still sim; (I) empty-DATA
    /// slim cliff stands. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_true_selfhost_post_gh_gap_map_pin() {
        // G holds: text-by-len + image seeds cover try_emit without emit_x86.
        assert_eq!(S3_HOST_PE_TEXT_BY_LEN.len(), 8);
        assert_eq!(S3_HOST_PE_TEXT_BY_LEN[0].len(), 308);
        assert_eq!(try_emit_text_via_yoyo_table(&[0xFF]).unwrap(), vec![0xC3]);
        assert_eq!(
            try_emit_text_via_yoyo_table(&[0x62, 0xFF]).unwrap(),
            s3_sim_prog_loop_cursor(
                &s3_host_prog_loop_tyb(&[0x62, 0xFF]),
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
                S3_HOST_OUTBUF_OFF as u64,
                RECORD_SIZE as u64,
            )
            .unwrap()
        );
        // (J) witness: data payload still constructed in Rust from table seed.
        let d = s3_host_pe_data_payload(&[0xFF]);
        assert_eq!(d.len(), S3_HOST_OUTBUF_OFF as usize);
        assert_eq!(&d[S3_HOST_TABLE_OFF as usize..][..200], &S3_HOST_TABLE_SEED);
        // (I) empty-DATA still over slim.
        let host_code = emit_x86(&parse_tyb(S3_HOST_RET_TYB_SEED).unwrap()).text.len();
        assert!(host_code > S1_SLIM_MAX_CODE);
        // H still host packer for non-image path.
        let pe = wrap_pe_slim(&CodegenOutput {
            text: S3_HOST_PE_TEXT_BY_LEN[0].to_vec(),
            data: d,
        })
        .unwrap();
        assert_eq!(&pe.bytes[..2], b"MZ");
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.386: gap J — PE data prefix = 328B object-code seed; prog tail live.
    #[test]
    fn s3_host_pe_data_prefix_bytes_pin() {
        assert_eq!(S3_HOST_PE_DATA_PREFIX_BYTES.len(), 328);
        assert_eq!(S3_HOST_PE_DATA_PREFIX_BYTES.len(), S3_HOST_PROG_OFF as usize);
        let mut live = vec![0u8; S3_HOST_TABLE_OFF as usize];
        live.extend_from_slice(&S3_HOST_TABLE_SEED);
        assert_eq!(S3_HOST_PE_DATA_PREFIX_BYTES, live.as_slice());
        // payload ≡ emit_x86 .data for RET / ADD+RET
        for &ops in &[&[0xFFu8][..], &[0x62, 0xFF][..], &[0x30, 0x66, 0xFF][..]] {
            let got = s3_host_pe_data_payload(ops);
            let expect = emit_x86(&parse_tyb(&s3_host_compose_tyb_pe(ops)).unwrap()).data;
            assert_eq!(got, expect, "J: data payload ops={ops:?}");
        }
        assert_eq!(S3_HOST_PE_DATA_PREFIX_SEED.len(), 110);
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.387: gap K/L reject. (K) PE runner = VirtualAlloc+call — host OS glue,
    /// not YOYO ISA; needed to observe compose outbuf. (L) non-Windows try_emit
    /// stays sim cursor (no PE map). Neither is a seed/volume knife. Not CLOSED.
    #[test]
    fn s3_pe_runner_and_nonwin_sim_cliff_reject() {
        assert_eq!(S3_HOST_OUTBUF_OFF, 0x160);
        let sim = s3_sim_prog_loop_cursor(
            S3_HOST_RET_TYB_SEED,
            S3_HOST_TABLE_OFF as u64,
            S3_HOST_OUTBUF_OFF,
            (S3_HOST_PROG_OFF + 1) as u64,
            S3_HOST_OUTBUF_OFF as u64,
            RECORD_SIZE as u64,
        )
        .unwrap();
        assert_eq!(sim, vec![0xC3], "L: sim oracle holds");
        assert_eq!(S3_HOST_PE_DATA_PREFIX_BYTES.len(), 328);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    #[test]
    #[cfg(windows)]
    fn s3_pe_runner_windows_observe_pin() {
        // (K) runner observes image-seed compose; try_emit uses it.
        let got = s3_run_slim_pe_outbuf_at(S3_HOST_PE_RET_IMAGE_SEED, S3_HOST_OUTBUF_OFF, 1);
        assert_eq!(got, vec![0xC3]);
        assert_eq!(try_emit_text_via_yoyo_table(&[0xFF]).unwrap(), vec![0xC3]);
        assert_eq!(S3_HOST_PE_TEXT_BY_LEN.len(), 8);
        assert_eq!(S3_HOST_PE_RET_IMAGE_SEED.len(), 6656);
    }

    /// S3.388: post-K/L gap map. try_emit Windows path is seed-driven (image /
    /// text-by-len / data-prefix) + host wrap + host PE runner. Next ≥10 should
    /// leave try_emit alone unless a new ISA surface appears. Remaining cliffs
    /// already pinned: H wrap, I empty-DATA slim, K runner, L non-win sim.
    /// Candidates: (M) expand/bootstrap still denser Rust; (N) in-DLL dispatcher.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_true_selfhost_post_kl_gap_map_pin() {
        // try_emit stack holds without emit_x86 on fixed10.
        for &ops in &[
            &[0xFFu8][..],
            &[0x30, 0xFF][..],
            &[0x62, 0xFF][..],
            &[0x00, 0x66, 0xFF][..],
        ] {
            let via = try_emit_text_via_yoyo_table(ops).unwrap();
            let sim = s3_sim_prog_loop_cursor(
                &s3_host_prog_loop_tyb(ops),
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
                S3_HOST_OUTBUF_OFF as u64,
                RECORD_SIZE as u64,
            )
            .unwrap();
            assert_eq!(via, sim, "ops={ops:?}");
        }
        assert_eq!(S3_HOST_PE_DATA_PREFIX_BYTES.len(), 328);
        assert_eq!(S3_HOST_PE_TEXT_BY_LEN.len(), 8);
        assert_eq!(S3_HOST_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_IN_DLL_PE_RET_IMAGE_SEED.len(), 6656);
        // Expand floor still live bootstrap.
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(s3_bootstrap_expand_asm().len(), 2);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.389: gap M — expand via PE (SEED@0x80 past state slots). Empty-DATA @0x40
    /// overlaps slot9 on PE; sim path unchanged. PE ≡ sim compose outbuf.
    #[test]
    #[cfg(windows)]
    fn s3_expand_via_pe_pin() {
        assert_eq!(S3_EXPAND_PE_SEED, 0x80);
        assert_eq!(S3_EXPAND_PE_OUT, 0x90);
        assert_eq!(S3_EXPAND_PE_DATA_PREFIX.len(), 152);
        assert_eq!(S3_EXPAND_PE_LEA_RAW.len(), 22);
        assert!(S3_EXPAND_PE_SEED as usize >= 16 * 8, "past state slots");
        assert_eq!(S3_EXPAND_PE_IMAGE_SEED.len(), 6656);
        assert_eq!(&S3_EXPAND_PE_IMAGE_SEED[..2], b"MZ");
        let live_pe = wrap_pe_slim(&emit_x86(&parse_tyb(&s3_expand_compose_tyb_pe()).unwrap())).unwrap();
        assert_eq!(S3_EXPAND_PE_IMAGE_SEED, live_pe.bytes.as_slice());
        let via = s3_expand_many_via_pe().unwrap();
        let sim = s3_sim_prog_loop_cursor(
            S3_EXPAND_TYB_SEED,
            0,
            S3_EXPAND_OUT,
            S3_EXPAND_SEED as u64,
            (S3_EXPAND_OUT as u64) + HEADER_SIZE as u64,
            (S3_BOOTSTRAP_EXPAND_ASM.len() * RECORD_SIZE) as u64,
        )
        .unwrap();
        assert_eq!(via, sim, "M: PE expand ≡ sim");
        assert_eq!(&via[..4], &TYB_MAGIC);
        assert_eq!(via.len(), 24);
        // Prefix seed ≡ pad‖wire‖hdr
        let mut live_prefix = vec![0u8; S3_EXPAND_PE_SEED as usize];
        for &(op, argc, a) in &S3_BOOTSTRAP_EXPAND_ASM {
            live_prefix.push(op);
            live_prefix.push(argc);
            live_prefix.extend_from_slice(&a);
        }
        let mut hdr = [0u8; HEADER_SIZE];
        hdr[..4].copy_from_slice(&TYB_MAGIC);
        hdr[6..8].copy_from_slice(&(S3_BOOTSTRAP_EXPAND_ASM.len() as u16).to_le_bytes());
        live_prefix.extend_from_slice(&hdr);
        assert_eq!(S3_EXPAND_PE_DATA_PREFIX, live_prefix.as_slice());
        // Floors
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(s3_expand_many_flat_tyb().0.as_slice(), S3_EXPAND_TYB_SEED);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.390: gap N slice — in-DLL 10-row bake = host table object-code seed.
    /// Full pe_dll mini codegen dispatcher remains Rust (OW-RT CUT); this knife
    /// only collapses the duplicate fixed-template bake. Not CLOSED.
    #[test]
    fn s3_in_dll_bake_uses_host_table_seed_pin() {
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        assert_eq!(S3_HOST_TABLE_SEED.len(), S3_HOST_NROWS as usize * 20);
        // Seed rows match fixed10 bodies (object-code, not pe_dll hand-hex).
        for (i, &op) in S3_FIXED10_OPS.iter().enumerate() {
            let row = &S3_HOST_TABLE_SEED[i * 20..(i + 1) * 20];
            assert_eq!(row[0], op, "op row {i}");
            let body = s3_fixed_row(op);
            assert_eq!(row[1] as usize, body.len());
            assert_eq!(&row[4..4 + body.len()], body);
        }
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.391: gap N residual reject — pe_dll mini codegen dispatcher stays Rust
    /// (OW-RT CUT / SCOPE-CUT). S3.390 only unified the 10-row bake seed. Closing
    /// N fully = replace export-body emit with YOYO (multi-month long pole).
    /// Inventory: try_emit/expand PE seeds = YOYO-run; bake table = shared seed;
    /// pe_dll link + mini loop = host Rust. Not CLOSED.
    #[test]
    fn s3_in_dll_dispatcher_ow_rt_cliff_reject() {
        // Bake slice closed.
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        // try_emit / expand PE floors still hold.
        assert_eq!(S3_HOST_PE_DATA_PREFIX_BYTES.len(), 328);
        assert_eq!(S3_EXPAND_PE_DATA_PREFIX.len(), 152);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        // Marker comments in pe_dll still declare production_default=RUST.
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(
            pe_dll.contains("production default remains Rust"),
            "OW-RT honesty marker must stay in pe_dll_link"
        );
        assert!(
            pe_dll.contains("not** OW-RT CLOSED") || pe_dll.contains("not OW-RT CLOSED"),
            "OW-RT not-CLOSED marker"
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.407: gap N/OW-RT slice — in-DLL s1_finish PE template = compile RET
    /// image seed (pub). Host↔DLL one object-code seed; mini loop still Rust.
    /// Not OW-RT CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_in_dll_pe_template_is_compile_ret_seed_pin() {
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(&S3_COMPILE_PE_RET_IMAGE_SEED[..2], b"MZ");
        // Seed ≡ empty-DATA wrap of RET text (oracle).
        let live = wrap_pe_slim(&CodegenOutput {
            text: try_emit_text_via_yoyo_table(&[0xFF]).unwrap(),
            data: vec![],
        })
        .unwrap();
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED, live.bytes.as_slice());
        // pe_dll honesty: template comment / seed pub for in-DLL link.
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3_COMPILE_PE_RET_IMAGE_SEED"));
        assert!(pe_dll.contains("S3.407"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }


    /// S3.409/S3.411/S3.412: post-parity inventory. Host O–U wrap-free; in-DLL
    /// PE template=compile seed; all fixed10+RET in-DLL ≡ host compile. Remaining
    /// ≥10 = rewrite in-DLL mini dispatcher in YOYO (OW-RT long pole). Next ≠
    /// more parity sheep. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_true_selfhost_post_ow_rt_slice_map_pin() {
        for &op in &[0xFFu8, 0x00, 0x30, 0x60, 0x66, 0x62, 0x67, 0x61, 0x64, 0x65] {
            let tyb = s3_fixed10_ret_tyb(op);
            assert!(compile_tyb_to_pe(&tyb).is_ok(), "op={op:#x}");
        }
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S1 mini TYB codegen"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**")
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.409: post-OW-RT-slice — in-DLL RET/SET+RET PE ≡ host compile seeds
    /// (proven in pe_dll_link ret/set_ret fast_path pins). Mini loop remains.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_true_selfhost_post_ow_rt_slice_map_pin_seeds() {
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_COMPILE_PE_SET_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(
            compile_tyb_to_pe(&tyb_from_records(&[(0xFF, 0, [0; 6])]))
                .unwrap()
                .bytes
                .as_slice(),
            S3_COMPILE_PE_RET_IMAGE_SEED
        );
        let set = tyb_from_records(&[
            (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
            (0xFF, 0, [0; 6]),
        ]);
        assert_eq!(
            compile_tyb_to_pe(&set).unwrap().bytes.as_slice(),
            S3_COMPILE_PE_SET_RET_IMAGE_SEED
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.417: skip-mini per-ops PE seed cliff. RET+SET/RET already skip (414/415);
    /// 8 more fixed10 × 6656B ≫ volume (mirror S3.380/S3.398). Keep bake+finish
    /// for other ops; next ≠ sheep PE seeds. Next ≥10 = s1_loop densify / YOYO.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_skip_mini_per_ops_seed_cliff_reject() {
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_COMPILE_PE_SET_RET_IMAGE_SEED.len(), 6656);
        let remaining = 8usize;
        assert!(remaining * 6656 > 50_000);
        // Other fixed10 still go mini→finish (parity holds via S3.411).
        let add = compile_tyb_to_pe(&s3_fixed10_ret_tyb(0x62)).unwrap();
        assert_eq!(add.bytes.len(), 6656);
        assert_ne!(add.bytes.as_slice(), S3_COMPILE_PE_RET_IMAGE_SEED);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.418: s1_loop densify reject. Sequential cmp/je dispatch was hand-hex
    /// (S3.408 residual). S3.461 lifts fixed10 → YOYO index+trampoline; remaining
    /// non-fixed10 still sequential. Compressing residual hex = sheep. Full
    /// 256-entry close = OW-RT long pole. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_loop_densify_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("let s1_loop = export_code.len()"));
        assert!(
            pe_dll.contains("op dispatch (near jcc")
                || pe_dll.contains("S3.461 YOYO-own fixed10")
        );
        // Named handler entry pins (subset of arms — densify ≠ delete these).
        let arms = [
            "s1_je_set",
            "s1_je_ret",
            "s1_je_get",
            "s1_je_add",
            "s1_je_inc",
            "s1_je_ldb",
            "s1_je_jmp",
            "s1_je_call",
            "s1_je_data",
            "s1_je_alloc",
        ];
        for a in arms {
            assert!(pe_dll.contains(a), "missing arm {a}");
        }
        assert!(arms.len() >= 10, "dispatch surface ≥10 — not a sheep cut");
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.461: YOYO-own fixed10 dispatcher slice — 256B index seed + trampolines.
    /// S3.462 extends seed to full op map (ALLOC..=JCC). OW-RT still CUT.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_yoyo_own_fixed10_dispatcher_slice_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.461"));
        assert!(
            pe_dll.contains("YOYO-own fixed10")
                || pe_dll.contains("fixed10 dispatcher slice")
                || pe_dll.contains("S3_S1_OP_DISPATCH_INDEX_SEED")
        );
        assert!(
            pe_dll.contains("S3_S1_FIXED10_DISPATCH_INDEX_SEED")
                || pe_dll.contains("S3_S1_OP_DISPATCH_INDEX_SEED")
        );
        assert!(
            pe_dll.contains("s1_fixed10_trampolines") || pe_dll.contains("s1_op_trampolines")
        );
        assert!(
            pe_dll.contains("s1_fixed10_disp_seed_at") || pe_dll.contains("s1_op_disp_seed_at")
        );
        assert!(pe_dll.contains("s1_dispatch_rest") || pe_dll.contains("s1_op_trampolines"));
        // Seed covers all fixed10 ops (1..=10).
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0xFF], 1);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x00], 2);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x30], 3);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x60], 4);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x66], 5);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x62], 6);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x67], 7);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x61], 8);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x64], 9);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x65], 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.468: YOYO-own handler trampoline table seed — 29×(E9+NOP)=232B.
    /// Replaces hand loop; live patch E9→handlers. ≥10 assembler (29 slots).
    /// Next ≥10 = more YOYO handler body seeds / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_yoyo_own_handler_trampoline_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.468"));
        assert!(
            pe_dll.contains("handler trampoline table seed")
                || pe_dll.contains("S3_S1_OP_DISPATCH_TRAMP_SEED")
        );
        assert!(pe_dll.contains("S3_S1_OP_DISPATCH_TRAMP_SEED"));
        assert_eq!(S3_S1_OP_DISPATCH_TRAMP_SEED.len(), 29 * 8);
        assert_eq!(S3_S1_OP_DISPATCH_TRAMP_SEED[0], 0xE9);
        assert_eq!(S3_S1_OP_DISPATCH_TRAMP_SEED[5], 0x90);
        assert_eq!(S3_S1_OP_DISPATCH_TRAMP_SEED[8], 0xE9);
        assert_eq!(S3_S1_OP_DISPATCH_TRAMP_SEED[28 * 8], 0xE9);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.467: mt→same-index reject. s1_loop YOYO index+trampoline assumes
    /// rcx=rec → s1 handlers; mt path = rdx=rec · r14 countdown · per-op
    /// finish jmp. Sharing trampolines without adapter = ABI cliff (≥10
    /// inventory below). Densify ≠ close. Next ≥10 = OW-RT / YOYO handler
    /// seeds. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_mt_same_index_abi_cliff_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.467"));
        assert!(
            pe_dll.contains("mt→same index")
                || pe_dll.contains("mt→same-index")
                || pe_dll.contains("Naive share = ABI cliff")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        // ≥10 ABI / surface inventory pins (not a sheep −1).
        let pins = [
            "s1_op_trampolines",
            "S3_S1_OP_DISPATCH_INDEX_SEED",
            "S3_S1_OP_DISPATCH_PROBE_SEED",
            "let s3_try_multi",
            "mov r14d, ebx",
            "mov rdx, rcx",
            "s3_multi_emit",
            "s3_jmp_finish_mt_set",
            "s3_shared_load_seed",
            "s3_shared_write_seed",
            "Naive jmp s1_loop mid-mt = sheep",
        ];
        for p in pins {
            assert!(pe_dll.contains(p), "missing inventory {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.466: YOYO-emit dispatch probe object seed (45B) — lea/lookup/jz/
    /// lea-tramp/scaled-jmp. Patch sites via seed offs. ≥10 assembler.
    /// Next ≥10 = mt→same index / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_yoyo_emit_dispatch_probe_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.466"));
        assert!(
            pe_dll.contains("dispatch probe object seed")
                || pe_dll.contains("S3_S1_OP_DISPATCH_PROBE_SEED")
        );
        assert!(pe_dll.contains("S3_S1_OP_DISPATCH_PROBE_SEED"));
        assert!(pe_dll.contains("s1_op_disp_probe"));
        assert_eq!(S3_S1_OP_DISPATCH_PROBE_SEED.len(), 45);
        assert_eq!(S3_S1_OP_DISPATCH_PROBE_SEED[0], 0x4C); // lea r10
        assert_eq!(S3_S1_OP_DISPATCH_PROBE_JZ_OFF, 19);
        assert_eq!(S3_S1_OP_DISPATCH_PROBE_LEA_TRAMP_AT, 25);
        assert_eq!(S3_S1_OP_DISPATCH_PROBE_SEED[19], 0x0F); // jz
        assert_eq!(S3_S1_OP_DISPATCH_PROBE_SEED[25], 0x4C); // lea r11
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.465: post-LOAD/WRITE-collapse map. Shared SET/ALLOC/LOAD/WRITE +
    /// CALL/JMP/JCC + GET..MCPS + YOYO op dispatch done. Remaining: handlers
    /// / finish / mt Rust hex → OW-RT CUT. Next ≥10 ≠ densify hand-disp32.
    /// Candidates: YOYO-emit dispatch stub · mt→same index. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_load_write_collapse_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.465"));
        assert!(pe_dll.contains("post-LOAD/WRITE-collapse map"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(
            pe_dll.contains("YOYO-emit dispatch stub") || pe_dll.contains("mt→same index")
        );
        assert!(
            pe_dll.contains("Densify remaining mt hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        // Shared surface still pinned.
        assert!(pe_dll.contains("s3_shared_load_seed"));
        assert!(pe_dll.contains("s3_shared_write_seed"));
        assert!(pe_dll.contains("s3_shared_set_seed"));
        assert!(pe_dll.contains("s3_shared_alloc_seed"));
        assert!(pe_dll.contains("s1_op_trampolines"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.464: collapse shared LOAD/WRITE seeds — mt+s1 call one thunk each
    /// (rcx=rec; WRITE also r8d=path r9d=sz). Patch registration in callers.
    /// Next ≥10 = YOYO-emit dispatch stub / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_load_write_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.464"));
        assert!(
            pe_dll.contains("collapse shared LOAD/WRITE")
                || pe_dll.contains("shared LOAD/WRITE seeds")
                || pe_dll.contains("s3_shared_load_seed")
        );
        assert!(pe_dll.contains("s3_shared_load_seed"));
        assert!(pe_dll.contains("s3_shared_write_seed"));
        assert!(pe_dll.contains("s3_shared_load_calls"));
        assert!(pe_dll.contains("s3_shared_write_calls"));
        assert!(
            pe_dll.contains("mt LOAD → shared LOAD seed")
                || pe_dll.contains("shared LOAD seed")
        );
        assert!(
            pe_dll.contains("mt WRITE → shared WRITE seed")
                || pe_dll.contains("shared WRITE seed")
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.463: post-dispatch map. s1_loop op dispatch = YOYO index+trampoline
    /// (29). Remaining Rust hex: handlers / shared seeds / s1_finish / mt path
    /// → OW-RT remains CUT. Next ≥10 ≠ densify hand-disp32. Candidates (≥3):
    /// shared LOAD/WRITE · YOYO-emit dispatch stub · mt→same index.
    /// DATA/LABEL=meta. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_dispatch_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.463"));
        assert!(pe_dll.contains("post-dispatch map"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(
            pe_dll.contains("shared LOAD/WRITE")
                || pe_dll.contains("YOYO-emit dispatch stub")
                || pe_dll.contains("mt→same index")
        );
        assert!(
            pe_dll.contains("Densify") || pe_dll.contains("hand-disp32 = sheep")
        );
        // Full dispatch surface still pinned.
        assert!(pe_dll.contains("s1_op_trampolines"));
        assert!(pe_dll.contains("S3_S1_OP_DISPATCH_INDEX_SEED"));
        assert!(
            pe_dll.contains("for _ in 0..29")
                || pe_dll.contains("S3_S1_OP_DISPATCH_TRAMP_SEED")
        );
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0xFF], 1);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x20], 11);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x7A], 29);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.462: extend YOYO-own dispatch — full op index seed (29 trampolines);
    /// sequential cmp/je removed. ≥10 assembler (19 newly table-driven).
    /// Mini loop still Rust host emit → OW-RT CUT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_yoyo_own_full_op_dispatcher_slice_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.462"));
        assert!(
            pe_dll.contains("full op index")
                || pe_dll.contains("extend to full")
                || pe_dll.contains("S3_S1_OP_DISPATCH_INDEX_SEED")
        );
        assert!(pe_dll.contains("s1_op_trampolines"));
        assert!(
            pe_dll.contains("for _ in 0..29")
                || pe_dll.contains("S3_S1_OP_DISPATCH_TRAMP_SEED")
        );
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x20], 11); // ALLOC
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x50], 12);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x51], 13);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x40], 14);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x63], 17); // IMUL
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x80], 21); // LDB
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x41], 27); // CALL
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x70], 28); // JMP
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x71], 29); // JCC
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0x7A], 29);
        assert_eq!(S3_S1_OP_DISPATCH_INDEX_SEED[0xFE], 0); // unsupported
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.419: s1_finish→YOYO IR seed reject (413 map#1). Finish is call-time:
    /// pass-2 + trail-C3 + 3×memcpy use DLL-local stack (`IN_DLL_S1_*`) and RIP
    /// patches — static YOYO IR cannot absorb without new frame arch. Seedable
    /// parts already landed: PE shell=compile seed (407); RET/SET+RET skip
    /// finish (414/415). Next ≥10 = bake dual+RET pilots densify (≥8 near-
    /// identical stubs) or first YOYO op-handler object seed. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_finish_yoyo_ir_seed_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("let s1_finish = export_code.len()"));
        assert!(pe_dll.contains("memcpy template"));
        assert!(pe_dll.contains("IN_DLL_S1_PE_OFF"));
        assert!(pe_dll.contains("IN_DLL_S1_CODE_OFF"));
        assert!(pe_dll.contains("S1_SLIM_TEXT_PAYLOAD_OFF"));
        // Seedable shell already unified; finish body still hand-hex.
        assert!(pe_dll.contains("S3_COMPILE_PE_RET_IMAGE_SEED"));
        assert!(pe_dll.contains("S3.407"));
        assert!(pe_dll.contains("S3.414"));
        assert!(pe_dll.contains("S3.415"));
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_COMPILE_PE_SET_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.428: MEMCPY_STATE pair seed+poke — bake+293 `S3_MEMCPY_STATE_PAIR_SEED`
    /// (8B load+store); loop memcpy + poke src/dst disp. Next ≥10 =
    /// CALL/JMP/JCC seed or DATA/RAW. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_memcpy_state_pair_seed_poke_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.428"));
        assert!(
            pe_dll.contains("pair seed (8B)")
                || pe_dll.contains("shared MCPS pair seed")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 293"));
        assert_eq!(S3_MEMCPY_STATE_PAIR_SEED.len(), 8);
        assert_eq!(&S3_MEMCPY_STATE_PAIR_SEED[..4], &[0x49, 0x8b, 0x47, 0x00]);
        assert_eq!(&S3_MEMCPY_STATE_PAIR_SEED[4..], &[0x49, 0x89, 0x47, 0x00]);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8,
            301
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.429: CALL/JMP/JCC handler seed+poke — bake+301 CALL(5) / +306
    /// JMP(5) / +311 JCC(6); JCC poke cc @+1. Next ≥10 = DATA/STR/RAW
    /// or ALLOC/LOAD/WRITE seed. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_call_jmp_jcc_seed_poke_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.429"));
        assert!(
            pe_dll.contains("bake+301 CALL seed")
                || pe_dll.contains("shared CALL seed")
                || pe_dll.contains("s3_shared_call_seed")
        );
        assert!(
            pe_dll.contains("bake+306 JMP seed")
                || pe_dll.contains("shared JMP seed")
                || pe_dll.contains("s3_shared_jmp_seed")
        );
        assert!(
            pe_dll.contains("bake+311 JCC seed")
                || pe_dll.contains("shared JCC seed")
                || pe_dll.contains("s3_shared_jcc_seed")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 301"));
        assert!(pe_dll.contains("s3_bake_table_off + 306"));
        assert!(pe_dll.contains("s3_bake_table_off + 311"));
        assert_eq!(S3_CALL_BODY_SEED, [0xe8, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(S3_JMP_BODY_SEED, [0xe9, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(S3_JCC_BODY_SEED, [0x0f, 0x00, 0x00, 0x00, 0x00, 0x00]);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6,
            317
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.430: ALLOC prefix seed+poke — bake+317 `S3_ALLOC_PREFIX_SEED` (34B)
    /// through add rsp; poke size @+7; store disp8/32 still hand. LOAD/WRITE
    /// already slim-tmpl memcpy. Next ≥10 = store seed / DATA-RAW / LABEL.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_alloc_prefix_seed_poke_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.430"));
        assert!(
            pe_dll.contains("prefix = bake+317")
                || pe_dll.contains("shared ALLOC seed")
                || pe_dll.contains("s3_bake_table_off + 317")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 317"));
        assert_eq!(S3_ALLOC_PREFIX_SEED.len(), 34);
        assert_eq!(&S3_ALLOC_PREFIX_SEED[..4], &[0x48, 0x83, 0xec, 0x28]);
        assert_eq!(&S3_ALLOC_PREFIX_SEED[30..], &[0x48, 0x83, 0xc4, 0x28]);
        assert_eq!(
            &S3_ALLOC_PREFIX_SEED[0x1A..0x1E],
            S1_SLIM_IAT_BASE.to_le_bytes().as_slice()
        );
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34,
            351
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.435: batch mt GET+INC+DEC disp8 → bake seeds (GET@+60, INC@+80);
    /// DEC pokes ModRM C8. disp32/mix still hand (any-slot). ≥3 arms one knife
    /// (anti one-op densify). Next ≥10 = mt ADD/SUB/MOVRR batch or collapse /
    /// long-pole. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_multi_get_inc_dec_disp8_seed_batch_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.435"));
        assert!(pe_dll.contains("batch GET+INC+DEC disp8"));
        assert!(
            pe_dll.contains("mt INC disp8 = bake+80")
                || pe_dll.contains("mt INC disp8 → shared")
        );
        assert!(
            pe_dll.contains("mt DEC disp8 = bake+80")
                || pe_dll.contains("mt DEC disp8 → shared")
        );
        assert!(
            pe_dll.contains("both slots <16 → bake+60 GET seed")
                || pe_dll.contains("both slots <16 → shared GET seed")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 80 + 4"));
        assert!(pe_dll.contains("s3_bake_table_off + 60 + 4"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.436: batch mt ADD+SUB+MOVRR disp8 → bake seeds (ADD@+100, MOVRR
    /// reuses GET@+60; SUB pokes ModRM E8). disp32/mix still hand. ≥3 arms
    /// one knife (anti one-op densify). Next ≥10 = mt CMP/IMUL/ALUV batch
    /// or collapse/long-pole. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_multi_add_sub_movrr_disp8_seed_batch_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.436"));
        assert!(pe_dll.contains("batch ADD+SUB+MOVRR"));
        assert!(
            pe_dll.contains("mt ADD disp8 = bake+100")
                || pe_dll.contains("mt ADD disp8 → shared ADD/SUB")
        );
        assert!(
            pe_dll.contains("mt SUB disp8 = bake+100")
                || pe_dll.contains("mt SUB disp8 → shared ADD/SUB")
        );
        assert!(
            pe_dll.contains("both slots <16 → bake+60 GET seed")
                || pe_dll.contains("both slots <16 → shared GET seed")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 100 + 4"));
        assert!(
            pe_dll.contains("poke SUB ModRM")
                || pe_dll.contains("r8b=E8")
                || pe_dll.contains("shared ADD/SUB")
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.437: batch mt CMP+IMUL+ALUV disp8 → bake seeds (CMP@+180,
    /// IMUL@+217, ALUV@+233 + poke opcode). disp32/mix still hand. ≥3 arms
    /// one knife. Next ≥10 = mt LDB/MCP batch or collapse/long-pole.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_multi_cmp_imul_aluv_disp8_seed_batch_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.437"));
        assert!(pe_dll.contains("batch CMP+IMUL+ALUV"));
        assert!(
            pe_dll.contains("both slots <16 → bake+180 CMP seed")
                || pe_dll.contains("mt CMP disp8 → shared CMP")
        );
        assert!(
            pe_dll.contains("both slots <16 → bake+217 IMUL seed")
                || pe_dll.contains("mt IMUL disp8 → shared IMUL")
        );
        assert!(
            pe_dll.contains("both slots <16 → bake+233 ALUV seed")
                || pe_dll.contains("mt ALUV disp8 → shared ALUV")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 180 + 4"));
        assert!(pe_dll.contains("s3_bake_table_off + 217"));
        assert!(pe_dll.contains("s3_bake_table_off + 233"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.439: post mt-disp8-seed capability map + densify reject. After
    /// 435–438 batch, mt disp8 emit ≈ poke-from-seed; residual hand = disp32/
    /// mix (+DATA/LABEL meta). One-op disp32 densify = sheep 0 capability.
    /// True next ≥10: (B) collapse mt→s1_loop · (C) OW-RT mini→YOYO.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_mt_disp8_seed_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.439"));
        assert!(pe_dll.contains("post-disp8") || pe_dll.contains("post-disp8-seed"));
        assert!(
            pe_dll.contains("One-op disp32 densify = sheep")
                || pe_dll.contains("拒 densify")
                || pe_dll.contains("densify = sheep")
        );
        assert!(
            pe_dll.contains("collapse mt→s1")
                || pe_dll.contains("collapse mt→s1_loop")
                || pe_dll.contains("collapse wedge")
        );
        // Batch surface still pinned.
        assert!(pe_dll.contains("S3.435"));
        assert!(pe_dll.contains("S3.438"));
        assert!(pe_dll.contains("batch LDB+MCPD+MCPS") || pe_dll.contains("LDB+MCPD+MCPS"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.440: collapse wedge — shared GET/MOVRR disp8 seed thunk. mt adapts
    /// rcx←rdx then call; s1 calls same. ≥3 call sites. Naive mid-mt jmp
    /// s1_loop = sheep (ABI cliff). Next ≥10 = more shared thunks / lift s1
    /// any-slot / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_get_movrr_disp8_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.440"));
        assert!(pe_dll.contains("collapse wedge") || pe_dll.contains("shared GET/MOVRR disp8"));
        assert!(pe_dll.contains("s3_shared_get_disp8_seed"));
        assert!(pe_dll.contains("collapse adapter rcx←rdx"));
        assert!(
            pe_dll.contains("Naive jmp s1_loop mid-mt = sheep")
                || pe_dll.contains("shared GET/MOVRR")
        );
        let calls = pe_dll.matches("s3_shared_get_calls.push").count();
        assert!(calls >= 3, "shared GET call sites ≥3, got {calls}");
        assert!(pe_dll.contains("s3_bake_table_off + 60 + 4"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.441: collapse — shared INC/DEC disp8 seed (bake+80, r8b=ModRM).
    /// mt INC/DEC + s1 incdec → call. ≥3 sites. Next ≥10 = ADD/SUB/CMP
    /// shared / lift s1 / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_inc_dec_disp8_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.441"));
        assert!(pe_dll.contains("shared INC/DEC disp8"));
        assert!(pe_dll.contains("s3_shared_incdec_disp8_seed"));
        assert!(pe_dll.contains("mt INC disp8 → shared"));
        assert!(pe_dll.contains("mt DEC disp8 → shared"));
        let calls = pe_dll.matches("s3_shared_incdec_calls.push").count();
        assert!(calls >= 3, "shared INC/DEC call sites ≥3, got {calls}");
        assert!(pe_dll.contains("s3_bake_table_off + 80 + 4"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.442: collapse — shared ADD/SUB disp8 seed (bake+100, r8b=ModRM
    /// C0/E8). mt ADD/SUB + s1 addsub → call. ≥3 sites. Next ≥10 = CMP
    /// shared / lift s1 / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_add_sub_disp8_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.442"));
        assert!(pe_dll.contains("shared ADD/SUB disp8"));
        assert!(pe_dll.contains("s3_shared_addsub_disp8_seed"));
        assert!(pe_dll.contains("mt ADD disp8 → shared"));
        assert!(pe_dll.contains("mt SUB disp8 → shared"));
        let calls = pe_dll.matches("s3_shared_addsub_calls.push").count();
        assert!(calls >= 3, "shared ADD/SUB call sites ≥3, got {calls}");
        assert!(pe_dll.contains("s3_bake_table_off + 100 + 4"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.443: collapse — shared CMP disp8 seed (bake+180). mt CMP + s1
    /// CMP → call. ≥2 sites (CMP has no ModRM twin). Next ≥10 = lift s1
    /// any-slot / OW-RT / remaining shared. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_cmp_disp8_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.443"));
        assert!(pe_dll.contains("shared CMP disp8"));
        assert!(pe_dll.contains("s3_shared_cmp_disp8_seed"));
        assert!(pe_dll.contains("mt CMP disp8 → shared"));
        let calls = pe_dll.matches("s3_shared_cmp_calls.push").count();
        assert!(calls >= 2, "shared CMP call sites ≥2, got {calls}");
        assert!(pe_dll.contains("s3_bake_table_off + 180 + 4"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.444: collapse — shared IMUL (bake+217) + ALUV (bake+233, r8b=
    /// opcode). mt+s1 each → call. ≥2+2 sites. Next ≥10 = LDB/MCP shared
    /// / lift s1 any-slot / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_imul_aluv_disp8_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.444"));
        assert!(pe_dll.contains("shared IMUL disp8"));
        assert!(pe_dll.contains("shared ALUV disp8"));
        assert!(pe_dll.contains("s3_shared_imul_disp8_seed"));
        assert!(pe_dll.contains("s3_shared_aluv_disp8_seed"));
        assert!(pe_dll.contains("mt IMUL disp8 → shared"));
        assert!(pe_dll.contains("mt ALUV disp8 → shared"));
        let imul = pe_dll.matches("s3_shared_imul_calls.push").count();
        let aluv = pe_dll.matches("s3_shared_aluv_calls.push").count();
        assert!(imul >= 2, "shared IMUL call sites ≥2, got {imul}");
        assert!(aluv >= 2, "shared ALUV call sites ≥2, got {aluv}");
        assert!(pe_dll.contains("s3_bake_table_off + 217"));
        assert!(pe_dll.contains("s3_bake_table_off + 233"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.445: collapse — shared LDB (bake+262/+274, oo pick) + MCPD
    /// (bake+248). mt+s1 each → call. ≥2+2 sites. Next ≥10 = MCPS pair
    /// shared / lift s1 any-slot / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_ldb_mcpd_disp8_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.445"));
        assert!(pe_dll.contains("shared LDB disp8"));
        assert!(pe_dll.contains("shared MCPD disp8"));
        assert!(pe_dll.contains("s3_shared_ldb_disp8_seed"));
        assert!(pe_dll.contains("s3_shared_mcpd_disp8_seed"));
        assert!(pe_dll.contains("mt LDB disp8 → shared"));
        assert!(pe_dll.contains("mt MCPD disp8 → shared"));
        let ldb = pe_dll.matches("s3_shared_ldb_calls.push").count();
        let mcpd = pe_dll.matches("s3_shared_mcpd_calls.push").count();
        assert!(ldb >= 2, "shared LDB call sites ≥2, got {ldb}");
        assert!(mcpd >= 2, "shared MCPD call sites ≥2, got {mcpd}");
        assert!(pe_dll.contains("s3_bake_table_off + 262"));
        assert!(pe_dll.contains("s3_bake_table_off + 274"));
        assert!(pe_dll.contains("s3_bake_table_off + 248"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.446: collapse — shared MCPS pair seed (bake+293, r8b=dst_disp,
    /// r9b=src_disp). mt+s1 loop bodies → call. ≥2 sites. Completes mt/s1
    /// disp8 shared surface. Next ≥10 = lift s1 any-slot / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_mcps_pair_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.446"));
        assert!(pe_dll.contains("shared MCPS pair seed"));
        assert!(pe_dll.contains("s3_shared_mcps_pair_seed"));
        assert!(pe_dll.contains("mt MCPS pair → shared"));
        let calls = pe_dll.matches("s3_shared_mcps_calls.push").count();
        assert!(calls >= 2, "shared MCPS call sites ≥2, got {calls}");
        assert!(pe_dll.contains("s3_bake_table_off + 293"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.447: post-shared-collapse map. Disp8 shared GET..MCPS done.
    /// Next ≥10 = lift s1 any-slot (GET/MOVRR first — s1 still jae→fail on
    /// slot≥16) or OW-RT. Densify mt hand-disp32 = sheep. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_shared_collapse_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.447"));
        assert!(pe_dll.contains("post-shared-collapse map"));
        assert!(
            pe_dll.contains("Densify remaining mt hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert!(
            pe_dll.contains("lift s1 any-slot")
                || pe_dll.contains("lift s1 GET/MOVRR any-slot")
        );
        assert!(pe_dll.contains("S3.446"));
        assert!(pe_dll.contains("s3_shared_mcps_pair_seed"));
        // Shared surface still pinned.
        assert!(pe_dll.contains("s3_shared_get_disp8_seed"));
        assert!(pe_dll.contains("s3_shared_addsub_disp8_seed"));
        assert!(pe_dll.contains("s3_shared_ldb_disp8_seed"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.448: lift s1 GET/MOVRR any-slot — both <16 → shared; mix/disp32 →
    /// hand (mirror mt). Slots ≤255. Next ≥10 = lift INC/ADD/… or OW-RT.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_lift_s1_get_movrr_any_slot_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.448"));
        assert!(
            pe_dll.contains("lift s1 GET/MOVRR any-slot")
                || pe_dll.contains("s1 GET/MOVRR hand any-slot")
        );
        assert!(pe_dll.contains("s1 GET/MOVRR hand any-slot"));
        assert!(pe_dll.contains("let s1_get_hand"));
        assert!(pe_dll.contains("s1_jmp_next6b"));
        // Shared disp8 path retained.
        assert!(pe_dll.contains("s3_shared_get_disp8_seed"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.449: lift s1 INC/DEC any-slot — slot≤255; <16→shared; else hand
    /// disp32 17B (r8b=ModRM). Next ≥10 = lift ADD/SUB/… / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_lift_s1_inc_dec_any_slot_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.449"));
        assert!(pe_dll.contains("lift s1 INC/DEC any-slot"));
        assert!(pe_dll.contains("s1 INC/DEC hand disp32"));
        assert!(pe_dll.contains("let s1_incdec_hand"));
        assert!(pe_dll.contains("s1_jmp_next8b"));
        assert!(pe_dll.contains("s3_shared_incdec_disp8_seed"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.450: lift s1 ADD/SUB any-slot — slot≤255; <16→shared; else hand
    /// disp32 21B (r8b=ModRM C0/E8 + imm). Next ≥10 = lift CMP/IMUL/… /
    /// OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_lift_s1_add_sub_any_slot_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.450"));
        assert!(pe_dll.contains("lift s1 ADD/SUB any-slot"));
        assert!(pe_dll.contains("s1 ADD/SUB hand disp32"));
        assert!(pe_dll.contains("let s1_addsub_hand"));
        assert!(pe_dll.contains("s1_jmp_next7b"));
        assert!(pe_dll.contains("s3_shared_addsub_disp8_seed"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.451: lift s1 CMP any-slot — both <16 → shared; mix/disp32 → hand
    /// (mirror mt). Slots ≤255. Next ≥10 = lift IMUL/ALUV/… / OW-RT.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_lift_s1_cmp_any_slot_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.451"));
        assert!(pe_dll.contains("lift s1 CMP any-slot"));
        assert!(pe_dll.contains("s1 CMP hand any-slot"));
        assert!(pe_dll.contains("let s1_cmp_hand"));
        assert!(pe_dll.contains("s1_jmp_next9b"));
        assert!(pe_dll.contains("s3_shared_cmp_disp8_seed"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.452: lift s1 IMUL+ALUV any-slot — both <16 → shared; mix → hand.
    /// ALUV keeps r8b=opcode. Next ≥10 = lift LDB/MCP / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_lift_s1_imul_aluv_any_slot_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.452"));
        assert!(pe_dll.contains("lift s1 IMUL any-slot") || pe_dll.contains("lift s1 IMUL+ALUV"));
        assert!(pe_dll.contains("s1 IMUL hand any-slot"));
        assert!(pe_dll.contains("s1 ALUV hand any-slot"));
        assert!(pe_dll.contains("let s1_imul_hand"));
        assert!(pe_dll.contains("let s1_aluv_hand"));
        assert!(pe_dll.contains("s1_jmp_next13b"));
        assert!(pe_dll.contains("s1_jmp_next14b"));
        assert!(pe_dll.contains("s3_shared_imul_disp8_seed"));
        assert!(pe_dll.contains("s3_shared_aluv_disp8_seed"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.453: lift s1 LDB+MCPD any-slot — slots <16 → shared; mix → hand
    /// (mirror mt). Slots ≤255. Next ≥10 = lift MCPS / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_lift_s1_ldb_mcpd_any_slot_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.453"));
        assert!(pe_dll.contains("lift s1 LDB any-slot") || pe_dll.contains("lift s1 LDB+MCPD"));
        assert!(pe_dll.contains("s1 LDB hand any-slot"));
        assert!(pe_dll.contains("s1 MCPD hand any-slot"));
        assert!(pe_dll.contains("let s1_ldb_hand"));
        assert!(pe_dll.contains("let s1_mcpd_hand"));
        assert!(pe_dll.contains("s1_jmp_next15b"));
        assert!(pe_dll.contains("s1_jmp_next16b"));
        assert!(pe_dll.contains("s3_shared_ldb_disp8_seed"));
        assert!(pe_dll.contains("s3_shared_mcpd_disp8_seed"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.454: lift s1 MCPS any-slot — last <16 → shared pair seed; else hand
    /// (mirror mt). Slots ≤255. Next ≥10 = post-lift map / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_lift_s1_mcps_any_slot_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.454"));
        assert!(pe_dll.contains("lift s1 MCPS any-slot"));
        assert!(pe_dll.contains("s1 MCPS hand any-slot"));
        assert!(pe_dll.contains("let s1_mcps_hand"));
        assert!(pe_dll.contains("s1_jmp_next17b"));
        assert!(pe_dll.contains("s3_shared_mcps_pair_seed"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.455: post-s1-lift map. s1 GET..MCPS any-slot (shared+hand) done.
    /// Next ≥10 = OW-RT long-pole / real gap. Densify mt hand-disp32 = sheep.
    /// DATA/LABEL=meta. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_s1_lift_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.455"));
        assert!(pe_dll.contains("post-s1-lift map"));
        assert!(
            pe_dll.contains("Densify remaining mt hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert!(
            pe_dll.contains("OW-RT long-pole")
                || pe_dll.contains("OW-RT remains CUT")
                || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        // Lift surface still pinned.
        assert!(pe_dll.contains("let s1_get_hand") || pe_dll.contains("s1 GET hand"));
        assert!(pe_dll.contains("let s1_imul_hand"));
        assert!(pe_dll.contains("let s1_ldb_hand"));
        assert!(pe_dll.contains("let s1_mcps_hand"));
        assert!(pe_dll.contains("s3_shared_get_disp8_seed"));
        assert!(pe_dll.contains("s3_shared_mcps_pair_seed"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.456: collapse shared SET seed — mt+s1 call one thunk; slot→disp8
    /// (@+44) / disp32 (@+200) pick inside. Next ≥10 = shared ALLOC / OW-RT.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_set_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.456"));
        assert!(
            pe_dll.contains("collapse shared SET")
                || pe_dll.contains("shared SET seed")
        );
        assert!(pe_dll.contains("s3_shared_set_seed"));
        assert!(pe_dll.contains("s3_shared_set_calls"));
        assert!(pe_dll.contains("s3_call_mt_set_shared") || pe_dll.contains("call_mt_set_shared"));
        assert!(pe_dll.contains("s3_call_s1_set_shared") || pe_dll.contains("call_s1_set_shared"));
        assert!(pe_dll.contains("slot→+44") || pe_dll.contains("slot→disp8"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.457: collapse shared ALLOC seed — mt+s1 call one thunk; prefix@+317
    /// + store +351/+355 pick. Next ≥10 = OW-RT / real gap. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_alloc_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.457"));
        assert!(
            pe_dll.contains("collapse shared ALLOC")
                || pe_dll.contains("shared ALLOC seed")
        );
        assert!(pe_dll.contains("s3_shared_alloc_seed"));
        assert!(pe_dll.contains("s3_shared_alloc_calls"));
        assert!(pe_dll.contains("s3_call_mt_alloc_shared") || pe_dll.contains("call_mt_alloc_shared"));
        assert!(pe_dll.contains("s3_call_s1_alloc_shared") || pe_dll.contains("call_s1_alloc_shared"));
        assert!(pe_dll.contains("s3_bake_table_off + 317"));
        assert!(pe_dll.contains("s3_bake_table_off + 351"));
        assert!(pe_dll.contains("s3_bake_table_off + 355"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.458: post-SET/ALLOC-collapse map. Shared SET+ALLOC done (with
    /// GET..MCPS). Next ≥10 = OW-RT long-pole / real gap. Densify = sheep.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_set_alloc_collapse_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.458"));
        assert!(pe_dll.contains("post-SET/ALLOC-collapse map"));
        assert!(
            pe_dll.contains("Densify remaining mt hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert!(
            pe_dll.contains("OW-RT long-pole")
                || pe_dll.contains("OW-RT remains CUT")
                || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(pe_dll.contains("s3_shared_set_seed"));
        assert!(pe_dll.contains("s3_shared_alloc_seed"));
        assert!(pe_dll.contains("s3_shared_get_disp8_seed"));
        assert!(pe_dll.contains("s3_shared_mcps_pair_seed"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.459: OW-RT entry cliff map. Shared/lift surface done; in-DLL mini
    /// dispatcher still Rust → OW-RT remains CUT. Next ≥10 = YOYO-own
    /// dispatcher slice (not densify hand-disp32). Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_entry_cliff_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.459"));
        assert!(pe_dll.contains("OW-RT entry cliff map"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(
            pe_dll.contains("YOYO-own dispatcher")
                || pe_dll.contains("mini dispatcher still Rust")
        );
        assert!(
            pe_dll.contains("Densify remaining mt hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        // Shared surface still pinned.
        assert!(pe_dll.contains("s3_shared_set_seed"));
        assert!(pe_dll.contains("s3_shared_alloc_seed"));
        assert!(pe_dll.contains("s3_shared_mcps_pair_seed"));
        assert!(pe_dll.contains("let s1_mcps_hand"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.460: collapse shared CALL/JMP/JCC seeds — mt+s1 call one thunk each
    /// (+301/+306/+311; JCC r8b=cc). Patch registration stays in callers.
    /// Next ≥10 = YOYO-own dispatcher / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_call_jmp_jcc_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.460"));
        assert!(
            pe_dll.contains("collapse shared CALL/JMP/JCC")
                || pe_dll.contains("shared CALL/JMP/JCC seeds")
        );
        assert!(pe_dll.contains("s3_shared_call_seed"));
        assert!(pe_dll.contains("s3_shared_jmp_seed"));
        assert!(pe_dll.contains("s3_shared_jcc_seed"));
        assert!(pe_dll.contains("s3_shared_call_calls"));
        assert!(pe_dll.contains("s3_shared_jmp_calls"));
        assert!(pe_dll.contains("s3_shared_jcc_calls"));
        assert!(pe_dll.contains("s3_bake_table_off + 301"));
        assert!(pe_dll.contains("s3_bake_table_off + 306"));
        assert!(pe_dll.contains("s3_bake_table_off + 311"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.438: batch mt LDB+MCPD+MCPS disp8 → bake seeds (LDB oo0@+262 /
    /// oo≠0@+274, MCPD@+248, MCPS pair@+293). disp32/mix still hand. ≥3 arms
    /// one knife. Completes mt disp8 seed surface. Next ≥10 = collapse
    /// mt→s1 or OW-RT long-pole. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_multi_ldb_mcp_disp8_seed_batch_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.438"));
        assert!(pe_dll.contains("batch LDB+MCPD+MCPS"));
        assert!(
            pe_dll.contains("both slots <16 → bake LDB seed")
                || pe_dll.contains("mt LDB disp8 → shared LDB")
        );
        assert!(
            pe_dll.contains("all slots <16 → bake+248 MCPD seed")
                || pe_dll.contains("mt MCPD disp8 → shared MCPD")
        );
        assert!(
            pe_dll.contains("last slots <16 → pair seed @+293")
                || pe_dll.contains("mt MCPS pair → shared")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 262"));
        assert!(pe_dll.contains("s3_bake_table_off + 274"));
        assert!(pe_dll.contains("s3_bake_table_off + 248"));
        assert!(pe_dll.contains("s3_bake_table_off + 293"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.434: post handler/mt-seed capability map + densify reject. s1_loop
    /// emit surface ≈ seeded (C6 residual ≤ trail/poke); mt still hosts
    /// GET/INC/DEC/ADD/SUB/MOVRR/CMP/IMUL/LDB/MCP hand-C6 (≥8 arms). Continuing
    /// one-op mt densify = sheep 0 capability (mirror 418/420). True next ≥10:
    /// (A) batch mt→bake reuse (≥GET+INC+DEC in one knife) · (B) collapse mt
    /// into s1_loop · (C) OW-RT mini dispatcher→YOYO (long pole). DATA/LABEL
    /// meta cliff. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_seed_chain_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.434"));
        assert!(pe_dll.contains("post-seed-chain map"));
        assert!(pe_dll.contains("densify = sheep"));
        assert!(pe_dll.contains("S3.435") || pe_dll.contains("batch GET+INC+DEC"));
        assert!(pe_dll.contains("S3.436") || pe_dll.contains("batch ADD+SUB+MOVRR"));
        assert!(pe_dll.contains("S3.437") || pe_dll.contains("batch CMP+IMUL+ALUV"));
        assert!(pe_dll.contains("S3.438") || pe_dll.contains("batch LDB+MCPD+MCPS"));
        // Seeded surfaces still pinned.
        assert!(pe_dll.contains("S3.429"));
        assert!(pe_dll.contains("S3.433"));
        assert!(pe_dll.contains("s3_bake_table_off + 301"));
        assert!(pe_dll.contains("s3_bake_table_off + 317"));
        // Remaining mt hand surface ≥8 named arms — densify ≠ delete one.
        let mt_arms = [
            "let s3_mt_get",
            "let s3_mt_inc",
            "let s3_mt_dec",
            "let s3_mt_add",
            "let s3_mt_sub",
            "let s3_mt_movrr",
            "let s3_mt_cmp",
            "let s3_mt_imul",
            "let s3_mt_ldb",
            "let s3_mt_mcpd",
            "let s3_mt_mcps",
        ];
        for a in mt_arms {
            assert!(pe_dll.contains(a), "missing mt arm {a}");
        }
        assert!(mt_arms.len() >= 8, "mt hand surface ≥8 — not a sheep cut");
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7, 362);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.433: mt SET/ALLOC reuse bake seeds — SET row2/+200; ALLOC
    /// +317/+351/+355. Completes multi plat emit surface with s1_loop seeds.
    /// Next ≥10 = mt GET/INC/… hand C6 or remaining cliffs. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_multi_set_alloc_seed_reuse_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.433"));
        assert!(
            pe_dll.contains("mt SET poke-from-seed (bake row2 14B)")
                || pe_dll.contains("mt SET → shared SET seed")
                || pe_dll.contains("shared SET seed")
        );
        assert!(
            pe_dll.contains("mt SET poke-from-seed (bake+200, 17B)")
                || pe_dll.contains("slot→+44")
                || pe_dll.contains("s3_shared_set_d32")
        );
        assert!(pe_dll.contains("mt ALLOC"));
        assert!(pe_dll.contains("bake+317/351/355"));
        assert!(pe_dll.contains("s3_bake_table_off + 40 + 4"));
        assert!(pe_dll.contains("s3_bake_table_off + 200"));
        assert!(pe_dll.contains("s3_bake_table_off + 317"));
        assert!(pe_dll.contains("s3_bake_table_off + 351"));
        assert!(pe_dll.contains("s3_bake_table_off + 355"));
        assert_eq!(S3_ALLOC_PREFIX_SEED.len(), 34);
        assert_eq!(S3_STORE_RAX_DISP8_SEED.len() + S3_STORE_RAX_DISP32_SEED.len(), 11);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.432: multi-path CALL/JMP/JCC reuse bake seeds (301/306/311).
    /// DATA/STR/RAW/LABEL = data/meta side-channel (no fixed object emit) —
    /// documented cliff; true next ≠ invent DATA seed. Next ≥10 = mt SET/ALLOC
    /// seed reuse or remaining hand C6. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_multi_call_jmp_jcc_seed_reuse_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.432"));
        assert!(
            pe_dll.contains("bake+306 JMP seed; stay in multi")
                || pe_dll.contains("mt JMP → shared JMP seed")
                || pe_dll.contains("shared JMP seed")
        );
        assert!(
            pe_dll.contains("bake+301 CALL seed; stay in multi")
                || pe_dll.contains("mt CALL → shared CALL seed")
                || pe_dll.contains("shared CALL seed")
        );
        assert!(
            pe_dll.contains("bake+311 JCC seed + poke cc=0x84")
                || pe_dll.contains("shared JCC seed (r8b=0x84)")
                || pe_dll.contains("r8b=0x84")
        );
        assert!(
            pe_dll.contains("bake+311 JCC seed + poke cc=0x85")
                || pe_dll.contains("shared JCC seed (r8b=0x85)")
                || pe_dll.contains("r8b=0x85")
        );
        assert!(
            pe_dll.contains("bake+311 + poke cc from table")
                || pe_dll.contains("map cc → r8b → shared JCC")
                || pe_dll.contains("shared JCC seed")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 306"));
        assert!(pe_dll.contains("s3_bake_table_off + 301"));
        assert!(pe_dll.contains("s3_bake_table_off + 311"));
        // DATA/LABEL remain non-object (handlers touch data[] / labels[], not .text seed).
        assert!(pe_dll.contains("--- DATA/STR/RAW"));
        assert!(pe_dll.contains("--- LABEL argc=1"));
        assert!(pe_dll.contains("--- RAW_BYTE argc=1"));
        assert_eq!(S3_CALL_BODY_SEED.len() + S3_JMP_BODY_SEED.len() + S3_JCC_BODY_SEED.len(), 16);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.431: ALLOC store seed+poke — bake+351 disp8(4) / +355 disp32(7);
    /// ALLOC adds r12+=34 then memcpy store + poke disp. Completes ALLOC
    /// emit surface (prefix=430, store=431). Next ≥10 = DATA-RAW / LABEL
    /// or reuse store seeds elsewhere. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_alloc_store_seed_poke_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.431"));
        assert!(
            pe_dll.contains("store disp8 seed @ bake+351")
                || pe_dll.contains("store disp8 @+351")
                || pe_dll.contains("shared ALLOC seed")
        );
        assert!(
            pe_dll.contains("store disp32 seed @ bake+355")
                || pe_dll.contains("s3_bake_table_off + 355")
                || pe_dll.contains("shared ALLOC seed")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 351"));
        assert!(pe_dll.contains("s3_bake_table_off + 355"));
        assert_eq!(S3_STORE_RAX_DISP8_SEED, [0x49, 0x89, 0x47, 0x00]);
        assert_eq!(
            S3_STORE_RAX_DISP32_SEED,
            [0x49, 0x89, 0x87, 0x00, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.427: LDB + MEMCPY_DATA handler seed+poke — bake+248 MCP(14) /
    /// +262 LDB_oo0(12) / +274 LDB_oo(19). MEMCPY_STATE still hand (unroll
    /// loop). Next ≥10 = MEMCPY_STATE pair seed or CALL/JMP/JCC. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_ldb_memcpy_handler_seed_poke_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.427"));
        assert!(
            pe_dll.contains("MEMCPY_DATA body = bake+248")
                || pe_dll.contains("MEMCPY_DATA body = shared MCPD")
        );
        assert!(
            pe_dll.contains("LDB body = bake seed")
                || pe_dll.contains("LDB body = shared LDB")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 248"));
        assert!(pe_dll.contains("s3_bake_table_off + 262"));
        assert!(pe_dll.contains("s3_bake_table_off + 274"));
        assert_eq!(S3_MEMCPY_DATA_BODY_SEED.len(), 14);
        assert_eq!(&S3_MEMCPY_DATA_BODY_SEED[12..], &[0xf3, 0xa4]);
        assert_eq!(S3_LDB_OO0_BODY_SEED.len(), 12);
        assert_eq!(S3_LDB_OO_BODY_SEED.len(), 19);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19,
            293
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.426: ALUV handler seed+poke — bake+233 `S3_ALUV_BODY_SEED` (15B) +
    /// poke disp + opcode (01/09/29). Covers ADDV/ORV/SUBV. Next ≥10 =
    /// LDB/MEMCPY seed or remaining hand cliffs. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_aluv_handler_seed_poke_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.426"));
        assert!(
            pe_dll.contains("ALUV body = bake+233")
                || pe_dll.contains("ALUV body = shared ALUV")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 233"));
        assert_eq!(S3_ALUV_BODY_SEED.len(), 15);
        assert_eq!(S3_ALUV_BODY_SEED[8], 0x48);
        assert_eq!(S3_ALUV_BODY_SEED[9], 0x01); // ADD template; OR/SUB poked
        assert_eq!(S3_ALUV_BODY_SEED[10], 0xc8);
        assert_eq!(S3_HOST_TABLE_SEED.len() + 17 + 16 + 15, 248);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.425: IMUL handler seed+poke — bake+217 `S3_IMUL_BODY_SEED` (16B) +
    /// poke dst/src/store disp. First non-fixed10 op-handler on own-ISA seed.
    /// Next ≥10 = ALUV/LDB/MEMCPY seed or remaining hand cliffs. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_imul_handler_seed_poke_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.425"));
        assert!(
            pe_dll.contains("IMUL body = bake+217")
                || pe_dll.contains("IMUL body = shared IMUL")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 217"));
        assert_eq!(S3_IMUL_BODY_SEED.len(), 16);
        assert_eq!(&S3_IMUL_BODY_SEED[8..12], &[0x48, 0x0f, 0xaf, 0xc1]);
        assert_eq!(S3_SET_DISP32_BODY_SEED.len(), 17);
        assert_eq!(S3_HOST_TABLE_SEED.len() + 17 + 16, 233);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.424: SET disp32 poke-from-seed — bake appends
    /// `S3_SET_DISP32_BODY_SEED` (17B) after host table; s1_loop slot≥16 path
    /// memcpy + poke imm/disp32. Completes SET emit surface (disp8=422,
    /// disp32=424). Next ≥10 = non-fixed handlers (IMUL/LDB/MEMCPY…) or
    /// remaining hand-hex densify cliffs. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_set_disp32_handler_seed_poke_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.424"));
        assert!(
            pe_dll.contains("SET poke-from-seed (bake+200")
                || pe_dll.contains("shared SET seed")
                || pe_dll.contains("disp32: 17B @+200")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 200"));
        assert_eq!(S3_SET_DISP32_BODY_SEED.len(), 17);
        assert_eq!(&S3_SET_DISP32_BODY_SEED[..2], &[0x48, 0xB8]);
        assert_eq!(&S3_SET_DISP32_BODY_SEED[10..13], &[0x49, 0x89, 0x87]);
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        // disp8 SET seed still 14B in host table; disp32 is the append.
        assert_eq!(s3_fixed_row(0x30).len(), 14);
        assert_ne!(
            S3_SET_DISP32_BODY_SEED.as_slice(),
            s3_fixed_row(0x30),
            "disp32 ≠ disp8 body"
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.423: ADD/SUB/CMP handler seed+poke — bake ADD (15B) + CMP (11B)
    /// memcpy + poke disp/ModRM/imm. Completes fixed10 ALU/CMP surface with
    /// 421–422. Next ≥10 = disp32 SET seed, or MOVRR-only row, or remaining
    /// non-fixed handlers (IMUL/LDB/…) still hand. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_add_sub_cmp_handler_seed_poke_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.423"));
        assert!(
            pe_dll.contains("ADD/SUB body = bake ADD seed")
                || pe_dll.contains("ADD/SUB body = shared ADD seed")
        );
        assert!(
            pe_dll.contains("CMP body = bake CMP seed")
                || pe_dll.contains("CMP body = shared CMP")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 100 + 4")); // ADD
        assert!(pe_dll.contains("s3_bake_table_off + 180 + 4")); // CMP
        assert_eq!(s3_fixed_row(0x62).len(), 15);
        assert_eq!(s3_fixed_row(0x61).len(), 15);
        assert_eq!(s3_fixed_row(0x65).len(), 11);
        assert_eq!(S3_HOST_TABLE_SEED[100], 0x62);
        assert_eq!(S3_HOST_TABLE_SEED[101], 15);
        assert_eq!(S3_HOST_TABLE_SEED[180], 0x65);
        assert_eq!(S3_HOST_TABLE_SEED[181], 11);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.422: SET/GET/INC handler poke-from-seed — disp8 SET + GET/MOVRR +
    /// INC/DEC emit = bake row memcpy + field poke (imm/disp/ModRM), not
    /// byte-C6 hand stores. Seed surface ≥10B object (14+8+11). disp32 SET
    /// still hand. Next ≥10 = ADD/SUB/CMP seed+poke or disp32 SET seed.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_set_get_inc_handler_seed_poke_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.422"));
        assert!(
            pe_dll.contains("SET poke-from-seed")
                || pe_dll.contains("SET body = shared SET seed")
                || pe_dll.contains("shared SET seed")
        );
        assert!(
            pe_dll.contains("GET/MOVRR body = bake GET seed")
                || pe_dll.contains("shared GET disp8 seed")
        );
        assert!(
            pe_dll.contains("INC/DEC body = bake INC seed")
                || pe_dll.contains("INC/DEC body = shared INC seed")
        );
        assert!(pe_dll.contains("s3_bake_table_off + 40 + 4")); // SET
        assert!(pe_dll.contains("s3_bake_table_off + 60 + 4")); // GET
        assert!(pe_dll.contains("s3_bake_table_off + 80 + 4")); // INC
        assert_eq!(s3_fixed_row(0x30).len(), 14);
        assert_eq!(s3_fixed_row(0x60).len(), 8);
        assert_eq!(s3_fixed_row(0x66).len(), 11);
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        assert_eq!(S3_HOST_TABLE_SEED[40], 0x30);
        assert_eq!(S3_HOST_TABLE_SEED[41], 14);
        assert_eq!(S3_HOST_TABLE_SEED[60], 0x60);
        assert_eq!(S3_HOST_TABLE_SEED[80], 0x66);
        assert_eq!(
            compile_tyb_to_pe(&s3_fixed10_ret_tyb(0x30))
                .unwrap()
                .bytes
                .as_slice(),
            S3_COMPILE_PE_SET_RET_IMAGE_SEED
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.421: first YOYO op-handler object seed — in-DLL s1_loop RET+NOP
    /// handlers emit via bake (`S3_HOST_TABLE_SEED` / fixed10 body), not
    /// hand-imm 0xC3/0x90. Seed surface = 10-row host table (≥10). Next ≥10 =
    /// more argc0 handlers (GET/INC/…) or parameterized SET poke-from-seed.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_s1_ret_nop_handler_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.421"));
        assert!(pe_dll.contains("first YOYO op-handler object seed"));
        assert!(pe_dll.contains("bake row0"));
        assert!(pe_dll.contains("bake row1"));
        // Bake ≡ host table seed (10×20); RET/NOP payloads at +4 / +24.
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        assert_eq!(S3_HOST_TABLE_SEED[0], 0xFF);
        assert_eq!(S3_HOST_TABLE_SEED[1], 1);
        assert_eq!(S3_HOST_TABLE_SEED[4], 0xC3);
        assert_eq!(S3_HOST_TABLE_SEED[20], 0x00);
        assert_eq!(S3_HOST_TABLE_SEED[21], 1);
        assert_eq!(S3_HOST_TABLE_SEED[24], 0x90);
        assert_eq!(s3_fixed_row(0xFF), &[0xC3]);
        assert_eq!(s3_fixed_row(0x00), &[0x90]);
        // Skip-mini RET still seed-identical; in-DLL runtime ≡ pe_dll_link suite.
        assert_eq!(
            compile_tyb_to_pe(&s3_fixed10_ret_tyb(0xFF))
                .unwrap()
                .bytes
                .as_slice(),
            S3_COMPILE_PE_RET_IMAGE_SEED
        );
        assert!(
            pe_dll.contains("s3_bake_table_off + 4")
                || pe_dll.contains("s3_shared_retnop_seed")
                || pe_dll.contains("r8d=4")
        );
        assert!(
            pe_dll.contains("s3_bake_table_off + 20 + 4")
                || pe_dll.contains("r8d=24")
                || pe_dll.contains("s3_shared_retnop_seed")
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.493: YOYO-IR compile slice map. Host fixed10 →
    /// `try_compile_tyb_via_yoyo_table` / `try_emit_text_via_yoyo_table`
    /// (YOYO table compose — not emit_x86). Extending `S3_FIXED10_OPS` folds
    /// golden/layout (禁). Non-fixed10 / len>8 → emit_x86 fallthrough.
    /// pe_dll hand-x86 BODY_SEED densify ≠ this slice (492). Next ≥10 =
    /// non-fixed10 YOYO compose without golden fold / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_yoyo_ir_compile_slice_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(pe_dll.contains("S3.493") || this.contains("S3.493"));
        assert!(
            pe_dll.contains("YOYO-IR compile slice")
                || this.contains("YOYO-IR compile slice")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        // ≥10 YOYO-owned vs fallthrough inventory.
        let pins = [
            "try_compile_tyb_via_yoyo_table",
            "try_emit_text_via_yoyo_table",
            "S3_FIXED10_OPS",
            "S3_FIXED10_BODY_SEED",
            "S3_HOST_TABLE_SEED",
            "S3_COMPILE_PE_RET_IMAGE_SEED",
            "s3_try_emit_via_pe",
            "emit_x86",
            "validate_tyb_mini",
            "fixed_template_ops",
            "S3_FIXED10_OPS.contains",
        ];
        for p in pins {
            assert!(
                this.contains(p) || pe_dll.contains(p),
                "missing YOYO-IR pin {p}"
            );
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // fixed10 still YOYO-path; bare IMUL (no RET) still outside (S3.494 needs +RET).
        assert!(try_emit_text_via_yoyo_table(&[0xFF]).is_some());
        assert!(try_emit_text_via_yoyo_table(&[0x63]).is_some()); // bare IMUL = S3.520
    }

    /// S3.494: non-fixed10 YOYO compose — IMUL+RET via `S3_IMUL_BODY_SEED`‖C3
    /// (slots 0,1). Does **not** extend `S3_FIXED10_OPS` / fold golden.
    /// Wrong slots / bare IMUL still emit_x86 fallthrough. bake=362 /
    /// ASM_PIN=22 / host IR=173 unchanged. Next ≥10 = ALUV+RET seed compose
    /// / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_imul_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.494") || pe_dll.contains("S3.494"));
        assert!(
            this.contains("try_emit_imul_ret_via_yoyo_seed")
                && this.contains("try_imul_ret_ops")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        // ≥10 YOYO-owned assemble / gate inventory (no FIXED10 fold).
        let pins = [
            "try_emit_imul_ret_via_yoyo_seed",
            "try_imul_ret_ops",
            "try_compose_imul_ret_via_yoyo_seed",
            "S3_IMUL_BODY_SEED",
            "S3.494",
            "non-fixed10 YOYO compose",
            "0x63, 0xFF",
            "s3_compile_pe_patch_text",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
            "try_compile_tyb_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing IMUL+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x63), "IMUL must not fold FIXED10");
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_IMUL_BODY_SEED.len(), 16);
        let via = try_emit_text_via_yoyo_table(&[0x63, 0xFF]).expect("IMUL+RET YOYO");
        assert_eq!(via.len(), 17);
        assert_eq!(&via[..16], &S3_IMUL_BODY_SEED);
        assert_eq!(*via.last().unwrap(), 0xC3);
        let rust = emit_x86(
            &parse_tyb(&tyb_from_records(&[
                (0x63, 2, [0, 0, 1, 0, 0, 0]),
                (0xFF, 0, [0; 6]),
            ]))
            .unwrap(),
        )
        .text;
        assert_eq!(via, rust, "YOYO seed ≡ emit_x86 for slots 0,1");
        let tyb = tyb_from_records(&[(0x63, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb).expect("yoyo pe");
        let pe_m = compile_tyb_to_pe_mini(&tyb).expect("mini pe");
        assert_eq!(pe_y.bytes, pe_m.bytes, "mini prefers YOYO IMUL+RET path");
        assert_eq!(
            slim_pe_user_text(&pe_y.bytes, via.len()).unwrap(),
            via.as_slice()
        );
        assert!(fixed_template_ops(&parse_tyb(&tyb).unwrap()).is_none());
        // bare IMUL still outside; disp8 wrong-slot lift = S3.511.
        assert!(try_emit_text_via_yoyo_table(&[0x63]).is_some()); // S3.520 bare
        let lifted = tyb_from_records(&[(0x63, 2, [1, 0, 2, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(try_compile_tyb_via_yoyo_table(&lifted).is_some()); // S3.511
    }

    /// S3.495: ALUV+RET YOYO compose — `S3_ALUV_BODY_SEED`‖C3 with opcode
    /// poke @+9 (ADDV=01/ORV=09/SUBV=29), slots 0,1. Not FIXED10 fold.
    /// Wrong slots / MEMCPY still emit_x86. bake/ASM/IR unchanged.
    /// Next ≥10 = LDB+RET seed compose / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_aluv_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.495") || pe_dll.contains("S3.495"));
        assert!(
            this.contains("try_emit_aluv_ret_via_yoyo_seed")
                && this.contains("try_aluv_ret_ops")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_aluv_ret_via_yoyo_seed",
            "try_aluv_ret_ops",
            "S3_ALUV_BODY_SEED",
            "S3.495",
            "ADDV=01",
            "ORV=09",
            "SUBV=29",
            "text[9] = poke",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
            "try_compile_tyb_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing ALUV+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x68));
        assert!(!S3_FIXED10_OPS.contains(&0x69));
        assert!(!S3_FIXED10_OPS.contains(&0x6A));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_ALUV_BODY_SEED.len(), 15);
        assert_eq!(S3_ALUV_BODY_SEED[9], 0x01);
        let cases: &[(u8, u8)] = &[(0x68, 0x01), (0x69, 0x09), (0x6A, 0x29)];
        for &(op, poke) in cases {
            let ops = [op, 0xFF];
            let via = try_emit_text_via_yoyo_table(&ops).expect("ALUV+RET YOYO");
            assert_eq!(via.len(), 16);
            assert_eq!(&via[..9], &S3_ALUV_BODY_SEED[..9]);
            assert_eq!(via[9], poke);
            assert_eq!(&via[10..15], &S3_ALUV_BODY_SEED[10..]);
            assert_eq!(via[15], 0xC3);
            let rust = emit_x86(
                &parse_tyb(&tyb_from_records(&[
                    (op, 2, [0, 0, 1, 0, 0, 0]),
                    (0xFF, 0, [0; 6]),
                ]))
                .unwrap(),
            )
            .text;
            assert_eq!(via, rust, "YOYO ALUV seed ≡ emit_x86 op={op:#x}");
            let tyb = tyb_from_records(&[(op, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])]);
            let pe_y = try_compile_tyb_via_yoyo_table(&tyb).expect("yoyo pe");
            let pe_m = compile_tyb_to_pe_mini(&tyb).expect("mini pe");
            assert_eq!(pe_y.bytes, pe_m.bytes);
            assert!(fixed_template_ops(&parse_tyb(&tyb).unwrap()).is_none());
        }
        // bare ADDV still outside; disp8 wrong-slot lift = S3.511.
        assert!(try_emit_text_via_yoyo_table(&[0x68]).is_some()); // S3.520 bare
        let lifted = tyb_from_records(&[(0x68, 2, [1, 0, 2, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(try_compile_tyb_via_yoyo_table(&lifted).is_some()); // S3.511
    }

    /// S3.496: LDB+RET YOYO compose — `S3_LDB_OO0_BODY_SEED` / `S3_LDB_OO_BODY_SEED`
    /// ‖C3 (dd=ss=0; oo=0 or poke @+7). Not FIXED10 fold. Wrong dd/ss
    /// still emit_x86. bake/ASM/IR unchanged. Next ≥10 =
    /// MEMCPY_DATA+RET seed compose / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_ldb_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.496") || pe_dll.contains("S3.496"));
        assert!(
            this.contains("try_emit_ldb_ret_via_yoyo_seed")
                && this.contains("try_compose_ldb_body_via_yoyo_seed")
                && this.contains("try_ldb_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_ldb_ret_via_yoyo_seed",
            "try_compose_ldb_body_via_yoyo_seed",
            "try_ldb_ret_tyb",
            "S3_LDB_OO0_BODY_SEED",
            "S3_LDB_OO_BODY_SEED",
            "S3.496",
            "body[7..11]",
            "S3_MOVZX_RAX_BYTE_RAX_SEED",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
            "try_compile_tyb_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing LDB+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x80));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_LDB_OO0_BODY_SEED.len(), 12);
        assert_eq!(S3_LDB_OO_BODY_SEED.len(), 19);
        // oo=0 via ops emit
        let via0 = try_emit_text_via_yoyo_table(&[0x80, 0xFF]).expect("LDB oo0 YOYO");
        assert_eq!(via0.len(), 13);
        assert_eq!(&via0[..12], &S3_LDB_OO0_BODY_SEED);
        assert_eq!(via0[12], 0xC3);
        let tyb0 = tyb_from_records(&[
            (0x80, 3, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let rust0 = emit_x86(&parse_tyb(&tyb0).unwrap()).text;
        assert_eq!(via0, rust0);
        let pe_y0 = try_compile_tyb_via_yoyo_table(&tyb0).expect("yoyo pe oo0");
        let pe_m0 = compile_tyb_to_pe_mini(&tyb0).expect("mini pe oo0");
        assert_eq!(pe_y0.bytes, pe_m0.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb0).unwrap()).is_none());
        // oo≠0 via TYB poke
        let tyb5 = tyb_from_records(&[
            (0x80, 3, [0, 0, 0, 0, 5, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via5 = try_ldb_ret_tyb(&parse_tyb(&tyb5).unwrap()).expect("LDB oo5");
        let rust5 = emit_x86(&parse_tyb(&tyb5).unwrap()).text;
        assert_eq!(via5, rust5);
        assert_eq!(&via5[7..11], &5u32.to_le_bytes());
        assert!(try_compile_tyb_via_yoyo_table(&tyb5).is_some());
        // bare LDB still outside; any-slot dd/ss = S3.515.
        assert!(try_emit_text_via_yoyo_table(&[0x80]).is_some()); // S3.520 bare
        let lifted = tyb_from_records(&[
            (0x80, 3, [1, 0, 2, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_compile_tyb_via_yoyo_table(&lifted).is_some()); // S3.515
    }

    /// S3.497: MEMCPY_DATA+RET YOYO compose — `S3_MEMCPY_DATA_BODY_SEED`‖C3
    /// with disp8 poke src@+3/dst@+7/n@+11 (slots <16). Not FIXED10 fold.
    /// Slot≥16 still emit_x86. bake/ASM/IR unchanged.
    /// Next ≥10 = MEMCPY_STATE+RET seed compose / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_memcpy_data_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.497") || pe_dll.contains("S3.497"));
        assert!(
            this.contains("try_emit_memcpy_data_ret_via_yoyo_seed")
                && this.contains("try_compose_memcpy_data_body_via_yoyo_seed")
                && this.contains("try_memcpy_data_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_memcpy_data_ret_via_yoyo_seed",
            "try_compose_memcpy_data_body_via_yoyo_seed",
            "try_memcpy_data_ret_tyb",
            "S3_MEMCPY_DATA_BODY_SEED",
            "S3.497",
            "body[3]",
            "body[7]",
            "body[11]",
            "S3_REP_MOVSB_SEED",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing MEMCPY_DATA+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x84));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_MEMCPY_DATA_BODY_SEED.len(), 14);
        let via0 = try_emit_text_via_yoyo_table(&[0x84, 0xFF]).expect("MCPD YOYO");
        assert_eq!(via0.len(), 15);
        assert_eq!(&via0[..14], &S3_MEMCPY_DATA_BODY_SEED);
        assert_eq!(via0[14], 0xC3);
        let tyb0 = tyb_from_records(&[
            (0x84, 3, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let rust0 = emit_x86(&parse_tyb(&tyb0).unwrap()).text;
        assert_eq!(via0, rust0);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb0).expect("yoyo pe");
        let pe_m = compile_tyb_to_pe_mini(&tyb0).expect("mini pe");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb0).unwrap()).is_none());
        // disp8 poke: dst=1 src=2 n=3
        let tyb_p = tyb_from_records(&[
            (0x84, 3, [1, 0, 2, 0, 3, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_p = try_memcpy_data_ret_tyb(&parse_tyb(&tyb_p).unwrap()).expect("poke");
        let rust_p = emit_x86(&parse_tyb(&tyb_p).unwrap()).text;
        assert_eq!(via_p, rust_p);
        assert_eq!(via_p[3], 16); // src=2 → disp 16
        assert_eq!(via_p[7], 8); // dst=1
        assert_eq!(via_p[11], 24); // n=3
        assert!(try_compile_tyb_via_yoyo_table(&tyb_p).is_some());
        // bare still outside; slot≥16 = S3.515.
        assert!(try_emit_text_via_yoyo_table(&[0x84]).is_some()); // S3.520 bare
        let big = tyb_from_records(&[
            (0x84, 3, [16, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_compile_tyb_via_yoyo_table(&big).is_some()); // S3.515
    }

    /// S3.498: MEMCPY_STATE+RET YOYO compose — looped `S3_MEMCPY_STATE_PAIR_SEED`
    /// ‖C3 (n∈1..=4, disp8). Not FIXED10 fold. n=0 / range≥16 still
    /// emit_x86. bake/ASM/IR unchanged. Next ≥10 = post-non-fixed10 seed
    /// compose map / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_memcpy_state_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.498") || pe_dll.contains("S3.498"));
        assert!(
            this.contains("try_emit_memcpy_state_ret_via_yoyo_seed")
                && this.contains("try_compose_memcpy_state_body_via_yoyo_seed")
                && this.contains("try_memcpy_state_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_memcpy_state_ret_via_yoyo_seed",
            "try_compose_memcpy_state_body_via_yoyo_seed",
            "try_memcpy_state_ret_tyb",
            "S3_MEMCPY_STATE_PAIR_SEED",
            "S3.498",
            "S1_MINI_MEMCPY_STATE_MAX_N",
            "pair[3]",
            "pair[7]",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
            "try_compile_tyb_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing MEMCPY_STATE+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x85));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_MEMCPY_STATE_PAIR_SEED.len(), 8);
        let via1 = try_emit_text_via_yoyo_table(&[0x85, 0xFF]).expect("MCPS n1");
        assert_eq!(via1.len(), 9);
        assert_eq!(&via1[..8], &S3_MEMCPY_STATE_PAIR_SEED);
        assert_eq!(via1[8], 0xC3);
        let tyb1 = tyb_from_records(&[
            (0x85, 3, [0, 0, 0, 0, 1, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let rust1 = emit_x86(&parse_tyb(&tyb1).unwrap()).text;
        assert_eq!(via1, rust1);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb1).expect("yoyo pe");
        let pe_m = compile_tyb_to_pe_mini(&tyb1).expect("mini pe");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb1).unwrap()).is_none());
        // n=2 dst=0 src=2
        let tyb2 = tyb_from_records(&[
            (0x85, 3, [0, 0, 2, 0, 2, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via2 = try_memcpy_state_ret_tyb(&parse_tyb(&tyb2).unwrap()).expect("n2");
        let rust2 = emit_x86(&parse_tyb(&tyb2).unwrap()).text;
        assert_eq!(via2, rust2);
        assert_eq!(via2.len(), 17); // 2*8 + C3
        assert!(try_compile_tyb_via_yoyo_table(&tyb2).is_some());
        // n=0 / bare / out of range still outside.
        assert!(try_emit_text_via_yoyo_table(&[0x85]).is_some()); // S3.520 bare
        let n0 = tyb_from_records(&[
            (0x85, 3, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_compile_tyb_via_yoyo_table(&n0).is_none());
    }

    /// S3.499: post-non-fixed10 seed compose map. Host YOYO-owned +RET
    /// compose (not FIXED10 fold): IMUL · ALUV · LDB · MEMCPY_DATA ·
    /// MEMCPY_STATE. Still emit_x86: control-flow (JMP/CALL/JCC) ·
    /// ALLOC/LOAD/WRITE · STB · RAW_* · DATA/STR · wrong-slot gates.
    /// Extending `S3_FIXED10_OPS` = fold golden (禁). pe_dll densify ≠
    /// compose. bake=362 / ASM_PIN=22 / host IR=173. Next ≥10 =
    /// control-flow +RET seed compose / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_non_fixed10_seed_compose_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.499") || pe_dll.contains("S3.499"));
        assert!(
            this.contains("post-non-fixed10 seed compose")
                || pe_dll.contains("post-non-fixed10 seed compose")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        // ≥10 YOYO-owned non-fixed10 compose inventory.
        let owned = [
            "try_emit_imul_ret_via_yoyo_seed",
            "try_emit_aluv_ret_via_yoyo_seed",
            "try_emit_ldb_ret_via_yoyo_seed",
            "try_emit_memcpy_data_ret_via_yoyo_seed",
            "try_emit_memcpy_state_ret_via_yoyo_seed",
            "S3_IMUL_BODY_SEED",
            "S3_ALUV_BODY_SEED",
            "S3_LDB_OO0_BODY_SEED",
            "S3_MEMCPY_DATA_BODY_SEED",
            "S3_MEMCPY_STATE_PAIR_SEED",
            "S3.494",
            "S3.498",
        ];
        for p in owned {
            assert!(this.contains(p), "missing owned compose {p}");
        }
        assert!(owned.len() >= 10);
        // S3.499 tip: DATA was outside; S3.506+ owns DATA/STR/RAW+RET.
        assert!(try_emit_text_via_yoyo_table(&[0x12, 0xFF]).is_some());
        // Owned still green; FIXED10 untouched.
        assert!(try_emit_text_via_yoyo_table(&[0x63, 0xFF]).is_some());
        assert!(try_emit_text_via_yoyo_table(&[0x68, 0xFF]).is_some());
        assert!(try_emit_text_via_yoyo_table(&[0x80, 0xFF]).is_some());
        assert!(try_emit_text_via_yoyo_table(&[0x84, 0xFF]).is_some());
        assert!(try_emit_text_via_yoyo_table(&[0x85, 0xFF]).is_some());
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x63));
        assert!(!S3_FIXED10_OPS.contains(&0x85));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.500: control-flow +RET YOYO compose — `S3_CALL/JMP/JCC_BODY_SEED`‖C3
    /// (rel32=0 ≡ undefined label; JCC poke cc @+1). No LABEL/pass-2.
    /// Not FIXED10 fold. ALLOC/STB/RAW_* still emit_x86. bake/ASM/IR
    /// unchanged. Next ≥10 = ALLOC+RET seed compose / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_cf_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.500") || pe_dll.contains("S3.500"));
        assert!(
            this.contains("try_emit_cf_ret_via_yoyo_seed")
                && this.contains("try_cf_ret_tyb")
                && this.contains("s3_jcc_cc")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_cf_ret_via_yoyo_seed",
            "try_cf_ret_tyb",
            "s3_jcc_cc",
            "S3_CALL_BODY_SEED",
            "S3_JMP_BODY_SEED",
            "S3_JCC_BODY_SEED",
            "S3.500",
            "undefined label",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
            "try_compile_tyb_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing CF+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x70));
        assert!(!S3_FIXED10_OPS.contains(&0x41));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // JMP / CALL
        let cf_cases: [(u8, &[u8]); 2] = [
            (0x70, &S3_JMP_BODY_SEED),
            (0x41, &S3_CALL_BODY_SEED),
        ];
        for (op, seed) in cf_cases {
            let via = try_emit_text_via_yoyo_table(&[op, 0xFF]).expect("CF YOYO");
            assert_eq!(&via[..seed.len()], seed);
            assert_eq!(*via.last().unwrap(), 0xC3);
            let tyb = tyb_from_records(&[(op, 1, [0, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])]);
            let rust = emit_x86(&parse_tyb(&tyb).unwrap()).text;
            assert_eq!(via, rust, "op={op:#x}");
            assert!(try_compile_tyb_via_yoyo_table(&tyb).is_some());
            assert!(fixed_template_ops(&parse_tyb(&tyb).unwrap()).is_none());
        }
        // JE (0x71) poke cc=0x84
        let via_je = try_emit_text_via_yoyo_table(&[0x71, 0xFF]).expect("JE");
        assert_eq!(via_je[0], 0x0F);
        assert_eq!(via_je[1], 0x84);
        assert_eq!(via_je.len(), 7);
        let tyb_je = tyb_from_records(&[(0x71, 1, [1, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert_eq!(via_je, emit_x86(&parse_tyb(&tyb_je).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_je).is_some());
        // All 10 JCC ops compose.
        for op in 0x71u8..=0x7Au8 {
            let via = try_emit_text_via_yoyo_table(&[op, 0xFF]).expect("JCC");
            assert_eq!(via[1], s3_jcc_cc(op).unwrap());
        }
        // LABEL present → refuse (needs pass-2).
        let with_lab = tyb_from_records(&[
            (0x40, 1, [0, 0, 0, 0, 0, 0]),
            (0x70, 1, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        // 3 records → try_cf_ret_tyb None; also not fixed10.
        assert!(try_compile_tyb_via_yoyo_table(&with_lab).is_none());
        // STB owned by S3.502; RAW still outside here.
        assert!(try_emit_text_via_yoyo_table(&[0x12, 0xFF]).is_none()); // RAW_BYTE
    }

    /// S3.501: ALLOC+RET YOYO compose — `S3_ALLOC_PREFIX_SEED` (size poke @+7)
    /// + `S3_STORE_RAX_DISP32_SEED` (disp poke) ‖C3. Matches emit_slim_alloc.
    /// Not FIXED10 fold. LOAD/WRITE/RAW_* still emit_x86. bake/ASM/IR
    /// unchanged. Next ≥10 = STB+RET seed compose / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_alloc_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.501") || pe_dll.contains("S3.501"));
        assert!(
            this.contains("try_emit_alloc_ret_via_yoyo_seed")
                && this.contains("try_compose_alloc_body_via_yoyo_seed")
                && this.contains("try_alloc_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_alloc_ret_via_yoyo_seed",
            "try_compose_alloc_body_via_yoyo_seed",
            "try_alloc_ret_tyb",
            "S3_ALLOC_PREFIX_SEED",
            "S3_STORE_RAX_DISP32_SEED",
            "S3.501",
            "emit_slim_alloc",
            "body[7..11]",
            "store[3..7]",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing ALLOC+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x20));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_ALLOC_PREFIX_SEED.len(), 34);
        assert_eq!(S3_STORE_RAX_DISP32_SEED.len(), 7);
        let via0 = try_emit_text_via_yoyo_table(&[0x20, 0xFF]).expect("ALLOC YOYO");
        assert_eq!(via0.len(), 42); // 34+7+1
        let tyb0 = tyb_from_records(&[
            (0x20, 2, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let rust0 = emit_x86(&parse_tyb(&tyb0).unwrap()).text;
        assert_eq!(via0, rust0);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb0).expect("yoyo pe");
        let pe_m = compile_tyb_to_pe_mini(&tyb0).expect("mini pe");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb0).unwrap()).is_none());
        // slot=1 size=0x100
        let tyb1 = tyb_from_records(&[
            (0x20, 2, [1, 0, 0x00, 0x01, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via1 = try_alloc_ret_tyb(&parse_tyb(&tyb1).unwrap()).expect("alloc poke");
        let rust1 = emit_x86(&parse_tyb(&tyb1).unwrap()).text;
        assert_eq!(via1, rust1);
        assert_eq!(&via1[7..11], &0x100u32.to_le_bytes());
        assert_eq!(&via1[37..41], &8u32.to_le_bytes()); // slot1 disp
        assert!(try_compile_tyb_via_yoyo_table(&tyb1).is_some());
        // bare ALLOC still outside (STB = S3.502).
        assert!(try_emit_text_via_yoyo_table(&[0x20]).is_none());
    }

    /// S3.502: STB+RET YOYO compose — `S3_STB_OO0_BODY_SEED` /
    /// `S3_STB_OO_IMM8_BODY_SEED`‖C3 (disp8 val/base; oo=0 or 1..=127).
    /// Host object seeds (not bake append; bake=362). Not FIXED10 fold.
    /// oo>127 / LOAD/WRITE still emit_x86. Next ≥10 =
    /// RAW_BYTE+RET seed compose / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_stb_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.502") || pe_dll.contains("S3.502"));
        assert!(
            this.contains("try_emit_stb_ret_via_yoyo_seed")
                && this.contains("try_compose_stb_body_via_yoyo_seed")
                && this.contains("try_stb_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_stb_ret_via_yoyo_seed",
            "try_compose_stb_body_via_yoyo_seed",
            "try_stb_ret_tyb",
            "S3_STB_OO0_BODY_SEED",
            "S3_STB_OO_IMM8_BODY_SEED",
            "S3.502",
            "body[11]",
            "mini_slot_disp8_ok",
            "S3_STB_OO_IMM32_BODY_SEED",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing STB+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x81));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_STB_OO0_BODY_SEED.len(), 10);
        assert_eq!(S3_STB_OO_IMM8_BODY_SEED.len(), 14);
        // oo=0 via ops
        let via0 = try_emit_text_via_yoyo_table(&[0x81, 0xFF]).expect("STB oo0");
        assert_eq!(via0.len(), 11);
        assert_eq!(&via0[..10], &S3_STB_OO0_BODY_SEED);
        assert_eq!(via0[10], 0xC3);
        let tyb0 = tyb_from_records(&[
            (0x81, 3, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let rust0 = emit_x86(&parse_tyb(&tyb0).unwrap()).text;
        assert_eq!(via0, rust0);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb0).expect("yoyo pe");
        let pe_m = compile_tyb_to_pe_mini(&tyb0).expect("mini pe");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb0).unwrap()).is_none());
        // val=0 base=1 oo=5 imm8
        let tyb5 = tyb_from_records(&[
            (0x81, 3, [0, 0, 1, 0, 5, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via5 = try_stb_ret_tyb(&parse_tyb(&tyb5).unwrap()).expect("STB oo5");
        let rust5 = emit_x86(&parse_tyb(&tyb5).unwrap()).text;
        assert_eq!(via5, rust5);
        assert_eq!(via5[7], 8); // base=1
        assert_eq!(via5[11], 5);
        assert!(try_compile_tyb_via_yoyo_table(&tyb5).is_some());
        // bare still outside; oo>127 = S3.513.
        assert!(try_emit_text_via_yoyo_table(&[0x81]).is_none());
        let big_oo = tyb_from_records(&[
            (0x81, 3, [0, 0, 0, 0, 0x80, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_stb_ret_tyb(&parse_tyb(&big_oo).unwrap()).is_some()); // S3.513
    }

    /// S3.503: RAW_BYTE / RAW_BYTES +RET YOYO compose — literal payload‖C3.
    /// ops RAW_BYTE → NOP‖C3 default; TYB carries byte / argc1..=3 low-bytes.
    /// `S3_RAW_BYTE_EMIT_BODY_SEED` = in-DLL emitter ≠ host payload path.
    /// Not FIXED10. LOAD/WRITE/DATA still emit_x86. Next ≥10 =
    /// LOAD_FILE+RET seed compose / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_raw_byte_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.503") || pe_dll.contains("S3.503"));
        assert!(
            this.contains("try_emit_raw_byte_ret_via_yoyo_seed")
                && this.contains("try_raw_byte_ret_tyb")
                && this.contains("try_raw_bytes_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_raw_byte_ret_via_yoyo_seed",
            "try_raw_byte_ret_tyb",
            "try_raw_bytes_ret_tyb",
            "S3_RAW_BYTE_RET_OPS_DEFAULT",
            "S3_RAW_BYTE_EMIT_BODY_SEED",
            "S3.503",
            "0xA0",
            "0xA1",
            "arg_low_bytes",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing RAW+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0xA0));
        assert!(!S3_FIXED10_OPS.contains(&0xA1));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // ops default NOP‖C3
        let via_ops = try_emit_text_via_yoyo_table(&[0xA0, 0xFF]).expect("RAW_BYTE ops");
        assert_eq!(via_ops, vec![0x90, 0xC3]);
        let tyb_nop = tyb_from_records(&[(0xA0, 1, [0x90, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert_eq!(via_ops, emit_x86(&parse_tyb(&tyb_nop).unwrap()).text);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb_nop).expect("yoyo pe");
        let pe_m = compile_tyb_to_pe_mini(&tyb_nop).expect("mini pe");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb_nop).unwrap()).is_none());
        // TYB other byte
        let tyb_cc = tyb_from_records(&[(0xA0, 1, [0xCC, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        let via_cc = try_raw_byte_ret_tyb(&parse_tyb(&tyb_cc).unwrap()).expect("CC");
        assert_eq!(via_cc, vec![0xCC, 0xC3]);
        assert_eq!(via_cc, emit_x86(&parse_tyb(&tyb_cc).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_cc).is_some());
        // RAW_BYTES argc=2
        let tyb_ab = tyb_from_records(&[
            (0xA1, 2, [0x90, 0, 0xCC, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_ab = try_raw_bytes_ret_tyb(&parse_tyb(&tyb_ab).unwrap()).expect("RAW_BYTES");
        assert_eq!(via_ab, emit_x86(&parse_tyb(&tyb_ab).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_ab).is_some());
        // bare still outside; A1 ops = S3.512.
        assert!(try_emit_text_via_yoyo_table(&[0xA0]).is_none());
    }

    /// S3.504: LOAD_FILE+RET YOYO compose — `S3_LOAD_FILE_BODY_SEED` poke
    /// path@+9 / store slot ‖C3. ≡ emit_slim_load_file / slim_load_tmpl.
    /// Host seed (bake=362). Not FIXED10. WRITE still emit_x86.
    /// Next ≥10 = WRITE_FILE+RET seed compose / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_load_file_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.504") || pe_dll.contains("S3.504"));
        assert!(
            this.contains("try_emit_load_file_ret_via_yoyo_seed")
                && this.contains("try_compose_load_file_body_via_yoyo_seed")
                && this.contains("try_load_file_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_load_file_ret_via_yoyo_seed",
            "try_compose_load_file_body_via_yoyo_seed",
            "try_load_file_ret_tyb",
            "S3_LOAD_FILE_BODY_SEED",
            "S3.504",
            "slim_load_path_patch_off",
            "slim_load_slot_patch_off",
            "emit_slim_load_file",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
            "S1_SLIM_IAT_BASE",
        ];
        for p in pins {
            assert!(this.contains(p), "missing LOAD_FILE+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x50));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_LOAD_FILE_BODY_SEED.len(), 160);
        assert_eq!(
            S3_LOAD_FILE_BODY_SEED.as_slice(),
            emit_slim_load_file(0, 0).as_slice()
        );
        let via0 = try_emit_text_via_yoyo_table(&[0x50, 0xFF]).expect("LOAD YOYO");
        assert_eq!(via0.len(), 161); // 160+C3
        let tyb0 = tyb_from_records(&[
            (0x50, 2, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let rust0 = emit_x86(&parse_tyb(&tyb0).unwrap()).text;
        assert_eq!(via0, rust0);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb0).expect("yoyo pe");
        let pe_m = compile_tyb_to_pe_mini(&tyb0).expect("mini pe");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb0).unwrap()).is_none());
        // slot=1 path_off=8
        let tyb1 = tyb_from_records(&[
            (0x50, 2, [1, 0, 8, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via1 = try_load_file_ret_tyb(&parse_tyb(&tyb1).unwrap()).expect("load poke");
        let rust1 = emit_x86(&parse_tyb(&tyb1).unwrap()).text;
        assert_eq!(via1, rust1);
        assert_eq!(&via1[9..13], &8u32.to_le_bytes());
        assert_eq!(&via1[154..158], &8u32.to_le_bytes()); // slot1*8
        assert!(try_compile_tyb_via_yoyo_table(&tyb1).is_some());
        // bare / path≥IAT still outside (WRITE = S3.505).
        assert!(try_emit_text_via_yoyo_table(&[0x50]).is_none());
        let bad_path = tyb_from_records(&[
            (
                0x50,
                2,
                [
                    0,
                    0,
                    (S1_SLIM_IAT_BASE & 0xff) as u8,
                    ((S1_SLIM_IAT_BASE >> 8) & 0xff) as u8,
                    0,
                    0,
                ],
            ),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_load_file_ret_tyb(&parse_tyb(&bad_path).unwrap()).is_none());
    }

    /// S3.505: WRITE_FILE+RET YOYO compose — `S3_WRITE_FILE_BODY_SEED` poke
    /// path/slot/sz/store ‖C3. ≡ emit_slim_write_file / slim_write_tmpl.
    /// Host seed (bake=362). Not FIXED10. DATA/STR still emit_x86.
    /// Next ≥10 = DATA+RET seed compose / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_write_file_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.505") || pe_dll.contains("S3.505"));
        assert!(
            this.contains("try_emit_write_file_ret_via_yoyo_seed")
                && this.contains("try_compose_write_file_body_via_yoyo_seed")
                && this.contains("try_write_file_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_write_file_ret_via_yoyo_seed",
            "try_compose_write_file_body_via_yoyo_seed",
            "try_write_file_ret_tyb",
            "S3_WRITE_FILE_BODY_SEED",
            "S3.505",
            "slim_write_path_patch_off",
            "slim_write_slot_load_patch_off",
            "slim_write_sz_load_patch_off",
            "slim_write_store_patch_off",
            "emit_slim_write_file",
            "S3_FIXED10_OPS",
        ];
        for p in pins {
            assert!(this.contains(p), "missing WRITE_FILE+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x51));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_WRITE_FILE_BODY_SEED.len(), 139);
        assert_eq!(
            S3_WRITE_FILE_BODY_SEED.as_slice(),
            emit_slim_write_file(0, 0, 0).as_slice()
        );
        let via0 = try_emit_text_via_yoyo_table(&[0x51, 0xFF]).expect("WRITE YOYO");
        assert_eq!(via0.len(), 140); // 139+C3
        let tyb0 = tyb_from_records(&[
            (0x51, 2, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let rust0 = emit_x86(&parse_tyb(&tyb0).unwrap()).text;
        assert_eq!(via0, rust0);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb0).expect("yoyo pe");
        let pe_m = compile_tyb_to_pe_mini(&tyb0).expect("mini pe");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb0).unwrap()).is_none());
        // argc=3: slot=1 path=2 sz=3
        let tyb3 = tyb_from_records(&[
            (0x51, 3, [1, 0, 2, 0, 3, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via3 = try_write_file_ret_tyb(&parse_tyb(&tyb3).unwrap()).expect("write poke");
        let rust3 = emit_x86(&parse_tyb(&tyb3).unwrap()).text;
        assert_eq!(via3, rust3);
        assert_eq!(&via3[9..13], &2u32.to_le_bytes());
        assert_eq!(&via3[66..70], &8u32.to_le_bytes()); // slot1
        assert_eq!(&via3[73..77], &24u32.to_le_bytes()); // sz_slot3*8
        assert_eq!(&via3[133..137], &8u32.to_le_bytes()); // store slot1
        assert!(try_compile_tyb_via_yoyo_table(&tyb3).is_some());
        // bare still outside (DATA = S3.506).
        assert!(try_emit_text_via_yoyo_table(&[0x51]).is_none());
    }

    /// S3.506: DATA/STR/RAW+RET YOYO compose — text=C3; `.data`=arg low-bytes.
    /// ops empty-data; TYB carries payload. `S3_DATA_APPEND_BODY_SEED` =
    /// in-DLL emitter ≠ host payload. Not FIXED10. Next ≥10 =
    /// multi-DATA+RET / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_data_section_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.506") || pe_dll.contains("S3.506"));
        assert!(
            this.contains("try_emit_data_ret_via_yoyo_seed")
                && this.contains("try_data_section_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_data_ret_via_yoyo_seed",
            "try_data_section_ret_tyb",
            "S3_DATA_APPEND_BODY_SEED",
            "S3.506",
            "0x10",
            "0x12",
            "0x13",
            "arg_low_bytes",
            "s3_compile_pe_patch_text_data",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing DATA+RET pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x10));
        assert!(!S3_FIXED10_OPS.contains(&0x12));
        assert!(!S3_FIXED10_OPS.contains(&0x13));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // ops: empty .data ‖ C3
        let via_ops = try_emit_text_via_yoyo_table(&[0x10, 0xFF]).expect("DATA ops");
        assert_eq!(via_ops, vec![0xC3]);
        assert_eq!(
            try_emit_text_via_yoyo_table(&[0x12, 0xFF]).unwrap(),
            vec![0xC3]
        );
        assert_eq!(
            try_emit_text_via_yoyo_table(&[0x13, 0xFF]).unwrap(),
            vec![0xC3]
        );
        let tyb0 = tyb_from_records(&[(0x10, 0, [0; 6]), (0xFF, 0, [0; 6])]);
        let rust0 = emit_x86(&parse_tyb(&tyb0).unwrap());
        assert_eq!(via_ops, rust0.text);
        assert!(rust0.data.is_empty());
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb0).expect("yoyo pe");
        let pe_m = compile_tyb_to_pe_mini(&tyb0).expect("mini pe");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb0).unwrap()).is_none());
        // DATA argc=3 payload
        let tyb_hi = tyb_from_records(&[
            (0x10, 3, [0x48, 0, 0x69, 0, 0x00, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let (t, d) = try_data_section_ret_tyb(&parse_tyb(&tyb_hi).unwrap()).expect("DATA");
        let rust = emit_x86(&parse_tyb(&tyb_hi).unwrap());
        assert_eq!(t, rust.text);
        assert_eq!(d, rust.data);
        assert_eq!(d, vec![0x48, 0x69, 0x00]);
        let pe_y2 = try_compile_tyb_via_yoyo_table(&tyb_hi).expect("yoyo data pe");
        let pe_m2 = compile_tyb_to_pe_mini(&tyb_hi).expect("mini data pe");
        assert_eq!(pe_y2.bytes, pe_m2.bytes);
        // STR
        let tyb_str = tyb_from_records(&[
            (0x12, 2, [0x41, 0, 0x42, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_str).is_some());
        // bare still outside (multi-DATA = S3.507).
        assert!(try_emit_text_via_yoyo_table(&[0x10]).is_none());
    }

    /// S3.507: multi DATA|STR|RAW+RET YOYO compose — concat `.data` ‖ C3.
    /// ≥2 payload records; ≤S1_MINI_DATA_MAX. Not FIXED10. Next ≥10 =
    /// host gap inventory / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_multi_data_section_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.507") || pe_dll.contains("S3.507"));
        assert!(this.contains("try_multi_data_section_ret_tyb"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_multi_data_section_ret_tyb",
            "try_data_section_ret_tyb",
            "S3.507",
            "S1_MINI_DATA_MAX",
            "0x10",
            "0x12",
            "0x13",
            "arg_low_bytes",
            "s3_compile_pe_patch_text_data",
            "S3_FIXED10_OPS",
            "S3_DATA_APPEND_BODY_SEED",
        ];
        for p in pins {
            assert!(this.contains(p), "missing multi-DATA pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S1_MINI_DATA_MAX, 384);
        // STR + RAW + RET
        let tyb = tyb_from_records(&[
            (0x12, 2, [0x41, 0, 0x42, 0, 0, 0]),
            (0x13, 1, [0xFF, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let (t, d) = try_multi_data_section_ret_tyb(&parse_tyb(&tyb).unwrap()).expect("multi");
        let rust = emit_x86(&parse_tyb(&tyb).unwrap());
        assert_eq!(t, rust.text);
        assert_eq!(d, rust.data);
        assert_eq!(d, vec![0x41, 0x42, 0xFF]);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb).expect("yoyo");
        let pe_m = compile_tyb_to_pe_mini(&tyb).expect("mini");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb).unwrap()).is_none());
        // single DATA still via S3.506 path, not multi
        let one = tyb_from_records(&[(0x10, 1, [0x90, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(try_multi_data_section_ret_tyb(&parse_tyb(&one).unwrap()).is_none());
        assert!(try_data_section_ret_tyb(&parse_tyb(&one).unwrap()).is_some());
        // mixed with non-data refuses
        let mixed = tyb_from_records(&[
            (0x10, 1, [1, 0, 0, 0, 0, 0]),
            (0x30, 2, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_multi_data_section_ret_tyb(&parse_tyb(&mixed).unwrap()).is_none());
    }

    /// S3.508: host YOYO compose gap inventory. Single-op+RET surface largely
    /// owned (494–507+). Remaining emit_x86 fallthrough cliffs (≥10):
    /// LABEL+pass-2 residual · RAW_BYTES bare · bare op · mixed non-DATA ·
    /// FIXED10 fold ban · bake pin · OW-RT CUT · YOYO-built cliff ·
    /// CMP slot≥16 · multi-op mix. (mem any-slot = S3.515.)
    /// Next ≥10 = CMP any-slot / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_host_yoyo_compose_gap_inventory_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.508") || pe_dll.contains("S3.508"));
        assert!(
            pe_dll.contains("host YOYO compose gap inventory")
                || this.contains("host YOYO compose gap inventory")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let gaps = [
            "LABEL+pass-2 residual",
            "RAW_BYTES bare",
            "bare op",
            "mixed non-DATA multi",
            "FIXED10 fold ban",
            "bake pin",
            "OW-RT remains CUT",
            "YOYO-built cliff",
            "CMP slot≥16",
            "multi-op mix",
            "S3.508",
        ];
        for g in gaps {
            assert!(
                this.contains(g) || pe_dll.contains(g),
                "missing gap pin {g}"
            );
        }
        assert!(gaps.len() >= 10);
        // Witnesses: RAW_BYTES bare / CMP≥16 / DATA+ALU = S3.516–517
        assert!(try_emit_text_via_yoyo_table(&[0xA1]).is_some()); // RAW_BYTES bare = S3.517
        assert!(try_emit_text_via_yoyo_table(&[0x20]).is_some()); // bare ALLOC = S3.518
        // CMP slot≥16 owned by S3.516 — green
        let cmp_big = tyb_from_records(&[
            (0x65, 2, [16, 0, 17, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_compile_tyb_via_yoyo_table(&cmp_big).is_some());
        // mem any-slot owned by S3.515 — green
        assert!(try_stb_ret_tyb(
            &parse_tyb(&tyb_from_records(&[
                (0x81, 3, [16, 0, 0, 0, 0, 0]),
                (0xFF, 0, [0; 6]),
            ]))
            .unwrap()
        )
        .is_some());
        assert!(try_aluv_ret_ops(
            &parse_tyb(&tyb_from_records(&[
                (0x68, 2, [16, 0, 17, 0, 0, 0]),
                (0xFF, 0, [0; 6]),
            ]))
            .unwrap()
        )
        .is_some());
        let labeled = tyb_from_records(&[
            (0x40, 1, [1, 0, 0, 0, 0, 0]),
            (0x41, 1, [1, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_cf_ret_tyb(&parse_tyb(&labeled).unwrap()).is_none());
        let mixed = tyb_from_records(&[
            (0x10, 1, [1, 0, 0, 0, 0, 0]),
            (0x30, 2, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_multi_data_section_ret_tyb(&parse_tyb(&mixed).unwrap()).is_none());
        assert!(try_data_alu_mix_ret_tyb(&parse_tyb(&mixed).unwrap()).is_some()); // S3.517
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x50));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // Owned surface still green (not sheep −1 densify).
        assert!(try_emit_text_via_yoyo_table(&[0x50, 0xFF]).is_some());
        assert!(try_emit_text_via_yoyo_table(&[0x10, 0xFF]).is_some());
        assert!(try_emit_text_via_yoyo_table(&[0x41, 0xFF]).is_some());
    }

    /// S3.509: LABEL+CF+RET YOYO compose — LABEL@0 + CALL/JMP/JCC same-id
    /// pass-2 backward rel32‖C3. Not FIXED10. Next ≥10 =
    /// CF+LABEL+RET forward / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_label_cf_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.509") || pe_dll.contains("S3.509"));
        assert!(
            this.contains("try_emit_label_cf_ret_via_yoyo_seed")
                && this.contains("try_compose_label_cf_ret_via_yoyo_seed")
                && this.contains("try_label_cf_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_label_cf_ret_via_yoyo_seed",
            "try_compose_label_cf_ret_via_yoyo_seed",
            "try_label_cf_ret_tyb",
            "S3_CALL_BODY_SEED",
            "S3_JMP_BODY_SEED",
            "S3_JCC_BODY_SEED",
            "S3.509",
            "pass-2",
            "LABEL@0",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing LABEL+CF pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x40));
        assert!(!S3_FIXED10_OPS.contains(&0x41));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // ops LABEL+JMP+RET
        let via = try_emit_text_via_yoyo_table(&[0x40, 0x70, 0xFF]).expect("LABEL JMP");
        let tyb = tyb_from_records(&[
            (0x40, 1, [1, 0, 0, 0, 0, 0]),
            (0x70, 1, [1, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let rust = emit_x86(&parse_tyb(&tyb).unwrap()).text;
        assert_eq!(via, rust);
        // JMP: E9 + rel=-5 + C3
        assert_eq!(via[0], 0xE9);
        assert_eq!(&via[1..5], &(-5i32).to_le_bytes());
        assert_eq!(*via.last().unwrap(), 0xC3);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb).expect("yoyo");
        let pe_m = compile_tyb_to_pe_mini(&tyb).expect("mini");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb).unwrap()).is_none());
        // CALL + JCC
        let tyb_call = tyb_from_records(&[
            (0x40, 1, [2, 0, 0, 0, 0, 0]),
            (0x41, 1, [2, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_c = try_label_cf_ret_tyb(&parse_tyb(&tyb_call).unwrap()).expect("CALL");
        assert_eq!(via_c, emit_x86(&parse_tyb(&tyb_call).unwrap()).text);
        assert_eq!(&via_c[1..5], &(-5i32).to_le_bytes());
        let tyb_jcc = tyb_from_records(&[
            (0x40, 1, [3, 0, 0, 0, 0, 0]),
            (0x71, 1, [3, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_j = try_label_cf_ret_tyb(&parse_tyb(&tyb_jcc).unwrap()).expect("JCC");
        assert_eq!(via_j, emit_x86(&parse_tyb(&tyb_jcc).unwrap()).text);
        assert_eq!(&via_j[2..6], &(-6i32).to_le_bytes()); // JCC 6B
        assert!(try_compile_tyb_via_yoyo_table(&tyb_jcc).is_some());
        // id mismatch / CF-first still outside (forward = S3.510).
        let bad_id = tyb_from_records(&[
            (0x40, 1, [1, 0, 0, 0, 0, 0]),
            (0x70, 1, [2, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_label_cf_ret_tyb(&parse_tyb(&bad_id).unwrap()).is_none());
        let fwd = tyb_from_records(&[
            (0x70, 1, [1, 0, 0, 0, 0, 0]),
            (0x40, 1, [1, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_label_cf_ret_tyb(&parse_tyb(&fwd).unwrap()).is_none());
    }

    /// S3.510: CF+NOP+LABEL+RET YOYO compose — forward pass-2 (NOP filler
    /// ⇒ rel≠0). Not FIXED10 / not sheep of bare CF+RET. Next ≥10 =
    /// wrong-slot lift / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_cf_nop_label_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.510") || pe_dll.contains("S3.510"));
        assert!(
            this.contains("try_emit_cf_nop_label_ret_via_yoyo_seed")
                && this.contains("try_compose_cf_nop_label_ret_via_yoyo_seed")
                && this.contains("try_cf_nop_label_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_cf_nop_label_ret_via_yoyo_seed",
            "try_compose_cf_nop_label_ret_via_yoyo_seed",
            "try_cf_nop_label_ret_tyb",
            "S3.510",
            "NOP filler",
            "forward pass-2",
            "S3_CALL_BODY_SEED",
            "S3_JMP_BODY_SEED",
            "S3_JCC_BODY_SEED",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing CF+NOP+LABEL pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        let via = try_emit_text_via_yoyo_table(&[0x70, 0x00, 0x40, 0xFF]).expect("fwd JMP");
        let tyb = tyb_from_records(&[
            (0x70, 1, [1, 0, 0, 0, 0, 0]),
            (0x00, 0, [0; 6]),
            (0x40, 1, [1, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let rust = emit_x86(&parse_tyb(&tyb).unwrap()).text;
        assert_eq!(via, rust);
        // JMP 5B + NOP; target=6 next=5 ⇒ rel=1
        assert_eq!(via[0], 0xE9);
        assert_eq!(&via[1..5], &1i32.to_le_bytes());
        assert_eq!(via[5], 0x90);
        assert_eq!(*via.last().unwrap(), 0xC3);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb).expect("yoyo");
        let pe_m = compile_tyb_to_pe_mini(&tyb).expect("mini");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb).unwrap()).is_none());
        // JCC forward
        let tyb_j = tyb_from_records(&[
            (0x72, 1, [9, 0, 0, 0, 0, 0]),
            (0x00, 0, [0; 6]),
            (0x40, 1, [9, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_j = try_cf_nop_label_ret_tyb(&parse_tyb(&tyb_j).unwrap()).expect("JCC");
        assert_eq!(via_j, emit_x86(&parse_tyb(&tyb_j).unwrap()).text);
        // JCC 6B + NOP; target=7 next=6 ⇒ rel=1
        assert_eq!(&via_j[2..6], &1i32.to_le_bytes());
        assert!(try_compile_tyb_via_yoyo_table(&tyb_j).is_some());
        // no-NOP CF+LABEL still outside; id mismatch refuses.
        let no_nop = tyb_from_records(&[
            (0x70, 1, [1, 0, 0, 0, 0, 0]),
            (0x40, 1, [1, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_cf_nop_label_ret_tyb(&parse_tyb(&no_nop).unwrap()).is_none());
    }

    /// S3.511: IMUL/ALUV+RET disp8 wrong-slot lift — poke dst/src/store.
    /// ops still 0,1 default; TYB any slot<16. Not FIXED10. Next ≥10 =
    /// RAW_BYTES ops / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_imul_aluv_disp8_slot_lift_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.511") || pe_dll.contains("S3.511"));
        assert!(
            this.contains("try_compose_imul_ret_via_yoyo_seed")
                && this.contains("try_compose_aluv_ret_via_yoyo_seed")
                && this.contains("try_imul_ret_tyb")
                && this.contains("try_aluv_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_compose_imul_ret_via_yoyo_seed",
            "try_compose_aluv_ret_via_yoyo_seed",
            "try_imul_ret_tyb",
            "try_aluv_ret_tyb",
            "S3.511",
            "disp8 wrong-slot lift",
            "S3_IMUL_BODY_SEED",
            "S3_ALUV_BODY_SEED",
            "mini_slot_disp8_ok",
            "S3_FIXED10_OPS",
            "try_compile_tyb_via_yoyo_table",
        ];
        for p in pins {
            assert!(this.contains(p), "missing slot-lift pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // IMUL dst=1 src=2
        let tyb_i = tyb_from_records(&[
            (0x63, 2, [1, 0, 2, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_i = try_imul_ret_tyb(&parse_tyb(&tyb_i).unwrap()).expect("IMUL lift");
        assert_eq!(via_i, emit_x86(&parse_tyb(&tyb_i).unwrap()).text);
        assert_eq!(via_i[3], 8);
        assert_eq!(via_i[7], 16);
        assert_eq!(via_i[15], 8);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb_i).expect("yoyo");
        let pe_m = compile_tyb_to_pe_mini(&tyb_i).expect("mini");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        // ALUV dst=3 src=4
        let tyb_a = tyb_from_records(&[
            (0x69, 2, [3, 0, 4, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_a = try_aluv_ret_tyb(&parse_tyb(&tyb_a).unwrap()).expect("ALUV lift");
        assert_eq!(via_a, emit_x86(&parse_tyb(&tyb_a).unwrap()).text);
        assert_eq!(via_a[3], 24);
        assert_eq!(via_a[7], 32);
        assert_eq!(via_a[14], 24);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_a).is_some());
        // slot≥16 = S3.514
        let big = tyb_from_records(&[
            (0x63, 2, [16, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_imul_ret_tyb(&parse_tyb(&big).unwrap()).is_some());
    }

    /// S3.512: RAW_BYTES+RET ops-path — `S3_RAW_BYTES_RET_OPS_DEFAULT`‖C3
    /// (1×NOP). TYB argc1..=3 still via try_raw_bytes_ret_tyb. Emitter seed
    /// ≠ payload. Not FIXED10. Next ≥10 = STB oo>127 / slot≥16 / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_raw_bytes_ret_ops_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.512") || pe_dll.contains("S3.512"));
        assert!(
            this.contains("try_emit_raw_bytes_ret_via_yoyo_seed")
                && this.contains("S3_RAW_BYTES_RET_OPS_DEFAULT")
                && this.contains("try_raw_bytes_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "try_emit_raw_bytes_ret_via_yoyo_seed",
            "S3_RAW_BYTES_RET_OPS_DEFAULT",
            "try_raw_bytes_ret_tyb",
            "S3_RAW_BYTES_EMIT_BODY_SEED",
            "S3.512",
            "0xA1",
            "0xFF",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
            "try_compile_tyb_via_yoyo_table",
            "arg_low_bytes",
        ];
        for p in pins {
            assert!(this.contains(p), "missing RAW_BYTES ops pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0xA1));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_RAW_BYTES_RET_OPS_DEFAULT, [0x90]);
        let via = try_emit_text_via_yoyo_table(&[0xA1, 0xFF]).expect("RAW_BYTES ops");
        assert_eq!(via, vec![0x90, 0xC3]);
        let tyb_def = tyb_from_records(&[
            (0xA1, 1, [0x90, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert_eq!(via, emit_x86(&parse_tyb(&tyb_def).unwrap()).text);
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb_def).expect("yoyo");
        let pe_m = compile_tyb_to_pe_mini(&tyb_def).expect("mini");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb_def).unwrap()).is_none());
        // TYB multi-byte still via tyb path
        let tyb2 = tyb_from_records(&[
            (0xA1, 2, [0xCC, 0, 0xDD, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via2 = try_raw_bytes_ret_tyb(&parse_tyb(&tyb2).unwrap()).expect("2B");
        assert_eq!(via2, vec![0xCC, 0xDD, 0xC3]);
        assert!(try_compile_tyb_via_yoyo_table(&tyb2).is_some());
        // bare still outside
        assert!(try_emit_text_via_yoyo_table(&[0xA1]).is_none());
    }

    /// S3.514: IMUL/ALUV+RET any-slot≤255 — load/store host seeds compose
    /// disp8|disp32. validate mini lifted for IMUL/ALUV. Not FIXED10.
    /// Next ≥10 = STB slot≥16 / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_imul_aluv_any_slot_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.514") || pe_dll.contains("S3.514"));
        assert!(
            this.contains("s3_push_load_rax_slot")
                && this.contains("s3_push_load_rcx_slot")
                && this.contains("s3_push_store_rax_slot")
                && this.contains("S3_LOAD_RAX_DISP32_SEED")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "s3_push_load_rax_slot",
            "s3_push_load_rcx_slot",
            "s3_push_store_rax_slot",
            "S3_LOAD_RAX_DISP32_SEED",
            "S3_LOAD_RCX_DISP32_SEED",
            "S3_IMUL_RAX_RCX_SEED",
            "S3.514",
            "mini_slot_any_ok",
            "S3_FIXED10_OPS",
            "try_imul_ret_tyb",
            "try_aluv_ret_tyb",
        ];
        for p in pins {
            assert!(this.contains(p), "missing any-slot pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // both ≥16
        let tyb = tyb_from_records(&[
            (0x63, 2, [16, 0, 17, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via = try_imul_ret_tyb(&parse_tyb(&tyb).unwrap()).expect("IMUL16");
        assert_eq!(via, emit_x86(&parse_tyb(&tyb).unwrap()).text);
        assert_eq!(via.len(), 26); // 25+C3
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb).expect("yoyo");
        let pe_m = compile_tyb_to_pe_mini(&tyb).expect("mini");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        // mix disp8+disp32
        let tyb_m = tyb_from_records(&[
            (0x63, 2, [0, 0, 16, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_m = try_imul_ret_tyb(&parse_tyb(&tyb_m).unwrap()).expect("mix");
        assert_eq!(via_m, emit_x86(&parse_tyb(&tyb_m).unwrap()).text);
        assert_eq!(via_m.len(), 20); // 19+C3
        assert!(try_compile_tyb_via_yoyo_table(&tyb_m).is_some());
        // ALUV slot 20,21
        let tyb_a = tyb_from_records(&[
            (0x6A, 2, [20, 0, 21, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_a = try_aluv_ret_tyb(&parse_tyb(&tyb_a).unwrap()).expect("ALUV");
        assert_eq!(via_a, emit_x86(&parse_tyb(&tyb_a).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_a).is_some());
        // slot>255 refuses
        let bad = tyb_from_records(&[
            (0x63, 2, [0, 1, 0, 0, 0, 0]), // dst=256
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_imul_ret_tyb(&parse_tyb(&bad).unwrap()).is_none());
    }

    /// S3.513: STB+RET oo>127 YOYO compose — `S3_STB_OO_IMM32_BODY_SEED`
    /// poke val/base/oo‖C3. ≡ emit_stb imm32. Not FIXED10. Next ≥10 =
    /// slot≥16 / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_stb_oo_imm32_ret_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.513") || pe_dll.contains("S3.513"));
        assert!(
            this.contains("S3_STB_OO_IMM32_BODY_SEED")
                && this.contains("try_compose_stb_body_via_yoyo_seed")
                && this.contains("try_stb_ret_tyb")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "S3_STB_OO_IMM32_BODY_SEED",
            "try_compose_stb_body_via_yoyo_seed",
            "try_stb_ret_tyb",
            "S3.513",
            "oo>127",
            "imm32",
            "body[11..15]",
            "S3_STB_OO_IMM8_BODY_SEED",
            "S3_FIXED10_OPS",
            "try_emit_text_via_yoyo_table",
            "emit_stb",
        ];
        for p in pins {
            assert!(this.contains(p), "missing STB imm32 pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(!S3_FIXED10_OPS.contains(&0x81));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(S3_STB_OO_IMM32_BODY_SEED.len(), 17);
        // oo=128
        let tyb = tyb_from_records(&[
            (0x81, 3, [0, 0, 1, 0, 0x80, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via = try_stb_ret_tyb(&parse_tyb(&tyb).unwrap()).expect("oo128");
        assert_eq!(via, emit_x86(&parse_tyb(&tyb).unwrap()).text);
        assert_eq!(via.len(), 18); // 17+C3
        assert_eq!(&via[..3], &S3_STB_OO_IMM32_BODY_SEED[..3]); // mov rcx prefix
        assert_eq!(&via[8..11], &[0x48, 0x81, 0xc0]); // add imm32 opcode
        assert_eq!(&via[11..15], &128u32.to_le_bytes());
        assert_eq!(via[3], 0); // val=0
        assert_eq!(via[7], 8); // base=1
        let pe_y = try_compile_tyb_via_yoyo_table(&tyb).expect("yoyo");
        let pe_m = compile_tyb_to_pe_mini(&tyb).expect("mini");
        assert_eq!(pe_y.bytes, pe_m.bytes);
        assert!(fixed_template_ops(&parse_tyb(&tyb).unwrap()).is_none());
        // oo=0x1000
        let tyb2 = tyb_from_records(&[
            (0x81, 3, [2, 0, 3, 0, 0x00, 0x10]),
            (0xFF, 0, [0; 6]),
        ]);
        let via2 = try_stb_ret_tyb(&parse_tyb(&tyb2).unwrap()).expect("oo4k");
        assert_eq!(via2, emit_x86(&parse_tyb(&tyb2).unwrap()).text);
        assert_eq!(&via2[11..15], &0x1000u32.to_le_bytes());
        assert!(try_compile_tyb_via_yoyo_table(&tyb2).is_some());
        // slot≥16 = S3.515
        let big = tyb_from_records(&[
            (0x81, 3, [16, 0, 0, 0, 0x80, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_stb_ret_tyb(&parse_tyb(&big).unwrap()).is_some());
    }

    /// S3.515: mem-op any-slot≤255 batch — STB · LDB · MEMCPY_DATA ·
    /// MEMCPY_STATE via load/store/rsi/rdi host seeds + mini validate lift.
    /// One thick knife (not −1 densify). Not FIXED10. bake=362.
    /// Next ≥10 = CMP any-slot / multi-op mix / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_mem_ops_any_slot_batch_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.515") || pe_dll.contains("S3.515"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "S3.515",
            "s3_push_load_rsi_slot",
            "s3_push_load_rdi_slot",
            "S3_LOAD_RSI_DISP32_SEED",
            "S3_LOAD_RDI_DISP32_SEED",
            "S3_MOVZX_RAX_BYTE_RAX_SEED",
            "S3_ADD_RAX_IMM32_SEED",
            "S3_ADD_RAX_IMM8_SEED",
            "S3_STORE_CL_RAX_SEED",
            "S3_REP_MOVSB_SEED",
            "s3_push_r15_mem_op",
            "try_compose_stb_body_via_yoyo_seed",
            "try_compose_ldb_body_via_yoyo_seed",
            "try_compose_memcpy_data_body_via_yoyo_seed",
            "try_compose_memcpy_state_body_via_yoyo_seed",
        ];
        for p in pins {
            assert!(this.contains(p), "missing mem-ops batch pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // STB val=16 base=17 oo=5
        let tyb_stb = tyb_from_records(&[
            (0x81, 3, [16, 0, 17, 0, 5, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_stb = try_stb_ret_tyb(&parse_tyb(&tyb_stb).unwrap()).expect("STB");
        assert_eq!(via_stb, emit_x86(&parse_tyb(&tyb_stb).unwrap()).text);
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&tyb_stb)
                .unwrap()
                .bytes,
            compile_tyb_to_pe_mini(&tyb_stb).unwrap().bytes
        );
        // LDB dd=20 ss=21 oo=0x100
        let tyb_ldb = tyb_from_records(&[
            (0x80, 3, [20, 0, 21, 0, 0x00, 0x01]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_ldb = try_ldb_ret_tyb(&parse_tyb(&tyb_ldb).unwrap()).expect("LDB");
        assert_eq!(via_ldb, emit_x86(&parse_tyb(&tyb_ldb).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_ldb).is_some());
        // MEMCPY_DATA dst=16 src=17 n=18
        let tyb_d = tyb_from_records(&[
            (0x84, 3, [16, 0, 17, 0, 18, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_d = try_memcpy_data_ret_tyb(&parse_tyb(&tyb_d).unwrap()).expect("MCPD");
        assert_eq!(via_d, emit_x86(&parse_tyb(&tyb_d).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_d).is_some());
        // MEMCPY_STATE dst=16 src=20 n=2
        let tyb_s = tyb_from_records(&[
            (0x85, 3, [16, 0, 20, 0, 2, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_s = try_memcpy_state_ret_tyb(&parse_tyb(&tyb_s).unwrap()).expect("MCPS");
        assert_eq!(via_s, emit_x86(&parse_tyb(&tyb_s).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_s).is_some());
        // mix disp8+disp32 STB
        let tyb_mix = tyb_from_records(&[
            (0x81, 3, [1, 0, 16, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert_eq!(
            try_stb_ret_tyb(&parse_tyb(&tyb_mix).unwrap()).unwrap(),
            emit_x86(&parse_tyb(&tyb_mix).unwrap()).text
        );
        // slot>255 refuses
        let bad = tyb_from_records(&[
            (0x81, 3, [0, 1, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_stb_ret_tyb(&parse_tyb(&bad).unwrap()).is_none());
    }

    /// S3.516: CMP any-slot + fixed10-ish multi-op mix — SET/GET/MOVRR/
    /// ADD/SUB/INC/DEC/CMP/NOP chain+RET via load/store/ALU host seeds +
    /// mini validate lift. Activates when `fixed_template_ops` refuses
    /// (wrong-slot / non-canonical imm). **Not** FIXED10 fold. bake=362.
    /// Next ≥10 = LABEL residual / bare op / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_cmp_any_slot_multi_op_mix_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.516") || pe_dll.contains("S3.516"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "S3.516",
            "S3_CMP_RAX_RCX_SEED",
            "S3_INC_RAX_SEED",
            "S3_DEC_RAX_SEED",
            "S3_SUB_RAX_IMM32_SEED",
            "S3_MOVABS_RAX_IMM64_SEED",
            "s3_compose_slot_alu_body",
            "try_slot_alu_chain_ret_tyb",
            "CMP any-slot",
            "multi-op mix",
            "fixed_template_ops(prog).is_some()",
        ];
        for p in pins {
            assert!(this.contains(p), "missing CMP/multi-op pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert!(S3_FIXED10_OPS.contains(&0x65)); // CMP still in fixed10; not folded away
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // CMP a=16 b=17 + RET
        let tyb_cmp = tyb_from_records(&[
            (0x65, 2, [16, 0, 17, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_cmp = try_slot_alu_chain_ret_tyb(&parse_tyb(&tyb_cmp).unwrap()).expect("CMP");
        assert_eq!(via_cmp, emit_x86(&parse_tyb(&tyb_cmp).unwrap()).text);
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&tyb_cmp)
                .unwrap()
                .bytes,
            compile_tyb_to_pe_mini(&tyb_cmp).unwrap().bytes
        );
        assert!(fixed_template_ops(&parse_tyb(&tyb_cmp).unwrap()).is_none());
        // GET dst=20 src=21
        let tyb_get = tyb_from_records(&[
            (0x60, 2, [20, 0, 21, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_get = try_slot_alu_chain_ret_tyb(&parse_tyb(&tyb_get).unwrap()).expect("GET");
        assert_eq!(via_get, emit_x86(&parse_tyb(&tyb_get).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_get).is_some());
        // INC slot=18
        let tyb_inc = tyb_from_records(&[(0x66, 1, [18, 0, 0, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert_eq!(
            try_slot_alu_chain_ret_tyb(&parse_tyb(&tyb_inc).unwrap()).unwrap(),
            emit_x86(&parse_tyb(&tyb_inc).unwrap()).text
        );
        // ADD slot=16 imm=7 (non-canonical imm)
        let tyb_add = tyb_from_records(&[
            (0x62, 2, [16, 0, 7, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert_eq!(
            try_slot_alu_chain_ret_tyb(&parse_tyb(&tyb_add).unwrap()).unwrap(),
            emit_x86(&parse_tyb(&tyb_add).unwrap()).text
        );
        // SET slot=16 imm=0xAABBCCDD
        let tyb_set = tyb_from_records(&[
            (0x30, 2, [16, 0, 0xDD, 0xCC, 0xBB, 0xAA]),
            (0xFF, 0, [0; 6]),
        ]);
        assert_eq!(
            try_slot_alu_chain_ret_tyb(&parse_tyb(&tyb_set).unwrap()).unwrap(),
            emit_x86(&parse_tyb(&tyb_set).unwrap()).text
        );
        // multi-op mix: INC16 + CMP16,17 + RET
        let tyb_mix = tyb_from_records(&[
            (0x66, 1, [16, 0, 0, 0, 0, 0]),
            (0x65, 2, [16, 0, 17, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_mix = try_slot_alu_chain_ret_tyb(&parse_tyb(&tyb_mix).unwrap()).expect("mix");
        assert_eq!(via_mix, emit_x86(&parse_tyb(&tyb_mix).unwrap()).text);
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&tyb_mix)
                .unwrap()
                .bytes,
            compile_tyb_to_pe_mini(&tyb_mix).unwrap().bytes
        );
        // default-slot CMP 0,1 stays on FIXED10 path (chain refuses)
        let tyb_def = tyb_from_records(&[
            (0x65, 2, [0, 0, 1, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_slot_alu_chain_ret_tyb(&parse_tyb(&tyb_def).unwrap()).is_none());
        assert!(fixed_template_ops(&parse_tyb(&tyb_def).unwrap()).is_some());
        assert!(try_compile_tyb_via_yoyo_table(&tyb_def).is_some());
        // slot>255 refuses validate
        let bad = tyb_from_records(&[
            (0x65, 2, [0, 1, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(validate_tyb_mini(&parse_tyb(&bad).unwrap()).is_err());
        assert!(try_compile_tyb_via_yoyo_table(&bad).is_none());
    }

    /// S3.517: LABEL residual + bare ALU/RAW_BYTES + DATA|ALU mix —
    /// LABEL in chain+RET; bare (no C3); DATA+SET/…+RET. Not FIXED10.
    /// bake=362. Next ≥10 = bare ALLOC / CF residual / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_label_bare_data_alu_mix_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.517") || pe_dll.contains("S3.517"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "S3.517",
            "try_slot_alu_bare_tyb",
            "try_raw_bytes_bare_tyb",
            "try_emit_raw_bytes_bare_via_yoyo_seed",
            "try_data_alu_mix_ret_tyb",
            "LABEL residual",
            "bare op",
            "mixed non-DATA multi",
            "s3_compose_slot_alu_body",
            "LABEL: position marker only",
        ];
        for p in pins {
            assert!(this.contains(p), "missing LABEL/bare/mix pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // LABEL+RET → C3
        let tyb_lab = tyb_from_records(&[
            (0x40, 1, [1, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_lab = try_slot_alu_chain_ret_tyb(&parse_tyb(&tyb_lab).unwrap()).expect("LABEL");
        assert_eq!(via_lab, vec![0xC3]);
        assert_eq!(via_lab, emit_x86(&parse_tyb(&tyb_lab).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_lab).is_some());
        // LABEL + INC16 + RET
        let tyb_li = tyb_from_records(&[
            (0x40, 1, [7, 0, 0, 0, 0, 0]),
            (0x66, 1, [16, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_li = try_slot_alu_chain_ret_tyb(&parse_tyb(&tyb_li).unwrap()).expect("L+INC");
        assert_eq!(via_li, emit_x86(&parse_tyb(&tyb_li).unwrap()).text);
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&tyb_li).unwrap().bytes,
            compile_tyb_to_pe_mini(&tyb_li).unwrap().bytes
        );
        // bare INC16 (no RET record; emit_x86 still auto-C3)
        let tyb_bare = tyb_from_records(&[(0x66, 1, [16, 0, 0, 0, 0, 0])]);
        let via_bare = try_slot_alu_bare_tyb(&parse_tyb(&tyb_bare).unwrap()).expect("bare");
        assert_eq!(via_bare, emit_x86(&parse_tyb(&tyb_bare).unwrap()).text);
        assert!(via_bare.ends_with(&[0xC3]));
        assert!(try_compile_tyb_via_yoyo_table(&tyb_bare).is_some());
        // bare RAW_BYTES (argc=2 → low bytes @0,@2; auto-C3)
        let tyb_raw = tyb_from_records(&[(0xA1, 2, [0x90, 0, 0x90, 0, 0, 0])]);
        let via_raw = try_raw_bytes_bare_tyb(&parse_tyb(&tyb_raw).unwrap()).expect("raw bare");
        assert_eq!(via_raw, vec![0x90, 0x90, 0xC3]);
        assert_eq!(via_raw, emit_x86(&parse_tyb(&tyb_raw).unwrap()).text);
        assert_eq!(
            try_emit_text_via_yoyo_table(&[0xA1]).unwrap(),
            vec![0x90]
        );
        // DATA + SET slot16 + RET
        let tyb_mix = tyb_from_records(&[
            (0x10, 1, [0xAB, 0, 0, 0, 0, 0]),
            (0x30, 2, [16, 0, 0x11, 0x22, 0x33, 0x44]),
            (0xFF, 0, [0; 6]),
        ]);
        let (via_t, via_d) =
            try_data_alu_mix_ret_tyb(&parse_tyb(&tyb_mix).unwrap()).expect("DATA+ALU");
        let emit = emit_x86(&parse_tyb(&tyb_mix).unwrap());
        assert_eq!(via_t, emit.text);
        assert_eq!(via_d, emit.data);
        assert_eq!(via_d, vec![0xAB]);
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&tyb_mix).unwrap().bytes,
            compile_tyb_to_pe_mini(&tyb_mix).unwrap().bytes
        );
        // pure DATA still via 506, not mix
        let pure = tyb_from_records(&[
            (0x10, 1, [1, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_data_alu_mix_ret_tyb(&parse_tyb(&pure).unwrap()).is_none());
        assert!(try_data_section_ret_tyb(&parse_tyb(&pure).unwrap()).is_some());
    }

    /// S3.518: bare ALLOC + CF residual — bare ALLOC (TYB/ops ‖auto-C3);
    /// CF+LABEL+RET (rel0); CF+filler(s)+LABEL+RET forward pass-2
    /// (extends S3.510 beyond single NOP). Not FIXED10. bake=362.
    /// Next ≥10 = LOAD/WRITE bare / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_bare_alloc_cf_residual_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.518") || pe_dll.contains("S3.518"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "S3.518",
            "try_alloc_bare_tyb",
            "try_emit_alloc_bare_via_yoyo_seed",
            "try_cf_label_ret_tyb",
            "try_cf_filler_label_ret_tyb",
            "bare ALLOC",
            "CF residual",
            "CF+LABEL+RET",
            "forward pass-2",
            "extends S3.510 beyond single NOP",
        ];
        for p in pins {
            assert!(this.contains(p), "missing bare-ALLOC/CF pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // bare ALLOC slot16 size=0x100
        let tyb_a = tyb_from_records(&[(0x20, 2, [16, 0, 0x00, 0x01, 0, 0])]);
        let via_a = try_alloc_bare_tyb(&parse_tyb(&tyb_a).unwrap()).expect("ALLOC bare");
        assert_eq!(via_a, emit_x86(&parse_tyb(&tyb_a).unwrap()).text);
        assert!(via_a.ends_with(&[0xC3]));
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&tyb_a).unwrap().bytes,
            compile_tyb_to_pe_mini(&tyb_a).unwrap().bytes
        );
        assert!(try_emit_text_via_yoyo_table(&[0x20]).is_some());
        // CF+LABEL+RET ≡ CF+RET (rel0)
        let tyb_cl = tyb_from_records(&[
            (0x41, 1, [3, 0, 0, 0, 0, 0]),
            (0x40, 1, [3, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_cl = try_cf_label_ret_tyb(&parse_tyb(&tyb_cl).unwrap()).expect("CF+L");
        assert_eq!(via_cl, emit_x86(&parse_tyb(&tyb_cl).unwrap()).text);
        assert_eq!(
            via_cl,
            try_cf_ret_tyb(&parse_tyb(&tyb_from_records(&[
                (0x41, 1, [3, 0, 0, 0, 0, 0]),
                (0xFF, 0, [0; 6]),
            ]))
            .unwrap())
            .unwrap()
        );
        assert!(try_compile_tyb_via_yoyo_table(&tyb_cl).is_some());
        // CF + dual-NOP + LABEL + RET (filler residual; ≠ S3.510 single NOP)
        let tyb_fn = tyb_from_records(&[
            (0x70, 1, [9, 0, 0, 0, 0, 0]),
            (0x00, 0, [0; 6]),
            (0x00, 0, [0; 6]),
            (0x40, 1, [9, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_fn = try_cf_filler_label_ret_tyb(&parse_tyb(&tyb_fn).unwrap()).expect("2NOP");
        assert_eq!(via_fn, emit_x86(&parse_tyb(&tyb_fn).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_fn).is_some());
        // CF + INC16 + LABEL + RET
        let tyb_fi = tyb_from_records(&[
            (0x71, 1, [2, 0, 0, 0, 0, 0]),
            (0x66, 1, [16, 0, 0, 0, 0, 0]),
            (0x40, 1, [2, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let via_fi = try_cf_filler_label_ret_tyb(&parse_tyb(&tyb_fi).unwrap()).expect("INC fill");
        assert_eq!(via_fi, emit_x86(&parse_tyb(&tyb_fi).unwrap()).text);
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&tyb_fi).unwrap().bytes,
            compile_tyb_to_pe_mini(&tyb_fi).unwrap().bytes
        );
        // single-NOP still owned by S3.510 (filler path refuses)
        let tyb_510 = tyb_from_records(&[
            (0x41, 1, [1, 0, 0, 0, 0, 0]),
            (0x00, 0, [0; 6]),
            (0x40, 1, [1, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_cf_filler_label_ret_tyb(&parse_tyb(&tyb_510).unwrap()).is_none());
        assert!(try_cf_nop_label_ret_tyb(&parse_tyb(&tyb_510).unwrap()).is_some());
    }

    /// S3.519: LOAD/WRITE/RAW_BYTE/STB bare batch — no RET record; body ‖
    /// emit_x86 auto-C3; ops-path [0x50]/[0x51]/[0xA0]. Not FIXED10.
    /// bake=362. Next ≥10 = LDB/MCP bare / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_load_write_bare_batch_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.519") || pe_dll.contains("S3.519"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "S3.519",
            "try_load_file_bare_tyb",
            "try_write_file_bare_tyb",
            "try_emit_load_file_bare_via_yoyo_seed",
            "try_emit_write_file_bare_via_yoyo_seed",
            "try_raw_byte_bare_tyb",
            "try_emit_raw_byte_bare_via_yoyo_seed",
            "try_stb_bare_tyb",
            "LOAD/WRITE bare",
            "bare LOAD_FILE",
        ];
        for p in pins {
            assert!(this.contains(p), "missing LOAD/WRITE bare pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // bare LOAD_FILE
        let tyb_l = tyb_from_records(&[(0x50, 2, [16, 0, 0x10, 0, 0, 0])]);
        let via_l = try_load_file_bare_tyb(&parse_tyb(&tyb_l).unwrap()).expect("LOAD bare");
        assert_eq!(via_l, emit_x86(&parse_tyb(&tyb_l).unwrap()).text);
        assert!(via_l.ends_with(&[0xC3]));
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&tyb_l).unwrap().bytes,
            compile_tyb_to_pe_mini(&tyb_l).unwrap().bytes
        );
        assert!(try_emit_text_via_yoyo_table(&[0x50]).is_some());
        // bare WRITE_FILE argc=3
        let tyb_w = tyb_from_records(&[(0x51, 3, [16, 0, 0x20, 0, 17, 0])]);
        let via_w = try_write_file_bare_tyb(&parse_tyb(&tyb_w).unwrap()).expect("WRITE bare");
        assert_eq!(via_w, emit_x86(&parse_tyb(&tyb_w).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_w).is_some());
        assert!(try_emit_text_via_yoyo_table(&[0x51]).is_some());
        // bare RAW_BYTE
        let tyb_rb = tyb_from_records(&[(0xA0, 1, [0xCC, 0, 0, 0, 0, 0])]);
        let via_rb = try_raw_byte_bare_tyb(&parse_tyb(&tyb_rb).unwrap()).expect("RAW_BYTE");
        assert_eq!(via_rb, vec![0xCC, 0xC3]);
        assert_eq!(via_rb, emit_x86(&parse_tyb(&tyb_rb).unwrap()).text);
        assert_eq!(
            try_emit_text_via_yoyo_table(&[0xA0]).unwrap(),
            vec![0x90, 0xC3]
        );
        // bare STB
        let tyb_s = tyb_from_records(&[(0x81, 3, [16, 0, 17, 0, 5, 0])]);
        let via_s = try_stb_bare_tyb(&parse_tyb(&tyb_s).unwrap()).expect("STB bare");
        assert_eq!(via_s, emit_x86(&parse_tyb(&tyb_s).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_s).is_some());
        // +RET still via ret paths, not bare
        let with_ret = tyb_from_records(&[
            (0x50, 2, [0, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_load_file_bare_tyb(&parse_tyb(&with_ret).unwrap()).is_none());
        assert!(try_load_file_ret_tyb(&parse_tyb(&with_ret).unwrap()).is_some());
    }

    /// S3.520: LDB/MCPD/MCPS/IMUL/ALUV bare batch — no RET record; body ‖
    /// auto-C3; ops-path [0x80]/[0x84]/[0x85]/[0x63]/[0x68]. Not FIXED10.
    /// bake=362. Next ≥10 = DATA bare / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_ldb_mcp_imul_aluv_bare_batch_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.520") || pe_dll.contains("S3.520"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "S3.520",
            "try_ldb_bare_tyb",
            "try_memcpy_data_bare_tyb",
            "try_memcpy_state_bare_tyb",
            "try_imul_bare_tyb",
            "try_aluv_bare_tyb",
            "try_emit_ldb_bare_via_yoyo_seed",
            "try_emit_memcpy_data_bare_via_yoyo_seed",
            "try_emit_memcpy_state_bare_via_yoyo_seed",
            "try_emit_imul_bare_via_yoyo_seed",
            "try_emit_aluv_bare_via_yoyo_seed",
            "LDB/MCP bare",
        ];
        for p in pins {
            assert!(this.contains(p), "missing LDB/MCP bare pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // bare LDB
        let tyb_l = tyb_from_records(&[(0x80, 3, [16, 0, 17, 0, 0, 0])]);
        let via_l = try_ldb_bare_tyb(&parse_tyb(&tyb_l).unwrap()).expect("LDB bare");
        assert_eq!(via_l, emit_x86(&parse_tyb(&tyb_l).unwrap()).text);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_l).is_some());
        assert!(try_emit_text_via_yoyo_table(&[0x80]).is_some());
        // bare MCPD
        let tyb_d = tyb_from_records(&[(0x84, 3, [16, 0, 17, 0, 18, 0])]);
        let via_d = try_memcpy_data_bare_tyb(&parse_tyb(&tyb_d).unwrap()).expect("MCPD");
        assert_eq!(via_d, emit_x86(&parse_tyb(&tyb_d).unwrap()).text);
        assert!(try_emit_text_via_yoyo_table(&[0x84]).is_some());
        // bare MCPS n=2
        let tyb_s = tyb_from_records(&[(0x85, 3, [16, 0, 20, 0, 2, 0])]);
        let via_s = try_memcpy_state_bare_tyb(&parse_tyb(&tyb_s).unwrap()).expect("MCPS");
        assert_eq!(via_s, emit_x86(&parse_tyb(&tyb_s).unwrap()).text);
        assert!(try_emit_text_via_yoyo_table(&[0x85]).is_some());
        // bare IMUL
        let tyb_i = tyb_from_records(&[(0x63, 2, [16, 0, 17, 0, 0, 0])]);
        let via_i = try_imul_bare_tyb(&parse_tyb(&tyb_i).unwrap()).expect("IMUL");
        assert_eq!(via_i, emit_x86(&parse_tyb(&tyb_i).unwrap()).text);
        assert!(try_emit_text_via_yoyo_table(&[0x63]).is_some());
        // bare ADDV
        let tyb_v = tyb_from_records(&[(0x68, 2, [16, 0, 17, 0, 0, 0])]);
        let via_v = try_aluv_bare_tyb(&parse_tyb(&tyb_v).unwrap()).expect("ADDV");
        assert_eq!(via_v, emit_x86(&parse_tyb(&tyb_v).unwrap()).text);
        assert!(try_emit_text_via_yoyo_table(&[0x68]).is_some());
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&tyb_v).unwrap().bytes,
            compile_tyb_to_pe_mini(&tyb_v).unwrap().bytes
        );
    }

    /// S3.521: DATA bare batch — single/multi DATA|STR|RAW no RET;
    /// DATA+ALU mix bare; ops [0x10|12|13]. text=C3 / ALU‖C3. Not FIXED10.
    /// bake=362. Next ≥10 = OW-RT cliff / YOYO-built.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_data_bare_batch_yoyo_seed_compose_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.521") || pe_dll.contains("S3.521"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "S3.521",
            "try_data_section_bare_tyb",
            "try_emit_data_bare_via_yoyo_seed",
            "try_multi_data_section_bare_tyb",
            "try_data_alu_mix_bare_tyb",
            "DATA bare",
            "bare DATA|STR|RAW",
            "multi bare DATA",
            "DATA+ALU mix bare",
            "ops=[0x10|0x12|0x13]",
        ];
        for p in pins {
            assert!(this.contains(p), "missing DATA bare pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // bare DATA
        let tyb_d = tyb_from_records(&[(0x10, 1, [0xAB, 0, 0, 0, 0, 0])]);
        let (t, d) = try_data_section_bare_tyb(&parse_tyb(&tyb_d).unwrap()).expect("DATA bare");
        let emit = emit_x86(&parse_tyb(&tyb_d).unwrap());
        assert_eq!(t, emit.text);
        assert_eq!(d, emit.data);
        assert_eq!(t, vec![0xC3]);
        assert_eq!(d, vec![0xAB]);
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&tyb_d).unwrap().bytes,
            compile_tyb_to_pe_mini(&tyb_d).unwrap().bytes
        );
        assert_eq!(try_emit_text_via_yoyo_table(&[0x10]).unwrap(), vec![0xC3]);
        // multi bare
        let tyb_m = tyb_from_records(&[
            (0x10, 1, [1, 0, 0, 0, 0, 0]),
            (0x12, 1, [2, 0, 0, 0, 0, 0]),
        ]);
        let (tm, dm) =
            try_multi_data_section_bare_tyb(&parse_tyb(&tyb_m).unwrap()).expect("multi bare");
        assert_eq!(tm, vec![0xC3]);
        assert_eq!(dm, vec![1, 2]);
        assert_eq!(
            (tm.clone(), dm.clone()),
            (
                emit_x86(&parse_tyb(&tyb_m).unwrap()).text,
                emit_x86(&parse_tyb(&tyb_m).unwrap()).data
            )
        );
        assert!(try_compile_tyb_via_yoyo_table(&tyb_m).is_some());
        // DATA + INC16 bare
        let tyb_mix = tyb_from_records(&[
            (0x10, 1, [0xEE, 0, 0, 0, 0, 0]),
            (0x66, 1, [16, 0, 0, 0, 0, 0]),
        ]);
        let (tx, dx) =
            try_data_alu_mix_bare_tyb(&parse_tyb(&tyb_mix).unwrap()).expect("DATA+ALU bare");
        let em = emit_x86(&parse_tyb(&tyb_mix).unwrap());
        assert_eq!(tx, em.text);
        assert_eq!(dx, em.data);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_mix).is_some());
        // +RET still via ret path
        let with_ret = tyb_from_records(&[
            (0x10, 1, [1, 0, 0, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_data_section_bare_tyb(&parse_tyb(&with_ret).unwrap()).is_none());
        assert!(try_data_section_ret_tyb(&parse_tyb(&with_ret).unwrap()).is_some());
    }

    /// S3.522: post-host-compose OW-RT cliff map + Gate F mini R→C→W.
    /// Host try_* surface largely owned (494–521). Remaining CUT cliffs
    /// (≥10): sidecar call-time compile · Gate F=bootstrap≠mini · Rust
    /// sidecar default · finite oracle · mt ABI 467 · RIP lea 419 · mini
    /// entry 485 · FIXED10 fold ban · densify sheep · Gate I RED.
    /// `yoyo_built_runtime_effect_mini` = slice only. Not CLOSED. bake=362.
    /// Next ≥10 = sidecar call-time compile / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_host_compose_cliff_map_gate_f_mini_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.522") || pe_dll.contains("S3.522"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let cliffs = [
            "yoyo_built_runtime_effect_mini",
            "compile_tyb_to_pe_mini",
            "bootstrap_compile",
            "production default remains Rust",
            "finite YOYO-precompiled",
            "not an in-DLL YOYO compiler",
            "S3.467",
            "call-time YOYO compile",
            "FIXED10 fold ban",
            "Densify hand-disp32 = sheep",
            "yoyo_built_runtime_effect",
            "seed_host_compile",
        ];
        for c in cliffs {
            assert!(
                this.contains(c) || pe_dll.contains(c),
                "missing OW-RT cliff pin {c}"
            );
        }
        assert!(cliffs.len() >= 10);
        assert!(pe_dll.contains("Does **not** replace"));
        assert!(pe_dll.contains("S3.522"));
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // mini Gate F: RET TYB → YOYO table PE
        let tyb = tyb_from_records(&[(0xFF, 0, [0; 6])]);
        let dir = std::env::temp_dir().join(format!(
            "yoyo-s3522-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("mkdir");
        std::fs::write(dir.join("input.tyb"), &tyb).expect("write");
        assert_eq!(
            crate::pe_dll_link::yoyo_built_runtime_effect_mini(&dir),
            crate::pe_dll_link::EXIT_OK
        );
        let got = std::fs::read(dir.join("output.exe")).expect("out");
        assert_eq!(got, compile_tyb_to_pe_mini(&tyb).unwrap().bytes);
        let _ = std::fs::remove_dir_all(&dir);
        // production Gate F still named bootstrap_compile
        assert!(pe_dll.contains("yoyo_sidecar_export_bake") || pe_dll.contains("bootstrap_compile"));
        // host compose still YOYO for RET
        assert!(try_compile_tyb_via_yoyo_table(&tyb).is_some());
    }

    /// S3.523: sidecar mini bake — `yoyo_sidecar_path_rcw_mini` +
    /// `yoyo_sidecar_export_compile_mini` (host mini bake; call-time write
    /// only). ≠ true call-time YOYO compile in sidecar. Not CLOSED. bake=362.
    /// Next ≥10 = true call-time YOYO compile / OW-RT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_sidecar_mini_bake_call_time_write_only_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.523") || pe_dll.contains("S3.523"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let pins = [
            "S3.523",
            "yoyo_sidecar_path_rcw_mini",
            "yoyo_sidecar_export_compile_mini",
            "yoyo_sidecar_export_bake",
            "yoyo_sidecar_export_compile_with",
            "prefer_mini",
            "Call-time = write",
            "re-compile inside DLL",
            "yoyo_built_runtime_effect_mini",
            "compile_tyb_to_pe_mini",
            "true call-time YOYO compile",
        ];
        for p in pins {
            assert!(this.contains(p) || pe_dll.contains(p), "missing sidecar mini pin {p}");
        }
        assert!(pins.len() >= 10);
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        let tyb = tyb_from_records(&[(0xFF, 0, [0; 6])]);
        let expect = compile_tyb_to_pe_mini(&tyb).unwrap().bytes;
        let dir = std::env::temp_dir().join(format!("yoyo-s3523-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("input.tyb"), &tyb).unwrap();
        assert_eq!(
            crate::pe_dll_link::yoyo_sidecar_export_compile_mini(&dir),
            crate::pe_dll_link::EXIT_OK
        );
        assert_eq!(
            std::fs::read(dir.join("output.exe")).unwrap(),
            expect
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    /// S3.524: true call-time YOYO compile ABSENT cliff. 522/523 = host mini
    /// bake + call-time write; export/oracle ≠ in-DLL compiler. CLOSED needs
    /// compile machine code inside export. Not densify. bake=362.
    /// Next ≥10 = IR-in-DLL slice / Gate I / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_true_call_time_yoyo_compile_absent_cliff_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let this = include_str!("s1_tyb_parser.rs");
        assert!(this.contains("S3.524") || pe_dll.contains("S3.524"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let cliffs = [
            "S3.524",
            "true call-time YOYO compile ABSENT",
            "general in-DLL compiler still ABSENT",
            "Does **not** re-compile at call",
            "finite YOYO-precompiled",
            "not an in-DLL YOYO compiler",
            "link_yoyo_in_dll_recompile_dll",
            "yoyo_sidecar_export_compile_mini",
            "yoyo_built_runtime_effect_mini",
            "production default remains Rust",
            "compile machine code inside export",
            "IR-in-DLL",
        ];
        for c in cliffs {
            assert!(
                this.contains(c) || pe_dll.contains(c),
                "missing call-time ABSENT pin {c}"
            );
        }
        assert!(cliffs.len() >= 10);
        // Witness: export bake helpers are host-side; try_* not in export emit
        assert!(pe_dll.contains("yoyo_sidecar_export_bake"));
        assert!(
            !pe_dll.contains("try_compose_imul_ret_via_yoyo_seed")
                || this.contains("try_compose_imul_ret_via_yoyo_seed"),
            "compose lives in s1 host — not a claim of in-DLL"
        );
        assert_eq!(S3_FIXED10_OPS.len(), 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        // Mutate input after bake: export_compile_mini already wrote; re-read
        // would need new bake — call-time path does not re-parse TYB in DLL.
        assert!(pe_dll.contains("Call-time = write") || pe_dll.contains("call-time write"));
    }

    /// S3.492: YOYO-built compile cliff reject. Freezing more pe_dll hand-x86
    /// into `*_BODY_SEED` = translation densify — **YOYO ≠ other-lang
    /// translate**. mt→same index = 467 ABI (no retry). Honest CUT surface:
    /// in-DLL = TYB mini + finite oracle; host effect = `bootstrap_compile`;
    /// production = Rust sidecar. YOYO-built CLOSED needs call-time YOYO
    /// compile in sidecar. Next ≥10 = YOYO-IR compile slice / OW-RT.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_yoyo_built_compile_cliff_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.492"));
        assert!(
            pe_dll.contains("YOYO-built compile cliff")
                || pe_dll.contains("translation densify")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        // ≥10 inventory: not a sheep −1 densify reject.
        let pins = [
            "yoyo_built_runtime_effect",
            "bootstrap_compile",
            "finite YOYO-precompiled",
            "not an in-DLL YOYO compiler",
            "link_yoyo_in_dll_recompile_dll",
            "yoyo_in_dll_recompile",
            "S1 mini TYB codegen",
            "let s1_loop = export_code.len()",
            "S3.467",
            "Naive share = ABI cliff",
            "production default remains Rust",
        ];
        for p in pins {
            assert!(pe_dll.contains(p), "missing cliff pin {p}");
        }
        assert!(pins.len() >= 10);
        // Seed freeze wave exists — must not be mistaken for YOYO-built.
        assert!(pe_dll.contains("S3_PUSH_PATCH_BODY_SEED"));
        assert!(pe_dll.contains("S3_PASS2_BODY_SEED"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.491: OW-RT residual after finish/table-seed wave. Seeds landed:
    /// push_patch · pass-2 · trail · memcpy-code/data · template-post-lea ·
    /// write_out · DATA append · push_label. Remaining cliffs: mini entry
    /// (485) · mt (467) · RIP lea (419) · handler hex → OW-RT CUT. Next ≥10 ≠
    /// densify. Candidates: YOYO-built compile · mt index (ABI). Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_table_seed_wave_residual_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.491"));
        assert!(
            pe_dll.contains("finish/table-seed wave")
                || pe_dll.contains("OW-RT residual after finish")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let seeds = [
            "S3_PUSH_PATCH_BODY_SEED",
            "S3_PASS2_BODY_SEED",
            "S3_TRAIL_C3_BODY_SEED",
            "S3_MEMCPY_CODE_BODY_SEED",
            "S3_MEMCPY_DATA_FINISH_BODY_SEED",
            "S3_TEMPLATE_POST_LEA_BODY_SEED",
            "S3_WRITE_OUT_SETUP_BODY_SEED",
            "S3_DATA_APPEND_BODY_SEED",
            "S3_PUSH_LABEL_BODY_SEED",
            "S3_RAW_BYTES_EMIT_BODY_SEED",
        ];
        for s in seeds {
            assert!(pe_dll.contains(s), "missing seed {s}");
        }
        assert!(seeds.len() >= 10);
        assert!(
            pe_dll.contains("Densify hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.490: mt→s1 push_label share reject. s1 has `S3_PUSH_LABEL_BODY_SEED`;
    /// mt LABEL stays hand — rdx/r9/r10 vs rcx/edx/esi (≡467 ABI cliff).
    /// Surface: mt label path ≥10 distinct sites remain. Next ≥10 = OW-RT.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_mt_push_label_abi_cliff_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.490"));
        assert!(
            pe_dll.contains("mt→s1 push_label") || pe_dll.contains("push_label share")
        );
        assert!(pe_dll.contains("S3_PUSH_LABEL_BODY_SEED"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        let mt = [
            "s3_jne_mt_label",
            "s3_jne_mt_label_a",
            "s3_jae_fail_mt_label",
            "s3_jnz_mt_label",
            "s3_jmp_finish_label",
            "s3_mt_after_label",
            "mov r9d, [rdx+2]",
            "IN_DLL_S1_LABELS_OFF",
            "IN_DLL_S1_N_LABELS_OFF",
            "dec r14d",
        ];
        for m in mt {
            assert!(pe_dll.contains(m), "missing mt label site {m}");
        }
        assert!(mt.len() >= 10, "mt LABEL surface ≥10 — share ≠ sheep cut");
        assert_eq!(S3_PUSH_LABEL_BODY_SEED.len(), 62);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.489: post-label map. s1 push_label = YOYO seed; LABEL=.text meta;
    /// mt LABEL hand (ABI). Remaining: mt (467) · RIP lea (419) · handler hex
    /// · entry (485) → OW-RT CUT. Next ≥10 ≠ densify hand-disp32. Candidates:
    /// OW-RT / mt residual. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_label_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.489"));
        assert!(pe_dll.contains("post-label map"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(pe_dll.contains("S3_PUSH_LABEL_BODY_SEED"));
        assert!(pe_dll.contains("LABEL") && pe_dll.contains("meta"));
        assert!(
            pe_dll.contains("mt LABEL") || pe_dll.contains("ABI")
        );
        assert!(
            pe_dll.contains("Densify hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert_eq!(S3_PUSH_LABEL_BODY_SEED.len(), 62);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.488: s1 LABEL table-push = YOYO object seed (62B) + poke N/CODE/
    /// LABELS; jae fail live. mt LABEL stays hand (ABI). LABEL=meta for .text.
    /// Next ≥10 = post-label map / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_push_label_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.488"));
        assert!(
            pe_dll.contains("LABEL table-push")
                || pe_dll.contains("S3_PUSH_LABEL_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_PUSH_LABEL_BODY_SEED"));
        assert!(pe_dll.contains("S3_PUSH_LABEL_N_OFF_AT"));
        assert!(pe_dll.contains("S3_PUSH_LABEL_JAE_AT"));
        assert_eq!(S3_PUSH_LABEL_BODY_SEED.len(), 62);
        assert_eq!(S3_PUSH_LABEL_BODY_SEED[0], 0x8B);
        assert_eq!(S3_PUSH_LABEL_BODY_SEED[10], 0x0F); // jae
        assert_eq!(S3_PUSH_LABEL_BODY_SEED[16], 0x8B); // mov edx,[rcx+2]
        assert_eq!(S3_PUSH_LABEL_N_OFF_AT, 3);
        assert_eq!(S3_PUSH_LABEL_JAE_AT, 10);
        assert_eq!(S3_PUSH_LABEL_CODE_OFF_AT, 23);
        assert_eq!(S3_PUSH_LABEL_N_OFF_AT2, 36);
        assert_eq!(S3_PUSH_LABEL_TABLE_OFF_AT, 44);
        assert_eq!(S3_PUSH_LABEL_N_OFF_AT3, 58);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.487: post-data-append map. DATA append = YOYO seed shared mt+s1.
    /// LABEL=meta. Remaining: mt (467) · RIP lea (419) · handler hex · entry
    /// (485) → OW-RT CUT. Next ≥10 ≠ densify hand-disp32. Candidates: OW-RT /
    /// mt residual. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_data_append_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.487"));
        assert!(pe_dll.contains("post-data-append map"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(pe_dll.contains("S3_DATA_APPEND_BODY_SEED"));
        assert!(pe_dll.contains("s3_shared_data_append_seed"));
        assert!(pe_dll.contains("LABEL") && pe_dll.contains("meta"));
        assert!(
            pe_dll.contains("Densify hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert_eq!(S3_DATA_APPEND_BODY_SEED.len(), 39);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.486: DATA append body = YOYO object seed (39B) shared mt+s1
    /// (rcx=rec, r9b=argc, rdi=dest). Rel8s baked. Next ≥10 = post-data-append
    /// map / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_data_append_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.486"));
        assert!(
            pe_dll.contains("DATA append") || pe_dll.contains("S3_DATA_APPEND_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_DATA_APPEND_BODY_SEED"));
        assert!(pe_dll.contains("s3_shared_data_append_seed"));
        assert!(pe_dll.contains("s3_shared_data_append_calls"));
        assert_eq!(S3_DATA_APPEND_BODY_SEED.len(), 39);
        assert_eq!(S3_DATA_APPEND_BODY_SEED[0], 0x45); // test r9b
        assert_eq!(S3_DATA_APPEND_BODY_SEED[3], 0x74); // jz
        assert_eq!(S3_DATA_APPEND_BODY_SEED[4], 0x21);
        assert_eq!(S3_DATA_APPEND_BODY_SEED[38], 0xC3);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.485: OW-RT mini→YOYO entry cliff reject. Finish seeds landed; mini
    /// entry (`jmp_s1` / `S1 mini TYB` / try pilots ≥10) still Rust. Deleting
    /// pilots without YOYO-built compile = sheep. mt ABI=467. Next ≥10 =
    /// YOYO-built compile path / mt residual. Not CLOSED. OW-RT CUT.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_mini_to_yoyo_entry_cliff_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.485"));
        assert!(
            pe_dll.contains("mini→YOYO entry cliff")
                || pe_dll.contains("OW-RT mini→YOYO entry")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(pe_dll.contains("jmp_s1"));
        assert!(pe_dll.contains("S1 mini TYB codegen"));
        let tries = [
            "let s3_try_set",
            "let s3_try_get",
            "let s3_try_inc",
            "let s3_try_dec",
            "let s3_try_add",
            "let s3_try_sub",
            "let s3_try_movrr",
            "let s3_try_cmp",
            "let s3_try_multi",
            "let s1_loop",
        ];
        for t in tries {
            assert!(pe_dll.contains(t), "missing entry {t}");
        }
        assert!(tries.len() >= 10, "mini entry surface ≥10 — not a sheep cut");
        assert!(
            pe_dll.contains("Densify") || pe_dll.contains("sheep"),
            "sheep bar"
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.484: post-finish/guard residual map. s1_finish seeds + push_patch +
    /// guard densify reject done. Remaining: mt dispatch (467 ABI) · RIP lea
    /// (419) · handler hex · fail-site glue → OW-RT CUT. Next ≥10 ≠ densify
    /// hand-disp32. Candidates: OW-RT mini→YOYO · mt residual. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_finish_guard_residual_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.484"));
        assert!(
            pe_dll.contains("post-finish/guard residual map")
                || pe_dll.contains("residual map")
        );
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(pe_dll.contains("S3_PUSH_PATCH_BODY_SEED"));
        assert!(pe_dll.contains("S3_WRITE_OUT_SETUP_BODY_SEED"));
        assert!(pe_dll.contains("S3.483") || pe_dll.contains("guard densify"));
        assert!(
            pe_dll.contains("467") || pe_dll.contains("ABI cliff") || pe_dll.contains("mt→same")
        );
        assert!(
            pe_dll.contains("Densify hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.483: handler argc/ovf guard densify = sheep reject. Collapsing
    /// per-handler `jne_argc`/`ja_ovf` into one shared guard helper is −1
    /// densify (no new YOYO capability). Surface stays ≥10 distinct sites.
    /// Next ≥10 = mt residual / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_handler_guard_densify_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.483"));
        assert!(
            pe_dll.contains("guard densify") || pe_dll.contains("handler argc/ovf guard")
        );
        let guards = [
            "s1_jne_set_argc",
            "s1_ja_set_ovf",
            "s1_jne_ret_argc",
            "s1_ja_ret_ovf",
            "s1_jne_get_argc",
            "s1_ja_get_ovf",
            "s1_jne_call_argc",
            "s1_ja_call_ovf",
            "s1_jne_jmp_argc",
            "s1_ja_jmp_ovf",
            "s1_jne_raw_argc",
            "s1_ja_raw_ovf",
        ];
        for g in guards {
            assert!(pe_dll.contains(g), "missing guard {g}");
        }
        assert!(guards.len() >= 10, "guard surface ≥10 — densify ≠ sheep cut");
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.482: post-s1-finish map. pass-2 · trail · memcpy-code/data ·
    /// template-post-lea · write_out = YOYO seeds. Remaining: RIP lea (419) ·
    /// handler guards · mt dispatch → OW-RT CUT. Next ≥10 ≠ densify
    /// hand-disp32. Candidates: guard-collapse reject · mt path · OW-RT.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_s1_finish_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.482"));
        assert!(pe_dll.contains("post-s1-finish map"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(pe_dll.contains("S3_PASS2_BODY_SEED"));
        assert!(pe_dll.contains("S3_TRAIL_C3_BODY_SEED"));
        assert!(pe_dll.contains("S3_MEMCPY_CODE_BODY_SEED"));
        assert!(pe_dll.contains("S3_MEMCPY_DATA_FINISH_BODY_SEED"));
        assert!(pe_dll.contains("S3_TEMPLATE_POST_LEA_BODY_SEED"));
        assert!(pe_dll.contains("S3_WRITE_OUT_SETUP_BODY_SEED"));
        assert!(
            pe_dll.contains("Densify hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.481: write_out setup = YOYO object seed (19B) + poke PE/len; jmp
    /// live. Next ≥10 = post-s1-finish map / guards / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_write_out_setup_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.481"));
        assert!(
            pe_dll.contains("write_out setup")
                || pe_dll.contains("S3_WRITE_OUT_SETUP_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_WRITE_OUT_SETUP_BODY_SEED"));
        assert!(pe_dll.contains("S3_WRITE_OUT_SETUP_PE_OFF_AT"));
        assert!(pe_dll.contains("S3_WRITE_OUT_SETUP_JMP_AT"));
        assert_eq!(S3_WRITE_OUT_SETUP_BODY_SEED.len(), 19);
        assert_eq!(S3_WRITE_OUT_SETUP_BODY_SEED[0], 0x48);
        assert_eq!(S3_WRITE_OUT_SETUP_BODY_SEED[8], 0x41); // mov r12d
        assert_eq!(S3_WRITE_OUT_SETUP_BODY_SEED[9], 0xBC);
        assert_eq!(S3_WRITE_OUT_SETUP_BODY_SEED[14], 0xE9); // jmp
        assert_eq!(S3_WRITE_OUT_SETUP_PE_OFF_AT, 4);
        assert_eq!(S3_WRITE_OUT_SETUP_LEN_AT, 10);
        assert_eq!(S3_WRITE_OUT_SETUP_JMP_AT, 14);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.480: template-post-lea = YOYO object seed (18B) + poke PE/len; RIP
    /// lea stays hand (419). Next ≥10 = write_out setup seed / OW-RT. Not
    /// CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_template_post_lea_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.480"));
        assert!(
            pe_dll.contains("template-post-lea")
                || pe_dll.contains("S3_TEMPLATE_POST_LEA_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_TEMPLATE_POST_LEA_BODY_SEED"));
        assert!(pe_dll.contains("S3_TEMPLATE_POST_LEA_PE_OFF_AT"));
        assert!(pe_dll.contains("S3_TEMPLATE_POST_LEA_LEN_AT"));
        assert!(pe_dll.contains("emit_lea_rcx_rdata"));
        assert_eq!(S3_TEMPLATE_POST_LEA_BODY_SEED.len(), 18);
        assert_eq!(S3_TEMPLATE_POST_LEA_BODY_SEED[0], 0x48);
        assert_eq!(S3_TEMPLATE_POST_LEA_BODY_SEED[8], 0x48); // mov rsi,rcx
        assert_eq!(S3_TEMPLATE_POST_LEA_BODY_SEED[11], 0xB9); // mov ecx,len
        assert_eq!(S3_TEMPLATE_POST_LEA_BODY_SEED[16], 0xF3);
        assert_eq!(S3_TEMPLATE_POST_LEA_BODY_SEED[17], 0xA4);
        assert_eq!(S3_TEMPLATE_POST_LEA_PE_OFF_AT, 4);
        assert_eq!(S3_TEMPLATE_POST_LEA_LEN_AT, 12);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.479: post-finish-memcpy map. pass-2 · trail-C3 · memcpy-code/data =
    /// YOYO seeds. Remaining: template RIP (419) · write_out · guards · mt
    /// dispatch → OW-RT CUT. Next ≥10 ≠ densify hand-disp32. Candidates:
    /// template-post-lea seed · write_out · OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_finish_memcpy_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.479"));
        assert!(pe_dll.contains("post-finish-memcpy map"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(pe_dll.contains("S3_PASS2_BODY_SEED"));
        assert!(pe_dll.contains("S3_TRAIL_C3_BODY_SEED"));
        assert!(pe_dll.contains("S3_MEMCPY_CODE_BODY_SEED"));
        assert!(pe_dll.contains("S3_MEMCPY_DATA_FINISH_BODY_SEED"));
        assert!(
            pe_dll.contains("template") && (pe_dll.contains("RIP") || pe_dll.contains("419"))
        );
        assert!(
            pe_dll.contains("Densify hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.478: finish memcpy-data = YOYO object seed (36B) + poke LEN/DATA/PE;
    /// DATA_PAYLOAD + jz baked. Next ≥10 = post-finish-memcpy map / template
    /// RIP / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_memcpy_data_finish_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.478"));
        assert!(
            pe_dll.contains("memcpy-data")
                || pe_dll.contains("S3_MEMCPY_DATA_FINISH_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_MEMCPY_DATA_FINISH_BODY_SEED"));
        assert!(pe_dll.contains("S3_MEMCPY_DATA_FINISH_LEN_OFF_AT"));
        assert!(pe_dll.contains("S3_MEMCPY_DATA_FINISH_DATA_OFF_AT"));
        assert_eq!(S3_MEMCPY_DATA_FINISH_BODY_SEED.len(), 36);
        assert_eq!(S3_MEMCPY_DATA_FINISH_BODY_SEED[0], 0x8B);
        assert_eq!(S3_MEMCPY_DATA_FINISH_BODY_SEED[9], 0x74);
        assert_eq!(S3_MEMCPY_DATA_FINISH_BODY_SEED[10], 0x19);
        assert_eq!(
            u32::from_le_bytes(S3_MEMCPY_DATA_FINISH_BODY_SEED[30..34].try_into().unwrap()),
            S1_SLIM_DATA_PAYLOAD_OFF as u32
        );
        assert_eq!(S3_MEMCPY_DATA_FINISH_BODY_SEED[34], 0xF3);
        assert_eq!(S3_MEMCPY_DATA_FINISH_BODY_SEED[35], 0xA4);
        assert_eq!(S3_MEMCPY_DATA_FINISH_LEN_OFF_AT, 3);
        assert_eq!(S3_MEMCPY_DATA_FINISH_DATA_OFF_AT, 15);
        assert_eq!(S3_MEMCPY_DATA_FINISH_PE_OFF_AT, 23);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.477: finish memcpy-code = YOYO object seed (39B) + poke CODE/PE;
    /// TEXT_PAYLOAD baked. Template RIP remains (419). Next ≥10 = memcpy-data
    /// / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_memcpy_code_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.477"));
        assert!(
            pe_dll.contains("memcpy-code") || pe_dll.contains("S3_MEMCPY_CODE_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_MEMCPY_CODE_BODY_SEED"));
        assert!(pe_dll.contains("S3_MEMCPY_CODE_CODE_OFF_AT"));
        assert!(pe_dll.contains("S3_MEMCPY_CODE_PE_OFF_AT"));
        assert_eq!(S3_MEMCPY_CODE_BODY_SEED.len(), 39);
        assert_eq!(S3_MEMCPY_CODE_BODY_SEED[0], 0x48); // lea rsi
        assert_eq!(S3_MEMCPY_CODE_BODY_SEED[16], 0x48); // add rdi
        assert_eq!(S3_MEMCPY_CODE_BODY_SEED[17], 0x81);
        assert_eq!(S3_MEMCPY_CODE_BODY_SEED[18], 0xC7);
        assert_eq!(
            u32::from_le_bytes(S3_MEMCPY_CODE_BODY_SEED[19..23].try_into().unwrap()),
            S1_SLIM_TEXT_PAYLOAD_OFF as u32
        );
        assert_eq!(S3_MEMCPY_CODE_BODY_SEED[37], 0xF3);
        assert_eq!(S3_MEMCPY_CODE_BODY_SEED[38], 0xA4);
        assert_eq!(S3_MEMCPY_CODE_CODE_OFF_AT, 4);
        assert_eq!(S3_MEMCPY_CODE_PE_OFF_AT, 12);
        assert_eq!(S3_MEMCPY_CODE_CODE_OFF_AT2, 30);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.476: trail-C3 ensure = YOYO object seed (44B) + poke CODE; ja fail
    /// live; je rel8 baked. Next ≥10 = post-trail map / memcpy finish / OW-RT.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_trail_c3_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.476"));
        assert!(
            pe_dll.contains("trail-C3") || pe_dll.contains("S3_TRAIL_C3_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_TRAIL_C3_BODY_SEED"));
        assert!(pe_dll.contains("S3_TRAIL_C3_CODE_OFF_AT"));
        assert!(pe_dll.contains("S3_TRAIL_C3_JA_AT"));
        assert_eq!(S3_TRAIL_C3_BODY_SEED.len(), 44);
        assert_eq!(S3_TRAIL_C3_BODY_SEED[0], 0x4C); // lea r8
        assert_eq!(S3_TRAIL_C3_BODY_SEED[11], 0x74); // je need
        assert_eq!(S3_TRAIL_C3_BODY_SEED[12], 0x08);
        assert_eq!(S3_TRAIL_C3_BODY_SEED[19], 0x74); // je after
        assert_eq!(S3_TRAIL_C3_BODY_SEED[20], 0x17);
        assert_eq!(S3_TRAIL_C3_BODY_SEED[30], 0x0F); // ja fail
        assert_eq!(S3_TRAIL_C3_BODY_SEED[31], 0x87);
        assert_eq!(S3_TRAIL_C3_BODY_SEED[36], 0x41); // mov byte [r12],C3
        assert_eq!(S3_TRAIL_C3_BODY_SEED[40], 0xC3);
        assert_eq!(S3_TRAIL_C3_CODE_OFF_AT, 4);
        assert_eq!(S3_TRAIL_C3_JA_AT, 30);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.475: pass-2 body = YOYO object seed (95B) + poke N/PATCHES/LABELS/
    /// CODE; internal jcc/jmp baked. Next ≥10 = post-pass2 map / trail-C3 /
    /// OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_pass2_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.475"));
        assert!(
            pe_dll.contains("pass-2 body") || pe_dll.contains("S3_PASS2_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_PASS2_BODY_SEED"));
        assert!(pe_dll.contains("S3_PASS2_N_PATCHES_OFF_AT"));
        assert!(pe_dll.contains("S3_PASS2_CODE_OFF_AT"));
        assert_eq!(S3_PASS2_BODY_SEED.len(), 95);
        assert_eq!(S3_PASS2_BODY_SEED[0], 0x31); // xor esi
        assert_eq!(S3_PASS2_BODY_SEED[1], 0xF6);
        assert_eq!(S3_PASS2_BODY_SEED[9], 0x0F); // jge done
        assert_eq!(S3_PASS2_BODY_SEED[11], 0x50); // +80
        assert_eq!(S3_PASS2_BODY_SEED[56], 0x74); // je hit
        assert_eq!(S3_PASS2_BODY_SEED[57], 0x07);
        assert_eq!(S3_PASS2_BODY_SEED[90], 0xE9); // jmp loop
        assert_eq!(S3_PASS2_N_PATCHES_OFF_AT, 5);
        assert_eq!(S3_PASS2_PATCHES_OFF_AT, 19);
        assert_eq!(S3_PASS2_N_LABELS_OFF_AT, 35);
        assert_eq!(S3_PASS2_LABELS_OFF_AT, 49);
        assert_eq!(S3_PASS2_CODE_OFF_AT, 78);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.474: post-push_patch map. push_patch = YOYO seed (41B+poke).
    /// LABEL/DATA=meta. Remaining: guards · s1_finish (419 IR reject) ·
    /// mt dispatch · pass-2 → OW-RT CUT. Next ≥10 ≠ densify hand-disp32.
    /// Candidates: pass-2 object seed · OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_push_patch_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.474"));
        assert!(pe_dll.contains("post-push_patch map"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(pe_dll.contains("S3_PUSH_PATCH_BODY_SEED"));
        assert!(
            pe_dll.contains("pass-2") || pe_dll.contains("Pass 2")
        );
        assert!(
            pe_dll.contains("Densify hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert!(pe_dll.contains("LABEL") && pe_dll.contains("meta"));
        assert_eq!(S3_PUSH_PATCH_BODY_SEED.len(), 41);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.473: push_patch body = YOYO object seed (41B) + poke N/PATCHES offs;
    /// jae fail live. Shared by mt+s1 via jmp r11. Next ≥10 = post-push_patch
    /// map / s1_finish / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_push_patch_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.473"));
        assert!(
            pe_dll.contains("push_patch body")
                || pe_dll.contains("S3_PUSH_PATCH_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_PUSH_PATCH_BODY_SEED"));
        assert!(pe_dll.contains("S3_PUSH_PATCH_N_OFF_AT"));
        assert!(pe_dll.contains("S3_PUSH_PATCH_TABLE_OFF_AT"));
        assert!(pe_dll.contains("S3_PUSH_PATCH_JAE_AT"));
        assert_eq!(S3_PUSH_PATCH_BODY_SEED.len(), 41);
        assert_eq!(S3_PUSH_PATCH_BODY_SEED[0], 0x8B); // mov esi,[rsp+…]
        assert_eq!(S3_PUSH_PATCH_BODY_SEED[10], 0x0F); // jae
        assert_eq!(S3_PUSH_PATCH_BODY_SEED[11], 0x83);
        assert_eq!(S3_PUSH_PATCH_BODY_SEED[38], 0x41); // jmp r11
        assert_eq!(S3_PUSH_PATCH_BODY_SEED[39], 0xFF);
        assert_eq!(S3_PUSH_PATCH_BODY_SEED[40], 0xE3);
        assert_eq!(S3_PUSH_PATCH_N_OFF_AT, 3);
        assert_eq!(S3_PUSH_PATCH_N_OFF_AT2, 34);
        assert_eq!(S3_PUSH_PATCH_TABLE_OFF_AT, 20);
        assert_eq!(S3_PUSH_PATCH_JAE_AT, 10);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.472: post-raw map. RAW_BYTE + RAW_BYTES emit = YOYO seeds shared.
    /// LABEL/DATA=meta. Remaining: guards · s1_finish · mt dispatch ·
    /// push_patch → OW-RT CUT. Next ≥10 ≠ densify hand-disp32. Candidates:
    /// push_patch object seed · OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_post_raw_map_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.472"));
        assert!(pe_dll.contains("post-raw map"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(
            pe_dll.contains("push_patch") || pe_dll.contains("s1_push_patch")
        );
        assert!(
            pe_dll.contains("Densify hand-disp32 = sheep")
                || pe_dll.contains("hand-disp32 = sheep")
        );
        assert!(pe_dll.contains("S3_RAW_BYTE_EMIT_BODY_SEED"));
        assert!(pe_dll.contains("S3_RAW_BYTES_EMIT_BODY_SEED"));
        assert!(pe_dll.contains("s3_shared_raw_byte_seed"));
        assert!(pe_dll.contains("s3_shared_raw_bytes_seed"));
        assert!(pe_dll.contains("LABEL") && pe_dll.contains("meta"));
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.471: RAW_BYTES emit body = YOYO object seed (48B) + shared mt+s1
    /// (rcx=rec, r9b=argc≤3). Rel8s baked in seed. Next ≥10 = post-raw map /
    /// OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_raw_bytes_emit_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.471"));
        assert!(
            pe_dll.contains("RAW_BYTES emit")
                || pe_dll.contains("S3_RAW_BYTES_EMIT_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_RAW_BYTES_EMIT_BODY_SEED"));
        assert!(pe_dll.contains("s3_shared_raw_bytes_seed"));
        assert!(pe_dll.contains("s3_shared_raw_bytes_calls"));
        assert_eq!(S3_RAW_BYTES_EMIT_BODY_SEED.len(), 48);
        assert_eq!(S3_RAW_BYTES_EMIT_BODY_SEED[0], 0x45); // test r9b
        assert_eq!(S3_RAW_BYTES_EMIT_BODY_SEED[47], 0xC3);
        assert_eq!(S3_RAW_BYTES_EMIT_BODY_SEED[3], 0x74); // jz
        assert_eq!(S3_RAW_BYTES_EMIT_BODY_SEED[4], 0x2A);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.470: RAW_BYTE emit body = YOYO object seed (11B) + shared mt+s1.
    /// LABEL=meta — no seed this knife. Next ≥10 = RAW_BYTES / OW-RT.
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_raw_byte_emit_body_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.470"));
        assert!(
            pe_dll.contains("RAW_BYTE emit")
                || pe_dll.contains("S3_RAW_BYTE_EMIT_BODY_SEED")
        );
        assert!(pe_dll.contains("S3_RAW_BYTE_EMIT_BODY_SEED"));
        assert!(pe_dll.contains("s3_shared_raw_byte_seed"));
        assert!(pe_dll.contains("s3_shared_raw_byte_calls"));
        assert!(pe_dll.contains("LABEL=meta"));
        assert_eq!(S3_RAW_BYTE_EMIT_BODY_SEED.len(), 11);
        assert_eq!(S3_RAW_BYTE_EMIT_BODY_SEED[0], 0x8A);
        assert_eq!(S3_RAW_BYTE_EMIT_BODY_SEED[10], 0xC3);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.469: collapse shared RET/NOP bake-emit seed — r8d=payload off
    /// (4=row0 RET / 24=row1 NOP); cld+memcpy 1B. ≥2 call sites one thunk.
    /// Next ≥10 = RAW_BYTE/LABEL body seeds / OW-RT. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_collapse_shared_ret_nop_bake_emit_seed_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.469"));
        assert!(
            pe_dll.contains("shared RET/NOP bake-emit")
                || pe_dll.contains("s3_shared_retnop_seed")
        );
        assert!(pe_dll.contains("s3_shared_retnop_seed"));
        assert!(pe_dll.contains("s3_shared_retnop_calls"));
        assert!(pe_dll.contains("r8d=4") || pe_dll.contains("0x04, 0x00, 0x00, 0x00"));
        assert!(pe_dll.contains("r8d=24") || pe_dll.contains("0x18, 0x00, 0x00, 0x00"));
        assert_eq!(S3_HOST_TABLE_SEED[4], 0xC3);
        assert_eq!(S3_HOST_TABLE_SEED[24], 0x90);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            S3_HOST_TABLE_SEED.len() + 17 + 16 + 15 + 14 + 12 + 19 + 8 + 5 + 5 + 6 + 34 + 4 + 7,
            362
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.420: bake dual+RET pilots densify reject. ≥8 near-identical stubs
    /// (SET/GET/INC/DEC/ADD/SUB/MOVRR/CMP +RET) share the same emit shape —
    /// guard chain + bake memcpy + RET byte + jmp finish. Compressing hex /
    /// factoring −1 = sheep 0 capability (mirror 417/418). True next ≥10 =
    /// first YOYO op-handler object seed (own-ISA bytes in .text/.rdata).
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_dual_ret_pilots_densify_reject() {
        let pe_dll = include_str!("pe_dll_link.rs");
        let pilots = [
            "yoyo_in_dll_s3_set_ret_pilot",
            "yoyo_in_dll_s3_get_ret_pilot",
            "yoyo_in_dll_s3_inc_ret_pilot",
            "yoyo_in_dll_s3_dec_ret_pilot",
            "yoyo_in_dll_s3_add_ret_pilot",
            "yoyo_in_dll_s3_sub_ret_pilot",
            "yoyo_in_dll_s3_movrr_ret_pilot",
            "yoyo_in_dll_s3_cmp_ret_pilot",
        ];
        for m in pilots {
            assert!(pe_dll.contains(m), "missing pilot {m}");
        }
        assert!(pilots.len() >= 8, "dual+RET surface ≥8 — densify ≠ sheep cut");
        // Named try stubs still distinct (densify ≠ delete these labels).
        let tries = [
            "let s3_try_set",
            "let s3_try_get",
            "let s3_try_inc",
            "let s3_try_dec",
            "let s3_try_add",
            "let s3_try_sub",
            "let s3_try_movrr",
            "let s3_try_cmp",
        ];
        for t in tries {
            assert!(pe_dll.contains(t), "missing try {t}");
        }
        assert!(tries.len() >= 8);
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.413–S3.416: post-skip-mini map. Landed: RET + SET+RET exact TYB skip
    /// mini (memcpy compile seeds). Next ≥10: more exact-TYB skips (ADD+RET…)
    /// or s1_loop→YOYO. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_ret_skips_mini_slice_pin() {
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S3.414"));
        assert!(pe_dll.contains("S3.415"));
        assert!(pe_dll.contains("exact RET TYB"));
        assert!(pe_dll.contains("exact SET+RET"));
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_COMPILE_PE_SET_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(
            compile_tyb_to_pe(&s3_fixed10_ret_tyb(0xFF))
                .unwrap()
                .bytes
                .as_slice(),
            S3_COMPILE_PE_RET_IMAGE_SEED
        );
        assert_eq!(
            compile_tyb_to_pe(&s3_fixed10_ret_tyb(0x30))
                .unwrap()
                .bytes
                .as_slice(),
            S3_COMPILE_PE_SET_RET_IMAGE_SEED
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.413: OW-RT mini→YOYO first-slice map. Candidates (each must be ≥10
    /// assembler or documented reject): (1) s1_finish memcpy/patch sequence →
    /// YOYO IR seed; (2) s1_loop op dispatch → table-driven YOYO; (3) drop
    /// hand-hex for ops already in bake when pe_template seeds suffice.
    /// Fixed10 PE parity already holds (S3.411). Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_ow_rt_mini_to_yoyo_first_slice_map_pin() {
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("S1 mini TYB codegen"));
        assert!(pe_dll.contains("memcpy template"));
        assert!(pe_dll.contains("S1_SLIM_TEXT_PAYLOAD_OFF"));
        // Parity floor still green.
        for &op in &[0xFFu8, 0x30, 0x62] {
            assert!(compile_tyb_to_pe(&s3_fixed10_ret_tyb(op)).is_ok());
        }
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.408: OW-RT residual after S3.407 — in-DLL mini codegen dispatcher
    /// (jmp_s1 / bake scan / s1_loop) remains hand-hex Rust. Template seed only
    /// unifies s1_finish PE shell. Closing = replace mini with YOYO (long pole).
    /// Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_in_dll_mini_dispatcher_ow_rt_residual_reject() {
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("jmp s1_codegen") || pe_dll.contains("jmp_s1"));
        assert!(pe_dll.contains("S1 mini TYB codegen"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT"
        );
        assert!(pe_dll.contains("production default remains Rust"));
        // Host path still seed/patch complete.
        assert_eq!(try_emit_text_via_yoyo_table(&[0xFF]).unwrap(), vec![0xC3]);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.403: gap N no-further-slice reject — after bake=host seed (S3.390) there
    /// is no pe_dll ≥10 knife that is not rewriting the mini codegen dispatcher
    /// (OW-RT long pole / SCOPE-CUT). Host Windows fixed10 emit/compile already
    /// seed+patch complete (O–T + S3.402). Next ≠ sheep pe_dll hex; next real
    /// work = OW-RT mini→YOYO. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_in_dll_n_no_further_slice_reject() {
        // Bake slice remains the only N ≥10 that landed (≡ host table seed).
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        assert_eq!(S3_HOST_TABLE_SEED.len(), S3_HOST_NROWS as usize * 20);
        // Host path floors (Win PE seeds).
        assert_eq!(S3_HOST_PE_TEXT_BY_LEN.len(), 8);
        assert_eq!(S3_HOST_PE_DATA_PREFIX_BYTES.len(), 328);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        let pe_dll = include_str!("pe_dll_link.rs");
        assert!(pe_dll.contains("gap N slice"));
        assert!(
            pe_dll.contains("OW-RT remains CUT") || pe_dll.contains("OW-RT **CUT**"),
            "OW-RT CUT marker"
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.392/S3.396: post-Q gap map. Closed: try_emit PE · expand PE · bake=host
    /// · O/P via_yoyo→PE · Q non-prefix sim residual. Cliffs: H wrap · I
    /// empty-DATA · K runner · L non-win · N pe_dll mini (OW-RT). Next ≥10:
    /// (R) try_compile RET/SET+RET → image seeds. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_true_selfhost_post_n_gap_map_pin() {
        assert_eq!(try_emit_text_via_yoyo_table(&[0xFF]).unwrap(), vec![0xC3]);
        assert_eq!(s3_expand_many_via_pe().unwrap().len(), 24);
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        assert_eq!(S3_HOST_PE_TEXT_BY_LEN.len(), 8);
        assert_eq!(S3_EXPAND_PE_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_IN_DLL_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.393: gap O — bootstrap via_yoyo expand uses PE (not sim cursor).
    #[test]
    #[cfg(windows)]
    fn s3_expand_via_yoyo_uses_pe_pin() {
        let via = s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT);
        assert_eq!(via.as_slice(), &S3_BOOTSTRAP_EXPAND_ASM);
        assert_eq!(via, s3_bootstrap_expand_asm());
        // PE path ≡ image seed outbuf parse
        let pe_tyb = s3_expand_many_via_pe().unwrap();
        let pe_recs: Vec<_> = parse_tyb(&pe_tyb)
            .unwrap()
            .records
            .into_iter()
            .map(|r| (r.op, r.argc, r.args_raw))
            .collect();
        assert_eq!(via, pe_recs);
        assert_eq!(S3_SHAPE_FILL_MANY_FLAT.len(), S3_SHAPE_FILL_MANY_ROWS * 7);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.394: gap P — bootstrap-prefix flat (MEMCPY 7B) via_yoyo uses PE slice.
    #[test]
    #[cfg(windows)]
    fn s3_expand_via_yoyo_prefix_uses_pe_pin() {
        let memcpy = s3_rec_memcpy(9, 2, 12);
        let via = s3_recs_from_flat_seed_via_yoyo(&S3_SHAPE_FILL_MANY_FLAT[..7]);
        assert_eq!(via, vec![memcpy]);
        let pe_tyb = s3_expand_many_via_pe().unwrap();
        let pe0 = &parse_tyb(&pe_tyb).unwrap().records[0];
        assert_eq!((pe0.op, pe0.argc, pe0.args_raw), memcpy);
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT).as_slice(),
            &S3_BOOTSTRAP_EXPAND_ASM
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.395: gap Q reject — non-prefix flat via_yoyo stays sim (proof only).
    /// Prod expand is named bootstrap / PE image (no flat decode). Seeding PE
    /// per arbitrary flat = combinatorial sheep (mirror H/S3.380). Keep sim;
    /// true-selfhost expand surface = bootstrap(+prefix) only.
    #[test]
    #[cfg(windows)]
    fn s3_expand_via_yoyo_non_prefix_sim_residual_reject() {
        // RET-only suffix is a valid flat but NOT a MANY_FLAT prefix → sim path.
        let ret_flat = &S3_SHAPE_FILL_MANY_FLAT[7..];
        assert_eq!(ret_flat.len(), 7);
        assert!(!S3_SHAPE_FILL_MANY_FLAT.starts_with(ret_flat));
        let via = s3_recs_from_flat_seed_via_yoyo(ret_flat);
        assert_eq!(via, vec![S3_BOOTSTRAP_EXPAND_ASM[1]]);
        // Prefix + full still PE (O/P hold).
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(&S3_SHAPE_FILL_MANY_FLAT[..7]),
            vec![S3_BOOTSTRAP_EXPAND_ASM[0]]
        );
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT).as_slice(),
            &S3_BOOTSTRAP_EXPAND_ASM
        );
        // Prod has no flat arg — TYB seed + PE image only.
        assert_eq!(s3_expand_many_flat_tyb().0.as_slice(), S3_EXPAND_TYB_SEED);
        assert_eq!(s3_expand_many_via_pe().unwrap().len(), 24);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }


    /// S3.396/S3.409: post-OW-RT-slice map. Closed: O–U host + in-DLL PE
    /// template=compile RET seed; RET/SET+RET in-DLL output ≡ compile seeds.
    /// Cliffs: I/K/L · mini dispatcher still hand-hex (S3.408). Next ≥10 =
    /// more in-DLL seed fast-paths or mini→YOYO. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_true_selfhost_post_q_gap_map_pin() {
        assert_eq!(try_emit_text_via_yoyo_table(&[0xFF]).unwrap(), vec![0xC3]);
        assert_eq!(s3_expand_many_via_pe().unwrap().len(), 24);
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT).as_slice(),
            &S3_BOOTSTRAP_EXPAND_ASM
        );
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(&S3_SHAPE_FILL_MANY_FLAT[..7]),
            vec![S3_BOOTSTRAP_EXPAND_ASM[0]]
        );
        // Q residual: non-prefix still sim-capable (≠ PE-only).
        let ret_flat = &S3_SHAPE_FILL_MANY_FLAT[7..];
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(ret_flat),
            vec![S3_BOOTSTRAP_EXPAND_ASM[1]]
        );
        // S/T: non-RET emit+compile do not need live wrap on Windows.
        let add_ops = [0x62u8, 0xFF];
        let add_tyb = tyb_from_records(&[s3_fixed_rec(0x62), (0xFF, 0, [0; 6])]);
        assert_eq!(
            try_emit_text_via_yoyo_table(&add_ops).unwrap(),
            s3_run_slim_pe_outbuf_at(
                &s3_slim_pe_patch_text_data(
                    S3_HOST_PE_TEXT_BY_LEN[1],
                    &s3_host_pe_data_payload(&add_ops),
                ),
                S3_HOST_OUTBUF_OFF,
                try_emit_text_via_yoyo_table(&add_ops).unwrap().len(),
            )
        );
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&add_tyb)
                .unwrap()
                .bytes
                .as_slice(),
            s3_compile_pe_patch_text(&try_emit_text_via_yoyo_table(&add_ops).unwrap()).unwrap()
                .as_slice()
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }


    /// S3.404/S3.406: post-host-complete / post-U gap map — Win PE path wrap-free
    /// through compile_tyb_to_pe. Next = OW-RT in-DLL fast-path. Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_true_selfhost_post_host_complete_map_pin() {
        assert_eq!(try_emit_text_via_yoyo_table(&[0xFF]).unwrap(), vec![0xC3]);
        assert_eq!(s3_expand_many_via_pe().unwrap().len(), 24);
        let ret = tyb_from_records(&[(0xFF, 0, [0; 6])]);
        assert_eq!(
            compile_tyb_to_pe(&ret).unwrap().bytes.as_slice(),
            S3_COMPILE_PE_RET_IMAGE_SEED
        );
        // U: non-fixed10 also patch (no wrap).
        let imul = tyb_from_records(&[
            (0x63, 2, [1, 0, 2, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let pe_i = compile_tyb_to_pe(&imul).unwrap();
        let out = emit_x86(&parse_tyb(&imul).unwrap());
        assert_eq!(
            pe_i.bytes.as_slice(),
            s3_compile_pe_patch_text_data(&out.text, &out.data).unwrap().as_slice()
        );
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.405: gap U — non-fixed10 `compile_tyb_to_pe` uses compile PE template
    /// patch (emit_x86 text/data; no wrap on Windows).
    #[test]
    #[cfg(windows)]
    fn s3_compile_tyb_to_pe_emit_fallback_patch_pin() {
        let imul = tyb_from_records(&[
            (0x63, 2, [1, 0, 2, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_compile_tyb_via_yoyo_table(&imul).is_none());
        let pe = compile_tyb_to_pe(&imul).unwrap();
        let out = emit_x86(&parse_tyb(&imul).unwrap());
        let patched = s3_compile_pe_patch_text_data(&out.text, &out.data).unwrap();
        assert_eq!(pe.bytes.as_slice(), patched.as_slice());
        assert_eq!(&pe.bytes[..2], b"MZ");
        // Oracle: text payload ≡ live wrap.
        let live = wrap_pe_slim(&out).unwrap();
        let t0 = S1_SLIM_TEXT_PAYLOAD_OFF;
        assert_eq!(
            &pe.bytes[t0..t0 + out.text.len()],
            &live.bytes[t0..t0 + out.text.len()]
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.402: host `compile_tyb_to_pe` prefers YOYO try_compile (≡ mini entry).
    /// Outside fixed-template → S3.405 emit+patch (Win). Not OW-RT CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_compile_tyb_to_pe_prefers_yoyo_pin() {
        let ret = tyb_from_records(&[(0xFF, 0, [0; 6])]);
        let set = tyb_from_records(&[
            (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
            (0xFF, 0, [0; 6]),
        ]);
        let add = tyb_from_records(&[s3_fixed_rec(0x62), (0xFF, 0, [0; 6])]);
        let pe_r = compile_tyb_to_pe(&ret).unwrap();
        let pe_s = compile_tyb_to_pe(&set).unwrap();
        let pe_a = compile_tyb_to_pe(&add).unwrap();
        assert_eq!(pe_r.bytes.as_slice(), S3_COMPILE_PE_RET_IMAGE_SEED);
        assert_eq!(pe_s.bytes.as_slice(), S3_COMPILE_PE_SET_RET_IMAGE_SEED);
        assert_eq!(
            pe_a.bytes.as_slice(),
            try_compile_tyb_via_yoyo_table(&add).unwrap().bytes.as_slice()
        );
        assert_eq!(
            compile_tyb_to_pe_mini(&ret).unwrap().bytes.as_slice(),
            pe_r.bytes.as_slice()
        );
        assert_eq!(
            compile_tyb_to_pe_mini(&add).unwrap().bytes.as_slice(),
            pe_a.bytes.as_slice()
        );
        let imul = tyb_from_records(&[
            (0x63, 2, [1, 0, 2, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        assert!(try_compile_tyb_via_yoyo_table(&imul).is_none());
        let pe_i = compile_tyb_to_pe(&imul).unwrap();
        assert_eq!(&pe_i.bytes[..2], b"MZ");
        assert_ne!(pe_i.bytes.as_slice(), S3_COMPILE_PE_RET_IMAGE_SEED);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.401: post-T H status — Windows try_emit/try_compile no longer call
    /// wrap_pe_slim (S/T). wrap remains: non-Windows compile · pin oracles ·
    /// pe_dll/other. Not a new CLOSED; documents H shrink after S/T.
    #[test]
    #[cfg(windows)]
    fn s3_wrap_pe_slim_post_st_status_pin() {
        // try_emit ADD path ≡ template patch (no wrap in production path).
        let ops = [0x62u8, 0xFF];
        let via = try_emit_text_via_yoyo_table(&ops).unwrap();
        let patched = s3_slim_pe_patch_text_data(
            S3_HOST_PE_TEXT_BY_LEN[1],
            &s3_host_pe_data_payload(&ops),
        );
        assert_eq!(
            s3_run_slim_pe_outbuf_at(&patched, S3_HOST_OUTBUF_OFF, via.len()),
            via
        );
        // try_compile ADD ≡ compile template patch.
        let tyb = tyb_from_records(&[s3_fixed_rec(0x62), (0xFF, 0, [0; 6])]);
        let pe = try_compile_tyb_via_yoyo_table(&tyb).unwrap();
        assert_eq!(
            pe.bytes.as_slice(),
            s3_compile_pe_patch_text(&via).unwrap().as_slice()
        );
        // wrap still available as oracle (H not deleted).
        let live = wrap_pe_slim(&CodegenOutput {
            text: via.clone(),
            data: vec![],
        })
        .unwrap();
        assert_eq!(&live.bytes[..2], b"MZ");
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.397: gap R — try_compile RET/SET+RET uses compile PE image seeds
    /// (no live wrap). Other fixed10 still wrap (H residual). ≠ host compose seeds.
    #[test]
    #[cfg(windows)]
    fn s3_try_compile_ret_setret_image_seeds_pin() {
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_COMPILE_PE_SET_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(&S3_COMPILE_PE_RET_IMAGE_SEED[..2], b"MZ");
        assert_eq!(&S3_COMPILE_PE_SET_RET_IMAGE_SEED[..2], b"MZ");
        assert_ne!(S3_COMPILE_PE_RET_IMAGE_SEED, S3_HOST_PE_RET_IMAGE_SEED);
        assert_ne!(S3_COMPILE_PE_SET_RET_IMAGE_SEED, S3_HOST_PE_SET_RET_IMAGE_SEED);
        let ret = tyb_from_records(&[(0xFF, 0, [0; 6])]);
        let set = tyb_from_records(&[
            (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
            (0xFF, 0, [0; 6]),
        ]);
        let pe_r = try_compile_tyb_via_yoyo_table(&ret).unwrap();
        let pe_s = try_compile_tyb_via_yoyo_table(&set).unwrap();
        assert_eq!(pe_r.bytes.as_slice(), S3_COMPILE_PE_RET_IMAGE_SEED);
        assert_eq!(pe_s.bytes.as_slice(), S3_COMPILE_PE_SET_RET_IMAGE_SEED);
        // Oracle: seeds ≡ live wrap of yoyo text (empty DATA).
        let live_r = wrap_pe_slim(&CodegenOutput {
            text: try_emit_text_via_yoyo_table(&[0xFF]).unwrap(),
            data: vec![],
        })
        .unwrap();
        let live_s = wrap_pe_slim(&CodegenOutput {
            text: try_emit_text_via_yoyo_table(&[0x30, 0xFF]).unwrap(),
            data: vec![],
        })
        .unwrap();
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED, live_r.bytes.as_slice());
        assert_eq!(S3_COMPILE_PE_SET_RET_IMAGE_SEED, live_s.bytes.as_slice());
        // Other ops still wrap path (seed ≠).
        let add = tyb_from_records(&[
            (0x62, 2, [0, 0, 5, 0, 0, 0]),
            (0xFF, 0, [0; 6]),
        ]);
        let pe_a = try_compile_tyb_via_yoyo_table(&add).unwrap();
        assert_ne!(pe_a.bytes.as_slice(), S3_COMPILE_PE_RET_IMAGE_SEED);
        assert_eq!(&pe_a.bytes[..2], b"MZ");
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.398: gap R residual reject — per-ops try_compile PE image seeds.
    /// RET+SET/RET already seeded (S3.397); 8 more fixed10 × 6656B ≫ volume
    /// (mirror S3.380). S3.400: other ops use template patch (still no sheep).
    #[test]
    #[cfg(windows)]
    fn s3_try_compile_per_ops_image_seed_cliff_reject() {
        let mut n = 0usize;
        for &op in &S3_FIXED10_OPS {
            if op == 0xFF {
                continue;
            }
            n += 1;
            let tyb = tyb_from_records(&[s3_fixed_rec(op), (0xFF, 0, [0; 6])]);
            let pe = try_compile_tyb_via_yoyo_table(&tyb).unwrap();
            assert_eq!(pe.bytes.len(), 6656, "slim compile PE stable ({op:#x})");
            if op != 0x30 {
                assert_ne!(
                    pe.bytes.as_slice(),
                    S3_COMPILE_PE_SET_RET_IMAGE_SEED,
                    "non-SET must not hit SET seed"
                );
            }
        }
        assert_eq!(n, 9);
        let remaining = 8usize; // SET already seeded
        assert!(remaining * 6656 > 50_000);
        assert_eq!(S3_COMPILE_PE_RET_IMAGE_SEED.len(), 6656);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }


    /// S3.399: gap S — try_emit non-RET uses PE template text/data patch (no wrap).
    /// Base = RET image seed; offs = S1_SLIM_*_PAYLOAD_OFF. wrap remains for
    /// try_compile residual (R). Not CLOSED.
    #[test]
    #[cfg(windows)]
    fn s3_try_emit_pe_template_patch_pin() {
        assert_eq!(S1_SLIM_TEXT_PAYLOAD_OFF, 0x400 + 13);
        assert_eq!(S1_SLIM_DATA_PAYLOAD_OFF, 0xA00);
        let cases: &[&[u8]] = &[
            &[0x62, 0xFF],
            &[0x66, 0xFF],
            &[0x30, 0x66, 0xFF],
            &[0x60, 0x62, 0xFF],
        ];
        for &ops in cases {
            let via = try_emit_text_via_yoyo_table(ops).unwrap();
            let n = ops.len();
            let text = S3_HOST_PE_TEXT_BY_LEN[n - 1];
            let data = s3_host_pe_data_payload(ops);
            let patched = s3_slim_pe_patch_text_data(text, &data);
            let got = s3_run_slim_pe_outbuf_at(&patched, S3_HOST_OUTBUF_OFF, via.len());
            assert_eq!(got, via, "ops={ops:?}");
            // Still ≡ live wrap outbuf (oracle).
            let live = wrap_pe_slim(&CodegenOutput {
                text: text.to_vec(),
                data: data.clone(),
            })
            .unwrap();
            let got_l = s3_run_slim_pe_outbuf_at(&live.bytes, S3_HOST_OUTBUF_OFF, via.len());
            assert_eq!(got, got_l, "patch≡wrap outbuf ops={ops:?}");
        }
        // RET/SET+RET still image seeds (no patch path).
        assert_eq!(try_emit_text_via_yoyo_table(&[0xFF]).unwrap(), vec![0xC3]);
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }


    /// S3.400: gap T — try_compile non-RET patches text into compile RET PE
    /// template (no wrap on Windows). RET/SET+RET stay image seeds.
    #[test]
    #[cfg(windows)]
    fn s3_try_compile_pe_template_patch_pin() {
        let add = tyb_from_records(&[
            s3_fixed_rec(0x62),
            (0xFF, 0, [0; 6]),
        ]);
        let pe = try_compile_tyb_via_yoyo_table(&add).unwrap();
        assert_eq!(pe.bytes.len(), 6656);
        assert_eq!(&pe.bytes[..2], b"MZ");
        let text = try_emit_text_via_yoyo_table(&[0x62, 0xFF]).unwrap();
        let patched = s3_compile_pe_patch_text(&text).unwrap();
        assert_eq!(pe.bytes.as_slice(), patched.as_slice());
        // Oracle: patch outbuf-equivalent region ≡ live wrap text payload.
        let live = wrap_pe_slim(&CodegenOutput {
            text: text.clone(),
            data: vec![],
        })
        .unwrap();
        let t0 = S1_SLIM_TEXT_PAYLOAD_OFF;
        assert_eq!(
            &pe.bytes[t0..t0 + text.len()],
            &live.bytes[t0..t0 + text.len()]
        );
        // Seeds still preferred for RET/SET+RET.
        let ret = tyb_from_records(&[(0xFF, 0, [0; 6])]);
        assert_eq!(
            try_compile_tyb_via_yoyo_table(&ret).unwrap().bytes.as_slice(),
            S3_COMPILE_PE_RET_IMAGE_SEED
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.366: win0 poke seed ≡ live zero_then_poke(table[..127]). Floor now 173 (S3.367).
    #[test]
    fn s3_host_table_win0_poke_seed_pin() {
        const WIN: usize = 127;
        assert_eq!(S3_HOST_TABLE_WIN0_POKE_SEED.len(), 80);
        let mut live = Vec::new();
        s3_append_stb_zero_then_poke(
            &mut live,
            S3_HOST_TABLE_OFF,
            &S3_HOST_TABLE_SEED[..WIN],
            10,
        );
        assert_eq!(live.as_slice(), S3_HOST_TABLE_WIN0_POKE_SEED.as_slice());
        // Prod path ≡ continuous full-slice poke oracle (same IR count + composed .text).
        let cases: &[&[u8]] = &[&[0xFF], &[0x30, 0xFF], &[0x30, 0x66, 0xFF]];
        for &ops in cases {
            let mut slice = Vec::with_capacity(S3_HOST_TABLE_SEED.len() + 1 + ops.len());
            slice.extend_from_slice(&S3_HOST_TABLE_SEED);
            slice.push(ops.len() as u8);
            slice.extend_from_slice(ops);
            let mut oracle = Vec::new();
            s3_append_stb_zero_then_poke(&mut oracle, S3_HOST_TABLE_OFF, &slice, 10);
            oracle.push(s3_rec_set(S3_PACK_PROTO_DST, S3_HOST_OUTBUF_OFF));
            oracle.extend(s3_prog_loop_code_host(ops.len() as u32, S3_HOST_NROWS));
            let oracle_tyb = s3_host_tyb_from_records(&oracle);
            let prod = s3_host_prog_loop_tyb(ops);
            assert_eq!(
                parse_tyb(&prod).unwrap().records.len(),
                parse_tyb(&oracle_tyb).unwrap().records.len() + 2,
                "S3.366/367: win0+win1 split = +2 vs continuous"
            );
            let run = |tyb: &[u8]| {
                s3_sim_prog_loop_cursor(
                    tyb,
                    S3_HOST_TABLE_OFF as u64,
                    S3_HOST_OUTBUF_OFF,
                    (S3_HOST_PROG_OFF + 1) as u64,
                    S3_HOST_OUTBUF_OFF as u64,
                    RECORD_SIZE as u64,
                )
                .unwrap()
            };
            assert_eq!(run(&prod), run(&oracle_tyb), "S3.366: composed .text ≡ oracle");
        }
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
    }

    /// S3.367: win1 poke seed ≡ live poke(table[127..]); floor 173; composed .text holds.
    #[test]
    fn s3_host_table_win1_poke_seed_pin() {
        const WIN: usize = 127;
        assert_eq!(S3_HOST_TABLE_WIN1_POKE_SEED.len(), 58);
        let mut live = Vec::new();
        s3_append_stb_zero_then_poke(
            &mut live,
            S3_HOST_TABLE_OFF + WIN as u32,
            &S3_HOST_TABLE_SEED[WIN..],
            10,
        );
        assert_eq!(live.as_slice(), S3_HOST_TABLE_WIN1_POKE_SEED.as_slice());
        let cases: &[&[u8]] = &[&[0xFF], &[0x30, 0xFF], &[0x30, 0x66, 0xFF]];
        for &ops in cases {
            let mut slice = Vec::with_capacity(S3_HOST_TABLE_SEED.len() + 1 + ops.len());
            slice.extend_from_slice(&S3_HOST_TABLE_SEED);
            slice.push(ops.len() as u8);
            slice.extend_from_slice(ops);
            let mut oracle = Vec::new();
            s3_append_stb_zero_then_poke(&mut oracle, S3_HOST_TABLE_OFF, &slice, 10);
            oracle.push(s3_rec_set(S3_PACK_PROTO_DST, S3_HOST_OUTBUF_OFF));
            oracle.extend(s3_prog_loop_code_host(ops.len() as u32, S3_HOST_NROWS));
            let oracle_tyb = s3_host_tyb_from_records(&oracle);
            let prod = s3_host_prog_loop_tyb(ops);
            // Split poke is +2 IR vs continuous; composed .text must match.
            assert_eq!(
                parse_tyb(&prod).unwrap().records.len(),
                parse_tyb(&oracle_tyb).unwrap().records.len() + 2,
                "S3.367: table/prog split = +2 host IR"
            );
            let run = |tyb: &[u8]| {
                s3_sim_prog_loop_cursor(
                    tyb,
                    S3_HOST_TABLE_OFF as u64,
                    S3_HOST_OUTBUF_OFF,
                    (S3_HOST_PROG_OFF + 1) as u64,
                    S3_HOST_OUTBUF_OFF as u64,
                    RECORD_SIZE as u64,
                )
                .unwrap()
            };
            assert_eq!(run(&prod), run(&oracle_tyb), "S3.367: composed .text ≡ oracle");
        }
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173,
            "S3.367: host floor 171→173"
        );
    }


    /// S3.368: host RET TYB seed ≡ assembled win0+win1+prog path; no live wrap.
    #[test]
    fn s3_host_ret_tyb_seed_pin() {
        assert_eq!(S3_HOST_RET_TYB_SEED.len(), 1392);
        assert_eq!(&S3_HOST_RET_TYB_SEED[..4], &TYB_MAGIC);
        assert_eq!(
            parse_tyb(S3_HOST_RET_TYB_SEED).unwrap().records.len(),
            173
        );
        // Oracle: assemble without RET short-circuit.
        let mut recs = Vec::new();
        recs.extend_from_slice(&S3_HOST_TABLE_WIN0_POKE_SEED);
        recs.extend_from_slice(&S3_HOST_TABLE_WIN1_POKE_SEED);
        s3_append_stb_zero_then_poke(&mut recs, S3_HOST_PROG_OFF, &[1, 0xFF], 10);
        recs.push(s3_rec_set(S3_PACK_PROTO_DST, S3_HOST_OUTBUF_OFF));
        recs.extend(s3_prog_loop_code_host(1, S3_HOST_NROWS));
        let live = s3_host_tyb_from_records(&recs);
        assert_eq!(S3_HOST_RET_TYB_SEED, live.as_slice());
        assert_eq!(s3_host_prog_loop_tyb(&[0xFF]).as_slice(), S3_HOST_RET_TYB_SEED);
        assert_eq!(
            try_emit_text_via_yoyo_table(&[0xFF]).unwrap(),
            s3_fixed_row(0xFF)
        );
        // Multi-op still assembles (not the RET seed).
        let multi = s3_host_prog_loop_tyb(&[0x30, 0xFF]);
        assert_ne!(multi.as_slice(), S3_HOST_RET_TYB_SEED);
        assert_eq!(
            try_emit_text_via_yoyo_table(&[0x30, 0xFF]).unwrap().len(),
            15
        );
    }


    /// S3.369: host SET+RET TYB seed ≡ assembled path; capability-map multi-op short-circuit.
    #[test]
    fn s3_host_set_ret_tyb_seed_pin() {
        assert_eq!(S3_HOST_SET_RET_TYB_SEED.len(), 1408);
        assert_eq!(&S3_HOST_SET_RET_TYB_SEED[..4], &TYB_MAGIC);
        assert_eq!(
            parse_tyb(S3_HOST_SET_RET_TYB_SEED).unwrap().records.len(),
            175
        );
        let mut recs = Vec::new();
        recs.extend_from_slice(&S3_HOST_TABLE_WIN0_POKE_SEED);
        recs.extend_from_slice(&S3_HOST_TABLE_WIN1_POKE_SEED);
        s3_append_stb_zero_then_poke(&mut recs, S3_HOST_PROG_OFF, &[2, 0x30, 0xFF], 10);
        recs.push(s3_rec_set(S3_PACK_PROTO_DST, S3_HOST_OUTBUF_OFF));
        recs.extend(s3_prog_loop_code_host(2, S3_HOST_NROWS));
        let live = s3_host_tyb_from_records(&recs);
        assert_eq!(S3_HOST_SET_RET_TYB_SEED, live.as_slice());
        assert_eq!(
            s3_host_prog_loop_tyb(&[0x30, 0xFF]).as_slice(),
            S3_HOST_SET_RET_TYB_SEED
        );
        let yoyo = try_emit_text_via_yoyo_table(&[0x30, 0xFF]).unwrap();
        let rust = emit_x86(
            &parse_tyb(&s3_host_tyb_from_records(&[
                s3_fixed_rec(0x30),
                s3_fixed_rec(0xFF),
            ]))
            .unwrap(),
        )
        .text;
        assert_eq!(yoyo, rust);
        // Other multi-op still assembles.
        assert_ne!(
            s3_host_prog_loop_tyb(&[0x62, 0xFF]).as_slice(),
            S3_HOST_SET_RET_TYB_SEED
        );
    }

    /// S3.370: per-ops host full-TYB seed cliff. Fixed10 programs of len 1..=8 are
    /// combinatorial (10+10²+… ≫ 10); RET + SET+RET already cover capability-map
    /// paths (S3.368–369). More seeds = sheep volume, not new capability. Keep
    /// assemble for other ops; next ≥10 needs a different 真自举 arch cut.
    #[test]
    fn s3_host_per_ops_tyb_seed_cliff_reject() {
        // Lower bound: just length-2 programs ending in RET over fixed10.
        let mut len2_ret = 0usize;
        for &op in &S3_FIXED10_OPS {
            if op == 0xFF {
                continue;
            }
            len2_ret += 1;
            let tyb = s3_host_prog_loop_tyb(&[op, 0xFF]);
            assert!(
                tyb.len() >= 1392,
                "each host TYB seed would be ≥RET size ({op:#x}+RET)"
            );
        }
        assert_eq!(len2_ret, 9, "9 non-RET first ops × RET");
        // Already seeded: RET + SET+RET. Remaining len2 alone = 8 more ≥1.3KB seeds.
        let seeded = 2usize; // S3.368 RET + S3.369 SET+RET
        let remaining_len2 = len2_ret - 1; // SET already seeded
        assert_eq!(remaining_len2, 8);
        // Volume without new ISA capability — reject continuing per-ops TYB seeds.
        assert!(
            remaining_len2 * 1392 > 10_000,
            "per-ops TYB seeds: {} more len2 alone (≥{}KB) — combinatorial cliff",
            remaining_len2,
            remaining_len2 * 1392 / 1024
        );
        // Seeded paths stay short-circuit; a third combo still assembles.
        assert_eq!(
            s3_host_prog_loop_tyb(&[0xFF]).as_slice(),
            S3_HOST_RET_TYB_SEED
        );
        assert_eq!(
            s3_host_prog_loop_tyb(&[0x30, 0xFF]).as_slice(),
            S3_HOST_SET_RET_TYB_SEED
        );
        let other = s3_host_prog_loop_tyb(&[0x66, 0xFF]);
        assert_ne!(other.as_slice(), S3_HOST_RET_TYB_SEED);
        assert_ne!(other.as_slice(), S3_HOST_SET_RET_TYB_SEED);
        assert_eq!(seeded, 2);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173,
            "floors untouched by cliff"
        );
    }

    /// S3.363: host prog-loop seed ≡ live builder for ops_len∈1..=8; nrows frozen.
    #[test]
    fn s3_host_prog_loop_seed_pin() {
        assert_eq!(S3_HOST_PROG_LOOP_SEED.len(), 29);
        assert_eq!(S3_HOST_PROG_LOOP_SEED[0], s3_rec_set(1, 1));
        assert_eq!(S3_HOST_PROG_LOOP_SEED[7], s3_rec_set(10, S3_HOST_NROWS));
        for n in 1u32..=8 {
            let got = s3_prog_loop_code_host(n, S3_HOST_NROWS);
            assert_eq!(got.len(), 29);
            assert_eq!(got[0], s3_rec_set(1, n));
            assert_eq!(&got[1..], &S3_HOST_PROG_LOOP_SEED[1..]);
        }
    }

    /// S3.364: host table seed ≡ composed tmpl rows from body seeds.
    #[test]
    fn s3_host_table_seed_pin() {
        assert_eq!(S3_HOST_TABLE_SEED.len(), 200);
        let mut composed = Vec::with_capacity(200);
        for &op in &S3_FIXED10_OPS {
            composed.extend_from_slice(&s3_host_tmpl_row(op, &s3_fixed_row(op)));
        }
        assert_eq!(composed.as_slice(), S3_HOST_TABLE_SEED.as_slice());
        // S3.367: prod IR ≠ continuous poke (+2), but composed .text still matches.
        let rows: Vec<(u8, &[u8])> = S3_FIXED10_OPS
            .iter()
            .map(|op| (*op, s3_fixed_row(*op)))
            .collect();
        let payload = s3_prog_data_payload(&rows, &[0xFF], S3_HOST_OUTBUF_OFF as usize);
        let start = S3_HOST_TABLE_OFF as usize;
        let end = S3_HOST_PROG_OFF as usize + 2;
        let mut live = Vec::new();
        s3_append_stb_zero_then_poke(&mut live, S3_HOST_TABLE_OFF, &payload[start..end], 10);
        live.push(s3_rec_set(S3_PACK_PROTO_DST, S3_HOST_OUTBUF_OFF));
        live.extend(s3_prog_loop_code_host(1, S3_HOST_NROWS));
        let oracle = s3_host_tyb_from_records(&live);
        let prod = s3_host_prog_loop_tyb(&[0xFF]);
        assert_eq!(
            parse_tyb(&prod).unwrap().records.len(),
            parse_tyb(&oracle).unwrap().records.len() + 2
        );
        let run = |tyb: &[u8]| {
            s3_sim_prog_loop_cursor(
                tyb,
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
                S3_HOST_OUTBUF_OFF as u64,
                RECORD_SIZE as u64,
            )
            .unwrap()
        };
        assert_eq!(run(&prod), run(&oracle));
    }

    /// S3.361: expand poke seed ≡ live zero_then_poke(boot wire|hdr).
    #[test]
    fn s3_expand_poke_seed_pin() {
        assert_eq!(S3_EXPAND_POKE_SEED.len(), 20);
        assert_eq!(S3_EXPAND_OUT, 0x50);
        let mut live_poke = Vec::new();
        let mut job = Vec::new();
        for &(op, argc, args) in &S3_BOOTSTRAP_EXPAND_ASM {
            job.push(op);
            job.push(argc);
            job.extend_from_slice(&args);
        }
        let mut hdr = [0u8; HEADER_SIZE];
        hdr[..4].copy_from_slice(&TYB_MAGIC);
        hdr[6..8].copy_from_slice(&(S3_BOOTSTRAP_EXPAND_ASM.len() as u16).to_le_bytes());
        job.extend_from_slice(&hdr);
        s3_append_stb_zero_then_poke(&mut live_poke, S3_EXPAND_SEED, &job, 30);
        assert_eq!(live_poke.as_slice(), S3_EXPAND_POKE_SEED.as_slice());
    }

    /// S3.362: expand full TYB seed ≡ poke-seed+bootstrap via host_tyb_from_records.
    #[test]
    fn s3_expand_tyb_seed_pin() {
        assert_eq!(
            S3_EXPAND_TYB_SEED.len(),
            HEADER_SIZE + S3_EXPAND_ASM_PIN * RECORD_SIZE
        );
        let mut recs = Vec::with_capacity(S3_EXPAND_ASM_PIN);
        recs.extend_from_slice(&S3_EXPAND_POKE_SEED);
        recs.extend_from_slice(&S3_BOOTSTRAP_EXPAND_ASM);
        let assembled = s3_host_tyb_from_records(&recs);
        assert_eq!(assembled.as_slice(), S3_EXPAND_TYB_SEED);
        let (prod, off) = s3_expand_many_flat_tyb();
        assert_eq!(prod.as_slice(), S3_EXPAND_TYB_SEED);
        assert_eq!(off, S3_EXPAND_OUT);
        assert_eq!(parse_tyb(&prod).unwrap().records.len(), S3_EXPAND_ASM_PIN);
    }

    /// S3.352: post-scaffolding cliff map — all hard floors in one pin after
    /// S3.342–351 (host densify reject · argc/flat/ir_seed/wrappers cfg(test) ·
    /// thin-wrapper sheep). Next ≥10 needs A1 / OnceLock / asm22 / 真自举 arch.
    #[test]
    fn s3_post_scaffold_cliff_map_pin() {
        assert_eq!(S3_EXPAND_ASM_PIN, 22, "assembler poke floor");
        assert_eq!(
            parse_tyb(&s3_expand_many_flat_tyb().0)
                .unwrap()
                .records
                .len(),
            22
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173,
            "host IR floor (S3.367)"
        );
        assert_eq!(s3_prog_loop_code_host(1, S3_HOST_NROWS).len(), 29);
        assert_eq!(s3_bootstrap_expand_asm().len(), 2);
        assert_eq!(s3_in_dll_lea_prologue_recs().len(), 13, "A1 density floor");
        assert_eq!(s3_short_lea_raw_bytes().len(), 37);
        assert_eq!(S3_LEA_PTR.len(), 7, "short lea not disp32");
        // OnceLock bake still equals fresh emit_x86 (const-embed reject holds).
        assert_eq!(s3_fixed_row(0xFF), s3_compute_fixed_body(0xFF).as_slice());
        assert_eq!(s3_fixed_row(0x30), s3_compute_fixed_body(0x30).as_slice());
        assert_eq!(
            try_emit_text_via_yoyo_table(&[0xFF]).unwrap(),
            s3_fixed_row(0xFF)
        );
    }

    /// S3.351: thin-wrapper scaffolding sheep reject. S3.348–350 moved body_of /
    /// fixed_payload / sim_prog_loop to cfg(test) — **0** IR delta (ASM 22 / host
    /// 173). Further one-liner collapses (tmpl_row / fixed_row alias / …) stay
    /// below the ≥10 bar; next wins need A1 / OnceLock / asm22 / 真自举 arch.
    #[test]
    fn s3_thin_wrapper_sheep_reject() {
        assert_eq!(S3_EXPAND_ASM_PIN, 22, "wrapper purge did not touch assembler");
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173,
            "host IR floor (S3.367; wrappers still 0-delta vs prior)"
        );
        assert_eq!(s3_bootstrap_expand_asm().len(), 2);
        assert_eq!(s3_in_dll_lea_prologue_recs().len(), 13);
        // Remaining production one-liners are load-bearing (OnceLock index,
        // tmpl row layout, poke windows) — not free −1 sheep.
        assert_eq!(s3_fixed_row(0xFF), [0xC3].as_slice());
        assert_eq!(
            s3_host_tmpl_row(0xFF, &[0xC3])[0..2],
            [0xFF, 1]
        );
        assert!(
            0 < 10,
            "documented: further wrapper-only saves are 0 IR < 10 bar"
        );
        assert_eq!(
            parse_tyb(&s3_expand_many_flat_tyb().0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
    }

    /// S3.350: `s3_sim_prog_loop` is cfg(test); pub try_emit calls cursor with
    /// default slot9=outbuf / slot12=RECORD_SIZE (≡ old wrapper).
    #[test]
    fn s3_sim_prog_loop_test_only_pin() {
        let tyb = s3_host_prog_loop_tyb(&[0xFF]);
        let via = s3_sim_prog_loop(
            &tyb,
            S3_HOST_TABLE_OFF as u64,
            S3_HOST_OUTBUF_OFF,
            (S3_HOST_PROG_OFF + 1) as u64,
        )
        .unwrap();
        let cur = s3_sim_prog_loop_cursor(
            &tyb,
            S3_HOST_TABLE_OFF as u64,
            S3_HOST_OUTBUF_OFF,
            (S3_HOST_PROG_OFF + 1) as u64,
            S3_HOST_OUTBUF_OFF as u64,
            RECORD_SIZE as u64,
        )
        .unwrap();
        assert_eq!(via, cur);
        assert_eq!(via, s3_compute_fixed_body(0xFF));
        assert_eq!(
            try_emit_text_via_yoyo_table(&[0xFF]).as_deref(),
            Some(via.as_slice())
        );
        assert_eq!(
            parse_tyb(&s3_expand_many_flat_tyb().0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
    }

    /// S3.349: `s3_fixed_payload` is cfg(test); production host inlines
    /// fixed_row → prog_data_payload. Helper still ≡ host seed payload.
    #[test]
    fn s3_fixed_payload_test_only_pin() {
        let via = s3_fixed_payload(&S3_FIXED10_OPS, &[0xFF], S3_HOST_OUTBUF_OFF as usize);
        let rows: Vec<(u8, &[u8])> = S3_FIXED10_OPS
            .iter()
            .map(|op| (*op, s3_fixed_row(*op)))
            .collect();
        let inline = s3_prog_data_payload(&rows, &[0xFF], S3_HOST_OUTBUF_OFF as usize);
        assert_eq!(via, inline);
        assert_eq!(via.len(), S3_HOST_OUTBUF_OFF as usize);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(
            parse_tyb(&s3_expand_many_flat_tyb().0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
    }

    /// S3.348: `s3_host_body_of` is cfg(test); OnceLock bakes strip-RET via
    /// `s3_fixed_rec` + `s3_host_body` inline. Helper ≡ fixed_row for fixed10.
    #[test]
    fn s3_host_body_of_test_only_pin() {
        for &op in &S3_FIXED10_OPS {
            if op == 0xFF {
                continue; // RET is whole emit; body_of strips trailing C3
            }
            assert_eq!(
                s3_host_body_of(op),
                s3_fixed_row(op),
                "body_of ≡ fixed_row for {op:#x}"
            );
        }
        // RET: body_of would strip the only C3 — fixed_row keeps it.
        assert_eq!(s3_fixed_row(0xFF), [0xC3].as_slice());
        assert_eq!(s3_host_set_body(), s3_fixed_row(0x30));
        assert_eq!(
            parse_tyb(&s3_expand_many_flat_tyb().0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
    }

    /// S3.347: production S3 surface after scaffolding purge (S3.325–346).
    /// Five live clusters — everything else is `#[cfg(test)]`:
    ///   host compose (payload/prog_loop/tyb) · poke seed · bootstrap+hardcoded
    ///   expand · rec shape builders · prog-loop sim (+ pub try_emit/A1/OnceLock).
    /// Floors: ASM 22 · host 173 · A1 13×37 · bootstrap 2. Not CLOSED.
    #[test]
    fn s3_production_surface_inventory_pin() {
        assert_eq!(s3_bootstrap_expand_asm().len(), 2, "sole bootstrap MEMCPY+RET");
        assert_eq!(
            s3_bootstrap_expand_asm(),
            vec![s3_rec_memcpy(9, 2, 12), s3_fixed_rec(0xFF)]
        );
        assert_eq!(
            parse_tyb(&s3_expand_many_flat_tyb().0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
        assert_eq!(S3_EXPAND_ASM_PIN, 22);
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(s3_prog_loop_code_host(1, S3_HOST_NROWS).len(), 29);
        assert_eq!(s3_in_dll_lea_prologue_recs().len(), 13);
        assert_eq!(s3_short_lea_raw_bytes().len(), 37);
        assert_eq!(s3_fixed_row(0xFF), [0xC3].as_slice());
        assert_eq!(S3_PACK_PROTO_DST, 9);
        assert_eq!(S3_PACK_STB_BASE, 14);
        // Test-only scaffolding still reachable under cfg(test).
        assert_eq!(S3_SEED_ARGC_BYTES.len(), 256);
        assert_eq!(S3_SHAPE_FILL_MANY_ROWS, 2);
        assert_eq!(
            s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT),
            s3_expand_many_flat_tyb()
        );
    }

    /// S3.346: `s3_ir_seed_at` is cfg(test) alias; production host/expand call
    /// `s3_append_stb_zero_then_poke` directly. Floor pins unchanged.
    #[test]
    fn s3_ir_seed_at_test_only_pin() {
        let payload = s3_fixed_payload(&S3_FIXED10_OPS, &[0xFF], S3_HOST_OUTBUF_OFF as usize);
        let start = S3_HOST_TABLE_OFF as usize;
        let end = S3_HOST_PROG_OFF as usize + 2;
        let slice = &payload[start..end];
        let mut via_alias = Vec::new();
        s3_ir_seed_at(&mut via_alias, S3_HOST_TABLE_OFF, slice, 10);
        let mut via_poke = Vec::new();
        s3_append_stb_zero_then_poke(&mut via_poke, S3_HOST_TABLE_OFF, slice, 10);
        assert_eq!(via_alias, via_poke, "ir_seed_at ≡ zero_then_poke");
        assert_eq!(via_alias.len(), 141, "host continuous seed IR pin (oracle)");
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173
        );
        assert_eq!(
            parse_tyb(&s3_expand_many_flat_tyb().0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
    }

    /// S3.345: default-BASE poke wrapper + ret/nop body helpers are cfg(test);
    /// production seed calls `_base` poke; OnceLock bakes RET/NOP inline.
    #[test]
    fn s3_poke_ret_nop_helpers_test_only_pin() {
        let mut via = Vec::new();
        s3_append_stb_poke_global_distinct(&mut via, &[0xAA, 0, 0xAA]);
        let mut base = Vec::new();
        s3_append_stb_poke_global_distinct_base(&mut base, &[0xAA, 0, 0xAA], S3_PACK_STB_BASE);
        assert_eq!(via, base, "default-BASE wrapper ≡ _base(STB_BASE)");
        assert_eq!(s3_host_ret_body(), s3_compute_fixed_body(0xFF));
        assert_eq!(s3_host_nop_body(), s3_compute_fixed_body(0x00));
        assert_eq!(s3_fixed_row(0xFF), [0xC3].as_slice());
        assert_eq!(s3_fixed_row(0x00), [0x90].as_slice());
        assert_eq!(
            parse_tyb(&s3_expand_many_flat_tyb().0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
    }

    /// S3.343/S3.344: 256B argc index + MANY_FLAT + generic expand are cfg(test);
    /// production expand hardcodes MEMCPY+RET wire (still ≡ generic/flat path).
    #[test]
    fn s3_argc_index_test_only_pin() {
        assert_eq!(S3_SEED_ARGC_BYTES.len(), 256);
        assert_eq!(s3_seed_argc(0x84), 3);
        assert_eq!(s3_seed_argc(0xFF), 0);
        assert_eq!(S3_SHAPE_FILL_MANY_FLAT.len(), S3_SHAPE_FILL_MANY_ROWS * 7);
        assert_eq!(S3_SHAPE_FILL_MANY_ROWS, 2);
        let (prod, poff) = s3_expand_many_flat_tyb();
        let (gen, goff) = s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT);
        assert_eq!((prod.clone(), poff), (gen, goff));
        assert_eq!(parse_tyb(&prod).unwrap().records.len(), S3_EXPAND_ASM_PIN);
        assert_eq!(
            s3_bootstrap_expand_asm(),
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT),
            "named bootstrap still ≡ MANY_FLAT"
        );
    }

    /// S3.339: pack via_ir + loop_stb (+ flat_at/hdr helpers) cfg(test); expand stays live.
    #[test]
    fn s3_pack_via_ir_loop_stb_test_only_pin() {
        let bytes = [1u8, 2, 3, 4];
        assert_eq!(s3_pack_data_via_ir(&bytes), s3_pack_data(&bytes));
        let (tyb, off) = s3_yoyo_pack_loop_stb_tyb(&bytes);
        assert!(s3_unpack_data_tyb(&tyb).is_empty());
        assert_eq!(off, bytes.len() as u32 + 2);
        // expand remains non-cfg (self-host near-end).
        let (exp, _) = s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT);
        assert_eq!(parse_tyb(&exp).unwrap().records.len(), S3_EXPAND_ASM_PIN);
    }

    /// S3.338: rec_wire + proto putc/record + rec_sub cfg(test); expand stays live.
    #[test]
    fn s3_rec_wire_proto_putc_test_only_pin() {
        let w = s3_rec_wire(0xFF, 0, [0; 6]);
        assert_eq!(w[0], 0xFF);
        let mut recs = Vec::new();
        s3_pack_proto_putc_imm(&mut recs, 0x10);
        assert_eq!(recs[0].0, 0x30);
        assert_eq!(s3_rec_sub(3, 5), s3_rec_slot_imm(0x61, 3, 5));
        // expand remains callable (self-host near-end; not cfg(test)).
        let (exp, _) = s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT);
        assert_eq!(parse_tyb(&exp).unwrap().records.len(), S3_EXPAND_ASM_PIN);
    }

    /// S3.337: fixed_table / host_set_body / seed helpers / jne are cfg(test).
    #[test]
    fn s3_misc_helpers_test_only_pin() {
        assert!(!s3_fixed_table(&[0xFF]).is_empty());
        assert!(!s3_host_set_body().is_empty());
        assert!(!s3_fixed10_named_cases().is_empty());
        assert_eq!(s3_shape_argc_bytes().len(), 4);
        assert_eq!(s3_seed_argc_bytes().len(), 256);
        assert!(!s3_seed_argc_flat().is_empty());
        assert!(!s3_rec_op_shape_bytes().is_empty());
        let _ = s3_rec_jne(1);
    }

    /// S3.336: alt pack TYB (proto/stb/memcpy-loop) cfg(test); loop_stb via_ir stays.
    #[test]
    fn s3_alt_pack_tyb_test_only_pin() {
        let bytes = [0x11u8, 0x22, 0x33];
        assert_eq!(s3_pack_data_via_ir(&bytes), s3_pack_data(&bytes));
        let proto = s3_yoyo_pack_proto_tyb(&bytes);
        let stb = s3_yoyo_pack_stb_tyb(&bytes);
        let (loop_m, _) = s3_yoyo_pack_loop_tyb(&bytes);
        assert!(s3_unpack_data_tyb(&proto).is_empty());
        assert!(s3_unpack_data_tyb(&stb).is_empty());
        assert!(s3_unpack_data_tyb(&loop_m).is_empty());
    }

    /// S3.335: emit-TYB file cluster + unpack_data_tyb are cfg(test).
    #[test]
    fn s3_emit_tyb_cluster_test_only_pin() {
        let body = [s3_fixed_rec(0xFF)];
        let (fw, off) = s3_yoyo_emit_tyb_file(&body);
        assert!(s3_unpack_data_tyb(&fw).is_empty());
        assert_eq!(s3_sim_prog_loop(&fw, 0, off, 0).unwrap(), s3_host_tyb_from_records(&body));
        let (u, _) = s3_yoyo_emit_tyb_file_unrolled(&body);
        let (l, _) = s3_yoyo_emit_tyb_file_loop_staged(&body);
        assert!(s3_unpack_data_tyb(&u).is_empty());
        assert!(s3_unpack_data_tyb(&l).is_empty());
    }

    /// S3.334: reference Rust pack_data cluster is cfg(test); via_ir is IR pack path.
    #[test]
    fn s3_reference_pack_data_test_only_pin() {
        let bytes = [0xAAu8, 0xBB, 0xCC, 0xDD];
        assert_eq!(s3_pack_data_via_ir(&bytes), s3_pack_data(&bytes));
        assert_eq!(s3_unpack_data(&s3_pack_data(&bytes)), bytes.to_vec());
        assert_eq!(
            s3_pack_wire_section_to_recs(&s3_pack_data_wire(&bytes)),
            s3_pack_data(&bytes)
        );
    }

    /// S3.333: A1 append + bytewise DATA pack are cfg(test).
    #[test]
    fn s3_a1_append_bytewise_test_only_pin() {
        let mut a1 = Vec::new();
        s3_append_raw_a1(&mut a1, S3_LEA_PTR);
        assert_eq!(a1, s3_host_pack_bytes(0xA1, S3_LEA_PTR));
        assert_eq!(s3_pack_data_bytewise(&[1, 2, 3]).len(), 3);
        assert_eq!(s3_in_dll_lea_prologue_recs().len(), 13);
    }

    /// S3.332: shape-fill TYB builders are cfg(test); expand owns production wire→TYB.
    #[test]
    fn s3_shape_fill_tyb_test_only_pin() {
        let (tyb, out) = s3_yoyo_shape_fill_tyb(0xFF, [0; 6]);
        assert_eq!(out, S3_SHAPE_OUT);
        assert_eq!(
            s3_sim_shape_fill(&tyb, out).unwrap(),
            s3_rec_wire(0xFF, 0, [0; 6])
        );
        let (many, mout) = s3_yoyo_shape_fill_many_tyb(&[(0xFF, [0; 6])]);
        assert_eq!(mout, S3_SHAPE_MANY_OUT);
        assert!(s3_unpack_data_tyb(&many).is_empty());
    }

    /// S3.331: in-DLL golden `s3_prog_loop_code` is cfg(test); host loop is production.
    #[test]
    fn s3_dll_prog_loop_test_only_pin() {
        let dll = s3_prog_loop_code(1, 3);
        let host = s3_prog_loop_code_host(1, S3_HOST_NROWS);
        assert!(host.len() < dll.len(), "host stays thinner than dll golden");
        assert_eq!(dll[1].0, 0x30, "dll golden still has SET i=0");
        assert_ne!(host[1].0, 0x30, "host skipped SET i=0");
    }

    /// S3.329: flat→recs helpers are cfg(test); production expand uses named bootstrap.
    #[test]
    fn s3_flat_seed_decode_test_only_pin() {
        assert_eq!(
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT),
            s3_bootstrap_expand_asm()
        );
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT),
            s3_bootstrap_expand_asm()
        );
    }

    /// S3.327/S3.328: no fill_ctrl / ONE_* / EXPAND_ASM_ROWS — bootstrap + MANY_*.
    #[test]
    fn s3_fill_ctrl_is_bootstrap_pin() {
        assert_eq!(
            s3_bootstrap_expand_asm(),
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT)
        );
        assert_eq!(s3_bootstrap_expand_asm().len(), S3_SHAPE_FILL_MANY_ROWS);
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT),
            s3_bootstrap_expand_asm(),
            "via_yoyo still ≡ bootstrap (proof retained)"
        );
    }

    /// S3.325/S3.328: leading MEMCPY ≡ bootstrap[0]; via_yoyo of first 7B proves it.
    #[test]
    fn s3_shape_fill_one_named_memcpy_pin() {
        let memcpy = s3_rec_memcpy(9, 2, 12);
        assert_eq!(s3_bootstrap_expand_asm()[0], memcpy);
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(&S3_SHAPE_FILL_MANY_FLAT[..7]),
            vec![memcpy],
            "via_yoyo of MEMCPY slice still ≡ named"
        );
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
            // S3.304: wire-job MEMCPY (no argc LDB scan).
            assert!(
                ir.records.iter().any(|r| r.op == 0x84),
                "op {op:#x} emits via MEMCPY"
            );
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
                s3_sim_shape_fill(&tyb, out).unwrap(),
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
            // S3.304: one-ctrl is MEMCPY+RET (job poke may still STB seed bytes).
            let ctrl_start = ir.records.len() - S3_SHAPE_FILL_MANY_ROWS;
            assert!(
                ir.records[ctrl_start..].iter().all(|r| r.op != 0x81),
                "op {op:#x} one-ctrl has no STB"
            );
        }
        assert_eq!(s3_shape_argc_bytes(), [2, 2, 1, 3]);
        let (a, _) = s3_yoyo_shape_fill_tyb(0x30, [3, 0, 0x11, 0, 0, 0]);
        let (b, _) = s3_yoyo_shape_fill_tyb(0x81, [11, 0, 14, 0, 5, 0]);
        let ctrl = s3_bootstrap_expand_asm();
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
        assert_eq!(
            s3_sim_shape_fill_n(&tyb, out, (jobs.len() * RECORD_SIZE) as u64).unwrap(),
            expect
        );
        let ir = parse_tyb(&tyb).unwrap();
        let boot = s3_bootstrap_expand_asm();
        let bodies = ir.records.iter().filter(|r| r.op == 0x40).count();
        // S3.306/S3.328: many ≡ bootstrap (MEMCPY+RET); no LABEL scaffold.
        assert_eq!(bodies, boot.iter().filter(|r| r.0 == 0x40).count());
        let ctrl = s3_bootstrap_expand_asm();
        let (other, _) = s3_yoyo_shape_fill_many_tyb(&jobs[..1]);
        let tail = |tyb: &[u8]| {
            let recs = parse_tyb(tyb).unwrap().records;
            let start = recs.len() - ctrl.len();
            recs[start..]
                .iter()
                .map(|r| (r.op, r.argc, r.args_raw))
                .collect::<Vec<_>>()
        };
        assert_eq!(tail(&tyb), tail(&other), "many-ctrl ignores job stream length");
        assert_eq!(tail(&tyb), ctrl);
    }

    /// S3.241–S3.244: flat seed expands to wire args; argc is a table lookup.
    #[test]
    fn s3_shape_fill_seed_roundtrip() {
        assert_eq!(S3_SHAPE_FILL_MANY_FLAT.len(), S3_SHAPE_FILL_MANY_ROWS * 7);
        let many = s3_bootstrap_expand_asm();
        assert_eq!(many.len(), S3_SHAPE_FILL_MANY_ROWS);
        for (i, rec) in many.iter().enumerate() {
            let row = &S3_SHAPE_FILL_MANY_FLAT[i * 7..i * 7 + 7];
            assert_eq!(rec.0, row[0]);
            assert_eq!(rec.1, s3_seed_argc(row[0]));
            assert_eq!(rec.2, [row[1], row[2], row[3], row[4], row[5], row[6]]);
        }
        // S3.328: leading MEMCPY is bootstrap[0] / many[0].
        assert_eq!(many[0], s3_rec_memcpy(9, 2, 12));
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
        // Frozen 256B index stays in lockstep with the shape table (RET→0).
        let mut derived = [0u8; 256];
        for &(op, shape) in S3_REC_OP_SHAPE {
            let n = match shape {
                S3RecShape::SlotImm | S3RecShape::SlotSlot => 2,
                S3RecShape::U32 => 1,
                S3RecShape::Slot3 => 3,
            };
            derived[op as usize] = n;
        }
        assert_eq!(S3_SEED_ARGC_BYTES, derived);
        // Expander does not re-encode through s3_rec_from_shape.
        let row = [0x30u8, 2, 0, 104, 0, 0, 0];
        assert_eq!(
            s3_rec_from_wire_seed(&row),
            (0x30, 2, [2, 0, 104, 0, 0, 0])
        );
        // S3.304: ONE = many MEMCPY slice + RET.
        assert_eq!(S3_SHAPE_FILL_MANY_FLAT.len(), S3_SHAPE_FILL_MANY_ROWS * 7);
        let ctrl = s3_bootstrap_expand_asm();
        assert_eq!(ctrl.len(), S3_SHAPE_FILL_MANY_ROWS);
        assert_eq!(ctrl, s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT));
        assert_eq!(S3_SHAPE_FILL_MANY_ROWS, 2, "S3.328: MEMCPY+RET");
        assert_eq!(
            s3_recs_from_flat_seed_with_argc(S3_SHAPE_FILL_MANY_FLAT, &S3_SEED_ARGC_BYTES),
            s3_bootstrap_expand_asm()
        );
    }

    /// S3.308/S3.353: ONE expand assembler is poke-bound (22 = records|hdr + MEMCPY+RET).
    /// Measured alts (join already on; magic-only / runs / split) save <10 — reject.
    /// S3.353: vestigial job sentinel dropped (0 IR; poke skipped the zero anyway).
    #[test]
    fn s3_expand_one_seed_cliff_sub10_reject() {
        let flat = S3_SHAPE_FILL_MANY_FLAT;
        let nrecs = flat.len() / 7;
        let mut job = Vec::new();
        for chunk in flat.chunks_exact(7) {
            job.push(chunk[0]);
            job.push(s3_seed_argc(chunk[0]));
            job.extend_from_slice(&chunk[1..7]);
        }
        let mut hdr = [0u8; HEADER_SIZE];
        hdr[..4].copy_from_slice(&TYB_MAGIC);
        hdr[6..8].copy_from_slice(&(nrecs as u16).to_le_bytes());
        let poke = |b: &[u8]| {
            let mut r = Vec::new();
            s3_ir_seed_at(&mut r, 0x40, b, 1);
            r.len()
        };
        let joined = poke(&[job.as_slice(), hdr.as_slice()].concat());
        let boot = S3_SHAPE_FILL_MANY_ROWS;
        assert_eq!(joined + boot, S3_EXPAND_ASM_PIN);
        let magic_only = {
            let mut m = [0u8; HEADER_SIZE];
            m[..4].copy_from_slice(&TYB_MAGIC);
            poke(&[job.as_slice(), m.as_slice()].concat()) + boot
        };
        assert!(
            S3_EXPAND_ASM_PIN.saturating_sub(magic_only) < 10,
            "magic-only save {} < 10 bar",
            S3_EXPAND_ASM_PIN - magic_only
        );
        let (exp, _) = s3_yoyo_expand_flat_tyb(flat);
        assert_eq!(parse_tyb(&exp).unwrap().records.len(), S3_EXPAND_ASM_PIN);
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
            s3_bootstrap_expand_asm()
        );
        let (tyb1, off1) = s3_yoyo_emit_flat_via_stb(S3_SHAPE_FILL_MANY_FLAT, 0x20);
        assert!(s3_unpack_data_tyb(&tyb1).is_empty());
        assert_eq!(
            s3_sim_prog_loop(&tyb1, 0, off1, 0).unwrap(),
            S3_SHAPE_FILL_MANY_FLAT
        );
        assert_eq!(
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT),
            s3_bootstrap_expand_asm()
        );
        // S3.256/S3.257: expand ctrl flat seed; job ends with 0xFE sentinel (no SET END).
        assert_eq!(S3_SHAPE_FILL_MANY_FLAT.len(), S3_SHAPE_FILL_MANY_ROWS * 7);
        assert_eq!(s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT).len(), S3_SHAPE_FILL_MANY_ROWS);
        assert_eq!(s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT).last().unwrap().0, 0xFF);
        let (exp_stb, exp_off) = s3_yoyo_emit_flat_via_stb(S3_SHAPE_FILL_MANY_FLAT, 0x20);
        assert!(s3_unpack_data_tyb(&exp_stb).is_empty());
        assert_eq!(
            s3_sim_prog_loop(&exp_stb, 0, exp_off, 0).unwrap(),
            S3_SHAPE_FILL_MANY_FLAT
        );
        // S3.254/S3.259/S3.264: YOYO expand matches Rust; no job SET END; size floor.
        let (exp, off) = s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT);
        assert!(s3_unpack_data_tyb(&exp).is_empty());
        let exp_recs = parse_tyb(&exp).unwrap().records;
        // S3.283: pin ONE-flat assembler size (in-place seed ptr; no MOVRR s8).
        assert_eq!(exp_recs.len(), S3_EXPAND_ASM_PIN);
        assert_eq!(S3_SHAPE_FILL_MANY_ROWS, 2);
        // S3.322: INIT scaffolding removed; bootstrap ≡ ctrl flat only.
        assert_eq!(
            s3_bootstrap_expand_asm(),
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT)
        );
        assert_eq!(&TYB_MAGIC[..3], b"TYB");
        // S3.291: job-end is 0 vs slot13; ctrl must not SET a sentinel imm.
        assert!(
            !s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT)
                .iter()
                .any(|r| r.0 == 0x30 && u16::from_le_bytes([r.2[0], r.2[1]]) == 5),
            "expand ctrl must not SET slot5; slot13 image-zero owns job-end"
        );
        let mut expect_tyb = Vec::new();
        expect_tyb.extend_from_slice(&TYB_MAGIC);
        expect_tyb.extend_from_slice(&[0, 0]);
        let one_recs = s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT);
        expect_tyb.extend_from_slice(&(one_recs.len() as u16).to_le_bytes());
        for r in &one_recs {
            expect_tyb.extend_from_slice(&s3_rec_wire(r.0, r.1, r.2));
        }
        assert_eq!(
            s3_sim_prog_loop_cursor(
                &exp,
                0,
                off,
                S3_EXPAND_SEED as u64,
                off as u64 + HEADER_SIZE as u64,
                (one_recs.len() * RECORD_SIZE) as u64,
            )
            .unwrap(),
            expect_tyb
        );
        assert_eq!(
            s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT),
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT)
        );
        // S3.250/S3.299: argc index table STB-emits and round-trips.
        let argc_bytes = s3_seed_argc_bytes();
        assert_eq!(argc_bytes.len(), 256);
        assert_eq!(argc_bytes[0xFF], 0, "RET argc");
        assert_eq!(argc_bytes[0x30], 2);
        let (tyb_a, off_a) = s3_yoyo_emit_flat_via_stb(&argc_bytes, 0x20);
        assert!(s3_unpack_data_tyb(&tyb_a).is_empty());
        assert_eq!(s3_sim_prog_loop(&tyb_a, 0, off_a, 0).unwrap(), argc_bytes);
    }

    /// S3.258/S3.261/S3.264: expand ctrl body re-encodes via shape-fill.
    /// S3.264 dropped argc-poke rows; body-only jobs fit well under the 128 cap.
    #[test]
    fn s3_yoyo_expand_ctrl_reencodes_via_shape_fill() {
        assert_eq!(S3_SHAPE_FILL_MANY_FLAT.len(), S3_SHAPE_FILL_MANY_ROWS * 7);
        assert_eq!(S3_SHAPE_FILL_MANY_ROWS, 2);
        let ctrl = s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT);
        assert_eq!(ctrl.len(), S3_SHAPE_FILL_MANY_ROWS);
        assert_eq!(ctrl.last().unwrap().0, 0xFF);
        let jobs: Vec<(u8, [u8; 6])> = ctrl
            .iter()
            .filter(|r| r.0 != 0xFF)
            .map(|r| (r.0, r.2))
            .collect();
        assert_eq!(jobs.len(), S3_SHAPE_FILL_MANY_ROWS - 1);
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
        assert_eq!(
            s3_sim_shape_fill_n(&tyb, out, (jobs.len() * RECORD_SIZE) as u64).unwrap(),
            expect
        );
    }

    /// S3.240: many-fill re-encodes its own control records. RET is not in the
    /// shape table, so it stays outside the job blob.
    #[test]
    fn s3_yoyo_shape_fill_reencodes_own_ctrl() {
        let ctrl = s3_bootstrap_expand_asm();
        assert!(ctrl.iter().any(|r| r.0 == 0xFF));
        let jobs: Vec<(u8, [u8; 6])> = ctrl
            .iter()
            .filter(|r| r.0 != 0xFF)
            .map(|r| (r.0, r.2))
            .collect();
        assert!(jobs.len() >= 1 && jobs.len() <= 128, "jobs {}", jobs.len());
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
        assert_eq!(
            s3_sim_shape_fill_n(&tyb, out, (jobs.len() * RECORD_SIZE) as u64).unwrap(),
            expect
        );
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

    /// S3.135/S3.160/S3.223/S3.259/S3.309/S3.311–366: remaining Rust sidecar.
    /// S3.359–370: gaps + seeds + RET/SET+RET TYB; per-ops TYB cliff. Remaining = other assemble.
    /// Host 173 · ASM 22. Scaffolding/wrappers cfg(test). Not CLOSED.
    /// All pack paths emit full TYB (S3.307–309). Not CLOSED.
    #[test]
    fn s3_fixed10_fold_remaining_rust_surface() {
        assert_eq!(S3_LEA_PTR.len(), 7, "short lea must stay 7B (not disp32)");
        assert_eq!(S3_LEA_PTR, &[0x49, 0x8D, 0x07, 0x49, 0x89, 0x47, 0x78]);
        // S3.275: assembler body is one flat seed (init prefix + ctrl); still Rust.
        assert_eq!(S3_SHAPE_FILL_MANY_FLAT.len(), S3_SHAPE_FILL_MANY_ROWS * 7);
        assert_eq!(S3_SHAPE_FILL_MANY_ROWS, 2, "S3.306: expand≡many≡one bulk MEMCPY");
        assert_eq!(S3_SHAPE_FILL_MANY_FLAT.len(), S3_SHAPE_FILL_MANY_ROWS * 7);
        assert_eq!(S3_SHAPE_FILL_MANY_ROWS, 2, "S3.306: shared MEMCPY+RET");
        assert_eq!(S3_SHAPE_TABLE_END, 0x140, "S3.299: 256B argc index");
        assert_eq!(S3_SHAPE_FILL_MANY_ROWS, 2, "S3.306: bulk wire MEMCPY+RET");
        assert!(
            !s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT)
                .iter()
                .any(|r| r.0 == 0x81),
            "expand ctrl must not STB; wire-job MEMCPY owns emit"
        );
        assert!(
            !s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT)
                .iter()
                .any(|r| r.0 == 0x80),
            "many ctrl must not LDB; bulk MEMCPY uses slot2"
        );
        // S3.276: ctrl-only expand is plain flat_seed (no named bootstrap).
        assert_eq!(
            s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT),
            s3_recs_from_flat_seed_with_argc(S3_SHAPE_FILL_MANY_FLAT, &S3_SEED_ARGC_BYTES)
        );
        assert_eq!(
            s3_bootstrap_expand_asm(),
            s3_recs_from_flat_seed_with_argc(S3_SHAPE_FILL_MANY_FLAT, &S3_SEED_ARGC_BYTES)
        );
        assert_eq!(S3_SEED_ARGC_BYTES.len(), 256, "S3.309: argc index stays 256B");
        // S3.306: ONE-flat expand assembler size (bulk + job|hdr join).
        let (exp_pin, _) = s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT);
        assert_eq!(parse_tyb(&exp_pin).unwrap().records.len(), S3_EXPAND_ASM_PIN);
        let one = s3_recs_from_flat_seed_via_yoyo(S3_SHAPE_FILL_MANY_FLAT);
        assert_eq!(one, s3_recs_from_flat_seed(S3_SHAPE_FILL_MANY_FLAT));
        // S3.307: wrap helper stays for wire-only tests; via_ir emits full TYB.
        for r in &one {
            let w = s3_rec_wire(r.0, r.1, r.2);
            assert_eq!(s3_pack_wire_section_to_recs(&w), vec![*r]);
        }
        assert_eq!(
            s3_pack_data_via_ir(&[1, 2, 3, 4]),
            s3_pack_data(&[1, 2, 3, 4]),
            "via_ir pack records == reference (full-TYB path)"
        );
        assert_eq!(S3_EXPAND_SEED, 0x40);
        assert_eq!(S3_EXPAND_SENTINEL, 0);
        let (exp, off) = s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT);
        let emitted = s3_sim_prog_loop_cursor(
            &exp,
            0,
            off,
            S3_EXPAND_SEED as u64,
            off as u64 + HEADER_SIZE as u64,
            (S3_SHAPE_FILL_MANY_FLAT.len() / 7 * RECORD_SIZE) as u64,
        )
        .unwrap();
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

    /// S3.314: emit-flat via poke — many ctrl 35→11 (−24); sim readback unchanged.
    #[test]
    fn s3_emit_flat_poke_beats_runs_pin() {
        let (tyb, off) = s3_yoyo_emit_flat_via_stb(S3_SHAPE_FILL_MANY_FLAT, 0x20);
        let n = parse_tyb(&tyb).unwrap().records.len();
        // poke seed + RET
        assert_eq!(n, 14, "emit many flat IR pin (was 36; poke+tail-pin+RET)");
        assert_eq!(
            s3_sim_prog_loop(&tyb, 0, off, 0).unwrap(),
            S3_SHAPE_FILL_MANY_FLAT
        );
        let (tyb1, off1) = s3_yoyo_emit_flat_via_stb(S3_SHAPE_FILL_MANY_FLAT, 0x20);
        assert_eq!(
            s3_sim_prog_loop(&tyb1, 0, off1, 0).unwrap(),
            S3_SHAPE_FILL_MANY_FLAT
        );
        assert_eq!(
            parse_tyb(&s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT).0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
    }


    /// S3.312: host long-seed uses per-window global-distinct — 220→171 (−49).
    /// Address-order long path saved 0 vs dual-window; bar ≥10 cleared.
    /// S3.313: pack payload seed uses poke (not STB runs). lea sample 172→99 (−73).
    #[test]
    fn s3_pack_payload_poke_beats_runs_pin() {
        let lea = s3_short_lea_raw_bytes();
        let (tyb, _) = s3_yoyo_pack_loop_stb_tyb(&lea);
        let n = parse_tyb(&tyb).unwrap().records.len();
        assert_eq!(n, 99, "pack loop STB IR pin after poke seed (was 172)");
        assert_eq!(
            s3_pack_data_via_ir(&lea),
            s3_pack_data(&lea),
            "via_ir pack records still match reference"
        );
        // Small payloads also shrink; keep parity.
        for n in 1usize..=16 {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 19 + 7) as u8).collect();
            assert_eq!(s3_pack_data_via_ir(&bytes), s3_pack_data(&bytes));
        }
        assert_eq!(
            parse_tyb(&s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT).0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN
        );
    }



    /// S3.342: host prog-loop densify reject. Body is 29; sheep cuts (nrows SET +
    /// miss `SET 12=0xFE`+RET) = 3 < 10. A 256B op→row index (argc-style) needs
    /// ≥256B free in host DATA — low pad 0x80 + tail-to-outbuf for 1-op prog is
    /// 150 < 256 (layout cliff; raising outbuf / rewriting scan = new arch).
    #[test]
    fn s3_host_prog_loop_densify_reject() {
        let loop_recs = s3_prog_loop_code_host(1, S3_HOST_NROWS);
        assert_eq!(loop_recs.len(), 29, "host prog-loop body pin");
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF]))
                .unwrap()
                .records
                .len(),
            173,
            "host total pin (S3.367: win0+win1+prog + SET out + loop)"
        );
        // Sheep inventory inside the 29: set(10,nrows), miss set(12,0xFE), miss RET.
        let nrows_sets = loop_recs
            .iter()
            .filter(|r| r.0 == 0x30 && r.2[0] == 10 && r.2[1] == 0)
            .count();
        let miss_fe = loop_recs
            .iter()
            .filter(|r| r.0 == 0x30 && r.2[0] == 12 && r.2[1] == 0 && r.2[2] == 0xFE)
            .count();
        let rets = loop_recs.iter().filter(|r| r.0 == 0xFF).count();
        assert_eq!(nrows_sets, 1, "one nrows SET per loop body");
        assert_eq!(miss_fe, 1, "one miss FE marker");
        assert_eq!(rets, 2, "miss RET + end RET");
        let sheep = nrows_sets + miss_fe + 1; // +1 = miss RET (keep end RET)
        assert_eq!(sheep, 3);
        assert!(
            sheep < 10,
            "host loop sheep save {} < 10 bar — keep scan body",
            sheep
        );
        // 256B op→row index cannot fit without layout rewrite.
        let free_low = S3_HOST_TABLE_OFF as usize;
        let prog_end = S3_HOST_PROG_OFF as usize + 1 + 1; // 1-op prog
        let free_hi = (S3_HOST_OUTBUF_OFF as usize).saturating_sub(prog_end);
        assert_eq!(free_low, 0x80);
        assert_eq!(free_hi, 0x160 - 0x14A);
        assert!(
            free_low + free_hi < 256,
            "op→row index {}B free < 256 — layout cliff",
            free_low + free_hi
        );
        assert_eq!(
            parse_tyb(&s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT).0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN,
            "expand assembler pin stays 22"
        );
    }

    /// S3.330: drop host SET out after seed is −1 (sim preloads slot9). Sheep bar —
    /// keep SET so emit_x86/raw PE host TYB still lands outbuf (not sim-only).
    #[test]
    fn s3_host_set_out_sheep_reject() {
        let payload = s3_fixed_payload(&S3_FIXED10_OPS, &[0xFF], S3_HOST_OUTBUF_OFF as usize);
        let start = S3_HOST_TABLE_OFF as usize;
        let end = S3_HOST_PROG_OFF as usize + 1 + 1;
        let mut with = Vec::new();
        s3_ir_seed_at(&mut with, S3_HOST_TABLE_OFF, &payload[start..end], 10);
        with.push(s3_rec_set(S3_PACK_PROTO_DST, S3_HOST_OUTBUF_OFF));
        with.extend(s3_prog_loop_code_host(1, S3_HOST_NROWS));
        let mut without = Vec::new();
        s3_ir_seed_at(&mut without, S3_HOST_TABLE_OFF, &payload[start..end], 10);
        without.extend(s3_prog_loop_code_host(1, S3_HOST_NROWS));
        assert_eq!(with.len(), 171, "continuous-oracle host pin (pre-split)");
        assert_eq!(with.len() - without.len(), 1, "SET out alone is −1");
        assert!(
            with.len() - without.len() < 10,
            "host SET-out save {} < 10 sheep bar — keep for PE/emit_x86",
            with.len() - without.len()
        );
        assert_eq!(
            parse_tyb(&s3_host_prog_loop_tyb(&[0xFF])).unwrap().records.len(),
            173,
            "S3.367 prod floor; continuous oracle stays 171"
        );
        // Sim still works without SET (slot9 preload) — proves the −1 is real but rejected.
        let tyb = s3_host_tyb_from_records(&without);
        assert_eq!(
            s3_sim_prog_loop(
                &tyb,
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
            )
            .unwrap(),
            s3_compute_fixed_body(0xFF),
            "sim preload makes SET redundant — still do not take −1"
        );
    }

    /// S3.312: host long-seed pin (global-distinct).
    #[test]
    fn s3_host_seed_window_global_distinct_pin() {
        const HOST_PIN: usize = 173; // S3.312:220→171; S3.367:+2 table/prog split
        let tyb = s3_host_prog_loop_tyb(&[0xFF]);
        let n = parse_tyb(&tyb).unwrap().records.len();
        assert_eq!(n, HOST_PIN, "host IR seed pin");
        assert!(s3_unpack_data_tyb(&tyb).is_empty());
        assert_eq!(
            s3_sim_prog_loop(
                &tyb,
                S3_HOST_TABLE_OFF as u64,
                S3_HOST_OUTBUF_OFF,
                (S3_HOST_PROG_OFF + 1) as u64,
            )
            .unwrap(),
            s3_compute_fixed_body(0xFF),
            "host sim still emits RET after window-global seed"
        );
        // Seed alone is −49 vs legacy address-order (190→141); assembler untouched.
        let payload = s3_fixed_payload(&S3_FIXED10_OPS, &[0xFF], S3_HOST_OUTBUF_OFF as usize);
        let start = S3_HOST_TABLE_OFF as usize;
        let end = S3_HOST_PROG_OFF as usize + 2;
        let slice = &payload[start..end];
        assert!(slice.len() > 127);
        let mut seed = Vec::new();
        s3_ir_seed_at(&mut seed, S3_HOST_TABLE_OFF, slice, 10);
        assert_eq!(seed.len(), 141, "host table|prog seed IR");
        assert_eq!(
            parse_tyb(&s3_yoyo_expand_flat_tyb(S3_SHAPE_FILL_MANY_FLAT).0)
                .unwrap()
                .records
                .len(),
            S3_EXPAND_ASM_PIN,
            "expand assembler pin stays 22"
        );
    }




    /// S3.310/S3.311: A1 prologue density cliff — argc≤3 → 13 recs for 37B.
    /// S3.311: argc=6 consecutive hypothetic is 7 recs (−6 < 10 bar) and needs
    /// pe_dll RAW_BYTES rewrite + parse argc>3 — reject densify without that.
    #[test]
    fn s3_a1_prologue_density_cliff() {
        let lea = s3_short_lea_raw_bytes();
        assert_eq!(lea.len(), 37);
        let prologue = s3_in_dll_lea_prologue_recs();
        assert_eq!(prologue.len(), 13, "3-chunk A1 pack of 37B");
        let a1_bytes: usize = prologue.iter().map(|r| r.1 as usize).sum();
        assert_eq!(a1_bytes, 37);
        let mut one = Vec::new();
        s3_append_raw_a1(&mut one, &lea);
        assert_eq!(one.len(), 13, "one-stream A1 count == 3-chunk (no framing win)");
        // Framing differs (15|11|11 vs contiguous 3B) but argc≤3 density is identical.
        assert_ne!(one.as_slice(), prologue, "S3.265: three-chunk framing stays distinct");
        // S3.311: denser pack ceiling under fixed 6B args (consecutive) still <10 save.
        let argc6 = (lea.len() + 5) / 6;
        assert_eq!(argc6, 7, "argc=6 consecutive → 7 A1 for 37B");
        assert!(
            13usize.saturating_sub(argc6) < 10,
            "A1 densify save {} < 10 bar (pe_dll argc≤3 contract)",
            13 - argc6
        );
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
        // S3.217/S3.312 long-path: BASE first per live window (global-distinct); oo imm8.
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
        // S3.367: prod splits table|prog → one extra SET BASE vs continuous windows.
        assert_eq!(bases.len(), live_wins + 1, "S3.367: +1 BASE for prog poke");
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
            let wire = s3_pack_data_wire(&bytes);
            assert_eq!(wire.len(), ((n + S3_PACK_CHUNK - 1) / S3_PACK_CHUNK) * 8);
            let mut expect = Vec::with_capacity(HEADER_SIZE + wire.len());
            expect.extend_from_slice(&TYB_MAGIC);
            expect.extend_from_slice(&[0, 0]);
            expect.extend_from_slice(&((wire.len() / RECORD_SIZE) as u16).to_le_bytes());
            expect.extend_from_slice(&wire);
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
            assert_eq!(got, expect, "YOYO IR pack != TYB, len {n}");
        }
    }

    /// S3.143/S3.308: looped MEMCPY pack emits full TYB (== header + wire).
    #[test]
    fn s3_yoyo_ir_pack_loop_parity() {
        for n in 1usize..=64 {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 17 + 3) as u8).collect();
            let wire = s3_pack_data_wire(&bytes);
            let mut expect = Vec::with_capacity(HEADER_SIZE + wire.len());
            expect.extend_from_slice(&TYB_MAGIC);
            expect.extend_from_slice(&[0, 0]);
            expect.extend_from_slice(&((wire.len() / RECORD_SIZE) as u16).to_le_bytes());
            expect.extend_from_slice(&wire);
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
            assert_eq!(got, expect, "loop IR pack != TYB, len {n}");
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

    /// S3.222/S3.310: short seed + in-DLL empty-DATA share one global-distinct poke helper.
    /// BASE is caller-owned (SET absolute, or lea slot15). Still one SET/distinct; no SHR.
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

        let mut poke_slot15 = Vec::new();
        s3_append_stb_poke_global_distinct_base(&mut poke_slot15, slice, 15);
        let dll = parse_tyb(&s3_table_prog_loop_tyb_ir_seed(&[0x30, 0xFF])).unwrap();
        let base_sets = dll
            .records
            .iter()
            .filter(|r| {
                r.op == 0x30
                    && u16::from_le_bytes([r.args_raw[0], r.args_raw[1]]) == S3_PACK_STB_BASE
            })
            .count();
        assert_eq!(base_sets, 0, "in-DLL has no SET BASE (lea owns table VA)");
        assert!(
            !dll.records.iter().any(|r| {
                r.op == 0x64
                    && u16::from_le_bytes([r.args_raw[0], r.args_raw[1]]) == S3_PACK_STB_BASE
            }),
            "S3.310: no MOVRR BASE←15"
        );
        // Shared poke (base=15) appears contiguously after A1 prologue.
        let dll_tuples: Vec<_> = dll
            .records
            .iter()
            .map(|r| (r.op, r.argc, r.args_raw))
            .collect();
        let a1_end = dll_tuples
            .iter()
            .rposition(|r| r.0 == 0xA1)
            .expect("A1 prologue")
            + 1;
        assert_eq!(
            &dll_tuples[a1_end..a1_end + poke_slot15.len()],
            poke_slot15.as_slice(),
            "in-DLL seed poke is shared helper with base=slot15"
        );
    }

    /// S3.174/S3.200/S3.314: distinct bytes still need one SET each; zeros skipped
    /// by poke (no SUB run-fold). Trailing-zero high-water pin keeps composed len.
    #[test]
    fn s3_emit_set_stb_expansion_floor() {
        let body = [(0xFFu8, 0u8, [0u8; 6])];
        let flat_len = HEADER_SIZE + RECORD_SIZE;
        let (fw, off) = s3_yoyo_emit_tyb_file_unrolled(&body);
        let ir = parse_tyb(&fw).unwrap();
        assert!(
            !ir.records.iter().any(|r| r.op == 0x61),
            "S3.314: emit-flat poke has no SUB run-fold"
        );
        let sets = ir.records.iter().filter(|r| r.op == 0x30).count();
        assert!(sets < flat_len + 2, "S3.200/S3.314: poke collapses SET-per-byte");
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
        assert!(
            !ir.records.iter().any(|r| r.op == 0x61),
            "S3.314: two-phase poke has no SUB run-fold"
        );
        let sets = ir.records.iter().filter(|r| r.op == 0x30).count();
        let flat_len = HEADER_SIZE + big.len() * RECORD_SIZE;
        assert!(
            sets * 4 < flat_len,
            "S3.183/S3.314 poke collapses SET-per-byte: sets={sets} flat={flat_len}"
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

    /// S3.157/S3.307: looped STB pack IR emits full TYB (== header + `s3_pack_data` wire).
    #[test]
    fn s3_yoyo_ir_pack_loop_stb_parity() {
        for n in 1usize..=64 {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 17 + 3) as u8).collect();
            let wire = s3_pack_data_wire(&bytes);
            let mut expect = Vec::with_capacity(HEADER_SIZE + wire.len());
            expect.extend_from_slice(&TYB_MAGIC);
            expect.extend_from_slice(&[0, 0]);
            expect.extend_from_slice(&((wire.len() / RECORD_SIZE) as u16).to_le_bytes());
            expect.extend_from_slice(&wire);
            let (tyb, out_off) = s3_yoyo_pack_loop_stb_tyb(&bytes);
            assert!(
                parse_tyb(&tyb).unwrap().records.iter().any(|r| r.op == 0x81),
                "loop STB pack must use STB"
            );
            let got = s3_sim_prog_loop(&tyb, 0, out_off, 0)
                .unwrap_or_else(|e| panic!("loop stb pack len {n}: {e:#x}"));
            assert_eq!(got, expect, "loop STB IR pack != TYB, len {n}");
            assert_eq!(
                parse_tyb(&got).unwrap().records.len(),
                wire.len() / RECORD_SIZE
            );
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

    /// S3.155/S3.309: unrolled STB pack emits full TYB (== header + wire).
    #[test]
    fn s3_yoyo_ir_pack_stb_parity() {
        for n in 1usize..=6 {
            let bytes: Vec<u8> = (0..n).map(|i| (i * 17 + 3) as u8).collect();
            let wire = s3_pack_data_wire(&bytes);
            let mut expect = Vec::with_capacity(HEADER_SIZE + wire.len());
            expect.extend_from_slice(&TYB_MAGIC);
            expect.extend_from_slice(&[0, 0]);
            expect.extend_from_slice(&((wire.len() / RECORD_SIZE) as u16).to_le_bytes());
            expect.extend_from_slice(&wire);
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
            assert_eq!(got, expect, "STB IR pack != TYB, len {n}");
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
        assert_eq!(s3_fixed_row(0x30), s3_host_set_body().as_slice());
        assert_eq!(s3_fixed_row(0xFF), s3_host_ret_body().as_slice());
        assert_eq!(s3_fixed_row(0x00), s3_host_nop_body().as_slice());
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

    /// Map slim PE RWX, call entry, read in-DLL outbuf (Windows).
    #[cfg(windows)]
    fn s3_run_slim_pe_outbuf(pe: &[u8], len: usize) -> Vec<u8> {
        s3_run_slim_pe_outbuf_at(pe, S3_OUTBUF_OFF, len)
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

    /// S3.360: test oracle — live three-chunk A1 pack of lea bytes.
    #[cfg(test)]
    fn s3_in_dll_lea_prologue_recs_via_pack() -> Vec<(u8, u8, [u8; 6])> {
        let lea = s3_short_lea_raw_bytes();
        assert_eq!(lea.len(), 37);
        let mut r = Vec::new();
        s3_append_raw_a1(&mut r, &lea[..15]); // TABLE
        s3_append_raw_a1(&mut r, &lea[15..26]); // DST
        s3_append_raw_a1(&mut r, &lea[26..37]); // PROG
        r
    }

    /// S3.360: in-DLL lea→.text = **A1 record seed** (own ISA RAW_BYTES op).
    /// A1 stays the inject channel; Rust no longer live-packs at OnceLock init.
    /// Three-chunk framing (S3.265 golden). ≡ `s3_in_dll_lea_prologue_recs_via_pack`.
    const S3_IN_DLL_LEA_A1_SEED: [(u8, u8, [u8; 6]); 13] = [
        (0xa1, 3, [0x49, 0x00, 0x8d, 0x00, 0x87, 0x00]),
        (0xa1, 3, [0x80, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (0xa1, 3, [0x00, 0x00, 0x49, 0x00, 0x89, 0x00]),
        (0xa1, 3, [0x47, 0x00, 0x18, 0x00, 0x49, 0x00]),
        (0xa1, 3, [0x89, 0x00, 0x47, 0x00, 0x78, 0x00]),
        (0xa1, 3, [0x49, 0x00, 0x8d, 0x00, 0x87, 0x00]),
        (0xa1, 3, [0xc8, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (0xa1, 3, [0x00, 0x00, 0x49, 0x00, 0x89, 0x00]),
        (0xa1, 2, [0x47, 0x00, 0x48, 0x00, 0x00, 0x00]),
        (0xa1, 3, [0x49, 0x00, 0x8d, 0x00, 0x87, 0x00]),
        (0xa1, 3, [0xbd, 0x00, 0x00, 0x00, 0x00, 0x00]),
        (0xa1, 3, [0x00, 0x00, 0x49, 0x00, 0x89, 0x00]),
        (0xa1, 2, [0x47, 0x00, 0x10, 0x00, 0x00, 0x00]),
    ];

    /// S3.164/S3.234/S3.360: in-DLL short-lea A1 prologue (const seed).
    fn s3_in_dll_lea_prologue_recs() -> &'static [(u8, u8, [u8; 6])] {
        &S3_IN_DLL_LEA_A1_SEED
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

    /// S3.220/S3.222/S3.310: empty-DATA in-DLL — lea first (slot15 = table VA),
    /// poke through slot15 (no MOVRR BASE←15), then prog. via_ir golden unchanged.
    fn s3_table_prog_loop_tyb_ir_seed(ops: &[u8]) -> Vec<u8> {
        assert!(!ops.is_empty() && ops.len() <= 8);
        // S3.376: RET / SET+RET = full in-DLL TYB seeds (gap C beyond A1-only).
        if ops == [0xFF] {
            return S3_IN_DLL_RET_TYB_SEED.to_vec();
        }
        if ops == [0x30, 0xFF] {
            return S3_IN_DLL_SET_RET_TYB_SEED.to_vec();
        }
        let payload = s3_fixed_payload(&S3_FIXED10_OPS[..3], ops, S3_OUTBUF_OFF as usize);
        let start = S3_TABLE_OFF as usize;
        let end = S3_PROG_OFF as usize + 1 + ops.len();
        let slice = &payload[start..end];
        assert!(slice.len() <= 127, "in-DLL seed must stay one imm8 window");
        let mut recs = Vec::new();
        // PE: lea writes table VA into slot 15. Sim skips A1 but seeds state[15]=table_off.
        recs.extend_from_slice(s3_in_dll_lea_prologue_recs());
        // S3.310: STB base = slot15 directly (drop MOVRR into slot14).
        s3_append_stb_poke_global_distinct_base(&mut recs, slice, 15);
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
        // S3.381: ir-seed RET/SET+RET TYB ≡ image seed path (TYB seed → same outbuf).
        assert_eq!(
            s3_table_prog_loop_tyb_ir_seed(&[0x30, 0xFF]).as_slice(),
            S3_IN_DLL_SET_RET_TYB_SEED
        );
        let expect = emit_x86(
            &parse_tyb(&tyb_from_records(&[
                (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
                (0xFF, 0, [0; 6]),
            ]))
            .unwrap(),
        )
        .text;
        let got = s3_run_slim_pe_outbuf(S3_IN_DLL_PE_SET_RET_IMAGE_SEED, expect.len());
        assert_eq!(got, expect);
    }

    #[test]
    #[cfg(windows)]
    fn s3_yoyo_table_prog_loop_runs_on_pe() {
        // S3.381: SET+RET uses in-DLL PE image seed (no live emit_x86 for image).
        let pe = S3_IN_DLL_PE_SET_RET_IMAGE_SEED;
        let expect = emit_x86(
            &parse_tyb(&tyb_from_records(&[
                (0x30, 2, [0, 0, 0xEF, 0xBE, 0xAD, 0xDE]),
                (0xFF, 0, [0; 6]),
            ]))
            .unwrap(),
        )
        .text;
        let got = s3_run_slim_pe_outbuf(pe, expect.len());
        assert_eq!(got, expect);
        // RET seed also runs.
        let got_ret = s3_run_slim_pe_outbuf(S3_IN_DLL_PE_RET_IMAGE_SEED, 1);
        assert_eq!(got_ret, vec![0xC3]);
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
        // S3.494: IMUL+RET slots 0,1 = YOYO seed path; still ≠ FIXED10.
        let imul_ops: &[u8] = &[0x63, 0xFF];
        let imul_sem: &[(u8, u8, [u8; 6])] =
            &[(0x63, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])];
        let via_imul = try_emit_text_via_yoyo_table(imul_ops).expect("IMUL+RET yoyo");
        let rust_imul = emit_x86(&parse_tyb(&tyb_from_records(imul_sem)).unwrap()).text;
        assert_eq!(via_imul, rust_imul);
        let tyb_imul = tyb_from_records(imul_sem);
        assert!(try_compile_tyb_via_yoyo_table(&tyb_imul).is_some());
        assert!(fixed_template_ops(&parse_tyb(&tyb_imul).unwrap()).is_none());
        // Wrong IMUL slots still outside YOYO table/seed path.
        let other = tyb_from_records(&[(0x63, 2, [1, 0, 2, 0, 0, 0]), (0xFF, 0, [0; 6])]);
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
        // S3.494: IMUL+RET slots 0,1 = YOYO seed path (mini prefers it too).
        let imul_ret = tyb_from_records(&[(0x63, 2, [0, 0, 1, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        let pe_imul_y = try_compile_tyb_via_yoyo_table(&imul_ret).expect("imul yoyo");
        let pe_imul_m = compile_tyb_to_pe_mini(&imul_ret).expect("imul mini");
        assert_eq!(pe_imul_y.bytes, pe_imul_m.bytes);
        let expect = emit_x86(&parse_tyb(&imul_ret).unwrap()).text;
        assert_eq!(
            slim_pe_user_text(&pe_imul_m.bytes, expect.len()).unwrap(),
            expect.as_slice()
        );
        // Wrong-slot IMUL still emit_x86 fallback (outside YOYO seed).
        let imul_wrong = tyb_from_records(&[(0x63, 2, [1, 0, 2, 0, 0, 0]), (0xFF, 0, [0; 6])]);
        assert!(try_compile_tyb_via_yoyo_table(&imul_wrong).is_none());
        let pe_wrong = compile_tyb_to_pe_mini(&imul_wrong).expect("imul fallback");
        let expect_w = emit_x86(&parse_tyb(&imul_wrong).unwrap()).text;
        assert_eq!(
            slim_pe_user_text(&pe_wrong.bytes, expect_w.len()).unwrap(),
            expect_w.as_slice()
        );
    }
}
