# parallel-batch-86 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-86-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-091 (pin `339bd482…`, handlers = 667, H_653..H_660 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-091 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_660 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x52 imm=0x1F0 | `498b87900200004881e8f001000049898790020000c3` (22) | same | same | Y | `21a46af767b04e47` | `21a46af767b04e47` | PASS |
| 2 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1F8 | `498b87000300004881c0f8010000480fb60049898780020000c3` (26) | same | same | Y | `e33190513a0b6fac` | `e33190513a0b6fac` | PASS |
| 3 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1F8 | `498b87000300004881c0f8010000480fb60049898788020000c3` (26) | same | same | Y | `754738a2ae8287ba` | `754738a2ae8287ba` | PASS |
| 4 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1F8 | `498b87000300004881c0f8010000480fb60049898790020000c3` (26) | same | same | Y | `b3d0c040cbafd1ed` | `b3d0c040cbafd1ed` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x50 imm=0x1F8 | `498b87800200004881c0f801000049898780020000c3` (22) | same | same | Y | `e4eb4882c94f477d` | `e4eb4882c94f477d` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x51 imm=0x1F8 | `498b87880200004881c0f801000049898788020000c3` (22) | same | same | Y | `767adbf6b2f425c9` | `767adbf6b2f425c9` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x52 imm=0x1F8 | `498b87900200004881c0f801000049898790020000c3` (22) | same | same | Y | `5e4ebbbafb63edb5` | `5e4ebbbafb63edb5` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x50 imm=0x1F8 | `498b87800200004881e8f801000049898780020000c3` (22) | same | same | Y | `8ebe141b655cf99d` | `8ebe141b655cf99d` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x52 imm=0x1F0 — **PASS**

- fixture: `_scratch_subimm_h52_1F0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8f001000049898790020000c3`
- js-sha256: `21a46af767b04e47650b619132dce8c0d8eb8853d90a43a9d7d1d28af98d0f1a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x50 ss=0x60 oo=0x1F8 — **PASS**

- fixture: `_scratch_ldb_5060_1F8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f8010000480fb60049898780020000c3`
- js-sha256: `e33190513a0b6fac13932e72229b5bbafeaff7083cc224cf8c53338378fee9c0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x51 ss=0x60 oo=0x1F8 — **PASS**

- fixture: `_scratch_ldb_5160_1F8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f8010000480fb60049898788020000c3`
- js-sha256: `754738a2ae8287ba25dd22fcc9ffef4d583b37c9061d495e26db3499044f8770`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x52 ss=0x60 oo=0x1F8 — **PASS**

- fixture: `_scratch_ldb_5260_1F8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f8010000480fb60049898790020000c3`
- js-sha256: `b3d0c040cbafd1ed0af4dbfd1514fe59fcc1c44f55d5b025a1d4602cba0cfd12`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x50 imm=0x1F8 — **PASS**

- fixture: `_scratch_addimm_h50_1F8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0f801000049898780020000c3`
- js-sha256: `e4eb4882c94f477d7369f849651cad1e3e4ebd2dbe762d456c009667bc3d37ad`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x51 imm=0x1F8 — **PASS**

- fixture: `_scratch_addimm_h51_1F8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0f801000049898788020000c3`
- js-sha256: `767adbf6b2f425c9fb3363b309c6fe79e3d1ba7874531cc4b34f0706d7e1b3c0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x52 imm=0x1F8 — **PASS**

- fixture: `_scratch_addimm_h52_1F8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0f801000049898790020000c3`
- js-sha256: `5e4ebbbafb63edb539dc75ecd54dd72456e8ff9afa398a2f746507a3c3f3ba2a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x50 imm=0x1F8 — **PASS**

- fixture: `_scratch_subimm_h50_1F8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8f801000049898780020000c3`
- js-sha256: `8ebe141b655cf99deb6c55f0869d36be0725d1fef0b58268d6d6751190a816a5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=52 imm=1F0 (finish deferred 1F0 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=1F8 (start 1F8 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=1F8 (start 1F8 ADD triad; imm32 22B).
- SUB-IMM slot=50 imm=1F8 (start 1F8 SUB triad; imm32 22B; SUB 51/52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 29B`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h52_1F0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_1F8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_1F8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_1F8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1F8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1F8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1F8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1F8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-86-log.md` — this file
- `scripts/_probe/parallel-batch-86-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-092 serialize PASSes + 1 Relock**

Pass pin from body-extend-091 Relock: `339bd482ae784eb8a80f7176ef5d7c6f3c90b0e491b08c6103512860ab5b918a`.
Handlers before consolidate = 667 (H_00..H_660). Next selectors `40 29B`.. for H_661.. if all serialize.

PASS list for body-extend-092:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_661 | 0x29B | 0x61 SUB-IMM | 0x52 0x1F0 | `498b87900200004881e8f001000049898790020000c3` (22B) | `21a46af767b04e47` |
| H_662 | 0x29C | 0x80 LDB | 0x50 0x60 0x1F8 | `498b87000300004881c0f8010000480fb60049898780020000c3` (26B) | `e33190513a0b6fac` |
| H_663 | 0x29D | 0x80 LDB | 0x51 0x60 0x1F8 | `498b87000300004881c0f8010000480fb60049898788020000c3` (26B) | `754738a2ae8287ba` |
| H_664 | 0x29E | 0x80 LDB | 0x52 0x60 0x1F8 | `498b87000300004881c0f8010000480fb60049898790020000c3` (26B) | `b3d0c040cbafd1ed` |
| H_665 | 0x29F | 0x62 ADD-IMM | 0x50 0x1F8 | `498b87800200004881c0f801000049898780020000c3` (22B) | `e4eb4882c94f477d` |
| H_666 | 0x2A0 | 0x62 ADD-IMM | 0x51 0x1F8 | `498b87880200004881c0f801000049898788020000c3` (22B) | `767adbf6b2f425c9` |
| H_667 | 0x2A1 | 0x62 ADD-IMM | 0x52 0x1F8 | `498b87900200004881c0f801000049898790020000c3` (22B) | `5e4ebbbafb63edb5` |
| H_668 | 0x2A2 | 0x61 SUB-IMM | 0x50 0x1F8 | `498b87800200004881e8f801000049898780020000c3` (22B) | `8ebe141b655cf99d` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-091 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_660.
- If the parent decides to serialize, append H_661.. at selectors `40 29B`..:
  - H_661 0x61 SUB-IMM (61 52 1F0) — pin `498b87900200004881e8f001000049898790020000c3`
  - H_662 0x80 LDB (80 50 60 1F8) — pin `498b87000300004881c0f8010000480fb60049898780020000c3`
  - H_663 0x80 LDB (80 51 60 1F8) — pin `498b87000300004881c0f8010000480fb60049898788020000c3`
  - H_664 0x80 LDB (80 52 60 1F8) — pin `498b87000300004881c0f8010000480fb60049898790020000c3`
  - H_665 0x62 ADD-IMM (62 50 1F8) — pin `498b87800200004881c0f801000049898780020000c3`
  - H_666 0x62 ADD-IMM (62 51 1F8) — pin `498b87880200004881c0f801000049898788020000c3`
  - H_667 0x62 ADD-IMM (62 52 1F8) — pin `498b87900200004881c0f801000049898790020000c3`
  - H_668 0x61 SUB-IMM (61 50 1F8) — pin `498b87800200004881e8f801000049898780020000c3`
- Plus 1 Relock after append from pin `339bd482…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: SUB-IMM slot=51/52 imm=1F8 (finish 1F8 SUB triad).

## §7. Consolidation handoff

parent next = body-extend-092 serialize PASSes + 1 Relock
