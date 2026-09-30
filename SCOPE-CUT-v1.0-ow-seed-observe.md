# OW-SEED observe — current facts (post-v1.0 path 2 · 关洞)

> **Status:** OBSERVE only · **disposition = CUT**  
> **Date:** 2026-09-29  
> **Gate:** `scripts/stage9-pure-m4.ps1` (Stage 9-C H_00 pure M4)  
> **Honest:** Rust `yoyo.exe` **still** emits seed — **OW-SEED remains CUT**.  
> **Not CLOSED.** Pinning emitter/seed hash ≠ non-Rust emit path.

## Purpose

Machine-verifiable snapshot of the seed/emitter surface as of 2026-09-29.
This is **detection bar evidence**, not a CLOSED claim. OW-SEED CLOSED
requires a non-Rust emit path (YOYO compiler self-bootstrap) — see
`SCOPE-CUT-v1.0-hole-inventory.md`.

## Stage 9-C result (this run)

```text
H_00 chain gen1→gen4: GREEN
gen4 ≡ gen3_direct (.text DDC): EQUAL
bootstrap --selfhost: NOT USED
Stage 9-C: may check [x]
```

| Step | Result | Bytes |
|------|--------|-------|
| seed gen1 via `yoyo link` | GREEN | 251392 |
| gen1→gen2 (H_00 zero-arg) | GREEN | 251392 |
| gen2→gen3 (H_00 zero-arg) | GREEN | 251392 |
| gen3→gen4 (H_00 zero-arg) | GREEN | 251392 |
| gen3_direct via `bootstrap` (no `--selfhost`) | GREEN | 251392 |
| gen4 vs gen3_direct `.text` DDC | EQUAL | 20992 compared |
| gen3 vs gen4 `.text` DDC | EQUAL | 20992 compared |

## OW-SEED pin (fail-closed observe)

| Field | Value |
|-------|-------|
| **emitter** | `yoyo-rust/target/release/yoyo.exe` |
| **emitter bytes** | `22292992` |
| **emitter sha256_prefix** | `52f0a813b354a0f5` |
| **emitter sha256 full** | `52f0a813b354a0f50d7581bbfde35a43af44ddf65ba9771821028adc85f2633e` |
| **emitter LastWriteTime** | 2026-09-14 07:59:05 |
| **seed** | `scripts/_stage9-pure-m4/gen1.exe` |
| **seed bytes** | `251392` |
| **seed sha256_prefix** | `b0a8dbb0d3133e2a` |
| **seed sha256 full** | `b0a8dbb0d3133e2a7f763fe79af19975bb2f2f505b0fed1d625d0e8b8f793a74` |
| **SEED_HOST path** | `h00` |
| **SEED_HOST observe** | `SEED_HOST cmd=link target=win32 path=h00 bytes=251392 dll_embed=- sha256_prefix=b0a8dbb0d3133e2a` |
| **SEED_HOST ≡ on-disk seed** | YES (bytes + sha256_prefix match) |
| **gen4 .text sha256_prefix** | `1ec3766f` |
| **file sha256_prefix (gen4 ≡ gen3_direct)** | `b0a8dbb0` |
| **rustc** | `1.98.1 (48a229cea 2026-09-01)` |
| **bootstrap --selfhost** | NOT USED |

## What this proves

1. **H_00 pure M4 algebra works** — gen1→gen4 without host `--selfhost`.
2. **gen4 ≡ gen3_direct** on `.text` (DDC EQUAL) — M4 algebra inside YOYO PEs matches the seed/link host path.
3. **Emitter + seed are machine-pinned** — size + sha256_prefix fail-closed (stage13/15 inventory contract).

## What this does **not** prove (honest CUT)

1. Seed is still emitted by **Rust** `yoyo.exe` → **OW-SEED CUT**.
2. Each genN still embeds **Rust** `yoyo_runtime.dll` → **OW-RT CUT**.
3. `yoyo.ty` (3326 lines) is IR/handler orchestration — **not** a full YOYO-written compiler source.
4. Closing OW-SEED requires a **non-Rust emit path** (YOYO compiler self-bootstrap) — multi-month long pole; not this observe tick.

## Fixed-point extension (2026-09-30)

YOYO-built PE as emitter (not Rust `yoyo.exe`):

```text
gen4 (H_00 YOYO PE) → gen5
gen5 ≡ gen4  (.text DDC EQUAL + full-file sha256 EQUAL)
disposition = CUT  (gen1 seed still from Rust yoyo.exe)
```

