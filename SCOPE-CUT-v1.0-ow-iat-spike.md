# OW-IAT spike — in-process PE mapper (post-v1.0 path 2)

> **Status:** SPIKE (not CLOSED)  
> **Gate:** `scripts/stage17-ow-iat-spike.ps1` (+ `scripts/stage17-ow-iat-spike.sh` on Linux)  
> **Wire-up gate:** `scripts/stage17-ow-iat-wireup.ps1` (+ CI `windows-latest`)  
> **Code:** `yoyo-rust/verifier/src/pe_manual_map.rs` · `h00_manual_map_wireup.rs`

## Goal

Remove host **`LoadLibraryA`** / **`dlopen`** from the approved H_00 seed path by loading cwd sidecar `yoyo_rt.dll` / `./libyoyo_runtime.so` with a **YOYO-emitted in-process loader** instead of the Windows loader / libdl.

**Honest today (post PR #8 + Win smoke):** H_00 **manual-map wired** (`stub_nz=905`); PEB `LoadLibraryA` **DROPPED**; CreateFile/Read/VirtualAlloc + `pe_manual_map`; cwd `yoyo_rt.dll` **required** (fail-closed smoke). **Still host-trusted I/O + sidecar.** **OW-IAT remains CUT.**

## CLOSED criteria (fail-closed · unchanged from SCOPE-CUT v1.0)

| Hole | CLOSED when | Not sufficient |
|------|-------------|----------------|
| **OW-IAT (Win)** | No host DLL load face (no `yoyo_rt.dll` sidecar marker) | Dropping IAT/ASCII `LoadLibraryA` only (manual-map + sidecar still CUT) |
| **OW-IAT (Linux)** | Seed ELF / tramp has **no** `dlopen` / `libdl` import surface on H_00 path | Dropping `dlsym` only |

After CLOSED, **kernel32 file I/O** (`CreateFileA` / `ReadFile` / `VirtualAlloc`) or **Linux syscalls** (`open` / `read` / `mmap`) remain **host-trusted** — a separate shrink track, not OW-IAT.

## Win path (wired)

```text
H_00 stub (~905B nz):
  lea rcx, "yoyo_rt.dll"
  CreateFileA → ReadFile → VirtualAlloc   (IAT slots 0–2)
  manual_map_pe_dll (in-stub emit)
  export AddressOfFunctions[0] → call → ExitProcess
```

`pe_manual_map.rs` implements the Rust side:

1. `map_pe_sections` — copy headers + sections into `SizeOfImage` buffer  
2. `apply_base_relocations` — `IMAGE_REL_BASED_DIR64`  
3. `resolve_imports` — fill IAT via caller callback (walk already-loaded modules; no `GetProcAddress` IAT)  
4. `export_function_rva_functions0` — same contract as current H_00 stub  

**Unit tests** prove reloc, import resolve, and `functions[0]` export walk without Windows runtime.

## Wire-up status

| Phase | Status |
|-------|--------|
| 1. File-read prelude emit | **WIRED** — `h00_manual_map_wireup.rs` |
| 2. Manual-map x64 body | **WIRED** — `gen_h00_manual_map_main` replaces PEB `LoadLibraryA` (**905B** nz stub) |
| 3. Three-peer sync | **EQUAL** — JS `h00-manual-map-peer.js` + asm delegate · full `.text` **`72c27c9f`** |
| 4. Win smoke | **WIRED** — `stage17-ow-iat-wireup.ps1` cwd `yoyo_rt.dll` + fail-closed without sidecar |
| 5. Gate flip | CLOSED only when `yoyo_rt.dll` sidecar marker absent |

## Linux (wired 2026-08-29)

Hybrid tramp: dynamic `-lc` + `libgcc_s` only (**no libdl NEEDED**); sidecar via **dlopen@PLT** + in-process sym walk (no dlsym); **no glibc/ld disk mmap**. **Still CUT** (dlopen + ld.so libc + cwd sidecar).

## Remaining CUT after Win OW-IAT CLOSED

- **OW-RT** — sidecar still Rust-built `yoyo_runtime.dll`  
- **OW-SEED** — still Rust `yoyo.exe` emitter  
- **Host I/O** — CreateFile / ReadFile / VirtualAlloc (or Linux syscalls)  
- **REL-FULLTEXT** — graduation semantics unchanged  

## Machine check

```powershell
cd F:\yoyo
.\scripts\stage17-ow-iat-spike.ps1
.\scripts\stage17-ow-iat-wireup.ps1
```

```bash
cd yoyo-rust && cargo test -p verifier pe_manual_map
./scripts/stage17-ow-iat-spike.sh
```

Gate prints `OW_IAT_SPIKE status=GREEN` and **`IAT_LoadLibraryA=ABSENT`**. Wire-up adds **`smoke=GREEN sidecar_required=YES`** — **OW-IAT still CUT**.

---

*Post-v1.0 path 2 · OW-IAT manual-map wired + Win smoke · seed still CUT · next: YOYO-built runtime*
