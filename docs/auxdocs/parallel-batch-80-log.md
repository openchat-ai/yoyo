# parallel-batch-80 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-80-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-085 (pin `58b9ca6e…`, handlers = 619, H_605..H_612 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-085 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_612 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x52 imm=0x1C8 | `498b87900200004881c0c801000049898790020000c3` (22) | same | same | Y | `dc11d2c2afb93a56` | `dc11d2c2afb93a56` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x50 imm=0x1C8 | `498b87800200004881e8c801000049898780020000c3` (22) | same | same | Y | `3c7c7cf3d889226e` | `3c7c7cf3d889226e` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x51 imm=0x1C8 | `498b87880200004881e8c801000049898788020000c3` (22) | same | same | Y | `63000a311432b0f3` | `63000a311432b0f3` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x52 imm=0x1C8 | `498b87900200004881e8c801000049898790020000c3` (22) | same | same | Y | `fa6d5ee090445380` | `fa6d5ee090445380` | PASS |
| 5 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1D0 | `498b87000300004881c0d0010000480fb60049898780020000c3` (26) | same | same | Y | `a8b6a7f0de518100` | `a8b6a7f0de518100` | PASS |
| 6 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1D0 | `498b87000300004881c0d0010000480fb60049898788020000c3` (26) | same | same | Y | `261db47e68ac40dd` | `261db47e68ac40dd` | PASS |
| 7 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1D0 | `498b87000300004881c0d0010000480fb60049898790020000c3` (26) | same | same | Y | `e6b3a3507a16a0ad` | `e6b3a3507a16a0ad` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x50 imm=0x1D0 | `498b87800200004881c0d001000049898780020000c3` (22) | same | same | Y | `16f0fd643450814e` | `16f0fd643450814e` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x52 imm=0x1C8 — **PASS**

- fixture: `_scratch_addimm_h52_1C8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0c801000049898790020000c3`
- js-sha256: `dc11d2c2afb93a5639d5181320f7bb5f8cd6b76b728cc953e444f0cd6f855552`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x50 imm=0x1C8 — **PASS**

- fixture: `_scratch_subimm_h50_1C8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8c801000049898780020000c3`
- js-sha256: `3c7c7cf3d889226ec5a5e56c043d94c7305b86fbeac9d416ec3579460d314e78`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x51 imm=0x1C8 — **PASS**

- fixture: `_scratch_subimm_h51_1C8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8c801000049898788020000c3`
- js-sha256: `63000a311432b0f32a16408c134e65bdf4b24fd57dbf9044977d5ef21834c1ee`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x52 imm=0x1C8 — **PASS**

- fixture: `_scratch_subimm_h52_1C8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8c801000049898790020000c3`
- js-sha256: `fa6d5ee09044538017d8b39642cead9b1ad243d5d2b3f7ad0fe6a157d90d4357`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x50 ss=0x60 oo=0x1D0 — **PASS**

- fixture: `_scratch_ldb_5060_1D0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d0010000480fb60049898780020000c3`
- js-sha256: `a8b6a7f0de5181005aab365a71ab7f9c98a8d73d9330c9cd3bcd381b681de8f4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x51 ss=0x60 oo=0x1D0 — **PASS**

- fixture: `_scratch_ldb_5160_1D0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d0010000480fb60049898788020000c3`
- js-sha256: `261db47e68ac40ddeee1b0bcdae0569d5e6c86d5d08f8bba83351a373ef7b16f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x52 ss=0x60 oo=0x1D0 — **PASS**

- fixture: `_scratch_ldb_5260_1D0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d0010000480fb60049898790020000c3`
- js-sha256: `e6b3a3507a16a0ad852ee9de1afec9f3a6e6edcfe3b3d7e2a1b334bdcb6871fc`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x50 imm=0x1D0 — **PASS**

