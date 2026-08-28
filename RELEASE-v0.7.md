# YOYO v0.7 — Release Boundary (Owner One-Pager)

> **Rule:** Only v0.7 scope may be published under a v0.7 tag. Anything labeled **ROADMAP**, **EXPERIMENTAL**, or stored as temp/debug artifacts **must not ship** with a v0.7 release.
>
> **Sources:** `SCOPE-v0.7.md`, `STAGE13_OWNER_CHECKLIST.md`, `RELEASE-v0.6.md`, `BACKEND_SUPPORT.md` · baseline 2026-08-29.

---

## North star: 打破后门魔咒

YOYO v0.7 **contracts the seed/link host blind zone** after v0.6 — fail-closed observe of `link`/`bootstrap` seed emission, plus **Win/Linux (+ stub OS honesty) parity** under one serial gate — so **fewer paths stay green only because a seed bypass, platform fork, or unobserved host surface left the comparable chain unobserved**.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs. That is a practical trust bar — **not** Thompson immunity or proof of purity. **Still Rust-compiled runtime + host LoadLibraryA / libdl** (not YOYO-built). **Seed is still Rust-emitted** (`yoyo.exe` host). Full `.text` peer compare may still DIFF (H_00 slot / extract stub / IAT / embedded runtime). Stub OS remain stub (not production I/O).

---

## What IS in v0.7

**Product identity (honest):** An **auditable x86-64 compiler ISA** that keeps the v0.6 three-peer I/O / selfhost-body / LoadLibrary-host / Win+Linux pure-M4 baseline, then **observes and fail-closes the seed/link host surface** and **thickens cross-platform parity**.

### Core deliverables (increment over v0.6)

| Area | v0.7 includes |
|------|----------------|
| **Seed/link host (Stage 13-A)** | Fail-closed `stage13-link-host.ps1` (+ `stage13-seed-link-host` alias); canonical seed = H_00 `seed_host_compile*`; `link`≡`bootstrap`(no `--selfhost`) DDC EQUAL; PE/ELF ≤ MAX; Win bans GetTempPathA; `--selfhost` must DIFF + `SEED_HOST path=gennrt` |
| **Cross-platform parity (Stage 13-B)** | `stage13-cross-platform-parity.ps1` (aliases `stage13-parity` / `stage13-cross-parity`); embeds stage12-three-peer-io + stage13-link-host (no SkipLinux) + stage9-pure-m4 + WSL stage10-linux-pure-m4 (no SkipWsl); stub OS honesty pins (freebsd/haiku EQUAL; plan9/serenity forks; apple/android→stub) |
| **v0.6 regression retained (Stage 13-C)** | `stage13-v06-regress.ps1` — serial A+B + stage12-v05-regress (stage12/11/10/9 + all/lock/gen12/fullbody + lock pin + WSL); post-build **zero cargo** (`-SkipBuild`) |
| **v0.6 baseline retained** | three-peer I/O · selfhost-body section-ddc · thinner runtime · LoadLibrary/libdl host · golden · backends · ddc · lock · gen12 · fullbody · H_00 · JS/asm peer · Win/Linux pure M4 |

### Trust-chain anchors (documented SHA)

| Monitor | Value | Notes |
|---------|-------|-------|
| **Lock pin (`yoyo.ty`)** | `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` | Decision #25 — **unchanged** (Stage 13 did not edit locked source) |
| **gen12 / fullbody `.text` (Win PE)** | SHA prefix **`d782166d`** · full `d782166dcb8a9c5de0bb8401203333e436ddc196af3b0a6145a66b5104b61568` | **18432**-byte compared window |
| **Selfhost-body window** | **17805** B EQUAL (JS=Rust=asm) | Skips H_00 entry slot; full `.text` may still DIFF |
| **Embedded runtime.dll** | size **154624** (v0.5+) | Still Rust-built; **outside** gen12 / selfhost-body windows |
| **Linux trampoline** | size **9768** (v0.5+) | Still host libdl path |

### Lock / Relock (v0.7 graduation) — Decision

Stage 13 **A/B/C/D did not modify `yoyo/projects/yoyo.ty`**. Trust gains came from **seed/link host contract + observe**, **cross-platform parity gates**, and **v0.6 regression harness** — not a source-body edit. Therefore:

- **No Relock required** — Decision #25 pin remains authoritative
- **Verified 2026-08-29:** `verify-lock-pin.ps1` exit 0 · `yoyo test lock` (via release binary) · Stage 13-C `stage13-v06-regress.ps1 -SkipBuild` ALL_GREEN · Stage 13-D accept
- v0.7 graduation documents **seed/link host observability + cross-platform parity**; the Lock pin still locks the **788-handler source artifact**

### A/B/C trust gains (one line each)

| Door | Trust gain |
|------|------------|
| **A** | Seed emission cannot silently slide onto genNrt / GetTempPath; dual CLI (`link`/`bootstrap`) pinned to one H_00 host surface under fail-closed size + marker checks |
| **B** | Win **and** Linux must green under one gate; stub OS honesty-pinned — closes「一平台绿、另一平台盲」 |
| **C** | v0.6 gates remain green — seed/parity expansion does not buy regressions |

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
.\scripts\stage12-three-peer-io.ps1
.\scripts\stage12-selfhost-body-section-ddc.ps1
.\scripts\stage12-v05-regress.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage13-link-host.ps1
.\scripts\stage13-cross-platform-parity.ps1
.\scripts\stage13-v06-regress.ps1
.\scripts\stage5-win-selfhost.ps1
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

