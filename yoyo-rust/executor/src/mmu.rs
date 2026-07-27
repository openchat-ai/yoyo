//! Mmu — flat byte region, fail-closed on OOB (PROMPT-v3 Part 4S.3).
//!
//! v0 surface: a single `Vec<u8>` region. Addresses are interpreted as
//! offsets from a runtime-chosen base pointer `r15`. NULL deref (base +
//! 0) reads/writes byte 0; we leave that to the user — the executor
//! does not simulate page faults but records them as faults.

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fault {
    ReadOob { addr: u64, len: usize },
    WriteOob { addr: u64, len: usize },
    ExecOob { rip: u64 },
    Decode { rip: u64, reason: &'static str },
    Unimplemented { rip: u64, byte: u8 },
    Diverged { rip: u64, msg: &'static str },
    StepLimit { steps: u64 },
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::ReadOob { addr, len } => write!(f, "read OOB at {:#x} ({}B)", addr, len),
            Fault::WriteOob { addr, len } => write!(f, "write OOB at {:#x} ({}B)", addr, len),
            Fault::ExecOob { rip } => write!(f, "execute OOB at {:#x}", rip),
            Fault::Decode { rip, reason } => write!(f, "decode fault at {:#x}: {}", rip, reason),
            Fault::Unimplemented { rip, byte } => {
                write!(f, "unimplemented opcode at {:#x}: {:#04x}", rip, byte)
            }
            Fault::Diverged { rip, msg } => write!(f, "diverge at {:#x}: {}", rip, msg),
            Fault::StepLimit { steps } => write!(f, "step limit {} reached", steps),
        }
    }
}

pub struct Mmu {
    pub bytes: Vec<u8>,
    pub base: u64,
    pub faults: Vec<Fault>,
}

impl Mmu {
    pub fn new(capacity: usize) -> Self {
        let mut v = Vec::with_capacity(capacity);
        v.resize(capacity, 0u8);
        Self {
            bytes: v,
            base: 0,
            faults: Vec::new(),
        }
    }

    pub fn read_u8(&mut self, addr: u64) -> Result<u8, Fault> {
        let off = self.resolve(addr, 1)?;
        Ok(self.bytes[off])
    }

    pub fn read_u64_le(&mut self, addr: u64) -> Result<u64, Fault> {
        let off = self.resolve(addr, 8)?;
        let mut buf = [0u8; 8];
        buf.copy_from_slice(&self.bytes[off..off + 8]);
        Ok(u64::from_le_bytes(buf))
    }

    pub fn write_u8(&mut self, addr: u64, val: u8) -> Result<(), Fault> {
        let off = self.resolve(addr, 1)?;
        self.bytes[off] = val;
        Ok(())
    }

    pub fn write_u64_le(&mut self, addr: u64, val: u64) -> Result<(), Fault> {
        let off = self.resolve(addr, 8)?;
        self.bytes[off..off + 8].copy_from_slice(&val.to_le_bytes());
        Ok(())
    }

    pub fn resolve(&mut self, addr: u64, len: usize) -> Result<usize, Fault> {
        if addr < self.base {
            let f = Fault::ReadOob { addr, len };
            self.faults.push(f);
            return Err(f);
        }
        let off = (addr - self.base) as usize;
        if off.checked_add(len).map_or(true, |end| end > self.bytes.len()) {
            let f = Fault::ReadOob { addr, len };
            self.faults.push(f);
            return Err(f);
        }
        Ok(off)
    }
}
