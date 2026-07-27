# body-extend-060 SPAWN · consolidate parallel-batch-54

> Continuous queue handoff from parallel-batch-54 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `bd7bad15e53fe296e790c57803a0d44930e95c7f7db99ee866685fbb5d504f12` (abbrev `bd7bad15…`).
> Handlers = 412 (H_00..H_405). Last selectors: 0x194..0x19B = H_398..H_405 (`40 194`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-54-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-059-log.md` / `docs/auxdocs/body-extend-059-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-059 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 19C`.. for H_406.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 19C`/`40 19D` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-060 (serialize + Relock)

Mirror body-extend-059 / body-extend-058 protocol:

1. Hand-author append H_406..H_413 to `yoyo/projects/yoyo.ty` at selectors `40 19C` .. `40 1A3` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h52_110,subimm_h50_110,subimm_h51_110,subimm_h52_110,ldb_5060_118,ldb_5160_118,ldb_5260_118,addimm_h50_118}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `bd7bad15e53fe296…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-060-log.md`.
7. Auto-spawn parallel-batch-55 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-55-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_406 | 0x19C | 0x62 ADD-IMM | 52 110 | `498b87900200004881c01001000049898790020000c3` (22B) | `aad3c15ce012a85e` |
| H_407 | 0x19D | 0x61 SUB-IMM | 50 110 | `498b87800200004881e81001000049898780020000c3` (22B) | `ab4a316c8b299ed0` |
| H_408 | 0x19E | 0x61 SUB-IMM | 51 110 | `498b87880200004881e81001000049898788020000c3` (22B) | `edaa468a46b020a6` |
| H_409 | 0x19F | 0x61 SUB-IMM | 52 110 | `498b87900200004881e81001000049898790020000c3` (22B) | `921cdaad23a0f9f0` |
| H_410 | 0x1A0 | 0x80 LDB | 50 60 118 | `498b87000300004881c018010000480fb60049898780020000c3` (26B) | `41253a7fe67f42ba` |
| H_411 | 0x1A1 | 0x80 LDB | 51 60 118 | `498b87000300004881c018010000480fb60049898788020000c3` (26B) | `2eaf03e9dc35344e` |
| H_412 | 0x1A2 | 0x80 LDB | 52 60 118 | `498b87000300004881c018010000480fb60049898790020000c3` (26B) | `aad78ddac628a62f` |
| H_413 | 0x1A3 | 0x62 ADD-IMM | 50 118 | `498b87800200004881c01801000049898780020000c3` (22B) | `c90d1c2f223e7e95` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x110/0x118 uses imm32 add (`48 81 c0`) → 22B pins (H_406, H_413); not imm8.
SUB-IMM imm=0x110 uses imm32 sub (`48 81 e8`) → 22B pins (H_407..H_409); not imm8.
LDB oo=0x118 uses imm32 add (`48 81 c0`) → 26B pins (H_410..H_412).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_406 | `aad3c15ce012a85efcb76653692940a6a0b953f7b6459f6a331b028c09c4e180` |
| H_407 | `ab4a316c8b299ed0f3c145147bde52c2396bde66a51e685fcf6ea2b59bb66a14` |
| H_408 | `edaa468a46b020a6aa1ba84494ea9eb934941ce25bb27eff05927330a7f1923c` |
| H_409 | `921cdaad23a0f9f053deebd0fe1e0f1ea1242167e3e1158a7dc0a50566420dd9` |
| H_410 | `41253a7fe67f42ba26e2b9da2ed9592275a5aa1a51933926aba81a2accca15ca` |
| H_411 | `2eaf03e9dc35344e88d8d8c1bce06533b75d7d15d969662ef045f05978e67ed1` |
| H_412 | `aad78ddac628a62ffa1ca1729860fa2110a9c4a7cd568e039e2b260a377ecb2b` |
| H_413 | `c90d1c2f223e7e95380b467aeeb09696776be7a69337f650ed7067d507fb08f1` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h52_110`, `_scratch_subimm_h50_110`, `_scratch_subimm_h51_110`, `_scratch_subimm_h52_110`,
`_scratch_ldb_5060_118`, `_scratch_ldb_5160_118`, `_scratch_ldb_5260_118`, `_scratch_addimm_h50_118`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 19C`.. for H_406.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
