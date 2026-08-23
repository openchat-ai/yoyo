//! Plan 9 / Acadia flat x64 interpreter — executes YOYO-emitted Plan9 flat binary
//! (defaults to x86-64 NOP/RET via PlatformBackend) for DDC against the TIR simulator.
//!
//! Flat binary, entry=0. NOP=0x90, RET=0xC3, HLT=0xF4 treated as exit.
//! Decodes x86-64 REX.W-prefixed instructions emitted by the default x64 assembler:
//!   MOVABS RAX, imm64 (48 B8), MOV [R15+disp], reg (49 89 ...), MOV reg, [R15+disp] (49 8B ...),
//!   ADD RAX, RCX (48 01 C8), CMP RAX, RCX (48 39 C8), Jcc/0F 8x, CALL/E8, JMP/E9.

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
const INITIAL_SP: u64 = 0x8000;

#[derive(Debug, Default, Clone, Copy)]
struct Flags {
    zf: bool,
    sf: bool,
    cf: bool,
    of: bool,
}

struct Cpu {
    rip: u64,
    rsp: u64,
    rax: u64,
    rcx: u64,
    r15: u64,
    flags: Flags,
    mem: Vec<u8>,
    steps: u64,
    step_limit: u64,
    call_depth: u32,
    state: HashMap<u16, u64>,
}

impl Cpu {
    fn new(mem: Vec<u8>) -> Self {
        Self {
            rip: 0, rsp: INITIAL_SP,
            rax: 0, rcx: 0, r15: 0,
            flags: Flags::default(),
            mem, steps: 0, step_limit: DEFAULT_STEP_LIMIT,
            call_depth: 0, state: HashMap::new(),
        }
    }

    fn mem_get(&self, addr: u64) -> u8 {
        let a = addr as usize;
        if a < self.mem.len() { self.mem[a] } else { 0 }
    }
    fn mem_set(&mut self, addr: u64, v: u8) {
        let a = addr as usize;
        if a >= self.mem.len() { self.mem.resize(a + 1, 0); }
        self.mem[a] = v;
    }
    fn load64_le(&self, addr: u64) -> u64 {
        let mut v = 0u64;
        for i in 0..8u64 {
            v |= (self.mem_get(addr.wrapping_add(i)) as u64) << (i * 8);
        }
        v
    }
    fn store64_le(&mut self, addr: u64, val: u64) {
        for i in 0..8u64 {
            self.mem_set(addr.wrapping_add(i), ((val >> (i * 8)) & 0xFF) as u8);
        }
    }
    fn push64(&mut self, v: u64) {
        self.rsp = self.rsp.wrapping_sub(8);
        self.store64_le(self.rsp, v);
    }
    fn pop64(&mut self) -> u64 {
        let v = self.load64_le(self.rsp);
        self.rsp = self.rsp.wrapping_add(8);
        v
    }

    fn fetch8(&mut self) -> Option<u8> {
        if (self.rip as usize) >= self.mem.len() { return None; }
        let v = self.mem_get(self.rip);
        self.rip = self.rip.wrapping_add(1);
        Some(v)
    }

    fn fetch_imm32(&mut self) -> Option<u32> {
        let mut v = 0u32;
        for i in 0..4u32 {
            v |= (self.fetch8()? as u32) << (i * 8);
        }
        Some(v)
    }

    fn reg64(&self, idx: u8) -> u64 {
        match idx {
            0 => self.rax,
            1 => self.rcx,
            _ => 0,
        }
    }

    fn set_flags_sub(&mut self, a: u64, b: u64) {
        let result = a.wrapping_sub(b);
        self.flags.zf = a == b;
        self.flags.sf = (result >> 63) != 0;
        self.flags.cf = a < b;
        self.flags.of = ((a ^ b) & (a ^ result)) >> 63 != 0;
    }

