# YOYO v0.9 — Release Boundary (Owner One-Pager)

> **Rule:** Only v0.9 scope may be published under a v0.9 tag. Anything labeled **ROADMAP**, **EXPERIMENTAL**, or stored as temp/debug artifacts **must not ship** with a v0.9 release.
>
> **Sources:** `SCOPE-v0.9.md`, `STAGE15_OWNER_CHECKLIST.md`, `SCOPE-CUT-v0.9-hole-inventory.md`, `SCOPE-CUT-v0.8-outside-window.md`, `RELEASE-v0.8.md`, `BACKEND_SUPPORT.md` · baseline 2026-08-29.

---

## North star: 打破后门魔咒

YOYO v0.9 **enumerates every OW-\* / REL-\* hole as machine-checkable CLOSED|CUT**, and **adds a serial pre-run keep-green gate** after v0.8 — so a lump SCOPE-CUT cannot silently count as “closed,” and graduation cannot blind-fly past v0.8 regressions.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs. That is a practical trust bar — **not** Thompson immunity or proof of purity. **HOLE_INVENTORY status=ACTIVE** · **closed=0 cut=7**. **Still Rust-compiled runtime + host LoadLibraryA / libdl** (not YOYO-built). **Seed is still Rust-emitted** (`yoyo.exe` host). Full `.text` peer compare **DIFF** (expected). Stub OS remain stub (not production I/O). Comparable EQUAL remains **selfhost-body window only** (17805 B).

---

## What IS in v0.9

**Product identity (honest):** An **auditable x86-64 compiler ISA** that keeps the v0.8 outside-window SCOPE-CUT / Lock harden / v0.7 baseline, then **per-hole CLOSED|CUT inventory** and **serial pre-run keep-green** before graduation.

### Core deliverables (increment over v0.8)

| Area | v0.9 includes |
|------|----------------|
| **Hole inventory (Stage 15-A)** | `SCOPE-CUT-v0.9-hole-inventory.md` + `stage15-hole-inventory.ps1` (+ `stage15-a` alias); OW-H00 / OW-STUB / OW-RT / OW-IAT / OW-SEED / REL-FULLTEXT / REL-STUBOS each `disposition=CLOSED\|CUT`; **closed=0 cut=7** · `HOLE_INVENTORY status=ACTIVE` |
| **Pre-run keep-green (Stage 15-B)** | `stage15-prerun.ps1` (+ `stage15-keep-green` / `stage15-b`); serial: hole-inventory + stage14-v07-regress nested; `driver.lock`; named `-SkipBuild`; **zero parallel cargo** |
| **v0.8 regression retained (Stage 15-C)** | `stage15-v08-regress.ps1` (+ `stage15-c`); serial stage13–9 + all/lock/gen12/fullbody + stage14 A/B + hole-inventory + WSL; post-build **zero cargo** |
| **v0.8 baseline retained** | outside-window SCOPE-CUT · Lock harden Decision #25 · seed/link · parity · three-peer I/O · selfhost-body · LoadLibrary/libdl · golden · backends · ddc · lock · gen12 · fullbody · Win/Linux pure M4 |

### Trust-chain anchors (documented SHA)

| Monitor | Value | Notes |
|---------|-------|-------|
| **Lock pin (`yoyo.ty`)** | `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` | Decision #25 — **unchanged** (Stage 15 did not edit locked source) |
| **gen12 / fullbody `.text` (Win PE)** | SHA prefix **`d782166d`** · full `d782166dcb8a9c5de0bb8401203333e436ddc196af3b0a6145a66b5104b61568` | **18432**-byte compared window |
| **Selfhost-body window** | **17805** B EQUAL (JS=Rust=asm) | Skips H_00 entry slot; full `.text` DIFF (inventory ACTIVE) |
| **Embedded runtime.dll** | size **154624** (v0.5+) | Still Rust-built; **OW-RT CUT**; exact embed_off **85543** |
| **H_00 extract stub** | `stub_tail_nonzero` **159** B | **OW-STUB CUT** (Rust-only; outside three-peer EQUAL) |
| **Hole inventory** | **ACTIVE** · closed=0 · cut=7 | All OW-\* / REL-\* **CUT** (honest; no fake CLOSED) |
| **Linux trampoline** | size **9768** (v0.5+) | Still host libdl path |

