//! Stage 13-A — seed/link host observability (fail-closed gate surface).
//!
//! Pure M4 still begins with host `yoyo link` / `yoyo bootstrap` (no `--selfhost`).
//! These helpers emit a machine-parseable `SEED_HOST` line so scripts can pin:
//! - seed PE/ELF uses the H_00 path (not genNrt `--selfhost` wrapper)
//! - link and bootstrap stay on one observable host surface
//! - post-v1.0 OW-SEED: `sha256_prefix` (16 hex) matches on-disk seed bytes under
//!   stage13/15 inventory (emitter identity is pinned by the gate against `yoyo.exe`)
//!
//! Honest remaining: host Rust `yoyo.exe` still performs the seed compile+link (OW-SEED CUT).

use crate::ddc;

/// Hex chars of seed SHA-256 printed on `SEED_HOST` (fail-closed match vs file hash).
pub const SEED_SHA256_PREFIX_LEN: usize = 16;

/// Classify a linked image's seed/host entry shape from embedded ASCII markers.
///
/// - `h00` — Stage 9-A+ / post-v1.0 sidecar path (Win cwd `yoyo_rt.dll` via PEB-resolved
///   LoadLibraryA — ASCII may be absent; or Linux cwd `libyoyo_runtime.so` + embedded
///   dlopen trampoline; libdl may live only in the trampoline blob)
/// - `gennrt` — `bootstrap --selfhost` / genNrt GetTempPath-style wrapper
/// - `plain` — no full-body runtime extract markers
pub fn classify_seed_path(bytes: &[u8]) -> &'static str {
    let has_temp = find_ascii(bytes, b"GetTempPathA");
    let has_win_ll = find_ascii(bytes, b"LoadLibraryA");
    let has_yoyo_rt =
        find_ascii(bytes, b"yoyo_rt.dll") || find_ascii(bytes, b"libyoyo_runtime.so");
    // Linux H_00 trampoline uses dlopen; string may appear in embedded tramp bytes.
    let has_linux_dl = find_ascii(bytes, b"dlopen") || find_ascii(bytes, b"libdl.so");
    if has_temp {
        "gennrt"
    } else if has_yoyo_rt && (has_win_ll || has_linux_dl) {
        "h00"
    } else if has_win_ll || has_linux_dl {
        "h00-like"
    } else if has_yoyo_rt {
        // Embedded runtime present — treat as H_00 seed even if loader string is opaque.
        "h00"
    } else {
        "plain"
    }
}

/// Print one machine-parseable observe line (stdout). Gate scripts grep `SEED_HOST`.
pub fn emit_observe(cmd: &str, target: &str, bytes: &[u8], dll_embed: Option<usize>) {
    let path = classify_seed_path(bytes);
    let sha = ddc::sha256_hex(bytes);
    let n = SEED_SHA256_PREFIX_LEN.min(sha.len());
    let prefix = &sha[..n];
    let dll = match dll_embed {
        Some(n) => n.to_string(),
        None => "-".to_string(),
    };
    println!(
        "SEED_HOST cmd={cmd} target={target} path={path} bytes={} dll_embed={dll} sha256_prefix={prefix}",
        bytes.len()
    );
}

fn find_ascii(hay: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || hay.len() < needle.len() {
        return false;
    }
    hay.windows(needle.len()).any(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_h00_vs_gennrt() {
        let h00 = b"....LoadLibraryA....yoyo_rt.dll....";
        assert_eq!(classify_seed_path(h00), "h00");
        let h00_linux = b"....libyoyo_runtime.so....dlopen....";
        assert_eq!(classify_seed_path(h00_linux), "h00");
        let h00_so_only = b"....libyoyo_runtime.so....";
        assert_eq!(classify_seed_path(h00_so_only), "h00");
        let gennrt = b"....GetTempPathA....LoadLibraryA....yoyo_runtime.dll....";
        assert_eq!(classify_seed_path(gennrt), "gennrt");
        assert_eq!(classify_seed_path(b"no markers"), "plain");
    }
}
