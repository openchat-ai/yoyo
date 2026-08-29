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

/// InMemoryOrderModuleList: Flink points at LDR entry + 0x10.
const LDR_INMEMORY_FLINK_OFF: u8 = 0x10;
const LDR_DLLBASE_OFF: u8 = 0x30;
const LDR_BASEDLLNAME_BUF_OFF: u8 = 0x60;

/// H_00 stub prologue (`push` saves + `sub rsp`) before file-read prelude.
pub const H00_PROLOGUE_LEN: u32 = 11;

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

/// Emit x64 that reads cwd sidecar `yoyo_rt.dll` into a VirtualAlloc buffer.
///
/// On success: `r12` = file bytes pointer, `r13d` = byte count.
/// On failure: jumps to `fail_label`.
pub fn gen_h00_read_sidecar_prelude(
    meta: &SelfhostMeta,
    text_rva: u32,
    chunk_text_off: u32,
    fail_label: usize,
) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();

    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x48]);

    let lea_path = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x0D, 0, 0, 0, 0]);
    c.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x80]);
    c.extend_from_slice(&[0x45, 0x31, 0xC0]);
    c.extend_from_slice(&[0x41, 0xB9, 0x03, 0x00, 0x00, 0x00]);
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00]);
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00]);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_CREATE_FILE);
    c.extend_from_slice(&[0x48, 0x83, 0xF8, 0xFF]);
    let jz_no_file = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xC3]);

    c.extend_from_slice(&[0x31, 0xC9]);
    c.extend_from_slice(&[0xBA, 0x00, 0x00, 0x08, 0x00]);
    c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x30, 0x00, 0x00]);
    c.extend_from_slice(&[0x41, 0xB9, 0x04, 0x00, 0x00, 0x00]);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_VIRTUAL_ALLOC);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_no_buf = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x89, 0xC4]);

    c.extend_from_slice(&[0x48, 0x89, 0xD9]);
    c.extend_from_slice(&[0x4C, 0x89, 0xE2]);
    c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x00, 0x08, 0x00]);
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

    for at in [jz_no_file, jz_no_buf, jz_empty] {
        patch_rel32(
            &mut c,
            at + 2,
            chunk_text_off as usize + at + 6,
            fail_label,
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
    fail_label: usize,
) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();
    let mut fail_jumps: Vec<usize> = Vec::new();

    // ebx = e_lfanew; r12 = file PE
    c.extend_from_slice(&[0x41, 0x8B, 0x5C, 0x24, 0x3C]); // mov ebx,[r12+3c]
    // VirtualAlloc(0, SizeOfImage, MEM_COMMIT|RESERVE, PAGE_EXECUTE_READWRITE)
    c.extend_from_slice(&[
        0x8B, 0x94, 0x1C, PE_OFF_SIZE_OF_IMAGE, 0x00, 0x00, 0x00,
    ]); // mov edx,[r12+rbx+50h]
    c.extend_from_slice(&[0x31, 0xC9]);
    c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x30, 0x00, 0x00]);
    c.extend_from_slice(&[0x41, 0xB9, 0x40, 0x00, 0x00, 0x00]);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, iat_rva, IAT_VIRTUAL_ALLOC);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    fail_jumps.push(c.len());
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x89, 0xC6]); // mov r14, rax (image)

    // Copy headers: rep movsb SizeOfHeaders
    c.extend_from_slice(&[
        0x8B, 0x8C, 0x1C, PE_OFF_SIZE_OF_HEADERS, 0x00, 0x00, 0x00,
    ]); // mov ecx,[r12+rbx+54h]
    c.extend_from_slice(&[0x4C, 0x89, 0xF7]); // mov rdi, r14
    c.extend_from_slice(&[0x4C, 0x89, 0xE6]); // mov rsi, r12
    c.extend_from_slice(&[0xF3, 0xA4]); // rep movsb

    // Section copy loop: esi = NumberOfSections, r8d = index
    c.extend_from_slice(&[
        0x0F, 0xB7, 0x74, 0x1C, PE_OFF_NUMBER_OF_SECTIONS,
    ]); // movzx esi,word [r12+rbx+6]
    c.extend_from_slice(&[0x45, 0x31, 0xC0]); // xor r8d,r8d
    let sec_loop = c.len();
    c.extend_from_slice(&[0x44, 0x39, 0xC6]); // cmp esi,r8d
    let jae_secs_done = c.len();
    c.extend_from_slice(&[0x0F, 0x83, 0, 0, 0, 0]);
    // section hdr = r12 + rbx + 24 + SizeOfOptionalHeader + r8*40
    c.extend_from_slice(&[
        0x0F, 0xB7, 0x84, 0x1C, PE_OFF_SIZE_OF_OPTIONAL_HEADER, 0x00, 0x00, 0x00,
    ]); // movzx eax,word [r12+rbx+14h]
    c.extend_from_slice(&[0x83, 0xC0, PE_OFF_OPTIONAL]); // add eax,24
    c.extend_from_slice(&[0x49, 0x8D, 0x3C, 0x1C]); // lea rdi,[r12+rbx]
    c.extend_from_slice(&[0x48, 0x01, 0xC7]); // add rdi,rax
    c.extend_from_slice(&[0x41, 0x6B, 0xC0, 0x28]); // imul eax,r8d,40
    c.extend_from_slice(&[0x48, 0x01, 0xC7]); // add rdi,rax

    c.extend_from_slice(&[0x8B, 0x4F, 0x0C]); // mov ecx,[rdi+0c] VirtualAddress
    c.extend_from_slice(&[0x8B, 0x57, 0x10]); // mov edx,[rdi+10] SizeOfRawData
    c.extend_from_slice(&[0x44, 0x8B, 0x4F, 0x14]); // mov r9d,[rdi+14] PointerToRawData
    c.extend_from_slice(&[0x85, 0xD2]);
    let jz_next_sec = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x8D, 0x3C, 0x0E]); // lea rdi,[r14+rcx]
    c.extend_from_slice(&[0x4B, 0x8D, 0x34, 0x0C]); // lea rsi,[r12+r9]
    c.extend_from_slice(&[0x89, 0xD1]); // mov ecx, edx
    c.extend_from_slice(&[0xF3, 0xA4]);
    let next_sec = c.len();
    patch_rel32(&mut c, jz_next_sec + 2, jz_next_sec + 6, next_sec);
    c.extend_from_slice(&[0x41, 0xFF, 0xC0]); // inc r8d
    let jmp_sec = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_sec + 1, jmp_sec + 5, sec_loop);
    let secs_done = c.len();
    patch_rel32(&mut c, jae_secs_done + 2, jae_secs_done + 6, secs_done);

    // Reloc delta: r10 = mapped_base - ImageBase
    c.extend_from_slice(&[
        0x4C, 0x8B, 0x94, 0x1C, PE_OFF_IMAGE_BASE, 0x00, 0x00, 0x00,
    ]); // mov r10,[r12+rbx+30h] (preferred ImageBase from file headers)
    c.extend_from_slice(&[0x4C, 0x89, 0xF0]); // mov rax, r14 (mapped base)
    c.extend_from_slice(&[0x4C, 0x29, 0xD0]); // sub rax, r10 → delta
    c.extend_from_slice(&[0x49, 0x89, 0xC2]); // mov r10, rax

    // Base reloc directory RVA (data directory index 5)
    c.extend_from_slice(&[
        0x8B, 0x84, 0x1C, PE_OFF_BASERELOC_DIR_RVA, 0x00, 0x00, 0x00,
    ]);
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

    // Import resolve: walk descriptors at [r14+import_rva]
    c.extend_from_slice(&[
        0x8B, 0x84, 0x1C, PE_OFF_IMPORT_DIR_RVA, 0x00, 0x00, 0x00,
    ]); // import dir rva
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
    // edx=FirstThunk rva — save IAT cursor before rdx becomes module-name ptr
    c.extend_from_slice(&[0x4D, 0x8D, 0x1C, 0x16]); // lea r11,[r14+rdx] IAT write cursor
    c.extend_from_slice(&[0x49, 0x8D, 0x14, 0x0E]); // lea rdx,[r14+rcx] module name
    let call_find_mod = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]); // call find_module
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    fail_jumps.push(c.len());
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xC7]); // rdi = module base
    c.extend_from_slice(&[0x49, 0x89, 0xF5]); // mov r13, rsi (save import descriptor ptr)
    c.extend_from_slice(&[0x85, 0xDB]); // cmp ebx,0 (OriginalFirstThunk)
    let jz_iat_read = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x8D, 0x34, 0x1E]); // lea rsi,[r14+rbx] read OFT
    let j_to_loop = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    let iat_read = c.len();
    patch_rel32(&mut c, jz_iat_read + 2, jz_iat_read + 6, iat_read);
    c.extend_from_slice(&[0x49, 0x89, 0xDE]); // mov rsi, r11 (read IAT when no OFT)
    let thunk_loop = c.len();
    patch_rel32(&mut c, j_to_loop + 1, j_to_loop + 5, thunk_loop);
    c.extend_from_slice(&[0x48, 0x8B, 0x06]); // thunk
    c.extend_from_slice(&[0x48, 0x85, 0xC0]);
    let jz_thunk_done = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xC1]); // mov rcx, rax (save thunk)
    c.extend_from_slice(&[0x48, 0x0F, 0xBA, 0xE8, 0x3F]); // bt rax,63
    let jc_ord = c.len();
    c.extend_from_slice(&[0x0F, 0x82, 0, 0, 0, 0]); // jc ord_path
    c.extend_from_slice(&[0x49, 0x8D, 0x54, 0x0E, 0x02]); // lea rdx,[r14+rcx+2] name
    let call_resolve = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    let jmp_store_iat = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    let ord_path = c.len();
    patch_rel32(&mut c, jc_ord + 2, jc_ord + 6, ord_path);
    c.extend_from_slice(&[0x89, 0xC8]); // mov eax, ecx
    c.extend_from_slice(&[0x25, 0xFF, 0xFF, 0x00, 0x00]); // and eax, 0xffff
    let call_resolve_ord = c.len();
    c.extend_from_slice(&[0xE8, 0, 0, 0, 0]);
    let store_iat = c.len();
    patch_rel32(&mut c, jmp_store_iat + 1, jmp_store_iat + 5, store_iat);
    c.extend_from_slice(&[0x49, 0x89, 0x03]); // mov [r11], rax (IAT slot)
    c.extend_from_slice(&[0x48, 0x83, 0xC6, 0x08]);
    c.extend_from_slice(&[0x49, 0x83, 0xC3, 0x08]);
    let jmp_thunk = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_thunk + 1, jmp_thunk + 5, thunk_loop);
    let thunk_done = c.len();
    patch_rel32(&mut c, jz_thunk_done + 2, jz_thunk_done + 6, thunk_done);
    c.extend_from_slice(&[0x49, 0x83, 0xC5, 0x14]); // add r13, 20 (next IMAGE_IMPORT_DESCRIPTOR)
    c.extend_from_slice(&[0x4C, 0x89, 0xEE]); // mov rsi, r13
    let jmp_id = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_id + 1, jmp_id + 5, import_desc);
    let import_done = c.len();
    patch_rel32(&mut c, jz_import_done + 2, jz_import_done + 6, import_done);
    patch_rel32(&mut c, jz_idone + 2, jz_idone + 6, import_done);

    // Rust cdylib sidecar needs DllMain(PROCESS_ATTACH) after imports (manual-map ≠ LoadLibrary).
    c.extend_from_slice(&[0x41, 0x8B, 0x46, 0x3C]); // mov eax,[r14+3c] e_lfanew
    c.extend_from_slice(&[0x41, 0x8B, 0x44, 0x06, 0x28]); // mov eax,[r14+rax+28] AddressOfEntryPoint
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_no_dllmain = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x4C, 0x89, 0xF1]); // mov rcx,r14 (hinst)
    c.extend_from_slice(&[0xBA, 0x01, 0x00, 0x00, 0x00]); // edx=DLL_PROCESS_ATTACH
    c.extend_from_slice(&[0x45, 0x31, 0xC0]); // r8d=0 lpReserved
    c.extend_from_slice(&[0x4C, 0x01, 0xF0]); // add rax,r14 → entry VA
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]); // shadow
    c.extend_from_slice(&[0xFF, 0xD0]); // call rax
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);
    let after_dllmain = c.len();
    patch_rel32(&mut c, jz_no_dllmain + 2, jz_no_dllmain + 6, after_dllmain);

    // rbx = mapped image (r14)
    c.extend_from_slice(&[0x4C, 0x89, 0xF3]); // mov rbx, r14

    // --- Internal helpers (placed after main path) ---
    let find_module = c.len();
    patch_rel32(&mut c, call_find_mod + 1, call_find_mod + 5, find_module);
    // rdx = ascii dll name → rax = DllBase
    c.extend_from_slice(&[0x65, 0x48, 0x8B, 0x04, 0x25, 0x60, 0x00, 0x00, 0x00]);
    c.extend_from_slice(&[0x48, 0x8B, 0x40, 0x18]); // PEB->Ldr
    c.extend_from_slice(&[0x48, 0x8B, 0x40, 0x20]); // InMemoryOrderModuleList.Flink → first module
    c.extend_from_slice(&[0x48, 0x83, 0xE8, LDR_INMEMORY_FLINK_OFF]); // entry = Flink - 0x10
    let mod_loop = c.len();
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
    c.extend_from_slice(&[0x45, 0x0F, 0xB6, 0x11]); // movzx r10d,byte [r9] (keep r11=IAT cursor)
    c.extend_from_slice(&[0x45, 0x85, 0xD2]);
    let jz_dll_mismatch = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    // tolower eax and r10b, compare
    c.extend_from_slice(&[0x3C, 0x41]); // cmp al,'A'
    c.extend_from_slice(&[0x72, 0x04]);
    c.extend_from_slice(&[0x3C, 0x5A]);
    c.extend_from_slice(&[0x77, 0x04]);
    c.extend_from_slice(&[0x0C, 0x20]); // or al,0x20
    c.extend_from_slice(&[0x41, 0x80, 0xFA, 0x41]);
    c.extend_from_slice(&[0x72, 0x04]);
    c.extend_from_slice(&[0x41, 0x80, 0xFA, 0x5A]);
    c.extend_from_slice(&[0x77, 0x04]);
    c.extend_from_slice(&[0x41, 0x80, 0xCA, 0x20]); // or r10b,0x20
    c.extend_from_slice(&[0x41, 0x38, 0xC1]); // cmp al,r10b
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
    c.extend_from_slice(&[0x48, 0x83, 0xE8, LDR_INMEMORY_FLINK_OFF]); // entry base
    let jmp_mod = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_mod + 1, jmp_mod + 5, mod_loop);
    let no_mod = c.len();
    patch_rel32(&mut c, jz_no_mod + 2, jz_no_mod + 6, no_mod);
    patch_rel32(&mut c, jz_next_mod + 2, jz_next_mod + 6, dll_mismatch);
    c.extend_from_slice(&[0x31, 0xC0]);
    c.extend_from_slice(&[0xC3]);
    let mod_found = c.len();
    patch_rel32(&mut c, jz_mod_found + 2, jz_mod_found + 6, mod_found);
    c.extend_from_slice(&[0x48, 0x8B, 0x40, LDR_DLLBASE_OFF]); // DllBase
    c.extend_from_slice(&[0xC3]);

    let resolve_export = c.len();
    patch_rel32(&mut c, call_resolve + 1, call_resolve + 5, resolve_export);
    // rdi=module, rdx=name → rax=func (preserve thunk rsi; rbx/rcx/rdx = PE tables)
    c.extend_from_slice(&[0x49, 0x89, 0xD1]); // mov r9, rdx — save import name before table walk
    c.extend_from_slice(&[0x8B, 0x47, 0x3C]);
    c.extend_from_slice(&[0x8B, 0x84, 0x38, 0x88, 0x00, 0x00, 0x00]);
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
    c.extend_from_slice(&[0x41, 0x8B, 0x04, 0x93]); // mov eax,[rbx+r10*4]
    c.extend_from_slice(&[0x48, 0x01, 0xF8]);
    // strcmp r9 with [rax]
    let cmp_name = c.len();
    c.extend_from_slice(&[0x0F, 0xB6, 0x08]); // movzx eax, byte [rax] export name
    c.extend_from_slice(&[0x41, 0x0F, 0xB6, 0x19]); // movzx ebx, byte [r9] import name
    c.extend_from_slice(&[0x38, 0xD8]); // cmp al, bl
    let jne_name = c.len();
    c.extend_from_slice(&[0x0F, 0x85, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x84, 0xC9]);
    let jz_name_found = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0xFF, 0xC0]);
    c.extend_from_slice(&[0x49, 0xFF, 0xC1]);
    let jmp_cmp = c.len();
    c.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut c, jmp_cmp + 1, jmp_cmp + 5, cmp_name);
    let name_found = c.len();
    patch_rel32(&mut c, jz_name_found + 2, jz_name_found + 6, name_found);
    c.extend_from_slice(&[0x41, 0x0F, 0xB7, 0x04, 0x51]); // movzx eax,word [rcx+r10*2]
    c.extend_from_slice(&[0x8B, 0x04, 0x82]); // mov eax,[rdx+rax*4]
    c.extend_from_slice(&[0x48, 0x01, 0xF8]);
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
    c.extend_from_slice(&[0x31, 0xC0]);
    c.extend_from_slice(&[0xC3]);

    let resolve_export_ordinal = c.len();
    patch_rel32(
        &mut c,
        call_resolve_ord + 1,
        call_resolve_ord + 5,
        resolve_export_ordinal,
    );
    c.extend_from_slice(&[0x89, 0xC1]); // mov ecx, eax (ordinal)
    c.extend_from_slice(&[0x8B, 0x47, 0x3C]);
    c.extend_from_slice(&[0x8B, 0x84, 0x38, 0x88, 0x00, 0x00, 0x00]);
    c.extend_from_slice(&[0x85, 0xC0]);
    let jz_no_ord = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x01, 0xF8]);
    c.extend_from_slice(&[0x8B, 0x50, 0x1C]);
    c.extend_from_slice(&[0x48, 0x01, 0xFA]);
    c.extend_from_slice(&[0x89, 0xC8]);
    c.extend_from_slice(&[0x8B, 0x04, 0x82]);
    c.extend_from_slice(&[0x48, 0x01, 0xF8]);
    c.extend_from_slice(&[0xC3]);
    patch_rel32(&mut c, jz_no_ord + 2, jz_no_ord + 6, no_exp);

    for at in fail_jumps {
        patch_rel32(
            &mut c,
            at + 2,
            chunk_text_off as usize + at + 6,
            fail_label,
        );
    }

    c
}

