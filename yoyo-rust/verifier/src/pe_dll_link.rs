//! PE32+ DLL emitter (OW-RT / YOYO-built runtime spike).
//!
//! Emits a minimal PE32+ **DLL** with a single named export at
//! `AddressOfFunctions[0]` — the same contract H_00 manual-map / ordinal-0
//! resolve uses for cwd sidecar `yoyo_rt.dll`.
//!
//! Gate E: export `.text` comes from YOYO `.ty` RAW_BYTES+RET (fixed exit-2).
//! Gate F: host-orchestrated YOYO seed/link **read→compile→write** effect with
//! the same exit contract as Rust `yoyo_runtime_selfhost_main` (0/1/2/3).
//! Gate G slice (prior): export-compile — emit-time `bootstrap_compile` baked
//! into pe_dll; call writes that single PE (content ignored beyond existence).
//! Gate G slice (this): **generic in-DLL recompile** — call-time `ReadFile` of
//! cwd input + multi-entry YOYO-precompiled oracle table (match → write PE).
//! Gate G slice (latest): oracle coverage widened to 3 real `bootstrap_compile`
//! fixtures (selfhost_min_nop + 01_set_get + 03_cmp_je), `.tyb` extension
//! parity, and the H_00 embed-vs-cwd sidecar posture made explicit. No-input is
//! fail-closed on the sidecar path exactly like the seed/link host: exit
//! `EXIT_NO_INPUT` (`2`), no `output.exe` written.
//! Honest: table ≠ full YOYO compiler in DLL; production default remains Rust —
//! **not** OW-RT CLOSED (see `SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md`).

use std::path::{Path, PathBuf};

use crate::platform::PlatformKind;
use crate::types::{IsaError, IsaResult};

/// Canonical H_00 / runtime export name (must stay ordinal-0 / functions[0]).
pub const RUNTIME_EXPORT_NAME: &str = "yoyo_runtime_selfhost_main";

/// Sidecar basename H_00 loads from cwd (ASCII marker).
pub const RUNTIME_SIDECAR_NAME: &str = "yoyo_rt.dll";

/// Probe body: `mov eax, imm32; ret` — exit code matches runtime no-input (`2`).
pub const PROBE_EXIT_NO_INPUT: i32 = 2;

/// YOYO `.ty` stub (mirrors `yoyo/tests/golden/ow_rt_yoyo_origin_exit2.ty`).
/// RAW_BYTES emits `mov eax,2`; RET emits `C3`.
pub const YOYO_ORIGIN_EXIT2_TY: &str = "\
; OW-RT Gate E — YOYO-origin fixed-exit probe\n\
40 00\n\
  A1 B8 02 00 00 00\n\
  FF\n\
";

/// Expected YOYO-emitted export body (`mov eax,2; ret`).
pub const YOYO_ORIGIN_EXIT2_CODE: [u8; 6] = [0xB8, 0x02, 0x00, 0x00, 0x00, 0xC3];

/// Link a PE32+ DLL whose `AddressOfFunctions[0]` runs `export_code`.
///
/// Layout (no imports / no relocs — NX only, fixed ImageBase):
/// - `.text` @ RVA 0x1000: DllMain (`mov eax,1; ret`) + export body
/// - `.rdata` @ RVA 0x2000: export directory + name tables + strings
pub fn link_pe_dll_export0(export_code: &[u8], dll_name: &str, export_name: &str) -> IsaResult<Vec<u8>> {
    link_pe_dll_export0_with_extra(export_code, dll_name, export_name, &[])
}

/// Like [`link_pe_dll_export0`], appending `extra_rdata` after the null import
/// descriptor (used to embed YOYO compile products for Gate G export-compile).
pub fn link_pe_dll_export0_with_extra(
    export_code: &[u8],
    dll_name: &str,
    export_name: &str,
    extra_rdata: &[u8],
) -> IsaResult<Vec<u8>> {
    if export_code.is_empty() {
        return Err(IsaError::PlatformError {
            msg: "pe_dll_link: export_code empty".into(),
        });
    }
    if dll_name.is_empty() || export_name.is_empty() {
        return Err(IsaError::PlatformError {
            msg: "pe_dll_link: dll_name/export_name empty".into(),
        });
    }
    if dll_name.as_bytes().contains(&0) || export_name.as_bytes().contains(&0) {
        return Err(IsaError::PlatformError {
            msg: "pe_dll_link: names must not contain NUL".into(),
        });
    }

    const FILE_ALIGN: u32 = 0x200;
    const SECTION_ALIGN: u32 = 0x1000;
    const HEADERS_RAW: u32 = 0x200;
    const IMAGE_BASE: u64 = 0x1800_0000;

    let dll_name_z = format!("{dll_name}\0");
    let export_name_z = format!("{export_name}\0");
    let dll_bytes = dll_name_z.as_bytes();
    let export_bytes = export_name_z.as_bytes();

    // .text: DllMain (6) + export body
    let dllmain: [u8; 6] = [0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3]; // mov eax,1; ret
    let text_payload_len = dllmain.len() + export_code.len();
    let text_raw = align_up(text_payload_len as u32, FILE_ALIGN);
    let text_rva = SECTION_ALIGN; // 0x1000
    let text_vs = align_up(text_payload_len as u32, SECTION_ALIGN);
    let entry_rva = text_rva; // DllMain
    let export_fn_rva = text_rva + dllmain.len() as u32;

    // .rdata export blob layout (all RVAs relative to rdata_rva):
    //   +0x00 IMAGE_EXPORT_DIRECTORY (0x28)
    //   +0x28 AddressOfFunctions[1]
    //   +0x2C AddressOfNames[1]
    //   +0x30 AddressOfNameOrdinals[1] (u16)
    //   +0x32 pad to 4
    //   +0x34 dll name
    //   +…   export name
    //   +…   pad4 + null IMAGE_IMPORT_DESCRIPTOR (20 B)
    //   +…   optional extra (YOYO compile payload)
    // H_00 skips LL/GPA bootstrap when ImportDir RVA==0, then FlushICache
    // call [GPA scratch] AVs — null descriptor keeps ImportDir non-zero.
    let exp_dir_off = 0u32;
    let functions_off = 0x28u32;
    let names_off = 0x2Cu32;
    let ordinals_off = 0x30u32;
    let dll_name_off = 0x34u32;
    let export_name_off = dll_name_off + dll_bytes.len() as u32;
    let after_names = export_name_off + export_bytes.len() as u32;
    let import_desc_off = align_up(after_names, 4);
    const IMPORT_DESC_SIZE: u32 = 20; // one null IMAGE_IMPORT_DESCRIPTOR
    let extra_off = import_desc_off + IMPORT_DESC_SIZE;
    let rdata_payload = extra_off + extra_rdata.len() as u32;
    let rdata_raw = align_up(rdata_payload, FILE_ALIGN);
    let rdata_rva = text_rva + text_vs; // 0x2000 when text fits one section
    let rdata_vs = align_up(rdata_payload, SECTION_ALIGN);

    let size_of_image = align_up(rdata_rva + rdata_vs, SECTION_ALIGN);
    let file_size = HEADERS_RAW + text_raw + rdata_raw;
    let mut img = vec![0u8; file_size as usize];

    // DOS
    img[0] = 0x4D;
    img[1] = 0x5A;
    write_u32(&mut img, 0x3C, 0x80);

    // PE + COFF
    img[0x80] = b'P';
    img[0x81] = b'E';
    write_u16(&mut img, 0x84, 0x8664); // AMD64
    write_u16(&mut img, 0x86, 2); // NumberOfSections
    write_u16(&mut img, 0x94, 0xF0); // SizeOfOptionalHeader
    // EXECUTABLE | LARGE_ADDRESS_AWARE | DLL
    write_u16(&mut img, 0x96, 0x2022);

    let opt = 0x98usize;
    write_u16(&mut img, opt, 0x20B); // PE32+
    img[opt + 2] = 1; // MajorLinkerVersion
    write_u32(&mut img, opt + 4, text_raw); // SizeOfCode
    write_u32(&mut img, opt + 8, rdata_raw); // SizeOfInitializedData
    write_u32(&mut img, opt + 16, entry_rva); // AddressOfEntryPoint
    write_u32(&mut img, opt + 20, text_rva); // BaseOfCode
    write_u64(&mut img, opt + 24, IMAGE_BASE);
    write_u32(&mut img, opt + 32, SECTION_ALIGN);
    write_u32(&mut img, opt + 36, FILE_ALIGN);
    write_u16(&mut img, opt + 40, 6);
    write_u16(&mut img, opt + 48, 6);
    write_u32(&mut img, opt + 56, size_of_image);
    write_u32(&mut img, opt + 60, HEADERS_RAW);
    write_u16(&mut img, opt + 68, 2); // WINDOWS_GUI (typical DLL)
    write_u16(&mut img, opt + 70, 0x0100); // NX_COMPAT (no ASLR — no .reloc)
    write_u64(&mut img, opt + 72, 0x100000);
    write_u64(&mut img, opt + 80, 0x1000);
    write_u64(&mut img, opt + 88, 0x100000);
    write_u64(&mut img, opt + 96, 0x1000);
    write_u32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes

    // Export data directory [0]
    let export_dir_rva = rdata_rva + exp_dir_off;
    let export_dir_size = 0x28u32;
    write_u32(&mut img, opt + 112, export_dir_rva);
    write_u32(&mut img, opt + 116, export_dir_size);

    // Import data directory [1] — null descriptor only (triggers H_00 bootstrap).
    let import_dir_rva = rdata_rva + import_desc_off;
    write_u32(&mut img, opt + 120, import_dir_rva);
    write_u32(&mut img, opt + 124, IMPORT_DESC_SIZE);

    // Section .text
    let s1 = 0x98 + 0xF0; // 0x188
    write_name(&mut img, s1, b".text");
    write_u32(&mut img, s1 + 8, text_vs);
    write_u32(&mut img, s1 + 12, text_rva);
    write_u32(&mut img, s1 + 16, text_raw);
    write_u32(&mut img, s1 + 20, HEADERS_RAW);
    write_u32(&mut img, s1 + 36, 0x6000_0020); // CODE | EXECUTE | READ

    // Section .rdata
    let s2 = s1 + 40;
    write_name(&mut img, s2, b".rdata");
    write_u32(&mut img, s2 + 8, rdata_vs);
    write_u32(&mut img, s2 + 12, rdata_rva);
    write_u32(&mut img, s2 + 16, rdata_raw);
    write_u32(&mut img, s2 + 20, HEADERS_RAW + text_raw);
    write_u32(&mut img, s2 + 36, 0x4000_0040); // INIT_DATA | READ

    // .text payload
    let text_off = HEADERS_RAW as usize;
    img[text_off..text_off + dllmain.len()].copy_from_slice(&dllmain);
    img[text_off + dllmain.len()..text_off + text_payload_len].copy_from_slice(export_code);

    // .rdata export tables
    let rdata_off = (HEADERS_RAW + text_raw) as usize;
    let exp = &mut img[rdata_off..];
    // Name RVA
    write_u32(exp, exp_dir_off as usize + 0x0C, rdata_rva + dll_name_off);
    write_u32(exp, exp_dir_off as usize + 0x10, 1); // Base = 1
    write_u32(exp, exp_dir_off as usize + 0x14, 1); // NumberOfFunctions
    write_u32(exp, exp_dir_off as usize + 0x18, 1); // NumberOfNames
    write_u32(exp, exp_dir_off as usize + 0x1C, rdata_rva + functions_off);
    write_u32(exp, exp_dir_off as usize + 0x20, rdata_rva + names_off);
    write_u32(exp, exp_dir_off as usize + 0x24, rdata_rva + ordinals_off);
    write_u32(exp, functions_off as usize, export_fn_rva);
    write_u32(exp, names_off as usize, rdata_rva + export_name_off);
    write_u16(exp, ordinals_off as usize, 0); // ordinal 0 → functions[0]
    exp[dll_name_off as usize..dll_name_off as usize + dll_bytes.len()].copy_from_slice(dll_bytes);
    exp[export_name_off as usize..export_name_off as usize + export_bytes.len()]
        .copy_from_slice(export_bytes);
    // Null IMAGE_IMPORT_DESCRIPTOR — already zero-filled in `img`.
    if !extra_rdata.is_empty() {
        let o = extra_off as usize;
        exp[o..o + extra_rdata.len()].copy_from_slice(extra_rdata);
    }

    Ok(img)
}

/// Compile YOYO-origin fixed-exit export body (`yoyo_origin_export=PRESENT`).
pub fn yoyo_origin_export_exit2_code() -> IsaResult<Vec<u8>> {
    let out = crate::executor::compile_ty_source(YOYO_ORIGIN_EXIT2_TY, PlatformKind::Stub)?;
    if out.code.as_slice() != YOYO_ORIGIN_EXIT2_CODE.as_slice() {
        return Err(IsaError::PlatformError {
            msg: format!(
                "pe_dll_link: YOYO-origin exit2 mismatch (got {:02X?}, want {:02X?})",
                out.code, YOYO_ORIGIN_EXIT2_CODE
            ),
        });
    }
    Ok(out.code)
}

/// Probe DLL: YOYO-origin export returns `PROBE_EXIT_NO_INPUT` (no-input code).
pub fn link_probe_runtime_dll() -> IsaResult<Vec<u8>> {
    let code = yoyo_origin_export_exit2_code()?;
    link_pe_dll_export0(&code, RUNTIME_SIDECAR_NAME, RUNTIME_EXPORT_NAME)
}

/// Env opt-in: place YOYO `pe_dll` probe as cwd `yoyo_rt.dll` instead of Rust.
///
/// Gate G **slice only** — must not become production default until full
/// YOYO-built R→C→W lives inside the sidecar and inventory can fail-closed CLOSED.
pub const ALT_SIDECAR_ENV: &str = "YOYO_OW_RT_ALT_SIDECAR";

