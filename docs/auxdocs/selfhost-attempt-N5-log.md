# W-START Attempt N5 Log · RUNTIME CANARY (FAIL-CLOSED, NON-GREEN)

> Tag: `attempt-N5-EXPERIMENTAL-runtime-canary` · Timestamp: 2026-07-24 (UTC+8)
> W-START: `EXPERIMENTAL` · Ref: `docs/auxdocs/selfhost-start-node.md`,
> `docs/auxdocs/selfhost-attempt-N-final-log.md`
> Status: **EXPERIMENTAL · FAIL-CLOSED · NON-GREEN**
> (≠ Phase 2 ≠ freeze ≠ full self-host ≠ runtime parity)

## TL;DR (one sentence)

Neither JS nor Rust peer has a bytecode executor / runtime; the "runtime
canary" beat the audit proposed is not executable inside the current M0 +
verifier surface, so this attempt STOPPED at the pre-flight check per the
hard rule *"any … run error → STOP and report"* — no synthesized fixtures,
no observed crash, no behavior diff, no claim of any kind.

## Inputs

- Lock pin: `b830a7f5074d814c320be0dc337874170c31d735059d7c4f3afc9e6545a1e685`
  (unchanged from N1–N4).
- Toolchain: `rustc 1.96.0`, `cargo 1.96.0`, `node v24.14.0`.
- Trust anchors UNTOUCHED: `yoyo.ty`, `*.lock`, `yoyo-js/src/*`,
  `yoyo-rust/src/*`, `expected/*.code.hex`, `yoyo-asm/*`, `PROMPT-v3.md`,
  `scripts/_probe/js-ty2text.mjs`, the prior N1–N4 logs.
- Files added: `scripts/_probe/_attempt_n5/README.md`,
  `scripts/_probe/_attempt_n5/_preflight.json`, this log.

## Phase A — Synth fixtures: SKIPPED (pre-flight failure)

Per the brief, this phase would have produced three `.ty` files
(`canary-A.ty` SET 0 + LDB NULL, `canary-B.ty` SET 0x7F + LDB poison,
`canary-C.ty` control-flow only). They were not written: a runtime canary
has no value if no runner exists, and writing fixtures first would have
violated the brief's *"any … run error → STOP and report"* clause.

## Phase B — JS runner: UNAVAILABLE

Inspected `yoyo-js/scripts/golden.js` and `yoyo-js/src/yoyo.js` for any
executor / emulator / `vm.run` / `child_process` usage:

| surface | contains `compileCode` | contains interpreter |
|---|---|---|
| `yoyo-js/scripts/golden.js` | yes (G00–G05 + LDB variants) | no |
| `yoyo-js/src/yoyo.js`     | yes (entry point) | no |
| `yoyo-js/src/platform/encode-x64.js` | yes (encoder only) | no |
| `yoyo-js/src/platform/pe-builder.js`, `elf-builder.js` | builders only | no |

JS M0's only runtime surface is `main()` which runs the disk-golden
harness. There is no `vm.runInNewContext(code, sandbox)` on the produced
bytes; the encoder/emit path produces `.text` only. A JS canary runner
would require writing a new JS interpreter for the encoder output — that
is a NEW COMPONENT, not a probe, and would change the trust surface.

## Phase C — Rust runner: UNAVAILABLE

`yoyo-rust/verifier/src/main.rs` CLI matrix (confirmed by re-reading
`usage()`):

```
yoyo link [--target=win32|linux|stub|baremetal] [--posture=...] [--morph=...] <in.ty> <out>
yoyo diff <a.bin> <b.bin>
yoyo hash <file>
yoyo selftest
yoyo render <input.ty>
yoyo test golden
```

There is no `run`, `exec`, `emulate`, or `interp` subcommand. The
`executor` module is named `executor::compile_ty_source` and is
compile-time only; it does NOT run the produced bytes. There is no
`executor::run`, no `cpu` / `mmu` / `memory` crate in the workspace
(`Cargo.toml` deps for `verifier`: `isa-proc`, `libyoyo`,
`yoyo-platform`, `sha2`, `hex` — none is an emulator).

