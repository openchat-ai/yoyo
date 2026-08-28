# YOYO v0.5.0 Release Notes

**Date:** 2026-08-28  
**Tag:** `v0.5.0`  
**Commit:** `39b32953be992b564e60a07d0fc5ebc9795c9964`

---

## One-line pitch

**YOYO v0.5 shrinks the remaining host-trust face again — thinner embedded Rust runtime, smaller LoadLibrary/libdl host surface — expanding fail-closed observability under DDC + Lock without claiming Thompson proof, YOYO-built runtime, or replacing C.**

---

## What this release is

YOYO v0.5 **closes the largest honest remaining holes after v0.4** on the runtime / host-loader face and puts more observability under fail-closed gates + Lock. It is not a pivot to general application development.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs — a practical trust bar, **not** Thompson immunity. **Still Rust runtime + LoadLibrary/libdl.**

---

## Included in v0.5

| Area | v0.5 includes |
|------|----------------|
| **Thinner runtime** | DLL **231936→154624** B fail-closed (`stage11-runtime-surface.ps1` MAX 170000); `.so` **407232**; gen12 SHA `d782166d…` |
| **LoadLibrary / libdl host** | Win cwd-relative extract (IAT 5→3); Linux tramp **14464→9768**; `stage11-loadlibrary-host.ps1` |
| **v0.4 baseline** | H_00 · JS peer · Win/Linux pure M4 · asm I/O · fullbody · lock · gen12 — still green |

### Trust-chain anchors

| Monitor | Value |
|---------|-------|
| **Lock pin** | `0275802d…` (Decision #25, **unchanged** — no Relock) |
| **gen12 `.text` (Win)** | **`d782166d…`** · **18432** bytes |
| **runtime.dll** | **154624** B (still Rust-built; outside gen12 window) |
| **Linux trampoline** | **9768** B (still libdl host path) |

---

## Explicitly NOT in v0.5

| Item | Status |
|------|--------|
| **Rust runtime still embedded** | `yoyo_runtime.dll` / `.so` still host-built; outside gen12 `.text` compare |
| **YOYO-built runtime** | Deferred — Stage 11-A is thinner host face, not YOYO-built |
| **LoadLibrary / libdl removed** | Still present; surface shrunk + observed only |
| **Thompson-proof / C replacement** | Forbidden claims |
| **Three-peer full-body section-ddc** | v0.6+ |
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
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage10-runtime-surface.ps1
.\scripts\stage10-asm-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
```

---

## North star reminder

**打破后门魔咒** — more bytes under DDC+Lock / fail-closed host-face gates, fewer "green only because host wrapped it" paths. Detection bar, not proof.
