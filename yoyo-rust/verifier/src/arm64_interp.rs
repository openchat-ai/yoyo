//! ARM64 (AArch64) interpreter — executes YOYO-emitted ARM64 machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! The emitted code uses:
//!   - state base register x15 (set by the ELF startup stub)
//!   - state slots as 64-bit words at [x15 + slot*8]
//!   - scratch registers x9/x10/x11
//!   - CMP stores a in x10, b in x11; JCC uses b.cond based on NZCV flags

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

#[derive(Debug, Default, Clone, Copy)]
struct Flags { n: bool, z: bool, c: bool, v: bool }

struct Cpu {
    regs: [u64; 31],
    pc: u64,
    flags: Flags,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
    call_stack: Vec<u64>,
}

impl Cpu {
    fn new(mem: Vec<u8>, entry: u64) -> Self {
        Self { regs: [0; 31], pc: entry, flags: Flags::default(), mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT, call_stack: Vec::new() }
    }

    fn alloc_state(&mut self, data_va: u64, n_slots: usize) {
        for i in 0..n_slots * 8 {
            let addr = data_va as usize + i;
            if addr >= self.mem.len() { self.mem.resize(addr + 1, 0); }
        }
    }

    fn r(&self, n: usize) -> u64 { if n == 31 { 0 } else { self.regs[n] } }
    fn rw(&mut self, n: usize) -> &mut u64 { if n == 31 { panic!("xzr not writable") } else { &mut self.regs[n] } }

    fn mem_get(&self, addr: u64) -> u8 { let a = addr as usize; if a < self.mem.len() { self.mem[a] } else { 0 } }
    fn mem_set(&mut self, addr: u64, v: u8) { let a = addr as usize; if a >= self.mem.len() { self.mem.resize(a + 1, 0); } self.mem[a] = v; }

    fn load64(&self, addr: u64) -> u64 {
        let mut v = 0u64;
        for i in 0..8 { v |= (self.mem_get(addr + i) as u64) << (i * 8); }
        v
    }

    fn store64(&mut self, addr: u64, val: u64) {
        for i in 0..8 { self.mem_set(addr + i, ((val >> (i * 8)) & 0xFF) as u8); }
    }

    fn cond_true(&self, cond: u32) -> bool {
        match cond {
            0x0 => self.flags.z,           // EQ
            0x1 => !self.flags.z,          // NE
            0xB => self.flags.n != self.flags.v, // LT
            0xA => self.flags.n == self.flags.v, // GE
            0xD => self.flags.z || self.flags.n != self.flags.v, // LE
            0xC => !self.flags.z && self.flags.n == self.flags.v, // GT
            0x3 => !self.flags.c,          // LO
            0x2 => self.flags.c,           // HS
            0x9 => !self.flags.c || self.flags.z, // LS
            0x8 => self.flags.c && !self.flags.z, // HI
            _ => false,
        }
    }

    fn set_flags_sub(&mut self, a: u64, b: u64) {
        self.flags.n = (a.wrapping_sub(b) >> 63) != 0;
        self.flags.z = a == b;
        self.flags.c = a >= b;
        self.flags.v = ((a >> 63) != (b >> 63)) && ((a.wrapping_sub(b) >> 63) != (a >> 63));
    }

    /// Execute one instruction. Returns Some(exit_reason) if should stop, None to continue.
    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        if self.pc as usize + 4 > self.mem.len() { return Some(ExecExitReason::Halted); }
        let insn = u32::from_le_bytes(self.mem[self.pc as usize..self.pc as usize + 4].try_into().unwrap());
        self.steps += 1;

        // B / BL
        let top6 = insn >> 26;
        if insn == 0xD503201F {
            // NOP
            self.pc += 4;
            return None;
        }
        if top6 == 0b000101 || top6 == 0b100101 {
            let imm26 = ((insn & 0x3FFFFFF) as i32) << 6 >> 6;
            let target = (self.pc as i64 + (imm26 as i64) << 2) as u64;
            if top6 == 0b100101 { self.regs[30] = self.pc + 4; }
            self.pc = target;
            return None;
        }

        // B.cond
        if insn >> 24 == 0b01010100 {
            let cond = (insn >> 4) & 0xF;
            let imm19 = ((insn >> 5) & 0x7FFFF) as i32;
            let imm19 = (imm19 << 13) >> 13;
            if self.cond_true(cond) { self.pc = (self.pc as i64 + (imm19 as i64) << 2) as u64; }
            else { self.pc += 4; }
            return None;
        }

