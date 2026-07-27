# body-extend-061 SPAWN · consolidate parallel-batch-55

> Continuous queue handoff from parallel-batch-55 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `8088b0d6b9acb4578b66c20fc7febf3994911b9a3ec4ea9eb7060ef3379d66b7` (abbrev `8088b0d6…`).
> Handlers = 420 (H_00..H_413). Last selectors: 0x19C..0x1A3 = H_406..H_413 (`40 19C`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-55-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-060-log.md` / `docs/auxdocs/body-extend-060-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-060 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1A4`.. for H_414.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1A4`/`40 1A5` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-061 (serialize + Relock)

Mirror body-extend-060 / body-extend-059 protocol:

1. Hand-author append H_414..H_421 to `yoyo/projects/yoyo.ty` at selectors `40 1A4` .. `40 1AB` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h51_118,addimm_h52_118,subimm_h50_118,subimm_h51_118,subimm_h52_118,ldb_5060_120,ldb_5160_120,ldb_5260_120}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `8088b0d6b9acb4578b66c20fc7febf3994911b9a3ec4ea9eb7060ef3379d66b7`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-061-log.md`.
7. Auto-spawn parallel-batch-56 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-56-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_414 | 0x1A4 | 0x62 ADD-IMM | 51 118 | `498b87880200004881c01801000049898788020000c3` (22B) | `ed700c44812c65a2` |
| H_415 | 0x1A5 | 0x62 ADD-IMM | 52 118 | `498b87900200004881c01801000049898790020000c3` (22B) | `7849e793c45812bc` |
| H_416 | 0x1A6 | 0x61 SUB-IMM | 50 118 | `498b87800200004881e81801000049898780020000c3` (22B) | `64028ef5fb249d3d` |
| H_417 | 0x1A7 | 0x61 SUB-IMM | 51 118 | `498b87880200004881e81801000049898788020000c3` (22B) | `38ca7c5e4033a507` |
| H_418 | 0x1A8 | 0x61 SUB-IMM | 52 118 | `498b87900200004881e81801000049898790020000c3` (22B) | `cfb3b7a4012d1bae` |
| H_419 | 0x1A9 | 0x80 LDB | 50 60 120 | `498b87000300004881c020010000480fb60049898780020000c3` (26B) | `44a5fa80f01aae38` |
| H_420 | 0x1AA | 0x80 LDB | 51 60 120 | `498b87000300004881c020010000480fb60049898788020000c3` (26B) | `324bf7d8b31a7308` |
| H_421 | 0x1AB | 0x80 LDB | 52 60 120 | `498b87000300004881c020010000480fb60049898790020000c3` (26B) | `3ada911d93412345` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x118 uses imm32 add (`48 81 c0`) → 22B pins (H_414..H_415); not imm8.
SUB-IMM imm=0x118 uses imm32 sub (`48 81 e8`) → 22B pins (H_416..H_418); not imm8.
LDB oo=0x120 uses imm32 add (`48 81 c0`) → 26B pins (H_419..H_421).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_414 | `ed700c44812c65a25841bd4870ab9cc77916614663b1a207ec34b93afd1f81cf` |
| H_415 | `7849e793c45812bc6b4d9b90e317809b25d86d6dc1a4cf7862250d229ef08863` |
| H_416 | `64028ef5fb249d3deaecba580f404b3431fc459e05cb2df513f0a8e02a2a6c32` |
| H_417 | `38ca7c5e4033a50702fd21b72e4602d230d28ac8f76dc022a858e89f2fe2cc01` |
| H_418 | `cfb3b7a4012d1bae005c876895b18c4bb3e4a9f6bc55e9cfaddb23ad225394dc` |
| H_419 | `44a5fa80f01aae3898af0d5d693c1414a23985485ad7e7482501b596d0d55c64` |
| H_420 | `324bf7d8b31a73085e153a48a3a56796565a2bd700f2b02310158b5d051e78ae` |
| H_421 | `3ada911d93412345279093ca1605b4436c8a81fe16dd62b2f9ca21a5e81d2fae` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h51_118`, `_scratch_addimm_h52_118`, `_scratch_subimm_h50_118`, `_scratch_subimm_h51_118`,
`_scratch_subimm_h52_118`, `_scratch_ldb_5060_120`, `_scratch_ldb_5160_120`, `_scratch_ldb_5260_120`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1A4`.. for H_414.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
