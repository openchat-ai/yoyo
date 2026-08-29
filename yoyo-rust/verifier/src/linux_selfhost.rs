//! Linux ELF selfhost: genNrt gcc loader (Stage 7/8) + H_00 pure path (Stage 10-B).
//!
//! Stage 8 `--selfhost`: dynamically linked stub (`dlopen` -> `yoyo_runtime_selfhost_main`)
//! via system cc when available (WSL/Linux).
//!
//! Stage 10-B / post-v1.0 OW-RT H_00: ELF entry -> H_00 -> `execve("./.yoyo_h00_tramp")`
//! (cwd sidecar trampoline + cwd `./libyoyo_runtime.so`). No exact embed of tramp or `.so`.
//! OW-RT stays CUT (still Rust runtime + glibc/libdl trampoline). genNrt `--selfhost`
//! remains a separate host surface.

use crate::types::{IsaError, IsaResult};
use std::path::Path;
use std::process::Command;

pub const RUNTIME_SO_NAME: &str = "libyoyo_runtime.so";
pub const EXPORT_NAME: &str = "yoyo_runtime_selfhost_main";

/// Cwd-relative names: both are sidecars (no exact embed in seed ELF).
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
    // Stage 11-A: prefer `release-runtime` (fat LTO + strip) over plain release.
    let candidates = [
        root.join("target/release-runtime/libyoyo_runtime.so"),
        root.join("target/release/libyoyo_runtime.so"),
        root.join("target/debug/libyoyo_runtime.so"),
        root.join(format!(
            "target/{}/release-runtime/libyoyo_runtime.so",
            current_target()
        )),
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
        msg: "libyoyo_runtime.so not found — run `cargo build --profile release-runtime -p yoyo-runtime` (Linux/WSL)"
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

/// Metadata for H_00 cwd-sidecar trampoline execve (offsets relative to data / r15).
/// Post-v1.0 OW-RT deepen: neither `.so` nor trampoline is exact-embedded (`*_embed_*` = 0).
#[derive(Clone, Debug)]
pub struct H00Meta {
    pub so_name_off: u32,
    pub tramp_name_off: u32,
    pub so_embed_off: u32,
    pub so_embed_size: u32,
    pub tramp_embed_off: u32,
    pub tramp_embed_size: u32,
}

/// Append path strings only (no trampoline / `.so` bytes).
/// Post-v1.0 OW-RT: seed ELF trusts cwd `./.yoyo_h00_tramp` + `./libyoyo_runtime.so`.
pub fn append_h00_runtime_data(user_data: &[u8]) -> IsaResult<(Vec<u8>, H00Meta)> {
    let mut blob = user_data.to_vec();
    while blob.len() % 16 != 0 {
        blob.push(0);
    }
    let base = blob.len();

    let so_name_off = 0usize;
    let tramp_name_off = so_name_off + H00_SO_NAME.len();
    let total = tramp_name_off + H00_TRAMP_NAME.len();
    let pad = (16 - (total % 16)) % 16;

    blob.resize(base + total + pad, 0);
    blob[base + so_name_off..base + so_name_off + H00_SO_NAME.len()].copy_from_slice(H00_SO_NAME);
    blob[base + tramp_name_off..base + tramp_name_off + H00_TRAMP_NAME.len()]
        .copy_from_slice(H00_TRAMP_NAME);

    Ok((
        blob,
        H00Meta {
            so_name_off: (base + so_name_off) as u32,
            tramp_name_off: (base + tramp_name_off) as u32,
            so_embed_off: 0,
            so_embed_size: 0,
            tramp_embed_off: 0,
            tramp_embed_size: 0,
        },
    ))
}

/// Stage 10-B / post-v1.0 H_00 body: `execve` cwd sidecar trampoline (no extract).
/// Expects cwd `./.yoyo_h00_tramp` + `./libyoyo_runtime.so` (OW-RT shrink; still CUT).
/// ELF entry is `jmp H_00` (not `call`); this must never return.
pub fn gen_h00_selfhost_main(meta: &H00Meta) -> Vec<u8> {
    let mut c: Vec<u8> = Vec::new();

    // execve(tramp_path, [tramp_path, NULL], NULL) -- tramp/dlopen is host-preplaced.
    let _so = meta.so_name_off; // observe marker lives in .data; body only needs tramp path
    let _ = (_so, meta.so_embed_off, meta.so_embed_size, meta.tramp_embed_off, meta.tramp_embed_size);
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
    fn h00_main_emits_execve_only() {
        let meta = H00Meta {
            so_name_off: 0x100,
            tramp_name_off: 0x120,
            so_embed_off: 0,
            so_embed_size: 0,
            tramp_embed_off: 0,
            tramp_embed_size: 0,
        };
        let body = gen_h00_selfhost_main(&meta);
        assert!(body.windows(2).any(|w| w == [0x0F, 0x05]));
        // execve + exit only (no open/write extract loop).
        let syscall_count = body.windows(2).filter(|w| *w == [0x0F, 0x05]).count();
        assert_eq!(syscall_count, 2, "H_00 should be execve+exit only");
        assert!(body.len() < 64, "H_00 body unexpectedly large");
    }

    #[test]
    fn append_h00_paths_only_no_embeds() {
        let tramp = trampoline_bytes();
        let (blob, meta) = append_h00_runtime_data(b"user").unwrap();
        assert_eq!(meta.so_embed_size, 0);
        assert_eq!(meta.so_embed_off, 0);
        assert_eq!(meta.tramp_embed_size, 0);
        assert_eq!(meta.tramp_embed_off, 0);
        assert!(blob.windows(H00_SO_NAME.len()).any(|w| w == H00_SO_NAME));
        assert!(blob.windows(H00_TRAMP_NAME.len()).any(|w| w == H00_TRAMP_NAME));
        assert!(!blob.windows(tramp.len()).any(|w| w == tramp), "tramp must not be embedded");
    }
}
