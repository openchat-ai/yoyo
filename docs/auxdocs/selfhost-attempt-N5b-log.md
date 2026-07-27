# W-START Attempt N5b Log · RUNTIME EXECUTOR BEAT (EXPERIMENTAL · NOT GREEN)

> Tag: `attempt-N5b-EXPERIMENTAL-runtime-executor` · Timestamp: 2026-07-24 (UTC+8)
> W-START: `EXPERIMENTAL` · Ref: `docs/auxdocs/selfhost-start-node.md`,
> `docs/auxdocs/selfhost-attempt-N-final-log.md`,
> `docs/auxdocs/selfhost-attempt-N5-log.md`
> Status: **EXPERIMENTAL · FAIL-CLOSED · NON-GREEN**
> (≠ Phase 2 ≠ freeze ≠ full self-host ≠ runtime parity ≠ cross-peer)

## TL;DR (one sentence)

A small Rust bytecode emulator (`yoyo-rust/executor/`, sibling of
`verifier/`) was added; it executes the N5 canaries (canary-A/B
fail-closed on NULL, canary-C clean HALT) and the locked `yoyo.ty`
entry path (3-step HALT, state[0x50]=0, no RAW_BYTE-fault reached) —
**Rust-side only**; the JS side is still MISSING (no JS interpreter
exists in the repo, per N5 preflight), and no claim of runtime
parity, DDC, freeze, or self-host is made.

## Inputs

- Lock pin: `b830a7f5074d814c320be0dc337874170c31d735059d7c4f3afc9e6545a1e685`
  (unchanged from N1–N5).
- Toolchain: `rustc 1.96.0`, `cargo 1.96.0`.
- Trust anchors UNTOUCHED: `yoyo.ty`, `*.lock`, `yoyo-js/src/*`,
  `yoyo-rust/verifier/src/*`, `yoyo-rust/libyoyo/src/*`,
  `yoyo-rust/platform/src/*`, `expected/*.code.hex`, `yoyo-asm/*`,
  `PROMPT-v3.md`, the N1–N5 logs and N5 preflight artifacts.

## Hard-scope decision matrix (chosen)

1. **Where**: option (a) — new crate `yoyo-rust/executor/` sibling
   of `verifier/`. NOT added to workspace `members`; built via
   `--manifest-path yoyo-rust/executor/Cargo.toml`. The crate
   declares its own empty `[workspace]` table to keep itself out
   of the verifier's workspace.
2. **Opcode subset**: the minimal set needed to run N5 fixtures
   and the locked `yoyo.ty` start-up: `movabs rax,imm64` /
   `mov [r15+disp32], rax` (store_state) / `mov rax,[r15+disp32]`
   (load_state) / `movzx rax,byte[rax]` (LDB load) /
   `add/sub rax,imm8|imm32` / `inc rax` / `dec rax` /
   `add/sub/or r64,r64` / `imul r64,r64` / `cmp r64,r64` /
   `jmp rel32` / `jcc rel32` (10 conditions) / `call rel32` /
   `ret` / `nop`. `0x84/0x85` (MEMCPY) are documented as
   NOT_IMPLEMENTED (verifier emits `0xC3` stub; locked yoyo.ty
   does not use them). Anything else → fail-closed `Unimplemented`.
3. **MMU model**: flat `Vec<u8>` region, base = `0x1000`, r15 =
   base. State slots are at `r15 + slot*8`. NULL deref (`addr < base`)
   and OOB → `Fault::ReadOob` / `Fault::WriteOob`. No
   memory-mapped I/O. The `0x20/0x50/0x51` platform ops are
   honored at the *byte* level (they emit `movabs+store` per the
   D-1 stub) but no actual platform syscall is performed.

## Phase A — Crate scaffold (≤10 calls)

- `yoyo-rust/executor/Cargo.toml` (declared `[workspace]`; only
  one external dep: `hex = "0.4"`).
- `yoyo-rust/executor/src/lib.rs` (top-level dispatch:
  `run_bytes`, `run_hex_text`, `RunOutcome`, `ExitReason`).
