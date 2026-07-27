# parallel-batch-62 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-62-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-067 (pin `deaf4013…`, handlers = 475, H_461..H_468 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-067 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_468 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x52 imm=0x148 | `498b87900200004881c04801000049898790020000c3` (22) | same | same | Y | `e5c549e3bb998799` | `e5c549e3bb998799` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x50 imm=0x148 | `498b87800200004881e84801000049898780020000c3` (22) | same | same | Y | `4310d24ed1a65b24` | `4310d24ed1a65b24` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x51 imm=0x148 | `498b87880200004881e84801000049898788020000c3` (22) | same | same | Y | `20c893f5b357112c` | `20c893f5b357112c` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x52 imm=0x148 | `498b87900200004881e84801000049898790020000c3` (22) | same | same | Y | `7b21e0e79d618564` | `7b21e0e79d618564` | PASS |
| 5 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x150 | `498b87000300004881c050010000480fb60049898780020000c3` (26) | same | same | Y | `a2f4d32aedf227d7` | `a2f4d32aedf227d7` | PASS |
| 6 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x150 | `498b87000300004881c050010000480fb60049898788020000c3` (26) | same | same | Y | `eebeaa9843e6b88f` | `eebeaa9843e6b88f` | PASS |
| 7 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x150 | `498b87000300004881c050010000480fb60049898790020000c3` (26) | same | same | Y | `34288a223e426de8` | `34288a223e426de8` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x50 imm=0x150 | `498b87800200004881c05001000049898780020000c3` (22) | same | same | Y | `62f0518dcdd6f717` | `62f0518dcdd6f717` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x52 imm=0x148 — **PASS**

- fixture: `_scratch_addimm_h52_148.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c04801000049898790020000c3`
- js-sha256: `e5c549e3bb9987999d6ffe4c84244f5291cfb5648c3d6b7e4096ce3656b58f15`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x50 imm=0x148 — **PASS**

- fixture: `_scratch_subimm_h50_148.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e84801000049898780020000c3`
- js-sha256: `4310d24ed1a65b24f4058f1ae7fa401d3ae961311efeddc498f48b7c0301018d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x51 imm=0x148 — **PASS**

- fixture: `_scratch_subimm_h51_148.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e84801000049898788020000c3`
- js-sha256: `20c893f5b357112c74292a5b6e248e47cce9f61d3bfe6796f6d98b78706c8228`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x52 imm=0x148 — **PASS**

- fixture: `_scratch_subimm_h52_148.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e84801000049898790020000c3`
- js-sha256: `7b21e0e79d61856485ee0371d9bf4ce4034ce96a933115f21a251a9993675d35`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x50 ss=0x60 oo=0x150 — **PASS**

- fixture: `_scratch_ldb_5060_150.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c050010000480fb60049898780020000c3`
- js-sha256: `a2f4d32aedf227d7e2d30ed001b3bd48bd520b742b9541b13b5798eb1abc43d2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x51 ss=0x60 oo=0x150 — **PASS**

- fixture: `_scratch_ldb_5160_150.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c050010000480fb60049898788020000c3`
- js-sha256: `eebeaa9843e6b88f062fabea58ce3413490e91045254351aff70c176a9481353`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x52 ss=0x60 oo=0x150 — **PASS**

- fixture: `_scratch_ldb_5260_150.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c050010000480fb60049898790020000c3`
- js-sha256: `34288a223e426de8456c6b5f4645e3b87c5c665600a86f1fc69a3ce44b49dab2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x50 imm=0x150 — **PASS**

- fixture: `_scratch_addimm_h50_150.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c05001000049898780020000c3`
- js-sha256: `62f0518dcdd6f7174c229cea63a6247f5d79a5e2fe97acdd544ce8fd81270a4c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=52 imm=148 (finish 148 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=148 (start 148 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=150 (start 150 LDB triad; imm32 26B).
- ADD-IMM slot=50 imm=150 (start 150 ADD triad; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1DB`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h52_148.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_148.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_148.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_148.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_150.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_150.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_150.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_150.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-62-log.md` — this file
- `scripts/_probe/parallel-batch-62-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-068 serialize PASSes + 1 Relock**

Pass pin from body-extend-067 Relock: `deaf40134394a58d9e81fd3a8f55c4ec9110fc93ad8d366e547f0628144dd098`.
Handlers before consolidate = 475 (H_00..H_468). Next selectors `40 1DB`.. for H_469.. if all serialize.

PASS list for body-extend-068:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_469 | 0x1DB | 0x62 ADD-IMM | 0x52 0x148 | `498b87900200004881c04801000049898790020000c3` (22B) | `e5c549e3bb998799` |
| H_470 | 0x1DC | 0x61 SUB-IMM | 0x50 0x148 | `498b87800200004881e84801000049898780020000c3` (22B) | `4310d24ed1a65b24` |
| H_471 | 0x1DD | 0x61 SUB-IMM | 0x51 0x148 | `498b87880200004881e84801000049898788020000c3` (22B) | `20c893f5b357112c` |
| H_472 | 0x1DE | 0x61 SUB-IMM | 0x52 0x148 | `498b87900200004881e84801000049898790020000c3` (22B) | `7b21e0e79d618564` |
| H_473 | 0x1DF | 0x80 LDB | 0x50 0x60 0x150 | `498b87000300004881c050010000480fb60049898780020000c3` (26B) | `a2f4d32aedf227d7` |
| H_474 | 0x1E0 | 0x80 LDB | 0x51 0x60 0x150 | `498b87000300004881c050010000480fb60049898788020000c3` (26B) | `eebeaa9843e6b88f` |
| H_475 | 0x1E1 | 0x80 LDB | 0x52 0x60 0x150 | `498b87000300004881c050010000480fb60049898790020000c3` (26B) | `34288a223e426de8` |
| H_476 | 0x1E2 | 0x62 ADD-IMM | 0x50 0x150 | `498b87800200004881c05001000049898780020000c3` (22B) | `62f0518dcdd6f717` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-067 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_468.
- If the parent decides to serialize, append H_469.. at selectors `40 1DB`..:
  - H_469 0x62 ADD-IMM (62 52 148) — pin `498b87900200004881c04801000049898790020000c3`
  - H_470 0x61 SUB-IMM (61 50 148) — pin `498b87800200004881e84801000049898780020000c3`
  - H_471 0x61 SUB-IMM (61 51 148) — pin `498b87880200004881e84801000049898788020000c3`
  - H_472 0x61 SUB-IMM (61 52 148) — pin `498b87900200004881e84801000049898790020000c3`
  - H_473 0x80 LDB (80 50 60 150) — pin `498b87000300004881c050010000480fb60049898780020000c3`
  - H_474 0x80 LDB (80 51 60 150) — pin `498b87000300004881c050010000480fb60049898788020000c3`
  - H_475 0x80 LDB (80 52 60 150) — pin `498b87000300004881c050010000480fb60049898790020000c3`
  - H_476 0x62 ADD-IMM (62 50 150) — pin `498b87800200004881c05001000049898780020000c3`
- Plus 1 Relock after append from pin `deaf4013…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-068 serialize PASSes + 1 Relock
