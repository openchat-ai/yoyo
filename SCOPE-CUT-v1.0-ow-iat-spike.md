# OW-IAT spike — in-process PE mapper (post-v1.0 path 2)

> **Status:** SPIKE (not CLOSED)  
> **Gate:** `scripts/stage17-ow-iat-spike.ps1` (+ `scripts/stage17-ow-iat-spike.sh` on Linux)  
> **Code:** `yoyo-rust/verifier/src/pe_manual_map.rs`

## Goal

Remove host **`LoadLibraryA`** / **`dlopen`** from the approved H_00 seed path by loading cwd sidecar `yoyo_rt.dll` / `./libyoyo_runtime.so` with a **YOYO-emitted in-process loader** instead of the Windows loader / libdl.

**Honest today:** seed PE still contains ASCII `LoadLibraryA` in the kernel32 IAT. H_00 stub still calls it (71 B span · `stub_nz=69`). **OW-IAT remains CUT.**

## CLOSED criteria (fail-closed · unchanged from SCOPE-CUT v1.0)

| Hole | CLOSED when | Not sufficient |
|------|-------------|----------------|
| **OW-IAT (Win)** | Rust seed PE has **no** ASCII `LoadLibraryA` | Dropping `GetProcAddress` only |
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
| **Stub size** | Manual map + file read ≫ 71 B; JS + asm peers must mirror before merge |
| **OW-H00** | `three_peer_full=EQUAL` · sha `808b9ec8` — do not land stub change without peer sync |
| **Gates** | `stage11` / `stage13` / `stage14` still **require** `LoadLibraryA` until flip PR |
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

Gate prints `OW_IAT_SPIKE status=GREEN` and **`LoadLibraryA=PRESENT`** until wire-up PR lands.

---

*Post-v1.0 path 2 · OW-IAT begin · spike proves loader algorithm; seed still CUT*
