# parallel-batch-81 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-81-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-086 (pin `9546a03e…`, handlers = 627, H_613..H_620 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-086 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_620 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x51 imm=0x1D0 | `498b87880200004881c0d001000049898788020000c3` (22) | same | same | Y | `c3c058088c10f83d` | `c3c058088c10f83d` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x52 imm=0x1D0 | `498b87900200004881c0d001000049898790020000c3` (22) | same | same | Y | `0a27be96ee13ece7` | `0a27be96ee13ece7` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x1D0 | `498b87800200004881e8d001000049898780020000c3` (22) | same | same | Y | `2ef2771b506c8417` | `2ef2771b506c8417` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x51 imm=0x1D0 | `498b87880200004881e8d001000049898788020000c3` (22) | same | same | Y | `45b0a1f1db1de34c` | `45b0a1f1db1de34c` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x52 imm=0x1D0 | `498b87900200004881e8d001000049898790020000c3` (22) | same | same | Y | `58eef69fd940e52e` | `58eef69fd940e52e` | PASS |
| 6 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1D8 | `498b87000300004881c0d8010000480fb60049898780020000c3` (26) | same | same | Y | `15e5685f4e776a0c` | `15e5685f4e776a0c` | PASS |
| 7 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1D8 | `498b87000300004881c0d8010000480fb60049898788020000c3` (26) | same | same | Y | `76fe53dd5b4d10f0` | `76fe53dd5b4d10f0` | PASS |
| 8 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1D8 | `498b87000300004881c0d8010000480fb60049898790020000c3` (26) | same | same | Y | `9a6919da685a02e0` | `9a6919da685a02e0` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x51 imm=0x1D0 — **PASS**

- fixture: `_scratch_addimm_h51_1D0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0d001000049898788020000c3`
- js-sha256: `c3c058088c10f83d4fcfade79d449baac7c28513edba68591ebafeb0c3abb50c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x52 imm=0x1D0 — **PASS**

- fixture: `_scratch_addimm_h52_1D0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0d001000049898790020000c3`
- js-sha256: `0a27be96ee13ece7972673d85b5654a23eee344d34373cb539b16147cb3526bf`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x1D0 — **PASS**

- fixture: `_scratch_subimm_h50_1D0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8d001000049898780020000c3`
- js-sha256: `2ef2771b506c8417b1cc2d80c9b8c2094e807cff80496af171a4fdf7e2c507b2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x51 imm=0x1D0 — **PASS**

- fixture: `_scratch_subimm_h51_1D0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8d001000049898788020000c3`
- js-sha256: `45b0a1f1db1de34c044d580d34b8c70fb11ec50a1b46182c9608dc64d6131fb6`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x52 imm=0x1D0 — **PASS**

- fixture: `_scratch_subimm_h52_1D0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8d001000049898790020000c3`
- js-sha256: `58eef69fd940e52e61c7da1738853b9d42f41aedc2819f354db5a0d797b6d43b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x50 ss=0x60 oo=0x1D8 — **PASS**

- fixture: `_scratch_ldb_5060_1D8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d8010000480fb60049898780020000c3`
- js-sha256: `15e5685f4e776a0c3cb0c740f5572ff6200335d566557b079afcb68ec8519972`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x51 ss=0x60 oo=0x1D8 — **PASS**

- fixture: `_scratch_ldb_5160_1D8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d8010000480fb60049898788020000c3`
- js-sha256: `76fe53dd5b4d10f0df7a013ab4be44413f3691fc8953a5df2e44bdf13460d462`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x52 ss=0x60 oo=0x1D8 — **PASS**

- fixture: `_scratch_ldb_5260_1D8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0d8010000480fb60049898790020000c3`
- js-sha256: `9a6919da685a02e06aa72835e35705110da398e37ceb4d9f2e809f57c5d3a3e7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=51/52 imm=1D0 (finish deferred 1D0 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=1D0 (start deferred 1D0 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=1D8 (start deferred 1D8 LDB triad; imm32 26B).
- ADD-IMM/SUB-IMM 50/51/52 1D8 deferred to next batch.
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 273`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h51_1D0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1D0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1D0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1D0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1D0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_1D8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_1D8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_1D8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-81-log.md` — this file
- `scripts/_probe/parallel-batch-81-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-087 serialize PASSes + 1 Relock**

Pass pin from body-extend-086 Relock: `9546a03ee5ac5d5254a4d887560694622666ef2cfc3a6035a937c978dfd5ee67`.
Handlers before consolidate = 627 (H_00..H_620). Next selectors `40 273`.. for H_621.. if all serialize.

PASS list for body-extend-087:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_621 | 0x273 | 0x62 ADD-IMM | 0x51 0x1D0 | `498b87880200004881c0d001000049898788020000c3` (22B) | `c3c058088c10f83d` |
| H_622 | 0x274 | 0x62 ADD-IMM | 0x52 0x1D0 | `498b87900200004881c0d001000049898790020000c3` (22B) | `0a27be96ee13ece7` |
| H_623 | 0x275 | 0x61 SUB-IMM | 0x50 0x1D0 | `498b87800200004881e8d001000049898780020000c3` (22B) | `2ef2771b506c8417` |
| H_624 | 0x276 | 0x61 SUB-IMM | 0x51 0x1D0 | `498b87880200004881e8d001000049898788020000c3` (22B) | `45b0a1f1db1de34c` |
| H_625 | 0x277 | 0x61 SUB-IMM | 0x52 0x1D0 | `498b87900200004881e8d001000049898790020000c3` (22B) | `58eef69fd940e52e` |
| H_626 | 0x278 | 0x80 LDB | 0x50 0x60 0x1D8 | `498b87000300004881c0d8010000480fb60049898780020000c3` (26B) | `15e5685f4e776a0c` |
| H_627 | 0x279 | 0x80 LDB | 0x51 0x60 0x1D8 | `498b87000300004881c0d8010000480fb60049898788020000c3` (26B) | `76fe53dd5b4d10f0` |
| H_628 | 0x27A | 0x80 LDB | 0x52 0x60 0x1D8 | `498b87000300004881c0d8010000480fb60049898790020000c3` (26B) | `9a6919da685a02e0` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-086 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_620.
- If the parent decides to serialize, append H_621.. at selectors `40 273`..:
  - H_621 0x62 ADD-IMM (62 51 1D0) — pin `498b87880200004881c0d001000049898788020000c3`
  - H_622 0x62 ADD-IMM (62 52 1D0) — pin `498b87900200004881c0d001000049898790020000c3`
  - H_623 0x61 SUB-IMM (61 50 1D0) — pin `498b87800200004881e8d001000049898780020000c3`
  - H_624 0x61 SUB-IMM (61 51 1D0) — pin `498b87880200004881e8d001000049898788020000c3`
  - H_625 0x61 SUB-IMM (61 52 1D0) — pin `498b87900200004881e8d001000049898790020000c3`
  - H_626 0x80 LDB (80 50 60 1D8) — pin `498b87000300004881c0d8010000480fb60049898780020000c3`
  - H_627 0x80 LDB (80 51 60 1D8) — pin `498b87000300004881c0d8010000480fb60049898788020000c3`
  - H_628 0x80 LDB (80 52 60 1D8) — pin `498b87000300004881c0d8010000480fb60049898790020000c3`
- Plus 1 Relock after append from pin `9546a03e…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-087 serialize PASSes + 1 Relock
