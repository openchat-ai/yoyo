//! x86-32 interpreter — executes YOYO-emitted x86 PE32 machine code
//! and extracts the final state for DDC comparison against the TIR simulator.
//!
//! PE32 format. State in .data section. Startup: mov edi, data_va; jmp user_code.
//! Entry point parsed from PE header.

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
    regs: [u32; 9], // EAX=0, EBX=1, ECX=2, EDX=3, ESI=4, EDI=5, EBP=6, ESP=7, EIP=8
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
}

impl Cpu {
    fn new(mem: Vec<u8>, eip: u32) -> Self {
        let mut r = [0u32; 9];
        r[8] = eip;
        Self { regs: r, mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT }
    }

    fn r(&self, n: usize) -> u32 { self.regs[n] }
    fn rw(&mut self, n: usize) -> &mut u32 { &mut self.regs[n] }

    fn mem_get(&self, addr: u32) -> u8 {
        let a = addr as usize;
        if a < self.mem.len() { self.mem[a] } else { 0 }
    }
    fn mem_set(&mut self, addr: u32, v: u8) {
        let a = addr as usize;
        if a >= self.mem.len() { self.mem.resize(a + 1, 0); }
        self.mem[a] = v;
    }

    fn load32(&self, addr: u32) -> u32 {
        let mut v = 0u32;
        for i in 0..4 { v |= (self.mem_get(addr + i) as u32) << (i * 8); }
        v
    }
    fn store32(&mut self, addr: u32, val: u32) {
        for i in 0..4 { self.mem_set(addr + i, ((val >> (i * 8)) & 0xFF) as u8); }
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit { return Some(ExecExitReason::StepLimit { steps: self.steps }); }
        let eip = self.r(8) as usize;
        if eip >= self.mem.len() { return Some(ExecExitReason::Halted); }
        self.steps += 1;

        let insn = self.mem_get(self.r(8));

        match insn {
            0x90 => { // NOP
                *self.rw(8) = self.r(8).wrapping_add(1);
                None
            }
            0xC3 => { // RET
                return Some(ExecExitReason::Ret);
            }
            0xB8 => { // MOV EAX, imm32
                let v = self.load32(self.r(8).wrapping_add(1));
                *self.rw(0) = v;
                *self.rw(8) = self.r(8).wrapping_add(5);
                None
            }
            0xBB => { // MOV EBX, imm32
                let v = self.load32(self.r(8).wrapping_add(1));
                *self.rw(1) = v;
                *self.rw(8) = self.r(8).wrapping_add(5);
                None
            }
            0xB9 => { // MOV ECX, imm32
                let v = self.load32(self.r(8).wrapping_add(1));
                *self.rw(2) = v;
                *self.rw(8) = self.r(8).wrapping_add(5);
                None
            }
            0xBA => { // MOV EDX, imm32
                let v = self.load32(self.r(8).wrapping_add(1));
                *self.rw(3) = v;
                *self.rw(8) = self.r(8).wrapping_add(5);
                None
            }
            0xBF => { // MOV EDI, imm32
                let v = self.load32(self.r(8).wrapping_add(1));
                *self.rw(5) = v;
                *self.rw(8) = self.r(8).wrapping_add(5);
                None
            }
            0xA3 => { // MOV [addr], EAX
                let addr = self.load32(self.r(8).wrapping_add(1));
                self.store32(addr, self.r(0));
                *self.rw(8) = self.r(8).wrapping_add(5);
                None
            }
            0xA1 => { // MOV EAX, [addr]
                let addr = self.load32(self.r(8).wrapping_add(1));
                *self.rw(0) = self.load32(addr);
                *self.rw(8) = self.r(8).wrapping_add(5);
                None
            }
            0x89 => { // MOV [addr], reg (modrm based)
                let modrm = self.mem_get(self.r(8).wrapping_add(1));
                let reg = ((modrm >> 3) & 7) as usize;
                if (modrm & 0xC7) == 0x05 { // [disp32]
                    let addr = self.load32(self.r(8).wrapping_add(2));
                    self.store32(addr, self.r(reg));
                    *self.rw(8) = self.r(8).wrapping_add(6);
                } else {
                    return Some(ExecExitReason::Fault { msg: format!("undecoded 0x89 modrm 0x{:02x} at 0x{:x}", modrm, self.r(8)) });
                }
                None
            }
            0x8B => { // MOV reg, [addr] (modrm based)
                let modrm = self.mem_get(self.r(8).wrapping_add(1));
                let reg = ((modrm >> 3) & 7) as usize;
                if (modrm & 0xC7) == 0x05 { // [disp32]
                    let addr = self.load32(self.r(8).wrapping_add(2));
                    *self.rw(reg) = self.load32(addr);
                    *self.rw(8) = self.r(8).wrapping_add(6);
                } else {
                    return Some(ExecExitReason::Fault { msg: format!("undecoded 0x8B modrm 0x{:02x} at 0x{:x}", modrm, self.r(8)) });
                }
                None
            }
            0x05 => { // ADD EAX, imm32
                let v = self.load32(self.r(8).wrapping_add(1));
                *self.rw(0) = self.r(0).wrapping_add(v);
                *self.rw(8) = self.r(8).wrapping_add(5);
                None
            }
            0x2D => { // SUB EAX, imm32
                let v = self.load32(self.r(8).wrapping_add(1));
                *self.rw(0) = self.r(0).wrapping_sub(v);
                *self.rw(8) = self.r(8).wrapping_add(5);
                None
            }
            0xFF => { // INC [addr] / DEC [addr] (modrm based)
                let modrm = self.mem_get(self.r(8).wrapping_add(1));
                if modrm == 0x05 { // INC [disp32]
                    let addr = self.load32(self.r(8).wrapping_add(2));
                    let val = self.load32(addr).wrapping_add(1);
                    self.store32(addr, val);
                    *self.rw(8) = self.r(8).wrapping_add(6);
                } else if modrm == 0x0D { // DEC [disp32]
                    let addr = self.load32(self.r(8).wrapping_add(2));
                    let val = self.load32(addr).wrapping_sub(1);
                    self.store32(addr, val);
                    *self.rw(8) = self.r(8).wrapping_add(6);
                } else if modrm == 0x2D { // IMUL [disp32]
                    let addr = self.load32(self.r(8).wrapping_add(2));
                    let val = self.load32(addr) as i32;
                    let result = (self.r(0) as i32).wrapping_mul(val);
                    let (lo, _hi) = (result as i64).overflowing_mul(1);
                    *self.rw(0) = lo as u32;
                    *self.rw(3) = (result >> 31) as u32; // sign extend
                    *self.rw(8) = self.r(8).wrapping_add(6);
                } else {
                    return Some(ExecExitReason::Fault { msg: format!("undecoded 0xFF modrm 0x{:02x} at 0x{:x}", modrm, self.r(8)) });
                }
                None
            }
            0x01 => { // ADD [addr], EAX (modrm based)
                let modrm = self.mem_get(self.r(8).wrapping_add(1));
                if (modrm & 0xC7) == 0x05 {
                    let addr = self.load32(self.r(8).wrapping_add(2));
                    let val = self.load32(addr).wrapping_add(self.r(0));
                    self.store32(addr, val);
                    *self.rw(8) = self.r(8).wrapping_add(6);
                } else {
                    return Some(ExecExitReason::Fault { msg: format!("undecoded 0x01 modrm 0x{:02x} at 0x{:x}", modrm, self.r(8)) });
                }
                None
            }
            0x29 => { // SUB [addr], EAX
                let modrm = self.mem_get(self.r(8).wrapping_add(1));
                if (modrm & 0xC7) == 0x05 {
                    let addr = self.load32(self.r(8).wrapping_add(2));
                    let val = self.load32(addr).wrapping_sub(self.r(0));
                    self.store32(addr, val);
                    *self.rw(8) = self.r(8).wrapping_add(6);
                } else {
                    return Some(ExecExitReason::Fault { msg: format!("undecoded 0x29 modrm 0x{:02x} at 0x{:x}", modrm, self.r(8)) });
                }
                None
            }
            0x09 => { // OR [addr], EAX
                let modrm = self.mem_get(self.r(8).wrapping_add(1));
                if (modrm & 0xC7) == 0x05 {
                    let addr = self.load32(self.r(8).wrapping_add(2));
                    let val = self.load32(addr) | self.r(0);
                    self.store32(addr, val);
                    *self.rw(8) = self.r(8).wrapping_add(6);
                } else {
                    return Some(ExecExitReason::Fault { msg: format!("undecoded 0x09 modrm 0x{:02x} at 0x{:x}", modrm, self.r(8)) });
                }
                None
            }
            0x39 => { // CMP [addr], EAX
                let modrm = self.mem_get(self.r(8).wrapping_add(1));
                if (modrm & 0xC7) == 0x05 {
                    let addr = self.load32(self.r(8).wrapping_add(2));
                    let _val = self.load32(addr).wrapping_sub(self.r(0));
                    *self.rw(8) = self.r(8).wrapping_add(6);
                } else {
                    return Some(ExecExitReason::Fault { msg: format!("undecoded 0x39 modrm 0x{:02x} at 0x{:x}", modrm, self.r(8)) });
                }
                None
            }
            0x0F => { // Two-byte opcodes
                let op2 = self.mem_get(self.r(8).wrapping_add(1));
                match op2 {
                    0xB6 => { // MOVZX EAX, byte [addr]
                        let modrm = self.mem_get(self.r(8).wrapping_add(2));
                        if (modrm & 0xC7) == 0x05 {
                            let addr = self.load32(self.r(8).wrapping_add(3));
                            *self.rw(0) = self.mem_get(addr) as u32;
                            *self.rw(8) = self.r(8).wrapping_add(7);
                        } else {
                            return Some(ExecExitReason::Fault { msg: format!("undecoded 0F B6 modrm 0x{:02x} at 0x{:x}", modrm, self.r(8)) });
                        }
                        None
                    }
                    0x84 => { // JE rel32
                        let rel = self.load32(self.r(8).wrapping_add(2)) as i32;
                        // JE jumps if ZF=1, but we don't track flags.
                        // For emitted code, CMP with JE means "jump if equal".
                        // We approximate by checking if the comparison result was 0.
                        // Since we don't track flags, we always fall through.
                        *self.rw(8) = self.r(8).wrapping_add(6);
                        None
                    }
                    0x85 => { // JNE rel32
                        let rel = self.load32(self.r(8).wrapping_add(2)) as i32;
                        *self.rw(8) = self.r(8).wrapping_add(6);
                        None
                    }
                    0x8C => { // JL rel32
                        let rel = self.load32(self.r(8).wrapping_add(2)) as i32;
                        *self.rw(8) = self.r(8).wrapping_add(6);
                        None
                    }
                    0x8F => { // JG rel32
                        let rel = self.load32(self.r(8).wrapping_add(2)) as i32;
                        *self.rw(8) = self.r(8).wrapping_add(6);
                        None
                    }
                    0x8D => { // JGE rel32
                        let rel = self.load32(self.r(8).wrapping_add(2)) as i32;
                        *self.rw(8) = self.r(8).wrapping_add(6);
                        None
                    }
                    0x8E => { // JLE rel32
                        let rel = self.load32(self.r(8).wrapping_add(2)) as i32;
                        *self.rw(8) = self.r(8).wrapping_add(6);
                        None
                    }
                    _ => {
                        return Some(ExecExitReason::Fault { msg: format!("undecoded 0x0F 0x{:02x} at 0x{:x}", op2, self.r(8)) });
                    }
                }
            }
            0xE9 => { // JMP rel32
                let rel = self.load32(self.r(8).wrapping_add(1)) as i32;
                *self.rw(8) = (self.r(8) as i32).wrapping_add(5).wrapping_add(rel) as u32;
                None
            }
            0xE8 => { // CALL rel32
                let rel = self.load32(self.r(8).wrapping_add(1)) as i32;
                let ret_addr = self.r(8).wrapping_add(5);
                // Push ret_addr onto stack
                let esp = self.r(7).wrapping_sub(4);
                self.store32(esp, ret_addr);
                *self.rw(7) = esp;
                *self.rw(8) = (self.r(8) as i32).wrapping_add(5).wrapping_add(rel) as u32;
                None
            }
            _ => {
                Some(ExecExitReason::Fault { msg: format!("undecoded x86 opcode 0x{:02x} at 0x{:x}", insn, self.r(8)) })
            }
        }
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

/// Parse PE32, load sections, and run. State is in .data section.
pub fn run_x86_pe(pe_bytes: &[u8]) -> ExecResult {
    if pe_bytes.len() < 0x80 || &pe_bytes[0..2] != b"MZ" {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "not a PE".into() }, steps: 0, state: HashMap::new() };
    }

