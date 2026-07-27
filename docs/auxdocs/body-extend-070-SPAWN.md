# body-extend-070 SPAWN · consolidate parallel-batch-64

> Continuous queue handoff from parallel-batch-64 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `f9afff3e953337091fdaa161a919f6d92488d72c1f70687907395922a811ec42` (abbrev `f9afff3e…`).
> Handlers = 491 (H_00..H_484). Last selectors: 0x1E3..0x1EA = H_477..H_484 (`40 1E3`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-64-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-069-log.md` / `docs/auxdocs/body-extend-069-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-069 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1EB`.. for H_485.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1EB`/`40 1EC` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-070 (serialize + Relock)

Mirror body-extend-069 / body-extend-068 protocol:

1. Hand-author append H_485..H_492 to `yoyo/projects/yoyo.ty` at selectors `40 1EB` .. `40 1F2` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h50_158,addimm_h51_158,addimm_h52_158,subimm_h50_158,subimm_h51_158,subimm_h52_158,ldb_5060_160,ldb_5160_160}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `f9afff3e953337091fdaa161a919f6d92488d72c1f70687907395922a811ec42`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-070-log.md`.
7. Auto-spawn parallel-batch-65 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-65-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_485 | 0x1EB | 0x62 ADD-IMM | 50 158 | `498b87800200004881c05801000049898780020000c3` (22B) | `41094166f79d1c0b` |
| H_486 | 0x1EC | 0x62 ADD-IMM | 51 158 | `498b87880200004881c05801000049898788020000c3` (22B) | `70fd4ef8381b04b2` |
| H_487 | 0x1ED | 0x62 ADD-IMM | 52 158 | `498b87900200004881c05801000049898790020000c3` (22B) | `25deea9b5b4ae288` |
| H_488 | 0x1EE | 0x61 SUB-IMM | 50 158 | `498b87800200004881e85801000049898780020000c3` (22B) | `401d7f68292fe70a` |
| H_489 | 0x1EF | 0x61 SUB-IMM | 51 158 | `498b87880200004881e85801000049898788020000c3` (22B) | `cb9589469f12483a` |
| H_490 | 0x1F0 | 0x61 SUB-IMM | 52 158 | `498b87900200004881e85801000049898790020000c3` (22B) | `8b00fcdbb741f29c` |
| H_491 | 0x1F1 | 0x80 LDB | 50 60 160 | `498b87000300004881c060010000480fb60049898780020000c3` (26B) | `0de356c3d4e6b935` |
| H_492 | 0x1F2 | 0x80 LDB | 51 60 160 | `498b87000300004881c060010000480fb60049898788020000c3` (26B) | `ca261b259166d021` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x158 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x158 uses imm32 sub (`48 81 e8`) → 22B pins (H_488..H_490); not imm8.
LDB oo=0x160 uses imm32 add (`48 81 c0`) → 26B pins (H_491..H_492); starts 160 LDB triad.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_485 | `41094166f79d1c0b5848813a8003cf059904ad414e849c3664fa66da318d783e` |
| H_486 | `70fd4ef8381b04b209647c78f71758546b4a983306f15ffb8a450ad49b27b69a` |
| H_487 | `25deea9b5b4ae288372fe2d5c07eac0af632775ae25fc314b494c04fcfbaba38` |
| H_488 | `401d7f68292fe70a4b207cbea1f37fee395f56a6a13e61f80b7b32c6aee335a2` |
| H_489 | `cb9589469f12483a3e83ace6902793c80f1567f10505e238d9e4d03f358b7668` |
| H_490 | `8b00fcdbb741f29c732e134cf5d642097ffe98735e35a3508cf5fb4f061c992e` |
| H_491 | `0de356c3d4e6b9355d7173a2506214efb8e07910d2f0574cf4601ed1e1385ed9` |
| H_492 | `ca261b259166d0210ae6626da2f3dcb22b089cec2d01fd4c72cac9db7233b9ef` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h50_158`, `_scratch_addimm_h51_158`, `_scratch_addimm_h52_158`, `_scratch_subimm_h50_158`,
`_scratch_subimm_h51_158`, `_scratch_subimm_h52_158`, `_scratch_ldb_5060_160`, `_scratch_ldb_5160_160`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1EB`.. for H_485.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
