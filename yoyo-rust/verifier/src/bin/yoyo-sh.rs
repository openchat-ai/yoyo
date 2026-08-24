//! Stage 5 runtime selfhost launcher (M2→M3).
//! Reads `input.ky` or `input.tyb` from cwd, compiles via Rust host, writes `output.exe`.

use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("yoyo-sh: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), String> {
    let input = read_first_existing(&["input.ky", "input.tyb", "input.ty"])?;
    let data = fs::read(&input).map_err(|e| format!("read {}: {e}", input.display()))?;
    let pe = verifier::selfhost::bootstrap_compile(&data).map_err(|e| e.to_string())?;
    fs::write("output.exe", &pe).map_err(|e| format!("write output.exe: {e}"))?;
    Ok(())
}

fn read_first_existing(names: &[&str]) -> Result<std::path::PathBuf, String> {
    for name in names {
        let p = Path::new(name);
        if p.is_file() {
            return Ok(p.to_path_buf());
        }
    }
    Err(format!("no input file (tried {})", names.join(", ")))
}
