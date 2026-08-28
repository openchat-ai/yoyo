# YOYO v0.4 — Release Boundary (Owner One-Pager)

> **Rule:** Only v0.4 scope may be published under a v0.4 tag. Anything labeled **ROADMAP**, **EXPERIMENTAL**, or stored as temp/debug artifacts **must not ship** with a v0.4 release.
>
> **Sources:** `SCOPE-v0.4.md`, `STAGE10_OWNER_CHECKLIST.md`, `RELEASE-v0.3.md`, `BACKEND_SUPPORT.md` · baseline 2026-08-28.

---

## North star: 打破后门魔咒

YOYO v0.4 **shrinks the biggest remaining host-trust holes left after v0.3** — embedded runtime.dll surface, Linux `--selfhost` M4 bypass, and Python asm I/O peer stubs — so **more self-host bytes sit under three-chain DDC + Lock**, and **fewer paths are green only because an opaque Rust host artifact said so**.

**Honest boundary:** DDC detects **output divergence** across independent peers under equal inputs. That is a practical trust bar — **not** Thompson immunity or proof of purity.

---

## What IS in v0.4

**Product identity (honest):** An **auditable x86-64 compiler ISA** that keeps the v0.3 H_00 / JS peer / Win pure-M4 baseline, then **contracts the embedded Rust runtime face**, **closes Linux pure M4 without `bootstrap --selfhost`**, and **aligns asm peer platform I/O** with Rust/JS for DDC observability.

### Core deliverables (increment over v0.3)

| Area | v0.4 includes |
|------|----------------|
| **Runtime.dll surface (Stage 10-A)** | Fail-closed size gate: DLL **485888→231936** B (`stage10-runtime-surface.ps1` MAX **250000**); genN PE **576512→322560**; gen12 window still **18432** B, SHA `b609a735`→`43ffde58` |
| **Linux ELF H_00 / pure M4 (Stage 10-B)** | `stage10-linux-pure-m4.sh`: gen1→gen4 **no** `bootstrap --selfhost`; gen4≡gen3_direct full-ELF DDC EQUAL (`085d07d4…` · 704512 B) |
| **asm peer platform I/O (Stage 10-C)** | `platform_io.py` win32 byte-equal Rust/JS `0x20/0x50/0x51`; linux ALLOC peer-checked; `stage10-asm-peer-io.ps1` |
| **v0.3 baseline retained** | golden · backends · ddc · lock · gen12 · fullbody · stage5 · stage9 H_00 / JS peer / Win pure M4 — all still green |

### Trust-chain anchors (documented SHA)

| Monitor | Value | Notes |
|---------|-------|-------|
| **Lock pin (`yoyo.ty`)** | `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` | Decision #25 — **unchanged** (Stage 10 did not edit locked source) |
| **gen12 / fullbody `.text` (Win PE)** | SHA prefix **`43ffde58`** · full `43ffde5827c73fda7abd51c11121753c83fa5ef49aaff071cec5d411b51a58f0` | **18432**-byte compared window |
| **Embedded runtime.dll** | size **231936** · SHA `606f5e7091b5f51661e321229f6e91aaa907d82aded5350bc688a349e0c33b04` | Still Rust-built; **outside** gen12 window |
| **M4 gen4 parity (Win pure H_00)** | Same **`43ffde58`** window — gen4≡gen3_direct EQUAL | `stage9-pure-m4.ps1` |
| **M4 gen4 parity (Linux ELF pure)** | SHA prefix **`085d07d4`** · 704512 B full-ELF | `stage10-linux-pure-m4.sh` (no `--selfhost`) |

### Lock / Relock (v0.4 graduation)

Stage 10 **A/B/C did not modify `yoyo/projects/yoyo.ty`**. Trust gains came from **runtime build posture**, **ELF H_00 trampoline / link path**, and **asm peer emit** — not a source-body edit. Therefore:

- **No Relock required** — Decision #25 pin remains authoritative
- **Verified 2026-08-28:** `verify-lock-pin.ps1` exit 0 · `verify-yoyo-ty.mjs` exit 0 · `yoyo test lock` exit 0
- v0.4 graduation documents **smaller host DLL face + Linux pure M4 + three-chain I/O parity**; the Lock pin still locks the **788-handler source artifact**

### A/B/C trust gains (one line each)