- `yoyo-rust/executor/src/mmu.rs` (flat `Vec<u8>`; `Fault` enum:
  `ReadOob` / `WriteOob` / `ExecOob` / `Decode` / `Unimplemented` /
  `Diverged` / `StepLimit`).
- `yoyo-rust/executor/src/cpu.rs` (register file: rax, rcx, r15,
  EFLAGS; 1-deep shadow call stack; 13x subset decoder).
- `yoyo-rust/executor/src/main.rs` (CLI: `run <path.bin>` /
  `run-hex <path.hex>` / `smoke`).
- Build: `cargo build --manifest-path yoyo-rust/executor/Cargo.toml`
  → `Finished dev profile`.

## Phase B — Unit tests + canary runs (≤10 calls)

`cargo test --manifest-path yoyo-rust/executor/Cargo.toml` →
8/8 pass:

| test | outcome |
|---|---|
| `nop_ret_halts` | ok |
| `movabs_store_ret` | ok |
| `movzx_inc_store_ret` | ok |
| `jmp_backward_to_nop` | ok |
| `jmp_je_taken` | ok |
| `decode_fault_on_unknown_opcode` | ok |
| `hex_text_roundtrip` | ok |
| `raw_byte_nop_chain_halts` | ok |

Three synthesized canaries (`scripts/_probe/_attempt_n5b/canaries/`)
run via `scripts/_probe/_attempt_n5b/run-all.cmd` →
`scripts/_probe/_attempt_n5b/out/canary-*.stripped.bin` →
`cargo run --manifest-path yoyo-rust/executor/Cargo.toml --bin yoyo-exec-run -- run <bin>`:

| canary | intent | Rust outcome | Rust exit | expected |
|---|---|---|---|---|
| canary-A | SET 0 + LDB NULL | `FAULT read OOB at 0x0 (1B)` after 4 steps | 1 | fail-closed per §4S.3 |
| canary-B | SET 0x7F + LDB NULL | `FAULT read OOB at 0x0 (1B)` after 4 steps | 1 | fail-closed; LDB target is `state[0x60]` (=0) not `state[0x50]`, so the SET 0x7F does not change the fault |
| canary-C | INC×3 + RET (control flow) | `HALT at 0x1034 after 10 steps, rax=0x3` | 0 | clean HALT with rax=3 |

JS column: **MISSING** — no JS interpreter exists; per brief
Phase C pick (a), the JS comparison is recorded as MISSING, not
synthesized. N5 preflight already established this.

## Phase C — Side-by-side honesty

This attempt is **Rust-side observation only**. The brief's option
(a) was chosen: do not synthesize a JS interpreter under the W-START
scope; record Rust outcomes honestly and mark JS as MISSING. Adding
a JS interpreter would be a NEW COMPONENT that expands the trust
surface, which the brief explicitly warns against for this beat.

| canary | JS outcome | Rust outcome | verdict |
|---|---|---|---|
| canary-A | MISSING | FAULT read OOB at 0x0 | Rust fail-closed; JS pending |
| canary-B | MISSING | FAULT read OOB at 0x0 | Rust fail-closed; JS pending |
| canary-C | MISSING | HALT, rax=3 | Rust clean; JS pending |

A full Rust-side pass over 19 golden fixtures (entry-handler-only
executions) was also run via
`scripts/_probe/_attempt_n5b/run-fixtures.ps1`. Every fixture
enters at H_00 (`SET state[0x50]=0; RET`-shaped) and HALTs at the
first RET in 3 steps. The non-entry-point handlers (H_01..) are
present in the .text but not reached because the entry handler
does not call/branch to them. **This is the same shape as the
locked yoyo.ty entry** and is **NOT** a proof of broader opcode
correctness; it only proves the entry-HANDLER dispatch path
works. (The unit tests in Phase B already exercise the wider
subset: JMP, JE, INC, MOVZX, etc.)

## Phase D — Locked `yoyo.ty` runtime

`scripts/_probe/_attempt_n5b/run-locked.cmd` compiles
`yoyo/projects/yoyo.ty` via the verifier (stub target) and pipes
the .text (931 bytes, matches N3 observation) into
`yoyo-exec-run run`. Outcome:

