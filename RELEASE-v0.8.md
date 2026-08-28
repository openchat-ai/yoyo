# YOYO v0.8 — Release Boundary (Owner One-Pager)

> **Rule:** Only v0.8 scope may be published under a v0.8 tag. Anything labeled **ROADMAP**, **EXPERIMENTAL**, or stored as temp/debug artifacts **must not ship** with a v0.8 release.
>
> **Sources:** `SCOPE-v0.8.md`, `STAGE14_OWNER_CHECKLIST.md`, `SCOPE-CUT-v0.8-outside-window.md`, `RELEASE-v0.7.md`, `BACKEND_SUPPORT.md` · baseline 2026-08-29.

---

## North star: 打破后门魔咒

YOYO v0.8 **nails the outside-window blind zone into an honest SCOPE-CUT** and **thickens Lock pin / Relock discipline** after v0.7 — so **full `.text` DIFF cannot silently count as “green enough,” and Lock drift cannot silently rewrite the pin**.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs. That is a practical trust bar — **not** Thompson immunity or proof of purity. **SCOPE-CUT status=ACTIVE.** **Still Rust-compiled runtime + host LoadLibraryA / libdl** (not YOYO-built). **Seed is still Rust-emitted** (`yoyo.exe` host). Full `.text` peer compare **DIFF** (expected). Stub OS remain stub (not production I/O). Comparable EQUAL remains **selfhost-body window only** (17805 B).

---

## What IS in v0.8

**Product identity (honest):** An **auditable x86-64 compiler ISA** that keeps the v0.7 seed/link host / cross-platform parity / three-peer I/O / selfhost-body / LoadLibrary-host / Win+Linux pure-M4 baseline, then **documents and machine-gates outside-window bytes as SCOPE-CUT**, and **fail-closes Lock pin / Relock discipline**.

### Core deliverables (increment over v0.7)

| Area | v0.8 includes |
|------|----------------|
| **Outside-window SCOPE-CUT (Stage 14-A)** | `SCOPE-CUT-v0.8-outside-window.md` + `stage14-outside-window-scope-cut.ps1` (+ `stage14-scope-cut` alias); OW-H00 / OW-STUB / OW-RT / OW-IAT / OW-SEED pinned; body EQUAL + full `.text` DIFF → `SCOPE_CUT status=ACTIVE` |
| **Lock harden (Stage 14-B)** | `stage14-lock-harden.ps1` (+ `stage14-lock` alias); Decision #25 nail; drift → `RELOCK_REQUIRED` (no silent lock rewrite / no auto-Relock) |
| **v0.7 regression retained (Stage 14-C)** | `stage14-v07-regress.ps1` — serial stage13/12/11/10/9 + all/lock/gen12/fullbody + A/B + WSL; post-build **zero cargo** (`-SkipBuild` named switch; PS5.1 array splat forbidden); `driver.lock` concurrency guard |
| **v0.7 baseline retained** | seed/link host · cross-platform parity · three-peer I/O · selfhost-body · LoadLibrary/libdl · golden · backends · ddc · lock · gen12 · fullbody · H_00 · JS/asm peer · Win/Linux pure M4 |

### Trust-chain anchors (documented SHA)

| Monitor | Value | Notes |
|---------|-------|-------|
| **Lock pin (`yoyo.ty`)** | `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` | Decision #25 — **unchanged** (Stage 14 did not edit locked source) |
| **gen12 / fullbody `.text` (Win PE)** | SHA prefix **`d782166d`** · full `d782166dcb8a9c5de0bb8401203333e436ddc196af3b0a6145a66b5104b61568` | **18432**-byte compared window |
| **Selfhost-body window** | **17805** B EQUAL (JS=Rust=asm) | Skips H_00 entry slot; full `.text` DIFF (SCOPE-CUT ACTIVE) |
| **Embedded runtime.dll** | size **154624** (v0.5+) | Still Rust-built; **OW-RT CUT**; outside gen12 / selfhost-body windows |
| **H_00 extract stub** | `stub_tail_nonzero` **159** B | **OW-STUB CUT** (Rust-only; outside three-peer EQUAL) |
| **Linux trampoline** | size **9768** (v0.5+) | Still host libdl path |

### Lock / Relock (v0.8 graduation) — Decision

Stage 14 **A/B/C/D did not modify `yoyo/projects/yoyo.ty`**. Trust gains came from **SCOPE-CUT observe + gate**, **Lock harden discipline**, and **v0.7 regression harness** — not a source-body edit. Therefore:

- **No Relock required** — Decision #25 pin remains authoritative
- **Verified 2026-08-29:** `stage14-lock-harden.ps1 -SkipBuild` · `LOCK_HARDEN status=PINNED decision=25 relock=NO ty_eq_lock=YES` · `verify-lock-pin.ps1` exit 0 · `stage14-v07-regress.ps1 -SkipBuild` ALL_GREEN · Stage 14-D accept
- v0.8 graduation documents **outside-window SCOPE-CUT + Lock harden**; the Lock pin still locks the **788-handler source artifact**

### A/B/C trust gains (one line each)

| Door | Trust gain |
|------|------------|
| **A** | Outside-window bytes cannot stay an unobserved DIFF blind zone — SCOPE-CUT ACTIVE with machine ceilings + markers |
| **B** | Lock pin cannot silently drift — Decision #25 nailed; mismatch → `RELOCK_REQUIRED` |
| **C** | v0.7 gates remain green — SCOPE-CUT / Lock expansion does not buy regressions |

