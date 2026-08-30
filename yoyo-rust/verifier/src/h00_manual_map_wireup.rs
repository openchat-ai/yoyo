//! OW-IAT wire-up: H_00 manual-map path (post spike `pe_manual_map.rs`).
//!
//! Phase 1: file-read prelude — CreateFileA / ReadFile / VirtualAlloc / CloseHandle.
//! Phase 2: inline manual-map x64 — sections + DIR64 reloc + import resolve via PEB walk.
//! Phase 3: three-peer lockstep (JS `win32-h00-selfhost.js` + asm delegate).
//!
//! Replaces PEB ROR13 `LoadLibraryA` resolve in `gen_h00_selfhost_main`.

use crate::pe_link::WIN32_IO_H00_SCRATCH_OFF;
use crate::win32_selfhost::{SelfhostMeta, IAT_EXIT_PROCESS};

/// IAT slots at r15+0 (see pe_link KERNEL32_IO_FUNCS).
pub const IAT_VIRTUAL_ALLOC: u32 = 0;
pub const IAT_CREATE_FILE: u32 = 1;
pub const IAT_READ_FILE: u32 = 2;
pub const IAT_CLOSE_HANDLE: u32 = 4;

/// Stack scratch for ReadFile nNumberOfBytesRead (Win64 5th-arg slot at [rsp+0x20] in shadow).
const READ_BYTES_STACK_OFF: u8 = 0x20;
/// GPA proc-name spill for FlushICache (same slot — must stay above shadow, not inside it).
const FLUSH_ICACHE_NAME_STACK_OFF: u8 = READ_BYTES_STACK_OFF;
/// One Win64 home (0x20) + CreateFile 3 stack args (0x18); 0x38 bytes total.
/// After JMP-entry prologue (RSP%16==8), frame must be 8 mod 16 so FF15 CALL is 0 mod 16.
const PRELUDE_IO_FRAME: u8 = 0x38;
/// Forwarder DLL-name copy frame in fix_forward (copy buffer + align call to find_module).
const FORWARDER_NAME_FRAME: u8 = 0x40;
/// find_module LoadLibraryA fallback: one push r10 (8 B) before shadow → sub 30h not 38h for CALL align.
const FIND_MODULE_LL_SHADOW: u8 = 0x30;

/// PE32+ optional-header field offsets from `e_lfanew` (ebx holds e_lfanew; COFF = 20 B after PE sig).
const PE_OFF_NUMBER_OF_SECTIONS: u8 = 6; // COFF + 2
const PE_OFF_SIZE_OF_OPTIONAL_HEADER: u8 = 20; // COFF + 16
const PE_OFF_OPTIONAL: u8 = 24; // PE sig (4) + COFF (20)
const PE_OFF_IMAGE_BASE: u8 = PE_OFF_OPTIONAL + 24; // 0x30
const PE_OFF_SIZE_OF_IMAGE: u8 = PE_OFF_OPTIONAL + 56; // 0x50
const PE_OFF_SIZE_OF_HEADERS: u8 = PE_OFF_OPTIONAL + 60; // 0x54
const PE_OFF_IMPORT_DIR_RVA: u8 = PE_OFF_OPTIONAL + 120; // 0x90
const PE_OFF_BASERELOC_DIR_RVA: u8 = PE_OFF_OPTIONAL + 152; // 0xB0

/// PEB_LDR_DATA.InMemoryOrderModuleList.Flink at +0x20 (not InLoadOrder @ +0x10).
const PEB_LDR_INMEMORY_FLINK_OFF: u8 = 0x20;
/// InMemoryOrderLinks: Flink points at LDR entry + 0x10.
const LDR_INMEMORY_FLINK_OFF: u8 = 0x10;
const LDR_DLLBASE_OFF: u8 = 0x30;
const LDR_BASEDLLNAME_BUF_OFF: u8 = 0x60;
/// Bootstrap scratch past kernel32+preload IAT and import metadata (see pe_link).
const H00_LOADLIBRARY_SCRATCH_OFF: u32 = WIN32_IO_H00_SCRATCH_OFF;
const H00_GETPROCADDRESS_SCRATCH_OFF: u32 = WIN32_IO_H00_SCRATCH_OFF + 8;
const H00_KERNEL32_SCRATCH_OFF: u32 = WIN32_IO_H00_SCRATCH_OFF + 16;
/// Success-path phase probe (survives until crash for post-mortem; not used on fail epilogue).
const H00_PHASE_SCRATCH_OFF: u32 = WIN32_IO_H00_SCRATCH_OFF + 24;

const PHASE_H00_ENTERED: u8 = 0x00;
const PHASE_PRELUDE_CREATE_OK: u8 = 0x01;
const PHASE_PRELUDE_BUF_OK: u8 = 0x02;
const PHASE_PRELUDE_READ_OK: u8 = 0x03;
const PHASE_PRELUDE_DONE: u8 = 0x04;
const PHASE_PRELUDE_OK: u8 = 0x05;
const PHASE_MAP_VALLOC_OK: u8 = 0x09;
const PHASE_MAP_IMAGE_OK: u8 = 0x0A;
const PHASE_SECTIONS_OK: u8 = 0x0B;
const PHASE_RELOC_OK: u8 = 0x0C;
const PHASE_IMPORT_OK: u8 = 0x0D;
const PHASE_FLUSH_ICACHE: u8 = 0x0E;
const PHASE_EXPORT_CALL: u8 = 0x0F;

/// Win64 home-space before `call` to kernel32 (RSP%16==8 at callee entry).
/// PE entry is JMP (not CALL) into H_00: four pushes → RSP%16==8; `sub 0x38` → 0 at CALL.
const WIN64_CALL_SHADOW: u8 = 0x38;
/// Short dll/api name spill for 2-arg bootstrap calls (inside 32 B home space; ret at [rsp+38h]).
const WIN64_STACK_STR_OFF: u8 = 0x20;
/// IAT cursor spill during import resolve_export_ordinal — transient at [rsp+28h].
const IMPORT_IAT_CURSOR_SPILL_OFF: u8 = 0x28;
/// hModule spill in import-descriptor Win64 shadow — above [rsp+28h] home (not clobbered by resolve_export).
const HMODULE_SPILL_OFF: u8 = 0x30;

fn emit_win64_call_shadow(c: &mut Vec<u8>) {
    c.extend_from_slice(&[0x48, 0x83, 0xEC, WIN64_CALL_SHADOW]);
}

fn emit_win64_pop_shadow(c: &mut Vec<u8>) {
    c.extend_from_slice(&[0x48, 0x83, 0xC4, WIN64_CALL_SHADOW]);
}

/// After `test`/`cmp` ZF=1 (fail): pop Win64 shadow then jmp fail; ZF=0 skip
/// trampoline. Inline `jz`→trampoline (rel=0) made success fall into exit=7.
fn emit_jz_pop_shadow_then_fail(c: &mut Vec<u8>, chunk_text_off: usize, fail_import: usize) {
    let jnz = c.len();
    c.extend_from_slice(&[0x0F, 0x85, 0, 0, 0, 0]); // jnz success
    emit_win64_pop_shadow(c);
    let jmp = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(
        c,
        jmp + 1,
        chunk_text_off + jmp + 5,
        fail_import,
    );
    let success = c.len();
    patch_rel32(
        c,
        jnz + 2,
        chunk_text_off + jnz + 6,
        chunk_text_off + success,
    );
}

/// ExitProcess via `[r15+ExitProcess]` after reload (IAT at r15+0).
fn emit_exit_process_iat(
    c: &mut Vec<u8>,
    text_rva: u32,
    chunk_text_off: u32,
    iat_rva: u32,
    exit_code: u8,
) {
    emit_reload_r15_data_base(c, text_rva, chunk_text_off, iat_rva);
    emit_win64_call_shadow(c);
    c.extend_from_slice(&[0xB9, exit_code, 0, 0, 0]);
    emit_call_iat_r15_slot(c, IAT_EXIT_PROCESS);
}

fn emit_mov_qword_to_r15_scratch(c: &mut Vec<u8>, off: u32, reg: u8) {
    // reg: 0=rax, 7=rdi
    c.push(0x49);
    c.push(0x89);
    c.push(0x87 | (reg << 3));
    c.extend_from_slice(&off.to_le_bytes());
}

fn emit_mov_qword_from_r15_scratch(c: &mut Vec<u8>, off: u32, reg: u8) {
    c.push(0x49);
    c.push(0x8B);
    c.push(0x87 | (reg << 3));
    c.extend_from_slice(&off.to_le_bytes());
}

fn emit_cmp_r15_scratch_qword_zero(c: &mut Vec<u8>, off: u32) {
    c.extend_from_slice(&[0x49, 0x83, 0xBF]);
    c.extend_from_slice(&off.to_le_bytes());
    c.push(0x00);
}

fn emit_call_r15_scratch(c: &mut Vec<u8>, off: u32) {
    c.extend_from_slice(&[0x41, 0xFF, 0x97]);
    c.extend_from_slice(&off.to_le_bytes());
}

fn emit_mov_qword_r15_scratch_imm0(c: &mut Vec<u8>, off: u32) {
    c.extend_from_slice(&[0x49, 0xC7, 0x87]);
    c.extend_from_slice(&off.to_le_bytes());
    c.extend_from_slice(&[0u8; 4]);
}

/// Post-mortem phase byte in `.data` scratch (rip-relative; no r15 — avoids Windows AV on [r15+scratch]).
fn emit_phase_probe(c: &mut Vec<u8>, text_rva: u32, chunk_text_off: u32, data_rva: u32, phase: u8) {
    let at = c.len();
    c.extend_from_slice(&[0xC6, 0x05, 0, 0, 0, 0]);
    c.push(phase);
    fix_rip_disp(
        c,
        at + 2,
        text_rva,
        chunk_text_off,
        at + 7,
        data_rva + H00_PHASE_SCRATCH_OFF,
    );
}

/// Bisect exit before phase probe; skip probe when compile-time bisect matches (no fall-through).
fn emit_phase_with_bisect(
    c: &mut Vec<u8>,
    text_rva: u32,
    chunk_text_off: u32,
    iat_rva: u32,
    phase: u8,
) {
    if maybe_bisect_exit_after_phase(c, text_rva, chunk_text_off, iat_rva, phase) {
        return;
    }
    emit_phase_probe(c, text_rva, chunk_text_off, iat_rva, phase);
}

/// reload r15 + Win64 shadow + `mov ecx,imm32` + `call [r15+ExitProcess]`.
const FAIL_EPILOGUE_LEN: usize = 23;

/// When `H00_BISECT_EXIT` matches `150 + phase` (151–165), exit before phase probe (CI bisect).
/// Returns true when bisect exit was emitted (caller must not emit phase probe).
fn maybe_bisect_exit_after_phase(
    c: &mut Vec<u8>,
    text_rva: u32,
    chunk_text_off: u32,
    iat_rva: u32,
    phase: u8,
) -> bool {
    if let Some(target) = option_env!("H00_BISECT_EXIT").and_then(|s| s.parse::<u8>().ok()) {
        let exit_code = 150u8.wrapping_add(phase);
        if exit_code == target {
            // Bisect inside prelude I/O frame: pop frame before ExitProcess shadow (else double-sub AV).
            if (PHASE_PRELUDE_CREATE_OK..=PHASE_PRELUDE_READ_OK).contains(&phase) {
                emit_prelude_io_frame_free(c);
            }
            emit_exit_process_iat(c, text_rva, chunk_text_off, iat_rva, exit_code);
            return true;
        }
    }
    false
}

fn emit_prelude_io_frame_alloc(c: &mut Vec<u8>) {
    c.extend_from_slice(&[0x48, 0x83, 0xEC, PRELUDE_IO_FRAME]);
}

fn emit_prelude_io_frame_free(c: &mut Vec<u8>) {
    c.extend_from_slice(&[0x48, 0x83, 0xC4, PRELUDE_IO_FRAME]);
}

/// After `test`/`cmp` ZF=1 (fail): pop prelude I/O frame then jmp fail; ZF=0
/// skip trampoline. Inline `jz`→trampoline forced CreateFile/ReadFile success
/// onto exit=2/3 (stage17 sidecar always Read/empty).
fn emit_jz_pop_prelude_frame_then_fail(c: &mut Vec<u8>, chunk_text_off: usize, fail: usize) {
    let jnz = c.len();
    c.extend_from_slice(&[0x0F, 0x85, 0, 0, 0, 0]); // jnz success
    emit_prelude_io_frame_free(c);
    let jmp = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(c, jmp + 1, chunk_text_off + jmp + 5, fail);
    let success = c.len();
    patch_rel32(
        c,
        jnz + 2,
        chunk_text_off + jnz + 6,
        chunk_text_off + success,
    );
}

