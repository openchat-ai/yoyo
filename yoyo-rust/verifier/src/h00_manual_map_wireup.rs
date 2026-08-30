//! OW-IAT wire-up: H_00 manual-map path (post spike `pe_manual_map.rs`).
//!
//! Phase 1: file-read prelude — CreateFileA / ReadFile / VirtualAlloc / CloseHandle.
//! Phase 2: inline manual-map x64 — sections + DIR64 reloc + import resolve via PEB walk.
//! Phase 3: three-peer lockstep (JS `win32-h00-selfhost.js` + asm delegate).
//!
//! Replaces PEB ROR13 `LoadLibraryA` resolve in `gen_h00_selfhost_main`.

use crate::win32_selfhost::{SelfhostMeta, IAT_EXIT_PROCESS};

/// IAT slots at r15+0 (see pe_link KERNEL32_IO_FUNCS).
pub const IAT_VIRTUAL_ALLOC: u32 = 0;
pub const IAT_CREATE_FILE: u32 = 1;
pub const IAT_READ_FILE: u32 = 2;
pub const IAT_CLOSE_HANDLE: u32 = 4;

/// Stack scratch for ReadFile nNumberOfBytesRead (above shadow 0x28).
const READ_BYTES_STACK_OFF: u8 = 0x30;

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
/// Scratch qword past kernel32+preload IAT (6+4 slots × 8 = 0x50) — bootstrap LoadLibraryA.
const H00_LOADLIBRARY_SCRATCH_OFF: u8 = 0x50;
const H00_GETPROCADDRESS_SCRATCH_OFF: u8 = 0x58;
const H00_KERNEL32_SCRATCH_OFF: u8 = 0x60;
/// Success-path phase probe (survives until crash for post-mortem; not used on fail epilogue).
const H00_PHASE_SCRATCH_OFF: u8 = 0x68;

const PHASE_MAP_IMAGE_OK: u8 = 0x0A;
const PHASE_SECTIONS_OK: u8 = 0x0B;
const PHASE_RELOC_OK: u8 = 0x0C;
const PHASE_IMPORT_OK: u8 = 0x0D;
const PHASE_FLUSH_ICACHE: u8 = 0x0E;
const PHASE_EXPORT_CALL: u8 = 0x0F;

/// Win64 home-space before `call` to kernel32 (requires RSP%16==8 at CALL → use after `and rsp,-16`).
const WIN64_CALL_SHADOW: u8 = 0x28;

fn emit_align_for_win64_call(c: &mut Vec<u8>) {
    c.extend_from_slice(&[0x48, 0x83, 0xE4, 0xF0]); // and rsp, -16
}

fn emit_win64_call_shadow(c: &mut Vec<u8>) {
    emit_align_for_win64_call(c);
    c.extend_from_slice(&[0x48, 0x83, 0xEC, WIN64_CALL_SHADOW]);
}

fn emit_win64_pop_shadow(c: &mut Vec<u8>) {
    c.extend_from_slice(&[0x48, 0x83, 0xC4, WIN64_CALL_SHADOW]);
}

/// ExitProcess via IAT — Win64 requires RSP%16==8 at `call` (prologue forces RSP%16==0).
fn emit_exit_process_iat(
    c: &mut Vec<u8>,
    text_rva: u32,
    chunk_text_off: u32,
    iat_rva: u32,
    exit_code: u8,
) {
    emit_win64_call_shadow(c);
    c.extend_from_slice(&[0xB9, exit_code, 0, 0, 0]);
    emit_call_iat_merged(c, text_rva, chunk_text_off, iat_rva, IAT_EXIT_PROCESS);
}

fn emit_phase_probe(c: &mut Vec<u8>, phase: u8) {
    c.extend_from_slice(&[0x41, 0xC6, 0x47, H00_PHASE_SCRATCH_OFF, phase]);
}

