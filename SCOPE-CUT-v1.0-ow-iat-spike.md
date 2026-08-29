# OW-IAT spike — in-process PE mapper (post-v1.0 path 2)

> **Status:** SPIKE (not CLOSED)  
> **Gate:** `scripts/stage17-ow-iat-spike.ps1` (+ `scripts/stage17-ow-iat-spike.sh` on Linux)  
> **Code:** `yoyo-rust/verifier/src/pe_manual_map.rs`

## Goal

Remove host **`LoadLibraryA`** / **`dlopen`** from the approved H_00 seed path by loading cwd sidecar `yoyo_rt.dll` / `./libyoyo_runtime.so` with a **YOYO-emitted in-process loader** instead of the Windows loader / libdl.

**Honest today (post deeper OW-IAT `3c4554d`):** seed PE has **no** ASCII `LoadLibraryA` in kernel32 IAT — H_00 resolves it via PEB→kernel32 ROR13 hash walk (`stub_nz=251`). **Still calls host LoadLibrary.** Manual-map spike **not wired**. **OW-IAT remains CUT.**

## CLOSED criteria (fail-closed · unchanged from SCOPE-CUT v1.0)

| Hole | CLOSED when | Not sufficient |
|------|-------------|----------------|
| **OW-IAT (Win)** | No host DLL load face (no `yoyo_rt.dll` sidecar marker) | Dropping IAT/ASCII `LoadLibraryA` only (PEB resolve still CUT) |
| **OW-IAT (Linux)** | Seed ELF / tramp has **no** `dlopen` / `libdl` import surface on H_00 path | Dropping `dlsym` only |

After CLOSED, **kernel32 file I/O** (`CreateFileA` / `ReadFile` / `VirtualAlloc`) or **Linux syscalls** (`open` / `read` / `mmap`) remain **host-trusted** — a separate shrink track, not OW-IAT.

## Spike path (Win — chosen first slice)

```text
H_00 stub (future, larger than 71 B):
  lea rcx, "yoyo_rt.dll"
  CreateFileA → ReadFile → VirtualAlloc   (existing IAT slots 1–3, 0)
  manual_map_pe_dll (in-stub or shared emit)
  export AddressOfFunctions[0] → call → ExitProcess
```

`pe_manual_map.rs` implements the Rust side:

1. `map_pe_sections` — copy headers + sections into `SizeOfImage` buffer  
2. `apply_base_relocations` — `IMAGE_REL_BASED_DIR64`  
3. `resolve_imports` — fill IAT via caller callback (walk already-loaded modules; no `GetProcAddress` IAT)  
4. `export_function_rva_functions0` — same contract as current H_00 stub  

**Unit tests** prove reloc, import resolve, and `functions[0]` export walk without Windows runtime.

## Not wired yet (why three-peer EQUAL is preserved)

| Blocker | Detail |
|---------|--------|
| **Stub size** | Manual map + file read ≫ 251 B PEB stub; JS + asm peers must mirror before merge |
| **OW-H00** | JS peer still **71 B** (LoadLibraryA IAT) vs Rust **251 B** (PEB) — three-peer **DIFF** until sync |
| **Gates** | `stage11`/`stage13`/`stage15` require IAT/ASCII `LoadLibraryA` **absent** (done); CLOSED needs no `yoyo_rt.dll` |
| **Smoke** | Needs Windows cwd sidecar smoke after wire-up |

## Linux (next slice)

Replace tramp `dlopen@PLT` with `open` + `read` + `mmap` + in-process ELF PT_LOAD map (mirror Win spike). Tramp blob is outside Win three-peer `.text` EQUAL but embedded in seed ELF.

## Remaining CUT after Win OW-IAT CLOSED

- **OW-RT** — sidecar still Rust-built `yoyo_runtime.dll`  
- **OW-SEED** — still Rust `yoyo.exe` emitter  
- **Host I/O** — CreateFile / ReadFile / VirtualAlloc (or Linux syscalls)  
- **REL-FULLTEXT** — graduation semantics unchanged  

## Machine check (spike only)

```bash
cd yoyo-rust && cargo test -p verifier pe_manual_map
./scripts/stage17-ow-iat-spike.sh
```

Gate prints `OW_IAT_SPIKE status=GREEN` and **`IAT_LoadLibraryA=ABSENT`** (PEB resolve / manual-map wire-up still CUT).

## Wire-up WIP (branch `cursor/ow-iat-manual-map-wireup-4d89`)

| Phase | Status |
|-------|--------|
| 1. File-read prelude emit | `h00_manual_map_wireup.rs` · gate `stage17-ow-iat-wireup` |
| 2. Manual-map x64 body | NOT STARTED — replace PEB `LoadLibraryA` call |
| 3. Three-peer sync | JS still **71 B** vs Rust **251 B** — lockstep required before land |
| 4. Gate flip | CLOSED only when `yoyo_rt.dll` sidecar marker absent |

---

*Post-v1.0 path 2 · OW-IAT begin · spike proves loader algorithm; seed still CUT*
