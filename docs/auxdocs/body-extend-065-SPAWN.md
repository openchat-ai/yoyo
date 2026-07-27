# body-extend-065 SPAWN · consolidate parallel-batch-59

> Continuous queue handoff from parallel-batch-59 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `d9aff9ed76e4f649fcee1c50496dd813e23690f73b35ce4cfc4e700ef466f276` (abbrev `d9aff9ed…`).
> Handlers = 451 (H_00..H_444). Last selectors: 0x1BB..0x1C2 = H_437..H_444 (`40 1BB`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-59-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-064-log.md` / `docs/auxdocs/body-extend-064-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-064 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1C3`.. for H_445.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1C3`/`40 1C4` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-065 (serialize + Relock)

Mirror body-extend-064 / body-extend-063 protocol:

1. Hand-author append H_445..H_452 to `yoyo/projects/yoyo.ty` at selectors `40 1C3` .. `40 1CA` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h52_130,ldb_5060_138,ldb_5160_138,ldb_5260_138,addimm_h50_138,addimm_h51_138,addimm_h52_138,subimm_h50_138}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `d9aff9ed76e4f649fcee1c50496dd813e23690f73b35ce4cfc4e700ef466f276`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-065-log.md`.
7. Auto-spawn parallel-batch-60 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-60-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_445 | 0x1C3 | 0x61 SUB-IMM | 52 130 | `498b87900200004881e83001000049898790020000c3` (22B) | `e775907813eab73c` |
| H_446 | 0x1C4 | 0x80 LDB | 50 60 138 | `498b87000300004881c038010000480fb60049898780020000c3` (26B) | `465ea202edfa6b33` |
| H_447 | 0x1C5 | 0x80 LDB | 51 60 138 | `498b87000300004881c038010000480fb60049898788020000c3` (26B) | `a55ee627c7c07cff` |
| H_448 | 0x1C6 | 0x80 LDB | 52 60 138 | `498b87000300004881c038010000480fb60049898790020000c3` (26B) | `2ac95e896392b10a` |
| H_449 | 0x1C7 | 0x62 ADD-IMM | 50 138 | `498b87800200004881c03801000049898780020000c3` (22B) | `3d9af9767bc85f81` |
| H_450 | 0x1C8 | 0x62 ADD-IMM | 51 138 | `498b87880200004881c03801000049898788020000c3` (22B) | `24769795853dcd61` |
| H_451 | 0x1C9 | 0x62 ADD-IMM | 52 138 | `498b87900200004881c03801000049898790020000c3` (22B) | `9e5da1e81f6ac0a0` |
| H_452 | 0x1CA | 0x61 SUB-IMM | 50 138 | `498b87800200004881e83801000049898780020000c3` (22B) | `44bbe548c077e01f` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x138 uses imm32 add (`48 81 c0`) → 22B pins (H_449..H_451); not imm8.
SUB-IMM imm=0x130/0x138 uses imm32 sub (`48 81 e8`) → 22B pins (H_445, H_452); not imm8.
LDB oo=0x138 uses imm32 add (`48 81 c0`) → 26B pins (H_446..H_448); starts 138 LDB triad.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_445 | `e775907813eab73ceb228ccbcf3865e86d16a6ee7e14cf83cd429dc711aeaac7` |
| H_446 | `465ea202edfa6b33925f864559b49e847c0ccee62248823cdc8060d0aa52f9de` |
| H_447 | `a55ee627c7c07cff978d793cff102a507a7285c341886e85b660a2069c4f0067` |
| H_448 | `2ac95e896392b10a07a20427da5b2852776a6dbc560534e7a356ea8a3d146f26` |
| H_449 | `3d9af9767bc85f81204bd5ad464b1dce044bc27b8fa8db58b0774cb7b38e4f35` |
| H_450 | `24769795853dcd61bed79a7cf0fd8f5b7935eff6b2854eec226512a7a4f462c4` |
| H_451 | `9e5da1e81f6ac0a07aa5e7a456be4597d0c2c85da4463cb2387240a4bd3a2f4d` |
| H_452 | `44bbe548c077e01fd77040ae745ba6d2c2d7a4edd5839bd382b5b6bb5b3c7113` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h52_130`, `_scratch_ldb_5060_138`, `_scratch_ldb_5160_138`, `_scratch_ldb_5260_138`,
`_scratch_addimm_h50_138`, `_scratch_addimm_h51_138`, `_scratch_addimm_h52_138`, `_scratch_subimm_h50_138`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1C3`.. for H_445.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
