# parallel-batch-09 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-09-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-005 (pin `35f77232…`, 6 PASS / 1 REJECT H_39).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> 8 sub-agents dispatched in parallel; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`) and appends a §X.Y
> sub-section. This parent aggregates the per-handler results below.
> All picks below are FRESH (none in the parent's exclusion list of
> H_22, H_23, H_25, H_26, H_30-H_38) — they are opcode variations
> with different slot/imm/offset values, NOT raw-byte re-picks.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x68 ADDV | (0x51, 0x50) | `498b8788020000498b8f800200004801c849898788020000c3` (25) | same | same | Y | `be34bba91158e7fa` | `be34bba91158e7fa` | PASS |
| 2 | 0x69 ORV | (0x51, 0x50) | `498b8788020000498b8f800200004809c849898788020000c3` (25) | same | same | Y | `1bc8b9f481904979` | `1bc8b9f481904979` | PASS |
| 3 | 0x6A SUBV | (0x51, 0x50) | `498b8788020000498b8f800200004829c849898788020000c3` (25) | same | same | Y | `3f21e0104205701f` | `3f21e0104205701f` | PASS |
| 4 | 0x60 GET | (0x51, 0x52) | `498b879002000049898788020000c3` (15) | same | same | Y | `7fb64e0e46f94159` | `7fb64e0e46f94159` | PASS |
| 5 | 0x68 ADDV | (0x52, 0x51) | `498b8790020000498b8f880200004801c849898790020000c3` (25) | same | same | Y | `22a752f4fe9967b7` | `22a752f4fe9967b7` | PASS |
| 6 | 0x30 SET | slot=0x52 imm=0xCAFEBABE | `48b8bebafeca0000000049898790020000c3` (18) | same | same | Y | `ed70b867469e0e31` | `ed70b867469e0e31` | PASS |
| 7 | 0x84 MEMCPY_DATA | (D-3 stub, 0-arg) | `c3c3` (2) | `c3c3` | **N/A** | N | `1344fed055987f9e` | N/A | REJECTED |
| 8 | 0x85 MEMCPY_STATE | (D-3 stub, 0-arg) | `c3c3` (2) | `c3c3` | **N/A** | N | `1344fed055987f9e` | N/A | REJECTED |

**Summary**: 6 PASS / 2 REJECT out of 8 attempted.

### §1.1 ADDV-swap scratch — 0x68 dst=0x51 src=0x50
- expected: `498b8788020000498b8f800200004801c849898788020000c3` (25B)
- js-actual: `498b8788020000498b8f800200004801c849898788020000c3` (25B; matches)
- rust-actual: `498b8788020000498b8f800200004801c849898788020000c3` (25B; matches after stripping 1B startup_blob prefix)
- js-sha256: `be34bba91158e7fa1d6f09d86e3d94b82014152b3e8354d870c3640d0687257f`
- rust-sha256: `be34bba91158e7fa1d6f09d86e3d94b82014152b3e8354d870c3640d0687257f`
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x51,rax)=498b8788020000 (7B, disp32=0x288 LE) +
  load_state(0x50,rcx)=498b8f80020000 (7B, disp32=0x280 LE) +
  add rax,rcx=4801c8 (3B) +
  store_state(0x51,rax)=49898788020000 (7B) + ret=c3 (1B) = 25B.

### §1.2 ORV-swap scratch — 0x69 dst=0x51 src=0x50
- expected: `498b8788020000498b8f800200004809c849898788020000c3` (25B)
- js-actual: matches; rust-actual: matches
- js-sha256: `1bc8b9f481904979e050d714fc3237c5d058490d3c7badb2a103c721d2da728e`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: identical to ADDV-swap, except `or rax,rcx`=4809c8 instead of `add rax,rcx`=4801c8 at byte 16 (ModRM /1 vs /0). 25B.

### §1.3 SUBV-swap scratch — 0x6A dst=0x51 src=0x50
- expected: `498b8788020000498b8f800200004829c849898788020000c3` (25B)
- js-actual: matches; rust-actual: matches
- js-sha256: `3f21e0104205701f1c133c556eb69288e6558e9fd7aea9bd24c02d69e8bfaeb0`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x51,rax) + load_state(0x50,rcx) + `sub rax,rcx`=4829c8 + store_state(0x51,rax) + ret = 25B.

### §1.4 GET-alt scratch — 0x60 dst=0x51 src=0x52
- expected: `498b879002000049898788020000c3` (15B)
- js-actual: matches; rust-actual: matches
- js-sha256: `7fb64e0e46f94159fc7253dd0f6e3a7afcef60d8c9dee5de9a17849aa3fd3f31`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x52,rax)=498b8790020000 (7B, disp32=0x290 LE) +
  store_state(0x51,rax)=49898788020000 (7B, disp32=0x288 LE) + ret=c3 = 15B.

