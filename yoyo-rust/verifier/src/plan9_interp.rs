//! Plan 9 / Acadia flat x64 interpreter — executes YOYO-emitted Plan9 flat binary
//! (defaults to x86-64 NOP/RET via PlatformBackend) for DDC against the TIR simulator.
//!
//! Flat binary, entry=0. NOP=0x90, RET=0xC3, HLT=0xF4 treated as exit.
//! Plan9Platform inherits x64 emit_set/get; slots via R15-relative state are not
//! decoded here beyond nop_ret (sufficient for current DDC fixture).

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecExitReason { Ret, Halted, StepLimit { steps: u64 }, Fault { msg: String } }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecResult {
    pub exit_reason: ExecExitReason,
    pub steps: u64,
    pub state: HashMap<u16, u64>,
}

const DEFAULT_STEP_LIMIT: u64 = 1_000_000;
const INITIAL_SP: u64 = 0x8000;

struct Cpu {
    rip: u64,
    rsp: u64,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
    call_depth: u32,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        Self {
            rip: 0, rsp: INITIAL_SP, mem,
            steps: 0, step_limit: DEFAULT_STEP_LIMIT, call_depth: 0,
        }
    }

    fn mem_get(&self, addr: u64) -> u8 {
        let a = addr as usize;
        if a < self.mem.len() { self.mem[a] } else { 0 }
    }
    fn mem_set(&mut self, addr: u64, v: u8) {
        let a = addr as usize;
        if a >= self.mem.len() { self.mem.resize(a + 1, 0); }
        self.mem[a] = v;
    }
    fn load64_le(&self, addr: u64) -> u64 {
        let mut v = 0u64;
        for i in 0..8u64 {
            v |= (self.mem_get(addr.wrapping_add(i)) as u64) << (i * 8);
        }
        v
    }
    fn store64_le(&mut self, addr: u64, val: u64) {
        for i in 0..8u64 {
            self.mem_set(addr.wrapping_add(i), ((val >> (i * 8)) & 0xFF) as u8);
        }
    }
    fn push64(&mut self, v: u64) {
        self.rsp = self.rsp.wrapping_sub(8);
        self.store64_le(self.rsp, v);
    }
    fn pop64(&mut self) -> u64 {
        let v = self.load64_le(self.rsp);
        self.rsp = self.rsp.wrapping_add(8);
        v
    }

    fn fetch8(&mut self) -> Option<u8> {
        if (self.rip as usize) >= self.mem.len() { return None; }
        let v = self.mem_get(self.rip);
        self.rip = self.rip.wrapping_add(1);
        Some(v)
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit {
            return Some(ExecExitReason::StepLimit { steps: self.steps });
        }
        if (self.rip as usize) >= self.mem.len() {
            return Some(ExecExitReason::Halted);
        }
        self.steps += 1;
        let Some(op) = self.fetch8() else {
            return Some(ExecExitReason::Halted);
        };

        match op {
            0x90 => None, // NOP
            0xC3 => { // RET
                if self.call_depth == 0 || self.rsp == INITIAL_SP {
                    return Some(ExecExitReason::Ret);
                }
                self.call_depth -= 1;
                self.rip = self.pop64();
                None
            }
            0xF4 => Some(ExecExitReason::Ret), // HLT → exit
            0xE8 => { // CALL rel32
                let mut disp = 0u32;
                for i in 0..4u32 {
                    let Some(b) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated CALL".into() });
                    };
                    disp |= (b as u32) << (i * 8);
                }
                let next = self.rip;
                self.push64(next);
                self.call_depth += 1;
                self.rip = next.wrapping_add(disp as i32 as i64 as u64);
                None
            }
            0xE9 => { // JMP rel32
                let mut disp = 0u32;
                for i in 0..4u32 {
                    let Some(b) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated JMP".into() });
                    };
                    disp |= (b as u32) << (i * 8);
                }
                let next = self.rip;
                self.rip = next.wrapping_add(disp as i32 as i64 as u64);
                None
            }
            _ => Some(ExecExitReason::Fault {
                msg: format!("undecoded Plan9/x64 insn at 0x{:04x}: {:02x}", self.rip.wrapping_sub(1), op),
            }),
        }
    }

    fn run(&mut self) -> ExecExitReason {
        loop {
            match self.step() {
                Some(r) => return r,
                None => continue,
            }
        }
    }
}

/// Run a flat Plan9 (x64) binary. Bytes loaded at 0x0000, RIP=0.
pub fn run_plan9(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x10000];
    let n = bytes.len().min(mem.len());
    mem[..n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    let exit_reason = cpu.run();

    // Plan9 inherits x64 state layout; nop_ret leaves all slots zero.
    ExecResult { exit_reason, steps: cpu.steps, state: HashMap::new() }
}
