//! Minimal PE32+ linker (PROMPT-v3 Phase 1).
//! Produces a valid Win10-compatible executable wrapping emitted .text + .data.
//! Data section size floor: 0x38000 (Phase 2 root-cause fix).

use crate::platform_io;
use crate::types::IsaResult;
use crate::win32_selfhost;

/// IMAGE_DOS_HEADER + PE signature offset convention.
const OUTPUT_DATA_NEED: u32 = 0x38000;

const KERNEL32_IO_FUNCS: &[&str] = &[
    "VirtualAlloc",
    "CreateFileA",
    "ReadFile",
    "WriteFile",
    "CloseHandle",
    // Stage 9-A / post-v1.0 H_00: cwd sidecar LoadLibrary (was slots 5–6).
    // Stage 11-B dropped GetTempPathA/lstrcatA; post-v1.0 dropped extract+WriteFile
    // embed + GetProcAddress; deeper OW-IAT: dropped LoadLibraryA from IAT — H_00
    // resolves it via PEB→kernel32 export hash walk (still host LoadLibrary; CUT).
    "ExitProcess",
];

/// Loaded by the Windows loader before H_00 runs so manual-map PEB walk can resolve
/// MSVC Rust sidecar imports (gen1 otherwise only maps kernel32).
const PRELOAD_RUNTIME_DLL_IMPORTS: &[(&str, &str)] = &[
    ("VCRUNTIME140.dll", "memset"),
    ("api-ms-win-crt-runtime-l1-1-0.dll", "_initialize_onexit_table"),
    ("api-ms-win-crt-heap-l1-1-0.dll", "malloc"),
    ("api-ms-win-crt-stdio-l1-1-0.dll", "__stdio_common_vsprintf"),
];

/// Bootstrap scratch for H_00 manual-map (LoadLibraryA / GetProcAddress / kernel32 / phase byte).
/// Placed after import metadata in `prepend_win32_io_iat` — must not overlap IAT or descriptors.
pub const WIN32_IO_H00_SCRATCH_BYTES: usize = 32;

/// Offset from r15 / `.data` base to H_00 scratch (pinned by `h00_scratch_off_pinned`).
pub const WIN32_IO_H00_SCRATCH_OFF: u32 = 0x25B;

fn hint_name_bytes(func: &str) -> Vec<u8> {
    let mut hn = Vec::new();
    hn.extend_from_slice(&0u16.to_le_bytes());
    hn.extend_from_slice(func.as_bytes());
    hn.push(0);
    while hn.len() % 2 != 0 {
        hn.push(0);
    }
    hn
}