/// `mov r8d, [r14+rbx+disp]` — mapped-image optional-header field (ebx = e_lfanew).
fn emit_mov_u32_pe_mapped(c: &mut Vec<u8>, disp: u8) {
    c.push(0x45); // REX.R+B for r8d + r14 SIB base
    c.push(0x8B);
    if disp < 0x80 {
        c.extend_from_slice(&[0x44, 0x1E, disp]); // mod=01 disp8, reg=r8, SIB [r14+rbx]
    } else {
        c.extend_from_slice(&[0x84, 0x1E, disp, 0, 0, 0]); // mod=10 disp32
    }
}

/// H_00 stub prologue (four `push` saves + reload r15) before prelude.
pub const H00_PROLOGUE_LEN: u32 = 14;

fn patch_rel32(c: &mut [u8], disp_off: usize, from: usize, to: usize) {
    let rel = to as i32 - from as i32;
    c[disp_off..disp_off + 4].copy_from_slice(&rel.to_le_bytes());
}

/// `call [r15+slot*8]` — prelude uses r15 (.data/IAT base) after `emit_reload_r15_data_base`.
fn emit_call_iat_r15_slot(c: &mut Vec<u8>, slot: u32) {
    c.extend_from_slice(&[0x41, 0xFF, 0x97]);
    c.extend_from_slice(&(slot * 8).to_le_bytes());
}

/// `chunk_text_off` = RVA offset from `.text` base to the start of the chunk being emitted.
fn emit_call_iat_merged(
    c: &mut Vec<u8>,
    text_rva: u32,
    chunk_text_off: u32,
    iat_rva: u32,
    slot: u32,
) {
    let at = c.len();
    c.extend_from_slice(&[0xFF, 0x15, 0, 0, 0, 0]);
    let next_rva = text_rva + chunk_text_off + at as u32 + 6;
    let disp = (iat_rva + slot * 8) as i32 - next_rva as i32;
    c[at + 2..at + 6].copy_from_slice(&disp.to_le_bytes());
}

fn fix_rip_disp(
    c: &mut [u8],
    disp_off: usize,
    text_rva: u32,
    chunk_text_off: u32,
    insn_end_in_chunk: usize,
    target_rva: u32,
) {
    let next = (text_rva + chunk_text_off + insn_end_in_chunk as u32) as i32;
    let disp = target_rva as i32 - next;
    c[disp_off..disp_off + 4].copy_from_slice(&disp.to_le_bytes());
}

/// SIB for `[r12 + rbx*1 + disp]` (requires REX.B on the opcode).
const SIB_R12_RBX: u8 = 0x1C;
/// SIB for `[r12 + disp8]` (index=none, base=r12 via REX.B on opcode).
const SIB_R12_ONLY: u8 = 0x24;
/// SIB for `[r14 + disp8]` (index=none, base=r14 via REX.B on opcode).
const SIB_R14_ONLY: u8 = 0x26;

/// `lea r15, [rip+disp]` — reload .data base (IAT at r15+0) before [r15+scratch] probes.
fn emit_reload_r15_data_base(c: &mut Vec<u8>, text_rva: u32, chunk_text_off: u32, data_rva: u32) {
    let at = c.len();
    c.extend_from_slice(&[0x4C, 0x8D, 0x3D, 0, 0, 0, 0]);
    fix_rip_disp(c, at + 3, text_rva, chunk_text_off, at + 7, data_rva);
}

/// `mov ebx, [r12+3Ch]` — e_lfanew from file PE buffer (r12).
fn emit_mov_e_lfanew_pe_file(c: &mut Vec<u8>) {
    c.extend_from_slice(&[0x41, 0x8B, 0x5C, SIB_R12_ONLY, 0x3C]);
}

/// `mov ebx, [r14+3Ch]` — e_lfanew from mapped image (r14).
/// MUST use SIB 26 (base r14); 41 8B 5E 3C is mod=11 mov ebx,r14 — NOT a memory load.
fn emit_mov_e_lfanew_pe_mapped(c: &mut Vec<u8>) {
    c.extend_from_slice(&[0x41, 0x8B, 0x5C, SIB_R14_ONLY, 0x3C]);
}

/// `mov r32, [r12+rbx+disp]` — PE optional-header field via e_lfanew in ebx.
fn emit_mov_u32_pe_file(c: &mut Vec<u8>, reg: u8, disp: u8) {
    c.push(0x41);
    c.push(0x8B);
    if disp < 0x80 {
        c.push(0x40 | (reg << 3) | 0x04); // mod=01 disp8
        c.push(SIB_R12_RBX);
        c.push(disp);
    } else {
        c.extend_from_slice(&[0x80 | (reg << 3) | 0x04, SIB_R12_RBX, disp, 0, 0, 0]);
    }
}

/// `movzx r32, word [r12+rbx+disp]`
fn emit_movzx_u16_pe_file(c: &mut Vec<u8>, reg: u8, disp: u8) {
    c.push(0x41);
    c.extend_from_slice(&[0x0F, 0xB7]);
    if disp < 0x80 {
        c.push(0x40 | (reg << 3) | 0x04); // mod=01 disp8
        c.push(SIB_R12_RBX);
        c.push(disp);
    } else {
        c.extend_from_slice(&[0x80 | (reg << 3) | 0x04, SIB_R12_RBX, disp, 0, 0, 0]);
    }
}

/// Emit x64 that reads cwd sidecar `yoyo_rt.dll` into a VirtualAlloc buffer.
///
/// On success: `r12` = file bytes pointer, `r13d` = byte count.
/// On failure: jumps to phase-specific fail labels (CreateFile=2, Read=3, VirtualAlloc=4).
pub fn gen_h00_read_sidecar_prelude(
    meta: &SelfhostMeta,
    text_rva: u32,
    chunk_text_off: u32,
    fail_create_file: usize,
    fail_read_empty: usize,
    fail_virtual_alloc: usize,
) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();

    emit_reload_r15_data_base(&mut c, text_rva, chunk_text_off, meta.iat_rva);

    // Single Win64 frame for CreateFile → VirtualAlloc → ReadFile → CloseHandle (platform_io layout).
    emit_prelude_io_frame_alloc(&mut c);

    // CreateFileA(path, GENERIC_READ, FILE_SHARE_READ, sa=NULL, OPEN_EXISTING, NORMAL, hTemplate=NULL)
    let lea_path = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x0D, 0, 0, 0, 0]); // rcx = path
    c.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x80]); // rdx = GENERIC_READ
    c.extend_from_slice(&[0x41, 0xB8, 0x01, 0x00, 0x00, 0x00]); // r8d = FILE_SHARE_READ
    c.extend_from_slice(&[0x45, 0x31, 0xC9]); // xor r9d,r9d
    c.extend_from_slice(&[0xC7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00]); // OPEN_EXISTING
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00]); // FILE_ATTRIBUTE_NORMAL
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00]); // hTemplateFile
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_CREATE_FILE);
    c.extend_from_slice(&[0x48, 0x83, 0xF8, 0xFF]);
    emit_jz_pop_prelude_frame_then_fail(&mut c, chunk_text_off as usize, fail_create_file);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]); // test rax,rax — NULL handle
    emit_jz_pop_prelude_frame_then_fail(&mut c, chunk_text_off as usize, fail_create_file);
    c.extend_from_slice(&[0x48, 0x89, 0xC3]);
    emit_reload_r15_data_base(&mut c, text_rva, chunk_text_off, meta.iat_rva);
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        meta.iat_rva,
        PHASE_PRELUDE_CREATE_OK,
    );

    c.extend_from_slice(&[0x31, 0xC9]);
    c.extend_from_slice(&[0xBA, 0x00, 0x00, 0x80, 0x00]); // max read 8 MiB (crt-static sidecar)
    c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x30, 0x00, 0x00]);
    c.extend_from_slice(&[0x41, 0xB9, 0x04, 0x00, 0x00, 0x00]);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_VIRTUAL_ALLOC);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    emit_jz_pop_prelude_frame_then_fail(&mut c, chunk_text_off as usize, fail_virtual_alloc);
    c.extend_from_slice(&[0x49, 0x89, 0xC4]);
    emit_reload_r15_data_base(&mut c, text_rva, chunk_text_off, meta.iat_rva);
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        meta.iat_rva,
        PHASE_PRELUDE_BUF_OK,
    );

    // ReadFile(h, buf, size, &nBytesRead, NULL) — match platform_io: r9=&[rsp+20h], clear slot before call.
    c.extend_from_slice(&[0x48, 0x89, 0xD9]);
    c.extend_from_slice(&[0x4C, 0x89, 0xE2]);
    c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x00, 0x80, 0x00]); // ReadFile size cap
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, READ_BYTES_STACK_OFF, 0, 0, 0, 0]); // nBytesRead = 0
    c.extend_from_slice(&[0x4C, 0x8D, 0x4C, 0x24, READ_BYTES_STACK_OFF]); // lea r9,[rsp+20h]
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0, 0, 0, 0]); // lpOverlapped NULL (platform_io [rsp+28h])
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_READ_FILE);
    c.extend_from_slice(&[0x85, 0xC0]); // test eax,eax — ReadFile BOOL
    emit_jz_pop_prelude_frame_then_fail(&mut c, chunk_text_off as usize, fail_read_empty);
    emit_reload_r15_data_base(&mut c, text_rva, chunk_text_off, meta.iat_rva);
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        meta.iat_rva,
        PHASE_PRELUDE_READ_OK,
    );
    c.extend_from_slice(&[0x44, 0x8B, 0x6C, 0x24, READ_BYTES_STACK_OFF]); // r13d = nBytesRead

    c.extend_from_slice(&[0x48, 0x89, 0xD9]);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_CLOSE_HANDLE);

    emit_prelude_io_frame_free(&mut c);

    c.extend_from_slice(&[0x45, 0x85, 0xED]);
    let jz_empty = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    emit_reload_r15_data_base(&mut c, text_rva, chunk_text_off, meta.iat_rva);
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        meta.iat_rva,
        PHASE_PRELUDE_DONE,
    );

    fix_rip_disp(
        &mut c,
        lea_path + 3,
        text_rva,
        chunk_text_off,
        lea_path + 7,
        meta.temp_name_rva,
    );

    for (at, fail) in [(jz_empty, fail_read_empty)] {
        patch_rel32(
            &mut c,
            at + 2,
            chunk_text_off as usize + at + 6,
            fail,
        );
    }

    emit_reload_r15_data_base(&mut c, text_rva, chunk_text_off, meta.iat_rva);
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        meta.iat_rva,
        PHASE_PRELUDE_OK,
    );

    c
}

