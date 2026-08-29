# YOYO v1.0 鈥?Release Boundary (Owner One-Pager)

> **Status:** graduated  
> **Rule:** Only v1.0 scope may be published under a v1.0 tag. Anything labeled **ROADMAP**, **EXPERIMENTAL**, or stored as temp/debug artifacts **must not ship** with a v1.0 release.
>
> **Sources:** `SCOPE-v1.0.md`, `STAGE16_OWNER_CHECKLIST.md`, `SCOPE-CUT-v1.0-hole-inventory.md`, `DETECTION-BANLIST-v1.0.md`, `RELEASE-v0.9.md` 路 baseline 2026-08-29.

---

## North star: 鎵撶牬鍚庨棬榄斿拻

YOYO v1.0 **finalizes** the v0.9 hole inventory as **SCOPE-CUT FINAL** (machine CLOSED|CUT) and **nails outward detection-only wording** so RELEASE cannot claim Thompson proof or fake hole closure.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs. That is a practical **detection** bar 鈥?**not** Thompson immunity or proof of purity. **HOLE_INVENTORY_V10 status=FINAL** 路 **closed=0 cut=7**. **Still Rust-compiled runtime (cwd sidecar; no exact embed) + host LoadLibraryA / libdl** (not YOYO-built). **Seed is still Rust-emitted** (`yoyo.exe` host). Full `.text` peer compare **DIFF** (expected). Stub OS remain stub (not production I/O). Comparable EQUAL remains **selfhost-body window only** (17805 B).

**Banlist cite:** `DETECTION-BANLIST-v1.0.md`锛圓CTIVE锛?  
**Inventory cite:** `SCOPE-CUT-v1.0-hole-inventory.md`锛團INAL 路 closed=0 cut=7锛?

---

## What IS in v1.0

**Product identity (honest):** An **auditable x86-64 compiler ISA** that keeps the v0.9 inventory / pre-run / regression baseline, then **FINAL SCOPE-CUT** + **detection-only RELEASE wording** 鈥?remaining host holes stay **CUT**, not pretended CLOSED.

### Core deliverables (increment over v0.9)

| Area | v1.0 includes |
|------|----------------|
| **SCOPE-CUT FINAL (Stage 16-A)** | `SCOPE-CUT-v1.0-hole-inventory.md` + `stage16-scope-cut-finalize.ps1` (+ `stage16-a`); OW-\*/REL-\* each `FINAL_HOLE 鈥?disposition=CLOSED\|CUT`; **closed=0 cut=7** 路 `HOLE_INVENTORY_V10 status=FINAL` |
| **Detection wording (Stage 16-B)** | `DETECTION-BANLIST-v1.0.md` + `stage16-detection-wording.ps1` (+ `stage16-b`); RELEASE cites CUT list; banned Thompson-proof / fully closed / fake EQUAL claims machine-checked |
| **v0.9 regression retained (Stage 16-C)** | `stage16-v09-regress.ps1` (+ `stage16-c`); serial stage15 A/B/C + stage14鈥? + all/lock/gen12/fullbody + Stage 16 A/B + WSL; named `-SkipBuild`; **zero parallel cargo** |
| **v0.9 baseline retained** | hole inventory 路 pre-run 路 v0.8 regress 路 Lock Decision #25 路 selfhost-body 路 LoadLibrary/libdl 路 gen12 路 fullbody 路 Win/Linux pure M4 |

### Remaining CUT list (must ship in RELEASE 鈥?from Stage 16-A FINAL)

| ID | Disposition | Notes |
|----|-------------|-------|
| **OW-H00** | **CUT** | H_00 entry slot; full `.text` DIFF; body skips slot |
| **OW-STUB** | **CUT** | Rust LoadLibrary stub_tail_nonzero (obs. 235; PE export walk) |
| **OW-RT** | **CUT** | Sidecar Rust runtime (Win DLL **141312** / Linux `.so`; **no exact embed** either OS) |
| **OW-IAT** | **CUT** | LoadLibraryA / libdl still present; **GetProcAddress absent** (in-process PE export walk) |
| **OW-SEED** | **CUT** | Seed still Rust-emitted (`yoyo.exe`); emitter+seed sha256_prefix + path=h00 pinned |
| **REL-FULLTEXT** | **CUT** | full `.text` not a graduation EQUAL claim |
| **REL-STUBOS** | **CUT** | Plan9/FreeBSD/Haiku/Serenity stub 鈥?not production I/O |

### Trust-chain anchors (documented SHA)

