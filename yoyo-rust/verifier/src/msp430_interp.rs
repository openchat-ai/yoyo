//! MSP430 interpreter — executes YOYO-emitted TI MSP430 flat binary machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary, entry=0. State at MSP430_STATE_BASE (0x0100), 16-bit LE slots (2 bytes each).

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
const MSP430_STATE_BASE: u16 = 0x0100;
const N_SLOTS: u16 = 16;
const INITIAL_SP: u16 = 0x0400;

struct Cpu {
    regs: [u16; 16], // R0..R15; R1=SP
    pc: u16,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
    call_depth: u32,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        let mut regs = [0u16; 16];
        regs[1] = INITIAL_SP; // SP
        Self { regs, pc: 0, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT, call_depth: 0 }
    }

    fn r(&self, n: usize) -> u16 { self.regs[n & 0xF] }
    fn rw(&mut self, n: usize) -> &mut u16 { &mut self.regs[n & 0xF] }

    fn mem_get(&self, addr: u16) -> u8 {
        let a = addr as usize;
        if a < self.mem.len() { self.mem[a] } else { 0 }
    }
    fn mem_set(&mut self, addr: u16, v: u8) {
        let a = addr as usize;
        if a >= self.mem.len() { self.mem.resize(a + 1, 0); }
        self.mem[a] = v;
    }
    fn load16_le(&self, addr: u16) -> u16 {
        let lo = self.mem_get(addr) as u16;
        let hi = self.mem_get(addr.wrapping_add(1)) as u16;
        lo | (hi << 8)
    }
    fn store16_le(&mut self, addr: u16, val: u16) {
        self.mem_set(addr, (val & 0xFF) as u8);
        self.mem_set(addr.wrapping_add(1), (val >> 8) as u8);
    }

    fn push16(&mut self, v: u16) {
        let sp = self.r(1).wrapping_sub(2);
        *self.rw(1) = sp;
        self.store16_le(sp, v);
    }
    fn pop16(&mut self) -> u16 {
        let sp = self.r(1);
        let v = self.load16_le(sp);
        *self.rw(1) = sp.wrapping_add(2);
        v
    }

    fn fetch_bytes(&self, n: usize) -> Option<Vec<u8>> {
        let pc = self.pc as usize;
        if pc + n > self.mem.len() { return None; }
        Some(self.mem[pc..pc + n].to_vec())
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit {
            return Some(ExecExitReason::StepLimit { steps: self.steps });
        }
        let pc = self.pc;
        if (pc as usize) >= self.mem.len() {
            return Some(ExecExitReason::Halted);
        }
        let Some(b) = self.fetch_bytes(2) else {
            return Some(ExecExitReason::Halted);
        };
        self.steps += 1;

        // NOP: 0x4303 LE → bytes 03 43
        if b[0] == 0x03 && b[1] == 0x43 {
            self.pc = pc.wrapping_add(2);
            return None;
        }
        // RET: 0x4130 LE → bytes 30 41
        if b[0] == 0x30 && b[1] == 0x41 {
            if self.call_depth == 0 {
                return Some(ExecExitReason::Ret);
            }
            self.call_depth -= 1;
            self.pc = self.pop16();
            return None;
        }
        // EXIT / halt word 00 00
        if b[0] == 0x00 && b[1] == 0x00 {
            return Some(ExecExitReason::Ret);
        }

        // MOV #imm16, Rr: B0 43 | rd_word | imm16  (6 bytes)
        // SUB #imm16, Rr: same prefix in this YOYO encoding (treated as MOV for imm form;
        //                 add_imm uses B0 53; platform sub_imm shares B0 43 — decode as MOV)
        if b[0] == 0xB0 && b[1] == 0x43 {
            let Some(ext) = self.fetch_bytes(6) else {
                return Some(ExecExitReason::Fault { msg: format!("truncated MOV#imm at 0x{:04x}", pc) });
            };
            let rd_word = u16::from_le_bytes([ext[2], ext[3]]);
            let imm = u16::from_le_bytes([ext[4], ext[5]]);
            let rd = ((rd_word >> 10) & 0x1F) as usize;
            *self.rw(rd) = imm;
            self.pc = pc.wrapping_add(6);
            return None;
        }
        // ADD #imm16, Rr: B0 53 | rd_word | imm16
        if b[0] == 0xB0 && b[1] == 0x53 {
            let Some(ext) = self.fetch_bytes(6) else {
                return Some(ExecExitReason::Fault { msg: format!("truncated ADD#imm at 0x{:04x}", pc) });
            };
            let rd_word = u16::from_le_bytes([ext[2], ext[3]]);
            let imm = u16::from_le_bytes([ext[4], ext[5]]);
            let rd = ((rd_word >> 10) & 0x1F) as usize;
            *self.rw(rd) = self.r(rd).wrapping_add(imm);
            self.pc = pc.wrapping_add(6);
            return None;
        }
        // MOV Rr, &addr: 80 03 | rd_word | addr16
        if b[0] == 0x80 && b[1] == 0x03 {
            let Some(ext) = self.fetch_bytes(6) else {
                return Some(ExecExitReason::Fault { msg: format!("truncated MOV r,&abs at 0x{:04x}", pc) });
            };
            let rd_word = u16::from_le_bytes([ext[2], ext[3]]);
            let addr = u16::from_le_bytes([ext[4], ext[5]]);
            let rd = ((rd_word >> 10) & 0x1F) as usize;
            self.store16_le(addr, self.r(rd));
            self.pc = pc.wrapping_add(6);
            return None;
        }
        // MOV &addr, Rr: 90 03 | rd_word | addr16
        if b[0] == 0x90 && b[1] == 0x03 {
            let Some(ext) = self.fetch_bytes(6) else {
                return Some(ExecExitReason::Fault { msg: format!("truncated MOV &abs,r at 0x{:04x}", pc) });
            };
            let rd_word = u16::from_le_bytes([ext[2], ext[3]]);
            let addr = u16::from_le_bytes([ext[4], ext[5]]);
            let rd = ((rd_word >> 10) & 0x1F) as usize;
            *self.rw(rd) = self.load16_le(addr);
            self.pc = pc.wrapping_add(6);
            return None;
        }
        // JMP &addr: 40 32 | addr16
        if b[0] == 0x40 && b[1] == 0x32 {
            let Some(ext) = self.fetch_bytes(4) else {
                return Some(ExecExitReason::Fault { msg: format!("truncated JMP at 0x{:04x}", pc) });
            };
            self.pc = u16::from_le_bytes([ext[2], ext[3]]);
            return None;
        }
        // CALL &addr: 40 44 | addr16
        if b[0] == 0x40 && b[1] == 0x44 {
            let Some(ext) = self.fetch_bytes(4) else {
                return Some(ExecExitReason::Fault { msg: format!("truncated CALL at 0x{:04x}", pc) });
            };
            let target = u16::from_le_bytes([ext[2], ext[3]]);
            self.push16(pc.wrapping_add(4));
            self.call_depth += 1;
            self.pc = target;
            return None;
        }

        // 2-byte ops emitted as [hi, lo] of the logical word (big-endian layout)
        let w = u16::from_be_bytes([b[0], b[1]]);
        let rd = ((w >> 10) & 0x1F) as usize;
        let rs = ((w >> 5) & 0x1F) as usize;
        let op_lo = w & 0xF01F;

        // INC: 0x0034 | (r<<10)
        if (w & !0x7C00) == 0x0034 {
            *self.rw(rd) = self.r(rd).wrapping_add(1);
            self.pc = pc.wrapping_add(2);
            return None;
        }
        // DEC: 0x0033 | (r<<10)
        if (w & !0x7C00) == 0x0033 {
            *self.rw(rd) = self.r(rd).wrapping_sub(1);
            self.pc = pc.wrapping_add(2);
            return None;
        }
        match op_lo {
            0x8010 => { // ADD Rs, Rd
                *self.rw(rd) = self.r(rd).wrapping_add(self.r(rs));
                self.pc = pc.wrapping_add(2);
                None
            }
            0x0010 => { // SUB Rs, Rd
                *self.rw(rd) = self.r(rd).wrapping_sub(self.r(rs));
                self.pc = pc.wrapping_add(2);
                None
            }
            0x8050 => { // OR Rs, Rd
                *self.rw(rd) = self.r(rd) | self.r(rs);
                self.pc = pc.wrapping_add(2);
                None
            }
            0x8000 => { // CMP Rs, Rd (flags ignored for DDC)
                let _ = self.r(rd).wrapping_sub(self.r(rs));
                self.pc = pc.wrapping_add(2);
                None
            }
            0x0007 => { // MUL Rs, Rd
                let prod = (self.r(rd) as u32).wrapping_mul(self.r(rs) as u32) as u16;
                *self.rw(rd) = prod;
                self.pc = pc.wrapping_add(2);
                None
            }
            _ => Some(ExecExitReason::Fault {
                msg: format!("undecoded MSP430 insn at 0x{:04x}: {:02x} {:02x}", pc, b[0], b[1]),
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

/// Run a flat MSP430 binary. Bytes loaded at 0x0000, PC=0.
pub fn run_msp430(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x2000];
    let n = bytes.len().min(mem.len());
    mem[..n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    for slot in 0..N_SLOTS {
        let addr = MSP430_STATE_BASE + slot * 2;
        let val = cpu.load16_le(addr) as u64;
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
