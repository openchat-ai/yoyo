//! 3-chain DDC helpers (PROMPT-v3 Part 6).

use sha2::{Digest, Sha256};

use crate::types::{IsaError, IsaResult};

#[derive(Debug, Clone)]
pub struct DdcReport {
    pub hash_a: String,
    pub hash_b: String,
    pub equal: bool,
    pub compared_bytes: usize,
}

/// PE link startup stub length (`lea r15` + `jmp` + nop) prepended before emit body.
pub const PE_STARTUP_LEN: usize = 13;
/// H_00 handler slot length in full-body emit (SET+RET or Stage 9-A JMP+NOPs).
pub const H00_SLOT_LEN: usize = 18;
/// Fail-closed floor for three-peer shared handler body (post-H_00).
pub const MIN_SHARED_HANDLER_BYTES: usize = 17000;

pub fn sha256_hex(data: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(data);
    hex::encode(h.finalize())
}

/// Compare two binaries for DDC (full-file for now; code-section-only later).
pub fn compare_bytes(a: &[u8], b: &[u8]) -> DdcReport {
    let hash_a = sha256_hex(a);
    let hash_b = sha256_hex(b);
    DdcReport {
        equal: a == b,
        compared_bytes: a.len().min(b.len()),
        hash_a,
        hash_b,
    }
}

/// Extract approximate .text from a PE for section-only DDC (Part 6.3).
pub fn pe_text_section(pe: &[u8]) -> IsaResult<Vec<u8>> {
    if pe.len() < 0x200 || &pe[0..2] != b"MZ" {
        return Ok(pe.to_vec());
    }
    let lfanew = u32::from_le_bytes(pe[0x3C..0x40].try_into().unwrap()) as usize;
    if lfanew + 0x180 > pe.len() {
        return Ok(pe.to_vec());
    }
    // First section header at optional_header start + SizeOfOptionalHeader
    // Our linker: COFF at lfanew+4, SizeOfOptionalHeader at lfanew+0x14
    let soh = u16::from_le_bytes(pe[lfanew + 0x14..lfanew + 0x16].try_into().unwrap()) as usize;
    let sec = lfanew + 0x18 + soh;
    if sec + 40 > pe.len() {
        return Ok(pe.to_vec());
    }
    let vs = u32::from_le_bytes(pe[sec + 8..sec + 12].try_into().unwrap()) as usize;
    let raw_sz = u32::from_le_bytes(pe[sec + 16..sec + 20].try_into().unwrap()) as usize;
    let raw_ptr = u32::from_le_bytes(pe[sec + 20..sec + 24].try_into().unwrap()) as usize;
    let n = vs.min(raw_sz).min(pe.len().saturating_sub(raw_ptr));
    Ok(pe[raw_ptr..raw_ptr + n].to_vec())
}

pub fn compare_pe_text(a: &[u8], b: &[u8]) -> IsaResult<DdcReport> {
    let ta = pe_text_section(a)?;
    let tb = pe_text_section(b)?;
    Ok(compare_bytes(&ta, &tb))
}

/// Emit-body length in `.text` after PE startup (excludes trailing pad / H_00 extract stub).
/// Patched H_00 (`E9` JMP): target offset is the original emit length. Canonical: strip zeros.
pub fn pe_emit_code_len(text: &[u8]) -> usize {
    if text.len() <= PE_STARTUP_LEN {
        return 0;
    }
    let user = &text[PE_STARTUP_LEN..];
    if user.len() >= 5 && user[0] == 0xE9 {
        let rel = i32::from_le_bytes(user[1..5].try_into().unwrap());
        let emit = (5i32.saturating_add(rel)) as usize;
        if emit >= H00_SLOT_LEN && emit <= user.len() {
            return emit;
        }
    }
    let mut end = user.len();
    while end > 0 && user[end - 1] == 0 {
        end -= 1;
    }
    end
}