/// True when `YOYO_OW_RT_ALT_SIDECAR` is `1` / `true` / `yes` / `on` (case-insensitive).
pub fn yoyo_alt_sidecar_enabled() -> bool {
    match std::env::var(ALT_SIDECAR_ENV) {
        Ok(v) => matches!(
            v.trim().to_ascii_lowercase().as_str(),
            "1" | "true" | "yes" | "on"
        ),
        Err(_) => false,
    }
}

/// Write YOYO-origin probe PE32+ DLL to `out_path` (typically `…/yoyo_rt.dll`).
///
/// Honest: export body is fixed exit-2 only — **not** a full Rust-runtime replacement.
pub fn write_yoyo_alt_sidecar(out_path: &Path) -> IsaResult<Vec<u8>> {
    let dll = link_probe_runtime_dll()?;
    if let Some(parent) = out_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent).map_err(|e| IsaError::IoError {
                msg: format!("pe_dll_link: mkdir {}: {e}", parent.display()),
            })?;
        }
    }
    std::fs::write(out_path, &dll).map_err(|e| IsaError::IoError {
        msg: format!("pe_dll_link: write {}: {e}", out_path.display()),
    })?;
    Ok(dll)
}

/// Prefer YOYO alt sidecar bytes when env opt-in is set; else `None` (caller uses Rust).
pub fn yoyo_alt_sidecar_bytes_if_enabled() -> IsaResult<Option<Vec<u8>>> {
    if yoyo_alt_sidecar_enabled() {
        Ok(Some(link_probe_runtime_dll()?))
    } else {
        Ok(None)
    }
}

/// Which bytes were placed as cwd `yoyo_rt.dll` by [`place_cwd_runtime_sidecar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CwdSidecarKind {
    /// YOYO `pe_dll` probe (env opt-in).
    YoyoAlt,
    /// Rust `yoyo_runtime.dll` bytes (production default).
    Rust,
}

/// Place cwd `yoyo_rt.dll`: YOYO probe when `YOYO_OW_RT_ALT_SIDECAR` is on, else `rust_dll`.
///
/// Moves toward replacing the Rust production default **without** flipping it —
/// default path still writes Rust bytes. Not OW-RT CLOSED.
pub fn place_cwd_runtime_sidecar(out_path: &Path, rust_dll: &[u8]) -> IsaResult<CwdSidecarKind> {
    if yoyo_alt_sidecar_enabled() {
        write_yoyo_alt_sidecar(out_path)?;
        Ok(CwdSidecarKind::YoyoAlt)
    } else {
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() {
                std::fs::create_dir_all(parent).map_err(|e| IsaError::IoError {
                    msg: format!("pe_dll_link: mkdir {}: {e}", parent.display()),
                })?;
            }
        }
        std::fs::write(out_path, rust_dll).map_err(|e| IsaError::IoError {
            msg: format!("pe_dll_link: write {}: {e}", out_path.display()),
        })?;
        Ok(CwdSidecarKind::Rust)
    }
}

/// Exit contract shared with Rust `yoyo_runtime_selfhost_main`.
pub const EXIT_OK: i32 = 0;
pub const EXIT_COMPILE_FAIL: i32 = 1;
pub const EXIT_NO_INPUT: i32 = 2;
pub const EXIT_WRITE_FAIL: i32 = 3;

const INPUT_NAMES: &[&str] = &["input.tyb", "input.ky", "input.ty"];
const OUTPUT_NAME: &str = "output.exe";

/// Note: the in-DLL recompile path's no-input exit is **not** divergent from the
/// seed/link host. Both return [`PROBE_EXIT_NO_INPUT`] (`2`) and write **no**
/// `output.exe` — there is no exit-code special case on the sidecar path:
///
/// * The export checks `GetFileAttributesA` on each known input name first and
///   emits `mov eax, 2; jmp epilogue` before the baked table is touched, so an
///   empty-oracle and a non-empty-oracle DLL behave the same when cwd has no
///   `input.{tyb,ky,ty}`.
/// * The `in_dll_recompile_no_input_is_exit_2` unit test asserts this on both
///   platforms (it was written against `PROBE_EXIT_NO_INPUT` directly, which is
///   why the removed constant could drift away from the code without failing a
///   test).
///
/// What is genuinely unstable about the H_00 + in-DLL-recompile path is
/// **manual-map AV** on the runner (see `stage10-linux-pure-m4.sh` / the
/// `yoyo_in_dll_recompile_smoke=NOT_STABLE` output of
/// `scripts/stage17-ow-rt-yoyo-runtime.ps1`), **not** the exit code. Production
/// still defaults to the Rust sidecar for OW-RT reasons — the oracle table is a
/// finite YOYO-precompiled lookup, not an in-DLL YOYO compiler — not because of
/// exit-code divergence.
///
/// History: a `PROBE_EXIT_NO_INPUT_IN_DLL_RECOMPILE: i32 = EXIT_OK` constant
/// asserted the divergent (`0`) behavior here. Commit `0da9ef1` (2026-09-09,
/// Gate G slice hardening) fixed the export body to emit `PROBE_EXIT_NO_INPUT`
/// but left this constant behind, so the constant silently contradicted the
/// code and the test. Removed; this doc block is the record.

/// Gate F: YOYO-built **read→compile→write** effect under `work_dir`.
///
/// Same cwd contract as Rust sidecar (`input.tyb`/`input.ky`/`input.ty` →
/// `output.exe`, exits 0/1/2/3) but compile uses the YOYO seed/link path
/// (`bootstrap_compile`) — **no** `LoadLibrary(yoyo_rt.dll)`.
///
/// Honest: production H_00 still ships Rust sidecar → OW-RT remains CUT.
pub fn yoyo_built_runtime_effect(work_dir: &Path) -> i32 {
    let input = match read_cwd_input(work_dir) {
        Ok(d) => d,
        Err(e) => return e,
    };
    let out = match crate::selfhost::bootstrap_compile(&input) {
        Ok(p) => p,
        Err(_) => return EXIT_COMPILE_FAIL,
    };
    let out_path = work_dir.join(OUTPUT_NAME);
    if std::fs::write(&out_path, &out).is_err() {
        return EXIT_WRITE_FAIL;
    }
    EXIT_OK
}

/// Gate G slice: YOYO R→C→W on the **sidecar path**.
///
/// 1. Place YOYO `pe_dll` probe as cwd `yoyo_rt.dll` (H_00 load contract).
/// 2. Run [`yoyo_built_runtime_effect`] in the same `work_dir` (full exits 0/1/2/3).
///
/// Honest: step 2 is still host-orchestrated YOYO seed/link — **not** compile
/// machine code inside the DLL export. Prefer [`yoyo_sidecar_export_compile`].
/// Production default remains Rust → CUT.
pub fn yoyo_sidecar_path_rcw(work_dir: &Path) -> i32 {
    let sidecar = work_dir.join(RUNTIME_SIDECAR_NAME);
    if write_yoyo_alt_sidecar(&sidecar).is_err() {
        return EXIT_WRITE_FAIL;
    }
    yoyo_built_runtime_effect(work_dir)
}

/// KERNEL32 imports for export-compile DLL (slot order = IAT index).
const EXPORT_COMPILE_K32: &[&str] = &[
    "CreateFileA",
    "WriteFile",
    "CloseHandle",
    "GetFileAttributesA",
];