| Monitor | Value | Notes |
|---------|-------|-------|
| **Lock pin (`yoyo.ty`)** | `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` | Decision #25 鈥?**unchanged** (Stage 16 did not edit locked source) |
| **gen12 / fullbody `.text` (Win PE)** | SHA prefix **`84a8c1c9`** · full `84a8c1c9d85ca2765a893f0d3840446b57b50fa534e57ef083feaa3e931a2422` | **18432**-byte compared window |
| **Selfhost-body window** | **17805** B EQUAL (JS=Rust=asm) | Skips H_00 entry slot; full `.text` DIFF (inventory FINAL+CUT) |
| **Runtime.dll (sidecar)** | size **141312** | Still Rust-built; **OW-RT CUT**; **no exact embed** |
| **H_00 LoadLibrary stub** | `stub_tail_nonzero` **235** B | **OW-STUB CUT** (Rust-only; PE export walk; outside three-peer EQUAL) |
| **Hole inventory** | **FINAL** 路 closed=0 路 cut=7 | All seven **CUT** (honest; no fake CLOSED) |
| **Detection banlist** | **ACTIVE** | Gate: `stage16-detection-wording.ps1` |
| **Linux trampoline** | size **9768** (v0.5+) | Still host libdl path |

### Lock / Relock (v1.0 graduation) 鈥?Decision

Stage 16 **A/B/C/D did not modify `yoyo/projects/yoyo.ty`**. Trust gains came from **FINAL SCOPE-CUT**, **detection banlist + RELEASE wording**, and **v0.9 regression harness** 鈥?not a source-body edit. Therefore:

- **No Relock required** 鈥?Decision #25 pin remains authoritative
- **Verified 2026-08-29:** `verify-lock-pin.ps1` exit 0 路 `stage14-lock-harden.ps1 -SkipBuild` 路 `LOCK_HARDEN status=PINNED decision=25 relock=NO ty_eq_lock=YES` 路 `stage16-v09-regress.ps1 -SkipBuild` ALL_GREEN (stamp **02:14:21**) 路 Stage 16-D accept
- v1.0 graduation documents **HOLE_INVENTORY_V10 FINAL (7 CUT) + detection banlist**; the Lock pin still locks the **788-handler source artifact**

### A/B/C trust gains (one line each)

