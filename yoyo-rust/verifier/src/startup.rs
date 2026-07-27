//! Startup blobs — hand-audited, separate from ISA (Decision #3).

/// Windows x64: shadow space only (R15 init is in pe_link startup).
pub fn startup_blob_windows() -> &'static [u8] {
    // sub rsp, 0x28
    static BLOB: [u8; 4] = [0x48, 0x83, 0xEC, 0x28];
    &BLOB
}

/// Bare-metal identity — full GDT/IDT/CR3 sequence lives in docs + Phase 5 asm.
/// This is a placeholder that simply returns (to be replaced by flat binary blob).
pub fn startup_blob_baremetal() -> &'static [u8] {
    static BLOB: [u8; 1] = [0xC3];
    &BLOB
}
