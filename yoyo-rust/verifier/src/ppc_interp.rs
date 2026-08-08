//! PPC64 LE interpreter -- executes YOYO-emitted PowerPC64 little-endian machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Little-endian ELF64. State base register r13. Slot N at [r13 + N*8].

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
    lr: u64,
    cr0: u8, // condition register field 0: bits: LT(0), GT(1), EQ(2), SO(3)
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(mem: Vec<u8>, entry: u64) -> Self {
        Self { regs: [0; 32], pc: entry, lr: 0, cr0: 0, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT }
    }

    fn r(&self, n: usize) -> u64 { if n == 0 { 0 } else { self.regs[n] } }
    fn rw(&mut self, n: usize) -> &mut u64 { &mut self.regs[n] }

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

    fn set_cr0(&mut self, a: i64, b: i64) {
        let diff = a - b;
        self.cr0 = 0;
        if diff < 0 { self.cr0 |= 0x8; } // LT
        if diff > 0 { self.cr0 |= 0x4; } // GT
        if diff == 0 { self.cr0 |= 0x2; } // EQ
        // SO bit remains 0
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        if self.pc as usize + 4 > self.mem.len() { return Some(ExecExitReason::Halted); }
        let insn = u32::from_le_bytes(self.mem[self.pc as usize..self.pc as usize + 4].try_into().unwrap());
        self.steps += 1;

        // PPC opcode is the top 6 bits of the instruction word (bits 26..31 of the LE u32)
        let op = (insn >> 26) & 0x3F;
        let rd = ((insn >> 21) & 0x1F) as usize;
        let ra = ((insn >> 16) & 0x1F) as usize;
        let rb = ((insn >> 11) & 0x1F) as usize;
        let rt = rd;
        let imm16 = (insn & 0xFFFF) as u32;
        let xo = (insn >> 1) & 0x3FF;

        // SC (opcode 17 = 0x11) �?system call, treat as exit
        if op == 0x11 {
            return Some(ExecExitReason::Ret);
        }

        // BLR (0x4E800020)
        if insn == 0x4E800020 {
            // If LR is 0, this is a top-level ret (no prior call)
            if self.lr == 0 {
                return Some(ExecExitReason::Ret);
            }
            self.pc = self.lr;
            return None;
        }

        // B / BL (opcode 18 = 0x12)
        // Encoding: opcode(6) | LI(24) | AA(1) | LK(1)
        if op == 0x12 {
            let li = (insn & 0x3FFFFFC) as i32;
            let li = (li << 6) >> 6; // sign-extend 24-bit (shifted left 2, then right)
            let aa = (insn >> 1) & 1;
            let lk = insn & 1;
            if lk != 0 { self.lr = self.pc + 4; }
            if aa != 0 { self.pc = li as u64; }
            else { self.pc = (self.pc as i64 + li as i64) as u64; }
            return None;
        }

        // Conditional branch (opcode 16 = 0x10)
        // Encoding: opcode(6) | BO(5) | BI(5) | BD(14) | AA(1) | LK(1)
        if op == 0x10 {
            let bo = (insn >> 21) & 0x1F;
            let bi = ((insn >> 16) & 0x1F) as usize;
            let bd = (insn & 0xFFFC) as i32;
            let bd = (bd << 16) >> 16; // sign-extend 14-bit
            let lk = insn & 1;

            // Determine condition
            let cr_bit = (self.cr0 >> (3 - bi)) & 1;
            let cond_true = if (bo & 0x4) != 0 {
                // BO[2] = 1: branch if condition true
                cr_bit != 0
            } else {
                // BO[2] = 0: branch if condition false
                cr_bit == 0
            };

            if lk != 0 { self.lr = self.pc + 4; }
            if cond_true {
                self.pc = (self.pc as i64 + bd as i64) as u64;
            } else {
                self.pc += 4;
            }
            return None;
        }

        // ADDIS (opcode 15 = 0x0F, first byte 0x3C)
        if op == 0x0F {
            let imm = (imm16 as u64) << 16;
            let val = if ra == 0 { 0 } else { self.r(ra) };
            *self.rw(rt) = val.wrapping_add(imm);
            self.pc += 4;
            return None;
        }

        // ORI (opcode 24 = 0x18, first byte 0x60)
        if op == 0x18 {
            *self.rw(ra) = self.r(ra) | (imm16 as u64);
            self.pc += 4;
            return None;
        }

        // ADDI (opcode 14 = 0x0E, first byte 0x38)
        if op == 0x0E {
            let imm = (imm16 as i32) << 16 >> 16;
            let val = if ra == 0 { 0 } else { self.r(ra) };
            *self.rw(rt) = (val as i64 + imm as i64) as u64;
            self.pc += 4;
            return None;
        }

        // LD (opcode 58 = 0x3A, first byte 0xE8) �?DS-form
        if op == 0x3A {
            let ds = (insn >> 2) & 0x3FFF; // 14 bits
            let ds = ((ds as i32) << 18) >> 18; // sign-extend
            let addr = (self.r(ra) as i64 + (ds as i64) * 4) as u64;
            *self.rw(rt) = self.load64(addr);
            self.pc += 4;
            return None;
        }

        // STD (opcode 62 = 0x3E, first byte 0xF8) �?DS-form
        if op == 0x3E {
            let ds = (insn >> 2) & 0x3FFF; // 14 bits
            let ds = ((ds as i32) << 18) >> 18; // sign-extend
            let addr = (self.r(ra) as i64 + (ds as i64) * 4) as u64;
            self.store64(addr, self.r(rd));
            self.pc += 4;
            return None;
        }

        // Opcode 31 (0x1F) �?ALU register instructions
        if op == 0x1F {
            match xo {
                0x0A => { // ADD (0x7C000214)
                    *self.rw(rd) = self.r(ra).wrapping_add(self.r(rb));
                    self.pc += 4; return None;
                }
                0x01 => { // SUBF (0x7C000050)
                    *self.rw(rd) = self.r(rb).wrapping_sub(self.r(ra));
                    self.pc += 4; return None;
                }
                0x1B => { // OR (0x7C000378)
                    *self.rw(ra) = self.r(ra) | self.r(rb);
                    self.pc += 4; return None;
                }
                0x0D => { // MULLD (0x7C0001D2)
                    *self.rw(rd) = self.r(ra).wrapping_mul(self.r(rb));
                    self.pc += 4; return None;
                }
                0x00 => { // CMP / CMPD
                    let bf = (insn >> 23) & 0x7; // CR field
                    let l = (insn >> 21) & 1; // 0=32-bit, 1=64-bit
                    if bf == 0 {
                        self.set_cr0(self.r(ra) as i64, self.r(rb) as i64);
                    }
                    self.pc += 4; return None;
                }
                _ => {}
            }
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
pub fn run_ppc_elf(elf_bytes: &[u8]) -> ExecResult {
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
        if p_type != 1 { continue; }
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

    // Find data VA from the second PT_LOAD segment (p_flags=6 = PF_R|PF_W)
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

    // Skip 16-byte startup stub, set r13 = data_va
    let mut cpu = Cpu::new(mem, e_entry + 16);
    cpu.regs[13] = data_va;
    let exit_reason = cpu.run();

    // Read state from memory at r13 (data_va) �?64-bit slots
    let mut state = HashMap::new();
    let base = cpu.regs[13] as usize;
    for slot in 0..256u16 {
        let addr = base + slot as usize * 8;
        if addr + 8 <= cpu.mem.len() {
            let val = u64::from_le_bytes(cpu.mem[addr..addr + 8].try_into().unwrap());
            if val != 0 { state.insert(slot, val); }
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