/// Stage 12-B: peer-comparable selfhost body window.
///
/// Full `.text` section-ddc DIFF across Rust vs JS/asm because Rust patches H_00 and
/// appends the extract stub. This window keeps PE startup + handlers **after** the
/// H_00 slot through the shared emit body — enlarging three-peer observability past
/// the "whole `.text` DIFF ⇒ stop" blind spot, while leaving H_00 slot / stub / DLL
/// as honest remaining DIFF.
pub fn selfhost_body_window(text: &[u8], emit_code_bytes: usize) -> IsaResult<Vec<u8>> {
    let body_end = PE_STARTUP_LEN.saturating_add(emit_code_bytes);
    if text.len() < body_end {
        return Err(IsaError::PlatformError {
            msg: format!(
                "selfhost-body window: .text {}B < body_end {}B (emit={emit_code_bytes})",
                text.len(),
                body_end
            ),
        });
    }
    let shared_start = PE_STARTUP_LEN + H00_SLOT_LEN;
    if body_end < shared_start {
        return Err(IsaError::PlatformError {
            msg: "selfhost-body window: emit shorter than H_00 slot".into(),
        });
    }
    let shared_len = body_end - shared_start;
    if shared_len < MIN_SHARED_HANDLER_BYTES {
        return Err(IsaError::PlatformError {
            msg: format!(
                "selfhost-body window: shared handlers {shared_len}B < min {MIN_SHARED_HANDLER_BYTES}B"
            ),
        });
    }
    let mut w = Vec::with_capacity(PE_STARTUP_LEN + shared_len);
    w.extend_from_slice(&text[..PE_STARTUP_LEN]);
    w.extend_from_slice(&text[shared_start..body_end]);
    Ok(w)
}

/// Bytes after the shared emit body (Rust H_00 extract stub + pad; peers usually zeros).
pub fn h00_stub_tail(text: &[u8], emit_code_bytes: usize) -> Vec<u8> {
    let body_end = PE_STARTUP_LEN.saturating_add(emit_code_bytes);
    if text.len() <= body_end {
        return Vec::new();
    }
    text[body_end..].to_vec()
}

/// Compare two PEs on the Stage 12-B selfhost-body window.
/// `emit_code_bytes` is the shared emit length (typically `min(pe_emit_code_len(a), pe_emit_code_len(b))`
/// after stripping a Rust-only stub, or the peer emit size without stub).
pub fn compare_pe_selfhost_body(
    a: &[u8],
    b: &[u8],
    emit_code_bytes: usize,
) -> IsaResult<DdcReport> {
    let ta = pe_text_section(a)?;
    let tb = pe_text_section(b)?;
    let wa = selfhost_body_window(&ta, emit_code_bytes)?;
    let wb = selfhost_body_window(&tb, emit_code_bytes)?;
    Ok(compare_bytes(&wa, &wb))
}

/// Infer shared emit length for three-peer body compare: min of trimmed emit lens.
/// When one peer carries an H_00 stub, the stub-free peer's shorter length wins.
pub fn infer_shared_emit_code_len(text_a: &[u8], text_b: &[u8]) -> usize {
    pe_emit_code_len(text_a).min(pe_emit_code_len(text_b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha_deterministic() {
        assert_eq!(sha256_hex(b"abc"), sha256_hex(b"abc"));
        assert_ne!(sha256_hex(b"abc"), sha256_hex(b"abd"));
    }

    #[test]
    fn selfhost_body_window_skips_h00_slot() {
        let mut text = vec![0xAAu8; PE_STARTUP_LEN];
        text.extend(std::iter::repeat(0xBBu8).take(H00_SLOT_LEN));
        text.extend(std::iter::repeat(0xCCu8).take(MIN_SHARED_HANDLER_BYTES + 8));
        let emit = text.len() - PE_STARTUP_LEN;
        let w = selfhost_body_window(&text, emit).unwrap();
        assert_eq!(&w[..PE_STARTUP_LEN], &text[..PE_STARTUP_LEN]);
        assert!(!w[PE_STARTUP_LEN..].contains(&0xBBu8));
        assert!(w[PE_STARTUP_LEN..].iter().all(|&b| b == 0xCC));
        assert_eq!(w.len(), PE_STARTUP_LEN + MIN_SHARED_HANDLER_BYTES + 8);
    }
}
