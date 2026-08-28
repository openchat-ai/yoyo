# YOYO v0.7.0 Release Notes

**Date:** 2026-08-29  
**Tag:** `v0.7.0`  
**Commit:** `(filled after graduation commit)`

---

## One-line pitch

**YOYO v0.7 puts seed/link host emission and Win/Linux (+ stub OS) parity under fail-closed DDC gates — contracting host/platform blind zones without claiming Thompson proof, YOYO-built runtime, full `.text` EQUAL, or replacing C.**

---

## What this release is

YOYO v0.7 **closes the largest honest remaining observation holes after v0.6** on seed/link host bypass and cross-platform parity, and keeps v0.6 gates green. It is not a pivot to general application development.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs — a practical trust bar, **not** Thompson immunity. **Still Rust runtime + LoadLibrary/libdl.** **Seed still Rust-emitted.** Full `.text` peer compare may still DIFF. Stub OS remain stub.

---

## Included in v0.7

| Area | v0.7 includes |
|------|----------------|
| **Seed/link host** | H_00 canonical seed; `link`≡`bootstrap`; fail-closed PE/ELF + markers; `stage13-link-host.ps1` |
| **Cross-platform parity** | Win+Linux under one gate; stub OS honesty pins; `stage13-cross-platform-parity.ps1` |
| **v0.6 baseline** | three-peer I/O · selfhost-body · LoadLibrary/libdl · pure M4 · fullbody · lock · gen12 — still green |

### Trust-chain anchors

| Monitor | Value |
|---------|-------|
| **Lock pin** | `0275802d…` (Decision #25, **unchanged** — no Relock) |
| **gen12 `.text` (Win)** | **`d782166d…`** · **18432** bytes |
| **Selfhost-body window** | **17805** B EQUAL (JS=Rust=asm) |
| **runtime.dll** | **154624** B (still Rust-built; outside compared windows) |
| **Linux trampoline** | **9768** B (still libdl host path) |

---

## Explicitly NOT in v0.7

| Item | Status |
|------|--------|
| **Rust runtime still embedded** | `yoyo_runtime.dll` / `.so` still host-built; outside gen12 / selfhost-body windows |
| **YOYO-built runtime** | Deferred — still Rust cdylib |
| **LoadLibrary / libdl removed** | Still present (v0.5 surface) |
| **Seed no longer Rust-emitted** | Still emitted by Rust `yoyo.exe` (A observes / fail-closes, does not eliminate) |
| **Full `.text` three-peer EQUAL** | Only selfhost-body window EQUAL; H_00 / extract stub / runtime still diverge |
| **Thompson-proof / C replacement** | Forbidden claims |
| **Outside-window / SCOPE-CUT** | v0.8+ |
| **MCU / Morph as main track** | OUT |

---

## Pre-publish gates (all exit 0)

```powershell
cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage13-v06-regress.ps1 -SkipBuild
# (embeds A+B + stage12-v05-regress: stage12/11/10/9 + all/lock/gen12/fullbody + WSL)
```

---

## North star reminder

**打破后门魔咒** — fewer seed/platform blind paths under DDC+Lock. Detection bar, not proof.
