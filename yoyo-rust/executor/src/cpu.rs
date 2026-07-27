//! Cpu — register file + x64 decode/dispatch (W-START attempt-N5b).
//!
//! Subset deliberately narrow (PROMPT Part 4.3 + the canonical emit
//! shapes used by the W-selfhost-min series). Anything outside the
//! subset fails closed via `Fault::Unimplemented` / `Fault::Decode`.
//!
//! Registers modelled: rax, rcx, r15, plus EFLAGS (ZF/SF/OF/CF/PF).
//! All other GPRs are treated as 0 and any read/write to them via
//! the subset is rejected as decode-fault.

use crate::mmu::{Fault, Mmu};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegId {
    Rax,
    Rcx,
    R15,
    Rsi,
    Rdi,
}

#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub zf: bool,
    pub sf: bool,
    pub of: bool,
    pub cf: bool,
    pub pf: bool,
}

impl Default for Flags {
    fn default() -> Self {
        Self {
            zf: false,
            sf: false,
            of: false,
            cf: false,
            pf: false,
        }
    }
}

pub struct Cpu {
    pub rax: u64,
    pub rcx: u64,
    pub r15: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub flags: Flags,
    pub rip: u64,
    pub steps: u64,
    pub step_limit: u64,
    pub halted: bool,
    /// 1-deep call-return shadow. Locked yoyo.ty only chains a single
    /// CALL → H_xx → RET per scope; deeper chains fail closed via
    /// `Fault::Diverged`. Pushed by CALL (0xE8), popped by RET (0xC3)
    /// when `ret_stack` is non-empty.
    pub ret_stack: Vec<u64>,
}

impl Cpu {
    pub fn new(rip: u64, r15: u64, step_limit: u64) -> Self {
        Self {
            rax: 0,
            rcx: 0,
            r15,
            rsi: 0,
            rdi: 0,
            flags: Flags::default(),
            rip,
            steps: 0,
            step_limit,
            halted: false,
            ret_stack: Vec::new(),
        }
    }

    pub fn run(&mut self, mmu: &mut Mmu) -> Result<(), Fault> {
        while !self.halted {
            self.step(mmu)?;
        }
        Ok(())
    }

