# body-extend-095 SPAWN · consolidate parallel-batch-89

> Continuous queue handoff from parallel-batch-89 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `0ef9611b50021d82d2c7870a29d1d4107164b7a3c586f41f5271a083fbdfec51` (abbrev `0ef9611b…`).
> Handlers = 691 (H_00..H_684). Last selectors: 0x2AB..0x2B2 = H_677..H_684 (`40 2AB`..`40 2B2` via label-width A).
> Source: `docs/auxdocs/parallel-batch-89-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-094-log.md` / `docs/auxdocs/body-extend-094-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-094 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 2B3`.. for H_685.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 2B3`/`40 2BA` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-095 (serialize + Relock)

Mirror body-extend-094 / body-extend-093 protocol:

1. Hand-author append H_685..H_692 to `yoyo/projects/yoyo.ty` at selectors `40 2B3` .. `40 2BA` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h52_208,subimm_h50_208,subimm_h51_208,subimm_h52_208,ldb_5060_210,ldb_5160_210,ldb_5260_210,addimm_h50_210}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `0ef9611b50021d82d2c7870a29d1d4107164b7a3c586f41f5271a083fbdfec51`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-095-log.md`.
7. Auto-spawn parallel-batch-90 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-90-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_685 | 0x2B3 | 0x62 ADD-IMM | 52 208 | `498b87900200004881c00802000049898790020000c3` (22B) | `bb7306a6accdaf1d` |
| H_686 | 0x2B4 | 0x61 SUB-IMM | 50 208 | `498b87800200004881e80802000049898780020000c3` (22B) | `f7711234e1f246db` |
| H_687 | 0x2B5 | 0x61 SUB-IMM | 51 208 | `498b87880200004881e80802000049898788020000c3` (22B) | `71f14163af6727da` |
| H_688 | 0x2B6 | 0x61 SUB-IMM | 52 208 | `498b87900200004881e80802000049898790020000c3` (22B) | `b95b3672e4031732` |
| H_689 | 0x2B7 | 0x80 LDB | 50 60 210 | `498b87000300004881c010020000480fb60049898780020000c3` (26B) | `e5d730581fb17e84` |
| H_690 | 0x2B8 | 0x80 LDB | 51 60 210 | `498b87000300004881c010020000480fb60049898788020000c3` (26B) | `ebbb4b6905b61aa1` |
| H_691 | 0x2B9 | 0x80 LDB | 52 60 210 | `498b87000300004881c010020000480fb60049898790020000c3` (26B) | `62a53f91d97addee` |
| H_692 | 0x2BA | 0x62 ADD-IMM | 50 210 | `498b87800200004881c01002000049898780020000c3` (22B) | `b28afba882b0e6c1` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x208/0x210 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x208 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x210 uses imm32 add (`48 81 c0`) → 26B pins.
ADD-IMM slot=52 imm=208 finishes deferred 208 ADD triad (H_685).
SUB-IMM slot=50/51/52 imm=208 starts 208 SUB triad (H_686/H_687/H_688).
LDB dd=50/51/52 ss=60 oo=210 starts 210 LDB triad (H_689/H_690/H_691).
ADD-IMM slot=50 imm=210 starts 210 ADD triad (H_692).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_685 | `bb7306a6accdaf1dd37bfa4c5811e5bed548e84ddf0e5182504d0a09bce5d0e2` |
| H_686 | `f7711234e1f246db5dde5d4b6bbd3c11b32f6a77fe2f884fd2dfcb6249890718` |
| H_687 | `71f14163af6727da4a31441ed23a21bb6a633eee6a4bfadd8308a0dc0ace6137` |
| H_688 | `b95b3672e4031732805ad8afb821479b85f2377415fd62a42e632eb19bce70ef` |
| H_689 | `e5d730581fb17e8481d5891459d7eecc79083d0cc1c554c6459d1e1b4c589e17` |
| H_690 | `ebbb4b6905b61aa16c3a1d05a370e9e1532f824bdbb7b3af8c9c52e360b0d3b9` |
| H_691 | `62a53f91d97addee0e04e77ad9d5f2f3a6de52ce7176a891db4a7e41903cbe04` |
| H_692 | `b28afba882b0e6c1af6d419af5ec0fb99881e922e186e75ea167ec65edfeb6d0` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h52_208`, `_scratch_subimm_h50_208`, `_scratch_subimm_h51_208`, `_scratch_subimm_h52_208`,
`_scratch_ldb_5060_210`, `_scratch_ldb_5160_210`, `_scratch_ldb_5260_210`, `_scratch_addimm_h50_210`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 2B3`.. for H_685.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-90

- ADD-IMM slot=51/52 imm=210 (finish 210 ADD triad)
- SUB-IMM slot=50/51/52 imm=210 (start 210 SUB triad)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
