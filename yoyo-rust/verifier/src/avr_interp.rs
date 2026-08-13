//! AVR interpreter — executes YOYO-emitted AVR flat binary machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary, entry=0. State is at AVR_SRAM_BASE (0x0100), 16-bit slots (2 bytes each).
//! Instructions are 2 bytes LE (one word), except LDS/STS which are 4 bytes (2 words).

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
const AVR_SRAM_BASE: u16 = 0x0100;
const N_SLOTS: u16 = 16;

struct Cpu {
    regs: [u16; 32],
    pc: u16,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        Self { regs: [0; 32], pc: 0, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT }
    }

    fn r(&self, n: usize) -> u16 { self.regs[n] }
    fn rw(&mut self, n: usize) -> &mut u16 { &mut self.regs[n] }

    fn mem_get(&self, addr: u16) -> u8 {
        let a = addr as usize;
        if a < self.mem.len() { self.mem[a] } else { 0 }
    }
    fn mem_set(&mut self, addr: u16, v: u8) {
        let a = addr as usize;
        if a >= self.mem.len() { self.mem.resize(a + 1, 0); }
        self.mem[a] = v;
    }

    fn load16(&self, addr: u16) -> u16 {
        let lo = self.mem_get(addr) as u16;
        let hi = self.mem_get(addr.wrapping_add(1)) as u16;
        lo | (hi << 8)
    }

    fn store16(&mut self, addr: u16, val: u16) {
        self.mem_set(addr, (val & 0xFF) as u8);
        self.mem_set(addr.wrapping_add(1), (val >> 8) as u8);
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        let pc = self.pc;
        let addr = pc as usize * 2; // AVR is word-addressed, each word = 2 bytes
        if addr + 2 > self.mem.len() { return Some(ExecExitReason::Halted); }

        let insn = u16::from_le_bytes(self.mem[addr..addr + 2].try_into().unwrap());
        self.steps += 1;

        // Decode fields
        let rd = ((insn >> 4) & 0x1F) as usize;
        let rr = (insn & 0x1F) as usize;
        let op_hi = insn >> 12;
        let op_mid = (insn >> 8) & 0x0F;

        // NOP: 0x0000
        if insn == 0x0000 {
            self.pc = pc.wrapping_add(1);
            return None;
        }

        // RET: 0x9508
        if insn == 0x9508 {
            // Pop PC (2 bytes from stack)
            let sp_lo = self.mem_get(0x5D + 0x100) as u16; // SPL = 0x5D + 0x100 offset
            // Actually AVR stack is at 0x0100+ (SRAM), but we don't track SP precisely.
            // For simple RET, check if we're at top level.
            // For emitted code, RET is the exit instruction.
            // The emitted code uses: NOP, ..., RET. No nested calls.
            return Some(ExecExitReason::Ret);
        }

        // SLEEP: 0x9588
        if insn == 0x9588 {
            self.pc = pc.wrapping_add(1);
            return None;
        }

        // LDI rd, k: 0xE000 | (rd<<4) | k (rd in 16-31)
        if op_hi == 0xE && rd >= 16 {
            let k = (insn & 0xFF) as u8;
            *self.rw(rd) = k as u16;
            self.pc = pc.wrapping_add(1);
            return None;
        }

        // SBR r, k: 0xE000 | (r<<4) | k (same as LDI but for r0-r7)
        if op_hi == 0xE && rd < 8 {
            let k = (insn & 0xFF) as u8;
            *self.rw(rd) = k as u16;
            self.pc = pc.wrapping_add(1);
            return None;
        }

        // MOV rd, rr: 0x2C00 | ((rd&0x1F)<<4) | (rr&0x1F) | ((rd&0x10)<<5) | ((rr&0x10)<<4)
        if op_hi == 0x2 && op_mid == 0xC {
            *self.rw(rd) = self.r(rr);
            self.pc = pc.wrapping_add(1);
            return None;
        }

        // ADD rd, rr: 0x0C00 | (rd<<4) | rr | 0x0E
        if (insn & 0xFC0F) == 0x0C0E {
            let rn = (insn >> 4) & 0x1F;
            let rm = insn & 0x1F;
            *self.rw(rn as usize) = self.r(rn as usize).wrapping_add(self.r(rm as usize));
            self.pc = pc.wrapping_add(1);
            return None;
        }

        // SUB rd, rr: 0x0C00 | (rd<<4) | rr | 0x06
        if (insn & 0xFC0F) == 0x0C06 {
            let rn = (insn >> 4) & 0x1F;
            let rm = insn & 0x1F;
            *self.rw(rn as usize) = self.r(rn as usize).wrapping_sub(self.r(rm as usize));
            self.pc = pc.wrapping_add(1);
            return None;
        }

        // OR rd, rr: 0x0C00 | (rd<<4) | rr | 0x02
        if (insn & 0xFC0F) == 0x0C02 {
            let rn = (insn >> 4) & 0x1F;
            let rm = insn & 0x1F;
            *self.rw(rn as usize) = self.r(rn as usize) | self.r(rm as usize);
            self.pc = pc.wrapping_add(1);
            return None;
        }

        // INC r: 0x9C00 | r
        if (insn & 0xFE0F) == 0x9C00 {
            let rn = (insn >> 4) & 0x1F;
            // Check if it's really INC (bit 3 = 0) vs DEC (bit 3 = 1)
            // 0x9C00 = 1001 1100 0000 0000 — mask op_hi=9, op_mid=C
            if (insn & 0x0008) == 0 {
                *self.rw(rn as usize) = self.r(rn as usize).wrapping_add(1);
                self.pc = pc.wrapping_add(1);
                return None;
            }
        }

        // DEC r: 0x9C00 | r | 0x08
        if (insn & 0xFE0F) == 0x9C08 {
            let rn = (insn >> 4) & 0x1F;
            *self.rw(rn as usize) = self.r(rn as usize).wrapping_sub(1);
            self.pc = pc.wrapping_add(1);
            return None;
        }

        // CP rd, rr: 0x0C00 | (rd<<4) | rr | 0x0A
        if (insn & 0xFC0F) == 0x0C0A {
            let rn = (insn >> 4) & 0x1F;
            let rm = insn & 0x1F;
            let _result = self.r(rn as usize).wrapping_sub(self.r(rm as usize));
            self.pc = pc.wrapping_add(1);
            return None;
        }

        // RJMP disp: 0xC000 | (disp & 0xFFF)
        if op_hi == 0xC {
            let disp12 = (insn & 0xFFF) as u16;
            let disp = if disp12 & 0x800 != 0 { disp12 | 0xF000 } else { disp12 };
            self.pc = (pc as i16).wrapping_add(disp as i16).wrapping_add(1) as u16;
            return None;
        }

        // RCALL disp: 0xD000 | (disp & 0xFFF)
        if op_hi == 0xD {
            let disp12 = (insn & 0xFFF) as u16;
            let disp = if disp12 & 0x800 != 0 { disp12 | 0xF000 } else { disp12 };
            let ret_pc = pc.wrapping_add(1);
            // Push ret_pc (2 bytes, little-endian) onto stack
            self.mem_set(0x100 + 0x5E, (ret_pc >> 8) as u8); // STACK_HI
            self.mem_set(0x100 + 0x5D, (ret_pc & 0xFF) as u8); // STACK_LO
            self.pc = (pc as i16).wrapping_add(disp as i16).wrapping_add(1) as u16;
            return None;
        }

        // LDS rd, addr: 0x9000 | (rd<<4) | 0x0000 + 2 more bytes for addr16
        if insn == (0x9000 | ((rd as u16) << 4)) && addr + 4 <= self.mem.len() {
            let addr16 = u16::from_le_bytes(self.mem[addr + 2..addr + 4].try_into().unwrap());
            *self.rw(rd) = self.load16(addr16);
            self.pc = pc.wrapping_add(2);
            return None;
        }

        // STS addr, rr: 0x9200 | ((rr as u16) << 4) + 2 more bytes for addr16
        if insn == (0x9200 | ((rr as u16) << 4)) && addr + 4 <= self.mem.len() {
            let addr16 = u16::from_le_bytes(self.mem[addr + 2..addr + 4].try_into().unwrap());
            self.store16(addr16, self.r(rr));
            self.pc = pc.wrapping_add(2);
            return None;
        }

        Some(ExecExitReason::Fault { msg: format!("undecoded AVR insn at word 0x{:04x}: 0x{:04x}", pc, insn) })
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

/// Run a flat AVR binary. Bytes are loaded into flash at 0x0000, PC=0.
pub fn run_avr(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x2000]; // 8KB SRAM
    // Load code into the flash address space (word-addressed at 0x0000)
    let n = bytes.len().min(0x1000);
    mem[..n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    for slot in 0..N_SLOTS {
        let addr = AVR_SRAM_BASE + slot * 2;
        let val = cpu.load16(addr) as u64;
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}