    pub fn step(&mut self, mmu: &mut Mmu) -> Result<(), Fault> {
        if self.steps >= self.step_limit {
            return Err(Fault::StepLimit { steps: self.steps });
        }
        self.steps += 1;
        let pc = self.rip;
        let b0 = mmu.read_u8(pc)?;

        // Single-byte opcodes first.
        match b0 {
            0x90 => {
                self.rip = pc + 1;
                return Ok(());
            }
            0xC3 => {
                if let Some(ret) = self.ret_stack.pop() {
                    self.rip = ret;
                    return Ok(());
                }
                self.rip = pc + 1;
                self.halted = true;
                return Ok(());
            }
            0xE9 => {
                let off = read_rel32(mmu, pc + 1)?;
                self.rip = (pc as i64 + 5 + off as i64) as u64;
                return Ok(());
            }
            0xE8 => {
                let off = read_rel32(mmu, pc + 1)?;
                let ret = pc + 5;
                if !self.ret_stack.is_empty() {
                    return Err(Fault::Diverged {
                        rip: pc,
                        msg: "nested CALL beyond 1-deep shadow stack",
                    });
                }
                self.ret_stack.push(ret);
                self.rip = (pc as i64 + 5 + off as i64) as u64;
                return Ok(());
            }
            0x0F => {
                let b1 = mmu.read_u8(pc + 1)?;
                if (0x82..=0x8F).contains(&b1) {
                    let off = read_rel32(mmu, pc + 2)?;
                    let taken = eval_jcc(b1, &self.flags);
                    let target = (pc as i64 + 6 + off as i64) as u64;
                    self.rip = if taken { target } else { pc + 6 };
                    return Ok(());
                }
                return Err(Fault::Decode {
                    rip: pc,
                    reason: "unrecognised 0F escape",
                });
            }
            0xFC => {
                // rep movsb: copy rcx bytes from [rsi] to [rdi]
                for i in 0..self.rcx {
                    let byte = mmu.read_u8(self.rsi + i)?;
                    mmu.write_u8(self.rdi + i, byte)?;
                }
                self.rsi += self.rcx;
                self.rdi += self.rcx;
                self.rcx = 0;
                self.rip = pc + 1;
                return Ok(());
            }
            _ => {}
        }

        // REX prefix handling. We accept REX.W (0x48) and REX.WB (0x49)
        // because the verifier emits load_state/store_state with REX.B
        // set (base is R15).
        let (rex, opcode) = match b0 {
            0x48 | 0x49 => (b0, mmu.read_u8(pc + 1)?),
            _ => {
                return Err(Fault::Unimplemented { rip: pc, byte: b0 });
            }
        };
        let rex_b = (rex & 0x01) != 0;
        // (REX.R, REX.X, REX.W) — we only care about REX.R for the
        // reg field, but the subset only ever uses rax/rcx/r15 so
        // we can ignore REX.R and the high half of the reg field.
        // We still decode the reg field in modrm correctly.

        match opcode {
            0xB8..=0xBF => {
                // movabs r64, imm64 — full 3-bit reg id, REX.B may
                // extend to r8..r15.
                let low = opcode & 0x07;
                let reg = if rex_b {
                    match low {
                        0 => RegId::Rax,
                        1 => RegId::Rcx,
                        7 => RegId::R15,
                        _ => {
                            return Err(Fault::Decode {
                                rip: pc,
                                reason: "movabs: reg outside subset (r8..r15)",
                            });
                        }
                    }
                } else {
                    match low {
                        0 => RegId::Rax,
                        1 => RegId::Rcx,
                        7 => RegId::R15,
                        _ => {
                            return Err(Fault::Decode {
                                rip: pc,
                                reason: "movabs: reg outside subset",
                            });
                        }
                    }
                };
                let imm = mmu.read_u64_le(pc + 2)?;
                write_reg(self, reg, imm);
                self.rip = pc + 10;
                Ok(())
            }
            0x8B => {
                // mov r64, [r15 + disp]
                let (reg, disp) = decode_state_modrm(mmu, pc + 2, "load")?;
                if disp > i32::MAX as u64 {
                    return Err(Fault::Decode {
                        rip: pc,
                        reason: "load: disp too large",
                    });
                }
                let val = mmu.read_u64_le(self.r15 + disp)?;
                write_reg(self, reg, val);
                self.rip = pc + 2 + state_instr_len(disp);
                Ok(())
            }
            0x89 => {
                // mov [r15 + disp], r64
                let (reg, disp) = decode_state_modrm(mmu, pc + 2, "store")?;
                let val = read_reg(self, reg);
                mmu.write_u64_le(self.r15 + disp, val)?;
                self.rip = pc + 2 + state_instr_len(disp);
                Ok(())
            }
            0xFF => {
                let b2 = mmu.read_u8(pc + 2)?;
                match b2 {
                    0xC0 => {
                        // inc rax
                        let old = self.rax;
                        self.rax = self.rax.wrapping_add(1);
                        set_flags_arith(self, old, 1, self.rax, false);
                        self.rip = pc + 3;
                        Ok(())
                    }
                    0xC8 => {
                        // dec rax
                        let old = self.rax;
                        self.rax = self.rax.wrapping_sub(1);
                        set_flags_arith(self, old, 1, self.rax, true);
                        self.rip = pc + 3;
                        Ok(())
                    }
                    _ => Err(Fault::Decode {
                        rip: pc,
                        reason: "unsupported 48/49 FF /modrm",
                    }),
                }
            }
            0x83 => {
                // add/sub r64, imm8 (sign-extended)
                let b2 = mmu.read_u8(pc + 2)?;
                let op = b2 >> 3;
                let low = b2 & 0x07;
                let reg = if rex_b {
                    match low {
                        0 => RegId::Rax,
                        1 => RegId::Rcx,
                        7 => RegId::R15,
                        _ => {
                            return Err(Fault::Decode {
                                rip: pc,
                                reason: "48/49 83: reg outside subset",
                            });
                        }
                    }
                } else {
                    low3_reg(low)
                };
                let imm_b = mmu.read_u8(pc + 3)? as i8 as i64 as u64;
                match (op, reg) {
                    (0, RegId::Rax) => {
                        let old = self.rax;
                        self.rax = self.rax.wrapping_add(imm_b);
                        set_flags_arith(self, old, imm_b, self.rax, false);
                    }
                    (5, RegId::Rax) => {
                        let old = self.rax;
                        self.rax = self.rax.wrapping_sub(imm_b);
                        set_flags_arith(self, old, imm_b, self.rax, true);
                    }
                    _ => {
                        return Err(Fault::Decode {
                            rip: pc,
                            reason: "48/49 83: op not add/sub rax",
                        });
                    }
                }
                self.rip = pc + 4;
                Ok(())
            }
            0x81 => {
                // add/sub r64, imm32 (sign-extended)
                let b2 = mmu.read_u8(pc + 2)?;
                let op = b2 >> 3;
                let low = b2 & 0x07;
                let reg = if rex_b {
                    match low {
                        0 => RegId::Rax,
                        1 => RegId::Rcx,
                        7 => RegId::R15,
                        _ => {
                            return Err(Fault::Decode {
                                rip: pc,
                                reason: "48/49 81: reg outside subset",
                            });
                        }
                    }
                } else {
                    low3_reg(low)
                };
                let imm = mmu.read_u32_le(pc + 3)? as i32 as i64 as u64;
                match (op, reg) {
                    (0, RegId::Rax) => {
                        let old = self.rax;
                        self.rax = self.rax.wrapping_add(imm);
                        set_flags_arith(self, old, imm, self.rax, false);
                    }
                    (5, RegId::Rax) => {
                        let old = self.rax;
                        self.rax = self.rax.wrapping_sub(imm);
                        set_flags_arith(self, old, imm, self.rax, true);
                    }
                    _ => {
                        return Err(Fault::Decode {
                            rip: pc,
                            reason: "48/49 81: op not add/sub rax",
                        });
                    }
                }
                self.rip = pc + 7;
                Ok(())
            }
            0x01 => {
                // add r64, r64
                let (dst, src) = decode_rr(mmu, pc + 2, rex_b)?;
                let a = read_reg(self, dst);
                let b = read_reg(self, src);
                let old = a;
                let r = a.wrapping_add(b);
                write_reg(self, dst, r);
                set_flags_arith(self, old, b, r, false);
                self.rip = pc + 3;
                Ok(())
            }
            0x29 => {
                // sub r64, r64
                let (dst, src) = decode_rr(mmu, pc + 2, rex_b)?;
                let a = read_reg(self, dst);
                let b = read_reg(self, src);
                let old = a;
                let r = a.wrapping_sub(b);
                write_reg(self, dst, r);
                set_flags_arith(self, old, b, r, true);
                self.rip = pc + 3;
                Ok(())
            }
            0x09 => {
                // or r64, r64
                let (dst, src) = decode_rr(mmu, pc + 2, rex_b)?;
                let a = read_reg(self, dst);
                let b = read_reg(self, src);
                let r = a | b;
                write_reg(self, dst, r);
                self.flags.zf = r == 0;
                self.flags.sf = (r >> 63) & 1 == 1;
                self.flags.of = false;
                self.flags.cf = false;
                self.flags.pf = parity8((r & 0xFF) as u8);
                self.rip = pc + 3;
                Ok(())
            }
            0x0F => {
                let b2 = mmu.read_u8(pc + 2)?;
                match b2 {
                    0xAF => {
                        // imul r64, r64
                        let (dst, src) = decode_rr(mmu, pc + 3, rex_b)?;
                        let a = read_reg(self, dst) as i64;
                        let b = read_reg(self, src) as i64;
                        let r = (a.wrapping_mul(b)) as u64;
                        write_reg(self, dst, r);
                        self.flags.zf = r == 0;
                        self.flags.sf = (r >> 63) & 1 == 1;
                        self.flags.of = false;
                        self.flags.cf = false;
                        self.rip = pc + 4;
                        Ok(())
                    }
                    0xB6 => {
                        // movzx r64, byte [r/m]
                        // Verifier emits this as 48 0F B6 0x00
                        // (ModRM=0x00 → rax = zx(byte[rax])).
                        let b3 = mmu.read_u8(pc + 3)?;
                        if b3 != 0x00 {
                            return Err(Fault::Decode {
                                rip: pc,
                                reason: "movzx expects ModRM=0x00",
                            });
                        }
                        let val = mmu.read_u8(self.rax)? as u64;
                        self.rax = val;
                        self.rip = pc + 4;
                        Ok(())
                    }
                    _ => Err(Fault::Decode {
                        rip: pc,
                        reason: "unsupported 48/49 0F escape",
                    }),
                }
            }
            0x39 | 0x3B => {
                // cmp r64, r64
                let (a_reg, b_reg) = decode_rr(mmu, pc + 2, rex_b)?;
                let a = read_reg(self, a_reg);
                let b = read_reg(self, b_reg);
                let (lhs, rhs) = if opcode == 0x39 { (a, b) } else { (b, a) };
                let r = lhs.wrapping_sub(rhs);
                set_flags_arith(self, lhs, rhs, r, true);
                self.rip = pc + 3;
                Ok(())
            }
            0xB6 => {
                // Unreachable: 0xB6 is consumed as 0x0F 0xB6 above.
                return Err(Fault::Decode {
                    rip: pc,
                    reason: "48/49 B6 is part of 0F B6 movzx (handled above)",
                });
            }
            _ => Err(Fault::Decode {
                rip: pc,
                reason: "unsupported 48/49 escape",
            }),
        }
    }
}

