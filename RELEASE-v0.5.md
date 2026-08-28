# YOYO v0.5 — Release Boundary (Owner One-Pager)

> **Rule:** Only v0.5 scope may be published under a v0.5 tag. Anything labeled **ROADMAP**, **EXPERIMENTAL**, or stored as temp/debug artifacts **must not ship** with a v0.5 release.
>
> **Sources:** `SCOPE-v0.5.md`, `STAGE11_OWNER_CHECKLIST.md`, `RELEASE-v0.4.md`, `BACKEND_SUPPORT.md` · baseline 2026-08-28.

---

## North star: 打破后门魔咒

YOYO v0.5 **shrinks the biggest remaining host-trust holes left after v0.4** — embedded Rust runtime face and H_00 LoadLibrary / libdl host trampoline — so **more runtime-related bytes sit under fail-closed size/parity gates + Lock**, and **fewer paths are green only because an opaque host loader / bloated cdylib said so**.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs. That is a practical trust bar — **not** Thompson immunity or proof of purity. **Still Rust-compiled runtime + host LoadLibraryA / libdl** (not YOYO-built).

---

## What IS in v0.5

**Product identity (honest):** An **auditable x86-64 compiler ISA** that keeps the v0.4 H_00 / JS peer / Win+Linux pure-M4 / asm I/O baseline, then **further contracts the embedded Rust runtime face** and **shrinks the H_00 host-loader surface** (cwd-relative extract; thinner Linux trampoline).

### Core deliverables (increment over v0.4)

| Area | v0.5 includes |
|------|----------------|
| **Thinner runtime (Stage 11-A)** | Fail-closed size gate: DLL **231936→154624** B (`stage11-runtime-surface.ps1` MAX **170000**; `profile.release-runtime` fat LTO + strip + `panic=abort` + `opt-level=z`); `.so` **592064→407232**; genN PE **322560→248832** / ELF **704512→512000**; gen12 window still **18432** B, SHA `43ffde58`→`d782166d`; embed exact + gen1 H_00 ≡ bootstrap `.text` DDC |
| **LoadLibrary / libdl host (Stage 11-B)** | Win H_00 **cwd-relative** `yoyo_rt.dll` (dropped GetTempPathA/lstrcatA; host-loader IAT **5→3**); Linux tramp **14464→9768** B (nostdlib; MAX **12000**; exact embed); `stage11-loadlibrary-host.ps1` |
| **v0.4 regression retained (Stage 11-C)** | stage10 / stage9 / fullbody / lock / gen12 — all still green |
| **v0.4 baseline retained** | golden · backends · ddc · lock · gen12 · fullbody · stage5 · stage9 H_00 / JS peer / Win pure M4 · Linux pure M4 · asm I/O |

### Trust-chain anchors (documented SHA)

| Monitor | Value | Notes |
|---------|-------|-------|
| **Lock pin (`yoyo.ty`)** | `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` | Decision #25 — **unchanged** (Stage 11 did not edit locked source) |
| **gen12 / fullbody `.text` (Win PE)** | SHA prefix **`d782166d`** · full `d782166dcb8a9c5de0bb8401203333e436ddc196af3b0a6145a66b5104b61568` | **18432**-byte compared window |
| **Embedded runtime.dll** | size **154624** · SHA `34783cfd3fc4470ef15e7556e27ebd34e5ed685396d510901429759d5925e5c2` | Still Rust-built; **outside** gen12 window |
| **Linux trampoline** | size **9768** · SHA `76931cd0da4e116812d0a0a4a2ef05548740c918f61b4cdd93f54d5107cdbfcc` | Still host libdl path |
| **M4 gen4 parity (Win pure H_00)** | Same **`d782166d`** window — gen4≡gen3_direct EQUAL | `stage9-pure-m4.ps1` |

### Lock / Relock (v0.5 graduation)

Stage 11 **A/B/C did not modify `yoyo/projects/yoyo.ty`**. Trust gains came from **runtime build posture**, **H_00 extract / IAT surface**, and **Linux trampoline** — not a source-body edit. Therefore:

- **No Relock required** — Decision #25 pin remains authoritative
- **Verified 2026-08-28:** `verify-lock-pin.ps1` exit 0 · `yoyo test lock` exit 0 · `yoyo test all` exit 0
- v0.5 graduation documents **thinner host DLL/`.so` face + smaller LoadLibrary/libdl host surface**; the Lock pin still locks the **788-handler source artifact**

### A/B/C trust gains (one line each)

