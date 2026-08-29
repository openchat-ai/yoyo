//! In-process PE32+ DLL manual mapper (OW-IAT spike).
//!
//! Replaces host `LoadLibraryA` for cwd sidecar `yoyo_rt.dll` by:
//! 1. Reading the file (host `CreateFileA` / `ReadFile` / `VirtualAlloc` — still CUT)
//! 2. Mapping sections into a private image buffer
//! 3. Applying `IMAGE_REL_BASED_DIR64` relocations
//! 4. Resolving imports via caller-provided export walk (no `GetProcAddress` IAT)
//! 5. Resolving exports the same way H_00 stub does today (`AddressOfFunctions[0]`)

const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D;
const IMAGE_NT_SIGNATURE: u32 = 0x0000_4550;
const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;
const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u16 = 0x20B;
const IMAGE_DIRECTORY_ENTRY_EXPORT: usize = 0;
const IMAGE_DIRECTORY_ENTRY_IMPORT: usize = 1;
const IMAGE_DIRECTORY_ENTRY_BASERELOC: usize = 5;
const IMAGE_REL_BASED_DIR64: u16 = 10;

#[derive(Debug, Clone, Copy)]
pub struct PeHeaders {
    pub image_base: u64,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub entry_rva: u32,
    pub export_dir_rva: u32,
    pub export_dir_size: u32,
    pub import_dir_rva: u32,
    pub import_dir_size: u32,
    pub reloc_dir_rva: u32,
    pub reloc_dir_size: u32,
}

#[derive(Debug)]
pub struct MappedPe {
    pub image: Vec<u8>,
    pub headers: PeHeaders,
    pub load_base: u64,
}

#[derive(Debug)]
pub enum MapError {
    Msg(String),
}

impl std::fmt::Display for MapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapError::Msg(s) => write!(f, "{s}"),
        }
    }
}

impl std::error::Error for MapError {}

fn err(msg: impl Into<String>) -> MapError {
    MapError::Msg(msg.into())
}

fn read_u16(pe: &[u8], off: usize) -> Result<u16, MapError> {
    pe.get(off..off + 2)
        .ok_or_else(|| err("truncated u16"))?
        .try_into()
        .map(u16::from_le_bytes)
        .map_err(|_| err("bad u16"))
}

fn read_u32(pe: &[u8], off: usize) -> Result<u32, MapError> {
    pe.get(off..off + 4)
        .ok_or_else(|| err("truncated u32"))?
        .try_into()
        .map(u32::from_le_bytes)
        .map_err(|_| err("bad u32"))
}

fn read_u64(pe: &[u8], off: usize) -> Result<u64, MapError> {
    pe.get(off..off + 8)
        .ok_or_else(|| err("truncated u64"))?
        .try_into()
        .map(u64::from_le_bytes)
        .map_err(|_| err("bad u64"))
}

fn pe_optional_offset(file: &[u8]) -> Result<usize, MapError> {
    if file.len() < 0x40 || read_u16(file, 0)? != IMAGE_DOS_SIGNATURE {
        return Err(err("not MZ"));
    }
    let e_lfanew = read_u32(file, 0x3C)? as usize;
    if file.get(e_lfanew..e_lfanew + 4) != Some(b"PE\x00\x00") {
        return Err(err("PE signature missing"));
    }
    Ok(e_lfanew + 4)
}

