# body-extend-069 SPAWN · consolidate parallel-batch-63

> Continuous queue handoff from parallel-batch-63 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `2f81b43ba9e34a3bbc786fc9d308d0cc6d38c866dfdfd8e52a51bfed15acb5b8` (abbrev `2f81b43b…`).
> Handlers = 483 (H_00..H_476). Last selectors: 0x1DB..0x1E2 = H_469..H_476 (`40 1DB`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-63-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-068-log.md` / `docs/auxdocs/body-extend-068-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-068 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1E3`.. for H_477.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1E3`/`40 1E4` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-069 (serialize + Relock)

Mirror body-extend-068 / body-extend-067 protocol:

1. Hand-author append H_477..H_484 to `yoyo/projects/yoyo.ty` at selectors `40 1E3` .. `40 1EA` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h51_150,addimm_h52_150,subimm_h50_150,subimm_h51_150,subimm_h52_150,ldb_5060_158,ldb_5160_158,ldb_5260_158}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `2f81b43ba9e34a3bbc786fc9d308d0cc6d38c866dfdfd8e52a51bfed15acb5b8`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-069-log.md`.
7. Auto-spawn parallel-batch-64 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-64-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_477 | 0x1E3 | 0x62 ADD-IMM | 51 150 | `498b87880200004881c05001000049898788020000c3` (22B) | `f1c7dd6bfae2b6d9` |
| H_478 | 0x1E4 | 0x62 ADD-IMM | 52 150 | `498b87900200004881c05001000049898790020000c3` (22B) | `ad7c246ef8f39fcf` |
| H_479 | 0x1E5 | 0x61 SUB-IMM | 50 150 | `498b87800200004881e85001000049898780020000c3` (22B) | `ae63f624dd2b47e7` |
| H_480 | 0x1E6 | 0x61 SUB-IMM | 51 150 | `498b87880200004881e85001000049898788020000c3` (22B) | `b89379b68feff397` |
| H_481 | 0x1E7 | 0x61 SUB-IMM | 52 150 | `498b87900200004881e85001000049898790020000c3` (22B) | `55fb7454745b2924` |
| H_482 | 0x1E8 | 0x80 LDB | 50 60 158 | `498b87000300004881c058010000480fb60049898780020000c3` (26B) | `0c2958ba1b0da5ee` |
| H_483 | 0x1E9 | 0x80 LDB | 51 60 158 | `498b87000300004881c058010000480fb60049898788020000c3` (26B) | `84dc9a2cf6fd51dc` |
| H_484 | 0x1EA | 0x80 LDB | 52 60 158 | `498b87000300004881c058010000480fb60049898790020000c3` (26B) | `28656b49e0c172e0` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x150 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x150 uses imm32 sub (`48 81 e8`) → 22B pins (H_479..H_481); not imm8.
LDB oo=0x158 uses imm32 add (`48 81 c0`) → 26B pins (H_482..H_484); starts 158 LDB triad.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_477 | `f1c7dd6bfae2b6d912c7476955e45d2ac3cb27d63d27f1c26e1c38828d7977fa` |
| H_478 | `ad7c246ef8f39fcfa54640805da6024a4a40044942606a11f57b9db4568f37f6` |
| H_479 | `ae63f624dd2b47e7cd1810c2ce208512abc1b66e0852e34b8b881f14494bc204` |
| H_480 | `b89379b68feff3975d9bbaa0ef008ef66bf3d4b8d13b356213de6ada87e2ebd8` |
| H_481 | `55fb7454745b29240dff3640219c9007bcecba33e856f2cf629635a4c1732bc6` |
| H_482 | `0c2958ba1b0da5ee2d9aeec8a2db14d4b3afeec84c9b41070f0efb48452d165c` |
| H_483 | `84dc9a2cf6fd51dcb5925778314a8c216772ac4992d0ebc6b2947be87f839eb4` |
| H_484 | `28656b49e0c172e016076e498b32e2d0fc96787ade59805a3f8d0c298aff78c7` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h51_150`, `_scratch_addimm_h52_150`, `_scratch_subimm_h50_150`, `_scratch_subimm_h51_150`,
`_scratch_subimm_h52_150`, `_scratch_ldb_5060_158`, `_scratch_ldb_5160_158`, `_scratch_ldb_5260_158`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1E3`.. for H_477.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
