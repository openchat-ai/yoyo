//! Linux ELF selfhost: genNrt gcc loader (Stage 7/8) + H_00 pure path (Stage 10-B).
//!
//! Stage 8 `--selfhost`: dynamically linked stub (`dlopen` → `yoyo_runtime_selfhost_main`)
//! via system cc when available (WSL/Linux).
//!
//! Stage 10-B H_00: ELF entry → H_00 → syscall extract of embedded trampoline +
//! `libyoyo_runtime.so` → `execve` trampoline (no `bootstrap --selfhost`).

use crate::types::{IsaError, IsaResult};
use std::path::Path;
use std::process::Command;

pub const RUNTIME_SO_NAME: &str = "libyoyo_runtime.so";
pub const EXPORT_NAME: &str = "yoyo_runtime_selfhost_main";

/// Cwd-relative names written by H_00 extract stub (must match trampoline expectations).
pub const H00_SO_NAME: &[u8] = b"./libyoyo_runtime.so\0";
pub const H00_TRAMP_NAME: &[u8] = b"./.yoyo_h00_tramp\0";

const LOADER_C: &str = r#"#include <dlfcn.h>
#include <limits.h>
#include <string.h>
#include <unistd.h>

int main(void) {
    char path[PATH_MAX];
    if (!getcwd(path, sizeof(path))) return 1;
    size_t n = strlen(path);
    if (n + 1 + sizeof("libyoyo_runtime.so") >= sizeof(path)) return 1;
    path[n] = '/';
    memcpy(path + n + 1, "libyoyo_runtime.so", sizeof("libyoyo_runtime.so"));
    void *h = dlopen(path, RTLD_LAZY);
    if (!h) return 1;
    int (*fn)(void) = (int (*)(void))dlsym(h, "yoyo_runtime_selfhost_main");
    if (!fn) return 2;
    return fn();
}
"#;

/// Load prebuilt `libyoyo_runtime.so` from release/debug target dirs.
pub fn runtime_so_bytes() -> IsaResult<Vec<u8>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    let candidates = [
        root.join("target/release/libyoyo_runtime.so"),
        root.join("target/debug/libyoyo_runtime.so"),
        root.join(format!(
            "target/{}/release/libyoyo_runtime.so",
            current_target()
        )),
        root.join(format!(
            "target/{}/debug/libyoyo_runtime.so",
            current_target()
        )),
    ];
    for path in &candidates {
        if path.is_file() {
            return std::fs::read(path).map_err(|e| IsaError::IoError {
                msg: format!("read {}: {e}", path.display()),
            });
        }
    }
    Err(IsaError::IoError {
        msg: "libyoyo_runtime.so not found — run `cargo build --release -p yoyo-runtime` (Linux/WSL)"
            .into(),
    })
}

/// Prebuilt dlopen trampoline ELF (committed blob; rebuild via scripts/build-linux-h00-tramp.sh).
pub fn trampoline_bytes() -> &'static [u8] {
    include_bytes!("../blobs/linux_h00_tramp.elf")
}

fn current_target() -> String {
    std::env::var("TARGET").unwrap_or_else(|_| "x86_64-unknown-linux-gnu".into())
}

/// Build minimal dynamically linked gen2rt ELF via system gcc (embedded startup).
pub fn link_elf_selfhost_runtime(
    _code: &[u8],
    _data: &[u8],
    _hot_table: &[u8],
) -> IsaResult<Vec<u8>> {
    let work = std::env::temp_dir().join(format!(
        "yoyo-linux-selfhost-{}",
        std::process::id()
    ));
    std::fs::create_dir_all(&work).map_err(|e| IsaError::IoError {
        msg: format!("create {}: {e}", work.display()),
    })?;

    let loader = work.join("loader.c");
    std::fs::write(&loader, LOADER_C).map_err(|e| IsaError::IoError {
        msg: e.to_string(),
    })?;
    let out_elf = work.join("gen2rt.elf");

    let status = Command::new("cc")
        .args([
            "-O2",
            "-o",
            out_elf.to_str().unwrap(),
            loader.to_str().unwrap(),
            "-ldl",
        ])
        .status()
        .map_err(|e| IsaError::IoError {
            msg: format!("spawn cc: {e} (need gcc on PATH — use WSL on Windows)"),
        })?;
    if !status.success() {
        let _ = std::fs::remove_dir_all(&work);
        return Err(IsaError::IoError {
            msg: format!("cc link gen2rt failed (exit {status:?})"),
        });
    }

    let bytes = std::fs::read(&out_elf).map_err(|e| IsaError::IoError {
        msg: e.to_string(),
    })?;
    let _ = std::fs::remove_dir_all(&work);
    Ok(bytes)
}

