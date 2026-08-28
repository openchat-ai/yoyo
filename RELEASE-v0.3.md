# YOYO v0.3 — Release Boundary (Owner One-Pager)

> **Rule:** Only v0.3 scope may be published under a v0.3 tag. Anything labeled **ROADMAP**, **EXPERIMENTAL**, or stored as temp/debug artifacts **must not ship** with a v0.3 release.
>
> **Sources:** `SCOPE-v0.3.md`, `STAGE9_OWNER_CHECKLIST.md`, `RELEASE-v0.2.md`, `BACKEND_SUPPORT.md` · baseline 2026-08-28.

---

## North star: 打破后门魔咒

YOYO v0.3 **closes trust holes left after v0.2** — gen1 H_00 pure self-host, JS peer I/O parity for DDC, and Win M4 without `bootstrap --selfhost` — so **more self-host bytes sit under three-chain DDC + Lock**, and **fewer paths are green only because Rust host wrapping said so**.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs. That is a practical trust bar — **not** Thompson immunity or proof of purity.

---

## What IS in v0.3

**Product identity (honest):** An **auditable x86-64 compiler ISA** that can self-host through **H_00** (not only genNrt), align **JS↔Rust** platform I/O emit for DDC, and run **Win pure M4** (gen1→gen4) without host `--selfhost` scaffolding — on top of the v0.2 full-body / M4 / Lock baseline.

### Core deliverables (increment over v0.2)

| Area | v0.3 includes |
|------|----------------|
| **gen1 H_00 pure path (Stage 9-A)** | PE entry → H_00 (no genNrt entry wrapper); zero-arg gen1 → `output.exe`; gates `stage9-gen1-h00-selfhost.ps1` + stage5 H_00 line |
| **JS peer platform I/O (Stage 9-B)** | Production PE `setEmitPlatform('win32')` + `platform-io.js` byte-equal Rust `platform_io.rs` for `0x20/0x50/0x51`; golden default still stub (G-SM-IO); `stage9-js-peer-io.ps1` |
| **Win pure M4 (Stage 9-C)** | `stage9-pure-m4.ps1`: gen1→gen4 via H_00, **no** `bootstrap --selfhost`; gen4≡gen3_direct `.text` DDC EQUAL |
| **v0.2 baseline retained** | golden · backends · ddc · lock · gen12 · fullbody · stage5 · stage8 M4 (genNrt) — all still green |

### Trust-chain anchors (documented SHA)

| Monitor | Value | Notes |
|---------|-------|-------|
| **Lock pin (`yoyo.ty`)** | `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` | Decision #25 — **unchanged** (Stage 9 did not edit locked source) |
| **gen12 / fullbody `.text` (Win PE)** | SHA prefix **`b609a735`** · full `b609a7354229449eae3701df4def828bfa82bc45251b473ae31293f955c05e5e` | **18432**-byte compared window (H_00 extract stub enlarged vs v0.2 17920 / `e92520ea`) |
| **M4 gen4 parity (Win pure H_00)** | Same **`b609a735`** window — gen4≡gen3_direct EQUAL | `stage9-pure-m4.ps1` |
| **M4 gen4 parity (Win genNrt)** | Same **`b609a735`** window | `stage8-extended-selfhost.ps1` (regression) |
| **M4 gen4 parity (Linux ELF)** | SHA prefix **`dab59f96`** · full `dab59f96fc0a78f20678d2f784a6ddc6194137f157c35ca78015d224c9b02518` | Full ELF `.text`; still via `--selfhost` |

### Lock / Relock (v0.3 graduation)

Stage 9 **A/B/C did not modify `yoyo/projects/yoyo.ty`**. Trust expansion came from **toolchain emit/runtime** (H_00 PE entry patch, JS `platform-io.js`, pure-M4 scripts) — not a source-body edit. Therefore:

- **No Relock required** — Decision #25 pin remains authoritative
- **Verified 2026-08-28:** `verify-lock-pin.ps1` exit 0 · `verify-yoyo-ty.mjs` exit 0 · `cargo run -- test lock` exit 0
- v0.3 graduation documents **expanded DDC observability** (H_00 path in gen12 window; JS I/O peer comparable); the Lock pin still locks the **788-handler source artifact**

### A/B/C trust gains (one line each)

