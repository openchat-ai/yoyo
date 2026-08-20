//! PIC16 mid-range interpreter — executes YOYO-emitted PIC flat binary machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary, entry=0. Instructions are 2-byte LE words matching PicPlatform helpers.
//! NOP=0x0000, RET/exit=0x0004 (YOYO emit_ret). State at PIC_STATE_BASE (0x0100), 8-bit slots.

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
const PIC_STATE_BASE: u16 = 0x0100;
const N_SLOTS: u16 = 16;

struct Cpu {
    w: u8,
    pc: u16,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
    call_depth: u32,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        Self { w: 0, pc: 0, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT, call_depth: 0 }
    }

    fn mem_get(&self, addr: u16) -> u8 {
        let a = addr as usize;
        if a < self.mem.len() { self.mem[a] } else { 0 }
    }
    fn mem_set(&mut self, addr: u16, v: u8) {
        let a = addr as usize;
        if a >= self.mem.len() { self.mem.resize(a + 1, 0); }
        self.mem[a] = v;
    }

    fn fetch_word(&self) -> Option<u16> {
        let pc = self.pc as usize;
        if pc + 2 > self.mem.len() { return None; }
        Some(u16::from_le_bytes([self.mem[pc], self.mem[pc + 1]]))
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit {
            return Some(ExecExitReason::StepLimit { steps: self.steps });
        }
        let pc = self.pc;
        if (pc as usize) >= self.mem.len() {
            return Some(ExecExitReason::Halted);
        }
        let Some(word) = self.fetch_word() else {
            return Some(ExecExitReason::Halted);
        };
        self.steps += 1;

        // NOP: 0x0000
        if word == 0x0000 {
            self.pc = pc.wrapping_add(2);
            return None;
        }
        // RET / CLRWDT placeholder used as exit by PicPlatform::emit_ret: 0x0004
        if word == 0x0004 {
            if self.call_depth == 0 {
                return Some(ExecExitReason::Ret);
            }
            self.call_depth -= 1;
            // No real stack in this YOYO encoding — treat nested ret as advance
            self.pc = pc.wrapping_add(2);
            return None;
        }
        // EXIT: 0x00FD as LE word → bytes FD 00
        if word == 0x00FD {
            return Some(ExecExitReason::Ret);
        }

        // YOYO PIC encoding: hi=op tag, lo=operand (see PicPlatform helpers)
        let lo = (word & 0xFF) as u8;
        let hi = (word >> 8) as u8;
        let addr = PIC_STATE_BASE.wrapping_add(lo as u16);
        match hi {
            0x00 => { // MOVLW
                self.w = lo;
                self.pc = pc.wrapping_add(2);
                return None;
            }
            0x01 => { // MOVWF
                self.mem_set(addr, self.w);
                self.pc = pc.wrapping_add(2);
                return None;
            }
            0x02 => { // MOVF
                self.w = self.mem_get(addr);
                self.pc = pc.wrapping_add(2);
                return None;
            }
            0x03 => { // ADDWF → mem[addr] += W
                let v = self.mem_get(addr).wrapping_add(self.w);
                self.mem_set(addr, v);
                self.pc = pc.wrapping_add(2);
                return None;
            }
            0x04 => { // SUBWF → mem[addr] -= W
                let v = self.mem_get(addr).wrapping_sub(self.w);
                self.mem_set(addr, v);
                self.pc = pc.wrapping_add(2);
                return None;
            }
            0x05 => { // ORWF
                let v = self.mem_get(addr) | self.w;
                self.mem_set(addr, v);
                self.pc = pc.wrapping_add(2);
                return None;
            }
            0x06 => { // INCF
                let v = self.mem_get(addr).wrapping_add(1);
                self.mem_set(addr, v);
                self.pc = pc.wrapping_add(2);
                return None;
            }
            0x07 => { // DECF
                let v = self.mem_get(addr).wrapping_sub(1);
                self.mem_set(addr, v);
                self.pc = pc.wrapping_add(2);
                return None;
            }
            _ => {}
        }

        Some(ExecExitReason::Fault {
            msg: format!("undecoded PIC insn at 0x{:04x}: {:04x}", pc, word),
        })
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

/// Run a flat PIC binary. Bytes loaded at 0x0000, PC=0.
pub fn run_pic(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x2000];
    let n = bytes.len().min(mem.len());
    mem[..n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    for slot in 0..N_SLOTS {
        let addr = PIC_STATE_BASE + slot;
        let val = cpu.mem_get(addr) as u64;
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