### Lock / Relock (v0.9 graduation) — Decision

Stage 15 **A/B/C/D did not modify `yoyo/projects/yoyo.ty`**. Trust gains came from **per-hole inventory + gate**, **pre-run keep-green**, and **v0.8 regression harness** — not a source-body edit. Therefore:

- **No Relock required** — Decision #25 pin remains authoritative
- **Verified 2026-08-29:** `verify-lock-pin.ps1` exit 0 · `stage14-lock-harden.ps1 -SkipBuild` · `LOCK_HARDEN status=PINNED decision=25 relock=NO ty_eq_lock=YES` · `stage15-v08-regress.ps1 -SkipBuild` ALL_GREEN (stamp **01:40:31**) · Stage 15-D accept
- v0.9 graduation documents **hole inventory ACTIVE (7 CUT) + pre-run**; the Lock pin still locks the **788-handler source artifact**

### A/B/C trust gains (one line each)

| Door | Trust gain |
|------|------------|
| **A** | Holes cannot stay a lump CUT — each OW-\*/REL-\* is machine `CLOSED\|CUT` with fail-closed CLOSED rules |
| **B** | Graduation cannot blind-fly — one serial keep-green pre-run before release |
| **C** | v0.8 gates remain green — inventory / pre-run expansion does not buy regressions |

