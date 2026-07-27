# body-extend-059 SPAWN · consolidate parallel-batch-53

> Continuous queue handoff from parallel-batch-53 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `c258ff3271396e1822dba5baf34c98aae7003f19c10a916a0aa3967142f5c2dc` (abbrev `c258ff32…`).
> Handlers = 404 (H_00..H_397). Last selectors: 0x18C..0x193 = H_390..H_397 (`40 18C`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-53-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-058-log.md` / `docs/auxdocs/body-extend-058-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-058 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 194`.. for H_398.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 194`/`40 195` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-059 (serialize + Relock)

Mirror body-extend-058 / body-extend-057 protocol:

1. Hand-author append H_398..H_405 to `yoyo/projects/yoyo.ty` at selectors `40 194` .. `40 19B` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h50_108,subimm_h51_108,subimm_h52_108,ldb_5060_110,ldb_5160_110,ldb_5260_110,addimm_h50_110,addimm_h51_110}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `c258ff3271396e18…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-059-log.md`.
7. Auto-spawn parallel-batch-54 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-54-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_398 | 0x194 | 0x61 SUB-IMM | 50 108 | `498b87800200004881e80801000049898780020000c3` (22B) | `f139f28243c08957` |
| H_399 | 0x195 | 0x61 SUB-IMM | 51 108 | `498b87880200004881e80801000049898788020000c3` (22B) | `f9c122832287170d` |
| H_400 | 0x196 | 0x61 SUB-IMM | 52 108 | `498b87900200004881e80801000049898790020000c3` (22B) | `2f027342f5447eeb` |
| H_401 | 0x197 | 0x80 LDB | 50 60 110 | `498b87000300004881c010010000480fb60049898780020000c3` (26B) | `215fc443528e6163` |
| H_402 | 0x198 | 0x80 LDB | 51 60 110 | `498b87000300004881c010010000480fb60049898788020000c3` (26B) | `bfd294f2e3edf3d2` |
| H_403 | 0x199 | 0x80 LDB | 52 60 110 | `498b87000300004881c010010000480fb60049898790020000c3` (26B) | `d9d4fceaca2783f1` |
| H_404 | 0x19A | 0x62 ADD-IMM | 50 110 | `498b87800200004881c01001000049898780020000c3` (22B) | `b2f08439005e085c` |
| H_405 | 0x19B | 0x62 ADD-IMM | 51 110 | `498b87880200004881c01001000049898788020000c3` (22B) | `2c0923f7af81d76c` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
SUB-IMM imm=0x108 uses imm32 sub (`48 81 e8`) → 22B pins (H_398..H_400); not imm8.
ADD-IMM imm=0x110 uses imm32 add (`48 81 c0`) → 22B pins (H_404..H_405); not imm8.
LDB oo=0x110 uses imm32 add (`48 81 c0`) → 26B pins (H_401..H_403).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_398 | `f139f28243c08957d01875c549d2ab72beb6b6e844db45f306aedd137c994dd5` |
| H_399 | `f9c122832287170d748906fa59e9ce9e3085a10658b2ec201bd1e834774e3607` |
| H_400 | `2f027342f5447eeb3b13996588d05f598230d8f22bf13d6c799cb964980195da` |
| H_401 | `215fc443528e616380c90b46f110f7b01f8c297ab5df4f7e43ba6f6517bf6451` |
| H_402 | `bfd294f2e3edf3d26b22892867c61dc4b0f3c145731d8adb4d7dbd2bebd50154` |
| H_403 | `d9d4fceaca2783f16f51d918c011da1cf2b3ef895013a53e0d84b507e328cdbe` |
| H_404 | `b2f08439005e085cf5d7f2c196caa87d536da8ae5a92b10d081c36dfb7056598` |
| H_405 | `2c0923f7af81d76cc393610f14c1931f17b91d275e9301c244237ca83c316164` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h50_108`, `_scratch_subimm_h51_108`, `_scratch_subimm_h52_108`, `_scratch_ldb_5060_110`,
`_scratch_ldb_5160_110`, `_scratch_ldb_5260_110`, `_scratch_addimm_h50_110`, `_scratch_addimm_h51_110`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 194`.. for H_398.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
