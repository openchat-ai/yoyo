# body-extend-078 SPAWN · consolidate parallel-batch-72

> Continuous queue handoff from parallel-batch-72 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `97ce84a29adb8c400408d7fec9d2d58a820766a61c18068b1b61eac59946e2b0` (abbrev `97ce84a2…`).
> Handlers = 555 (H_00..H_548). Last selectors: 0x223..0x22A = H_541..H_548 (`40 223`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-72-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-077-log.md` / `docs/auxdocs/body-extend-077-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-077 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 22B`.. for H_549.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 22B`/`40 22C` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-078 (serialize + Relock)

Mirror body-extend-077 / body-extend-076 protocol:

1. Hand-author append H_549..H_556 to `yoyo/projects/yoyo.ty` at selectors `40 22B` .. `40 232` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h51_190,addimm_h52_190,subimm_h50_190,subimm_h51_190,subimm_h52_190,ldb_5060_198,ldb_5160_198,ldb_5260_198}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `97ce84a29adb8c400408d7fec9d2d58a820766a61c18068b1b61eac59946e2b0`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-078-log.md`.
7. Auto-spawn parallel-batch-73 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-73-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_549 | 0x22B | 0x62 ADD-IMM | 51 190 | `498b87880200004881c09001000049898788020000c3` (22B) | `5248421affee5c66` |
| H_550 | 0x22C | 0x62 ADD-IMM | 52 190 | `498b87900200004881c09001000049898790020000c3` (22B) | `648351f8db48af34` |
| H_551 | 0x22D | 0x61 SUB-IMM | 50 190 | `498b87800200004881e89001000049898780020000c3` (22B) | `f7e06d035b717d9d` |
| H_552 | 0x22E | 0x61 SUB-IMM | 51 190 | `498b87880200004881e89001000049898788020000c3` (22B) | `489b9cd85b80cad9` |
| H_553 | 0x22F | 0x61 SUB-IMM | 52 190 | `498b87900200004881e89001000049898790020000c3` (22B) | `0535305934d986e2` |
| H_554 | 0x230 | 0x80 LDB | 50 60 198 | `498b87000300004881c098010000480fb60049898780020000c3` (26B) | `f68f3fdd889f57db` |
| H_555 | 0x231 | 0x80 LDB | 51 60 198 | `498b87000300004881c098010000480fb60049898788020000c3` (26B) | `1fd1cefc37ee2f6a` |
| H_556 | 0x232 | 0x80 LDB | 52 60 198 | `498b87000300004881c098010000480fb60049898790020000c3` (26B) | `84e2d29d21835c65` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x190 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x190 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x198 starts 198 LDB triad (H_554 dd=50, H_555 dd=51, H_556 dd=52).
ADD-IMM slot=51/52 imm=190 finishes deferred 190 ADD triad (H_549/H_550).
SUB-IMM slot=50/51/52 imm=190 starts 190 SUB triad (H_551/H_552/H_553).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_549 | `5248421affee5c6657036a849aa23a1e5dfc1fe09c3c78d6ba06947038c8fccb` |
| H_550 | `648351f8db48af34920624da1b6b8d4df396537d997f5d58ddcb7aaa1c9dfe13` |
| H_551 | `f7e06d035b717d9d783242b3bd9592372b879c5e7a3430e5fec96e4651567ef0` |
| H_552 | `489b9cd85b80cad9d028bde2952f8ea33130dce8590b16075ac241d7ba5db55e` |
| H_553 | `0535305934d986e2de06cc9fba0add950c71f2de2df62c86f61efec012dce7b7` |
| H_554 | `f68f3fdd889f57db0bd956d924302f6e0b15d18498ec45b3ba3ad6c3b0f2637a` |
| H_555 | `1fd1cefc37ee2f6a1243b5a4886c08ecaf023e56b8d3e08babd762e4ba45fd3b` |
| H_556 | `84e2d29d21835c6502cff74e89e6fdc5c3bec404c1e20379cc6eb1af9d7cb26d` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h51_190`, `_scratch_addimm_h52_190`, `_scratch_subimm_h50_190`, `_scratch_subimm_h51_190`,
`_scratch_subimm_h52_190`, `_scratch_ldb_5060_198`, `_scratch_ldb_5160_198`, `_scratch_ldb_5260_198`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 22B`.. for H_549.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
