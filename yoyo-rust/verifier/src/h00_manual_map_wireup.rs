//! OW-IAT wire-up scaffold: H_00 manual-map path (post spike `pe_manual_map.rs`).
//!
//! Phase 1 (this module): file-read prelude emit — CreateFileA / ReadFile / VirtualAlloc /
//! CloseHandle using existing r15+0 IAT slots (matches `pe_link::KERNEL32_IO_FUNCS`).
//! Phase 2: inline manual-map x64 (sections + DIR64 reloc + import resolve via PEB walk).
//! Phase 3: three-peer lockstep (JS `win32-h00-selfhost.js` + asm delegate).
//!
//! **Not wired into `gen_h00_selfhost_main` yet** — PEB ROR13 LoadLibrary path remains live.

use crate::win32_selfhost::SelfhostMeta;

/// IAT slots at r15+0 (see pe_link KERNEL32_IO_FUNCS).
pub const IAT_VIRTUAL_ALLOC: u32 = 0;
pub const IAT_CREATE_FILE: u32 = 1;
pub const IAT_READ_FILE: u32 = 2;
pub const IAT_CLOSE_HANDLE: u32 = 4;

/// Stack scratch for ReadFile nNumberOfBytesRead (above shadow 0x28).
const READ_BYTES_STACK_OFF: u8 = 0x30;

fn patch_rel32(c: &mut [u8], disp_off: usize, from: usize, to: usize) {
    let rel = to as i32 - from as i32;
    c[disp_off..disp_off + 4].copy_from_slice(&rel.to_le_bytes());
}

fn emit_call_iat_merged(c: &mut Vec<u8>, text_rva: u32, code_base_off: u32, iat_rva: u32, slot: u32) {
    let at = c.len();
    c.extend_from_slice(&[0xFF, 0x15, 0, 0, 0, 0]);
    let next_rva = text_rva + code_base_off + at as u32 + 6;
    let disp = (iat_rva + slot * 8) as i32 - next_rva as i32;
    c[at + 2..at + 6].copy_from_slice(&disp.to_le_bytes());
}

fn fix_rip_disp(
    c: &mut [u8],
    disp_off: usize,
    text_rva: u32,
    code_base_off: u32,
    insn_end: usize,
    target_rva: u32,
) {
    let next = (text_rva + code_base_off + insn_end as u32) as i32;
    let disp = target_rva as i32 - next;
    c[disp_off..disp_off + 4].copy_from_slice(&disp.to_le_bytes());
}

