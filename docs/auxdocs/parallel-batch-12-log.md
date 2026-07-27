# parallel-batch-12 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-12-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-017 (pin `d1d92927…`, handlers ≈ 76, H_62..H_69 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_69 and
> not already present as handlers in current `yoyo.ty`. Slot/imm/dst
> variations of SUB-IMM/DEC/INC/ORV/SUBV/GET/SET/CMP. Skipped D-1
> 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x51 imm=0x03 | `498b87880200004883e80349898788020000c3` (19) | same | same | Y | `ad41505ee5509528` | `ad41505ee5509528` | PASS |
| 2 | 0x67 DEC | slot=0x52 | `498b879002000048ffc849898790020000c3` (18) | same | same | Y | `1042c1dcf85cddf2` | `1042c1dcf85cddf2` | PASS |
| 3 | 0x66 INC | slot=0x52 | `498b879002000048ffc049898790020000c3` (18) | same | same | Y | `b5913485423d3a9b` | `b5913485423d3a9b` | PASS |
| 4 | 0x69 ORV | (0x50, 0x52) | `498b8780020000498b8f900200004809c849898780020000c3` (25) | same | same | Y | `27b0f48ef4d8f0cd` | `27b0f48ef4d8f0cd` | PASS |
| 5 | 0x6A SUBV | (0x50, 0x52) | `498b8780020000498b8f900200004829c849898780020000c3` (25) | same | same | Y | `457b792b23dd64d2` | `457b792b23dd64d2` | PASS |
| 6 | 0x60 GET | (0x52, 0x51) | `498b878802000049898790020000c3` (15) | same | same | Y | `a247d06b13b6b12f` | `a247d06b13b6b12f` | PASS |
| 7 | 0x30 SET | slot=0x50 imm=0xF00DBABE | `48b8beba0df00000000049898780020000c3` (18) | same | same | Y | `107c6ec772518411` | `107c6ec772518411` | PASS |
| 8 | 0x65 CMP | (0x52, 0x50) | `498b8790020000498b8f800200004839c8c3` (18) | same | same | Y | `616114e143a02b80` | `616114e143a02b80` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §2. Pick rationale

- SUB-IMM at slot 0x51 imm=0x03 (canonical uses slot 0x50; not in H_62..H_69).
- DEC/INC at slot 0x52 (H_62/H_63 use 0x51; H_11/H_12 use 0x50).
- ORV/SUBV at 50 52 (canonical pairs differ; ADDV-5052 is H_66 but ORV/SUBV-5052 absent).
- GET at 52 51 (H_39/H_51/H_59/H_67 cover other pairs).
- SET at slot 0x50 imm=0xF00DBABE (distinct from CAFEBABE/DEADBEEF/12345678).
- CMP at 52 50 (H_36/H_58/H_65 cover other pairs).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h51.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_dec_h52.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_inc_h52.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_orv_5052.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subv_5052.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_get_5251.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_f00dbabe.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_cmp_5250.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-12-log.md` — this file
- `scripts/_probe/parallel-batch-12-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-018 serialize PASSes + 1 Relock**

Pass pin from body-extend-017 Relock: `d1d92927a66b19ae2ca5b8f13861a58b956da81a969944943c0d68f03104986c`.
Handlers before consolidate ≈ 76 (H_00..H_69). Next selectors 0x4C.. for H_70.. if all serialize.

PASS list for body-extend-018:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_70 | 0x4C | 0x61 SUB-IMM | 0x51 0x03 | `498b87880200004883e80349898788020000c3` (19B) | `ad41505ee5509528` |
| H_71 | 0x4D | 0x67 DEC | 0x52 | `498b879002000048ffc849898790020000c3` (18B) | `1042c1dcf85cddf2` |
| H_72 | 0x4E | 0x66 INC | 0x52 | `498b879002000048ffc049898790020000c3` (18B) | `b5913485423d3a9b` |
| H_73 | 0x4F | 0x69 ORV | 0x50, 0x52 | `498b8780020000498b8f900200004809c849898780020000c3` (25B) | `27b0f48ef4d8f0cd` |
| H_74 | 0x50 | 0x6A SUBV | 0x50, 0x52 | `498b8780020000498b8f900200004829c849898780020000c3` (25B) | `457b792b23dd64d2` |
| H_75 | 0x51 | 0x60 GET | 0x52, 0x51 | `498b878802000049898790020000c3` (15B) | `a247d06b13b6b12f` |
| H_76 | 0x52 | 0x30 SET | 0x50 0xF00DBABE | `48b8beba0df00000000049898780020000c3` (18B) | `107c6ec772518411` |
| H_77 | 0x53 | 0x65 CMP | 0x52, 0x50 | `498b8790020000498b8f800200004839c8c3` (18B) | `616114e143a02b80` |
