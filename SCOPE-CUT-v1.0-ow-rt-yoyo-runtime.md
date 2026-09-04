# OW-RT spike — PE DLL emit toward YOYO-built runtime (post-v1.0 path 2 · 整仓竣工)

> **Status:** SPIKE Gate G **slice** (alt sidecar emit — **not** CLOSED)  
> **Gate:** `scripts/stage17-ow-rt-yoyo-runtime.ps1` · Linux/cloud: `scripts/stage17-ow-rt-yoyo-runtime.sh`  
> **Code:** `yoyo-rust/verifier/src/pe_dll_link.rs` · bin `emit-rt-sidecar` · stub `yoyo/tests/golden/ow_rt_yoyo_origin_exit2.ty` · fixture `yoyo/tests/golden/selfhost_min_nop.ty`  
> **Honest:** Rust `yoyo_rt.dll` / `yoyo_runtime.dll` **still** the production default sidecar — **OW-RT remains CUT**.

## Goal

Replace the Rust-built cwd sidecar runtime (`yoyo_runtime_selfhost_main`) with a **YOYO-built** PE DLL / `.so` that H_00 manual-map can load under the same ordinal-0 export contract — then drop Rust runtime host trust.

**整仓竣工 long pole.** Expect multi-month; one tick ≠ CLOSED.

## CLOSED criteria (fail-closed · from SCOPE-CUT v1.0)

| Hole | CLOSED when | Not sufficient |
|------|-------------|----------------|
| **OW-RT** | No exact embed **and** no Rust LoadLibrary/libdl / `yoyo_rt.dll` sidecar host trust (YOYO-built runtime) | Sidecar-only shrink; thinner LTO DLL; PE DLL emit spike; YOYO-origin export probe; R→C→W effect harness; **alt-sidecar emit / opt-in** |
| **OW-IAT** (follow-on) | No host DLL load face (no `yoyo_rt.dll` marker) | Manual-map + Rust sidecar still CUT |

## Phased path (checklist Gates D→G)

| Gate | Deliverable | Disposition after |
|------|-------------|-------------------|
| **D** | `pe_dll_link` emits PE32+ DLL with `AddressOfFunctions[0]` = `yoyo_runtime_selfhost_main`; unit + optional manual-map call | **CUT** · `yoyo_built=ABSENT` |
| **E** | YOYO `.ty` RAW_BYTES+RET fills export body — stub fixed exit-2; host-linked DLL shell OK; bytes are YOYO-origin | **CUT** · `yoyo_origin_export=PRESENT` · production still Rust |
| **F** | YOYO-built read→compile→write **effect** (`yoyo_built_runtime_effect`) — exit 0/1/2/3 + PE vs fixture parity; no LoadLibrary | **CUT** · `yoyo_built=EFFECT` · `yoyo_built_effect=PRESENT` · Rust sidecar still shipped |
| **G slice** (this tick) | Emit YOYO `pe_dll` as **alt** cwd `yoyo_rt.dll` (`emit-rt-sidecar`); production default still Rust | **CUT** · `yoyo_built=ALT_SIDECAR` · `yoyo_alt_sidecar=EMITTED` |
| **G CLOSED** | Production H_00 uses YOYO-built sidecar; Rust `yoyo_runtime` **gone** from trust path; inventory fail-closed CLOSED | **OW-RT CLOSED** only with evidence |

## Export / effect contract (must not drift)

```text
cwd sidecar name:  yoyo_rt.dll
export ordinal-0:  yoyo_runtime_selfhost_main  (AddressOfFunctions[0])
no-input exit:     2
compile fail:      1
write fail:        3
success:           0  (+ write output.exe PE)
```

Probe DLL from `link_probe_runtime_dll()` / `write_yoyo_alt_sidecar()` returns **2** from the YOYO-origin export (DllMain returns 1).  
`yoyo_built_runtime_effect(work_dir)` mirrors the same exits using YOYO seed/link `bootstrap_compile` (dual-track OK; production still Rust).  
Place alt bytes with `emit-rt-sidecar` / `write_yoyo_alt_sidecar` as cwd `yoyo_rt.dll` (H_00 loads whatever is present; production scripts still Copy-Item Rust). Env `YOYO_OW_RT_ALT_SIDECAR` documents the opt-in intent for future production switch.

## Machine check

```powershell
cd F:\yoyo
& .\scripts\stage17-ow-rt-yoyo-runtime.ps1
```

Linux/cloud (unit + emit; no Win H_00 smoke):

```bash
bash scripts/stage17-ow-rt-yoyo-runtime.sh
```

Gate prints `OW_RT_SPIKE status=GREEN`, **`yoyo_alt_sidecar=EMITTED`**, and **`yoyo_built=ALT_SIDECAR disposition=CUT`**.

### Local Windows — H_00 loads YOYO alt (optional)

```powershell
cd F:\yoyo\yoyo-rust
cargo build --release -p verifier
$work = Join-Path $env:TEMP 'yoyo-ow-rt-alt'
New-Item -ItemType Directory -Force -Path $work | Out-Null
cargo run -p verifier --bin emit-rt-sidecar -- (Join-Path $work 'yoyo_rt.dll')
& .\target\release\yoyo.exe link --target=win32 ..\yoyo\projects\yoyo.ty (Join-Path $work 'gen1.exe')
Push-Location $work; & .\gen1.exe; Pop-Location   # expect exit 2 (no input.*)
```

## Remaining CUT after OW-RT CLOSED

- **OW-SEED** — Rust `yoyo.exe` emitter  
- **OW-STUB** / **OW-H00** — stub / full `.text` peer DIFF as applicable  
- **Host I/O** — CreateFile / ReadFile / VirtualAlloc (or Linux syscalls)  
- **REL-FULLTEXT** — never graduation CLOSED  
- **REL-STUBOS** — until production I/O on stub OS  

---

*Post-v1.0 path 2 · 整仓竣工 · OW-RT long pole · Gate G slice = alt sidecar emit only*