/// Parse PE32+ optional header fields needed for manual map.
pub fn parse_pe64_headers(file: &[u8]) -> Result<PeHeaders, MapError> {
    let pe = pe_optional_offset(file)?;
    if read_u16(file, pe + 0)? != IMAGE_FILE_MACHINE_AMD64 {
        return Err(err("not AMD64 PE"));
    }
    let opt = pe + 20;
    let _opt_hdr_size = read_u16(file, pe + 16)? as usize;
    if read_u16(file, opt)? != IMAGE_NT_OPTIONAL_HDR64_MAGIC {
        return Err(err("not PE32+ optional header"));
    }
    let export_dir_rva = read_u32(file, opt + 112 + IMAGE_DIRECTORY_ENTRY_EXPORT * 8)?;
    let export_dir_size = read_u32(file, opt + 116 + IMAGE_DIRECTORY_ENTRY_EXPORT * 8)?;
    let import_dir_rva = read_u32(file, opt + 112 + IMAGE_DIRECTORY_ENTRY_IMPORT * 8)?;
    let import_dir_size = read_u32(file, opt + 116 + IMAGE_DIRECTORY_ENTRY_IMPORT * 8)?;
    let reloc_dir_rva = read_u32(file, opt + 112 + IMAGE_DIRECTORY_ENTRY_BASERELOC * 8)?;
    let reloc_dir_size = read_u32(file, opt + 116 + IMAGE_DIRECTORY_ENTRY_BASERELOC * 8)?;
    Ok(PeHeaders {
        image_base: read_u64(file, opt + 24)?,
        size_of_image: read_u32(file, opt + 56)?,
        size_of_headers: read_u32(file, opt + 60)?,
        entry_rva: read_u32(file, opt + 16)?,
        export_dir_rva,
        export_dir_size,
        import_dir_rva,
        import_dir_size,
        reloc_dir_rva,
        reloc_dir_size,
    })
}

fn num_sections(file: &[u8]) -> Result<usize, MapError> {
    let pe = pe_optional_offset(file)?;
    Ok(read_u16(file, pe + 2)? as usize)
}

fn section_header_offset(file: &[u8], index: usize) -> Result<usize, MapError> {
    let pe = pe_optional_offset(file)?;
    let opt_hdr_size = read_u16(file, pe + 16)? as usize;
    Ok(pe + 20 + opt_hdr_size + index * 40)
}

/// Copy PE headers + sections into a zeroed `size_of_image` buffer (RVA-indexed).
pub fn map_pe_sections(file: &[u8], headers: &PeHeaders) -> Result<Vec<u8>, MapError> {
    let size = headers.size_of_image as usize;
    if size == 0 {
        return Err(err("size_of_image is zero"));
    }
    let mut image = vec![0u8; size];
    let hdr_copy = headers
        .size_of_headers
        .min(file.len() as u32)
        .min(headers.size_of_image) as usize;
    image[..hdr_copy].copy_from_slice(&file[..hdr_copy]);

    let n = num_sections(file)?;
    for i in 0..n {
        let sh = section_header_offset(file, i)?;
        let virtual_size = read_u32(file, sh + 8)? as usize;
        let virtual_addr = read_u32(file, sh + 12)? as usize;
        let raw_size = read_u32(file, sh + 16)? as usize;
        let raw_addr = read_u32(file, sh + 20)? as usize;
        if virtual_addr >= size {
            continue;
        }
        let copy_n = raw_size.min(virtual_size).min(size - virtual_addr);
        if raw_addr + copy_n > file.len() {
            return Err(err(format!(
                "section {i} raw range {raw_addr}+{copy_n} exceeds file"
            )));
        }
        if copy_n > 0 {
            image[virtual_addr..virtual_addr + copy_n]
                .copy_from_slice(&file[raw_addr..raw_addr + copy_n]);
        }
    }
    Ok(image)
}

