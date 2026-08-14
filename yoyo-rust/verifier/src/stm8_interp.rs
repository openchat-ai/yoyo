//! STM8 interpreter — executes YOYO-emitted STM8 flat binary machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary, entry=0. Decodes the YOYO Stm8Platform helper encodings (simplified).
//! NOP=0x9D, RET=0x81. State at STM8_STATE_BASE (0x4000), 8-bit slots.

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
const STM8_STATE_BASE: u16 = 0x4000;
const N_SLOTS: u16 = 16;
const INITIAL_SP: u16 = 0x03FF;

struct Cpu {
    a: u8,
    /// Scratch regs R0..R7 used by YOYO stm8_st_reg_addr / add_a_reg helpers
    regs: [u8; 8],
    sp: u16,
    pc: u16,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
    call_depth: u32,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        Self {
            a: 0, regs: [0; 8], sp: INITIAL_SP, pc: 0, mem,
            steps: 0, step_limit: DEFAULT_STEP_LIMIT, call_depth: 0,
        }
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

    fn fetch8(&mut self) -> Option<u8> {
        if (self.pc as usize) >= self.mem.len() { return None; }
        let v = self.mem_get(self.pc);
        self.pc = self.pc.wrapping_add(1);
        Some(v)
    }
    fn fetch16_le(&mut self) -> Option<u16> {
        let lo = self.fetch8()? as u16;
        let hi = self.fetch8()? as u16;
        Some(lo | (hi << 8))
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit {
            return Some(ExecExitReason::StepLimit { steps: self.steps });
        }
        let pc = self.pc;
        if (pc as usize) >= self.mem.len() {
            return Some(ExecExitReason::Halted);
        }
        let Some(op) = self.fetch8() else {
            return Some(ExecExitReason::Halted);
        };
        self.steps += 1;

        match op {
            0x9D => None, // NOP
            0x81 => { // RET
                if self.call_depth == 0 {
                    return Some(ExecExitReason::Ret);
                }
                self.call_depth -= 1;
                self.pc = self.pop16();
                None
            }
            0x83 => Some(ExecExitReason::Ret), // EXIT / halt
            0x3F => { // LD A, #imm
                let Some(imm) = self.fetch8() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated LD A,# at 0x{:04x}", pc) });
                };
                self.a = imm;
                None
            }
            0x86 => { // LD A, addr16
                let Some(addr) = self.fetch16_le() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated LD A,abs at 0x{:04x}", pc) });
                };
                self.a = self.mem_get(addr);
                None
            }
            0x87 => { // ST A, addr16
                let Some(addr) = self.fetch16_le() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated ST A,abs at 0x{:04x}", pc) });
                };
                self.mem_set(addr, self.a);
                None
            }
            0x4F => { // ADD A, #imm
                let Some(imm) = self.fetch8() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated ADD A,# at 0x{:04x}", pc) });
                };
                self.a = self.a.wrapping_add(imm);
                None
            }
            0x5F => { // SUB A, #imm
                let Some(imm) = self.fetch8() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated SUB A,# at 0x{:04x}", pc) });
                };
                self.a = self.a.wrapping_sub(imm);
                None
            }
            0x3C => { // INC A
                self.a = self.a.wrapping_add(1);
                None
            }
            0x3D => { // DEC A
                self.a = self.a.wrapping_sub(1);
                None
            }
            0x0F => { // ADD A, Rr
                let Some(r) = self.fetch8() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated ADD A,r at 0x{:04x}", pc) });
                };
                let ri = (r & 0x07) as usize;
                self.a = self.a.wrapping_add(self.regs[ri]);
                None
            }
            0x1F => { // SUB A, Rr
                let Some(r) = self.fetch8() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated SUB A,r at 0x{:04x}", pc) });
                };
                let ri = (r & 0x07) as usize;
                self.a = self.a.wrapping_sub(self.regs[ri]);
                None
            }
            0x6F => { // OR A, Rr
                let Some(r) = self.fetch8() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated OR A,r at 0x{:04x}", pc) });
                };
                let ri = (r & 0x07) as usize;
                self.a |= self.regs[ri];
                None
            }
            0x95 => { // ST Rr, addr16  — bytes: 95, reg, lo, hi
                let Some(r) = self.fetch8() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated ST r,abs at 0x{:04x}", pc) });
                };
                let Some(addr) = self.fetch16_le() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated ST r,abs addr at 0x{:04x}", pc) });
                };
                let ri = (r & 0x07) as usize;
                // YOYO emit_addv stores A into reg via this after LD A — here store A into regs[ri]
                // and also write regs[ri] to addr. Matching stm8_st_reg_addr which stores reg to addr;
                // prior instruction typically put A into memory then we copy A→reg.
                self.regs[ri] = self.a;
                self.mem_set(addr, self.regs[ri]);
                None
            }
            0x94 => { // LD Rr, addr16
                let Some(r) = self.fetch8() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated LD r,abs at 0x{:04x}", pc) });
                };
                let Some(addr) = self.fetch16_le() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated LD r,abs addr at 0x{:04x}", pc) });
                };
                let ri = (r & 0x07) as usize;
                self.regs[ri] = self.mem_get(addr);
                None
            }
            0x89 => { // JMP addr16
                let Some(addr) = self.fetch16_le() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated JMP at 0x{:04x}", pc) });
                };
                self.pc = addr;
                None
            }
            0xD3 => { // CALL addr16
                let Some(addr) = self.fetch16_le() else {
                    return Some(ExecExitReason::Fault { msg: format!("truncated CALL at 0x{:04x}", pc) });
                };
                self.push16(self.pc);
                self.call_depth += 1;
                self.pc = addr;
                None
            }
            _ => {
                // stm8_ld_a_xr: 0x16 | (reg&7), 0x00 — LD A from reg
                if (op & 0xF8) == 0x16 {
                    let _ = self.fetch8(); // 0x00 pad
                    let ri = (op & 0x07) as usize;
                    self.a = self.regs[ri];
                    return None;
                }
                // stm8_ld_imm_reg: 0x2F | ((reg&7)<<5), imm
                if (op & 0x1F) == 0x2F {
                    let Some(imm) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: format!("truncated LD r,# at 0x{:04x}", pc) });
                    };
                    let ri = ((op >> 5) & 0x07) as usize;
                    self.regs[ri] = imm;
                    return None;
                }
                Some(ExecExitReason::Fault {
                    msg: format!("undecoded STM8 insn at 0x{:04x}: {:02x}", pc, op),
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

/// Run a flat STM8 binary. Bytes loaded at 0x0000, PC=0.
pub fn run_stm8(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x8000];
    let n = bytes.len().min(mem.len());
    mem[..n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    for slot in 0..N_SLOTS {
        let addr = STM8_STATE_BASE + slot;
        let val = cpu.mem_get(addr) as u64;
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