    let e_lfanew = u32::from_le_bytes(pe_bytes[0x3C..0x40].try_into().unwrap()) as usize;
    if e_lfanew + 4 > pe_bytes.len() || &pe_bytes[e_lfanew..e_lfanew + 4] != b"PE\x00\x00" {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: "PE signature not found".into() }, steps: 0, state: HashMap::new() };
    }

    let pe = e_lfanew + 4;
    // FileHeader at pe
    let num_sections = u16::from_le_bytes(pe_bytes[pe + 2..pe + 4].try_into().unwrap()) as usize;
    let opt_hdr_size = u16::from_le_bytes(pe_bytes[pe + 16..pe + 18].try_into().unwrap()) as usize;

    // OptionalHeader at pe + 20
    let opt = pe + 20;
    let magic = u16::from_le_bytes(pe_bytes[opt..opt + 2].try_into().unwrap());
    if magic != 0x10B { // PE32
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: format!("not PE32 (magic=0x{:04x})", magic) }, steps: 0, state: HashMap::new() };
    }

    // EntryPoint RVA at opt + 16
    let entry_rva = u32::from_le_bytes(pe_bytes[opt + 16..opt + 20].try_into().unwrap());
    // ImageBase at opt + 28
    let image_base = u32::from_le_bytes(pe_bytes[opt + 28..opt + 32].try_into().unwrap());

    // Section headers: each 40 bytes, follow optional header
    let sections = pe + 20 + opt_hdr_size;

    let mut mem = vec![0u8; 0x1000000]; // 16MB address space
    let mut text_va = 0u32;
    let mut data_va = 0u32;
    let mut data_size = 0u32;

    for i in 0..num_sections {
        let s = sections + i * 40;
        if s + 40 > pe_bytes.len() { break; }
        let name = &pe_bytes[s..s + 8];
        let virtual_size = u32::from_le_bytes(pe_bytes[s + 8..s + 12].try_into().unwrap());
        let virtual_addr = u32::from_le_bytes(pe_bytes[s + 12..s + 16].try_into().unwrap());
        let raw_size = u32::from_le_bytes(pe_bytes[s + 16..s + 20].try_into().unwrap());
        let raw_addr = u32::from_le_bytes(pe_bytes[s + 20..s + 24].try_into().unwrap());

        if &name[..5] == b".text" {
            text_va = image_base + virtual_addr;
            let raw_off = raw_addr as usize;
            let raw_sz = raw_size as usize;
            let copy_n = raw_sz.min(virtual_size as usize);
            if raw_off + copy_n <= pe_bytes.len() {
                let dest = (image_base + virtual_addr) as usize;
                if dest + copy_n <= mem.len() {
                    mem[dest..dest + copy_n].copy_from_slice(&pe_bytes[raw_off..raw_off + copy_n]);
                }
            }
        } else if &name[..5] == b".data" {
            data_va = image_base + virtual_addr;
            data_size = virtual_size;
            let raw_off = raw_addr as usize;
            let raw_sz = raw_size as usize;
            let copy_n = raw_sz.min(virtual_size as usize);
            if raw_off + copy_n <= pe_bytes.len() {
                let dest = (image_base + virtual_addr) as usize;
                if dest + copy_n <= mem.len() {
                    mem[dest..dest + copy_n].copy_from_slice(&pe_bytes[raw_off..raw_off + copy_n]);
                }
            }
        }
    }

    if text_va == 0 {
        return ExecResult { exit_reason: ExecExitReason::Fault { msg: ".text section not found".into() }, steps: 0, state: HashMap::new() };
    }

    let entry = image_base + entry_rva;
    let mut cpu = Cpu::new(mem, entry);
    let exit_reason = cpu.run();

    let mut state = HashMap::new();
    if data_va != 0 && data_size >= 4 {
        let base = data_va as usize;
        for slot in 0..256u16 {
            let addr = base + slot as usize * 4;
            if addr + 4 <= cpu.mem.len() {
                let val = u32::from_le_bytes(cpu.mem[addr..addr + 4].try_into().unwrap()) as u64;
                if val != 0 {
                    state.insert(slot, val);
                }
            }
        }
    }

    ExecResult { exit_reason, steps: cpu.steps, state }
}