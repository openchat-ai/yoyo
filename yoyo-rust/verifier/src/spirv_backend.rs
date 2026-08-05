//! Vulkan SPIR-V backend — maps YOYO TIR operations to SPIR-V binary.
//!
//! Produces a `.spv` binary file containing a minimal valid SPIR-V module.
//! For Phase 1, this emits a stub SPIR-V module with a minimal compute shader.

use crate::tir::{instr_branch_kind, BranchKind, TirInst, TirOp};
use crate::types::IsaResult;

/// SPIR-V word (u32) helper
fn spv_word(v: u32) -> Vec<u8> {
    v.to_le_bytes().to_vec()
}

/// SPIR-V string helper (null-terminated, padded to 4 bytes)
fn spv_string(s: &str) -> Vec<u8> {
    let mut bytes = s.as_bytes().to_vec();
    bytes.push(0x00); // null terminator
    // Pad to multiple of 4
    while bytes.len() % 4 != 0 {
        bytes.push(0x00);
    }
    bytes
}

/// Emit SPIR-V binary from TIR instructions.
///
/// Returns valid SPIR-V binary bytes.
pub fn emit_spirv(tir: &[TirInst]) -> IsaResult<Vec<u8>> {
    let mut spv = Vec::new();

    // Count instructions and handlers for bound calculation
    let mut max_id: u32 = 0;
    let mut next_id: u32 = 1; // ID 1 is reserved for the entry point

    // SPIR-V Header (5 words)
    // Magic number
    spv.extend_from_slice(&spv_word(0x07230203));
    // Version: 1.0 = 0x00010000
    spv.extend_from_slice(&spv_word(0x00010000));
    // Generator: unknown
    spv.extend_from_slice(&spv_word(0x00000000));
    // Bound: maximum ID + 1 (will be patched at the end)
    let bound_pos = spv.len();
    spv.extend_from_slice(&spv_word(0x00000000));
    // Reserved (schema)
    spv.extend_from_slice(&spv_word(0x00000000));

    // Use a simple approach: emit a minimal valid SPIR-V module
    // with Capability, MemoryModel, EntryPoint, and stub functions.

    // Collect handler IDs for entry point declaration
    let mut handler_ids: Vec<u32> = Vec::new();
    for inst in tir {
        if let TirOp::Handler = inst.op {
            let hh = inst.args.first().copied().unwrap_or(0) as u16;
            let id = 1 + hh as u32;
            handler_ids.push(id);
            if id + 1 > max_id {
                max_id = id + 1;
            }
        }
    }

    // 1. Capability (Shader)
    // OpCapability: word count=2, opcode=17, capability=Shader(1)
    spv.extend_from_slice(&spv_word(0x00020011)); // word count=2 << 16 | opcode=17
    spv.extend_from_slice(&spv_word(1)); // Capability Shader

    // 2. MemoryModel
    // OpMemoryModel: word count=3, opcode=14
    // Addressing model = Logical(0), Memory model = GLSL450(1)
    spv.extend_from_slice(&spv_word(0x0003000E));
    spv.extend_from_slice(&spv_word(0)); // Logical
    spv.extend_from_slice(&spv_word(1)); // GLSL450

    // 3. EntryPoint
    // OpEntryPoint: word count=4 + num_handlers, opcode=15
    // Execution model = GLCompute(5)
    // Entry point = id of first handler
    let entry_id = if handler_ids.is_empty() { 1u32 } else { handler_ids[0] };
    let entry_point_word_count = 4u32 + handler_ids.len() as u32;
    spv.extend_from_slice(&spv_word((entry_point_word_count << 16) | 15));
    spv.extend_from_slice(&spv_word(5)); // GLCompute
    spv.extend_from_slice(&spv_word(entry_id)); // Entry point ID
    spv.extend_from_slice(&spv_string("main"));

    // Add interface IDs (none for now, but we need to list them)
    for &hid in &handler_ids {
        spv.extend_from_slice(&spv_word(hid));
    }

    // 4. Type declarations
    // OpTypeVoid: word count=2, opcode=19
    // Result ID = next_id++
    let void_type_id = next_id;
    next_id += 1;
    spv.extend_from_slice(&spv_word(0x00020013));
    spv.extend_from_slice(&spv_word(void_type_id));

    // OpTypeFunction: word count=3, opcode=33
    // Result ID, return type (void)
    let func_type_id = next_id;
    next_id += 1;
    spv.extend_from_slice(&spv_word(0x00030021));
    spv.extend_from_slice(&spv_word(func_type_id));
    spv.extend_from_slice(&spv_word(void_type_id));

    // 5. Stub functions for each handler
    for &hid in &handler_ids {
        // OpFunction: word count=5, opcode=54
        // Result type (void), Result ID, FunctionControl(0), FunctionType
        spv.extend_from_slice(&spv_word(0x00050036));
        spv.extend_from_slice(&spv_word(void_type_id));
        spv.extend_from_slice(&spv_word(hid));
        spv.extend_from_slice(&spv_word(0)); // FunctionControl: none
        spv.extend_from_slice(&spv_word(func_type_id));

        // OpLabel: word count=2, opcode=248
        let label_id = next_id;
        next_id += 1;
        spv.extend_from_slice(&spv_word(0x000200F8));
        spv.extend_from_slice(&spv_word(label_id));

        // OpReturn: word count=1, opcode=253
        spv.extend_from_slice(&spv_word(0x000100FD));

        // OpFunctionEnd: word count=1, opcode=56
        spv.extend_from_slice(&spv_word(0x00010038));
    }

    // If no handlers, emit a minimal function
    if handler_ids.is_empty() {
        let func_id = 1u32;
        // OpFunction
        spv.extend_from_slice(&spv_word(0x00050036));
        spv.extend_from_slice(&spv_word(void_type_id));
        spv.extend_from_slice(&spv_word(func_id));
        spv.extend_from_slice(&spv_word(0));
        spv.extend_from_slice(&spv_word(func_type_id));

        // OpLabel
        let label_id = next_id;
        next_id += 1;
        spv.extend_from_slice(&spv_word(0x000200F8));
        spv.extend_from_slice(&spv_word(label_id));

        // OpReturn
        spv.extend_from_slice(&spv_word(0x000100FD));

        // OpFunctionEnd
        spv.extend_from_slice(&spv_word(0x00010038));
    }

    // Patch bound
    let bound = next_id;
    let bound_bytes = bound.to_le_bytes();
    spv[8] = bound_bytes[0];
    spv[9] = bound_bytes[1];
    spv[10] = bound_bytes[2];
    spv[11] = bound_bytes[3];

    Ok(spv)
}