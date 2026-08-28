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

### Platform I/O (Stage 8-A · D-1 closed for production)

| Backend | `0x20` ALLOC | `0x50` LOAD_FILE | `0x51` WRITE_FILE | Notes |
|---------|--------------|------------------|-------------------|-------|
| **Win32** | VirtualAlloc via kernel32 IAT `[r15+0]` | CreateFileA + ReadFile + CloseHandle | CreateFileA + WriteFile + CloseHandle | `pe_link` prepends IAT blob; emit in `platform_io.rs` |
| **Linux** | mmap syscall | open/read/close | open/write/close | inline x64 syscalls in `platform_io.rs` |
| **Stub** | movabs+store | movabs+store | movabs+store | G-SM-IO / golden; JS default encode stays stub |
| **Plan9 / FreeBSD / Haiku / Serenity** | movabs+store | movabs+store | movabs+store | x64 hosted peers; real OS I/O deferred |

**D-1 (2026-08-27 / Stage 9-B 2026-08-28 / Stage 10-C 2026-08-28):** Rust **Win32 + Linux** production link paths no longer use movabs+store placeholders for I/O opcodes. **Stub** remains deterministic for golden fixtures (G-SM-IO). **JS peer (Stage 9-B):** production `yoyo-js` PE path uses `setEmitPlatform('win32')` + `platform-io.js` matching Rust `platform_io.rs` byte-for-byte; golden default stays `stub`. Gate: `scripts/stage9-js-peer-io.ps1` + golden `PEER-IO-WIN32` / `PEER-IO-LINUX` / `PEER-IO-STUB`. **Asm peer (Stage 10-C):** `yoyo-asm/platform_io.py` + `asm.py` `--target=win32|linux|stub` — win32 `0x20/0x50/0x51` byte-equal Rust; linux ALLOC syscall path peer-checked. Gate: `scripts/stage10-asm-peer-io.ps1`.

**Trust-chain observability:** gen12 `.text` SHA as of Stage 10-A (H_00 stub carries `dll_embed_size`): **`43ffde58…`** · **18432** bytes (was `b609a735…` at Stage 9 / v0.3 when embedded DLL was 485888B). Self-host bootstrap (gen1≡gen2) still EQUAL; I/O syscall/IAT + H_00 extract stub bytes are inside the compared `.text` window. **Stage 9-B / 10-C:** JS↔Rust↔asm win32 I/O handler bodies EQUAL on minimal fixtures; stub remains for G-SM-IO.

### Stage 10-A — embedded `yoyo_runtime.dll` surface shrink (fail-closed)

| Item | Detail |
|------|--------|
| **Before (v0.3)** | Embedded DLL **485888** B · genN PE **576512** B · gen12 SHA `b609a735…` |
| **After** | Embedded DLL **231936** B (−253952) · genN PE **322560** B · gen12 SHA `43ffde58…` (same **18432** B window) |
| **How** | `yoyo-runtime` depends on `verifier` with `default-features = false` (no `full-backends` / no wasmtime); `select_platform` only Win32/Linux/Stub; `profile.release.package.yoyo-runtime` `opt-level=z` |
| **Gate** | `scripts/stage10-runtime-surface.ps1` — fail-closed MAX **250000** B; exact embed match in gen1; forbids wasmtime/cranelift markers |
| **Still host-trusted** | DLL still Rust-compiled (Win32 compile path); bytes **outside** gen12 remain the bulk of the hole; not a YOYO-built runtime yet |

### Stage 9-B — JS peer platform I/O (DDC blind zone closed)

| Item | Detail |
|------|--------|
| **JS production** | `yoyo-js/src/yoyo.js` → `setEmitPlatform('win32')`; emit in `platform-io.js` |
| **JS golden / G-SM-IO** | Default `stub` movabs+store (unchanged pin) |
| **Comparable scope** | ALLOC/LOAD_FILE/WRITE_FILE handler bytes JS==Rust win32; linux syscall path peer-checked |
| **Gate** | `node yoyo-js/scripts/golden.js` + `scripts/stage9-js-peer-io.ps1` |
| **Still divergent** | full `yoyo.ty` section-ddc may still DIFF (H_00 runtime / IAT width); asm peer closed in Stage 10-C |

### Stage 10-C — Python asm peer platform I/O (三链 I/O 可比对)

| Item | Detail |
|------|--------|
| **Asm production** | `yoyo-asm/asm.py` `--target=win32|linux`; emit in `platform_io.py` |
| **Asm golden / G-SM-IO** | Default / `--target=stub` movabs+store (unchanged contract) |
| **Comparable scope** | win32 ALLOC/LOAD/WRITE handler bytes asm==Rust (==JS); linux ALLOC not stub + `0F 05` |
| **Gate** | `scripts/stage10-asm-peer-io.ps1` (embeds stage9-js-peer-io) |
| **Still divergent** | full `yoyo.ty` section-ddc may DIFF (H_00 / IAT width / embedded runtime) |

### Full body compiler (Stage 8-B · W5.5 body)