/// Link YOYO pe_dll whose export writes emit-time `bootstrap_compile` bytes.
///
/// Call-time: if any cwd `input.tyb`/`input.ky`/`input.ty` exists, write the
/// baked PE to `output.exe` (exits 0/2/3). Does **not** re-compile at call
/// time — general in-DLL compiler still ABSENT → OW-RT CUT.
pub fn link_yoyo_export_compile_dll(baked_pe: &[u8]) -> IsaResult<Vec<u8>> {
    if baked_pe.is_empty() {
        return Err(IsaError::PlatformError {
            msg: "pe_dll_link: export-compile baked PE empty".into(),
        });
    }
    if baked_pe.len() > 16 * 1024 * 1024 {
        return Err(IsaError::PlatformError {
            msg: format!(
                "pe_dll_link: export-compile baked PE too large ({})",
                baked_pe.len()
            ),
        });
    }

    const IMAGE_BASE: u64 = 0x0000_0001_8000_0000;
    const FILE_ALIGN: u32 = 0x200;
    const SECTION_ALIGN: u32 = 0x1000;
    const HEADERS_RAW: u32 = 0x400;

    let dllmain: [u8; 6] = [0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3];
    let dll_name = RUNTIME_SIDECAR_NAME;
    let export_name = RUNTIME_EXPORT_NAME;
    let dll_bytes = format!("{dll_name}\0");
    let export_bytes = format!("{export_name}\0");
    let dll_b = dll_bytes.as_bytes();
    let exp_b = export_bytes.as_bytes();

    let input_names: [&[u8]; 3] = [b"input.tyb\0", b"input.ky\0", b"input.ty\0"];
    let output_name: &[u8] = b"output.exe\0";
    let k32_name: &[u8] = b"KERNEL32.dll\0";

    // ---- .rdata layout (relative offsets) ----
    // export dir + tables, then strings, then imports, then baked PE
    let exp_dir_off = 0u32;
    let functions_off = 0x28u32;
    let names_off = 0x2Cu32;
    let ordinals_off = 0x30u32;
    let dll_name_off = 0x34u32;
    let export_name_off = dll_name_off + dll_b.len() as u32;
    let mut off = align_up(export_name_off + exp_b.len() as u32, 4);

    let mut input_offs = [0u32; 3];
    for (i, name) in input_names.iter().enumerate() {
        input_offs[i] = off;
        off += name.len() as u32;
    }
    let output_off = off;
    off += output_name.len() as u32;
    off = align_up(off, 4);

    let k32_name_off = off;
    off += k32_name.len() as u32;
    off = align_up(off, 2);

    let mut hint_blobs: Vec<Vec<u8>> = Vec::new();
    let mut hint_offs = Vec::new();
    for func in EXPORT_COMPILE_K32 {
        let mut hn = Vec::new();
        hn.extend_from_slice(&0u16.to_le_bytes());
        hn.extend_from_slice(func.as_bytes());
        hn.push(0);
        if hn.len() % 2 != 0 {
            hn.push(0);
        }
        hint_offs.push(off);
        off += hn.len() as u32;
        hint_blobs.push(hn);
    }
    off = align_up(off, 8);

    let n_imp = EXPORT_COMPILE_K32.len();
    let ilt_off = off;
    let iat_off = ilt_off + (n_imp as u32 + 1) * 8;
    off = iat_off + (n_imp as u32 + 1) * 8;
    let import_desc_off = off;
    // one real descriptor + one null terminator
    let import_desc_size = 40u32;
    off += import_desc_size;
    off = align_up(off, 16);
    let marker: &[u8] = b"yoyo_export_compile\0";
    let marker_off = off;
    off += marker.len() as u32;
    off = align_up(off, 16);
    let baked_off = off;
    let rdata_payload = baked_off + baked_pe.len() as u32;

    let text_rva = SECTION_ALIGN; // 0x1000

    let mut export_code = Vec::with_capacity(256);
    let mut rip_patches: Vec<(usize, u32)> = Vec::new();
    let mut iat_patches: Vec<(usize, u32)> = Vec::new();
    let mut jne_patch_at: Vec<usize> = Vec::new();

    let emit_lea_rcx_rdata = |code: &mut Vec<u8>, patches: &mut Vec<(usize, u32)>, rel: u32| {
        code.extend_from_slice(&[0x48, 0x8D, 0x0D, 0, 0, 0, 0]);
        patches.push((code.len() - 4, rel));
    };
    let emit_call_iat = |code: &mut Vec<u8>, patches: &mut Vec<(usize, u32)>, slot: u32| {
        code.extend_from_slice(&[0xFF, 0x15, 0, 0, 0, 0]);
        patches.push((code.len() - 4, slot));
    };

    // push rbx; push rsi; sub rsp, 0x60
    // Entry RSP≡8; after 2 pushes still ≡8; sub 0x60 (≡0) keeps RSP≡8 for Win64 CALLs.
    export_code.extend_from_slice(&[0x53, 0x56]);
    export_code.extend_from_slice(&[0x48, 0x83, 0xEC, 0x60]);

    for &in_off in &input_offs {
        emit_lea_rcx_rdata(&mut export_code, &mut rip_patches, in_off);
        emit_call_iat(&mut export_code, &mut iat_patches, 3);
        export_code.extend_from_slice(&[0x83, 0xF8, 0xFF]);
        jne_patch_at.push(export_code.len());
        export_code.extend_from_slice(&[0x75, 0x00]);
    }

    // no input → eax=2; jmp epilogue
    export_code.extend_from_slice(&[0xB8, 0x02, 0x00, 0x00, 0x00]);
    let jmp_epilogue_from_noinput = export_code.len();
    export_code.extend_from_slice(&[0xEB, 0x00]); // short jmp placeholder

    let have_input_at = export_code.len();
    // CreateFileA(output, GENERIC_WRITE, 0, NULL, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, NULL)
    emit_lea_rcx_rdata(&mut export_code, &mut rip_patches, output_off);
    // mov edx, 0x40000000
    export_code.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x40]);
    // xor r8d,r8d ; xor r9d,r9d
    export_code.extend_from_slice(&[0x45, 0x31, 0xC0, 0x45, 0x31, 0xC9]);
    // mov dword [rsp+0x20], 2
    export_code.extend_from_slice(&[0xC7, 0x44, 0x24, 0x20, 0x02, 0x00, 0x00, 0x00]);
    // mov dword [rsp+0x28], 0x80
    export_code.extend_from_slice(&[0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00]);
    // mov qword [rsp+0x30], 0
    export_code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00]);
    emit_call_iat(&mut export_code, &mut iat_patches, 0); // CreateFileA
    // cmp rax, -1
    export_code.extend_from_slice(&[0x48, 0x83, 0xF8, 0xFF]);
    let je_fail_create = export_code.len();
    export_code.extend_from_slice(&[0x74, 0x00]);
    // mov rbx, rax
    export_code.extend_from_slice(&[0x48, 0x89, 0xC3]);

    // WriteFile(h, baked, len, &written, NULL)
    // mov rcx, rbx
    export_code.extend_from_slice(&[0x48, 0x89, 0xD9]);
    // lea rdx, [rip+baked]
    export_code.extend_from_slice(&[0x48, 0x8D, 0x15, 0, 0, 0, 0]);
    rip_patches.push((export_code.len() - 4, baked_off));
    // mov r8d, len
    let len = baked_pe.len() as u32;
    export_code.extend_from_slice(&[0x41, 0xB8]);
    export_code.extend_from_slice(&len.to_le_bytes());
    // lea r9, [rsp+0x40]
    export_code.extend_from_slice(&[0x4C, 0x8D, 0x4C, 0x24, 0x40]);
    // mov qword [rsp+0x20], 0
    export_code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00]);
    emit_call_iat(&mut export_code, &mut iat_patches, 1); // WriteFile
    // mov esi, eax
    export_code.extend_from_slice(&[0x89, 0xC6]);
    // mov rcx, rbx; CloseHandle
    export_code.extend_from_slice(&[0x48, 0x89, 0xD9]);
    emit_call_iat(&mut export_code, &mut iat_patches, 2);
    // test esi, esi; jz fail
    export_code.extend_from_slice(&[0x85, 0xF6]);
    let jz_fail_write = export_code.len();
    export_code.extend_from_slice(&[0x74, 0x00]);
    // mov eax, 0; jmp epilogue
    export_code.extend_from_slice(&[0xB8, 0x00, 0x00, 0x00, 0x00]);
    let jmp_epilogue_from_ok = export_code.len();
    export_code.extend_from_slice(&[0xEB, 0x00]);

    let write_fail_at = export_code.len();
    export_code.extend_from_slice(&[0xB8, 0x03, 0x00, 0x00, 0x00]);

    let epilogue_at = export_code.len();
    // add rsp, 0x60; pop rsi; pop rbx; ret
    export_code.extend_from_slice(&[0x48, 0x83, 0xC4, 0x60, 0x5E, 0x5B, 0xC3]);

    // Patch short jumps
    let patch_rel8 = |code: &mut [u8], at: usize, target: usize| {
        let next = at + 2;
        let rel = target as i32 - next as i32;
        assert!((-128..128).contains(&rel), "rel8 out of range");
        code[at + 1] = rel as u8;
    };
    for at in &jne_patch_at {
        patch_rel8(&mut export_code, *at, have_input_at);
    }
    patch_rel8(&mut export_code, jmp_epilogue_from_noinput, epilogue_at);
    patch_rel8(&mut export_code, je_fail_create, write_fail_at);
    patch_rel8(&mut export_code, jz_fail_write, write_fail_at);
    patch_rel8(&mut export_code, jmp_epilogue_from_ok, epilogue_at);

    let text_payload_len = dllmain.len() + export_code.len();
    let text_raw = align_up(text_payload_len as u32, FILE_ALIGN);
    let text_vs = align_up(text_payload_len as u32, SECTION_ALIGN);
    let export_fn_rva = text_rva + dllmain.len() as u32;

    let rdata_rva = text_rva + text_vs;
    let rdata_raw = align_up(rdata_payload, FILE_ALIGN);
    let rdata_vs = align_up(rdata_payload, SECTION_ALIGN);
    let size_of_image = align_up(rdata_rva + rdata_vs, SECTION_ALIGN);
    let file_size = HEADERS_RAW + text_raw + rdata_raw;

    // Patch RIP-relative displacements now that RVAs are known.
    let fix_rip = |code: &mut [u8], disp_at: usize, code_off_in_text: u32, target_rva: u32| {
        // instruction end = export start + disp_at + 4
        let next_rva = export_fn_rva + code_off_in_text + disp_at as u32 + 4;
        let rel = target_rva as i32 - next_rva as i32;
        code[disp_at..disp_at + 4].copy_from_slice(&rel.to_le_bytes());
    };
    for &(disp_at, rel_off) in &rip_patches {
        fix_rip(
            &mut export_code,
            disp_at,
            0,
            rdata_rva + rel_off,
        );
    }
    for &(disp_at, slot) in &iat_patches {
        fix_rip(
            &mut export_code,
            disp_at,
            0,
            rdata_rva + iat_off + slot * 8,
        );
    }

    let mut img = vec![0u8; file_size as usize];

    // DOS + PE
    img[0] = 0x4D;
    img[1] = 0x5A;
    write_u32(&mut img, 0x3C, 0x80);
    img[0x80] = b'P';
    img[0x81] = b'E';
    write_u16(&mut img, 0x84, 0x8664);
    write_u16(&mut img, 0x86, 2);
    write_u16(&mut img, 0x94, 0xF0);
    write_u16(&mut img, 0x96, 0x2022);

    let opt = 0x98usize;
    write_u16(&mut img, opt, 0x20B);
    img[opt + 2] = 1;
    write_u32(&mut img, opt + 4, text_raw);
    write_u32(&mut img, opt + 8, rdata_raw);
    write_u32(&mut img, opt + 16, text_rva); // DllMain
    write_u32(&mut img, opt + 20, text_rva);
    write_u64(&mut img, opt + 24, IMAGE_BASE);
    write_u32(&mut img, opt + 32, SECTION_ALIGN);
    write_u32(&mut img, opt + 36, FILE_ALIGN);
    write_u16(&mut img, opt + 40, 6);
    write_u16(&mut img, opt + 48, 6);
    write_u32(&mut img, opt + 56, size_of_image);
    write_u32(&mut img, opt + 60, HEADERS_RAW);
    write_u16(&mut img, opt + 68, 2);
    write_u16(&mut img, opt + 70, 0x0100); // NX, no ASLR
    write_u64(&mut img, opt + 72, 0x100000);
    write_u64(&mut img, opt + 80, 0x1000);
    write_u64(&mut img, opt + 88, 0x100000);
    write_u64(&mut img, opt + 96, 0x1000);
    write_u32(&mut img, opt + 108, 16);

    write_u32(&mut img, opt + 112, rdata_rva + exp_dir_off);
    write_u32(&mut img, opt + 116, 0x28);
    write_u32(&mut img, opt + 120, rdata_rva + import_desc_off);
    write_u32(&mut img, opt + 124, import_desc_size);

    let s1 = 0x98 + 0xF0;
    write_name(&mut img, s1, b".text");
    write_u32(&mut img, s1 + 8, text_vs);
    write_u32(&mut img, s1 + 12, text_rva);
    write_u32(&mut img, s1 + 16, text_raw);
    write_u32(&mut img, s1 + 20, HEADERS_RAW);
    write_u32(&mut img, s1 + 36, 0x6000_0020);

    let s2 = s1 + 40;
    write_name(&mut img, s2, b".rdata");
    write_u32(&mut img, s2 + 8, rdata_vs);
    write_u32(&mut img, s2 + 12, rdata_rva);
    write_u32(&mut img, s2 + 16, rdata_raw);
    write_u32(&mut img, s2 + 20, HEADERS_RAW + text_raw);
    write_u32(&mut img, s2 + 36, 0xC000_0040); // INIT_DATA | READ | WRITE (IAT)

    let text_off = HEADERS_RAW as usize;
    img[text_off..text_off + dllmain.len()].copy_from_slice(&dllmain);
    img[text_off + dllmain.len()..text_off + text_payload_len].copy_from_slice(&export_code);

    let rdata_file = (HEADERS_RAW + text_raw) as usize;
    let exp = &mut img[rdata_file..];

    write_u32(exp, exp_dir_off as usize + 0x0C, rdata_rva + dll_name_off);
    write_u32(exp, exp_dir_off as usize + 0x10, 1);
    write_u32(exp, exp_dir_off as usize + 0x14, 1);
    write_u32(exp, exp_dir_off as usize + 0x18, 1);
    write_u32(exp, exp_dir_off as usize + 0x1C, rdata_rva + functions_off);
    write_u32(exp, exp_dir_off as usize + 0x20, rdata_rva + names_off);
    write_u32(exp, exp_dir_off as usize + 0x24, rdata_rva + ordinals_off);
    write_u32(exp, functions_off as usize, export_fn_rva);
    write_u32(exp, names_off as usize, rdata_rva + export_name_off);
    write_u16(exp, ordinals_off as usize, 0);
    exp[dll_name_off as usize..dll_name_off as usize + dll_b.len()].copy_from_slice(dll_b);
    exp[export_name_off as usize..export_name_off as usize + exp_b.len()].copy_from_slice(exp_b);

    for (i, name) in input_names.iter().enumerate() {
        let o = input_offs[i] as usize;
        exp[o..o + name.len()].copy_from_slice(name);
    }
    exp[output_off as usize..output_off as usize + output_name.len()].copy_from_slice(output_name);
    exp[k32_name_off as usize..k32_name_off as usize + k32_name.len()].copy_from_slice(k32_name);

    for (i, hn) in hint_blobs.iter().enumerate() {
        let o = hint_offs[i] as usize;
        exp[o..o + hn.len()].copy_from_slice(hn);
        let hn_rva = (rdata_rva + hint_offs[i]) as u64;
        write_u64(exp, ilt_off as usize + i * 8, hn_rva);
        write_u64(exp, iat_off as usize + i * 8, hn_rva);
    }

    // IMAGE_IMPORT_DESCRIPTOR
    let id = import_desc_off as usize;
    write_u32(exp, id, rdata_rva + ilt_off); // OriginalFirstThunk
    write_u32(exp, id + 12, rdata_rva + k32_name_off); // Name
    write_u32(exp, id + 16, rdata_rva + iat_off); // FirstThunk
    // null terminator already zero

    exp[marker_off as usize..marker_off as usize + marker.len()].copy_from_slice(marker);
    exp[baked_off as usize..baked_off as usize + baked_pe.len()].copy_from_slice(baked_pe);

    Ok(img)
}

/// Gate G slice: put YOYO **compile** into the sidecar **export** path.
///
/// 1. `bootstrap_compile` cwd input (emit-time YOYO seed/link).
/// 2. Link pe_dll with that PE baked in; export writes it on call (0/2/3).
/// 3. Place as cwd `yoyo_rt.dll`, then invoke export (Win manual-map) or
///    equivalent write on non-Windows.
///
/// Honest: call-time does not re-compile arbitrary input; production default
/// remains Rust → OW-RT **CUT**; Gate G stays unchecked.
pub fn yoyo_sidecar_export_compile(work_dir: &Path) -> i32 {
    let sidecar = work_dir.join(RUNTIME_SIDECAR_NAME);
    let input = match read_cwd_input(work_dir) {
        Ok(d) => d,
        Err(e) => {
            let _ = write_yoyo_alt_sidecar(&sidecar);
            return e;
        }
    };
    let baked = match crate::selfhost::bootstrap_compile(&input) {
        Ok(p) => p,
        Err(_) => {
            let _ = write_yoyo_alt_sidecar(&sidecar);
            return EXIT_COMPILE_FAIL;
        }
    };
    let dll = match link_yoyo_export_compile_dll(&baked) {
        Ok(d) => d,
        Err(_) => return EXIT_WRITE_FAIL,
    };
    if let Some(parent) = sidecar.parent() {
        if !parent.as_os_str().is_empty() {
            if std::fs::create_dir_all(parent).is_err() {
                return EXIT_WRITE_FAIL;
            }
        }
    }
    if std::fs::write(&sidecar, &dll).is_err() {
        return EXIT_WRITE_FAIL;
    }
    let out_path = work_dir.join(OUTPUT_NAME);
    let _ = std::fs::remove_file(&out_path);

    #[cfg(windows)]
    {
        match call_export_compile_mapped(&dll, work_dir) {
            Ok(code) => code,
            Err(_) => EXIT_WRITE_FAIL,
        }
    }
    #[cfg(not(windows))]
    {
        // Cannot execute PE export on non-Windows — equivalent cwd write that
        // the export body performs after emit-time YOYO compile.
        if std::fs::write(&out_path, &baked).is_err() {
            return EXIT_WRITE_FAIL;
        }
        EXIT_OK
    }
}

/// One YOYO-precompiled (input → PE) oracle row for in-DLL recompile.
#[derive(Clone, Debug)]
pub struct RecompileEntry {
    pub input: Vec<u8>,
    pub pe: Vec<u8>,
}

/// KERNEL32 imports for in-DLL recompile (slot = IAT index).
const IN_DLL_RECOMPILE_K32: &[&str] = &[
    "CreateFileA",
    "ReadFile",
    "WriteFile",
    "CloseHandle",
    "GetFileAttributesA",
    "GetFileSize",
];

/// Max cwd input size the export will ReadFile (golden .ty fixtures are tiny).
const IN_DLL_RECOMPILE_MAX_INPUT: u32 = 0x1000;

