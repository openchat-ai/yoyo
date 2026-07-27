# parallel-batch-57 Log · 7-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-57-EXPERIMENTAL-7-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-062 (pin `c5b95f37…`, handlers = 436, H_422..H_429 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-062 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 7-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_429 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x128 | `498b87000300004881c028010000480fb60049898790020000c3` (26) | same | same | Y | `6468bf9a05c742b4` | `6468bf9a05c742b4` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x50 imm=0x128 | `498b87800200004881c02801000049898780020000c3` (22) | same | same | Y | `e8b7b5eb74790fbc` | `e8b7b5eb74790fbc` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0x128 | `498b87880200004881c02801000049898788020000c3` (22) | same | same | Y | `f19522688ae984fb` | `f19522688ae984fb` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0x128 | `498b87900200004881c02801000049898790020000c3` (22) | same | same | Y | `ba685e27eb2e7e2b` | `ba685e27eb2e7e2b` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x50 imm=0x128 | `498b87800200004881e82801000049898780020000c3` (22) | same | same | Y | `d2dc131f67b41898` | `d2dc131f67b41898` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x51 imm=0x128 | `498b87880200004881e82801000049898788020000c3` (22) | same | same | Y | `0327f33cd15c5c5a` | `0327f33cd15c5c5a` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0x128 | `498b87900200004881e82801000049898790020000c3` (22) | same | same | Y | `d6207001a19bc3e5` | `d6207001a19bc3e5` | PASS |

**Summary**: 7 PASS / 0 REJECT out of 7 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x52 ss=0x60 oo=0x128 — **PASS**

- fixture: `_scratch_ldb_5260_128.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c028010000480fb60049898790020000c3`
- js-sha256: `6468bf9a05c742b4a8324439fa582bdf0a572b156108c02fc99ac75c43db4fe9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x50 imm=0x128 — **PASS**

- fixture: `_scratch_addimm_h50_128.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c02801000049898780020000c3`
- js-sha256: `e8b7b5eb74790fbc210574195020a357d9c93c4a87b6e703f455cef5a0dc024a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0x128 — **PASS**

- fixture: `_scratch_addimm_h51_128.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c02801000049898788020000c3`
- js-sha256: `f19522688ae984fb32b06787291b6a469efc321a36b92e121d746e3930b5dc6d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0x128 — **PASS**

- fixture: `_scratch_addimm_h52_128.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c02801000049898790020000c3`
- js-sha256: `ba685e27eb2e7e2b366d311ebd56e65f2183ce16f8ffae11da8a5037f28f6ea9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x50 imm=0x128 — **PASS**

- fixture: `_scratch_subimm_h50_128.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e82801000049898780020000c3`
- js-sha256: `d2dc131f67b41898d008cc80a3291d284f2332039159892e5a4724b51a88c01f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x51 imm=0x128 — **PASS**

- fixture: `_scratch_subimm_h51_128.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e82801000049898788020000c3`
- js-sha256: `0327f33cd15c5c5afa65b2e3504e6f017f2ceb1788582bdccb2c0f9328a8ac3f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0x128 — **PASS**

- fixture: `_scratch_subimm_h52_128.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e82801000049898790020000c3`
- js-sha256: `d6207001a19bc3e5dc2b8e6f1830add95a4cf18fa617daf57e799eafb35db756`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=52 ss=60 oo=128 (finish 128 LDB triad; H_428/H_429=50/51; imm32 26B).
- ADD-IMM slot=50/51/52 imm=128 (start 128 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=128 (start 128 SUB triad; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1B4`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5260_128.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_128.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_128.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_128.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_128.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_128.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_128.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-57-log.md` — this file
- `scripts/_probe/parallel-batch-57-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-063 serialize PASSes + 1 Relock**

Pass pin from body-extend-062 Relock: `c5b95f3792afa572a774aa41d22dd49fb27b6905aa7ab891273b77db49a3af0a`.
Handlers before consolidate = 436 (H_00..H_429). Next selectors `40 1B4`.. for H_430.. if all serialize.

PASS list for body-extend-063:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_430 | 0x1B4 | 0x80 LDB | 0x52 0x60 0x128 | `498b87000300004881c028010000480fb60049898790020000c3` (26B) | `6468bf9a05c742b4` |
| H_431 | 0x1B5 | 0x62 ADD-IMM | 0x50 0x128 | `498b87800200004881c02801000049898780020000c3` (22B) | `e8b7b5eb74790fbc` |
| H_432 | 0x1B6 | 0x62 ADD-IMM | 0x51 0x128 | `498b87880200004881c02801000049898788020000c3` (22B) | `f19522688ae984fb` |
| H_433 | 0x1B7 | 0x62 ADD-IMM | 0x52 0x128 | `498b87900200004881c02801000049898790020000c3` (22B) | `ba685e27eb2e7e2b` |
| H_434 | 0x1B8 | 0x61 SUB-IMM | 0x50 0x128 | `498b87800200004881e82801000049898780020000c3` (22B) | `d2dc131f67b41898` |
| H_435 | 0x1B9 | 0x61 SUB-IMM | 0x51 0x128 | `498b87880200004881e82801000049898788020000c3` (22B) | `0327f33cd15c5c5a` |
| H_436 | 0x1BA | 0x61 SUB-IMM | 0x52 0x128 | `498b87900200004881e82801000049898790020000c3` (22B) | `d6207001a19bc3e5` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 7 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-062 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 7 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_429.
- If the parent decides to serialize, append H_430.. at selectors `40 1B4`..:
  - H_430 0x80 LDB (80 52 60 128) — pin `498b87000300004881c028010000480fb60049898790020000c3`
  - H_431 0x62 ADD-IMM (62 50 128) — pin `498b87800200004881c02801000049898780020000c3`
  - H_432 0x62 ADD-IMM (62 51 128) — pin `498b87880200004881c02801000049898788020000c3`
  - H_433 0x62 ADD-IMM (62 52 128) — pin `498b87900200004881c02801000049898790020000c3`
  - H_434 0x61 SUB-IMM (61 50 128) — pin `498b87800200004881e82801000049898780020000c3`
  - H_435 0x61 SUB-IMM (61 51 128) — pin `498b87880200004881e82801000049898788020000c3`
  - H_436 0x61 SUB-IMM (61 52 128) — pin `498b87900200004881e82801000049898790020000c3`
- Plus 1 Relock after append from pin `c5b95f37…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-063 serialize PASSes + 1 Relock