- fixture: `_scratch_addimm_h50_1D0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0d001000049898780020000c3`
- js-sha256: `16f0fd643450814e8259b7fd79b368cec2ec55083311ed7af2be5f3768de712b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=52 imm=1C8 (finish deferred 1C8 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=1C8 (start deferred 1C8 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=1D0 (start deferred 1D0 LDB triad; imm32 26B).
- ADD-IMM slot=50 imm=1D0 (start 1D0 ADD triad; imm32 22B).
- ADD-IMM 51/52 1D0 + SUB-IMM 50/51/52 1D0 deferred to next batch.
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 26B`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h52_1C8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1C8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1C8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1C8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_1D0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_1D0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_1D0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1D0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-80-log.md` — this file
- `scripts/_probe/parallel-batch-80-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-086 serialize PASSes + 1 Relock**

Pass pin from body-extend-085 Relock: `58b9ca6ef16f3ee48e22fae95f20dd6f6fa3492705659dfe181ec7857e9cf231`.
Handlers before consolidate = 619 (H_00..H_612). Next selectors `40 26B`.. for H_613.. if all serialize.

PASS list for body-extend-086:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_613 | 0x26B | 0x62 ADD-IMM | 0x52 0x1C8 | `498b87900200004881c0c801000049898790020000c3` (22B) | `dc11d2c2afb93a56` |
| H_614 | 0x26C | 0x61 SUB-IMM | 0x50 0x1C8 | `498b87800200004881e8c801000049898780020000c3` (22B) | `3c7c7cf3d889226e` |
| H_615 | 0x26D | 0x61 SUB-IMM | 0x51 0x1C8 | `498b87880200004881e8c801000049898788020000c3` (22B) | `63000a311432b0f3` |
| H_616 | 0x26E | 0x61 SUB-IMM | 0x52 0x1C8 | `498b87900200004881e8c801000049898790020000c3` (22B) | `fa6d5ee090445380` |
| H_617 | 0x26F | 0x80 LDB | 0x50 0x60 0x1D0 | `498b87000300004881c0d0010000480fb60049898780020000c3` (26B) | `a8b6a7f0de518100` |
| H_618 | 0x270 | 0x80 LDB | 0x51 0x60 0x1D0 | `498b87000300004881c0d0010000480fb60049898788020000c3` (26B) | `261db47e68ac40dd` |
| H_619 | 0x271 | 0x80 LDB | 0x52 0x60 0x1D0 | `498b87000300004881c0d0010000480fb60049898790020000c3` (26B) | `e6b3a3507a16a0ad` |
| H_620 | 0x272 | 0x62 ADD-IMM | 0x50 0x1D0 | `498b87800200004881c0d001000049898780020000c3` (22B) | `16f0fd643450814e` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-085 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_612.
- If the parent decides to serialize, append H_613.. at selectors `40 26B`..:
  - H_613 0x62 ADD-IMM (62 52 1C8) — pin `498b87900200004881c0c801000049898790020000c3`
  - H_614 0x61 SUB-IMM (61 50 1C8) — pin `498b87800200004881e8c801000049898780020000c3`
  - H_615 0x61 SUB-IMM (61 51 1C8) — pin `498b87880200004881e8c801000049898788020000c3`
  - H_616 0x61 SUB-IMM (61 52 1C8) — pin `498b87900200004881e8c801000049898790020000c3`
  - H_617 0x80 LDB (80 50 60 1D0) — pin `498b87000300004881c0d0010000480fb60049898780020000c3`
  - H_618 0x80 LDB (80 51 60 1D0) — pin `498b87000300004881c0d0010000480fb60049898788020000c3`
  - H_619 0x80 LDB (80 52 60 1D0) — pin `498b87000300004881c0d0010000480fb60049898790020000c3`
  - H_620 0x62 ADD-IMM (62 50 1D0) — pin `498b87800200004881c0d001000049898780020000c3`
- Plus 1 Relock after append from pin `58b9ca6e…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-086 serialize PASSes + 1 Relock
