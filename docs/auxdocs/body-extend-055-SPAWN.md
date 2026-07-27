# body-extend-055 SPAWN · consolidate parallel-batch-49

> Continuous queue handoff from parallel-batch-49 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `13cb91ab1e1cc24d3f4b6d9a151a2e9a8d487556099cc030a189d6ac30554d9b` (abbrev `13cb91ab…`).
> Handlers = 372 (H_00..H_365). Last selectors: 0x16C..0x173 = H_358..H_365 (`40 16C`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-49-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-054-log.md` / `docs/auxdocs/body-extend-054-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-054 DDC PE `.text` measured DIFFER this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 174`.. for H_366.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-055 (serialize + Relock)

Mirror body-extend-054 / body-extend-053 protocol:

1. Hand-author append H_366..H_373 to `yoyo/projects/yoyo.ty` at selectors `40 174` .. `40 17B` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5160_f0,ldb_5260_f0,addimm_h50_f0,addimm_h51_f0,addimm_h52_f0,subimm_h50_f0,subimm_h51_f0,subimm_h52_f0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `13cb91ab1e1cc24d…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-055-log.md`.
7. Auto-spawn parallel-batch-50 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-50-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_366 | 0x174 | 0x80 LDB | 51 60 F0 | `498b87000300004881c0f0000000480fb60049898788020000c3` (26B) | `878beef94d2aaca0` |
| H_367 | 0x175 | 0x80 LDB | 52 60 F0 | `498b87000300004881c0f0000000480fb60049898790020000c3` (26B) | `39e79a02c3bbc071` |
| H_368 | 0x176 | 0x62 ADD-IMM | 50 F0 | `498b87800200004881c0f000000049898780020000c3` (22B) | `cfd72ee65ddb08fc` |
| H_369 | 0x177 | 0x62 ADD-IMM | 51 F0 | `498b87880200004881c0f000000049898788020000c3` (22B) | `5aa3b0e69138d4d3` |
| H_370 | 0x178 | 0x62 ADD-IMM | 52 F0 | `498b87900200004881c0f000000049898790020000c3` (22B) | `e67473702a13c78e` |
| H_371 | 0x179 | 0x61 SUB-IMM | 50 F0 | `498b87800200004881e8f000000049898780020000c3` (22B) | `3404141d925462bb` |
| H_372 | 0x17A | 0x61 SUB-IMM | 51 F0 | `498b87880200004881e8f000000049898788020000c3` (22B) | `d52a7558bdad1d89` |
| H_373 | 0x17B | 0x61 SUB-IMM | 52 F0 | `498b87900200004881e8f000000049898790020000c3` (22B) | `4128c048e41cad1a` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xF0 uses imm32 add (`48 81 c0`) → 22B pins (H_368..H_370); not imm8.
SUB-IMM imm=0xF0 uses imm32 sub (`48 81 e8`) → 22B pins (H_371..H_373); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_366..H_367).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_366 | `878beef94d2aaca04e5efb8e45f4a867cd7039e2a1f9bca97382f4d09e6e78a2` |
| H_367 | `39e79a02c3bbc071604456f1d4180450dfd918f46c3e1247ae61e18ed26e2f9f` |
| H_368 | `cfd72ee65ddb08fcb37ba69e9675363ad8e8e6f4e25a5ee7f5a623fb4aa1c397` |
| H_369 | `5aa3b0e69138d4d37568053650679921a47fb1741bea2da664dba51ab20dca86` |
| H_370 | `e67473702a13c78e2404bdc37f5673489ca0a7bf1b01003879dbfa40e71ca334` |
| H_371 | `3404141d925462bb6adcc0d67957347b9630be8581829283c16ce21acf444d9a` |
| H_372 | `d52a7558bdad1d89bd1cf87a9774c4bdee4da8d91f27d8f461a290fb6d7ca3f5` |
| H_373 | `4128c048e41cad1af95d1cfc3872edbb23b864dd29b56ae1946a7dfcbd9a1ca3` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5160_f0`, `_scratch_ldb_5260_f0`, `_scratch_addimm_h50_f0`, `_scratch_addimm_h51_f0`,
`_scratch_addimm_h52_f0`, `_scratch_subimm_h50_f0`, `_scratch_subimm_h51_f0`, `_scratch_subimm_h52_f0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 174`.. for H_366.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