/// Metadata for H_00 extract + execve stub (offsets relative to data / r15).
#[derive(Clone, Debug)]
pub struct H00Meta {
    pub so_name_off: u32,
    pub tramp_name_off: u32,
    pub so_embed_off: u32,
    pub so_embed_size: u32,
    pub tramp_embed_off: u32,
    pub tramp_embed_size: u32,
}

/// Append path strings + embedded runtime .so + trampoline after user data.
pub fn append_h00_runtime_data(
    user_data: &[u8],
    so_bytes: &[u8],
    tramp_bytes: &[u8],
) -> IsaResult<(Vec<u8>, H00Meta)> {
    let mut blob = user_data.to_vec();
    while blob.len() % 16 != 0 {
        blob.push(0);
    }
    let base = blob.len();

    let so_name_off = 0usize;
    let tramp_name_off = so_name_off + H00_SO_NAME.len();
    let so_embed_off = align_up(tramp_name_off + H00_TRAMP_NAME.len(), 16);
    let so_pad = (16 - (so_bytes.len() % 16)) % 16;
    let tramp_embed_off = so_embed_off + so_bytes.len() + so_pad;
    let tramp_pad = (16 - (tramp_bytes.len() % 16)) % 16;
    let total = tramp_embed_off + tramp_bytes.len() + tramp_pad;

    blob.resize(base + total, 0);
    blob[base + so_name_off..base + so_name_off + H00_SO_NAME.len()].copy_from_slice(H00_SO_NAME);
    blob[base + tramp_name_off..base + tramp_name_off + H00_TRAMP_NAME.len()]
        .copy_from_slice(H00_TRAMP_NAME);
    blob[base + so_embed_off..base + so_embed_off + so_bytes.len()].copy_from_slice(so_bytes);
    blob[base + tramp_embed_off..base + tramp_embed_off + tramp_bytes.len()]
        .copy_from_slice(tramp_bytes);

    Ok((
        blob,
        H00Meta {
            so_name_off: (base + so_name_off) as u32,
            tramp_name_off: (base + tramp_name_off) as u32,
            so_embed_off: (base + so_embed_off) as u32,
            so_embed_size: so_bytes.len() as u32,
            tramp_embed_off: (base + tramp_embed_off) as u32,
            tramp_embed_size: tramp_bytes.len() as u32,
        },
    ))
}

fn align_up(v: usize, a: usize) -> usize {
    (v + a - 1) & !(a - 1)
}

/// Stage 10-B H_00 body: write embedded .so + trampoline via syscalls, then execve trampoline.
/// ELF entry is `jmp H_00` (not `call`); this must never return.
pub fn gen_h00_selfhost_main(meta: &H00Meta) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();

    emit_write_embedded(&mut c, meta.so_name_off, meta.so_embed_off, meta.so_embed_size);
    emit_write_embedded(
        &mut c,
        meta.tramp_name_off,
        meta.tramp_embed_off,
        meta.tramp_embed_size,
    );

    // execve(tramp_path, [tramp_path, NULL], NULL)
    emit_lea_r15(&mut c, 7, meta.tramp_name_off); // rdi
    c.extend_from_slice(&[0x6A, 0x00]); // push 0
    c.push(0x57); // push rdi
    c.extend_from_slice(&[0x48, 0x89, 0xE6]); // mov rsi, rsp
    c.extend_from_slice(&[0x48, 0x31, 0xD2]); // xor rdx, rdx
    emit_mov_eax_imm(&mut c, 59); // SYS_execve
    c.extend_from_slice(&[0x0F, 0x05]);

    emit_mov_eax_imm(&mut c, 60); // SYS_exit
    emit_mov_edi_imm(&mut c, 127);
    c.extend_from_slice(&[0x0F, 0x05]);

    c
}

