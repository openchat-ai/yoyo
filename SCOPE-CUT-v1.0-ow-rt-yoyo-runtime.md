# OW-RT spike — PE DLL emit toward YOYO-built runtime (post-v1.0 path 2 · 整仓竣工)

> **Status:** SPIKE Gate G **slice** (sidecar-path R→C→W — **not** CLOSED)  
> **Gate:** `scripts/stage17-ow-rt-yoyo-runtime.ps1` · Linux/cloud: `scripts/stage17-ow-rt-yoyo-runtime.sh`  
> **Code:** `yoyo-rust/verifier/src/pe_dll_link.rs` · bin `emit-rt-sidecar` (`--rcw`) · stub `yoyo/tests/golden/ow_rt_yoyo_origin_exit2.ty` · fixture `yoyo/tests/golden/selfhost_min_nop.ty`  
> **Honest:** Rust `yoyo_rt.dll` / `yoyo_runtime.dll` **still** the production default sidecar — **OW-RT remains CUT**.

## Goal

Replace the Rust-built cwd sidecar runtime (`yoyo_runtime_selfhost_main`) with a **YOYO-built** PE DLL / `.so` that H_00 manual-map can load under the same ordinal-0 export contract — then drop Rust runtime host trust.

**整仓竣工 long pole.** Expect multi-month; one tick ≠ CLOSED.

## CLOSED criteria (fail-closed · from SCOPE-CUT v1.0)

| Hole | CLOSED when | Not sufficient |
|------|-------------|----------------|
| **OW-RT** | No exact embed **and** no Rust LoadLibrary/libdl / `yoyo_rt.dll` sidecar host trust (YOYO-built runtime) | Sidecar-only shrink; thinner LTO DLL; PE DLL emit spike; YOYO-origin export probe; R→C→W effect harness; alt-sidecar emit; **sidecar-path RCW (host-orchestrated)** |
| **OW-IAT** (follow-on) | No host DLL load face (no `yoyo_rt.dll` marker) | Manual-map + Rust sidecar still CUT |

## Phased path (checklist Gates D→G)

| Gate | Deliverable | Disposition after |
|------|-------------|-------------------|
| **D** | `pe_dll_link` emits PE32+ DLL with `AddressOfFunctions[0]` = `yoyo_runtime_selfhost_main`; unit + optional manual-map call | **CUT** · `yoyo_built=ABSENT` |
| **E** | YOYO `.ty` RAW_BYTES+RET fills export body — stub fixed exit-2; host-linked DLL shell OK; bytes are YOYO-origin | **CUT** · `yoyo_origin_export=PRESENT` · production still Rust |
| **F** | YOYO-built read→compile→write **effect** (`yoyo_built_runtime_effect`) — exit 0/1/2/3 + PE vs fixture parity; no LoadLibrary | **CUT** · `yoyo_built=EFFECT` · `yoyo_built_effect=PRESENT` · Rust sidecar still shipped |
| **G slice** (alt emit) | Emit YOYO `pe_dll` as **alt** cwd `yoyo_rt.dll` (`emit-rt-sidecar`); production default still Rust | **CUT** · `yoyo_built=ALT_SIDECAR` · `yoyo_alt_sidecar=EMITTED` |
| **G slice** (this tick) | `yoyo_sidecar_path_rcw` — place YOYO `yoyo_rt.dll` + host R→C→W under same cwd; `place_cwd_runtime_sidecar` opt-in | **CUT** · `yoyo_built=SIDECAR_RCW` · `yoyo_sidecar_rcw=PRESENT` · `gate_g_slice=sidecar_rcw` · export still exit-2 · production_default=RUST |
| **G CLOSED** | Production H_00 uses YOYO-built **compile** sidecar; Rust `yoyo_runtime` **gone** from trust path; inventory fail-closed CLOSED | **OW-RT CLOSED** only with evidence |

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
`yoyo_built_runtime_effect(work_dir)` mirrors the same exits using YOYO seed/link `bootstrap_compile`.  
`yoyo_sidecar_path_rcw(work_dir)` places that probe as cwd `yoyo_rt.dll` **then** runs the effect (sidecar path layout). Honest: compile is **not** yet inside the DLL.  
`place_cwd_runtime_sidecar` / `win32_selfhost::write_cwd_runtime_sidecar` write YOYO when `YOYO_OW_RT_ALT_SIDECAR` is set, else Rust bytes — default remains Rust (production `Copy-Item` scripts unchanged until they call the helper).

## Machine check

```powershell
cd F:\yoyo
& .\scripts\stage17-ow-rt-yoyo-runtime.ps1
```

Linux/cloud (unit + sidecar-path RCW; no Win H_00 smoke):

```bash
bash scripts/stage17-ow-rt-yoyo-runtime.sh
```

Gate prints `OW_RT_SPIKE status=GREEN`, **`yoyo_sidecar_rcw=PRESENT`**, **`gate_g_slice=sidecar_rcw`**, and **`yoyo_built=SIDECAR_RCW disposition=CUT`**.  
Win H_00+probe may print `yoyo_alt_sidecar_smoke=NOT_STABLE` (AV) — honest; does not fail the gate (RCW path is the slice evidence).

## Remaining CUT after OW-RT CLOSED

- **OW-SEED** — Rust `yoyo.exe` emitter  
- **OW-STUB** / **OW-H00** — stub / full `.text` peer DIFF as applicable  
- **Host I/O** — CreateFile / ReadFile / VirtualAlloc (or Linux syscalls)  
- **REL-FULLTEXT** — never graduation CLOSED  
- **REL-STUBOS** — until production I/O on stub OS  

---

*Post-v1.0 path 2 · 整仓竣工 · OW-RT long pole · Gate G slice = sidecar-path RCW (not CLOSED)*
