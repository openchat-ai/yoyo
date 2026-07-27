# body-extend-063 SPAWN · consolidate parallel-batch-57

> Continuous queue handoff from parallel-batch-57 (scratch-only complete: 7 PASS / 0 REJECT).
> Current pin: `c5b95f3792afa572a774aa41d22dd49fb27b6905aa7ab891273b77db49a3af0a` (abbrev `c5b95f37…`).
> Handlers = 436 (H_00..H_429). Last selectors: 0x1AC..0x1B3 = H_422..H_429 (`40 1AC`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-57-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-062-log.md` / `docs/auxdocs/body-extend-062-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-062 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1B4`.. for H_430.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1B4`/`40 1B5` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-063 (serialize + Relock)

Mirror body-extend-062 / body-extend-061 protocol:

1. Hand-author append H_430..H_436 to `yoyo/projects/yoyo.ty` at selectors `40 1B4` .. `40 1BA` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5260_128,addimm_h50_128,addimm_h51_128,addimm_h52_128,subimm_h50_128,subimm_h51_128,subimm_h52_128}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+7 JS/Rust counts).
4. Verify + Relock once chaining from `c5b95f3792afa572a774aa41d22dd49fb27b6905aa7ab891273b77db49a3af0a`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-063-log.md`.
7. Auto-spawn parallel-batch-58 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-58-SPAWN.md` if no Task tool.

### PASS picks (ALL 7)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_430 | 0x1B4 | 0x80 LDB | 52 60 128 | `498b87000300004881c028010000480fb60049898790020000c3` (26B) | `6468bf9a05c742b4` |
| H_431 | 0x1B5 | 0x62 ADD-IMM | 50 128 | `498b87800200004881c02801000049898780020000c3` (22B) | `e8b7b5eb74790fbc` |
| H_432 | 0x1B6 | 0x62 ADD-IMM | 51 128 | `498b87880200004881c02801000049898788020000c3` (22B) | `f19522688ae984fb` |
| H_433 | 0x1B7 | 0x62 ADD-IMM | 52 128 | `498b87900200004881c02801000049898790020000c3` (22B) | `ba685e27eb2e7e2b` |
| H_434 | 0x1B8 | 0x61 SUB-IMM | 50 128 | `498b87800200004881e82801000049898780020000c3` (22B) | `d2dc131f67b41898` |
| H_435 | 0x1B9 | 0x61 SUB-IMM | 51 128 | `498b87880200004881e82801000049898788020000c3` (22B) | `0327f33cd15c5c5a` |
| H_436 | 0x1BA | 0x61 SUB-IMM | 52 128 | `498b87900200004881e82801000049898790020000c3` (22B) | `d6207001a19bc3e5` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 7 pins above.
ADD-IMM imm=0x128 uses imm32 add (`48 81 c0`) → 22B pins (H_431..H_433); not imm8.
SUB-IMM imm=0x128 uses imm32 sub (`48 81 e8`) → 22B pins (H_434..H_436); not imm8.
LDB oo=0x128 uses imm32 add (`48 81 c0`) → 26B pin (H_430); finishes 128 LDB triad after H_428/H_429.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_430 | `6468bf9a05c742b4a8324439fa582bdf0a572b156108c02fc99ac75c43db4fe9` |
| H_431 | `e8b7b5eb74790fbc210574195020a357d9c93c4a87b6e703f455cef5a0dc024a` |
| H_432 | `f19522688ae984fb32b06787291b6a469efc321a36b92e121d746e3930b5dc6d` |
| H_433 | `ba685e27eb2e7e2b366d311ebd56e65f2183ce16f8ffae11da8a5037f28f6ea9` |
| H_434 | `d2dc131f67b41898d008cc80a3291d284f2332039159892e5a4724b51a88c01f` |
| H_435 | `0327f33cd15c5c5afa65b2e3504e6f017f2ceb1788582bdccb2c0f9328a8ac3f` |
| H_436 | `d6207001a19bc3e5dc2b8e6f1830add95a4cf18fa617daf57e799eafb35db756` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5260_128`, `_scratch_addimm_h50_128`, `_scratch_addimm_h51_128`, `_scratch_addimm_h52_128`,
`_scratch_subimm_h50_128`, `_scratch_subimm_h51_128`, `_scratch_subimm_h52_128`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1B4`.. for H_430.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
