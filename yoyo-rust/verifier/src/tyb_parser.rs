//! .tyb binary parser — paper-tape format (8-byte records).
//!
//! .tyb is a binary intermediate representation of .ty source.
//! Each .ty line → exactly 8 bytes, fitting one state slot.
//!
//! Format:
//!   [magic:4] = b"TYB\0"
//!   [entry_hh:2][rec_cnt:2]      — header
//!   [op:1][argc:1][a0:4][a1:2] × rec_cnt — records
//!   [rec_idx:4][label_hh:2] [... — fixup table (after records)
//!   [data_bytes]                  — data section (after fixups)
//!
//! For 3-arg instructions: a1 high byte = arg1, low byte = arg2.
//! All args ≤ 0xFF for 3-arg ops, so this fits.
//!
//! Returns Vec<SourceLine> compatible with the existing emit pipeline.

use crate::types::{IsaError, IsaResult};
use crate::variable::Arg;

use crate::ty_parser::SourceLine;

const TYB_MAGIC: &[u8; 4] = b"TYB\0";
const RECORD_SIZE: usize = 8;

/// Parse a .tyb binary into SourceLine entries (compatible with emit pipeline).
/// Labels (opcode 0x40) are emitted as SourceLine entries with opcode=0x40.
/// Branch instructions carry their label-hh as arg[0] raw.
/// Data/STR/RAW opcodes are emitted as SourceLine entries.
pub fn parse_tyb(data: &[u8]) -> IsaResult<Vec<SourceLine>> {
    if data.len() < 8 {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(".tyb too short: {} bytes", data.len()),
        });
    }
    if &data[0..4] != TYB_MAGIC {
        return Err(IsaError::ParseError {
            line: 0,
            msg: "bad .tyb magic: expected TYB\\0".into(),
        });
    }

    let rec_cnt = u16::from_le_bytes([data[6], data[7]]) as usize;
    let header_size = 8;

    // Validate size
    let expected_min = header_size + rec_cnt * RECORD_SIZE;
    if data.len() < expected_min {
        return Err(IsaError::ParseError {
            line: 0,
            msg: format!(
                ".tyb truncated: need {} bytes for {} records, got {}",
                expected_min,
                rec_cnt,
                data.len()
            ),
        });
    }

    let mut out = Vec::with_capacity(rec_cnt);
    let mut line_no = 0;

    for i in 0..rec_cnt {
        let off = header_size + i * RECORD_SIZE;
        if off + RECORD_SIZE > data.len() {
            break;
        }
        line_no += 1;

        let op = data[off];
        let argc = data[off + 1] as usize;

        let args_raw: Vec<u64> = if argc == 0 {
            vec![]
        } else if argc == 1 {
            let a0 = u32::from_le_bytes([data[off + 2], data[off + 3], data[off + 4], data[off + 5]]) as u64;
            vec![a0]
        } else if argc == 2 {
            let a0 = u16::from_le_bytes([data[off + 2], data[off + 3]]) as u64;
            let a1 = u32::from_le_bytes([data[off + 4], data[off + 5], data[off + 6], data[off + 7]]) as u64;
            vec![a0, a1]
        } else {
            // argc == 3
            let a0 = u16::from_le_bytes([data[off + 2], data[off + 3]]) as u64;
            let a1 = u16::from_le_bytes([data[off + 4], data[off + 5]]) as u64;
            let a2 = u16::from_le_bytes([data[off + 6], data[off + 7]]) as u64;
            vec![a0, a1, a2]
        };
        let args: Vec<Arg> = args_raw[..argc.min(3)]
            .iter()
            .map(|&v| Arg::Hex(v))
            .collect();

        // Build a raw token string for SourceLine.raw
        let raw = format!(
            "{:02X} {}",
            op,
            args.iter()
                .map(|a| match a {
                    Arg::Hex(v) => format!("{:X}", v),
                    Arg::Name(n) => n.clone(),
                })
                .collect::<Vec<_>>()
                .join(" ")
        );

        out.push(SourceLine {
            line: line_no,
            opcode: op,
            args,
            raw: String::new(), // raw tokens not needed for binary path
        });
        let _ = raw;
    }

    Ok(out)
}

/// Detect if a file is .tyb by checking magic bytes.
pub fn is_tyb(data: &[u8]) -> bool {
    data.len() >= 4 && &data[0..4] == TYB_MAGIC
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_record(op: u8, argc: u8, a0: u32, a1: u32, a2: u16) -> Vec<u8> {
        let mut r = Vec::with_capacity(8);
        r.push(op);
        r.push(argc);
        match argc {
            0 => r.extend_from_slice(&[0u8; 6]),
            1 => {
                r.extend_from_slice(&a0.to_le_bytes());
                r.extend_from_slice(&[0u8; 2]);
            }
            2 => {
                r.extend_from_slice(&(a0 as u16).to_le_bytes());
                r.extend_from_slice(&a1.to_le_bytes());
            }
            _ => {
                // 3
                r.extend_from_slice(&(a0 as u16).to_le_bytes());
                r.extend_from_slice(&(a1 as u16).to_le_bytes());
                r.extend_from_slice(&a2.to_le_bytes());
            }
        }
        r
    }

    #[test]
    fn parse_tyb_set_ret() {
        let mut data = Vec::new();
        data.extend_from_slice(b"TYB\0");
        data.extend_from_slice(&[0x00u8, 0x00u8]); // entry_hh=0
        data.extend_from_slice(&[2u8, 0x00u8]); // rec_cnt=2
        // SET slot=0x50 imm=0
        data.extend_from_slice(&make_record(0x30, 2, 0x50, 0x00, 0));
        // RET
        data.extend_from_slice(&make_record(0xFF, 0, 0, 0, 0));

        let lines = parse_tyb(&data).unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].opcode, 0x30);
        assert_eq!(lines[1].opcode, 0xFF);
    }

    #[test]
    fn parse_tyb_label() {
        let mut data = Vec::new();
        data.extend_from_slice(b"TYB\0");
        data.extend_from_slice(&[0x14u8, 0x00u8]); // entry_hh=0x14
        data.extend_from_slice(&[2u8, 0x00u8]); // rec_cnt=2
        // LABEL H_14
        data.extend_from_slice(&make_record(0x40, 1, 0x14, 0, 0));
        // SET slot=0x50 imm=0
        data.extend_from_slice(&make_record(0x30, 2, 0x50, 0, 0));

        let lines = parse_tyb(&data).unwrap();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0].opcode, 0x40);
    }

    #[test]
    fn bad_magic() {
        let data = b"NOTTYB\0";
        assert!(parse_tyb(data).is_err());
    }

    #[test]
    fn detect_tyb() {
        assert!(is_tyb(b"TYB\0..."));
        assert!(!is_tyb(b"30 50 00"));
    }
}