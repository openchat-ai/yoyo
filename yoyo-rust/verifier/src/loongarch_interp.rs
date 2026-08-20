//! LoongArch LA64 interpreter — executes YOYO-emitted LoongArch machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Little-endian ELF64. State lives at data_va (0x120010000), slot N at [data_va + N*8].
//! No startup preamble — code starts at entry (0x120000000).

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

struct Cpu {
    regs: [u64; 32],
    pc: u64,
    mem: HashMap<u64, u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(entry: u64) -> Self {
        Self { regs: [0; 32], pc: entry, mem: HashMap::new(), steps: 0, step_limit: DEFAULT_STEP_LIMIT }
    }

    fn r(&self, n: usize) -> u64 { if n == 0 { 0 } else { self.regs[n] } }
    fn set_reg(&mut self, n: usize, v: u64) { if n != 0 { self.regs[n] = v; } }

    fn mem_get(&self, addr: u64) -> u8 { self.mem.get(&addr).copied().unwrap_or(0) }
    fn mem_set(&mut self, addr: u64, v: u8) { self.mem.insert(addr, v); }

    fn load64(&self, addr: u64) -> u64 {
        let mut v = 0u64;
        for i in 0..8 { v |= (self.mem_get(addr + i) as u64) << (i * 8); }
        v
    }

    fn store64(&mut self, addr: u64, val: u64) {
        for i in 0..8 { self.mem_set(addr + i, ((val >> (i * 8)) & 0xFF) as u8); }
    }