/// Prepend kernel32 IAT at r15+0 for Stage 8 platform I/O emit.
fn prepend_win32_io_iat(user_data: &[u8], data_rva: u32) -> (Vec<u8>, u32, u32) {
    const DESC_SIZE: usize = 20; // IMAGE_IMPORT_DESCRIPTOR (was 40 — broke loader chain)
    let desc_size = DESC_SIZE;
    let kernel32_name = b"kernel32.dll\0";
    let kern_n = KERNEL32_IO_FUNCS.len();
    let preload_n = PRELOAD_RUNTIME_DLL_IMPORTS.len();
    let num_desc = 1 + preload_n + 1; // trailing null descriptor

    let kern_iat_off = 0usize;
    let preload_iat_off = kern_n * 8;
    let desc_off = preload_iat_off + preload_n * 8;

    let kern_hints: Vec<Vec<u8>> = KERNEL32_IO_FUNCS.iter().map(|s| hint_name_bytes(s)).collect();
    let preload_hints: Vec<Vec<u8>> = PRELOAD_RUNTIME_DLL_IMPORTS
        .iter()
        .map(|(_, f)| hint_name_bytes(f))
        .collect();

    let strings_off = desc_off + desc_size * num_desc;
    let mut cursor = strings_off;

    let kern_name_off = cursor;
    cursor += kernel32_name.len();

    let mut kern_hn_off = Vec::new();
    for hn in &kern_hints {
        kern_hn_off.push(cursor);
        cursor += hn.len();
    }
    let kern_ilt_off = cursor;
    cursor += (kern_n + 1) * 8;

    let mut preload_meta: Vec<(usize, usize, usize, usize)> = Vec::new();
    for (i, (dll, _)) in PRELOAD_RUNTIME_DLL_IMPORTS.iter().enumerate() {
        let name_off = cursor;
        cursor += dll.len() + 1;
        let hn_off = cursor;
        cursor += preload_hints[i].len();
        let ilt_off = cursor;
        cursor += 16; // one thunk + null
        let iat_off = preload_iat_off + i * 8;
        preload_meta.push((name_off, hn_off, ilt_off, iat_off));
    }

    let h00_scratch_off = cursor;
    cursor += WIN32_IO_H00_SCRATCH_BYTES;
    let header_end = cursor;
    let pad = align_up_usize(header_end, 16);
    let _ = h00_scratch_off;
    let mut blob = vec![0u8; pad + user_data.len()];
    let user_base = pad;

    // kernel32 descriptor
    write_u32(&mut blob, desc_off, data_rva + kern_ilt_off as u32);
    write_u32(&mut blob, desc_off + 12, data_rva + kern_name_off as u32);
    write_u32(&mut blob, desc_off + 16, data_rva + kern_iat_off as u32);

    for (i, (name_off, hn_off, ilt_off, iat_off)) in preload_meta.iter().enumerate() {
        let at = desc_off + desc_size * (1 + i);
        write_u32(&mut blob, at, data_rva + *ilt_off as u32);
        write_u32(&mut blob, at + 12, data_rva + *name_off as u32);
        write_u32(&mut blob, at + 16, data_rva + *iat_off as u32);
    }

    blob[kern_name_off..kern_name_off + kernel32_name.len()].copy_from_slice(kernel32_name);
    for (off, hn) in kern_hn_off.iter().zip(&kern_hints) {
        blob[*off..*off + hn.len()].copy_from_slice(hn);
    }
    for (i, &hn_off) in kern_hn_off.iter().enumerate() {
        let hn_rva = data_rva + hn_off as u32;
        write_u64(&mut blob, kern_ilt_off + i * 8, hn_rva as u64);
        write_u64(&mut blob, kern_iat_off + i * 8, hn_rva as u64);
    }

    for (i, ((dll, _), (name_off, hn_off, ilt_off, iat_off))) in PRELOAD_RUNTIME_DLL_IMPORTS
        .iter()
        .zip(preload_meta.iter())
        .enumerate()
    {
        blob[*name_off..*name_off + dll.len() + 1]
            .copy_from_slice(&[dll.as_bytes(), b"\0"].concat());
        let hn = &preload_hints[i];
        blob[*hn_off..*hn_off + hn.len()].copy_from_slice(hn);
        let hn_rva = data_rva + *hn_off as u32;
        write_u64(&mut blob, *ilt_off, hn_rva as u64);
        write_u64(&mut blob, *ilt_off + 8, 0);
        write_u64(&mut blob, *iat_off, hn_rva as u64);
    }

    blob[user_base..user_base + user_data.len()].copy_from_slice(user_data);
    let import_dir_bytes = desc_size * num_desc;
    (
        blob,
        data_rva + desc_off as u32,
        import_dir_bytes as u32,
    )
}

fn align_up_usize(v: usize, a: usize) -> usize {
    (v + a - 1) & !(a - 1)
}

pub struct PeImage {
    pub bytes: Vec<u8>,
}

/// Wrap raw x64 code (+ optional data) in a PE32+ image.
/// Entry: sets up R15 → .data (state base), then jumps to `code`.
pub fn link_pe(code: &[u8], data: &[u8]) -> IsaResult<PeImage> {
    link_pe_win32(code, data, &[])
}

