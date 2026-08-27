# YOYO v0.2 — Release Boundary (Owner One-Pager)

> **Rule:** Only v0.2 scope may be published under a v0.2 tag. Anything labeled **ROADMAP**, **EXPERIMENTAL**, or stored as temp/debug artifacts **must not ship** with a v0.2 release.
>
> **Sources:** `SCOPE-v0.2.md`, `STAGE8_OWNER_CHECKLIST.md`, `RELEASE-v0.1.md`, `BACKEND_SUPPORT.md` · baseline 2026-08-27.

---

## North star: 打破后门魔咒

YOYO v0.2 **expands how many compiler bytes sit under three-chain DDC + Lock monitoring** — it is not a pivot to “general application development.” v0.2 recovers v0.1 **SCOPE-CUT** items (W5.5 full body + libyoyo real I/O + M4 self-host) **as trust-chain coverage**, not as product completeness theater.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs. That is a practical trust bar — **not** Thompson immunity or proof of purity.

---

## What IS in v0.2

**Product identity (honest):** An **auditable x86-64 compiler ISA** that can **read/write real files**, **allocate memory**, compile its **full 788-handler body**, and run **M2→M3→M4 self-host** on Windows + Linux — with machine-checkable DDC + Lock gates. Built for people who need to **detect compiler-level divergence**, not a C/Rust replacement.

### Core deliverables (increment over v0.1)

| Area | v0.2 includes |
|------|----------------|
| **Real platform I/O (D-1 closed)** | Win32: VirtualAlloc + CreateFile/ReadFile/WriteFile via kernel32 IAT (`platform_io.rs`); Linux: mmap + open/read/write syscalls. Production Rust link paths no longer movabs+store stub for `0x20/0x50/0x51` |
| **libyoyo migration (W5.5 platform half)** | Platform syscall emit centralized in `yoyo-rust/verifier/src/platform_io.rs`; golden stub path retained for fixtures |
| **Full body compiler path (W5.5 body half)** | Complete 788-handler `yoyo.ty` compiles via bootstrap; `test fullbody` green; gen12 DDC window **17920 bytes** (not W-SM scoped=34 only) |
| **Extended self-host M2→M3→M4** | `scripts/stage8-extended-selfhost.ps1` (Win) + `.sh` (Linux); gen4≡gen3_direct `.text` DDC EQUAL |
| **v0.1 baseline retained** | golden 739/739 · backends · ddc · lock · gen12 · stage5 M2→M3 — all still green |

### Trust-chain anchors (documented SHA)

| Monitor | Value | Notes |
|---------|-------|-------|
| **Lock pin (`yoyo.ty`)** | `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` | Decision #25 — **unchanged** (Stage 8 did not edit locked source) |
| **gen12 / fullbody `.text` (Win PE)** | SHA prefix **`e92520ea`** · full `e92520eadbfddbbc336c5a73261cb3788d0d6183590080eb3d271a3aab63ed66` | 17920-byte compared window; includes I/O emit from Stage 8-A |
| **M4 gen4 parity (Win)** | Same **`e92520ea`** window — gen4≡gen3_direct EQUAL | `stage8-extended-selfhost.ps1` |
| **M4 gen4 parity (Linux ELF)** | SHA prefix **`dab59f96`** · full `dab59f96fc0a78f20678d2f784a6ddc6194137f157c35ca78015d224c9b02518` | Full ELF `.text` compare on Linux path |

### Lock / Relock (v0.2 graduation)

Stage 8 **A/B/C did not modify `yoyo/projects/yoyo.ty`**. Trust expansion came from **toolchain emit/runtime** (real I/O, full-body bootstrap path, M4 scripts) — not a source-body edit. Therefore:

- **No Relock required** — Decision #25 pin remains authoritative
- **Verified 2026-08-27:** `verify-lock-pin.ps1` exit 0 · `verify-yoyo-ty.mjs` exit 0 · `cargo run -- test lock` exit 0
- v0.2 graduation documents **expanded DDC observability** on emitted output; the Lock pin still locks the **788-handler source artifact**

