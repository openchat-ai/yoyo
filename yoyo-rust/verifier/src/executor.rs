//! V3 runtime executor sketch (Phase 1b) — hex-token → x64 two-pass.
//! Full PE-at-runtime wrapping is in pe_link; this module owns opcode dispatch
//! for the 22 supported executor opcodes.

use crate::emit;
use crate::platform::PlatformKind;
use crate::tir::lower_op_checked;
use crate::ty_parser;
use crate::tyb_parser;
use crate::types::IsaResult;

/// Compile `.ty` source text to raw x64 (+ data) via the standard pipeline.
pub fn compile_ty_source(src: &str, platform: PlatformKind) -> IsaResult<emit::EmitOutput> {
    let lines = ty_parser::parse(src)?;
    compile_source_lines(&lines, platform)
}

/// Compile `.tyb` binary to raw x64 (+ data) via the standard pipeline.
/// Paper-tape format: 8-byte records, no parser needed.
pub fn compile_tyb_source(data: &[u8], platform: PlatformKind) -> IsaResult<emit::EmitOutput> {
    let lines = tyb_parser::parse_tyb(data)?;
    compile_source_lines(&lines, platform)
}

/// Compile one handler selected directly from canonical `.ty` source bytes.
/// The selected range starts at `HANDLER hh` and stops before the next handler.
/// `hh` is u16 so selectors past `40 FF` (e.g. `40 100`) resolve without wrap.
pub fn compile_one_handler(
    src: &str,
    hh: u16,
    platform: PlatformKind,
) -> IsaResult<emit::EmitOutput> {
    let lines = ty_parser::parse(src)?;
    let start = lines
        .iter()
        .position(|line| {
            line.opcode == 0x40
                && ty_parser::resolve_line(line)
                    .ok()
                    .and_then(|args| args.first().copied())
                    == Some(hh as u64)
        })
        .ok_or(crate::types::IsaError::LabelOutOfRange { hh })?;
    let end = lines[start + 1..]
        .iter()
        .position(|line| line.opcode == 0x40)
        .map(|offset| start + 1 + offset)
        .unwrap_or(lines.len());
    compile_source_lines(&lines[start..end], platform)
}

fn compile_source_lines(
    lines: &[ty_parser::SourceLine],
    platform: PlatformKind,
) -> IsaResult<emit::EmitOutput> {
    let mut tir = Vec::new();
    for line in lines {
        let args = ty_parser::resolve_line(line)?;
        // Skip pure data-definition opcodes that the executor ignores at runtime
        // (still lower them so data section is populated).
        tir.push(lower_op_checked(line.opcode, &args, line.line)?);
    }
    emit::emit(&tir, platform)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_ternary_fragment() {
        let src = r#"
40 20
  30 50 00
  FF
"#;
        let out = compile_ty_source(src, PlatformKind::Stub).unwrap();
        assert!(!out.code.is_empty());
        assert_eq!(*out.code.last().unwrap(), 0xC3);
    }
}
