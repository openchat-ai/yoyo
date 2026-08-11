# YOYO — Multi-Architecture Backend Instructions

## Goal
Add platform backends to the YOYO verifier. Each backend must:
1. Register a `PlatformKind` variant and `BinaryFormat` (if needed)
2. Implement `PlatformBackend` trait (stub emit is fine for Phase 1)
3. If the architecture produces a file format that needs a special linker wrapper, provide it
4. Wire `cmd_link` in `main.rs` to dispatch to the correct path
5. Pass `cargo test` without regressions (all 74 existing tests + any new ones)
6. Produce real binary output when run on a `.ty` fixture

## Codebase structure (Rust)
- `yoyo-rust/verifier/src/platform.rs` — `PlatformKind` enum, `BinaryFormat` enum, `TemplateInfo`, `PlatformBackend` trait, `select_platform()`, `parse_platform()`. All backends are structs implementing `PlatformBackend` here. **Edit this file for every new backend.**
- `yoyo-rust/verifier/src/main.rs` — CLI entry. Add `mod xxx` at the top; add a `PlatformKind::Xxx => { ... }` arm in `cmd_link`. Update usage() text. Also add `mod xxx_link` for linker modules.
- `yoyo-rust/verifier/src/emit.rs` — **DO NOT EDIT.** Emits TIR → x64 bytes. The stub backends emit architecture-native placeholder bytes (NOP + marker) which is sufficient for this phase.
- `yoyo-rust/verifier/src/assembler.rs` — **DO NOT EDIT.** x64 bytecode assembler.
- `yoyo-rust/verifier/src/elf_link.rs` — x64 ELF64 linker. Do not edit.
- `yoyo-rust/verifier/src/arm64_elf_link.rs` — ARM64 ELF64 linker. Do not edit.
- `yoyo-rust/verifier/src/pe_link.rs` — x64 PE32+ linker. Do not edit.
- `yoyo-rust/verifier/src/apple_backend.rs` — ARM64 Mach-O64 linker. Do not edit.
- `yoyo-rust/verifier/src/cuda_backend.rs` — CUDA PTX emitter. Do not edit.
- `yoyo-rust/verifier/src/types.rs` — `IsaError`, `IsaResult`, `Reg`, etc. Do not edit.

## Backend pattern (from platform.rs)

Each backend is a struct:

```rust
pub struct XxxPlatform;

impl XxxPlatform {
    pub fn new() -> Self { Self }
}

impl PlatformBackend for XxxPlatform {
    fn emit_alloc(&mut self, slot: u16, size: u64) -> IsaResult<Vec<u8>> {
        // stub: NOP + marker bytes (architecture-native)
    }
    fn emit_load_file(&mut self, slot: u16, str_idx: u8) -> IsaResult<Vec<u8>> { ... }
    fn emit_write_file(&mut self, slot: u16, str_idx: u8, sz_slot: u16) -> IsaResult<Vec<u8>> { ... }
    fn emit_exit(&mut self, code: u8) -> IsaResult<Vec<u8>> {
        // arch-native exit/halt instruction
    }
    fn startup_blob(&self) -> &[u8] { &[] }
    fn template(&self) -> TemplateInfo { ... }
}
```

### Stub emit pattern
For I/O ops (alloc/load_file/write_file) emit: `architecture-NOP` + some value bytes. This is DDC-matched stub (same as Win32/Linux/8051 stubs).

**NOP per arch (big-endian encoding, convert to le for le arches):**
- x64: `0x90` (1 byte)
- x86 32-bit: `0x90` (1 byte)
- ARM64: `0xD503201F` (4 bytes, le)
- ARM32: `0xE1A00000` (4 bytes, le)
- MIPS: `0x00000000` (4 bytes)
- RISC-V: `0x00000013` (4 bytes, le = `13 00 00 00`)
- PPC64LE: `0x60000000` → LE = `00 00 00 60`
- AVR: `0x0000` (2 bytes)
- 8051: `0x00` (1 byte)
- Wasm: `0x00` (1 byte)
- Real-mode x86: `0x90`

### `template()` for FlatBinary targets
Use `BinaryFormat::FlatBinary`, `entry_point: 0`, reasonable stack/data offsets for the target.

## Linker dispatch (cmd_link in main.rs)

For each new `PlatformKind`, add a match arm:

```rust
PlatformKind::Xxx => {
    let out = emit::emit(&tir, platform)?;
    // Option A: flat binary, no linker
    fs::write(&rest[1], &out.code).map_err(...)
    println!("wrote Xxx flat ...");
}
```