        // BR / RET
        if insn == 0xD65F03C0 {
            // ret = br x30
            if let Some(addr) = self.call_stack.pop() { self.pc = addr; return None; }
            return Some(ExecExitReason::Ret);
        }
        if (insn & 0xFFFFFC1F) == 0xD61F0000 {
            let rn = ((insn >> 5) & 0x1F) as usize;
            if rn == 30 { // ret
                if let Some(addr) = self.call_stack.pop() { self.pc = addr; return None; }
                return Some(ExecExitReason::Ret);
            }
            self.pc = self.regs[rn]; // br x16 etc
            return None;
        }

        // MOVZ / MOVK
        if (insn >> 23) & 0x7F == 0b1101001 || (insn >> 23) & 0x7F == 0b1111001 {
            let is_movk = (insn >> 23) & 0x7F == 0b1111001;
            let hw = (insn >> 21) & 0x3;
            let imm16 = (insn >> 5) & 0xFFFF;
            let rd = (insn & 0x1F) as usize;
            let val = (imm16 as u64) << (hw * 16);
            if is_movk { *self.rw(rd) = (self.r(rd) & !(0xFFFFu64 << (hw * 16))) | val; }
            else { *self.rw(rd) = val; }
            self.pc += 4; return None;
        }

        // ADRP
        if insn >> 24 == 0b10010000 {
            let rd = (insn & 0x1F) as usize;
            let immhi = ((insn >> 5) & 0x7FFFF) as i64;
            let immlo = ((insn >> 29) & 0x3) as i64;
            let imm = ((immhi << 2) | immlo) << 11 >> 11;
            let base_page = (self.pc >> 12) << 12;
            *self.rw(rd) = (base_page as i64 + (imm << 12)) as u64;
            self.pc += 4; return None;
        }

        // ADD/SUB immediate (64-bit)
        if (insn >> 24) == 0b10010001 || (insn >> 24) == 0b11010001 {
            let is_sub = (insn >> 24) == 0b11010001;
            let imm12 = (insn >> 10) & 0xFFF;
            let shift = (insn >> 22) & 0x1;
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rd = (insn & 0x1F) as usize;
            let val = (imm12 as u64) << (if shift == 1 { 12 } else { 0 });
            *self.rw(rd) = if is_sub { self.r(rn).wrapping_sub(val) } else { self.r(rn).wrapping_add(val) };
            self.pc += 4; return None;
        }

        // LDR/STR (unsigned immediate, 64-bit)
        if (insn >> 24) == 0b11111001 {
            let op = (insn >> 22) & 0x1; // 0=ldr, 1=str
            let size = (insn >> 30) & 0x3;
            let imm12 = (insn >> 10) & 0xFFF;
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rt = (insn & 0x1F) as usize;
            let scale = byte_size(size);
            let addr = self.r(rn) + (imm12 as u64 * scale);
            if op == 0 { *self.rw(rt) = self.load64(addr); }
            else { self.store64(addr, self.r(rt)); }
            self.pc += 4; return None;
        }

        // LDRB (register offset, 0-bit)
        if (insn >> 22) == 0b0011100101 {
            let imm12 = (insn >> 10) & 0xFFF;
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rt = (insn & 0x1F) as usize;
            let addr = self.r(rn) + imm12 as u64;
            *self.rw(rt) = self.mem_get(addr) as u64;
            self.pc += 4; return None;
        }

        // Register-register ops: ADD, SUB, MUL, ORR, CMP(SUBS)
        // bits 31:24 = 0b10001011 (41) for ADD reg, 0b11001011 (CB) for SUB reg
        // 0b10011011 (9B) for MUL, 0b10101011 (AB) for ORR, 0b11101011 (EB) for CMP(SUBS)
        let top8 = (insn >> 24) as u8;
        let top21 = (insn >> 11) as u32;
        if top8 == 0x8B && (top21 & 0b11111111111) == 0b00001011000 {
            // ADD (shifted register)
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rm = ((insn >> 16) & 0x1F) as usize;
            let rd = (insn & 0x1F) as usize;
            *self.rw(rd) = self.r(rn).wrapping_add(self.r(rm));
            self.pc += 4; return None;
        }
        if top8 == 0xCB && (top21 & 0b11111111111) == 0b01001011000 {
            // SUB (shifted register)
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rm = ((insn >> 16) & 0x1F) as usize;
            let rd = (insn & 0x1F) as usize;
            *self.rw(rd) = self.r(rn).wrapping_sub(self.r(rm));
            self.pc += 4; return None;
        }
        if top8 == 0x9B && (top21 & 0b11111111111) == 0b00011011000 {
            // MUL
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rm = ((insn >> 16) & 0x1F) as usize;
            let rd = (insn & 0x1F) as usize;
            *self.rw(rd) = self.r(rn).wrapping_mul(self.r(rm));
            self.pc += 4; return None;
        }
        if top8 == 0xAB && (top21 & 0b11111111111) == 0b00101011000 {
            // ORR (shifted register)
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rm = ((insn >> 16) & 0x1F) as usize;
            let rd = (insn & 0x1F) as usize;
            *self.rw(rd) = self.r(rn) | self.r(rm);
            self.pc += 4; return None;
        }
        if top8 == 0xEB && (top21 & 0b11111111111) == 0b01101011000 {
            // CMP (SUBS xzr, rn, rm)
            let rn = ((insn >> 5) & 0x1F) as usize;
            let rm = ((insn >> 16) & 0x1F) as usize;
            self.set_flags_sub(self.r(rn), self.r(rm));
            self.pc += 4; return None;
        }