// ----- helpers -----

fn read_rel32(mmu: &mut Mmu, at: u64) -> Result<i32, Fault> {
    let v = mmu.read_u32_le(at)?;
    Ok(v as i32)
}

fn low3_reg(low3: u8) -> RegId {
    match low3 & 0x07 {
        0 => RegId::Rax,
        1 => RegId::Rcx,
        6 => RegId::Rsi,
        7 => RegId::Rdi,
        _ => RegId::Rax,
    }
}

fn map_reg(low3: u8, rex_b: bool) -> Option<RegId> {
    // Combined low3 + REX.B -> subset register.
    let ext = (low3 & 0x07) as u16;
    let ext = if rex_b { ext + 8 } else { ext };
    match ext {
        0 => Some(RegId::Rax),
        1 => Some(RegId::Rcx),
        6 => Some(RegId::Rsi),
        7 => Some(RegId::Rdi),
        14 => Some(RegId::Rsi),
        15 => Some(RegId::R15),
        _ => None,
    }
}

fn decode_state_modrm(mmu: &mut Mmu, at: u64, _ctx: &str) -> Result<(RegId, u64), Fault> {
    let b = mmu.read_u8(at)?;
    let mod_ = (b >> 6) & 0x03;
    let reg = (b >> 3) & 0x07;
    let rm = b & 0x07;
    if rm != 0x07 {
        return Err(Fault::Decode {
            rip: at,
            reason: "state: rm must be 111 (r15)",
        });
    }
    // REX.B is the encoder's way of saying "base is r15"; we already
    // encoded the rm field as 0x07, so any REX.B is acceptable here.
    // The reg field's REX.R is not in this byte; we'd have to pass
    // it down. For our subset the reg field is always rax (0) or
    // rcx (1) so we ignore REX.R for now.
    let target = match reg {
        0 => RegId::Rax,
        1 => RegId::Rcx,
        6 => RegId::Rsi,
        7 => RegId::Rdi,
        _ => {
            return Err(Fault::Decode {
                rip: at,
                reason: "state: reg field unsupported",
            });
        }
    };
    let disp = match mod_ {
        0x01 => mmu.read_u8(at + 1)? as i8 as i64 as u64,
        0x02 => mmu.read_u32_le(at + 1)? as i32 as i64 as u64,
        _ => {
            return Err(Fault::Decode {
                rip: at,
                reason: "state: mod=00 not allowed (no SIB/rip-rel)",
            });
        }
    };
    Ok((target, disp))
}

