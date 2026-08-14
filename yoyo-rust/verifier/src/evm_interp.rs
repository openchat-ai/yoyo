//! EVM interpreter — executes YOYO-emitted Ethereum EVM flat bytecode
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary, entry=0. JUMPDEST(0x5B)=NOP, STOP(0x00)=Ret.
//! Slots live in memory at slot*0x20 (32-byte words), matching EvmPlatform.

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
const N_SLOTS: u16 = 16;
const MEM_SIZE: usize = 0x10000;

struct Cpu {
    pc: usize,
    stack: Vec<[u8; 32]>,
    mem: Vec<u8>,
    code: Vec<u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(code: Vec<u8>) -> Self {
        Self {
            pc: 0,
            stack: Vec::new(),
            mem: vec![0u8; MEM_SIZE],
            code,
            steps: 0,
            step_limit: DEFAULT_STEP_LIMIT,
        }
    }

    fn push_word(&mut self, w: [u8; 32]) {
        self.stack.push(w);
    }
    fn pop_word(&mut self) -> Option<[u8; 32]> {
        self.stack.pop()
    }

    fn word_from_u64(v: u64) -> [u8; 32] {
        let mut w = [0u8; 32];
        w[24..32].copy_from_slice(&v.to_be_bytes());
        w
    }
    fn word_to_u64(w: &[u8; 32]) -> u64 {
        let mut b = [0u8; 8];
        b.copy_from_slice(&w[24..32]);
        u64::from_be_bytes(b)
    }
    fn word_to_usize(w: &[u8; 32]) -> usize {
        Self::word_to_u64(w) as usize
    }

    fn mstore(&mut self, addr: usize, w: &[u8; 32]) {
        if addr + 32 > self.mem.len() {
            self.mem.resize(addr + 32, 0);
        }
        self.mem[addr..addr + 32].copy_from_slice(w);
    }
    fn mload(&self, addr: usize) -> [u8; 32] {
        let mut w = [0u8; 32];
        for i in 0..32 {
            let a = addr + i;
            w[i] = if a < self.mem.len() { self.mem[a] } else { 0 };
        }
        w
    }

    fn binop_u64(&mut self, f: fn(u64, u64) -> u64) -> Option<()> {
        let b = self.pop_word()?;
        let a = self.pop_word()?;
        let r = f(Self::word_to_u64(&a), Self::word_to_u64(&b));
        self.push_word(Self::word_from_u64(r));
        Some(())
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit {
            return Some(ExecExitReason::StepLimit { steps: self.steps });
        }
        if self.pc >= self.code.len() {
            return Some(ExecExitReason::Halted);
        }
        let op = self.code[self.pc];
        self.steps += 1;
        self.pc += 1;

        match op {
            0x00 => return Some(ExecExitReason::Ret), // STOP
            0x5B => return None, // JUMPDEST used as NOP
            0x01 => { // ADD
                if self.binop_u64(u64::wrapping_add).is_none() {
                    return Some(ExecExitReason::Fault { msg: "ADD stack underflow".into() });
                }
                None
            }
            0x02 => { // MUL
                if self.binop_u64(u64::wrapping_mul).is_none() {
                    return Some(ExecExitReason::Fault { msg: "MUL stack underflow".into() });
                }
                None
            }
            0x03 => { // SUB
                if self.binop_u64(u64::wrapping_sub).is_none() {
                    return Some(ExecExitReason::Fault { msg: "SUB stack underflow".into() });
                }
                None
            }
            0x14 => { // EQ
                let Some(b) = self.pop_word() else {
                    return Some(ExecExitReason::Fault { msg: "EQ stack underflow".into() });
                };
                let Some(a) = self.pop_word() else {
                    return Some(ExecExitReason::Fault { msg: "EQ stack underflow".into() });
                };
                let eq = if a == b { 1u64 } else { 0u64 };
                self.push_word(Self::word_from_u64(eq));
                None
            }
            0x17 => { // OR
                if self.binop_u64(|a, b| a | b).is_none() {
                    return Some(ExecExitReason::Fault { msg: "OR stack underflow".into() });
                }
                None
            }
            0x51 => { // MLOAD
                let Some(addr_w) = self.pop_word() else {
                    return Some(ExecExitReason::Fault { msg: "MLOAD stack underflow".into() });
                };
                let addr = Self::word_to_usize(&addr_w);
                let w = self.mload(addr);
                self.push_word(w);
                None
            }
            0x52 => { // MSTORE
                let Some(addr_w) = self.pop_word() else {
                    return Some(ExecExitReason::Fault { msg: "MSTORE stack underflow".into() });
                };
                let Some(val) = self.pop_word() else {
                    return Some(ExecExitReason::Fault { msg: "MSTORE stack underflow".into() });
                };
                let addr = Self::word_to_usize(&addr_w);
                self.mstore(addr, &val);
                None
            }
            0x56 => { // JUMP
                let Some(dest_w) = self.pop_word() else {
                    return Some(ExecExitReason::Fault { msg: "JUMP stack underflow".into() });
                };
                self.pc = Self::word_to_usize(&dest_w);
                None
            }
            0x57 => { // JUMPI
                let Some(dest_w) = self.pop_word() else {
                    return Some(ExecExitReason::Fault { msg: "JUMPI stack underflow".into() });
                };
                let Some(cond_w) = self.pop_word() else {
                    return Some(ExecExitReason::Fault { msg: "JUMPI stack underflow".into() });
                };
                if Self::word_to_u64(&cond_w) != 0 {
                    self.pc = Self::word_to_usize(&dest_w);
                }
                None
            }
            // PUSH1..PUSH32
            b if (0x60..=0x7F).contains(&b) => {
                let n = (b - 0x5F) as usize;
                if self.pc + n > self.code.len() {
                    return Some(ExecExitReason::Fault {
                        msg: format!("truncated PUSH{} at 0x{:04x}", n, self.pc.saturating_sub(1)),
                    });
                }
                let mut w = [0u8; 32];
                let src = &self.code[self.pc..self.pc + n];
                w[32 - n..].copy_from_slice(src);
                self.pc += n;
                self.push_word(w);
                None
            }
            _ => Some(ExecExitReason::Fault {
                msg: format!("undecoded EVM opcode at 0x{:04x}: {:02x}", self.pc.saturating_sub(1), op),
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

/// Run flat EVM bytecode. PC=0.
pub fn run_evm(bytes: &[u8]) -> ExecResult {
    let mut cpu = Cpu::new(bytes.to_vec());
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    for slot in 0..N_SLOTS {
        let addr = (slot as usize).wrapping_mul(0x20);
        let w = cpu.mload(addr);
        let val = Cpu::word_to_u64(&w);
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
