# body-extend-048 SPAWN · consolidate parallel-batch-42

> Continuous queue handoff from parallel-batch-42 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `000042c8ea316c07fce78e5bb05814229058adea09ac196d0d1e8a90987336f2` (abbrev `000042c8…`).
> Handlers = 316 (H_00..H_309). Last selectors: 0x134..0x13B = H_302..H_309 (`40 134`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-42-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-047-log.md` / `docs/auxdocs/body-extend-047-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-047 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 13C`.. for H_310.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-048 (serialize + Relock)

Mirror body-extend-047 / body-extend-046 protocol:

1. Hand-author append H_310..H_317 to `yoyo/projects/yoyo.ty` at selectors `40 13C` .. `40 143` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h52_b0,addimm_h50_b8,addimm_h51_b8,addimm_h52_b8,subimm_h50_b8,subimm_h51_b8,subimm_h52_b8,ldb_5060_d0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `000042c8ea316c07…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-048-log.md`.
7. Auto-spawn parallel-batch-43 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-43-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_310 | 0x13C | 0x61 SUB-IMM | 52 B0 | `498b87900200004881e8b000000049898790020000c3` (22B) | `1eabf19e87df5652` |
| H_311 | 0x13D | 0x62 ADD-IMM | 50 B8 | `498b87800200004881c0b800000049898780020000c3` (22B) | `9f7f7147fbb9f533` |
| H_312 | 0x13E | 0x62 ADD-IMM | 51 B8 | `498b87880200004881c0b800000049898788020000c3` (22B) | `3817887afb58b853` |
| H_313 | 0x13F | 0x62 ADD-IMM | 52 B8 | `498b87900200004881c0b800000049898790020000c3` (22B) | `65f24a01717f98f9` |
| H_314 | 0x140 | 0x61 SUB-IMM | 50 B8 | `498b87800200004881e8b800000049898780020000c3` (22B) | `a086a4139a5285c0` |
| H_315 | 0x141 | 0x61 SUB-IMM | 51 B8 | `498b87880200004881e8b800000049898788020000c3` (22B) | `d8eeef300a793b35` |
| H_316 | 0x142 | 0x61 SUB-IMM | 52 B8 | `498b87900200004881e8b800000049898790020000c3` (22B) | `3aecc01b59d73b5a` |
| H_317 | 0x143 | 0x80 LDB | 50 60 D0 | `498b87000300004881c0d0000000480fb60049898780020000c3` (26B) | `e88fcc130f63d22f` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xB8 uses imm32 add (`48 81 c0`) → 22B pins (H_311..H_313); not imm8.
SUB-IMM imm=0xB0/0xB8 uses imm32 sub (`48 81 e8`) → 22B pins (H_310, H_314..H_316); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pin (H_317).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_310 | `1eabf19e87df565236fea87b7386ebdf32057acb914bf71a1d91dbcad74bc800` |
| H_311 | `9f7f7147fbb9f5333359d1ab52c82b5025716c11226323db3319ca238a908555` |
| H_312 | `3817887afb58b853de9a43c9b984b204ca5ed7b218ca7cbf84a07c931f2e4c28` |
| H_313 | `65f24a01717f98f95abef0f986cdff72cc6c7b38d8a33f9f77189bfe6015863b` |
| H_314 | `a086a4139a5285c04e909715cc2de8b9e4dd5557810f8837b874506f66ed8c08` |
| H_315 | `d8eeef300a793b3563d7438618b19db38b99437e49d9efbbc9ac610ccb9b8e97` |
| H_316 | `3aecc01b59d73b5a6ab5b6143fd0773a97de0f4c10426684fb5786a6af8eeb55` |
| H_317 | `e88fcc130f63d22f79242d93e14170cd74259ced3b0711a0b2d6043d01943e49` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h52_b0`, `_scratch_addimm_h50_b8`, `_scratch_addimm_h51_b8`, `_scratch_addimm_h52_b8`,
`_scratch_subimm_h50_b8`, `_scratch_subimm_h51_b8`, `_scratch_subimm_h52_b8`, `_scratch_ldb_5060_d0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 13C`.. for H_310.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