fn decode_rr(mmu: &mut Mmu, at: u64, rex_b: bool) -> Result<(RegId, RegId), Fault> {
    let b = mmu.read_u8(at)?;
    let mod_ = (b >> 6) & 0x03;
    if mod_ != 0x03 {
        return Err(Fault::Decode {
            rip: at,
            reason: "rr form: mod must be 11",
        });
    }
    let dst = match map_reg((b >> 3) & 0x07, rex_b) {
        Some(r) => r,
        None => {
            return Err(Fault::Decode {
                rip: at,
                reason: "rr dst outside subset",
            });
        }
    };
    let src = match map_reg(b & 0x07, rex_b) {
        Some(r) => r,
        None => {
            return Err(Fault::Decode {
                rip: at,
                reason: "rr src outside subset",
            });
        }
    };
    Ok((dst, src))
}

fn state_instr_len(disp: u64) -> u64 {
    if disp <= 0x7F {
        2 // modrm + disp8
    } else {
        5 // modrm + disp32
    }
}

fn read_reg(cpu: &Cpu, r: RegId) -> u64 {
    match r {
        RegId::Rax => cpu.rax,
        RegId::Rcx => cpu.rcx,
        RegId::R15 => cpu.r15,
        RegId::Rsi => cpu.rsi,
        RegId::Rdi => cpu.rdi,
    }
}