/// Emit inline PE manual-map: r12=file PE → rbx=mapped image base (or fail_label).
///
/// Uses r14 for mapped image during mapping. Preserves r15 (.data base).
fn gen_h00_manual_map_body(
    text_rva: u32,
    chunk_text_off: u32,
    iat_rva: u32,
    fail_virtual_alloc: usize,
    fail_import: usize,
) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();
    let mut fail_jumps: Vec<(usize, usize)> = Vec::new();

    // ebx = e_lfanew; r12 = file PE
    emit_mov_e_lfanew_pe_file(&mut c);
    // VirtualAlloc(0, SizeOfImage, MEM_COMMIT|RESERVE, PAGE_EXECUTE_READWRITE)
    emit_mov_u32_pe_file(&mut c, 2, PE_OFF_SIZE_OF_IMAGE); // mov edx,[r12+rbx+50h]
    c.extend_from_slice(&[0x31, 0xC9]);
    c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x30, 0x00, 0x00]);
    c.extend_from_slice(&[0x41, 0xB9, 0x40, 0x00, 0x00, 0x00]);
    emit_win64_call_shadow(&mut c);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, iat_rva, IAT_VIRTUAL_ALLOC);
    emit_win64_pop_shadow(&mut c);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    fail_jumps.push((c.len(), fail_virtual_alloc));
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x89, 0xC6]); // mov r14, rax (image)
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        iat_rva,
        PHASE_MAP_VALLOC_OK,
    );
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        iat_rva,
        PHASE_MAP_IMAGE_OK,
    );

    // Copy headers: rep movsb min(SizeOfHeaders, r13d=file bytes read)
    emit_mov_u32_pe_file(&mut c, 1, PE_OFF_SIZE_OF_HEADERS); // mov ecx,[r12+rbx+54h]
    c.extend_from_slice(&[0x44, 0x39, 0xED]); // cmp r13d, ecx
    c.extend_from_slice(&[0x41, 0x0F, 0x42, 0xCD]); // cmovb ecx, r13d
    c.extend_from_slice(&[0x4C, 0x89, 0xF7]); // mov rdi, r14
    c.extend_from_slice(&[0x4C, 0x89, 0xE6]); // mov rsi, r12
    c.extend_from_slice(&[0xF3, 0xA4]); // rep movsb

    // Section copy loop: esi = NumberOfSections, r8d = index
    emit_movzx_u16_pe_file(&mut c, 6, PE_OFF_NUMBER_OF_SECTIONS); // movzx esi,[r12+rbx+6]
    c.extend_from_slice(&[0x45, 0x31, 0xC0]); // xor r8d,r8d
    let sec_loop = c.len();
    c.extend_from_slice(&[0x44, 0x39, 0xC6]); // cmp esi,r8d
    let jae_secs_done = c.len();
    c.extend_from_slice(&[0x0F, 0x83, 0, 0, 0, 0]);
    // section hdr = r12 + rbx + 24 + SizeOfOptionalHeader + r8*40
    emit_movzx_u16_pe_file(&mut c, 0, PE_OFF_SIZE_OF_OPTIONAL_HEADER); // movzx eax,[r12+rbx+14h]
    c.extend_from_slice(&[0x83, 0xC0, PE_OFF_OPTIONAL]); // add eax,24
    c.extend_from_slice(&[0x49, 0x8D, 0x3C, 0x1C]); // lea rdi,[r12+rbx]
    c.extend_from_slice(&[0x48, 0x01, 0xC7]); // add rdi,rax
    c.extend_from_slice(&[0x41, 0x6B, 0xC0, 0x28]); // imul eax,r8d,40
    c.extend_from_slice(&[0x48, 0x01, 0xC7]); // add rdi,rax

    c.extend_from_slice(&[0x8B, 0x4F, 0x0C]); // mov ecx,[rdi+0c] VirtualAddress
    c.extend_from_slice(&[0x8B, 0x57, 0x10]); // mov edx,[rdi+10] SizeOfRawData
    c.extend_from_slice(&[0x8B, 0x47, 0x08]); // mov eax,[rdi+8] VirtualSize
    c.extend_from_slice(&[0x39, 0xC2]); // cmp edx,eax (raw vs virtual)
    let jbe_raw_le_virtual = c.len();
    c.extend_from_slice(&[0x0F, 0x86, 0, 0, 0, 0]); // jbe use_raw_size
    c.extend_from_slice(&[0x89, 0xC2]); // mov edx,eax — copy min(raw,virtual)
    let use_raw_size = c.len();
    patch_rel32(
        &mut c,
        jbe_raw_le_virtual + 2,
        jbe_raw_le_virtual + 6,
        use_raw_size,
    );
    c.extend_from_slice(&[0x44, 0x8B, 0x4F, 0x14]); // mov r9d,[rdi+14] PointerToRawData
    c.extend_from_slice(&[0x85, 0xD2]);
    let jz_next_sec = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    // Cap copy: edx=min(edx, r13d-r9d, SizeOfImage-ecx).
    c.extend_from_slice(&[0x44, 0x89, 0xE8]); // mov eax, r13d
    c.extend_from_slice(&[0x44, 0x29, 0xC8]); // sub eax, r9d (file bytes left)
    c.extend_from_slice(&[0x85, 0xC0]);
    let jle_next_sec = c.len();
    c.extend_from_slice(&[0x0F, 0x8E, 0, 0, 0, 0]); // jle next_sec
    c.extend_from_slice(&[0x39, 0xC2]); // cmp edx, eax
    c.extend_from_slice(&[0x0F, 0x42, 0xD0]); // cmovb edx, eax
    emit_mov_u32_pe_file(&mut c, 0, PE_OFF_SIZE_OF_IMAGE); // eax = SizeOfImage
    c.extend_from_slice(&[0x29, 0xC8]); // sub eax, ecx
    c.extend_from_slice(&[0x39, 0xC2]); // cmp edx, eax
    c.extend_from_slice(&[0x0F, 0x42, 0xD0]); // cmovb edx, eax
    c.extend_from_slice(&[0x85, 0xD2]);
    let jz_next_sec2 = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x8D, 0x3C, 0x0E]); // lea rdi,[r14+rcx]
    c.extend_from_slice(&[0x4B, 0x8D, 0x34, 0x0C]); // lea rsi,[r12+r9] (REX.W|X|B — 4A lacks B → [rsp+r9])
    c.extend_from_slice(&[0x89, 0xD1]); // mov ecx, edx
    c.extend_from_slice(&[0xF3, 0xA4]);
    let next_sec = c.len();
    patch_rel32(&mut c, jz_next_sec + 2, jz_next_sec + 6, next_sec);
    patch_rel32(&mut c, jle_next_sec + 2, jle_next_sec + 6, next_sec);
    patch_rel32(&mut c, jz_next_sec2 + 2, jz_next_sec2 + 6, next_sec);
    c.extend_from_slice(&[0x41, 0xFF, 0xC0]); // inc r8d
    let jmp_sec = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_sec + 1, jmp_sec + 5, sec_loop);
    let secs_done = c.len();
    patch_rel32(&mut c, jae_secs_done + 2, jae_secs_done + 6, secs_done);
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        iat_rva,
        PHASE_SECTIONS_OK,
    );

    // Reloc delta: r10 = mapped_base - ImageBase
    c.extend_from_slice(&[
        0x4D, 0x8B, 0x94, 0x1C, PE_OFF_IMAGE_BASE, 0x00, 0x00, 0x00,
    ]); // mov r10,[r12+rbx+30h] — 4D=REX.W|R|B; 4F adds REX.X→index r11
    c.extend_from_slice(&[0x4C, 0x89, 0xF0]); // mov rax, r14 (mapped base)
    c.extend_from_slice(&[0x4C, 0x29, 0xD0]); // sub rax, r10 → delta
    c.extend_from_slice(&[0x49, 0x89, 0xC2]); // mov r10, rax

    // Base reloc directory RVA (data directory index 5)
    emit_mov_u32_pe_file(&mut c, 0, PE_OFF_BASERELOC_DIR_RVA);
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_reloc_done = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x8D, 0x34, 0x06]); // lea rsi,[r14+rax]
    let reloc_block = c.len();
    c.extend_from_slice(&[0x8B, 0x0E]); // mov ecx,[rsi] page rva
    c.extend_from_slice(&[0x85, 0xC9]);
    let jz_reloc_done2 = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x8B, 0x56, 0x04]); // mov edx,[rsi+4] block size
    c.extend_from_slice(&[0x83, 0xFA, 0x08]);
    let jb_reloc_done = c.len();
    c.extend_from_slice(&[0x0F, 0x82, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x8D, 0x7E, 0x08]); // lea rdi,[rsi+8]
    c.extend_from_slice(&[0x8D, 0x42, 0xF8]); // lea eax,[rdx-8]
    c.extend_from_slice(&[0xD1, 0xE8]); // shr eax,1  entry count
    c.extend_from_slice(&[0x41, 0x89, 0xC0]); // mov r8d, eax (preserve ebx = e_lfanew)
    let reloc_entry = c.len();
    c.extend_from_slice(&[0x45, 0x85, 0xC0]); // test r8d,r8d
    let jbe_next_block = c.len();
    c.extend_from_slice(&[0x0F, 0x86, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x0F, 0xB7, 0x07]); // movzx eax,word [rdi]
    c.extend_from_slice(&[0x89, 0xC2]); // mov edx, eax
    c.extend_from_slice(&[0xC1, 0xEA, 0x0C]); // shr edx,12 type
    c.extend_from_slice(&[0x83, 0xFA, 0x0A]); // cmp edx,10 DIR64
    let jne_re = c.len();
    c.extend_from_slice(&[0x0F, 0x85, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x25, 0xFF, 0x0F, 0x00, 0x00]); // and eax, 0xfff (page offset)
    c.extend_from_slice(&[0x89, 0xC2]); // mov edx, eax
    c.extend_from_slice(&[0x4D, 0x8D, 0x1C, 0x0E]); // lea r11,[r14+rcx]
    c.extend_from_slice(&[0x49, 0x01, 0xD3]); // add r11, rdx
    c.extend_from_slice(&[0x4D, 0x01, 0x13]); // add [r11], r10
    let next_re = c.len();
    patch_rel32(&mut c, jne_re + 2, jne_re + 6, next_re);
    c.extend_from_slice(&[0x48, 0x83, 0xC7, 0x02]); // next reloc entry (rdi preserved)
    c.extend_from_slice(&[0x41, 0xFF, 0xC8]); // dec r8d
    let jmp_re = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_re + 1, jmp_re + 5, reloc_entry);
    let next_block = c.len();
    patch_rel32(&mut c, jbe_next_block + 2, jbe_next_block + 6, next_block);
    c.extend_from_slice(&[0x48, 0x01, 0xD6]); // add rsi, rdx
    let jmp_rb = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_rb + 1, jmp_rb + 5, reloc_block);
    let reloc_done = c.len();
    patch_rel32(&mut c, jz_reloc_done + 2, jz_reloc_done + 6, reloc_done);
    patch_rel32(&mut c, jz_reloc_done2 + 2, jz_reloc_done2 + 6, reloc_done);
    patch_rel32(&mut c, jb_reloc_done + 2, jb_reloc_done + 6, reloc_done);
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        iat_rva,
        PHASE_RELOC_OK,
    );

    // Bootstrap LoadLibraryA at [r15+scratch] for find_module fallback (api-set forwarders).
    emit_reload_r15_data_base(&mut c, text_rva, chunk_text_off, iat_rva);
    emit_mov_qword_r15_scratch_imm0(&mut c, H00_LOADLIBRARY_SCRATCH_OFF);
    emit_mov_u32_pe_file(&mut c, 0, PE_OFF_IMPORT_DIR_RVA);
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_skip_ll_boot = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    // Bootstrap must find kernel32 via PEB — not sidecar import[0] (order varies).
    emit_win64_call_shadow(&mut c);
    for (off, ch) in b"kernel32.dll\0".iter().enumerate() {
        c.extend_from_slice(&[0xC6, 0x44, 0x24, WIN64_STACK_STR_OFF + off as u8, *ch]);
    }
    c.extend_from_slice(&[0x48, 0x8D, 0x54, 0x24, WIN64_STACK_STR_OFF]); // lea rdx,[rsp+20h]
    let call_boot_find = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_skip_ll_boot2 = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xC7]); // rdi = kernel32
    // Reuse stack slot for LoadLibraryA export name (no contiguous needle in PE).
    for (off, ch) in [
        (0u8, b'L'),
        (1, b'o'),
        (2, b'a'),
        (3, b'd'),
        (4, b'L'),
        (5, b'i'),
        (6, b'b'),
        (7, b'r'),
        (8, b'a'),
        (9, b'r'),
        (10, b'y'),
        (11, b'A'),
        (12, 0),
    ] {
        c.extend_from_slice(&[0xC6, 0x44, 0x24, WIN64_STACK_STR_OFF + off, ch]);
    }
    c.extend_from_slice(&[0x48, 0x8D, 0x54, 0x24, WIN64_STACK_STR_OFF]); // lea rdx,[rsp+20h]
    let call_boot_resolve = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    emit_mov_qword_to_r15_scratch(&mut c, H00_LOADLIBRARY_SCRATCH_OFF, 0); // [r15+scratch]=LoadLibraryA
    emit_mov_qword_to_r15_scratch(&mut c, H00_KERNEL32_SCRATCH_OFF, 7); // [r15+0x60]=kernel32
    // Bootstrap GetProcAddress (sidecar IAT resolve uses host LoadLibrary+GetProcAddress).
    for (off, ch) in [
        (0u8, b'G'),
        (1, b'e'),
        (2, b't'),
        (3, b'P'),
        (4, b'r'),
        (5, b'o'),
        (6, b'c'),
        (7, b'A'),
        (8, b'd'),
        (9, b'd'),
        (10, b'r'),
        (11, b'e'),
        (12, b's'),
        (13, b's'),
        (14, 0),
    ] {
        c.extend_from_slice(&[0xC6, 0x44, 0x24, WIN64_STACK_STR_OFF + off, ch]);
    }
    c.extend_from_slice(&[0x48, 0x8D, 0x54, 0x24, WIN64_STACK_STR_OFF]);
    let call_boot_gpa = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    emit_mov_qword_to_r15_scratch(&mut c, H00_GETPROCADDRESS_SCRATCH_OFF, 0);
    emit_win64_pop_shadow(&mut c);
    let jmp_boot_ok = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]); // success: skip failure pop
    let skip_ll_boot_pop = c.len();
    emit_win64_pop_shadow(&mut c); // find_module failed — pop bootstrap shadow
    let jmp_boot_fail = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jz_skip_ll_boot2 + 2, jz_skip_ll_boot2 + 6, skip_ll_boot_pop);

    // Import resolve: walk descriptors at [r14+import_rva]
    let import_walk_start = c.len();
    // Bootstrap resolve_export clobbers ebx (AddressOfNames); reload file e_lfanew.
    emit_reload_r15_data_base(&mut c, text_rva, chunk_text_off, iat_rva);
    emit_mov_e_lfanew_pe_file(&mut c);
    emit_mov_u32_pe_file(&mut c, 0, PE_OFF_IMPORT_DIR_RVA); // import dir rva
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_import_done = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x8D, 0x34, 0x06]); // lea rsi,[r14+rax] desc
    let import_desc = c.len();
    c.extend_from_slice(&[0x8B, 0x06]); // OriginalFirstThunk
    c.extend_from_slice(&[0x89, 0xC3]); // mov ebx, eax
    c.extend_from_slice(&[0x8B, 0x4E, 0x0C]); // Name RVA
    c.extend_from_slice(&[0x8B, 0x56, 0x10]); // FirstThunk
    // Null descriptor: Name, FirstThunk, and OFT all zero (OFT alone may be 0).
    c.extend_from_slice(&[0x85, 0xC9]);
    let jnz_process = c.len();
    c.extend_from_slice(&[0x0F, 0x85, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x85, 0xD2]);
    let jnz_process2 = c.len();
    c.extend_from_slice(&[0x0F, 0x85, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_idone = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    let process_desc = c.len();
    patch_rel32(&mut c, jnz_process + 2, jnz_process + 6, process_desc);
    patch_rel32(&mut c, jnz_process2 + 2, jnz_process2 + 6, process_desc);
    // Bootstrap must have filled LL/GPA scratch — null call [r15+50h] AVs (fail-closed → exit 7).
    emit_cmp_r15_scratch_qword_zero(&mut c, H00_LOADLIBRARY_SCRATCH_OFF);
    fail_jumps.push((c.len(), fail_import));
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    emit_cmp_r15_scratch_qword_zero(&mut c, H00_GETPROCADDRESS_SCRATCH_OFF);
    fail_jumps.push((c.len(), fail_import));
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    emit_win64_call_shadow(&mut c);
    // FirstThunk rva is in edx — load module first, then lea r11 (LL/GPA clobber r11).
    c.extend_from_slice(&[0x49, 0x8D, 0x14, 0x0E]); // lea rdx,[r14+rcx] module name
    c.extend_from_slice(&[0x48, 0x89, 0xD1]); // mov rcx, rdx — LoadLibraryA(lpLibFileName)
    emit_call_r15_scratch(&mut c, H00_LOADLIBRARY_SCRATCH_OFF);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    emit_jz_pop_shadow_then_fail(&mut c, chunk_text_off as usize, fail_import);
    c.extend_from_slice(&[0x48, 0x89, 0xC7]); // rdi = hModule
    c.extend_from_slice(&[0x48, 0x89, 0x7C, 0x24, HMODULE_SPILL_OFF]); // [rsp+30h]=hModule
    c.extend_from_slice(&[0x49, 0x89, 0xF5]); // mov r13, rsi (save import descriptor ptr)
    c.extend_from_slice(&[0x41, 0x8B, 0x55, 0x10]); // mov edx,[r13+10] FirstThunk RVA
    c.extend_from_slice(&[0x4D, 0x8D, 0x1C, 0x16]); // lea r11,[r14+rdx] IAT write cursor
    c.extend_from_slice(&[0x85, 0xDB]); // cmp ebx,0 (OriginalFirstThunk)
    let jz_iat_read = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x8D, 0x34, 0x1E]); // lea rsi,[r14+rbx] read OFT
    let j_to_loop = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    let iat_read = c.len();
    patch_rel32(&mut c, jz_iat_read + 2, jz_iat_read + 6, iat_read);
    c.extend_from_slice(&[0x4C, 0x89, 0xDE]); // mov rsi, r11 (read IAT when no OFT) — NOT 49 89 DE (=mov r14,rbx)
    let thunk_loop = c.len();
    patch_rel32(&mut c, j_to_loop + 1, j_to_loop + 5, thunk_loop);
    c.extend_from_slice(&[0x48, 0x8B, 0x06]); // thunk
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_thunk_done = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x89, 0xC2]); // mov r10, rax (save thunk)
    c.extend_from_slice(&[0x48, 0x8B, 0x7C, 0x24, HMODULE_SPILL_OFF]); // mov rdi,[rsp+30h] hModule
    c.extend_from_slice(&[0x49, 0x0F, 0xBA, 0xE2, 0x3F]); // bt r10,63 (ordinal if high bit set)
    let jc_ord = c.len();
    c.extend_from_slice(&[0x0F, 0x82, 0, 0, 0, 0]); // jc ord_resolve
    c.extend_from_slice(&[0x4B, 0x8D, 0x54, 0x16, 0x02]); // lea rdx,[r14+r10+2] import name
    let call_thunk_by_name = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]); // call resolve_export (rdi=hModule)
    let jmp_thunk_resolved = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    let ord_resolve = c.len();
    patch_rel32(&mut c, jc_ord + 2, jc_ord + 6, ord_resolve);
    c.extend_from_slice(&[0x44, 0x89, 0xD0]); // mov eax, r10d
    c.extend_from_slice(&[0x25, 0xFF, 0xFF, 0x00, 0x00]); // and eax, 0xffff — ordinal
    // resolve_export_ordinal clobbers r11 — spill above Win64 home slots.
    c.extend_from_slice(&[0x4C, 0x89, 0x5C, 0x24, IMPORT_IAT_CURSOR_SPILL_OFF]); // mov [rsp+28h], r11
    let call_thunk_by_ord = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]); // call resolve_export_ordinal
    c.extend_from_slice(&[0x4C, 0x8B, 0x5C, 0x24, IMPORT_IAT_CURSOR_SPILL_OFF]); // mov r11, [rsp+28h]
    let thunk_resolved = c.len();
    patch_rel32(
        &mut c,
        jmp_thunk_resolved + 1,
        jmp_thunk_resolved + 5,
        thunk_resolved,
    );
    c.extend_from_slice(&[0x48, 0x85, 0xC0]); // resolve failed → fail_import
    emit_jz_pop_shadow_then_fail(&mut c, chunk_text_off as usize, fail_import);
    c.extend_from_slice(&[0x49, 0x89, 0x03]); // mov [r11], rax (IAT slot)
    c.extend_from_slice(&[0x48, 0x83, 0xC6, 0x08]);
    c.extend_from_slice(&[0x49, 0x83, 0xC3, 0x08]);
    let jmp_thunk = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_thunk + 1, jmp_thunk + 5, thunk_loop);
    let thunk_done = c.len();
    patch_rel32(&mut c, jz_thunk_done + 2, jz_thunk_done + 6, thunk_done);
    emit_win64_pop_shadow(&mut c);
    c.extend_from_slice(&[0x49, 0x83, 0xC5, 0x14]); // add r13, 20 (next IMAGE_IMPORT_DESCRIPTOR)
    c.extend_from_slice(&[0x4C, 0x89, 0xEE]); // mov rsi, r13
    let jmp_id = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_id + 1, jmp_id + 5, import_desc);
    let import_done = c.len();
    patch_rel32(&mut c, jz_import_done + 2, jz_import_done + 6, import_done);
    patch_rel32(&mut c, jz_idone + 2, jz_idone + 6, import_done);
    // Bootstrap exits: jmp over failure pop into import walk (not into skip_ll_boot_pop).
    patch_rel32(&mut c, jmp_boot_ok + 1, jmp_boot_ok + 5, import_walk_start);
    patch_rel32(&mut c, jmp_boot_fail + 1, jmp_boot_fail + 5, import_walk_start);
    patch_rel32(
        &mut c,
        jz_skip_ll_boot + 2,
        jz_skip_ll_boot + 6,
        import_done,
    );
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        iat_rva,
        PHASE_IMPORT_OK,
    );

    // FlushInstructionCache before calling mapped sidecar code (matches reference mapper).
    emit_mov_e_lfanew_pe_mapped(&mut c);
    emit_mov_u32_pe_mapped(&mut c, PE_OFF_SIZE_OF_IMAGE); // r8d = SizeOfImage (keep through GPA)
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        iat_rva,
        PHASE_FLUSH_ICACHE,
    );
    emit_win64_call_shadow(&mut c);
    emit_mov_qword_from_r15_scratch(&mut c, H00_KERNEL32_SCRATCH_OFF, 1); // rcx = kernel32
    for (off, ch) in b"FlushInstructionCache\0".iter().enumerate() {
        c.extend_from_slice(&[
            0xC6,
            0x44,
            0x24,
            FLUSH_ICACHE_NAME_STACK_OFF.wrapping_add(off as u8),
            *ch,
        ]);
    }
    c.extend_from_slice(&[
        0x48,
        0x8D,
        0x54,
        0x24,
        FLUSH_ICACHE_NAME_STACK_OFF,
    ]); // lea rdx,[rsp+30h] — above shadow; sub rsp,30h clobbered Win64 home space
    emit_call_r15_scratch(&mut c, H00_GETPROCADDRESS_SCRATCH_OFF); // rax = FlushICache
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_skip_flush2 = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    // rcx=-1, rdx=r14; reload r8d=SizeOfImage (GPA clobbers volatile r8).
    emit_mov_u32_pe_mapped(&mut c, PE_OFF_SIZE_OF_IMAGE);
    c.extend_from_slice(&[0x48, 0xC7, 0xC1, 0xFF, 0xFF, 0xFF, 0xFF]); // GetCurrentProcess()
    c.extend_from_slice(&[0x4C, 0x89, 0xF2]); // mov rdx, r14 (FlushInstructionCache lpBaseAddress)
    c.extend_from_slice(&[0xFF, 0xD0]); // call rax (FlushInstructionCache)
    emit_win64_pop_shadow(&mut c);
    let skip_flush = c.len();
    patch_rel32(&mut c, jz_skip_flush2 + 2, jz_skip_flush2 + 6, skip_flush);

    // Skip DllMain: CRT/TLS entry AVs on manual-mapped image; smoke probe uses kernel32 IAT only.
    // rbx = mapped image (r14)
    c.extend_from_slice(&[0x4C, 0x89, 0xF3]); // mov rbx, r14
    // Success path must skip inline helpers (find_module/resolve_export); export tail follows map body.
    let jmp_over_helpers = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);

    // --- Internal helpers (placed after main path; reached only via call) ---
    let find_module = c.len();
    patch_rel32(&mut c, call_boot_find + 1, call_boot_find + 5, find_module);
    // rdx = ascii dll name → rax = DllBase
    // x64: gs:[0x60] = PEB; Ldr.InMemoryOrderModuleList.Flink (+0x20); entry = Flink-0x10.
    c.extend_from_slice(&[0x41, 0x52]); // push r10 — list head
    c.extend_from_slice(&[0x65, 0x48, 0x8B, 0x04, 0x25, 0x60, 0x00, 0x00, 0x00]);
    c.extend_from_slice(&[0x48, 0x8B, 0x40, 0x18]); // mov rax,[rax+18h] PEB->Ldr
    c.extend_from_slice(&[0x4C, 0x8D, 0x50, 0x20]); // lea r10,[rax+20h] list head — NOT 49 8D 50 (=lea rdx,[r8+20h])
    c.extend_from_slice(&[0x48, 0x8B, 0x40, 0x20]); // InMemoryOrderModuleList.Flink
    let mod_loop = c.len();
    c.extend_from_slice(&[0x4C, 0x39, 0xD0]); // cmp rax,r10 (back at list head?)
    let je_no_mod = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x83, 0xE8, LDR_INMEMORY_FLINK_OFF]); // entry = Flink - 0x10
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_no_mod = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x4C, 0x8B, 0x40, LDR_BASEDLLNAME_BUF_OFF]); // BaseDllName.Buffer
    c.extend_from_slice(&[0x4D, 0x85, 0xC0]);
    let jz_next_mod = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    // compare rdx (ascii) with r8 (unicode) case-insensitive
    c.extend_from_slice(&[0x49, 0x89, 0xD1]); // mov r9, rdx (save ascii)
    let cmp_dll = c.len();
    c.extend_from_slice(&[0x41, 0x0F, 0xB7, 0x00]); // movzx eax,word [r8]
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_ascii_chk = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x41, 0x0F, 0xB6, 0x09]); // movzx ecx,byte [r9] (preserve r10=list head)
    c.extend_from_slice(&[0x85, 0xC9]); // test ecx,ecx
    let jz_dll_mismatch = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    // tolower eax and cl, compare
    c.extend_from_slice(&[0x3C, 0x41]); // cmp al,'A'
    c.extend_from_slice(&[0x72, 0x04]);
    c.extend_from_slice(&[0x3C, 0x5A]);
    c.extend_from_slice(&[0x77, 0x04]);
    c.extend_from_slice(&[0x0C, 0x20]); // or al,0x20
    c.extend_from_slice(&[0x80, 0xF9, 0x41]); // cmp cl,'A'
    c.extend_from_slice(&[0x72, 0x04]);
    c.extend_from_slice(&[0x80, 0xF9, 0x5A]); // cmp cl,'Z'
    c.extend_from_slice(&[0x77, 0x04]);
    c.extend_from_slice(&[0x80, 0xC9, 0x20]); // or cl,0x20
    c.extend_from_slice(&[0x38, 0xC8]); // cmp al,cl
    let jne_dll = c.len();
    c.extend_from_slice(&[0x0F, 0x85, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x83, 0xC0, 0x02]); // add r8,2 — UTF-16 BaseDllName (not inc r8)
    c.extend_from_slice(&[0x49, 0xFF, 0xC1]);
    let jmp_cmp_dll = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_cmp_dll + 1, jmp_cmp_dll + 5, cmp_dll);
    let ascii_chk = c.len();
    patch_rel32(&mut c, jz_ascii_chk + 2, jz_ascii_chk + 6, ascii_chk);
    c.extend_from_slice(&[0x41, 0x80, 0x39, 0x00]);
    let jz_mod_found = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    let dll_mismatch = c.len();
    patch_rel32(&mut c, jz_dll_mismatch + 2, jz_dll_mismatch + 6, dll_mismatch);
    patch_rel32(&mut c, jne_dll + 2, jne_dll + 6, dll_mismatch);
    c.extend_from_slice(&[0x48, 0x8B, 0x40, LDR_INMEMORY_FLINK_OFF]); // next Flink
    let jmp_mod = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_mod + 1, jmp_mod + 5, mod_loop);
    let no_mod = c.len();
    patch_rel32(&mut c, je_no_mod + 2, je_no_mod + 6, no_mod);
    patch_rel32(&mut c, jz_no_mod + 2, jz_no_mod + 6, no_mod);
    patch_rel32(&mut c, jz_next_mod + 2, jz_next_mod + 6, dll_mismatch);
    // LoadLibraryA fallback when PEB walk misses (api-set / forwarder targets).
    emit_mov_qword_from_r15_scratch(&mut c, H00_LOADLIBRARY_SCRATCH_OFF, 0); // mov rax,[r15+scratch]
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_ll_fail = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xD1]); // mov rcx, rdx (dll name)
    c.extend_from_slice(&[0x48, 0x83, 0xEC, FIND_MODULE_LL_SHADOW]); // after push r10 → RSP%16==0 at CALL
    c.extend_from_slice(&[0xFF, 0xD0]); // call rax (LoadLibraryA)
    c.extend_from_slice(&[0x48, 0x83, 0xC4, FIND_MODULE_LL_SHADOW]);
    c.extend_from_slice(&[0x41, 0x5A]); // pop r10
    c.extend_from_slice(&[0xC3]);
    let ll_fail = c.len();
    patch_rel32(&mut c, jz_ll_fail + 2, jz_ll_fail + 6, ll_fail);
    c.extend_from_slice(&[0x31, 0xC0]);
    c.extend_from_slice(&[0x41, 0x5A]); // pop r10
    c.extend_from_slice(&[0xC3]);
    let mod_found = c.len();
    patch_rel32(&mut c, jz_mod_found + 2, jz_mod_found + 6, mod_found);
    c.extend_from_slice(&[0x48, 0x8B, 0x40, LDR_DLLBASE_OFF]); // DllBase
    c.extend_from_slice(&[0x41, 0x5A]); // pop r10
    c.extend_from_slice(&[0xC3]);

    let resolve_export = c.len();
    patch_rel32(
        &mut c,
        call_boot_resolve + 1,
        call_boot_resolve + 5,
        resolve_export,
    );
    patch_rel32(
        &mut c,
        call_boot_gpa + 1,
        call_boot_gpa + 5,
        resolve_export,
    );
    patch_rel32(
        &mut c,
        call_thunk_by_name + 1,
        call_thunk_by_name + 5,
        resolve_export,
    );
    // rdi=module, rdx=name → rax=func (preserve thunk rsi; rbx/rcx/rdx = PE tables; r11=IAT cursor)
    c.extend_from_slice(&[0x41, 0x53]); // push r11 — caller IAT cursor survives resolve_export
    c.extend_from_slice(&[0x49, 0x89, 0xD1]); // mov r9, rdx — save import name before table walk
    c.extend_from_slice(&[0x8B, 0x47, 0x3C]); // eax = e_lfanew
    // mov eax,[rdi+rax+88h] — SIB 07 (index=rax, base=rdi); 27=[rdi+disp] (index suppressed); 38=[rax+rsi]
    c.extend_from_slice(&[0x8B, 0x84, 0x07, 0x88, 0x00, 0x00, 0x00]);
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_no_exp = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x01, 0xF8]);
    c.extend_from_slice(&[0x44, 0x8B, 0x40, 0x18]); // num names
    c.extend_from_slice(&[0x45, 0x85, 0xC0]);
    let jz_no_exp2 = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x8B, 0x58, 0x20]); // mov ebx,[rax+20] AddressOfNames
    c.extend_from_slice(&[0x48, 0x01, 0xFB]); // add rbx, rdi
    c.extend_from_slice(&[0x8B, 0x48, 0x24]); // mov ecx,[rax+24] AddressOfNameOrdinals
    c.extend_from_slice(&[0x48, 0x01, 0xF9]); // add rcx, rdi
    c.extend_from_slice(&[0x8B, 0x50, 0x1C]); // mov edx,[rax+1c] AddressOfFunctions
    c.extend_from_slice(&[0x48, 0x01, 0xFA]); // add rdx, rdi
    c.extend_from_slice(&[0x45, 0x31, 0xD2]); // xor r10d,r10d — export index (keep rsi=thunk ptr)
    let exp_loop = c.len();
    c.extend_from_slice(&[0x45, 0x39, 0xC2]); // cmp r10d,r8d
    let jae_no_exp = c.len();
    c.extend_from_slice(&[0x0F, 0x83, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x42, 0x8B, 0x04, 0x93]); // mov eax,[rbx+r10*4] (REX.X; SIB 93=scale*4 index=r10 base=rbx)
    c.extend_from_slice(&[0x48, 0x01, 0xF8]);
    // strcmp r9 (import) with [rax] (export) — do not clobber rbx=AddressOfNames
    let cmp_name = c.len();
    c.extend_from_slice(&[0x0F, 0xB6, 0x08]); // movzx eax, byte [rax] export name
    c.extend_from_slice(&[0x41, 0x38, 0x01]); // cmp byte ptr [r9], al
    let jne_name = c.len();
    c.extend_from_slice(&[0x0F, 0x85, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x84, 0xC0]); // test al, al (export char — both strings ended)
    let jz_name_found = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0xFF, 0xC0]);
    c.extend_from_slice(&[0x49, 0xFF, 0xC1]);
    let jmp_cmp = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_cmp + 1, jmp_cmp + 5, cmp_name);
    let name_found = c.len();
    patch_rel32(&mut c, jz_name_found + 2, jz_name_found + 6, name_found);
    c.extend_from_slice(&[0x42, 0x0F, 0xB7, 0x04, 0x51]); // movzx eax,word [rcx+r10*2] (REX.X; SIB 51=scale*2 index=r10 base=rcx)
    c.extend_from_slice(&[0x8B, 0x04, 0x82]); // mov eax,[rdx+rax*4]
    c.extend_from_slice(&[0x48, 0x01, 0xF8]);
    let call_fixup_name = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x41, 0x5B]); // pop r11
    c.extend_from_slice(&[0xC3]);
    let next_name = c.len();
    patch_rel32(&mut c, jne_name + 2, jne_name + 6, next_name);
    c.extend_from_slice(&[0x41, 0xFF, 0xC2]); // inc r10d
    let jmp_exp = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_exp + 1, jmp_exp + 5, exp_loop);
    let no_exp = c.len();
    patch_rel32(&mut c, jz_no_exp + 2, jz_no_exp + 6, no_exp);
    patch_rel32(&mut c, jz_no_exp2 + 2, jz_no_exp2 + 6, no_exp);
    patch_rel32(&mut c, jae_no_exp + 2, jae_no_exp + 6, no_exp);
    c.extend_from_slice(&[0x41, 0x5B]); // pop r11
    c.extend_from_slice(&[0x31, 0xC0]);
    c.extend_from_slice(&[0xC3]);

    let resolve_export_ordinal = c.len();
    patch_rel32(
        &mut c,
        call_thunk_by_ord + 1,
        call_thunk_by_ord + 5,
        resolve_export_ordinal,
    );
    c.extend_from_slice(&[0x89, 0xC1]); // mov ecx, eax (ordinal)
    c.extend_from_slice(&[0x8B, 0x47, 0x3C]); // eax = e_lfanew
    c.extend_from_slice(&[0x8B, 0x84, 0x07, 0x88, 0x00, 0x00, 0x00]); // [rdi+rax+88h] export dir RVA
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_no_ord = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x01, 0xF8]);
    c.extend_from_slice(&[0x8B, 0x50, 0x1C]);
    c.extend_from_slice(&[0x48, 0x01, 0xFA]);
    c.extend_from_slice(&[0x44, 0x8B, 0x48, 0x10]); // mov r9d,[rax+10] BaseOrdinal
    c.extend_from_slice(&[0x44, 0x29, 0xC9]); // sub ecx, r9d (ordinal index)
    c.extend_from_slice(&[0x8B, 0x04, 0x8A]);
    c.extend_from_slice(&[0x48, 0x01, 0xF8]);
    let call_fixup_ord = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    c.extend_from_slice(&[0xC3]);
    patch_rel32(&mut c, jz_no_ord + 2, jz_no_ord + 6, no_exp);

    // KERNEL32 forwarders land in export dir as "OTHERDLL.name" — recurse resolve.
    let fix_forward = c.len();
    patch_rel32(&mut c, call_fixup_name + 1, call_fixup_name + 5, fix_forward);
    patch_rel32(&mut c, call_fixup_ord + 1, call_fixup_ord + 5, fix_forward);
    // rdi=module, rax=export va → rax=code va; preserves rsi
    c.extend_from_slice(&[0x56]); // push rsi
    c.extend_from_slice(&[0x8B, 0x77, 0x3C]); // mov esi,[rdi+3c]
    c.extend_from_slice(&[0x8B, 0x8C, 0x37, 0x88, 0x00, 0x00, 0x00]); // export rva
    c.extend_from_slice(&[0x85, 0xC9]);
    let jz_ff_ret = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x8D, 0x14, 0x0F]); // lea rdx,[rdi+rcx] exp start
    c.extend_from_slice(&[0x49, 0x89, 0xD2]); // mov r10, rdx
    c.extend_from_slice(&[0x8B, 0x94, 0x37, 0x8C, 0x00, 0x00, 0x00]); // export size
    c.extend_from_slice(&[0x89, 0xD3]); // mov ebx, edx (export dir size)
    c.extend_from_slice(&[0x4C, 0x01, 0xD3]); // add rbx, r10 → export dir end (r10=start; rdx=size)
    c.extend_from_slice(&[0x4C, 0x39, 0xD0]); // cmp rax,r10
    let jb_ff_ret = c.len();
    c.extend_from_slice(&[0x0F, 0x82, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x39, 0xD8]); // cmp rax,rbx
    let jae_ff_ret = c.len();
    c.extend_from_slice(&[0x0F, 0x83, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x89, 0xC1]); // mov r9, rax (forwarder "DLL.name")
    c.extend_from_slice(&[0x49, 0x89, 0xC0]); // mov r8, rax (save dll name start)
    let ff_scan = c.len();
    c.extend_from_slice(&[0x41, 0x80, 0x39, 0x2E]); // cmp byte [r9], '.'
    let jz_ff_dot = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x41, 0x80, 0x39, 0x00]);
    let jz_ff_bad = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0xFF, 0xC1]); // inc r9
    let jmp_ff_scan = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_ff_scan + 1, jmp_ff_scan + 5, ff_scan);
    let ff_dot = c.len();
    patch_rel32(&mut c, jz_ff_dot + 2, jz_ff_dot + 6, ff_dot);
    // Copy forwarder DLL prefix [r8,r9) to stack — never patch host .rdata forwarders.
    c.extend_from_slice(&[0x48, 0x83, 0xEC, FORWARDER_NAME_FRAME]); // 0 mod 16 → find_module entry aligned
    c.extend_from_slice(&[0x48, 0x89, 0xE7]); // mov rdi, rsp
    c.extend_from_slice(&[0x4C, 0x89, 0xC6]); // mov rsi, r8
    let ff_copy = c.len();
    c.extend_from_slice(&[0x4C, 0x39, 0xCE]); // cmp rsi, r9
    let jae_ff_copy_done = c.len();
    c.extend_from_slice(&[0x0F, 0x83, 0, 0, 0, 0]); // jae copy_done
    c.extend_from_slice(&[0x8A, 0x06]); // mov al, [rsi]
    c.extend_from_slice(&[0x88, 0x07]); // mov [rdi], al
    c.extend_from_slice(&[0x48, 0xFF, 0xC6]); // inc rsi
    c.extend_from_slice(&[0x48, 0xFF, 0xC7]); // inc rdi
    let jmp_ff_copy = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_ff_copy + 1, jmp_ff_copy + 5, ff_copy);
    let ff_copy_done = c.len();
    patch_rel32(&mut c, jae_ff_copy_done + 2, jae_ff_copy_done + 6, ff_copy_done);
    c.extend_from_slice(&[0xC6, 0x07, 0x00]); // mov byte [rdi], 0
    c.extend_from_slice(&[0x48, 0x89, 0xE2]); // mov rdx, rsp (dll name)
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x08]); // RSP%16==0 at CALL (fix_forward frame leaves 8 mod 16)
    let call_ff_find = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x08]);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_ff_bad2 = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xC7]); // mov rdi, rax (target module)
    c.extend_from_slice(&[0x49, 0x8D, 0x51, 0x01]); // lea rdx,[r9+1] func name
    let call_ff_resolve = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x83, 0xC4, FORWARDER_NAME_FRAME]);
    let ff_ret = c.len();
    c.extend_from_slice(&[0x5E]); // pop rsi
    c.extend_from_slice(&[0xC3]);
    let ff_ret_pop = c.len();
    c.extend_from_slice(&[0x48, 0x83, 0xC4, FORWARDER_NAME_FRAME]); // find_module failed
    let jmp_ff_ret = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]); // jmp ff_ret
    patch_rel32(&mut c, jz_ff_bad2 + 2, jz_ff_bad2 + 6, ff_ret_pop);
    patch_rel32(&mut c, jmp_ff_ret + 1, jmp_ff_ret + 5, ff_ret);
    patch_rel32(&mut c, call_ff_find + 1, call_ff_find + 5, find_module);
    patch_rel32(&mut c, call_ff_resolve + 1, call_ff_resolve + 5, resolve_export);
    patch_rel32(&mut c, jz_ff_ret + 2, jz_ff_ret + 6, ff_ret);
    patch_rel32(&mut c, jb_ff_ret + 2, jb_ff_ret + 6, ff_ret);
    patch_rel32(&mut c, jae_ff_ret + 2, jae_ff_ret + 6, ff_ret);
    patch_rel32(&mut c, jz_ff_bad + 2, jz_ff_bad + 6, ff_ret);

    let helpers_end = c.len();
    patch_rel32(&mut c, jmp_over_helpers + 1, jmp_over_helpers + 5, helpers_end);

    for (at, fail) in fail_jumps {
        patch_rel32(
            &mut c,
            at + 2,
            chunk_text_off as usize + at + 6,
            fail,
        );
    }

    c
}

