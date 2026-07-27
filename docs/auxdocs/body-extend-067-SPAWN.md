# body-extend-067 SPAWN · consolidate parallel-batch-61

> Continuous queue handoff from parallel-batch-61 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `d52ed6373d5b085118d5a601ac8f25b8a529e7c16b36b6dd3bce2115d73ec080` (abbrev `d52ed637…`).
> Handlers = 467 (H_00..H_460). Last selectors: 0x1CB..0x1D2 = H_453..H_460 (`40 1CB`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-61-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-066-log.md` / `docs/auxdocs/body-extend-066-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-066 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1D3`.. for H_461.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1D3`/`40 1D4` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-067 (serialize + Relock)

Mirror body-extend-066 / body-extend-065 protocol:

1. Hand-author append H_461..H_468 to `yoyo/projects/yoyo.ty` at selectors `40 1D3` .. `40 1DA` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h50_140,subimm_h51_140,subimm_h52_140,ldb_5060_148,ldb_5160_148,ldb_5260_148,addimm_h50_148,addimm_h51_148}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `d52ed6373d5b085118d5a601ac8f25b8a529e7c16b36b6dd3bce2115d73ec080`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-067-log.md`.
7. Auto-spawn parallel-batch-62 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-62-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_461 | 0x1D3 | 0x61 SUB-IMM | 50 140 | `498b87800200004881e84001000049898780020000c3` (22B) | `cc93e3af0d6d31c3` |
| H_462 | 0x1D4 | 0x61 SUB-IMM | 51 140 | `498b87880200004881e84001000049898788020000c3` (22B) | `4c436b4f07ea2fa3` |
| H_463 | 0x1D5 | 0x61 SUB-IMM | 52 140 | `498b87900200004881e84001000049898790020000c3` (22B) | `7338547b13d01af3` |
| H_464 | 0x1D6 | 0x80 LDB | 50 60 148 | `498b87000300004881c048010000480fb60049898780020000c3` (26B) | `e043dad6b063887b` |
| H_465 | 0x1D7 | 0x80 LDB | 51 60 148 | `498b87000300004881c048010000480fb60049898788020000c3` (26B) | `0e0373648d5bea88` |
| H_466 | 0x1D8 | 0x80 LDB | 52 60 148 | `498b87000300004881c048010000480fb60049898790020000c3` (26B) | `d146b52055b94f9f` |
| H_467 | 0x1D9 | 0x62 ADD-IMM | 50 148 | `498b87800200004881c04801000049898780020000c3` (22B) | `32552f824b2e13d9` |
| H_468 | 0x1DA | 0x62 ADD-IMM | 51 148 | `498b87880200004881c04801000049898788020000c3` (22B) | `b44518792801dac1` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x148 uses imm32 add (`48 81 c0`) → 22B pins (H_467..H_468); not imm8.
SUB-IMM imm=0x140 uses imm32 sub (`48 81 e8`) → 22B pins (H_461..H_463); not imm8.
LDB oo=0x148 uses imm32 add (`48 81 c0`) → 26B pins (H_464..H_466); starts 148 LDB triad.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_461 | `cc93e3af0d6d31c360bc669fdb3f94fe102689ae4b5c426ee4b48e1ff3514502` |
| H_462 | `4c436b4f07ea2fa32f15218c8106244654688d8ccc6a85a9e807b6dc6637cc9f` |
| H_463 | `7338547b13d01af3acbb249208be136bb00731cae223870b3b226e92a0fb6019` |
| H_464 | `e043dad6b063887b9f6d5ba2ed78d3ee8416a2a0382675e97aff5f3aeca66757` |
| H_465 | `0e0373648d5bea887264ead6ce6e39e26d58f55e58db5f56d36ba1174667d748` |
| H_466 | `d146b52055b94f9f293f53d9482917bd2e5a5452e278ab5f1507d5a82d0b22d2` |
| H_467 | `32552f824b2e13d998986ac270a4b0d47f552321dc6c2163ce8ce8a73fb7ff2a` |
| H_468 | `b44518792801dac1c176295e0c3beee2169d3dca87ea69b882f7f56a0d8e8657` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h50_140`, `_scratch_subimm_h51_140`, `_scratch_subimm_h52_140`, `_scratch_ldb_5060_148`,
`_scratch_ldb_5160_148`, `_scratch_ldb_5260_148`, `_scratch_addimm_h50_148`, `_scratch_addimm_h51_148`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1D3`.. for H_461.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
