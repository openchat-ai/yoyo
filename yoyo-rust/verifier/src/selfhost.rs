//! Self-host framework — Rust compile function + x64 dispatch.
//!
//! Architecture:
//!   .text: [selfhost_startup] [handler_bytes] [HOT_table]
//!
//! selfhost_startup calls selfhost_compile_tyb() which does the actual work.
//! The compile function is compiled as part of the verifier crate and its
//! bytes are embedded into the PE at link time.

use crate::elf_link;
use crate::executor;
use crate::linux_selfhost;
use crate::pe_link;
use crate::platform::PlatformKind;
use crate::types::IsaResult;

/// Compile .tyb data to a PE binary using the standard emit pipeline.
/// This proves that .tyb → .exe produces the same result as .ty → .exe.
pub fn selfhost_compile_tyb(tyb_data: &[u8]) -> IsaResult<Vec<u8>> {
    let out = executor::compile_tyb_source(tyb_data, PlatformKind::Win32)?;
    let pe = pe_link::link_pe_win32(&out.code, &out.data, &out.handler_offsets)?;
    Ok(pe.bytes)
}

/// Stage 13-A fail-closed ceilings for the H_00 seed/link host image (keep in sync with
/// `scripts/stage13-link-host.ps1`). Observed @ Stage 11-B/v0.6: PE 248832 / ELF 512000.
/// Post-v1.0 Win: no exact-embed -> seed PE still <=270000 (obs. 248832; data floor 0x38000).
/// Post-v1.0 Linux: no exact-embed tramp/`.so` -> seed ELF <=300000 (obs. 253952; data floor; was ~512000).
/// Do not raise casually -- growth here is growth of the selfhost entry host surface.
pub const STAGE13_MAX_SEED_PE_BYTES: usize = 270_000;
pub const STAGE13_MAX_SEED_ELF_BYTES: usize = 300_000;

/// Stage 13-A canonical **seed/link host** compile (Win32).
///
/// Pure M4 / gen12 seed algebra must enter through this path (H_00 `link_pe_win32`),
/// shared by `yoyo link` and `yoyo bootstrap` **without** `--selfhost`.
/// `bootstrap --selfhost` (genNrt / GetTempPath) is a different host surface and must DIFF.
pub fn seed_host_compile(input: &[u8]) -> IsaResult<Vec<u8>> {
    if crate::tyb_parser::is_tyb(input) {
        selfhost_compile_tyb(input)
    } else {
        let src = std::str::from_utf8(input).map_err(|e| crate::types::IsaError::ParseError {
            line: 0,
            msg: format!("seed/link host input not valid UTF-8 .ty: {e}"),
        })?;
        let out = executor::compile_ty_source(src, PlatformKind::Win32)?;
        Ok(pe_link::link_pe_win32(&out.code, &out.data, &out.handler_offsets)?.bytes)
    }
}

/// Compile `.ty` text or `.tyb` binary to Win32 PE bytes.
/// Alias of [`seed_host_compile`] — Stage 5 M1→M2 interim / Stage 13-A seed path
/// (external compiler, not `bootstrap --selfhost` runtime wrapper).
pub fn bootstrap_compile(input: &[u8]) -> IsaResult<Vec<u8>> {
    seed_host_compile(input)
}

/// Link emitted code as Win32 PE with embedded runtime selfhost startup + HOT table.
pub fn link_pe_selfhost_runtime(
    code: &[u8],
    data: &[u8],
    hot_table: &[u8],
    embedded_dll: &[u8],
) -> IsaResult<Vec<u8>> {
    Ok(pe_link::link_pe_selfhost(code, data, hot_table, embedded_dll)?.bytes)
}

/// Prebuilt `yoyo_runtime.dll` bytes (embedded in PE .data at link time).
pub fn runtime_dll_bytes() -> IsaResult<Vec<u8>> {
    crate::win32_selfhost::runtime_dll_bytes()
}

/// Stage 5 M2→M3: compile `.tyb` input and link PE with embedded runtime selfhost startup.
pub fn bootstrap_selfhost_runtime(input_tyb: &[u8]) -> IsaResult<Vec<u8>> {
    let dll = runtime_dll_bytes()?;
    let out = executor::compile_tyb_source(input_tyb, PlatformKind::Win32)?;
    let hot = build_hot(&out.handler_offsets);
    link_pe_selfhost_runtime(&out.code, &out.data, &hot, &dll)
}