/// `mov r8d, [r14+rbx+disp]` — mapped-image optional-header field (ebx = e_lfanew).
fn emit_mov_u32_pe_mapped(c: &mut Vec<u8>, disp: u8) {
    c.push(0x45); // REX.R+B for r8d + r14 SIB base
    c.push(0x8B);
    if disp < 0x80 {
        c.extend_from_slice(&[0x84, 0x1E, disp]); // mod=10, reg=r8, SIB base=r14 index=rbx
    } else {
        c.extend_from_slice(&[0x84, 0x1E, disp, 0, 0, 0]);
    }
}

/// H_00 stub prologue (`push` saves + `sub rsp` + align) before file-read prelude.
pub const H00_PROLOGUE_LEN: u32 = 15;

fn patch_rel32(c: &mut [u8], disp_off: usize, from: usize, to: usize) {
    let rel = to as i32 - from as i32;
    c[disp_off..disp_off + 4].copy_from_slice(&rel.to_le_bytes());
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
/// SIB for `[r12 + disp8]` (index suppressed; base r12 via REX.B).
const SIB_R12_ONLY: u8 = 0x3C;

/// `mov ebx, [r12+3Ch]` — e_lfanew from file PE buffer (r12).
fn emit_mov_e_lfanew_pe_file(c: &mut Vec<u8>) {
    c.extend_from_slice(&[0x41, 0x8B, 0x5C, SIB_R12_ONLY, 0x3C]);
}

/// `mov ebx, [r14+3Ch]` — e_lfanew from mapped image (r14).
fn emit_mov_e_lfanew_pe_mapped(c: &mut Vec<u8>) {
    // ModRM 5E = ebx,[r14+disp8] (REX.B); 4E=ecx and 7E=edi — both break [r14+rbx+disp] PE reads.
    c.extend_from_slice(&[0x41, 0x8B, 0x5E, 0x3C]);
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

    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x48]);

    let lea_path = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x0D, 0, 0, 0, 0]);
    // CreateFileA(path, GENERIC_READ, share=0, sa=NULL, OPEN_EXISTING, NORMAL, hTemplate=NULL)
    c.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x80]); // GENERIC_READ
    c.extend_from_slice(&[0x45, 0x31, 0xC0]); // xor r8d,r8d
    c.extend_from_slice(&[0x45, 0x31, 0xC9]); // xor r9d,r9d
    c.extend_from_slice(&[0xC7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00]); // OPEN_EXISTING
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00]); // FILE_ATTRIBUTE_NORMAL
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00]); // hTemplateFile
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_CREATE_FILE);
    c.extend_from_slice(&[0x48, 0x83, 0xF8, 0xFF]);
    let jz_no_file = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xC3]);

    c.extend_from_slice(&[0x31, 0xC9]);
    c.extend_from_slice(&[0xBA, 0x00, 0x00, 0x80, 0x00]); // max read 8 MiB (crt-static sidecar)
    c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x30, 0x00, 0x00]);
    c.extend_from_slice(&[0x41, 0xB9, 0x04, 0x00, 0x00, 0x00]);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_VIRTUAL_ALLOC);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_no_buf = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x89, 0xC4]);

    c.extend_from_slice(&[0x48, 0x89, 0xD9]);
    c.extend_from_slice(&[0x4C, 0x89, 0xE2]);
    c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x00, 0x80, 0x00]); // ReadFile size cap
    c.extend_from_slice(&[0x4C, 0x8D, 0x4C, 0x24, READ_BYTES_STACK_OFF]);
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00]);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_READ_FILE);

    c.extend_from_slice(&[0x48, 0x89, 0xD9]);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_CLOSE_HANDLE);

    c.extend_from_slice(&[0x44, 0x8B, 0x6C, 0x24, READ_BYTES_STACK_OFF]);
    c.extend_from_slice(&[0x45, 0x85, 0xED]);
    let jz_empty = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);

    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x48]);

    fix_rip_disp(
        &mut c,
        lea_path + 3,
        text_rva,
        chunk_text_off,
        lea_path + 7,
        meta.temp_name_rva,
    );

    for (at, fail) in [
        (jz_no_file, fail_create_file),
        (jz_no_buf, fail_virtual_alloc),
        (jz_empty, fail_read_empty),
    ] {
        patch_rel32(
            &mut c,
            at + 2,
            chunk_text_off as usize + at + 6,
            fail,
        );
    }

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
    emit_phase_probe(&mut c, PHASE_MAP_IMAGE_OK);

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
    c.extend_from_slice(&[0x4B, 0x8D, 0x34, 0x0C]); // lea rsi,[r12+r9]
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
    emit_phase_probe(&mut c, PHASE_SECTIONS_OK);

    // Reloc delta: r10 = mapped_base - ImageBase
    c.extend_from_slice(&[
        0x4F, 0x8B, 0x94, 0x1C, PE_OFF_IMAGE_BASE, 0x00, 0x00, 0x00,
    ]); // mov r10,[r12+rbx+30h] (REX.W+R+B — was 4D 8B 94 = mov rdx)
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
    emit_phase_probe(&mut c, PHASE_RELOC_OK);

    // Bootstrap LoadLibraryA at [r15+scratch] for find_module fallback (api-set forwarders).
    c.extend_from_slice(&[0x49, 0xC7, 0x47, H00_LOADLIBRARY_SCRATCH_OFF, 0, 0, 0, 0]);
    emit_mov_u32_pe_file(&mut c, 0, PE_OFF_IMPORT_DIR_RVA);
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_skip_ll_boot = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x8D, 0x34, 0x06]); // lea rsi,[r14+rax] first import desc
    c.extend_from_slice(&[0x8B, 0x4E, 0x0C]); // Name RVA
    c.extend_from_slice(&[0x49, 0x8D, 0x54, 0x0E, 0x00]); // lea rdx,[r14+rcx] dll name (KERNEL32)
    let call_boot_find = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_skip_ll_boot2 = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xC7]); // rdi = kernel32
    // Build export name on stack byte-by-byte (no contiguous "LoadLibraryA" in PE).
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x20]); // sub rsp, 0x20
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
        c.extend_from_slice(&[0xC6, 0x44, 0x24, off, ch]);
    }
    c.extend_from_slice(&[0x48, 0x8D, 0x14, 0x24]); // lea rdx, [rsp]
    let call_boot_resolve = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x20]); // add rsp, 0x20
    c.extend_from_slice(&[0x49, 0x89, 0x47, H00_LOADLIBRARY_SCRATCH_OFF]); // [r15+scratch]=LoadLibraryA
    c.extend_from_slice(&[0x49, 0x89, 0x7F, H00_KERNEL32_SCRATCH_OFF]); // [r15+0x60]=kernel32
    // Bootstrap GetProcAddress (sidecar IAT resolve uses host LoadLibrary+GetProcAddress).
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x20]);
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
        c.extend_from_slice(&[0xC6, 0x44, 0x24, off, ch]);
    }
    c.extend_from_slice(&[0x48, 0x8D, 0x14, 0x24]);
    let call_boot_gpa = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x20]);
    c.extend_from_slice(&[0x49, 0x89, 0x47, H00_GETPROCADDRESS_SCRATCH_OFF]);
    let skip_ll_boot = c.len();
    patch_rel32(&mut c, jz_skip_ll_boot + 2, jz_skip_ll_boot + 6, skip_ll_boot);
    patch_rel32(&mut c, jz_skip_ll_boot2 + 2, jz_skip_ll_boot2 + 6, skip_ll_boot);

    // Import resolve: walk descriptors at [r14+import_rva]
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
    emit_win64_call_shadow(&mut c);
    // FirstThunk rva is in edx — load module first, then lea r11 (LL/GPA clobber r11).
    c.extend_from_slice(&[0x49, 0x8D, 0x14, 0x0E]); // lea rdx,[r14+rcx] module name
    c.extend_from_slice(&[0x48, 0x89, 0xD1]); // mov rcx, rdx — LoadLibraryA(lpLibFileName)
    c.extend_from_slice(&[0x41, 0xFF, 0x57, H00_LOADLIBRARY_SCRATCH_OFF]); // call [r15+LoadLibraryA]
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    fail_jumps.push((c.len(), fail_import));
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xC7]); // rdi = hModule
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
    c.extend_from_slice(&[0x48, 0x89, 0xF9]); // mov rcx, rdi — hModule
    c.extend_from_slice(&[0x49, 0x0F, 0xBA, 0xE2, 0x3F]); // bt r10,63 (not BTS /4→EA)
    let jc_ord = c.len();
    c.extend_from_slice(&[0x0F, 0x82, 0, 0, 0, 0]); // jc ord_gpa
    c.extend_from_slice(&[0x4B, 0x8D, 0x54, 0x16, 0x02]); // lea rdx,[r14+r10+2] name (REX.R for r10 index)
    let gpa_call = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    let ord_gpa = c.len();
    patch_rel32(&mut c, jc_ord + 2, jc_ord + 6, ord_gpa);
    c.extend_from_slice(&[0x44, 0x89, 0xD0]); // mov eax, r10d
    c.extend_from_slice(&[0x25, 0xFF, 0xFF, 0x00, 0x00]); // and eax, 0xffff — ordinal LPCSTR
    c.extend_from_slice(&[0x89, 0xC2]); // mov edx, eax
    let gpa_call_site = c.len();
    patch_rel32(&mut c, gpa_call + 1, gpa_call + 5, gpa_call_site);
    c.extend_from_slice(&[0x41, 0x53]); // push r11 — GPA clobbers volatile IAT cursor
    c.extend_from_slice(&[0x41, 0xFF, 0x57, H00_GETPROCADDRESS_SCRATCH_OFF]); // GetProcAddress
    c.extend_from_slice(&[0x41, 0x5B]); // pop r11
    c.extend_from_slice(&[0x48, 0x85, 0xC0]); // resolve failed → fail_import
    fail_jumps.push((c.len(), fail_import));
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
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
    emit_phase_probe(&mut c, PHASE_IMPORT_OK);

    // Realign stack after nested find/resolve helper calls (Win64 movaps safety).
    emit_align_for_win64_call(&mut c);

    // FlushInstructionCache before calling mapped sidecar code (matches reference mapper).
    // r12 was clobbered — read PE headers from mapped image r14; load r8d before GPA (rax).
    emit_mov_e_lfanew_pe_mapped(&mut c);
    emit_mov_u32_pe_mapped(&mut c, PE_OFF_SIZE_OF_IMAGE); // r8d = SizeOfImage (keep through GPA)
    c.extend_from_slice(&[
        0x49, 0x8B, 0x84, 0x1E, PE_OFF_IMPORT_DIR_RVA, 0x00, 0x00, 0x00,
    ]); // eax = import dir RVA from mapped image
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_skip_flush = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    emit_phase_probe(&mut c, PHASE_FLUSH_ICACHE);
    emit_win64_call_shadow(&mut c);
    c.extend_from_slice(&[0x49, 0x8B, 0x4F, H00_KERNEL32_SCRATCH_OFF]); // rcx = kernel32
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x30]);
    for (off, ch) in b"FlushInstructionCache\0".iter().enumerate() {
        c.extend_from_slice(&[0xC6, 0x44, 0x24, off as u8, *ch]);
    }
    c.extend_from_slice(&[0x48, 0x8D, 0x14, 0x24]);
    c.extend_from_slice(&[0x41, 0xFF, 0x57, H00_GETPROCADDRESS_SCRATCH_OFF]); // rax = FlushICache
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x30]);
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
    patch_rel32(&mut c, jz_skip_flush + 2, jz_skip_flush + 6, skip_flush);
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
    c.extend_from_slice(&[0x49, 0xFF, 0xC0]);
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
    c.extend_from_slice(&[0x49, 0x8B, 0x47, H00_LOADLIBRARY_SCRATCH_OFF]); // mov rax,[r15+scratch]
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_ll_fail = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xD1]); // mov rcx, rdx (dll name)
    emit_win64_call_shadow(&mut c);
    c.extend_from_slice(&[0xFF, 0xD0]); // call rax
    emit_win64_pop_shadow(&mut c);
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
    c.extend_from_slice(&[0x49, 0x8B, 0x04, 0xA3]); // mov eax,[rbx+r10*4] (REX.R+B; 93=rdx index)
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
    c.extend_from_slice(&[0x49, 0x0F, 0xB7, 0x04, 0x61]); // movzx eax,word [rcx+r10*2] (REX.R; 51=rdx index)
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
    c.extend_from_slice(&[0x29, 0xC9]); // sub ecx,r9d (ordinal index)
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
    c.extend_from_slice(&[0x49, 0x8D, 0x1C, 0x12]); // lea rbx,[r10+rdx] exp end — NOT 1C 0A (=+rcx)
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
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x40]); // sub rsp, 0x40
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
    let call_ff_find = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_ff_bad2 = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xC7]); // mov rdi, rax (target module)
    c.extend_from_slice(&[0x49, 0x8D, 0x51, 0x01]); // lea rdx,[r9+1] func name
    let call_ff_resolve = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x40]); // add rsp, 0x40
    let ff_ret = c.len();
    c.extend_from_slice(&[0x5E]); // pop rsi
    c.extend_from_slice(&[0xC3]);
    let ff_ret_pop = c.len();
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x40]); // add rsp, 0x40 (find_module failed)
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
    emit_phase_probe(&mut c, PHASE_EXPORT_CALL);
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
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x20]);
    // PE entry is JMP (not CALL) into H_00 — force RSP%16==0 before any Win64 calls.
    c.extend_from_slice(&[0x48, 0x83, 0xE4, 0xF0]); // and rsp, -16

    let prelude_text_off = code_base_off + H00_PROLOGUE_LEN;

    const EPILOGUE_LEN: usize = 8 * 19; // Win64 shadow + mov ecx,imm32 + FF15 rel32 per phase

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
    let epilogue_base = code_base_off as usize
        + H00_PROLOGUE_LEN as usize
        + prelude_len
        + map_len
        + tail_len;

    let fail_create_file = epilogue_base;
    let fail_read_empty = epilogue_base + 11;
    let fail_virtual_alloc = epilogue_base + 22;
    let _fail_section_copy = epilogue_base + 33;
    let _fail_reloc = epilogue_base + 44;
    let fail_import = epilogue_base + 55;
    let fail_export = epilogue_base + 66;

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
            body.len() < 230,
            "file-read prelude should stay <230B (got {}B)",
            body.len()
        );
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
            body.len() > 400 && body.len() < 2000,
            "manual-map H_00 stub should fit OW-STUB pin [40,2000] (got {}B)",
            body.len()
        );
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
        // REX.W+B without REX.R on [r12+rbx] reads ImageBase into rdx not r10 (breaks reloc delta).
        assert!(
            !body.windows(4).any(|w| w == [0x4D, 0x8B, 0x94, 0x1C]),
            "must not emit mov rdx,[r12+rbx] (4D 8B 94 1C) for ImageBase — need mov r10 (4F 8B 94 1C)"
        );
        assert!(
            body.windows(4).any(|w| w == [0x4F, 0x8B, 0x94, 0x1C]),
            "missing mov r10,[r12+rbx] ImageBase read (4F 8B 94 1C)"
        );
        // REX.W+B without REX.R on [r14+rbx] reads SizeOfImage into rax — clobbers GPA result before call.
        assert!(
            !body.windows(4).any(|w| w == [0x4D, 0x8B, 0x84, 0x1E]),
            "must not emit mov rax,[r14+rbx] (4D 8B 84 1E) for SizeOfImage — need mov r8d (45 8B 84 1E)"
        );
        assert!(
            body.windows(4).any(|w| w == [0x45, 0x8B, 0x84, 0x1E]),
            "missing mov r8d,[r14+rbx] SizeOfImage read (45 8B 84 1E)"
        );
        assert!(
            body.windows(4)
                .filter(|w| **w == [0x49, 0x8B, 0x04, 0xA3])
                .count()
                >= 1,
            "resolve_export needs mov eax,[rbx+r10*4] (49 8B 04 A3); 41 8B 04 93 uses rdx index"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x41, 0x8B, 0x04, 0x93]),
            "must not emit mov eax,[rbx+rdx*4] (41 8B 04 93) — export name index is r10"
        );
        assert!(
            body.windows(5)
                .any(|w| *w == [0x49, 0x0F, 0xB7, 0x04, 0x61]),
            "resolve_export needs movzx eax,word [rcx+r10*2] (49 0F B7 04 61)"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x41, 0x0F, 0xB7, 0x04, 0x51]),
            "must not emit movzx eax,word [rcx+rdx*2] (41 0F B7 04 51)"
        );
        // Success-path phase probes (survive until crash).
        assert!(
            body.windows(5).any(|w| w == [0x41, 0xC6, 0x47, H00_PHASE_SCRATCH_OFF, PHASE_FLUSH_ICACHE]),
            "missing FlushICache phase probe at [r15+68h]"
        );
        // e_lfanew reads must hit PE base, not [rsp+disp] / wrong reg.
        assert!(
            body.windows(5).any(|w| w == [0x41, 0x8B, 0x5C, SIB_R12_ONLY, 0x3C]),
            "missing mov ebx,[r12+3Ch] (file PE e_lfanew)"
        );
        assert!(
            !body.windows(5).any(|w| w == [0x41, 0x8B, 0x5C, 0x24, 0x3C]),
            "must not emit mov ebx,[rsp+3Ch] (SIB 24 = rsp+rsp, not r12)"
        );
        assert!(
            body.windows(4).any(|w| w == [0x41, 0x8B, 0x5E, 0x3C]),
            "missing mov ebx,[r14+3Ch] (mapped image e_lfanew; ModRM 5E=ebx)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x41, 0x8B, 0x4E, 0x3C]),
            "must not emit mov ecx,[r14+3Ch] (4E=ecx — ebx stays clobbered after import loop)"
        );
        assert!(
            !body.windows(4).any(|w| w == [0x41, 0x8B, 0x7E, 0x3C]),
            "must not emit mov edi,[r14+3Ch] (7E=edi not ebx)"
        );
        // Export call after `and rsp,-16` needs sub rsp,0x28 (0x20 → pre-call RSP%16==0 → callee AV).
        let export_align = body
            .windows(8)
            .position(|w| w == [0x48, 0x83, 0xE4, 0xF0, 0x48, 0x83, 0xEC, 0x28]);
        assert!(
            export_align.is_some(),
            "export tail must and rsp,-16 then sub rsp,0x28 before call (Win64 alignment)"
        );
        // Fail epilogues: shadow + mov ecx,imm + FF15
        assert!(
            body.windows(14)
                .filter(|w| {
                    w[0..8] == [0x48, 0x83, 0xE4, 0xF0, 0x48, 0x83, 0xEC, 0x28]
                        && w[8] == 0xB9
                        && w[13] == 0xFF
                })
                .count()
                >= 8,
            "fail epilogues need Win64 shadow before ExitProcess"
        );
        assert!(
            body.windows(11).any(|w| {
                w[0..3] == [0x89, 0xC1, 0x48]
                    && w[3..11] == [0x83, 0xE4, 0xF0, 0x48, 0x83, 0xEC, 0x28, 0xFF]
            }),
            "export success path needs mov ecx,eax + Win64 shadow before ExitProcess"
        );
    }
}
