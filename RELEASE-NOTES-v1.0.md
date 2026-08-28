# YOYO v1.0.0 Release Notes

**Date:** 2026-08-29  
**Tag:** `v1.0.0`  
**Commit:** `8b7151cdf287abc9c738c36aef8e582c76ecf627`

---

## One-line pitch

**YOYO v1.0 finalizes host-hole SCOPE-CUT (closed=0 cut=7) and nails detection-only RELEASE wording — contracting false Thompson / fake-closed claims without claiming proof, YOYO-built runtime, or hole closure.**

---

## What this release is

YOYO v1.0 is the **ROADMAP endpoint**: it promotes the v0.9 hole inventory to **SCOPE-CUT FINAL**, ships a machine-checkable **detection banlist**, keeps v0.9 gates green, and publishes honest RELEASE docs. It is not a pivot to general application development, and it does **not** invent Stage 17.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs — a practical **detection** bar, **not** Thompson immunity. **HOLE_INVENTORY_V10 status=FINAL** · **closed=0 cut=7**. **Still Rust runtime + LoadLibrary/libdl.** **Seed still Rust-emitted.** Full `.text` peer compare **DIFF** (expected). Stub OS remain stub. Comparable EQUAL = selfhost-body window only.

**Cites:** `SCOPE-CUT-v1.0-hole-inventory.md` (FINAL) · `DETECTION-BANLIST-v1.0.md` (ACTIVE).

---

## Included in v1.0

| Area | v1.0 includes |
|------|----------------|
| **SCOPE-CUT FINAL** | OW-\* / REL-\* each `FINAL_HOLE … CLOSED\|CUT`; `SCOPE-CUT-v1.0-hole-inventory.md`; `stage16-scope-cut-finalize.ps1` |
| **Detection banlist** | `DETECTION-BANLIST-v1.0.md` + `stage16-detection-wording.ps1`; RELEASE cites seven **CUT** |
| **v0.9 baseline** | hole inventory · pre-run · Lock harden Decision #25 · seed/link · parity · three-peer · selfhost-body · pure M4 · fullbody · lock · gen12 — still green |

### Remaining CUT (honest — not CLOSED)

| ID | Disposition |
|----|-------------|
| **OW-H00** | **CUT** |
| **OW-STUB** | **CUT** |
| **OW-RT** | **CUT** |
| **OW-IAT** | **CUT** |
| **OW-SEED** | **CUT** |
| **REL-FULLTEXT** | **CUT** |
| **REL-STUBOS** | **CUT** |

### Trust-chain anchors

| Monitor | Value |
|---------|-------|
| **Lock pin** | `0275802d…` (Decision #25, **unchanged** — no Relock) |
| **HOLE_INVENTORY_V10** | **FINAL** · closed=0 · cut=7 · full `.text` DIFF · body EQUAL **17805** · stub_nz **159** · dll **154624** |
| **gen12 `.text` (Win)** | **`d782166d…`** · **18432** bytes |
| **runtime.dll** | **154624** B (still Rust-built; **OW-RT CUT**) |
| **Linux trampoline** | **9768** B (still libdl host path) |

---

## Explicitly NOT in v1.0

| Item | Status |
|------|--------|
| **Holes closed** | All seven remain **CUT** (inventory FINAL; closed=0) — do **not** claim fully closed / 洞已全关 |
| **Rust runtime still embedded** | `yoyo_runtime.dll` / `.so` still host-built; **OW-RT** |
| **YOYO-built runtime** | Deferred — still Rust cdylib |
| **LoadLibrary / libdl removed** | Still present (**OW-IAT**) |
| **Seed no longer Rust-emitted** | Still emitted by Rust `yoyo.exe` (**OW-SEED**) |
| **Full `.text` three-peer EQUAL** | Only selfhost-body window EQUAL; full `.text` DIFF under CUT |
| **Thompson-proof / C replacement** | Forbidden claims |
| **Stage 17 feature track** | ROADMAP endpoint — none |
| **MCU / Morph as main track** | OUT |

---

## Pre-publish gates (all exit 0)

```powershell
cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage14-lock-harden.ps1 -SkipBuild
.\scripts\stage16-v09-regress.ps1 -SkipBuild
# (embeds stage15/14/13–9 + all/lock/gen12/fullbody + Stage 16 A/B + WSL)
```

**Stage 16-D re-verify:** Lock PINNED · `stage16-v09-regress.ps1 -SkipBuild` ALL_GREEN (stamp **02:14:21**).

---

## North star reminder

**打破后门魔咒** — FINAL SCOPE-CUT + detection banlist under DDC+Lock. Detection bar, not proof. FINAL inventory with cut=7 means honest remaining holes (CUT), not closed holes.