/// Apply PE32+ base relocations for `load_base` (delta from `headers.image_base`).
pub fn apply_base_relocations(
    image: &mut [u8],
    headers: &PeHeaders,
    load_base: u64,
) -> Result<(), MapError> {
    let delta = load_base.wrapping_sub(headers.image_base);
    if delta == 0 || headers.reloc_dir_rva == 0 {
        return Ok(());
    }
    let mut off = headers.reloc_dir_rva as usize;
    let end = off.saturating_add(headers.reloc_dir_size as usize);
    if end > image.len() {
        return Err(err("reloc directory out of image"));
    }
    while off + 8 <= end {
        let page_rva = read_u32(image, off)?;
        let block_size = read_u32(image, off + 4)? as usize;
        if block_size < 8 {
            break;
        }
        if off + block_size > end {
            return Err(err("reloc block exceeds directory"));
        }
        let count = (block_size - 8) / 2;
        for i in 0..count {
            let entry = read_u16(image, off + 8 + i * 2)?;
            let typ = entry >> 12;
            let rva_off = (entry & 0x0FFF) as usize;
            if typ == 0 {
                continue;
            }
            if typ != IMAGE_REL_BASED_DIR64 {
                return Err(err(format!("unsupported reloc type {typ}")));
            }
            let target = page_rva as usize + rva_off;
            if target + 8 > image.len() {
                return Err(err("reloc target out of image"));
            }
            let val = read_u64(image, target)?;
            write_u64(image, target, val.wrapping_add(delta));
        }
        off += block_size;
    }
    Ok(())
}

fn rva_slice<'a>(image: &'a [u8], rva: u32) -> Result<&'a [u8], MapError> {
    let off = rva as usize;
    image
        .get(off..)
        .ok_or_else(|| err(format!("rva {rva:#x} out of range")))
}

fn cstr_at(image: &[u8], rva: u32) -> Result<&str, MapError> {
    let bytes = rva_slice(image, rva)?;
    let end = bytes
        .iter()
        .position(|&b| b == 0)
        .ok_or_else(|| err("unterminated import name"))?;
    std::str::from_utf8(&bytes[..end]).map_err(|_| err("import name not utf8"))
}

/// List imported DLL names from a PE file (for sidecar diagnostics).
pub fn pe_import_dll_names(file: &[u8]) -> Result<Vec<String>, MapError> {
    let headers = parse_pe64_headers(file)?;
    if headers.import_dir_rva == 0 {
        return Ok(Vec::new());
    }
    let mut out = Vec::new();
    let mut desc_rva = headers.import_dir_rva;
    loop {
        let desc = rva_slice(file, desc_rva)?;
        if desc.len() < 20 {
            break;
        }
        let name_rva = read_u32(desc, 12)?;
        let first_thunk = read_u32(desc, 16)?;
        let orig_first_thunk = read_u32(desc, 0)?;
        if name_rva == 0 && first_thunk == 0 && orig_first_thunk == 0 {
            break;
        }
        if name_rva != 0 {
            out.push(cstr_at(file, name_rva)?.to_string());
        }
        desc_rva += 20;
    }
    Ok(out)
}

/// Resolve PE import thunks using `resolve(dll, name) -> host VA`.
pub fn resolve_imports<F>(image: &mut [u8], headers: &PeHeaders, mut resolve: F) -> Result<(), MapError>
where
    F: FnMut(&str, &str) -> Option<u64>,
{
    if headers.import_dir_rva == 0 {
        return Ok(());
    }
    let mut desc_rva = headers.import_dir_rva;
    loop {
        let desc = rva_slice(image, desc_rva)?;
        if desc.len() < 20 {
            break;
        }
        let orig_first_thunk = read_u32(desc, 0)?;
        let name_rva = read_u32(desc, 12)?;
        let first_thunk = read_u32(desc, 16)?;
        if orig_first_thunk == 0 && name_rva == 0 && first_thunk == 0 {
            break;
        }
        let dll = cstr_at(image, name_rva)?.to_string();
        let mut thunk_rva = if orig_first_thunk != 0 {
            orig_first_thunk
        } else {
            first_thunk
        };
        let mut iat_rva = first_thunk;
        loop {
            let thunk_off = thunk_rva as usize;
            let iat_off = iat_rva as usize;
            if thunk_off + 8 > image.len() || iat_off + 8 > image.len() {
                return Err(err("import thunk out of image"));
            }
            let thunk = read_u64(image, thunk_off)?;
            if thunk == 0 {
                break;
            }
            let addr = if thunk & (1u64 << 63) != 0 {
                // Ordinal import — spike does not need these for yoyo_rt.dll tests.
                resolve(&dll, &format!("#{}", thunk & 0xFFFF))
            } else {
                let hint_name_rva = (thunk & 0x7FFF_FFFF) as u32;
                let name = if hint_name_rva == 0 {
                    return Err(err("import by name missing hint/name rva"));
                } else {
                    // Skip 2-byte hint.
                    cstr_at(image, hint_name_rva + 2)?
                };
                resolve(&dll, name)
            };
            let Some(va) = addr else {
                return Err(err(format!("unresolved import {dll}! (thunk={thunk:#x})")));
            };
            write_u64(image, iat_off, va);
            thunk_rva += 8;
            iat_rva += 8;
        }
        desc_rva += 20;
    }
    Ok(())
}