| Field | Value |
|-------|-------|
| **emitter this step** | `scripts/_stage9-pure-m4/gen4.exe` (YOYO PE · not Rust) |
| **output** | `scripts/_stage9-pure-m4/gen5.exe` |
| **gen5 bytes** | `251392` |
| **gen4 sha256_prefix** | `b0a8dbb0d3133e2a` |
| **gen5 sha256_prefix** | `b0a8dbb0d3133e2a` |
| **full-file EQUAL** | YES |
| **`.text` DDC** | EQUAL · compared_bytes=`20992` · hash=`1ec3766f…` |

**Proves:** once past the Rust seed, the YOYO H_00 PE is a **stable fixed-point emitter** (genN→genN+1 ≡ genN).

**Does not prove:** OW-SEED CLOSED — the *first* seed (`gen1`) is still `yoyo link` from Rust `yoyo.exe`. Fixed-point after seed ≠ replacing the seed emitter.

### Repro (fixed-point)

```powershell
cd F:\yoyo\scripts\_stage9-pure-m4
# after stage9-pure-m4.ps1 left gen4.exe + input.tyb/input.ky
& .\gen4.exe
Copy-Item -Force output.exe gen5.exe
& F:\yoyo\yoyo-rust\target\release\yoyo.exe diff gen5.exe gen4.exe
# expect DDC: EQUAL; file hashes equal
```

## YOYO PE emit seed (2026-09-30 · Gate G slice)

Operational path: **seed emitted by YOYO PE**, not by Rust `yoyo.exe` on the emit step.

```text
hop1: emitter_gen4.exe → seed_yoyo ≡ seed_rust
hop2: seed_yoyo.exe    → seed2     ≡ seed_yoyo
rust_yoyo_exe_on_emit_hops = ABSENT
rust_sidecar_cwd = PRESENT
disposition = CUT
```

| Field | Value |
|-------|-------|
| **gate** | `scripts/stage17-ow-seed-yoyo-emit.ps1` |
| **status** | GREEN · **hops=2** |
| **hop1 emitter** | `emitter_gen4.exe` · YOYO_PE · bytes=`251392` · sha256_prefix=`b0a8dbb0d3133e2a` |
| **hop1 out** | `seed_yoyo.exe` · same sha |
| **hop2 emitter** | `seed_yoyo.exe` · YOYO_PE |
| **hop2 out** | `seed2.exe` · same sha |
| **seed_rust** | contrast only · same sha · via `yoyo link` |
| **parity** | hop1↔rust EQUAL · hop2↔hop1 EQUAL · full-file EQUAL |
| **Rust on emit hops** | ABSENT |

**Proves:** multi-hop emit chain with **no Rust `yoyo.exe` on any emit hop**; seed bytes match Rust `yoyo link`.

**Does not prove CLOSED:** trust-root `gen4` provenance still chains to stage9 Rust `gen1`. Cwd still needs Rust `yoyo_rt.dll`. CLOSED needs **no Rust in trust root or sidecar**.

### Repro (YOYO emit)

```powershell
cd F:\yoyo
& .\scripts\stage17-ow-seed-trust-root.ps1   # Gate H
& .\scripts\stage17-ow-seed-yoyo-emit.ps1    # Gate G (depends on H)
# expect both status=GREEN disposition=CUT
```

## Gate H — trust-root pin (2026-09-30)

| Field | Value |
|-------|-------|
| **pin file** | `scripts/ow-seed-trust-root.pin` |
| **gate** | `scripts/stage17-ow-seed-trust-root.ps1` |
| **status** | GREEN |
| **root** | `scripts/_stage9-pure-m4/gen4.exe` |
| **bytes** | `251392` |
| **sha256** | `b0a8dbb0d3133e2a7f763fe79af19975bb2f2f505b0fed1d625d0e8b8f793a74` |
| **provenance** | `rust_stage9_gen1_then_H00_chain` |
| **disposition** | CUT |

**Proves:** trust-root is machine-pinned; drift → RED.

**Does not prove CLOSED:** pin explicitly records Rust ancestry; cwd Rust sidecar still required (Gate I).

## Gate I experiment — no Rust sidecar (2026-09-30 · RED)

Try: run pinned YOYO emitter **without** Rust `yoyo_rt.dll`, using the YOYO
in-DLL-recompile pe_dll as cwd sidecar.

```text
gate:    scripts/stage17-ow-seed-no-rust-sidecar.ps1
status:  RED (expected)
```

| Field | Value |
|-------|-------|
| **contrast** gen4 + Rust sidecar | exit=`0` · seed bytes=`251392` · sha256_prefix=`b0a8dbb0d3133e2a` |
| **YOYO sidecar** | in-DLL-recompile pe_dll · bytes=`1620992` |
| **experiment** gen4 + YOYO sidecar | exit=`1` · output.exe **absent** |
| **rust_yoyo_rt_dll_in_cwd** | ABSENT |
| **disposition** | CUT |

