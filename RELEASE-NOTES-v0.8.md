# YOYO v0.8.0 Release Notes

**Date:** 2026-08-29  
**Tag:** `v0.8.0`  
**Commit:** `f5997a8aaf069b3d46676f4a38b42b0be04be15c`

---

## One-line pitch

**YOYO v0.8 nails outside-window bytes into an honest SCOPE-CUT and thickens Lock pin / Relock discipline — contracting DIFF blind zones without claiming Thompson proof, YOYO-built runtime, full `.text` EQUAL, or replacing C.**

---

## What this release is

YOYO v0.8 **closes the largest honest observation hole after v0.7** on outside-window / full `.text` DIFF blindness, hardens Lock pin / Relock fail-closed discipline, and keeps v0.7 gates green. It is not a pivot to general application development.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs — a practical trust bar, **not** Thompson immunity. **SCOPE-CUT status=ACTIVE.** **Still Rust runtime + LoadLibrary/libdl.** **Seed still Rust-emitted.** Full `.text` peer compare **DIFF** (expected). Stub OS remain stub. Comparable EQUAL = selfhost-body window only.

---

## Included in v0.8

| Area | v0.8 includes |
|------|----------------|
| **Outside-window SCOPE-CUT** | OW-H00 / OW-STUB / OW-RT / OW-IAT / OW-SEED pinned; `SCOPE-CUT-v0.8-outside-window.md`; `stage14-outside-window-scope-cut.ps1` |
| **Lock harden** | Decision #25 nail; drift → `RELOCK_REQUIRED`; `stage14-lock-harden.ps1` |
| **v0.7 baseline** | seed/link · parity · three-peer I/O · selfhost-body · LoadLibrary/libdl · pure M4 · fullbody · lock · gen12 — still green |

### Trust-chain anchors

| Monitor | Value |
|---------|-------|
| **Lock pin** | `0275802d…` (Decision #25, **unchanged** — no Relock) |
| **SCOPE-CUT** | **ACTIVE** · full `.text` DIFF · body EQUAL **17805** · stub_nz **159** · dll **154624** |
| **gen12 `.text` (Win)** | **`d782166d…`** · **18432** bytes |
| **runtime.dll** | **154624** B (still Rust-built; **OW-RT CUT**) |
| **Linux trampoline** | **9768** B (still libdl host path) |

---

## Explicitly NOT in v0.8

| Item | Status |
|------|--------|
| **SCOPE-CUT ACTIVE (not closed)** | OW-H00 / OW-STUB / OW-RT / OW-IAT / OW-SEED remain CUT |
| **Rust runtime still embedded** | `yoyo_runtime.dll` / `.so` still host-built; outside compared windows |
| **YOYO-built runtime** | Deferred — still Rust cdylib |
| **LoadLibrary / libdl removed** | Still present (**OW-IAT**) |
| **Seed no longer Rust-emitted** | Still emitted by Rust `yoyo.exe` (**OW-SEED**) |
| **Full `.text` three-peer EQUAL** | Only selfhost-body window EQUAL; full `.text` DIFF under CUT |
| **Thompson-proof / C replacement** | Forbidden claims |
| **Hole inventory close-or-CUT** | v0.9+ |
| **MCU / Morph as main track** | OUT |

---

## Pre-publish gates (all exit 0)

```powershell
cd F:\yoyo
.\scripts\stage14-lock-harden.ps1 -SkipBuild
.\scripts\stage14-v07-regress.ps1 -SkipBuild
# (embeds stage13/12/11/10/9 + all/lock/gen12/fullbody + Stage 14 A/B + WSL)
```

---

## North star reminder

**打破后门魔咒** — fewer outside-window / Lock-drift blind paths under DDC+Lock. Detection bar, not proof. SCOPE-CUT ACTIVE means honest remaining holes, not closed holes.