/// Emit x64 that reads cwd sidecar `yoyo_rt.dll` into a VirtualAlloc buffer.
///
/// On success: `r12` = file bytes pointer, `r13d` = byte count (caller frees via VirtualFree later).
/// On failure: jumps to `fail_label` (caller patches rel32 at `jz_fail_*` sites).
///
/// Preserves `r15` (.data base). Uses `rbx` as file handle during read.
pub fn gen_h00_read_sidecar_prelude(
    meta: &SelfhostMeta,
    text_rva: u32,
    code_base_off: u32,
    fail_label: usize,
) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();

    // sub rsp, 0x48 (shadow + ReadFile out-param + align)
    c.extend_from_slice(&[0x48, 0x83, 0xEC, 0x48]);

    // lea rcx, [rip+yoyo_rt.dll]
    let lea_path = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x0D, 0, 0, 0, 0]);
    // CreateFileA(GENERIC_READ, OPEN_EXISTING)
    c.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x80]); // edx = GENERIC_READ
    c.extend_from_slice(&[0x45, 0x31, 0xC0]); // xor r8d,r8d
    c.extend_from_slice(&[0x41, 0xB9, 0x03, 0x00, 0x00, 0x00]); // r9d = OPEN_EXISTING
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00]); // [rsp+20]=0
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00]); // [rsp+28]=0
    emit_call_iat_merged(&mut c, text_rva, code_base_off, meta.iat_rva, IAT_CREATE_FILE);
    c.extend_from_slice(&[0x48, 0x83, 0xF8, 0xFF]); // cmp rax,-1
    let jz_no_file = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x48, 0x89, 0xC3]); // mov rbx, rax (handle)

    // VirtualQuery-style size: ReadFile in loop with 64K chunks — spike uses single ReadFile
    // after VirtualAlloc(max=512K) for yoyo_rt.dll (~141KB).
    c.extend_from_slice(&[0x31, 0xC9]); // xor ecx, ecx
    c.extend_from_slice(&[0xBA, 0x00, 0x00, 0x08, 0x00]); // edx = 512K
    c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x30, 0x00, 0x00]); // r8 = MEM_COMMIT|RESERVE
    c.extend_from_slice(&[0x41, 0xB9, 0x04, 0x00, 0x00, 0x00]); // r9 = PAGE_READWRITE
    emit_call_iat_merged(&mut c, text_rva, code_base_off, meta.iat_rva, IAT_VIRTUAL_ALLOC);
    c.extend_from_slice(&[0x48, 0x85, 0xC0]); // test rax,rax
    let jz_no_buf = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    c.extend_from_slice(&[0x49, 0x89, 0xC4]); // mov r12, rax (file buffer)

    // ReadFile(rbx, r12, 512K, &stack, NULL)
    c.extend_from_slice(&[0x48, 0x89, 0xD9]); // mov rcx, rbx
    c.extend_from_slice(&[0x4C, 0x89, 0xE2]); // mov rdx, r12
    c.extend_from_slice(&[0x41, 0xB8, 0x00, 0x00, 0x08, 0x00]); // r8d = 512K
    c.extend_from_slice(&[0x4C, 0x8D, 0x4C, 0x24, READ_BYTES_STACK_OFF]); // lea r9, [rsp+30h]
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00]);
    emit_call_iat_merged(&mut c, text_rva, code_base_off, meta.iat_rva, IAT_READ_FILE);

    // CloseHandle(rbx)
    c.extend_from_slice(&[0x48, 0x89, 0xD9]);
    emit_call_iat_merged(&mut c, text_rva, code_base_off, meta.iat_rva, IAT_CLOSE_HANDLE);

    // r13d = bytes read; fail if zero
    c.extend_from_slice(&[0x44, 0x8B, 0x6C, 0x24, READ_BYTES_STACK_OFF]); // mov r13d, [rsp+30h]
    c.extend_from_slice(&[0x45, 0x85, 0xED]); // test r13d,r13d
    let jz_empty = c.len();
    c.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);

    c.extend_from_slice(&[0x48, 0x83, 0xC4, 0x48]); // add rsp, 0x48

    fix_rip_disp(
        &mut c,
        lea_path + 3,
        text_rva,
        code_base_off,
        lea_path + 7,
        meta.temp_name_rva,
    );

    // Patch fail jumps to caller's fail_label (relative from each jz+6).
    let fail = fail_label;
    for at in [jz_no_file, jz_no_buf, jz_empty] {
        patch_rel32(&mut c, at + 2, at + 6, fail);
    }

    c
}

/// Estimated total H_00 stub span once manual-map body is appended (for gate pins).
pub fn estimate_manual_map_stub_span(file_read_len: usize) -> usize {
    // PEB import resolve (~120B) + section copy loop (~80B) + reloc loop (~60B) +
    // export ordinal-0 (~40B) + epilogue (~20B) — conservative.
    const MANUAL_MAP_BODY_EST: usize = 300;
    file_read_len + MANUAL_MAP_BODY_EST
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::win32_selfhost::SelfhostMeta;

    #[test]
    fn read_sidecar_prelude_nonempty_and_bounded() {
        let meta = SelfhostMeta {
            temp_name_rva: 0x30_000,
            export_name_rva: 0,
            dll_embed_rva: 0,
            dll_embed_size: 0,
            iat_rva: 0x20_000,
            import_dir_rva: 0,
            import_dir_size: 0,
        };
        let fail = 0x50_000usize; // absolute offset in final H_00 stub (caller responsibility)
        let body = gen_h00_read_sidecar_prelude(&meta, 0x1000, 17_823, fail);
        assert!(body.len() > 80, "prelude should be substantial");
        assert!(
            body.len() < 220,
            "file-read prelude should stay <220B (got {}B)",
            body.len()
        );
        let est = estimate_manual_map_stub_span(body.len());
        assert!(
            est > 300 && est < 900,
            "estimated full manual-map stub {est} should fit OW-STUB pin [40,900]"
        );
    }
}
