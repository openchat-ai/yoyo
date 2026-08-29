//! In-process selfhost runtime — loaded by gen2 startup stub.
//! Reads `input.tyb` (or `input.ky`), compiles via verifier, writes `output.exe` / `output.elf`.

use std::ffi::CStr;
use std::os::raw::c_char;

#[cfg(windows)]
mod win_mm_probe;

#[cfg(target_os = "linux")]
fn compile_input(input: &[u8]) -> Result<Vec<u8>, i32> {
    verifier::selfhost::bootstrap_compile_linux(input).map_err(|_| 1)
}

#[cfg(not(target_os = "linux"))]
fn compile_input(input: &[u8]) -> Result<Vec<u8>, i32> {
    verifier::selfhost::bootstrap_compile(input).map_err(|_| 1)
}

#[cfg(target_os = "linux")]
fn default_output_name() -> &'static str {
    "output.elf"
}

#[cfg(not(target_os = "linux"))]
fn default_output_name() -> &'static str {
    "output.exe"
}

fn read_input() -> Result<Vec<u8>, i32> {
    for name in ["input.tyb", "input.ky", "input.ty"] {
        if let Ok(data) = std::fs::read(name) {
            return Ok(data);
        }
    }
    Err(2)
}

/// Main entry — called from embedded startup via LoadLibrary/dlopen export.
#[no_mangle]
pub extern "C" fn yoyo_runtime_selfhost_main() -> i32 {
    // Manual-map smoke: no DllMain/CRT — use kernel32 IAT only (resolved by H_00 stub).
    #[cfg(windows)]
    if let Some(code) = win_mm_probe::run_if_env_set() {
        return code;
    }
    let input = match read_input() {
        Ok(d) => d,
        Err(e) => return e,
    };
    let out = match compile_input(&input) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if std::fs::write(default_output_name(), &out).is_err() {
        return 3;
    }
    0
}

/// Optional path-based entry for tests.
#[no_mangle]
pub unsafe extern "C" fn yoyo_runtime_selfhost_paths(
    input_path: *const c_char,
    output_path: *const c_char,
) -> i32 {
    if input_path.is_null() || output_path.is_null() {
        return 4;
    }
    let in_path = match CStr::from_ptr(input_path).to_str() {
        Ok(s) => s,
        Err(_) => return 5,
    };
    let out_path = match CStr::from_ptr(output_path).to_str() {
        Ok(s) => s,
        Err(_) => return 6,
    };
    let input = match std::fs::read(in_path) {
        Ok(d) => d,
        Err(_) => return 2,
    };
    let out = match compile_input(&input) {
        Ok(p) => p,
        Err(e) => return e,
    };
    if std::fs::write(out_path, &out).is_err() {
        return 3;
    }
    0
}