/// Win32 link with optional H_00 in-process selfhost (Stage 9-A).
/// When `handler_offsets` contains H_20/H_21 I/O handlers, patch H_00 entry to
/// LoadLibraryA cwd sidecar `yoyo_rt.dll` then runtime compile (PE entry stays lea r15 → H_00).
/// Post-v1.0: H_00 path does **not** exact-embed the Rust runtime DLL (OW-RT shrink; still CUT).
///
/// Stage 13-A: the H_00 branch is the approved **seed/link host** path (same bytes as
/// `yoyo bootstrap` without `--selfhost`). Fail-closed PE size ceiling applied there.
pub fn link_pe_win32(
    code: &[u8],
    data: &[u8],
    handler_offsets: &[(u16, u32, u32)],
) -> IsaResult<PeImage> {
    if should_h00_selfhost(handler_offsets) {
        let pe = link_pe_h00_runtime(code, data, handler_offsets)?;
        // Stage 13-A: pin seed/link host PE surface (keep sync w/ stage13-link-host.ps1).
        if pe.bytes.len() > crate::selfhost::STAGE13_MAX_SEED_PE_BYTES {
            return Err(crate::types::IsaError::PlatformError {
                msg: format!(
                    "Stage 13-A seed/link host PE {} bytes exceeds fail-closed MAX {} (H_00 path)",
                    pe.bytes.len(),
                    crate::selfhost::STAGE13_MAX_SEED_PE_BYTES
                ),
            });
        }
        Ok(pe)
    } else {
        let section_align: u32 = 0x1000;
        let text_rva = section_align;
        let est_text = code.len() as u32 + 0x40;
        let text_vs = align_up(est_text, section_align);
        let data_rva = text_rva + text_vs;
        let (extended, import_dir_rva, import_dir_size) = prepend_win32_io_iat(data, data_rva);
        let _ = platform_io::WIN32_IAT_DATA_RESERVE;
        link_pe_impl(code, &extended, true, import_dir_rva, import_dir_size, None)
    }
}

fn should_h00_selfhost(handler_offsets: &[(u16, u32, u32)]) -> bool {
    let has_load = handler_offsets.iter().any(|(h, _, _)| *h == 0x20);
    let has_write = handler_offsets.iter().any(|(h, _, _)| *h == 0x21);
    has_load && has_write
}

fn handler_off(handler_offsets: &[(u16, u32, u32)], hh: u16) -> Option<u32> {
    handler_offsets
        .iter()
        .find(|(h, _, _)| *h == hh)
        .map(|(_, off, _)| *off)
}

