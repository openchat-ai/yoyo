//! Minimal PE reader.

pub fn is_pe(bytes: &[u8]) -> bool {
    if bytes.len() < 0x40 || &bytes[0..2] != b"MZ" {
        return false;
    }
    let lfanew = u32::from_le_bytes(bytes[0x3C..0x40].try_into().unwrap()) as usize;
    lfanew + 4 <= bytes.len() && &bytes[lfanew..lfanew + 4] == b"PE\0\0"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reject_empty() {
        assert!(!is_pe(&[]));
    }
}
