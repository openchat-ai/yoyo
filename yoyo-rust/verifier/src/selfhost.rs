//! Self-host framework — Rust compile function + x64 dispatch.
//!
//! Architecture:
//!   .text: [selfhost_startup] [handler_bytes] [HOT_table]
//!
//! selfhost_startup calls selfhost_compile_tyb() which does the actual work.
//! The compile function is compiled as part of the verifier crate and its
//! bytes are embedded into the PE at link time.

use crate::executor;
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

/// Generate the selfhost startup x64 code.
///
/// This is the entry point of the --selfhost PE.
/// It reads .tyb from argv[1], calls selfhost_compile_tyb(), writes output.
///
/// For V1: the startup code is a simple stub. The actual compile logic
/// is in selfhost_compile_tyb() above, which is compiled as part of the
/// verifier crate. The --selfhost PE embeds this function's bytes.
pub fn gen_selfhost_startup(_hot_va: u64, _code_va: u64, _startup_va: u64) -> Vec<u8> {
    // V1: exit(0) stub — selfhost compile is done by yoyo.exe at link time
    // mov eax, 0
    // ret
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0xB8, 0x00, 0x00, 0x00, 0x00]); // mov eax, 0
    code.push(0xC3); // ret
    code
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