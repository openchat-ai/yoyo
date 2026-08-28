//! Win32 selfhost startup x64 + embedded runtime DLL in PE .data.

use crate::types::{IsaError, IsaResult};

pub const RUNTIME_DLL_NAME: &str = "yoyo_runtime.dll";

const TEMP_DLL_NAME: &[u8] = b"yoyo_rt.dll\0";

/// Merged kernel32 IAT slots (same base as platform_io r15+0). See pe_link prepend_win32_io_iat.
pub const IAT_GET_TEMPPATH: u32 = 5;
pub const IAT_LSTRCAT: u32 = 6;
pub const IAT_LOADLIBRARY: u32 = 7;
pub const IAT_GETPROCADDRESS: u32 = 8;
pub const IAT_EXIT_PROCESS: u32 = 9;

pub struct SelfhostMeta {
    pub temp_name_rva: u32,
    pub export_name_rva: u32,
    pub dll_embed_rva: u32,
    pub dll_embed_size: u32,
    pub iat_rva: u32,
    pub import_dir_rva: u32,
    pub import_dir_size: u32,
}

const KERNEL32_FUNCS: &[&str] = &[
    "GetTempPathA",
    "lstrcatA",
    "CreateFileA",
    "WriteFile",
    "CloseHandle",
    "LoadLibraryA",
    "GetProcAddress",
    "ExitProcess",
];

pub fn runtime_dll_bytes() -> IsaResult<Vec<u8>> {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    for path in [
        root.join("target/release/yoyo_runtime.dll"),
        root.join("target/debug/yoyo_runtime.dll"),
        root.join("target-selfhost/release/yoyo_runtime.dll"),
    ] {
        if path.is_file() {
            return std::fs::read(&path).map_err(|e| IsaError::IoError {
                msg: format!("read {}: {e}", path.display()),
            });
        }
    }
    Err(IsaError::IoError {
        msg: "yoyo_runtime.dll not found — run `cargo build --release -p yoyo-runtime`".into(),
    })
}

pub fn build_selfhost_metadata(
    user_data: &[u8],
    data_rva: u32,
    dll_bytes: &[u8],
) -> IsaResult<(Vec<u8>, SelfhostMeta)> {
    let mut blob = user_data.to_vec();
    while blob.len() % 16 != 0 {
        blob.push(0);
    }
    let base = blob.len();

    let kernel32_name = b"kernel32.dll\0";
    let export_name = b"yoyo_runtime_selfhost_main\0";

    let mut hint_names: Vec<Vec<u8>> = Vec::new();
    for name in KERNEL32_FUNCS {
        let mut hn = Vec::new();
        hn.extend_from_slice(&0u16.to_le_bytes());
        hn.extend_from_slice(name.as_bytes());
        hn.push(0);
        while hn.len() % 2 != 0 {
            hn.push(0);
        }
        hint_names.push(hn);
    }

    let desc_size = 40;
    let kern_off = desc_size;
    let temp_off = kern_off + kernel32_name.len();
    let export_off = temp_off + TEMP_DLL_NAME.len();

    let mut hn_off = export_off + export_name.len();
    let mut hn_rvas: Vec<u32> = Vec::new();
    for hn in &hint_names {
        hn_rvas.push(data_rva + (base + hn_off) as u32);
        hn_off += hn.len();
    }

    let n = KERNEL32_FUNCS.len();
    let ilt_off = hn_off;
    let iat_off = ilt_off + (n + 1) * 8;
    let embed_off = iat_off + (n + 1) * 8;
    let embed_pad = (16 - (dll_bytes.len() % 16)) % 16;
    let total = embed_off + dll_bytes.len() + embed_pad;

    blob.resize(base + total, 0);
    let rva = |off: usize| data_rva + (base + off) as u32;

    write_u32(&mut blob, base, rva(ilt_off));
    write_u32(&mut blob, base + 12, rva(kern_off));
    write_u32(&mut blob, base + 16, rva(iat_off));

    blob[base + kern_off..base + kern_off + kernel32_name.len()].copy_from_slice(kernel32_name);
    blob[base + temp_off..base + temp_off + TEMP_DLL_NAME.len()].copy_from_slice(TEMP_DLL_NAME);
    blob[base + export_off..base + export_off + export_name.len()].copy_from_slice(export_name);

    let mut off = export_off + export_name.len();
    for hn in &hint_names {
        blob[base + off..base + off + hn.len()].copy_from_slice(hn);
        off += hn.len();
    }

    for (i, &hn_rva) in hn_rvas.iter().enumerate() {
        write_u64(&mut blob, base + ilt_off + i * 8, hn_rva as u64);
        write_u64(&mut blob, base + iat_off + i * 8, hn_rva as u64);
    }

    blob[base + embed_off..base + embed_off + dll_bytes.len()].copy_from_slice(dll_bytes);

    Ok((
        blob,
        SelfhostMeta {
            temp_name_rva: rva(temp_off),
            export_name_rva: rva(export_off),
            dll_embed_rva: rva(embed_off),
            dll_embed_size: dll_bytes.len() as u32,
            iat_rva: rva(iat_off),
            import_dir_rva: rva(0),
            import_dir_size: desc_size as u32,
        },
    ))
}

