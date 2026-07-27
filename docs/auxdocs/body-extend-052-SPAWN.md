# body-extend-052 SPAWN · consolidate parallel-batch-46

> Continuous queue handoff from parallel-batch-46 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `ee5b881e34301f79f6c647181243709ea5ccfdbf03a2088c7d44b1de98d91b4f` (abbrev `ee5b881e…`).
> Handlers = 348 (H_00..H_341). Last selectors: 0x154..0x15B = H_334..H_341 (`40 154`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-46-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-051-log.md` / `docs/auxdocs/body-extend-051-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-051 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 15C`.. for H_342.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-052 (serialize + Relock)

Mirror body-extend-051 / body-extend-050 protocol:

1. Hand-author append H_342..H_349 to `yoyo/projects/yoyo.ty` at selectors `40 15C` .. `40 163` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5160_e0,ldb_5260_e0,addimm_h50_d8,addimm_h51_d8,addimm_h52_d8,subimm_h50_d8,subimm_h51_d8,subimm_h52_d8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `ee5b881e34301f79…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-052-log.md`.
7. Auto-spawn parallel-batch-47 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-47-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_342 | 0x15C | 0x80 LDB | 51 60 E0 | `498b87000300004881c0e0000000480fb60049898788020000c3` (26B) | `50f40ec03eee29c8` |
| H_343 | 0x15D | 0x80 LDB | 52 60 E0 | `498b87000300004881c0e0000000480fb60049898790020000c3` (26B) | `4c6401f2595fc5c8` |
| H_344 | 0x15E | 0x62 ADD-IMM | 50 D8 | `498b87800200004881c0d800000049898780020000c3` (22B) | `3f9b979485c6551c` |
| H_345 | 0x15F | 0x62 ADD-IMM | 51 D8 | `498b87880200004881c0d800000049898788020000c3` (22B) | `959f55bf7e28a72e` |
| H_346 | 0x160 | 0x62 ADD-IMM | 52 D8 | `498b87900200004881c0d800000049898790020000c3` (22B) | `300854c0d5bd80ba` |
| H_347 | 0x161 | 0x61 SUB-IMM | 50 D8 | `498b87800200004881e8d800000049898780020000c3` (22B) | `82866db77dd7973c` |
| H_348 | 0x162 | 0x61 SUB-IMM | 51 D8 | `498b87880200004881e8d800000049898788020000c3` (22B) | `98d0142fba622c9f` |
| H_349 | 0x163 | 0x61 SUB-IMM | 52 D8 | `498b87900200004881e8d800000049898790020000c3` (22B) | `0cf496fbf781f92d` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xD8 uses imm32 add (`48 81 c0`) → 22B pins (H_344..H_346); not imm8.
SUB-IMM imm=0xD8 uses imm32 sub (`48 81 e8`) → 22B pins (H_347..H_349); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_342..H_343).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_342 | `50f40ec03eee29c87621b39ec8e4393b42d51e5aaa9ec1cd9babfc4838395a04` |
| H_343 | `4c6401f2595fc5c8713655c8cc7c4a05f15d142353cb3f3d6fdc529ac5eaf24d` |
| H_344 | `3f9b979485c6551c8143bb58f2145cd0dc176d5a14b38bcb605a0e24e00ee127` |
| H_345 | `959f55bf7e28a72eb3ba4780c0e718c93aceea5fd2e8bfe125192d56ed8689a7` |
| H_346 | `300854c0d5bd80bae9ecac392e64777bf498458e5ada461b5ff1b9ed88a16a32` |
| H_347 | `82866db77dd7973c2ce55b37a8606089236bdf71b17b4bb5dc515b37f9c1b968` |
| H_348 | `98d0142fba622c9f86bcbccf4393c58c7c51bd449778442ebdd08ae390caec57` |
| H_349 | `0cf496fbf781f92d16d66cfa432e351604bbb83d073603eb6ad6f9283cb90ea0` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5160_e0`, `_scratch_ldb_5260_e0`, `_scratch_addimm_h50_d8`, `_scratch_addimm_h51_d8`,
`_scratch_addimm_h52_d8`, `_scratch_subimm_h50_d8`, `_scratch_subimm_h51_d8`, `_scratch_subimm_h52_d8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 15C`.. for H_342.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
