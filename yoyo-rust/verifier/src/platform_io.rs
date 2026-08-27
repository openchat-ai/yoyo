//! Platform I/O x64 emit (Stage 8-A / libyoyo Part 7.6).
//! Win32: kernel32 IAT at [r15 + slot*8] (pe_link reserves IAT header).
//! Linux: inline syscalls (mmap/open/read/write/close).

use crate::assembler::{load_state, movabs, store_state};
use crate::types::{IsaResult, Reg};

/// IAT slots at r15+0 (pe_link `prepend_win32_iat`).
pub const WIN32_IAT_VIRTUAL_ALLOC: u32 = 0;
pub const WIN32_IAT_CREATE_FILE: u32 = 1;
pub const WIN32_IAT_READ_FILE: u32 = 2;
pub const WIN32_IAT_WRITE_FILE: u32 = 3;
pub const WIN32_IAT_CLOSE_HANDLE: u32 = 4;

/// String table base from state/data base (r15). Matches yoyo data layout.
pub const STR_TABLE_OFF: u32 = 0x10000;
pub const STR_ENTRY_SIZE: u32 = 64;

const READ_CHUNK: u32 = 0x10000;

fn emit_call_r15_iat(slot: u32) -> Vec<u8> {
    // call qword [r15 + slot*8]
    let mut b = vec![0x41, 0xFF, 0x97];
    b.extend_from_slice(&(slot * 8).to_le_bytes());
    b
}

fn emit_lea_r15_r64(disp: u32, dest: Reg) -> IsaResult<Vec<u8>> {
    // lea dest, [r15 + disp32]
    let rm = dest.low3();
    let rex_r = if dest.rex_bit() { 0x04 } else { 0 };
    Ok(vec![
        0x49 | rex_r,
        0x8D,
        0x87 | (rm << 3),
        disp as u8,
        (disp >> 8) as u8,
        (disp >> 16) as u8,
        (disp >> 24) as u8,
    ])
}

fn str_path_off(str_idx: u8) -> u32 {
    STR_TABLE_OFF + (str_idx as u32) * STR_ENTRY_SIZE
}

// ── Linux (libyoyo: mmap / open / read / write / close) ─────────────────────

