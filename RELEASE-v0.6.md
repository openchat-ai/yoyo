# YOYO v0.6 — Release Boundary (Owner One-Pager)

> **Rule:** Only v0.6 scope may be published under a v0.6 tag. Anything labeled **ROADMAP**, **EXPERIMENTAL**, or stored as temp/debug artifacts **must not ship** with a v0.6 release.
>
> **Sources:** `SCOPE-v0.6.md`, `STAGE12_OWNER_CHECKLIST.md`, `RELEASE-v0.5.md`, `BACKEND_SUPPORT.md` · baseline 2026-08-28.

---

## North star: 打破后门魔咒

YOYO v0.6 **expands three-peer observability** after v0.5 — production I/O paths across Rust/JS/asm, plus a fail-closed **selfhost-body section-ddc window** — so **more selfhost / peer bytes sit under DDC + Lock**, and **fewer paths stay green only because a stub, platform fork, or “whole `.text` DIFF” left the comparable body unobserved**.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs. That is a practical trust bar — **not** Thompson immunity or proof of purity. **Still Rust-compiled runtime + host LoadLibraryA / libdl** (not YOYO-built). Full `.text` peer compare may still DIFF (H_00 slot / extract stub / IAT / embedded runtime).

---

## What IS in v0.6

**Product identity (honest):** An **auditable x86-64 compiler ISA** that keeps the v0.5 thinner-runtime / LoadLibrary-host / Win+Linux pure-M4 / asm I/O baseline, then **closes residual three-peer I/O blind zones** and **puts a larger selfhost body window under section-ddc**.

### Core deliverables (increment over v0.5)

| Area | v0.6 includes |
|------|----------------|
| **Three-peer I/O (Stage 12-A)** | Fail-closed `stage12-three-peer-io.ps1` (embeds stage10-asm + stage9-js); win32+linux `0x20/0x50/0x51` **Rust=JS=asm** byte-equal; closes stage10 linux LOAD/WRITE ALLOC-only blind zone; stub G-SM-IO 17B; unknown OS → stub pinned |
| **Selfhost body section-ddc (Stage 12-B)** | `yoyo test body-ddc` + `yoyo diff --selfhost-body`; three-peer window EQUAL (**17805** B; startup + post-H_00; floor ≥17013); Rust stub_tail_nonzero=159 pinned; `stage12-selfhost-body-section-ddc.ps1` |
| **v0.5 regression retained (Stage 12-C)** | `stage12-v05-regress.ps1` (alias `stage12-regression.ps1`) — stage11/10/9 + fullbody/lock/gen12 + WSL linux-pure-m4 stay green |
| **v0.5 baseline retained** | thinner runtime · LoadLibrary/libdl host · golden · backends · ddc · lock · gen12 · fullbody · H_00 · JS/asm peer · Win/Linux pure M4 |

### Trust-chain anchors (documented SHA)

| Monitor | Value | Notes |
|---------|-------|-------|
| **Lock pin (`yoyo.ty`)** | `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` | Decision #25 — **unchanged** (Stage 12 did not edit locked source) |
| **gen12 / fullbody `.text` (Win PE)** | SHA prefix **`d782166d`** · full `d782166dcb8a9c5de0bb8401203333e436ddc196af3b0a6145a66b5104b61568` | **18432**-byte compared window |
| **Selfhost-body window** | **17805** B EQUAL (JS=Rust=asm) | Skips H_00 entry slot; full `.text` may still DIFF |
| **Embedded runtime.dll** | size **154624** (v0.5) | Still Rust-built; **outside** gen12 / selfhost-body windows |
| **Linux trampoline** | size **9768** (v0.5) | Still host libdl path |

### Lock / Relock (v0.6 graduation)

Stage 12 **A/B/C/D did not modify `yoyo/projects/yoyo.ty`**. Trust gains came from **peer I/O contract gates**, **selfhost-body DDC window**, and **regression harness** — not a source-body edit. Therefore:

- **No Relock required** — Decision #25 pin remains authoritative
- **Verified 2026-08-28:** `verify-lock-pin.ps1` exit 0 · `yoyo test lock` exit 0 · `yoyo test all` exit 0 · Stage 12-D accept ALL_GREEN · prior C-gate `stage12-v05-regress` ALL_GREEN
- v0.6 graduation documents **wider three-peer I/O + selfhost-body observability**; the Lock pin still locks the **788-handler source artifact**

### A/B/C trust gains (one line each)

| Door | Trust gain |
|------|------------|
| **A** | Three production peers share win32+linux ALLOC/LOAD/WRITE bytes; stub/unknown-OS forks fail-closed — closes residual I/O blind zones |
| **B** | Selfhost body (startup + post-H_00 handlers) under section-ddc EQUAL across peers — shrinks “whole `.text` DIFF ⇒ body still green” blind spot |
| **C** | v0.5 gates remain green — expanded observation does not buy regressions |

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
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage10-runtime-surface.ps1
.\scripts\stage10-asm-peer-io.ps1
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage12-v05-regress.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage5-win-selfhost.ps1
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

