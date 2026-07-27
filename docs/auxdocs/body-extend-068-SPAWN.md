# body-extend-068 SPAWN · consolidate parallel-batch-62

> Continuous queue handoff from parallel-batch-62 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `deaf40134394a58d9e81fd3a8f55c4ec9110fc93ad8d366e547f0628144dd098` (abbrev `deaf4013…`).
> Handlers = 475 (H_00..H_468). Last selectors: 0x1D3..0x1DA = H_461..H_468 (`40 1D3`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-62-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-067-log.md` / `docs/auxdocs/body-extend-067-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-067 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1DB`.. for H_469.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1DB`/`40 1DC` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-068 (serialize + Relock)

Mirror body-extend-067 / body-extend-066 protocol:

1. Hand-author append H_469..H_476 to `yoyo/projects/yoyo.ty` at selectors `40 1DB` .. `40 1E2` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h52_148,subimm_h50_148,subimm_h51_148,subimm_h52_148,ldb_5060_150,ldb_5160_150,ldb_5260_150,addimm_h50_150}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `deaf40134394a58d9e81fd3a8f55c4ec9110fc93ad8d366e547f0628144dd098`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-068-log.md`.
7. Auto-spawn parallel-batch-63 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-63-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_469 | 0x1DB | 0x62 ADD-IMM | 52 148 | `498b87900200004881c04801000049898790020000c3` (22B) | `e5c549e3bb998799` |
| H_470 | 0x1DC | 0x61 SUB-IMM | 50 148 | `498b87800200004881e84801000049898780020000c3` (22B) | `4310d24ed1a65b24` |
| H_471 | 0x1DD | 0x61 SUB-IMM | 51 148 | `498b87880200004881e84801000049898788020000c3` (22B) | `20c893f5b357112c` |
| H_472 | 0x1DE | 0x61 SUB-IMM | 52 148 | `498b87900200004881e84801000049898790020000c3` (22B) | `7b21e0e79d618564` |
| H_473 | 0x1DF | 0x80 LDB | 50 60 150 | `498b87000300004881c050010000480fb60049898780020000c3` (26B) | `a2f4d32aedf227d7` |
| H_474 | 0x1E0 | 0x80 LDB | 51 60 150 | `498b87000300004881c050010000480fb60049898788020000c3` (26B) | `eebeaa9843e6b88f` |
| H_475 | 0x1E1 | 0x80 LDB | 52 60 150 | `498b87000300004881c050010000480fb60049898790020000c3` (26B) | `34288a223e426de8` |
| H_476 | 0x1E2 | 0x62 ADD-IMM | 50 150 | `498b87800200004881c05001000049898780020000c3` (22B) | `62f0518dcdd6f717` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x148/0x150 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x148 uses imm32 sub (`48 81 e8`) → 22B pins (H_470..H_472); not imm8.
LDB oo=0x150 uses imm32 add (`48 81 c0`) → 26B pins (H_473..H_475); starts 150 LDB triad.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_469 | `e5c549e3bb9987999d6ffe4c84244f5291cfb5648c3d6b7e4096ce3656b58f15` |
| H_470 | `4310d24ed1a65b24f4058f1ae7fa401d3ae961311efeddc498f48b7c0301018d` |
| H_471 | `20c893f5b357112c74292a5b6e248e47cce9f61d3bfe6796f6d98b78706c8228` |
| H_472 | `7b21e0e79d61856485ee0371d9bf4ce4034ce96a933115f21a251a9993675d35` |
| H_473 | `a2f4d32aedf227d7e2d30ed001b3bd48bd520b742b9541b13b5798eb1abc43d2` |
| H_474 | `eebeaa9843e6b88f062fabea58ce3413490e91045254351aff70c176a9481353` |
| H_475 | `34288a223e426de8456c6b5f4645e3b87c5c665600a86f1fc69a3ce44b49dab2` |
| H_476 | `62f0518dcdd6f7174c229cea63a6247f5d79a5e2fe97acdd544ce8fd81270a4c` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h52_148`, `_scratch_subimm_h50_148`, `_scratch_subimm_h51_148`, `_scratch_subimm_h52_148`,
`_scratch_ldb_5060_150`, `_scratch_ldb_5160_150`, `_scratch_ldb_5260_150`, `_scratch_addimm_h50_150`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1DB`.. for H_469.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
