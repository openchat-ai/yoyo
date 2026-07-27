//! Fixed label table — replaces HashMap (PROMPT-v3 Part 9.2.1).
//! body-extend-040: widened from 256 → 512 so HANDLER ids ≥0x100
//! (e.g. `40 100`) do not wrap and collide with H_00.. via u8 truncation.
//! body-extend-072: widened 512 → 1024 so HANDLER ids ≥0x200 (e.g. `40 200`)
//! stay in-range; JS peer already accepts 0..0xffff via Map (fail-closed parity).

use crate::types::{IsaError, IsaResult};

/// Max handler/label id + 1. Ids are u16; table stays fixed-cap (no HashMap).
pub const LABEL_CAP: usize = 1024;

/// [(hh, offset); LABEL_CAP] — hh is the handler id.
#[derive(Debug, Clone)]
pub struct FixupTable {
    /// offset[hh] = Some(code_offset) when defined
    offs: [Option<u32>; LABEL_CAP],
}

impl FixupTable {
    pub fn new() -> Self {
        Self {
            offs: [None; LABEL_CAP],
        }
    }

    pub fn define(&mut self, hh: u16, offset: u32) -> IsaResult<()> {
        let idx = hh as usize;
        if idx >= LABEL_CAP {
            return Err(IsaError::LabelOutOfRange { hh });
        }
        if self.offs[idx].is_some() {
            // Allow redefinition to same offset; reject conflicting
            if self.offs[idx] != Some(offset) {
                return Err(IsaError::LabelOutOfRange { hh });
            }
        }
        self.offs[idx] = Some(offset);
        Ok(())
    }

    pub fn lookup(&self, hh: u16) -> Option<u32> {
        let idx = hh as usize;
        if idx >= LABEL_CAP {
            return None;
        }
        self.offs[idx]
    }
}

impl Default for FixupTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn define_and_lookup() {
        let mut t = FixupTable::new();
        t.define(0x20, 100).unwrap();
        assert_eq!(t.lookup(0x20), Some(100));
        assert_eq!(t.lookup(0x21), None);
    }

    #[test]
    fn define_past_u8_no_wrap() {
        let mut t = FixupTable::new();
        t.define(0x00, 10).unwrap();
        t.define(0x100, 999).unwrap();
        assert_eq!(t.lookup(0x00), Some(10));
        assert_eq!(t.lookup(0x100), Some(999));
        // Must NOT have overwritten H_00 via u8 wrap
        assert_ne!(t.lookup(0x00), Some(999));
    }

    #[test]
    fn define_past_512_no_wrap() {
        let mut t = FixupTable::new();
        t.define(0x00, 10).unwrap();
        t.define(0x200, 888).unwrap();
        t.define(0x202, 777).unwrap();
        assert_eq!(t.lookup(0x00), Some(10));
        assert_eq!(t.lookup(0x200), Some(888));
        assert_eq!(t.lookup(0x202), Some(777));
        assert_ne!(t.lookup(0x00), Some(888));
    }
}
