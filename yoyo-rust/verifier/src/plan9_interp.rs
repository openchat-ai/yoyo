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

/// Run a flat Plan9 (x64) binary. Bytes loaded at 0x0000, RIP=0.
/// R15 is set to 0x20000 (beyond code+stack) as the state base, matching the
/// x64 assembler's R15-relative state addressing.
pub fn run_plan9(bytes: &[u8]) -> ExecResult {
    let mut mem = vec![0u8; 0x40000];
    let n = bytes.len().min(mem.len());
    mem[..n].copy_from_slice(&bytes[..n]);

    let mut cpu = Cpu::new(mem);
    cpu.r15 = 0x20000;
    let exit_reason = cpu.run();

    ExecResult { exit_reason, steps: cpu.steps, state: cpu.state }
}
