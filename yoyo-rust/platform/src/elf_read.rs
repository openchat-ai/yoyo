//! Minimal ELF reader.

pub fn is_elf(bytes: &[u8]) -> bool {
    bytes.len() >= 4 && bytes[0..4] == [0x7F, b'E', b'L', b'F']
}
