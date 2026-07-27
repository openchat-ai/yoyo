# body-extend-062 SPAWN · consolidate parallel-batch-56

> Continuous queue handoff from parallel-batch-56 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `d4437da8f517c8d37c1335b590cae185c0be035d120d84f5ffa0e9354ae484a9` (abbrev `d4437da8…`).
> Handlers = 428 (H_00..H_421). Last selectors: 0x1A4..0x1AB = H_414..H_421 (`40 1A4`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-56-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-061-log.md` / `docs/auxdocs/body-extend-061-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-061 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1AC`.. for H_422.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1AC`/`40 1AD` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-062 (serialize + Relock)

Mirror body-extend-061 / body-extend-060 protocol:

1. Hand-author append H_422..H_429 to `yoyo/projects/yoyo.ty` at selectors `40 1AC` .. `40 1B3` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h50_120,addimm_h51_120,addimm_h52_120,subimm_h50_120,subimm_h51_120,subimm_h52_120,ldb_5060_128,ldb_5160_128}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `d4437da8f517c8d37c1335b590cae185c0be035d120d84f5ffa0e9354ae484a9`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-062-log.md`.
7. Auto-spawn parallel-batch-57 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-57-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_422 | 0x1AC | 0x62 ADD-IMM | 50 120 | `498b87800200004881c02001000049898780020000c3` (22B) | `ec142e42a7c76bc5` |
| H_423 | 0x1AD | 0x62 ADD-IMM | 51 120 | `498b87880200004881c02001000049898788020000c3` (22B) | `98a5ad08376f8e1a` |
| H_424 | 0x1AE | 0x62 ADD-IMM | 52 120 | `498b87900200004881c02001000049898790020000c3` (22B) | `4ffb72a7006ad4be` |
| H_425 | 0x1AF | 0x61 SUB-IMM | 50 120 | `498b87800200004881e82001000049898780020000c3` (22B) | `ac80c150be69c45f` |
| H_426 | 0x1B0 | 0x61 SUB-IMM | 51 120 | `498b87880200004881e82001000049898788020000c3` (22B) | `63cc573f936e533d` |
| H_427 | 0x1B1 | 0x61 SUB-IMM | 52 120 | `498b87900200004881e82001000049898790020000c3` (22B) | `587d869f509256fb` |
| H_428 | 0x1B2 | 0x80 LDB | 50 60 128 | `498b87000300004881c028010000480fb60049898780020000c3` (26B) | `753ecfc2db0ae0be` |
| H_429 | 0x1B3 | 0x80 LDB | 51 60 128 | `498b87000300004881c028010000480fb60049898788020000c3` (26B) | `6aa74dbb4c649602` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x120 uses imm32 add (`48 81 c0`) → 22B pins (H_422..H_424); not imm8.
SUB-IMM imm=0x120 uses imm32 sub (`48 81 e8`) → 22B pins (H_425..H_427); not imm8.
LDB oo=0x128 uses imm32 add (`48 81 c0`) → 26B pins (H_428..H_429).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_422 | `ec142e42a7c76bc5ee048bfa19bdbddee63a69fbd007cf5e50164da1b5b43cc3` |
| H_423 | `98a5ad08376f8e1a58183e2e727d570627bb2246fcdb991d674f88a9840bc999` |
| H_424 | `4ffb72a7006ad4be4d889189084b03e235879fa92a3fea0c2746aca4ef3d9b53` |
| H_425 | `ac80c150be69c45f64a085fa9af1de68d8a518e585c5f16a8074be8e9d2346ca` |
| H_426 | `63cc573f936e533d455b5b5c4c6006bd92e940d7c9706f97ac77b92946605d7e` |
| H_427 | `587d869f509256fbfc04222665313076ac453ef8e44e174fa2867e5587202831` |
| H_428 | `753ecfc2db0ae0be4ed16b35417fc1ad0556ca38075595b85c4d6a1c0d99e14f` |
| H_429 | `6aa74dbb4c6496022ae0c6be1510e534d21026dcb4de8800dfbc198f9ff2f58b` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h50_120`, `_scratch_addimm_h51_120`, `_scratch_addimm_h52_120`, `_scratch_subimm_h50_120`,
`_scratch_subimm_h51_120`, `_scratch_subimm_h52_120`, `_scratch_ldb_5060_128`, `_scratch_ldb_5160_128`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1AC`.. for H_422.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
