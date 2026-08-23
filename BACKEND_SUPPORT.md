# YOYO Backend Support Matrix

## Overview

**负责人看板**：[`STAGE4_OWNER_CHECKLIST.md`](../STAGE4_OWNER_CHECKLIST.md)（每日 `cargo run -- test ddc` + Stage 4 毕业勾选）。

The YOYO verifier can cross-compile `.ty` programs to **36 target platforms** across 5 categories:
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
| RISC-V RV64 | riscv64 | RV64 | ELF64 | LE | 64 | ✅ | ⏳ |
| RISC-V RV32 | riscv32 | RV32 | ELF32 | LE | 32 | ✅ | ⏳ |
| MIPS | mips | MIPS32 | ELF32BE | BE | 32 | ✅ | ⏳ |
| PPC64 LE | ppc64le | PPC64 | ELF64 | LE | 64 | ✅ | ⏳ |
| LoongArch | loongarch | LA64 | ELF64 | LE | 64 | ⏳ | ❌ |
| SPARC v8 | sparc | SPARCv8 | ELF32BE | BE | 32 | ⏳ | ❌ |
| x86-32 | x86 | x86-32 | PE32 | LE | 32 | ✅ (stub) | ❌ |
| FreeDOS | freedos | x86-16 | COM | LE | 16 | ✅ (stub) | ❌ |
| BareMetal | baremetal | x64 | Flat | LE | 64 | ✅ (stub) | ❌ |
| Stub | stub | x64 | Flat | LE | 64 | ✅ (stub) | ❌ |
| 8051 | 8051 | 8051 | Flat | LE | 8 | ✅ | ❌ |
| Z80 | z80 | Z80 | Flat | LE | 8 | ⏳ | ❌ |
| 6502 | 6502 | 6502 | Flat | LE | 8 | ⏳ | ❌ |
| AVR | avr | AVR | Flat | LE | 8 | ⏳ | ❌ |
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

Legend: ✅ = done, ⏳ = in progress, ❌ = not yet

## DDC Verification

Run from `yoyo-rust/verifier`: `cargo run -- test ddc`

| Fixture | Semantics | Fatal (core) | Soft / non-fatal | Status |
|---------|-----------|--------------|------------------|--------|
| `00_nop_ret.ty` | NOP+RET | sim + 22 arch interps (incl. wasm trap) | — | PASS |
| `01_arith.ty` | SET+ADDV → slot0=8 | sim + arm64/rv64/rv32/mips/ppc/arm32/sparc/loong/x86/plan9/win32/linux | MCU+EVM (8051/avr/z80/6502/m68k/msp430/freedos/xtensa/pic/stm8/evm) | CORE fatal |
| `02_branch.ty` | CMP+JE → slot0=5 | sim + arm64/rv64/rv32/mips/ppc/arm32/sparc/loong/plan9/x86/win32/linux | other MCU | CORE fatal |
| `03_mem.ty` | MEMCPY_STATE → slot0=7 | sim + arm64/rv64/rv32/mips/ppc/arm32/sparc/loong/plan9/x86/win32/linux | MCU | CORE fatal |
| container | PE/ELF container NOP+RET | PE32+ x64 + ELF64 x64 via plan9_interp | — | PASS |

Known gaps: LDB absolute-pointer DDC not yet wired.

### How to run

```
yoyo test golden|backends|ddc|all
```

- `yoyo test golden` — Appendix F G00-G05 integrity tests
- `yoyo test backends` — compile+link all targets, verify output
- `yoyo test ddc` — nop + arith + branch + mem DDC suites

## CLI Usage

```
yoyo link [--target=<target>] <input.ty> <output>
yoyo simulate <input.ty>
yoyo run-wasm <input.ty>
yoyo exec <input.ty> [--target=android|apple]
yoyo ddcmp <A.elf> <B.elf> <input.ty>
yoyo test golden|backends|ddc|all
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
              platform backends (36 targets)
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

## DDC / Cross-Architecture Validation

The key insight: all backends compile the same TIR. The TIR simulator is the ground truth.
Two backends compiling the same TIR are DDC-equivalent by construction.

For end-to-end proof, the ARM64 interpreter and Wasm runner execute the real machine code
and compare against the TIR simulator, confirming semantic equivalence at the binary level.