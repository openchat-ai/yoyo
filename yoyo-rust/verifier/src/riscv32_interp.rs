//! RISC-V RV32 interpreter -- executes YOYO-emitted RISC-V 32-bit machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Same as RV64 but:
//!   - 32-bit registers (stored in u64, masked to 32 bits)
//!   - lw/sw instead of ld/sd
//!   - Entry at 0x8001000 + 16 (skip startup)

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
    fn set_reg(&mut self, n: usize, val: u64) { if n != 0 { self.regs[n] = val; } }
    /// Mask a register value to 32 bits
    fn mask32(&self, n: usize) -> u64 { if n == 0 { 0 } else { self.regs[n] & 0xFFFFFFFF } }

    fn mem_get(&self, addr: u64) -> u8 { let a = addr as usize; if a < self.mem.len() { self.mem[a] } else { 0 } }
    fn mem_set(&mut self, addr: u64, v: u8) { let a = addr as usize; if a >= self.mem.len() { self.mem.resize(a + 1, 0); } self.mem[a] = v; }

    fn load32(&self, addr: u64) -> u32 {
        let mut v = 0u32;
        for i in 0..4 { v |= (self.mem_get(addr + i) as u32) << (i * 8); }
        v
    }

    fn load16(&self, addr: u64) -> u16 {
        let mut v = 0u16;
        for i in 0..2 { v |= (self.mem_get(addr + i) as u16) << (i * 8); }
        v
    }

    fn store32(&mut self, addr: u64, val: u32) {
        for i in 0..4 { self.mem_set(addr + i, ((val >> (i * 8)) & 0xFF) as u8); }
    }

    fn store16(&mut self, addr: u64, val: u16) {
        for i in 0..2 { self.mem_set(addr + i, ((val >> (i * 8)) & 0xFF) as u8); }
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        if self.pc as usize + 4 > self.mem.len() { return Some(ExecExitReason::Halted); }
        let insn = u32::from_le_bytes(self.mem[self.pc as usize..self.pc as usize + 4].try_into().unwrap());
        self.steps += 1;

        let opcode = insn & 0x7F;
        let rd = ((insn >> 7) & 0x1F) as usize;
        let rs1 = ((insn >> 15) & 0x1F) as usize;
        let rs2 = ((insn >> 20) & 0x1F) as usize;
        let funct3 = (insn >> 12) & 0x7;
        let funct7 = (insn >> 25) & 0x7F;

        // ECALL (0x73 with funct3=0, imm=0)
        if insn == 0x00000073 {
            return Some(ExecExitReason::Ret);
        }

        // LUI (0x37)
        if opcode == 0x37 {
            let imm = (insn & 0xFFFFF000) as u64;
            self.set_reg(rd, imm);
            self.pc += 4;
            return None;
        }

        // AUIPC (0x17)
        if opcode == 0x17 {
            let imm = (insn & 0xFFFFF000) as i64 >> 12 << 12;
            self.set_reg(rd, (self.pc as i64 + imm) as u64 & 0xFFFFFFFF);
            self.pc += 4;
            return None;
        }

        // JAL (0x6F)
        if opcode == 0x6F {
            let imm = decode_jal_imm(insn);
            let target = ((self.pc as i64 + imm as i64) as u64) & 0xFFFFFFFF;
            self.set_reg(rd, (self.pc + 4) & 0xFFFFFFFF);
            self.pc = target;
            return None;
        }

        // JALR (0x67)
        if opcode == 0x67 {
            let imm12 = ((insn >> 20) as i32) << 20 >> 20;
            let temp = (self.pc + 4) & 0xFFFFFFFF;
            let target = ((self.r(rs1) as i64 + imm12 as i64) as u64) & !1;
            self.set_reg(rd, temp);
            // If this is a ret (jalr x0, x1, 0) and x1 is 0, this is a top-level ret (no prior call)
            if rd == 0 && rs1 == 1 && imm12 == 0 && self.r(rs1) == 0 {
                return Some(ExecExitReason::Ret);
            }
            self.pc = target;
            return None;
        }

        // Branch (opcode 0x63)
        if opcode == 0x63 {
            let imm = decode_b_imm(insn);
            let cond = match funct3 {
                0 => self.mask32(rs1) == self.mask32(rs2),   // BEQ
                1 => self.mask32(rs1) != self.mask32(rs2),   // BNE
                4 => (self.mask32(rs1) as i32) < (self.mask32(rs2) as i32), // BLT
                5 => (self.mask32(rs1) as i32) >= (self.mask32(rs2) as i32), // BGE
                6 => self.mask32(rs1) < self.mask32(rs2),     // BLTU
                7 => self.mask32(rs1) >= self.mask32(rs2),    // BGEU
                _ => false,
            };
            if cond {
                self.pc = ((self.pc as i64 + imm as i64) as u64) & 0xFFFFFFFF;
            } else {
                self.pc += 4;
            }
            return None;
        }

        // Load (opcode 0x03)
        if opcode == 0x03 {
            let imm12 = ((insn >> 20) as i32) << 20 >> 20;
            let addr = ((self.r(rs1) as i64 + imm12 as i64) as u64) & 0xFFFFFFFF;
            let val = match funct3 {
                0 => self.mem_get(addr) as u64,           // LB (sign-extend)
                1 => self.load16(addr) as u64,             // LH (sign-extend)
                2 => self.load32(addr) as u64,             // LW (sign-extend)
                4 => self.mem_get(addr) as u64,            // LBU
                5 => self.load16(addr) as u64,             // LHU
                _ => return Some(ExecExitReason::Fault { msg: format!("bad load funct3 at 0x{:x}", self.pc) }),
            };
            self.set_reg(rd, val);
            self.pc += 4;
            return None;
        }

        // Store (opcode 0x23)
        if opcode == 0x23 {
            let imm_lo = (insn >> 7) & 0x1F;
            let imm_hi = (insn >> 25) & 0x7F;
            let imm12 = ((imm_hi << 5) | imm_lo) as i32;
            let imm12 = (imm12 << 20) >> 20;
            let addr = ((self.r(rs1) as i64 + imm12 as i64) as u64) & 0xFFFFFFFF;
            let val = self.mask32(rs2);
            match funct3 {
                0 => self.mem_set(addr, val as u8),        // SB
                1 => self.store16(addr, val as u16),       // SH
                2 => self.store32(addr, val as u32),       // SW
                _ => return Some(ExecExitReason::Fault { msg: format!("bad store funct3 at 0x{:x}", self.pc) }),
            }
            self.pc += 4;
            return None;
        }

        // ALU immediate (opcode 0x13)
        if opcode == 0x13 {
            let imm12 = ((insn >> 20) as i32) << 20 >> 20;
            let rs1v = self.mask32(rs1);
            let result = match funct3 {
                0 => ((rs1v as i32).wrapping_add(imm12)) as u64 & 0xFFFFFFFF, // ADDI
                1 => (rs1v << ((insn >> 20) & 0x1F)) & 0xFFFFFFFF,           // SLLI (32-bit shamt)
                2 => {
                    if (rs1v as i32) < imm12 { 1 } else { 0 }                // SLTI
                }
                3 => { if rs1v < (imm12 as u32 as u64) { 1 } else { 0 } }   // SLTIU
                4 => (rs1v ^ (imm12 as u32 as u64)) & 0xFFFFFFFF,            // XORI
                5 => {
                    let shamt = (insn >> 20) & 0x1F;
                    let shift_type = (insn >> 30) & 0x1;
                    if shift_type == 0 { rs1v >> shamt }                     // SRLI
                    else { ((rs1v as i32) >> shamt) as u64 & 0xFFFFFFFF }   // SRAI
                }
                6 => (rs1v | (imm12 as u32 as u64)) & 0xFFFFFFFF,            // ORI
                7 => (rs1v & (imm12 as u32 as u64)) & 0xFFFFFFFF,            // ANDI
                _ => 0,
            };
            self.set_reg(rd, result);
            self.pc += 4;
            return None;
        }

        // ALU register (opcode 0x33)
        if opcode == 0x33 {
            let rs1v = self.mask32(rs1);
            let rs2v = self.mask32(rs2);
            let result = match (funct3, funct7) {
                (0, 0x00) => (rs1v.wrapping_add(rs2v)) & 0xFFFFFFFF,            // ADD
                (0, 0x20) => (rs1v.wrapping_sub(rs2v)) & 0xFFFFFFFF,            // SUB
                (1, 0x00) => (rs1v << (rs2v & 0x1F)) & 0xFFFFFFFF,             // SLL
                (2, 0x00) => if (rs1v as i32) < (rs2v as i32) { 1 } else { 0 }, // SLT
                (3, 0x00) => if rs1v < rs2v { 1 } else { 0 },                   // SLTU
                (4, 0x00) => (rs1v ^ rs2v) & 0xFFFFFFFF,                       // XOR
                (5, 0x00) => rs1v >> (rs2v & 0x1F),                            // SRL
                (5, 0x20) => ((rs1v as i32) >> (rs2v & 0x1F)) as u64 & 0xFFFFFFFF, // SRA
                (6, 0x00) => (rs1v | rs2v) & 0xFFFFFFFF,                       // OR
                (7, 0x00) => (rs1v & rs2v) & 0xFFFFFFFF,                       // AND
                (0, 0x01) => (rs1v.wrapping_mul(rs2v)) & 0xFFFFFFFF,           // MUL
                (1, 0x01) => {
                    // MULH: (signed)signed
                    let a = rs1v as i32 as i64;
                    let b = rs2v as i32 as i64;
                    ((a.wrapping_mul(b) >> 32) as u64) & 0xFFFFFFFF
                }
                (2, 0x01) => {
                    // MULHSU: signed x unsigned
                    let a = rs1v as i32 as i64;
                    let b = rs2v as i64;
                    ((a.wrapping_mul(b) >> 32) as u64) & 0xFFFFFFFF
                }
                (3, 0x01) => {
                    // MULHU: unsigned
                    ((rs1v as u64).wrapping_mul(rs2v as u64) >> 32) & 0xFFFFFFFF
                }
                (4, 0x01) => if rs2v == 0 { 0 } else { (rs1v.wrapping_div(rs2v)) & 0xFFFFFFFF }, // DIV
                (5, 0x01) => if rs2v == 0 { 0 } else { (rs1v.wrapping_div(rs2v)) & 0xFFFFFFFF }, // DIVU
                (6, 0x01) => if rs2v == 0 { rs1v } else { (rs1v.wrapping_rem(rs2v)) & 0xFFFFFFFF }, // REM
                (7, 0x01) => if rs2v == 0 { rs1v } else { (rs1v.wrapping_rem(rs2v)) & 0xFFFFFFFF }, // REMU
                _ => return Some(ExecExitReason::Fault { msg: format!("undecoded ALU reg at 0x{:x}: 0x{:08x}", self.pc, insn) }),
            };
            self.set_reg(rd, result);
            self.pc += 4;
            return None;
        }

        // FENCE (opcode 0x0F) �?treat as NOP
        if opcode == 0x0F {
            self.pc += 4;
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

fn decode_jal_imm(insn: u32) -> i32 {
    let imm = ((insn >> 31) & 0x1) << 20
        | ((insn >> 21) & 0x3FF) << 1
        | ((insn >> 20) & 0x1) << 11
        | ((insn >> 12) & 0xFF) << 12;
    (imm as i32) << 11 >> 11
}

fn decode_b_imm(insn: u32) -> i32 {
    let imm = ((insn >> 31) & 0x1) << 12
        | ((insn >> 25) & 0x3F) << 5
        | ((insn >> 8) & 0xF) << 1
        | ((insn >> 7) & 0x1) << 11;
    (imm as i32) << 19 >> 19
}

/// Parse ELF32, find .text and .data segments, set up initial state, and run.
pub fn run_riscv32_elf(elf_bytes: &[u8]) -> ExecResult {
    if elf_bytes.len() < 52 || &elf_bytes[0..4] != b"\x7fELF" {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "not an ELF".into() }, steps: 0, state: HashMap::new() };
    }
    let e_phoff = u32::from_le_bytes(elf_bytes[28..32].try_into().unwrap()) as usize;
    let e_phnum = u16::from_le_bytes(elf_bytes[44..46].try_into().unwrap()) as usize;
    let e_entry = u32::from_le_bytes(elf_bytes[24..28].try_into().unwrap()) as u64;

    // Determine max address for allocation
    let mut max_addr = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 24 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_vaddr = u32::from_le_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as u64;
        let p_memsz = u32::from_le_bytes(elf_bytes[off + 20..off + 24].try_into().unwrap()) as u64;
        let end = p_vaddr + p_memsz;
        if end > max_addr { max_addr = end; }
    }

    let mem_size = max_addr as usize + 0x1000;
    let mut mem = vec![0u8; mem_size];

    // Load segments
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 24 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_offset = u32::from_le_bytes(elf_bytes[off + 4..off + 8].try_into().unwrap()) as usize;
        let p_vaddr = u32::from_le_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as usize;
        let p_filesz = u32::from_le_bytes(elf_bytes[off + 16..off + 20].try_into().unwrap()) as usize;
        if p_offset + p_filesz <= elf_bytes.len() {
            mem[p_vaddr..p_vaddr + p_filesz].copy_from_slice(&elf_bytes[p_offset..p_offset + p_filesz]);
        }
    }

    // Find data VA from the second PT_LOAD segment (p_flags = 6 for RW)
    let mut data_va = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 24 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_flags = u32::from_le_bytes(elf_bytes[off + 24..off + 28].try_into().unwrap());
        if p_flags == 6 { // PF_R|PF_W = .data
            data_va = u32::from_le_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as u64;
        }
    }

    // Skip 16-byte startup stub, set x5 = data_va
    let mut cpu = Cpu::new(mem, e_entry + 16);
    cpu.regs[5] = data_va;
    let exit_reason = cpu.run();

    // Read state from memory at x5 (data_va) �?32-bit slots stored as 32-bit values
    let mut state = HashMap::new();
    let base = cpu.regs[5] as usize;
    for slot in 0..256u16 {
        let addr = base + slot as usize * 4;
        if addr + 4 <= cpu.mem.len() {
            let val = u32::from_le_bytes(cpu.mem[addr..addr + 4].try_into().unwrap()) as u64;
            if val != 0 { state.insert(slot, val); }
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}
