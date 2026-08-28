# YOYO v0.9.0 Release Notes

**Date:** 2026-08-29  
**Tag:** `v0.9.0`  
**Commit:** *(filled after graduation commit)*

---

## One-line pitch

**YOYO v0.9 enumerates every remaining host hole as machine CLOSED|CUT and adds serial pre-run keep-green — contracting blind graduation without claiming Thompson proof, YOYO-built runtime, hole closure, or replacing C.**

---

## What this release is

YOYO v0.9 **closes the largest honest observation hole after v0.8** on lump SCOPE-CUT blindness (per-hole disposition), adds a serial pre-run keep-green gate, and keeps v0.8 gates green. It is not a pivot to general application development.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs — a practical trust bar, **not** Thompson immunity. **HOLE_INVENTORY status=ACTIVE** · **closed=0 cut=7**. **Still Rust runtime + LoadLibrary/libdl.** **Seed still Rust-emitted.** Full `.text` peer compare **DIFF** (expected). Stub OS remain stub. Comparable EQUAL = selfhost-body window only.

---

## Included in v0.9

| Area | v0.9 includes |
|------|----------------|
| **Hole inventory** | OW-\* / REL-\* each `CLOSED\|CUT`; `SCOPE-CUT-v0.9-hole-inventory.md`; `stage15-hole-inventory.ps1` |
| **Pre-run keep-green** | Serial `stage15-prerun.ps1` / keep-green; `driver.lock`; named `-SkipBuild`; zero parallel cargo |
| **v0.8 baseline** | outside-window SCOPE-CUT · Lock harden · seed/link · parity · three-peer · selfhost-body · pure M4 · fullbody · lock · gen12 — still green |

### Trust-chain anchors

| Monitor | Value |
|---------|-------|
| **Lock pin** | `0275802d…` (Decision #25, **unchanged** — no Relock) |
| **HOLE_INVENTORY** | **ACTIVE** · closed=0 · cut=7 · full `.text` DIFF · body EQUAL **17805** · stub_nz **159** · dll **154624** |
| **gen12 `.text` (Win)** | **`d782166d…`** · **18432** bytes |
| **runtime.dll** | **154624** B (still Rust-built; **OW-RT CUT**) |
| **Linux trampoline** | **9768** B (still libdl host path) |

---

## Explicitly NOT in v0.9

| Item | Status |
|------|--------|
| **Holes closed** | All seven remain **CUT** (inventory ACTIVE; closed=0) |
| **Rust runtime still embedded** | `yoyo_runtime.dll` / `.so` still host-built; **OW-RT** |
| **YOYO-built runtime** | Deferred — still Rust cdylib |
| **LoadLibrary / libdl removed** | Still present (**OW-IAT**) |
| **Seed no longer Rust-emitted** | Still emitted by Rust `yoyo.exe` (**OW-SEED**) |
| **Full `.text` three-peer EQUAL** | Only selfhost-body window EQUAL; full `.text` DIFF under CUT |
| **Thompson-proof / C replacement** | Forbidden claims |
| **1.0 SCOPE-CUT final / full close** | Stage 16 / v1.0 |
| **MCU / Morph as main track** | OUT |

---

## Pre-publish gates (all exit 0)

```powershell
cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage14-lock-harden.ps1 -SkipBuild
.\scripts\stage15-v08-regress.ps1 -SkipBuild
# (embeds stage13/12/11/10/9 + all/lock/gen12/fullbody + Stage 14 A/B + Stage 15-A + WSL)
```

---

## North star reminder

**打破后门魔咒** — fewer lump-CUT / blind-graduation paths under DDC+Lock. Detection bar, not proof. HOLE_INVENTORY ACTIVE means honest remaining holes (CUT), not closed holes.