| Door | Trust gain |
|------|------------|
| **A** | v0.9 ACTIVE inventory becomes v1.0 FINAL SCOPE-CUT 鈥?each OW-\*/REL-\* machine `CLOSED\|CUT` (here all CUT) |
| **B** | Outward RELEASE without claiming Thompson-proof / fully closed / fake EQUAL 鈥?banlist machine-checked |
| **C** | v0.9 gates remain green 鈥?FINAL SCOPE-CUT / banlist expansion does not buy regressions |

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
.\scripts\stage16-scope-cut-finalize.ps1 -SkipBuild
.\scripts\stage16-detection-wording.ps1 -SkipBuild
.\scripts\stage16-v09-regress.ps1 -SkipBuild
```

**Stage 16-D re-verify (2026-08-29):** Lock PINNED 路 `stage16-v09-regress.ps1 -SkipBuild` ALL_GREEN (serial; stamp 02:14:21; no parallel cargo).

### Docs that belong in v1.0

- `SCOPE-v1.0.md` 鈥?v1.0 boundary one-pager (graduated)
- `SCOPE-CUT-v1.0-hole-inventory.md` 鈥?FINAL CLOSED|CUT inventory (closed=0 cut=7)
- `DETECTION-BANLIST-v1.0.md` 鈥?outward forbidden claims (ACTIVE)
- `STAGE16_OWNER_CHECKLIST.md` 鈥?Stage 16 A鈫扗 graduation board
- `RELEASE-v1.0.md` 鈥?this file
- `RELEASE-NOTES-v1.0.md` 鈥?short external notes
- Upstream retained: `RELEASE-v0.9.md` 路 `SCOPE-CUT-v0.9-hole-inventory.md`
- Pinned artifacts unchanged: `yoyo/tests/yoyo.ty.lock` (Decision #25)

---

## What MUST NOT be published / claimed

### Still OUT (ROADMAP / later / forbidden)

| Item | Why OUT |
|------|---------|
| **MCU/chip as main product track** | `custom-mcu` scaffold only |
| **C/Rust/Go replacement** | No struct/GC/async/module system |
| **Thompson-proof / DDC 鈬?provably correct** | Forbidden claim 鈥?see banlist |
| **G06+ full golden suite** | Beyond v1.0 conformance |
| **macOS production gate** | MAY work; not required for v1.0 graduation |
| **YOYO-built runtime (non-Rust)** | Still Rust-compiled DLL/`.so` 鈥?**OW-RT CUT** |
| **Full `.text` three-peer EQUAL** | Selfhost-body window EQUAL only; inventory FINAL+CUT |
| **Seed no longer Rust-emitted** | Still `yoyo.exe` host emits seed (**OW-SEED CUT**) |
| **Stage 17 feature track** | ROADMAP endpoint 鈥?no invent Stage 17 |
| **TheoryManifest / CDS theater** | N.4.1 FORBIDDEN |

### Remaining surface (honest 鈥?HOLE_INVENTORY_V10 FINAL; not pretended CLOSED)

| Item | Status |
|------|--------|
| **HOLE_INVENTORY_V10 FINAL** | closed=0 cut=7; full `.text` DIFF; body window EQUAL 17805 |
| **OW-H00 / OW-STUB** | H_00 slot + Rust extract stub (**CUT**) |
| **OW-RT** | Sidecar Rust `yoyo_runtime.dll` no exact embed (**CUT**) |
| **OW-IAT** | LoadLibraryA / libdl host trampoline (**CUT**; GetProcAddress dropped) |
| **OW-SEED** | Seed still Rust-emitted (**CUT**) |
| **REL-FULLTEXT / REL-STUBOS** | full `.text` not graduation EQUAL; stub OS not production I/O (**CUT**) |
| **Non-Win/Linux stub OS** | Plan9/FreeBSD/Haiku/Serenity production I/O still stub |

### Misleading claims 鈥?forbidden in v1.0 release notes

Do **not** publish wording that implies:

- 鈥淭hompson-proof鈥?or 鈥渋mmune to compiler backdoors鈥?- Holes are **fully closed** / 娲炲凡鍏ㄥ叧 / closed=7 (they remain **CUT**, closed=0)
- DDC covers **every** byte / full `.text` three-peer EQUAL as graduation
- Runtime is YOYO-built / free of Rust host trust
- LoadLibrary / libdl host path is gone
- Seed is no longer host-emitted
- Stub OS are production I/O
- v1.0 is a daily-use application language or C replacement

**Machine banlist:** `DETECTION-BANLIST-v1.0.md` (`BAN id=THOMPSON|FULLY_CLOSED|FAKE_EQUAL|鈥).

### Temp / debug artifacts 鈥?never publish

```
scripts/_stage8-*/
scripts/_stage9-*/
scripts/_stage10*/
scripts/_stage11*/
scripts/_stage12*/
scripts/_stage13*/
scripts/_stage14*/
scripts/_stage15*/
scripts/_stage16*/
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

Stage 16-D re-verify: `verify-lock-pin.ps1` 路 `stage14-lock-harden.ps1 -SkipBuild` PINNED 路 `stage16-v09-regress.ps1 -SkipBuild` ALL_GREEN 2026-08-29 02:14:21. Prior A/B/C gates ALL_GREEN.

### 2. Confirm pin integrity

- `yoyo/tests/yoyo.ty.lock` SHA matches live `yoyo/projects/yoyo.ty`
- Pin: `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` (Decision #25, unchanged 鈥?**No Relock**)

### 3. Stage 16 four doors

- [x] A 鈥?SCOPE-CUT FINAL (closed=0 cut=7)
- [x] B 鈥?detection wording / banlist
- [x] C 鈥?v0.9 regression retained
- [x] D 鈥?v1.0 graduation gate + docs + tag

### 4. Release notes honesty pass

- [x] No ROADMAP items listed as 鈥渄one in v1.0鈥?- [x] DDC described as **detection**, not proof
- [x] HOLE_INVENTORY_V10 FINAL + remaining OW-\* **CUT** surface noted (no fake CLOSED)
- [x] gen12 window (**18432** bytes, `84a8c1c9`) + selfhost-body (**17805** B EQUAL) documented
- [x] Banlist cited; seven CUT ids listed
- [x] No temp `_stage*` / `_tmp*` / `.c11_*` dirs in artifact

---

## One-line pitch (external)

**YOYO v1.0 finalizes host-hole SCOPE-CUT (closed=0 cut=7) and nails detection-only RELEASE wording 鈥?contracting false Thompson / fake-closed claims without claiming proof, YOYO-built runtime, or hole closure.**

---

*Maintainer: update when Stage 16 gates or trust-chain SHA monitors change. v1.0 graduation: 2026-08-29 路 Stage 16 A/B/C/D all green 路 ROADMAP endpoint (no Stage 17).*
