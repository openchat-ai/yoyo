# body-extend-045 SPAWN · consolidate parallel-batch-39

> Continuous queue handoff from parallel-batch-39 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `3514c8c6558f7028fdc93ea64a26dc007fe2df25592035494342ab5fbe6e102c` (abbrev `3514c8c6…`).
> Handlers = 292 (H_00..H_285). Last selectors: 0x11C..0x123 = H_278..H_285 (`40 11C`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-39-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-044-log.md` / `docs/auxdocs/body-extend-044-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-044 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 124`.. for H_286.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-045 (serialize + Relock)

Mirror body-extend-044 / body-extend-043 protocol:

1. Hand-author append H_286..H_293 to `yoyo/projects/yoyo.ty` at selectors `40 124` .. `40 12B` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5260_b8,addimm_h50_a0,addimm_h51_a0,addimm_h52_a0,subimm_h50_a0,subimm_h51_a0,subimm_h52_a0,ldb_5060_c0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `3514c8c6558f7028…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-045-log.md`.
7. Auto-spawn parallel-batch-40 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-40-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_286 | 0x124 | 0x80 LDB | 52 60 B8 | `498b87000300004881c0b8000000480fb60049898790020000c3` (26B) | `1f2f5d3657c8a950` |
| H_287 | 0x125 | 0x62 ADD-IMM | 50 A0 | `498b87800200004881c0a000000049898780020000c3` (22B) | `c1ce6933aae1f9f6` |
| H_288 | 0x126 | 0x62 ADD-IMM | 51 A0 | `498b87880200004881c0a000000049898788020000c3` (22B) | `7ee6f52e149ddaf7` |
| H_289 | 0x127 | 0x62 ADD-IMM | 52 A0 | `498b87900200004881c0a000000049898790020000c3` (22B) | `21fbb86c3234cc5d` |
| H_290 | 0x128 | 0x61 SUB-IMM | 50 A0 | `498b87800200004881e8a000000049898780020000c3` (22B) | `1588c7457cf93fd9` |
| H_291 | 0x129 | 0x61 SUB-IMM | 51 A0 | `498b87880200004881e8a000000049898788020000c3` (22B) | `8aca9b975c5fdce4` |
| H_292 | 0x12A | 0x61 SUB-IMM | 52 A0 | `498b87900200004881e8a000000049898790020000c3` (22B) | `fc8ca4c4c8e50fd5` |
| H_293 | 0x12B | 0x80 LDB | 50 60 C0 | `498b87000300004881c0c0000000480fb60049898780020000c3` (26B) | `cf7c2bda3d5ae346` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xA0 uses imm32 add (`48 81 c0`) → 22B pins (H_287..H_289); not imm8.
SUB-IMM imm=0xA0 uses imm32 sub (`48 81 e8`) → 22B pins (H_290..H_292); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_286, H_293).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_286 | `1f2f5d3657c8a950cafb8678cacf0656ecf6fe49ea44d6ecf5b6a724e9ee20a4` |
| H_287 | `c1ce6933aae1f9f6b6fcd7793ad38e829661827aa72d0ec9ea01953f79037ab1` |
| H_288 | `7ee6f52e149ddaf76175450068bc46096b132caa063ba44fd8c32c8d133f6646` |
| H_289 | `21fbb86c3234cc5d1e9d789d63bcca613d6ec3e4d135415b8b71aed335262a2d` |
| H_290 | `1588c7457cf93fd9bb89205aebd40431dfb361951ee4a07628822ce44c97474b` |
| H_291 | `8aca9b975c5fdce4f5005adc81f373a3236dd512279793a7be1877b4b17c188a` |
| H_292 | `fc8ca4c4c8e50fd520c005acecde274357878f918d629c21fe10b2f5ef7b3ef0` |
| H_293 | `cf7c2bda3d5ae346f027aa17e3271265c1075a3675686ed8a859ca238d4e8356` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5260_b8`, `_scratch_addimm_h50_a0`, `_scratch_addimm_h51_a0`, `_scratch_addimm_h52_a0`,
`_scratch_subimm_h50_a0`, `_scratch_subimm_h51_a0`, `_scratch_subimm_h52_a0`, `_scratch_ldb_5060_c0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 124`.. for H_286.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
