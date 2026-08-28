# YOYO v0.4.0 Release Notes

**Date:** 2026-08-28  
**Tag:** `v0.4.0`  
**Commit:** `174f113002e1d13b20229266e949fa1c2b8e6875`

---

## One-line pitch

**YOYO v0.4 shrinks the remaining host-trust face â smaller embedded runtime, Linux pure M4 without `--selfhost`, asm I/O peer parity â expanding three-chain DDC + Lock coverage without claiming Thompson proof or replacing C.**

---

## What this release is

YOYO v0.4 **closes the largest honest remaining holes after v0.3** and puts more self-host observability under DDC + Lock. It is not a pivot to general application development.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs â a practical trust bar, **not** Thompson immunity.

---

## Included in v0.4

| Area | v0.4 includes |
|------|----------------|
| **Runtime.dll surface** | DLL **485888â231936** B fail-closed (`stage10-runtime-surface.ps1`); gen12 SHA `43ffde58â¦` |
| **Linux pure M4** | gen1âgen4 without `bootstrap --selfhost`; gen4â¡gen3_direct EQUAL (`stage10-linux-pure-m4.sh`) |
| **asm peer I/O parity** | win32 `0x20/0x50/0x51` byte-equal Rust/JS; `stage10-asm-peer-io.ps1` |
| **v0.3 baseline** | H_00 Â· JS peer Â· Win pure M4 Â· fullbody Â· lock Â· gen12 â still green |

### Trust-chain anchors

| Monitor | Value |
|---------|-------|
| **Lock pin** | `0275802dâ¦` (Decision #25, **unchanged** â no Relock) |
| **gen12 `.text` (Win)** | **`43ffde58â¦`** Â· **18432** bytes |
| **runtime.dll** | **231936** B (still Rust-built; outside gen12 window) |
| **Linux M4 ELF** | `085d07d4â¦` Â· 704512 B (pure path, no `--selfhost`) |

---

## Explicitly NOT in v0.4

| Item | Status |
|------|--------|
| **Rust runtime still embedded** | `yoyo_runtime.dll` / `.so` still host-built; outside gen12 `.text` compare |
| **YOYO-built runtime** | Deferred to v0.5+ |
| **Thompson-proof / C replacement** | Forbidden claims |
| **MCU / Morph as main track** | OUT |

---

## Pre-publish gates (all exit 0)

```powershell
cd F:\yoyo\yoyo-rusterifier
cargo run --release -- test all
cargo run --release -- test lock
cargo run --release -- test gen12
cargo run --release -- test fullbody

cd F:\yoyo
.\scriptserify-lock-pin.ps1
node .\scriptserify-yoyo-ty.mjs
.\scripts\stage10-runtime-surface.ps1
.\scripts\stage10-asm-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage5-win-selfhost.ps1
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

---

## North star reminder

**æç ´åé¨é­å** â more bytes under DDC+Lock, fewer "green only because host wrapped it" paths. Detection bar, not proof.
