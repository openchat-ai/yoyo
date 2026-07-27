# body-extend-092 SPAWN · consolidate parallel-batch-86

> Continuous queue handoff from parallel-batch-86 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `339bd482ae784eb8a80f7176ef5d7c6f3c90b0e491b08c6103512860ab5b918a` (abbrev `339bd482…`).
> Handlers = 667 (H_00..H_660). Last selectors: 0x293..0x29A = H_653..H_660 (`40 293`..`40 29A` via label-width A).
> Source: `docs/auxdocs/parallel-batch-86-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-091-log.md` / `docs/auxdocs/body-extend-091-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-091 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 29B`.. for H_661.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 29B`/`40 2A2` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-092 (serialize + Relock)

Mirror body-extend-091 / body-extend-090 protocol:

1. Hand-author append H_661..H_668 to `yoyo/projects/yoyo.ty` at selectors `40 29B` .. `40 2A2` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h52_1F0,ldb_5060_1F8,ldb_5160_1F8,ldb_5260_1F8,addimm_h50_1F8,addimm_h51_1F8,addimm_h52_1F8,subimm_h50_1F8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `339bd482ae784eb8a80f7176ef5d7c6f3c90b0e491b08c6103512860ab5b918a`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-092-log.md`.
7. Auto-spawn parallel-batch-87 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-87-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_661 | 0x29B | 0x61 SUB-IMM | 52 1F0 | `498b87900200004881e8f001000049898790020000c3` (22B) | `21a46af767b04e47` |
| H_662 | 0x29C | 0x80 LDB | 50 60 1F8 | `498b87000300004881c0f8010000480fb60049898780020000c3` (26B) | `e33190513a0b6fac` |
| H_663 | 0x29D | 0x80 LDB | 51 60 1F8 | `498b87000300004881c0f8010000480fb60049898788020000c3` (26B) | `754738a2ae8287ba` |
| H_664 | 0x29E | 0x80 LDB | 52 60 1F8 | `498b87000300004881c0f8010000480fb60049898790020000c3` (26B) | `b3d0c040cbafd1ed` |
| H_665 | 0x29F | 0x62 ADD-IMM | 50 1F8 | `498b87800200004881c0f801000049898780020000c3` (22B) | `e4eb4882c94f477d` |
| H_666 | 0x2A0 | 0x62 ADD-IMM | 51 1F8 | `498b87880200004881c0f801000049898788020000c3` (22B) | `767adbf6b2f425c9` |
| H_667 | 0x2A1 | 0x62 ADD-IMM | 52 1F8 | `498b87900200004881c0f801000049898790020000c3` (22B) | `5e4ebbbafb63edb5` |
| H_668 | 0x2A2 | 0x61 SUB-IMM | 50 1F8 | `498b87800200004881e8f801000049898780020000c3` (22B) | `8ebe141b655cf99d` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1F8 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1F0/0x1F8 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1F8 uses imm32 add (`48 81 c0`) → 26B pins.
SUB-IMM slot=52 imm=1F0 finishes deferred 1F0 SUB triad (H_661).
LDB dd=50/51/52 ss=60 oo=1F8 starts 1F8 LDB triad (H_662/H_663/H_664).
ADD-IMM slot=50/51/52 imm=1F8 starts 1F8 ADD triad (H_665/H_666/H_667).
SUB-IMM slot=50 imm=1F8 starts 1F8 SUB triad (H_668; SUB 51/52 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_661 | `21a46af767b04e47650b619132dce8c0d8eb8853d90a43a9d7d1d28af98d0f1a` |
| H_662 | `e33190513a0b6fac13932e72229b5bbafeaff7083cc224cf8c53338378fee9c0` |
| H_663 | `754738a2ae8287ba25dd22fcc9ffef4d583b37c9061d495e26db3499044f8770` |
| H_664 | `b3d0c040cbafd1ed0af4dbfd1514fe59fcc1c44f55d5b025a1d4602cba0cfd12` |
| H_665 | `e4eb4882c94f477d7369f849651cad1e3e4ebd2dbe762d456c009667bc3d37ad` |
| H_666 | `767adbf6b2f425c9fb3363b309c6fe79e3d1ba7874531cc4b34f0706d7e1b3c0` |
| H_667 | `5e4ebbbafb63edb539dc75ecd54dd72456e8ff9afa398a2f746507a3c3f3ba2a` |
| H_668 | `8ebe141b655cf99deb6c55f0869d36be0725d1fef0b58268d6d6751190a816a5` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h52_1F0`, `_scratch_ldb_5060_1F8`, `_scratch_ldb_5160_1F8`, `_scratch_ldb_5260_1F8`,
`_scratch_addimm_h50_1F8`, `_scratch_addimm_h51_1F8`, `_scratch_addimm_h52_1F8`, `_scratch_subimm_h50_1F8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 29B`.. for H_661.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-87

- SUB-IMM slot=51/52 imm=1F8 (finish 1F8 SUB triad)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
