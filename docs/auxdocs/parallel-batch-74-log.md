# parallel-batch-74 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-74-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-079 (pin `0e5b612c…`, handlers = 571, H_557..H_564 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-079 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_564 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1A0 | `498b87000300004881c0a0010000480fb60049898790020000c3` (26) | same | same | Y | `5492824be268600b` | `5492824be268600b` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x50 imm=0x1A0 | `498b87800200004881c0a001000049898780020000c3` (22) | same | same | Y | `d6c054ff35b9b724` | `d6c054ff35b9b724` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0x1A0 | `498b87880200004881c0a001000049898788020000c3` (22) | same | same | Y | `5db8f3bc0d22ae9b` | `5db8f3bc0d22ae9b` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0x1A0 | `498b87900200004881c0a001000049898790020000c3` (22) | same | same | Y | `88b0244979ff3341` | `88b0244979ff3341` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x50 imm=0x1A0 | `498b87800200004881e8a001000049898780020000c3` (22) | same | same | Y | `f7a21b3a8775eaaa` | `f7a21b3a8775eaaa` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x51 imm=0x1A0 | `498b87880200004881e8a001000049898788020000c3` (22) | same | same | Y | `47d4190d9e3f6f16` | `47d4190d9e3f6f16` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0x1A0 | `498b87900200004881e8a001000049898790020000c3` (22) | same | same | Y | `131a705e499f8031` | `131a705e499f8031` | PASS |
| 8 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1A8 | `498b87000300004881c0a8010000480fb60049898780020000c3` (26) | same | same | Y | `c7b2148d29e6d1e4` | `c7b2148d29e6d1e4` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x52 ss=0x60 oo=0x1A0 — **PASS**

- fixture: `_scratch_ldb_5260_1A0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a0010000480fb60049898790020000c3`
- js-sha256: `5492824be268600b5039c102e6cc5b2234f3a4f2eec3d48f3e98c84ec925e6ad`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x50 imm=0x1A0 — **PASS**

- fixture: `_scratch_addimm_h50_1A0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0a001000049898780020000c3`
- js-sha256: `d6c054ff35b9b7243e4ea13622770cce0b2bf107b0f5ecf8de84af4dfab278cc`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0x1A0 — **PASS**

- fixture: `_scratch_addimm_h51_1A0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0a001000049898788020000c3`
- js-sha256: `5db8f3bc0d22ae9bfd0c05d9b0108f773636661abf7e7b01d9cce27a4b3b1f91`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0x1A0 — **PASS**

- fixture: `_scratch_addimm_h52_1A0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0a001000049898790020000c3`
- js-sha256: `88b0244979ff3341a701f1f794e9f386dbf49752fb945e5874f6d718c59e8a59`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x50 imm=0x1A0 — **PASS**

- fixture: `_scratch_subimm_h50_1A0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8a001000049898780020000c3`
- js-sha256: `f7a21b3a8775eaaa9c962110aef4e47a2080b754c20491a201f469cf487bd150`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x51 imm=0x1A0 — **PASS**

- fixture: `_scratch_subimm_h51_1A0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8a001000049898788020000c3`
- js-sha256: `47d4190d9e3f6f163671227861fd10a658c1d8e66090bd5a43b3a9bb1d36472d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0x1A0 — **PASS**

- fixture: `_scratch_subimm_h52_1A0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8a001000049898790020000c3`
- js-sha256: `131a705e499f8031844b1588db2e92cc14a76c34ea836184fcc19fe54969b8fa`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x50 ss=0x60 oo=0x1A8 — **PASS**

- fixture: `_scratch_ldb_5060_1A8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a8010000480fb60049898780020000c3`
- js-sha256: `c7b2148d29e6d1e4a952b2684f7018d0dd59b6b47843aa0faeb712e7a00316b0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=52 ss=60 oo=1A0 (finish deferred 1A0 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=1A0 (start deferred 1A0 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=1A0 (start deferred 1A0 SUB triad; imm32 22B).
- LDB dd=50 ss=60 oo=1A8 (start 1A8 LDB triad; imm32 26B; leave dd=51/52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 23B`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5260_1A0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1A0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1A0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1A0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1A0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1A0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1A0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_1A8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-74-log.md` — this file
- `scripts/_probe/parallel-batch-74-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-080 serialize PASSes + 1 Relock**

Pass pin from body-extend-079 Relock: `0e5b612c7e4882a1de87b39c35cafe0e6ccdfdc174e4f378dcd28b799de58c73`.
Handlers before consolidate = 571 (H_00..H_564). Next selectors `40 23B`.. for H_565.. if all serialize.

PASS list for body-extend-080:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_565 | 0x23B | 0x80 LDB | 0x52 0x60 0x1A0 | `498b87000300004881c0a0010000480fb60049898790020000c3` (26B) | `5492824be268600b` |
| H_566 | 0x23C | 0x62 ADD-IMM | 0x50 0x1A0 | `498b87800200004881c0a001000049898780020000c3` (22B) | `d6c054ff35b9b724` |
| H_567 | 0x23D | 0x62 ADD-IMM | 0x51 0x1A0 | `498b87880200004881c0a001000049898788020000c3` (22B) | `5db8f3bc0d22ae9b` |
| H_568 | 0x23E | 0x62 ADD-IMM | 0x52 0x1A0 | `498b87900200004881c0a001000049898790020000c3` (22B) | `88b0244979ff3341` |
| H_569 | 0x23F | 0x61 SUB-IMM | 0x50 0x1A0 | `498b87800200004881e8a001000049898780020000c3` (22B) | `f7a21b3a8775eaaa` |
| H_570 | 0x240 | 0x61 SUB-IMM | 0x51 0x1A0 | `498b87880200004881e8a001000049898788020000c3` (22B) | `47d4190d9e3f6f16` |
| H_571 | 0x241 | 0x61 SUB-IMM | 0x52 0x1A0 | `498b87900200004881e8a001000049898790020000c3` (22B) | `131a705e499f8031` |
| H_572 | 0x242 | 0x80 LDB | 0x50 0x60 0x1A8 | `498b87000300004881c0a8010000480fb60049898780020000c3` (26B) | `c7b2148d29e6d1e4` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-079 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_564.
- If the parent decides to serialize, append H_565.. at selectors `40 23B`..:
  - H_565 0x80 LDB (80 52 60 1A0) — pin `498b87000300004881c0a0010000480fb60049898790020000c3`
  - H_566 0x62 ADD-IMM (62 50 1A0) — pin `498b87800200004881c0a001000049898780020000c3`
  - H_567 0x62 ADD-IMM (62 51 1A0) — pin `498b87880200004881c0a001000049898788020000c3`
  - H_568 0x62 ADD-IMM (62 52 1A0) — pin `498b87900200004881c0a001000049898790020000c3`
  - H_569 0x61 SUB-IMM (61 50 1A0) — pin `498b87800200004881e8a001000049898780020000c3`
  - H_570 0x61 SUB-IMM (61 51 1A0) — pin `498b87880200004881e8a001000049898788020000c3`
  - H_571 0x61 SUB-IMM (61 52 1A0) — pin `498b87900200004881e8a001000049898790020000c3`
  - H_572 0x80 LDB (80 50 60 1A8) — pin `498b87000300004881c0a8010000480fb60049898780020000c3`
- Plus 1 Relock after append from pin `0e5b612c…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-080 serialize PASSes + 1 Relock