/// Export ordinal-0 tail: rbx=module → call functions[0] → ExitProcess.
fn gen_h00_export_call_tail(
    meta: &SelfhostMeta,
    text_rva: u32,
    chunk_text_off: u32,
    fail_export: usize,
) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();
    let mut fail_jumps: Vec<(usize, usize)> = Vec::new();

    c.extend_from_slice(&[0x8B, 0x73, 0x3C]);
    c.extend_from_slice(&[0x8B, 0x84, 0x33, 0x88, 0x00, 0x00, 0x00]);
    c.extend_from_slice(&[0x85, 0xC0]);
    fail_jumps.push((c.len(), fail_export));
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x8D, 0x3C, 0x03]);
    c.extend_from_slice(&[0x8B, 0x47, 0x1C]);
    c.extend_from_slice(&[0x48, 0x01, 0xD8]); // add rax, rbx — functions RVA is image-relative
    c.extend_from_slice(&[0x8B, 0x00]);
    c.extend_from_slice(&[0x48, 0x01, 0xD8]);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    fail_jumps.push((c.len(), fail_export));
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    emit_phase_with_bisect(
        &mut c,
        text_rva,
        chunk_text_off,
        meta.iat_rva,
        PHASE_EXPORT_CALL,
    );
    emit_win64_call_shadow(&mut c);
    c.extend_from_slice(&[0xFF, 0xD0]); // call export (yoyo_runtime_selfhost_main)
    emit_win64_pop_shadow(&mut c);
    c.extend_from_slice(&[0x89, 0xC1]); // mov ecx, eax — export return → ExitProcess
    emit_win64_call_shadow(&mut c);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_EXIT_PROCESS);

    for (at, fail) in fail_jumps {
        patch_rel32(
            &mut c,
            at + 2,
            chunk_text_off as usize + at + 6,
            fail,
        );
    }
    c
}

