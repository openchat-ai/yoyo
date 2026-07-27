# body-extend-076 SPAWN · consolidate parallel-batch-70

> Continuous queue handoff from parallel-batch-70 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `69f1bb2f223e28673dfb97de72b1305d451313a4865d02e766ed947748a10cf9` (abbrev `69f1bb2f…`).
> Handlers = 539 (H_00..H_532). Last selectors: 0x213..0x21A = H_525..H_532 (`40 213`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-70-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-075-log.md` / `docs/auxdocs/body-extend-075-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-075 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 21B`.. for H_533.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 21B`/`40 21C` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-076 (serialize + Relock)

Mirror body-extend-075 / body-extend-074 protocol:

1. Hand-author append H_533..H_540 to `yoyo/projects/yoyo.ty` at selectors `40 21B` .. `40 222` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h50_180,subimm_h51_180,subimm_h52_180,ldb_5060_188,ldb_5160_188,ldb_5260_188,addimm_h50_188,addimm_h51_188}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `69f1bb2f223e28673dfb97de72b1305d451313a4865d02e766ed947748a10cf9`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-076-log.md`.
7. Auto-spawn parallel-batch-71 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-71-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_533 | 0x21B | 0x61 SUB-IMM | 50 180 | `498b87800200004881e88001000049898780020000c3` (22B) | `f31ae79928dbdd81` |
| H_534 | 0x21C | 0x61 SUB-IMM | 51 180 | `498b87880200004881e88001000049898788020000c3` (22B) | `050072b4e44aee5c` |
| H_535 | 0x21D | 0x61 SUB-IMM | 52 180 | `498b87900200004881e88001000049898790020000c3` (22B) | `6ad9c3df1ba66463` |
| H_536 | 0x21E | 0x80 LDB | 50 60 188 | `498b87000300004881c088010000480fb60049898780020000c3` (26B) | `18667432b27ded5f` |
| H_537 | 0x21F | 0x80 LDB | 51 60 188 | `498b87000300004881c088010000480fb60049898788020000c3` (26B) | `565922cabac58b5a` |
| H_538 | 0x220 | 0x80 LDB | 52 60 188 | `498b87000300004881c088010000480fb60049898790020000c3` (26B) | `0ccdef6304b031b3` |
| H_539 | 0x221 | 0x62 ADD-IMM | 50 188 | `498b87800200004881c08801000049898780020000c3` (22B) | `b9c2434436452b99` |
| H_540 | 0x222 | 0x62 ADD-IMM | 51 188 | `498b87880200004881c08801000049898788020000c3` (22B) | `4710e829b779fc66` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
SUB-IMM imm=0x180 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
ADD-IMM imm=0x188 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
LDB oo=0x188 starts 188 LDB triad (H_536 dd=50, H_537 dd=51, H_538 dd=52).
SUB-IMM slot=50/51/52 imm=180 finishes deferred 180 SUB triad (H_533/H_534/H_535).
ADD-IMM slot=52 imm=188 deferred to a later scratch batch.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_533 | `f31ae79928dbdd81951f15879484ce8c3f347dfc9d091d93d5655997fef8a891` |
| H_534 | `050072b4e44aee5c00ee9d299f1fccc170e724256993ea6237244e2c7c6bacc7` |
| H_535 | `6ad9c3df1ba6646323eaebc33e665e4cba6212891bc657d9b95e2135585b9ee0` |
| H_536 | `18667432b27ded5f98f065124b9ae0f537b16849d3cfb3d581f76fe51a551b0e` |
| H_537 | `565922cabac58b5a3c474b57981d5f7e084af4b6a2e0ae51f4dda2f710206a5f` |
| H_538 | `0ccdef6304b031b3dcc4752687b7c97f4c254aa7391506f05168f9662d26a17e` |
| H_539 | `b9c2434436452b99be0787c38ad6fb9c0679e92c716d4d9a112283396a7ddff3` |
| H_540 | `4710e829b779fc660201a66890536c1a7970e81284ab0be822197e187d4e5b31` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h50_180`, `_scratch_subimm_h51_180`, `_scratch_subimm_h52_180`, `_scratch_ldb_5060_188`,
`_scratch_ldb_5160_188`, `_scratch_ldb_5260_188`, `_scratch_addimm_h50_188`, `_scratch_addimm_h51_188`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 21B`.. for H_533.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