```rust
PlatformKind::Xxx => {
    let out = emit::emit(&tir, platform)?;
    let img = xxx_link::link_xxx(&out.code, &out.data)?;
    fs::write(&rest[1], &img.bytes).map_err(...)
    println!("wrote Xxx ...");
}
```

For **text output** (like CUDA PTX):
```rust
PlatformKind::Xxx => {
    let ptx = xxx_backend::emit_xxx(&tir)?;
    fs::write(&rest[1], ptx).map_err(...)
    println!("wrote Xxx ...");
}
```

## File format conventions

### ELF (Linux)
- Magic: `7F 45 4C 46`, class64, data2LSB (unless big-endian arch), current, version 1
- e_type = 2 (ET_EXEC)
- e_phdr: 2 PT_LOAD segments (.text, .data)
- Stack alignment 16B
- **MIPS big-endian**: data2MSB (1 → 2), e_machine = 0x08 (EM_MIPS), `e_ident[5]=2`
- **RISC-V RV64**: e_machine = 0xF3 (EM_RISCV), data2LSB
- **PPC64LE**: e_machine = 0x142 (EM_PPC64), data2LSB (LSB), version 1
- **ARM32**: e_machine = 0x28 (EM_ARM), data2LSB

### PE (Windows)
- **PE32 (32-bit x86)**: `0x14C` in optional header magic, machine = 0x014C, EntryPoint and addresses all u32, section VA/RVA all u32, ImageBase = 0x00400000

### Mach-O
- Magic: `0xFEFAFEDD` (MH_MAGIC_64_LE, LE form) or `0xFEEDFACF` (BE form for PPC) — use LE for x64/ARM64, BE for PPC
- Use `apple_backend.rs` as a template for PPC Mach-O; for x64 Mach-O the adrp startup can use x64 REX prefix

### Wasm (WebAssembly)
- Magic: `00 61 73 6D` (null + "asm"), version `01 00 00 00`
- Module sections: Type section, Function section, Code section, Data section
- Functions return i32; use `local.get`/`local.set` for state slots
- Exit via `unreachable` or custom trap

### Real-mode x86 (FreeDOS)
- COM file: no header, code starts at 0x0000:0x0100
- Exit: `INT 0x20` (terminate) or `INT 0x21 / AH=0x4C` (DOS exit)
- Flat binary at offset 0x100 (DOS loads COM at CS=IP=0x0100)

### AVR
- Flat binary, addresses 16-bit
- Exit: `JMP .` infinite loop or `SLEEP`
- 16-bit registers r0-r31

### SerenityOS
- ELF64 with custom program headers / magic — **use FlatBinary format with Serenity-specific loader magic as header**, OR just a minimal ELF with e_machine = 0x100000 (EM_SOMETHING). For Phase 1: flat binary with Serenity loader header at top (e.g. 4-byte magic `SERE`)

## Key rules
1. **Do NOT modify emit.rs, assembler.rs, elf_link.rs, pe_link.rs, arm64_elf_link.rs, apple_backend.rs, cuda_backend.rs, types.rs, tir.rs.** Only platform.rs, main.rs, and new linker files.
2. After making changes, run: `cargo build` then `cargo test`. Fix all errors. All 74 existing tests must still pass.
3. After tests pass, smoke-test link with: `cargo run -- link --target=xxx "f:\yoyo\yoyo\tests\golden\00_nop_ret.ty" "f:\yoyo\yoyo-rust\build\test_xxx.ext"` — must print "wrote" line.
4. PowerPC big-endian: all multi-byte constants written as `to_be_bytes()` instead of `to_le_bytes()`.
5. Use `to_le_bytes()` everywhere except big-endian targets.

## Build/test workflow
```powershell
cd f:\yoyo\yoyo-rust\verifier
cargo build
cargo test
cargo run -- link --target=<target> "f:\yoyo\yoyo\tests\golden\00_nop_ret.ty" "f:\yoyo\yoyo-rust\build\test_<target>.ext"
```

## Testing
Each new backend should have 2-3 unit tests in its new file:
- File magic/header bytes are correct
- File is non-empty and size makes sense
- Exit/halt bytes are present

## Sub-agent assignment

