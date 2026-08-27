# YOYO v0.1.0 Release Notes

**Date:** 2026-08-27  
**Tag:** `v0.1.0`  
**Commit:** _(filled after release commit)_

---

## One-line pitch

**YOYO v0.1 is an auditable x86-64 compiler ISA with three independent implementations and machine-checkable cross-peer verification—built for people who need to *detect* compiler-level divergence, not a general-purpose programming language.**

---

## What this release is

YOYO v0.1 delivers a **verifiable, compiler-specialized ISA and toolchain** for x86-64:

- **38 core opcodes**, 256-slot state machine
- **Three independent peer implementations** (JavaScript, Rust, Python asm) compared by Diverse Double-Compiling (DDC)
- DDC **detects cross-peer divergence** under independence assumptions; it does **not** prove compiler purity or Thompson-proof correctness

---

## Included in v0.1

### Language & spec

- Layer-S `.ty` grammar, 38-op ISA table, operational semantics (`PROMPT-v3.md` Parts 4 / 4S / G)
- Normative spec: Parts N, L, 4, 4S, G, 5–9, F, Deduce, Gnd, Appendix F/G

### Toolchain (4 projects)

| Project | Role |
|---------|------|
| `yoyo/` | Locked compiler source (`yoyo/projects/yoyo.ty`) |
| `yoyo-js/` | Independent JS peer |
| `yoyo-rust/` | Verifier + libyoyo (primary test harness) |
| `yoyo-asm/` | Independent Python asm peer |

### Verification & conformance

- **3-chain DDC:** JS == Rust == Python asm section-ddc EQUAL (SHA-256: `4fb8b87f`)
- **gen1≡gen2** via `cargo run -- test gen12`
- **Golden fixtures:** 739/739 PASS (G00–G05 + extended emit matrix)
- **DDC fixtures:** `00_nop_ret` … `04_ldb_ptr` + container PE/ELF — all PASS
- **Backends:** 37/37 compile+link smoke; win32/linux x64 production paths green

### Lock lifecycle

- 8-step Lock Protocol, `test lock`
- `scripts/verify-lock-pin.ps1`, `scripts/verify-yoyo-ty.mjs`
- Pinned body: `yoyo/tests/yoyo.ty.lock` (Decision #25, SHA `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb`)

### Self-host (scoped)

- Windows M2→M3: `scripts/stage5-win-selfhost.ps1` (no-sidecar path green)
- Linux M2→M3: `scripts/stage5-linux-selfhost.sh`

### MCU hook (scaffold only)

- `--target=custom-mcu` copy-and-replace scaffold + smoke DDC
- **Not** a finished chip backend — extend emit + interp before promotion

---

## Explicitly NOT in v0.1 (SCOPE-CUT)

Do **not** interpret v0.1 as shipping any of the following:

| Item | Status |
|------|--------|
| **W5.5 — full `yoyo.ty` body + libyoyo migration** | SCOPE-CUT (post–v0.1) |
| **Phase 2 “≤1500 lines full body” exit** | Future gate, not v0.1 |
| **Full compiler self-host as product claim** | M2→M3 green for **scoped** startup only |
| **G06 and beyond** | Not in v0.1 conformance set |
| **Thompson-proof / DDC ⇒ provably correct** | Forbidden claim |
| **Frozen full-body compiler** | W5.5 still SCOPE-CUT |
| **`custom-mcu` as production backend** | Scaffold only |
| **37 backends = 37 fully interpreted MCUs** | Many emit-only or stub (see `BACKEND_SUPPORT.md` Legend) |
| **C/Rust/Go replacement or general-purpose PL** | Out of scope |

### EXPERIMENTAL (in repo for history — not v0.1 features)

- W-START NODE, body-extend queue, attempt logs under `docs/auxdocs/`
- Internal stage owner checklist (`STAGE4_OWNER_CHECKLIST.md`) — maintainer board, not product spec
- TheoryManifest / DeriveTick / CDS daemon narratives
- Part 12 SIMD, Part 15–16 demos/roadmap, Appendix H future deduction substrate

---

## Pre-publish gates (all exit 0)

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
cargo run -- test lock
cargo run -- test gen12

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\verify-yoyo-ty.mjs
.\scripts\stage5-win-selfhost.ps1
```

---

## Upgrade / install

Ship **source + spec + tests + scripts**. Build from tag:

```powershell
cd F:\yoyo\yoyo-rust
cargo build --release -p verifier -p yoyo-runtime

cd F:\yoyo\yoyo-js
npm ci
```

Prebuilt `*.exe` / `target/` trees are not part of the release artifact.

---

## Known limitations

- DDC detects divergence; it does not certify semantic correctness of emitted code.
- Self-host paths cover scoped startup flows, not full-body compiler reproduction.
- Backend matrix includes stubs and emit-only targets; check `BACKEND_SUPPORT.md` before assuming runtime support.
- Windows self-host uses no-sidecar PE embedding; sidecar path is not the v0.1 gate.

---

## Files in this release commit

- `RELEASE-v0.1.md` — release boundary one-pager
- `RELEASE-NOTES-v0.1.md` — this document
- `.gitignore` — excludes temp/debug scratch from accidental publish

---

*Maintainer: see `RELEASE-v0.1.md` for full exclude list and honesty checklist.*
