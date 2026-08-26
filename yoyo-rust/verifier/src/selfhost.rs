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
    let pe = pe_link::link_pe(&out.code, &out.data)?;
    Ok(pe.bytes)
}

/// Compile `.ty` text or `.tyb` binary to Win32 PE bytes.
/// Used by `yoyo bootstrap` for Stage 5 M1→M2 interim (external compiler, not runtime selfhost).
pub fn bootstrap_compile(input: &[u8]) -> IsaResult<Vec<u8>> {
    if crate::tyb_parser::is_tyb(input) {
        selfhost_compile_tyb(input)
    } else {
        let src = std::str::from_utf8(input).map_err(|e| crate::types::IsaError::ParseError {
            line: 0,
            msg: format!("bootstrap input not valid UTF-8 .ty: {e}"),
        })?;
        let out = executor::compile_ty_source(src, PlatformKind::Win32)?;
        Ok(pe_link::link_pe(&out.code, &out.data)?.bytes)
    }
}

/// Link emitted code as Win32 PE with embedded runtime selfhost startup + HOT table.
pub fn link_pe_selfhost_runtime(code: &[u8], data: &[u8], hot_table: &[u8]) -> IsaResult<Vec<u8>> {
    Ok(pe_link::link_pe_selfhost(code, data, hot_table)?.bytes)
}

/// Prebuilt `yoyo_runtime.dll` bytes (for sidecar extraction at bootstrap).
pub fn runtime_dll_bytes() -> IsaResult<Vec<u8>> {
    crate::win32_selfhost::runtime_dll_bytes()
}

/// Stage 5 M2→M3: compile `.tyb` input and link PE with runtime selfhost startup.
pub fn bootstrap_selfhost_runtime(input_tyb: &[u8]) -> IsaResult<Vec<u8>> {
    let out = executor::compile_tyb_source(input_tyb, PlatformKind::Win32)?;
    let hot = build_hot(&out.handler_offsets);
    link_pe_selfhost_runtime(&out.code, &out.data, &hot)
}

/// Compile `.tyb` to Linux ELF64 bytes (M1→M2 interim, Rust host compiler).
pub fn selfhost_compile_tyb_linux(tyb_data: &[u8]) -> IsaResult<Vec<u8>> {
    let out = executor::compile_tyb_source(tyb_data, PlatformKind::Linux)?;
    Ok(elf_link::link_elf(&out.code, &out.data)?.bytes)
}

/// Compile `.ty` text or `.tyb` binary to Linux ELF64 bytes.
pub fn bootstrap_compile_linux(input: &[u8]) -> IsaResult<Vec<u8>> {
    if crate::tyb_parser::is_tyb(input) {
        selfhost_compile_tyb_linux(input)
    } else {
        let src = std::str::from_utf8(input).map_err(|e| crate::types::IsaError::ParseError {
            line: 0,
            msg: format!("bootstrap input not valid UTF-8 .ty: {e}"),
        })?;
        let out = executor::compile_ty_source(src, PlatformKind::Linux)?;
        Ok(elf_link::link_elf(&out.code, &out.data)?.bytes)
    }
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
        let pe_self = selfhost_compile_tyb(&tyb_data).unwrap();
        
        let src = fs::read_to_string(&ty_path).unwrap();
        let out = crate::executor::compile_ty_source(&src, crate::platform::PlatformKind::Win32).unwrap();
        let pe_normal = crate::pe_link::link_pe(&out.code, &out.data).unwrap();
        
        let text_self = crate::ddc::pe_text_section(&pe_self).unwrap();
        let text_normal = crate::ddc::pe_text_section(&pe_normal.bytes).unwrap();
        
        assert_eq!(text_self, text_normal, "selfhost compile must match normal compile");
    }
}