| Door | Trust gain |
|------|------------|
| **A** | Host-compiled runtime.dll bytes cut again (~⅓ vs v0.4) and fail-closed; compile-parity vs bootstrap monitored |
| **B** | H_00 no longer trusts temp-path string concat; IAT host-loader slice 5→3; Linux tramp size fail-closed |
| **C** | v0.4 gates remain green — thinner face does not buy regressions |

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
.\scripts\stage10-runtime-surface.ps1
.\scripts\stage11-runtime-surface.ps1
.\scripts\stage11-loadlibrary-host.ps1
.\scripts\stage10-asm-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage5-win-selfhost.ps1
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

### Docs that belong in v0.5

- `SCOPE-v0.5.md` — v0.5 boundary one-pager
- `STAGE11_OWNER_CHECKLIST.md` — Stage 11 A→D graduation board
- `RELEASE-v0.5.md` — this file
- `RELEASE-NOTES-v0.5.md` — short external notes
- Pinned artifacts unchanged: `yoyo/tests/yoyo.ty.lock` (Decision #25)

---

## What MUST NOT be published / claimed

### Still OUT (ROADMAP / later)

| Item | Why OUT |
|------|---------|
| **MCU/chip as main product track** | `custom-mcu` scaffold only |
| **C/Rust/Go replacement** | No struct/GC/async/module system |
| **Thompson-proof / DDC ⇒ provably correct** | Forbidden claim |
| **G06+ full golden suite** | Beyond v0.5 conformance |
| **macOS production gate** | MAY work; not required for v0.5 graduation |
| **YOYO-built runtime (non-Rust)** | Still Rust-compiled DLL/`.so` |
| **Three-peer full-body section-ddc EQUAL** | v0.6 theme |
| **TheoryManifest / CDS theater** | N.4.1 FORBIDDEN |

### Remaining surface (honest — not blocking v0.5)

| Item | Status |
|------|--------|
| **Embedded Rust runtime still present** | `yoyo_runtime.dll` / `libyoyo_runtime.so` still Rust-built and embedded; bytes **outside** gen12 18432-byte `.text` window |
| **LoadLibrary / libdl host trampoline** | H_00 still extracts + loads via host LoadLibraryA / libdl (cwd-relative / thinner tramp only) |
| **Full-body section-ddc may DIFF** | H_00 / IAT width / embedded runtime can still diverge across peers on whole-PE compare |
| **Seed / reference host** | Pure M4 still seeds via `yoyo link` + `bootstrap` (without `--selfhost`) for gen3_direct |

### Misleading claims — forbidden in v0.5 release notes

Do **not** publish wording that implies:

- “Thompson-proof” or “immune to compiler backdoors”
- DDC covers **every** byte in genN PE (embedded runtime DLL is outside gen12 window)
- Runtime is YOYO-built / free of Rust host trust
- LoadLibrary / libdl host path is gone
- v0.5 is a daily-use application language or C replacement

### Temp / debug artifacts — never publish

```
scripts/_stage8-*/
scripts/_stage9-*/
scripts/_stage10*/
scripts/_stage11*/
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

Run the machine-checkable gates block above (Stage 11-D quick re-verify: stage11-runtime-surface -SkipBuild · stage11-loadlibrary-host -SkipBuild · verify-lock-pin · cargo test all).

### 2. Confirm pin integrity

- `yoyo/tests/yoyo.ty.lock` SHA matches live `yoyo/projects/yoyo.ty`
- Pin: `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` (Decision #25, unchanged)

### 3. Stage 11 four doors

- [x] A — thinner / YOYO-built-path runtime surface
- [x] B — LoadLibrary / libdl host contraction
- [x] C — v0.4 regression retained
- [x] D — v0.5 graduation gate + docs

### 4. Release notes honesty pass

- [x] No ROADMAP items listed as “done in v0.5”
- [x] DDC described as **detection**, not proof
- [x] Remaining surface (Rust runtime still embedded / LoadLibrary+libdl host) noted
- [x] gen12 window boundary (**18432** bytes, `d782166d`) documented
- [x] No temp `_stage*` / `_tmp*` dirs in artifact

---

## One-line pitch (external)

**YOYO v0.5 shrinks the remaining host-trust face again — thinner embedded Rust runtime, smaller LoadLibrary/libdl host surface — expanding fail-closed observability under DDC + Lock without claiming Thompson proof, YOYO-built runtime, or replacing C.**

---

*Maintainer: update when Stage 11 gates or trust-chain SHA monitors change. v0.5 graduation: 2026-08-28 · Stage 11 A/B/C/D all green.*