| Door | Trust gain |
|------|------------|
| **A** | Host-compiled runtime.dll bytes cut ~half and fail-closed; “green” can no longer hide an unbounded DLL bloat |
| **B** | Linux M3→M4 algebra completes without Rust host `--selfhost` orchestration |
| **C** | asm↔Rust/JS win32 I/O handler bytes EQUAL — closes the last peer stub blind zone on production PE path |

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
.\scripts\stage10-asm-peer-io.ps1
.\scripts\stage9-pure-m4.ps1
.\scripts\stage5-win-selfhost.ps1
wsl bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh
```

### Docs that belong in v0.4

- `SCOPE-v0.4.md` — v0.4 boundary one-pager
- `STAGE10_OWNER_CHECKLIST.md` — Stage 10 A→D graduation board
- `RELEASE-v0.4.md` — this file
- `RELEASE-NOTES-v0.4.md` — short external notes
- Pinned artifacts unchanged: `yoyo/tests/yoyo.ty.lock` (Decision #25)

---

## What MUST NOT be published / claimed

### Still OUT (ROADMAP / later)

| Item | Why OUT |
|------|---------|
| **MCU/chip as main product track** | `custom-mcu` scaffold only |
| **C/Rust/Go replacement** | No struct/GC/async/module system |
| **Thompson-proof / DDC ⇒ provably correct** | Forbidden claim |
| **G06+ full golden suite** | Beyond v0.4 conformance |
| **macOS production gate** | MAY work; not required for v0.4 graduation |
| **YOYO-built runtime (non-Rust)** | Still Rust-compiled DLL/`.so` |
| **TheoryManifest / CDS theater** | N.4.1 FORBIDDEN |

### Remaining surface (honest — not blocking v0.4)

| Item | Status |
|------|--------|
| **Embedded Rust runtime still present** | `yoyo_runtime.dll` / `libyoyo_runtime.so` still Rust-built and embedded; bytes **outside** gen12 18432-byte `.text` window |
| **LoadLibrary / libdl host trampoline** | H_00 still extracts + loads host-built runtime |
| **Full-body section-ddc may DIFF** | H_00 / IAT width / embedded runtime can still diverge across peers on whole-PE compare |
| **Seed / reference host** | Pure M4 still seeds via `yoyo link` + `bootstrap` (without `--selfhost`) for gen3_direct |

### Misleading claims — forbidden in v0.4 release notes

Do **not** publish wording that implies:

- “Thompson-proof” or “immune to compiler backdoors”
- DDC covers **every** byte in genN PE (embedded runtime DLL is outside gen12 window)
- Runtime is YOYO-built / free of Rust host trust
- v0.4 is a daily-use application language or C replacement

### Temp / debug artifacts — never publish

```
scripts/_stage8-*/
scripts/_stage9-*/
scripts/_stage10*/
scripts/_tmp*
yoyo-rust/target-nosidecar/
yoyo-rust/target-selfhost-build/
yoyo-test/
```

---

## Pre-publish checklist

### 1. Verify green (exit code 0 everywhere)

Run the machine-checkable gates block above.

### 2. Confirm pin integrity

- `yoyo/tests/yoyo.ty.lock` SHA matches live `yoyo/projects/yoyo.ty`
- Pin: `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` (Decision #25, unchanged)

### 3. Stage 10 four doors

- [x] A — runtime.dll surface contraction
- [x] B — Linux ELF H_00 / pure M4
- [x] C — Python asm peer platform I/O
- [x] D — v0.4 graduation gate + docs

### 4. Release notes honesty pass

- [x] No ROADMAP items listed as “done in v0.4”
- [x] DDC described as **detection**, not proof
- [x] Remaining surface (Rust runtime still embedded / LoadLibrary host) noted
- [x] gen12 window boundary (**18432** bytes, `43ffde58`) documented
- [x] No temp `_stage*` / `_tmp*` dirs in artifact

---

## One-line pitch (external)

**YOYO v0.4 shrinks the remaining host-trust face — smaller embedded runtime, Linux pure M4 without `--selfhost`, asm I/O peer parity — expanding three-chain DDC + Lock coverage without claiming Thompson proof or replacing C.**

---

*Maintainer: update when Stage 10 gates or trust-chain SHA monitors change. v0.4 graduation: 2026-08-28 · Stage 10 A/B/C/D all green.*
