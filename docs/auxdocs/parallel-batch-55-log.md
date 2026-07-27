# parallel-batch-55 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-55-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-060 (pin `8088b0d6…`, handlers = 420, H_406..H_413 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-060 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_413 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x51 imm=0x118 | `498b87880200004881c01801000049898788020000c3` (22) | same | same | Y | `ed700c44812c65a2` | `ed700c44812c65a2` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x52 imm=0x118 | `498b87900200004881c01801000049898790020000c3` (22) | same | same | Y | `7849e793c45812bc` | `7849e793c45812bc` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x118 | `498b87800200004881e81801000049898780020000c3` (22) | same | same | Y | `64028ef5fb249d3d` | `64028ef5fb249d3d` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x51 imm=0x118 | `498b87880200004881e81801000049898788020000c3` (22) | same | same | Y | `38ca7c5e4033a507` | `38ca7c5e4033a507` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x52 imm=0x118 | `498b87900200004881e81801000049898790020000c3` (22) | same | same | Y | `cfb3b7a4012d1bae` | `cfb3b7a4012d1bae` | PASS |
| 6 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x120 | `498b87000300004881c020010000480fb60049898780020000c3` (26) | same | same | Y | `44a5fa80f01aae38` | `44a5fa80f01aae38` | PASS |
| 7 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x120 | `498b87000300004881c020010000480fb60049898788020000c3` (26) | same | same | Y | `324bf7d8b31a7308` | `324bf7d8b31a7308` | PASS |
| 8 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x120 | `498b87000300004881c020010000480fb60049898790020000c3` (26) | same | same | Y | `3ada911d93412345` | `3ada911d93412345` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x51 imm=0x118 — **PASS**

- fixture: `_scratch_addimm_h51_118.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c01801000049898788020000c3`
- js-sha256: `ed700c44812c65a25841bd4870ab9cc77916614663b1a207ec34b93afd1f81cf`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x52 imm=0x118 — **PASS**

- fixture: `_scratch_addimm_h52_118.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c01801000049898790020000c3`
- js-sha256: `7849e793c45812bc6b4d9b90e317809b25d86d6dc1a4cf7862250d229ef08863`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x118 — **PASS**

- fixture: `_scratch_subimm_h50_118.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e81801000049898780020000c3`
- js-sha256: `64028ef5fb249d3deaecba580f404b3431fc459e05cb2df513f0a8e02a2a6c32`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x51 imm=0x118 — **PASS**

- fixture: `_scratch_subimm_h51_118.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e81801000049898788020000c3`
- js-sha256: `38ca7c5e4033a50702fd21b72e4602d230d28ac8f76dc022a858e89f2fe2cc01`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x52 imm=0x118 — **PASS**

- fixture: `_scratch_subimm_h52_118.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e81801000049898790020000c3`
- js-sha256: `cfb3b7a4012d1bae005c876895b18c4bb3e4a9f6bc55e9cfaddb23ad225394dc`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x50 ss=0x60 oo=0x120 — **PASS**

- fixture: `_scratch_ldb_5060_120.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c020010000480fb60049898780020000c3`
- js-sha256: `44a5fa80f01aae3898af0d5d693c1414a23985485ad7e7482501b596d0d55c64`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x51 ss=0x60 oo=0x120 — **PASS**

- fixture: `_scratch_ldb_5160_120.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c020010000480fb60049898788020000c3`
- js-sha256: `324bf7d8b31a73085e153a48a3a56796565a2bd700f2b02310158b5d051e78ae`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x52 ss=0x60 oo=0x120 — **PASS**

- fixture: `_scratch_ldb_5260_120.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c020010000480fb60049898790020000c3`
- js-sha256: `3ada911d93412345279093ca1605b4436c8a81fe16dd62b2f9ca21a5e81d2fae`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=51/52 imm=118 (finish 118 ADD triad after H_413; imm32 22B).
- SUB-IMM slot=50/51/52 imm=118 (finish 118 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=120 (next oo after 118 triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1A4`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h51_118.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_118.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_118.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_118.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_118.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_120.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_120.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_120.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-55-log.md` — this file
- `scripts/_probe/parallel-batch-55-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-061 serialize PASSes + 1 Relock**

Pass pin from body-extend-060 Relock: `8088b0d6b9acb4578b66c20fc7febf3994911b9a3ec4ea9eb7060ef3379d66b7`.
Handlers before consolidate = 420 (H_00..H_413). Next selectors `40 1A4`.. for H_414.. if all serialize.

PASS list for body-extend-061:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_414 | 0x1A4 | 0x62 ADD-IMM | 0x51 0x118 | `498b87880200004881c01801000049898788020000c3` (22B) | `ed700c44812c65a2` |
| H_415 | 0x1A5 | 0x62 ADD-IMM | 0x52 0x118 | `498b87900200004881c01801000049898790020000c3` (22B) | `7849e793c45812bc` |
| H_416 | 0x1A6 | 0x61 SUB-IMM | 0x50 0x118 | `498b87800200004881e81801000049898780020000c3` (22B) | `64028ef5fb249d3d` |
| H_417 | 0x1A7 | 0x61 SUB-IMM | 0x51 0x118 | `498b87880200004881e81801000049898788020000c3` (22B) | `38ca7c5e4033a507` |
| H_418 | 0x1A8 | 0x61 SUB-IMM | 0x52 0x118 | `498b87900200004881e81801000049898790020000c3` (22B) | `cfb3b7a4012d1bae` |
| H_419 | 0x1A9 | 0x80 LDB | 0x50 0x60 0x120 | `498b87000300004881c020010000480fb60049898780020000c3` (26B) | `44a5fa80f01aae38` |
| H_420 | 0x1AA | 0x80 LDB | 0x51 0x60 0x120 | `498b87000300004881c020010000480fb60049898788020000c3` (26B) | `324bf7d8b31a7308` |
| H_421 | 0x1AB | 0x80 LDB | 0x52 0x60 0x120 | `498b87000300004881c020010000480fb60049898790020000c3` (26B) | `3ada911d93412345` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-060 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_413.
- If the parent decides to serialize, append H_414.. at selectors `40 1A4`..:
  - H_414 0x62 ADD-IMM (62 51 118) — pin `498b87880200004881c01801000049898788020000c3`
  - H_415 0x62 ADD-IMM (62 52 118) — pin `498b87900200004881c01801000049898790020000c3`
  - H_416 0x61 SUB-IMM (61 50 118) — pin `498b87800200004881e81801000049898780020000c3`
  - H_417 0x61 SUB-IMM (61 51 118) — pin `498b87880200004881e81801000049898788020000c3`
  - H_418 0x61 SUB-IMM (61 52 118) — pin `498b87900200004881e81801000049898790020000c3`
  - H_419 0x80 LDB (80 50 60 120) — pin `498b87000300004881c020010000480fb60049898780020000c3`
  - H_420 0x80 LDB (80 51 60 120) — pin `498b87000300004881c020010000480fb60049898788020000c3`
  - H_421 0x80 LDB (80 52 60 120) — pin `498b87000300004881c020010000480fb60049898790020000c3`
- Plus 1 Relock after append from pin `8088b0d6…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-061 serialize PASSes + 1 Relock
