//! Intel 8051 interpreter — executes YOYO-emitted 8051 flat binary machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary, entry=0. State is at E8051_STATE_BASE (0x30), 8-bit slots.
//! No ELF, no PE — just raw bytes.

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
const E8051_STATE_BASE: u8 = 0x30;
const N_SLOTS: u16 = 16;

struct Cpu {
    // 8051 regs: A=0, B=1, PSW=2, DPL=3, DPH=4, SP=5, PC=6
    regs: [u64; 7],
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        Self { regs: [0; 7], mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT }
    }

    fn r(&self, n: usize) -> u8 { self.regs[n] as u8 }
    fn rw(&mut self, n: usize) -> &mut u64 { &mut self.regs[n] }

    fn pc(&self) -> u16 { self.regs[6] as u16 }
    fn set_pc(&mut self, v: u16) { self.regs[6] = v as u64; }

    fn mem_get(&self, addr: u16) -> u8 {
        let a = addr as usize;
        if a < self.mem.len() { self.mem[a] } else { 0 }
    }
    fn mem_set(&mut self, addr: u16, v: u8) {
        let a = addr as usize;
        if a >= self.mem.len() { self.mem.resize(a + 1, 0); }
        self.mem[a] = v;
    }

    fn push_stack(&mut self, v: u8) {
        let sp = self.r(5);
        self.mem_set(0x100 + sp as u16, v);
        *self.rw(5) = (sp.wrapping_add(1)) as u64;
    }

    fn pop_stack(&mut self) -> u8 {
        let sp = self.r(5);
        *self.rw(5) = (sp.wrapping_sub(1)) as u64;
        let sp2 = self.r(5);
        self.mem_get(0x100 + sp2 as u16)
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        let pc = self.pc();
        if (pc as usize) >= self.mem.len() { return Some(ExecExitReason::Halted); }

        let opcode = self.mem_get(pc);
        self.steps += 1;

        match opcode {
            0x00 => { // NOP
                self.set_pc(pc.wrapping_add(1));
                None
            }
            0x22 => { // RET
                let lo = self.pop_stack();
                let hi = self.pop_stack();
                let ret_addr = (hi as u16) << 8 | lo as u16;
                if ret_addr == 0 && self.r(5) == 0x07 {
                    // Top-level ret — SP back to initial
                    return Some(ExecExitReason::Ret);
                }
                self.set_pc(ret_addr);
                None
            }
            0xE5 => { // MOV A, direct
                let direct = self.mem_get(pc.wrapping_add(1));
                *self.rw(0) = self.mem_get(direct as u16) as u64;
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0xF5 => { // MOV direct, A
                let direct = self.mem_get(pc.wrapping_add(1));
                // If direct is B (0xF0), DPL (0x82), DPH (0x83) — these are SFRs
                match direct {
                    0xF0 => { *self.rw(1) = self.r(0) as u64; } // B
                    0x82 => { *self.rw(3) = self.r(0) as u64; } // DPL
                    0x83 => { *self.rw(4) = self.r(0) as u64; } // DPH
                    _ => { self.mem_set(direct as u16, self.r(0)); }
                }
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0x75 => { // MOV direct, #imm
                let direct = self.mem_get(pc.wrapping_add(1));
                let imm = self.mem_get(pc.wrapping_add(2));
                match direct {
                    0x83 => { *self.rw(4) = imm as u64; } // DPH
                    0xF0 => { *self.rw(1) = imm as u64; } // B
                    _ => { self.mem_set(direct as u16, imm); }
                }
                self.set_pc(pc.wrapping_add(3));
                None
            }
            0x74 => { // MOV A, #imm
                let imm = self.mem_get(pc.wrapping_add(1));
                *self.rw(0) = imm as u64;
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0x24 => { // ADD A, #imm
                let imm = self.mem_get(pc.wrapping_add(1));
                let result = self.r(0).wrapping_add(imm);
                *self.rw(0) = result as u64;
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0x25 => { // ADD A, direct
                let direct = self.mem_get(pc.wrapping_add(1));
                let val = self.mem_get(direct as u16);
                let result = self.r(0).wrapping_add(val);
                *self.rw(0) = result as u64;
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0x94 => { // SUBB A, #imm (with borrow)
                let imm = self.mem_get(pc.wrapping_add(1));
                let c = (self.r(2) & 0x80) >> 7; // carry flag from PSW bit 7
                let result = self.r(0).wrapping_sub(imm).wrapping_sub(c);
                *self.rw(0) = result as u64;
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0x95 => { // SUBB A, direct (with borrow)
                let direct = self.mem_get(pc.wrapping_add(1));
                let val = self.mem_get(direct as u16);
                let c = (self.r(2) & 0x80) >> 7;
                let result = self.r(0).wrapping_sub(val).wrapping_sub(c);
                *self.rw(0) = result as u64;
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0x45 => { // ORL A, direct
                let direct = self.mem_get(pc.wrapping_add(1));
                let val = self.mem_get(direct as u16);
                *self.rw(0) = (self.r(0) | val) as u64;
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0x55 => { // ANL A, direct
                let direct = self.mem_get(pc.wrapping_add(1));
                let val = self.mem_get(direct as u16);
                *self.rw(0) = (self.r(0) & val) as u64;
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0x05 => { // INC direct
                let direct = self.mem_get(pc.wrapping_add(1));
                let val = self.mem_get(direct as u16).wrapping_add(1);
                self.mem_set(direct as u16, val);
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0x15 => { // DEC direct
                let direct = self.mem_get(pc.wrapping_add(1));
                let val = self.mem_get(direct as u16).wrapping_sub(1);
                self.mem_set(direct as u16, val);
                self.set_pc(pc.wrapping_add(2));
                None
            }
            0xA4 => { // MUL AB
                let result = (self.r(0) as u16) * (self.r(1) as u16);
                *self.rw(0) = (result & 0xFF) as u64;
                *self.rw(1) = ((result >> 8) & 0xFF) as u64;
                self.set_pc(pc.wrapping_add(1));
                None
            }
            0xC3 => { // CLR C
                *self.rw(2) = (self.r(2) as u64) & !0x80u64; // clear carry bit
                self.set_pc(pc.wrapping_add(1));
                None
            }
            0xE0 => { // MOVX A, @DPTR
                let dpl = self.r(3) as u16;
                let dph = self.r(4) as u16;
                let addr = (dph << 8) | dpl;
                *self.rw(0) = self.mem_get(addr) as u64;
                self.set_pc(pc.wrapping_add(1));
                None
            }
            0x90 => { // MOV DPTR, #imm16
                let hi = self.mem_get(pc.wrapping_add(1));
                let lo = self.mem_get(pc.wrapping_add(2));
                *self.rw(3) = lo as u64; // DPL
                *self.rw(4) = hi as u64; // DPH
                self.set_pc(pc.wrapping_add(3));
                None
            }
            0xB5 => { // CJNE A, direct, rel
                let direct = self.mem_get(pc.wrapping_add(1));
                let rel = self.mem_get(pc.wrapping_add(2)) as i8 as i16;
                let val = self.mem_get(direct as u16);
                if self.r(0) != val {
                    self.set_pc((pc.wrapping_add(3) as i16).wrapping_add(rel) as u16);
                } else {
                    self.set_pc(pc.wrapping_add(3));
                }
                None
            }
            0x02 => { // LJMP addr16
                let hi = self.mem_get(pc.wrapping_add(1)) as u16;
                let lo = self.mem_get(pc.wrapping_add(2)) as u16;
                self.set_pc((hi << 8) | lo);
                None
            }
            0x12 => { // LCALL addr16
                let hi = self.mem_get(pc.wrapping_add(1)) as u16;
                let lo = self.mem_get(pc.wrapping_add(2)) as u16;
                let target = (hi << 8) | lo;
                let ret_addr = pc.wrapping_add(3);
                self.push_stack((ret_addr & 0xFF) as u8);
                self.push_stack((ret_addr >> 8) as u8);
                self.set_pc(target);
                None
            }
            0x80 => { // SJMP rel
                let rel = self.mem_get(pc.wrapping_add(1)) as i8 as i16;
                self.set_pc((pc.wrapping_add(2) as i16).wrapping_add(rel) as u16);
                None
            }
            0x60 => { // JZ rel
                let rel = self.mem_get(pc.wrapping_add(1)) as i8 as i16;
                if self.r(0) == 0 {
                    self.set_pc((pc.wrapping_add(2) as i16).wrapping_add(rel) as u16);
                } else {
                    self.set_pc(pc.wrapping_add(2));
                }
                None
            }
            0x70 => { // JNZ rel
                let rel = self.mem_get(pc.wrapping_add(1)) as i8 as i16;
                if self.r(0) != 0 {
                    self.set_pc((pc.wrapping_add(2) as i16).wrapping_add(rel) as u16);
                } else {
                    self.set_pc(pc.wrapping_add(2));
                }
                None
            }
            0x40 => { // JC rel
                let rel = self.mem_get(pc.wrapping_add(1)) as i8 as i16;
                let c = (self.r(2) & 0x80) >> 7;
                if c != 0 {
                    self.set_pc((pc.wrapping_add(2) as i16).wrapping_add(rel) as u16);
                } else {
                    self.set_pc(pc.wrapping_add(2));
                }
                None
            }
            0x50 => { // JNC rel
                let rel = self.mem_get(pc.wrapping_add(1)) as i8 as i16;
                let c = (self.r(2) & 0x80) >> 7;
                if c == 0 {
                    self.set_pc((pc.wrapping_add(2) as i16).wrapping_add(rel) as u16);
                } else {
                    self.set_pc(pc.wrapping_add(2));
                }
                None
            }
            _ => {
                Some(ExecExitReason::Fault { msg: format!("undecoded 8051 opcode 0x{:02x} at pc=0x{:04x}", opcode, pc) })
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

/// Run a flat 8051 binary. Bytes are loaded at 0x0000, SP initialized to 0x07.
pub fn run_8051(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x200]; // 512 bytes of internal RAM
    let n = bytes.len().min(0x200);
    mem[..n].copy_from_slice(&bytes[..n]);

    // Initialize SP to 0x07 (stack starts at 0x08)
    let mut cpu = Cpu::new(mem);
    *cpu.rw(5) = 0x07; // SP = 0x07
    cpu.set_pc(0);

    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    let base = E8051_STATE_BASE as u16;
    for slot in 0..N_SLOTS {
        let addr = base + slot;
        let val = cpu.mem_get(addr) as u64;
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}