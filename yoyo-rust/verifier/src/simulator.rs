//! TIR semantic simulator — architecture-independent reference interpreter (DDC ground truth).
//!
//! All backends compile TIR -> native bytes. Simulating TIR semantics gives an
//! architecture-independent reference. Any backend that compiles the same TIR
//! correctly should produce bytecode whose execution is semantically equivalent
//! to this simulation.

use std::collections::HashMap;

use crate::tir::{lower_op_checked, BranchKind, TirInst};
use crate::ty_parser;
use crate::types::{IsaError, IsaResult};

const DEFAULT_STEP_LIMIT: u64 = 1_000_000;

/// Raw comparison flags produced by CMP, consumed by JCC.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CmpFlags {
    eq: bool,
    lt: bool,
    le: bool,
    gt: bool,
    ge: bool,
    below: bool,
    above: bool,
    ae: bool,
    be: bool,
}

impl CmpFlags {
    fn from_pair(a: u64, b: u64) -> Self {
        Self {
            eq: a == b,
            lt: (a as i64) < (b as i64),
            le: (a as i64) <= (b as i64),
            gt: (a as i64) > (b as i64),
            ge: (a as i64) >= (b as i64),
            below: a < b,
            above: a > b,
            ae: a >= b,
            be: a <= b,
        }
    }
}

/// Exit reason when simulation stops.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimExitReason {
    Ret,
    Halted,
    StepLimit { steps: u64 },
    Trap { msg: String },
}

/// Result of a simulation run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimResult {
    pub exit_code: u8,
    pub halted: bool,
    pub steps: u64,
    pub state: HashMap<u16, u64>,
    pub data: Vec<u8>,
    pub exit_reason: SimExitReason,
}

/// TIR semantic simulator.
pub struct Simulator {
    /// 64-bit words at slots 0..N.
    state: HashMap<u16, u64>,
    /// Raw data section (from DATA/STR/RAW).
    data: Vec<u8>,
    /// Current pc as an index into the TIR instruction array.
    pc: usize,
    /// Instructions executed.
    steps: u64,
    /// Hard step limit.
    step_limit: u64,
    /// TIR instructions.
    program: Vec<TirInst>,
    /// Handler ID -> TIR index.
    handlers: HashMap<u16, usize>,
    /// Call return stack.
    call_stack: Vec<usize>,
    /// Flags produced by the most recent CMP.
    cmp_flags: CmpFlags,
}

impl Simulator {
    pub fn new(program: Vec<TirInst>) -> Self {
        let mut handlers = HashMap::new();
        for (idx, inst) in program.iter().enumerate() {
            if let BranchKind::LabelDef = crate::tir::instr_branch_kind(inst.op) {
                let hh = inst.args.first().copied().unwrap_or(0) as u16;
                handlers.insert(hh, idx);
            }
        }

        let step_limit = DEFAULT_STEP_LIMIT;
        let pc = find_entry(&handlers);

        let mut me = Self {
            state: HashMap::new(),
            data: Vec::new(),
            pc,
            steps: 0,
            step_limit,
            program,
            handlers,
            call_stack: Vec::new(),
            cmp_flags: CmpFlags::from_pair(0, 0),
        };
        me.collect_data();
        me
    }

    pub fn with_step_limit(mut self, limit: u64) -> Self {
        self.step_limit = limit;
        self
    }

    fn find_entry(&mut self) {
        let entry = find_entry(&self.handlers);
        self.pc = entry;
    }

