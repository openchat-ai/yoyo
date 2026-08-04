//! Wasm module runner via wasmtime — executes a `.wasm` binary and reports
//! semantics suitable for comparison against the TIR simulator (DDC peer).
//!
//! The YOYO Wasm backend emits a module with exactly one exported function
//! whose signature is `(param N i32) -> ()`, a single 1-page memory, and an
//! optional data section. State slots are represented as the i32 params
//! (initially all zero); RET is encoded as `unreachable`.

use crate::simulator::SimExitReason;
use crate::types::{IsaError, IsaResult};

use wasmtime::{Config, Engine, ExternType, Instance, Module, Store, Val};

/// Exit outcome of a Wasm execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WasmExitReason {
    Normal,
    Trap { kind: TrapKind },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrapKind {
    Unreachable,
    OutOfFuel,
    Unknown { msg: String },
}

/// Result returned by `run_wasm`. Mirrors `SimResult` enough for DDC
/// comparison.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WasmRunResult {
    pub exit_reason: WasmExitReason,
    pub steps: u64,
    pub final_memory: Vec<u8>,
    pub memory_size_pages: u32,
    pub slot_count: u32,
    pub error: Option<String>,
}

/// Upper bound on how much fuel to grant a module.
const DEFAULT_FUEL: u64 = 1_000_000;

/// Load the Wasm bytes, instantiate, call the entry function with N zero
/// i32 params, and return execution + memory state.
pub fn run_wasm(wasm_bytes: &[u8]) -> IsaResult<WasmRunResult> {
    let mut config = Config::new();
    config.consume_fuel(true);
    config.cranelift_opt_level(wasmtime::OptLevel::None);
    let engine = Engine::new(&config).map_err(|e| IsaError::IoError {
        msg: format!("Engine::new failed: {e}"),
    })?;

    let module =
        Module::new(&engine, wasm_bytes).map_err(|e| IsaError::IoError {
            msg: format!("Module::new failed: {e}"),
        })?;

    let slot_count = determine_slot_count(&module)?;

    let linker = wasmtime::Linker::new(&engine);
    let mut store = Store::new(&engine, ());
    store
        .set_fuel(DEFAULT_FUEL)
        .map_err(|e| IsaError::IoError {
            msg: format!("set_fuel failed: {e}"),
        })?;

    let instance = linker
        .instantiate(&mut store, &module)
        .map_err(|e| IsaError::IoError {
            msg: format!("linker.instantiate failed: {e}"),
        })?;

    let fuel_before = store
        .get_fuel()
        .map_err(|e| IsaError::IoError {
            msg: format!("get_fuel (before) failed: {e}"),
        })?;

    let call_result = call_entry(&mut store, &instance, slot_count);

    let fuel_after = match &call_result {
        Ok(()) => store.get_fuel().map_err(|e| IsaError::IoError {
            msg: format!("get_fuel (after) failed: {e}"),
        })?,
        Err(trap) => {
            let trap_kind = classify_trap(trap);
            let remaining = store
                .get_fuel()
                .map_err(|e| IsaError::IoError {
                    msg: format!("get_fuel (after trap) failed: {e}"),
                })
                .unwrap_or(0);
            return Ok(WasmRunResult {
                exit_reason: WasmExitReason::Trap { kind: trap_kind },
                steps: if fuel_before > remaining {
                    fuel_before - remaining
                } else {
                    0
                },
                final_memory: dump_memory(&mut store, &instance),
                memory_size_pages: memory_pages(&mut store, &instance),
                slot_count,
                error: None,
            });
        }
    };

    let steps = if fuel_before > fuel_after {
        fuel_before - fuel_after
    } else {
        0
    };

    let final_memory = dump_memory(&mut store, &instance);
    let memory_pages = memory_pages(&mut store, &instance);

    Ok(WasmRunResult {
        exit_reason: WasmExitReason::Normal,
        steps,
        final_memory,
        memory_size_pages: memory_pages,
        slot_count,
        error: None,
    })
}

/// Number of i32 state slots = param count of the exported function.
fn determine_slot_count(module: &Module) -> IsaResult<u32> {
    let mut func_types = module.exports().filter_map(|ex| match ex.ty() {
        ExternType::Func(ft) => Some(ft),
        _ => None,
    });
    let func_type = func_types.next().ok_or_else(|| IsaError::IoError {
        msg: "no exported function found in Wasm module".to_string(),
    })?;
    Ok(func_type.params().count() as u32)
}

fn dump_memory(store: &mut Store<()>, instance: &Instance) -> Vec<u8> {
    let mem = instance.get_memory(&mut *store, "memory");
    match mem {
        Some(m) => {
            let sz = m.data_size(&*store);
            let mut buf = vec![0u8; sz];
            let data = m.data(&*store);
            buf.copy_from_slice(data);
            buf
        }
        None => Vec::new(),
    }
}

fn memory_pages(store: &mut Store<()>, instance: &Instance) -> u32 {
    let mem = instance.get_memory(&mut *store, "memory");
    match mem {
        Some(m) => m.size(&*store) as u32,
        None => 0,
    }
}