    fn load_insn(&self, addr: u64) -> Option<u32> {
        let b0 = self.mem.get(&addr)?;
        let b1 = self.mem.get(&(addr + 1)).copied().unwrap_or(0);
        let b2 = self.mem.get(&(addr + 2)).copied().unwrap_or(0);
        let b3 = self.mem.get(&(addr + 3)).copied().unwrap_or(0);
        Some(u32::from_le_bytes([*b0, b1, b2, b3]))
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        let Some(insn) = self.load_insn(self.pc) else {
            return Some(ExecExitReason::Halted);
        };
        self.steps += 1;

        let op = (insn >> 22) & 0x3FF;
        let rd = (insn & 0x1F) as usize;
        let rj = ((insn >> 5) & 0x1F) as usize;
        let rk = ((insn >> 10) & 0x1F) as usize; // 3R form: rk at bits[14:10]
        let op15 = (insn >> 15) & 0x1FFFF;
        let si12 = (((insn >> 10) & 0xFFF) as i32) << 20 >> 20;
        let si12 = si12 as i64;
        let offs16 = (((insn >> 10) & 0xFFFF) as i32) << 16 >> 16;
        let offs16 = offs16 as i64;
        let offs26 = (insn & 0x3FFFFFF) as i64;

        // NOP (andn r0, r0, r0) = 0x00000000
        if insn == 0x00000000 {
            self.pc += 4; return None;
        }

        // SYSCALL (0x0000000D): used as exit
        if insn == 0x0000000D {
            return Some(ExecExitReason::Ret);
        }

        // RET (jirl r0, ra, 0) = 0x04C00020
        if insn == 0x04C00020 {
            // jirl zero, ra, 0 — if ra is 0, top-level ret
            if self.r(1) == 0 {
                return Some(ExecExitReason::Ret);
            }
            self.pc = self.r(1);
            return None;
        }

        // lu12i.w: opcode bits[31:25]=0x0A, si20 in [24:5] — check before generic op match
        // because si20 high bits overlap a 10-bit (insn>>22) field.
        if ((insn >> 25) & 0x7F) == 0x0A {
            let si20 = (insn >> 5) & 0xFFFFF;
            // Sign-extend the 32-bit {si20,12'b0} to 64-bit (LoongArch lu12i.w)
            let v32 = si20 << 12;
            let v = v32 as i32 as i64 as u64;
            self.set_reg(rd, v);
            self.pc += 4; return None;
        }

        // beq/bne use 6-bit opcode at [31:26] (010110 / 010111); check before 10-bit op match
        let op6 = (insn >> 26) & 0x3F;
        if op6 == 0x16 {
            // beq rj, rd, offs16 — rj[9:5], rd[4:0]
            let rj2 = ((insn >> 5) & 0x1F) as usize;
            let rd2 = (insn & 0x1F) as usize;
            if self.r(rj2) == self.r(rd2) {
                self.pc = (self.pc as i64 + (offs16 << 2)) as u64;
            } else {
                self.pc += 4;
            }
            return None;
        }
        if op6 == 0x17 {
            let rj2 = ((insn >> 5) & 0x1F) as usize;
            let rd2 = (insn & 0x1F) as usize;
            if self.r(rj2) != self.r(rd2) {
                self.pc = (self.pc as i64 + (offs16 << 2)) as u64;
            } else {
                self.pc += 4;
            }
            return None;
        }

        match op {
            0x038 => { // ori rd, rj, ui12
                let ui12 = (insn >> 10) & 0xFFF;
                self.set_reg(rd, self.r(rj) | ui12 as u64);
                self.pc += 4; return None;
            }
            0x029 => { // addi.d rd, rj, si12
                self.set_reg(rd, (self.r(rj) as i64 + si12) as u64);
                self.pc += 4; return None;
            }
            0x28C => { // ld.d rd, rj, si12
                let addr = (self.r(rj) as i64 + si12) as u64;
                self.set_reg(rd, self.load64(addr));
                self.pc += 4; return None;
            }
            0x29D => { // st.d rd, rj, si12
                let addr = (self.r(rj) as i64 + si12) as u64;
                self.store64(addr, self.r(rd));
                self.pc += 4; return None;
            }
            0x28D => { // ld.b rd, rj, si12
                let addr = (self.r(rj) as i64 + si12) as u64;
                self.set_reg(rd, self.mem_get(addr) as u64);
                self.pc += 4; return None;
            }
            0x13 => { // jirl rd, rj, offs16 (offs16 << 2 for byte offset)
                let target = (self.r(rj) as i64 + (offs16 << 2)) as u64;
                self.set_reg(rd, self.pc + 4);
                self.pc = target;
                return None;
            }
            0x10 => { // b offs26 (offs26 << 2 for byte offset)
                self.pc = (self.pc as i64 + (offs26 << 2)) as u64;
                return None;
            }
            0x11 => { // bl offs26 (offs26 << 2 for byte offset)
                self.set_reg(1, self.pc + 4); // ra = return address
                self.pc = (self.pc as i64 + (offs26 << 2)) as u64;
                return None;
            }
            // 0x16/0x17 beq/bne handled above via op6
            _ => {}
        }

        // 3R ALU: opcode in bits[31:15]
        match op15 {
            0x21 => { // add.d
                self.set_reg(rd, self.r(rj).wrapping_add(self.r(rk)));
                self.pc += 4; return None;
            }
            0x23 => { // sub.d
                self.set_reg(rd, self.r(rj).wrapping_sub(self.r(rk)));
                self.pc += 4; return None;
            }
            0x2A => { // or
                self.set_reg(rd, self.r(rj) | self.r(rk));
                self.pc += 4; return None;
            }
            0x39 => { // mul.d
                self.set_reg(rd, self.r(rj).wrapping_mul(self.r(rk)));
                self.pc += 4; return None;
            }
            0x24 => { // slt
                self.set_reg(rd, if (self.r(rj) as i64) < (self.r(rk) as i64) { 1 } else { 0 });
                self.pc += 4; return None;
            }
            0x25 => { // sltu
                self.set_reg(rd, if self.r(rj) < self.r(rk) { 1 } else { 0 });
                self.pc += 4; return None;
            }
            _ => {}
        }

        // slli.d rd, rj, ui6: opcode[31:16]=0x0041, ui6 in bits[15:10]
        if ((insn >> 16) & 0xFFFF) == 0x0041 {
            let ui6 = ((insn >> 10) & 0x3F) as u32;
            self.set_reg(rd, self.r(rj) << (ui6 & 63));
            self.pc += 4; return None;
        }

        Some(ExecExitReason::Fault { msg: format!("undecoded insn at 0x{:x}: 0x{:08x}", self.pc, insn) })
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

/// Parse ELF64, find .text and .data segments, set up initial state, and run.
/// Uses sparse HashMap memory — LoongArch VAs are ~0x120000000 (too large for a flat Vec).
pub fn run_loongarch_elf(elf_bytes: &[u8]) -> ExecResult {
    if elf_bytes.len() < 64 || &elf_bytes[0..4] != b"\x7fELF" {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "not an ELF".into() }, steps: 0, state: HashMap::new() };
    }
    let e_phoff = u64::from_le_bytes(elf_bytes[32..40].try_into().unwrap()) as usize;
    let e_phnum = u16::from_le_bytes(elf_bytes[56..58].try_into().unwrap()) as usize;
    let e_entry = u64::from_le_bytes(elf_bytes[24..32].try_into().unwrap());

    let mut cpu = Cpu::new(e_entry);

    let mut data_va = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 56;
        if off + 48 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_flags = u32::from_le_bytes(elf_bytes[off + 4..off + 8].try_into().unwrap());
        let p_offset = u64::from_le_bytes(elf_bytes[off + 8..off + 16].try_into().unwrap()) as usize;
        let p_vaddr = u64::from_le_bytes(elf_bytes[off + 16..off + 24].try_into().unwrap());
        let p_filesz = u64::from_le_bytes(elf_bytes[off + 32..off + 40].try_into().unwrap()) as usize;
        if p_flags == 6 {
            data_va = p_vaddr;
        }
        if p_offset + p_filesz <= elf_bytes.len() {
            for j in 0..p_filesz {
                cpu.mem_set(p_vaddr + j as u64, elf_bytes[p_offset + j]);
            }
        }
    }

    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    for slot in 0..256u16 {
        let addr = data_va + slot as u64 * 8;
        let val = cpu.load64(addr);
        if val != 0 { state.insert(slot, val); }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}