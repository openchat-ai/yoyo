# YOYO v0.6.0 Release Notes

**Date:** 2026-08-28  
**Tag:** `v0.6.0`  
**Commit:** `f3c8b96335f0bad66a2f89bfc7289f82fe60c20d`

---

## One-line pitch

**YOYO v0.6 puts three-peer production I/O and a larger selfhost-body window under fail-closed DDC — expanding detection coverage without claiming Thompson proof, YOYO-built runtime, full `.text` EQUAL, or replacing C.**

---

## What this release is

YOYO v0.6 **closes the largest honest remaining observation holes after v0.5** on three-peer I/O and selfhost-body section-ddc, and keeps v0.5 gates green. It is not a pivot to general application development.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs — a practical trust bar, **not** Thompson immunity. **Still Rust runtime + LoadLibrary/libdl.** Full `.text` peer compare may still DIFF.

---

## Included in v0.6

| Area | v0.6 includes |
|------|----------------|
| **Three-peer I/O** | win32+linux `0x20/0x50/0x51` **Rust=JS=asm**; `stage12-three-peer-io.ps1`; stub/unknown-OS pinned |
| **Selfhost body section-ddc** | three-peer window **17805** B EQUAL; `yoyo test body-ddc` / `diff --selfhost-body`; `stage12-selfhost-body-section-ddc.ps1` |
| **v0.5 baseline** | thinner runtime · LoadLibrary/libdl host · pure M4 · fullbody · lock · gen12 — still green |

### Trust-chain anchors

| Monitor | Value |
|---------|-------|
| **Lock pin** | `0275802d…` (Decision #25, **unchanged** — no Relock) |
| **gen12 `.text` (Win)** | **`d782166d…`** · **18432** bytes |
| **Selfhost-body window** | **17805** B EQUAL (JS=Rust=asm) |
| **runtime.dll** | **154624** B (still Rust-built; outside compared windows) |
| **Linux trampoline** | **9768** B (still libdl host path) |

---

## Explicitly NOT in v0.6

| Item | Status |
|------|--------|
| **Rust runtime still embedded** | `yoyo_runtime.dll` / `.so` still host-built; outside gen12 / selfhost-body windows |
| **YOYO-built runtime** | Deferred — still Rust cdylib |
| **LoadLibrary / libdl removed** | Still present (v0.5 surface) |
| **Full `.text` three-peer EQUAL** | Only selfhost-body window EQUAL; H_00 / extract stub / runtime still diverge |
| **Thompson-proof / C replacement** | Forbidden claims |
| **seed/link host main cut** | v0.7+ |
| **MCU / Morph as main track** | OUT |

---

## Pre-publish gates (all exit 0)

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run --release -- test all
cargo run --release -- test lock
cargo run --release -- test gen12
cargo run --release -- test fullbody

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage12-v05-regress.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
```

---

## North star reminder

**打破后门魔咒** — more selfhost / peer bytes under DDC+Lock, fewer “green only because stub / platform fork / unobserved body” paths. Detection bar, not proof.
