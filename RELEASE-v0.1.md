# YOYO v0.1 — Release Boundary (Owner One-Pager)

> **Rule:** Only v0.1 scope may be published. Anything labeled **SCOPE-CUT**, **EXPERIMENTAL**, **ROADMAP**, or stored as temp/debug artifacts **must not ship** with a v0.1 tag.
>
> **Sources:** `PROMPT-v3.md` (Part L, Week axis, N.4 Forbidden), `STAGE4_OWNER_CHECKLIST.md`, `BACKEND_SUPPORT.md` · baseline 2026-08-26.

---

## What IS in v0.1

**Product identity (honest):** A **verifiable, compiler-specialized ISA and toolchain** for x86-64 — 38 core opcodes, 256-slot state machine, three **independent** peer implementations compared by Diverse Double-Compiling (DDC). YOYO detects cross-peer divergence under independence assumptions; it does **not** prove compiler purity.

### Core deliverables

| Area | v0.1 includes |
|------|----------------|
| **Language** | Layer-S `.ty` grammar, 38-op ISA table, operational semantics (Parts 4 / 4S / G in `PROMPT-v3.md`) |
| **4 projects** | `yoyo/` (locked source), `yoyo-js/`, `yoyo-rust/` (verifier + libyoyo), `yoyo-asm/` (independent asm peer) |
| **Locked compiler body** | `yoyo/projects/yoyo.ty` pinned by `yoyo/tests/yoyo.ty.lock` (Decision #25) |
| **3-chain DDC** | JS == Rust == Python asm section-ddc EQUAL (SHA-256: `4fb8b87f`); gen1≡gen2 via `test gen12` |
| **Conformance** | Golden **739/739** PASS (Appendix F fixtures G00–G05 + extended emit matrix) |
| **Cross-arch verification** | DDC fixtures `00_nop_ret` … `04_ldb_ptr` + container PE/ELF — all **PASS** (see `BACKEND_SUPPORT.md`) |
| **Backends** | **37/37** compile+link smoke (`yoyo test backends`); includes win32/linux x64 production paths |
| **Lock lifecycle** | 8-step Lock Protocol, `test lock`, `scripts/verify-lock-pin.ps1`, `scripts/verify-yoyo-ty.mjs` |
| **Self-host (scoped)** | Windows M2→M3 (`scripts/stage5-win-selfhost.ps1`, no-sidecar path green); Linux M2→M3 (`scripts/stage5-linux-selfhost.sh`) |
| **MCU hook (scaffold only)** | `--target=custom-mcu` copy-and-replace scaffold + smoke DDC — **not** a finished chip backend |

### Machine-checkable gates (all must exit 0 before publish)

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all          # golden + backends + ddc
cargo run -- test lock

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\verify-yoyo-ty.mjs
node .\yoyo-js\scripts\golden.js
.\scripts\stage5-win-selfhost.ps1
# Linux/WSL when applicable:
# bash scripts/stage5-linux-selfhost.sh
```

### Docs that belong in v0.1

- `PROMPT-v3.md` — normative spec (Parts N, L, 4, 4S, G, 5–9, F, Deduce, Gnd, Appendix F/G)
- `BACKEND_SUPPORT.md` — backend + DDC matrix
- `AGENTS.md` — maintainer debugging notes (optional but in-repo)
- Golden hash files under `docs/GOLDEN_HASH_*.txt` when present
- Pinned artifacts: `yoyo/tests/yoyo.ty.lock`, golden fixtures under `yoyo/tests/golden/`

---

## What MUST NOT be published

### SCOPE-CUT (explicit — do not advertise as v0.1)

| Item | Why OUT |
|------|---------|
| **W5.5 — full `yoyo.ty` body + libyoyo migration** | Marked **SCOPE-CUT** in `PROMPT-v3.md` Week 5; Phase 4c migration is post–v0.1 |
| **Phase 2 “≤1500 lines full body” exit** | Future compiler completeness gate, not v0.1 deliverable |
| **Full compiler self-host as product claim** | M2→M3 paths are green for **scoped** startup; “full body self-hosting compiler” remains SCOPE-CUT |
| **G06 and beyond** | Not in v0.1 conformance claim set |

### EXPERIMENTAL (in repo for history — exclude from release tarball/tag)

| Item | Location / label |
|------|------------------|
| **W-START NODE** | `PROMPT-v3.md` Week axis · `docs/auxdocs/selfhost-start-node.md` — `EXPERIMENTAL · OUT-OF-v0.1-body` |
| **body-extend queue + attempt logs** | `docs/auxdocs/body-extend-*.md`, `parallel-batch-*.md`, `selfhost-attempt-*.md`, `*-SPAWN.md` (~400 files) |
| **W-SM closure / emit matrix memos** | `docs/auxdocs/wsm-closure-memo.md`, `selfhost-emit-matrix.md`, etc. — anti-rewrite context, not product spec |
| **Internal stage owner checklist** | `STAGE4_OWNER_CHECKLIST.md` — daily maintainer board; optional in public repo, **not** a v0.1 feature list |

### ROADMAP / NON-NORMATIVE (may stay in full repo clone; do not sell as shipped v0.1)

- `PROMPT-v3.md` Part 12 (SIMD), Part 15–16 (demos / master roadmap), Appendix H (future deduction substrate)
- Part E morph as **product** (interfaces may exist; do not claim Thompson-proof or “any environment” morph)
- Space-grade / radiation-hardened / flight-software narratives (Part 15.8 / 16.5)
- TheoryManifest / DeriveTick / CDS daemon theater (N.4.1 forbidden for v0.1)
- Trit conventions (Part 4.6) — application convention, not acceptance criteria
- `scripts/check-sugar.mjs` — ROADMAP / fail-closed until real

### Misleading claims — forbidden in v0.1 release notes

Do **not** publish wording that implies:

- “Thompson-proof” / “DDC ⇒ output provably correct”
- “Frozen compiler” meaning **full-body** freeze (W5.5 is still SCOPE-CUT)
- `custom-mcu` is a **production MCU backend** (it is a **scaffold** — extend emit + interp before promotion)
- 37 backends = 37 **fully interpreted** MCUs (many are emit-only or stub; see matrix Legend in `BACKEND_SUPPORT.md`)
- C/Rust/Go replacement or general-purpose PL

### Temp / debug artifacts — never publish

These exist locally as untracked scratch; **exclude from tag, tarball, and npm/cargo publish**:

```
# Root scratch
build-out.txt
build_log.txt
target-selfhost-build.log
gen_mcu_helpers.py
patch_main_ddc.py
mcu_ddc_helpers.rs.txt

# Agent / CI capture logs
scripts/_*.txt

# Local test harness dirs (ELF/exe/log dumps)
scripts/_stage5-win/
scripts/_stage5-linux/
scripts/_stage5-chain/

# Non-standard Cargo output trees
yoyo-rust/target-coord/
yoyo-rust/target-selfhost/

# Python cache
**/__pycache__/
**/*.pyc

# Build products (already in .gitignore — verify absent from release)
**/target/
*.exe *.dll *.so *.o *.obj *.pdb
```

---

## Pre-publish checklist

### 1. Verify green (exit code 0 everywhere)

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo run -- test all
cargo run -- test lock
cargo run -- test gen12

cd F:\yoyo
.\scripts\verify-lock-pin.ps1
.\scripts\verify-yoyo-ty.mjs
node .\yoyo-js\scripts\golden.js
.\scripts\stage5-win-selfhost.ps1
```

### 2. Confirm pin integrity

- `yoyo/tests/yoyo.ty.lock` SHA matches live `yoyo/projects/yoyo.ty`
- Documented pin: `0275802d2b4459e6ece0801a73af3e988d203c6a34dacfa382f2e48fe8ad43cb` (Decision #25)

### 3. Exclude paths (release script / `git archive` / manual review)

Use an exclude list equivalent to:

```
scripts/_*
scripts/_stage5-*
build-out.txt
build_log.txt
target-selfhost-build.log
gen_mcu_helpers.py
patch_main_ddc.py
mcu_ddc_helpers.rs.txt
yoyo-rust/target-coord
yoyo-rust/target-selfhost
**/__pycache__
```

Optional: omit entire `docs/auxdocs/` from **public** v0.1 artifact (EXPERIMENTAL attempt history); keep in private/full maintainer clone if needed.

### 4. Release notes honesty pass

- [ ] No SCOPE-CUT / EXPERIMENTAL / ROADMAP items listed as “done in v0.1”
- [ ] `custom-mcu` described as **scaffold**, not product backend
- [ ] DDC described as **detection**, not proof
- [ ] Full-body self-host and libyoyo migration explicitly **deferred**
- [ ] No temp log files or `target-coord/` trees in artifact

### 5. Build from clean tree

```powershell
cd F:\yoyo\yoyo-rust
cargo build --release -p verifier -p yoyo-runtime
cd F:\yoyo\yoyo-js
npm ci
```

Ship **source + spec + tests + scripts**; do not ship prebuilt `*.exe` / `target/` unless explicitly intended and rebuilt from tag.

---

## One-line pitch (external)

**YOYO v0.1 is an auditable x86-64 compiler ISA with three independent implementations and machine-checkable cross-peer verification—built for people who need to *detect* compiler-level divergence, not a general-purpose programming language.**

---

*Maintainer: update this file when Week axis SCOPE-CUT boundaries or green gates change. Do not conflate Stage 7 maintenance graduation with W5.5 full-body scope.*
