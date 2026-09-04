//! Emit YOYO pe_dll as cwd `yoyo_rt.dll`, optionally run Gate G slices.
//!
//! Usage:
//!   emit-rt-sidecar <out.dll>
//!   emit-rt-sidecar --rcw <workdir>              # place + host-orchestrated R→C→W
//!   emit-rt-sidecar --export-compile <workdir>   # bake YOYO compile into export
//!
//! Honest: production default remains Rust; still CUT — not OW-RT CLOSED.

use std::env;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.first().map(|s| s.as_str()) == Some("--export-compile") {
        args.remove(0);
        let Some(work) = args.first() else {
            eprintln!("usage: emit-rt-sidecar --export-compile <workdir>");
            eprintln!("  emit-time bootstrap_compile baked into YOYO pe_dll export;");
            eprintln!("  call writes output.exe (exits 0/1/2/3). Still CUT.");
            return ExitCode::from(2);
        };
        let work_dir = Path::new(work);
        if let Err(e) = std::fs::create_dir_all(work_dir) {
            eprintln!("error: mkdir {}: {e}", work_dir.display());
            return ExitCode::FAILURE;
        }
        let code = verifier::pe_dll_link::yoyo_sidecar_export_compile(work_dir);
        let sidecar = work_dir.join(verifier::pe_dll_link::RUNTIME_SIDECAR_NAME);
        let side_bytes = std::fs::metadata(&sidecar).map(|m| m.len()).unwrap_or(0);
        println!(
            "OW_RT_EXPORT_COMPILE workdir={} sidecar_bytes={} exit={} disposition=CUT",
            work_dir.display(),
            side_bytes,
            code
        );
        return ExitCode::from(u8::try_from(code).unwrap_or(1));
    }
    if args.first().map(|s| s.as_str()) == Some("--rcw") {
        args.remove(0);
        let Some(work) = args.first() else {
            eprintln!("usage: emit-rt-sidecar --rcw <workdir>");
            eprintln!("  places <workdir>/yoyo_rt.dll then YOYO R→C→W (exits 0/1/2/3)");
            eprintln!("  Still CUT — not OW-RT CLOSED");
            return ExitCode::from(2);
        };
        let work_dir = Path::new(work);
        if let Err(e) = std::fs::create_dir_all(work_dir) {
            eprintln!("error: mkdir {}: {e}", work_dir.display());
            return ExitCode::FAILURE;
        }
        let code = verifier::pe_dll_link::yoyo_sidecar_path_rcw(work_dir);
        let sidecar = work_dir.join(verifier::pe_dll_link::RUNTIME_SIDECAR_NAME);
        let side_bytes = std::fs::metadata(&sidecar).map(|m| m.len()).unwrap_or(0);
        println!(
            "OW_RT_SIDECAR_RCW workdir={} sidecar_bytes={} exit={} disposition=CUT",
            work_dir.display(),
            side_bytes,
            code
        );
        return ExitCode::from(u8::try_from(code).unwrap_or(1));
    }

    let Some(out) = args.first() else {
        eprintln!("usage: emit-rt-sidecar <out.dll>");
        eprintln!("       emit-rt-sidecar --rcw <workdir>");
        eprintln!("       emit-rt-sidecar --export-compile <workdir>");
        eprintln!("  Gate G: YOYO pe_dll alt / sidecar-path R→C→W / export-compile");
        eprintln!("  Still CUT — not OW-RT CLOSED");
        return ExitCode::from(2);
    };
    match verifier::pe_dll_link::write_yoyo_alt_sidecar(Path::new(out)) {
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
