//! Xtensa LX6 interpreter — executes YOYO-emitted Xtensa flat binary machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary, entry=0. a0 is state base (XTENSA_STATE_BASE=0); slot n at [a0+n*4].
//! Density ops are 3 bytes (NOP, RET); ALU/memory helpers are 4-byte YOYO encodings.

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
const XTENSA_STATE_BASE: u32 = 0;
const N_SLOTS: u16 = 16;

struct Cpu {
    regs: [u32; 16], // a0..a15
    pc: u32,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
    call_depth: u32,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        let mut regs = [0u32; 16];
        regs[0] = XTENSA_STATE_BASE; // a0 = state base
        Self { regs, pc: 0, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT, call_depth: 0 }
    }

    fn r(&self, n: usize) -> u32 { self.regs[n & 0xF] }
    fn rw(&mut self, n: usize) -> &mut u32 { &mut self.regs[n & 0xF] }

    fn mem_get(&self, addr: u32) -> u8 {
        let a = addr as usize;
        if a < self.mem.len() { self.mem[a] } else { 0 }
    }
    fn mem_set(&mut self, addr: u32, v: u8) {
        let a = addr as usize;
        if a >= self.mem.len() { self.mem.resize(a + 1, 0); }
        self.mem[a] = v;
    }
    fn load32_le(&self, addr: u32) -> u32 {
        let mut v = 0u32;
        for i in 0..4u32 {
            v |= (self.mem_get(addr.wrapping_add(i)) as u32) << (i * 8);
        }
        v
    }
    fn store32_le(&mut self, addr: u32, val: u32) {
        for i in 0..4u32 {
            self.mem_set(addr.wrapping_add(i), ((val >> (i * 8)) & 0xFF) as u8);
        }
    }

    fn peek3(&self) -> Option<[u8; 3]> {
        let pc = self.pc as usize;
        if pc + 3 > self.mem.len() { return None; }
        Some([self.mem[pc], self.mem[pc + 1], self.mem[pc + 2]])
    }
    fn peek4(&self) -> Option<[u8; 4]> {
        let pc = self.pc as usize;
        if pc + 4 > self.mem.len() { return None; }
        Some([self.mem[pc], self.mem[pc + 1], self.mem[pc + 2], self.mem[pc + 3]])
    }

    fn is_known_4byte_op(op: u32) -> bool {
        matches!(op, 0x0D | 0x2D | 0x1D | 0x19 | 0x05 | 0x0F | 0x0C | 0x0E | 0x21 | 0x23)
    }

    fn sext15(imm: u32) -> i32 {
        let imm = imm & 0x7FFF;
        if imm & 0x4000 != 0 {
            (imm | 0xFFFF_8000) as i32
        } else {
            imm as i32
        }
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit {
            return Some(ExecExitReason::StepLimit { steps: self.steps });
        }
        let pc = self.pc;
        if (pc as usize) >= self.mem.len() {
            return Some(ExecExitReason::Halted);
        }
        self.steps += 1;

        let Some(b3) = self.peek3() else {
            return Some(ExecExitReason::Halted);
        };

        // RET: F0 00 00
        if b3 == [0xF0, 0x00, 0x00] {
            if self.call_depth == 0 {
                return Some(ExecExitReason::Ret);
            }
            self.call_depth -= 1;
            // a0 holds return in real Xtensa CALL0; YOYO uses flat RET as exit — pop from a1 scratch unused
            self.pc = pc.wrapping_add(3);
            return Some(ExecExitReason::Ret);
        }

        // XtensaImm18 patched j/jcc/callx: byte2=0x06 (jmp/jcc) or 0x0A (call)
        if b3[2] == 0x06 || b3[2] == 0x0A {
            let imm18 = (b3[0] as u32) | ((b3[1] as u32) << 8);
            let diff = if imm18 & 0x20000 != 0 {
                (imm18 | 0xFFFC0000) as i32
            } else {
                imm18 as i32
            };
            if b3[2] == 0x0A {
                self.call_depth += 1;
            }
            self.pc = (pc as i32).wrapping_add(diff) as u32;
            return None;
        }

        // Prefer 4-byte YOYO encoding when opcode field matches known helpers
        if let Some(b4) = self.peek4() {
            let word = u32::from_le_bytes(b4);
            let op = word >> 26;
            if Self::is_known_4byte_op(op) {
                let ra = ((word >> 23) & 0x7) as usize;
                let rd = ((word >> 20) & 0x7) as usize;
                let rb = ((word >> 15) & 0x7) as usize;
                match op {
                    0x0D => { // ADD: rd = ra + rb
                        *self.rw(rd) = self.r(ra).wrapping_add(self.r(rb));
                        self.pc = pc.wrapping_add(4);
                        return None;
                    }
                    0x2D => { // SUB
                        *self.rw(rd) = self.r(ra).wrapping_sub(self.r(rb));
                        self.pc = pc.wrapping_add(4);
                        return None;
                    }
                    0x1D => { // OR
                        *self.rw(rd) = self.r(ra) | self.r(rb);
                        self.pc = pc.wrapping_add(4);
                        return None;
                    }
                    0x19 => { // MUL
                        *self.rw(rd) = self.r(ra).wrapping_mul(self.r(rb));
                        self.pc = pc.wrapping_add(4);
                        return None;
                    }
                    0x05 => { // SLL — rb field holds shift amount immediate in YOYO emit
                        *self.rw(rd) = self.r(ra).wrapping_shl(rb as u32);
                        self.pc = pc.wrapping_add(4);
                        return None;
                    }
                    0x0F => { // ADDI rd = ra + sext(imm15)
                        let imm = Self::sext15(word & 0x7FFF);
                        *self.rw(rd) = (self.r(ra) as i32).wrapping_add(imm) as u32;
                        self.pc = pc.wrapping_add(4);
                        return None;
                    }
                    0x0C => { // L32I rd, ra, wi
                        let addr = self.r(ra).wrapping_add((rb as u32) * 4);
                        *self.rw(rd) = self.load32_le(addr);
                        self.pc = pc.wrapping_add(4);
                        return None;
                    }
                    0x0E => { // S32I rd, ra, wi
                        let addr = self.r(ra).wrapping_add((rb as u32) * 4);
                        self.store32_le(addr, self.r(rd));
                        self.pc = pc.wrapping_add(4);
                        return None;
                    }
                    0x21 => { // L32I disp8
                        let disp = (word & 0xFF) as u32;
                        let addr = self.r(ra).wrapping_add(disp);
                        *self.rw(rd) = self.load32_le(addr);
                        self.pc = pc.wrapping_add(4);
                        return None;
                    }
                    0x23 => { // S32I disp8
                        let disp = (word & 0xFF) as u32;
                        let addr = self.r(ra).wrapping_add(disp);
                        self.store32_le(addr, self.r(rd));
                        self.pc = pc.wrapping_add(4);
                        return None;
                    }
                    _ => {}
                }
            }
        }

        // NOP: 00 00 00 (3-byte) — after ruling out 4-byte ops starting with zeros
        if b3 == [0x00, 0x00, 0x00] {
            self.pc = pc.wrapping_add(3);
            return None;
        }

        Some(ExecExitReason::Fault {
            msg: format!(
                "undecoded Xtensa insn at 0x{:04x}: {:02x} {:02x} {:02x}",
                pc, b3[0], b3[1], b3[2]
            ),
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

/// Run a flat Xtensa binary. Bytes loaded at 0x0000, PC=0, a0=state base.
pub fn run_xtensa(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x10000];
    let n = bytes.len().min(mem.len());
    mem[..n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    for slot in 0..N_SLOTS {
        let addr = XTENSA_STATE_BASE + (slot as u32) * 4;
        let val = cpu.load32_le(addr) as u64;
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
