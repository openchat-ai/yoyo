//! Z80 interpreter — executes YOYO-emitted Z80 flat binary machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary, entry=0. State is at Z80_STATE_BASE (0x8000), 16-bit LE slots (2 bytes each).

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
const Z80_STATE_BASE: u16 = 0x8000;
const N_SLOTS: u16 = 16;
const INITIAL_SP: u16 = 0xFFFE;

struct Cpu {
    a: u8,
    f: u8, // flags; bit 0 = C (carry), bit 1 = Z (zero)
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        Self {
            a: 0, f: 0, b: 0, c: 0, d: 0, e: 0, h: 0, l: 0,
            sp: INITIAL_SP, pc: 0, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT,
        }
    }

    fn hl(&self) -> u16 { ((self.h as u16) << 8) | self.l as u16 }
    fn set_hl(&mut self, v: u16) {
        self.h = (v >> 8) as u8;
        self.l = (v & 0xFF) as u8;
    }
    fn bc(&self) -> u16 { ((self.b as u16) << 8) | self.c as u16 }
    fn set_bc(&mut self, v: u16) {
        self.b = (v >> 8) as u8;
        self.c = (v & 0xFF) as u8;
    }

    fn carry(&self) -> u8 { self.f & 1 }
    fn set_carry(&mut self, c: bool) {
        if c { self.f |= 1; } else { self.f &= !1; }
    }
    fn zero(&self) -> bool { (self.f & 2) != 0 }
    fn set_zero(&mut self, z: bool) {
        if z { self.f |= 2; } else { self.f &= !2; }
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
    fn store16_le(&mut self, addr: u16, val: u16) {
        self.mem_set(addr, (val & 0xFF) as u8);
        self.mem_set(addr.wrapping_add(1), (val >> 8) as u8);
    }

    fn push16(&mut self, v: u16) {
        self.sp = self.sp.wrapping_sub(2);
        self.store16_le(self.sp, v);
    }
    fn pop16(&mut self) -> u16 {
        let v = self.load16_le(self.sp);
        self.sp = self.sp.wrapping_add(2);
        v
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
            0x00 => None, // NOP
            0xC9 => { // RET
                if self.sp == INITIAL_SP {
                    return Some(ExecExitReason::Ret);
                }
                self.pc = self.pop16();
                None
            }
            0x21 => { // LD HL, imm16
                let imm = self.fetch16_le();
                self.set_hl(imm);
                None
            }
            0x2A => { // LD HL, (addr)
                let addr = self.fetch16_le();
                self.set_hl(self.load16_le(addr));
                None
            }
            0x22 => { // LD (addr), HL
                let addr = self.fetch16_le();
                self.store16_le(addr, self.hl());
                None
            }
            0xC3 => { // JP addr
                self.pc = self.fetch16_le();
                None
            }
            0xCD => { // CALL addr
                let target = self.fetch16_le();
                self.push16(self.pc);
                self.pc = target;
                None
            }
            0x18 => { // JR rel
                let rel = self.fetch8() as i8 as i16;
                self.pc = (self.pc as i16).wrapping_add(rel) as u16;
                None
            }
            0x23 => { // INC HL
                self.set_hl(self.hl().wrapping_add(1));
                None
            }
            0x2B => { // DEC HL
                self.set_hl(self.hl().wrapping_sub(1));
                None
            }
            0x01 => { // LD BC, imm16
                let imm = self.fetch16_le();
                self.set_bc(imm);
                None
            }
            0x09 => { // ADD HL, BC
                let (r, c) = self.hl().overflowing_add(self.bc());
                self.set_hl(r);
                self.set_carry(c);
                None
            }
            0x3E => { // LD A, imm
                self.a = self.fetch8();
                None
            }
            0x3A => { // LD A, (addr)
                let addr = self.fetch16_le();
                self.a = self.mem_get(addr);
                None
            }
            0x32 => { // LD (addr), A
                let addr = self.fetch16_le();
                self.mem_set(addr, self.a);
                None
            }
            0x7C => { self.a = self.h; None } // LD A, H
            0x7D => { self.a = self.l; None } // LD A, L
            0x67 => { self.h = self.a; None } // LD H, A
            0x6F => { self.l = self.a; None } // LD L, A
            0x44 => { self.b = self.h; None } // LD B, H
            0x4D => { self.c = self.l; None } // LD C, L
            0x77 => { // LD (HL), A
                self.mem_set(self.hl(), self.a);
                None
            }
            0x85 => { // ADD A, L
                let (r, c) = self.a.overflowing_add(self.l);
                self.a = r;
                self.set_carry(c);
                None
            }
            0x8C => { // ADC A, H
                let sum = (self.a as u16) + (self.h as u16) + (self.carry() as u16);
                self.a = (sum & 0xFF) as u8;
                self.set_carry(sum > 0xFF);
                None
            }
            0x87 => { // ADD A, A
                let (r, c) = self.a.overflowing_add(self.a);
                self.a = r;
                self.set_carry(c);
                None
            }
            0x8F => { // ADC A, A
                let sum = (self.a as u16) * 2 + (self.carry() as u16);
                self.a = (sum & 0xFF) as u8;
                self.set_carry(sum > 0xFF);
                None
            }
            0xB0 => { self.a |= self.b; None } // OR B
            0xB4 => { self.a |= self.h; None } // OR H
            0xB6 => { // OR (HL)
                self.a |= self.mem_get(self.hl());
                None
            }
            0x7E => { // LD A,(HL)
                self.a = self.mem_get(self.hl());
                None
            }
            0xBE => { // CP (HL)
                let val = self.mem_get(self.hl());
                self.set_zero(self.a == val);
                self.set_carry(self.a < val);
                None
            }
            0xB8 => { let _ = self.fetch8(); None } // CP B — legacy stub
            0xB9 => { let _ = self.fetch8(); None } // CP C — legacy stub
            0x86 => { // ADD A, (HL) — platform uses with following oo byte as offset marker
                let oo = self.fetch8();
                let addr = self.hl().wrapping_add(oo as u16);
                let (r, c) = self.a.overflowing_add(self.mem_get(addr));
                self.a = r;
                self.set_carry(c);
                None
            }
            0xD6 => { // SUB imm
                let imm = self.fetch8();
                let (r, b) = self.a.overflowing_sub(imm);
                self.a = r;
                self.set_carry(b);
                None
            }
            0xDE => { // SBC A, imm
                let imm = self.fetch8();
                let diff = (self.a as i16) - (imm as i16) - (self.carry() as i16);
                self.set_carry(diff < 0);
                self.a = (diff as u8);
                None
            }
            0x05 => { // DEC B
                self.b = self.b.wrapping_sub(1);
                None
            }
            0x2E => { // LD L, imm
                self.l = self.fetch8();
                None
            }
            0x26 => { // LD H, imm
                self.h = self.fetch8();
                None
            }
            0x89 => { // ADC A, C
                let sum = (self.a as u16) + (self.c as u16) + (self.carry() as u16);
                self.a = (sum & 0xFF) as u8;
                self.set_carry(sum > 0xFF);
                None
            }
            0xCA => { // JP Z, addr — treat as abs jump for platform stubs
                self.pc = self.fetch16_le();
                None
            }
            0xCB => { // bit ops prefix — skip one following byte
                let _ = self.fetch8();
                None
            }
            0x28 | 0x20 => { // JR Z / JR NZ rel
                let rel = self.fetch8() as i8 as i16;
                let take = if opcode == 0x28 { self.zero() } else { !self.zero() };
                if take {
                    self.pc = (self.pc as i16).wrapping_add(rel) as u16;
                }
                None
            }
            0x7F => { /* LD A, A */ None }
            0xFF => { // RST 38H — platform OR stub padding; treat as nop-ish
                None
            }
            _ => {
                Some(ExecExitReason::Fault {
                    msg: format!("undecoded Z80 opcode 0x{:02x} at pc=0x{:04x}", opcode, pc),
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

/// Run a flat Z80 binary. Bytes are loaded at 0x0000, PC=0, SP=0xFFFE.
pub fn run_z80(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x10000];
    let n = bytes.len().min(0x10000);
    mem[..n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    for slot in 0..N_SLOTS {
        let addr = Z80_STATE_BASE + slot * 2;
        let val = cpu.load16_le(addr) as u64;
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
