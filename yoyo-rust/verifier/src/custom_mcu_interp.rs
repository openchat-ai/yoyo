//! Custom MCU scaffold interpreter — smoke NOP (0x00) + RET (0xC3) for DDC/backends.
//!
//! Copy this module when adding a real chip backend: replace opcode constants and
//! extend the decode loop to match your `CustomMcuPlatform` emit encodings.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecExitReason {
    Ret,
    Halted,
    StepLimit { steps: u64 },
    Fault { msg: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecResult {
    pub exit_reason: ExecExitReason,
    pub steps: u64,
}

pub const CUSTOM_MCU_NOP: u8 = 0x00;
pub const CUSTOM_MCU_RET: u8 = 0xC3;

const STEP_LIMIT: u64 = 1_000_000;

pub fn run_custom_mcu(bytes: &[u8]) -> ExecResult {
    let mut pc = 0usize;
    let mut steps = 0u64;
    loop {
        if steps >= STEP_LIMIT {
            return ExecResult {
                exit_reason: ExecExitReason::StepLimit { steps },
                steps,
            };
        }
        if pc >= bytes.len() {
            return ExecResult {
                exit_reason: ExecExitReason::Halted,
                steps,
            };
        }
        steps += 1;
        match bytes[pc] {
            CUSTOM_MCU_NOP => pc += 1,
            CUSTOM_MCU_RET => {
                return ExecResult {
                    exit_reason: ExecExitReason::Ret,
                    steps,
                };
            }
            b => {
                return ExecResult {
                    exit_reason: ExecExitReason::Fault {
                        msg: format!("unknown opcode 0x{b:02X} at offset {pc}"),
                    },
                    steps,
                };
            }
        }
    }
}
