//! MOS 6502 interpreter — executes YOYO-emitted 6502 flat binary machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary, entry=0. State is at M6502_STATE_BASE (0x0200), 16-bit LE slots (2 bytes each).

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
const M6502_STATE_BASE: u16 = 0x0200;
const N_SLOTS: u16 = 16;
const INITIAL_SP: u8 = 0xFF;

struct Cpu {
    a: u8,
    x: u8,
    y: u8,
    sp: u8,
    pc: u16,
    p: u8, // NV-BDIZC; bit 0 = C
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        Self {
            a: 0, x: 0, y: 0, sp: INITIAL_SP, pc: 0, p: 0x20,
            mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT,
        }
    }

    fn carry(&self) -> u8 { self.p & 1 }
    fn set_carry(&mut self, c: bool) {
        if c { self.p |= 1; } else { self.p &= !1; }
    }
    fn set_zn(&mut self, v: u8) {
        if v == 0 { self.p |= 0x02; } else { self.p &= !0x02; }
        if v & 0x80 != 0 { self.p |= 0x80; } else { self.p &= !0x80; }
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
    fn load16_le(&self, addr: u16) -> u16 {
        let lo = self.mem_get(addr) as u16;
        let hi = self.mem_get(addr.wrapping_add(1)) as u16;
        lo | (hi << 8)
    }

    fn push8(&mut self, v: u8) {
        self.mem_set(0x0100 | self.sp as u16, v);
        self.sp = self.sp.wrapping_sub(1);
    }
    fn pop8(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.mem_get(0x0100 | self.sp as u16)
    }

    fn fetch8(&mut self) -> u8 {
        let v = self.mem_get(self.pc);
        self.pc = self.pc.wrapping_add(1);
        v
    }
    fn fetch16_le(&mut self) -> u16 {
        let lo = self.fetch8() as u16;
        let hi = self.fetch8() as u16;
        lo | (hi << 8)
    }

    fn adc(&mut self, imm: u8) {
        let sum = (self.a as u16) + (imm as u16) + (self.carry() as u16);
        self.set_carry(sum > 0xFF);
        self.a = (sum & 0xFF) as u8;
        self.set_zn(self.a);
    }
    fn sbc(&mut self, imm: u8) {
        // SBC with borrow: A - imm - (1-C)
        let diff = (self.a as i16) - (imm as i16) - (1 - self.carry() as i16);
        self.set_carry(diff >= 0);
        self.a = diff as u8;
        self.set_zn(self.a);
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit {
            return Some(ExecExitReason::StepLimit { steps: self.steps });
        }
        let pc = self.pc;
        if (pc as usize) >= self.mem.len() {
            return Some(ExecExitReason::Halted);
        }

        let opcode = self.fetch8();
        self.steps += 1;

        match opcode {
            0xEA => None, // NOP
            0x60 => { // RTS
                if self.sp == INITIAL_SP {
                    return Some(ExecExitReason::Ret);
                }
                let lo = self.pop8() as u16;
                let hi = self.pop8() as u16;
                // 6502 RTS adds 1 to the popped address
                self.pc = (lo | (hi << 8)).wrapping_add(1);
                None
            }
            0x00 => { // BRK — treat as halt/exit
                Some(ExecExitReason::Ret)
            }
            0xA9 => { // LDA #imm
                self.a = self.fetch8();
                self.set_zn(self.a);
                None
            }
            0xAD => { // LDA abs
                let addr = self.fetch16_le();
                self.a = self.mem_get(addr);
                self.set_zn(self.a);
                None
            }
            0x8D => { // STA abs
                let addr = self.fetch16_le();
                self.mem_set(addr, self.a);
                None
            }
            0x4C => { // JMP abs
                self.pc = self.fetch16_le();
                None
            }
            0x20 => { // JSR abs
                let target = self.fetch16_le();
                let ret = self.pc.wrapping_sub(1); // 6502 pushes PC-1
                self.push8((ret >> 8) as u8);
                self.push8((ret & 0xFF) as u8);
                self.pc = target;
                None
            }
            0x18 => { self.set_carry(false); None } // CLC
            0x38 => { self.set_carry(true); None }  // SEC
            0x69 => { // ADC #imm
                let imm = self.fetch8();
                self.adc(imm);
                None
            }
            0xE9 => { // SBC #imm
                let imm = self.fetch8();
                self.sbc(imm);
                None
            }
            0xA2 => { // LDX #imm
                self.x = self.fetch8();
                self.set_zn(self.x);
                None
            }
            0xA0 => { // LDY #imm
                self.y = self.fetch8();
                self.set_zn(self.y);
                None
            }
            0xA8 => { self.y = self.a; self.set_zn(self.y); None } // TAY
            0xCA => { self.x = self.x.wrapping_sub(1); self.set_zn(self.x); None } // DEX
            0xC8 => { self.y = self.y.wrapping_add(1); self.set_zn(self.y); None } // INY
            0x85 => { // STA zp
                let zp = self.fetch8() as u16;
                self.mem_set(zp, self.a);
                None
            }
            0x86 => { // STX zp
                let zp = self.fetch8() as u16;
                self.mem_set(zp, self.x);
                None
            }
            0xA5 => { // LDA zp
                let zp = self.fetch8() as u16;
                self.a = self.mem_get(zp);
                self.set_zn(self.a);
                None
            }
            0xA6 => { // LDX zp
                let zp = self.fetch8() as u16;
                self.x = self.mem_get(zp);
                self.set_zn(self.x);
                None
            }
            0xA4 => { // LDY zp
                let zp = self.fetch8() as u16;
                self.y = self.mem_get(zp);
                self.set_zn(self.y);
                None
            }
            0x65 => { // ADC zp
                let zp = self.fetch8() as u16;
                let v = self.mem_get(zp);
                self.adc(v);
                None
            }
            0x6D => { // ADC abs
                let addr = self.fetch16_le();
                let v = self.mem_get(addr);
                self.adc(v);
                None
            }
            0x29 => { // AND #imm
                self.a &= self.fetch8();
                self.set_zn(self.a);
                None
            }
            0x49 => { // EOR #imm
                self.a ^= self.fetch8();
                self.set_zn(self.a);
                None
            }
            0x09 => { // ORA #imm
                self.a |= self.fetch8();
                self.set_zn(self.a);
                None
            }
            0xC9 => { // CMP #imm
                let imm = self.fetch8();
                let (r, b) = self.a.overflowing_sub(imm);
                self.set_carry(!b);
                self.set_zn(r);
                None
            }
            0xE0 => { // CPX #imm
                let imm = self.fetch8();
                let (r, b) = self.x.overflowing_sub(imm);
                self.set_carry(!b);
                self.set_zn(r);
                None
            }
            0xC6 => { // DEC zp
                let zp = self.fetch8() as u16;
                let v = self.mem_get(zp).wrapping_sub(1);
                self.mem_set(zp, v);
                self.set_zn(v);
                None
            }
            0xF0 | 0xD0 => { // BEQ / BNE rel — always take for platform stubs
                let rel = self.fetch8() as i8 as i16;
                self.pc = (self.pc as i16).wrapping_add(rel) as u16;
                None
            }
            _ => {
                Some(ExecExitReason::Fault {
                    msg: format!("undecoded 6502 opcode 0x{:02x} at pc=0x{:04x}", opcode, pc),
                })
            }
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

/// Run a flat 6502 binary. Bytes are loaded at 0x0000, PC=0, SP=0xFF.
pub fn run_m6502(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x10000];
    let n = bytes.len().min(0x10000);
    mem[..n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    for slot in 0..N_SLOTS {
        let addr = M6502_STATE_BASE + slot * 2;
        let val = cpu.load16_le(addr) as u64;
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
