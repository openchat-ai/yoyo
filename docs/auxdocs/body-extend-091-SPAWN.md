# body-extend-091 SPAWN · consolidate parallel-batch-85

> Continuous queue handoff from parallel-batch-85 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `63204ed031f1ad84c28688effab4ef4148b7c9e6277c1a08d68a7067dfe56aa1` (abbrev `63204ed0…`).
> Handlers = 659 (H_00..H_652). Last selectors: 0x28B..0x292 = H_645..H_652 (`40 28B`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-85-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-090-log.md` / `docs/auxdocs/body-extend-090-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-090 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 293`.. for H_653.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 293`/`40 29A` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-091 (serialize + Relock)

Mirror body-extend-090 / body-extend-089 protocol:

1. Hand-author append H_653..H_660 to `yoyo/projects/yoyo.ty` at selectors `40 293` .. `40 29A` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5060_1F0,ldb_5160_1F0,ldb_5260_1F0,addimm_h50_1F0,addimm_h51_1F0,addimm_h52_1F0,subimm_h50_1F0,subimm_h51_1F0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `63204ed031f1ad84c28688effab4ef4148b7c9e6277c1a08d68a7067dfe56aa1`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-091-log.md`.
7. Auto-spawn parallel-batch-86 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-86-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_653 | 0x293 | 0x80 LDB | 50 60 1F0 | `498b87000300004881c0f0010000480fb60049898780020000c3` (26B) | `1dd7536ff94f210b` |
| H_654 | 0x294 | 0x80 LDB | 51 60 1F0 | `498b87000300004881c0f0010000480fb60049898788020000c3` (26B) | `e2c1e0f004de6eab` |
| H_655 | 0x295 | 0x80 LDB | 52 60 1F0 | `498b87000300004881c0f0010000480fb60049898790020000c3` (26B) | `ad43445e924ece15` |
| H_656 | 0x296 | 0x62 ADD-IMM | 50 1F0 | `498b87800200004881c0f001000049898780020000c3` (22B) | `17b7b25157e9d135` |
| H_657 | 0x297 | 0x62 ADD-IMM | 51 1F0 | `498b87880200004881c0f001000049898788020000c3` (22B) | `ad1776283d15b543` |
| H_658 | 0x298 | 0x62 ADD-IMM | 52 1F0 | `498b87900200004881c0f001000049898790020000c3` (22B) | `3c8d698c14cd2075` |
| H_659 | 0x299 | 0x61 SUB-IMM | 50 1F0 | `498b87800200004881e8f001000049898780020000c3` (22B) | `43db5ead3bfc62f7` |
| H_660 | 0x29A | 0x61 SUB-IMM | 51 1F0 | `498b87880200004881e8f001000049898788020000c3` (22B) | `dac7533ba9ab5adb` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1F0 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1F0 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1F0 uses imm32 add (`48 81 c0`) → 26B pins.
LDB dd=50/51/52 ss=60 oo=1F0 starts deferred 1F0 LDB triad (H_653/H_654/H_655).
ADD-IMM slot=50/51/52 imm=1F0 starts deferred 1F0 ADD triad (H_656/H_657/H_658).
SUB-IMM slot=50/51 imm=1F0 starts deferred 1F0 SUB triad (H_659/H_660; SUB 52 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_653 | `1dd7536ff94f210bd645058b78b04d700e36ffcc02f83a27751f4884a3c5f452` |
| H_654 | `e2c1e0f004de6eab107bf86ea1c3731d8d3ad1fab935977c9c817f24b348b676` |
| H_655 | `ad43445e924ece151de60e3a022b4dc1bacc431c105858a57983bd4fe559e13e` |
| H_656 | `17b7b25157e9d1359b6bf473502a844f3b2ab639269729b6c365fec13cdd0507` |
| H_657 | `ad1776283d15b543ad830e89f699a757a239e8eb8aae61713f0260e5967a4c51` |
| H_658 | `3c8d698c14cd20755b85feb3d0d41c083447b42c4eca61ed1f06015fc5fae172` |
| H_659 | `43db5ead3bfc62f7b1ddc851953e2ef0966523e442f1e053cc1ebd4691764add` |
| H_660 | `dac7533ba9ab5adb7ba3cbddfedac2ea91a3c50bd8784a92a138e73671e06e9e` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5060_1F0`, `_scratch_ldb_5160_1F0`, `_scratch_ldb_5260_1F0`, `_scratch_addimm_h50_1F0`,
`_scratch_addimm_h51_1F0`, `_scratch_addimm_h52_1F0`, `_scratch_subimm_h50_1F0`, `_scratch_subimm_h51_1F0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 293`.. for H_653.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-86

- SUB-IMM slot=52 imm=1F0 (finish 1F0 SUB triad)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