| Door | Trust gain |
|------|------------|
| **A** | Self-host “green” is no longer only via Rust genNrt entry wrap — H_00 runtime path is script-gated and in the gen12 `.text` window |
| **B** | JS↔Rust win32 I/O handler bytes EQUAL — closes the “Rust real syscall / JS movabs+store” DDC blind zone on production PE path |
| **C** | Win M3→M4 algebra completes inside H_00-patched PEs without host `--selfhost` orchestration |

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
.\scripts\stage5-win-selfhost.ps1
.\scripts\stage9-gen1-h00-selfhost.ps1
.\scripts\stage9-js-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage8-extended-selfhost.ps1
# Linux/WSL when applicable:
# wsl bash /mnt/f/yoyo/scripts/stage8-extended-selfhost.sh
```

### Docs that belong in v0.3

- `SCOPE-v0.3.md` — v0.3 boundary one-pager
- `STAGE9_OWNER_CHECKLIST.md` — Stage 9 A→D graduation board
- `RELEASE-v0.3.md` — this file
- `RELEASE-NOTES-v0.3.md` — short external notes
- `BACKEND_SUPPORT.md` — Stage 9-A/B/C trust-chain sections
- `PROMPT-v3.md` — Week axis Stage 9 GREEN
- Pinned artifacts unchanged: `yoyo/tests/yoyo.ty.lock` (Decision #25)

---

## What MUST NOT be published / claimed

### Still OUT (ROADMAP / later)

| Item | Why OUT |
|------|---------|
| **MCU/chip as main product track** | `custom-mcu` scaffold only |
| **C/Rust/Go replacement** | No struct/GC/async/module system |
| **Thompson-proof / DDC ⇒ provably correct** | Forbidden claim |
| **G06+ full golden suite** | Beyond v0.3 conformance |
| **macOS production gate** | MAY work; not required for v0.3 graduation |
| **ELF H_00 pure M4** | Linux still uses `bootstrap --selfhost` |
| **TheoryManifest / CDS theater** | N.4.1 FORBIDDEN |

### Remaining surface (honest — not blocking v0.3)

| Item | Status |
|------|--------|
| **Embedded `yoyo_runtime.dll`** | Still Rust-compiled and embedded; bytes **outside** gen12 18432-byte `.text` window |
| **Linux M4 `--selfhost`** | Still required; no ELF H_00 pure path yet |
| **Python asm I/O stubs** | Still movabs+store (honest peer fork until asm migration) |
| **Seed / reference host** | Pure M4 still seeds via `yoyo link` + `bootstrap` (without `--selfhost`) for gen3_direct |

### Misleading claims — forbidden in v0.3 release notes

Do **not** publish wording that implies:

- “Thompson-proof” or “immune to compiler backdoors”
- DDC covers **every** byte in genN PE (embedded runtime DLL is outside gen12 window)
- asm peer emits real OS syscalls
- v0.3 is a daily-use application language or C replacement

### Temp / debug artifacts — never publish

```
scripts/_stage8-win/
scripts/_stage8-linux/
scripts/_stage9-*/
yoyo-rust/target-nosidecar/
yoyo-rust/target-selfhost-build/
yoyo-rust/target-sh-build/
yoyo-test/
```

---

## Pre-publish checklist

### 1. Verify green (exit code 0 everywhere)

Run the machine-checkable gates block above.

### 2. Confirm pin integrity

- `yoyo/tests/yoyo.ty.lock` SHA matches live `yoyo/projects/yoyo.ty`
- Pin: `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` (Decision #25, unchanged)

### 3. Stage 9 four doors

- [x] A — gen1 H_00 / pure self-host path
- [x] B — JS peer platform I/O parity
- [x] C — Win pure M4 (no `--selfhost`)
- [x] D — v0.3 graduation gate + docs

### 4. Release notes honesty pass

- [ ] No ROADMAP items listed as “done in v0.3”
- [ ] DDC described as **detection**, not proof
- [ ] Remaining surface (runtime.dll / Linux `--selfhost` / asm stubs) noted
- [ ] gen12 window boundary (**18432** bytes, `b609a735`) documented
- [ ] No temp `_stage8-*` / `_stage9-*` dirs in artifact

---

## One-line pitch (external)

**YOYO v0.3 closes the biggest remaining self-host trust holes — H_00 pure path, JS I/O DDC parity, Win M4 without host `--selfhost` — expanding three-chain DDC + Lock coverage without claiming Thompson proof or replacing C.**

---

*Maintainer: update when Stage 9 gates or trust-chain SHA monitors change. v0.3 graduation: 2026-08-28 · Stage 9 A/B/C/D all green.*
