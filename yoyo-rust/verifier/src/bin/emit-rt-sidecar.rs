//! Emit YOYO pe_dll probe as alternative cwd `yoyo_rt.dll` (Gate G slice).
//!
//! Usage: `cargo run -p verifier --bin emit-rt-sidecar -- <out.dll>`
//! Honest: fixed exit-2 export only; production default remains Rust sidecar.

use std::env;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    let Some(out) = args.next() else {
        eprintln!("usage: emit-rt-sidecar <out.dll>");
        eprintln!("  Gate G slice: write YOYO pe_dll probe as alt yoyo_rt.dll");
        eprintln!("  Still CUT — not OW-RT CLOSED");
        return ExitCode::from(2);
    };
    match verifier::pe_dll_link::write_yoyo_alt_sidecar(Path::new(&out)) {
        Ok(bytes) => {
            println!(
                "OW_RT_ALT_SIDECAR path={} bytes={} export={} disposition=CUT",
                out,
                bytes.len(),
                verifier::pe_dll_link::RUNTIME_EXPORT_NAME
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