    /// First pass: collect DATA/STR/RAW bytes into data[] in TIR order.
    fn collect_data(&mut self) {
        for inst in &self.program {
            match inst.op {
                crate::tir::TirOp::Data => {
                    // DATA <str_idx>: we record the string index as a byte placeholder.
                    // The full string data is not embedded in TIR (emit resolves names);
                    // record the index value itself so the simulator sees something.
                    if let Some(&v) = inst.args.first() {
                        self.data.push(v as u8);
                    }
                }
                crate::tir::TirOp::Str => {
                    // STR <string>: each arg byte is a literal string byte.
                    for &v in &inst.args {
                        self.data.push(v as u8);
                    }
                }
                crate::tir::TirOp::Raw => {
                    // RAW <bytes>: variadic byte list.
                    for &v in &inst.args {
                        self.data.push(v as u8);
                    }
                }
                crate::tir::TirOp::RawBytes => {
                    for &v in &inst.args {
                        self.data.push(v as u8);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn run(mut self) -> SimResult {
        let mut reason = SimExitReason::Halted;
        let mut halted = false;
        let mut exit_code = 1u8;

        loop {
            if self.steps >= self.step_limit {
                reason = SimExitReason::StepLimit {
                    steps: self.steps,
                };
                break;
            }

            if self.pc >= self.program.len() {
                halted = true;
                break;
            }

            let inst = self.program[self.pc].clone();
            self.pc += 1;

            let ctrl = match self.exec(&inst) {
                Ok(c) => c,
                Err(_) => {
                    reason = SimExitReason::Trap {
                        msg: "instruction arg mismatch".to_string(),
                    };
                    break;
                }
            };

            match ctrl {
                SimControl::Continue => {}
                SimControl::Jump(target) => self.pc = target,
                SimControl::Call(target) => {
                    self.call_stack.push(self.pc);
                    self.pc = target;
                }
                SimControl::Ret => {
                    if let Some(ret_pc) = self.call_stack.pop() {
                        self.pc = ret_pc;
                    } else {
                        exit_code = 0;
                        reason = SimExitReason::Ret;
                        break;
                    }
                }
                SimControl::Trap(msg) => {
                    reason = SimExitReason::Trap { msg };
                    break;
                }
            }

            self.steps += 1;
        }

        SimResult {
            exit_code,
            halted,
            steps: self.steps,
            state: self.state,
            data: self.data,
            exit_reason: reason,
        }
    }
}

/// Control-flow outcome of executing one TIR instruction.
enum SimControl {
    Continue,
    Jump(usize),
    Call(usize),
    Ret,
    Trap(String),
}

fn resolve_handler(handlers: &HashMap<u16, usize>, hh: u16) -> IsaResult<usize> {
    handlers
        .get(&hh)
        .copied()
        .ok_or(IsaError::LabelOutOfRange { hh })
}

impl Simulator {
    fn exec(&mut self, inst: &TirInst) -> IsaResult<SimControl> {
        let a = |i: usize| -> IsaResult<u64> {
            inst.args
                .get(i)
                .copied()
                .ok_or(IsaError::ArgCountMismatch {
                    op: 0,
                    expected: i + 1,
                    got: inst.args.len(),
                })
        };

        match inst.op {
            crate::tir::TirOp::Nop => Ok(SimControl::Continue),
            crate::tir::TirOp::Data
            | crate::tir::TirOp::Str
            | crate::tir::TirOp::Raw
            | crate::tir::TirOp::RawByte
            | crate::tir::TirOp::RawBytes => Ok(SimControl::Continue),

            crate::tir::TirOp::Alloc => {
                let slot = a(0)? as u16;
                let size = a(1)?;
                self.state.insert(slot, size);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Set => {
                let slot = a(0)? as u16;
                let imm = a(1)?;
                self.state.insert(slot, imm);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::LoadFile => {
                let slot = a(0)? as u16;
                let str_idx = a(1)?;
                self.state.insert(slot, str_idx);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::WriteFile => {
                let slot = a(0)? as u16;
                let str_idx = a(1)?;
                self.state.insert(slot, str_idx);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Get | crate::tir::TirOp::Movrr => {
                let dst = a(0)? as u16;
                let src = a(1)? as u16;
                let v = self.state.get(&src).copied().unwrap_or(0);
                self.state.insert(dst, v);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Sub => {
                let slot = a(0)? as u16;
                let imm = a(1)?;
                let v = self.state.get(&slot).copied().unwrap_or(0).wrapping_sub(imm);
                self.state.insert(slot, v);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Add => {
                let slot = a(0)? as u16;
                let imm = a(1)?;
                let v = self.state.get(&slot).copied().unwrap_or(0).wrapping_add(imm);
                self.state.insert(slot, v);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Imul => {
                let dst = a(0)? as u16;
                let src = a(1)? as u16;
                let dst_v = self.state.get(&dst).copied().unwrap_or(0);
                let src_v = self.state.get(&src).copied().unwrap_or(0);
                self.state.insert(dst, dst_v.wrapping_mul(src_v));
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Inc => {
                let slot = a(0)? as u16;
                let v = self.state.get(&slot).copied().unwrap_or(0).wrapping_add(1);
                self.state.insert(slot, v);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Dec => {
                let slot = a(0)? as u16;
                let v = self.state.get(&slot).copied().unwrap_or(0).wrapping_sub(1);
                self.state.insert(slot, v);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Addv => {
                let dst = a(0)? as u16;
                let src = a(1)? as u16;
                let dst_v = self.state.get(&dst).copied().unwrap_or(0);
                let src_v = self.state.get(&src).copied().unwrap_or(0);
                self.state.insert(dst, dst_v.wrapping_add(src_v));
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Orv => {
                let dst = a(0)? as u16;
                let src = a(1)? as u16;
                let dst_v = self.state.get(&dst).copied().unwrap_or(0);
                let src_v = self.state.get(&src).copied().unwrap_or(0);
                self.state.insert(dst, dst_v | src_v);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Subv => {
                let dst = a(0)? as u16;
                let src = a(1)? as u16;
                let dst_v = self.state.get(&dst).copied().unwrap_or(0);
                let src_v = self.state.get(&src).copied().unwrap_or(0);
                self.state.insert(dst, dst_v.wrapping_sub(src_v));
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Cmp => {
                let va = self.state.get(&(a(0)? as u16)).copied().unwrap_or(0);
                let vb = self.state.get(&(a(1)? as u16)).copied().unwrap_or(0);
                self.cmp_flags = CmpFlags::from_pair(va, vb);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::Ldb => {
                let dd = a(0)? as u16;
                let ss = a(1)? as u16;
                let oo = a(2)?;
                let base = self.state.get(&ss).copied().unwrap_or(0) as usize;
                let addr = base.saturating_add(oo as usize);
                let v = if addr < self.data.len() {
                    self.data[addr] as u64
                } else {
                    0
                };
                self.state.insert(dd, v);
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::MemcpyData => {
                let _dst = a(0)?;
                let _src = a(1)?;
                let _n = a(2)?;
                Ok(SimControl::Continue)
            }
            crate::tir::TirOp::MemcpyState => {
                let dst = a(0)? as u16;
                let src = a(1)? as u16;
                let n = a(2)? as u16;
                for i in 0..n {
                    let src_slot = (src as u16).checked_add(i).unwrap_or(src as u16);
                    let dst_slot = (dst as u16).checked_add(i).unwrap_or(dst as u16);
                    let v = self.state.get(&src_slot).copied().unwrap_or(0);
                    self.state.insert(dst_slot, v);
                }
                Ok(SimControl::Continue)
            }

            crate::tir::TirOp::Handler => Ok(SimControl::Continue),

            crate::tir::TirOp::Call => {
                let hh = a(0)? as u16;
                let target = resolve_handler(&self.handlers, hh)?;
                Ok(SimControl::Call(target))
            }
            crate::tir::TirOp::Jmp => {
                let hh = a(0)? as u16;
                let target = resolve_handler(&self.handlers, hh)?;
                Ok(SimControl::Jump(target))
            }

            crate::tir::TirOp::Je
            | crate::tir::TirOp::Jne
            | crate::tir::TirOp::Jl
            | crate::tir::TirOp::Jge
            | crate::tir::TirOp::Jle
            | crate::tir::TirOp::Jg
            | crate::tir::TirOp::Jb
            | crate::tir::TirOp::Jae
            | crate::tir::TirOp::Jbe
            | crate::tir::TirOp::Ja => {
                let hh = a(0)? as u16;
                let target = resolve_handler(&self.handlers, hh)?;
                let jump = self.jcc_cond(inst.op);
                if jump {
                    Ok(SimControl::Jump(target))
                } else {
                    Ok(SimControl::Continue)
                }
            }

            crate::tir::TirOp::Ret => Ok(SimControl::Ret),
        }
    }

    fn jcc_cond(&self, op: crate::tir::TirOp) -> bool {
        let f = &self.cmp_flags;
        match op {
            crate::tir::TirOp::Je => f.eq,
            crate::tir::TirOp::Jne => !f.eq,
            crate::tir::TirOp::Jl => f.lt,
            crate::tir::TirOp::Jge => f.ge,
            crate::tir::TirOp::Jle => f.le,
            crate::tir::TirOp::Jg => f.gt,
            crate::tir::TirOp::Jb => f.below,
            crate::tir::TirOp::Jae => f.ae,
            crate::tir::TirOp::Jbe => f.be,
            crate::tir::TirOp::Ja => f.above,
            _ => false,
        }
    }
}

fn find_entry(handlers: &HashMap<u16, usize>) -> usize {
    let mut best = usize::MAX;
    for idx in handlers.values() {
        if *idx < best {
            best = *idx;
        }
    }
    if best == usize::MAX {
        return 0;
    }
    best
}

/// High-level entry: parse .ty text, build simulator, run it.
pub fn simulate_tir_source(src: &str) -> IsaResult<SimResult> {
    let lines = ty_parser::parse(src)?;
    let program = lower_lines_to_tir(&lines)?;
    simulate(&program)
}

/// High-level entry: simulate a TIR instruction array.
pub fn simulate(tir: &[TirInst]) -> IsaResult<SimResult> {
    let program = tir.to_vec();
    Ok(Simulator::new(program).run())
}

fn lower_lines_to_tir(lines: &[ty_parser::SourceLine]) -> IsaResult<Vec<TirInst>> {
    let mut tir = Vec::new();
    for line in lines {
        let args = ty_parser::resolve_line(line)?;
        tir.push(lower_op_checked(line.opcode, &args, line.line)?);
    }
    Ok(tir)
}

impl std::fmt::Display for SimExitReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimExitReason::Ret => write!(f, "RET"),
            SimExitReason::Halted => write!(f, "HALT"),
            SimExitReason::StepLimit { steps } => write!(f, "STEP_LIMIT({steps})"),
            SimExitReason::Trap { msg } => write!(f, "TRAP({msg})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Simulate NOP + RET -> RET, 0 state.
    #[test]
    fn test_nop_ret() {
        let src = "40 00\n  00\n  FF\n";
        let res = simulate_tir_source(src).unwrap();
        assert_eq!(res.exit_reason, SimExitReason::Ret);
        assert_eq!(res.steps, 2); // HANDLER + NOP, RET fires before steps increment
        assert!(res.state.is_empty());
    }

    /// Simulate SET + GET -> correct value propagation.
    #[test]
    fn test_set_get() {
        let src = "40 00\n  30 00 2a\n  60 01 00\n  FF\n";
        let res = simulate_tir_source(src).unwrap();
        assert_eq!(res.exit_reason, SimExitReason::Ret);
        assert_eq!(res.state.get(&0), Some(&0x2a));
        assert_eq!(res.state.get(&1), Some(&0x2a));
    }

    /// Simulate CALL + JMP + RET -> correct control flow.
    #[test]
    fn test_call_jmp_ret() {
        // 0x40 00 = handler 0 (entry)
        // 0x40 01 = handler 1 (target)
        // SET 0 05; CALL handler1; SET 1 0A; RET
        // at handler1: SET 2 0F; RET (returns to after call)
        let src = "40 00\n  30 00 05\n  41 01\n  30 01 0a\n  FF\n40 01\n  30 02 0f\n  FF\n";
        let res = simulate_tir_source(src).unwrap();
        assert_eq!(res.exit_reason, SimExitReason::Ret);
        assert_eq!(res.state.get(&0), Some(&5));
        assert_eq!(res.state.get(&1), Some(&0x0a));
        assert_eq!(res.state.get(&2), Some(&0x0f));
    }
}