/// Resolve `AddressOfFunctions[0]` export RVA (matches H_00 stub contract).
pub fn export_function_rva_functions0(image: &[u8], headers: &PeHeaders) -> Result<u32, MapError> {
    if headers.export_dir_rva == 0 {
        return Err(err("no export directory"));
    }
    let exp = rva_slice(image, headers.export_dir_rva)?;
    if exp.len() < 0x28 {
        return Err(err("export dir too small"));
    }
    let num_names = read_u32(exp, 0x18)?;
    if num_names == 0 {
        return Err(err("export NumberOfNames is zero"));
    }
    let functions_rva = read_u32(exp, 0x1C)?;
    let functions = rva_slice(image, functions_rva)?;
    read_u32(functions, 0)
}

/// Full manual map: sections + reloc + imports.
pub fn manual_map_pe_dll<F>(
    file: &[u8],
    load_base: u64,
    resolve_import: F,
) -> Result<MappedPe, MapError>
where
    F: FnMut(&str, &str) -> Option<u64>,
{
    let headers = parse_pe64_headers(file)?;
    let mut image = map_pe_sections(file, &headers)?;
    apply_base_relocations(&mut image, &headers, load_base)?;
    resolve_imports(&mut image, &headers, resolve_import)?;
    Ok(MappedPe {
        image,
        headers,
        load_base,
    })
}