/// Append runtime DLL + name strings for H_00 path (merged IAT at r15+0, no second import dir).
pub fn append_h00_runtime_data(
    user_data: &[u8],
    data_rva: u32,
    dll_bytes: &[u8],
) -> IsaResult<(Vec<u8>, SelfhostMeta)> {
    let export_name = b"yoyo_runtime_selfhost_main\0";
    let mut blob = user_data.to_vec();
    while blob.len() % 16 != 0 {
        blob.push(0);
    }
    let base = blob.len();
    let temp_off = 0usize;
    let export_off = temp_off + TEMP_DLL_NAME.len();
    let embed_off = export_off + export_name.len();
    let embed_pad = (16 - (dll_bytes.len() % 16)) % 16;
    let total = embed_off + dll_bytes.len() + embed_pad;
    blob.resize(base + total, 0);
    let rva = |off: usize| data_rva + (base + off) as u32;

    blob[base + temp_off..base + temp_off + TEMP_DLL_NAME.len()].copy_from_slice(TEMP_DLL_NAME);
    blob[base + export_off..base + export_off + export_name.len()].copy_from_slice(export_name);
    blob[base + embed_off..base + embed_off + dll_bytes.len()].copy_from_slice(dll_bytes);

    Ok((
        blob,
        SelfhostMeta {
            temp_name_rva: rva(temp_off),
            export_name_rva: rva(export_off),
            dll_embed_rva: rva(embed_off),
            dll_embed_size: dll_bytes.len() as u32,
            iat_rva: data_rva,
            import_dir_rva: 0,
            import_dir_size: 0,
        },
    ))
}

/// Stage 9-A H_00 runtime body: extract embedded runtime via Stage 8-A merged
/// kernel32 IAT (CreateFile/WriteFile/LoadLibrary), invoke compile, ExitProcess.
/// PE entry is `jmp H_00` (not `call`), so this must never return.
pub fn gen_h00_selfhost_main(
    meta: &SelfhostMeta,
    _data_rva: u32,
    text_rva: u32,
    pe_startup_len: u32,
    main_user_off: u32,
    _h20_user_off: u32,
) -> Vec<u8> {
    let main_text_off = pe_startup_len + main_user_off;
    let mut c: Vec<u8> = Vec::new();

    // Same Win64 stack frame as gen2rt entry (PATH buffer at [rsp+0x80] after sub 0x208).
    c.extend_from_slice(&[0x53, 0x41, 0x54, 0x41, 0x55, 0x56]);
    c.extend_from_slice(&[0x48, 0x81, 0xEC, 0x08, 0x02, 0x00, 0x00]);

    gen_dll_extract_invoke(&mut c, text_rva, main_text_off, meta, true, true);
    // exit_process=true → ExitProcess; never returns.
    c
}