pub fn emit_linux_alloc(slot: u16, size: u64) -> IsaResult<Vec<u8>> {
    let mut out = Vec::new();
    // mmap(NULL, size, PROT_READ|PROT_WRITE, MAP_PRIVATE|MAP_ANONYMOUS, -1, 0)
    out.extend_from_slice(&[0x48, 0x31, 0xFF]); // xor rdi, rdi
    if size <= u32::MAX as u64 {
        out.extend_from_slice(&[0x48, 0xC7, 0xC6]); // mov rsi, imm32
        out.extend_from_slice(&(size as u32).to_le_bytes());
    } else {
        out.extend(movabs(Reg::Rsi, size)?);
    }
    out.extend_from_slice(&[0x48, 0xC7, 0xC2, 0x03, 0x00, 0x00, 0x00]); // rdx=3
    out.extend_from_slice(&[0x49, 0xC7, 0xC2, 0x22, 0x00, 0x00, 0x00]); // r10=0x22
    out.extend_from_slice(&[0x49, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF]); // r8=-1
    out.extend_from_slice(&[0x4D, 0x31, 0xC9]); // xor r9, r9
    out.extend_from_slice(&[0xB8, 0x09, 0x00, 0x00, 0x00]); // rax=9
    out.extend_from_slice(&[0x0F, 0x05]); // syscall
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

pub fn emit_linux_load_file(slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
    let path = str_path_off(str_idx);
    let mut out = Vec::new();
    // open(path, O_RDONLY, 0)
    out.extend(emit_lea_r15_r64(path, Reg::Rdi)?);
    out.extend_from_slice(&[0x31, 0xF6]); // xor esi, esi
    out.extend_from_slice(&[0xB8, 0x02, 0x00, 0x00, 0x00]); // rax=2
    out.extend_from_slice(&[0x0F, 0x05]);
    out.extend_from_slice(&[0x49, 0x89, 0xC4]); // mov r12, rax (fd)

    // mmap buffer
    out.extend_from_slice(&[0x48, 0x31, 0xFF]);
    out.extend_from_slice(&[0xBE]); // mov esi, READ_CHUNK
    out.extend_from_slice(&READ_CHUNK.to_le_bytes());
    out.extend_from_slice(&[0x48, 0xC7, 0xC2, 0x03, 0x00, 0x00, 0x00]);
    out.extend_from_slice(&[0x49, 0xC7, 0xC2, 0x22, 0x00, 0x00, 0x00]);
    out.extend_from_slice(&[0x49, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF]);
    out.extend_from_slice(&[0x4D, 0x31, 0xC9]);
    out.extend_from_slice(&[0xB8, 0x09, 0x00, 0x00, 0x00]);
    out.extend_from_slice(&[0x0F, 0x05]);
    out.extend_from_slice(&[0x49, 0x89, 0xC5]); // mov r13, rax (buf)

    // read(fd, buf, READ_CHUNK)
    out.extend_from_slice(&[0xB8, 0x00, 0x00, 0x00, 0x00]);
    out.extend_from_slice(&[0x4C, 0x89, 0xE7]); // mov rdi, r12
    out.extend_from_slice(&[0x4C, 0x89, 0xEE]); // mov rsi, r13
    out.extend_from_slice(&[0xBA]); // mov edx, READ_CHUNK
    out.extend_from_slice(&READ_CHUNK.to_le_bytes());
    out.extend_from_slice(&[0x0F, 0x05]);

    // close(fd)
    out.extend_from_slice(&[0x4C, 0x89, 0xE7]);
    out.extend_from_slice(&[0xB8, 0x03, 0x00, 0x00, 0x00]);
    out.extend_from_slice(&[0x0F, 0x05]);

    out.extend(movabs(Reg::Rax, 0)?); // use r13
    out.extend_from_slice(&[0x4C, 0x89, 0xE8]); // mov rax, r13
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

pub fn emit_linux_write_file(slot: u16, str_idx: u8, sz_slot: u16) -> IsaResult<Vec<u8>> {
    let path = str_path_off(str_idx);
    let mut out = Vec::new();
    // open(path, O_WRONLY|O_CREAT|O_TRUNC, 0666)
    out.extend(emit_lea_r15_r64(path, Reg::Rdi)?);
    out.extend_from_slice(&[0xBE, 0x41, 0x02, 0x00, 0x00]); // mov esi, 577
    out.extend_from_slice(&[0xBA, 0xB6, 0x01, 0x00, 0x00]); // mov edx, 438
    out.extend_from_slice(&[0xB8, 0x02, 0x00, 0x00, 0x00]);
    out.extend_from_slice(&[0x0F, 0x05]);
    out.extend_from_slice(&[0x49, 0x89, 0xC4]); // fd -> r12

    out.extend(load_state(slot, Reg::Rsi)?); // buf
    out.extend(load_state(sz_slot, Reg::Rdx)?); // count (low 32 used)
    out.extend_from_slice(&[0x4C, 0x89, 0xE7]); // mov rdi, r12
    out.extend_from_slice(&[0xB8, 0x01, 0x00, 0x00, 0x00]); // write
    out.extend_from_slice(&[0x0F, 0x05]);

    out.extend_from_slice(&[0x4C, 0x89, 0xE7]);
    out.extend_from_slice(&[0xB8, 0x03, 0x00, 0x00, 0x00]); // close
    out.extend_from_slice(&[0x0F, 0x05]);

    out.extend(movabs(Reg::Rax, 0)?);
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

// ── Win32 (libyoyo: VirtualAlloc / CreateFileA / ReadFile / WriteFile) ───────

pub fn emit_win32_alloc(slot: u16, size: u64) -> IsaResult<Vec<u8>> {
    let mut out = Vec::new();
    out.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]); // shadow
    out.extend_from_slice(&[0x31, 0xC9]); // xor ecx, ecx
    if size <= u32::MAX as u64 {
        out.extend_from_slice(&[0xBA]);
        out.extend_from_slice(&(size as u32).to_le_bytes());
    } else {
        out.extend(movabs(Reg::Rdx, size)?);
    }
    out.extend_from_slice(&[0x41, 0xB8, 0x00, 0x30, 0x00, 0x00]); // r8=MEM_COMMIT|RESERVE
    out.extend_from_slice(&[0x41, 0xB9, 0x04, 0x00, 0x00, 0x00]); // r9=PAGE_READWRITE
    out.extend(emit_call_r15_iat(WIN32_IAT_VIRTUAL_ALLOC));
    out.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

pub fn emit_win32_load_file(slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> {
    let path = str_path_off(str_idx);
    let mut out = Vec::new();
    // CreateFileA(path, GENERIC_READ, FILE_SHARE_READ, NULL, OPEN_EXISTING, ...)
    out.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);
    out.extend(emit_lea_r15_r64(path, Reg::Rcx)?);
    out.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x80]); // GENERIC_READ
    out.extend_from_slice(&[0x45, 0x31, 0xC0]); // xor r8d, r8d
    out.extend_from_slice(&[0x45, 0x31, 0xC9]); // xor r9d, r9d
    out.extend_from_slice(&[0xC7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00]); // OPEN_EXISTING
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00]); // FILE_ATTRIBUTE_NORMAL
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00]); // hTemplate=NULL
    out.extend(emit_call_r15_iat(WIN32_IAT_CREATE_FILE));
    out.extend_from_slice(&[0x48, 0x89, 0xC3]); // mov rbx, rax (handle)

    // VirtualAlloc(NULL, READ_CHUNK, ...)
    out.extend_from_slice(&[0x31, 0xC9]);
    out.extend_from_slice(&[0xBA]);
    out.extend_from_slice(&READ_CHUNK.to_le_bytes());
    out.extend_from_slice(&[0x41, 0xB8, 0x00, 0x30, 0x00, 0x00]);
    out.extend_from_slice(&[0x41, 0xB9, 0x04, 0x00, 0x00, 0x00]);
    out.extend(emit_call_r15_iat(WIN32_IAT_VIRTUAL_ALLOC));
    out.extend_from_slice(&[0x48, 0x89, 0xC6]); // mov rsi, rax (buf)

    // ReadFile(h, buf, READ_CHUNK, &read, NULL)
    out.extend_from_slice(&[0x48, 0x89, 0xD9]); // mov rcx, rbx
    out.extend_from_slice(&[0x48, 0x89, 0xF2]); // mov rdx, rsi
    out.extend_from_slice(&[0x41, 0xB8]);
    out.extend_from_slice(&READ_CHUNK.to_le_bytes());
    out.extend_from_slice(&[0x4C, 0x8D, 0x4C, 0x24, 0x20]); // lea r9, [rsp+0x20]
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00]);
    out.extend(emit_call_r15_iat(WIN32_IAT_READ_FILE));

    // CloseHandle(h)
    out.extend_from_slice(&[0x48, 0x89, 0xD9]);
    out.extend(emit_call_r15_iat(WIN32_IAT_CLOSE_HANDLE));
    out.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);

    out.extend_from_slice(&[0x48, 0x89, 0xF0]); // mov rax, rsi
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

