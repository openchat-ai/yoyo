# body-extend-047 SPAWN · consolidate parallel-batch-41

> Continuous queue handoff from parallel-batch-41 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `422c843275989ac30c1ba7406c7ff47076310df79ef0c3193903bca15460afde` (abbrev `422c8432…`).
> Handlers = 308 (H_00..H_301). Last selectors: 0x12C..0x133 = H_294..H_301 (`40 12C`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-41-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-046-log.md` / `docs/auxdocs/body-extend-046-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-046 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 134`.. for H_302.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-047 (serialize + Relock)

Mirror body-extend-046 / body-extend-045 protocol:

1. Hand-author append H_302..H_309 to `yoyo/projects/yoyo.ty` at selectors `40 134` .. `40 13B` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5060_c8,ldb_5160_c8,ldb_5260_c8,addimm_h50_b0,addimm_h51_b0,addimm_h52_b0,subimm_h50_b0,subimm_h51_b0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `422c843275989ac3…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-047-log.md`.
7. Auto-spawn parallel-batch-42 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-42-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_302 | 0x134 | 0x80 LDB | 50 60 C8 | `498b87000300004881c0c8000000480fb60049898780020000c3` (26B) | `236016ef799b3ff7` |
| H_303 | 0x135 | 0x80 LDB | 51 60 C8 | `498b87000300004881c0c8000000480fb60049898788020000c3` (26B) | `7eb39f3637eb2267` |
| H_304 | 0x136 | 0x80 LDB | 52 60 C8 | `498b87000300004881c0c8000000480fb60049898790020000c3` (26B) | `b9fa804bcc69d95c` |
| H_305 | 0x137 | 0x62 ADD-IMM | 50 B0 | `498b87800200004881c0b000000049898780020000c3` (22B) | `9be2c80577bd6f4a` |
| H_306 | 0x138 | 0x62 ADD-IMM | 51 B0 | `498b87880200004881c0b000000049898788020000c3` (22B) | `e3c08eecc6fae6f3` |
| H_307 | 0x139 | 0x62 ADD-IMM | 52 B0 | `498b87900200004881c0b000000049898790020000c3` (22B) | `9d760ed911115fb1` |
| H_308 | 0x13A | 0x61 SUB-IMM | 50 B0 | `498b87800200004881e8b000000049898780020000c3` (22B) | `1d73d8c916bc7e20` |
| H_309 | 0x13B | 0x61 SUB-IMM | 51 B0 | `498b87880200004881e8b000000049898788020000c3` (22B) | `3e629652dbf4e5ea` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xB0 uses imm32 add (`48 81 c0`) → 22B pins (H_305..H_307); not imm8.
SUB-IMM imm=0xB0 uses imm32 sub (`48 81 e8`) → 22B pins (H_308..H_309); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_302..H_304).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_302 | `236016ef799b3ff7d89b78654d7883b6c39ca7d2c189119062bb8b33b86840be` |
| H_303 | `7eb39f3637eb22675442d8ffe75672093ad49f3f2dc4a05e2644c9728c72b192` |
| H_304 | `b9fa804bcc69d95c3299850d3f4e6ed7a796ed8bcf76a8eb0e7a295d9879a1b0` |
| H_305 | `9be2c80577bd6f4a435643f56840c2814984507ebe8839d981cc6508aea8eced` |
| H_306 | `e3c08eecc6fae6f3bb9d6f51283e2059bf578617053767715801d4a537c476aa` |
| H_307 | `9d760ed911115fb192c20eff1e4bedbf28e0461c687efc20f466cdd9aca5d063` |
| H_308 | `1d73d8c916bc7e2063ccb408819463585335a4e013fe46aeceac142e33363813` |
| H_309 | `3e629652dbf4e5eafd4154012e99a9c8aa5c5b67369b29036257fb60ca4ea603` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5060_c8`, `_scratch_ldb_5160_c8`, `_scratch_ldb_5260_c8`, `_scratch_addimm_h50_b0`,
`_scratch_addimm_h51_b0`, `_scratch_addimm_h52_b0`, `_scratch_subimm_h50_b0`, `_scratch_subimm_h51_b0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 134`.. for H_302.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
