# body-extend-101 SPAWN · consolidate parallel-batch-95

> Continuous queue handoff from parallel-batch-95 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `7c07906496a7af9cbaec74b5590ec3677117ced6c36241823bd69b6a4ff1ae51` (abbrev `7c079064…`).
> Handlers = 739 (H_00..H_732). Last selectors: 0x2DB..0x2E2 = H_725..H_732 (`40 2DB`..`40 2E2` via label-width A).
> Source: `docs/auxdocs/parallel-batch-95-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-100-log.md` / `docs/auxdocs/body-extend-100-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-100 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green. DDC may EQUAL or DIFFER next beat.
> Next selectors: `40 2E3`.. for H_733.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 2E3`/`40 2EA` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body; do NOT emit D-1 opcodes.

## Queue control

See `docs/auxdocs/body-extend-queue.md`: scratch pool ≤8 concurrent; **one** consolidator Relock; **AUTO-STOP** after Relock if handlers ≥ 800 (do not auto-spawn parallel-batch-96). This beat 739→747 if all 8 land — continue queue after Relock.

## Task: body-extend-101 (serialize + Relock)

Mirror body-extend-100 / body-extend-099 protocol:

1. Hand-author append H_733..H_740 to `yoyo/projects/yoyo.ty` at selectors `40 2E3` .. `40 2EA` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h52_230,ldb_5060_232,ldb_5160_232,ldb_5260_232,addimm_h50_232,addimm_h51_232,addimm_h52_232,subimm_h50_232}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `7c07906496a7af9cbaec74b5590ec3677117ced6c36241823bd69b6a4ff1ae51`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize EQUAL or DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-101-log.md`.
7. If handlers after Relock **≥ 800**: AUTO-STOP（见 queue 控制面）；else auto-spawn parallel-batch-96 scratch-only, or write `docs/auxdocs/parallel-batch-96-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_733 | 0x2E3 | 0x61 SUB-IMM | 52 230 | `498b87900200004881e83002000049898790020000c3` (22B) | `5a2ce924b1a66050` |
| H_734 | 0x2E4 | 0x80 LDB | 50 60 232 | `498b87000300004881c032020000480fb60049898780020000c3` (26B) | `2c8b3aa576062c39` |
| H_735 | 0x2E5 | 0x80 LDB | 51 60 232 | `498b87000300004881c032020000480fb60049898788020000c3` (26B) | `d935a5d3f24953e7` |
| H_736 | 0x2E6 | 0x80 LDB | 52 60 232 | `498b87000300004881c032020000480fb60049898790020000c3` (26B) | `1d9a2681b4fac7a1` |
| H_737 | 0x2E7 | 0x62 ADD-IMM | 50 232 | `498b87800200004881c03202000049898780020000c3` (22B) | `da80cde8ed742a1c` |
| H_738 | 0x2E8 | 0x62 ADD-IMM | 51 232 | `498b87880200004881c03202000049898788020000c3` (22B) | `4aa3b5563616b6a6` |
| H_739 | 0x2E9 | 0x62 ADD-IMM | 52 232 | `498b87900200004881c03202000049898790020000c3` (22B) | `f9199c6bd9783045` |
| H_740 | 0x2EA | 0x61 SUB-IMM | 50 232 | `498b87800200004881e83202000049898780020000c3` (22B) | `922bcb642443cdc9` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x232 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x230/0x232 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x232 uses imm32 add (`48 81 c0`) → 26B pins.
SUB-IMM slot=52 imm=230 finishes deferred 230 SUB triad (H_733).
LDB dd=50/51/52 ss=60 oo=232 starts 232 LDB ladder (H_734/H_735/H_736).
ADD-IMM slot=50/51/52 imm=232 starts 232 ADD triad (H_737/H_738/H_739).
SUB-IMM slot=50 imm=232 starts 232 SUB triad (H_740; SUB 51/52 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_733 | `5a2ce924b1a66050cd8317c147e86ed49e8277cc463083ae9a8c0eb691989b89` |
| H_734 | `2c8b3aa576062c3900b06a28cef3c8d5505960f829c94454bb8154c3e33eccf3` |
| H_735 | `d935a5d3f24953e7037800a6a859243d8d5e12c711fd4ea0105a13617016acb2` |
| H_736 | `1d9a2681b4fac7a1dfc3d43209e67426fda3041dee1bcfc1c51f3433838f73da` |
| H_737 | `da80cde8ed742a1c98a87f2e0e0c0f69e62d1cf12b4cb73fe03d51b8c2a2e3eb` |
| H_738 | `4aa3b5563616b6a6dbbab36788b7117488fc12b67a0cc851ddb3cccc6a4671cd` |
| H_739 | `f9199c6bd9783045ccd6c049dbc65401650062d1a850a6269d18a6fc35617d89` |
| H_740 | `922bcb642443cdc9d80af6a993395e5d90bb69b7b4e59539c7b5dd327a22ce2a` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h52_230`, `_scratch_ldb_5060_232`, `_scratch_ldb_5160_232`, `_scratch_ldb_5260_232`,
`_scratch_addimm_h50_232`, `_scratch_addimm_h51_232`, `_scratch_addimm_h52_232`, `_scratch_subimm_h50_232`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 2E3`.. for H_733.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-96

- SUB-IMM slot=51 imm=232 · SUB-IMM slot=52 imm=232 (finish 232 SUB triad)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
- Next ladder if continuing (e.g. LDB/ADD-IMM/SUB-IMM beyond 232) — only fresh, not duplicates
