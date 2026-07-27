# body-extend-050 SPAWN · consolidate parallel-batch-44

> Continuous queue handoff from parallel-batch-44 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `69adc5a0b11c8f176687deff6753b2fa51b6611c3cd1193c79bf1143b7b4c957` (abbrev `69adc5a0…`).
> Handlers = 332 (H_00..H_325). Last selectors: 0x144..0x14B = H_318..H_325 (`40 144`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-44-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-049-log.md` / `docs/auxdocs/body-extend-049-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-049 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 14C`.. for H_326.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-050 (serialize + Relock)

Mirror body-extend-049 / body-extend-048 protocol:

1. Hand-author append H_326..H_333 to `yoyo/projects/yoyo.ty` at selectors `40 14C` .. `40 153` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5060_d8,ldb_5160_d8,ldb_5260_d8,addimm_h50_c8,addimm_h51_c8,addimm_h52_c8,subimm_h50_c8,subimm_h51_c8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `69adc5a0b11c8f17…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-050-log.md`.
7. Auto-spawn parallel-batch-45 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-45-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_326 | 0x14C | 0x80 LDB | 50 60 D8 | `498b87000300004881c0d8000000480fb60049898780020000c3` (26B) | `661c8bfff21fc20e` |
| H_327 | 0x14D | 0x80 LDB | 51 60 D8 | `498b87000300004881c0d8000000480fb60049898788020000c3` (26B) | `d9fa04f9279ab0fe` |
| H_328 | 0x14E | 0x80 LDB | 52 60 D8 | `498b87000300004881c0d8000000480fb60049898790020000c3` (26B) | `f155284380f7580d` |
| H_329 | 0x14F | 0x62 ADD-IMM | 50 C8 | `498b87800200004881c0c800000049898780020000c3` (22B) | `1ecdb5e66e168372` |
| H_330 | 0x150 | 0x62 ADD-IMM | 51 C8 | `498b87880200004881c0c800000049898788020000c3` (22B) | `5705b35865532f87` |
| H_331 | 0x151 | 0x62 ADD-IMM | 52 C8 | `498b87900200004881c0c800000049898790020000c3` (22B) | `863fee834853a91a` |
| H_332 | 0x152 | 0x61 SUB-IMM | 50 C8 | `498b87800200004881e8c800000049898780020000c3` (22B) | `521f857a16de934d` |
| H_333 | 0x153 | 0x61 SUB-IMM | 51 C8 | `498b87880200004881e8c800000049898788020000c3` (22B) | `5692683211522a54` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xC8 uses imm32 add (`48 81 c0`) → 22B pins (H_329..H_331); not imm8.
SUB-IMM imm=0xC8 uses imm32 sub (`48 81 e8`) → 22B pins (H_332..H_333); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_326..H_328).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_326 | `661c8bfff21fc20e65b1aea5e299c8711bc60815786c103043ece69116e2a489` |
| H_327 | `d9fa04f9279ab0fef7b3d1e724fa58465b4dd12191250cd94d62fbda95e6f474` |
| H_328 | `f155284380f7580d079e6c9083b7632ca21cacc683f54d34a880d8cec37fcac8` |
| H_329 | `1ecdb5e66e168372702777d893dc24e666e1f5d661e49821a72554eacdfda622` |
| H_330 | `5705b35865532f87af5696d7c926a6137466e3c1b6650c47b9e7536ff32246c6` |
| H_331 | `863fee834853a91a3f217416203559e58d702249de7c79ef0158dada08946e66` |
| H_332 | `521f857a16de934dae3c5f6327db923510cb13422d3d128afc6d969e1bbdec3b` |
| H_333 | `5692683211522a5477ebecb7908ae44c7593f28528208c28e4db791240905d73` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5060_d8`, `_scratch_ldb_5160_d8`, `_scratch_ldb_5260_d8`, `_scratch_addimm_h50_c8`,
`_scratch_addimm_h51_c8`, `_scratch_addimm_h52_c8`, `_scratch_subimm_h50_c8`, `_scratch_subimm_h51_c8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 14C`.. for H_326.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
