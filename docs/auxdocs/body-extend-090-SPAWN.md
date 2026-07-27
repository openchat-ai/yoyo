# body-extend-090 SPAWN · consolidate parallel-batch-84

> Continuous queue handoff from parallel-batch-84 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `e8500277650750c55bc94ec1a9c5e0277367daa257b09371e33f569a8d46c129` (abbrev `e8500277…`).
> Handlers = 651 (H_00..H_644). Last selectors: 0x283..0x28A = H_637..H_644 (`40 283`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-84-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-089-log.md` / `docs/auxdocs/body-extend-089-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-089 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 28B`.. for H_645.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 28B`/`40 292` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-090 (serialize + Relock)

Mirror body-extend-089 / body-extend-088 protocol:

1. Hand-author append H_645..H_652 to `yoyo/projects/yoyo.ty` at selectors `40 28B` .. `40 292` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5160_1E8,ldb_5260_1E8,addimm_h50_1E8,addimm_h51_1E8,addimm_h52_1E8,subimm_h50_1E8,subimm_h51_1E8,subimm_h52_1E8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `e8500277650750c55bc94ec1a9c5e0277367daa257b09371e33f569a8d46c129`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-090-log.md`.
7. Auto-spawn parallel-batch-85 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-85-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_645 | 0x28B | 0x80 LDB | 51 60 1E8 | `498b87000300004881c0e8010000480fb60049898788020000c3` (26B) | `ba62e4ad2c2e56ee` |
| H_646 | 0x28C | 0x80 LDB | 52 60 1E8 | `498b87000300004881c0e8010000480fb60049898790020000c3` (26B) | `aac7a387b001d803` |
| H_647 | 0x28D | 0x62 ADD-IMM | 50 1E8 | `498b87800200004881c0e801000049898780020000c3` (22B) | `a63c229b97189c94` |
| H_648 | 0x28E | 0x62 ADD-IMM | 51 1E8 | `498b87880200004881c0e801000049898788020000c3` (22B) | `356a1a0b3408f7f6` |
| H_649 | 0x28F | 0x62 ADD-IMM | 52 1E8 | `498b87900200004881c0e801000049898790020000c3` (22B) | `ea596d905acbddb7` |
| H_650 | 0x290 | 0x61 SUB-IMM | 50 1E8 | `498b87800200004881e8e801000049898780020000c3` (22B) | `0e13aa7197e06d20` |
| H_651 | 0x291 | 0x61 SUB-IMM | 51 1E8 | `498b87880200004881e8e801000049898788020000c3` (22B) | `58e9756f847685f3` |
| H_652 | 0x292 | 0x61 SUB-IMM | 52 1E8 | `498b87900200004881e8e801000049898790020000c3` (22B) | `eeda72c92f5324fc` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1E8 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1E8 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1E8 uses imm32 add (`48 81 c0`) → 26B pins.
LDB dd=51/52 ss=60 oo=1E8 finishes deferred 1E8 LDB triad (H_645/H_646; H_644 dd=50 already locked).
ADD-IMM slot=50/51/52 imm=1E8 starts deferred 1E8 ADD triad (H_647/H_648/H_649).
SUB-IMM slot=50/51/52 imm=1E8 starts deferred 1E8 SUB triad (H_650/H_651/H_652).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_645 | `ba62e4ad2c2e56ee2ffdfc86fb5d52b43bc7ff65642a4246282f010fbdd9d5d1` |
| H_646 | `aac7a387b001d803071588118024c4b3edd529e4996f70f642c645e5d2eeed22` |
| H_647 | `a63c229b97189c942fd07bdd4622bcfcc67f550f5e4fe7972808180865b7ed9f` |
| H_648 | `356a1a0b3408f7f686339abad6a21ef6d856e7db3c340818c548755f60751813` |
| H_649 | `ea596d905acbddb77450f2f693618792308c05549e59eeeae5b4d04cdb102a04` |
| H_650 | `0e13aa7197e06d2067d67e5ce88f977dd7c9dc1746ef126e65a69268df08d635` |
| H_651 | `58e9756f847685f381c05f00f272297a49fba2942d372b4fea3c875df5fbed2f` |
| H_652 | `eeda72c92f5324fcc96b121d2202021424f924135b4b7baa7cbc96156e26585e` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5160_1E8`, `_scratch_ldb_5260_1E8`, `_scratch_addimm_h50_1E8`, `_scratch_addimm_h51_1E8`,
`_scratch_addimm_h52_1E8`, `_scratch_subimm_h50_1E8`, `_scratch_subimm_h51_1E8`, `_scratch_subimm_h52_1E8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 28B`.. for H_645.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-85

- LDB / ADD-IMM / SUB-IMM imm=1F0 (start next ladder) if triad space allows
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