### Machine-checkable gates (all must exit 0 before publish)

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run --release -- test all
cargo run --release -- test lock
cargo run --release -- test gen12
cargo run --release -- test fullbody

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
node .\scripts\verify-yoyo-ty.mjs
.\scripts\stage13-link-host.ps1
.\scripts\stage13-cross-platform-parity.ps1
.\scripts\stage13-v06-regress.ps1
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage12-v05-regress.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage14-outside-window-scope-cut.ps1
.\scripts\stage14-lock-harden.ps1
.\scripts\stage14-v07-regress.ps1
.\scripts\stage5-win-selfhost.ps1
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

**Stage 14-D re-verify (2026-08-29):** `stage14-lock-harden.ps1 -SkipBuild` exit 0 · `stage14-v07-regress.ps1 -SkipBuild` ALL_GREEN (serial; stamp 01:14:12; no parallel cargo).

### Docs that belong in v0.8

- `SCOPE-v0.8.md` — v0.8 boundary one-pager (graduated)
- `SCOPE-CUT-v0.8-outside-window.md` — honest outside-window CUT inventory
- `STAGE14_OWNER_CHECKLIST.md` — Stage 14 A→D graduation board
- `RELEASE-v0.8.md` — this file
- `RELEASE-NOTES-v0.8.md` — short external notes
- Pinned artifacts unchanged: `yoyo/tests/yoyo.ty.lock` (Decision #25)

---

## What MUST NOT be published / claimed

### Still OUT (ROADMAP / later)

| Item | Why OUT |
|------|---------|
| **MCU/chip as main product track** | `custom-mcu` scaffold only |
| **C/Rust/Go replacement** | No struct/GC/async/module system |
| **Thompson-proof / DDC ⇒ provably correct** | Forbidden claim |
| **G06+ full golden suite** | Beyond v0.8 conformance |
| **macOS production gate** | MAY work; not required for v0.8 graduation |
| **YOYO-built runtime (non-Rust)** | Still Rust-compiled DLL/`.so` — **OW-RT CUT** |
| **Hole inventory close-or-CUT (full list)** | v0.9 theme |
| **Full `.text` three-peer EQUAL** | Selfhost-body window EQUAL only; SCOPE-CUT ACTIVE |
| **Seed no longer Rust-emitted** | Still `yoyo.exe` host emits seed (**OW-SEED CUT**) |
| **TheoryManifest / CDS theater** | N.4.1 FORBIDDEN |

### Remaining surface (honest — not blocking v0.8; SCOPE-CUT ACTIVE)

| Item | Status |
|------|--------|
| **SCOPE-CUT ACTIVE** | full `.text` DIFF; body window EQUAL 17805; OW-H00 / OW-STUB / OW-RT / OW-IAT / OW-SEED |
| **Embedded Rust runtime still present** | `yoyo_runtime.dll` / `libyoyo_runtime.so` still Rust-built; **OW-RT**; bytes outside gen12 / selfhost-body windows |
| **LoadLibrary / libdl host trampoline** | H_00 still extracts + loads via host LoadLibraryA / libdl — **OW-IAT** |
| **Full `.text` peer DIFF** | H_00 entry slot / Rust-only extract stub / IAT / embedded runtime — expected under CUT |
| **Non-Win/Linux stub OS** | Plan9/FreeBSD/Haiku/Serenity production I/O still stub (honest fork pinned) |
| **Seed still Rust-emitted** | Pure M4 still seeds via Rust-built `yoyo link` / `bootstrap` — **OW-SEED** |

### Misleading claims — forbidden in v0.8 release notes

Do **not** publish wording that implies:

- “Thompson-proof” or “immune to compiler backdoors”
- DDC covers **every** byte in genN PE / full `.text` three-peer EQUAL
- Outside-window holes are **closed** (they are **CUT**, not closed)
- Runtime is YOYO-built / free of Rust host trust
- LoadLibrary / libdl host path is gone
- Seed is no longer host-emitted / Thompson-closed at seed
- Stub OS are production I/O
- v0.8 is a daily-use application language or C replacement

### Temp / debug artifacts — never publish

```
scripts/_stage8-*/
scripts/_stage9-*/
scripts/_stage10*/
scripts/_stage11*/
scripts/_stage12*/
scripts/_stage13*/
scripts/_stage14*/
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

Stage 14-D quick re-verify: `stage14-lock-harden.ps1 -SkipBuild` · `stage14-v07-regress.ps1 -SkipBuild` — ALL_GREEN 2026-08-29 01:14:12. Prior A/B/C gates ALL_GREEN.

### 2. Confirm pin integrity

- `yoyo/tests/yoyo.ty.lock` SHA matches live `yoyo/projects/yoyo.ty`
- Pin: `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` (Decision #25, unchanged — **No Relock**)

### 3. Stage 14 four doors

- [x] A — outside-window SCOPE-CUT draft
- [x] B — Lock harden
- [x] C — v0.7 regression retained
- [x] D — v0.8 graduation gate + docs

### 4. Release notes honesty pass

- [x] No ROADMAP items listed as “done in v0.8”
- [x] DDC described as **detection**, not proof
- [x] SCOPE-CUT ACTIVE + remaining surface (Rust runtime / LoadLibrary+libdl / full `.text` DIFF / stub OS / seed still Rust-emitted) noted
- [x] gen12 window (**18432** bytes, `d782166d`) + selfhost-body (**17805** B EQUAL) documented
- [x] No temp `_stage*` / `_tmp*` / `.c11_*` dirs in artifact

---

## One-line pitch (external)

**YOYO v0.8 nails outside-window bytes into an honest SCOPE-CUT and thickens Lock pin / Relock discipline — contracting DIFF blind zones without claiming Thompson proof, YOYO-built runtime, full `.text` EQUAL, or replacing C.**

---

*Maintainer: update when Stage 14 gates or trust-chain SHA monitors change. v0.8 graduation: 2026-08-29 · Stage 14 A/B/C/D all green.*