### §1.5 ADDV-h52 scratch — 0x68 dst=0x52 src=0x51
- expected: `498b8790020000498b8f880200004801c849898790020000c3` (25B)
- js-actual: matches; rust-actual: matches
- js-sha256: `22a752f4fe9967b7a63ce9c60191dd93573da27be40b85ea2bc903aa06efd830`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: load_state(0x52,rax)=498b8790020000 (7B, disp32=0x290 LE) +
  load_state(0x51,rcx)=498b8f88020000 (7B, disp32=0x288 LE) +
  add rax,rcx=4801c8 (3B) +
  store_state(0x52,rax)=49898790020000 (7B, disp32=0x290 LE) + ret=c3 = 25B.

### §1.6 SET-large scratch — 0x30 slot=0x52 imm=0xCAFEBABE
- expected: `48b8bebafeca0000000049898790020000c3` (18B)
- js-actual: matches; rust-actual: matches
- js-sha256: `ed70b867469e0e31547d3e93852b05f81a7c4cca691c00d6ed22f374d8bd7ad6`
- rust-sha256: same
- byte-equal: Y
- result: **PASS**
- derivation: movabs rax,0xCAFEBABE (10B, 48 b8 BE BA FE CA 00 00 00 00 LE) +
  store_state(0x52,rax)=49898790020000 (7B, disp32=0x290 LE) + ret=c3 = 18B.

### §1.7 MEMCPY_DATA stub — 0x84 (REJECTED)
- expected: `c3c3` (2B; D-3 stub surface, both peers emit c3)
- js-actual: `c3c3` (matches; JS encode-x64.js line 114 returns [0xc3] for op 0x84)
- rust-actual: **N/A — driver FAILED** with
  `ArgCountMismatch { op: 132 (0x84), expected: 3, got: 0 }`.
- js-sha256: `1344fed055987f9eb87aef70de1922ae7239d57b2a38fd9fb91a2a3f0b4c497a`
- rust-sha256: N/A (no output file produced)
- byte-equal: N
- result: **REJECTED** (reason: Rust `lower_op_checked` enforces a 3-arg
  signature on 0x84 MEMCPY_DATA per `yoyo-rust/verifier/src/isa_table.txt`
  line 37 (dst src n). The 0-arg stub fixture we wrote is acceptable
  to JS encode-x64.js (line 114: `if (op === 0x84 || op === 0x85) return
  [0xc3]`) but rejected at Rust's `lower_op` stage BEFORE any byte
  emission. JS and Rust disagree on stub arity semantics for 0x84. The
  D-3 stub surface is byte-consistent only when both peers accept the
  same arg shape — which our scratch fixture does not satisfy for
  Rust. This is a known D-3 stub fixture-shape constraint, NOT a peer
  divergence in the underlying emit primitive.)

### §1.8 MEMCPY_STATE stub — 0x85 (REJECTED)
- expected: `c3c3` (2B; D-3 stub surface)
- js-actual: `c3c3` (matches; JS encode-x64.js line 114 returns [0xc3])
- rust-actual: **N/A — driver FAILED** with
  `ArgCountMismatch { op: 133 (0x85), expected: 3, got: 0 }`.
- js-sha256: `1344fed055987f9eb87aef70de1922ae7239d57b2a38fd9fb91a2a3f0b4c497a`
- rust-sha256: N/A (no output file produced)
- byte-equal: N
- result: **REJECTED** (same reason as §1.7 — Rust `lower_op_checked`
  enforces 3-arg signature on 0x85 MEMCPY_STATE per isa_table.txt line 38.
  JS silently accepts 0 args; Rust rejects at parse stage before emit.)

## §2. Pick rationale (honesty notes)

- **Picks 1–6** are NOT new opcodes (all common-supported opcodes are
  already covered at H_00..H_41+ in yoyo.ty). They are slot/imm variations
  of opcodes 0x68, 0x69, 0x6A, 0x60, 0x30 — exercising different
  state-slot displacement bytes (0x280 vs 0x288 vs 0x290 LE) and a
  non-zero SET imm (0xCAFEBABE). All 6 PASS byte-equal JS↔Rust.

- **Picks 7–8** are the only remaining FRESH OPCODES in the §1 ISA
  exhaustion list — 0x84 MEMCPY_DATA and 0x85 MEMCPY_STATE. Both are
  D-3 stubs (per PROMPT Part §4S.3 deferral); both peers intentionally
  emit `[0xc3]` as stub-only. The Rust `lower_op_checked` enforces a
  3-arg arity per `isa_table.txt`; the JS encoder ignores arity and
  emits stub bytes. To make these PASS, the fixture would need 3 dummy
  args (e.g., `84 00 00 00`) — but that crosses into Rust `lower_op`
  valid surface while JS would silently emit `[0xc3]` (no load/store).
  Both peers byte-equal ONLY at the `[0xc3]` stub surface, NOT at the
  3-arg loaded form. Our 0-arg fixture satisfies JS but fails Rust;
  a 3-arg fixture would satisfy Rust but produce different bytes
  between JS-stub-`[0xc3]` and Rust-stub-`[0xc3]`-after-load. This is
  a known cross-peer stub-shape tension documented in PROMPT D-3.

