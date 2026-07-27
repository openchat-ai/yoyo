//! .ty source parser — hex tokens + optional named slots (Part 8).

use crate::types::{IsaError, IsaResult};
use crate::variable::{Arg, NameTable};

#[derive(Debug, Clone)]
pub struct SourceLine {
    pub line: usize,
    pub opcode: u8,
    pub args: Vec<Arg>,
    pub raw: String,
}

/// Parse a full `.ty` source into SourceLine entries.
/// Skips blank lines and `;` / `#` comments.
pub fn parse(source: &str) -> IsaResult<Vec<SourceLine>> {
    let mut names = NameTable::new();
    let mut out = Vec::new();

    for (idx, raw_line) in source.lines().enumerate() {
        let line_no = idx + 1;
        let mut line = raw_line.to_string();
        if let Some(i) = line.find(';') {
            line.truncate(i);
        }
        if let Some(i) = line.find('#') {
            // don't strip # inside strings — .ty has no strings in tokens
            line.truncate(i);
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        // LAYOUT / END_LAYOUT handled by variable layer
        if line.eq_ignore_ascii_case("LAYOUT") || line.eq_ignore_ascii_case("END_LAYOUT") {
            continue;
        }

        let toks: Vec<&str> = line.split_whitespace().collect();
        if toks.is_empty() {
            continue;
        }
        let opcode = parse_hex_u8(toks[0]).map_err(|msg| IsaError::ParseError {
            line: line_no,
            msg,
        })?;

        let mut args = Vec::new();
        for t in &toks[1..] {
            args.push(names.resolve_or_bind(t).map_err(|e| match e {
                IsaError::UndefinedName { name } => IsaError::ParseError {
                    line: line_no,
                    msg: format!("undefined name '{name}'"),
                },
                other => other,
            })?);
        }

        out.push(SourceLine {
            line: line_no,
            opcode,
            args,
            raw: raw_line.to_string(),
        });
    }
    Ok(out)
}

fn parse_hex_u8(s: &str) -> Result<u8, String> {
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u8::from_str_radix(s, 16).map_err(|_| format!("bad hex byte '{s}'"))
}

pub fn parse_hex_u64(s: &str) -> Result<u64, String> {
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u64::from_str_radix(s, 16).map_err(|_| format!("bad hex u64 '{s}'"))
}

/// Resolve SourceLine args to numeric u64 (after name binding).
pub fn resolve_line(line: &SourceLine) -> IsaResult<Vec<u64>> {
    line.args
        .iter()
        .map(|a| match a {
            Arg::Hex(v) => Ok(*v),
            Arg::Name(n) => Err(IsaError::UndefinedName { name: n.clone() }),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_set_hex() {
        let lines = parse("30 50 00\n").unwrap();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].opcode, 0x30);
        assert_eq!(resolve_line(&lines[0]).unwrap(), vec![0x50, 0]);
    }

    #[test]
    fn parse_named() {
        let lines = parse("30 i 0\n30 n 7\n66 i\n").unwrap();
        assert_eq!(lines.len(), 3);
        let a = resolve_line(&lines[0]).unwrap();
        let b = resolve_line(&lines[1]).unwrap();
        let c = resolve_line(&lines[2]).unwrap();
        assert_eq!(a[0], 0x50); // first name → 0x50
        assert_eq!(b[0], 0x51);
        assert_eq!(c[0], 0x50); // i again
    }

    #[test]
    fn skip_comments() {
        let lines = parse("; hi\n30 50 00 ; set\n").unwrap();
        assert_eq!(lines.len(), 1);
    }
}