const FAIL_EXIT_CODES: [u8; 8] = [2, 3, 4, 5, 6, 7, 8, 9];

fn emit_phase_fail_epilogues(
    c: &mut Vec<u8>,
    text_rva: u32,
    code_base_off: u32,
    iat_rva: u32,
) -> [usize; 8] {
    let mut labels = [0usize; 8];
    for (i, &code) in FAIL_EXIT_CODES.iter().enumerate() {
        labels[i] = code_base_off as usize + c.len();
        emit_exit_process_iat(c, text_rva, code_base_off, iat_rva, code);
    }
    labels
}

/// Full H_00 manual-map stub: file read + manual map + export call + fail epilogue.
pub fn gen_h00_manual_map_main(
    meta: &SelfhostMeta,
    text_rva: u32,
    code_base_off: u32,
) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();

    c.extend_from_slice(&[0x53, 0x41, 0x54, 0x41, 0x55, 0x41, 0x56]);
    // User .text may clobber r15 before the jmp into H_00; reload before any [r15+scratch] probe.
    emit_reload_r15_data_base(&mut c, text_rva, code_base_off, meta.iat_rva);

    emit_phase_with_bisect(
        &mut c,
        text_rva,
        code_base_off,
        meta.iat_rva,
        PHASE_H00_ENTERED,
    );

    let prelude_text_off = code_base_off + c.len() as u32;

    const EPILOGUE_LEN: usize = 8 * FAIL_EPILOGUE_LEN;

    // Size pass with placeholder fail labels.
    let prelude_len = gen_h00_read_sidecar_prelude(
        meta,
        text_rva,
        prelude_text_off,
        usize::MAX,
        usize::MAX,
        usize::MAX,
    )
    .len();
    let map_text_off_m = prelude_text_off + prelude_len as u32;
    let map_len = gen_h00_manual_map_body(
        text_rva,
        map_text_off_m,
        meta.iat_rva,
        usize::MAX,
        usize::MAX,
    )
    .len();
    let tail_text_off_m = map_text_off_m + map_len as u32;
    let tail_len =
        gen_h00_export_call_tail(meta, text_rva, tail_text_off_m, usize::MAX).len();
    let epilogue_base = code_base_off as usize + c.len() + prelude_len + map_len + tail_len;

    let fail_label = |i: usize| epilogue_base + i * FAIL_EPILOGUE_LEN;
    let fail_create_file = fail_label(0); // ExitProcess(2)
    let fail_read_empty = fail_label(1); // ExitProcess(3)
    let fail_virtual_alloc = fail_label(2); // ExitProcess(4)
    let _fail_section_copy = fail_label(3); // ExitProcess(5)
    let _fail_reloc = fail_label(4); // ExitProcess(6)
    let fail_import = fail_label(5); // ExitProcess(7)
    let fail_export = fail_label(6); // ExitProcess(8)

    c.extend_from_slice(&gen_h00_read_sidecar_prelude(
        meta,
        text_rva,
        prelude_text_off,
        fail_create_file,
        fail_read_empty,
        fail_virtual_alloc,
    ));
    let map_text_off = code_base_off + c.len() as u32;
    c.extend_from_slice(&gen_h00_manual_map_body(
        text_rva,
        map_text_off,
        meta.iat_rva,
        fail_virtual_alloc,
        fail_import,
    ));
    let tail_text_off = code_base_off + c.len() as u32;
    c.extend_from_slice(&gen_h00_export_call_tail(
        meta,
        text_rva,
        tail_text_off,
        fail_export,
    ));

    let _ = emit_phase_fail_epilogues(&mut c, text_rva, code_base_off, meta.iat_rva);
    let _ = EPILOGUE_LEN; // keep size estimate stable for readers
    c
}

