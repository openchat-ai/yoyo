# body-extend-075 SPAWN · consolidate parallel-batch-69

> Continuous queue handoff from parallel-batch-69 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `9243965c886555e99575615e4637331b6c2a49573d50ec183fb616c3ae3d2d98` (abbrev `9243965c…`).
> Handlers = 531 (H_00..H_524). Last selectors: 0x20B..0x212 = H_517..H_524 (`40 20B`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-69-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-074-log.md` / `docs/auxdocs/body-extend-074-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-074 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 213`.. for H_525.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 213`/`40 214` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-075 (serialize + Relock)

Mirror body-extend-074 / body-extend-073 protocol:

1. Hand-author append H_525..H_532 to `yoyo/projects/yoyo.ty` at selectors `40 213` .. `40 21A` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h51_178,subimm_h52_178,ldb_5060_180,ldb_5160_180,ldb_5260_180,addimm_h50_180,addimm_h51_180,addimm_h52_180}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `9243965c886555e99575615e4637331b6c2a49573d50ec183fb616c3ae3d2d98`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-075-log.md`.
7. Auto-spawn parallel-batch-70 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-70-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_525 | 0x213 | 0x61 SUB-IMM | 51 178 | `498b87880200004881e87801000049898788020000c3` (22B) | `c18f2917305b68fa` |
| H_526 | 0x214 | 0x61 SUB-IMM | 52 178 | `498b87900200004881e87801000049898790020000c3` (22B) | `29f631d8a2fd2ed7` |
| H_527 | 0x215 | 0x80 LDB | 50 60 180 | `498b87000300004881c080010000480fb60049898780020000c3` (26B) | `b83050617eb70487` |
| H_528 | 0x216 | 0x80 LDB | 51 60 180 | `498b87000300004881c080010000480fb60049898788020000c3` (26B) | `8905cf5ed3ca338f` |
| H_529 | 0x217 | 0x80 LDB | 52 60 180 | `498b87000300004881c080010000480fb60049898790020000c3` (26B) | `c958b80396d606de` |
| H_530 | 0x218 | 0x62 ADD-IMM | 50 180 | `498b87800200004881c08001000049898780020000c3` (22B) | `1c96efa23061fbf4` |
| H_531 | 0x219 | 0x62 ADD-IMM | 51 180 | `498b87880200004881c08001000049898788020000c3` (22B) | `8732710ac0cc4d60` |
| H_532 | 0x21A | 0x62 ADD-IMM | 52 180 | `498b87900200004881c08001000049898790020000c3` (22B) | `b32b4364c0efbe04` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x180 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x178 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x180 starts 180 LDB triad (H_527 dd=50, H_528 dd=51, H_529 dd=52).
SUB-IMM slot=51/52 imm=178 finishes deferred 178 SUB triad (H_525/H_526; H_524 was slot=50).
SUB-IMM slot=50/51/52 imm=180 deferred to a later scratch batch.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_525 | `c18f2917305b68faf3ecee09ce6d76e94a17b19bc14fafdc604b2ae3d1fa8aaf` |
| H_526 | `29f631d8a2fd2ed7623889a6357eeac2bdf4327d41dc274dd257293f917a01e5` |
| H_527 | `b83050617eb70487ee68e88992b6b35c8863ed9424cde661db3f4d51ebc8fb36` |
| H_528 | `8905cf5ed3ca338ffc6c372b6ab744ec6dd06850be653613669b8320543ecd9b` |
| H_529 | `c958b80396d606de3757923cfb2dea5e473ff1337ad4a674e9ba06f408b5268b` |
| H_530 | `1c96efa23061fbf4a12fbd69e1e45d5e250cd8f825074da7f8b82f938a348106` |
| H_531 | `8732710ac0cc4d604d003e0d4b83bf8fa16ebc22c512b3bb2d9a2a4132fece6a` |
| H_532 | `b32b4364c0efbe042ce84930704039fd1849e30c1755dd1e73fc2136376b8c09` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h51_178`, `_scratch_subimm_h52_178`, `_scratch_ldb_5060_180`, `_scratch_ldb_5160_180`,
`_scratch_ldb_5260_180`, `_scratch_addimm_h50_180`, `_scratch_addimm_h51_180`, `_scratch_addimm_h52_180`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 213`.. for H_525.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