**Stage 13-D re-verify (2026-08-29):** `verify-lock-pin` exit 0 · `stage13-v06-regress.ps1 -SkipBuild` ALL_GREEN (serial; no parallel cargo).

### Docs that belong in v0.7

- `SCOPE-v0.7.md` — v0.7 boundary one-pager (graduated)
- `STAGE13_OWNER_CHECKLIST.md` — Stage 13 A→D graduation board
- `RELEASE-v0.7.md` — this file
- `RELEASE-NOTES-v0.7.md` — short external notes
- Pinned artifacts unchanged: `yoyo/tests/yoyo.ty.lock` (Decision #25)

---

## What MUST NOT be published / claimed

### Still OUT (ROADMAP / later)

| Item | Why OUT |
|------|---------|
| **MCU/chip as main product track** | `custom-mcu` scaffold only |
| **C/Rust/Go replacement** | No struct/GC/async/module system |
| **Thompson-proof / DDC ⇒ provably correct** | Forbidden claim |
| **G06+ full golden suite** | Beyond v0.7 conformance |
| **macOS production gate** | MAY work; not required for v0.7 graduation |
| **YOYO-built runtime (non-Rust)** | Still Rust-compiled DLL/`.so` |
| **Outside-window bytes / SCOPE-CUT draft** | v0.8 theme |
| **Full `.text` three-peer EQUAL** | Selfhost-body window EQUAL only; H_00/extract/runtime still diverge |
| **Seed no longer Rust-emitted** | Still `yoyo.exe` host emits seed (A observes, does not eliminate) |
| **TheoryManifest / CDS theater** | N.4.1 FORBIDDEN |

### Remaining surface (honest — not blocking v0.7)

| Item | Status |
|------|--------|
| **Embedded Rust runtime still present** | `yoyo_runtime.dll` / `libyoyo_runtime.so` still Rust-built and embedded; bytes **outside** gen12 / selfhost-body windows |
| **LoadLibrary / libdl host trampoline** | H_00 still extracts + loads via host LoadLibraryA / libdl |
| **Full `.text` peer may DIFF** | H_00 entry slot / Rust-only extract stub / IAT / embedded runtime |
| **Non-Win/Linux stub OS** | Plan9/FreeBSD/Haiku/Serenity production I/O still stub (honest fork pinned) |
| **Seed still Rust-emitted** | Pure M4 still seeds via Rust-built `yoyo link` / `bootstrap` (without `--selfhost`); A fail-closes observe, does not replace host emitter |
| **Linux SEED_HOST path=plain** | May still report `path=plain` on some release binaries until classifier rebuild lands `path=h00`; markers (`libyoyo_runtime.so`+`dlopen`) remain fail-closed |

### Misleading claims — forbidden in v0.7 release notes

Do **not** publish wording that implies:

- “Thompson-proof” or “immune to compiler backdoors”
- DDC covers **every** byte in genN PE (embedded runtime DLL and H_00 extract stub remain outside selfhost-body EQUAL window)
- Runtime is YOYO-built / free of Rust host trust
- LoadLibrary / libdl host path is gone
- Seed is no longer host-emitted / Thompson-closed at seed
- Full three-peer `.text` EQUAL (only selfhost-body window EQUAL)
- Stub OS are production I/O
- v0.7 is a daily-use application language or C replacement

### Temp / debug artifacts — never publish

```
scripts/_stage8-*/
scripts/_stage9-*/
scripts/_stage10*/
scripts/_stage11*/
scripts/_stage12*/
scripts/_stage13*/
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

Stage 13-D quick re-verify: `verify-lock-pin` · `stage13-v06-regress.ps1 -SkipBuild` — ALL_GREEN 2026-08-29. Prior A/B/C gates ALL_GREEN (includes fullbody + stage12/11/10/9 + WSL).

### 2. Confirm pin integrity

- `yoyo/tests/yoyo.ty.lock` SHA matches live `yoyo/projects/yoyo.ty`
- Pin: `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` (Decision #25, unchanged — **No Relock**)

### 3. Stage 13 four doors

- [x] A — seed/link host
- [x] B — cross-platform parity
- [x] C — v0.6 regression retained
- [x] D — v0.7 graduation gate + docs

### 4. Release notes honesty pass

- [x] No ROADMAP items listed as “done in v0.7”
- [x] DDC described as **detection**, not proof
- [x] Remaining surface (Rust runtime / LoadLibrary+libdl / full `.text` DIFF / stub OS / seed still Rust-emitted) noted
- [x] gen12 window (**18432** bytes, `d782166d`) + selfhost-body (**17805** B EQUAL) documented
- [x] No temp `_stage*` / `_tmp*` dirs in artifact

---

## One-line pitch (external)

**YOYO v0.7 puts seed/link host emission and Win/Linux (+ stub OS) parity under fail-closed DDC gates — contracting host/platform blind zones without claiming Thompson proof, YOYO-built runtime, full `.text` EQUAL, or replacing C.**

---

*Maintainer: update when Stage 13 gates or trust-chain SHA monitors change. v0.7 graduation: 2026-08-29 · Stage 13 A/B/C/D all green.*
