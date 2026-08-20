//! ARM32 (ARMv7 EABI) interpreter — executes YOYO-emitted ARM32 LE machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! Little-endian ELF32. State base register r8 (set by 16-byte startup).
//! Startup: movw r8, #lo16(data_va); movt r8, #hi16(data_va); b <user_code>; NOP
//! Entry at 0x8001000, data_va at 0x8002000.

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
    regs: [u64; 16],
    pc: u64,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(mem: Vec<u8>, entry: u64) -> Self {
        Self { regs: [0; 16], pc: entry, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT }
    }

    fn r(&self, n: usize) -> u32 { self.regs[n] as u32 }
    fn rw(&mut self, n: usize) -> &mut u64 { &mut self.regs[n] }

    fn mem_get(&self, addr: u64) -> u8 { let a = addr as usize; if a < self.mem.len() { self.mem[a] } else { 0 } }
    fn mem_set(&mut self, addr: u64, v: u8) { let a = addr as usize; if a >= self.mem.len() { self.mem.resize(a + 1, 0); } self.mem[a] = v; }

    fn load32(&self, addr: u64) -> u32 {
        let mut v = 0u32;
        for i in 0..4 { v |= (self.mem_get(addr + i) as u32) << (i * 8); }
        v
    }

    fn store32(&mut self, addr: u64, val: u32) {
        for i in 0..4 { self.mem_set(addr + i, ((val >> (i * 8)) & 0xFF) as u8); }
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        if self.pc as usize + 4 > self.mem.len() { return Some(ExecExitReason::Halted); }
        let insn = u32::from_le_bytes(self.mem[self.pc as usize..self.pc as usize + 4].try_into().unwrap());
        self.steps += 1;

        let cond = insn >> 28;
        let opcode = (insn >> 25) & 0x7;
        let rd = ((insn >> 12) & 0xF) as usize;
        let rn = ((insn >> 16) & 0xF) as usize;
        let rm = (insn & 0xF) as usize;

        // NOP (mov r0, r0) = 0xE1A00000
        if insn == 0xE1A00000 {
            self.pc += 4; return None;
        }

        // SWI (used as exit): 0xEF000000 (cond=0xE, opcode=0xF, imm24=0)
        if (insn & 0xFF000000) == 0xEF000000 && (insn & 0xFFFFFF) == 0 {
            return Some(ExecExitReason::Ret);
        }

        // BX lr (ret): 0xE12FFF1E
        if insn == 0xE12FFF1E {
            // If lr (r14) is 0, this is a top-level ret
            if self.r(14) == 0 {
                return Some(ExecExitReason::Ret);
            }
            self.pc = self.r(14) as u64;
            return None;
        }

        // B (branch) / BL (branch with link): cond=0xE, opcode=0x5 (BL = 0x5, B = 0x5 with L=0)
        // Actually B: cond=0xE, 1, 0, 1, 0, L=0, imm24
        // BL: cond=0xE, 1, 0, 1, 0, L=1, imm24
        // B: 0xEA000000 | imm24; BL: 0xEB000000 | imm24
        if (insn & 0x0F000000) == 0x0A000000 {
            let l_bit = (insn >> 24) & 1;
            let imm24 = insn & 0xFFFFFF;
            let disp = ((imm24 as i32) << 8) >> 6; // sign-extend 24-bit, then multiply by 4
            // Actually ARM branch offset is imm24 << 2, sign-extended
            let disp = ((imm24 << 2) as i32) << 6 >> 6;
            let target = (self.pc as i64 + 8 + disp as i64) as u64;
            if l_bit == 1 {
                *self.rw(14) = self.pc + 4; // lr = return address
            }
            self.pc = target;
            return None;
        }

        // B{cond} (conditional branch): 0x0A000000 | (cond << 28) | imm24
        // But B{cond} uses cond != 0xE, and the encoding is cond:28, 1, 0, 1, 0, imm24
        if ((insn >> 24) & 0xF) == 0xA && cond != 0xE {
            let imm24 = insn & 0xFFFFFF;
            let disp = ((imm24 << 2) as i32) << 6 >> 6;
            let target = (self.pc as i64 + 8 + disp as i64) as u64;
            let taken = match cond {
                0 => self.r(rd) == 0,      // BEQ
                1 => self.r(rd) != 0,      // BNE
                10 => (self.r(rd) as i32) >= 0, // BGE (not actually used with CMP result)
                11 => (self.r(rd) as i32) < 0,  // BLT
                12 => false, // BGT (approximate)
                13 => false, // BLE (approximate)
                _ => false,
            };
            if taken {
                self.pc = target;
            } else {
                self.pc += 4;
            }
            return None;
        }

        // MOVW rd, #imm16: bits[19:16]=imm[15:12], bits[11:0]=imm[11:0]
        if (insn & 0x0FF00000) == 0x03000000 {
            let imm16 = (((insn >> 16) & 0xF) << 12) | (insn & 0xFFF);
            let rd = ((insn >> 12) & 0xF) as usize;
            *self.rw(rd) = imm16 as u64;
            self.pc += 4; return None;
        }

        // MOVT rd, #imm16: same imm16 split as MOVW
        if (insn & 0x0FF00000) == 0x03400000 {
            let imm16 = (((insn >> 16) & 0xF) << 12) | (insn & 0xFFF);
            let rd = ((insn >> 12) & 0xF) as usize;
            *self.rw(rd) = (self.r(rd) & 0xFFFF) as u64 | ((imm16 as u64) << 16);
            self.pc += 4; return None;
        }

        // MOV rd, #imm (ARM immediate): 0xE3A00000 | (rd << 12) | imm8_rotated
        if (insn & 0x0FF00000) == 0x03A00000 {
            let imm8 = insn & 0xFF;
            let rotate = ((insn >> 8) & 0xF) as u32;
            let imm = (imm8 >> (rotate * 2)) | (imm8 << (32 - rotate * 2));
            *self.rw(rd) = imm as u64;
            self.pc += 4; return None;
        }

        // LDR rd, [rn, #imm12]: 0xE5900000 | (rd << 12) | (rn << 16) | imm12
        if (insn & 0x0F000000) == 0x05000000 && (insn & 0x00100000) == 0x00100000 {
            // bits 27-24 = 0101 (data transfer), bit 20 = 1 (L=load), bit 22 = 0 (B=word)
            let imm12 = insn & 0xFFF;
            let u_bit = (insn >> 23) & 1;
            let addr = if u_bit == 1 { self.r(rn) as u64 + imm12 as u64 } else { self.r(rn) as u64 - imm12 as u64 };
            *self.rw(rd) = self.load32(addr) as u64;
            self.pc += 4; return None;
        }

        // STR rd, [rn, #imm12]: 0xE5800000 | (rd << 12) | (rn << 16) | imm12
        if (insn & 0x0F000000) == 0x05000000 && (insn & 0x00100000) == 0x00000000 {
            // bits 27-24 = 0101, bit 20 = 0 (L=store), bit 22 = 0 (B=word)
            let imm12 = insn & 0xFFF;
            let u_bit = (insn >> 23) & 1;
            let addr = if u_bit == 1 { self.r(rn) as u64 + imm12 as u64 } else { self.r(rn) as u64 - imm12 as u64 };
            self.store32(addr, self.r(rd));
            self.pc += 4; return None;
        }

        // LDRB rd, [rn, #imm12]: 0xE5D00000 | (rd << 12) | (rn << 16) | imm12
        if (insn & 0x0F200000) == 0x05D00000 {
            let imm12 = insn & 0xFFF;
            let u_bit = (insn >> 23) & 1;
            let addr = if u_bit == 1 { self.r(rn) as u64 + imm12 as u64 } else { self.r(rn) as u64 - imm12 as u64 };
            *self.rw(rd) = self.mem_get(addr) as u64;
            self.pc += 4; return None;
        }

        // ADD rd, rn, rm: 0xE0800000 | (rd << 12) | (rn << 16) | rm
        if (insn & 0x0FE00010) == 0x00800000 {
            *self.rw(rd) = self.r(rn).wrapping_add(self.r(rm)) as u64;
            self.pc += 4; return None;
        }

        // SUB rd, rn, rm: 0xE0400000 | (rd << 12) | (rn << 16) | rm
        if (insn & 0x0FE00010) == 0x00400000 {
            *self.rw(rd) = self.r(rn).wrapping_sub(self.r(rm)) as u64;
            self.pc += 4; return None;
        }

        // ADD rd, rn, #imm: 0xE2800000 | (rd << 12) | (rn << 16) | imm8_rotated
        if (insn & 0x0FE00000) == 0x02800000 {
            let imm8 = insn & 0xFF;
            let rotate = (insn >> 8) & 0xF;
            let imm = (imm8 >> (rotate * 2)) | (imm8 << (32 - rotate * 2));
            *self.rw(rd) = (self.r(rn) as u64 + imm as u64) as u64;
            self.pc += 4; return None;
        }

        // SUB rd, rn, #imm: 0xE2400000 | (rd << 12) | (rn << 16) | imm8_rotated
        if (insn & 0x0FE00000) == 0x02400000 {
            let imm8 = insn & 0xFF;
            let rotate = (insn >> 8) & 0xF;
            let imm = (imm8 >> (rotate * 2)) | (imm8 << (32 - rotate * 2));
            *self.rw(rd) = (self.r(rn) as u64 - imm as u64) as u64;
            self.pc += 4; return None;
        }

        // CMP rn, rm: 0xE1500000 | (rn << 16) | rm
        if (insn & 0x0FE00010) == 0x01500000 {
            let result = self.r(rn).wrapping_sub(self.r(rm));
            // Store result in r0 (or a scratch) for branch to check
            *self.rw(0) = result as u64;
            self.pc += 4; return None;
        }

        // MUL rd, rn, rm: 0xE0000090 | (rd << 16) | (rn << 8) | rm
        // Actually: 0xE0000090 | (rd << 16) | (rn) | (rm << 8)
        if (insn & 0x0FC000F0) == 0x00000090 {
            let rn = ((insn >> 8) & 0xF) as usize;
            let rd = ((insn >> 16) & 0xF) as usize;
            let rm = (insn & 0xF) as usize;
            *self.rw(rd) = self.r(rn).wrapping_mul(self.r(rm)) as u64;
            self.pc += 4; return None;
        }

        // ORR rd, rn, rm: 0xE1800000 | (rd << 12) | (rn << 16) | rm
        if (insn & 0x0FE00010) == 0x01800000 {
            *self.rw(rd) = (self.r(rn) | self.r(rm)) as u64;
            self.pc += 4; return None;
        }

        // AND rd, rn, rm: 0xE0000000 | (rd << 12) | (rn << 16) | rm
        if (insn & 0x0FE00010) == 0x00000000 {
            *self.rw(rd) = (self.r(rn) & self.r(rm)) as u64;
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

/// Parse ELF32, find .text and .data segments, set up initial state, and run.
pub fn run_arm32_elf(elf_bytes: &[u8]) -> ExecResult {
    if elf_bytes.len() < 52 || &elf_bytes[0..4] != b"\x7fELF" {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "not an ELF".into() }, steps: 0, state: HashMap::new() };
    }
    if elf_bytes[4] != 1 {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "not a 32-bit ELF".into() }, steps: 0, state: HashMap::new() };
    }
    let e_phoff = u32::from_le_bytes(elf_bytes[28..32].try_into().unwrap()) as usize;
    let e_phnum = u16::from_le_bytes(elf_bytes[44..46].try_into().unwrap()) as usize;
    let e_entry = u32::from_le_bytes(elf_bytes[24..28].try_into().unwrap()) as u64;

    let mut max_addr = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 32 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_vaddr = u32::from_le_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as u64;
        let p_memsz = u32::from_le_bytes(elf_bytes[off + 20..off + 24].try_into().unwrap()) as u64;
        let end = p_vaddr + p_memsz;
        if end > max_addr { max_addr = end; }
    }

    let mem_size = max_addr as usize + 0x1000;
    let mut mem = vec![0u8; mem_size];

    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 32 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_offset = u32::from_le_bytes(elf_bytes[off + 4..off + 8].try_into().unwrap()) as usize;
        let p_vaddr = u32::from_le_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as usize;
        let p_filesz = u32::from_le_bytes(elf_bytes[off + 16..off + 20].try_into().unwrap()) as usize;
        if p_offset + p_filesz <= elf_bytes.len() {
            mem[p_vaddr..p_vaddr + p_filesz].copy_from_slice(&elf_bytes[p_offset..p_offset + p_filesz]);
        }
    }

    let mut data_va = 0u64;
    for i in 0..e_phnum {
        let off = e_phoff + i * 32;
        if off + 32 > elf_bytes.len() { break; }
        let p_type = u32::from_le_bytes(elf_bytes[off..off + 4].try_into().unwrap());
        if p_type != 1 { continue; }
        let p_flags = u32::from_le_bytes(elf_bytes[off + 24..off + 28].try_into().unwrap());
        if p_flags == 6 {
            data_va = u32::from_le_bytes(elf_bytes[off + 8..off + 12].try_into().unwrap()) as u64;
        }
    }

    // Skip 16-byte startup: movw r8, lo16(data_va); movt r8, hi16(data_va); b <user_code>; NOP
    let mut cpu = Cpu::new(mem, e_entry + 16);
    cpu.regs[8] = data_va; // r8 = state base
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    let base = cpu.regs[8] as usize;
    for slot in 0..256u16 {
        let addr = base + slot as usize * 4;
        if addr + 4 <= cpu.mem.len() {
            let val = u32::from_le_bytes(cpu.mem[addr..addr + 4].try_into().unwrap()) as u64;
            if val != 0 { state.insert(slot, val); }
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}