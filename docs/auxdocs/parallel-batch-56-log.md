# parallel-batch-56 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-56-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-061 (pin `d4437da8…`, handlers = 428, H_414..H_421 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-061 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_421 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x50 imm=0x120 | `498b87800200004881c02001000049898780020000c3` (22) | same | same | Y | `ec142e42a7c76bc5` | `ec142e42a7c76bc5` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x51 imm=0x120 | `498b87880200004881c02001000049898788020000c3` (22) | same | same | Y | `98a5ad08376f8e1a` | `98a5ad08376f8e1a` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x52 imm=0x120 | `498b87900200004881c02001000049898790020000c3` (22) | same | same | Y | `4ffb72a7006ad4be` | `4ffb72a7006ad4be` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x120 | `498b87800200004881e82001000049898780020000c3` (22) | same | same | Y | `ac80c150be69c45f` | `ac80c150be69c45f` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x51 imm=0x120 | `498b87880200004881e82001000049898788020000c3` (22) | same | same | Y | `63cc573f936e533d` | `63cc573f936e533d` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x52 imm=0x120 | `498b87900200004881e82001000049898790020000c3` (22) | same | same | Y | `587d869f509256fb` | `587d869f509256fb` | PASS |
| 7 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x128 | `498b87000300004881c028010000480fb60049898780020000c3` (26) | same | same | Y | `753ecfc2db0ae0be` | `753ecfc2db0ae0be` | PASS |
| 8 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x128 | `498b87000300004881c028010000480fb60049898788020000c3` (26) | same | same | Y | `6aa74dbb4c649602` | `6aa74dbb4c649602` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x50 imm=0x120 — **PASS**

- fixture: `_scratch_addimm_h50_120.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c02001000049898780020000c3`
- js-sha256: `ec142e42a7c76bc5ee048bfa19bdbddee63a69fbd007cf5e50164da1b5b43cc3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x51 imm=0x120 — **PASS**

- fixture: `_scratch_addimm_h51_120.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c02001000049898788020000c3`
- js-sha256: `98a5ad08376f8e1a58183e2e727d570627bb2246fcdb991d674f88a9840bc999`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x52 imm=0x120 — **PASS**

- fixture: `_scratch_addimm_h52_120.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c02001000049898790020000c3`
- js-sha256: `4ffb72a7006ad4be4d889189084b03e235879fa92a3fea0c2746aca4ef3d9b53`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x120 — **PASS**

- fixture: `_scratch_subimm_h50_120.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e82001000049898780020000c3`
- js-sha256: `ac80c150be69c45f64a085fa9af1de68d8a518e585c5f16a8074be8e9d2346ca`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x51 imm=0x120 — **PASS**

- fixture: `_scratch_subimm_h51_120.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e82001000049898788020000c3`
- js-sha256: `63cc573f936e533d455b5b5c4c6006bd92e940d7c9706f97ac77b92946605d7e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x52 imm=0x120 — **PASS**

- fixture: `_scratch_subimm_h52_120.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e82001000049898790020000c3`
- js-sha256: `587d869f509256fbfc04222665313076ac453ef8e44e174fa2867e5587202831`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x50 ss=0x60 oo=0x128 — **PASS**

- fixture: `_scratch_ldb_5060_128.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c028010000480fb60049898780020000c3`
- js-sha256: `753ecfc2db0ae0be4ed16b35417fc1ad0556ca38075595b85c4d6a1c0d99e14f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x51 ss=0x60 oo=0x128 — **PASS**

- fixture: `_scratch_ldb_5160_128.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c028010000480fb60049898788020000c3`
- js-sha256: `6aa74dbb4c6496022ae0c6be1510e534d21026dcb4de8800dfbc198f9ff2f58b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=50/51/52 imm=120 (start 120 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=120 (start 120 SUB triad; imm32 22B).
- LDB dd=50/51 ss=60 oo=128 (next oo after 120 triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 1AC`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h50_120.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_120.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_120.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_120.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_120.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_120.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_128.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_128.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-56-log.md` — this file
- `scripts/_probe/parallel-batch-56-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-062 serialize PASSes + 1 Relock**

Pass pin from body-extend-061 Relock: `d4437da8f517c8d37c1335b590cae185c0be035d120d84f5ffa0e9354ae484a9`.
Handlers before consolidate = 428 (H_00..H_421). Next selectors `40 1AC`.. for H_422.. if all serialize.

PASS list for body-extend-062:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_422 | 0x1AC | 0x62 ADD-IMM | 0x50 0x120 | `498b87800200004881c02001000049898780020000c3` (22B) | `ec142e42a7c76bc5` |
| H_423 | 0x1AD | 0x62 ADD-IMM | 0x51 0x120 | `498b87880200004881c02001000049898788020000c3` (22B) | `98a5ad08376f8e1a` |
| H_424 | 0x1AE | 0x62 ADD-IMM | 0x52 0x120 | `498b87900200004881c02001000049898790020000c3` (22B) | `4ffb72a7006ad4be` |
| H_425 | 0x1AF | 0x61 SUB-IMM | 0x50 0x120 | `498b87800200004881e82001000049898780020000c3` (22B) | `ac80c150be69c45f` |
| H_426 | 0x1B0 | 0x61 SUB-IMM | 0x51 0x120 | `498b87880200004881e82001000049898788020000c3` (22B) | `63cc573f936e533d` |
| H_427 | 0x1B1 | 0x61 SUB-IMM | 0x52 0x120 | `498b87900200004881e82001000049898790020000c3` (22B) | `587d869f509256fb` |
| H_428 | 0x1B2 | 0x80 LDB | 0x50 0x60 0x128 | `498b87000300004881c028010000480fb60049898780020000c3` (26B) | `753ecfc2db0ae0be` |
| H_429 | 0x1B3 | 0x80 LDB | 0x51 0x60 0x128 | `498b87000300004881c028010000480fb60049898788020000c3` (26B) | `6aa74dbb4c649602` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-061 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_421.
- If the parent decides to serialize, append H_422.. at selectors `40 1AC`..:
  - H_422 0x62 ADD-IMM (62 50 120) — pin `498b87800200004881c02001000049898780020000c3`
  - H_423 0x62 ADD-IMM (62 51 120) — pin `498b87880200004881c02001000049898788020000c3`
  - H_424 0x62 ADD-IMM (62 52 120) — pin `498b87900200004881c02001000049898790020000c3`
  - H_425 0x61 SUB-IMM (61 50 120) — pin `498b87800200004881e82001000049898780020000c3`
  - H_426 0x61 SUB-IMM (61 51 120) — pin `498b87880200004881e82001000049898788020000c3`
  - H_427 0x61 SUB-IMM (61 52 120) — pin `498b87900200004881e82001000049898790020000c3`
  - H_428 0x80 LDB (80 50 60 128) — pin `498b87000300004881c028010000480fb60049898780020000c3`
  - H_429 0x80 LDB (80 51 60 128) — pin `498b87000300004881c028010000480fb60049898788020000c3`
- Plus 1 Relock after append from pin `d4437da8…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-062 serialize PASSes + 1 Relock