pub fn emit_win32_write_file(slot: u16, str_idx: u8, sz_slot: u16) -> IsaResult<Vec<u8>> {
    let path = str_path_off(str_idx);
    let mut out = Vec::new();
    out.extend_from_slice(&[0x48, 0x83, 0xEC, 0x28]);
    out.extend(emit_lea_r15_r64(path, Reg::Rcx)?);
    out.extend_from_slice(&[0xBA, 0x00, 0x00, 0x00, 0x40]); // GENERIC_WRITE
    out.extend_from_slice(&[0x45, 0x31, 0xC0]);
    out.extend_from_slice(&[0x45, 0x31, 0xC9]);
    out.extend_from_slice(&[0xC7, 0x44, 0x24, 0x20, 0x02, 0x00, 0x00, 0x00]); // CREATE_ALWAYS
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00]);
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00]);
    out.extend(emit_call_r15_iat(WIN32_IAT_CREATE_FILE));
    out.extend_from_slice(&[0x48, 0x89, 0xC3]);

    out.extend(load_state(slot, Reg::Rdx)?);
    out.extend(load_state(sz_slot, Reg::R8)?);
    out.extend_from_slice(&[0x48, 0x89, 0xD9]); // rcx = handle
    out.extend_from_slice(&[0x4C, 0x8D, 0x4C, 0x24, 0x20]); // lea r9, [rsp+0x20]
    out.extend_from_slice(&[0x48, 0xC7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00]);
    out.extend(emit_call_r15_iat(WIN32_IAT_WRITE_FILE));

    out.extend_from_slice(&[0x48, 0x89, 0xD9]);
    out.extend(emit_call_r15_iat(WIN32_IAT_CLOSE_HANDLE));
    out.extend_from_slice(&[0x48, 0x83, 0xC4, 0x28]);

    out.extend(movabs(Reg::Rax, 0)?);
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

/// Reserved .data prefix size for Win32 kernel32 IAT (5 slots + import desc).
pub const WIN32_IAT_DATA_RESERVE: usize = 512;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linux_alloc_not_movabs_store_stub() {
        let b = emit_linux_alloc(0x50, 0x1000).unwrap();
        assert!(b.windows(2).any(|w| w == [0x0F, 0x05]), "expect syscall");
        assert!(!b.starts_with(&[0x48, 0xB8]), "not movabs+store stub");
    }

    #[test]
    fn win32_alloc_uses_iat_call() {
        let b = emit_win32_alloc(0x50, 0x1000).unwrap();
        assert!(b.windows(3).any(|w| w == [0x41, 0xFF, 0x97]), "call [r15+disp]");
    }

    #[test]
    fn load_file_emits_open_path() {
        let b = emit_linux_load_file(0x50, 0).unwrap();
        assert!(b.len() > 40);
        assert!(b.windows(2).any(|w| w == [0x0F, 0x05]));
    }
}