/// Estimated total H_00 stub span (for gate pins).
pub fn estimate_manual_map_stub_span(file_read_len: usize) -> usize {
    const MANUAL_MAP_BODY_EST: usize = 360;
    file_read_len + MANUAL_MAP_BODY_EST + 40
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::win32_selfhost::SelfhostMeta;

    fn sample_meta() -> SelfhostMeta {
        SelfhostMeta {
            temp_name_rva: 0x30_000,
            export_name_rva: 0,
            dll_embed_rva: 0,
            dll_embed_size: 0,
            iat_rva: 0x20_000,
            import_dir_rva: 0,
            import_dir_size: 0,
        }
    }

    #[test]
    fn read_sidecar_prelude_nonempty_and_bounded() {
        let meta = sample_meta();
        let body = gen_h00_read_sidecar_prelude(&meta, 0x1000, 17_823, 0x50_000, 0x50_020, 0x50_010);
        assert!(body.len() > 80, "prelude should be substantial");
        assert!(
            body.len() < 320,
            "file-read prelude should stay <320B (got {}B)",
            body.len()
        );
        assert!(
            body.windows(4)
                .filter(|w| **w == [0x48, 0x83, 0xEC, PRELUDE_IO_FRAME])
                .count()
                >= 1,
            "prelude needs unified I/O stack frame (sub rsp,38h)"
        );
        assert!(
            body.windows(2)
                .filter(|w| **w == [0xFF, 0x15])
                .count()
                >= 4,
            "prelude kernel32 I/O must use rip-relative IAT (FF 15)"
        );
    }

    #[test]
    fn fail_jump_create_file_lands_on_exit2_epilogue() {
        let meta = sample_meta();
        let body = gen_h00_manual_map_main(&meta, 0x1000, 17_823);
        let cmp_pat = [0x48u8, 0x83, 0xF8, 0xFF];
        let cmp_off = body
            .windows(cmp_pat.len())
            .position(|w| w == cmp_pat)
            .expect("CreateFile cmp rax,-1");
        let after_cmp = &body[cmp_off..cmp_off + 24];
        assert!(
            after_cmp.windows(4).any(|w| w == [0x48, 0x83, 0xC4, PRELUDE_IO_FRAME]),
            "CreateFile fail path must pop prelude I/O frame before ExitProcess(2)"
        );
        assert!(
            body.windows(23).any(|w| {
                w[0..3] == [0x4C, 0x8D, 0x3D]
                    && w[7..11] == [0x48, 0x83, 0xEC, WIN64_CALL_SHADOW]
                    && w[11..16] == [0xB9, 0x02, 0x00, 0x00, 0x00]
                    && w[16..19] == [0x41, 0xFF, 0x97]
            }),
            "ExitProcess(2) fail epilogue present"
        );
    }

    #[test]
    fn read_file_fail_pops_shadow_before_exit3() {
        let meta = sample_meta();
        let body = gen_h00_manual_map_main(&meta, 0x1000, 17_823);
        let mut found = false;
        for i in 0..body.len().saturating_sub(14) {
            // test eax,eax ; jnz success ; add rsp,imm8 ; jmp fail
            if body[i..i + 2] != [0x85, 0xC0] || body[i + 2..i + 4] != [0x0F, 0x85] {
                continue;
            }
            for j in i + 4..(i + 20).min(body.len().saturating_sub(5)) {
                if body[j..j + 4] == [0x48, 0x83, 0xC4, PRELUDE_IO_FRAME] && body[j + 4] == 0xE9 {
                    let rel = i32::from_le_bytes(body[i + 4..i + 8].try_into().unwrap());
                    assert!(
                        rel > 0,
                        "ReadFile success jnz must skip fail trampoline (rel={rel})"
                    );
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        assert!(
            found,
            "ReadFile fail must jnz-skip trampoline that pops prelude I/O frame then jmp ExitProcess(3)"
        );
    }

    #[test]
    fn fail_epilogue_labels_are_non_overlapping() {
        let meta = sample_meta();
        let body = gen_h00_manual_map_main(&meta, 0x1000, 17_823);
        let mut starts = Vec::new();
        for i in 0..body.len() {
            if i + 23 > body.len() {
                break;
            }
            let w = &body[i..i + 23];
            if w[0..3] == [0x4C, 0x8D, 0x3D]
                && w[7..11] == [0x48, 0x83, 0xEC, WIN64_CALL_SHADOW]
                && w[11] == 0xB9
                && w[16..19] == [0x41, 0xFF, 0x97]
            {
                starts.push(i);
            }
        }
        assert_eq!(
            starts.len(),
            8,
            "expected 8 fail epilogues (reload + shadow + mov ecx + call [r15+ExitProcess])"
        );
        for w in starts.windows(2) {
            assert_eq!(
                w[1] - w[0],
                FAIL_EPILOGUE_LEN,
                "fail epilogues must be spaced {}B (got {}B)",
                FAIL_EPILOGUE_LEN,
                w[1] - w[0]
            );
        }
    }

    #[test]
    fn manual_map_main_pinned_span() {
        let meta = sample_meta();
        let body = gen_h00_manual_map_main(&meta, 0x1000, 17_823);
        eprintln!("manual_map_stub_len={}", body.len());
        if std::env::var("DUMP_H00_MANUAL_MAP_HEX").is_ok() {
            let hex: String = body.iter().map(|b| format!("{:02x}", b)).collect();
            eprintln!("H00_MANUAL_MAP_HEX={hex}");
            for i in 0..body.len().saturating_sub(1) {
                if body[i] == 0xFF && body[i + 1] == 0x15 {
                    eprintln!("H00_IAT_CALL at={i}");
                }
            }
            if body.len() >= 7 {
                for i in 0..body.len().saturating_sub(6) {
                    if body[i] == 0x48 && body[i + 1] == 0x8D && body[i + 2] == 0x0D {
                        eprintln!("H00_LEA_RIP at={i}");
                    }
                }
            }
        }
        assert!(
            body.len() > 400 && body.len() < 2350,
            "manual-map H_00 stub should fit OW-STUB pin [40,2350] (got {}B)",
            body.len()
        );
        assert_eq!(
            &body[0..7],
            &[0x53, 0x41, 0x54, 0x41, 0x55, 0x41, 0x56],
            "H_00 prologue must save rbx/r12/r13/r14"
        );
        assert_eq!(
            &body[7..10],
            &[0x4C, 0x8D, 0x3D],
            "H_00 prologue must reload r15 via lea r15,[rip+disp] (no and rsp,-16)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x48, 0x83, 0xE4, 0xF0]),
            "must not and rsp,-16 — pairs with 0x40 shadow → 8-mod-16 at CALL (movaps AV)"
        );
        assert!(
            !body.windows(7).any(|w| w == [0x48, 0x81, 0xEC, 0x00, 0x02, 0x00, 0x00]),
            "must not sub rsp,0x200 — misaligns CALL after JMP entry (kernel32 movaps AV)"
        );
        assert!(
            !body.windows(7).any(|w| w == [0x48, 0x81, 0xEC, 0x08, 0x02, 0x00, 0x00]),
            "must not sub rsp,0x208 — misaligns CALL after JMP entry (kernel32 movaps AV)"
        );
        assert_eq!(
            PRELUDE_IO_FRAME % 16,
            8,
            "prelude I/O frame must be 8 mod 16 for aligned kernel32 FF15 after JMP entry"
        );
        for i in 0..body.len().saturating_sub(10) {
            if body[i..i + 4] == [0x48, 0x83, 0xEC, 0x40]
                && body[i + 4] == 0xB9
                && body[i + 9] == 0xFF
                && body[i + 10] == 0x15
            {
                panic!(
                    "ExitProcess epilogue/bisect at stub+{i} must sub rsp,38h not 40h (Win64 CALL align)"
                );
            }
            if body[i..i + 4] == [0x48, 0x83, 0xEC, 0x40]
                && body[i + 4] == 0xFF
                && body[i + 5] == 0x15
            {
                panic!(
                    "kernel32 IAT call at stub+{i} must sub rsp,38h not 40h (Win64 CALL align)"
                );
            }
        }
        // No un-prefixed [r12+rbx] PE reads (without REX.B they decode as [rsp+rbx]).
        for i in 0..body.len().saturating_sub(3) {
            let slice = &body[i..i + 3];
            let needs_rex = matches!(
                slice,
                [0x8B, 0x84, 0x1C]
                    | [0x8B, 0x8C, 0x1C]
                    | [0x8B, 0x94, 0x1C]
                    | [0x0F, 0xB7, 0x74]
                    | [0x0F, 0xB7, 0x84]
            );
            if needs_rex {
                let rex = body.get(i.wrapping_sub(1)).copied();
                assert!(
                    rex.map(|b| (0x40..=0x4F).contains(&b)).unwrap_or(false),
                    "missing REX on [r12+rbx] PE read at stub offset {i}"
                );
                // r12 base in SIB requires REX.B (e.g. 0x4D for mov r10,[r12+rbx+disp32]).
                assert!(
                    rex.unwrap() & 1 != 0,
                    "REX.B not set on [r12+rbx] PE read at stub offset {i}"
                );
            }
        }
        // No LoadLibraryA ROR13 hash needle (0x8E 0x4E 0x0E 0xEC)
        assert!(
            !body.windows(4).any(|w| w == [0x8E, 0x4E, 0x0E, 0xEC]),
            "stub must not embed LoadLibraryA hash"
        );
        // OFT==0 path must mov rsi,r11 (4C 89 DE), not mov r14,rbx (49 89 DE) which clobbers mapped base.
        assert!(
            body.windows(3).any(|w| w == [0x4C, 0x89, 0xDE]),
            "missing mov rsi,r11 for IAT-read fallback (OFT==0)"
        );
        assert!(
            !body.windows(3).any(|w| w == [0x49, 0x89, 0xDE]),
            "must not emit mov r14,rbx (49 89 DE) — destroys mapped image base r14"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x49, 0x8D, 0x50, 0x20]),
            "must not emit lea rdx,[r8+20h] (49 8D 50 20) — PEB list head needs lea r10,[rax+20h]"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x49, 0x8D, 0x1C, 0x0A]),
            "must not emit lea rbx,[r10+rcx] (49 8D 1C 0A) — forwarder end uses rdx size"
        );
        // resolve_export reads export dir via [rdi+rax+88h] (SIB 07); 38=[rax+rsi]; 27=[rdi+disp] only.
        assert!(
            !body.windows(7).any(|w| w == [0x8B, 0x84, 0x38, 0x88, 0x00, 0x00, 0x00]),
            "must not emit mov eax,[rax+rsi+88h] in resolve_export (bootstrap AV)"
        );
        assert!(
            !body.windows(7).any(|w| w == [0x8B, 0x84, 0x27, 0x88, 0x00, 0x00, 0x00]),
            "must not emit mov eax,[rdi+88h] (SIB 27 suppresses rax index)"
        );
        assert!(
            body.windows(7)
                .filter(|w| **w == [0x8B, 0x84, 0x07, 0x88, 0x00, 0x00, 0x00])
                .count()
                >= 2,
            "resolve_export + resolve_export_ordinal need mov eax,[rdi+rax+88h] (SIB 07)"
        );
        assert!(
            body.windows(7)
                .any(|w| *w == [0x8B, 0x84, 0x33, 0x88, 0x00, 0x00, 0x00]),
            "export tail needs mov eax,[rbx+rsi+88h] (SIB 33, esi=e_lfanew)"
        );
        assert!(
            !body.windows(7).any(|w| w == [0x49, 0x8D, 0x34, 0x06, 0x8B, 0x4E, 0x0C]),
            "bootstrap must not use sidecar import[0] name for find_module (use kernel32.dll stack)"
        );
        assert!(
            body.windows(5).any(|w| w == [0xC6, 0x44, 0x24, WIN64_STACK_STR_OFF, b'k']),
            "bootstrap must build kernel32.dll in Win64 shadow (C6 44 24 28 6B)"
        );
        assert!(
            body.windows(4).any(|w| w == [0x49, 0x83, 0xC0, 0x02]),
            "find_module must add r8,2 for UTF-16 BaseDllName (not inc r8)"
        );
        assert!(
            !body.windows(3).any(|w| w == [0x49, 0xFF, 0xC0]),
            "must not inc r8 in find_module (UTF-16 stride is 2 bytes)"
        );
        assert!(
            body.windows(8)
                .filter(|w| {
                    w[0..3] == [0x49, 0x83, 0xBF]
                        && w[3..7] == H00_LOADLIBRARY_SCRATCH_OFF.to_le_bytes()
                        && w[7] == 0x00
                })
                .count()
                >= 1,
            "import loop must cmp qword [r15+LoadLibraryA scratch] before call"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x49, 0x83, 0x7F, 0x50, 0x00]),
            "must not cmp [r15+50h] — collides with import descriptors"
        );
        // ImageBase for reloc delta: mov r10,[r12+rbx+30h] needs REX.W|R|B (4D 8B 94 1C).
        assert!(
            body.windows(4).any(|w| w == [0x4D, 0x8B, 0x94, 0x1C]),
            "missing mov r10,[r12+rbx] ImageBase read (4D 8B 94 1C)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x4F, 0x8B, 0x94, 0x1C]),
            "must not emit mov rdx,[r12+r11] (4F 8B 94 1C — REX.X=1 makes SIB index r11 not rbx)"
        );
        // REX.W+B without REX.R on [r14+rbx] reads SizeOfImage into rax — clobbers GPA result before call.
        assert!(
            !body.windows(4).any(|w| w == [0x4D, 0x8B, 0x84, 0x1E]),
            "must not emit mov rax,[r14+rbx] (4D 8B 84 1E) for SizeOfImage — need mov r8d (45 8B 44 1E)"
        );
        assert!(
            body.windows(5)
                .filter(|w| **w == [0x45, 0x8B, 0x44, 0x1E, PE_OFF_SIZE_OF_IMAGE])
                .count()
                >= 2,
            "FlushICache path needs mov r8d,[r14+rbx+50h] disp8 (45 8B 44 1E 50) twice"
        );
        assert!(
            body.windows(6).any(|w| {
                w[0..5] == [0x48, 0x8D, 0x54, 0x24, FLUSH_ICACHE_NAME_STACK_OFF]
                    && w[5] == 0x41
            }),
            "FlushICache GPA must lea rdx,[rsp+30h] then call [r15+GPA] (name above Win64 shadow)"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x48, 0x83, 0xEC, 0x30, 0xC6]),
            "must not sub rsp,30h before FlushICache GPA name (clobbers Win64 shadow → AV)"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x45, 0x8B, 0x84, 0x1E, PE_OFF_SIZE_OF_IMAGE]),
            "must not emit mod=10 disp32 mov r8d with 1-byte disp (45 8B 84 1E 50 misaligns stream)"
        );
        assert!(
            body.windows(4)
                .filter(|w| **w == [0x42, 0x8B, 0x04, 0x93])
                .count()
                >= 1,
            "resolve_export needs mov eax,[rbx+r10*4] (42 8B 04 93 REX.X)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x41, 0x8B, 0x04, 0x93]),
            "must not emit mov eax,[r11+rdx*4] (41 8B 04 93) — export name index is r10"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x49, 0x8B, 0x04, 0xA3]),
            "must not emit mov eax,[r11+disp] (49 8B 04 A3) — need REX.X + SIB 93"
        );
        assert!(
            body.windows(5)
                .any(|w| *w == [0x42, 0x0F, 0xB7, 0x04, 0x51]),
            "resolve_export needs movzx eax,word [rcx+r10*2] (42 0F B7 04 51)"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x41, 0x0F, 0xB7, 0x04, 0x51]),
            "must not emit movzx eax,word [rcx+rdx*2] (41 0F B7 04 51)"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x49, 0x0F, 0xB7, 0x04, 0x61]),
            "must not emit movzx eax,word [r9+disp] (49 0F B7 04 61) — need REX.X + SIB 51"
        );
        // Success-path phase probes (survive until crash) — rip-relative `.data` scratch.
        assert!(
            body.windows(8).any(|w| {
                w[0..2] == [0xC6, 0x05]
                    && w[6] == PHASE_FLUSH_ICACHE
            }),
            "missing FlushICache phase probe at [rip+phase scratch]"
        );
        // e_lfanew reads must hit PE base, not [rsp+rdi] (SIB 3C) or wrong reg.
        assert!(
            body.windows(5).any(|w| w == [0x41, 0x8B, 0x5C, SIB_R12_ONLY, 0x3C]),
            "missing mov ebx,[r12+3Ch] (file PE e_lfanew; SIB 24)"
        );
        assert!(
            body.windows(3).filter(|w| **w == [0x4C, 0x8D, 0x3D]).count() >= 3,
            "must reload r15 (lea r15,[rip+disp]) in prelude, bootstrap, and import walk"
        );
        // Bootstrap scratch zero must follow lea r15 — stale r15 after reloc corrupts .data (import AV).
        let reloc_ok = body
            .windows(7)
            .position(|w| w.len() == 7 && w[0] == 0xC6 && w[5] == 0x00 && w[6] == PHASE_RELOC_OK)
            .expect("PHASE_RELOC_OK probe");
        let scratch_zero = body
            .windows(7)
            .position(|w| {
                w[0..3] == [0x49, 0xC7, 0x87]
                    && w[3..7] == H00_LOADLIBRARY_SCRATCH_OFF.to_le_bytes()
            })
            .expect("mov qword [r15+LoadLibrary scratch],0");
        let lea_r15_before = body[reloc_ok..scratch_zero]
            .windows(3)
            .any(|w| *w == [0x4C, 0x8D, 0x3D]);
        assert!(
            lea_r15_before && scratch_zero > reloc_ok,
            "bootstrap must lea r15 before mov qword [r15+LoadLibrary scratch],0 (post-reloc)"
        );
        // import_walk_start must reload file e_lfanew after bootstrap resolve_export clobbers ebx.
        assert!(
            body.windows(10).any(|w| {
                w[0..5] == [0x41, 0x8B, 0x5C, SIB_R12_ONLY, 0x3C]
                    && w[5..10] == [0x41, 0x8B, 0x84, 0x1C, PE_OFF_IMPORT_DIR_RVA]
            }),
            "import walk must reload ebx from [r12+3Ch] before [r12+rbx+import_rva]"
        );
        assert!(
            body.windows(5).any(|w| w == [0x41, 0x8B, 0x5C, SIB_R14_ONLY, 0x3C]),
            "missing mov ebx,[r14+3Ch] (mapped e_lfanew; SIB 26 base=r14)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x41, 0x8B, 0x5E, 0x3C]),
            "must not emit mov ebx,r14 (41 8B 5E 3C mod=11) — NOT [r14+3Ch]"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x41, 0x8B, 0xDE, 0x3C]),
            "must not emit mov ebx,r14d (41 8B DE 3C mod=11) — NOT [r14+3Ch]"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x41, 0x8B, 0x5C, 0x3C, 0x3C]),
            "must not emit mov ebx,[rsp+rdi+disp] (SIB 3C = rsp+rdi, not r12/r14)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x41, 0x8B, 0x4E, 0x3C]),
            "must not emit mov ecx,[r14+3Ch] (4E=ecx — ebx stays clobbered after import loop)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x41, 0x8B, 0x7E, 0x3C]),
            "must not emit mov edi,[r14+3Ch] (7E=edi not ebx)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x49, 0x8D, 0x1C, 0x12]),
            "must not emit lea rbx,[rdx+rdx] (49 8D 1C 12)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x4C, 0x8D, 0x1C, 0x42]),
            "must not emit lea r11,[rdx+rax*2] (4C 8D 1C 42) — forwarder end is rdx+size"
        );
        assert!(
            body.windows(7).any(|w| {
                w[0..4] == [0x48, 0x83, 0xEC, FORWARDER_NAME_FRAME]
                    && w[4..7] == [0x48, 0x89, 0xE7]
            }),
            "fix_forward must sub rsp,40h for forwarder DLL-name copy frame"
        );
        assert!(
            body.windows(13).any(|w| {
                w[0..4] == [0x48, 0x83, 0xEC, 0x08]
                    && w[4] == 0xE8
                    && w[9..13] == [0x48, 0x83, 0xC4, 0x08]
            }),
            "fix_forward must sub/add rsp,8 around call find_module (Win64 CALL align)"
        );
        assert!(
            body.windows(6).any(|w| {
                w[0..4] == [0x48, 0x83, 0xEC, FIND_MODULE_LL_SHADOW]
                    && w[4] == 0xFF
                    && w[5] == 0xD0
            }),
            "find_module LL fallback needs sub rsp,30h before call rax (not 38h after push r10)"
        );
        assert!(
            body.windows(6).any(|w| {
                w[0..4] == [0x48, 0x83, 0xC4, FIND_MODULE_LL_SHADOW]
                    && w[4] == 0x41
                    && w[5] == 0x5A
            }),
            "find_module LL fallback needs add rsp,30h before pop r10"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x89, 0xD3, 0x48, 0x01, 0xD3]),
            "must not add rbx,rdx after rdx holds export dir size (breaks forwarder range)"
        );
        assert!(
            body.windows(4).any(|w| w == [0x4B, 0x8D, 0x34, 0x0C]),
            "section copy needs lea rsi,[r12+r9] (4B 8D 34 0C REX.W|X|B)"
        );
        assert!(
            body.windows(5).any(|w| w == [0x4B, 0x8D, 0x54, 0x16, 0x02]),
            "import name needs lea rdx,[r14+r10+2] (4B 8D 54 16 02 REX.W|X|B)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x4A, 0x8D, 0x34, 0x0C]),
            "must not emit lea rsi,[rsp+r9] (4A 8D 34 0C — missing REX.B for r12 base)"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x4A, 0x8D, 0x54, 0x16, 0x02]),
            "must not emit lea rdx,[rsi+r10+2] (4A 8D 54 16 02 — missing REX.B for r14 base)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x29, 0xC9]),
            "must not emit sub ecx,ecx (29 C9) in ordinal export path"
        );
        // Bootstrap success must jmp over failure pop — not fall through into add rsp; jmp add rsp loop.
        let gpa_store = body.windows(7).position(|w| {
            w[0..3] == [0x49, 0x89, 0x87] && w[3..7] == H00_GETPROCADDRESS_SCRATCH_OFF.to_le_bytes()
        });
        if let Some(at) = gpa_store {
            let tail = &body[at + 7..at + 7 + 12];
            assert_eq!(
                &tail[0..5],
                &[0x48, 0x83, 0xC4, WIN64_CALL_SHADOW, 0xE9][..],
                "after GPA bootstrap store expect add rsp,40h; jmp (skip failure pop)"
            );
            assert!(
                !body[at + 7..].windows(2).any(|w| w == [0xE9, 0xF7]),
                "bootstrap must not jmp back into skip_ll_boot_pop (infinite stack unwind)"
            );
        }
        assert!(
            body.windows(3).any(|w| w == [0x44, 0x29, 0xC9]),
            "resolve_export_ordinal needs sub ecx,r9d (44 29 C9)"
        );
        assert!(
            !body.windows(7).any(|w| w == [0x44, 0x8B, 0x48, 0x10, 0x44, 0x29, 0xC8]),
            "must not emit sub eax,r9d (44 29 C8) after BaseOrdinal — need sub ecx,r9d"
        );
        // Export call: Win64 shadow then `call export` / ExitProcess(FF15).
        let export_shadow = body
            .windows(4)
            .position(|w| w == [0x48, 0x83, 0xEC, WIN64_CALL_SHADOW]);
        assert!(
            export_shadow.is_some(),
            "export tail must sub rsp,38h before call (Win64 shadow)"
        );
        assert_eq!(
            WIN64_CALL_SHADOW % 16,
            8,
            "CALL shadow 0x38 after JMP entry → RSP%16==8 at kernel32 CALL"
        );
        // Fail epilogues: reload r15 + shadow + mov ecx + call [r15+ExitProcess]
        assert!(
            body.windows(23)
                .filter(|w| {
                    w[0..3] == [0x4C, 0x8D, 0x3D]
                        && w[7..11] == [0x48, 0x83, 0xEC, WIN64_CALL_SHADOW]
                        && w[11] == 0xB9
                        && w[16..19] == [0x41, 0xFF, 0x97]
                })
                .count()
                >= 8,
            "fail epilogues need reload r15 + Win64 shadow + call [r15+ExitProcess]"
        );
        assert!(
            body.windows(7).any(|w| {
                w[0..3] == [0x89, 0xC1, 0x48]
                    && w[3..7] == [0x83, 0xEC, WIN64_CALL_SHADOW, 0xFF]
            }),
            "export success path needs mov ecx,eax + Win64 shadow before ExitProcess"
        );
        // Import thunks: reload hModule once before bt; resolve_export clobbers rdi.
        assert!(
            body.windows(15).any(|w| {
                w[0..5] == [0x48, 0x8B, 0x7C, 0x24, HMODULE_SPILL_OFF]
                    && w[5..10] == [0x49, 0x0F, 0xBA, 0xE2, 0x3F]
            }),
            "import thunk needs mov rdi,[rsp+30h] before bt r10,63"
        );
        assert!(
            body.windows(6).any(|w| {
                w[0..5] == [0x4B, 0x8D, 0x54, 0x16, 0x02] && w[5] == 0xE8
            }),
            "import name thunk needs lea rdx,[r14+r10+2] then call resolve_export (E8)"
        );
        assert!(
            !body.windows(10).any(|w| {
                w[0..5] == [0x48, 0x8B, 0x7C, 0x24, HMODULE_SPILL_OFF]
                    && w[5..10] == [0x44, 0x89, 0xD0, 0x25, 0xFF]
            }),
            "import ordinal path must not reload rdi twice per thunk"
        );
        assert!(
            body.windows(14).any(|w| {
                w[0..10] == [0x44, 0x89, 0xD0, 0x25, 0xFF, 0xFF, 0x00, 0x00, 0x4C, 0x89]
                    && w[10..14] == [0x5C, 0x24, IMPORT_IAT_CURSOR_SPILL_OFF, 0xE8]
            }),
            "import ordinal thunk needs spill r11 at [rsp+28h] / call resolve_export_ordinal (E8)"
        );
        assert!(
            !body.windows(12).any(|w| {
                w[0..5] == [0x4C, 0x89, 0x5C, 0x24, IMPORT_IAT_CURSOR_SPILL_OFF]
                    && w[5..8] == [0x41, 0xFF, 0x97]
                    && w[8..12] == H00_GETPROCADDRESS_SCRATCH_OFF.to_le_bytes()
            }),
            "import loop must not call [r15+GPA] per thunk — use resolve_export"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x48, 0x89, 0x7C, 0x24, IMPORT_IAT_CURSOR_SPILL_OFF]),
            "hModule must not spill at [rsp+28h] — resolve_export clobbers home slot"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x41, 0x53, 0x41, 0xFF, 0x57]),
            "must not push r11 before call [r15+GPA] (41 53 41 FF 57) — misaligns Win64 shadow"
        );
    }
}
