# body-extend-080 SPAWN · consolidate parallel-batch-74

> Continuous queue handoff from parallel-batch-74 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `0e5b612c7e4882a1de87b39c35cafe0e6ccdfdc174e4f378dcd28b799de58c73` (abbrev `0e5b612c…`).
> Handlers = 571 (H_00..H_564). Last selectors: 0x233..0x23A = H_557..H_564 (`40 233`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-74-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-079-log.md` / `docs/auxdocs/body-extend-079-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-079 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 23B`.. for H_565.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 23B`/`40 23C` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-080 (serialize + Relock)

Mirror body-extend-079 / body-extend-078 protocol:

1. Hand-author append H_565..H_572 to `yoyo/projects/yoyo.ty` at selectors `40 23B` .. `40 242` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5260_1A0,addimm_h50_1A0,addimm_h51_1A0,addimm_h52_1A0,subimm_h50_1A0,subimm_h51_1A0,subimm_h52_1A0,ldb_5060_1A8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `0e5b612c7e4882a1de87b39c35cafe0e6ccdfdc174e4f378dcd28b799de58c73`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-080-log.md`.
7. Auto-spawn parallel-batch-75 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-75-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_565 | 0x23B | 0x80 LDB | 52 60 1A0 | `498b87000300004881c0a0010000480fb60049898790020000c3` (26B) | `5492824be268600b` |
| H_566 | 0x23C | 0x62 ADD-IMM | 50 1A0 | `498b87800200004881c0a001000049898780020000c3` (22B) | `d6c054ff35b9b724` |
| H_567 | 0x23D | 0x62 ADD-IMM | 51 1A0 | `498b87880200004881c0a001000049898788020000c3` (22B) | `5db8f3bc0d22ae9b` |
| H_568 | 0x23E | 0x62 ADD-IMM | 52 1A0 | `498b87900200004881c0a001000049898790020000c3` (22B) | `88b0244979ff3341` |
| H_569 | 0x23F | 0x61 SUB-IMM | 50 1A0 | `498b87800200004881e8a001000049898780020000c3` (22B) | `f7a21b3a8775eaaa` |
| H_570 | 0x240 | 0x61 SUB-IMM | 51 1A0 | `498b87880200004881e8a001000049898788020000c3` (22B) | `47d4190d9e3f6f16` |
| H_571 | 0x241 | 0x61 SUB-IMM | 52 1A0 | `498b87900200004881e8a001000049898790020000c3` (22B) | `131a705e499f8031` |
| H_572 | 0x242 | 0x80 LDB | 50 60 1A8 | `498b87000300004881c0a8010000480fb60049898780020000c3` (26B) | `c7b2148d29e6d1e4` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1A0 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1A0 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1A0 finishes 1A0 LDB triad (H_565 dd=52; H_563/H_564 already locked).
ADD-IMM slot=50/51/52 imm=1A0 starts deferred 1A0 ADD triad (H_566/H_567/H_568).
SUB-IMM slot=50/51/52 imm=1A0 starts deferred 1A0 SUB triad (H_569/H_570/H_571).
LDB oo=0x1A8 starts 1A8 LDB triad (H_572 dd=50; leave dd=51/52 deferred for batch-75).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_565 | `5492824be268600b5039c102e6cc5b2234f3a4f2eec3d48f3e98c84ec925e6ad` |
| H_566 | `d6c054ff35b9b7243e4ea13622770cce0b2bf107b0f5ecf8de84af4dfab278cc` |
| H_567 | `5db8f3bc0d22ae9bfd0c05d9b0108f773636661abf7e7b01d9cce27a4b3b1f91` |
| H_568 | `88b0244979ff3341a701f1f794e9f386dbf49752fb945e5874f6d718c59e8a59` |
| H_569 | `f7a21b3a8775eaaa9c962110aef4e47a2080b754c20491a201f469cf487bd150` |
| H_570 | `47d4190d9e3f6f163671227861fd10a658c1d8e66090bd5a43b3a9bb1d36472d` |
| H_571 | `131a705e499f8031844b1588db2e92cc14a76c34ea836184fcc19fe54969b8fa` |
| H_572 | `c7b2148d29e6d1e4a952b2684f7018d0dd59b6b47843aa0faeb712e7a00316b0` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5260_1A0`, `_scratch_addimm_h50_1A0`, `_scratch_addimm_h51_1A0`, `_scratch_addimm_h52_1A0`,
`_scratch_subimm_h50_1A0`, `_scratch_subimm_h51_1A0`, `_scratch_subimm_h52_1A0`, `_scratch_ldb_5060_1A8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 23B`.. for H_565.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-75

- LDB 51 60 1A8 / LDB 52 60 1A8 (finish 1A8 LDB triad)
- ADD-IMM/SUB-IMM slot=50/51/52 imm=1A8 (start 1A8 ADD/SUB triads)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
