# body-extend-071 SPAWN · consolidate parallel-batch-65

> Continuous queue handoff from parallel-batch-65 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `192ba67ac8bb814df865a108032dd1e9301c93c4e3fc89f44c8c4edfaf84791f` (abbrev `192ba67a…`).
> Handlers = 499 (H_00..H_492). Last selectors: 0x1EB..0x1F2 = H_485..H_492 (`40 1EB`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-65-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-070-log.md` / `docs/auxdocs/body-extend-070-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-070 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1F3`.. for H_493.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1F3`/`40 1F4` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-071 (serialize + Relock)

Mirror body-extend-070 / body-extend-069 protocol:

1. Hand-author append H_493..H_500 to `yoyo/projects/yoyo.ty` at selectors `40 1F3` .. `40 1FA` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5260_160,addimm_h50_160,addimm_h51_160,addimm_h52_160,subimm_h50_160,subimm_h51_160,subimm_h52_160,ldb_5060_168}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `192ba67ac8bb814df865a108032dd1e9301c93c4e3fc89f44c8c4edfaf84791f`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-071-log.md`.
7. Auto-spawn parallel-batch-66 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-66-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_493 | 0x1F3 | 0x80 LDB | 52 60 160 | `498b87000300004881c060010000480fb60049898790020000c3` (26B) | `9daf84e1a128dac3` |
| H_494 | 0x1F4 | 0x62 ADD-IMM | 50 160 | `498b87800200004881c06001000049898780020000c3` (22B) | `3b8d32f8073e00b9` |
| H_495 | 0x1F5 | 0x62 ADD-IMM | 51 160 | `498b87880200004881c06001000049898788020000c3` (22B) | `be65ff093c4ef72d` |
| H_496 | 0x1F6 | 0x62 ADD-IMM | 52 160 | `498b87900200004881c06001000049898790020000c3` (22B) | `8eae86a7c8b26fc7` |
| H_497 | 0x1F7 | 0x61 SUB-IMM | 50 160 | `498b87800200004881e86001000049898780020000c3` (22B) | `cb0f44be7ee7be5e` |
| H_498 | 0x1F8 | 0x61 SUB-IMM | 51 160 | `498b87880200004881e86001000049898788020000c3` (22B) | `ce408999f0330ce3` |
| H_499 | 0x1F9 | 0x61 SUB-IMM | 52 160 | `498b87900200004881e86001000049898790020000c3` (22B) | `17997181ac08f1e4` |
| H_500 | 0x1FA | 0x80 LDB | 50 60 168 | `498b87000300004881c068010000480fb60049898780020000c3` (26B) | `c6ea0ffbc5102366` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x160 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x160 uses imm32 sub (`48 81 e8`) → 22B pins (H_497..H_499); not imm8.
LDB oo=0x160 finishes 160 LDB triad (H_493 dd=52); LDB oo=0x168 starts 168 triad (H_500).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_493 | `9daf84e1a128dac3db7b56a14504d2af971c50dafb3c941b4334f18acb877680` |
| H_494 | `3b8d32f8073e00b9cc1776f3ae4c7571f619edc80656d7d8c205a59d564aab8c` |
| H_495 | `be65ff093c4ef72d1b18b6a6147be3ee5eeba8a1fbc6cca22ff65f62bba230db` |
| H_496 | `8eae86a7c8b26fc7caf8bd5b650843d1fbc19699843c66dd7490e607beec5fab` |
| H_497 | `cb0f44be7ee7be5e0c5fd7f216a2875e1ff1e0a118dcdd9d05e3100de566c7d3` |
| H_498 | `ce408999f0330ce3d49fe066acb8919d8ce3a45d6a23d5b02cb419a7b0c22759` |
| H_499 | `17997181ac08f1e426048cbb9445d17c75f1cf67950f99126d984c76bbfee04d` |
| H_500 | `c6ea0ffbc5102366eebbf2593c147be7a0438ccc0d5b3c95fba3b1c3a049b41c` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5260_160`, `_scratch_addimm_h50_160`, `_scratch_addimm_h51_160`, `_scratch_addimm_h52_160`,
`_scratch_subimm_h50_160`, `_scratch_subimm_h51_160`, `_scratch_subimm_h52_160`, `_scratch_ldb_5060_168`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1F3`.. for H_493.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