### Docs that belong in v0.6

- `SCOPE-v0.6.md` — v0.6 boundary one-pager (graduated)
- `STAGE12_OWNER_CHECKLIST.md` — Stage 12 A→D graduation board
- `RELEASE-v0.6.md` — this file
- `RELEASE-NOTES-v0.6.md` — short external notes
- Pinned artifacts unchanged: `yoyo/tests/yoyo.ty.lock` (Decision #25)

---

## What MUST NOT be published / claimed

### Still OUT (ROADMAP / later)

| Item | Why OUT |
|------|---------|
| **MCU/chip as main product track** | `custom-mcu` scaffold only |
| **C/Rust/Go replacement** | No struct/GC/async/module system |
| **Thompson-proof / DDC ⇒ provably correct** | Forbidden claim |
| **G06+ full golden suite** | Beyond v0.6 conformance |
| **macOS production gate** | MAY work; not required for v0.6 graduation |
| **YOYO-built runtime (non-Rust)** | Still Rust-compiled DLL/`.so` |
| **seed/link host main contraction** | v0.7 theme |
| **Full `.text` three-peer EQUAL** | Selfhost-body window EQUAL only; H_00/extract/runtime still diverge |
| **TheoryManifest / CDS theater** | N.4.1 FORBIDDEN |

### Remaining surface (honest — not blocking v0.6)

| Item | Status |
|------|--------|
| **Embedded Rust runtime still present** | `yoyo_runtime.dll` / `libyoyo_runtime.so` still Rust-built and embedded; bytes **outside** gen12 / selfhost-body windows |
| **LoadLibrary / libdl host trampoline** | H_00 still extracts + loads via host LoadLibraryA / libdl |
| **Full `.text` peer may DIFF** | H_00 entry slot / Rust-only extract stub / IAT / embedded runtime |
| **Non-Win/Linux stub OS** | Plan9/FreeBSD/Haiku/Serenity production I/O still stub (honest fork pinned) |
| **Seed / reference host** | Pure M4 still seeds via `yoyo link` + `bootstrap` (without `--selfhost`) for gen3_direct |

### Misleading claims — forbidden in v0.6 release notes

Do **not** publish wording that implies:

- “Thompson-proof” or “immune to compiler backdoors”
- DDC covers **every** byte in genN PE (embedded runtime DLL and H_00 extract stub remain outside selfhost-body EQUAL window)
- Runtime is YOYO-built / free of Rust host trust
- LoadLibrary / libdl host path is gone
- Full three-peer `.text` EQUAL (only selfhost-body window EQUAL)
- v0.6 is a daily-use application language or C replacement

### Temp / debug artifacts — never publish

```
scripts/_stage8-*/
scripts/_stage9-*/
scripts/_stage10*/
scripts/_stage11*/
scripts/_stage12*/
scripts/_tmp*
yoyo-rust/target-nosidecar/
yoyo-rust/target-selfhost-build/
yoyo-rust/target-runtime-z/
yoyo-rust/target-stage*/
yoyo-test/
```

---

## Pre-publish checklist

### 1. Verify green (exit code 0 everywhere)

Stage 12-D quick re-verify: `verify-lock-pin` · `stage12-three-peer-io` · `stage12-selfhost-body-section-ddc` · `cargo test all/lock/gen12` — ALL_GREEN 2026-08-28. Prior C-gate: `stage12-v05-regress` ALL_GREEN (includes fullbody + stage11/10/9 + WSL).

### 2. Confirm pin integrity

- `yoyo/tests/yoyo.ty.lock` SHA matches live `yoyo/projects/yoyo.ty`
- Pin: `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` (Decision #25, unchanged — **No Relock**)

### 3. Stage 12 four doors

- [x] A — three-peer I/O
- [x] B — selfhost body section-ddc
- [x] C — v0.5 regression retained
- [x] D — v0.6 graduation gate + docs

### 4. Release notes honesty pass

- [x] No ROADMAP items listed as “done in v0.6”
- [x] DDC described as **detection**, not proof
- [x] Remaining surface (Rust runtime / LoadLibrary+libdl / full `.text` DIFF / stub OS) noted
- [x] gen12 window (**18432** bytes, `d782166d`) + selfhost-body (**17805** B EQUAL) documented
- [x] No temp `_stage*` / `_tmp*` dirs in artifact

---

## One-line pitch (external)

**YOYO v0.6 puts three-peer production I/O and a larger selfhost-body window under fail-closed DDC — expanding detection coverage without claiming Thompson proof, YOYO-built runtime, full `.text` EQUAL, or replacing C.**

---

*Maintainer: update when Stage 12 gates or trust-chain SHA monitors change. v0.6 graduation: 2026-08-28 · Stage 12 A/B/C/D all green.*