/// Compile `.tyb` to Linux ELF64 bytes (M1→M2 interim, Rust host compiler).
/// Stage 10-B / post-v1.0: full-body images use H_00 (cwd tramp + `.so` sidecars).
pub fn selfhost_compile_tyb_linux(tyb_data: &[u8]) -> IsaResult<Vec<u8>> {
    let out = executor::compile_tyb_source(tyb_data, PlatformKind::Linux)?;
    Ok(elf_link::link_elf_linux(&out.code, &out.data, &out.handler_offsets)?.bytes)
}

/// Stage 13-A canonical **seed/link host** compile (Linux ELF H_00).
/// Shared by `yoyo link --target=linux` and `yoyo bootstrap --target=linux` (no `--selfhost`).
pub fn seed_host_compile_linux(input: &[u8]) -> IsaResult<Vec<u8>> {
    if crate::tyb_parser::is_tyb(input) {
        selfhost_compile_tyb_linux(input)
    } else {
        let src = std::str::from_utf8(input).map_err(|e| crate::types::IsaError::ParseError {
            line: 0,
            msg: format!("seed/link host input not valid UTF-8 .ty: {e}"),
        })?;
        let out = executor::compile_ty_source(src, PlatformKind::Linux)?;
        Ok(elf_link::link_elf_linux(&out.code, &out.data, &out.handler_offsets)?.bytes)
    }
}

/// Compile `.ty` text or `.tyb` binary to Linux ELF64 bytes.
/// Alias of [`seed_host_compile_linux`] (Stage 13-A seed path).
pub fn bootstrap_compile_linux(input: &[u8]) -> IsaResult<Vec<u8>> {
    seed_host_compile_linux(input)
}

/// Link emitted code as Linux ELF with embedded runtime selfhost startup.
pub fn link_elf_selfhost_runtime(code: &[u8], data: &[u8], hot_table: &[u8]) -> IsaResult<Vec<u8>> {
    linux_selfhost::link_elf_selfhost_runtime(code, data, hot_table)
}

/// Stage 7 M2→M3 Linux: compile `.tyb` and link ELF with embedded startup.
pub fn bootstrap_selfhost_runtime_linux(input_tyb: &[u8]) -> IsaResult<Vec<u8>> {
    let out = executor::compile_tyb_source(input_tyb, PlatformKind::Linux)?;
    let hot = build_hot(&out.handler_offsets);
    link_elf_selfhost_runtime(&out.code, &out.data, &hot)
}

/// Prebuilt `libyoyo_runtime.so` bytes (for sidecar extraction at bootstrap).
pub fn runtime_so_bytes() -> IsaResult<Vec<u8>> {
    linux_selfhost::runtime_so_bytes()
}

/// Build the HOT table from handler offset data.
pub fn build_hot(handler_offsets: &[(u16, u32, u32)]) -> Vec<u8> {
    let mut hot = Vec::new();
    for (hh, off, len) in handler_offsets {
        hot.extend_from_slice(&hh.to_le_bytes());
        hot.extend_from_slice(&off.to_le_bytes());
        hot.extend_from_slice(&(*len as u16).to_le_bytes());
    }
    hot.extend_from_slice(&0xFFFFu16.to_le_bytes()); // terminator
    hot
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn selfhost_compile_tyb_matches_normal() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap().parent().unwrap()
            .join("yoyo/projects");
        let tyb_path = root.join("yoyo.tyb");
        let ty_path = root.join("yoyo.ty");

        let tyb_data = fs::read(&tyb_path).unwrap();
        let src = fs::read_to_string(&ty_path).unwrap();

        // Emit-layer DDC: .ty vs .tyb must produce identical code/data/handlers.
        // Full PE H_00 link (needs yoyo_runtime.dll) is covered by `yoyo test gen12`.
        let out_tyb =
            crate::executor::compile_tyb_source(&tyb_data, PlatformKind::Win32).unwrap();
        let out_ty = crate::executor::compile_ty_source(&src, PlatformKind::Win32).unwrap();

        assert_eq!(out_tyb.code, out_ty.code, "emit code must match (.ty vs .tyb)");
        assert_eq!(out_tyb.data, out_ty.data, "emit data must match (.ty vs .tyb)");
        assert_eq!(
            out_tyb.handler_offsets, out_ty.handler_offsets,
            "handler offsets must match (.ty vs .tyb)"
        );
    }
}