# body-extend-066 SPAWN · consolidate parallel-batch-60

> Continuous queue handoff from parallel-batch-60 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `b84d7f1b4bb1d8eefeca1832f12c3f7380658897813b1a321f2b75b27187258e` (abbrev `b84d7f1b…`).
> Handlers = 459 (H_00..H_452). Last selectors: 0x1C3..0x1CA = H_445..H_452 (`40 1C3`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-60-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-065-log.md` / `docs/auxdocs/body-extend-065-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-065 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1CB`.. for H_453.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1CB`/`40 1CC` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-066 (serialize + Relock)

Mirror body-extend-065 / body-extend-064 protocol:

1. Hand-author append H_453..H_460 to `yoyo/projects/yoyo.ty` at selectors `40 1CB` .. `40 1D2` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h51_138,subimm_h52_138,ldb_5060_140,ldb_5160_140,ldb_5260_140,addimm_h50_140,addimm_h51_140,addimm_h52_140}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `b84d7f1b4bb1d8eefeca1832f12c3f7380658897813b1a321f2b75b27187258e`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-066-log.md`.
7. Auto-spawn parallel-batch-61 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-61-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_453 | 0x1CB | 0x61 SUB-IMM | 51 138 | `498b87880200004881e83801000049898788020000c3` (22B) | `23ad7ac033aa9ec5` |
| H_454 | 0x1CC | 0x61 SUB-IMM | 52 138 | `498b87900200004881e83801000049898790020000c3` (22B) | `5e85ef9f64f70096` |
| H_455 | 0x1CD | 0x80 LDB | 50 60 140 | `498b87000300004881c040010000480fb60049898780020000c3` (26B) | `7b8558d3978f497d` |
| H_456 | 0x1CE | 0x80 LDB | 51 60 140 | `498b87000300004881c040010000480fb60049898788020000c3` (26B) | `cf076e94edbe5a82` |
| H_457 | 0x1CF | 0x80 LDB | 52 60 140 | `498b87000300004881c040010000480fb60049898790020000c3` (26B) | `4468abc2e0b7e44b` |
| H_458 | 0x1D0 | 0x62 ADD-IMM | 50 140 | `498b87800200004881c04001000049898780020000c3` (22B) | `f60b265b7a3dc3f9` |
| H_459 | 0x1D1 | 0x62 ADD-IMM | 51 140 | `498b87880200004881c04001000049898788020000c3` (22B) | `fe39737bd6fc8a3f` |
| H_460 | 0x1D2 | 0x62 ADD-IMM | 52 140 | `498b87900200004881c04001000049898790020000c3` (22B) | `dfda4be88622d37d` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x140 uses imm32 add (`48 81 c0`) → 22B pins (H_458..H_460); not imm8.
SUB-IMM imm=0x138 uses imm32 sub (`48 81 e8`) → 22B pins (H_453..H_454); not imm8.
LDB oo=0x140 uses imm32 add (`48 81 c0`) → 26B pins (H_455..H_457); starts 140 LDB triad.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_453 | `23ad7ac033aa9ec586f70f25b2715c58f15cbb38616f9083366a6130538c1180` |
| H_454 | `5e85ef9f64f7009681de2d56b231c4d942d0e12e8c74148d68d6120478fd4c0c` |
| H_455 | `7b8558d3978f497d9cb3923014e788f6dba285aaa36193f7e792f41060ebeb4d` |
| H_456 | `cf076e94edbe5a823d5a090afc8f503a205803bd9a25b527673bb31a3d512bd9` |
| H_457 | `4468abc2e0b7e44b42eb9d8dd9905d43c2d61425eb73ee496fc5b26da7370597` |
| H_458 | `f60b265b7a3dc3f99919c45e013d87e9907eef3b224cfd06f7c7039bc09ae922` |
| H_459 | `fe39737bd6fc8a3fdf55010dd7f7238946ff5917d3193e5b4084b4e9d72c147b` |
| H_460 | `dfda4be88622d37dece8c6497433408ae45eb9577e73a04c57d085b90fce066d` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h51_138`, `_scratch_subimm_h52_138`, `_scratch_ldb_5060_140`, `_scratch_ldb_5160_140`,
`_scratch_ldb_5260_140`, `_scratch_addimm_h50_140`, `_scratch_addimm_h51_140`, `_scratch_addimm_h52_140`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1CB`.. for H_453.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
