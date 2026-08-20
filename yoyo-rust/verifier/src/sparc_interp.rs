//! SPARC v8 interpreter — executes YOYO-emitted SPARC BE machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Big-endian ELF32. State access via g1 (r1) as address base, slot N at [g1 + N*4].
//! No startup preamble — entry at 0x10000, data_va at 0x20000.

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
    icc_z: bool,
}

impl Cpu {
    fn new(mem: Vec<u8>, entry: u64) -> Self {
        Self { regs: [0; 32], pc: entry, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT, icc_z: false }
    }

    fn r(&self, n: usize) -> u64 { self.regs[n] }
    fn rw(&mut self, n: usize) -> &mut u64 { &mut self.regs[n] }

    fn mem_get(&self, addr: u64) -> u8 { let a = addr as usize; if a < self.mem.len() { self.mem[a] } else { 0 } }
    fn mem_set(&mut self, addr: u64, v: u8) { let a = addr as usize; if a >= self.mem.len() { self.mem.resize(a + 1, 0); } self.mem[a] = v; }

    fn load32_be(&self, addr: u64) -> u32 {
        let mut v = 0u32;
        for i in 0..4 { v |= (self.mem_get(addr + i) as u32) << (24 - i * 8); }
        v
    }

    fn store32_be(&mut self, addr: u64, val: u32) {
        for i in 0..4 { self.mem_set(addr + i, ((val >> (24 - i * 8)) & 0xFF) as u8); }
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        if self.pc as usize + 4 > self.mem.len() { return Some(ExecExitReason::Halted); }
        let insn = u32::from_be_bytes(self.mem[self.pc as usize..self.pc as usize + 4].try_into().unwrap());
        self.steps += 1;

        let op = insn >> 30;
        let rd = ((insn >> 25) & 0x1F) as usize;
        let rs1 = ((insn >> 14) & 0x1F) as usize;
        let i_bit = (insn >> 13) & 1;
        // Register form: rs2 in bits[4:0]; immediate form: simm13 in bits[12:0]
        let rs2_or_imm = if i_bit == 1 { insn & 0x1FFF } else { insn & 0x1F };

        // NOP (sethi 0, %g0) = 0x01000000
        if insn == 0x01000000 || insn == 0x00000000 {
            self.pc += 4; return None;
        }

        // RET (jmpl %i7+8, %g0) = 0x81C3E008
        // decode: op=2, rd=0, op3=0b111000, rs1=31(i7), i=1, imm=8
        // General: JMPL, RET, RETL, etc.
        // JMPL rd, [rs1+imm]: op=2, op3=0b111000 (0x38)
        // RET: rd=0, rs1=31, i=1, imm=8 → 0x81C3E008
        if op == 2 && ((insn >> 19) & 0x3F) == 0x38 {
            let imm = if i_bit == 1 { (insn & 0x1FFF) as i32 as i64 } else { self.r(rs2_or_imm as usize) as i64 };
            let target = (self.r(rs1) as i64 + imm) as u64;
            // RET (jmpl %o7+8, %g0): rd=0, rs1=15, target=8 → top-level ret if o7 is 0
            if rd == 0 && rs1 == 15 && self.r(15) == 0 {
                return Some(ExecExitReason::Ret);
            }
            // If rd=0 (no link), just jump
            // If rs1=0 and target=0, this is a top-level ret (no prior call)
            if rd == 0 && rs1 == 0 && target == 0 {
                return Some(ExecExitReason::Ret);
            }
            self.pc = target;
            return None;
        }

        // CALL (op=1): disp30
        if op == 1 {
            let disp30 = insn & 0x3FFFFFFF;
            let disp = ((disp30 as i32) << 2) as i64;
            *self.rw(15) = self.pc + 4; // o7 = return address
            self.pc = (self.pc as i64 + disp as i64) as u64;
            return None;
        }

        // SETHI (op=0, op2=0b100=4)
        if op == 0 && ((insn >> 22) & 0x7) == 4 {
            // SETHI: op=0, rd, op2=0b100, imm22
            let imm22 = insn & 0x3FFFFF;
            *self.rw(rd) = (imm22 as u64) << 10;
            self.pc += 4; return None;
        }

        // Bicc (op=0, op2=0b010=2)
        if op == 0 && ((insn >> 22) & 0x7) == 2 {
            let cond = (insn >> 25) & 0xF;
            let disp22 = insn & 0x3FFFFF;
            let disp = ((disp22 as i32) << 10) >> 10; // signed word offset
            let taken = match cond {
                0 => false,           // BN
                1 => self.icc_z,      // BE
                2 => !self.icc_z,     // BNE
                8 => true,            // BA
                _ => false,
            };
            if taken {
                self.pc = (self.pc as i64 + ((disp as i64) << 2)) as u64;
            } else {
                self.pc += 4;
            }
            return None;
        }

        // Arithmetic (op=2, op3 varied)
        if op == 2 {
            let op3 = (insn >> 19) & 0x3F;
            let imm_val: i64 = if i_bit == 1 {
                let imm13 = insn & 0x1FFF;
                if imm13 & 0x1000 != 0 { (imm13 as i64) - 0x2000 } else { imm13 as i64 }
            } else {
                self.r(rs2_or_imm as usize) as i64
            };

            match op3 {
                0x00 => { // ADD
                    *self.rw(rd) = (self.r(rs1) as i64 + imm_val) as u64;
                    self.pc += 4; return None;
                }
                0x04 => { // SUB
                    *self.rw(rd) = (self.r(rs1) as i64 - imm_val) as u64;
                    self.pc += 4; return None;
                }
                0x01 => { // AND
                    if i_bit == 1 {
                        *self.rw(rd) = self.r(rs1) & (imm_val as u64 & 0x1FFF);
                    } else {
                        *self.rw(rd) = self.r(rs1) & self.r(rs2_or_imm as usize);
                    }
                    self.pc += 4; return None;
                }
                0x02 => { // OR
                    if i_bit == 1 {
                        *self.rw(rd) = self.r(rs1) | (imm_val as u64 & 0x1FFF);
                    } else {
                        *self.rw(rd) = self.r(rs1) | self.r(rs2_or_imm as usize);
                    }
                    self.pc += 4; return None;
                }
                0x05 => { // ANDN
                    if i_bit == 1 {
                        *self.rw(rd) = self.r(rs1) & !(imm_val as u64 & 0x1FFF);
                    } else {
                        *self.rw(rd) = self.r(rs1) & !self.r(rs2_or_imm as usize);
                    }
                    self.pc += 4; return None;
                }
                0x0E => { // SMUL
                    let a = self.r(rs1) as i32 as i64;
                    let b = if i_bit == 1 { imm_val } else { self.r(rs2_or_imm as usize) as i32 as i64 };
                    *self.rw(rd) = (a.wrapping_mul(b)) as u64;
                    self.pc += 4; return None;
                }
                0x25 => { // SLL
                    let shift = if i_bit == 1 { (imm_val as u32 & 0x1F) as u64 } else { self.r(rs2_or_imm as usize) & 0x1F };
                    *self.rw(rd) = self.r(rs1) << shift;
                    self.pc += 4; return None;
                }
                0x26 => { // SRL
                    let shift = if i_bit == 1 { (imm_val as u32 & 0x1F) as u64 } else { self.r(rs2_or_imm as usize) & 0x1F };
                    *self.rw(rd) = self.r(rs1) >> shift;
                    self.pc += 4; return None;
                }
                0x14 => { // SUBcc
                    let result = (self.r(rs1) as i64 - imm_val) as u64;
                    if rd != 0 { *self.rw(rd) = result; }
                    self.icc_z = result == 0;
                    self.pc += 4; return None;
                }
                _ => {}
            }
        }

        // Load/Store (op=3)
        if op == 3 {
            let op3 = (insn >> 19) & 0x3F;
            let imm13 = if i_bit == 1 {
                (insn & 0x1FFF) as i32 as i64
            } else {
                self.r(rs2_or_imm as usize) as i64
            };
            let addr = (self.r(rs1) as i64 + imm13) as u64;

            match op3 {
                0x00 => { // LD
                    *self.rw(rd) = self.load32_be(addr) as u64;
                    self.pc += 4; return None;
                }
                0x04 => { // ST
                    self.store32_be(addr, self.r(rd) as u32);
                    self.pc += 4; return None;
                }
                0x01 => { // LDUB
                    *self.rw(rd) = self.mem_get(addr) as u64;
                    self.pc += 4; return None;
                }
                0x05 => { // STB
                    self.mem_set(addr, self.r(rd) as u8);
                    self.pc += 4; return None;
                }
                _ => {}
            }
        }

        // TA 0 (trap always, used as exit): 0x91D02000
        // op=2, rd=8(%g0), op3=0b111010, rs1=0, i=1, imm=0
        if insn & 0xFFFFFFC0 == 0x91D02000 {
            // Actually 0x91D02000 = ta 0x00
            return Some(ExecExitReason::Ret);
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
pub fn run_sparc_elf(elf_bytes: &[u8]) -> ExecResult {
    if elf_bytes.len() < 52 || &elf_bytes[0..4] != b"\x7fELF" {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "not an ELF".into() }, steps: 0, state: HashMap::new() };
    }
    if elf_bytes[4] != 1 { // ELFCLASS32
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "not a 32-bit ELF".into() }, steps: 0, state: HashMap::new() };
    }
    let e_phoff = u32::from_be_bytes(elf_bytes[28..32].try_into().unwrap()) as usize;
    let e_phnum = u16::from_be_bytes(elf_bytes[44..46].try_into().unwrap()) as usize;
    let e_entry = u32::from_be_bytes(elf_bytes[24..28].try_into().unwrap()) as u64;

    // Determine max address for allocation
    let mut max_addr = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 32 > elf_bytes.len() { break; }
        let p_type = u32::from_be_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_vaddr = u32::from_be_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as u64;
        let p_memsz = u32::from_be_bytes(elf_bytes[off + 20..off + 24].try_into().unwrap()) as u64;
        let end = p_vaddr + p_memsz;
        if end > max_addr { max_addr = end; }
    }

    let mem_size = max_addr as usize + 0x1000;
    let mut mem = vec![0u8; mem_size];

    // Load segments
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 32 > elf_bytes.len() { break; }
        let p_type = u32::from_be_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_offset = u32::from_be_bytes(elf_bytes[off + 4..off + 8].try_into().unwrap()) as usize;
        let p_vaddr = u32::from_be_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as usize;
        let p_filesz = u32::from_be_bytes(elf_bytes[off + 16..off + 20].try_into().unwrap()) as usize;
        if p_offset + p_filesz <= elf_bytes.len() {
            mem[p_vaddr..p_vaddr + p_filesz].copy_from_slice(&elf_bytes[p_offset..p_offset + p_filesz]);
        }
    }

    // Find data VA from the second PT_LOAD segment (PF_R|PF_W = 6)
    let mut data_va = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 32 > elf_bytes.len() { break; }
        let p_type = u32::from_be_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_flags = u32::from_be_bytes(elf_bytes[off + 24..off + 28].try_into().unwrap());
        if p_flags == 6 { // PF_R|PF_W = .data
            data_va = u32::from_be_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as u64;
        }
    }

    // No startup preamble — code starts at entry. Set g1 = data_va
    let mut cpu = Cpu::new(mem, e_entry);
    cpu.regs[1] = data_va; // g1 = state base
    let exit_reason = cpu.run();

    // Read state from memory at data_va
    let mut state = HashMap::new();
    let base = data_va as usize;
    for slot in 0..256u16 {
        let addr = base + slot as usize * 4;
        if addr + 4 <= cpu.mem.len() {
            let val = u32::from_be_bytes(cpu.mem[addr..addr + 4].try_into().unwrap()) as u64;
            if val != 0 { state.insert(slot, val); }
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}