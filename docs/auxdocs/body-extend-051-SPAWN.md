# body-extend-051 SPAWN · consolidate parallel-batch-45

> Continuous queue handoff from parallel-batch-45 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `1566906f85667e97cb5701b0d3ba8fdd806e893b1982fa3ad11a1138efb8adfe` (abbrev `1566906f…`).
> Handlers = 340 (H_00..H_333). Last selectors: 0x14C..0x153 = H_326..H_333 (`40 14C`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-45-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-050-log.md` / `docs/auxdocs/body-extend-050-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-050 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 154`.. for H_334.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-051 (serialize + Relock)

Mirror body-extend-050 / body-extend-049 protocol:

1. Hand-author append H_334..H_341 to `yoyo/projects/yoyo.ty` at selectors `40 154` .. `40 15B` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h52_c8,addimm_h50_d0,addimm_h51_d0,addimm_h52_d0,subimm_h50_d0,subimm_h51_d0,subimm_h52_d0,ldb_5060_e0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `1566906f85667e97…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-051-log.md`.
7. Auto-spawn parallel-batch-46 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-46-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_334 | 0x154 | 0x61 SUB-IMM | 52 C8 | `498b87900200004881e8c800000049898790020000c3` (22B) | `3b32f5875666e837` |
| H_335 | 0x155 | 0x62 ADD-IMM | 50 D0 | `498b87800200004881c0d000000049898780020000c3` (22B) | `5cdff426638d0c76` |
| H_336 | 0x156 | 0x62 ADD-IMM | 51 D0 | `498b87880200004881c0d000000049898788020000c3` (22B) | `a4c8fb5e23221fc9` |
| H_337 | 0x157 | 0x62 ADD-IMM | 52 D0 | `498b87900200004881c0d000000049898790020000c3` (22B) | `d3a3f45f884525f8` |
| H_338 | 0x158 | 0x61 SUB-IMM | 50 D0 | `498b87800200004881e8d000000049898780020000c3` (22B) | `308c801c542d857b` |
| H_339 | 0x159 | 0x61 SUB-IMM | 51 D0 | `498b87880200004881e8d000000049898788020000c3` (22B) | `744b3918b3f5fe8e` |
| H_340 | 0x15A | 0x61 SUB-IMM | 52 D0 | `498b87900200004881e8d000000049898790020000c3` (22B) | `ee26c6478e1bedb5` |
| H_341 | 0x15B | 0x80 LDB | 50 60 E0 | `498b87000300004881c0e0000000480fb60049898780020000c3` (26B) | `3fcfa899104fe81a` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xD0 uses imm32 add (`48 81 c0`) → 22B pins (H_335..H_337); not imm8.
SUB-IMM imm=0xC8/0xD0 uses imm32 sub (`48 81 e8`) → 22B pins (H_334, H_338..H_340); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pin (H_341).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_334 | `3b32f5875666e8373a280b3a1286f8992ed1cb91c944ea5802da691c198b7d97` |
| H_335 | `5cdff426638d0c76397e38d765e16b075ace43f3a67fe9fd20066a191c4812a5` |
| H_336 | `a4c8fb5e23221fc91addf13af5b7196ac39981cfae5baac16dc2b9b66302dbc5` |
| H_337 | `d3a3f45f884525f8096effcf3cc1607acf843190ee2f698c79969773d1b3922e` |
| H_338 | `308c801c542d857beb0fc1ac66260fd6386d87e7e0fe477cf557953a6d2f596c` |
| H_339 | `744b3918b3f5fe8e5d42fbfcb1a7c33fca66ebfdcc44c3b056bef1a0c9561b1c` |
| H_340 | `ee26c6478e1bedb558571013d8552266352879f1d00151658d7d5f5d1aae4e79` |
| H_341 | `3fcfa899104fe81aacb435e9380b6862db53b37e9bf2bc607a9c458a64cedbd3` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h52_c8`, `_scratch_addimm_h50_d0`, `_scratch_addimm_h51_d0`, `_scratch_addimm_h52_d0`,
`_scratch_subimm_h50_d0`, `_scratch_subimm_h51_d0`, `_scratch_subimm_h52_d0`, `_scratch_ldb_5060_e0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 154`.. for H_334.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
