# parallel-batch-63 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-63-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-068 (pin `2f81b43b…`, handlers = 483, H_469..H_476 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-068 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_476 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x51 imm=0x150 | `498b87880200004881c05001000049898788020000c3` (22) | same | same | Y | `f1c7dd6bfae2b6d9` | `f1c7dd6bfae2b6d9` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x52 imm=0x150 | `498b87900200004881c05001000049898790020000c3` (22) | same | same | Y | `ad7c246ef8f39fcf` | `ad7c246ef8f39fcf` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x150 | `498b87800200004881e85001000049898780020000c3` (22) | same | same | Y | `ae63f624dd2b47e7` | `ae63f624dd2b47e7` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x51 imm=0x150 | `498b87880200004881e85001000049898788020000c3` (22) | same | same | Y | `b89379b68feff397` | `b89379b68feff397` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x52 imm=0x150 | `498b87900200004881e85001000049898790020000c3` (22) | same | same | Y | `55fb7454745b2924` | `55fb7454745b2924` | PASS |
| 6 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x158 | `498b87000300004881c058010000480fb60049898780020000c3` (26) | same | same | Y | `0c2958ba1b0da5ee` | `0c2958ba1b0da5ee` | PASS |
| 7 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x158 | `498b87000300004881c058010000480fb60049898788020000c3` (26) | same | same | Y | `84dc9a2cf6fd51dc` | `84dc9a2cf6fd51dc` | PASS |
| 8 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x158 | `498b87000300004881c058010000480fb60049898790020000c3` (26) | same | same | Y | `28656b49e0c172e0` | `28656b49e0c172e0` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x51 imm=0x150 — **PASS**

- fixture: `_scratch_addimm_h51_150.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c05001000049898788020000c3`
- js-sha256: `f1c7dd6bfae2b6d912c7476955e45d2ac3cb27d63d27f1c26e1c38828d7977fa`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x52 imm=0x150 — **PASS**

- fixture: `_scratch_addimm_h52_150.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c05001000049898790020000c3`
- js-sha256: `ad7c246ef8f39fcfa54640805da6024a4a40044942606a11f57b9db4568f37f6`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x150 — **PASS**

- fixture: `_scratch_subimm_h50_150.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e85001000049898780020000c3`
- js-sha256: `ae63f624dd2b47e7cd1810c2ce208512abc1b66e0852e34b8b881f14494bc204`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x51 imm=0x150 — **PASS**

- fixture: `_scratch_subimm_h51_150.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e85001000049898788020000c3`
- js-sha256: `b89379b68feff3975d9bbaa0ef008ef66bf3d4b8d13b356213de6ada87e2ebd8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x52 imm=0x150 — **PASS**

- fixture: `_scratch_subimm_h52_150.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e85001000049898790020000c3`
- js-sha256: `55fb7454745b29240dff3640219c9007bcecba33e856f2cf629635a4c1732bc6`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x50 ss=0x60 oo=0x158 — **PASS**

- fixture: `_scratch_ldb_5060_158.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c058010000480fb60049898780020000c3`
- js-sha256: `0c2958ba1b0da5ee2d9aeec8a2db14d4b3afeec84c9b41070f0efb48452d165c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x51 ss=0x60 oo=0x158 — **PASS**

- fixture: `_scratch_ldb_5160_158.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c058010000480fb60049898788020000c3`
- js-sha256: `84dc9a2cf6fd51dcb5925778314a8c216772ac4992d0ebc6b2947be87f839eb4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x52 ss=0x60 oo=0x158 — **PASS**

- fixture: `_scratch_ldb_5260_158.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c058010000480fb60049898790020000c3`
- js-sha256: `28656b49e0c172e016076e498b32e2d0fc96787ade59805a3f8d0c298aff78c7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=51/52 imm=150 (finish 150 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=150 (start 150 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=158 (start 158 LDB triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1E3`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h51_150.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_150.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_150.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_150.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_150.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_158.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_158.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_158.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-63-log.md` — this file
- `scripts/_probe/parallel-batch-63-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-069 serialize PASSes + 1 Relock**

Pass pin from body-extend-068 Relock: `2f81b43ba9e34a3bbc786fc9d308d0cc6d38c866dfdfd8e52a51bfed15acb5b8`.
Handlers before consolidate = 483 (H_00..H_476). Next selectors `40 1E3`.. for H_477.. if all serialize.

PASS list for body-extend-069:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_477 | 0x1E3 | 0x62 ADD-IMM | 0x51 0x150 | `498b87880200004881c05001000049898788020000c3` (22B) | `f1c7dd6bfae2b6d9` |
| H_478 | 0x1E4 | 0x62 ADD-IMM | 0x52 0x150 | `498b87900200004881c05001000049898790020000c3` (22B) | `ad7c246ef8f39fcf` |
| H_479 | 0x1E5 | 0x61 SUB-IMM | 0x50 0x150 | `498b87800200004881e85001000049898780020000c3` (22B) | `ae63f624dd2b47e7` |
| H_480 | 0x1E6 | 0x61 SUB-IMM | 0x51 0x150 | `498b87880200004881e85001000049898788020000c3` (22B) | `b89379b68feff397` |
| H_481 | 0x1E7 | 0x61 SUB-IMM | 0x52 0x150 | `498b87900200004881e85001000049898790020000c3` (22B) | `55fb7454745b2924` |
| H_482 | 0x1E8 | 0x80 LDB | 0x50 0x60 0x158 | `498b87000300004881c058010000480fb60049898780020000c3` (26B) | `0c2958ba1b0da5ee` |
| H_483 | 0x1E9 | 0x80 LDB | 0x51 0x60 0x158 | `498b87000300004881c058010000480fb60049898788020000c3` (26B) | `84dc9a2cf6fd51dc` |
| H_484 | 0x1EA | 0x80 LDB | 0x52 0x60 0x158 | `498b87000300004881c058010000480fb60049898790020000c3` (26B) | `28656b49e0c172e0` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-068 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_476.
- If the parent decides to serialize, append H_477.. at selectors `40 1E3`..:
  - H_477 0x62 ADD-IMM (62 51 150) — pin `498b87880200004881c05001000049898788020000c3`
  - H_478 0x62 ADD-IMM (62 52 150) — pin `498b87900200004881c05001000049898790020000c3`
  - H_479 0x61 SUB-IMM (61 50 150) — pin `498b87800200004881e85001000049898780020000c3`
  - H_480 0x61 SUB-IMM (61 51 150) — pin `498b87880200004881e85001000049898788020000c3`
  - H_481 0x61 SUB-IMM (61 52 150) — pin `498b87900200004881e85001000049898790020000c3`
  - H_482 0x80 LDB (80 50 60 158) — pin `498b87000300004881c058010000480fb60049898780020000c3`
  - H_483 0x80 LDB (80 51 60 158) — pin `498b87000300004881c058010000480fb60049898788020000c3`
  - H_484 0x80 LDB (80 52 60 158) — pin `498b87000300004881c058010000480fb60049898790020000c3`
- Plus 1 Relock after append from pin `2f81b43b…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-069 serialize PASSes + 1 Relock