Per the brief's Phase C step 7, the audit acknowledges Rust could be
asked to consume the locked `yoyo.ty` only. The locked `yoyo.ty` cannot
be substituted by a synth fixture without modifying the source — and
`yoyo.ty` is on the do-not-touch list. So even the fallback
"canary-C against locked `yoyo.ty`" path is **not viable for runtime**:
the verifier has no `run` action against `yoyo.ty` either.

## Phase D — Pre-flight scan

A small pre-flight scan was performed to record what tooling is and is
not present in the repo before this attempt terminated:

| scan | tool used | finding |
|---|---|---|
| grep `"run" \| "exec" \| "emulate"` over `yoyo-rust` | ripgrep | zero matches |
| grep `interpreter\|emulator\|memory` over `yoyo-js/src` | ripgrep | zero matches in `yoyo.js`, `golden.js`; only one false-positive in `elf-builder.js` (the word "memory" in a phase comment) |
| read `verifier/src/main.rs::usage()` | file read | five CLI commands only |
| read `verifier/src/executor.rs` for runtime symbols | file read + grep | `executor::compile_ty_source` present; no `run` / `exec` |

Output captured at `scripts/_probe/_attempt_n5/_preflight.json`.

## Per-canary diff verdict

| canary | JS outcome | Rust outcome | verdict |
|---|---|---|---|
| canary-A (SET 0; LDB [S[0x50]+0]) | **N/A** — no JS runtime | **N/A** — no Rust runtime | **NOT RUN** (fail-closed) |
| canary-B (SET 0x7F; LDB [S[0x50]+0]) | **N/A** | **N/A** | **NOT RUN** (fail-closed) |
| canary-C (control flow only) | **N/A** | **N/A** | **NOT RUN** (fail-closed) |

## Interpretation (what this attempt shows)

- The N-series' byte-compare template (N1–N4) was exhaustively applied;
  N-final concluded saturation and suggested a runtime canary as the
  "what next" item. The audit was correct that byte-identity ≠ runtime
  parity, but it implicitly assumed a runner exists.
- This pre-flight shows **the runtime beat does not fit the M0 + Rust
  verifier surface as-shipped**. Both peers *emit* bytes; neither
  *interprets* them.
- No claim of D-1 resolution, no claim of mismatch, no claim of
  parity. The beat's pre-condition (a runner) is unmet in the current
  repo. The canary cannot be brought up by writing fixtures, by passing
  CLI flags, or by adjusting the locked `yoyo.ty`.

## What's still RED (verbatim from `PROMPT-v3.md`, unchanged)

full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 ·
冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI

This attempt does **not** resolve any item. The bytecode executors required
to actually run a runtime canary are themselves still RED (no `executor::run`
in Rust, no JS interpreter in `yoyo-js/src`). That is a **new** RED
surfacing, distinct from the nine-item PROMPT line — see "Suggested next
direction" below.

## Suggested next direction (one of)

1. **Add a small emulator** (option A: Rust `executor::run` + a tiny
   `Mmu`/`Cpu` consuming the locked-stub encodings; option B: a JS
   interpreter via `vm.runInContext` on the byte buffer with a hand-rolled
   decoder for `movabs/load_state/store_state/inc/dec/jmp/jcc/ret`).
   Both new components must be reviewed against the trust anchor rules
   (no edits to `yoyo.ty` / `*.lock` / goldens / `PROMPT-v3.md`). After
   that lands, retry N5 with the three synthesized canaries.
2. **Drop the runtime template** and accept that the M0/verifier surface
   is byte-emit only — the W-START N-series would then be closed not
   because parity is established but because the canary beat is out of
   scope for the current layer. (Per the W-START scope, this is the
   STOP-N-SERIES-FOR-GOOD option the audit offered.)
3. **Switch the canary target**: instead of running the emitted bytes,
   compare the **two encoders' emit logic on the byte-equal streams**.
   N4 already proved byte-equality for D-1 ops 0x20/0x50/0x51 under the
   slot-by-name path; if that is the only access pattern, the runtime
   question is mooted by N4's observation (still not "D-1 resolved",
   still observation-only).

## Files touched

- `scripts/_probe/_attempt_n5/README.md` (new)
- `scripts/_probe/_attempt_n5/_preflight.json` (new)
- `docs/auxdocs/selfhost-attempt-N5-log.md` (this file)

No other files created or modified. No commit.