/// Stage 9-A / post-v1.0: gen1 H_00 — patch entry handler; sidecar name strings only
/// (no exact-embed of `yoyo_runtime.dll`). Entry stays lea r15 → H_00.
/// H_00: LoadLibraryA(`yoyo_rt.dll`) → runtime → ExitProcess.
pub fn link_pe_h00_runtime(
    code: &[u8],
    data: &[u8],
    handler_offsets: &[(u16, u32, u32)],
) -> IsaResult<PeImage> {
    const PE_STARTUP_LEN: u32 = 13;
    let section_align: u32 = 0x1000;

    let mut code = code.to_vec();
    if code.len() < 18 {
        return Err(crate::types::IsaError::PlatformError {
            msg: "H_00 selfhost: code too short for entry patch".into(),
        });
    }

    let main_user_off = code.len() as u32;
    // Presence of W-SM H_20/H_21 marks a full-body image (gate only; not CALL targets).
    let h20 = handler_off(handler_offsets, 0x20).ok_or_else(|| {
        crate::types::IsaError::PlatformError {
            msg: "H_00 selfhost: missing H_20 (full-body marker)".into(),
        }
    })?;

    // Two-pass: measure h00_main length, then fix data_rva + rebuild with correct RVAs.
    let probe = win32_selfhost::gen_h00_selfhost_main(
        &win32_selfhost::SelfhostMeta {
            temp_name_rva: 0,
            export_name_rva: 0,
            dll_embed_rva: 0,
            dll_embed_size: 0,
            iat_rva: 0,
            import_dir_rva: 0,
            import_dir_size: 0,
        },
        0,
        section_align,
        PE_STARTUP_LEN,
        main_user_off,
        h20,
    );
    let text_vs = align_up(PE_STARTUP_LEN + code.len() as u32 + probe.len() as u32 + 0x40, section_align);
    let data_rva = section_align + text_vs;

    let with_strings = embed_string_table(data);
    // Prepend IAT before sidecar name strings so RVAs include the IAT header pad.
    let (io_prepended, import_dir_rva, import_dir_size) =
        prepend_win32_io_iat(&with_strings, data_rva);
    let (extended, meta) = win32_selfhost::append_h00_runtime_data(&io_prepended, data_rva)?;

    let h00_main = win32_selfhost::gen_h00_selfhost_main(
        &meta,
        data_rva,
        section_align,
        PE_STARTUP_LEN,
        main_user_off,
        h20,
    );
    code.extend_from_slice(&h00_main);

    // Patch H_00 (user code offset 0): JMP h00_main (not CALL).
    // PE entry is already `jmp H_00`; an extra CALL would leave RSP misaligned by 8
    // vs gen2rt's entry-style frame (sub 0x208), causing movaps AV in LoadLibrary.
    let rel = main_user_off as i32 - 5;
    code[0] = 0xE9;
    code[1..5].copy_from_slice(&rel.to_le_bytes());
    for i in 5..18 {
        code[i] = 0x90;
    }
    let _ = platform_io::WIN32_IAT_DATA_RESERVE;
    link_pe_impl(
        &code,
        &extended,
        true,
        import_dir_rva,
        import_dir_size,
        Some(data_rva),
    )
}

/// Embed default selfhost paths at r15+STR_TABLE_OFF (platform_io layout).
fn embed_string_table(user_data: &[u8]) -> Vec<u8> {
    let table_off = platform_io::STR_TABLE_OFF as usize;
    let need = table_off + platform_io::STR_ENTRY_SIZE as usize * 3;
    let mut blob = user_data.to_vec();
    if blob.len() < need {
        blob.resize(need, 0);
    }
    write_cstr_entry(&mut blob, table_off, b"input.tyb");
    write_cstr_entry(
        &mut blob,
        table_off + platform_io::STR_ENTRY_SIZE as usize,
        b"input.ky",
    );
    write_cstr_entry(
        &mut blob,
        table_off + platform_io::STR_ENTRY_SIZE as usize * 2,
        b"output.exe",
    );
    blob
}

fn write_cstr_entry(blob: &mut [u8], base: usize, s: &[u8]) {
    let n = s.len().min(platform_io::STR_ENTRY_SIZE as usize - 1);
    blob[base..base + n].copy_from_slice(&s[..n]);
    blob[base + n] = 0;
}