fn emit_write_embedded(c: &mut Vec<u8>, name_off: u32, embed_off: u32, size: u32) {
    emit_lea_r15(c, 7, name_off);
    emit_mov_eax_imm(c, 87); // SYS_unlink
    c.extend_from_slice(&[0x0F, 0x05]);

    emit_lea_r15(c, 7, name_off);
    emit_mov_esi_imm(c, 0x241); // O_WRONLY|O_CREAT|O_TRUNC
    emit_mov_edx_imm(c, 0o755);
    emit_mov_eax_imm(c, 2); // SYS_open
    c.extend_from_slice(&[0x0F, 0x05]);
    c.extend_from_slice(&[0x49, 0x89, 0xC4]); // mov r12, rax

    // r13 = remaining, r14 = cursor
    emit_mov_r13_imm(c, size);
    emit_lea_r15(c, 6, embed_off); // rsi = buf
    c.extend_from_slice(&[0x49, 0x89, 0xF6]); // mov r14, rsi

    // write_loop:
    let loop_top = c.len();
    c.extend_from_slice(&[0x4D, 0x85, 0xED]); // test r13, r13
    let jz_patch = c.len();
    c.extend_from_slice(&[0x74, 0x00]); // jz done (patch)

    c.extend_from_slice(&[0x4C, 0x89, 0xE7]); // mov rdi, r12
    c.extend_from_slice(&[0x4C, 0x89, 0xF6]); // mov rsi, r14
    c.extend_from_slice(&[0x4C, 0x89, 0xEA]); // mov rdx, r13
    emit_mov_eax_imm(c, 1); // SYS_write
    c.extend_from_slice(&[0x0F, 0x05]);
    // if rax <= 0: exit 126
    c.extend_from_slice(&[0x48, 0x85, 0xC0]); // test rax, rax
    let jle_patch = c.len();
    c.extend_from_slice(&[0x7E, 0x00]); // jle fail

    c.extend_from_slice(&[0x49, 0x01, 0xC6]); // add r14, rax
    c.extend_from_slice(&[0x49, 0x29, 0xC5]); // sub r13, rax
    let jmp_back = c.len();
    c.extend_from_slice(&[0xEB, 0x00]); // jmp loop_top
    c[jmp_back + 1] = (loop_top as i32 - (jmp_back as i32 + 2)) as u8;

    let fail = c.len();
    c[jle_patch + 1] = (fail as i32 - (jle_patch as i32 + 2)) as u8;
    emit_mov_eax_imm(c, 60);
    emit_mov_edi_imm(c, 126);
    c.extend_from_slice(&[0x0F, 0x05]);

    let done = c.len();
    c[jz_patch + 1] = (done as i32 - (jz_patch as i32 + 2)) as u8;

    c.extend_from_slice(&[0x4C, 0x89, 0xE7]); // mov rdi, r12
    emit_mov_eax_imm(c, 3); // SYS_close
    c.extend_from_slice(&[0x0F, 0x05]);
}

fn emit_mov_r13_imm(c: &mut Vec<u8>, imm: u32) {
    // mov r13, imm32 (zero-extends): 41 BD imm32
    c.extend_from_slice(&[0x41, 0xBD]);
    c.extend_from_slice(&imm.to_le_bytes());
}

fn emit_lea_r15(c: &mut Vec<u8>, reg_low3: u8, disp: u32) {
    // lea r64, [r15 + disp32]: REX.W+B (0x49), not 0x4C (that sets R instead of B).
    let rex = 0x49;
    let modrm = 0x80 | (reg_low3 << 3) | 0x07;
    c.extend_from_slice(&[rex, 0x8D, modrm]);
    c.extend_from_slice(&disp.to_le_bytes());
}

fn emit_mov_eax_imm(c: &mut Vec<u8>, imm: u32) {
    c.push(0xB8);
    c.extend_from_slice(&imm.to_le_bytes());
}

fn emit_mov_edi_imm(c: &mut Vec<u8>, imm: u32) {
    c.push(0xBF);
    c.extend_from_slice(&imm.to_le_bytes());
}

fn emit_mov_esi_imm(c: &mut Vec<u8>, imm: u32) {
    c.push(0xBE);
    c.extend_from_slice(&imm.to_le_bytes());
}

fn emit_mov_edx_imm(c: &mut Vec<u8>, imm: u32) {
    c.push(0xBA);
    c.extend_from_slice(&imm.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_so_name() {
        assert!(RUNTIME_SO_NAME.ends_with(".so"));
    }

    #[test]
    fn trampoline_blob_is_elf() {
        let t = trampoline_bytes();
        assert!(t.len() > 64);
        assert_eq!(&t[0..4], b"\x7fELF");
    }

    #[test]
    fn h00_main_emits_syscalls() {
        let meta = H00Meta {
            so_name_off: 0x100,
            tramp_name_off: 0x120,
            so_embed_off: 0x200,
            so_embed_size: 0x1000,
            tramp_embed_off: 0x2000,
            tramp_embed_size: 0x100,
        };
        let body = gen_h00_selfhost_main(&meta);
        assert!(body.windows(2).any(|w| w == [0x0F, 0x05]));
        assert!(body.len() > 40);
    }
}
