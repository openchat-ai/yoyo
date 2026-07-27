# body-extend-098 SPAWN · consolidate parallel-batch-92

> Continuous queue handoff from parallel-batch-92 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `e6ba7d6cfcbb11da0a3a63dab93cde597a265934cf95064968d97697c85cd68a` (abbrev `e6ba7d6c…`).
> Handlers = 715 (H_00..H_708). Last selectors: 0x2C3..0x2CA = H_701..H_708 (`40 2C3`..`40 2CA` via label-width A).
> Source: `docs/auxdocs/parallel-batch-92-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-097-log.md` / `docs/auxdocs/body-extend-097-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-097 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 2CB`.. for H_709.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 2CB`/`40 2D2` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-098 (serialize + Relock)

Mirror body-extend-097 / body-extend-096 protocol:

1. Hand-author append H_709..H_716 to `yoyo/projects/yoyo.ty` at selectors `40 2CB` .. `40 2D2` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5260_220,addimm_h50_220,addimm_h51_220,addimm_h52_220,subimm_h50_220,subimm_h51_220,subimm_h52_220,ldb_5060_228}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `e6ba7d6cfcbb11da0a3a63dab93cde597a265934cf95064968d97697c85cd68a`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-098-log.md`.
7. Auto-spawn parallel-batch-93 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-93-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_709 | 0x2CB | 0x80 LDB | 52 60 220 | `498b87000300004881c020020000480fb60049898790020000c3` (26B) | `3fc747bcdb5a7814` |
| H_710 | 0x2CC | 0x62 ADD-IMM | 50 220 | `498b87800200004881c02002000049898780020000c3` (22B) | `1bbf4fad113bcab7` |
| H_711 | 0x2CD | 0x62 ADD-IMM | 51 220 | `498b87880200004881c02002000049898788020000c3` (22B) | `8504700ade40627c` |
| H_712 | 0x2CE | 0x62 ADD-IMM | 52 220 | `498b87900200004881c02002000049898790020000c3` (22B) | `c0a102f97c62576f` |
| H_713 | 0x2CF | 0x61 SUB-IMM | 50 220 | `498b87800200004881e82002000049898780020000c3` (22B) | `3a44dbe899e12859` |
| H_714 | 0x2D0 | 0x61 SUB-IMM | 51 220 | `498b87880200004881e82002000049898788020000c3` (22B) | `740509fefa4bff85` |
| H_715 | 0x2D1 | 0x61 SUB-IMM | 52 220 | `498b87900200004881e82002000049898790020000c3` (22B) | `2ae2a9625cac581c` |
| H_716 | 0x2D2 | 0x80 LDB | 50 60 228 | `498b87000300004881c028020000480fb60049898780020000c3` (26B) | `9e1963a796211cc1` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x220 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x220 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x220/0x228 uses imm32 add (`48 81 c0`) → 26B pins.
LDB dd=52 ss=60 oo=220 finishes deferred 220 LDB triad (H_709).
ADD-IMM slot=50/51/52 imm=220 starts deferred 220 ADD triad (H_710/H_711/H_712).
SUB-IMM slot=50/51/52 imm=220 starts 220 SUB triad (H_713/H_714/H_715).
LDB dd=50 ss=60 oo=228 starts 228 LDB ladder (H_716; LDB 51/52 228 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_709 | `3fc747bcdb5a781461fd0348cdb022ece3e3a9661b215e684e0233690b5f8f8d` |
| H_710 | `1bbf4fad113bcab70b19bba185dc4e67008fe55aa83c534a13df23ecf474b482` |
| H_711 | `8504700ade40627c64cde82afa2ac6385f4194d5798cf34258012669602545fb` |
| H_712 | `c0a102f97c62576f4163546150ecd9d07a1dc8ee2c781e1a0cae0c70957e47cb` |
| H_713 | `3a44dbe899e12859ab6ac9679f62b181a83f1321ea5328d991efa218669f239e` |
| H_714 | `740509fefa4bff8502ceff79f94cfdfc9a24681c1941768c4663818ba95c0279` |
| H_715 | `2ae2a9625cac581c393c14716bf4dcbd8f576e8a0c12c7d6b56671ecc70917bb` |
| H_716 | `9e1963a796211cc173505fa2bd3c4864753788fffb324d73944ead0e6682c2f2` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5260_220`, `_scratch_addimm_h50_220`, `_scratch_addimm_h51_220`, `_scratch_addimm_h52_220`,
`_scratch_subimm_h50_220`, `_scratch_subimm_h51_220`, `_scratch_subimm_h52_220`, `_scratch_ldb_5060_228`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 2CB`.. for H_709.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-93

- LDB dd=51/52 ss=60 oo=228 (finish 228 LDB triad)
- ADD-IMM / SUB-IMM slot=50/51/52 imm=228 (start 228 ADD/SUB triads) if continuing ladder
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
