//! Emit YOYO pe_dll as cwd `yoyo_rt.dll`, optionally run Gate G / M2 slices.
//!
//! Usage:
//!   emit-rt-sidecar <out.dll>
//!   emit-rt-sidecar --rcw <workdir>
//!   emit-rt-sidecar --export-compile <workdir>
//!   emit-rt-sidecar --in-dll-recompile <workdir>
//!   emit-rt-sidecar --m2-h00-subset <workdir>
//!   emit-rt-sidecar --gate-i-gen4-yoyo-tyb <workdir>
//!
//! Honest: S3.738 human A — OW-RT / G CLOSED; production Off→YOYO + CreateProcess-compiler; follow-ons remain.

use std::env;
use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args: Vec<String> = env::args().skip(1).collect();
    // HB-FX: dump pre-link parse1 code+data for JS peer link (offline regen only).
    if args.first().map(|s| s.as_str()) == Some("--dump-compiler-payload") {
        args.remove(0);
        let Some(work) = args.first() else {
            eprintln!("usage: emit-rt-sidecar --dump-compiler-payload <outdir>");
            return ExitCode::from(2);
        };
        let work_dir = Path::new(work);
        if let Err(e) = std::fs::create_dir_all(work_dir) {
            eprintln!("error: mkdir {}: {e}", work_dir.display());
            return ExitCode::FAILURE;
        }
        #[cfg(windows)]
        {
            match verifier::pe_dll_link::yoyo_dump_compiler_payload(work_dir) {
                Ok((c, d)) => {
                    println!(
                        "HB_FX_DUMP_COMPILER_PAYLOAD code={} data={} dir={} status=GREEN",
                        c,
                        d,
                        work_dir.display()
                    );
                    return ExitCode::SUCCESS;
                }
                Err(()) => {
                    eprintln!("HB_FX_DUMP_COMPILER_PAYLOAD status=RED");
                    return ExitCode::FAILURE;
                }
            }
        }
        #[cfg(not(windows))]
        {
            eprintln!("HB_FX_DUMP_COMPILER_PAYLOAD status=SKIP reason=non-Windows");
            return ExitCode::from(2);
        }
    }
    // FULLBAR FS-1: co-place CreateProcess compiler next to YOYO sidecar (Gate I).
    if args.first().map(|s| s.as_str()) == Some("--place-compiler") {
        args.remove(0);
        let Some(work) = args.first() else {
            eprintln!("usage: emit-rt-sidecar --place-compiler <workdir>");
            eprintln!("  writes cwd yoyo_compiler.exe (parse1 PE) for CreateProcess path");
            return ExitCode::from(2);
        };
        let work_dir = Path::new(work);
        if let Err(e) = std::fs::create_dir_all(work_dir) {
            eprintln!("error: mkdir {}: {e}", work_dir.display());
            return ExitCode::FAILURE;
        }
        #[cfg(windows)]
        {
            match verifier::pe_dll_link::yoyo_place_compiler_pe(work_dir) {
                Ok(p) => {
                    let n = std::fs::metadata(&p).map(|m| m.len()).unwrap_or(0);
                    println!(
                        "OW_RT_PLACE_COMPILER path={} bytes={} status=GREEN",
                        p.display(),
                        n
                    );
                    return ExitCode::SUCCESS;
                }
                Err(()) => {
                    eprintln!("OW_RT_PLACE_COMPILER status=RED reason=no_compiler_pe_bytes");
                    return ExitCode::FAILURE;
                }
            }
        }
        #[cfg(not(windows))]
        {
            eprintln!("OW_RT_PLACE_COMPILER status=SKIP reason=non-Windows");
            return ExitCode::from(2);
        }
    }
    if args.first().map(|s| s.as_str()) == Some("--gate-i-gen4-yoyo-tyb") {
        args.remove(0);
        let Some(work) = args.first() else {
            eprintln!("usage: emit-rt-sidecar --gate-i-gen4-yoyo-tyb <workdir>");
            eprintln!("  S3.531 Gate I: trust-root gen4 + YOYO in-DLL oracle+yoyo.tyb;");
            eprintln!("  oracle != live compile. OW-RT CLOSED (human A); follow-ons remain.");
            return ExitCode::from(2);
        };
        let work_dir = Path::new(work);
        if let Err(e) = std::fs::create_dir_all(work_dir) {
            eprintln!("error: mkdir {}: {e}", work_dir.display());
            return ExitCode::FAILURE;
        }
        #[cfg(windows)]
        {
            let code = verifier::pe_dll_link::yoyo_gen4_path_gate_i_yoyo_tyb(work_dir);
            let sidecar = work_dir.join(verifier::pe_dll_link::RUNTIME_SIDECAR_NAME);
            let side_bytes = std::fs::metadata(&sidecar).map(|m| m.len()).unwrap_or(0);
            let out = work_dir.join("output.exe");
            let has_out = out.is_file();
            println!(
                "OW_RT_GATE_I_GEN4_YOYO_TYB workdir={} sidecar_bytes={} output={} exit={} disposition=CLOSED",
                work_dir.display(),
                side_bytes,
                has_out,
                code
            );
            return ExitCode::from(u8::try_from(code).unwrap_or(1));
        }
        #[cfg(not(windows))]
        {
            eprintln!("OW_RT_GATE_I_GEN4_YOYO_TYB status=SKIP reason=non-Windows");
            return ExitCode::from(2);
        }
    }
    if args.first().map(|s| s.as_str()) == Some("--m2-h00-subset") {
        args.remove(0);
        let Some(work) = args.first() else {
            eprintln!("usage: emit-rt-sidecar --m2-h00-subset <workdir>");
            eprintln!("  M2 / Gate I subset: H_00 seed + YOYO in-DLL DLL + mini RET;");
            eprintln!("  no Rust sidecar. OW-RT CLOSED (human A); follow-ons remain.");
            return ExitCode::from(2);
        };
        let work_dir = Path::new(work);
        if let Err(e) = std::fs::create_dir_all(work_dir) {
            eprintln!("error: mkdir {}: {e}", work_dir.display());
            return ExitCode::FAILURE;
        }
        #[cfg(windows)]
        {
            let code = verifier::pe_dll_link::yoyo_h00_path_m2_mini(work_dir);
            let sidecar = work_dir.join(verifier::pe_dll_link::RUNTIME_SIDECAR_NAME);
            let side_bytes = std::fs::metadata(&sidecar).map(|m| m.len()).unwrap_or(0);
            let out = work_dir.join("output.exe");
            let has_out = out.is_file();
            println!(
                "OW_RT_M2_H00_SUBSET workdir={} sidecar_bytes={} output={} exit={} disposition=CLOSED",
                work_dir.display(),
                side_bytes,
                has_out,
                code
            );
            return ExitCode::from(u8::try_from(code).unwrap_or(1));
        }
        #[cfg(not(windows))]
        {
            eprintln!("OW_RT_M2_H00_SUBSET status=SKIP reason=non-Windows");
            return ExitCode::from(2);
        }
    }
    if args.first().map(|s| s.as_str()) == Some("--in-dll-recompile") {
        args.remove(0);
        let Some(work) = args.first() else {
            eprintln!("usage: emit-rt-sidecar --in-dll-recompile <workdir>");
            eprintln!("  call-time ReadFile + multi-entry YOYO oracle match;");
            eprintln!("  writes output.exe (exits 0/1/2/3). OW-RT CLOSED (human A).");
            return ExitCode::from(2);
        };
        let work_dir = Path::new(work);
        if let Err(e) = std::fs::create_dir_all(work_dir) {
            eprintln!("error: mkdir {}: {e}", work_dir.display());
            return ExitCode::FAILURE;
        }
        let code = verifier::pe_dll_link::yoyo_sidecar_in_dll_recompile(work_dir);
        let sidecar = work_dir.join(verifier::pe_dll_link::RUNTIME_SIDECAR_NAME);
        let side_bytes = std::fs::metadata(&sidecar).map(|m| m.len()).unwrap_or(0);
        println!(
            "OW_RT_IN_DLL_RECOMPILE workdir={} sidecar_bytes={} exit={} disposition=CLOSED",
            work_dir.display(),
            side_bytes,
            code
        );
        return ExitCode::from(u8::try_from(code).unwrap_or(1));
    }
    if args.first().map(|s| s.as_str()) == Some("--export-compile") {
        args.remove(0);
        let Some(work) = args.first() else {
            eprintln!("usage: emit-rt-sidecar --export-compile <workdir>");
            eprintln!("  emit-time bootstrap_compile baked into YOYO pe_dll export;");
            eprintln!("  call writes output.exe (exits 0/1/2/3). OW-RT CLOSED (human A).");
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
            "OW_RT_EXPORT_COMPILE workdir={} sidecar_bytes={} exit={} disposition=CLOSED",
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
            eprintln!("  OW-RT CLOSED (human A); follow-ons remain");
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
            "OW_RT_SIDECAR_RCW workdir={} sidecar_bytes={} exit={} disposition=CLOSED",
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
        eprintln!("       emit-rt-sidecar --in-dll-recompile <workdir>");
        eprintln!("       emit-rt-sidecar --m2-h00-subset <workdir>");
        eprintln!("       emit-rt-sidecar --gate-i-gen4-yoyo-tyb <workdir>");
        eprintln!("  Gate G: alt / RCW / export-compile / in-DLL recompile");
        eprintln!("  M2: H_00 + YOYO in-DLL mini (Gate I subset)");
        eprintln!("  S3.531: gen4 + oracle+yoyo.tyb (Gate I; oracle != live)");
        eprintln!("  OW-RT CLOSED (human A); follow-ons remain");
        return ExitCode::from(2);
    };
    match verifier::pe_dll_link::write_yoyo_alt_sidecar(Path::new(out)) {
        Ok(bytes) => {
            println!(
                "OW_RT_ALT_SIDECAR path={} bytes={} export={} disposition=CLOSED",
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
