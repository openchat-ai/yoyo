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
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(mem: Vec<u8>, entry: u64) -> Self {
        Self { regs: [0; 32], pc: entry, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT }
    }

    fn r(&self, n: usize) -> u64 { if n == 0 { 0 } else { self.regs[n] } }
    fn set_reg(&mut self, n: usize, v: u64) { if n != 0 { self.regs[n] = v; } }

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

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        if self.pc as usize + 4 > self.mem.len() { return Some(ExecExitReason::Halted); }
        let insn = u32::from_le_bytes(self.mem[self.pc as usize..self.pc as usize + 4].try_into().unwrap());
        self.steps += 1;

        let op = (insn >> 22) & 0x3FF;
        let rd = (insn & 0x1F) as usize;
        let rj = ((insn >> 5) & 0x1F) as usize;
        let rk = ((insn >> 27) & 0x1F) as usize;
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

        match op {
            0x14 => { // lu12i.w rd, si20
                let si20 = (insn >> 5) & 0xFFFFF;
                self.set_reg(rd, (si20 as u64) << 12);
                self.pc += 4; return None;
            }
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
            0x04 => { // add.d rd, rj, rk
                self.set_reg(rd, self.r(rj).wrapping_add(self.r(rk)));
                self.pc += 4; return None;
            }
            0x05 => { // sub.d rd, rj, rk
                self.set_reg(rd, self.r(rj).wrapping_sub(self.r(rk)));
                self.pc += 4; return None;
            }
            0x19 => { // or rd, rj, rk
                self.set_reg(rd, self.r(rj) | self.r(rk));
                self.pc += 4; return None;
            }
            0x0B => { // mul.d rd, rj, rk
                self.set_reg(rd, self.r(rj).wrapping_mul(self.r(rk)));
                self.pc += 4; return None;
            }
            0x1D => { // slt rd, rj, rk (signed)
                self.set_reg(rd, if (self.r(rj) as i64) < (self.r(rk) as i64) { 1 } else { 0 });
                self.pc += 4; return None;
            }
            0x1E => { // sltu rd, rj, rk
                self.set_reg(rd, if self.r(rj) < self.r(rk) { 1 } else { 0 });
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
            0x16 => { // beq rj, rd, offs16 (offs16 << 2)
                let rj2 = rj;
                let rd2 = rd;
                if self.r(rj2) == self.r(rd2) {
                    self.pc = (self.pc as i64 + (offs16 << 2)) as u64;
                } else {
                    self.pc += 4;
                }
                return None;
            }
            0x17 => { // bne rj, rd, offs16 (offs16 << 2)
                let rj2 = rj;
                let rd2 = rd;
                if self.r(rj2) != self.r(rd2) {
                    self.pc = (self.pc as i64 + (offs16 << 2)) as u64;
                } else {
                    self.pc += 4;
                }
                return None;
            }
            _ => {}
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
pub fn run_loongarch_elf(elf_bytes: &[u8]) -> ExecResult {
    if elf_bytes.len() < 64 || &elf_bytes[0..4] != b"\x7fELF" {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "not an ELF".into() }, steps: 0, state: HashMap::new() };
    }
    let e_phoff = u64::from_le_bytes(elf_bytes[32..40].try_into().unwrap()) as usize;
    let e_phnum = u16::from_le_bytes(elf_bytes[56..58].try_into().unwrap()) as usize;
    let e_entry = u64::from_le_bytes(elf_bytes[24..32].try_into().unwrap());

    let mut max_addr = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 56;
        if off + 48 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_vaddr = u64::from_le_bytes(elf_bytes[off + 16..off + 24].try_into().unwrap());
        let p_memsz = u64::from_le_bytes(elf_bytes[off + 40..off + 48].try_into().unwrap());
        let end = p_vaddr + p_memsz;
        if end > max_addr { max_addr = end; }
    }

    let mem_size = max_addr as usize + 0x1000;
    let mut mem = vec![0u8; mem_size];

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

    // Find data VA from the second PT_LOAD segment (PF_R|PF_W = 6)
    let mut data_va = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 56;
        if off + 48 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_flags = u32::from_le_bytes(elf_bytes[off + 4..off + 8].try_into().unwrap());
        if p_flags == 6 {
            data_va = u64::from_le_bytes(elf_bytes[off + 16..off + 24].try_into().unwrap());
        }
    }

    // No startup preamble — code starts at entry
    let mut cpu = Cpu::new(mem, e_entry);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    let base = data_va as usize;
    for slot in 0..256u16 {
        let addr = base + slot as usize * 8;
        if addr + 8 <= cpu.mem.len() {
            let val = u64::from_le_bytes(cpu.mem[addr..addr + 8].try_into().unwrap());
            if val != 0 { state.insert(slot, val); }
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}