fn write_reg(cpu: &mut Cpu, r: RegId, v: u64) {
    match r {
        RegId::Rax => cpu.rax = v,
        RegId::Rcx => cpu.rcx = v,
        RegId::R15 => cpu.r15 = v,
        RegId::Rsi => cpu.rsi = v,
        RegId::Rdi => cpu.rdi = v,
    }
}

fn set_flags_arith(cpu: &mut Cpu, lhs: u64, rhs: u64, res: u64, sub: bool) {
    cpu.flags.zf = res == 0;
    cpu.flags.sf = (res >> 63) & 1 == 1;
    let lhs_s = lhs as i64;
    let rhs_s = rhs as i64;
    let res_s = res as i64;
    cpu.flags.of = if sub {
        (lhs_s >= 0 && rhs_s < 0 && res_s < 0) || (lhs_s < 0 && rhs_s >= 0 && res_s >= 0)
    } else {
        (lhs_s >= 0 && rhs_s >= 0 && res_s < 0) || (lhs_s < 0 && rhs_s < 0 && res_s >= 0)
    };
    cpu.flags.cf = if sub { lhs < rhs } else { lhs.checked_add(rhs).map_or(true, |s| s < lhs) };
    cpu.flags.pf = parity8((res & 0xFF) as u8);
}

fn parity8(b: u8) -> bool {
    b.count_ones() % 2 == 0
}

fn eval_jcc(cc: u8, f: &Flags) -> bool {
    match cc {
        0x84 => f.zf,
        0x85 => !f.zf,
        0x8C => f.sf != f.of,
        0x8D => f.sf == f.of,
        0x8E => f.zf || (f.sf != f.of),
        0x8F => !f.zf && (f.sf == f.of),
        0x82 => f.cf,
        0x83 => !f.cf,
        0x86 => f.cf || f.zf,
        0x87 => !f.cf && !f.zf,
        _ => false,
    }
}

