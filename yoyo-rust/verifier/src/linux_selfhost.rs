//! Linux ELF selfhost startup + runtime `.so` sidecar (Stage 7 S7.4).
//!
//! M2→M3: dynamically linked stub (`dlopen` → `yoyo_runtime_selfhost_main`) built via gcc/ld
//! when available (WSL/Linux). M1→M2 uses `elf_link::link_elf` through `bootstrap_compile_linux`.

use crate::types::{IsaError, IsaResult};
use std::path::Path;
use std::process::Command;

pub const RUNTIME_SO_NAME: &str = "libyoyo_runtime.so";
pub const EXPORT_NAME: &str = "yoyo_runtime_selfhost_main";

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
        msg: "libyoyo_runtime.so not found — run `cargo build --release -p yoyo-runtime`"
            .into(),
    })
}

fn current_target() -> String {
    std::env::var("TARGET").unwrap_or_else(|_| "x86_64-unknown-linux-gnu".into())
}

/// Build minimal dynamically linked gen2rt ELF via system gcc (embedded startup).
pub fn link_elf_selfhost_runtime(_code: &[u8], _data: &[u8], _hot_table: &[u8]) -> IsaResult<Vec<u8>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_so_name() {
        assert!(RUNTIME_SO_NAME.ends_with(".so"));
    }
}
