# body-extend-054 SPAWN · consolidate parallel-batch-48

> Continuous queue handoff from parallel-batch-48 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `86485f4822e891c4f11dbc5f181c43dc3f23d7ed779b61831f2426f2329e504d` (abbrev `86485f48…`).
> Handlers = 364 (H_00..H_357). Last selectors: 0x164..0x16B = H_350..H_357 (`40 164`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-48-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-053-log.md` / `docs/auxdocs/body-extend-053-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-053 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 16C`.. for H_358.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-054 (serialize + Relock)

Mirror body-extend-053 / body-extend-052 protocol:

1. Hand-author append H_358..H_365 to `yoyo/projects/yoyo.ty` at selectors `40 16C` .. `40 173` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h52_e0,addimm_h50_e8,addimm_h51_e8,addimm_h52_e8,subimm_h50_e8,subimm_h51_e8,subimm_h52_e8,ldb_5060_f0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `86485f4822e891c4…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-054-log.md`.
7. Auto-spawn parallel-batch-49 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-49-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_358 | 0x16C | 0x61 SUB-IMM | 52 E0 | `498b87900200004881e8e000000049898790020000c3` (22B) | `7986c4bc9ebed8c6` |
| H_359 | 0x16D | 0x62 ADD-IMM | 50 E8 | `498b87800200004881c0e800000049898780020000c3` (22B) | `51760cec223058e1` |
| H_360 | 0x16E | 0x62 ADD-IMM | 51 E8 | `498b87880200004881c0e800000049898788020000c3` (22B) | `75755148da277056` |
| H_361 | 0x16F | 0x62 ADD-IMM | 52 E8 | `498b87900200004881c0e800000049898790020000c3` (22B) | `e8d397ad24fcfa8c` |
| H_362 | 0x170 | 0x61 SUB-IMM | 50 E8 | `498b87800200004881e8e800000049898780020000c3` (22B) | `45dace9bedbf51e3` |
| H_363 | 0x171 | 0x61 SUB-IMM | 51 E8 | `498b87880200004881e8e800000049898788020000c3` (22B) | `ce05fadbd17ed30f` |
| H_364 | 0x172 | 0x61 SUB-IMM | 52 E8 | `498b87900200004881e8e800000049898790020000c3` (22B) | `87083a564ea9a2de` |
| H_365 | 0x173 | 0x80 LDB | 50 60 F0 | `498b87000300004881c0f0000000480fb60049898780020000c3` (26B) | `a8241e1de5be2a76` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xE8 uses imm32 add (`48 81 c0`) → 22B pins (H_359..H_361); not imm8.
SUB-IMM imm=0xE0/E8 uses imm32 sub (`48 81 e8`) → 22B pins (H_358, H_362..H_364); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pin (H_365).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_358 | `7986c4bc9ebed8c615b5512585c1c715369f0e06406186b9e26de02550c5087b` |
| H_359 | `51760cec223058e1e2efa167c7c6e97cd9c880e38a09616a625aed8010d6bd7a` |
| H_360 | `75755148da27705618ea6e348d9f98e107a5c70c9be4ab5442b4275e80d059cb` |
| H_361 | `e8d397ad24fcfa8c05d2981b1e71db11a7183c30e2b30f49a39c20ed51c6565e` |
| H_362 | `45dace9bedbf51e3a8f01a2676a43b1ebbd7c807513786231cb31af596d29c12` |
| H_363 | `ce05fadbd17ed30f59808e6379fb6e21f94ab2cff74e67c0b9ae7ed5914c4a98` |
| H_364 | `87083a564ea9a2ded7da403c8650e858a8971f03af2bf780607b959c6faef966` |
| H_365 | `a8241e1de5be2a76725f8966555ade3cbf05387853d23d3b4a5d3f5dd601caf2` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h52_e0`, `_scratch_addimm_h50_e8`, `_scratch_addimm_h51_e8`, `_scratch_addimm_h52_e8`,
`_scratch_subimm_h50_e8`, `_scratch_subimm_h51_e8`, `_scratch_subimm_h52_e8`, `_scratch_ldb_5060_f0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 16C`.. for H_358.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
