# parallel-batch-89 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-89-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-094 (pin `0ef9611b…`, handlers = 691, H_677..H_684 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-094 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_684 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x52 imm=0x208 | `498b87900200004881c00802000049898790020000c3` (22) | same | same | Y | `bb7306a6accdaf1d` | `bb7306a6accdaf1d` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x50 imm=0x208 | `498b87800200004881e80802000049898780020000c3` (22) | same | same | Y | `f7711234e1f246db` | `f7711234e1f246db` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x51 imm=0x208 | `498b87880200004881e80802000049898788020000c3` (22) | same | same | Y | `71f14163af6727da` | `71f14163af6727da` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x52 imm=0x208 | `498b87900200004881e80802000049898790020000c3` (22) | same | same | Y | `b95b3672e4031732` | `b95b3672e4031732` | PASS |
| 5 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x210 | `498b87000300004881c010020000480fb60049898780020000c3` (26) | same | same | Y | `e5d730581fb17e84` | `e5d730581fb17e84` | PASS |
| 6 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x210 | `498b87000300004881c010020000480fb60049898788020000c3` (26) | same | same | Y | `ebbb4b6905b61aa1` | `ebbb4b6905b61aa1` | PASS |
| 7 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x210 | `498b87000300004881c010020000480fb60049898790020000c3` (26) | same | same | Y | `62a53f91d97addee` | `62a53f91d97addee` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x50 imm=0x210 | `498b87800200004881c01002000049898780020000c3` (22) | same | same | Y | `b28afba882b0e6c1` | `b28afba882b0e6c1` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x52 imm=0x208 — **PASS**

- fixture: `_scratch_addimm_h52_208.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c00802000049898790020000c3`
- js-sha256: `bb7306a6accdaf1dd37bfa4c5811e5bed548e84ddf0e5182504d0a09bce5d0e2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x50 imm=0x208 — **PASS**

- fixture: `_scratch_subimm_h50_208.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e80802000049898780020000c3`
- js-sha256: `f7711234e1f246db5dde5d4b6bbd3c11b32f6a77fe2f884fd2dfcb6249890718`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x51 imm=0x208 — **PASS**

- fixture: `_scratch_subimm_h51_208.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e80802000049898788020000c3`
- js-sha256: `71f14163af6727da4a31441ed23a21bb6a633eee6a4bfadd8308a0dc0ace6137`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x52 imm=0x208 — **PASS**

- fixture: `_scratch_subimm_h52_208.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e80802000049898790020000c3`
- js-sha256: `b95b3672e4031732805ad8afb821479b85f2377415fd62a42e632eb19bce70ef`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x50 ss=0x60 oo=0x210 — **PASS**

- fixture: `_scratch_ldb_5060_210.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c010020000480fb60049898780020000c3`
- js-sha256: `e5d730581fb17e8481d5891459d7eecc79083d0cc1c554c6459d1e1b4c589e17`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x51 ss=0x60 oo=0x210 — **PASS**

- fixture: `_scratch_ldb_5160_210.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c010020000480fb60049898788020000c3`
- js-sha256: `ebbb4b6905b61aa16c3a1d05a370e9e1532f824bdbb7b3af8c9c52e360b0d3b9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x52 ss=0x60 oo=0x210 — **PASS**

- fixture: `_scratch_ldb_5260_210.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c010020000480fb60049898790020000c3`
- js-sha256: `62a53f91d97addee0e04e77ad9d5f2f3a6de52ce7176a891db4a7e41903cbe04`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x50 imm=0x210 — **PASS**

- fixture: `_scratch_addimm_h50_210.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c01002000049898780020000c3`
- js-sha256: `b28afba882b0e6c1af6d419af5ec0fb99881e922e186e75ea167ec65edfeb6d0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=52 imm=208 (finish deferred 208 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=208 (start 208 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=210 (start 210 LDB triad; imm32 26B).
- ADD-IMM slot=50 imm=210 (start 210 ADD triad; imm32 22B).
- ADD-IMM slot=51/52 imm=210 deferred to next batch.
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 2B3`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h52_208.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_208.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_208.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_208.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_210.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_210.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_210.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_210.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-89-log.md` — this file
- `scripts/_probe/parallel-batch-89-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-095 serialize PASSes + 1 Relock**

Pass pin from body-extend-094 Relock: `0ef9611b50021d82d2c7870a29d1d4107164b7a3c586f41f5271a083fbdfec51`.
Handlers before consolidate = 691 (H_00..H_684). Next selectors `40 2B3`.. for H_685.. if all serialize.

PASS list for body-extend-095:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_685 | 0x2B3 | 0x62 ADD-IMM | 0x52 0x208 | `498b87900200004881c00802000049898790020000c3` (22B) | `bb7306a6accdaf1d` |
| H_686 | 0x2B4 | 0x61 SUB-IMM | 0x50 0x208 | `498b87800200004881e80802000049898780020000c3` (22B) | `f7711234e1f246db` |
| H_687 | 0x2B5 | 0x61 SUB-IMM | 0x51 0x208 | `498b87880200004881e80802000049898788020000c3` (22B) | `71f14163af6727da` |
| H_688 | 0x2B6 | 0x61 SUB-IMM | 0x52 0x208 | `498b87900200004881e80802000049898790020000c3` (22B) | `b95b3672e4031732` |
| H_689 | 0x2B7 | 0x80 LDB | 0x50 0x60 0x210 | `498b87000300004881c010020000480fb60049898780020000c3` (26B) | `e5d730581fb17e84` |
| H_690 | 0x2B8 | 0x80 LDB | 0x51 0x60 0x210 | `498b87000300004881c010020000480fb60049898788020000c3` (26B) | `ebbb4b6905b61aa1` |
| H_691 | 0x2B9 | 0x80 LDB | 0x52 0x60 0x210 | `498b87000300004881c010020000480fb60049898790020000c3` (26B) | `62a53f91d97addee` |
| H_692 | 0x2BA | 0x62 ADD-IMM | 0x50 0x210 | `498b87800200004881c01002000049898780020000c3` (22B) | `b28afba882b0e6c1` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-094 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_684.
- If the parent decides to serialize, append H_685.. at selectors `40 2B3`..:
  - H_685 0x62 ADD-IMM (62 52 208) — pin `498b87900200004881c00802000049898790020000c3`
  - H_686 0x61 SUB-IMM (61 50 208) — pin `498b87800200004881e80802000049898780020000c3`
  - H_687 0x61 SUB-IMM (61 51 208) — pin `498b87880200004881e80802000049898788020000c3`
  - H_688 0x61 SUB-IMM (61 52 208) — pin `498b87900200004881e80802000049898790020000c3`
  - H_689 0x80 LDB (80 50 60 210) — pin `498b87000300004881c010020000480fb60049898780020000c3`
  - H_690 0x80 LDB (80 51 60 210) — pin `498b87000300004881c010020000480fb60049898788020000c3`
  - H_691 0x80 LDB (80 52 60 210) — pin `498b87000300004881c010020000480fb60049898790020000c3`
  - H_692 0x62 ADD-IMM (62 50 210) — pin `498b87800200004881c01002000049898780020000c3`
- Plus 1 Relock after append from pin `0ef9611b…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: ADD-IMM slot=51/52 imm=210; finish 210 ADD/SUB ladder.

## §7. Consolidation handoff

parent next = body-extend-095 serialize PASSes + 1 Relock
