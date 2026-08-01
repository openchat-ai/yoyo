//! Self-host framework — x64 dispatch for `--selfhost` PE.
//!
//! Architecture:
//!   .text: [selfhost_startup] [handler_bytes] [HOT_table]
//!
//! selfhost_startup is x64 machine code that:
//!   1. Reads .tyb from argv[1]
//!   2. For each 8B record: HOT lookup → copy handler bytes to output buffer
//!   3. Builds PE header around output buffer
//!   4. Writes output .exe
//!
//! "Copy, not call" — the handler bytes are pre-computed by yoyo.exe at link time.
//! M1.exe doesn't need emit functions, just the pre-computed results.

use crate::types::IsaResult;

/// HOT entry: [hh:2][offset:4][len:2] = 8 bytes, terminated by 0xFFFF
pub const HOT_ENTRY_SIZE: usize = 8;

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

/// Generate the selfhost startup x64 code.
///
/// This is the entry point of the --selfhost PE.
/// It calls a Rust-compiled helper function that does the actual work.
/// The helper function is compiled into the PE as a flat binary blob.
///
/// For V1, we use a pre-compiled Rust function stored as a static array.
/// The function signature is:
///   fn selfhost_main(hot_addr: u64, code_addr: u64, tyb_addr: u64, tyb_len: u64, out_path: *const u8) -> i32
///
/// The startup code:
/// 1. Sets up registers (hot_addr, code_addr, tyb_addr, tyb_len, out_path)
/// 2. Calls the helper function
/// 3. Exits with the return code
pub fn gen_selfhost_startup(hot_va: u64, code_va: u64, startup_va: u64) -> Vec<u8> {
    // V1: embed a simple Rust helper that does the actual compilation.
    // The helper is compiled as a separate function and its bytes are
    // stored here as a static blob.
    //
    // For now, V1 is a minimal stub that exits with code 42.
    // The startup sequence:
    //   mov eax, 42      ; exit code
    //   ret              ; return to caller
    let mut code: Vec<u8> = Vec::new();
    code.extend_from_slice(&[0xB8, 0x2A, 0x00, 0x00, 0x00]); // mov eax, 42
    code.push(0xC3); // ret
    code
}