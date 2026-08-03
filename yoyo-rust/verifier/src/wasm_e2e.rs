//! End-to-end Wasm verification.
//!
//! Validates a `.wasm` binary by loading and instantiating it in wasmtime,
//! the reference Wasm runtime.  Phase 1 asserts that the module is
//! syntactically valid and executable (does not trap on instantiation).
//! Phase 2 (future) will compare the Wasm runtime state against the TIR
//! semantic simulator for full DDC equivalence.

use crate::types::{IsaError, IsaResult};

/// Validate (and instantiate) a Wasm module in wasmtime.
///
/// # Errors
/// Returns [`IsaError::PlatformError`] on any wasmtime failure (bad
/// bytecode, instantiation trap, etc.).
pub fn validate_wasm(wasm_bytes: &[u8]) -> IsaResult<()> {
    let engine = wasmtime::Engine::default();
    let module = wasmtime::Module::new(&engine, wasm_bytes)
        .map_err(wasm_error_to_isa)?;
    let mut store = wasmtime::Store::new(&engine, ());

    // Instantiate with no imports (the YOYO backend produces standalone
    // modules with no imports).  Success proves the module is valid and
    // executable.
    wasmtime::Instance::new(&mut store, &module, &[]).map_err(wasm_error_to_isa)?;
    Ok(())
}

fn wasm_error_to_isa(e: wasmtime::Error) -> IsaError {
    IsaError::PlatformError { msg: e.to_string() }
}

// ── Unit tests ───────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tir::{TirInst, TirOp};
    use crate::wasm_backend::emit_wasm;

    fn build_tir_nop_ret() -> Vec<TirInst> {
        vec![
            TirInst { op: TirOp::Handler, args: vec![0], line: 1 },
            TirInst { op: TirOp::Nop, args: vec![], line: 2 },
            TirInst { op: TirOp::Ret, args: vec![], line: 3 },
        ]
    }

    #[test]
    fn wasm_validate_00_nop_ret() {
        let wasm = emit_wasm(&build_tir_nop_ret()).unwrap();
        validate_wasm(&wasm).expect("wasmtime validation failed");
    }

    /// Golden-file integration test: parse `00_nop_ret.ty`, validate the
    /// expected opcodes, then emit a matching TIR and validate in wasmtime.
    #[test]
    fn wasm_e2e_nop_ret() {
        let src = std::fs::read_to_string("../../yoyo/tests/golden/00_nop_ret.ty").unwrap();
        let lines = crate::ty_parser::parse(&src).unwrap();
        let opcodes: Vec<u8> = lines.iter().map(|l| l.opcode).collect();
        // The golden file has: 0x40 (HANDLER), 0x00 (NOP), 0xFF (RET).
        assert!(
            opcodes.contains(&0x40) && opcodes.contains(&0x00) && opcodes.contains(&0xFF),
            "golden file should contain HANDLER+NOP+RET opcodes, got {opcodes:?}",
        );
        // Exercise the full emit→validate path on the equivalent TIR.
        let wasm = emit_wasm(&build_tir_nop_ret()).unwrap();
        validate_wasm(&wasm).expect("wasmtime validation failed (e2e)");
    }
}