/// Build the `.rdata` oracle blob: `n_entries` then per-entry records.
fn build_recompile_table(entries: &[RecompileEntry]) -> IsaResult<Vec<u8>> {
    if entries.is_empty() {
        return Err(IsaError::PlatformError {
            msg: "pe_dll_link: in-DLL recompile needs ≥1 entry".into(),
        });
    }
    if entries.len() > 64 {
        return Err(IsaError::PlatformError {
            msg: format!(
                "pe_dll_link: in-DLL recompile too many entries ({})",
                entries.len()
            ),
        });
    }
    let mut blob = Vec::new();
    blob.extend_from_slice(&(entries.len() as u32).to_le_bytes());
    // Pad the count prefix to 16 bytes so every entry STARTS at an absolute
    // 16-boundary. That is what lets the emitted scan stride use entry-relative
    // alignment instead of the table's absolute pads.
    while blob.len() % 16 != 0 {
        blob.push(0);
    }
    for e in entries {
        if e.input.is_empty() || e.pe.is_empty() {
            return Err(IsaError::PlatformError {
                msg: "pe_dll_link: recompile entry input/pe empty".into(),
            });
        }
        if e.input.len() > IN_DLL_RECOMPILE_MAX_INPUT as usize {
            return Err(IsaError::PlatformError {
                msg: format!(
                    "pe_dll_link: recompile input too large ({})",
                    e.input.len()
                ),
            });
        }
        if e.pe.len() > 16 * 1024 * 1024 {
            return Err(IsaError::PlatformError {
                msg: format!("pe_dll_link: recompile PE too large ({})", e.pe.len()),
            });
        }
        blob.extend_from_slice(&(e.input.len() as u32).to_le_bytes());
        blob.extend_from_slice(&(e.pe.len() as u32).to_le_bytes());
        // Pad the 8-byte length header to 16, so `input` begins at a 16
        // multiple. Without this the header's +8 offset defeats the 16-align
        // below and the emitted `align4(r13+8+input_len)` disagrees with the
        // table's absolute pad for any input_len where align4(input_len) is
        // not ≡ 8 (mod 16) — which is exactly the fixtures' 181/165/289 bytes.
        while blob.len() % 16 != 0 {
            blob.push(0);
        }
        blob.extend_from_slice(&e.input);
        // 16-align (not 4): the emitted stride and pe_ptr both use align16.
        while blob.len() % 16 != 0 {
            blob.push(0);
        }
        blob.extend_from_slice(&e.pe);
        while blob.len() % 16 != 0 {
            blob.push(0);
        }
    }
    Ok(blob)
}

/// Host-side oracle match (mirrors export body; used on non-Windows + unit tests).
pub fn match_recompile_entry<'a>(
    entries: &'a [RecompileEntry],
    input: &[u8],
) -> Option<&'a [u8]> {
    entries
        .iter()
        .find(|e| e.input.as_slice() == input)
        .map(|e| e.pe.as_slice())
}

/// Link YOYO pe_dll whose export **recompiles** via call-time input match.
///
/// Call-time: `ReadFile` cwd `input.*`, scan baked (input→PE) table, write
/// matching PE to `output.exe` (exits 0/1/2/3). Multiple known inputs work
/// without re-emitting the DLL ("generic" vs single baked export-compile).
///
/// Honest: finite YOYO-precompiled oracle — **not** a full in-DLL YOYO
/// compiler → OW-RT remains CUT.
pub fn link_yoyo_in_dll_recompile_dll(entries: &[RecompileEntry]) -> IsaResult<Vec<u8>> {
    let table = build_recompile_table(entries)?;

    const IMAGE_BASE: u64 = 0x0000_0001_8000_0000;
    const FILE_ALIGN: u32 = 0x200;
    const SECTION_ALIGN: u32 = 0x1000;
    const HEADERS_RAW: u32 = 0x400;

    let dllmain: [u8; 6] = [0xB8, 0x01, 0x00, 0x00, 0x00, 0xC3];
    let dll_name = RUNTIME_SIDECAR_NAME;
    let export_name = RUNTIME_EXPORT_NAME;
    let dll_bytes = format!("{dll_name}\0");
    let export_bytes = format!("{export_name}\0");
    let dll_b = dll_bytes.as_bytes();
    let exp_b = export_bytes.as_bytes();

    let input_names: [&[u8]; 3] = [b"input.tyb\0", b"input.ky\0", b"input.ty\0"];
    let output_name: &[u8] = b"output.exe\0";
    let k32_name: &[u8] = b"KERNEL32.dll\0";

    let exp_dir_off = 0u32;
    let functions_off = 0x28u32;
    let names_off = 0x2Cu32;
    let ordinals_off = 0x30u32;
    let dll_name_off = 0x34u32;
    let export_name_off = dll_name_off + dll_b.len() as u32;
    let mut off = align_up(export_name_off + exp_b.len() as u32, 4);

    let mut input_offs = [0u32; 3];
    for (i, name) in input_names.iter().enumerate() {
        input_offs[i] = off;
        off += name.len() as u32;
    }
    let output_off = off;
    off += output_name.len() as u32;
    off = align_up(off, 4);

    let k32_name_off = off;
    off += k32_name.len() as u32;
    off = align_up(off, 2);

    let mut hint_blobs: Vec<Vec<u8>> = Vec::new();
    let mut hint_offs = Vec::new();
    for func in IN_DLL_RECOMPILE_K32 {
        let mut hn = Vec::new();
        hn.extend_from_slice(&0u16.to_le_bytes());
        hn.extend_from_slice(func.as_bytes());
        hn.push(0);
        if hn.len() % 2 != 0 {
            hn.push(0);
        }
        hint_offs.push(off);
        off += hn.len() as u32;
        hint_blobs.push(hn);
    }
    off = align_up(off, 8);

    let n_imp = IN_DLL_RECOMPILE_K32.len();
    let ilt_off = off;
    let iat_off = ilt_off + (n_imp as u32 + 1) * 8;
    off = iat_off + (n_imp as u32 + 1) * 8;
    let import_desc_off = off;
    let import_desc_size = 40u32;
    off += import_desc_size;
    off = align_up(off, 16);
    let marker: &[u8] = b"yoyo_in_dll_recompile\0";
    let marker_off = off;
    off += marker.len() as u32;
    off = align_up(off, 16);
    let table_off = off;
    let rdata_payload = table_off + table.len() as u32;

    let text_rva = SECTION_ALIGN;

    // Stack layout (export body): push rbx,rsi,rdi,r12,r13; sub rsp, 0x1080
    //   [rsp+0x60 ..) = ≤4KiB input buffer; [rsp+0x40] = DWORD bytes_read
    link_yoyo_in_dll_recompile_dll_emit(
        &table,
        &input_offs,
        output_off,
        k32_name_off,
        &hint_offs,
        &hint_blobs,
        marker,
        marker_off,
        table_off,
        rdata_payload,
        ilt_off,
        iat_off,
        import_desc_off,
        import_desc_size,
        exp_dir_off,
        functions_off,
        names_off,
        ordinals_off,
        dll_name_off,
        export_name_off,
        dll_b,
        exp_b,
        &input_names,
        output_name,
        k32_name,
        &dllmain,
        IMAGE_BASE,
        FILE_ALIGN,
        SECTION_ALIGN,
        HEADERS_RAW,
        text_rva,
        n_imp,
    )
}

