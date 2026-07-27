# body-extend-088 SPAWN · consolidate parallel-batch-82

> Continuous queue handoff from parallel-batch-82 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `db550629db78a974cd83bec8db879fec415cd6fe37c94b35f57ce10a6917010d` (abbrev `db550629…`).
> Handlers = 635 (H_00..H_628). Last selectors: 0x273..0x27A = H_621..H_628 (`40 273`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-82-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-087-log.md` / `docs/auxdocs/body-extend-087-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-087 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 27B`.. for H_629.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 27B`/`40 280` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-088 (serialize + Relock)

Mirror body-extend-087 / body-extend-086 protocol:

1. Hand-author append H_629..H_636 to `yoyo/projects/yoyo.ty` at selectors `40 27B` .. `40 282` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h50_1D8,addimm_h51_1D8,addimm_h52_1D8,subimm_h50_1D8,subimm_h51_1D8,subimm_h52_1D8,ldb_5060_1E0,ldb_5160_1E0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `db550629db78a974cd83bec8db879fec415cd6fe37c94b35f57ce10a6917010d`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-088-log.md`.
7. Auto-spawn parallel-batch-83 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-83-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_629 | 0x27B | 0x62 ADD-IMM | 50 1D8 | `498b87800200004881c0d801000049898780020000c3` (22B) | `985fc739129b28e5` |
| H_630 | 0x27C | 0x62 ADD-IMM | 51 1D8 | `498b87880200004881c0d801000049898788020000c3` (22B) | `529c91e6cee0c610` |
| H_631 | 0x27D | 0x62 ADD-IMM | 52 1D8 | `498b87900200004881c0d801000049898790020000c3` (22B) | `11e5e0737f59a060` |
| H_632 | 0x27E | 0x61 SUB-IMM | 50 1D8 | `498b87800200004881e8d801000049898780020000c3` (22B) | `a387a1d628c84d7e` |
| H_633 | 0x27F | 0x61 SUB-IMM | 51 1D8 | `498b87880200004881e8d801000049898788020000c3` (22B) | `f7f546cac9fd3bab` |
| H_634 | 0x280 | 0x61 SUB-IMM | 52 1D8 | `498b87900200004881e8d801000049898790020000c3` (22B) | `9f9c8525bbf76801` |
| H_635 | 0x281 | 0x80 LDB | 50 60 1E0 | `498b87000300004881c0e0010000480fb60049898780020000c3` (26B) | `54ae10749db49954` |
| H_636 | 0x282 | 0x80 LDB | 51 60 1E0 | `498b87000300004881c0e0010000480fb60049898788020000c3` (26B) | `fab08f3c3976d127` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1D8 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1D8 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1E0 uses imm32 add (`48 81 c0`) → 26B pins.
ADD-IMM slot=50/51/52 imm=1D8 starts deferred 1D8 ADD triad (H_629/H_630/H_631).
SUB-IMM slot=50/51/52 imm=1D8 starts deferred 1D8 SUB triad (H_632/H_633/H_634).
LDB oo=0x1E0 starts 1E0 LDB triad (H_635/H_636 dd=50/51; LDB 52 1E0 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_629 | `985fc739129b28e5206f7a44af2242038d962dfab346dc8508df252fec254a18` |
| H_630 | `529c91e6cee0c6109a96db3ce2e03499601a8842698be17f37b7ae849b66ceb7` |
| H_631 | `11e5e0737f59a0604afe87cbec770b18cd8c096583d1007d45c2129506c8dccd` |
| H_632 | `a387a1d628c84d7eb3f1149a46c9d3568e2c7fa70e19b289f1fdc4d11735d875` |
| H_633 | `f7f546cac9fd3bab9ba41979b7477e535b189d1c59de5113ebea881f368f0c42` |
| H_634 | `9f9c8525bbf768014398e291dffe27d2523d9350d2b10d71c990bfafd40ff3bd` |
| H_635 | `54ae10749db49954ff3e0f998b123304cb8bdc5d88a0b54a728d95efb27044e9` |
| H_636 | `fab08f3c3976d127fad45fef564cdb0b17def5621b4350fcc26df85fd86693d0` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h50_1D8`, `_scratch_addimm_h51_1D8`, `_scratch_addimm_h52_1D8`, `_scratch_subimm_h50_1D8`,
`_scratch_subimm_h51_1D8`, `_scratch_subimm_h52_1D8`, `_scratch_ldb_5060_1E0`, `_scratch_ldb_5160_1E0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 27B`.. for H_629.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-83

- LDB dd=52 ss=60 oo=1E0 (finish 1E0 LDB triad)
- ADD-IMM / SUB-IMM slot=50/51/52 imm=1E0 (start 1E0 ADD/SUB triads)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
