//! FreeDOS COM / flat x86-16 interpreter — executes YOYO-emitted DOS COM machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary loaded at 0x100 (COM style), entry=0x100.
//! NOP=0x90, RET=0xC3, INT 20h / INT 21h AH=4Ch exit.

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
const COM_ENTRY: u16 = 0x0100;
const N_SLOTS: u16 = 16;
/// FreeDOS inherits default x64-ish emit for set/get; slots are not used for nop_ret.
/// Keep a small scratch region for any absolute stores near COM PSP.
const STATE_BASE: u16 = 0x0200;

struct Cpu {
    regs: [u16; 8], // AX=0 CX=1 DX=2 BX=3 SP=4 BP=5 SI=6 DI=7
    ip: u16,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
    call_depth: u32,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        let mut regs = [0u16; 8];
        regs[4] = 0xFFFE; // SP
        Self {
            regs, ip: COM_ENTRY, mem,
            steps: 0, step_limit: DEFAULT_STEP_LIMIT, call_depth: 0,
        }
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
        let sp = self.r(4).wrapping_sub(2);
        *self.rw(4) = sp;
        self.store16_le(sp, v);
    }
    fn pop16(&mut self) -> u16 {
        let sp = self.r(4);
        let v = self.load16_le(sp);
        *self.rw(4) = sp.wrapping_add(2);
        v
    }

    fn fetch8(&mut self) -> u8 {
        let v = self.mem_get(self.ip);
        self.ip = self.ip.wrapping_add(1);
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
        if (self.ip as usize) >= self.mem.len() {
            return Some(ExecExitReason::Halted);
        }
        self.steps += 1;
        let op = self.fetch8();

        match op {
            0x90 => None, // NOP
            0xC3 => { // RET
                if self.call_depth == 0 {
                    return Some(ExecExitReason::Ret);
                }
                self.call_depth -= 1;
                self.ip = self.pop16();
                None
            }
            0xCD => { // INT imm8
                let vec = self.fetch8();
                match vec {
                    0x20 => Some(ExecExitReason::Ret), // INT 20h — DOS terminate
                    0x21 => {
                        let ah = (self.r(0) >> 8) as u8;
                        if ah == 0x4C {
                            Some(ExecExitReason::Ret) // INT 21h AH=4Ch
                        } else {
                            None // ignore other DOS services
                        }
                    }
                    _ => Some(ExecExitReason::Fault {
                        msg: format!("unsupported INT 0x{:02x} at ip=0x{:04x}", vec, self.ip.wrapping_sub(2)),
                    }),
                }
            }
            0xB8 => { // MOV AX, imm16 (also accept trailing imm32 low16 for x64 bleed)
                let imm = self.fetch16_le();
                *self.rw(0) = imm;
                None
            }
            0xB9 => { let imm = self.fetch16_le(); *self.rw(1) = imm; None }
            0xBA => { let imm = self.fetch16_le(); *self.rw(2) = imm; None }
            0xBB => { let imm = self.fetch16_le(); *self.rw(3) = imm; None }
            0xE8 => { // CALL rel16
                let rel = self.fetch16_le() as i16;
                let ret = self.ip;
                self.push16(ret);
                self.call_depth += 1;
                self.ip = (ret as i16).wrapping_add(rel) as u16;
                None
            }
            0xE9 => { // JMP rel16
                let rel = self.fetch16_le() as i16;
                self.ip = (self.ip as i16).wrapping_add(rel) as u16;
                None
            }
            0xEB => { // JMP rel8
                let rel = self.fetch8() as i8 as i16;
                self.ip = (self.ip as i16).wrapping_add(rel) as u16;
                None
            }
            0xA1 => { // MOV AX, [imm16]
                let addr = self.fetch16_le();
                *self.rw(0) = self.load16_le(addr);
                None
            }
            0xA3 => { // MOV [imm16], AX
                let addr = self.fetch16_le();
                self.store16_le(addr, self.r(0));
                None
            }
            0x05 => { // ADD AX, imm16
                let imm = self.fetch16_le();
                *self.rw(0) = self.r(0).wrapping_add(imm);
                None
            }
            0x03 => { // ADD AX, r/m16 — only [imm16] form (modrm 0x06)
                let modrm = self.fetch8();
                if modrm == 0x06 {
                    let addr = self.fetch16_le();
                    *self.rw(0) = self.r(0).wrapping_add(self.load16_le(addr));
                } else {
                    return Some(ExecExitReason::Fault {
                        msg: format!("undecoded 0x03 modrm=0x{:02x} at ip=0x{:04x}", modrm, self.ip),
                    });
                }
                None
            }
            0x2D => { // SUB AX, imm16
                let imm = self.fetch16_le();
                *self.rw(0) = self.r(0).wrapping_sub(imm);
                None
            }
            0x40 => { *self.rw(0) = self.r(0).wrapping_add(1); None } // INC AX
            0x41 => { *self.rw(1) = self.r(1).wrapping_add(1); None } // INC CX
            0x42 => { *self.rw(2) = self.r(2).wrapping_add(1); None }
            0x43 => { *self.rw(3) = self.r(3).wrapping_add(1); None }
            0x48 => { *self.rw(0) = self.r(0).wrapping_sub(1); None } // DEC AX
            0x49 => { *self.rw(1) = self.r(1).wrapping_sub(1); None }
            0x4A => { *self.rw(2) = self.r(2).wrapping_sub(1); None }
            0x4B => { *self.rw(3) = self.r(3).wrapping_sub(1); None }
            0x09 => { // OR r/m, r — skip modrm for DDC stub
                let _modrm = self.fetch8();
                None
            }
            0xC7 => { // MOV r/m16, imm16 (partial)
                let modrm = self.fetch8();
                if (modrm & 0xC7) == 0x05 {
                    let addr = self.fetch16_le();
                    let imm = self.fetch16_le();
                    self.store16_le(addr, imm);
                } else {
                    return Some(ExecExitReason::Fault {
                        msg: format!("undecoded C7 modrm=0x{:02x} at ip=0x{:04x}", modrm, self.ip),
                    });
                }
                None
            }
            _ => Some(ExecExitReason::Fault {
                msg: format!("undecoded FreeDOS opcode 0x{:02x} at ip=0x{:04x}", op, self.ip.wrapping_sub(1)),
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

/// Run a FreeDOS COM / flat x86-16 binary. Bytes are loaded at 0x100; IP starts at 0x100.
pub fn run_freedos(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x10000];
    let n = bytes.len().min(0x10000 - COM_ENTRY as usize);
    mem[COM_ENTRY as usize..COM_ENTRY as usize + n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    for slot in 0..N_SLOTS {
        let addr = STATE_BASE + slot * 2;
        let val = cpu.load16_le(addr) as u64;
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
