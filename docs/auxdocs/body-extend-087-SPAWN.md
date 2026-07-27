# body-extend-087 SPAWN · consolidate parallel-batch-81

> Continuous queue handoff from parallel-batch-81 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `9546a03ee5ac5d5254a4d887560694622666ef2cfc3a6035a937c978dfd5ee67` (abbrev `9546a03e…`).
> Handlers = 627 (H_00..H_620). Last selectors: 0x26B..0x272 = H_613..H_620 (`40 26B`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-81-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-086-log.md` / `docs/auxdocs/body-extend-086-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-086 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 273`.. for H_621.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 273`/`40 274` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-087 (serialize + Relock)

Mirror body-extend-086 / body-extend-085 protocol:

1. Hand-author append H_621..H_628 to `yoyo/projects/yoyo.ty` at selectors `40 273` .. `40 27A` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h51_1D0,addimm_h52_1D0,subimm_h50_1D0,subimm_h51_1D0,subimm_h52_1D0,ldb_5060_1D8,ldb_5160_1D8,ldb_5260_1D8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `9546a03ee5ac5d5254a4d887560694622666ef2cfc3a6035a937c978dfd5ee67`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-087-log.md`.
7. Auto-spawn parallel-batch-82 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-82-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_621 | 0x273 | 0x62 ADD-IMM | 51 1D0 | `498b87880200004881c0d001000049898788020000c3` (22B) | `c3c058088c10f83d` |
| H_622 | 0x274 | 0x62 ADD-IMM | 52 1D0 | `498b87900200004881c0d001000049898790020000c3` (22B) | `0a27be96ee13ece7` |
| H_623 | 0x275 | 0x61 SUB-IMM | 50 1D0 | `498b87800200004881e8d001000049898780020000c3` (22B) | `2ef2771b506c8417` |
| H_624 | 0x276 | 0x61 SUB-IMM | 51 1D0 | `498b87880200004881e8d001000049898788020000c3` (22B) | `45b0a1f1db1de34c` |
| H_625 | 0x277 | 0x61 SUB-IMM | 52 1D0 | `498b87900200004881e8d001000049898790020000c3` (22B) | `58eef69fd940e52e` |
| H_626 | 0x278 | 0x80 LDB | 50 60 1D8 | `498b87000300004881c0d8010000480fb60049898780020000c3` (26B) | `15e5685f4e776a0c` |
| H_627 | 0x279 | 0x80 LDB | 51 60 1D8 | `498b87000300004881c0d8010000480fb60049898788020000c3` (26B) | `76fe53dd5b4d10f0` |
| H_628 | 0x27A | 0x80 LDB | 52 60 1D8 | `498b87000300004881c0d8010000480fb60049898790020000c3` (26B) | `9a6919da685a02e0` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1D0 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1D0 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1D8 uses imm32 add (`48 81 c0`) → 26B pins.
ADD-IMM slot=51/52 imm=1D0 finishes deferred 1D0 ADD triad (H_621/H_622; after H_620).
SUB-IMM slot=50/51/52 imm=1D0 starts deferred 1D0 SUB triad (H_623/H_624/H_625).
LDB oo=0x1D8 starts 1D8 LDB triad (H_626/H_627/H_628 dd=50/51/52).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_621 | `c3c058088c10f83d4fcfade79d449baac7c28513edba68591ebafeb0c3abb50c` |
| H_622 | `0a27be96ee13ece7972673d85b5654a23eee344d34373cb539b16147cb3526bf` |
| H_623 | `2ef2771b506c8417b1cc2d80c9b8c2094e807cff80496af171a4fdf7e2c507b2` |
| H_624 | `45b0a1f1db1de34c044d580d34b8c70fb11ec50a1b46182c9608dc64d6131fb6` |
| H_625 | `58eef69fd940e52e61c7da1738853b9d42f41aedc2819f354db5a0d797b6d43b` |
| H_626 | `15e5685f4e776a0c3cb0c740f5572ff6200335d566557b079afcb68ec8519972` |
| H_627 | `76fe53dd5b4d10f0df7a013ab4be44413f3691fc8953a5df2e44bdf13460d462` |
| H_628 | `9a6919da685a02e06aa72835e35705110da398e37ceb4d9f2e809f57c5d3a3e7` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h51_1D0`, `_scratch_addimm_h52_1D0`, `_scratch_subimm_h50_1D0`, `_scratch_subimm_h51_1D0`,
`_scratch_subimm_h52_1D0`, `_scratch_ldb_5060_1D8`, `_scratch_ldb_5160_1D8`, `_scratch_ldb_5260_1D8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 273`.. for H_621.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-82

- ADD-IMM slot=50/51/52 imm=1D8 (start 1D8 ADD triad)
- SUB-IMM slot=50/51/52 imm=1D8 (start 1D8 SUB triad)
- LDB / ADD-IMM / SUB-IMM next imm ladder (1E0…) if continuing
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