/// Wrap raw x64 code with Win32 runtime selfhost startup + HOT table.
/// Entry: extract embedded runtime to %TEMP% → LoadLibraryA → compile → ExitProcess.
/// (gen2rt / Stage 8-C regression path; keeps private GetTempPath IAT via build_selfhost_metadata.)
pub fn link_pe_selfhost(
    code: &[u8],
    data: &[u8],
    hot_table: &[u8],
    embedded_dll: &[u8],
) -> IsaResult<PeImage> {
    let section_align: u32 = 0x1000;
    let text_rva = section_align;
    let dummy_meta = win32_selfhost::SelfhostMeta {
        temp_name_rva: 0,
        export_name_rva: 0,
        dll_embed_rva: 0,
        dll_embed_size: embedded_dll.len() as u32,
        iat_rva: 0,
        import_dir_rva: 0,
        import_dir_size: 40,
    };
    let body_len = win32_selfhost::gen_selfhost_startup(&dummy_meta).len();
    let est_code_len = body_len + code.len() + hot_table.len();
    let text_vs = align_up(est_code_len as u32 + 0x40, section_align);
    let data_rva = text_rva + text_vs;

    let (io_data, _io_imp_rva, _io_imp_sz) = prepend_win32_io_iat(data, data_rva);
    let (extended_data, meta) =
        win32_selfhost::build_selfhost_metadata(&io_data, data_rva, embedded_dll)?;
    let startup_body = win32_selfhost::gen_selfhost_startup(&meta);

    let mut full_code = Vec::new();
    full_code.extend_from_slice(&startup_body);
    full_code.extend_from_slice(code);
    full_code.extend_from_slice(hot_table);

    link_pe_impl(
        &full_code,
        &extended_data,
        true,
        meta.import_dir_rva,
        meta.import_dir_size,
        None,
    )
}

