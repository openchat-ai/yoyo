# YOYO v0.2.0 Release Notes

**Date:** 2026-08-27  
**Tag:** `v0.2.0`  
**Commit:** `f6f323cb7e259edf3cf84d16ce47f3dbd1e5b421`

---

## One-line pitch

**YOYO v0.2 is an auditable x86-64 compiler that reads real files, runs a full 788-handler self-host through M4, and expands three-chain DDC + Lock coverage — for people who need to *detect* compiler-level divergence, not write everyday applications.**

---

## What this release is

YOYO v0.2 **expands how many compiler bytes sit under three-chain DDC + Lock monitoring** — it is not a pivot to general application development. v0.2 recovers v0.1 SCOPE-CUT items (W5.5 full body + libyoyo real I/O + M4 self-host) as **trust-chain coverage**, not product completeness theater.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs. That is a practical trust bar — **not** Thompson immunity or proof of purity.

---

## Included in v0.2

### Core deliverables (increment over v0.1)

| Area | v0.2 includes |
|------|----------------|
| **Real platform I/O (D-1 closed)** | Win32: VirtualAlloc + CreateFile/ReadFile/WriteFile via kernel32 IAT; Linux: mmap + open/read/write syscalls. Production Rust link paths no longer movabs+store stub for `0x20/0x50/0x51` |
| **libyoyo migration (W5.5 platform half)** | Platform syscall emit centralized in `yoyo-rust/verifier/src/platform_io.rs`; golden stub path retained for fixtures |
| **Full body compiler path (W5.5 body half)** | Complete 788-handler `yoyo.ty` compiles via bootstrap; `test fullbody` green; gen12 DDC window **17920 bytes** |
| **Extended self-host M2→M3→M4** | `scripts/stage8-extended-selfhost.ps1` (Win) + `.sh` (Linux); gen4≡gen3_direct `.text` DDC EQUAL |
| **v0.1 baseline retained** | golden 739/739 · backends · ddc · lock · gen12 · stage5 M2→M3 — all still green |

### Trust-chain anchors

| Monitor | Value | Notes |
|---------|-------|-------|
| **Lock pin (`yoyo.ty`)** | `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` | Decision #25 — unchanged (Stage 8 did not edit locked source) |
| **gen12 / fullbody `.text` (Win PE)** | SHA prefix **`e92520ea`** | 17920-byte compared window |
| **M4 gen4 parity (Win)** | Same **`e92520ea`** window — gen4≡gen3_direct EQUAL | `stage8-extended-selfhost.ps1` |
| **M4 gen4 parity (Linux ELF)** | SHA prefix **`dab59f96`** | Full ELF `.text` compare on Linux path |

### Lock / Relock

Stage 8 A/B/C did **not** modify `yoyo/projects/yoyo.ty`. Trust expansion came from toolchain emit/runtime — not a source-body edit. **No Relock required** — Decision #25 pin remains authoritative.

---

## Explicitly NOT in v0.2

| Item | Status |
|------|--------|
| **MCU/chip as main product track** | `custom-mcu` scaffold only |
| **C/Rust/Go replacement** | No struct/GC/async/module system |
| **Thompson-proof / DDC ⇒ provably correct** | Forbidden claim |
| **G06+ full golden suite** | Beyond v0.2 conformance |
| **macOS production gate** | MAY work; not required for v0.2 graduation |
| **37 backends = 37 interpreters** | Many emit-only or stub (see matrix Legend) |

### Known RED (honest — not blocking v0.2)

| Item | Status |
|------|--------|
| **gen1 H_00 runtime self-host** | **RED** (pre-existing): gen1.exe zero-arg embedded-path selfhost does not produce `output.exe`. **Not blocking** — M2→M3 and M3→M4 use gen2rt/gen3rt embedded startup (GREEN). |

---

## Pre-publish gates (all exit 0)

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\stage8-extended-selfhost.ps1
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
- DDC covers the gen12 17920-byte `.text` window; embedded Rust startup stub is outside that window.
- JS chain may still movabs+store for syscalls — intentional peer divergence until JS migration.
- gen1 H_00 zero-arg self-host path remains RED; M2→M3→M4 paths are green.

---

## Files in this release

- `RELEASE-v0.2.md` — release boundary one-pager
- `RELEASE-NOTES-v0.2.md` — this document
- `SCOPE-v0.2.md` — v0.2 boundary spec
- `STAGE8_OWNER_CHECKLIST.md` — Stage 8 A→D graduation board

---

*Maintainer: see `RELEASE-v0.2.md` for full exclude list and honesty checklist.*
