# parallel-batch-46 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-46-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-051 (pin `ee5b881e…`, handlers = 348, H_334..H_341 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-051 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_341 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xE0 | `498b87000300004881c0e0000000480fb60049898788020000c3` (26) | same | same | Y | `50f40ec03eee29c8` | `50f40ec03eee29c8` | PASS |
| 2 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xE0 | `498b87000300004881c0e0000000480fb60049898790020000c3` (26) | same | same | Y | `4c6401f2595fc5c8` | `4c6401f2595fc5c8` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x50 imm=0xD8 | `498b87800200004881c0d800000049898780020000c3` (22) | same | same | Y | `3f9b979485c6551c` | `3f9b979485c6551c` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x51 imm=0xD8 | `498b87880200004881c0d800000049898788020000c3` (22) | same | same | Y | `959f55bf7e28a72e` | `959f55bf7e28a72e` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x52 imm=0xD8 | `498b87900200004881c0d800000049898790020000c3` (22) | same | same | Y | `300854c0d5bd80ba` | `300854c0d5bd80ba` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x50 imm=0xD8 | `498b87800200004881e8d800000049898780020000c3` (22) | same | same | Y | `82866db77dd7973c` | `82866db77dd7973c` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0xD8 | `498b87880200004881e8d800000049898788020000c3` (22) | same | same | Y | `98d0142fba622c9f` | `98d0142fba622c9f` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x52 imm=0xD8 | `498b87900200004881e8d800000049898790020000c3` (22) | same | same | Y | `0cf496fbf781f92d` | `0cf496fbf781f92d` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x51 ss=0x60 oo=0xE0 — **PASS**

- fixture: `_scratch_ldb_5160_e0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e0000000480fb60049898788020000c3`
- js-sha256: `50f40ec03eee29c87621b39ec8e4393b42d51e5aaa9ec1cd9babfc4838395a04`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x52 ss=0x60 oo=0xE0 — **PASS**

- fixture: `_scratch_ldb_5260_e0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e0000000480fb60049898790020000c3`
- js-sha256: `4c6401f2595fc5c8713655c8cc7c4a05f15d142353cb3f3d6fdc529ac5eaf24d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x50 imm=0xD8 — **PASS**

- fixture: `_scratch_addimm_h50_d8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0d800000049898780020000c3`
- js-sha256: `3f9b979485c6551c8143bb58f2145cd0dc176d5a14b38bcb605a0e24e00ee127`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x51 imm=0xD8 — **PASS**

- fixture: `_scratch_addimm_h51_d8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0d800000049898788020000c3`
- js-sha256: `959f55bf7e28a72eb3ba4780c0e718c93aceea5fd2e8bfe125192d56ed8689a7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x52 imm=0xD8 — **PASS**

- fixture: `_scratch_addimm_h52_d8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0d800000049898790020000c3`
- js-sha256: `300854c0d5bd80bae9ecac392e64777bf498458e5ada461b5ff1b9ed88a16a32`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x50 imm=0xD8 — **PASS**

- fixture: `_scratch_subimm_h50_d8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8d800000049898780020000c3`
- js-sha256: `82866db77dd7973c2ce55b37a8606089236bdf71b17b4bb5dc515b37f9c1b968`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0xD8 — **PASS**

- fixture: `_scratch_subimm_h51_d8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8d800000049898788020000c3`
- js-sha256: `98d0142fba622c9f86bcbccf4393c58c7c51bd449778442ebdd08ae390caec57`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x52 imm=0xD8 — **PASS**

- fixture: `_scratch_subimm_h52_d8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8d800000049898790020000c3`
- js-sha256: `0cf496fbf781f92d16d66cfa432e351604bbb83d073603eb6ad6f9283cb90ea0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=51/52 ss=60 oo=E0 (finish E0 triad after H_341; imm32 26B).
- ADD-IMM slot=50/51/52 imm=D8 (fresh imm after D0; imm32 22B).
- SUB-IMM slot=50/51/52 imm=D8 (complements ADD-IMM * D8; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 15C`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5160_e0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_e0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_d8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_d8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_d8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_d8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_d8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_d8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-46-log.md` — this file
- `scripts/_probe/parallel-batch-46-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-052 serialize PASSes + 1 Relock**

Pass pin from body-extend-051 Relock: `ee5b881e34301f79f6c647181243709ea5ccfdbf03a2088c7d44b1de98d91b4f`.
Handlers before consolidate = 348 (H_00..H_341). Next selectors `40 15C`.. for H_342.. if all serialize.

PASS list for body-extend-052:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_342 | 0x15C | 0x80 LDB | 0x51 0x60 0xE0 | `498b87000300004881c0e0000000480fb60049898788020000c3` (26B) | `50f40ec03eee29c8` |
| H_343 | 0x15D | 0x80 LDB | 0x52 0x60 0xE0 | `498b87000300004881c0e0000000480fb60049898790020000c3` (26B) | `4c6401f2595fc5c8` |
| H_344 | 0x15E | 0x62 ADD-IMM | 0x50 0xD8 | `498b87800200004881c0d800000049898780020000c3` (22B) | `3f9b979485c6551c` |
| H_345 | 0x15F | 0x62 ADD-IMM | 0x51 0xD8 | `498b87880200004881c0d800000049898788020000c3` (22B) | `959f55bf7e28a72e` |
| H_346 | 0x160 | 0x62 ADD-IMM | 0x52 0xD8 | `498b87900200004881c0d800000049898790020000c3` (22B) | `300854c0d5bd80ba` |
| H_347 | 0x161 | 0x61 SUB-IMM | 0x50 0xD8 | `498b87800200004881e8d800000049898780020000c3` (22B) | `82866db77dd7973c` |
| H_348 | 0x162 | 0x61 SUB-IMM | 0x51 0xD8 | `498b87880200004881e8d800000049898788020000c3` (22B) | `98d0142fba622c9f` |
| H_349 | 0x163 | 0x61 SUB-IMM | 0x52 0xD8 | `498b87900200004881e8d800000049898790020000c3` (22B) | `0cf496fbf781f92d` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-051 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_341.
- If the parent decides to serialize, append H_342.. at selectors `40 15C`..:
  - H_342 0x80 LDB (80 51 60 E0) — pin `498b87000300004881c0e0000000480fb60049898788020000c3`
  - H_343 0x80 LDB (80 52 60 E0) — pin `498b87000300004881c0e0000000480fb60049898790020000c3`
  - H_344 0x62 ADD-IMM (62 50 D8) — pin `498b87800200004881c0d800000049898780020000c3`
  - H_345 0x62 ADD-IMM (62 51 D8) — pin `498b87880200004881c0d800000049898788020000c3`
  - H_346 0x62 ADD-IMM (62 52 D8) — pin `498b87900200004881c0d800000049898790020000c3`
  - H_347 0x61 SUB-IMM (61 50 D8) — pin `498b87800200004881e8d800000049898780020000c3`
  - H_348 0x61 SUB-IMM (61 51 D8) — pin `498b87880200004881e8d800000049898788020000c3`
  - H_349 0x61 SUB-IMM (61 52 D8) — pin `498b87900200004881e8d800000049898790020000c3`
- Plus 1 Relock after append from pin `ee5b881e…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-052 serialize PASSes + 1 Relock
