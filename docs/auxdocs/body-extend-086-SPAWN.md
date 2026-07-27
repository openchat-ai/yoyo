# body-extend-086 SPAWN · consolidate parallel-batch-80

> Continuous queue handoff from parallel-batch-80 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `58b9ca6ef16f3ee48e22fae95f20dd6f6fa3492705659dfe181ec7857e9cf231` (abbrev `58b9ca6e…`).
> Handlers = 619 (H_00..H_612). Last selectors: 0x263..0x26A = H_605..H_612 (`40 263`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-80-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-085-log.md` / `docs/auxdocs/body-extend-085-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-085 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 26B`.. for H_613.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 26B`/`40 26C` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-086 (serialize + Relock)

Mirror body-extend-085 / body-extend-084 protocol:

1. Hand-author append H_613..H_620 to `yoyo/projects/yoyo.ty` at selectors `40 26B` .. `40 272` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h52_1C8,subimm_h50_1C8,subimm_h51_1C8,subimm_h52_1C8,ldb_5060_1D0,ldb_5160_1D0,ldb_5260_1D0,addimm_h50_1D0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `58b9ca6ef16f3ee48e22fae95f20dd6f6fa3492705659dfe181ec7857e9cf231`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-086-log.md`.
7. Auto-spawn parallel-batch-81 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-81-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_613 | 0x26B | 0x62 ADD-IMM | 52 1C8 | `498b87900200004881c0c801000049898790020000c3` (22B) | `dc11d2c2afb93a56` |
| H_614 | 0x26C | 0x61 SUB-IMM | 50 1C8 | `498b87800200004881e8c801000049898780020000c3` (22B) | `3c7c7cf3d889226e` |
| H_615 | 0x26D | 0x61 SUB-IMM | 51 1C8 | `498b87880200004881e8c801000049898788020000c3` (22B) | `63000a311432b0f3` |
| H_616 | 0x26E | 0x61 SUB-IMM | 52 1C8 | `498b87900200004881e8c801000049898790020000c3` (22B) | `fa6d5ee090445380` |
| H_617 | 0x26F | 0x80 LDB | 50 60 1D0 | `498b87000300004881c0d0010000480fb60049898780020000c3` (26B) | `a8b6a7f0de518100` |
| H_618 | 0x270 | 0x80 LDB | 51 60 1D0 | `498b87000300004881c0d0010000480fb60049898788020000c3` (26B) | `261db47e68ac40dd` |
| H_619 | 0x271 | 0x80 LDB | 52 60 1D0 | `498b87000300004881c0d0010000480fb60049898790020000c3` (26B) | `e6b3a3507a16a0ad` |
| H_620 | 0x272 | 0x62 ADD-IMM | 50 1D0 | `498b87800200004881c0d001000049898780020000c3` (22B) | `16f0fd643450814e` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1C8 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1C8 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1D0 uses imm32 add (`48 81 c0`) → 26B pins.
ADD-IMM imm=0x1D0 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
ADD-IMM slot=52 imm=1C8 finishes deferred 1C8 ADD triad (H_613; after H_611/H_612).
SUB-IMM slot=50/51/52 imm=1C8 starts deferred 1C8 SUB triad (H_614/H_615/H_616).
LDB oo=0x1D0 starts 1D0 LDB triad (H_617/H_618/H_619 dd=50/51/52).
ADD-IMM slot=50 imm=1D0 starts deferred 1D0 ADD triad (H_620; slot=51/52 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_613 | `dc11d2c2afb93a5639d5181320f7bb5f8cd6b76b728cc953e444f0cd6f855552` |
| H_614 | `3c7c7cf3d889226ec5a5e56c043d94c7305b86fbeac9d416ec3579460d314e78` |
| H_615 | `63000a311432b0f32a16408c134e65bdf4b24fd57dbf9044977d5ef21834c1ee` |
| H_616 | `fa6d5ee09044538017d8b39642cead9b1ad243d5d2b3f7ad0fe6a157d90d4357` |
| H_617 | `a8b6a7f0de5181005aab365a71ab7f9c98a8d73d9330c9cd3bcd381b681de8f4` |
| H_618 | `261db47e68ac40ddeee1b0bcdae0569d5e6c86d5d08f8bba83351a373ef7b16f` |
| H_619 | `e6b3a3507a16a0ad852ee9de1afec9f3a6e6edcfe3b3d7e2a1b334bdcb6871fc` |
| H_620 | `16f0fd643450814e8259b7fd79b368cec2ec55083311ed7af2be5f3768de712b` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h52_1C8`, `_scratch_subimm_h50_1C8`, `_scratch_subimm_h51_1C8`, `_scratch_subimm_h52_1C8`,
`_scratch_ldb_5060_1D0`, `_scratch_ldb_5160_1D0`, `_scratch_ldb_5260_1D0`, `_scratch_addimm_h50_1D0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 26B`.. for H_613.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-81

- ADD-IMM slot=51/52 imm=1D0 (continue/finish 1D0 ADD triad)
- SUB-IMM slot=50/51/52 imm=1D0 (start 1D0 SUB triad)
- LDB / ADD-IMM / SUB-IMM next imm ladder (1D8…) if continuing
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
