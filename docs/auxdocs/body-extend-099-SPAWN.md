# body-extend-099 SPAWN · consolidate parallel-batch-93

> Continuous queue handoff from parallel-batch-93 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `8d4277255b098dc108295590e42155afd50ffca67fbab34ea1430ef615405136` (abbrev `8d427725…`).
> Handlers = 723 (H_00..H_716). Last selectors: 0x2CB..0x2D2 = H_709..H_716 (`40 2CB`..`40 2D2` via label-width A).
> Source: `docs/auxdocs/parallel-batch-93-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-098-log.md` / `docs/auxdocs/body-extend-098-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-098 DDC PE `.text` measured DIFFER this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 2D3`.. for H_717.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 2D3`/`40 2DA` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-099 (serialize + Relock)

Mirror body-extend-098 / body-extend-097 protocol:

1. Hand-author append H_717..H_724 to `yoyo/projects/yoyo.ty` at selectors `40 2D3` .. `40 2DA` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5160_228,ldb_5260_228,addimm_h50_228,addimm_h51_228,addimm_h52_228,subimm_h50_228,subimm_h51_228,subimm_h52_228}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `8d4277255b098dc108295590e42155afd50ffca67fbab34ea1430ef615405136`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-099-log.md`.
7. Auto-spawn parallel-batch-94 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-94-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_717 | 0x2D3 | 0x80 LDB | 51 60 228 | `498b87000300004881c028020000480fb60049898788020000c3` (26B) | `ec662f4d79ff8add` |
| H_718 | 0x2D4 | 0x80 LDB | 52 60 228 | `498b87000300004881c028020000480fb60049898790020000c3` (26B) | `0a14cf8c72933615` |
| H_719 | 0x2D5 | 0x62 ADD-IMM | 50 228 | `498b87800200004881c02802000049898780020000c3` (22B) | `308359b06a3c0b71` |
| H_720 | 0x2D6 | 0x62 ADD-IMM | 51 228 | `498b87880200004881c02802000049898788020000c3` (22B) | `30a3548d2b182ab8` |
| H_721 | 0x2D7 | 0x62 ADD-IMM | 52 228 | `498b87900200004881c02802000049898790020000c3` (22B) | `bb5db527c469beec` |
| H_722 | 0x2D8 | 0x61 SUB-IMM | 50 228 | `498b87800200004881e82802000049898780020000c3` (22B) | `f21787f68d23f722` |
| H_723 | 0x2D9 | 0x61 SUB-IMM | 51 228 | `498b87880200004881e82802000049898788020000c3` (22B) | `b4edd744e6cbfd23` |
| H_724 | 0x2DA | 0x61 SUB-IMM | 52 228 | `498b87900200004881e82802000049898790020000c3` (22B) | `a64562f9de393830` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x228 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x228 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x228 uses imm32 add (`48 81 c0`) → 26B pins.
LDB dd=51/52 ss=60 oo=228 finishes deferred 228 LDB triad (H_717/H_718; H_716=50/228 already locked).
ADD-IMM slot=50/51/52 imm=228 starts deferred 228 ADD triad (H_719/H_720/H_721).
SUB-IMM slot=50/51/52 imm=228 starts 228 SUB triad (H_722/H_723/H_724).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_717 | `ec662f4d79ff8add66c6b0606f5a408b3c485d2361c271118c0eef2d41ed60d3` |
| H_718 | `0a14cf8c7293361575919dd2df7f3ffb4bdb7fa27f1ef29919b7a2b4a3ba149e` |
| H_719 | `308359b06a3c0b715e6575564a5adf581bb2bab054dee4b61ba0b5ab4d8c52d8` |
| H_720 | `30a3548d2b182ab87c5fdd862f32dd467b0bf1c4078df243023e90e6c0c0a874` |
| H_721 | `bb5db527c469beeca3feb4b57ce15971c13ba3a1916646a503c65e9042a608ff` |
| H_722 | `f21787f68d23f722623c13531402795368893b64a61a65a658b8953e26320347` |
| H_723 | `b4edd744e6cbfd23b6f73bc312697cbee7bce3125f544060286caeeccd04cd57` |
| H_724 | `a64562f9de393830d164b6493bb727b27106b09e043cc597c4f75ff11ecababd` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5160_228`, `_scratch_ldb_5260_228`, `_scratch_addimm_h50_228`, `_scratch_addimm_h51_228`,
`_scratch_addimm_h52_228`, `_scratch_subimm_h50_228`, `_scratch_subimm_h51_228`, `_scratch_subimm_h52_228`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 2D3`.. for H_717.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-94

- LDB dd=50/51/52 ss=60 oo=230 (start 230 LDB ladder)
- ADD-IMM / SUB-IMM slot=50/51/52 imm=230 (start 230 ADD/SUB triads) if continuing ladder
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
