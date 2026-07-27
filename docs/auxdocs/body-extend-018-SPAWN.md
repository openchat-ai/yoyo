# body-extend-018 SPAWN · consolidate parallel-batch-12

> Continuous queue handoff from parallel-batch-12 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `d1d92927a66b19ae2ca5b8f13861a58b956da81a969944943c0d68f03104986c` (abbrev `d1d92927…`).
> Handlers ≈ 76 (H_00..H_69). Last selectors: 0x44..0x4B = H_62..H_69.
> Source: `docs/auxdocs/parallel-batch-12-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-017-log.md`.

## Task: body-extend-018 (serialize + Relock)

Mirror body-extend-017 / body-extend-016 protocol:

1. Hand-author append H_70..H_77 to `yoyo/projects/yoyo.ty` at selectors `40 4C` .. `40 53`.
2. Promote fixtures from `_scratch_{subimm_h51,dec_h52,inc_h52,orv_5052,subv_5052,get_5251,set_f00dbabe,cmp_5250}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect 60→68 JS, 67→75 Rust).
4. Verify + Relock once chaining from `d1d92927a66b19ae…`.
5. DDC via `verify-selfhost.ps1`.
6. Write `docs/auxdocs/body-extend-018-log.md`.
7. Auto-spawn parallel-batch-13 scratch-only (continuous queue).

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_70 | 0x4C | 0x61 SUB-IMM | 51 03 | `498b87880200004883e80349898788020000c3` (19B) | `ad41505ee5509528` |
| H_71 | 0x4D | 0x67 DEC | 52 | `498b879002000048ffc849898790020000c3` (18B) | `1042c1dcf85cddf2` |
| H_72 | 0x4E | 0x66 INC | 52 | `498b879002000048ffc049898790020000c3` (18B) | `b5913485423d3a9b` |
| H_73 | 0x4F | 0x69 ORV | 50 52 | `498b8780020000498b8f900200004809c849898780020000c3` (25B) | `27b0f48ef4d8f0cd` |
| H_74 | 0x50 | 0x6A SUBV | 50 52 | `498b8780020000498b8f900200004829c849898780020000c3` (25B) | `457b792b23dd64d2` |
| H_75 | 0x51 | 0x60 GET | 52 51 | `498b878802000049898790020000c3` (15B) | `a247d06b13b6b12f` |
| H_76 | 0x52 | 0x30 SET | 50 F00DBABE | `48b8beba0df00000000049898780020000c3` (18B) | `107c6ec772518411` |
| H_77 | 0x53 | 0x65 CMP | 52 50 | `498b8790020000498b8f800200004839c8c3` (18B) | `616114e143a02b80` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h51`, `_scratch_dec_h52`, `_scratch_inc_h52`, `_scratch_orv_5052`,
`_scratch_subv_5052`, `_scratch_get_5251`, `_scratch_set_f00dbabe`, `_scratch_cmp_5250`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY 0x84/0x85.
