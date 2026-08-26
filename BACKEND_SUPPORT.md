# YOYO Backend Support Matrix

## Overview

**负责人看板**：[`STAGE4_OWNER_CHECKLIST.md`](../STAGE4_OWNER_CHECKLIST.md) — Stage 4 已毕业（A/B/C）；Stage 5 主线见看板「预置任务」。日常验收：`cargo run -- test ddc` 或 `cargo run -- test all`。

The YOYO verifier can cross-compile `.ty` programs to **37 target platforms** across 5 categories:
8-bit MCUs, 32-bit/64-bit CPUs, GPUs, blockchain VMs, and quantum computing.

## Quick Start

```bash
cd yoyo-rust/verifier
cargo run -- link --target=linux input.ty output.elf    # Linux x64
cargo run -- link --target=android input.ty output.elf   # Android ARM64
cargo run -- link --target=wasm input.ty output.wasm     # WebAssembly
cargo run -- link --target=8051 input.ty output.bin      # Intel 8051
cargo run -- link --target=evm input.ty output.evm       # Ethereum
cargo run -- link --target=qiskit input.ty output.qasm   # IBM Quantum
cargo run -- link --target=custom-mcu input.ty output.bin  # Custom MCU scaffold
```

## Full Backend Matrix

| Target | --target= | Arch | Format | Endian | Bits | Real Asm | Interp |
|--------|-----------|------|--------|--------|------|----------|--------|
| Win32 | win32 | x64 | PE32+ | LE | 64 | ✅ (x64) | ❌ |
| Linux | linux | x64 | ELF64 | LE | 64 | ✅ (x64) | ❌ |
| FreeBSD | freebsd | x64 | ELF64 | LE | 64 | ✅ (x64) | ❌ |
| Haiku | haiku | x64 | ELF64 | LE | 64 | ✅ (x64) | ❌ |
| SerenityOS | serenity | x64 | SERE | LE | 64 | ✅ (x64) | ❌ |
| Plan9 | plan9 | x64 | Flat | LE | 64 | ✅ (x64) | ❌ |
| Maze | maze | x64 | Maze | LE | 64 | ✅ (x64) | ❌ |
| Android | android | ARM64 | ELF64 | LE | 64 | ✅ | ✅ |
| Apple | apple | ARM64 | Mach-O64 | LE | 64 | ✅ | ✅ |
| ARM64 Windows | arm64-win | ARM64 | PE32+ | LE | 64 | ✅ | ✅ |
| ARM32 | arm32 | ARM32 | ELF32 | LE | 32 | ✅ | ❌ |
| RISC-V RV64 | riscv64 | RV64 | ELF64 | LE | 64 | ✅ | ✅ |
| RISC-V RV32 | riscv32 | RV32 | ELF32 | LE | 32 | ✅ | ✅ |
| MIPS | mips | MIPS32 | ELF32BE | BE | 32 | ✅ | ⏳ |
| PPC64 LE | ppc64le | PPC64 | ELF64 | LE | 64 | ✅ | ⏳ |
| LoongArch | loongarch | LA64 | ELF64 | LE | 64 | ✅ | ✅ |
| SPARC v8 | sparc | SPARCv8 | ELF32BE | BE | 32 | ✅ | ✅ |
| x86-32 | x86 | x86-32 | PE32 | LE | 32 | ✅ (stub) | ❌ |
| FreeDOS | freedos | x86-16 | COM | LE | 16 | ✅ (stub) | ❌ |
| BareMetal | baremetal | x64 | Flat | LE | 64 | ✅ (stub) | ❌ |
| Stub | stub | x64 | Flat | LE | 64 | ✅ (stub) | ❌ |
| 8051 | 8051 | 8051 | Flat | LE | 8 | ✅ | ❌ |
| Z80 | z80 | Z80 | Flat | LE | 8 | ✅ | ❌ |
| 6502 | 6502 | 6502 | Flat | LE | 8 | ✅ | ❌ |
| AVR | avr | AVR | Flat | LE | 8 | ✅ | ❌ |
| Xtensa | xtensa | Xtensa | Flat | LE | 32 | ❌ | ❌ |
| MSP430 | msp430 | MSP430 | Flat | LE | 16 | ❌ | ❌ |
| PIC | pic | PIC16 | Flat | LE | 8 | ❌ | ❌ |
| STM8 | stm8 | STM8 | Flat | LE | 8 | ❌ | ❌ |
| M68k | m68k | M68k | Flat | BE | 32 | ❌ | ❌ |
| CUDA | cuda | PTX | Text | LE | 64 | ✅ (text) | ❌ |
| ROCm | rocm | HIP | Text | LE | 64 | ✅ (text) | ❌ |
| Vulkan | vulkan | SPIR-V | Binary | LE | 32 | ✅ (binary) | ❌ |
| WebAssembly | wasm | Wasm | Wasm | LE | 32 | ✅ | ✅ |
| EVM | evm | EVM | Flat | BE | 256 | ✅ | ❌ |
| Qiskit | qiskit | OpenQASM | Text | LE | 0 | ✅ (text) | ❌ |
| Custom MCU | custom-mcu | Custom | Flat | LE | 8 | ✅ (scaffold) | ✅ |

Legend: ✅ = done, ⏳ = in progress, ❌ = not yet

## DDC Verification

Run from `yoyo-rust/verifier`: `cargo run -- test ddc`