/// Shared DLL extract + invoke sequence (gen2rt entry or H_00 body).
fn gen_dll_extract_invoke(
    c: &mut Vec<u8>,
    text_rva: u32,
    code_base_off: u32,
    meta: &SelfhostMeta,
    exit_process: bool,
    nested: bool,
) {
    const PATH_OFF: u32 = 0x80;
    let body_start = c.len();

    if !nested {
        c.extend_from_slice(&[0x53, 0x41, 0x54, 0x41, 0x55, 0x56]);
        c.extend_from_slice(&[0x48, 0x81, 0xEC, 0x08, 0x02, 0x00, 0x00]);
    }

    let code_off = code_base_off + body_start as u32;

    emit_mov_ecx_imm(c, 260);
    emit_lea_reg_rsp(c, 2, PATH_OFF);
    emit_call_iat_merged(c, text_rva, code_base_off, meta.iat_rva, IAT_GET_TEMPPATH);

    emit_lea_reg_rsp(c, 1, PATH_OFF);
    let lea_temp = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x15, 0, 0, 0, 0]);
    emit_call_iat_merged(c, text_rva, code_base_off, meta.iat_rva, IAT_LSTRCAT);

    let lea_embed = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x35, 0, 0, 0, 0]);
    emit_mov_r13_imm32(c, meta.dll_embed_size);

    emit_lea_reg_rsp(c, 1, PATH_OFF);
    emit_mov_edx_imm(c, 0x4000_0000);
    c.extend_from_slice(&[0x45, 0x31, 0xC0, 0x45, 0x31, 0xC9]);
    emit_mov_dword_rsp(c, 0x20, 2);
    emit_mov_dword_rsp(c, 0x28, 0x80);
    emit_mov_qword_rsp(c, 0x30, 0);
    emit_call_iat_merged(c, text_rva, code_base_off, meta.iat_rva, 1); // CreateFileA

    c.extend_from_slice(&[0x49, 0x89, 0xC4]);
    c.extend_from_slice(&[0x4C, 0x89, 0xE1, 0x48, 0x89, 0xF2, 0x4D, 0x89, 0xE8]);
    emit_lea_reg_rsp(c, 9, 0x40);
    emit_mov_qword_rsp(c, 0x20, 0);
    emit_call_iat_merged(c, text_rva, code_base_off, meta.iat_rva, 3); // WriteFile

    c.extend_from_slice(&[0x4C, 0x89, 0xE1]);
    emit_call_iat_merged(c, text_rva, code_base_off, meta.iat_rva, 4); // CloseHandle

    emit_lea_reg_rsp(c, 1, PATH_OFF);
    emit_call_iat_merged(c, text_rva, code_base_off, meta.iat_rva, IAT_LOADLIBRARY);
    c.extend_from_slice(&[0x48, 0x89, 0xC3]);

    c.extend_from_slice(&[0x48, 0x89, 0xD9]);
    let lea_export = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x15, 0, 0, 0, 0]);
    emit_call_iat_merged(c, text_rva, code_base_off, meta.iat_rva, IAT_GETPROCADDRESS);

    c.extend_from_slice(&[0xFF, 0xD0]); // call runtime export
    if exit_process {
        // mov ecx, eax ; ExitProcess(exit_code) — never returns (PE entry was jmp, not call).
        c.extend_from_slice(&[0x89, 0xC1]);
        emit_call_iat_merged(c, text_rva, code_base_off, meta.iat_rva, IAT_EXIT_PROCESS);
    }
    let _ = code_off;

    // Ensure fixup sites are in bounds (lea disp32 writes 4 bytes past opcode).
    let need = lea_export + 7;
    if c.len() < need {
        c.resize(need, 0x90);
    }

    fix_rip_disp(
        c,
        lea_temp + 3,
        text_rva,
        code_base_off,
        lea_temp + 7,
        meta.temp_name_rva,
    );
    fix_rip_disp(
        c,
        lea_embed + 3,
        text_rva,
        code_base_off,
        lea_embed + 7,
        meta.dll_embed_rva,
    );
    fix_rip_disp(
        c,
        lea_export + 3,
        text_rva,
        code_base_off,
        lea_export + 7,
        meta.export_name_rva,
    );
}

