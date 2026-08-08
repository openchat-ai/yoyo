//! MIPS big-endian interpreter -- executes YOYO-emitted MIPS BE machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Big-endian ELF32. State base register t8 (r24). Slot N at [t8 + N*4].

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
    hi: u64,
    lo: u64,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(mem: Vec<u8>, entry: u64) -> Self {
        Self { regs: [0; 32], pc: entry, hi: 0, lo: 0, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT }
    }

    fn r(&self, n: usize) -> u64 { if n == 0 { 0 } else { self.regs[n] } }
    fn set_reg(&mut self, n: usize, val: u64) { if n != 0 { self.regs[n] = val; } }

    fn mem_get(&self, addr: u64) -> u8 { let a = addr as usize; if a < self.mem.len() { self.mem[a] } else { 0 } }
    fn mem_set(&mut self, addr: u64, v: u8) { let a = addr as usize; if a >= self.mem.len() { self.mem.resize(a + 1, 0); } self.mem[a] = v; }

    /// Big-endian load32
    fn load32_be(&self, addr: u64) -> u32 {
        let mut v = 0u32;
        for i in 0..4 { v |= (self.mem_get(addr + i) as u32) << (24 - i * 8); }
        v
    }

    /// Big-endian store32
    fn store32_be(&mut self, addr: u64, val: u32) {
        for i in 0..4 { self.mem_set(addr + i, ((val >> (24 - i * 8)) & 0xFF) as u8); }
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        if self.pc as usize + 4 > self.mem.len() { return Some(ExecExitReason::Halted); }
        let insn = u32::from_be_bytes(self.mem[self.pc as usize..self.pc as usize + 4].try_into().unwrap());
        self.steps += 1;

        let op = (insn >> 26) & 0x3F;
        let rs = ((insn >> 21) & 0x1F) as usize;
        let rt = ((insn >> 16) & 0x1F) as usize;
        let rd = ((insn >> 11) & 0x1F) as usize;
        let funct = insn & 0x3F;

        // SYSCALL (op=0x00, funct=0x0C)
        if op == 0x00 && funct == 0x0C {
            return Some(ExecExitReason::Ret);
        }

        // LUI (0x3C)
        if op == 0x3C {
            let imm16 = (insn & 0xFFFF) as u32;
            self.set_reg(rt, (imm16 as u64) << 16);
            self.pc += 4;
            return None;
        }

        // ORI (0x34)
        if op == 0x34 {
            let imm16 = (insn & 0xFFFF) as u64;
            self.set_reg(rt, self.r(rs) | imm16);
            self.pc += 4;
            return None;
        }

        // ADDIU (0x24)
        if op == 0x24 {
            let imm16 = ((insn & 0xFFFF) as i32) << 16 >> 16; // sign-extend 16-bit
            self.set_reg(rt, (self.r(rs) as i64 + imm16 as i64) as u64);
            self.pc += 4;
            return None;
        }

        // LW (0x8C)
        if op == 0x8C {
            let imm16 = ((insn & 0xFFFF) as i32) << 16 >> 16;
            let addr = (self.r(rs) as i64 + imm16 as i64) as u64;
            self.set_reg(rt, self.load32_be(addr) as u64);
            self.pc += 4;
            return None;
        }

        // SW (0xAC)
        if op == 0xAC {
            let imm16 = ((insn & 0xFFFF) as i32) << 16 >> 16;
            let addr = (self.r(rs) as i64 + imm16 as i64) as u64;
            self.store32_be(addr, self.r(rt) as u32);
            self.pc += 4;
            return None;
        }

        // LBU (0x90)
        if op == 0x90 {
            let imm16 = ((insn & 0xFFFF) as i32) << 16 >> 16;
            let addr = (self.r(rs) as i64 + imm16 as i64) as u64;
            self.set_reg(rt, self.mem_get(addr) as u64);
            self.pc += 4;
            return None;
        }

        // R-type (op=0x00)
        if op == 0x00 {
            match funct {
                0x00 => { // SLL (NOP when rd=0, shamt=0)
                    self.pc += 4; return None;
                }
                0x21 => { // ADDU
                    self.set_reg(rd, self.r(rs).wrapping_add(self.r(rt)));
                    self.pc += 4; return None;
                }
                0x23 => { // SUBU
                    self.set_reg(rd, self.r(rs).wrapping_sub(self.r(rt)));
                    self.pc += 4; return None;
                }
                0x25 => { // OR
                    self.set_reg(rd, self.r(rs) | self.r(rt));
                    self.pc += 4; return None;
                }
                0x19 => { // MULTU
                    let prod = (self.r(rs) as u128).wrapping_mul(self.r(rt) as u128);
                    self.lo = prod as u64;
                    self.hi = (prod >> 64) as u64;
                    self.pc += 4; return None;
                }
                0x12 => { // MFLO
                    self.set_reg(rd, self.lo);
                    self.pc += 4; return None;
                }
                0x08 => { // JR
                    let target = self.r(rs);
                    // If this is a ret (jr $ra) and $ra is 0, return Ret (initial call, no prior call)
                    if rs == 31 && target == 0 {
                        return Some(ExecExitReason::Ret);
                    }
                    self.pc = target;
                    return None;
                }
                _ => {}
            }
        }

        // J (0x08)
        if op == 0x08 {
            let target = (insn & 0x3FFFFFF) as u64;
            self.pc = (self.pc & 0xF0000000) | (target << 2);
            return None;
        }

        // JAL (0x0C)
        if op == 0x0C {
            let target = (insn & 0x3FFFFFF) as u64;
            self.set_reg(31, self.pc + 8); // ra = PC+8
            self.pc = (self.pc & 0xF0000000) | (target << 2);
            return None;
        }

        // BEQ (0x10)
        if op == 0x10 {
            let imm16 = ((insn & 0xFFFF) as i32) << 16 >> 16;
            if self.r(rs) == self.r(rt) {
                self.pc = (self.pc as i64 + (imm16 as i64) << 2) as u64;
            } else {
                self.pc += 4;
            }
            return None;
        }

        // BNE (0x14)
        if op == 0x14 {
            let imm16 = ((insn & 0xFFFF) as i32) << 16 >> 16;
            if self.r(rs) != self.r(rt) {
                self.pc = (self.pc as i64 + (imm16 as i64) << 2) as u64;
            } else {
                self.pc += 4;
            }
            return None;
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

/// Parse ELF32 (big-endian), find .text and .data segments, set up initial state, and run.
pub fn run_mips_elf(elf_bytes: &[u8]) -> ExecResult {
    if elf_bytes.len() < 52 || &elf_bytes[0..4] != b"\x7fELF" {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "not an ELF".into() }, steps: 0, state: HashMap::new() };
    }
    let e_phoff = u32::from_be_bytes(elf_bytes[28..32].try_into().unwrap()) as usize;
    let e_phnum = u16::from_be_bytes(elf_bytes[44..46].try_into().unwrap()) as usize;
    let e_entry = u32::from_be_bytes(elf_bytes[24..28].try_into().unwrap()) as u64;

    // Determine max address for allocation
    let mut max_addr = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 24 > elf_bytes.len() { break; }
        let p_type = u32::from_be_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_vaddr = u32::from_be_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as u64;
        let p_memsz = u32::from_be_bytes(elf_bytes[off + 20..off + 24].try_into().unwrap()) as u64;
        let end = p_vaddr + p_memsz;
        if end > max_addr { max_addr = end; }
    }

    let mem_size = max_addr as usize + 0x1000;
    let mut mem = vec![0u8; mem_size];

    // Load segments (big-endian ELF32)
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 24 > elf_bytes.len() { break; }
        let p_type = u32::from_be_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_offset = u32::from_be_bytes(elf_bytes[off + 4..off + 8].try_into().unwrap()) as usize;
        let p_vaddr = u32::from_be_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as usize;
        let p_filesz = u32::from_be_bytes(elf_bytes[off + 16..off + 20].try_into().unwrap()) as usize;
        if p_offset + p_filesz <= elf_bytes.len() {
            mem[p_vaddr..p_vaddr + p_filesz].copy_from_slice(&elf_bytes[p_offset..p_offset + p_filesz]);
        }
    }

    // Find data VA from the second PT_LOAD segment (p_flags = 6 for RW)
    let mut data_va = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 28 > elf_bytes.len() { break; }
        let p_type = u32::from_be_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_flags = u32::from_be_bytes(elf_bytes[off + 24..off + 28].try_into().unwrap());
        if p_flags == 6 { // PF_R|PF_W = .data
            data_va = u32::from_be_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as u64;
        }
    }

    // Skip 16-byte startup stub, set t8 (r24) = data_va
    let mut cpu = Cpu::new(mem, e_entry + 16);
    cpu.regs[24] = data_va;
    let exit_reason = cpu.run();

    // Read state from memory at t8 (data_va) �?32-bit slots, big-endian
    let mut state = HashMap::new();
    let base = cpu.regs[24] as usize;
    for slot in 0..256u16 {
        let addr = base + slot as usize * 4;
        if addr + 4 <= cpu.mem.len() {
            let val = u32::from_be_bytes(cpu.mem[addr..addr + 4].try_into().unwrap()) as u64;
            if val != 0 { state.insert(slot, val); }
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
