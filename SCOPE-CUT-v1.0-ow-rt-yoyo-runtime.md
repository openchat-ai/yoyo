# OW-RT spike — 自举路径 runtime 信任收口（post-v1.0 path 2）

> **Status:** Gate G **CLOSED** (OW-RT CLOSED · human auth **A**)  
> **北星**：打破后门魔咒 / 自举  
> **Gate:** `scripts/stage17-ow-rt-yoyo-runtime.ps1` · Linux/cloud: `scripts/stage17-ow-rt-yoyo-runtime.sh`  
> **Code:** `yoyo-rust/verifier/src/pe_dll_link.rs` · bin `emit-rt-sidecar` (`--in-dll-recompile`) · fixtures `selfhost_min_nop.ty` + `01_set_get.ty`  
> **Honest:** S3.738 human **A** — CreateProcess-compiler is production trust; disposition **CLOSED** (`closed=1`).

## Goal

加强 **自举**：生产路径不再默认信 Rust runtime；CreateProcess-compiler（auth A）。  
OW-\* 标签 = 宿主债附录（非 ISA）；禁词表 = 对外防吹牛，**不是**进度尺。

## CLOSED criteria (fail-closed · from SCOPE-CUT v1.0)

| Hole | CLOSED when | Not sufficient |
|------|-------------|----------------|
| **OW-RT** | No exact embed **and** no Rust LoadLibrary/libdl / `yoyo_rt.dll` sidecar **production** host trust (YOYO-built runtime + CreateProcess-compiler) | Sidecar-only shrink; thinner LTO DLL; PE DLL emit spike alone; oracle-only graduation |
| **OW-IAT** (follow-on) | No host DLL load face (no `yoyo_rt.dll` marker) | Manual-map + cwd sidecar marker still present |

## Phased path (checklist Gates D→G)

| Gate | Deliverable | Disposition after |
|------|-------------|-------------------|
| **D** | `pe_dll_link` emits PE32+ DLL with `AddressOfFunctions[0]` = `yoyo_runtime_selfhost_main`; unit + optional manual-map call | **CUT** · `yoyo_built=ABSENT` |
| **E** | YOYO `.ty` RAW_BYTES+RET fills export body — stub fixed exit-2; host-linked DLL shell OK; bytes are YOYO-origin | **CUT** · `yoyo_origin_export=PRESENT` |
| **F** | YOYO-built read→compile→write **effect** (`yoyo_built_runtime_effect`) — exit 0/1/2/3 + PE vs fixture parity; no LoadLibrary | **CUT** · `yoyo_built=EFFECT` |
| **G slice** (alt / RCW / export-compile / in-dll-recompile) | in-DLL recompile oracle + Off→YOYO + co-place + fail-closed | landed under CUT trail S732–737 |
| **G CLOSED** | Production H_00 uses YOYO-built path; Rust `yoyo_runtime` **not** Off default; inventory fail-closed CLOSED | **OW-RT CLOSED** · `disposition=CLOSED` · `closed=1` · human auth A (S3.738) |

## Export / effect contract (must not drift)

```text
cwd sidecar name:  yoyo_rt.dll
export ordinal-0:  yoyo_runtime_selfhost_main  (AddressOfFunctions[0])
no-input exit:     2
compile fail:      1
write fail:        3
success:           0  (+ write output.exe PE)
```

`place_cwd_runtime_sidecar` Off → YOYO in-dll candidate **and** co-places `yoyo_compiler.exe` (S732/S734). Fail-closed refuses oracle-only graduation (S735). Auth A: CreateProcess-compiler = primary trust; residual oracle table ≠ sole graduation path.

## Machine check

```powershell
cd F:\yoyo
& .\scripts\stage17-ow-rt-yoyo-runtime.ps1
```

Gate prints `OW_RT_SPIKE status=GREEN`, **`disposition=CLOSED`**, **`g_closed=PRESENT`**, **`closed=1`**.

## S3.738 human auth A (CLOSED)

**Decision:** CreateProcess-compiler is production trust; co-place + fail-closed refuse oracle-only; residual finite oracle in sidecar body is **not** the trust path.

**Evidence trail:** E1–E5 + CreateProcess-primary pack + inventory hard-gate map (S732–737) → human authorize CLOSED (S738).

## Remaining CUT after OW-RT CLOSED

> Follow-on cliffs **mapped** — still ABSENT as CLOSED. **Path 2 practical floor frozen (2026-11-06):**  
> OW-RT CLOSED = 实用自给下限；其余为宿主债病历名（非 ISA），**CUT 冻结**，勿开无限 FO/S4 tip。  
> 再缩宿主 = 另开**有界单门**，不是自动主线。

- **OW-IAT** — cwd `yoyo_rt.dll` load face · CUT 冻结（FO.1 refresh only）  
- **OW-SEED** — **CLOSED** @ human A 2026-11-07（FULLBAR pin 生产信任根；见 `SCOPE-CUT-v1.0-ow-seed-observe.md`）  
- **OW-STUB** / **OW-H00** — stub / full `.text` · CUT 冻结  
- **Host I/O** — CreateFile / ReadFile / VirtualAlloc · CUT 冻结  
- **REL-FULLTEXT** — never graduation CLOSED  
- **REL-STUBOS** — far / not mainline  

---

*Post-v1.0 path 2 · 实用自给下限 · OW-RT CLOSED (S3.738) · 其余 CUT 冻结*