fn link_pe_impl(
    code: &[u8],
    data: &[u8],
    is_selfhost: bool,
    import_dir_rva: u32,
    import_dir_size: u32,
    data_rva_override: Option<u32>,
) -> IsaResult<PeImage> {
    let section_align: u32 = 0x1000;
    let file_align: u32 = 0x200;
    const PE_STARTUP_LEN: u32 = 13;

    let code_raw = align_up(code.len() as u32, file_align);
    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, section_align));
    let data_raw = align_up(data_need, file_align);

    // Layout:
    // 0x0000: DOS + PE headers (1 section-align = 0x1000 file, 0x200 min)
    // text VA 0x1000, data VA 0x1000 + align(startup + code)
    let headers_raw = 0x400u32;
    let text_rva = section_align; // 0x1000
    let text_vs = align_up(code.len() as u32 + PE_STARTUP_LEN + 0x40, section_align);
    let data_rva = data_rva_override.unwrap_or(text_rva + text_vs);
    let data_vs = data_need;

    let size_of_image = align_up(data_rva + data_vs, section_align);
    let size_of_headers = headers_raw;

    let mut img = vec![0u8; (headers_raw + code_raw + data_raw) as usize];

    // DOS header
    img[0] = 0x4D;
    img[1] = 0x5A; // MZ
    write_u32(&mut img, 0x3C, 0x80); // e_lfanew

    // PE signature at 0x80
    img[0x80] = b'P';
    img[0x81] = b'E';

    // COFF header
    write_u16(&mut img, 0x84, 0x8664); // Machine AMD64
    write_u16(&mut img, 0x86, 2); // NumberOfSections
    write_u16(&mut img, 0x94, 0xF0); // SizeOfOptionalHeader
    write_u16(&mut img, 0x96, 0x22); // Characteristics: EXECUTABLE | LARGE_ADDRESS_AWARE

    // Optional header (PE32+)
    let opt = 0x98usize;
    write_u16(&mut img, opt, 0x20B); // PE32+
    img[opt + 2] = 1; // MajorLinkerVersion
    write_u32(&mut img, opt + 16, text_rva); // AddressOfEntryPoint = startup in .text
    write_u64(&mut img, opt + 24, 0x140000000); // ImageBase
    write_u32(&mut img, opt + 32, section_align);
    write_u32(&mut img, opt + 36, file_align);
    write_u16(&mut img, opt + 40, 6); // MajorOperatingSystemVersion
    write_u16(&mut img, opt + 42, 0); // MinorOperatingSystemVersion
    write_u16(&mut img, opt + 44, 0); // MajorImageVersion
    write_u16(&mut img, opt + 46, 0); // MinorImageVersion
    write_u16(&mut img, opt + 48, 6); // MajorSubsystemVersion
    write_u16(&mut img, opt + 50, 0); // MinorSubsystemVersion
    write_u32(&mut img, opt + 56, size_of_image);
    write_u32(&mut img, opt + 60, size_of_headers);
    write_u16(&mut img, opt + 68, 3); // Subsystem = CONSOLE
    write_u16(&mut img, opt + 70, 0x8160); // DllCharacteristics
    write_u64(&mut img, opt + 72, 0x100000); // Stack Reserve
    write_u64(&mut img, opt + 80, 0x1000); // Stack Commit
    write_u64(&mut img, opt + 88, 0x100000); // Heap Reserve
    write_u64(&mut img, opt + 96, 0x1000); // Heap Commit
    write_u32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes

    // Section .text
    let s1 = 0x98 + 0xF0; // 0x188
    write_name(&mut img, s1, b".text");
    write_u32(&mut img, s1 + 8, text_vs);
    write_u32(&mut img, s1 + 12, text_rva);
    write_u32(&mut img, s1 + 16, code_raw);
    write_u32(&mut img, s1 + 20, headers_raw);
    write_u32(&mut img, s1 + 36, 0x60000020); // CODE | EXECUTE | READ

    // Section .data
    let s2 = s1 + 40;
    write_name(&mut img, s2, b".data");
    write_u32(&mut img, s2 + 8, data_vs);
    write_u32(&mut img, s2 + 12, data_rva);
    write_u32(&mut img, s2 + 16, data_raw);
    write_u32(&mut img, s2 + 20, headers_raw + code_raw);
    write_u32(&mut img, s2 + 36, 0xC0000040); // INIT_DATA | READ | WRITE

    // SizeOfCode / SizeOfInitializedData in optional header
    write_u32(&mut img, opt + 4, code_raw);
    write_u32(&mut img, opt + 8, data_raw);
    write_u32(&mut img, opt + 20, text_rva); // BaseOfCode

    // Build startup at start of .text:
    //   lea r15, [rip + disp]  ; r15 = data base (state)
    //   jmp user_code
    let text_file_off = headers_raw as usize;
    let startup_len = PE_STARTUP_LEN; // lea r15, [rip+d] (7) + jmp rel32 (5) + align nop

    // lea r15, [rip + disp32]
    // After this 7-byte insn, RIP = text_rva + 7
    // Want r15 = imagebase+data_rva  →  disp = data_rva - (text_rva + 7)
    let lea_disp = data_rva as i32 - (text_rva as i32 + 7);
    img[text_file_off] = 0x4C; // REX.WR
    img[text_file_off + 1] = 0x8D;
    img[text_file_off + 2] = 0x3D; // ModRM: r15, [rip+disp]
    img[text_file_off + 3..text_file_off + 7].copy_from_slice(&lea_disp.to_le_bytes());

    // jmp rel32 to user code (right after startup)
    let jmp_from = text_rva + 7;
    let user_code_rva = text_rva + startup_len;
    let jmp_rel = user_code_rva as i32 - (jmp_from as i32 + 5);
    img[text_file_off + 7] = 0xE9;
    img[text_file_off + 8..text_file_off + 12].copy_from_slice(&jmp_rel.to_le_bytes());
    img[text_file_off + 12] = 0x90; // nop pad → startup_len=13

    // Copy user code
    let code_dst = text_file_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // Copy data
    let data_file_off = (headers_raw + code_raw) as usize;
    let copy_n = data.len().min(data_raw as usize);
    img[data_file_off..data_file_off + copy_n].copy_from_slice(&data[..copy_n]);

    // Entry point = text_rva (startup)
    write_u32(&mut img, opt + 16, text_rva);

    if is_selfhost && import_dir_rva != 0 {
        // Data directory[1] = Import Table (offset 120 from optional header start).
        write_u32(&mut img, opt + 120, import_dir_rva);
        write_u32(&mut img, opt + 124, import_dir_size);
    }

    Ok(PeImage { bytes: img })
}