/// Call the entry function with N i32(0) params. Prefer an export named
/// "main"; otherwise pick the first exported function.
fn call_entry(
    store: &mut Store<()>,
    instance: &Instance,
    n_params: u32,
) -> Result<(), wasmtime::Error> {
    let exports: Vec<_> = instance
        .exports(&mut *store)
        .filter_map(|ex| {
            let name = ex.name().to_string();
            ex.into_func().map(|f| (name, f))
        })
        .collect();
    if exports.is_empty() {
        return Err(wasmtime::Error::msg(
            "no exported function found in Wasm module",
        ));
    }
    let (_, func) = exports
        .iter()
        .find(|(name, _)| name == "main")
        .unwrap_or(&exports[0]);
    let actual_params = {
        let ty = func.ty(&*store);
        ty.params().count()
    };
    let _ = n_params;
    let args: Vec<Val> = (0..actual_params).map(|_| Val::I32(0)).collect();
    let mut results = Vec::new();
    func.call(store, &args, &mut results)
}

fn classify_trap(err: &wasmtime::Error) -> TrapKind {
    // Try to extract the wasmtime Trap enum from the error.
    if let Some(trap) = err.downcast_ref::<wasmtime::Trap>() {
        return match trap {
            wasmtime::Trap::UnreachableCodeReached => TrapKind::Unreachable,
            wasmtime::Trap::OutOfFuel => TrapKind::OutOfFuel,
            _ => TrapKind::Unknown {
                msg: format!("{trap:?}"),
            },
        };
    }
    // Fallback: heuristics on the stringified message.
    let msg = err.to_string().to_lowercase();
    if msg.contains("out of fuel") || msg.contains("fuel") {
        TrapKind::OutOfFuel
    } else if msg.contains("unreachable") {
        TrapKind::Unreachable
    } else {
        TrapKind::Unknown {
            msg: err.to_string(),
        }
    }
}

/// Pretty-print the WasmRunResult.
pub fn print_result(result: &WasmRunResult) {
    let reason_str = match &result.exit_reason {
        WasmExitReason::Normal => "NORMAL".to_string(),
        WasmExitReason::Trap {
            kind: TrapKind::Unreachable,
        } => "RET".to_string(),
        WasmExitReason::Trap {
            kind: TrapKind::OutOfFuel,
        } => "OUT_OF_FUEL".to_string(),
        WasmExitReason::Trap {
            kind: TrapKind::Unknown { msg },
        } => format!("TRAP({msg})"),
    };
    if let Some(ref err) = result.error {
        eprintln!("error: {err}");
        return;
    }
    println!(
        "wasm    : exit={} fuel_used={} memory_pages={} slots={}",
        reason_str,
        result.steps,
        result.memory_size_pages,
        result.slot_count,
    );
    let hex: Vec<String> = result
        .final_memory
        .iter()
        .take(64)
        .map(|b| format!("{b:02X}"))
        .collect();
    println!("memory  : {}", hex.join(" "));
}

/// Compare a WasmRunResult with the TIR simulator's exit reason at a
/// semantic level: both should indicate RET, or both a fatal trap / halt.
pub fn compare_with_sim(wasm: &WasmRunResult, sim: &SimExitReason) -> String {
    let wasm_is_ret = matches!(
        &wasm.exit_reason,
        WasmExitReason::Trap {
            kind: TrapKind::Unreachable
        }
    );
    let sim_is_ret = matches!(sim, SimExitReason::Ret);

    match (wasm_is_ret, sim_is_ret) {
        (true, true) => "MATCH: both RET".into(),
        (false, false) => {
            if wasm.error.is_some() {
                "MISMATCH: Wasm failed to load/run".into()
            } else {
                "MATCH: both halted/non-RET (DDC semantic-equivalent)".into()
            }
        }
        (true, false) => format!("MISMATCH: Wasm=RET, simulator={sim}"),
        (false, true) => format!("MISMATCH: Wasm=non-RET, simulator=RET"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wasm_backend::emit_wasm;
    use crate::tir::{TirInst, TirOp};

    fn tir_nop_ret() -> Vec<TirInst> {
        vec![
            TirInst {
                op: TirOp::Handler,
                args: vec![0],
                line: 1,
            },
            TirInst {
                op: TirOp::Nop,
                args: vec![],
                line: 2,
            },
            TirInst {
                op: TirOp::Ret,
                args: vec![],
                line: 3,
            },
        ]
    }

    #[test]
    fn run_wasm_nop_ret_traps_with_unreachable() {
        let bytes = emit_wasm(&tir_nop_ret()).unwrap();
        let result = run_wasm(&bytes).unwrap();
        assert!(result.error.is_none());
        assert!(
            matches!(
                &result.exit_reason,
                WasmExitReason::Trap {
                    kind: TrapKind::Unreachable
                }
            ),
            "expected unreachable trap, got {:#?}",
            result.exit_reason
        );
    }

    #[test]
    fn run_wasm_memory_dumped_one_page() {
        let bytes = emit_wasm(&tir_nop_ret()).unwrap();
        let result = run_wasm(&bytes).unwrap();
        assert_eq!(result.final_memory.len(), 65536);
        assert_eq!(result.memory_size_pages, 1);
    }

    #[test]
    fn run_wasm_slot_count() {
        let bytes = emit_wasm(&tir_nop_ret()).unwrap();
        let result = run_wasm(&bytes).unwrap();
        // nop-ret uses no state slots (max_slot = 0 -> local_count = 0 + 1 = 1)
        assert!(result.slot_count >= 1);
    }
}