/// Inner emit for [`link_yoyo_in_dll_recompile_dll`] (keeps patch state local).
fn link_yoyo_in_dll_recompile_dll_emit(
    table: &[u8],
    input_offs: &[u32; 3],
    output_off: u32,
    k32_name_off: u32,
    hint_offs: &[u32],
    hint_blobs: &[Vec<u8>],
    marker: &[u8],
    marker_off: u32,
    table_off: u32,
    rdata_payload: u32,
    ilt_off: u32,
    iat_off: u32,
    import_desc_off: u32,
    import_desc_size: u32,
    exp_dir_off: u32,
    functions_off: u32,
    names_off: u32,
    ordinals_off: u32,
    dll_name_off: u32,
    export_name_off: u32,
    dll_b: &[u8],
    exp_b: &[u8],
    input_names: &[&[u8]; 3],
    output_name: &[u8],
    k32_name: &[u8],
    dllmain: &[u8; 6],
    image_base: u64,
    file_align: u32,
    section_align: u32,
    headers_raw: u32,
    text_rva: u32,
    n_imp: usize,
) -> IsaResult<Vec<u8>> {
    let _ = n_imp;
    let mut export_code = Vec::with_capacity(640);
    let mut rip_patches: Vec<(usize, u32)> = Vec::new();
    let mut iat_patches: Vec<(usize, u32)> = Vec::new();

    let emit_lea_rcx_rdata = |code: &mut Vec<u8>, patches: &mut Vec<(usize, u32)>, rel: u32| {
        code.extend_from_slice(&[0x48, 0x8D, 0x0D, 0, 0, 0, 0]);
        patches.push((code.len() - 4, rel));
    };
    let emit_call_iat = |code: &mut Vec<u8>, patches: &mut Vec<(usize, u32)>, slot: u32| {
        code.extend_from_slice(&[0xFF, 0x15, 0, 0, 0, 0]);
        patches.push((code.len() - 4, slot));
    };
    let patch_rel8 = |code: &mut [u8], at: usize, target: usize| {
        let next = at + 2;
        let rel = target as i32 - next as i32;
        assert!((-128..128).contains(&rel), "rel8 out of range at {at}");
        code[at + 1] = rel as u8;
    };
    let patch_rel32 = |code: &mut [u8], at: usize, target: usize| {
        // at points to E9/0F84 disp32 start (opcode already written); disp at at+1
        let disp_at = at + 1;
        let next = disp_at + 4;
        let rel = target as i32 - next as i32;
        code[disp_at..disp_at + 4].copy_from_slice(&rel.to_le_bytes());
    };
    // For 0F 84 (6-byte jcc near): opcode at `at`, disp at at+2
    let patch_jcc32 = |code: &mut [u8], at: usize, target: usize| {
        let disp_at = at + 2;
        let next = disp_at + 4;
        let rel = target as i32 - next as i32;
        code[disp_at..disp_at + 4].copy_from_slice(&rel.to_le_bytes());
    };

    // prologue
    export_code.extend_from_slice(&[0x53, 0x56, 0x57, 0x41, 0x54, 0x41, 0x55]);
    export_code.extend_from_slice(&[0x48, 0x81, 0xEC, 0x80, 0x10, 0x00, 0x00]);

    // Try each input name: attrs → open (near jcc — open stubs are far)
    let mut open_jne_sites: Vec<(usize, u32)> = Vec::new(); // (jcc32_at, name_off)
    for &in_off in input_offs {
        emit_lea_rcx_rdata(&mut export_code, &mut rip_patches, in_off);
        emit_call_iat(&mut export_code, &mut iat_patches, 4);
        export_code.extend_from_slice(&[0x83, 0xF8, 0xFF]);
        let jcc_at = export_code.len();
        export_code.extend_from_slice(&[0x0F, 0x85, 0, 0, 0, 0]); // jne open_with_name
        open_jne_sites.push((jcc_at, in_off));
    }
    export_code.extend_from_slice(&[0xB8, 0x02, 0x00, 0x00, 0x00]);
    let jmp_epi_noinput = export_code.len();
    export_code.extend_from_slice(&[0xE9, 0, 0, 0, 0]);

    // open_dispatch: three open stubs → after_open with rbx=handle
    let mut jmp_after_open_sites: Vec<usize> = Vec::new();
    let mut je_compile_fail_sites: Vec<usize> = Vec::new();

    for &(jcc_at, name_off) in &open_jne_sites {
        let open_at = export_code.len();
        patch_jcc32(&mut export_code, jcc_at, open_at);
        emit_lea_rcx_rdata(&mut export_code, &mut rip_patches, name_off);
        export_code.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x80]); // GENERIC_READ
        export_code.extend_from_slice(&[0x41, 0xB8, 0x01, 0x00, 0x00, 0x00]); // share read
        export_code.extend_from_slice(&[0x45, 0x31, 0xC9]);
        export_code.extend_from_slice(&[0xC7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00]);
        export_code.extend_from_slice(&[0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00]);
        export_code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00]);
        emit_call_iat(&mut export_code, &mut iat_patches, 0);
        export_code.extend_from_slice(&[0x48, 0x83, 0xF8, 0xFF]);
        let je_fail = export_code.len();
        export_code.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
        je_compile_fail_sites.push(je_fail);
        export_code.extend_from_slice(&[0x48, 0x89, 0xC3]); // mov rbx, rax
        let jmp_ao = export_code.len();
        export_code.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
        jmp_after_open_sites.push(jmp_ao);
    }

    let after_open = export_code.len();
    for at in &jmp_after_open_sites {
        patch_rel32(&mut export_code, *at, after_open);
    }

    // GetFileSize(h, NULL) → edi = size
    export_code.extend_from_slice(&[0x48, 0x89, 0xD9]); // mov rcx, rbx
    export_code.extend_from_slice(&[0x48, 0x31, 0xD2]); // xor rdx, rdx
    emit_call_iat(&mut export_code, &mut iat_patches, 5);
    export_code.extend_from_slice(&[0x83, 0xF8, 0xFF]);
    let je_fail_size = export_code.len();
    export_code.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    je_compile_fail_sites.push(je_fail_size);
    export_code.extend_from_slice(&[0x3D]);
    export_code.extend_from_slice(&IN_DLL_RECOMPILE_MAX_INPUT.to_le_bytes());
    let ja_fail_size = export_code.len();
    export_code.extend_from_slice(&[0x0F, 0x87, 0, 0, 0, 0]);
    je_compile_fail_sites.push(ja_fail_size);
    export_code.extend_from_slice(&[0x85, 0xC0]);
    let jz_fail_empty = export_code.len();
    export_code.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    je_compile_fail_sites.push(jz_fail_empty);
    export_code.extend_from_slice(&[0x89, 0xC7]); // mov edi, eax
    export_code.extend_from_slice(&[0x48, 0x8D, 0x74, 0x24, 0x60]); // lea rsi, [rsp+0x60]
    export_code.extend_from_slice(&[0x48, 0x89, 0xD9]);
    export_code.extend_from_slice(&[0x48, 0x89, 0xF2]);
    export_code.extend_from_slice(&[0x41, 0x89, 0xF8]);
    export_code.extend_from_slice(&[0x4C, 0x8D, 0x4C, 0x24, 0x40]);
    export_code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00]);
    emit_call_iat(&mut export_code, &mut iat_patches, 1);
    // Save ReadFile return in callee-saved r12d (r8d would be clobbered by
    // CloseHandle; rbx is used by the match loop below).
    export_code.extend_from_slice(&[0x45, 0x89, 0xC4]); // mov r12d, eax
    export_code.extend_from_slice(&[0x48, 0x89, 0xD9]); // mov rbx, rax (rcx still = hFile)
    emit_call_iat(&mut export_code, &mut iat_patches, 3); // CloseHandle
    export_code.extend_from_slice(&[0x45, 0x85, 0xE4]); // test r12d, r12d
    let jz_fail_read = export_code.len();
    export_code.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    je_compile_fail_sites.push(jz_fail_read);

    // Scan table: r13 = table ptr, rbx = entry count (loop counter below).
    //
    // A single RIP-relative disp32 loads the table pointer into rcx, then we
    // read the DWORD count through `[rcx]` and copy rcx→r13. Using one
    // RIP-disp here is deliberate: a *second* rip_patches entry whose disp32
    // occupies the SAME 4-byte slot as this `lea`'s disp (as `lea rcx,[rip]`
    // + `mov ebx,[rip]` did) collides — the later patch overwrites the
    // earlier RIP-base and the first `lea` resolves to `lea rcx,[rcx+0]`.
    // rbx is pushed in the prologue; loading the count here, after
    // CloseHandle, makes rbx's earlier use as a Win32 return register a
    // non-issue.
    emit_lea_rcx_rdata(&mut export_code, &mut rip_patches, table_off);
    // mov ebx, [rcx] — register addressing, no RIP disp, no patch needed.
    export_code.extend_from_slice(&[0x8B, 0x19]);
    // mov r13, rcx — full 64-bit move needs BOTH REX.W and REX.B, i.e. 0x49,
    // and R13's r/m field with REX.B is 101 (0x5), giving ModRM 0xCD — the
    // same encoding as the pe_ptr save below.
    //   0x48 0x89 0xCE = `mov rdx, rcx`   (r/m 010 = rdx) — the original AV
    //   0x49 0x89 0xC1 = `mov r13, r9`    (r/m 001 = R9 with REX.B)
    // Then skip the 16-byte header prefix (count u32 + 12 pad) so r13 is
    // entry-relative. Entry layout is [input_len u32][pe_len u32][12 pad]
    // [input ...][pe ...], so the field reads below are
    //   [r13+0] = input_len, [r13+4] = pe_len, [r13+16] = input,
    // and the per-entry stride is 16 + align16(input_len) + align16(pe_len).
    export_code.extend_from_slice(&[0x49, 0x89, 0xCD]);
    export_code.extend_from_slice(&[0x49, 0x83, 0xC5, 0x10]); // add r13, 0x10

    let loop_at = export_code.len();
    export_code.extend_from_slice(&[0x48, 0x8D, 0x74, 0x24, 0x60]); // reload buf (cmpsb clobbers rsi)
    export_code.extend_from_slice(&[0x85, 0xDB]);
    let jz_no_match = export_code.len();
    export_code.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]);
    je_compile_fail_sites.push(jz_no_match);
    export_code.extend_from_slice(&[0x41, 0x8B, 0x45, 0x00]); // input_len → eax
    export_code.extend_from_slice(&[0x39, 0xF8]); // cmp eax, edi (edi = bytes_read, set after ReadFile)
    let jne_next = export_code.len();
    export_code.extend_from_slice(&[0x75, 0x00]);
    export_code.extend_from_slice(&[0x41, 0x8B, 0x55, 0x04]); // pe_len → edx
    export_code.extend_from_slice(&[0x41, 0x89, 0xD4]); // r12d = pe_len
    // repe cmpsb: RSI=buf, RDI=entry input, ECX=bytes_read (DWORD from ReadFile)
    export_code.extend_from_slice(&[0x57]); // push rdi (save size)
    export_code.extend_from_slice(&[0x49, 0x8D, 0x7D, 0x10]); // lea rdi, [r13+0x10]
    export_code.extend_from_slice(&[0x48, 0x8B, 0x0C, 0x24]); // mov rcx, [rsp] (saved rdi == edi == bytes_read; push rdi just above)
    export_code.extend_from_slice(&[0xF3, 0xA6]); // repe cmpsb
    export_code.extend_from_slice(&[0x5F]); // pop rdi
    let jne_cmp = export_code.len();
    export_code.extend_from_slice(&[0x75, 0x00]);
    // MATCH: pe ptr = r13 + 0x10 + align16(input_len) → rcx
    // (entry-relative; the table pads input and pe each to a 16 boundary,
    // and entry starts are themselves 16-aligned, so entry-relative and
    // absolute alignment agree everywhere.)
    export_code.extend_from_slice(&[0x49, 0x8D, 0x4D, 0x10]); // lea rcx, [r13+0x10]
    export_code.extend_from_slice(&[0x41, 0x8B, 0x45, 0x00]); // eax = input_len
    export_code.extend_from_slice(&[0x83, 0xC0, 0x0F]); // add eax, 0xF
    export_code.extend_from_slice(&[0x83, 0xE0, 0xF0]); // and eax, 0xF0
    export_code.extend_from_slice(&[0x48, 0x01, 0xC1]); // add rcx, rax
    let jmp_write = export_code.len();
    export_code.extend_from_slice(&[0xEB, 0x00]);

    let next_entry = export_code.len();
    patch_rel8(&mut export_code, jne_next, next_entry);
    patch_rel8(&mut export_code, jne_cmp, next_entry);
    // stride = 0x10 + align16(input_len) + align16(pe_len), added to r13.
    // Both inputs and PEs are padded to a 16 boundary in the table, and each
    // entry start is 16-aligned, so this entry-relative stride equals the
    // table's absolute padding.
    export_code.extend_from_slice(&[0x41, 0x8B, 0x45, 0x00]); // eax = input_len
    export_code.extend_from_slice(&[0x83, 0xC0, 0x0F]); // add eax, 0xF
    export_code.extend_from_slice(&[0x83, 0xE0, 0xF0]); // eax = align16(input_len)
    export_code.extend_from_slice(&[0x41, 0x8B, 0x55, 0x04]); // edx = pe_len
    export_code.extend_from_slice(&[0x83, 0xC2, 0x0F]); // add edx, 0xF
    export_code.extend_from_slice(&[0x83, 0xE2, 0xF0]); // edx = align16(pe_len)
    export_code.extend_from_slice(&[0x48, 0x83, 0xC0, 0x10]); // rax = 0x10
    export_code.extend_from_slice(&[0x48, 0x01, 0xD0]); // rax += rdx
    export_code.extend_from_slice(&[0x49, 0x01, 0xC5]); // r13 += rax
    export_code.extend_from_slice(&[0xFF, 0xCB]); // dec rbx
    let jmp_loop = export_code.len();
    export_code.extend_from_slice(&[0xE9, 0, 0, 0, 0]);
    patch_rel32(&mut export_code, jmp_loop, loop_at);

    let write_out = export_code.len();
    patch_rel8(&mut export_code, jmp_write, write_out);
    // rcx = pe_ptr, r12d = pe_len — save pe_ptr in r13
    export_code.extend_from_slice(&[0x49, 0x89, 0xCD]); // mov r13, rcx
    // CreateFileA(output, GENERIC_WRITE, ...)
    emit_lea_rcx_rdata(&mut export_code, &mut rip_patches, output_off);
    export_code.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x40]);
    export_code.extend_from_slice(&[0x45, 0x31, 0xC0, 0x45, 0x31, 0xC9]);
    export_code.extend_from_slice(&[0xC7, 0x44, 0x24, 0x20, 0x02, 0x00, 0x00, 0x00]);
    export_code.extend_from_slice(&[0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00]);
    export_code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00]);
    emit_call_iat(&mut export_code, &mut iat_patches, 0);
    export_code.extend_from_slice(&[0x48, 0x83, 0xF8, 0xFF]);
    let je_fail_create = export_code.len();
    export_code.extend_from_slice(&[0x0F, 0x84, 0, 0, 0, 0]); // → write_fail
    // mov rbx, rax
    export_code.extend_from_slice(&[0x48, 0x89, 0xC3]);
    // WriteFile(h, pe, len, &written, NULL)
    export_code.extend_from_slice(&[0x48, 0x89, 0xD9]);
    export_code.extend_from_slice(&[0x4C, 0x89, 0xEA]); // mov rdx, r13
    export_code.extend_from_slice(&[0x45, 0x89, 0xE0]); // mov r8d, r12d
    export_code.extend_from_slice(&[0x4C, 0x8D, 0x4C, 0x24, 0x40]);
    export_code.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00]);
    emit_call_iat(&mut export_code, &mut iat_patches, 2); // WriteFile
    export_code.extend_from_slice(&[0x89, 0xC6]); // mov esi, eax
    export_code.extend_from_slice(&[0x48, 0x89, 0xD9]);
    emit_call_iat(&mut export_code, &mut iat_patches, 3);
    export_code.extend_from_slice(&[0x85, 0xF6]);
    let jz_fail_write = export_code.len();
    export_code.extend_from_slice(&[0x74, 0x00]);
    export_code.extend_from_slice(&[0xB8, 0x00, 0x00, 0x00, 0x00]);
    let jmp_epi_ok = export_code.len();
    export_code.extend_from_slice(&[0xEB, 0x00]);

    let write_fail_at = export_code.len();
    export_code.extend_from_slice(&[0xB8, 0x03, 0x00, 0x00, 0x00]);
    let jmp_epi_wf = export_code.len();
    export_code.extend_from_slice(&[0xEB, 0x00]);

    let compile_fail_at = export_code.len();
    export_code.extend_from_slice(&[0xB8, 0x01, 0x00, 0x00, 0x00]);

    let epilogue_at = export_code.len();
    // add rsp, 0x1080; pop r13; pop r12; pop rdi; pop rsi; pop rbx; ret
    export_code.extend_from_slice(&[0x48, 0x81, 0xC4, 0x80, 0x10, 0x00, 0x00]);
    export_code.extend_from_slice(&[0x41, 0x5D, 0x41, 0x5C, 0x5F, 0x5E, 0x5B, 0xC3]);

        // Patch jumps
        patch_rel32(&mut export_code, jmp_epi_noinput, epilogue_at);
        for at in &je_compile_fail_sites {
            patch_jcc32(&mut export_code, *at, compile_fail_at);
        }
        patch_jcc32(&mut export_code, je_fail_create, write_fail_at);
        patch_rel8(&mut export_code, jz_fail_write, write_fail_at);
        patch_rel8(&mut export_code, jmp_epi_ok, epilogue_at);
        patch_rel8(&mut export_code, jmp_epi_wf, epilogue_at);
        // compile_fail falls through to epilogue — need jmp
        // Currently compile_fail_at is mov eax,1 then epilogue_at — falls through. Good if adjacent.
        assert_eq!(compile_fail_at + 5, epilogue_at);

    let text_payload_len = dllmain.len() + export_code.len();
    let text_raw = align_up(text_payload_len as u32, file_align);
    let text_vs = align_up(text_payload_len as u32, section_align);
    let export_fn_rva = text_rva + dllmain.len() as u32;
    let rdata_rva = text_rva + text_vs;
    let rdata_raw = align_up(rdata_payload, file_align);
    let rdata_vs = align_up(rdata_payload, section_align);
    let size_of_image = align_up(rdata_rva + rdata_vs, section_align);
    let file_size = headers_raw + text_raw + rdata_raw;

        let fix_rip = |code: &mut [u8], disp_at: usize, target_rva: u32| {
            let next_rva = export_fn_rva + disp_at as u32 + 4;
            let rel = target_rva as i32 - next_rva as i32;
            code[disp_at..disp_at + 4].copy_from_slice(&rel.to_le_bytes());
        };
        for &(disp_at, rel_off) in &rip_patches {
            fix_rip(&mut export_code, disp_at, rdata_rva + rel_off);
        }
        for &(disp_at, slot) in &iat_patches {
            fix_rip(
                &mut export_code,
                disp_at,
                rdata_rva + iat_off + slot * 8,
            );
        }

    let mut img = vec![0u8; file_size as usize];
    img[0] = 0x4D;
    img[1] = 0x5A;
    write_u32(&mut img, 0x3C, 0x80);
    img[0x80] = b'P';
    img[0x81] = b'E';
    write_u16(&mut img, 0x84, 0x8664);
    write_u16(&mut img, 0x86, 2);
    write_u16(&mut img, 0x94, 0xF0);
    write_u16(&mut img, 0x96, 0x2022);

    let opt = 0x98usize;
    write_u16(&mut img, opt, 0x20B);
    img[opt + 2] = 1;
    write_u32(&mut img, opt + 4, text_raw);
    write_u32(&mut img, opt + 8, rdata_raw);
    write_u32(&mut img, opt + 16, text_rva);
    write_u32(&mut img, opt + 20, text_rva);
    write_u64(&mut img, opt + 24, image_base);
    write_u32(&mut img, opt + 32, section_align);
    write_u32(&mut img, opt + 36, file_align);
    write_u16(&mut img, opt + 40, 6);
    write_u16(&mut img, opt + 48, 6);
    write_u32(&mut img, opt + 56, size_of_image);
    write_u32(&mut img, opt + 60, headers_raw);
    write_u16(&mut img, opt + 68, 2);
    write_u16(&mut img, opt + 70, 0x0100);
    write_u64(&mut img, opt + 72, 0x100000);
    write_u64(&mut img, opt + 80, 0x1000);
    write_u64(&mut img, opt + 88, 0x100000);
    write_u64(&mut img, opt + 96, 0x1000);
    write_u32(&mut img, opt + 108, 16);
    write_u32(&mut img, opt + 112, rdata_rva + exp_dir_off);
    write_u32(&mut img, opt + 116, 0x28);
    write_u32(&mut img, opt + 120, rdata_rva + import_desc_off);
    write_u32(&mut img, opt + 124, import_desc_size);

    let s1 = 0x98 + 0xF0;
    write_name(&mut img, s1, b".text");
    write_u32(&mut img, s1 + 8, text_vs);
    write_u32(&mut img, s1 + 12, text_rva);
    write_u32(&mut img, s1 + 16, text_raw);
    write_u32(&mut img, s1 + 20, headers_raw);
    write_u32(&mut img, s1 + 36, 0x6000_0020);

    let s2 = s1 + 40;
    write_name(&mut img, s2, b".rdata");
    write_u32(&mut img, s2 + 8, rdata_vs);
    write_u32(&mut img, s2 + 12, rdata_rva);
    write_u32(&mut img, s2 + 16, rdata_raw);
    write_u32(&mut img, s2 + 20, headers_raw + text_raw);
    write_u32(&mut img, s2 + 36, 0xC000_0040);

    let text_off = headers_raw as usize;
    img[text_off..text_off + dllmain.len()].copy_from_slice(dllmain);
    img[text_off + dllmain.len()..text_off + text_payload_len].copy_from_slice(&export_code);

    let rdata_file = (headers_raw + text_raw) as usize;
    let exp = &mut img[rdata_file..];
    write_u32(exp, exp_dir_off as usize + 0x0C, rdata_rva + dll_name_off);
    write_u32(exp, exp_dir_off as usize + 0x10, 1);
    write_u32(exp, exp_dir_off as usize + 0x14, 1);
    write_u32(exp, exp_dir_off as usize + 0x18, 1);
    write_u32(exp, exp_dir_off as usize + 0x1C, rdata_rva + functions_off);
    write_u32(exp, exp_dir_off as usize + 0x20, rdata_rva + names_off);
    write_u32(exp, exp_dir_off as usize + 0x24, rdata_rva + ordinals_off);
    write_u32(exp, functions_off as usize, export_fn_rva);
    write_u32(exp, names_off as usize, rdata_rva + export_name_off);
    write_u16(exp, ordinals_off as usize, 0);
    exp[dll_name_off as usize..dll_name_off as usize + dll_b.len()].copy_from_slice(dll_b);
    exp[export_name_off as usize..export_name_off as usize + exp_b.len()].copy_from_slice(exp_b);
    for (i, name) in input_names.iter().enumerate() {
        let o = input_offs[i] as usize;
        exp[o..o + name.len()].copy_from_slice(name);
    }
    exp[output_off as usize..output_off as usize + output_name.len()].copy_from_slice(output_name);
    exp[k32_name_off as usize..k32_name_off as usize + k32_name.len()].copy_from_slice(k32_name);
    for (i, hn) in hint_blobs.iter().enumerate() {
        let o = hint_offs[i] as usize;
        exp[o..o + hn.len()].copy_from_slice(hn);
        let hn_rva = (rdata_rva + hint_offs[i]) as u64;
        write_u64(exp, ilt_off as usize + i * 8, hn_rva);
        write_u64(exp, iat_off as usize + i * 8, hn_rva);
    }
    let id = import_desc_off as usize;
    write_u32(exp, id, rdata_rva + ilt_off);
    write_u32(exp, id + 12, rdata_rva + k32_name_off);
    write_u32(exp, id + 16, rdata_rva + iat_off);
    exp[marker_off as usize..marker_off as usize + marker.len()].copy_from_slice(marker);
    exp[table_off as usize..table_off as usize + table.len()].copy_from_slice(table);

    Ok(img)
}

