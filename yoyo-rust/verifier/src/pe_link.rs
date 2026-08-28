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
    // Stage 9-A H_00 runtime: DLL extract + LoadLibrary (slots 5–9, same IAT base as r15+0).
    "GetTempPathA",
    "lstrcatA",
    "LoadLibraryA",
    "GetProcAddress",
    "ExitProcess",
];

/// Prepend kernel32 IAT at r15+0 for Stage 8 platform I/O emit.
fn prepend_win32_io_iat(user_data: &[u8], data_rva: u32) -> (Vec<u8>, u32, u32) {
    let n = KERNEL32_IO_FUNCS.len();
    let desc_size = 40usize;
    let kernel32_name = b"kernel32.dll\0";
    let iat_slots_off = 0usize; // r15+0 .. r15+40

    let mut hint_names: Vec<Vec<u8>> = Vec::new();
    for name in KERNEL32_IO_FUNCS {
        let mut hn = Vec::new();
        hn.extend_from_slice(&0u16.to_le_bytes());
        hn.extend_from_slice(name.as_bytes());
        hn.push(0);
        while hn.len() % 2 != 0 {
            hn.push(0);
        }
        hint_names.push(hn);
    }

    let desc_off = (n + 1) * 8;
    let kern_off = desc_off + desc_size;
    let hn_start = kern_off + kernel32_name.len();
    let mut hn_off = hn_start;
    let mut hn_rvas: Vec<u32> = Vec::new();
    for hn in &hint_names {
        hn_rvas.push(data_rva + hn_off as u32);
        hn_off += hn.len();
    }

    let ilt_off = hn_off;
    let header_end = ilt_off + (n + 1) * 8;
    let pad = align_up_usize(header_end, 16);
    let mut blob = vec![0u8; pad + user_data.len()];
    let user_base = pad;

    write_u32(&mut blob, desc_off, data_rva + ilt_off as u32);
    write_u32(&mut blob, desc_off + 12, data_rva + kern_off as u32);
    write_u32(&mut blob, desc_off + 16, data_rva + iat_slots_off as u32);

    blob[kern_off..kern_off + kernel32_name.len()].copy_from_slice(kernel32_name);

    let mut off = hn_start;
    for hn in &hint_names {
        blob[off..off + hn.len()].copy_from_slice(hn);
        off += hn.len();
    }

    for (i, &hn_rva) in hn_rvas.iter().enumerate() {
        write_u64(&mut blob, ilt_off + i * 8, hn_rva as u64);
        write_u64(&mut blob, iat_slots_off + i * 8, hn_rva as u64);
    }

    blob[user_base..user_base + user_data.len()].copy_from_slice(user_data);
    (
        blob,
        data_rva + desc_off as u32,
        desc_size as u32,
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
/// call platform I/O handlers then embedded runtime compile (PE entry stays lea r15 → H_00).
pub fn link_pe_win32(
    code: &[u8],
    data: &[u8],
    handler_offsets: &[(u16, u32, u32)],
) -> IsaResult<PeImage> {
    if should_h00_selfhost(handler_offsets) {
        let dll = win32_selfhost::runtime_dll_bytes()?;
        link_pe_h00_runtime(code, data, handler_offsets, &dll)
    } else {
        let section_align: u32 = 0x1000;
        let text_rva = section_align;
        let est_text = code.len() as u32 + 0x40;
        let text_vs = align_up(est_text, section_align);
        let data_rva = text_rva + text_vs;
        let (extended, import_dir_rva, import_dir_size) = prepend_win32_io_iat(data, data_rva);
        let _ = platform_io::WIN32_IAT_DATA_RESERVE;
        link_pe_impl(code, &extended, true, import_dir_rva, import_dir_size)
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

/// Stage 9-A: gen1 H_00 pure runtime — patch entry handler, embed strings + runtime DLL.
/// Entry stays lea r15 → H_00 (not genNrt startup wrapper). H_00 calls extract+runtime+ExitProcess.
pub fn link_pe_h00_runtime(
    code: &[u8],
    data: &[u8],
    handler_offsets: &[(u16, u32, u32)],
    dll_bytes: &[u8],
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
            dll_embed_size: dll_bytes.len() as u32,
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
    // Prepend IAT before runtime embed so string/DLL RVAs include the IAT header pad.
    let (io_prepended, import_dir_rva, import_dir_size) =
        prepend_win32_io_iat(&with_strings, data_rva);
    let (extended, meta) =
        win32_selfhost::append_h00_runtime_data(&io_prepended, data_rva, dll_bytes)?;

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
    link_pe_impl(&code, &extended, true, import_dir_rva, import_dir_size)
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
    )
}

fn link_pe_impl(
    code: &[u8],
    data: &[u8],
    is_selfhost: bool,
    import_dir_rva: u32,
    import_dir_size: u32,
) -> IsaResult<PeImage> {
    let section_align: u32 = 0x1000;
    let file_align: u32 = 0x200;

    let code_raw = align_up(code.len() as u32, file_align);
    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, section_align));
    let data_raw = align_up(data_need, file_align);

    // Layout:
    // 0x0000: DOS + PE headers (1 section-align = 0x1000 file, 0x200 min)
    // text VA 0x1000, data VA 0x1000 + align(code)
    let headers_raw = 0x400u32;
    let text_rva = section_align; // 0x1000
    let text_vs = align_up(code.len() as u32 + 0x40, section_align); // room for startup
    let data_rva = text_rva + text_vs;
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
    let startup_len = 13u32; // lea r15, [rip+d] (7) + jmp rel32 (5) + align nop

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
}
