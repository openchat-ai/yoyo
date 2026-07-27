//! yoyo_executor — W-START attempt-N5b (EXPERIMENTAL · NON-GREEN).
//!
//! Public surface: `run_bytes` and `run_hex_text`. The crate is a
//! *sibling* of `verifier/`; it does **not** touch the verifier's
//! `executor::compile_ty_source` (which is compile-time only) nor
//! `libyoyo`'s syscall ABI.
//!
//! `EXPERIMENTAL` tag is fixed and the crate is not added to the
//! workspace's `members` list — it is built explicitly via
//! `--manifest-path yoyo-rust/executor/Cargo.toml` (per
//! `scripts/_probe/_attempt_n5b/run.sh`).

pub mod cpu;
pub mod mmu;

use cpu::Cpu;
use mmu::{Fault, Mmu};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RunLimits {
    pub steps: u64,
    pub mmu_capacity: usize,
}

impl Default for RunLimits {
    fn default() -> Self {
        Self {
            steps: 10_000,
            mmu_capacity: 4096,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitReason {
    Halted { rip: u64, steps: u64 },
    Fault(Fault),
}

#[derive(Debug, Clone, Copy)]
pub struct RunOutcome {
    pub exit: ExitReason,
    pub rax: u64,
    pub rcx: u64,
    pub r15: u64,
    pub steps: u64,
}

pub fn run_bytes(code: &[u8], limits: RunLimits) -> RunOutcome {
    let mut mmu = Mmu::new(limits.mmu_capacity);
    mmu.base = 0x1000;
    // Lay the code out starting at `mmu.base`.
    let dst_start = (mmu.base as usize) - (mmu.base as usize); // 0 in mmu.bytes
    // Copy into mmu.bytes[0..code.len()]
    let n = code.len().min(mmu.bytes.len());
    mmu.bytes[..n].copy_from_slice(&code[..n]);
    let _ = dst_start;

    let mut cpu = Cpu::new(mmu.base, mmu.base, limits.steps);
    let exit = match cpu.run(&mut mmu) {
        Ok(()) => ExitReason::Halted {
            rip: cpu.rip,
            steps: cpu.steps,
        },
        Err(f) => ExitReason::Fault(f),
    };
    RunOutcome {
        exit,
        rax: cpu.rax,
        rcx: cpu.rcx,
        r15: cpu.r15,
        steps: cpu.steps,
    }
}

pub fn run_hex_text(hex: &str, limits: RunLimits) -> RunOutcome {
    let cleaned: String = hex.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = match hex_decode(&cleaned) {
        Ok(b) => b,
        Err(_) => {
            return RunOutcome {
                exit: ExitReason::Fault(Fault::Decode {
                    rip: 0,
                    reason: "hex-decode-failed",
                }),
                rax: 0,
                rcx: 0,
                r15: 0,
                steps: 0,
            };
        }
    };
    run_bytes(&bytes, limits)
}

fn hex_decode(s: &str) -> Result<Vec<u8>, ()> {
    if s.len() % 2 != 0 {
        return Err(());
    }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let h = hex_nibble(bytes[i])?;
        let l = hex_nibble(bytes[i + 1])?;
        out.push((h << 4) | l);
        i += 2;
    }
    Ok(out)
}

fn hex_nibble(c: u8) -> Result<u8, ()> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(()),
    }
}