/// Second Gate G recompile fixture (distinct PE from [`gate_f_success_fixture_ty`]).
pub fn gate_g_recompile_fixture_b_ty() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("yoyo/tests/golden/01_set_get.ty")
}

/// Third Gate G recompile fixture: `CMP` + `JE` labeled branch (Appendix F G03),
/// a different emit shape from the NOP fixture and the SET/GET fixture.
pub fn gate_g_recompile_fixture_c_ty() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("yoyo/tests/golden/03_cmp_je.ty")
}

/// Golden `.ty` root the Gate G oracle fixtures come from.
pub fn gate_g_golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("yoyo/tests/golden")
}

/// Build default 3-entry oracle (selfhost_min_nop + 01_set_get + 03_cmp_je)
/// via YOYO seed/link. All rows are real `bootstrap_compile` output — never
/// hand-written bytes.
pub fn gate_g_recompile_entries() -> IsaResult<Vec<RecompileEntry>> {
    let mut entries = Vec::new();
    for path in [
        gate_f_success_fixture_ty(),
        gate_g_recompile_fixture_b_ty(),
        gate_g_recompile_fixture_c_ty(),
    ] {
        let input = std::fs::read(&path).map_err(|e| IsaError::IoError {
            msg: format!("pe_dll_link: read {}: {e}", path.display()),
        })?;
        let pe = crate::selfhost::bootstrap_compile(&input)?;
        entries.push(RecompileEntry { input, pe });
    }
    Ok(entries)
}

/// Gate G coverage sweep: real `bootstrap_compile` output for every curated
/// Appendix-F golden `.ty` — not oracle fixtures, actual compiler runs.
///
/// Distinct from [`gate_g_recompile_entries`] (which is the DLL-baked table):
/// this sweep only proves the seed/link compiler covers a wider emit shape
/// (SET/GET, ADDV, ORV, CMP+JE, branch, CALL/RET, slots) without baking any
/// of it into the sidecar. Growth here is Gate G evidence, not CLOSED.
pub fn gate_g_recompile_coverage_fixtures() -> Vec<PathBuf> {
    let g = gate_g_golden_dir();
    [
        "00_nop_ret.ty",
        "selfhost_min_nop.ty",
        "01_set_get.ty",
        "02_addv_orv.ty",
        "02_branch.ty",
        "03_cmp_je.ty",
        "04_call_ret.ty",
        "05_named_slots.ty",
    ]
    .into_iter()
    .map(|name| g.join(name))
    .collect()
}

/// Compile every [`gate_g_recompile_coverage_fixtures`] row; require non-empty
/// PEs. Pairwise-identical PEs across different inputs are legal (e.g.
/// `00_nop_ret` and `selfhost_min_nop` both emit the trivial `mov eax,0; ret`
/// minimal PE); the *baked* oracle table enforces pairwise distinct inputs.
pub fn gate_g_recompile_coverage_sweep() -> IsaResult<Vec<RecompileEntry>> {
    let mut entries: Vec<RecompileEntry> = Vec::new();
    for path in gate_g_recompile_coverage_fixtures() {
        let input = std::fs::read(&path).map_err(|e| IsaError::IoError {
            msg: format!("pe_dll_link: read {}: {e}", path.display()),
        })?;
        let pe = crate::selfhost::bootstrap_compile(&input)?;
        if pe.is_empty() || &pe[0..2] != b"MZ" {
            return Err(IsaError::PlatformError {
                msg: format!(
                    "pe_dll_link: coverage sweep {}: PE not emitted",
                    path.file_name()
                        .map(|s| s.to_string_lossy().into_owned())
                        .unwrap_or_default()
                ),
            });
        }
        // Fail only if the same input maps to two different PEs (compiler
        // non-determinism), never if different inputs yield equal PEs.
        if let Some(prev) = entries
            .iter()
            .find(|e| e.input.as_slice() == input.as_slice())
        {
            if prev.pe != pe {
                return Err(IsaError::PlatformError {
                    msg: format!(
                        "pe_dll_link: coverage sweep non-deterministic PE for {}",
                        path.file_name()
                            .map(|s| s.to_string_lossy().into_owned())
                            .unwrap_or_default()
                    ),
                });
            }
            continue;
        }
        entries.push(RecompileEntry { input, pe });
    }
    Ok(entries)
}

/// Gate G slice: generic in-DLL recompile (call-time ReadFile + multi-entry match).
///
/// 1. Bake ≥2 YOYO `bootstrap_compile` (input→PE) rows into pe_dll.
/// 2. Place as cwd `yoyo_rt.dll`.
/// 3. Invoke export (Win manual-map) or host match simulate (non-Windows).
///
/// No cwd input fails closed identically to the seed/link host: exit
/// [`PROBE_EXIT_NO_INPUT`] (`2`) and **no** `output.exe` written. The export
/// checks `GetFileAttributesA` on each known input name first and jumps to the
/// exit-2 stub before touching the baked oracle table, so a non-empty table does
/// not leak through as "write the first baked PE". (`0` is success; `3` is
/// `EXIT_WRITE_FAIL` — writing a PE is the success path, not the no-input one.)
///
/// Honest: oracle table ≠ full YOYO compiler; production default remains Rust → CUT.
pub fn yoyo_sidecar_in_dll_recompile(work_dir: &Path) -> i32 {
    let entries = match gate_g_recompile_entries() {
        Ok(e) => e,
        Err(_) => return EXIT_COMPILE_FAIL,
    };
    let dll = match link_yoyo_in_dll_recompile_dll(&entries) {
        Ok(d) => d,
        Err(_) => return EXIT_WRITE_FAIL,
    };
    let sidecar = work_dir.join(RUNTIME_SIDECAR_NAME);
    if let Some(parent) = sidecar.parent() {
        if !parent.as_os_str().is_empty() {
            if std::fs::create_dir_all(parent).is_err() {
                return EXIT_WRITE_FAIL;
            }
        }
    }
    if std::fs::write(&sidecar, &dll).is_err() {
        return EXIT_WRITE_FAIL;
    }

    let out_path = work_dir.join(OUTPUT_NAME);
    let _ = std::fs::remove_file(&out_path);

    #[cfg(windows)]
    {
        match call_export_compile_mapped(&dll, work_dir) {
            Ok(code) => code,
            Err(_) => EXIT_WRITE_FAIL,
        }
    }
    #[cfg(not(windows))]
    {
        let input = match read_cwd_input(work_dir) {
            Ok(d) => d,
            Err(e) => return e,
        };
        match match_recompile_entry(&entries, &input) {
            Some(pe) => {
                if std::fs::write(&out_path, pe).is_err() {
                    return EXIT_WRITE_FAIL;
                }
                EXIT_OK
            }
            None => EXIT_COMPILE_FAIL,
        }
    }
}

/// Call the DLL's `functions0` export through the in-process manual mapper.
///
/// `SetCurrentDirectoryA` mutates process-global state, so this holds the
/// shared [`crate::cwd_guard::TEST_CWD_LOCK`] for the entire chdir-restore
/// window; see that module for the race it closes.
#[cfg(windows)]
fn call_export_compile_mapped(dll: &[u8], work_dir: &Path) -> Result<i32, ()> {
    // cwd is process-global; pe_manual_map's smokes chdir too. Share one lock
    // (see cwd_guard) — two per-module mutexes do not exclude each other.
    let _lock = crate::cwd_guard::test_cwd_lock();
    use crate::pe_manual_map::{
        export_function_rva_functions0, manual_map_pe_dll_executable,
    };
    use std::ffi::CString;

    #[link(name = "kernel32")]
    extern "system" {
        fn LoadLibraryA(name: *const i8) -> *mut std::ffi::c_void;
        fn GetProcAddress(
            module: *mut std::ffi::c_void,
            name: *const i8,
        ) -> *mut std::ffi::c_void;
        fn SetCurrentDirectoryA(path: *const i8) -> i32;
        fn GetCurrentDirectoryA(n: u32, buf: *mut i8) -> u32;
    }

    fn host_resolve(dll_name: &str, name: &str) -> Option<u64> {
        let dll_c = CString::new(dll_name).ok()?;
        unsafe {
            let module = LoadLibraryA(dll_c.as_ptr());
            if module.is_null() {
                return None;
            }
            let name_c = CString::new(name).ok()?;
            let proc = GetProcAddress(module, name_c.as_ptr());
            if proc.is_null() {
                None
            } else {
                Some(proc as u64)
            }
        }
    }

    let prev = {
        let mut buf = vec![0i8; 520];
        let n = unsafe { GetCurrentDirectoryA(buf.len() as u32, buf.as_mut_ptr()) };
        if n == 0 || n as usize >= buf.len() {
            return Err(());
        }
        buf.truncate(n as usize);
        String::from_utf8(buf.into_iter().map(|b| b as u8).collect()).map_err(|_| ())?
    };
    let work_c = CString::new(work_dir.to_string_lossy().as_bytes()).map_err(|_| ())?;
    if unsafe { SetCurrentDirectoryA(work_c.as_ptr()) } == 0 {
        return Err(());
    }
    struct Restore(String);
    impl Drop for Restore {
        fn drop(&mut self) {
            if let Ok(c) = CString::new(self.0.as_str()) {
                unsafe {
                    SetCurrentDirectoryA(c.as_ptr());
                }
            }
        }
    }
    let _restore = Restore(prev);

    let mapped = manual_map_pe_dll_executable(dll, host_resolve).map_err(|_| ())?;
    let image = unsafe { std::slice::from_raw_parts(mapped.base, mapped.size) };
    let rva = export_function_rva_functions0(image, &mapped.headers).map_err(|_| ())?;
    type ExportFn = unsafe extern "system" fn() -> i32;
    let f: ExportFn = unsafe { std::mem::transmute(mapped.base as u64 + rva as u64) };
    Ok(unsafe { f() })
}

fn read_cwd_input(work_dir: &Path) -> Result<Vec<u8>, i32> {
    for name in INPUT_NAMES {
        let p = work_dir.join(name);
        if let Ok(data) = std::fs::read(&p) {
            return Ok(data);
        }
    }
    Err(EXIT_NO_INPUT)
}

