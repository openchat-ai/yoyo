//! ISA table parser (PROMPT-v3 Part 4.4.3 grammar).
//!
//! Line form: `0x30 SET slot imm => movabs rax imm store_state slot rax`

#[derive(Debug, Clone)]
pub struct IsaRow {
    pub opcode: u8,
    pub mnemonic: String,
    pub params: Vec<String>,
    pub pattern: Vec<String>,
}

impl IsaRow {
    pub fn variant_name(&self) -> String {
        // SET → Set, LOAD_FILE → LoadFile, ADDV → Addv
        let mut out = String::new();
        let mut cap = true;
        for ch in self.mnemonic.chars() {
            if ch == '_' {
                cap = true;
                continue;
            }
            if cap {
                out.extend(ch.to_uppercase());
                cap = false;
            } else {
                out.extend(ch.to_lowercase());
            }
        }
        out
    }
}

pub fn parse_isa_table(src: &str) -> Result<Vec<IsaRow>, String> {
    let mut rows = Vec::new();
    let mut pending = String::new();

    for (lineno, raw) in src.lines().enumerate() {
        let line_no = lineno + 1;
        let mut line = raw.to_string();
        if let Some(i) = line.find(';') {
            line.truncate(i);
        }
        if let Some(i) = line.find('#') {
            line.truncate(i);
        }
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Some(stripped) = line.strip_suffix('+') {
            pending.push_str(stripped.trim());
            pending.push(' ');
            continue;
        }

        let full = if pending.is_empty() {
            line.to_string()
        } else {
            let mut s = pending.clone();
            s.push_str(line);
            pending.clear();
            s
        };

        rows.push(parse_row(&full).map_err(|e| format!("line {line_no}: {e}"))?);
    }

    if !pending.is_empty() {
        return Err("dangling continuation (+) at EOF".into());
    }
    Ok(rows)
}

fn parse_row(line: &str) -> Result<IsaRow, String> {
    let (left, right) = line
        .split_once("=>")
        .ok_or_else(|| "missing => separator".to_string())?;
    let left_toks: Vec<&str> = left.split_whitespace().collect();
    if left_toks.len() < 2 {
        return Err("need at least opcode and mnemonic".into());
    }
    let opcode = parse_opcode(left_toks[0])?;
    let mnemonic = left_toks[1].to_string();
    if !mnemonic.chars().next().map(|c| c.is_ascii_alphabetic()).unwrap_or(false) {
        return Err(format!("bad mnemonic '{mnemonic}'"));
    }
    let params: Vec<String> = left_toks[2..].iter().map(|s| s.to_string()).collect();
    let pattern: Vec<String> = right.split_whitespace().map(|s| s.to_string()).collect();
    Ok(IsaRow {
        opcode,
        mnemonic,
        params,
        pattern,
    })
}

fn parse_opcode(s: &str) -> Result<u8, String> {
    let s = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u8::from_str_radix(s, 16).map_err(|_| format!("bad opcode '{s}'"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_set() {
        let rows = parse_isa_table("0x30 SET slot imm => movabs rax imm store_state slot rax\n").unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].opcode, 0x30);
        assert_eq!(rows[0].mnemonic, "SET");
        assert_eq!(rows[0].variant_name(), "Set");
    }
}