fn write_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_u32(buf: &mut [u8], off: usize, v: u32) {
        buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
    }

    fn write_u64(buf: &mut [u8], off: usize, v: u64) {
        buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
    }

    /// Minimal mapped image with export dir at 0x1000 and one function at RVA 0x1050.
    fn image_with_export(functions0_rva: u32) -> (Vec<u8>, PeHeaders) {
        let mut image = vec![0u8; 0x2000];
        let exp = 0x1000usize;
        write_u32(&mut image, exp + 0x18, 1);
        write_u32(&mut image, exp + 0x1C, 0x1080);
        write_u32(&mut image, 0x1080, functions0_rva);
        let headers = PeHeaders {
            image_base: 0x1800_0000,
            size_of_image: 0x2000,
            size_of_headers: 0x200,
            entry_rva: functions0_rva,
            export_dir_rva: 0x1000,
            export_dir_size: 0x40,
            import_dir_rva: 0,
            import_dir_size: 0,
            reloc_dir_rva: 0,
            reloc_dir_size: 0,
        };
        (image, headers)
    }

    #[test]
    fn export_functions0_matches_h00_stub() {
        let (image, headers) = image_with_export(0x1050);
        let rva = export_function_rva_functions0(&image, &headers).expect("export");
        assert_eq!(rva, 0x1050);
    }

    #[test]
    fn base_relocation_applies_dir64_delta() {
        let mut image = vec![0u8; 0x2000];
        write_u64(&mut image, 0x1008, 0x1800_0042);
        write_u32(&mut image, 0x1200, 0x1000);
        write_u32(&mut image, 0x1204, 0x10);
        write_u16(&mut image, 0x1208, (IMAGE_REL_BASED_DIR64 << 12) | 8);
        let headers = PeHeaders {
            image_base: 0x1800_0000,
            size_of_image: 0x2000,
            size_of_headers: 0x200,
            entry_rva: 0,
            export_dir_rva: 0,
            export_dir_size: 0,
            import_dir_rva: 0,
            import_dir_size: 0,
            reloc_dir_rva: 0x1200,
            reloc_dir_size: 0x18,
        };
        let load_base = 0x2000_0000u64;
        apply_base_relocations(&mut image, &headers, load_base).expect("reloc");
        let ptr = read_u64(&image, 0x1008).expect("ptr");
        assert_eq!(ptr, 0x2000_0042);
    }

    #[test]
    fn import_resolve_fills_iat_slot() {
        let mut image = vec![0u8; 0x2000];
        write_u32(&mut image, 0x1100, 0x1140); // OriginalFirstThunk
        write_u32(&mut image, 0x110C, 0x1180); // Name
        write_u32(&mut image, 0x1110, 0x11A0); // FirstThunk
        image[0x1180..0x1180 + 13].copy_from_slice(b"KERNEL32.dll\0");
        write_u64(&mut image, 0x1140, 0x11C0);
        write_u64(&mut image, 0x1148, 0);
        image[0x11C2..0x11C2 + 12].copy_from_slice(b"ExitProcess\0");
        write_u64(&mut image, 0x11A0, 0);
        let headers = PeHeaders {
            image_base: 0x1800_0000,
            size_of_image: 0x2000,
            size_of_headers: 0x200,
            entry_rva: 0,
            export_dir_rva: 0,
            export_dir_size: 0,
            import_dir_rva: 0x1100,
            import_dir_size: 0x20,
            reloc_dir_rva: 0,
            reloc_dir_size: 0,
        };
        resolve_imports(&mut image, &headers, |dll, name| {
            if dll.eq_ignore_ascii_case("KERNEL32.dll") && name == "ExitProcess" {
                Some(0x7FF0_0000_1234)
            } else {
                None
            }
        })
        .expect("imports");
        let iat = read_u64(&image, 0x11A0).expect("iat");
        assert_eq!(iat, 0x7FF0_0000_1234);
    }

    /// When yoyo_runtime.dll is built, prove manual_map + functions[0] on real sidecar bytes.
    /// Log sidecar import DLL names when built (Windows CI diagnostic for manual-map smoke).
    #[test]
    fn log_sidecar_import_dlls_if_present() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let paths = [
            root.join("target/release-runtime/yoyo_runtime.dll"),
            root.join("target/release/yoyo_runtime.dll"),
        ];
        let Some(path) = paths.iter().find(|p| p.is_file()) else {
            return;
        };
        let file = std::fs::read(path).expect("read dll");
        let dlls = pe_import_dll_names(&file).expect("imports");
        eprintln!("SIDEcar_IMPORT_DLLS={dlls:?}");
    }

    #[test]
    fn manual_map_real_sidecar_if_present() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
        let paths = [
            root.join("target/release-runtime/yoyo_runtime.dll"),
            root.join("target/release/yoyo_runtime.dll"),
        ];
        let Some(path) = paths.iter().find(|p| p.is_file()) else {
            eprintln!("skip manual_map_real_sidecar_if_present: yoyo_runtime.dll not built");
            return;
        };
        let file = std::fs::read(path).expect("read dll");
        let load_base = 0x1_8000_0000u64;
        let mapped = manual_map_pe_dll(&file, load_base, |dll, name| {
            if dll.eq_ignore_ascii_case("KERNEL32.dll") && name == "ExitProcess" {
                Some(0x7FFE_0000)
            } else {
                // Spike test: any resolved VA suffices to walk export dir.
                Some(0x7FFE_1000)
            }
        })
        .expect("manual map sidecar");
        let rva = export_function_rva_functions0(&mapped.image, &mapped.headers).expect("export");
        assert!(rva > 0x1000, "export RVA should be in image");
    }

    /// Windows-only: manual-map sidecar with host GetProcAddress resolver (isolates stub vs runtime).
    #[test]
    #[cfg(windows)]
    fn manual_map_runtime_smoke_host_resolve() {
        use std::ffi::CString;
        use std::path::PathBuf;

        #[link(name = "kernel32")]
        extern "system" {
            fn LoadLibraryA(name: *const i8) -> *mut std::ffi::c_void;
            fn GetProcAddress(module: *mut std::ffi::c_void, name: *const i8) -> *mut std::ffi::c_void;
            fn SetCurrentDirectoryA(path: *const i8) -> i32;
        }

        fn host_resolve(dll: &str, name: &str) -> Option<u64> {
            let dll_c = CString::new(dll).ok()?;
            unsafe {
                let module = LoadLibraryA(dll_c.as_ptr());
                if module.is_null() {
                    return None;
                }
                if let Some(rest) = name.strip_prefix('#') {
                    let ord: u16 = rest.parse().ok()?;
                    let proc = GetProcAddress(module, ord as usize as *const i8);
                    if proc.is_null() {
                        None
                    } else {
                        Some(proc as u64)
                    }
                } else {
                    let name_c = CString::new(name).ok()?;
                    let proc = GetProcAddress(module, name_c.as_ptr());
                    if proc.is_null() {
                        None
                    } else {
                        Some(proc as u64)
                    }
                }
            }
        }

        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let dll_path = [
            root.join("yoyo-rust/target/release-runtime/yoyo_runtime.dll"),
            root.join("yoyo-rust/target/release/yoyo_runtime.dll"),
        ]
        .into_iter()
        .find(|p| p.is_file())
        .expect("yoyo_runtime.dll not built");
        assert!(
            !String::from_utf8_lossy(&std::fs::read(&dll_path).expect("read dll"))
                .contains("VCRUNTIME140"),
            "sidecar must be crt-static (no VCRUNTIME140 import) for manual-map PEB walk"
        );

        let work = std::env::temp_dir().join(format!("yoyo-manual-map-smoke-{}", std::process::id()));
        std::fs::create_dir_all(&work).expect("mkdir work");
        let tyb = root.join("yoyo/projects/yoyo.tyb");
        std::fs::copy(&tyb, work.join("input.tyb")).expect("copy input.tyb");
        let work_c = CString::new(work.to_string_lossy().as_bytes()).expect("work path");
        unsafe {
            assert_ne!(SetCurrentDirectoryA(work_c.as_ptr()), 0, "SetCurrentDirectoryA");
        }

        let file = std::fs::read(&dll_path).expect("read sidecar");
        let load_base = 0x1_8000_0000u64;
        let mapped = manual_map_pe_dll(&file, load_base, host_resolve).expect("manual map");
        let hinst = load_base as *mut std::ffi::c_void;

        if mapped.headers.entry_rva != 0 {
            type DllEntry =
                unsafe extern "system" fn(*mut std::ffi::c_void, u32, *mut std::ffi::c_void) -> i32;
            let entry: DllEntry = unsafe {
                std::mem::transmute(load_base + mapped.headers.entry_rva as u64)
            };
            unsafe {
                assert_ne!(entry(hinst, 1, std::ptr::null_mut()), 0, "DllMain attach");
            }
        }

        let export_rva =
            export_function_rva_functions0(&mapped.image, &mapped.headers).expect("export rva");
        type ExportFn = unsafe extern "system" fn() -> i32;
        let export_fn: ExportFn =
            unsafe { std::mem::transmute(load_base + export_rva as u64) };
        let code = unsafe { export_fn() };
        assert_eq!(code, 0, "yoyo_runtime_selfhost_main exit code");
        assert!(
            work.join("output.exe").is_file(),
            "output.exe missing after manual-map host-resolve smoke"
        );
    }

    fn write_u16(buf: &mut [u8], off: usize, v: u16) {
        buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
    }
}
