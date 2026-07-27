# body-extend-058 SPAWN · consolidate parallel-batch-52

> Continuous queue handoff from parallel-batch-52 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `0643c8f550fbb85d6e85eac409cf7ac90a26d7fece1b33bcfe04af260a9f2d5a` (abbrev `0643c8f5…`).
> Handlers = 396 (H_00..H_389). Last selectors: 0x184..0x18B = H_382..H_389 (`40 184`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-52-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-057-log.md` / `docs/auxdocs/body-extend-057-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-057 DDC PE `.text` measured DIFFER this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 18C`.. for H_390.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 18C`/`40 18D` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-058 (serialize + Relock)

Mirror body-extend-057 / body-extend-056 protocol:

1. Hand-author append H_390..H_397 to `yoyo/projects/yoyo.ty` at selectors `40 18C` .. `40 193` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h51_100,subimm_h52_100,ldb_5060_108,ldb_5160_108,ldb_5260_108,addimm_h50_108,addimm_h51_108,addimm_h52_108}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `0643c8f550fbb85d…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-058-log.md`.
7. Auto-spawn parallel-batch-53 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-53-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_390 | 0x18C | 0x61 SUB-IMM | 51 100 | `498b87880200004881e80001000049898788020000c3` (22B) | `114da116f5fa5311` |
| H_391 | 0x18D | 0x61 SUB-IMM | 52 100 | `498b87900200004881e80001000049898790020000c3` (22B) | `3f28a582a9c075b7` |
| H_392 | 0x18E | 0x80 LDB | 50 60 108 | `498b87000300004881c008010000480fb60049898780020000c3` (26B) | `bdf235d9350d7497` |
| H_393 | 0x18F | 0x80 LDB | 51 60 108 | `498b87000300004881c008010000480fb60049898788020000c3` (26B) | `3b65bdaff0e56bf1` |
| H_394 | 0x190 | 0x80 LDB | 52 60 108 | `498b87000300004881c008010000480fb60049898790020000c3` (26B) | `86e5cf11a57df77e` |
| H_395 | 0x191 | 0x62 ADD-IMM | 50 108 | `498b87800200004881c00801000049898780020000c3` (22B) | `fc5f70d4e243183e` |
| H_396 | 0x192 | 0x62 ADD-IMM | 51 108 | `498b87880200004881c00801000049898788020000c3` (22B) | `d00fb3f6020656aa` |
| H_397 | 0x193 | 0x62 ADD-IMM | 52 108 | `498b87900200004881c00801000049898790020000c3` (22B) | `2ddfc84367ac3ec1` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
SUB-IMM imm=0x100 uses imm32 sub (`48 81 e8`) → 22B pins (H_390..H_391); not imm8.
ADD-IMM imm=0x108 uses imm32 add (`48 81 c0`) → 22B pins (H_395..H_397); not imm8.
LDB oo=0x108 uses imm32 add (`48 81 c0`) → 26B pins (H_392..H_394).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_390 | `114da116f5fa531189a387eca1ad22515497da5f495eb5fb1564de0398031f79` |
| H_391 | `3f28a582a9c075b78d4c8f87917d81f11fc876764e69c16d3955c71cc5670e26` |
| H_392 | `bdf235d9350d74971fa150a308a71d424c436606e4d6272f9a19171175c1093f` |
| H_393 | `3b65bdaff0e56bf1da128dae7fe9f0392050d4da5e14fcbc8fab89074e4d27a0` |
| H_394 | `86e5cf11a57df77e9404ad63619a2802acf5b27fd543931beff68332b1beaf26` |
| H_395 | `fc5f70d4e243183eb35be37fc3620ea96c6e710da2e67f718e54a640869d10ac` |
| H_396 | `d00fb3f6020656aab95eb7534dd349484ad6910c4b98a18937f3c99f3e9f659a` |
| H_397 | `2ddfc84367ac3ec18c85edb05f250de1b7a2fb8fb032048a129db209c91c546f` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h51_100`, `_scratch_subimm_h52_100`, `_scratch_ldb_5060_108`, `_scratch_ldb_5160_108`,
`_scratch_ldb_5260_108`, `_scratch_addimm_h50_108`, `_scratch_addimm_h51_108`, `_scratch_addimm_h52_108`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 18C`.. for H_390.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