### Machine-checkable gates (all must exit 0 before publish)

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all          # golden + backends + ddc + gen12 + fullbody
cargo run -- test lock
cargo run -- test gen12
cargo run -- test fullbody

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\verify-yoyo-ty.mjs
node .\yoyo-js\scripts\golden.js
.\scripts\stage5-win-selfhost.ps1
.\scripts\stage8-extended-selfhost.ps1
# Linux/WSL when applicable:
# wsl bash /mnt/f/yoyo/scripts/stage8-extended-selfhost.sh
```

### Docs that belong in v0.2

- `SCOPE-v0.2.md` — v0.2 boundary one-pager
- `STAGE8_OWNER_CHECKLIST.md` — Stage 8 A→D graduation board
- `RELEASE-v0.2.md` — this file
- `BACKEND_SUPPORT.md` — Stage 8-A/B/C trust-chain sections
- `PROMPT-v3.md` — Week axis Stage 8 GREEN line
- Pinned artifacts unchanged: `yoyo/tests/yoyo.ty.lock` (Decision #25)

---

## What MUST NOT be published / claimed

### Still OUT (ROADMAP / v0.3+)

| Item | Why OUT |
|------|---------|
| **MCU/chip as main product track** | `custom-mcu` scaffold only; v0.2 does not expand 8051/AVR fatal DDC |
| **C/Rust/Go replacement** | No struct/GC/async/module system |
| **Thompson-proof / DDC ⇒ provably correct** | Forbidden claim |
| **G06+ full golden suite** | Beyond v0.2 conformance |
| **macOS production gate** | MAY work; not required for v0.2 graduation |
| **37 backends = 37 interpreters** | Many emit-only or stub (see matrix Legend) |
| **TheoryManifest / CDS theater** | N.4.1 FORBIDDEN |

### Known RED (honest — not blocking v0.2)

| Item | Status |
|------|--------|
| **gen1 H_00 runtime self-host** | **RED** (pre-existing): gen1.exe zero-arg embedded-path selfhost does not produce `output.exe`. **Not blocking** — M2→M3 and M3→M4 use gen2rt/gen3rt embedded startup (GREEN). Documented in stage5/stage8 script summaries. |

### Misleading claims — forbidden in v0.2 release notes

Do **not** publish wording that implies:

- “Thompson-proof” or “immune to compiler backdoors”
- DDC covers **every** byte in gen2rt/gen3rt PE (embedded Rust startup stub is **outside** gen12 17920-byte window — see `BACKEND_SUPPORT.md`)
- JS chain emits real OS syscalls (JS may still movabs+store — intentional peer divergence until JS migration)
- v0.2 is a daily-use application language

### Temp / debug artifacts — never publish

Same exclusions as v0.1 (`RELEASE-v0.1.md`), plus Stage 8 scratch:

```
scripts/_stage8-win/
scripts/_stage8-linux/
yoyo-rust/target-nosidecar/
yoyo-rust/target-selfhost-build/
yoyo-rust/target-sh-build/
```

---

## Pre-publish checklist

### 1. Verify green (exit code 0 everywhere)

Run the machine-checkable gates block above.

### 2. Confirm pin integrity

- `yoyo/tests/yoyo.ty.lock` SHA matches live `yoyo/projects/yoyo.ty`
- Pin: `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` (Decision #25, unchanged)

### 3. Stage 8 four doors

- [x] A — libyoyo real platform I/O
- [x] B — Full body compiler path
- [x] C — M2→M3→M4 Win + Linux
- [x] D — v0.2 graduation gate + docs

### 4. Release notes honesty pass

- [ ] No ROADMAP items listed as “done in v0.2”
- [ ] DDC described as **detection**, not proof
- [ ] gen1 H_00 RED noted where self-host is discussed
- [ ] gen12 window boundary (17920 bytes) documented
- [ ] No temp `_stage8-*` dirs in artifact

---

## One-line pitch (external)

**YOYO v0.2 is an auditable x86-64 compiler that reads real files, runs a full 788-handler self-host through M4, and expands three-chain DDC + Lock coverage — for people who need to *detect* compiler-level divergence, not write everyday applications.**

---

*Maintainer: update when Stage 8 gates or trust-chain SHA monitors change. v0.2 graduation: 2026-08-27 · Stage 8 A/B/C/D all green.*