fn emit_call_iat_merged(c: &mut Vec<u8>, text_rva: u32, code_base_off: u32, iat_rva: u32, slot: u32) {
    let at = c.len();
    c.extend_from_slice(&[0xFF, 0x15, 0, 0, 0, 0]);
    let next_rva = text_rva + code_base_off + at as u32 + 6;
    let disp = (iat_rva + slot * 8) as i32 - next_rva as i32;
    c[at + 2..at + 6].copy_from_slice(&disp.to_le_bytes());
}

pub fn gen_selfhost_startup(meta: &SelfhostMeta) -> Vec<u8> {
    let text_rva = 0x1000u32;
    let startup_off = 13u32;
    const PATH_OFF: u32 = 0x80;
    let mut c: Vec<u8> = Vec::new();

    c.extend_from_slice(&[0x53, 0x41, 0x54, 0x41, 0x55, 0x56]);
    c.extend_from_slice(&[0x48, 0x81, 0xEC, 0x08, 0x02, 0x00, 0x00]);

    emit_mov_ecx_imm(&mut c, 260);
    emit_lea_reg_rsp(&mut c, 2, PATH_OFF);
    emit_call_iat(&mut c, text_rva, startup_off, meta.iat_rva, 0);

    emit_lea_reg_rsp(&mut c, 1, PATH_OFF);
    let lea_temp = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x15, 0, 0, 0, 0]);
    emit_call_iat(&mut c, text_rva, startup_off, meta.iat_rva, 1);

    let lea_embed = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x35, 0, 0, 0, 0]);
    emit_mov_r13_imm32(&mut c, meta.dll_embed_size);

    emit_lea_reg_rsp(&mut c, 1, PATH_OFF);
    emit_mov_edx_imm(&mut c, 0x4000_0000);
    c.extend_from_slice(&[0x45, 0x31, 0xC0, 0x45, 0x31, 0xC9]);
    emit_mov_dword_rsp(&mut c, 0x20, 2);
    emit_mov_dword_rsp(&mut c, 0x28, 0x80);
    emit_mov_qword_rsp(&mut c, 0x30, 0);
    emit_call_iat(&mut c, text_rva, startup_off, meta.iat_rva, 2);
    c.extend_from_slice(&[0x49, 0x89, 0xC4]);

    c.extend_from_slice(&[0x4C, 0x89, 0xE1, 0x48, 0x89, 0xF2, 0x4D, 0x89, 0xE8]);
    emit_lea_reg_rsp(&mut c, 9, 0x40);
    emit_mov_qword_rsp(&mut c, 0x20, 0);
    emit_call_iat(&mut c, text_rva, startup_off, meta.iat_rva, 3);

    c.extend_from_slice(&[0x4C, 0x89, 0xE1]);
    emit_call_iat(&mut c, text_rva, startup_off, meta.iat_rva, 4);

    emit_lea_reg_rsp(&mut c, 1, PATH_OFF);
    emit_call_iat(&mut c, text_rva, startup_off, meta.iat_rva, 5);
    c.extend_from_slice(&[0x48, 0x89, 0xC3]);

    c.extend_from_slice(&[0x48, 0x89, 0xD9]);
    let lea_export = c.len();
    c.extend_from_slice(&[0x48, 0x8D, 0x15, 0, 0, 0, 0]);
    emit_call_iat(&mut c, text_rva, startup_off, meta.iat_rva, 6);

    c.extend_from_slice(&[0xFF, 0xD0, 0x89, 0xC1]);
    emit_call_iat(&mut c, text_rva, startup_off, meta.iat_rva, 7);

    fix_rip_disp(&mut c, lea_temp + 3, text_rva, startup_off, lea_temp + 7, meta.temp_name_rva);
    fix_rip_disp(
        &mut c,
        lea_embed + 3,
        text_rva,
        startup_off,
        lea_embed + 7,
        meta.dll_embed_rva,
    );
    fix_rip_disp(
        &mut c,
        lea_export + 3,
        text_rva,
        startup_off,
        lea_export + 7,
        meta.export_name_rva,
    );

    c
}

