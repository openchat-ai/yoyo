# YOYO v0.3.0 Release Notes

**Date:** 2026-08-28  
**Tag:** _(not tagged yet — owner decides)_  
**Commit:** `d7d5f443fec7a65539ab694463497a7488c34a15`

---

## One-line pitch

**YOYO v0.3 closes the biggest remaining self-host trust holes — H_00 pure path, JS I/O DDC parity, Win M4 without host `--selfhost` — expanding three-chain DDC + Lock coverage without claiming Thompson proof or replacing C.**

---

## What this release is

YOYO v0.3 **shrinks host-trust holes left after v0.2** and puts more self-host bytes under DDC + Lock monitoring. It is not a pivot to general application development.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs — a practical trust bar, **not** Thompson immunity.

---

## Included in v0.3

| Area | v0.3 includes |
|------|----------------|
| **gen1 H_00 pure path** | Zero-arg gen1 → `output.exe` without genNrt entry wrap; `stage9-gen1-h00-selfhost.ps1` |
| **JS peer I/O parity** | Production win32 `0x20/0x50/0x51` byte-equal Rust; `stage9-js-peer-io.ps1` |
| **Win pure M4** | gen1→gen4 via H_00, no `bootstrap --selfhost`; gen4≡gen3_direct DDC EQUAL |
| **v0.2 baseline** | fullbody · stage8 genNrt M4 · lock · gen12 — still green |

### Trust-chain anchors

| Monitor | Value |
|---------|-------|
| **Lock pin** | `0275802d…` (Decision #25, **unchanged** — no Relock) |
| **gen12 `.text` (Win)** | **`b609a735…`** · **18432** bytes |
| **Linux M4 ELF** | `dab59f96…` (still via `--selfhost`) |

---

## Explicitly NOT in v0.3

| Item | Status |
|------|--------|
| **Embedded runtime outside DDC window** | `yoyo_runtime.dll` still Rust-built; outside gen12 `.text` compare |
| **Linux H_00 pure M4** | Still uses `bootstrap --selfhost` |
| **asm peer real I/O** | Still movabs+store stubs |
| **Thompson-proof / C replacement** | Forbidden claims |
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
.\scripts\stage9-gen1-h00-selfhost.ps1
.\scripts\stage9-js-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage8-extended-selfhost.ps1
```

---

## North star reminder

**打破后门魔咒** — more bytes under DDC+Lock, fewer “green only because host wrapped it” paths. Detection bar, not proof.