        Some(ExecExitReason::Fault { msg: format!("undecoded insn at 0x{:x}: 0x{:08x}", self.pc, insn) })
    }
}

fn byte_size(size: u32) -> u64 { match size { 3 => 8, 2 => 4, 1 => 2, _ => 1 } }

/// Parse ELF64, find .text and .data segments, set up initial state, and run.
pub fn run_arm64_elf(elf_bytes: &[u8]) -> ExecResult {
    if elf_bytes.len() < 64 || &elf_bytes[0..4] != b"\x7fELF" {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "not an ELF".into() }, steps: 0, state: HashMap::new() };
    }
    let e_phoff = u64::from_le_bytes(elf_bytes[32..40].try_into().unwrap()) as usize;
    let e_phnum = u16::from_le_bytes(elf_bytes[56..58].try_into().unwrap()) as usize;
    let e_entry = u64::from_le_bytes(elf_bytes[24..32].try_into().unwrap());

    // Determine max address for allocation
    let mut max_addr = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 56;
        if off + 48 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; } // PT_LOAD
        let p_vaddr = u64::from_le_bytes(elf_bytes[off + 16..off + 24].try_into().unwrap());
        let p_memsz = u64::from_le_bytes(elf_bytes[off + 40..off + 48].try_into().unwrap());
        let end = p_vaddr + p_memsz;
        if end > max_addr { max_addr = end; }
    }

    let mem_size = max_addr as usize + 0x1000;
    let mut mem = vec![0u8; mem_size];

    // Load segments
    for i in 0..e_phnum {
        let off = e_phoff + i * 56;
        if off + 48 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_offset = u64::from_le_bytes(elf_bytes[off + 8..off + 16].try_into().unwrap()) as usize;
        let p_vaddr = u64::from_le_bytes(elf_bytes[off + 16..off + 24].try_into().unwrap()) as usize;
        let p_filesz = u64::from_le_bytes(elf_bytes[off + 32..off + 40].try_into().unwrap()) as usize;
        if p_offset + p_filesz <= elf_bytes.len() {
            mem[p_vaddr..p_vaddr + p_filesz].copy_from_slice(&elf_bytes[p_offset..p_offset + p_filesz]);
        }
    }

    // Find data VA from the second PT_LOAD segment
    let mut data_va = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 56;
        if off + 48 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_flags = u32::from_le_bytes(elf_bytes[off + 4..off + 8].try_into().unwrap());
        if p_flags == 6 { // PF_R|PF_W = .data
            data_va = u64::from_le_bytes(elf_bytes[off + 16..off + 24].try_into().unwrap());
        }
    }

    // Skip the startup stub and set x15 = data_va directly.
    let mut cpu = Cpu::new(mem, e_entry + 20);
    cpu.regs[15] = data_va;
    let exit_reason = cpu.run();

    // Read state from memory at x15 (data_va)
    let mut state = HashMap::new();
    let base = cpu.regs[15] as usize;
    for slot in 0..256u16 {
        let addr = base + slot as usize * 8;
        if addr + 8 <= cpu.mem.len() {
            let val = u64::from_le_bytes(cpu.mem[addr..addr + 8].try_into().unwrap());
            if val != 0 { state.insert(slot, val); }
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}

impl Cpu {
    fn run(&mut self) -> ExecExitReason {
        loop {
            match self.step() {
                Some(r) => return r,
                None => continue,
            }
        }
    }
}