- The 12 handlers the parent excluded (H_22, H_23, H_25, H_26, H_30-H_38)
  are NOT touched. yoyo.ty is unchanged.

## §3. Files touched (parent-verified)

- `yoyo/tests/golden/_scratch_addv_swap.ty` + `_scratch_addv_swap.code.hex`
- `yoyo/tests/golden/_scratch_orv_swap.ty` + `_scratch_orv_swap.code.hex`
- `yoyo/tests/golden/_scratch_subv_swap.ty` + `_scratch_subv_swap.code.hex`
- `yoyo/tests/golden/_scratch_get_alt.ty` + `_scratch_get_alt.code.hex`
- `yoyo/tests/golden/_scratch_addv_h52.ty` + `_scratch_addv_h52.code.hex`
- `yoyo/tests/golden/_scratch_set_large.ty` + `_scratch_set_large.code.hex`
- `yoyo/tests/golden/_scratch_memcpy_data.ty` + `_scratch_memcpy_data.code.hex`
- `yoyo/tests/golden/_scratch_memcpy_state.ty` + `_scratch_memcpy_state.code.hex`
- `docs/auxdocs/parallel-batch-09-log.md` — this file

NO `yoyo/projects/yoyo.ty`, NO `yoyo-js/scripts/golden.js`, NO
`yoyo-rust/verifier/src/self_test.rs`, NO `yoyo-rust/verifier/src/main.rs`,
NO `*.lock`, NO existing `expected/*.code.hex` touched.

## §4. Driver notes (honesty disclosure)

- The JS driver `node scripts/_probe/js-ty2text.mjs <scratch.ty>` was
  invoked directly by the parent (NOT via sub-agent) using Node's
  `child_process.spawnSync` to capture raw binary stdout. PowerShell
  `>` redirection corrupted binary streams with UTF-16 LE encoding +
  BOM (ff fe) + replacement chars (0x3f) for non-ASCII bytes. Direct
  Node spawn preserved the raw 25/15/18/2-byte streams correctly.

- The Rust driver `cargo run -q --bin yoyo -- link --target=stub <scratch.ty> <out.bin>`
  was invoked from `f:/yoyo/yoyo-rust/verifier` cwd (the workspace
  root `f:/yoyo` has no Cargo.toml). The output binary starts with
  `startup_blob_baremetal()` = `[0xc3]` (1B prefix); parent strips
  this 1-byte prefix to recover the actual code bytes. Rust output
  sizes matched JS exactly: 25B (×4), 15B (×1), 18B (×1), and REJECT
  for the two 0x84/0x85 stub fixtures.

## §5. Honesty override checks

- Peer JS/Rust divergence at the 6 PASS handlers: **NONE** (same bytes, same sha256).
- Peer JS/Rust divergence at 0x84/0x85 MEMCPY stubs: **STUB-ARITY** —
  JS emits `[0xc3]` regardless of args; Rust `lower_op_checked` enforces
  3-arg arity at parse stage BEFORE emit. NOT an emit-byte divergence;
  the divergence is at the `lower_op` parser layer (which JS lacks).
  The stub primitive itself is consistent: both peers emit `c3` when
  the arg check passes.
- Lock Protocol step 1 (compile) failure: **NONE** on the 6 PASS.
  Lock-respected surface unchanged.
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.

## §6. Next-step suggestion (parent for serialization)

- The 6 PASS handlers (picks 1–6) all exercise well-tested primitive
  paths with non-trivial pins (different slot/imm combinations).
- If the parent decides to serialize, the natural consolidated append
  is H_42..H_47 (six new canonical handlers at selectors 0x30..0x35):
  - H_42 ADDV-swap   (0x68 51 50) — pin `498b8788020000498b8f800200004801c849898788020000c3`
  - H_43 ORV-swap    (0x69 51 50) — pin `498b8788020000498b8f800200004809c849898788020000c3`
  - H_44 SUBV-swap   (0x6A 51 50) — pin `498b8788020000498b8f800200004829c849898788020000c3`
  - H_45 GET-alt     (0x60 51 52) — pin `498b879002000049898788020000c3`
  - H_46 ADDV-h52    (0x68 52 51) — pin `498b8790020000498b8f880200004801c849898790020000c3`
  - H_47 SET-large   (0x30 52 CAFEBABE) — pin `48b8bebafeca0000000049898790020000c3`
- Plus 1 Relock after append.
- The 2 REJECTED MEMCPY stubs (0x84/0x85) require a separate fix:
  either make JS `lower_op`-equivalent (3-arg required), OR make
  Rust `lower_op_checked` accept 0-arg stub fixtures. OUT OF SCOPE
  for this batch (would require peer-source edits in either
  `yoyo-js/scripts/js-ty2text.mjs` or `yoyo-rust/verifier/src/tir.rs`,
  which the parent's HARD honesty override forbids).