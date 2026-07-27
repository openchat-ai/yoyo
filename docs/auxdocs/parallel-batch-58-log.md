# parallel-batch-58 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-58-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-063 (pin `f4fa77a5…`, handlers = 443, H_430..H_436 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-063 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_436 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x130 | `498b87000300004881c030010000480fb60049898780020000c3` (26) | same | same | Y | `31e8129afecd8ba8` | `31e8129afecd8ba8` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x130 | `498b87000300004881c030010000480fb60049898788020000c3` (26) | same | same | Y | `d0ba625ab36e77ee` | `d0ba625ab36e77ee` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x130 | `498b87000300004881c030010000480fb60049898790020000c3` (26) | same | same | Y | `addb80d146c8758b` | `addb80d146c8758b` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=0x130 | `498b87800200004881c03001000049898780020000c3` (22) | same | same | Y | `dd2d08fe3b6bdad6` | `dd2d08fe3b6bdad6` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x51 imm=0x130 | `498b87880200004881c03001000049898788020000c3` (22) | same | same | Y | `e7e0160df815fc7d` | `e7e0160df815fc7d` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0x130 | `498b87900200004881c03001000049898790020000c3` (22) | same | same | Y | `ca98b1cd15714881` | `ca98b1cd15714881` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0x130 | `498b87800200004881e83001000049898780020000c3` (22) | same | same | Y | `c505da6e0e035cb4` | `c505da6e0e035cb4` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0x130 | `498b87880200004881e83001000049898788020000c3` (22) | same | same | Y | `cfe5afe593eb6bf8` | `cfe5afe593eb6bf8` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x50 ss=0x60 oo=0x130 — **PASS**

- fixture: `_scratch_ldb_5060_130.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c030010000480fb60049898780020000c3`
- js-sha256: `31e8129afecd8ba85a3d891940c5ddb399c390af72c855d53c8c5aa089b884de`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0x130 — **PASS**

- fixture: `_scratch_ldb_5160_130.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c030010000480fb60049898788020000c3`
- js-sha256: `d0ba625ab36e77eefecf996a76d1a732510dce8824ddb07a5094ff4ebc56bd92`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0x130 — **PASS**

- fixture: `_scratch_ldb_5260_130.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c030010000480fb60049898790020000c3`
- js-sha256: `addb80d146c8758ba56d2ce41ec4006e60a2a8f133de06d8e400f7a1eb4e1bfa`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=0x130 — **PASS**

- fixture: `_scratch_addimm_h50_130.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c03001000049898780020000c3`
- js-sha256: `dd2d08fe3b6bdad6401573da7bfeb58ec7e9829594d20006ecc0f94fb6998414`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x51 imm=0x130 — **PASS**

- fixture: `_scratch_addimm_h51_130.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c03001000049898788020000c3`
- js-sha256: `e7e0160df815fc7df5cbdcc96dec33306970d8732d213baf52ee80eb81d7b5c2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0x130 — **PASS**

- fixture: `_scratch_addimm_h52_130.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c03001000049898790020000c3`
- js-sha256: `ca98b1cd157148817fb766b0d42e123e30f61adf56c3bbe1a1dde634d3d9e039`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0x130 — **PASS**

- fixture: `_scratch_subimm_h50_130.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e83001000049898780020000c3`
- js-sha256: `c505da6e0e035cb495ca102022e0d4fede53f91954f9eb25d8d1d688e7132d9d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0x130 — **PASS**

- fixture: `_scratch_subimm_h51_130.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e83001000049898788020000c3`
- js-sha256: `cfe5afe593eb6bf8209a9d4ddfb1ae279997a30ffe4e12854dc17e3d0fa519ca`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=50/51/52 ss=60 oo=130 (start 130 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=130 (start 130 ADD triad; imm32 22B).
- SUB-IMM slot=50/51 imm=130 (start 130 SUB pair; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1BB`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5060_130.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_130.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_130.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_130.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_130.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_130.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_130.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_130.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-58-log.md` — this file
- `scripts/_probe/parallel-batch-58-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-064 serialize PASSes + 1 Relock**

Pass pin from body-extend-063 Relock: `f4fa77a59520fda01683d3ceffe44de6886ba77752450ffbb0947e0ba15f0d96`.
Handlers before consolidate = 443 (H_00..H_436). Next selectors `40 1BB`.. for H_437.. if all serialize.

PASS list for body-extend-064:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_437 | 0x1BB | 0x80 LDB | 0x50 0x60 0x130 | `498b87000300004881c030010000480fb60049898780020000c3` (26B) | `31e8129afecd8ba8` |
| H_438 | 0x1BC | 0x80 LDB | 0x51 0x60 0x130 | `498b87000300004881c030010000480fb60049898788020000c3` (26B) | `d0ba625ab36e77ee` |
| H_439 | 0x1BD | 0x80 LDB | 0x52 0x60 0x130 | `498b87000300004881c030010000480fb60049898790020000c3` (26B) | `addb80d146c8758b` |
| H_440 | 0x1BE | 0x62 ADD-IMM | 0x50 0x130 | `498b87800200004881c03001000049898780020000c3` (22B) | `dd2d08fe3b6bdad6` |
| H_441 | 0x1BF | 0x62 ADD-IMM | 0x51 0x130 | `498b87880200004881c03001000049898788020000c3` (22B) | `e7e0160df815fc7d` |
| H_442 | 0x1C0 | 0x62 ADD-IMM | 0x52 0x130 | `498b87900200004881c03001000049898790020000c3` (22B) | `ca98b1cd15714881` |
| H_443 | 0x1C1 | 0x61 SUB-IMM | 0x50 0x130 | `498b87800200004881e83001000049898780020000c3` (22B) | `c505da6e0e035cb4` |
| H_444 | 0x1C2 | 0x61 SUB-IMM | 0x51 0x130 | `498b87880200004881e83001000049898788020000c3` (22B) | `cfe5afe593eb6bf8` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-063 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_436.
- If the parent decides to serialize, append H_437.. at selectors `40 1BB`..:
  - H_437 0x80 LDB (80 50 60 130) — pin `498b87000300004881c030010000480fb60049898780020000c3`
  - H_438 0x80 LDB (80 51 60 130) — pin `498b87000300004881c030010000480fb60049898788020000c3`
  - H_439 0x80 LDB (80 52 60 130) — pin `498b87000300004881c030010000480fb60049898790020000c3`
  - H_440 0x62 ADD-IMM (62 50 130) — pin `498b87800200004881c03001000049898780020000c3`
  - H_441 0x62 ADD-IMM (62 51 130) — pin `498b87880200004881c03001000049898788020000c3`
  - H_442 0x62 ADD-IMM (62 52 130) — pin `498b87900200004881c03001000049898790020000c3`
  - H_443 0x61 SUB-IMM (61 50 130) — pin `498b87800200004881e83001000049898780020000c3`
  - H_444 0x61 SUB-IMM (61 51 130) — pin `498b87880200004881e83001000049898788020000c3`
- Plus 1 Relock after append from pin `f4fa77a5…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-064 serialize PASSes + 1 Relock