    fn jcc_taken(&self, cc: u8) -> bool {
        match cc {
            0x84 => self.flags.zf,                                           // JE/JZ
            0x85 => !self.flags.zf,                                          // JNE/JNZ
            0x8C => self.flags.sf != self.flags.of,                          // JL/JNGE
            0x8D => self.flags.sf == self.flags.of,                          // JGE/JNL
            0x8E => self.flags.zf || self.flags.sf != self.flags.of,         // JLE/JNG
            0x8F => !self.flags.zf && self.flags.sf == self.flags.of,        // JG/JNLE
            0x82 => self.flags.cf,                                           // JB/JNAE
            0x83 => !self.flags.cf,                                          // JAE/JNB
            0x86 => self.flags.cf || self.flags.zf,                          // JBE/JNA
            0x87 => !self.flags.cf && !self.flags.zf,                        // JA/JNBE
            _ => false,
        }
    }

    /// The assembler's store_state emits: REX.WB=0x49, 0x89, ModRM(reg=src, r/m=R15), disp8
    /// ModRM: mod=01(disp8), reg=src_low3, r/m=111 (R15 when B=1)
    fn decode_store_state(&mut self, _rex_w: bool, rex_b: bool, modrm: u8) -> Option<ExecExitReason> {
        let mod_field = modrm >> 6;
        let reg_field = (modrm >> 3) & 0x7;
        let rm_field = modrm & 0x7;
        if rm_field != 7 || !rex_b { return None; } // not R15 base
        let base = if rex_b { self.r15 } else { 0 };
        let disp = if mod_field == 0x01 {
            self.fetch8()? as u32 as u64
        } else if mod_field == 0x02 {
            self.fetch_imm32()? as u64
        } else {
            return None;
        };
        let base_plus_disp = base.wrapping_add(disp);
        // Determine source register from reg_field + rex.R
        let src_val = match reg_field {
            0 => self.rax, // RAX
            1 => self.rcx, // RCX
            _ => return None, // unsupported src reg
        };
        // Determine slot from disp
        let slot = (disp / 8) as u16;
        // Store to memory and track state
        self.store64_le(base_plus_disp, src_val);
        self.state.insert(slot, src_val);
        None
    }

    /// The assembler's load_state emits: REX.WB=0x49, 0x8B, ModRM(reg=dst, r/m=R15), disp8
    fn decode_load_state(&mut self, _rex_w: bool, rex_b: bool, modrm: u8) -> Option<ExecExitReason> {
        let mod_field = modrm >> 6;
        let reg_field = (modrm >> 3) & 0x7;
        let rm_field = modrm & 0x7;
        if rm_field != 7 || !rex_b { return None; }
        let base = if rex_b { self.r15 } else { 0 };
        let disp = if mod_field == 0x01 {
            self.fetch8()? as u32 as u64
        } else if mod_field == 0x02 {
            self.fetch_imm32()? as u64
        } else {
            return None;
        };
        let base_plus_disp = base.wrapping_add(disp);
        let val = self.load64_le(base_plus_disp);
        // Load into destination register
        match reg_field {
            0 => self.rax = val,
            1 => self.rcx = val,
            _ => return None,
        }
        None
    }