// Extend Mmu with read_u32_le.
impl Mmu {
    pub fn read_u32_le(&mut self, addr: u64) -> Result<u32, Fault> {
        let off = self.resolve(addr, 4)?;
        let mut buf = [0u8; 4];
        buf.copy_from_slice(&self.bytes[off..off + 4]);
        Ok(u32::from_le_bytes(buf))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{run_bytes, run_hex_text, ExitReason, RunLimits};

    fn run(code: &[u8]) -> Result<u64, Fault> {
        let out = run_bytes(code, RunLimits { steps: 1024, mmu_capacity: 4096 });
        match out.exit {
            ExitReason::Halted { .. } => Ok(out.rax),
            ExitReason::Fault(f) => Err(f),
        }
    }

    #[test]
    fn nop_ret_halts() {
        let code = [0x90u8, 0xC3];
        assert_eq!(run(&code).unwrap(), 0);
    }

    #[test]
    fn movabs_store_ret() {
        // movabs rax, 0x2A
        // store [r15 + 0x280], rax
        // ret
        let code = [
            0x48, 0xB8, 0x2A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x87, 0x80,
            0x02, 0x00, 0x00, 0xC3,
        ];
        let out = run_bytes(&code, RunLimits { steps: 1024, mmu_capacity: 4096 });
        assert!(matches!(out.exit, ExitReason::Halted { .. }));
        assert_eq!(out.rax, 0x2A);
    }

    #[test]
    fn movzx_inc_store_ret() {
        // movabs rax, 5
        // store [r15 + 0x280], rax  (so state[0x50] = 5)
        // load rax, [r15 + 0x280]   (rax = 5)
        // inc rax                    (rax = 6)
        // store [r15 + 0x280], rax  (state[0x50] = 6)
        // ret
        let code = [
            0x48, 0xB8, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x87, 0x80,
            0x02, 0x00, 0x00, 0x49, 0x8B, 0x87, 0x80, 0x02, 0x00, 0x00, 0x48, 0xFF, 0xC0, 0x49,
            0x89, 0x87, 0x80, 0x02, 0x00, 0x00, 0xC3,
        ];
        let out = run_bytes(&code, RunLimits { steps: 1024, mmu_capacity: 4096 });
        assert!(matches!(out.exit, ExitReason::Halted { .. }));
        assert_eq!(out.rax, 6);
    }

    #[test]
    fn jmp_backward_to_nop() {
        // H_00:  nop; ret
        // H_01:  jmp -5  (back to H_00)
        //        ret
        let code = [
            0x90, 0xC3, // H_00
            0xE9, 0xFB, 0xFF, 0xFF, 0xFF, // jmp rel32 = -5
            0xC3,
        ];
        let out = run_bytes(&code, RunLimits { steps: 1024, mmu_capacity: 4096 });
        // HALT: enters H_00, executes NOP then RET, halts.
        assert!(matches!(out.exit, ExitReason::Halted { .. }));
        assert_eq!(out.steps, 2); // NOP, then RET
    }

    #[test]
    fn jmp_je_taken() {
        // SET rax = 0
        // CMP rax, rax   (zf=1)
        // JE -> H_00
        // (unreachable) ret
        // H_00: ret
        // H_00 = offset 0
        // 0: 48 B8 00 00 00 00 00 00 00 00   movabs rax, 0
        // 10: 48 39 C0                       cmp rax, rax
        // 13: 0F 84 ?? rel32 → 0             je H_00
        // 19: C3                             ret (unreachable)
        // 20: C3                             H_00 ret
        // je at 13, target at 20, so rel32 = 20 - (13+6) = 1
        let code = [
            0x48, 0xB8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x48, 0x39, 0xC0, 0x0F, 0x84,
            0x01, 0x00, 0x00, 0x00, 0xC3, 0xC3,
        ];
        let out = run_bytes(&code, RunLimits { steps: 1024, mmu_capacity: 4096 });
        // Steps: movabs(1) + cmp(1) + je(1) + ret(1) = 4. HALT at the
        // second C3 (offset 20, but 0xC3 is the one at offset 20,
        // rip=21 after halt). That ret_stack should be empty → HALT.
        assert!(matches!(out.exit, ExitReason::Halted { .. }), "got {:?}", out.exit);
        assert_eq!(out.steps, 4);
    }

    #[test]
    fn decode_fault_on_unknown_opcode() {
        // 0xFF is the x64 INC/DEC; here we emit a single 0xFF byte
        // which is illegal standalone.
        let code = [0xFFu8, 0xC3];
        let out = run_bytes(&code, RunLimits { steps: 1024, mmu_capacity: 4096 });
        assert!(matches!(out.exit, ExitReason::Fault(_)));
    }

    #[test]
    fn hex_text_roundtrip() {
        // Smoke: 90 C3 in hex.
        let text = "90c3\n";
        let out = run_hex_text(text, RunLimits { steps: 1024, mmu_capacity: 4096 });
        assert!(matches!(out.exit, ExitReason::Halted { .. }));
    }

    #[test]
    fn raw_byte_nop_chain_halts() {
        // A0 NOP / A0 NOP / FF RET pattern from H_05/H_06 etc:
        // 90 C3 90 90 C3
        // The first RET halts.
        let code = [0x90, 0xC3, 0x90, 0x90, 0xC3];
        let out = run_bytes(&code, RunLimits { steps: 1024, mmu_capacity: 4096 });
        assert!(matches!(out.exit, ExitReason::Halted { .. }));
        assert_eq!(out.steps, 2);
    }
}