| Fixture | Semantics | Fatal (core) | Soft / non-fatal | Status |
|---------|-----------|--------------|------------------|--------|
| `00_nop_ret.ty` | NOP+RET | sim + 23 arch interps (incl. wasm trap + **custom_mcu** scaffold) | — | PASS |
| `01_arith.ty` | SET+ADDV → slot0=8 | sim + arm64/rv64/rv32/mips/ppc/arm32/sparc/loong/x86/plan9/win32/linux + **11 MCU fatal** (8051/avr/z80/6502/m68k/msp430/freedos/xtensa/pic/stm8/evm) | — | PASS |
| `02_branch.ty` | CMP+JE → slot0=5 | sim + arm64/rv64/rv32/mips/ppc/arm32/sparc/loong/plan9/x86/win32/linux + **11 MCU fatal** | — | PASS |
| `03_mem.ty` | MEMCPY_STATE → slot0=7 | sim + arm64/rv64/rv32/mips/ppc/arm32/sparc/loong/plan9/x86/win32/linux + **11 MCU fatal** | — | PASS |
| `04_ldb_ptr.ty` | LDB pointer-form → slot0=7 | sim + Win32 PE + Linux ELF container | — | PASS |
| container | PE/ELF container NOP+RET | PE32+ x64 + ELF64 x64 via plan9_interp | — | PASS |
| `custom_mcu` scaffold | NOP+RET smoke (`--target=custom-mcu`) | flat binary + `custom_mcu_interp` | — | PASS |

Known gaps: **none** for Stage 4 DDC graduation fixtures (00–04 + container all PASS). `custom-mcu` is a **copy-and-replace scaffold** for chip-specific work — extend emit + interp before promoting to full MCU DDC fatal.

### How to run

```
yoyo test golden|backends|ddc|all|gen12
```

- `yoyo test golden` — Appendix F G00-G05 integrity tests (739/739)
- `yoyo test backends` — compile+link all 37 targets, verify output
- `yoyo test ddc` — nop + arith + branch + mem + ldb + container DDC suites
- `yoyo test all` — golden + backends + ddc (CI-level one-shot)
- `yoyo test gen12` — gen1≡gen2 SHA monitor (`4fb8b87f`)

## CLI Usage

```
yoyo link [--target=<target>] <input.ty> <output>
yoyo simulate <input.ty>
yoyo run-wasm <input.ty>
yoyo exec <input.ty> [--target=android|apple]
yoyo ddcmp <A.elf> <B.elf> <input.ty>
yoyo test golden|backends|ddc|all|gen12
yoyo info [--target=<target>]
yoyo diff <a.bin> <b.bin>
yoyo hash <file>
yoyo render <input.ty>
```

## Architecture

```
.ty source → ty_parser → TIR (intermediate representation)
                                  ↓
                     emit.rs (per-arch dispatch)
                                  ↓
              platform backends (37 targets)
                                  ↓
                    linker (ELF/PE/Mach-O/Wasm)
                                  ↓
                    binary output + DDC validation
```

## Adding a New Backend

1. Add variant to `PlatformKind` enum in `platform.rs`
2. Add variant to `BinaryFormat` if needed
3. Implement `PlatformBackend` trait
4. Add dispatch arm in `main.rs` cmd_link()
5. Add to `yoyo info` list
6. Add to `yoyo test backends` iteration
7. Add to this document

## Adding a Custom MCU Backend

Use the built-in **`custom-mcu`** scaffold when bringing up a new chip or proprietary ISA. It already links NOP+RET, passes `yoyo test backends`, and has a smoke DDC path via `custom_mcu_interp`.

### Hook steps (copy & replace)

1. **Fork the scaffold** — duplicate `CustomMcuPlatform` in `platform.rs` (or rename variant to your chip, e.g. `MyChip`).
2. **Define opcode bytes** — override `emit_nop`, `emit_ret`, then `emit_set` / `emit_get` / branches as needed (see `Stm8Platform` or `AvrPlatform` for full MCU examples).
3. **Add interpreter** — copy `custom_mcu_interp.rs` → `mychip_interp.rs`; decode loop must match your emit encodings for DDC.
4. **Wire dispatch** — `select_platform`, `parse_platform` (`--target=mychip`), `cmd_link` (flat `.bin`), `emit_target_bytes`, `cmd_test_backends`, `cmd_info`.
5. **DDC promotion** — start with `00_nop_ret` smoke (`ddc_custom_mcu` pattern); add `01_arith`/`02_branch`/`03_mem` helpers when interpreter reads state slots.
6. **Document** — add a row to the matrix above and update the DDC table in this file.

### Scaffold encoding (default)

| Op | Byte | Notes |
|----|------|-------|
| NOP | `0x00` | 1-byte placeholder |
| RET | `0xC3` | must match `custom_mcu_interp` |

```bash
cd yoyo-rust/verifier
cargo run -- link --target=custom-mcu ../../yoyo/tests/golden/00_nop_ret.ty /tmp/out.bin
cargo run -- test backends   # expect custommcu PASS
```

## DDC / Cross-Architecture Validation

The key insight: all backends compile the same TIR. The TIR simulator is the ground truth.
Two backends compiling the same TIR are DDC-equivalent by construction.

For end-to-end proof, the ARM64 interpreter and Wasm runner execute the real machine code
and compare against the TIR simulator, confirming semantic equivalence at the binary level.