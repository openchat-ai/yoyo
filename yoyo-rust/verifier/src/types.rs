//! Phase 0 foundation types (PROMPT-v3 Part 4.3 / Part 9).
//! Zero dynamic allocation on the emit hot path via FixedBuf.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

/// Emit-path error. Public APIs return `IsaResult`; no panics in emit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IsaError {
    SlotOutOfRange { slot: u16 },
    ImmOutOfRange { value: u64, max: u64 },
    InvalidConditionCode { cc: u8 },
    InvalidRegister { reg: u8 },
    LabelOutOfRange { hh: u16 },
    BufferOverflow { needed: usize, available: usize },
    ArgCountMismatch { op: u8, expected: usize, got: usize },
    UndefinedName { name: String },
    DuplicateOpcode { op: u8 },
    BudgetExceeded { used: u64, max: u64 },
    ParseError { line: usize, msg: String },
    IoError { msg: String },
    PlatformError { msg: String },
}

impl fmt::Display for IsaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for IsaError {}

pub type IsaResult<T> = Result<T, IsaError>;

/// Fixed-capacity byte buffer. No heap growth on the emit path.
#[derive(Clone)]
pub struct FixedBuf<const N: usize> {
    data: [u8; N],
    len: usize,
}

impl<const N: usize> FixedBuf<N> {
    pub const fn new() -> Self {
        Self {
            data: [0u8; N],
            len: 0,
        }
    }

    /// Heap-allocate without a 1MB stack temporary (Windows default stack is 1MB).
    pub fn new_boxed() -> Box<Self> {
        unsafe {
            let layout = std::alloc::Layout::new::<Self>();
            let ptr = std::alloc::alloc_zeroed(layout) as *mut Self;
            if ptr.is_null() {
                std::alloc::handle_alloc_error(layout);
            }
            // zeroed ⇒ len == 0, data all zeros — valid FixedBuf
            Box::from_raw(ptr)
        }
    }

    pub fn push(&mut self, byte: u8) -> IsaResult<()> {
        if self.len >= N {
            return Err(IsaError::BufferOverflow {
                needed: self.len + 1,
                available: N,
            });
        }
        self.data[self.len] = byte;
        self.len += 1;
        Ok(())
    }

    pub fn extend_from_slice(&mut self, bytes: &[u8]) -> IsaResult<()> {
        if self.len + bytes.len() > N {
            return Err(IsaError::BufferOverflow {
                needed: self.len + bytes.len(),
                available: N,
            });
        }
        self.data[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }

    pub fn slice(&self) -> &[u8] {
        &self.data[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.data[..self.len]
    }

    pub fn tell(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        self.len = 0;
    }

    pub fn capacity(&self) -> usize {
        N
    }

    /// Patch a previously written u32 LE at absolute offset.
    pub fn patch_u32_le(&mut self, off: usize, val: u32) -> IsaResult<()> {
        if off + 4 > self.len {
            return Err(IsaError::BufferOverflow {
                needed: off + 4,
                available: self.len,
            });
        }
        self.data[off..off + 4].copy_from_slice(&val.to_le_bytes());
        Ok(())
    }
}

impl<const N: usize> Default for FixedBuf<N> {
    fn default() -> Self {
        Self::new()
    }
}

/// x64 GPRs used by YOYO emit (Decision #7).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Reg {
    Rax = 0,
    Rcx = 1,
    Rdx = 2,
    Rbx = 3,
    Rsp = 4,
    Rbp = 5,
    Rsi = 6,
    Rdi = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,
}

impl Reg {
    pub fn from_u8(v: u8) -> IsaResult<Self> {
        match v {
            0 => Ok(Reg::Rax),
            1 => Ok(Reg::Rcx),
            2 => Ok(Reg::Rdx),
            3 => Ok(Reg::Rbx),
            4 => Ok(Reg::Rsp),
            5 => Ok(Reg::Rbp),
            6 => Ok(Reg::Rsi),
            7 => Ok(Reg::Rdi),
            8 => Ok(Reg::R8),
            9 => Ok(Reg::R9),
            10 => Ok(Reg::R10),
            11 => Ok(Reg::R11),
            12 => Ok(Reg::R12),
            13 => Ok(Reg::R13),
            14 => Ok(Reg::R14),
            15 => Ok(Reg::R15),
            _ => Err(IsaError::InvalidRegister { reg: v }),
        }
    }

    /// Low 3 bits for ModRM / opcode+rd.
    pub fn low3(self) -> u8 {
        (self as u8) & 7
    }

    /// REX.R / REX.B high bit.
    pub fn rex_bit(self) -> bool {
        (self as u8) >= 8
    }

    /// ModRM reg field bits (bits 3-5), without REX.
    pub fn modrm_bits(self) -> u8 {
        self.low3() << 3
    }
}

/// Budget-limited execution (Decision #10).
pub struct Budget {
    pub max: u64,
    current: AtomicU64,
}

impl Budget {
    pub fn new(max: u64) -> Self {
        Self {
            max,
            current: AtomicU64::new(0),
        }
    }

    pub fn consume(&self, n: u64) -> IsaResult<()> {
        let prev = self.current.fetch_add(n, Ordering::SeqCst);
        if prev.saturating_add(n) > self.max {
            return Err(IsaError::BudgetExceeded {
                used: prev.saturating_add(n),
                max: self.max,
            });
        }
        Ok(())
    }

    pub fn used(&self) -> u64 {
        self.current.load(Ordering::SeqCst)
    }

    pub fn remaining(&self) -> u64 {
        self.max.saturating_sub(self.used())
    }

    pub fn reset(&self) {
        self.current.store(0, Ordering::SeqCst);
    }
}

/// Progress observer hook (Decision #11).
pub struct Progress {
    pub ops: AtomicU64,
}

impl Progress {
    pub fn new() -> Self {
        Self {
            ops: AtomicU64::new(0),
        }
    }

    pub fn tick(&self) {
        self.ops.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get(&self) -> u64 {
        self.ops.load(Ordering::Relaxed)
    }
}

impl Default for Progress {
    fn default() -> Self {
        Self::new()
    }
}

/// Default budgets from Part 9.2.4.
pub const BUDGET_PHASE0: u64 = 1_000_000;
pub const BUDGET_PHASE1: u64 = 1_000_000_000;
pub const BUDGET_PHASE2: u64 = 10_000_000_000;

/// Code buffer capacity (1 MB).
pub const CODE_BUF_CAP: usize = 1_048_576;
/// Data buffer capacity (64 KB).
pub const DATA_BUF_CAP: usize = 65_536;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_buf_push_and_overflow() {
        let mut b = FixedBuf::<4>::new();
        assert!(b.push(1).is_ok());
        assert!(b.push(2).is_ok());
        assert!(b.push(3).is_ok());
        assert!(b.push(4).is_ok());
        assert!(matches!(
            b.push(5),
            Err(IsaError::BufferOverflow { .. })
        ));
        assert_eq!(b.slice(), &[1, 2, 3, 4]);
    }

    #[test]
    fn reg_modrm_and_rex() {
        assert_eq!(Reg::Rax.low3(), 0);
        assert_eq!(Reg::Rdi.low3(), 7);
        assert!(!Reg::Rax.rex_bit());
        assert!(Reg::R15.rex_bit());
        assert_eq!(Reg::Rcx.modrm_bits(), 0x08);
    }

    #[test]
    fn budget_consume() {
        let b = Budget::new(10);
        assert!(b.consume(5).is_ok());
        assert!(b.consume(5).is_ok());
        assert!(matches!(
            b.consume(1),
            Err(IsaError::BudgetExceeded { .. })
        ));
    }
}
