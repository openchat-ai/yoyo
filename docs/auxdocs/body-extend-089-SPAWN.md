# body-extend-089 SPAWN · consolidate parallel-batch-83

> Continuous queue handoff from parallel-batch-83 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `697ad7847ba15e825ee7a2663be37eb71de542256a38f42ed2e7dc16ddca549c` (abbrev `697ad784…`).
> Handlers = 643 (H_00..H_636). Last selectors: 0x27B..0x282 = H_629..H_636 (`40 27B`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-83-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-088-log.md` / `docs/auxdocs/body-extend-088-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-088 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 283`.. for H_637.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 283`/`40 28A` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Task: body-extend-089 (serialize + Relock)

Mirror body-extend-088 / body-extend-087 protocol:

1. Hand-author append H_637..H_644 to `yoyo/projects/yoyo.ty` at selectors `40 283` .. `40 28A` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5260_1E0,addimm_h50_1E0,addimm_h51_1E0,addimm_h52_1E0,subimm_h50_1E0,subimm_h51_1E0,subimm_h52_1E0,ldb_5060_1E8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `697ad7847ba15e825ee7a2663be37eb71de542256a38f42ed2e7dc16ddca549c`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-089-log.md`.
7. Auto-spawn parallel-batch-84 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-84-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_637 | 0x283 | 0x80 LDB | 52 60 1E0 | `498b87000300004881c0e0010000480fb60049898790020000c3` (26B) | `a8e2361d68cd8eae` |
| H_638 | 0x284 | 0x62 ADD-IMM | 50 1E0 | `498b87800200004881c0e001000049898780020000c3` (22B) | `f8386b9a462dfb05` |
| H_639 | 0x285 | 0x62 ADD-IMM | 51 1E0 | `498b87880200004881c0e001000049898788020000c3` (22B) | `1eba92f3a87f8de9` |
| H_640 | 0x286 | 0x62 ADD-IMM | 52 1E0 | `498b87900200004881c0e001000049898790020000c3` (22B) | `e15ba36fe8e77c0c` |
| H_641 | 0x287 | 0x61 SUB-IMM | 50 1E0 | `498b87800200004881e8e001000049898780020000c3` (22B) | `485f29f7f7612705` |
| H_642 | 0x288 | 0x61 SUB-IMM | 51 1E0 | `498b87880200004881e8e001000049898788020000c3` (22B) | `aceddcae0b9c827f` |
| H_643 | 0x289 | 0x61 SUB-IMM | 52 1E0 | `498b87900200004881e8e001000049898790020000c3` (22B) | `1641521a26d49973` |
| H_644 | 0x28A | 0x80 LDB | 50 60 1E8 | `498b87000300004881c0e8010000480fb60049898780020000c3` (26B) | `6089535af769e9fe` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1E0 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1E0 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1E0 / 0x1E8 uses imm32 add (`48 81 c0`) → 26B pins.
LDB dd=52 ss=60 oo=1E0 finishes deferred 1E0 LDB triad (H_637).
ADD-IMM slot=50/51/52 imm=1E0 starts deferred 1E0 ADD triad (H_638/H_639/H_640).
SUB-IMM slot=50/51/52 imm=1E0 starts deferred 1E0 SUB triad (H_641/H_642/H_643).
LDB oo=0x1E8 starts next 1E8 LDB ladder (H_644 dd=50; LDB 51/52 1E8 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_637 | `a8e2361d68cd8eae6c92e118c9e39d49a60d9a09ae5069255f07a61cfc6c5cd0` |
| H_638 | `f8386b9a462dfb05f58cbf376c60a5c859566fdd49e199324cd680fe41c2ed09` |
| H_639 | `1eba92f3a87f8de92d8440f8638f1aa250783c5413bb0bd219fd3623324d8f8b` |
| H_640 | `e15ba36fe8e77c0ceee37fb2cddd486e6aa388f0986b8a8c99f3fd69bf0c2aeb` |
| H_641 | `485f29f7f7612705dbad5255c877b24d6e90c20ddba3266039bc4709b01836e7` |
| H_642 | `aceddcae0b9c827f4a8c54a6b402b433a8793dd97eb123aa66f110007b721a39` |
| H_643 | `1641521a26d49973a6b927072b1f7d18f933f02e7f527f704a8fdb52e1185779` |
| H_644 | `6089535af769e9fe003c54c3f2ec91e0d295c3d4cfaf4e6d8348fef1cdef0d6e` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5260_1E0`, `_scratch_addimm_h50_1E0`, `_scratch_addimm_h51_1E0`, `_scratch_addimm_h52_1E0`,
`_scratch_subimm_h50_1E0`, `_scratch_subimm_h51_1E0`, `_scratch_subimm_h52_1E0`, `_scratch_ldb_5060_1E8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 283`.. for H_637.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-84

- LDB dd=51/52 ss=60 oo=1E8 (finish 1E8 LDB triad)
- ADD-IMM / SUB-IMM slot=50/51/52 imm=1E8 (start 1E8 ADD/SUB triads)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