fn fix_rip_disp(
    c: &mut [u8],
    disp_off: usize,
    text_rva: u32,
    startup_off: u32,
    insn_end: usize,
    target_rva: u32,
) {
    let next = (text_rva + startup_off + insn_end as u32) as i32;
    let disp = target_rva as i32 - next;
    c[disp_off..disp_off + 4].copy_from_slice(&disp.to_le_bytes());
}

fn emit_call_iat(c: &mut Vec<u8>, text_rva: u32, startup_off: u32, iat_rva: u32, slot: u32) {
    let at = c.len();
    c.extend_from_slice(&[0xFF, 0x15, 0, 0, 0, 0]);
    fix_rip_disp(c, at + 2, text_rva, startup_off, at + 6, iat_rva + slot * 8);
}

/// lea r64, [rsp+disp32]. `reg`: rcx=1, rdx=2, r9=9 (REX.R for reg≥8).
fn emit_lea_reg_rsp(c: &mut Vec<u8>, reg: u8, disp: u32) {
    let rex_r = if reg >= 8 { 0x04 } else { 0 };
    let rm = reg & 7;
    c.push(0x48 | rex_r);
    c.push(0x8D);
    c.push(0x84 | (rm << 3)); // mod=10, reg=rm, r/m=100 (SIB)
    c.push(0x24); // SIB base=rsp, index=none
    c.extend_from_slice(&disp.to_le_bytes());
}

fn emit_mov_ecx_imm(c: &mut Vec<u8>, v: u32) {
    c.extend_from_slice(&[0xB9]);
    c.extend_from_slice(&v.to_le_bytes());
}

fn emit_mov_edx_imm(c: &mut Vec<u8>, v: u32) {
    c.extend_from_slice(&[0xBA]);
    c.extend_from_slice(&v.to_le_bytes());
}

fn emit_mov_r13_imm32(c: &mut Vec<u8>, v: u32) {
    c.extend_from_slice(&[0x41, 0xBD]);
    c.extend_from_slice(&v.to_le_bytes());
}

fn emit_mov_dword_rsp(c: &mut Vec<u8>, off: u8, v: u32) {
    c.extend_from_slice(&[0xC7, 0x44, 0x24, off]);
    c.extend_from_slice(&v.to_le_bytes());
}

fn emit_mov_qword_rsp(c: &mut Vec<u8>, off: u8, v: u64) {
    // mov qword [rsp+off], imm32 — imm is sign-extended; only 4 immediate bytes.
    c.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, off]);
    c.extend_from_slice(&(v as i32 as u32).to_le_bytes());
}

fn write_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn write_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata_layout_with_embed() {
        let dll = vec![0x4D, 0x5A, 0x90, 0x00];
        let (_, meta) = build_selfhost_metadata(&[1, 2, 3], 0x20000, &dll).unwrap();
        assert!(meta.dll_embed_rva >= 0x20000);
        assert_eq!(meta.dll_embed_size, 4);
    }

    #[test]
    fn embedded_startup_nonempty() {
        let dll = vec![0u8; 64];
        let (_, meta) = build_selfhost_metadata(&[], 0x30000, &dll).unwrap();
        assert!(gen_selfhost_startup(&meta).len() > 64);
    }
}
