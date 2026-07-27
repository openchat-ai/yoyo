//! 3-chain DDC helpers (PROMPT-v3 Part 6).

use sha2::{Digest, Sha256};

use crate::types::IsaResult;

#[derive(Debug, Clone)]
pub struct DdcReport {
    pub hash_a: String,
    pub hash_b: String,
    pub equal: bool,
    pub compared_bytes: usize,
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha_deterministic() {
        assert_eq!(sha256_hex(b"abc"), sha256_hex(b"abc"));
        assert_ne!(sha256_hex(b"abc"), sha256_hex(b"abd"));
    }
}
