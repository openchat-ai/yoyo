# body-extend-053 SPAWN · consolidate parallel-batch-47

> Continuous queue handoff from parallel-batch-47 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `edee584aa21a26569fe08e60d5089daf8d823c9df4c829c62b788b10815f4a51` (abbrev `edee584a…`).
> Handlers = 356 (H_00..H_349). Last selectors: 0x15C..0x163 = H_342..H_349 (`40 15C`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-47-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-052-log.md` / `docs/auxdocs/body-extend-052-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-052 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 164`.. for H_350.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-053 (serialize + Relock)

Mirror body-extend-052 / body-extend-051 protocol:

1. Hand-author append H_350..H_357 to `yoyo/projects/yoyo.ty` at selectors `40 164` .. `40 16B` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5060_e8,ldb_5160_e8,ldb_5260_e8,addimm_h50_e0,addimm_h51_e0,addimm_h52_e0,subimm_h50_e0,subimm_h51_e0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `edee584aa21a2656…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-053-log.md`.
7. Auto-spawn parallel-batch-48 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-48-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_350 | 0x164 | 0x80 LDB | 50 60 E8 | `498b87000300004881c0e8000000480fb60049898780020000c3` (26B) | `8707f42f9e69fe94` |
| H_351 | 0x165 | 0x80 LDB | 51 60 E8 | `498b87000300004881c0e8000000480fb60049898788020000c3` (26B) | `1aa2e13843e522b5` |
| H_352 | 0x166 | 0x80 LDB | 52 60 E8 | `498b87000300004881c0e8000000480fb60049898790020000c3` (26B) | `465cb3e854ecc953` |
| H_353 | 0x167 | 0x62 ADD-IMM | 50 E0 | `498b87800200004881c0e000000049898780020000c3` (22B) | `9ef1fb8eb620deee` |
| H_354 | 0x168 | 0x62 ADD-IMM | 51 E0 | `498b87880200004881c0e000000049898788020000c3` (22B) | `4d09c2a3e224d2d4` |
| H_355 | 0x169 | 0x62 ADD-IMM | 52 E0 | `498b87900200004881c0e000000049898790020000c3` (22B) | `cd251baeb9a188f0` |
| H_356 | 0x16A | 0x61 SUB-IMM | 50 E0 | `498b87800200004881e8e000000049898780020000c3` (22B) | `6d7c5904f21181f1` |
| H_357 | 0x16B | 0x61 SUB-IMM | 51 E0 | `498b87880200004881e8e000000049898788020000c3` (22B) | `345b5a0581126cf4` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0xE0 uses imm32 add (`48 81 c0`) → 22B pins (H_353..H_355); not imm8.
SUB-IMM imm=0xE0 uses imm32 sub (`48 81 e8`) → 22B pins (H_356..H_357); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_350..H_352).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_350 | `8707f42f9e69fe94d91d27d11d46d30b579f1face242fca3b462ed6326d31418` |
| H_351 | `1aa2e13843e522b5978ad735a3835ccb3c5615f1c90f6c51068bf4a43ccb4d27` |
| H_352 | `465cb3e854ecc953423252e22214fd57713b1b578b7587ce65a901f56dc3e925` |
| H_353 | `9ef1fb8eb620deee26bffb39e983df220c047ba07c696a7ffd5b3abf58d20fac` |
| H_354 | `4d09c2a3e224d2d48d82a155a10e6aefa3c2d737f607d3472a32aa49b2457ede` |
| H_355 | `cd251baeb9a188f09b0b9a47fbb1160d773dc317eb7fecf06e455fc1fc20fc1e` |
| H_356 | `6d7c5904f21181f10ab244e96496582e13be627cead076bdc5fe8af5a3b7af7a` |
| H_357 | `345b5a0581126cf42158fac6fd12fd3b2d6b697ac475c2ce4891e9aa078641e4` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5060_e8`, `_scratch_ldb_5160_e8`, `_scratch_ldb_5260_e8`, `_scratch_addimm_h50_e0`,
`_scratch_addimm_h51_e0`, `_scratch_addimm_h52_e0`, `_scratch_subimm_h50_e0`, `_scratch_subimm_h51_e0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 164`.. for H_350.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