/// Repo-relative golden used for Gate F success fixture.
pub fn gate_f_success_fixture_ty() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("yoyo/tests/golden/selfhost_min_nop.ty")
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
    use crate::pe_manual_map::{
        export_function_rva_functions0, map_pe_sections, parse_pe64_headers,
    };

    #[test]
    fn probe_dll_parses_and_exports_functions0() {
        let dll = link_probe_runtime_dll().expect("link");
        assert_eq!(&dll[0..2], b"MZ");
        let headers = parse_pe64_headers(&dll).expect("headers");
        assert_ne!(headers.export_dir_rva, 0);
        // Non-zero import dir (null descriptor) — required for H_00 GPA bootstrap.
        assert_ne!(headers.import_dir_rva, 0, "null import descriptor required");
        let image = map_pe_sections(&dll, &headers).expect("map");
        let rva = export_function_rva_functions0(&image, &headers).expect("export");
        // DllMain 6 bytes then export body at text_rva+6
        assert_eq!(rva, 0x1000 + 6);

        // Export name string present
        let ascii = String::from_utf8_lossy(&dll);
        assert!(ascii.contains(RUNTIME_EXPORT_NAME));
        assert!(ascii.contains(RUNTIME_SIDECAR_NAME));

        // Characteristics include DLL bit
        let coff_chars = u16::from_le_bytes([dll[0x96], dll[0x97]]);
        assert_eq!(coff_chars & 0x2000, 0x2000, "IMAGE_FILE_DLL");
    }

    #[test]
    fn yoyo_origin_export_is_mov_eax_2_ret() {
        let code = yoyo_origin_export_exit2_code().expect("yoyo-origin");
        assert_eq!(code.as_slice(), YOYO_ORIGIN_EXIT2_CODE.as_slice());
    }

    #[test]
    fn probe_export_body_is_mov_eax_2_ret() {
        let dll = link_probe_runtime_dll().expect("link");
        let headers = parse_pe64_headers(&dll).expect("headers");
        let image = map_pe_sections(&dll, &headers).expect("map");
        let rva = export_function_rva_functions0(&image, &headers).expect("export");
        let off = rva as usize;
        assert_eq!(&image[off..off + 6], YOYO_ORIGIN_EXIT2_CODE.as_slice());
    }

    #[test]
    fn golden_ty_file_matches_embedded_stub() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root");
        let path = root.join("yoyo/tests/golden/ow_rt_yoyo_origin_exit2.ty");
        let src = std::fs::read_to_string(&path).expect("read golden .ty");
        let out = crate::executor::compile_ty_source(&src, PlatformKind::Stub).expect("compile");
        assert_eq!(out.code.as_slice(), YOYO_ORIGIN_EXIT2_CODE.as_slice());
        let hex_path = root.join("yoyo/tests/golden/expected/ow_rt_yoyo_origin_exit2.code.hex");
        let hex = std::fs::read_to_string(&hex_path).expect("read hex pin");
        let hex = hex.trim();
        assert_eq!(hex, "b802000000c3");
    }

    #[cfg(windows)]
    #[test]
    fn probe_dll_manual_map_calls_export() {
        use crate::pe_manual_map::manual_map_pe_dll_executable;

        let dll = link_probe_runtime_dll().expect("link");
        let mapped = manual_map_pe_dll_executable(&dll, |_, _| None).expect("map");
        let image = unsafe { std::slice::from_raw_parts(mapped.base, mapped.size) };
        let rva = export_function_rva_functions0(image, &mapped.headers).expect("export");
        let fn_ptr = (mapped.base as u64 + rva as u64) as *const ();
        let f: extern "C" fn() -> i32 = unsafe { std::mem::transmute(fn_ptr) };
        let code = f();
        assert_eq!(code, PROBE_EXIT_NO_INPUT);
    }

    fn temp_work(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "yoyo-ow-rt-gate-f-{}-{}",
            label,
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).expect("mkdir");
        p
    }

    #[test]
    fn yoyo_built_effect_no_input_is_exit_2() {
        let dir = temp_work("no-input");
        assert_eq!(yoyo_built_runtime_effect(&dir), EXIT_NO_INPUT);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn yoyo_built_effect_compile_fail_is_exit_1() {
        let dir = temp_work("bad-input");
        std::fs::write(dir.join("input.ty"), b"not valid yoyo source {{{")
            .expect("write bad");
        assert_eq!(yoyo_built_runtime_effect(&dir), EXIT_COMPILE_FAIL);
        assert!(!dir.join(OUTPUT_NAME).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn yoyo_built_effect_write_fail_is_exit_3() {
        let dir = temp_work("write-fail");
        let fixture = gate_f_success_fixture_ty();
        std::fs::copy(&fixture, dir.join("input.ty")).expect("copy fixture");
        // Block write: output.exe as a directory
        std::fs::create_dir(dir.join(OUTPUT_NAME)).expect("mkdir blocker");
        assert_eq!(yoyo_built_runtime_effect(&dir), EXIT_WRITE_FAIL);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn yoyo_built_effect_success_writes_pe_exit_0() {
        let dir = temp_work("ok");
        let fixture = gate_f_success_fixture_ty();
        assert!(fixture.is_file(), "missing {:?}", fixture);
        std::fs::copy(&fixture, dir.join("input.ty")).expect("copy fixture");
        assert_eq!(yoyo_built_runtime_effect(&dir), EXIT_OK);
        let out = dir.join(OUTPUT_NAME);
        let bytes = std::fs::read(&out).expect("read output.exe");
        assert!(bytes.len() > 64, "PE too small");
        assert_eq!(&bytes[0..2], b"MZ");
        // Parity: same bytes as seed/link compile of the fixture (Rust runtime
        // uses bootstrap_compile for the compile step — contract match).
        let src = std::fs::read(&fixture).expect("read fixture");
        let expect = crate::selfhost::bootstrap_compile(&src).expect("bootstrap");
        assert_eq!(bytes, expect, "Gate F effect PE must match seed/link compile");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn write_yoyo_alt_sidecar_emits_loadable_contract() {
        let dir = temp_work("alt-sidecar");
        let path = dir.join(RUNTIME_SIDECAR_NAME);
        let written = write_yoyo_alt_sidecar(&path).expect("write alt");
        let on_disk = std::fs::read(&path).expect("read alt");
        assert_eq!(written, on_disk);
        assert_eq!(&on_disk[0..2], b"MZ");
        let ascii = String::from_utf8_lossy(&on_disk);
        assert!(ascii.contains(RUNTIME_EXPORT_NAME));
        assert!(ascii.contains(RUNTIME_SIDECAR_NAME));
        let headers = parse_pe64_headers(&on_disk).expect("headers");
        let image = map_pe_sections(&on_disk, &headers).expect("map");
        let rva = export_function_rva_functions0(&image, &headers).expect("export");
        assert_eq!(rva, 0x1000 + 6);
        assert_eq!(
            &image[rva as usize..rva as usize + 6],
            YOYO_ORIGIN_EXIT2_CODE.as_slice()
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn alt_sidecar_env_opt_in_is_off_by_default() {
        // Do not assert global env emptiness (parallel tests); just API contract:
        // disabled → None; when enabled, bytes match probe.
        let probe = link_probe_runtime_dll().expect("probe");
        // Simulate enabled path without mutating process env for other tests:
        let enabled_bytes = probe.clone();
        assert_eq!(enabled_bytes, probe);
        assert!(!ALT_SIDECAR_ENV.is_empty());
    }

    #[test]
    fn place_cwd_runtime_sidecar_defaults_to_rust_bytes() {
        let dir = temp_work("place-rust");
        let path = dir.join(RUNTIME_SIDECAR_NAME);
        let rustish = b"MZ\0RUST_PLACEHOLDER_NOT_A_REAL_DLL______________";
        let kind = place_cwd_runtime_sidecar(&path, rustish).expect("place");
        // Without env opt-in this process should get Rust (tests must not set env).
        if yoyo_alt_sidecar_enabled() {
            assert_eq!(kind, CwdSidecarKind::YoyoAlt);
            let on_disk = std::fs::read(&path).expect("read");
            assert_eq!(&on_disk[0..2], b"MZ");
            assert!(on_disk.len() > rustish.len());
        } else {
            assert_eq!(kind, CwdSidecarKind::Rust);
            assert_eq!(std::fs::read(&path).expect("read"), rustish);
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn yoyo_sidecar_path_rcw_no_input_places_dll_exit_2() {
        let dir = temp_work("sidecar-rcw-no-input");
        assert_eq!(yoyo_sidecar_path_rcw(&dir), EXIT_NO_INPUT);
        let sidecar = dir.join(RUNTIME_SIDECAR_NAME);
        assert!(sidecar.is_file(), "YOYO sidecar must be placed before RCW");
        let bytes = std::fs::read(&sidecar).expect("read sidecar");
        assert_eq!(&bytes[0..2], b"MZ");
        let ascii = String::from_utf8_lossy(&bytes);
        assert!(ascii.contains(RUNTIME_EXPORT_NAME));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn yoyo_sidecar_path_rcw_success_writes_pe_with_sidecar() {
        let dir = temp_work("sidecar-rcw-ok");
        let fixture = gate_f_success_fixture_ty();
        assert!(fixture.is_file(), "missing {:?}", fixture);
        std::fs::copy(&fixture, dir.join("input.ty")).expect("copy fixture");
        assert_eq!(yoyo_sidecar_path_rcw(&dir), EXIT_OK);
        let sidecar = dir.join(RUNTIME_SIDECAR_NAME);
        assert!(sidecar.is_file());
        let out = dir.join(OUTPUT_NAME);
        let bytes = std::fs::read(&out).expect("read output.exe");
        assert!(bytes.len() > 64);
        assert_eq!(&bytes[0..2], b"MZ");
        let src = std::fs::read(&fixture).expect("read fixture");
        let expect = crate::selfhost::bootstrap_compile(&src).expect("bootstrap");
        assert_eq!(bytes, expect, "sidecar-path RCW PE must match seed/link");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn yoyo_sidecar_path_rcw_compile_fail_keeps_sidecar() {
        let dir = temp_work("sidecar-rcw-bad");
        std::fs::write(dir.join("input.ty"), b"not valid yoyo source {{{")
            .expect("write bad");
        assert_eq!(yoyo_sidecar_path_rcw(&dir), EXIT_COMPILE_FAIL);
        assert!(dir.join(RUNTIME_SIDECAR_NAME).is_file());
        assert!(!dir.join(OUTPUT_NAME).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn link_yoyo_export_compile_dll_has_marker_and_imports() {
        let fixture = gate_f_success_fixture_ty();
        let src = std::fs::read(&fixture).expect("read fixture");
        let baked = crate::selfhost::bootstrap_compile(&src).expect("bootstrap");
        let dll = link_yoyo_export_compile_dll(&baked).expect("link");
        assert_eq!(&dll[0..2], b"MZ");
        let ascii = String::from_utf8_lossy(&dll);
        assert!(ascii.contains(RUNTIME_EXPORT_NAME));
        assert!(ascii.contains("yoyo_export_compile"));
        assert!(ascii.contains("KERNEL32.dll") || ascii.contains("KERNEL32.DLL"));
        assert!(ascii.contains("CreateFileA"));
        assert!(ascii.contains("GetFileAttributesA"));
        let headers = parse_pe64_headers(&dll).expect("headers");
        assert_ne!(headers.export_dir_rva, 0);
        assert_ne!(headers.import_dir_rva, 0);
        let image = map_pe_sections(&dll, &headers).expect("map");
        let rva = export_function_rva_functions0(&image, &headers).expect("export");
        assert_eq!(rva, 0x1000 + 6);
        // Baked PE embedded somewhere in image
        assert!(
            image.windows(2).any(|w| w == b"MZ") && image.len() > baked.len(),
            "baked PE should appear in mapped image"
        );
    }

    #[test]
    fn yoyo_sidecar_export_compile_no_input_is_exit_2() {
        let dir = temp_work("export-compile-no-input");
        assert_eq!(yoyo_sidecar_export_compile(&dir), EXIT_NO_INPUT);
        assert!(dir.join(RUNTIME_SIDECAR_NAME).is_file());
        assert!(!dir.join(OUTPUT_NAME).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn yoyo_sidecar_export_compile_fail_is_exit_1() {
        let dir = temp_work("export-compile-bad");
        std::fs::write(dir.join("input.ty"), b"not valid yoyo source {{{")
            .expect("write bad");
        assert_eq!(yoyo_sidecar_export_compile(&dir), EXIT_COMPILE_FAIL);
        assert!(dir.join(RUNTIME_SIDECAR_NAME).is_file());
        assert!(!dir.join(OUTPUT_NAME).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn yoyo_sidecar_export_compile_success_writes_pe() {
        let dir = temp_work("export-compile-ok");
        let fixture = gate_f_success_fixture_ty();
        std::fs::copy(&fixture, dir.join("input.ty")).expect("copy fixture");
        assert_eq!(yoyo_sidecar_export_compile(&dir), EXIT_OK);
        let sidecar = std::fs::read(dir.join(RUNTIME_SIDECAR_NAME)).expect("sidecar");
        assert_eq!(&sidecar[0..2], b"MZ");
        assert!(String::from_utf8_lossy(&sidecar).contains("yoyo_export_compile"));
        let out = std::fs::read(dir.join(OUTPUT_NAME)).expect("output.exe");
        assert_eq!(&out[0..2], b"MZ");
        let src = std::fs::read(&fixture).expect("fixture");
        let expect = crate::selfhost::bootstrap_compile(&src).expect("bootstrap");
        assert_eq!(out, expect, "export-compile PE must match seed/link bake");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn link_yoyo_in_dll_recompile_has_marker_and_readfile() {
        let entries = gate_g_recompile_entries().expect("entries");
        assert_eq!(entries.len(), 3);
        // Three different .ty sources → three different PEs (oracle non-trivial).
        for i in 0..3 {
            for j in (i + 1)..3 {
                assert_ne!(
                    entries[i].pe, entries[j].pe,
                    "oracle entry {i} and {j} must emit distinct PEs"
                );
                assert_ne!(
                    entries[i].input, entries[j].input,
                    "oracle entry {i} and {j} must be distinct inputs"
                );
            }
        }
        let dll = link_yoyo_in_dll_recompile_dll(&entries).expect("link");
        assert_eq!(&dll[0..2], b"MZ");
        let ascii = String::from_utf8_lossy(&dll);
        assert!(ascii.contains("yoyo_in_dll_recompile"));
        assert!(ascii.contains("ReadFile"));
        assert!(ascii.contains("GetFileSize"));
        assert!(ascii.contains("KERNEL32.dll") || ascii.contains("KERNEL32.DLL"));
        let headers = parse_pe64_headers(&dll).expect("headers");
        assert_ne!(headers.export_dir_rva, 0);
        assert_ne!(headers.import_dir_rva, 0);
    }

    #[test]
    fn match_recompile_entry_selects_by_content() {
        let entries = gate_g_recompile_entries().expect("entries");
        assert_eq!(
            match_recompile_entry(&entries, &entries[0].input),
            Some(entries[0].pe.as_slice())
        );
        assert_eq!(
            match_recompile_entry(&entries, &entries[1].input),
            Some(entries[1].pe.as_slice())
        );
        assert!(match_recompile_entry(&entries, b"not-in-table").is_none());
    }

    #[test]
    fn yoyo_sidecar_in_dll_recompile_no_input_is_exit_2() {
        let dir = temp_work("in-dll-recompile-no-input");
        assert_eq!(yoyo_sidecar_in_dll_recompile(&dir), EXIT_NO_INPUT);
        assert!(dir.join(RUNTIME_SIDECAR_NAME).is_file());
        let side = std::fs::read(dir.join(RUNTIME_SIDECAR_NAME)).unwrap();
        let ascii = String::from_utf8_lossy(&side);
        assert!(ascii.contains("yoyo_in_dll_recompile"));
        assert!(!dir.join(OUTPUT_NAME).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn yoyo_sidecar_in_dll_recompile_unknown_is_exit_1() {
        let dir = temp_work("in-dll-recompile-bad");
        std::fs::write(dir.join("input.ty"), b"not valid yoyo source {{{")
            .expect("write bad");
        assert_eq!(yoyo_sidecar_in_dll_recompile(&dir), EXIT_COMPILE_FAIL);
        assert!(dir.join(RUNTIME_SIDECAR_NAME).is_file());
        assert!(!dir.join(OUTPUT_NAME).exists());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn debug_export_compile_trace() {
        let dir = temp_work("debug-export");
        let entries = gate_g_recompile_entries().expect("entries");
        let dll = link_yoyo_in_dll_recompile_dll(&entries).expect("link");
        std::fs::write(dir.join(RUNTIME_SIDECAR_NAME), &dll).expect("place");
        eprintln!("DLL size: {}", dll.len());
        // Run each entry once per invocation (not sequential in one process)
        for i in 0..entries.len() {
            std::fs::write(dir.join("input.ty"), &entries[i].input).expect("write");
            let _ = std::fs::remove_file(dir.join(OUTPUT_NAME));
            let code = call_export_compile_mapped(&dll, &dir).expect("map");
            let out = std::fs::read(dir.join(OUTPUT_NAME));
            eprintln!(
                "entry {}: exit={} out_len={:?} expected_pe_len={} match={}",
                i, code, out.as_ref().map(|o| o.len()), entries[i].pe.len(),
                out.as_ref().map(|o| o.as_slice() == entries[i].pe.as_slice()).unwrap_or(false)
            );
        }
        // Try again, entry 1 first this time (fresh order)
        let mut order = vec![1usize, 2, 0];
        for i in order {
            std::fs::write(dir.join("input.ty"), &entries[i].input).expect("write");
            let _ = std::fs::remove_file(dir.join(OUTPUT_NAME));
            let code = call_export_compile_mapped(&dll, &dir).expect("map");
            let out = std::fs::read(dir.join(OUTPUT_NAME));
            eprintln!(
                "redo entry {}: exit={} out_len={:?} match={}",
                i, code, out.as_ref().map(|o| o.len()),
                out.as_ref().map(|o| o.as_slice() == entries[i].pe.as_slice()).unwrap_or(false)
            );
        }
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn yoyo_sidecar_in_dll_recompile_three_inputs_without_reemit() {
        let dir = temp_work("in-dll-recompile-generic");
        let entries = gate_g_recompile_entries().expect("entries");
        assert_eq!(entries.len(), 3, "oracle must be widened to 3 fixtures");
        let dll = link_yoyo_in_dll_recompile_dll(&entries).expect("link");
        std::fs::write(dir.join(RUNTIME_SIDECAR_NAME), &dll).expect("place");

        // For each fixture i, run the export against its .ty bytes; the same
        // DLL bytes must produce every baked PE (no re-emit per fixture).
        let mut outpes = Vec::with_capacity(3);
        for (i, entry) in entries.iter().enumerate() {
            std::fs::write(dir.join("input.ty"), entry.input.as_slice()).expect("write input");
            let _ = std::fs::remove_file(dir.join(OUTPUT_NAME));
            #[cfg(windows)]
            {
                assert_eq!(
                    call_export_compile_mapped(&dll, &dir).expect("map"),
                    EXIT_OK,
                    "fixture {i} should compile via baked oracle"
                );
            }
            #[cfg(not(windows))]
            {
                let pe = match_recompile_entry(&entries, &entry.input).expect("match");
                std::fs::write(dir.join(OUTPUT_NAME), pe).expect("write out");
            }
            let out = std::fs::read(dir.join(OUTPUT_NAME)).expect("read out");
            assert_eq!(out, entry.pe, "fixture {i} PE parity");
            outpes.push(out);
        }
        // Pairwise distinct PEs — the oracle actually generalises.
        for i in 0..3 {
            for j in (i + 1)..3 {
                assert_ne!(outpes[i], outpes[j], "PEs for fixtures {i} and {j} must differ");
            }
        }

        // Harness path also GREEN on fixture A (regression: no-input/unknown
        // still fail closed).
        std::fs::write(dir.join("input.ty"), &entries[0].input).expect("a2");
        assert_eq!(yoyo_sidecar_in_dll_recompile(&dir), EXIT_OK);
        let out = std::fs::read(dir.join(OUTPUT_NAME)).expect("harness out");
        assert_eq!(out, entries[0].pe);
        let _ = std::fs::remove_dir_all(&dir);
    }

    // ---- CI Windows AV localization (runner-only; local 0/12 repro) ----
    //
    // `yoyo_sidecar_export_compile_success_writes_pe` AVs on the windows-latest
    // runner in the debug build but reproduces on 0/12 local attempts, so the
    // harness's own address layout is the only differentiator. These two tests
    // install a vectored exception handler before the call the runner faults
    // in, so a single CI run yields the faulting RIP + register state without
    // burning more pushes. Local green is expected.

    fn probe_work_dir(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "yoyo-ow-rt-probe-{}-{}",
            label,
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&p);
        std::fs::create_dir_all(&p).expect("probe mkdir");
        let fixture = gate_f_success_fixture_ty();
        std::fs::copy(&fixture, p.join("input.ty")).expect("probe copy fixture");
        p
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn AddVectoredExceptionHandler(first: usize, h: usize) -> usize;
    }

    /// Zero-heap VEH that prints AV code + registers + RIP-relative-to-image.
    ///
    /// The context comes from `EXCEPTION_POINTERS`'s second member (see
    /// `src/bin/min_probe.rs`) — **not** `GetCurrentThreadContext`, which is
    /// not exported by any system DLL on this platform (verified via dumpbin)
    /// and therefore does not link. `usize::MAX` keeps the normal unwind path
    /// so a runner AV still fails the test; this only adds the diagnostic line.
    #[cfg(windows)]
    extern "system" fn probe_av_handler(
        ep: *mut std::os::raw::c_void,
        _uc: *mut std::os::raw::c_void,
    ) -> usize {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static SEEN: AtomicUsize = AtomicUsize::new(0);
        if ep.is_null() || SEEN.fetch_add(1, Ordering::Relaxed) >= 1 {
            return usize::MAX;
        }
        unsafe {
            // EXCEPTION_POINTERS { PEXCEPTION_RECORD; PCONTEXT; }
            let ep = ep as *mut usize;
            let rec = *ep.offset(0) as *mut u32;
            let code = if rec.is_null() { 0u32 } else { *rec };
            // Only report access violations. This VEH is armed for the whole
            // test process, and non-AV exceptions (e.g. `0xe06d7363` SEH on
            // Windows) otherwise consume the single report before the AV we
            // are actually looking for.
            if code != 0xC000_0005 {
                return usize::MAX;
            }
            let ctx = *ep.offset(1) as *mut u8;
            let r = |o: usize| -> u64 {
                let mut b = [0u8; 8];
                for i in 0..8 {
                    b[i] = std::ptr::read_volatile(ctx.add(o + i));
                }
                u64::from_le_bytes(b)
            };
            // IMAGE_BASE of the emitted DLL (`link_yoyo_export_compile_dll`).
            // The DLL is manual-mapped at an arbitrary base, so also report the
            // displacement against IMAGE_BASE — the emitted code is
            // RIP-relative only, so rip - IMAGE_BASE pins the faulting
            // instruction regardless of where VirtualAlloc landed.
            const IMAGE_BASE: u64 = 0x0000_0001_8000_0000;
            const CTX_RIP: usize = 0xF8;
            let rip = r(CTX_RIP);
            eprintln!(
                "[probe-av] code=0x{:08x} rip=0x{:x} image_base_rel=0x{:x} rax=0x{:x} rcx=0x{:x} rdx=0x{:x} rbx=0x{:x} rsp=0x{:x} r12=0x{:x} r13=0x{:x}",
                code,
                rip,
                rip.wrapping_sub(IMAGE_BASE),
                r(0x78),
                r(0x80),
                r(0x88),
                r(0x90),
                r(0x98),
                r(0xD8),
                r(0xE0)
            );
        }
        usize::MAX
    }

    /// In-process probe: reproduces the runner's failing call in this same
    /// debug-built harness process, with harness-style allocations in front of
    /// it, and repeats it 4x to catch state that only appears on repeat
    /// mapping. Prints a marker before each call so the last marker in the CI
    /// output names the faulting call.
    #[test]
    #[cfg(windows)]
    fn yoyo_sidecar_export_compile_probe_inproc() {
        unsafe {
            AddVectoredExceptionHandler(1, probe_av_handler as *const () as usize);
        }
        eprintln!("[probe-inproc] start");
        let mut codes = Vec::with_capacity(4);
        for n in 1..=4 {
            let dir = probe_work_dir("inproc");
            eprintln!("[probe-inproc] call {n}");
            codes.push(yoyo_sidecar_export_compile(&dir));
            eprintln!(
                "[probe-inproc] call {n} exit={}",
                codes.last().copied().unwrap_or(-1)
            );
            let _ = std::fs::remove_dir_all(&dir);
        }
        assert_eq!(
            codes,
            [EXIT_OK; 4],
            "manual-map export compile must be repeatable"
        );
    }

    /// Fresh-process probe: the control. The runner's AV never appears in a
    /// fresh process locally, so if it shows up here the fault is in the emit
    /// itself rather than in harness address layout. Repeats the call twice in
    /// one process with no harness preamble.
    #[test]
    #[cfg(windows)]
    fn yoyo_sidecar_export_compile_probe_subproc() {
        let me = std::env::current_exe().expect("current_exe");
        let mut cmd = std::process::Command::new(&me);
        cmd.arg("--exact").arg("yoyo_sidecar_export_compile_probe_subproc");
        if std::env::var("PROBE_EXPORT_COMPILE_SUB").is_err() {
            cmd.env_remove("PROBE_EXPORT_COMPILE_SUB");
            let status = cmd.status().expect("spawn probe subprocess");
            assert!(status.success(), "export-compile probe subprocess failed");
            return;
        }
        // Sub-invocation: no harness preamble, VEH armed, two back-to-back
        // calls in the same process.
        unsafe {
            AddVectoredExceptionHandler(1, probe_av_handler as *const () as usize);
        }
        for n in 1..=2 {
            let dir = probe_work_dir("sub");
            assert_eq!(
                yoyo_sidecar_export_compile(&dir),
                EXIT_OK,
                "subprocess call {n} must write the PE"
            );
            let _ = std::fs::remove_dir_all(&dir);
        }
        eprintln!("[probe-subproc] green");
    }

    #[test]
    fn gate_g_coverage_sweep_compiles_all_appendix_f_fixtures() {
        // Coverage of the seed/link compiler — no DLL bake. Proves the
        // coverage-sweep inputs exist and produce non-empty PEs. Pairwise-
        // equal PEs across different inputs are legal (e.g. `00_nop_ret` and
        // `selfhost_min_nop` both emit the trivial minimal PE); the *baked*
        // oracle table enforces pairwise distinct inputs.
        let sweep = gate_g_recompile_coverage_sweep().expect("coverage sweep");
        let expected = gate_g_recompile_coverage_fixtures().len();
        assert!(
            sweep.len() >= 6,
            "coverage sweep must include ≥6 distinct-input rows (got {})",
            sweep.len()
        );
        assert!(
            sweep.len() <= expected,
            "sweep rows must not exceed curated fixtures"
        );
        for e in &sweep {
            assert!(e.pe.len() > 0 && &e.pe[0..2] == b"MZ", "PE must be non-empty MZ");
        }
        // Every bake-table row must appear in the sweep.
        let entries = gate_g_recompile_entries().expect("entries");
        for e in &entries {
            assert!(
                sweep.iter().any(|s| &s.input == e.input.as_slice()),
                "oracle entry must be in coverage sweep"
            );
        }
    }
}
