//! Motorola 68000 interpreter — executes YOYO-emitted M68k flat binary machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Flat binary, entry=0. State is relative to A0 (initialized to M68K_STATE_BASE=0x0100),
//! 16-bit big-endian slots at [A0 + slot*2].
//!
//! Decodes the exact encodings produced by M68kPlatform helpers in platform.rs
//! (some are simplified/non-canonical M68k forms).

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
const M68K_STATE_BASE: u32 = 0x0100;
const N_SLOTS: u16 = 16;
const INITIAL_SP: u32 = 0x8000;

struct Cpu {
    d: [u32; 8],
    a: [u32; 8],
    pc: u32,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
    call_depth: u32,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        let mut a = [0u32; 8];
        a[0] = M68K_STATE_BASE;
        a[7] = INITIAL_SP; // USP/SSP
        Self {
            d: [0; 8], a, pc: 0, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT, call_depth: 0,
        }
    }

    fn mem_get(&self, addr: u32) -> u8 {
        let a = addr as usize;
        if a < self.mem.len() { self.mem[a] } else { 0 }
    }
    fn mem_set(&mut self, addr: u32, v: u8) {
        let a = addr as usize;
        if a >= self.mem.len() { self.mem.resize(a + 1, 0); }
        self.mem[a] = v;
    }
    fn load16_be(&self, addr: u32) -> u16 {
        let hi = self.mem_get(addr) as u16;
        let lo = self.mem_get(addr.wrapping_add(1)) as u16;
        (hi << 8) | lo
    }
    fn store16_be(&mut self, addr: u32, val: u16) {
        self.mem_set(addr, (val >> 8) as u8);
        self.mem_set(addr.wrapping_add(1), (val & 0xFF) as u8);
    }
    fn load32_be(&self, addr: u32) -> u32 {
        let b0 = self.mem_get(addr) as u32;
        let b1 = self.mem_get(addr.wrapping_add(1)) as u32;
        let b2 = self.mem_get(addr.wrapping_add(2)) as u32;
        let b3 = self.mem_get(addr.wrapping_add(3)) as u32;
        (b0 << 24) | (b1 << 16) | (b2 << 8) | b3
    }
    fn store32_be(&mut self, addr: u32, val: u32) {
        self.mem_set(addr, (val >> 24) as u8);
        self.mem_set(addr.wrapping_add(1), (val >> 16) as u8);
        self.mem_set(addr.wrapping_add(2), (val >> 8) as u8);
        self.mem_set(addr.wrapping_add(3), (val & 0xFF) as u8);
    }

    fn fetch16(&mut self) -> u16 {
        let v = self.load16_be(self.pc);
        self.pc = self.pc.wrapping_add(2);
        v
    }
    fn fetch32(&mut self) -> u32 {
        let v = self.load32_be(self.pc);
        self.pc = self.pc.wrapping_add(4);
        v
    }

    fn push32(&mut self, v: u32) {
        self.a[7] = self.a[7].wrapping_sub(4);
        self.store32_be(self.a[7], v);
    }
    fn pop32(&mut self) -> u32 {
        let v = self.load32_be(self.a[7]);
        self.a[7] = self.a[7].wrapping_add(4);
        v
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit {
            return Some(ExecExitReason::StepLimit { steps: self.steps });
        }
        let pc = self.pc;
        if (pc as usize) + 2 > self.mem.len() {
            return Some(ExecExitReason::Halted);
        }

        let op = self.fetch16();
        self.steps += 1;

        // NOP: 0x4E71
        if op == 0x4E71 {
            return None;
        }
        // RTS: 0x4E75
        if op == 0x4E75 {
            if self.call_depth == 0 {
                return Some(ExecExitReason::Ret);
            }
            self.call_depth -= 1;
            self.pc = self.pop32();
            return None;
        }
        // TRAP / exit: 0x4E40
        if op == 0x4E40 {
            return Some(ExecExitReason::Ret);
        }

        // MOVE.W #imm16, (disp16,a0) — platform: 0x30FC + disp16 BE + imm16 BE
        if op == 0x30FC {
            let disp = self.fetch16() as i16 as i32;
            let imm = self.fetch16();
            let addr = (self.a[0] as i32).wrapping_add(disp) as u32;
            self.store16_be(addr, imm);
            return None;
        }

        // MOVE.W (disp16,a0), d0 — platform: 0x2080 + disp16 BE
        if op == 0x2080 {
            let disp = self.fetch16() as i16 as i32;
            let addr = (self.a[0] as i32).wrapping_add(disp) as u32;
            self.d[0] = self.load16_be(addr) as u32;
            return None;
        }

        // MOVE.W d0, (disp16,a0) — platform: 0x2280 + disp16 BE
        if op == 0x2280 {
            let disp = self.fetch16() as i16 as i32;
            let addr = (self.a[0] as i32).wrapping_add(disp) as u32;
            self.store16_be(addr, (self.d[0] & 0xFFFF) as u16);
            return None;
        }

        // MOVE.B (disp16,a0), d0 — platform: 0x1080 + disp16 BE
        if op == 0x1080 {
            let disp = self.fetch16() as i16 as i32;
            let addr = (self.a[0] as i32).wrapping_add(disp) as u32;
            self.d[0] = self.mem_get(addr) as u32;
            return None;
        }

        // MOVE.B d0, (disp16,a0) — platform: 0x1280 + disp16 BE
        if op == 0x1280 {
            let disp = self.fetch16() as i16 as i32;
            let addr = (self.a[0] as i32).wrapping_add(disp) as u32;
            self.mem_set(addr, (self.d[0] & 0xFF) as u8);
            return None;
        }

        // MOVE.W d0, d1 — platform: 0x2000
        if op == 0x2000 {
            self.d[1] = self.d[0] & 0xFFFF;
            return None;
        }

        // ADD.W #imm16, d0 — platform: 0x3030 + imm16 BE
        if op == 0x3030 {
            let imm = self.fetch16() as u32;
            self.d[0] = (self.d[0].wrapping_add(imm)) & 0xFFFF;
            return None;
        }

        // SUB.W #imm16, d0 — platform: 0x30B0 + imm16 BE
        if op == 0x30B0 {
            let imm = self.fetch16() as u32;
            self.d[0] = (self.d[0].wrapping_sub(imm)) & 0xFFFF;
            return None;
        }

        // ADD.W d1, d0 / ADD.W d1, (disp,a0) — platform: 0x00C0 [+ disp]
        if op == 0x00C0 {
            // Ambiguous: could be reg-reg or mem. Peek: if next looks like a small disp
            // used by add_w_reg_to_a0_disp, consume disp and add d1 into memory.
            // For emit_addv the sequence is MOVE then ADD.W d1,d0 without extra word.
            // Heuristic: if following word is a tiny displacement (<= 0x100) and we're
            // mid-stream after a store pattern, prefer mem form only when called via
            // m68k_add_w_reg_to_a0_disp. Since emit_addv uses reg-reg, treat bare 0x00C0
            // as ADD.W d1, d0.
            self.d[0] = (self.d[0].wrapping_add(self.d[1])) & 0xFFFF;
            return None;
        }

        // SUB.W d1, d0 — platform: 0x0040
        if op == 0x0040 {
            self.d[0] = (self.d[0].wrapping_sub(self.d[1])) & 0xFFFF;
            return None;
        }

        // OR.W d1, d0 — platform: 0x02C0
        if op == 0x02C0 {
            self.d[0] = (self.d[0] | self.d[1]) & 0xFFFF;
            return None;
        }

        // ADDA.L d0, a0 — platform: 0x0680
        if op == 0x0680 {
            self.a[0] = self.a[0].wrapping_add(self.d[0]);
            return None;
        }

        // CMP.W d1, d0 — platform: 0x10C0
        if op == 0x10C0 {
            let _ = (self.d[0] as u16).wrapping_sub(self.d[1] as u16);
            return None;
        }

        // JMP abs32 — platform placeholder: 0x4EF9 + ea word + abs
        if op == 0x4EF9 {
            let _ea = self.fetch16();
            let target = self.fetch16() as u32; // AbsAddr16 fixup field
            self.pc = target;
            return None;
        }

        // JSR abs — platform: 0x4EB9 + ea + abs
        if op == 0x4EB9 {
            let _ea = self.fetch16();
            let target = self.fetch16() as u32;
            self.push32(self.pc);
            self.call_depth += 1;
            self.pc = target;
            return None;
        }

        // BRA.S / BSR.S rel8 — platform jcc: 0x60xx
        if (op & 0xFF00) == 0x6000 {
            let rel = (op & 0xFF) as i8 as i32;
            if rel == 0 {
                // 16-bit extension word
                let rel16 = self.fetch16() as i16 as i32;
                self.pc = (self.pc as i32).wrapping_add(rel16) as u32;
            } else {
                self.pc = (self.pc as i32).wrapping_add(rel) as u32;
            }
            return None;
        }

        Some(ExecExitReason::Fault {
            msg: format!("undecoded M68k insn at pc=0x{:08x}: 0x{:04x}", pc, op),
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

/// Run a flat M68k binary. Bytes are loaded at 0x0000, PC=0, A0=0x0100.
pub fn run_m68k(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x10000];
    let n = bytes.len().min(0x10000);
    mem[..n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    let base = cpu.a[0];
    for slot in 0..N_SLOTS {
        let addr = base.wrapping_add((slot as u32) * 2);
        let val = cpu.load16_be(addr) as u64;
        if val != 0 {
            state.insert(slot, val);
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
