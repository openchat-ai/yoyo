# parallel-batch-49 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-49-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-054 (pin `13cb91ab…`, handlers = 372, H_358..H_365 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-054 DDC PE `.text` measured DIFFER — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_365 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xF0 | `498b87000300004881c0f0000000480fb60049898788020000c3` (26) | same | same | Y | `878beef94d2aaca0` | `878beef94d2aaca0` | PASS |
| 2 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xF0 | `498b87000300004881c0f0000000480fb60049898790020000c3` (26) | same | same | Y | `39e79a02c3bbc071` | `39e79a02c3bbc071` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x50 imm=0xF0 | `498b87800200004881c0f000000049898780020000c3` (22) | same | same | Y | `cfd72ee65ddb08fc` | `cfd72ee65ddb08fc` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x51 imm=0xF0 | `498b87880200004881c0f000000049898788020000c3` (22) | same | same | Y | `5aa3b0e69138d4d3` | `5aa3b0e69138d4d3` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x52 imm=0xF0 | `498b87900200004881c0f000000049898790020000c3` (22) | same | same | Y | `e67473702a13c78e` | `e67473702a13c78e` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x50 imm=0xF0 | `498b87800200004881e8f000000049898780020000c3` (22) | same | same | Y | `3404141d925462bb` | `3404141d925462bb` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0xF0 | `498b87880200004881e8f000000049898788020000c3` (22) | same | same | Y | `d52a7558bdad1d89` | `d52a7558bdad1d89` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x52 imm=0xF0 | `498b87900200004881e8f000000049898790020000c3` (22) | same | same | Y | `4128c048e41cad1a` | `4128c048e41cad1a` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x51 ss=0x60 oo=0xF0 — **PASS**

- fixture: `_scratch_ldb_5160_f0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f0000000480fb60049898788020000c3`
- js-sha256: `878beef94d2aaca04e5efb8e45f4a867cd7039e2a1f9bca97382f4d09e6e78a2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x52 ss=0x60 oo=0xF0 — **PASS**

- fixture: `_scratch_ldb_5260_f0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f0000000480fb60049898790020000c3`
- js-sha256: `39e79a02c3bbc071604456f1d4180450dfd918f46c3e1247ae61e18ed26e2f9f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x50 imm=0xF0 — **PASS**

- fixture: `_scratch_addimm_h50_f0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0f000000049898780020000c3`
- js-sha256: `cfd72ee65ddb08fcb37ba69e9675363ad8e8e6f4e25a5ee7f5a623fb4aa1c397`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x51 imm=0xF0 — **PASS**

- fixture: `_scratch_addimm_h51_f0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0f000000049898788020000c3`
- js-sha256: `5aa3b0e69138d4d37568053650679921a47fb1741bea2da664dba51ab20dca86`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x52 imm=0xF0 — **PASS**

- fixture: `_scratch_addimm_h52_f0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0f000000049898790020000c3`
- js-sha256: `e67473702a13c78e2404bdc37f5673489ca0a7bf1b01003879dbfa40e71ca334`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x50 imm=0xF0 — **PASS**

- fixture: `_scratch_subimm_h50_f0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8f000000049898780020000c3`
- js-sha256: `3404141d925462bb6adcc0d67957347b9630be8581829283c16ce21acf444d9a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0xF0 — **PASS**

- fixture: `_scratch_subimm_h51_f0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8f000000049898788020000c3`
- js-sha256: `d52a7558bdad1d89bd1cf87a9774c4bdee4da8d91f27d8f461a290fb6d7ca3f5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x52 imm=0xF0 — **PASS**

- fixture: `_scratch_subimm_h52_f0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8f000000049898790020000c3`
- js-sha256: `4128c048e41cad1af95d1cfc3872edbb23b864dd29b56ae1946a7dfcbd9a1ca3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=51/52 ss=60 oo=F0 (finish F0 triad after H_365; imm32 26B).
- ADD-IMM slot=50/51/52 imm=F0 (fresh imm after E8; imm32 22B).
- SUB-IMM slot=50/51/52 imm=F0 (complements ADD-IMM * F0; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 174`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5160_f0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_f0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_f0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_f0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_f0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_f0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_f0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_f0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-49-log.md` — this file
- `scripts/_probe/parallel-batch-49-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-055 serialize PASSes + 1 Relock**

Pass pin from body-extend-054 Relock: `13cb91ab1e1cc24d3f4b6d9a151a2e9a8d487556099cc030a189d6ac30554d9b`.
Handlers before consolidate = 372 (H_00..H_365). Next selectors `40 174`.. for H_366.. if all serialize.

PASS list for body-extend-055:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_366 | 0x174 | 0x80 LDB | 0x51 0x60 0xF0 | `498b87000300004881c0f0000000480fb60049898788020000c3` (26B) | `878beef94d2aaca0` |
| H_367 | 0x175 | 0x80 LDB | 0x52 0x60 0xF0 | `498b87000300004881c0f0000000480fb60049898790020000c3` (26B) | `39e79a02c3bbc071` |
| H_368 | 0x176 | 0x62 ADD-IMM | 0x50 0xF0 | `498b87800200004881c0f000000049898780020000c3` (22B) | `cfd72ee65ddb08fc` |
| H_369 | 0x177 | 0x62 ADD-IMM | 0x51 0xF0 | `498b87880200004881c0f000000049898788020000c3` (22B) | `5aa3b0e69138d4d3` |
| H_370 | 0x178 | 0x62 ADD-IMM | 0x52 0xF0 | `498b87900200004881c0f000000049898790020000c3` (22B) | `e67473702a13c78e` |
| H_371 | 0x179 | 0x61 SUB-IMM | 0x50 0xF0 | `498b87800200004881e8f000000049898780020000c3` (22B) | `3404141d925462bb` |
| H_372 | 0x17A | 0x61 SUB-IMM | 0x51 0xF0 | `498b87880200004881e8f000000049898788020000c3` (22B) | `d52a7558bdad1d89` |
| H_373 | 0x17B | 0x61 SUB-IMM | 0x52 0xF0 | `498b87900200004881e8f000000049898790020000c3` (22B) | `4128c048e41cad1a` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-054 DDC PE `.text` DIFFER noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_365.
- If the parent decides to serialize, append H_366.. at selectors `40 174`..:
  - H_366 0x80 LDB (80 51 60 F0) — pin `498b87000300004881c0f0000000480fb60049898788020000c3`
  - H_367 0x80 LDB (80 52 60 F0) — pin `498b87000300004881c0f0000000480fb60049898790020000c3`
  - H_368 0x62 ADD-IMM (62 50 F0) — pin `498b87800200004881c0f000000049898780020000c3`
  - H_369 0x62 ADD-IMM (62 51 F0) — pin `498b87880200004881c0f000000049898788020000c3`
  - H_370 0x62 ADD-IMM (62 52 F0) — pin `498b87900200004881c0f000000049898790020000c3`
  - H_371 0x61 SUB-IMM (61 50 F0) — pin `498b87800200004881e8f000000049898780020000c3`
  - H_372 0x61 SUB-IMM (61 51 F0) — pin `498b87880200004881e8f000000049898788020000c3`
  - H_373 0x61 SUB-IMM (61 52 F0) — pin `498b87900200004881e8f000000049898790020000c3`
- Plus 1 Relock after append from pin `13cb91ab…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-055 serialize PASSes + 1 Relock