```
steps : 3
rax   : 0x0000000000000000
rcx   : 0x0000000000000000
r15   : 0x0000000000001000
exit  : HALT at 0x1012 after 3 steps
```

The entry handler H_00 is `30 50 00; FF` (`SET state[0x50]=0; RET`),
which compiles to `movabs rax,0; store_state(0x50); ret` — three
x64 instructions. The executor executes those three and halts at
the empty `ret_stack`. The remaining 33 handlers (H_01..H_21) are
present in the .text but are not entered because the locked source
does not CALL or JMP to them from H_00.

**No "missing 0xA0 opcode" fault was reached**, contrary to the
brief's prediction. The reason: `0xA0` and `0xA1` are *YOYO*
opcodes (`RAW_BYTE` / `RAW_BYTES`); at the x64 layer the
verifier just emits the bytes following the opcode directly into
.text, so the runtime only ever sees the raw emitted bytes (NOPs
`0x90` and RET `0xC3` for handlers H_05..H_10). This is itself
a small diagnostic observation about the locked yoyo.ty: the
RAW_BYTE chain handlers are byte-equivalent to a NOP/RET chain
at the x64 layer and run cleanly if entered.

## Phase E — Log + finalize

This file. Companion JSON: `scripts/_probe/_attempt_n5b/_outcomes.json`.

## What's still RED (verbatim from `PROMPT-v3.md`, unchanged)

full compiler self-host · 3-chain `section-ddc` 实现 · G06 · Phase 2 出口 ·
冻结编译器 · M-morph · Phase 4c libyoyo · gen1≡gen2 · CI

New red surfacing in this attempt (not in the PROMPT line):

- **JS executor MISSING** — runtime parity can only be claimed
  once a JS-side interpreter is built. Until then, the canary
  comparison table has only a Rust column. Adding a JS
  interpreter is a separate beat that the brief asked us to
  flag but not to take on here.

No item was promoted to GREEN. The W-START N-series remains
EXPERIMENTAL.

## Files touched

| file | role |
|---|---|
| `yoyo-rust/executor/Cargo.toml` | new crate manifest (workspace-local) |
| `yoyo-rust/executor/src/lib.rs` | new top-level dispatch |
| `yoyo-rust/executor/src/cpu.rs` | new register file + subset decoder |
| `yoyo-rust/executor/src/mmu.rs` | new flat-bytes Mmu + Fault enum |
| `yoyo-rust/executor/src/main.rs` | new CLI (`yoyo-exec-run`) |
| `scripts/_probe/_attempt_n5b/canaries/canary-A.ty` | new canary fixture |
| `scripts/_probe/_attempt_n5b/canaries/canary-B.ty` | new canary fixture |
| `scripts/_probe/_attempt_n5b/canaries/canary-C.ty` | new canary fixture |
| `scripts/_probe/_attempt_n5b/run-all.cmd` | new driver (3 canaries) |
| `scripts/_probe/_attempt_n5b/run-fixture.cmd` | new driver (single fixture) |
| `scripts/_probe/_attempt_n5b/run-fixtures.ps1` | new driver (19 fixtures) |
| `scripts/_probe/_attempt_n5b/run-locked.cmd` | new driver (locked yoyo.ty) |
| `scripts/_probe/_attempt_n5b/_outcomes.json` | new machine-readable outcomes |
| `docs/auxdocs/selfhost-attempt-N5b-log.md` | this log |

No other files created or modified. No commit.

## Suggested next direction (one of)

1. **Add a JS-side interpreter** with the same opcode subset,
   then re-run the canary table with both columns filled. This
   is the smallest step that would let runtime parity be
   observed cross-peer.
2. **Add asm-peer runtime**: extend `yoyo-asm/` to accept a hex
   stream and execute it under the same subset (or a sibling
   subset), then do 3-chain runtime comparison.
3. **STOP N-series here** with the explicit acknowledgement that
   the canary beat is now a single-peer observation. Reopen only
   when the JS/asm-side runtime is in-tree.

The next direction the brief offered that was NOT taken is
"add a JS interpreter" — that is explicitly the next step the
audit should consider, not this attempt.
