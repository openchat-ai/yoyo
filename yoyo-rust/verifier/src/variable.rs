//! Named slot layer (PROMPT-v3 Part 8).

use crate::types::{IsaError, IsaResult};

/// First user slot (Part 8.4).
pub const USER_SLOT_BASE: u16 = 0x50;
pub const USER_SLOT_MAX: u16 = 0xCF;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg {
    Hex(u64),
    Name(String),
}

#[derive(Debug, Clone)]
pub struct NameEntry {
    pub name: String,
    pub slot: u16,
}

#[derive(Debug, Default)]
pub struct NameTable {
    names: Vec<NameEntry>,
    next_slot: u16,
}

impl NameTable {
    pub fn new() -> Self {
        Self {
            names: Vec::new(),
            next_slot: USER_SLOT_BASE,
        }
    }

    pub fn lookup(&self, name: &str) -> Option<u16> {
        self.names
            .iter()
            .find(|e| e.name == name)
            .map(|e| e.slot)
    }

    pub fn bind(&mut self, name: &str, slot: u16) -> IsaResult<()> {
        if self.lookup(name).is_some() {
            return Ok(());
        }
        self.names.push(NameEntry {
            name: name.to_string(),
            slot,
        });
        Ok(())
    }

    /// Resolve a token: hex → Hex, name → bind-on-first-occurrence → Hex(slot).
    pub fn resolve_or_bind(&mut self, tok: &str) -> IsaResult<Arg> {
        if looks_like_hex(tok) {
            let v = parse_hex(tok).map_err(|msg| IsaError::ParseError {
                line: 0,
                msg,
            })?;
            return Ok(Arg::Hex(v));
        }
        // Named slot
        if let Some(slot) = self.lookup(tok) {
            return Ok(Arg::Hex(slot as u64));
        }
        if self.next_slot > USER_SLOT_MAX {
            return Err(IsaError::SlotOutOfRange {
                slot: self.next_slot,
            });
        }
        let slot = self.next_slot;
        self.next_slot += 1;
        self.bind(tok, slot)?;
        Ok(Arg::Hex(slot as u64))
    }
}

fn looks_like_hex(s: &str) -> bool {
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

fn parse_hex(s: &str) -> Result<u64, String> {
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u64::from_str_radix(s, 16).map_err(|_| format!("bad hex '{s}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_occurrence_assign() {
        let mut t = NameTable::new();
        assert_eq!(t.resolve_or_bind("i").unwrap(), Arg::Hex(0x50));
        assert_eq!(t.resolve_or_bind("n").unwrap(), Arg::Hex(0x51));
        assert_eq!(t.resolve_or_bind("i").unwrap(), Arg::Hex(0x50));
    }

    #[test]
    fn hex_passthrough() {
        let mut t = NameTable::new();
        assert_eq!(t.resolve_or_bind("50").unwrap(), Arg::Hex(0x50));
        assert_eq!(t.resolve_or_bind("0xFF").unwrap(), Arg::Hex(0xFF));
    }
}
