# body-extend-094 SPAWN · consolidate parallel-batch-88

> Continuous queue handoff from parallel-batch-88 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `04656bbbbb152b5402bd76daa324a51a7f68477df3b3615827ef88aa2907978b` (abbrev `04656bbb…`).
> Handlers = 683 (H_00..H_676). Last selectors: 0x2A3..0x2AA = H_669..H_676 (`40 2A3`..`40 2AA` via label-width A).
> Source: `docs/auxdocs/parallel-batch-88-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-093-log.md` / `docs/auxdocs/body-extend-093-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-093 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 2AB`.. for H_677.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 2AB`/`40 2B2` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-094 (serialize + Relock)

Mirror body-extend-093 / body-extend-092 protocol:

1. Hand-author append H_677..H_684 to `yoyo/projects/yoyo.ty` at selectors `40 2AB` .. `40 2B2` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h50_200,subimm_h51_200,subimm_h52_200,ldb_5060_208,ldb_5160_208,ldb_5260_208,addimm_h50_208,addimm_h51_208}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `04656bbbbb152b5402bd76daa324a51a7f68477df3b3615827ef88aa2907978b`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-094-log.md`.
7. Auto-spawn parallel-batch-89 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-89-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_677 | 0x2AB | 0x61 SUB-IMM | 50 200 | `498b87800200004881e80002000049898780020000c3` (22B) | `616e435fa3303d6d` |
| H_678 | 0x2AC | 0x61 SUB-IMM | 51 200 | `498b87880200004881e80002000049898788020000c3` (22B) | `c68ac43f8d46d532` |
| H_679 | 0x2AD | 0x61 SUB-IMM | 52 200 | `498b87900200004881e80002000049898790020000c3` (22B) | `aa5d87726f97aedf` |
| H_680 | 0x2AE | 0x80 LDB | 50 60 208 | `498b87000300004881c008020000480fb60049898780020000c3` (26B) | `454561f22b4cd018` |
| H_681 | 0x2AF | 0x80 LDB | 51 60 208 | `498b87000300004881c008020000480fb60049898788020000c3` (26B) | `4d6d099ee46ef004` |
| H_682 | 0x2B0 | 0x80 LDB | 52 60 208 | `498b87000300004881c008020000480fb60049898790020000c3` (26B) | `49ede9483394add3` |
| H_683 | 0x2B1 | 0x62 ADD-IMM | 50 208 | `498b87800200004881c00802000049898780020000c3` (22B) | `20c12c152bbba594` |
| H_684 | 0x2B2 | 0x62 ADD-IMM | 51 208 | `498b87880200004881c00802000049898788020000c3` (22B) | `612703982c8eadbb` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x208 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x200 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x208 uses imm32 add (`48 81 c0`) → 26B pins.
SUB-IMM slot=50/51/52 imm=200 finishes deferred 200 SUB triad (H_677/H_678/H_679).
LDB dd=50/51/52 ss=60 oo=208 starts 208 LDB triad (H_680/H_681/H_682).
ADD-IMM slot=50/51 imm=208 starts 208 ADD triad (H_683/H_684).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_677 | `616e435fa3303d6d6ba0710790f2689e99f6628131c1c135d4560e43a12ce990` |
| H_678 | `c68ac43f8d46d532ba9c6f4d1d060cc3b879145e6a2e6770f015e6145d763379` |
| H_679 | `aa5d87726f97aedfbd932d90d43047686e049ef1ac3a86a8492b02739b852c73` |
| H_680 | `454561f22b4cd018ef79befab6dd2911e4dfb00566eeabd5111866aea8ff8895` |
| H_681 | `4d6d099ee46ef0045a4eb3e81c5b58b73a4a8ad82b907f9192f1368a33112139` |
| H_682 | `49ede9483394add3545ffd850a337cf4e2a608953a0b1db5a7bbce046b8ea331` |
| H_683 | `20c12c152bbba59406e5c82303bc3ccd3ddc945fdf57984d2226d57c16426da0` |
| H_684 | `612703982c8eadbb83922b686ef84d5dd929cde146cc8e77328b01241092d313` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h50_200`, `_scratch_subimm_h51_200`, `_scratch_subimm_h52_200`, `_scratch_ldb_5060_208`,
`_scratch_ldb_5160_208`, `_scratch_ldb_5260_208`, `_scratch_addimm_h50_208`, `_scratch_addimm_h51_208`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 2AB`.. for H_677.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-89

- ADD-IMM slot=52 imm=208 (finish 208 ADD triad)
- SUB-IMM slot=50/51/52 imm=208 (start 208 SUB triad)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
