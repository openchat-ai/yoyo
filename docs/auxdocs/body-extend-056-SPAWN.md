# body-extend-056 SPAWN · consolidate parallel-batch-50

> Continuous queue handoff from parallel-batch-50 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `fba1f97e01a9ef7e6285451fe34b6b52a972caf99ae81f93518563d7eb1ec442` (abbrev `fba1f97e…`).
> Handlers = 380 (H_00..H_373). Last selectors: 0x174..0x17B = H_366..H_373 (`40 174`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-50-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-055-log.md` / `docs/auxdocs/body-extend-055-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-055 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 17C`.. for H_374.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-056 (serialize + Relock)

Mirror body-extend-055 / body-extend-054 protocol:

1. Hand-author append H_374..H_381 to `yoyo/projects/yoyo.ty` at selectors `40 17C` .. `40 183` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5060_f8,ldb_5160_f8,ldb_5260_f8,addimm_h50_f8,addimm_h51_f8,addimm_h52_f8,subimm_h50_f8,subimm_h51_f8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `fba1f97e01a9ef7e…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-056-log.md`.
7. Auto-spawn parallel-batch-51 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-51-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_374 | 0x17C | 0x80 LDB | 50 60 F8 | `498b87000300004881c0f8000000480fb60049898780020000c3` (26B) | `58d6062a26266dd7` |
| H_375 | 0x17D | 0x80 LDB | 51 60 F8 | `498b87000300004881c0f8000000480fb60049898788020000c3` (26B) | `03ca25f17de5059c` |
| H_376 | 0x17E | 0x80 LDB | 52 60 F8 | `498b87000300004881c0f8000000480fb60049898790020000c3` (26B) | `a94d6b39ac0bfbcd` |
| H_377 | 0x17F | 0x62 ADD-IMM | 50 F8 | `498b87800200004881c0f800000049898780020000c3` (22B) | `5179a4fbad6d4cda` |
| H_378 | 0x180 | 0x62 ADD-IMM | 51 F8 | `498b87880200004881c0f800000049898788020000c3` (22B) | `4670b7c563c506d0` |
| H_379 | 0x181 | 0x62 ADD-IMM | 52 F8 | `498b87900200004881c0f800000049898790020000c3` (22B) | `c84a511509fceff1` |
| H_380 | 0x182 | 0x61 SUB-IMM | 50 F8 | `498b87800200004881e8f800000049898780020000c3` (22B) | `9ffb9228f48ec264` |
| H_381 | 0x183 | 0x61 SUB-IMM | 51 F8 | `498b87880200004881e8f800000049898788020000c3` (22B) | `dbb8d1ae964b7218` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xF8 uses imm32 add (`48 81 c0`) → 22B pins (H_377..H_379); not imm8.
SUB-IMM imm=0xF8 uses imm32 sub (`48 81 e8`) → 22B pins (H_380..H_381); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_374..H_376).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_374 | `58d6062a26266dd726f7fe903ed872a53097e76cb8c1646e361a639a6bc8ac20` |
| H_375 | `03ca25f17de5059cc8205e34937baaee8eadf73082fa32ecc97870ffc23b752f` |
| H_376 | `a94d6b39ac0bfbcd810dcbbff961594e565bbdb2effa1269e6779871594477bd` |
| H_377 | `5179a4fbad6d4cdae7c1b54b9ce95ba981f453ce5a2be75ba5e38d4effcb2b0e` |
| H_378 | `4670b7c563c506d01ede9f962190ca8badc5cf631854d12917401214a61a2d0c` |
| H_379 | `c84a511509fceff17e958e241f8f0dab1ad2241b221e8043b0dd2dd04b2afc99` |
| H_380 | `9ffb9228f48ec264d8e4dfac651be3427eb69c82ccdb4ff9cc99b69892f6946a` |
| H_381 | `dbb8d1ae964b7218ed4aeb5d56e58a15797ee4527effb61b24f0755fb8751939` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5060_f8`, `_scratch_ldb_5160_f8`, `_scratch_ldb_5260_f8`, `_scratch_addimm_h50_f8`,
`_scratch_addimm_h51_f8`, `_scratch_addimm_h52_f8`, `_scratch_subimm_h50_f8`, `_scratch_subimm_h51_f8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 17C`.. for H_374.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
