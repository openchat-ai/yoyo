//! yoyo-exec-run — W-START attempt-N5b CLI (EXPERIMENTAL · NON-GREEN).
//!
//! Usage:
//!   yoyo-exec-run run <path.bin>            # raw .text bytes
//!   yoyo-exec-run run-hex <path.hex>        # hex-encoded .text (e.g. from `yoyo hash`)
//!   yoyo-exec-run smoke                     # run a tiny built-in movabs+store+ret
//!
//! Exits:
//!   0  on clean HALT (RET in handler H_xx with empty call stack)
//!   1  on FAULT (decode / OOB / unimplemented / step-limit)
//!   2  on bad CLI args / bad file
//!
//! This binary is a NEW component and is NOT part of the locked trust
//! surface. It MUST NOT be advertised as DDC / freeze / self-host.

use std::env;
use std::fs;
use std::process::ExitCode;

use yoyo_executor::{run_bytes, run_hex_text, ExitReason, RunLimits};

fn usage() -> ! {
    eprintln!(
        "yoyo-exec-run — W-START attempt-N5b (EXPERIMENTAL)\n\
         \n\
         Usage:\n\
           yoyo-exec-run run <path.bin>      raw .text bytes from `yoyo link`\n\
           yoyo-exec-run run-hex <path.hex>  hex-encoded .text\n\
           yoyo-exec-run smoke                built-in movabs+store+ret probe\n\
         \n\
         Exit codes:\n\
           0 — clean HALT\n\
           1 — FAULT (decode/OOB/unimplemented/step-limit)\n\
           2 — bad args / file\n"
    );
    std::process::exit(2);
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        usage();
    }
    let cmd = args[0].as_str();
    let limits = RunLimits {
        steps: 50_000,
        mmu_capacity: 16 * 1024,
    };
    let rest: Vec<String> = args[1..].to_vec();
    match cmd {
        "run" => {
            if rest.len() != 1 {
                usage();
            }
            let bytes = match fs::read(&rest[0]) {
                Ok(b) => b,
                Err(e) => {
                    eprintln!("read error: {e}");
                    return ExitCode::from(2);
                }
            };
            let outcome = run_bytes(&bytes, limits);
            print_outcome(&outcome)
        }
        "run-hex" => {
            if rest.len() != 1 {
                usage();
            }
            let text = match fs::read_to_string(&rest[0]) {
                Ok(t) => t,
                Err(e) => {
                    eprintln!("read error: {e}");
                    return ExitCode::from(2);
                }
            };
            let outcome = run_hex_text(&text, limits);
            print_outcome(&outcome)
        }
        "smoke" => {
            // movabs rax, 0x2A; store_state rax, [r15 + 0x280]; ret
            //   rax = 0x2A
            //   48 B8 2A 00 00 00 00 00 00 00  (10B)
            //   mov [r15 + 0x280], rax
            //   49 89 87 80 02 00 00           (7B, disp32)
            //   C3                              (1B)
            let bytes: Vec<u8> = vec![
                0x48, 0xB8, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x87, 0x80,
                0x02, 0x00, 0x00, 0xC3,
            ];
            let outcome = run_bytes(&bytes, limits);
            print_outcome(&outcome)
        }
        _ => usage(),
    }
}

fn print_outcome(o: &yoyo_executor::RunOutcome) -> ExitCode {
    println!("steps : {}", o.steps);
    println!("rax   : {:#018x}", o.rax);
    println!("rcx   : {:#018x}", o.rcx);
    println!("r15   : {:#018x}", o.r15);
    match o.exit {
        ExitReason::Halted { rip, steps } => {
            println!("exit  : HALT at {:#x} after {} steps", rip, steps);
            ExitCode::SUCCESS
        }
        ExitReason::Fault(f) => {
            println!("exit  : FAULT {}", f);
            ExitCode::from(1)
        }
    }
}