fn align_up(v: u32, a: u32) -> u32 {
    (v + a - 1) & !(a - 1)
}

fn write_u16(buf: &mut [u8], off: usize, v: u16) {
    buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
}

fn write_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn write_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

fn write_name(buf: &mut [u8], off: usize, name: &[u8]) {
    let n = name.len().min(8);
    buf[off..off + n].copy_from_slice(&name[..n]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pe_has_mz_and_pe() {
        let pe = link_pe(&[0xC3], &[]).unwrap();
        assert_eq!(&pe.bytes[0..2], b"MZ");
        let lfanew = u32::from_le_bytes(pe.bytes[0x3C..0x40].try_into().unwrap()) as usize;
        assert_eq!(&pe.bytes[lfanew..lfanew + 4], b"PE\0\0");
    }

    #[test]
    fn data_floor_0x38000() {
        let pe = link_pe(&[0xC3], &[]).unwrap();
        // file should be large enough to hold data section raw size
        assert!(pe.bytes.len() > 0x38000);
    }

    #[test]
    fn h00_scratch_off_pinned() {
        const DESC_SIZE: usize = 20; // IMAGE_IMPORT_DESCRIPTOR (was 40 — broke loader chain)
    let desc_size = DESC_SIZE;
        let kern_n = KERNEL32_IO_FUNCS.len();
        let preload_n = PRELOAD_RUNTIME_DLL_IMPORTS.len();
        let num_desc = 1 + preload_n + 1;
        let desc_off = kern_n * 8 + preload_n * 8;
        let strings_off = desc_off + desc_size * num_desc;
        let mut cursor = strings_off + b"kernel32.dll\0".len();
        for f in KERNEL32_IO_FUNCS {
            let mut hn = Vec::new();
            hn.extend_from_slice(&0u16.to_le_bytes());
            hn.extend_from_slice(f.as_bytes());
            hn.push(0);
            while hn.len() % 2 != 0 {
                hn.push(0);
            }
            cursor += hn.len();
        }
        cursor += (kern_n + 1) * 8;
        for (dll, f) in PRELOAD_RUNTIME_DLL_IMPORTS {
            cursor += dll.len() + 1;
            let mut hn = Vec::new();
            hn.extend_from_slice(&0u16.to_le_bytes());
            hn.extend_from_slice(f.as_bytes());
            hn.push(0);
            while hn.len() % 2 != 0 {
                hn.push(0);
            }
            cursor += hn.len();
            cursor += 16;
        }
        let scratch_off = cursor;
        assert_eq!(
            scratch_off,
            WIN32_IO_H00_SCRATCH_OFF as usize,
            "update WIN32_IO_H00_SCRATCH_OFF in pe_link + h00_manual_map_wireup"
        );
        assert!(
            scratch_off > desc_off,
            "scratch must be past import descriptors (desc_off=0x{desc_off:x})"
        );
    }

    /// H_00 seed PE: startup `lea r15` and stub `FF 15` IAT calls must agree on `.data` base.
    #[test]
    fn h00_seed_pe_rva_consistency() {
        use crate::ddc::PE_STARTUP_LEN;

        let mut code = vec![0u8; 32];
        code[0] = 0xC3;
        let handler_offsets = [(0x20u16, 8, 4), (0x21, 16, 4)];
        let pe = link_pe_win32(&code, &[1, 2, 3], &handler_offsets).expect("link h00 seed");
        let img = &pe.bytes;

        let lfanew = u32::from_le_bytes(img[0x3C..0x40].try_into().unwrap()) as usize;
        let opt = lfanew + 24;
        let entry_rva = u32::from_le_bytes(img[opt + 16..opt + 20].try_into().unwrap());
        let soh = u16::from_le_bytes(img[lfanew + 20..lfanew + 22].try_into().unwrap()) as usize;
        let sec = lfanew + 24 + soh;
        let mut text_rva = 0u32;
        let mut text_raw = 0usize;
        let mut data_rva = 0u32;
        let mut data_raw = 0usize;
        for i in 0..2 {
            let s = sec + i * 40;
            let name = &img[s..s + 8];
            let vrva = u32::from_le_bytes(img[s + 12..s + 16].try_into().unwrap());
            let rawptr = u32::from_le_bytes(img[s + 20..s + 24].try_into().unwrap()) as usize;
            if name.starts_with(b".text") {
                text_rva = vrva;
                text_raw = rawptr;
            } else if name.starts_with(b".data") {
                data_rva = vrva;
                data_raw = rawptr;
            }
        }
        assert_eq!(entry_rva, text_rva, "entry must be startup in .text");

        let lea_disp = i32::from_le_bytes(img[text_raw + 3..text_raw + 7].try_into().unwrap());
        let r15_from_startup = text_rva + 7 + lea_disp as u32;
        assert_eq!(
            r15_from_startup, data_rva,
            "startup lea r15 must target .data base (got 0x{r15_from_startup:x} data=0x{data_rva:x})"
        );

        let h00_jmp_rel = i32::from_le_bytes(
            img[text_raw + PE_STARTUP_LEN + 1..text_raw + PE_STARTUP_LEN + 5]
                .try_into()
                .unwrap(),
        );
        let stub_off = PE_STARTUP_LEN + 5 + h00_jmp_rel as usize;
        let stub = &img[text_raw + stub_off..];

        // Prelude kernel32 I/O uses call [r15+slot*8] after reload r15 (not FF15 RIP-relative).
        let reload_r15 = stub
            .windows(3)
            .position(|w| w == [0x4C, 0x8D, 0x3D])
            .expect("prelude reload r15 lea in stub");
        let reload_disp =
            i32::from_le_bytes(stub[reload_r15 + 3..reload_r15 + 7].try_into().unwrap());
        let reload_next = text_rva + stub_off as u32 + reload_r15 as u32 + 7;
        assert_eq!(
            reload_next as i32 + reload_disp,
            data_rva as i32,
            "prelude reload r15 must target .data base"
        );

        let create_r15 = stub
            .windows(7)
            .position(|w| w == [0x41, 0xFF, 0x97, 0x08, 0x00, 0x00, 0x00])
            .expect("CreateFile call [r15+8] in stub");
        assert!(
            create_r15 > reload_r15,
            "CreateFile [r15+8] must follow prelude reload r15"
        );

        // Manual-map body still uses FF15 for VirtualAlloc(image) and ExitProcess epilogues.
        let va_ff15 = stub
            .windows(2)
            .position(|w| w == [0xFF, 0x15])
            .expect("VirtualAlloc(image) FF 15 in stub");
        let disp = i32::from_le_bytes(stub[va_ff15 + 2..va_ff15 + 6].try_into().unwrap());
        let call_rva = text_rva + stub_off as u32 + va_ff15 as u32 + 6;
        let iat_rva = (call_rva as i32 + disp) as u32;
        assert_eq!(
            iat_rva,
            data_rva,
            "first FF15 IAT slot must be VirtualAlloc at data_rva+0 (got 0x{iat_rva:x})"
        );

        let lea_rcx = stub
            .windows(3)
            .position(|w| w == [0x48, 0x8D, 0x0D])
            .expect("lea rcx,[rip+disp] for yoyo_rt.dll");
        let path_disp =
            i32::from_le_bytes(stub[lea_rcx + 3..lea_rcx + 7].try_into().unwrap());
        let path_rva = text_rva + stub_off as u32 + lea_rcx as u32 + 7 + path_disp as u32;
        let path_raw = data_raw + (path_rva - data_rva) as usize;
        assert_eq!(
            &img[path_raw..path_raw + 12],
            b"yoyo_rt.dll\0",
            "sidecar path must point at yoyo_rt.dll string in .data"
        );
    }
}