### Sub-agent A (handle in this order):
1. **x86-32-bit (PE32)** — `PlatformKind::X86`, `BinaryFormat::Pe32` variant. Add `x86_link.rs` with PE32 writer (32-bit magic, u32 addresses, ImageBase 0x400000). Exit: x86 32-bit `mov eax,0x30; int 0x80` (Linux x86 syscall) — actually Windows exit via `exitprocess` IAT. Use stub NOP like others. Template: entry 0x1000, data 0x2000.
   - **Wait**: x86 32-bit Windows. Exit stub: `mov eax, <code>; int 0x2E` (old DOS interrupt) — no. Use: `ret` = `0xC3` as stub. Or proper Windows: just NOP markers. Use `0xC3` ret as exit stub.
   - PE32 linker: same as pe_link.rs but with 32-bit magic (0x10B), machine 0x014C, ImageBase 0x00400000.
   
2. **FreeDOS** — `PlatformKind::Freedos`, `BinaryFormat::DosCom`. Flat binary, COM-style. Exit: `INT 21h / AH=4Ch` = `0xCD 21` (but DOS syscall number in AH, so: `0xB4 4C` (mov ah,4c) `0xCD 21` (int 21h) — no exit code passed. Or just `0xCD 20` (terminate program). Use `0xCD 20` for stub. Template: FlatBinary, entry 0x100.
   - No linker needed; just write flat code blob.

### Sub-agent B (handle in this order):
3. **RISC-V RV64 Linux ELF** — `PlatformKind::Riscv64`, elf64. e_machine = 0xF3 (EM_RISCV). data2LSB. Entry 0x1001000. Exit: `ecall` (0x00000073, LE `73 00 00 00`). Stub NOP: `0x00000013` (LE `13 00 00 00`). Add `riscv_elf_link.rs` (based on elf_link.rs, e_machine=0xF3, entry 0x1001000, startup: `auipc/a` to set x15→data).
4. **MIPS big-endian** — `PlatformKind::Mips`, `BinaryFormat::Elf32BE`. e_machine=0x08, data2MSB (`e_ident[5]=2`). All writes use `to_be_bytes()`. Entry 0x4001000. Exit: `syscall` (0x0000000C). Stub NOP: `0x00000000`. Add `mips_elf_link.rs`. Stack align 8B.
5. **PowerPC64 LE (Linux ELF)** — `PlatformKind::PowerPc64Le`, elf64. e_machine=0x142 (EM_PPC64), data2LSB. Entry 0x10000000. Exit: `sc` (0x44000002, LE `02 00 00 44`). Stub NOP: `0x60000000` (LE `00 00 00 60`). Add `ppc_elf_link.rs`.
6. **AVR** — `PlatformKind::Avr`, FlatBinary. Exit: `SLEEP` (0x9588) or infinite `JMP .` (0xCCCF → LE `CF CC`). Stub NOP: `0x0000`. Template: FlatBinary, entry 0. No linker.

### Sub-agent C (handle in this order):
7. **ARM32 (Android EABI)** — `PlatformKind::Arm32`, elf32. e_machine=0x28 (EM_ARM), data2LSB. Entry 0x8001000. Exit: `swi #0` (0xEF000000, LE `00 00 00 EF`). Stub NOP: `0xE1A00000` (LE `00 00 A0 E1`). Add `arm32_elf_link.rs` (32-bit ELF, e_ident[4]=1 ELFCLASS32).
8. **WebAssembly** — `PlatformKind::Wasm`, `BinaryFormat::Wasm`. Text output (like CUDA) producing `.wasm` bytes. Magic `00 61 73 6D 01 00 00 00`. Module with type/function/code/data sections. State slots as local variables. Exit: `unreachable` (0x00). Add `wasm_backend.rs`.
9. **Mach-O x64 (Apple Intel macOS)** — `PlatformKind::MachoX64`, `BinaryFormat::MachO64`. Same as apple_backend.rs but x64. Magic LE (0xFEFAFED0), CPU_TYPE_X86_64 (0x01000007), entry 0x100001000. Startup: x64 `lea r15,[rip+disp]; jmp` like Linux x64. Exit: `mov eax,0x20000003; mov edi,code; syscall`. Add `macho_x64_link.rs`.
10. **SerenityOS** — `PlatformKind::Serenity`, `BinaryFormat::Serenity`. Flat binary with `SERE` magic (4B) at start, followed by code. Entry: after magic. Stub exit: `MOV eax,0; SYSCALL` (Linux-style). Or just `hlt` (0xF4). No real linker, just prepend magic.

## Sub-agent protocol
- Each sub-agent: edit platform.rs, add new linker file(s), edit main.rs
- After all edits: `cargo build` → fix errors → `cargo test` → must pass
- Then: `cargo run -- link --target=<xxx> ...` smoke test
- Report: which files changed/created, build status, test results, smoke test output, any decisions/issues
- **All 74 existing tests must still pass. If a sub-agent breaks existing tests, fix them.**
