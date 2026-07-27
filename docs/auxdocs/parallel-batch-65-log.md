# parallel-batch-65 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-65-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-070 (pin `192ba67a…`, handlers = 499, H_485..H_492 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-070 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_492 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x160 | `498b87000300004881c060010000480fb60049898790020000c3` (26) | same | same | Y | `9daf84e1a128dac3` | `9daf84e1a128dac3` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x50 imm=0x160 | `498b87800200004881c06001000049898780020000c3` (22) | same | same | Y | `3b8d32f8073e00b9` | `3b8d32f8073e00b9` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0x160 | `498b87880200004881c06001000049898788020000c3` (22) | same | same | Y | `be65ff093c4ef72d` | `be65ff093c4ef72d` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0x160 | `498b87900200004881c06001000049898790020000c3` (22) | same | same | Y | `8eae86a7c8b26fc7` | `8eae86a7c8b26fc7` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x50 imm=0x160 | `498b87800200004881e86001000049898780020000c3` (22) | same | same | Y | `cb0f44be7ee7be5e` | `cb0f44be7ee7be5e` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x51 imm=0x160 | `498b87880200004881e86001000049898788020000c3` (22) | same | same | Y | `ce408999f0330ce3` | `ce408999f0330ce3` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0x160 | `498b87900200004881e86001000049898790020000c3` (22) | same | same | Y | `17997181ac08f1e4` | `17997181ac08f1e4` | PASS |
| 8 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x168 | `498b87000300004881c068010000480fb60049898780020000c3` (26) | same | same | Y | `c6ea0ffbc5102366` | `c6ea0ffbc5102366` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x52 ss=0x60 oo=0x160 — **PASS**

- fixture: `_scratch_ldb_5260_160.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c060010000480fb60049898790020000c3`
- js-sha256: `9daf84e1a128dac3db7b56a14504d2af971c50dafb3c941b4334f18acb877680`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x50 imm=0x160 — **PASS**

- fixture: `_scratch_addimm_h50_160.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c06001000049898780020000c3`
- js-sha256: `3b8d32f8073e00b9cc1776f3ae4c7571f619edc80656d7d8c205a59d564aab8c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0x160 — **PASS**

- fixture: `_scratch_addimm_h51_160.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c06001000049898788020000c3`
- js-sha256: `be65ff093c4ef72d1b18b6a6147be3ee5eeba8a1fbc6cca22ff65f62bba230db`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0x160 — **PASS**

- fixture: `_scratch_addimm_h52_160.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c06001000049898790020000c3`
- js-sha256: `8eae86a7c8b26fc7caf8bd5b650843d1fbc19699843c66dd7490e607beec5fab`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x50 imm=0x160 — **PASS**

- fixture: `_scratch_subimm_h50_160.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e86001000049898780020000c3`
- js-sha256: `cb0f44be7ee7be5e0c5fd7f216a2875e1ff1e0a118dcdd9d05e3100de566c7d3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x51 imm=0x160 — **PASS**

- fixture: `_scratch_subimm_h51_160.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e86001000049898788020000c3`
- js-sha256: `ce408999f0330ce3d49fe066acb8919d8ce3a45d6a23d5b02cb419a7b0c22759`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0x160 — **PASS**

- fixture: `_scratch_subimm_h52_160.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e86001000049898790020000c3`
- js-sha256: `17997181ac08f1e426048cbb9445d17c75f1cf67950f99126d984c76bbfee04d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x50 ss=0x60 oo=0x168 — **PASS**

- fixture: `_scratch_ldb_5060_168.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c068010000480fb60049898780020000c3`
- js-sha256: `c6ea0ffbc5102366eebbf2593c147be7a0438ccc0d5b3c95fba3b1c3a049b41c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=52 ss=60 oo=160 (finish 160 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=160 (start 160 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=160 (start 160 SUB triad; imm32 22B).
- LDB dd=50 ss=60 oo=168 (start 168 LDB triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1F3`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5260_160.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_160.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_160.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_160.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_160.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_160.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_160.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_168.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-65-log.md` — this file
- `scripts/_probe/parallel-batch-65-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-071 serialize PASSes + 1 Relock**

Pass pin from body-extend-070 Relock: `192ba67ac8bb814df865a108032dd1e9301c93c4e3fc89f44c8c4edfaf84791f`.
Handlers before consolidate = 499 (H_00..H_492). Next selectors `40 1F3`.. for H_493.. if all serialize.

PASS list for body-extend-071:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_493 | 0x1F3 | 0x80 LDB | 0x52 0x60 0x160 | `498b87000300004881c060010000480fb60049898790020000c3` (26B) | `9daf84e1a128dac3` |
| H_494 | 0x1F4 | 0x62 ADD-IMM | 0x50 0x160 | `498b87800200004881c06001000049898780020000c3` (22B) | `3b8d32f8073e00b9` |
| H_495 | 0x1F5 | 0x62 ADD-IMM | 0x51 0x160 | `498b87880200004881c06001000049898788020000c3` (22B) | `be65ff093c4ef72d` |
| H_496 | 0x1F6 | 0x62 ADD-IMM | 0x52 0x160 | `498b87900200004881c06001000049898790020000c3` (22B) | `8eae86a7c8b26fc7` |
| H_497 | 0x1F7 | 0x61 SUB-IMM | 0x50 0x160 | `498b87800200004881e86001000049898780020000c3` (22B) | `cb0f44be7ee7be5e` |
| H_498 | 0x1F8 | 0x61 SUB-IMM | 0x51 0x160 | `498b87880200004881e86001000049898788020000c3` (22B) | `ce408999f0330ce3` |
| H_499 | 0x1F9 | 0x61 SUB-IMM | 0x52 0x160 | `498b87900200004881e86001000049898790020000c3` (22B) | `17997181ac08f1e4` |
| H_500 | 0x1FA | 0x80 LDB | 0x50 0x60 0x168 | `498b87000300004881c068010000480fb60049898780020000c3` (26B) | `c6ea0ffbc5102366` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-070 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_492.
- If the parent decides to serialize, append H_493.. at selectors `40 1F3`..:
  - H_493 0x80 LDB (80 52 60 160) — pin `498b87000300004881c060010000480fb60049898790020000c3`
  - H_494 0x62 ADD-IMM (62 50 160) — pin `498b87800200004881c06001000049898780020000c3`
  - H_495 0x62 ADD-IMM (62 51 160) — pin `498b87880200004881c06001000049898788020000c3`
  - H_496 0x62 ADD-IMM (62 52 160) — pin `498b87900200004881c06001000049898790020000c3`
  - H_497 0x61 SUB-IMM (61 50 160) — pin `498b87800200004881e86001000049898780020000c3`
  - H_498 0x61 SUB-IMM (61 51 160) — pin `498b87880200004881e86001000049898788020000c3`
  - H_499 0x61 SUB-IMM (61 52 160) — pin `498b87900200004881e86001000049898790020000c3`
  - H_500 0x80 LDB (80 50 60 168) — pin `498b87000300004881c068010000480fb60049898780020000c3`
- Plus 1 Relock after append from pin `192ba67a…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-071 serialize PASSes + 1 Relock