### Machine-checkable gates (all must exit 0 before publish)

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run --release -- test all
cargo run --release -- test lock
cargo run --release -- test gen12
cargo run --release -- test fullbody

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage14-lock-harden.ps1 -SkipBuild
.\scripts\stage15-hole-inventory.ps1 -SkipBuild
.\scripts\stage15-prerun.ps1 -SkipBuild
.\scripts\stage15-v08-regress.ps1 -SkipBuild
```

**Stage 15-D re-verify (2026-08-29):** Lock PINNED · `stage15-v08-regress.ps1 -SkipBuild` ALL_GREEN (serial; stamp 01:40:31; no parallel cargo).

### Docs that belong in v0.9

- `SCOPE-v0.9.md` — v0.9 boundary one-pager (graduated)
- `SCOPE-CUT-v0.9-hole-inventory.md` — per-hole CLOSED|CUT inventory
- `SCOPE-CUT-v0.8-outside-window.md` — upstream outside-window baseline (still ACTIVE)
- `STAGE15_OWNER_CHECKLIST.md` — Stage 15 A→D graduation board
- `RELEASE-v0.9.md` — this file
- `RELEASE-NOTES-v0.9.md` — short external notes
- Pinned artifacts unchanged: `yoyo/tests/yoyo.ty.lock` (Decision #25)

---

## What MUST NOT be published / claimed

### Still OUT (ROADMAP / later)

| Item | Why OUT |
|------|---------|
| **MCU/chip as main product track** | `custom-mcu` scaffold only |
| **C/Rust/Go replacement** | No struct/GC/async/module system |
| **Thompson-proof / DDC ⇒ provably correct** | Forbidden claim |
| **G06+ full golden suite** | Beyond v0.9 conformance |
| **macOS production gate** | MAY work; not required for v0.9 graduation |
| **YOYO-built runtime (non-Rust)** | Still Rust-compiled DLL/`.so` — **OW-RT CUT** |
| **Full `.text` three-peer EQUAL** | Selfhost-body window EQUAL only; inventory ACTIVE |
| **Seed no longer Rust-emitted** | Still `yoyo.exe` host emits seed (**OW-SEED CUT**) |
| **1.0 full close or SCOPE-CUT final** | Stage 16 / v1.0 |
| **TheoryManifest / CDS theater** | N.4.1 FORBIDDEN |

### Remaining surface (honest — not blocking v0.9; HOLE_INVENTORY ACTIVE)

| Item | Status |
|------|--------|
| **HOLE_INVENTORY ACTIVE** | closed=0 cut=7; full `.text` DIFF; body window EQUAL 17805 |
| **OW-H00 / OW-STUB** | H_00 slot + Rust extract stub (**CUT**) |
| **OW-RT** | Embedded Rust `yoyo_runtime.dll` exact embed (**CUT**) |
| **OW-IAT** | LoadLibraryA / libdl host trampoline (**CUT**) |
| **OW-SEED** | Seed still Rust-emitted (**CUT**) |
| **REL-FULLTEXT / REL-STUBOS** | full `.text` not graduation EQUAL; stub OS not production I/O (**CUT**) |
| **Non-Win/Linux stub OS** | Plan9/FreeBSD/Haiku/Serenity production I/O still stub |

### Misleading claims — forbidden in v0.9 release notes

Do **not** publish wording that implies:

- “Thompson-proof” or “immune to compiler backdoors”
- DDC covers **every** byte in genN PE / full `.text` three-peer EQUAL
- Holes are **closed** (they are **CUT**, inventory ACTIVE — closed=0)
- Runtime is YOYO-built / free of Rust host trust
- LoadLibrary / libdl host path is gone
- Seed is no longer host-emitted
- Stub OS are production I/O
- v0.9 is a daily-use application language or C replacement

### Temp / debug artifacts — never publish

```
scripts/_stage8-*/
scripts/_stage9-*/
scripts/_stage10*/
scripts/_stage11*/
scripts/_stage12*/
scripts/_stage13*/
scripts/_stage14*/
scripts/_stage15*/
scripts/_tmp*
.c11_*
_*.log
yoyo-rust/target-nosidecar/
yoyo-rust/target-selfhost-build/
yoyo-rust/target-runtime-z/
yoyo-rust/target-stage*/
yoyo-test/
```

---

## Pre-publish checklist

### 1. Verify green (exit code 0 everywhere)

Stage 15-D re-verify: `verify-lock-pin.ps1` · `stage14-lock-harden.ps1 -SkipBuild` PINNED · `stage15-v08-regress.ps1 -SkipBuild` ALL_GREEN 2026-08-29 01:40:31. Prior A/B/C gates ALL_GREEN.

### 2. Confirm pin integrity

- `yoyo/tests/yoyo.ty.lock` SHA matches live `yoyo/projects/yoyo.ty`
- Pin: `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` (Decision #25, unchanged — **No Relock**)

### 3. Stage 15 four doors

- [x] A — hole inventory CLOSED|CUT
- [x] B — pre-run keep-green
- [x] C — v0.8 regression retained
- [x] D — v0.9 graduation gate + docs

### 4. Release notes honesty pass

- [x] No ROADMAP items listed as “done in v0.9”
- [x] DDC described as **detection**, not proof
- [x] HOLE_INVENTORY ACTIVE + remaining OW-\* CUT surface noted
- [x] gen12 window (**18432** bytes, `d782166d`) + selfhost-body (**17805** B EQUAL) documented
- [x] No temp `_stage*` / `_tmp*` / `.c11_*` dirs in artifact

---

## One-line pitch (external)

**YOYO v0.9 enumerates every remaining host hole as machine CLOSED|CUT and adds serial pre-run keep-green — contracting blind graduation without claiming Thompson proof, YOYO-built runtime, hole closure, or replacing C.**

---

*Maintainer: update when Stage 15 gates or trust-chain SHA monitors change. v0.9 graduation: 2026-08-29 · Stage 15 A/B/C/D all green.*