/// Export ordinal-0 tail: rbx=module → call functions[0] → ExitProcess.
fn gen_h00_export_call_tail(
    meta: &SelfhostMeta,
    text_rva: u32,
    chunk_text_off: u32,
    fail_label: usize,
) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();
    let mut fail_jumps: Vec<usize> = Vec::new();

    c.extend_from_slice(&[0x8B, 0x73, 0x3C]);
    c.extend_from_slice(&[0x8B, 0x84, 0x33, 0x88, 0x00, 0x00, 0x00]);
    c.extend_from_slice(&[0x85, 0xC0]);
    fail_jumps.push(c.len());
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x8D, 0x3C, 0x03]);
    c.extend_from_slice(&[0x8B, 0x47, 0x1C]);
    c.extend_from_slice(&[0x48, 0x01, 0xD8]); // add rax, rbx — functions RVA is image-relative
    c.extend_from_slice(&[0x8B, 0x00]);
    c.extend_from_slice(&[0x48, 0x01, 0xD8]);
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]); // shadow for Win64 call convention
    c.extend_from_slice(&[0xFF, 0xD0]); // call export (yoyo_runtime_selfhost_main)
    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);
    c.extend_from_slice(&[0x89, 0xC1]);
    emit_call_iat_merged(&mut c, text_rva, chunk_text_off, meta.iat_rva, IAT_EXIT_PROCESS);

    for at in fail_jumps {
        patch_rel32(
            &mut c,
            at + 2,
            chunk_text_off as usize + at + 6,
            fail_label,
        );
    }
    c
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

    let prelude_text_off = code_base_off + H00_PROLOGUE_LEN;

    // Size pass (placeholder fail_label) then emit with real fail epilogue offset.
    let prelude_len =
        gen_h00_read_sidecar_prelude(meta, text_rva, prelude_text_off, usize::MAX).len();
    let map_text_off_m = prelude_text_off + prelude_len as u32;
    let map_len =
        gen_h00_manual_map_body(text_rva, map_text_off_m, meta.iat_rva, usize::MAX).len();
    let tail_text_off_m = map_text_off_m + map_len as u32;
    let tail_len =
        gen_h00_export_call_tail(meta, text_rva, tail_text_off_m, usize::MAX).len();
    let fail_label = code_base_off as usize
        + H00_PROLOGUE_LEN as usize
        + prelude_len
        + map_len
        + tail_len;

    c.extend_from_slice(&gen_h00_read_sidecar_prelude(
        meta,
        text_rva,
        prelude_text_off,
        fail_label,
    ));
    let map_text_off = code_base_off + c.len() as u32;
    c.extend_from_slice(&gen_h00_manual_map_body(
        text_rva,
        map_text_off,
        meta.iat_rva,
        fail_label,
    ));
    let tail_text_off = code_base_off + c.len() as u32;
    c.extend_from_slice(&gen_h00_export_call_tail(
        meta,
        text_rva,
        tail_text_off,
        fail_label,
    ));

    c.extend_from_slice(&[0xB9, 0x01, 0x00, 0x00, 0x00]);
    emit_call_iat_merged(
        &mut c,
        text_rva,
        code_base_off,
        meta.iat_rva,
        IAT_EXIT_PROCESS,
    );
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
        let fail = 0x50_000usize;
        let body = gen_h00_read_sidecar_prelude(&meta, 0x1000, 17_823, fail);
        assert!(body.len() > 80, "prelude should be substantial");
        assert!(
            body.len() < 220,
            "file-read prelude should stay <220B (got {}B)",
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
            body.len() > 400 && body.len() < 1100,
            "manual-map H_00 stub should fit OW-STUB pin [40,1100] (got {}B)",
            body.len()
        );
        // No LoadLibraryA ROR13 hash needle (0x8E 0x4E 0x0E 0xEC)
        assert!(
            !body.windows(4).any(|w| w == [0x8E, 0x4E, 0x0E, 0xEC]),
            "stub must not embed LoadLibraryA hash"
        );
    }
}
