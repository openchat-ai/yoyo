# parallel-batch-83 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-83-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-088 (pin `697ad784…`, handlers = 643, H_629..H_636 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-088 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_636 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1E0 | `498b87000300004881c0e0010000480fb60049898790020000c3` (26) | same | same | Y | `a8e2361d68cd8eae` | `a8e2361d68cd8eae` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x50 imm=0x1E0 | `498b87800200004881c0e001000049898780020000c3` (22) | same | same | Y | `f8386b9a462dfb05` | `f8386b9a462dfb05` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0x1E0 | `498b87880200004881c0e001000049898788020000c3` (22) | same | same | Y | `1eba92f3a87f8de9` | `1eba92f3a87f8de9` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0x1E0 | `498b87900200004881c0e001000049898790020000c3` (22) | same | same | Y | `e15ba36fe8e77c0c` | `e15ba36fe8e77c0c` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x50 imm=0x1E0 | `498b87800200004881e8e001000049898780020000c3` (22) | same | same | Y | `485f29f7f7612705` | `485f29f7f7612705` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x51 imm=0x1E0 | `498b87880200004881e8e001000049898788020000c3` (22) | same | same | Y | `aceddcae0b9c827f` | `aceddcae0b9c827f` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0x1E0 | `498b87900200004881e8e001000049898790020000c3` (22) | same | same | Y | `1641521a26d49973` | `1641521a26d49973` | PASS |
| 8 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1E8 | `498b87000300004881c0e8010000480fb60049898780020000c3` (26) | same | same | Y | `6089535af769e9fe` | `6089535af769e9fe` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x52 ss=0x60 oo=0x1E0 — **PASS**

- fixture: `_scratch_ldb_5260_1E0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e0010000480fb60049898790020000c3`
- js-sha256: `a8e2361d68cd8eae6c92e118c9e39d49a60d9a09ae5069255f07a61cfc6c5cd0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x50 imm=0x1E0 — **PASS**

- fixture: `_scratch_addimm_h50_1E0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0e001000049898780020000c3`
- js-sha256: `f8386b9a462dfb05f58cbf376c60a5c859566fdd49e199324cd680fe41c2ed09`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0x1E0 — **PASS**

- fixture: `_scratch_addimm_h51_1E0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0e001000049898788020000c3`
- js-sha256: `1eba92f3a87f8de92d8440f8638f1aa250783c5413bb0bd219fd3623324d8f8b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0x1E0 — **PASS**

- fixture: `_scratch_addimm_h52_1E0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0e001000049898790020000c3`
- js-sha256: `e15ba36fe8e77c0ceee37fb2cddd486e6aa388f0986b8a8c99f3fd69bf0c2aeb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x50 imm=0x1E0 — **PASS**

- fixture: `_scratch_subimm_h50_1E0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8e001000049898780020000c3`
- js-sha256: `485f29f7f7612705dbad5255c877b24d6e90c20ddba3266039bc4709b01836e7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x51 imm=0x1E0 — **PASS**

- fixture: `_scratch_subimm_h51_1E0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8e001000049898788020000c3`
- js-sha256: `aceddcae0b9c827f4a8c54a6b402b433a8793dd97eb123aa66f110007b721a39`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0x1E0 — **PASS**

- fixture: `_scratch_subimm_h52_1E0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8e001000049898790020000c3`
- js-sha256: `1641521a26d49973a6b927072b1f7d18f933f02e7f527f704a8fdb52e1185779`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x50 ss=0x60 oo=0x1E8 — **PASS**

- fixture: `_scratch_ldb_5060_1E8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e8010000480fb60049898780020000c3`
- js-sha256: `6089535af769e9fe003c54c3f2ec91e0d295c3d4cfaf4e6d8348fef1cdef0d6e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=52 ss=60 oo=1E0 (finish deferred 1E0 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=1E0 (start deferred 1E0 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=1E0 (start deferred 1E0 SUB triad; imm32 22B).
- LDB dd=50 ss=60 oo=1E8 (start next 1E8 LDB ladder; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 283`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5260_1E0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1E0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1E0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1E0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1E0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1E0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1E0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_1E8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-83-log.md` — this file
- `scripts/_probe/parallel-batch-83-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-089 serialize PASSes + 1 Relock**

Pass pin from body-extend-088 Relock: `697ad7847ba15e825ee7a2663be37eb71de542256a38f42ed2e7dc16ddca549c`.
Handlers before consolidate = 643 (H_00..H_636). Next selectors `40 283`.. for H_637.. if all serialize.

PASS list for body-extend-089:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_637 | 0x283 | 0x80 LDB | 0x52 0x60 0x1E0 | `498b87000300004881c0e0010000480fb60049898790020000c3` (26B) | `a8e2361d68cd8eae` |
| H_638 | 0x284 | 0x62 ADD-IMM | 0x50 0x1E0 | `498b87800200004881c0e001000049898780020000c3` (22B) | `f8386b9a462dfb05` |
| H_639 | 0x285 | 0x62 ADD-IMM | 0x51 0x1E0 | `498b87880200004881c0e001000049898788020000c3` (22B) | `1eba92f3a87f8de9` |
| H_640 | 0x286 | 0x62 ADD-IMM | 0x52 0x1E0 | `498b87900200004881c0e001000049898790020000c3` (22B) | `e15ba36fe8e77c0c` |
| H_641 | 0x287 | 0x61 SUB-IMM | 0x50 0x1E0 | `498b87800200004881e8e001000049898780020000c3` (22B) | `485f29f7f7612705` |
| H_642 | 0x288 | 0x61 SUB-IMM | 0x51 0x1E0 | `498b87880200004881e8e001000049898788020000c3` (22B) | `aceddcae0b9c827f` |
| H_643 | 0x289 | 0x61 SUB-IMM | 0x52 0x1E0 | `498b87900200004881e8e001000049898790020000c3` (22B) | `1641521a26d49973` |
| H_644 | 0x28A | 0x80 LDB | 0x50 0x60 0x1E8 | `498b87000300004881c0e8010000480fb60049898780020000c3` (26B) | `6089535af769e9fe` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-088 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_636.
- If the parent decides to serialize, append H_637.. at selectors `40 283`..:
  - H_637 0x80 LDB (80 52 60 1E0) — pin `498b87000300004881c0e0010000480fb60049898790020000c3`
  - H_638 0x62 ADD-IMM (62 50 1E0) — pin `498b87800200004881c0e001000049898780020000c3`
  - H_639 0x62 ADD-IMM (62 51 1E0) — pin `498b87880200004881c0e001000049898788020000c3`
  - H_640 0x62 ADD-IMM (62 52 1E0) — pin `498b87900200004881c0e001000049898790020000c3`
  - H_641 0x61 SUB-IMM (61 50 1E0) — pin `498b87800200004881e8e001000049898780020000c3`
  - H_642 0x61 SUB-IMM (61 51 1E0) — pin `498b87880200004881e8e001000049898788020000c3`
  - H_643 0x61 SUB-IMM (61 52 1E0) — pin `498b87900200004881e8e001000049898790020000c3`
  - H_644 0x80 LDB (80 50 60 1E8) — pin `498b87000300004881c0e8010000480fb60049898780020000c3`
- Plus 1 Relock after append from pin `697ad784…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-089 serialize PASSes + 1 Relock