| Path | What it validates | DDC scope |
|------|-------------------|-----------|
| **`test gen12`** | `.ty` link ≡ `.tyb` bootstrap (788 handlers) | `.text` window (H_00 path may enlarge vs v0.2 17920) |
| **`test fullbody`** | Handler count ≥700 (fail-closed vs W-SM scoped=34); `.ty`/`.tyb` bootstrap; gen2rt runtime → `output.exe` | Same gen12 `.text` window + runtime gen3 parity |
| **`stage5-win-selfhost.ps1`** | M1→M2 bootstrap + **gen1 H_00 runtime** + M2→M3 embedded startup | gen1≡gen2 EQUAL; gen1 zero-arg → `output.exe`; gen3 from full-body compile |
| **`stage9-gen1-h00-selfhost.ps1`** | Stage 9-A: gen1 H_00 pure path only | PE entry → H_00 (no genNrt wrapper); exit 0 + `output.exe` |
| **`stage9-pure-m4.ps1`** | Stage 9-C: Win M4 via H_00 chain (no `--selfhost`) | gen1→gen2→gen3→gen4; gen4≡gen3_direct `.text` DDC EQUAL |
| **`stage10-linux-pure-m4.sh`** | Stage 10-B: Linux M4 via ELF H_00 (no `--selfhost`) | gen1→gen2→gen3→gen4; gen4≡gen3_direct full-ELF DDC EQUAL |
| **`stage8-extended-selfhost.ps1/.sh`** | M2→M3→M4 embedded chain (Stage 8-C; regression) | gen4≡gen3_direct `.text` DDC EQUAL; same gen12 window as fullbody |

**DDC coverage boundary (honest):** gen12/fullbody monitor the **788-handler emit body** (handler `.text`), including Stage 9-A H_00 patch + appended extract stub when present (stub embeds `dll_embed_size`, so Stage 10-A DLL shrink moves gen12 SHA). Embedded Rust `yoyo_runtime.dll` payload bytes remain outside that compared window (Stage 10-A: **231936** B, was 485888). Golden G-SM fixtures still test W-selfhost-min opcode shapes; fullbody gate ensures production compile uses the complete body, not scoped H_05–H_16 subset only.

### Stage 9-A — gen1 H_00 pure runtime (trust hole closed)

| Item | Detail |
|------|--------|
| **Path** | `yoyo link` → `gen1.exe`; PE entry `lea r15; jmp H_00`; H_00 patched to `jmp` extract+`LoadLibrary`+`yoyo_runtime_selfhost_main`+`ExitProcess` |
| **Not used** | genNrt-style entry that *replaces* user entry with a separate startup blob (gen2rt still uses that for Stage 8-C regression) |
| **I/O** | Same Stage 8-A merged kernel32 IAT at `r15+0` (CreateFile/WriteFile/LoadLibrary/ExitProcess) for in-process DLL extract |
| **Gate** | `scripts/stage9-gen1-h00-selfhost.ps1` + `stage5-win-selfhost.ps1` gen1 line GREEN |
| **Still host-trusted** | Embedded `yoyo_runtime.dll` compile (Rust; Stage 10-A shrunk surface — see above); see Stage 9-C for M4 without genNrt wrapper |

### Stage 9-C — Win pure M4 (host `--selfhost` wrapper shrunk)

| Item | Detail |
|------|--------|
| **Path** | `yoyo link` → gen1; run gen1→gen2→gen3→gen4 (each zero-arg H_00); reference = `bootstrap` **without** `--selfhost` |
| **Not used** | `bootstrap --selfhost` / genNrt entry wrapper for the M4 gate (`stage9-pure-m4.ps1`) |
| **Parity** | gen4 ≡ gen3_direct `.text` section-ddc EQUAL (same window as stage8/fullbody) |
| **Gate** | `scripts/stage9-pure-m4.ps1`; stage8 extended selfhost remains GREEN (regression; still documents genNrt path) |
| **Still host-trusted** | Seed `link` + gen3_direct `bootstrap`; embedded `yoyo_runtime.dll` bytes (Stage 10-A: 231936B fail-closed); Linux pure M4 = Stage 10-B |

### Stage 10-B — Linux ELF H_00 / pure M4 (host `--selfhost` closed)

| Item | Detail |
|------|--------|
| **Path** | `yoyo link --target=linux` → gen1; ELF entry `lea r15; jmp H_00`; H_00 extracts embedded `libyoyo_runtime.so` + `linux_h00_tramp.elf` via syscalls then `execve` trampoline |
| **Not used** | `bootstrap --selfhost` / genNrt gcc loader for the M4 gate (`stage10-linux-pure-m4.sh`) |
| **Parity** | gen4 ≡ gen3_direct full-ELF DDC EQUAL (sha prefix `085d07d4…` · **704512** B as of 2026-08-28; H_00 write-loop) |
| **Gate** | `scripts/stage10-linux-pure-m4.sh`; stage8-extended-selfhost.sh remains GREEN (regression; still documents genNrt `--selfhost`) |
| **Still host-trusted** | Seed `link` + gen3_direct `bootstrap`; embedded Rust `.so` + committed trampoline blob (dlopen/libdl); trampoline still uses system libc |

### How to run

```
yoyo test golden|backends|ddc|all|gen12|fullbody
```

- `yoyo test golden` — Appendix F G00-G05 integrity tests (739/739)
- `yoyo test backends` — compile+link all 37 targets, verify output
- `yoyo test ddc` — nop + arith + branch + mem + ldb + container DDC suites
- `yoyo test all` — golden + backends + ddc + gen12 + fullbody (CI-level one-shot)
- `yoyo test gen12` — gen1≡gen2 SHA monitor (`43ffde58`, Stage 10-A H_00 window 18432B; was `b609a735` at v0.3)
- `yoyo test fullbody` — full 788-handler body compile + runtime smoke (Stage 8-B)

## CLI Usage

```
yoyo link [--target=<target>] <input.ty> <output>
yoyo simulate <input.ty>
yoyo run-wasm <input.ty>
yoyo exec <input.ty> [--target=android|apple]
yoyo ddcmp <A.elf> <B.elf> <input.ty>
yoyo test golden|backends|ddc|all|gen12|fullbody
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