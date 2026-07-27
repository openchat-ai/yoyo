# body-extend-043 SPAWN · consolidate parallel-batch-37

> Continuous queue handoff from parallel-batch-37 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `afceb388015dd4a7e7a2de16a109eb8649189bb28471d021bb4b82eeaa9d1311` (abbrev `afceb388…`).
> Handlers = 276 (H_00..H_269). Last selectors: 0x10C..0x113 = H_262..H_269 (`40 10C`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-37-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-042-log.md` / `docs/auxdocs/body-extend-042-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-042 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 114`.. for H_270.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-043 (serialize + Relock)

Mirror body-extend-042 / body-extend-041 protocol:

1. Hand-author append H_270..H_277 to `yoyo/projects/yoyo.ty` at selectors `40 114` .. `40 11B` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h51_90,addimm_h52_90,subimm_h50_90,subimm_h51_90,subimm_h52_90,ldb_5060_b0,ldb_5160_b0,ldb_5260_b0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `afceb388015dd4a7…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-043-log.md`.
7. Auto-spawn parallel-batch-38 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-38-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_270 | 0x114 | 0x62 ADD-IMM | 51 90 | `498b87880200004881c09000000049898788020000c3` (22B) | `30d80ac5f98d5b91` |
| H_271 | 0x115 | 0x62 ADD-IMM | 52 90 | `498b87900200004881c09000000049898790020000c3` (22B) | `1f4ed4e242ed21b3` |
| H_272 | 0x116 | 0x61 SUB-IMM | 50 90 | `498b87800200004881e89000000049898780020000c3` (22B) | `5108f62107ced6f5` |
| H_273 | 0x117 | 0x61 SUB-IMM | 51 90 | `498b87880200004881e89000000049898788020000c3` (22B) | `07c48bf0e15bc2fd` |
| H_274 | 0x118 | 0x61 SUB-IMM | 52 90 | `498b87900200004881e89000000049898790020000c3` (22B) | `ce43fa09ae8fd687` |
| H_275 | 0x119 | 0x80 LDB | 50 60 B0 | `498b87000300004881c0b0000000480fb60049898780020000c3` (26B) | `64f22f32cf0fab77` |
| H_276 | 0x11A | 0x80 LDB | 51 60 B0 | `498b87000300004881c0b0000000480fb60049898788020000c3` (26B) | `8de79951c51e9c4a` |
| H_277 | 0x11B | 0x80 LDB | 52 60 B0 | `498b87000300004881c0b0000000480fb60049898790020000c3` (26B) | `24662dc0540eff95` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x90 uses imm32 add (`48 81 c0`) → 22B pins (H_270..H_271); not imm8.
SUB-IMM imm=0x90 uses imm32 sub (`48 81 e8`) → 22B pins (H_272..H_274); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_275..H_277).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_270 | `30d80ac5f98d5b91b3cdc3f176a37ee810973825a55af337dcae8a95254b1023` |
| H_271 | `1f4ed4e242ed21b3ca4c1cb13bf3cb58db65ef6da56e6e50fd47e52edbcc4953` |
| H_272 | `5108f62107ced6f51fcd11dbc43815601e1f26890847c3cfc43a79f180eeb873` |
| H_273 | `07c48bf0e15bc2fd1bb1150b111268d555cca8eec7ed5c71c7bcfcb8df2c5309` |
| H_274 | `ce43fa09ae8fd687347e34b8f70dd0aeeaf4e3d7be6046e6dc282624d7daec66` |
| H_275 | `64f22f32cf0fab77e475d17ef59518340cf5bdf1b03f84eda58bc971fe8ec9d8` |
| H_276 | `8de79951c51e9c4af62035f837dbbc6f6f640d0b29ed526b4597d16a4fb58cd8` |
| H_277 | `24662dc0540eff95f240064e796f6ea1a35fbd9814241e9f76b3f7c7780a1384` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h51_90`, `_scratch_addimm_h52_90`, `_scratch_subimm_h50_90`, `_scratch_subimm_h51_90`,
`_scratch_subimm_h52_90`, `_scratch_ldb_5060_b0`, `_scratch_ldb_5160_b0`, `_scratch_ldb_5260_b0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 114`.. for H_270.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
