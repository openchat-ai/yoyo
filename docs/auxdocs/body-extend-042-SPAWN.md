# body-extend-042 SPAWN · consolidate parallel-batch-36

> Continuous queue handoff from parallel-batch-36 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `4cb656812b03c0fdb229b2d0d9278c479ab83b33d6cc7782e75f2397b0e165db` (abbrev `4cb65681…`).
> Handlers = 268 (H_00..H_261). Last selectors: 0x104..0x10B = H_254..H_261 (`40 104`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-36-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-041-log.md` / `docs/auxdocs/body-extend-041-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-041 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 10C`.. for H_262.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-042 (serialize + Relock)

Mirror body-extend-041 / body-extend-040 protocol:

1. Hand-author append H_262..H_269 to `yoyo/projects/yoyo.ty` at selectors `40 10C` .. `40 113` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{addimm_h52_88,subimm_h50_88,subimm_h51_88,subimm_h52_88,ldb_5060_a8,ldb_5160_a8,ldb_5260_a8,addimm_h50_90}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `4cb656812b03c0fd…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-042-log.md`.
7. Auto-spawn parallel-batch-37 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-37-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_262 | 0x10C | 0x62 ADD-IMM | 52 88 | `498b87900200004881c08800000049898790020000c3` (22B) | `97f31856e0e0bace` |
| H_263 | 0x10D | 0x61 SUB-IMM | 50 88 | `498b87800200004881e88800000049898780020000c3` (22B) | `031eecb381c11df4` |
| H_264 | 0x10E | 0x61 SUB-IMM | 51 88 | `498b87880200004881e88800000049898788020000c3` (22B) | `e032f65c781b8d24` |
| H_265 | 0x10F | 0x61 SUB-IMM | 52 88 | `498b87900200004881e88800000049898790020000c3` (22B) | `a35fd747b10ad6c0` |
| H_266 | 0x110 | 0x80 LDB | 50 60 A8 | `498b87000300004881c0a8000000480fb60049898780020000c3` (26B) | `9406298c7e1a9bb7` |
| H_267 | 0x111 | 0x80 LDB | 51 60 A8 | `498b87000300004881c0a8000000480fb60049898788020000c3` (26B) | `21a57bbe40cd51a3` |
| H_268 | 0x112 | 0x80 LDB | 52 60 A8 | `498b87000300004881c0a8000000480fb60049898790020000c3` (26B) | `6ce7678316409535` |
| H_269 | 0x113 | 0x62 ADD-IMM | 50 90 | `498b87800200004881c09000000049898780020000c3` (22B) | `606ca6ba641f5721` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x88/0x90 uses imm32 add (`48 81 c0`) → 22B pins (H_262, H_269); not imm8.
SUB-IMM imm=0x88 uses imm32 sub (`48 81 e8`) → 22B pins (H_263..H_265); not imm8.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_266..H_268).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_262 | `97f31856e0e0baceaed798f77604b2aff777ecb608b46a6719c6f4d0533e968e` |
| H_263 | `031eecb381c11df4dc03b6e63f7180b0548c2c6a4b5d7449c03f6b80737d8eda` |
| H_264 | `e032f65c781b8d241d0ad68b35a5869154e97e6eea83386d3bdc567e1c5f06a7` |
| H_265 | `a35fd747b10ad6c00daa10d453b7ab1cf6634f2f583e8fdb97df753be554fb2a` |
| H_266 | `9406298c7e1a9bb70892f6a9e517ae808a991428b3e948f3841407476c6fb62d` |
| H_267 | `21a57bbe40cd51a35371ea11cf3e31a8c3048a3519a6d5757d9045dfa4930007` |
| H_268 | `6ce7678316409535d605ab8f7e632d45e6fa1dd39f9570eb6d264b01ed9e6036` |
| H_269 | `606ca6ba641f572123be83228e402a8392e65cac1ed4b8e8dca1e566f5410610` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h52_88`, `_scratch_subimm_h50_88`, `_scratch_subimm_h51_88`, `_scratch_subimm_h52_88`,
`_scratch_ldb_5060_a8`, `_scratch_ldb_5160_a8`, `_scratch_ldb_5260_a8`, `_scratch_addimm_h50_90`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 10C`.. for H_262.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