    fn step(&mut self) -> Option<ExecExitReason> {
        if self.steps >= self.step_limit {
            return Some(ExecExitReason::StepLimit { steps: self.steps });
        }
        if (self.rip as usize) >= self.mem.len() {
            return Some(ExecExitReason::Halted);
        }
        self.steps += 1;
        let Some(op) = self.fetch8() else {
            return Some(ExecExitReason::Halted);
        };

        // REX.W prefix (0x48 = REX.W, 0x49 = REX.WB)
        if op == 0x48 || op == 0x49 {
            let rex = op;
            let rex_w = (rex >> 3) & 1 == 1;
            let rex_r = (rex >> 2) & 1 == 1;
            let rex_b = rex & 1 == 1;
            let Some(op2) = self.fetch8() else {
                return Some(ExecExitReason::Fault { msg: "truncated after REX".into() });
            };
            match op2 {
                0xB8..=0xBF => {
                    // MOVABS r64, imm64 (opcode = 0xB8 + register)
                    let reg = (op2 - 0xB8) | (if rex_b { 8 } else { 0 });
                    let mut imm = 0u64;
                    for i in 0..8u64 {
                        let Some(b) = self.fetch8() else {
                            return Some(ExecExitReason::Fault { msg: "truncated MOVABS".into() });
                        };
                        imm |= (b as u64) << (i * 8);
                    }
                    match reg {
                        0 => self.rax = imm,
                        1 => self.rcx = imm,
                        _ => return Some(ExecExitReason::Fault { msg: format!("MOVABS to unsupported reg {}", reg) }),
                    }
                    None
                }
                0x89 => {
                    // MOV r/m64, r64 — store_state
                    let Some(modrm) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated MOV store".into() });
                    };
                    self.decode_store_state(rex_w, rex_b, modrm)
                }
                0x8B => {
                    // MOV r64, r/m64 — load_state
                    let Some(modrm) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated MOV load".into() });
                    };
                    self.decode_load_state(rex_w, rex_b, modrm)
                }
                0x8D => {
                    // LEA r64, [r15+disp] — fixture init: lea rax,[r15+8]
                    let Some(modrm) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated LEA".into() });
                    };
                    let mod_field = modrm >> 6;
                    let reg_field = (modrm >> 3) & 0x7;
                    let rm_field = modrm & 0x7;
                    if reg_field != 0 || rm_field != 7 || !rex_b {
                        return Some(ExecExitReason::Fault {
                            msg: format!("undecoded LEA modrm 0x{:02x}", modrm),
                        });
                    }
                    let disp = if mod_field == 0x01 {
                        self.fetch8()? as u32 as u64
                    } else if mod_field == 0x02 {
                        self.fetch_imm32()? as u64
                    } else {
                        return Some(ExecExitReason::Fault {
                            msg: format!("undecoded LEA modrm 0x{:02x}", modrm),
                        });
                    };
                    let addr = self.r15.wrapping_add(disp);
                    self.rax = addr;
                    None
                }
                0x01 => {
                    // ADD r/m64, r64 (RAX = RAX + RCX)
                    let Some(modrm) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated ADD".into() });
                    };
                    let mod_field = modrm >> 6;
                    let reg_field = (modrm >> 3) & 0x7;
                    let rm_field = modrm & 0x7;
                    if mod_field == 0x03 {
                        // register form
                        let src = reg_field | (if rex_r { 8 } else { 0 });
                        let dst = rm_field | (if rex_b { 8 } else { 0 });
                        let src_val = match src { 0 => self.rax, 1 => self.rcx, _ => 0 };
                        let dst_val = match dst { 0 => self.rax, 1 => self.rcx, _ => 0 };
                        let result = dst_val.wrapping_add(src_val);
                        match dst { 0 => self.rax = result, 1 => self.rcx = result, _ => {} }
                    }
                    None
                }
                0x39 => {
                    // CMP r/m64, r64 — sets flags from dst - src (e.g. 48 39 C8 = cmp rax, rcx)
                    let Some(modrm) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated CMP".into() });
                    };
                    let mod_field = modrm >> 6;
                    let reg_field = (modrm >> 3) & 0x7;
                    let rm_field = modrm & 0x7;
                    if mod_field != 0x03 {
                        return Some(ExecExitReason::Fault {
                            msg: format!("undecoded CMP modrm 0x{:02x}", modrm),
                        });
                    }
                    let src = reg_field | (if rex_r { 8 } else { 0 });
                    let dst = rm_field | (if rex_b { 8 } else { 0 });
                    self.set_flags_sub(self.reg64(dst), self.reg64(src));
                    None
                }
                0x83 => {
                    // ADD/SUB/... r64, imm8 — LDB offset uses 48 83 C0 imm (ADD rax, imm8)
                    let Some(modrm) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated 83".into() });
                    };
                    let mod_field = modrm >> 6;
                    let reg_field = (modrm >> 3) & 0x7;
                    let rm_field = modrm & 0x7;
                    if mod_field != 0x03 || reg_field != 0 || rm_field != 0 {
                        return Some(ExecExitReason::Fault {
                            msg: format!("undecoded 83 modrm 0x{:02x}", modrm),
                        });
                    }
                    let Some(imm) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated 83 imm8".into() });
                    };
                    self.rax = self.rax.wrapping_add(imm as i8 as i64 as u64);
                    None
                }
                0x81 => {
                    // ADD r64, imm32 — LDB large offset uses 48 81 C0 imm32
                    let Some(modrm) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated 81".into() });
                    };
                    let mod_field = modrm >> 6;
                    let reg_field = (modrm >> 3) & 0x7;
                    let rm_field = modrm & 0x7;
                    if mod_field != 0x03 || reg_field != 0 || rm_field != 0 {
                        return Some(ExecExitReason::Fault {
                            msg: format!("undecoded 81 modrm 0x{:02x}", modrm),
                        });
                    }
                    let Some(imm) = self.fetch_imm32() else {
                        return Some(ExecExitReason::Fault { msg: "truncated 81 imm32".into() });
                    };
                    self.rax = self.rax.wrapping_add(imm as i32 as i64 as u64);
                    None
                }
                0x0F => {
                    // Two-byte after REX — movzx for LDB (48 0F B6 00 = movzx rax, byte [rax])
                    let Some(op3) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated after REX 0F".into() });
                    };
                    if op3 == 0xB6 {
                        let Some(modrm) = self.fetch8() else {
                            return Some(ExecExitReason::Fault { msg: "truncated movzx".into() });
                        };
                        let mod_field = modrm >> 6;
                        let reg_field = (modrm >> 3) & 0x7;
                        let rm_field = modrm & 0x7;
                        if reg_field != 0 {
                            return Some(ExecExitReason::Fault {
                                msg: format!("movzx to non-rax reg modrm 0x{:02x}", modrm),
                            });
                        }
                        if mod_field == 0 && rm_field == 0 {
                            self.rax = self.mem_get(self.rax) as u64;
                        } else {
                            return Some(ExecExitReason::Fault {
                                msg: format!("undecoded movzx modrm 0x{:02x}", modrm),
                            });
                        }
                        None
                    } else {
                        Some(ExecExitReason::Fault {
                            msg: format!("undecoded REX 0F {:02x} at 0x{:04x}", op3, self.rip.wrapping_sub(3)),
                        })
                    }
                }
                _ => Some(ExecExitReason::Fault {
                    msg: format!("undecoded REX instruction at 0x{:04x}: {:02x} {:02x}", self.rip.wrapping_sub(2), rex, op2),
                }),
            }
        } else {
            match op {
                0x90 => None, // NOP
                0xC3 => { // RET
                    if self.call_depth == 0 || self.rsp == INITIAL_SP {
                        return Some(ExecExitReason::Ret);
                    }
                    self.call_depth -= 1;
                    self.rip = self.pop64();
                    None
                }
                0xF4 => Some(ExecExitReason::Ret), // HLT → exit
                0x0F => {
                    // Two-byte opcodes: Jcc rel32 (0F 8x)
                    let Some(cc) = self.fetch8() else {
                        return Some(ExecExitReason::Fault { msg: "truncated after 0F".into() });
                    };
                    if (0x80..=0x8F).contains(&cc) {
                        let Some(disp) = self.fetch_imm32() else {
                            return Some(ExecExitReason::Fault { msg: "truncated Jcc".into() });
                        };
                        let next = self.rip;
                        if self.jcc_taken(cc) {
                            self.rip = next.wrapping_add(disp as i32 as i64 as u64);
                        }
                        None
                    } else {
                        Some(ExecExitReason::Fault {
                            msg: format!("undecoded 0F {:02x} at 0x{:04x}", cc, self.rip.wrapping_sub(2)),
                        })
                    }
                }
                0xE8 => { // CALL rel32
                    let Some(disp) = self.fetch_imm32() else {
                        return Some(ExecExitReason::Fault { msg: "truncated CALL".into() });
                    };
                    let next = self.rip;
                    self.push64(next);
                    self.call_depth += 1;
                    self.rip = next.wrapping_add(disp as i32 as i64 as u64);
                    None
                }
                0xE9 => { // JMP rel32
                    let Some(disp) = self.fetch_imm32() else {
                        return Some(ExecExitReason::Fault { msg: "truncated JMP".into() });
                    };
                    let next = self.rip;
                    self.rip = next.wrapping_add(disp as i32 as i64 as u64);
                    None
                }
                _ => Some(ExecExitReason::Fault {
                    msg: format!("undecoded Plan9/x64 insn at 0x{:04x}: {:02x}", self.rip.wrapping_sub(1), op),
                }),
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

/// Run x64 code at `entry` with state base `r15` in a pre-built memory image.
pub fn run_x64_at(mut mem: Vec<u8>, entry: u64, r15: u64) -> ExecResult {
    let need = (entry as usize).saturating_add(0x10000).max(r15 as usize + 0x40000);
    if mem.len() < need {
        mem.resize(need, 0);
    }
    let mut cpu = Cpu::new(mem);
    cpu.rip = entry;
    cpu.r15 = r15;
    let exit_reason = cpu.run();
    ExecResult { exit_reason, steps: cpu.steps, state: cpu.state }
}

/// Startup stub size in pe_link / elf_link images (lea r15 + jmp + nop).
const LINKER_STARTUP_LEN: u64 = 13;

fn load_pe_for_interp(pe: &[u8]) -> Result<(Vec<u8>, u64, u64), String> {
    if pe.len() < 0x40 || &pe[0..2] != b"MZ" {
        return Err("invalid PE: missing MZ".into());
    }
    let lfanew = u32::from_le_bytes(pe[0x3C..0x40].try_into().map_err(|_| "bad e_lfanew")?) as usize;
    if lfanew + 0xF8 > pe.len() || &pe[lfanew..lfanew + 4] != b"PE\0\0" {
        return Err("invalid PE signature".into());
    }
    let coff = lfanew + 4;
    let num_sections = u16::from_le_bytes(pe[coff + 2..coff + 4].try_into().unwrap()) as usize;
    let size_opt = u16::from_le_bytes(pe[coff + 16..coff + 18].try_into().unwrap()) as usize;
    let opt = coff + 20;
    let entry_rva = u32::from_le_bytes(pe[opt + 16..opt + 20].try_into().unwrap()) as u64;
    let size_of_image = u32::from_le_bytes(pe[opt + 56..opt + 60].try_into().unwrap()) as usize;
    let section_table = opt + size_opt;

    // Flat RVA map (like ELF loader): skip ImageBase to avoid multi-GB allocations.
    let mut mem = vec![0u8; size_of_image.max(0x500000)];
    let mut data_rva = 0u64;
    for i in 0..num_sections {
        let sec = section_table + i * 40;
        if sec + 40 > pe.len() {
            break;
        }
        let name = &pe[sec..sec + 8];
        let virt_size = u32::from_le_bytes(pe[sec + 8..sec + 12].try_into().unwrap()) as usize;
        let virt_addr = u32::from_le_bytes(pe[sec + 12..sec + 16].try_into().unwrap()) as u64;
        let raw_size = u32::from_le_bytes(pe[sec + 16..sec + 20].try_into().unwrap()) as usize;
        let raw_ptr = u32::from_le_bytes(pe[sec + 20..sec + 24].try_into().unwrap()) as usize;
        let copy_n = raw_size.min(virt_size).min(pe.len().saturating_sub(raw_ptr));
        if copy_n > 0 && raw_ptr < pe.len() {
            let dst_off = virt_addr as usize;
            let end = dst_off + copy_n;
            if end > mem.len() {
                mem.resize(end, 0);
            }
            mem[dst_off..end].copy_from_slice(&pe[raw_ptr..raw_ptr + copy_n]);
        }
        if name.starts_with(b".data") {
            data_rva = virt_addr;
        }
    }
    if data_rva == 0 {
        return Err("PE: no .data section".into());
    }
    // Startup stub sets R15=data; we skip it and set R15 directly (same as ELF path).
    let entry = entry_rva + LINKER_STARTUP_LEN;
    let r15 = data_rva;
    Ok((mem, entry, r15))
}

fn load_elf_for_interp(elf: &[u8]) -> Result<(Vec<u8>, u64, u64), String> {
    if elf.len() < 64 || &elf[0..4] != b"\x7fELF" || elf[4] != 2 {
        return Err("invalid ELF64".into());
    }
    let e_phoff = u64::from_le_bytes(elf[32..40].try_into().map_err(|_| "bad e_phoff")?);
    let e_phnum = u16::from_le_bytes(elf[56..58].try_into().unwrap()) as usize;
    let mut mem = vec![0u8; 0x500000];
    let mut data_va = 0u64;
    let mut text_va = 0u64;
    for i in 0..e_phnum {
        let ph = e_phoff as usize + i * 56;
        if ph + 56 > elf.len() {
            break;
        }
        let p_type = u32::from_le_bytes(elf[ph..ph + 4].try_into().unwrap());
        if p_type != 1 {
            continue; // PT_LOAD
        }
        let p_offset = u64::from_le_bytes(elf[ph + 8..ph + 16].try_into().unwrap()) as usize;
        let p_vaddr = u64::from_le_bytes(elf[ph + 16..ph + 24].try_into().unwrap());
        let p_filesz = u64::from_le_bytes(elf[ph + 32..ph + 40].try_into().unwrap()) as usize;
        let copy_n = p_filesz.min(elf.len().saturating_sub(p_offset));
        if copy_n > 0 {
            let dst = p_vaddr as usize;
            let end = dst + copy_n;
            if end > mem.len() {
                mem.resize(end, 0);
            }
            mem[dst..end].copy_from_slice(&elf[p_offset..p_offset + copy_n]);
        }
        if text_va == 0 || p_vaddr < text_va {
            text_va = p_vaddr;
        }
        if p_vaddr > text_va {
            data_va = p_vaddr;
        }
    }
    if data_va == 0 {
        return Err("ELF: no data PT_LOAD".into());
    }
    let e_entry = u64::from_le_bytes(elf[24..32].try_into().unwrap());
    let entry = e_entry + LINKER_STARTUP_LEN;
    Ok((mem, entry, data_va))
}

/// Run a PE32+ x64 image produced by pe_link (Win32 production path).
pub fn run_x64_pe(pe_bytes: &[u8]) -> ExecResult {
    match load_pe_for_interp(pe_bytes) {
        Ok((mem, entry, r15)) => run_x64_at(mem, entry, r15),
        Err(msg) => ExecResult {
            exit_reason: ExecExitReason::Fault { msg },
            steps: 0,
            state: HashMap::new(),
        },
    }
}

/// Run an ELF64 x64 image produced by elf_link (Linux production path).
pub fn run_x64_elf(elf_bytes: &[u8]) -> ExecResult {
    match load_elf_for_interp(elf_bytes) {
        Ok((mem, entry, r15)) => run_x64_at(mem, entry, r15),
        Err(msg) => ExecResult {
            exit_reason: ExecExitReason::Fault { msg },
            steps: 0,
            state: HashMap::new(),
        },
    }
}

/// Run a flat Plan9 (x64) binary. Bytes loaded at 0x0000, RIP=0.
/// R15 is set to 0x20000 (beyond code+stack) as the state base, matching the
/// x64 assembler's R15-relative state addressing.
pub fn run_plan9(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x40000];
    let n = bytes.len().min(mem.len());
    mem[..n].copy_from_slice(&bytes[..n]);
    run_x64_at(mem, 0, 0x20000)
}