**Proves:** the in-DLL-recompile sidecar **cannot** replace Rust `yoyo_rt.dll` on the
H_00 seed path. It is an oracle-table sidecar, not a full H_00 runtime contract.

**Does not prove CLOSED:** Gate I remains `[ ]`. Replacing the runtime sidecar is the
real long pole (multi-month).

### Repro (Gate I experiment)

```powershell
cd F:\yoyo
& .\scripts\stage17-ow-seed-no-rust-sidecar.ps1
# expect status=RED reason=yoyo_sidecar_cannot_replace_rust_runtime
```

## I-1 analysis — gen4.exe runtime contract (2026-09-30)

Reverse-engineered what `gen4.exe` actually needs from `yoyo_rt.dll`. This
**explains** the Gate I experiment RED and gives a concrete target for I.

### gen4.exe PE structure

```text
sections:         2 (no `.rdata` with typical import dir)
import dir:       empty (rva=0x0 size=0x0)
data dirs set:    1 only (reloc, size=25)
```

**Interpretation:** `gen4.exe` has **no standard Windows import table**. The
strings `kernel32.dll`, `ReadFile`, `ExitProcess`, `yoyo_rt.dll` are string
literals **inside code** — gen4 implements its own PE loader, does its own
syscall / syscall-like dispatch for `ReadFile` / `ExitProcess`, and does
its own DLL resolution against `yoyo_rt.dll` by parsing that DLL's export
table itself.

### What gen4 actually calls into `yoyo_rt.dll`

**Honest note:** `gen4.exe` contains the string `yoyo_rt.dll` but **no
`yoyo_runtime*` symbol string** anywhere in its bytes. It invokes the DLL's
entry point by name through its own PE loader, resolving the symbol name
against the DLL's export table at runtime — so the required symbol name
lives in gen4's code instructions, not in a visible string literal.

The Rust reference runtime `yoyo_rt.dll` exports exactly two symbols:

| Rust `yoyo_rt.dll` exports | Present in YOYO in-DLL-recompile pe_dll? |
|---|---|
| `yoyo_runtime_selfhost_main` | **YES** ✓ |
| `yoyo_runtime_selfhost_paths` | **NO** ✗ |

**Real export gap = `{ yoyo_runtime_selfhost_paths }`** — the path-taking
compile entry point that gen4's own loader resolves to run H_00.

**This is the exact reason Gate I experiment returned exit=1 with no output:**
gen4 couldn't resolve its one required export.

### Implication for Gate I scope

- **Not** "migrate H_00 runtime to a new language." The runtime already exists
  (Rust).
- **Not** "write N thousand lines of H_00 interpreter from scratch." The
  compiler already emits H_00 bytecode; only the **link-time packaging** is
  the issue.
- **Actually** = make YOYO's in-DLL-recompile pe_dll emit an **equivalent
  export surface** for `yoyo_runtime_selfhost_paths`, using the same
  oracle-table + codegen machinery that already produces 7 fixtures ×
  2-ABI parity. This is a **bounded engineering task** — not month-scale
  from today's evidence, but still not hours-scale (oracle needs to cover
  full H_00 instruction set, not just 7 fixtures).

### I-1 result

```text
gen4_import_dir_entries:      0
gen4_dll_dependencies:        0
rust_yoyo_rt_exports:         [yoyo_runtime_selfhost_main, yoyo_runtime_selfhost_paths]
yoyo_sidecar_actual_exports:  [yoyo_runtime_selfhost_main, yoyo_in_dll_recompile(marker)]
gap:                          yoyo_runtime_selfhost_paths missing
disposition:                  CUT (still)
next_step:                    expand YOYO pe_dll codegen to also emit
                              yoyo_runtime_selfhost_paths with full H_00 ISA
```

## Repro

```powershell
cd F:\yoyo
& .\scripts\stage9-pure-m4.ps1
# Then pin:
$Yoyo = "F:\yoyo\yoyo-rust\target\release\yoyo.exe"
$Seed = "F:\yoyo\scripts\_stage9-pure-m4\gen1.exe"
(Get-Item $Yoyo).Length
(Get-FileHash $Yoyo -Algorithm SHA256).Hash.ToLowerInvariant()
(Get-Item $Seed).Length
(Get-FileHash $Seed -Algorithm SHA256).Hash.ToLowerInvariant()
```

## Related

- `SCOPE-CUT-v1.0-hole-inventory.md` — OW-SEED CLOSED criteria
- `scripts/stage13-link-host.ps1` — seed/link host contract + OW-SEED pin
- `scripts/stage9-pure-m4.ps1` — H_00 pure M4 gate (this run)
- `POST-1.0-HOLE-CHECKLIST.md` — path 2 关洞看板

---

*Post-v1.0 path 2 · OW-SEED observe only · disposition=CUT · 2026-09-29*
