# parallel-batch-79 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-79-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-084 (pin `9eafc9ce…`, handlers = 611, H_597..H_604 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-084 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_604 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x50 imm=0x1C0 | `498b87800200004881e8c001000049898780020000c3` (22) | same | same | Y | `2dd291d1df0ff186` | `2dd291d1df0ff186` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x51 imm=0x1C0 | `498b87880200004881e8c001000049898788020000c3` (22) | same | same | Y | `162f63e6a4ed8641` | `162f63e6a4ed8641` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x52 imm=0x1C0 | `498b87900200004881e8c001000049898790020000c3` (22) | same | same | Y | `649c06ddcb80956d` | `649c06ddcb80956d` | PASS |
| 4 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1C8 | `498b87000300004881c0c8010000480fb60049898780020000c3` (26) | same | same | Y | `b299fd62cea22ef7` | `b299fd62cea22ef7` | PASS |
| 5 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1C8 | `498b87000300004881c0c8010000480fb60049898788020000c3` (26) | same | same | Y | `18e61721bdda72c3` | `18e61721bdda72c3` | PASS |
| 6 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1C8 | `498b87000300004881c0c8010000480fb60049898790020000c3` (26) | same | same | Y | `9612ef36d64f34eb` | `9612ef36d64f34eb` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x50 imm=0x1C8 | `498b87800200004881c0c801000049898780020000c3` (22) | same | same | Y | `435f20ebb01bbc21` | `435f20ebb01bbc21` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x51 imm=0x1C8 | `498b87880200004881c0c801000049898788020000c3` (22) | same | same | Y | `d6e88e4f8c96211e` | `d6e88e4f8c96211e` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x50 imm=0x1C0 — **PASS**

- fixture: `_scratch_subimm_h50_1C0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8c001000049898780020000c3`
- js-sha256: `2dd291d1df0ff186d43b5edce6e88baac2de7cdfe15cbf3067278a79a62ce4ac`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x51 imm=0x1C0 — **PASS**

- fixture: `_scratch_subimm_h51_1C0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8c001000049898788020000c3`
- js-sha256: `162f63e6a4ed8641cbcc38e6bb5c8bbbfaca6370421c764393cab71092041e2a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x52 imm=0x1C0 — **PASS**

- fixture: `_scratch_subimm_h52_1C0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8c001000049898790020000c3`
- js-sha256: `649c06ddcb80956d738bd791477f5e28f3d562d44ae564f9ebe68323ea677074`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x50 ss=0x60 oo=0x1C8 — **PASS**

- fixture: `_scratch_ldb_5060_1C8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c8010000480fb60049898780020000c3`
- js-sha256: `b299fd62cea22ef7655b78fa27f156de19e0b5359ff7616f7e6dd739060ca0c9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x51 ss=0x60 oo=0x1C8 — **PASS**

- fixture: `_scratch_ldb_5160_1C8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c8010000480fb60049898788020000c3`
- js-sha256: `18e61721bdda72c38fbeac19fbe27d118aa52b762f7908a80ccf1dc295ad0549`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x52 ss=0x60 oo=0x1C8 — **PASS**

- fixture: `_scratch_ldb_5260_1C8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c8010000480fb60049898790020000c3`
- js-sha256: `9612ef36d64f34ebefdf68541995941ac8d39b3cd84816f3da60e3955d94ae33`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x50 imm=0x1C8 — **PASS**

- fixture: `_scratch_addimm_h50_1C8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0c801000049898780020000c3`
- js-sha256: `435f20ebb01bbc215ee27da9f187dece6dce5783705b4241c548b9be6deccecc`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x51 imm=0x1C8 — **PASS**

- fixture: `_scratch_addimm_h51_1C8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0c801000049898788020000c3`
- js-sha256: `d6e88e4f8c96211e39163d7f39a8e213245a69e2c1dccfbc34e708ea1d6c6639`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=50/51/52 imm=1C0 (start deferred 1C0 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=1C8 (start deferred 1C8 LDB triad; imm32 26B).
- ADD-IMM slot=50/51 imm=1C8 (start deferred 1C8 ADD triad; imm32 22B).
- ADD-IMM slot=52 imm=1C8 + SUB-IMM slot=50/51/52 imm=1C8 deferred to next batch.
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 263`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h50_1C0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1C0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1C0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_1C8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_1C8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_1C8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1C8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1C8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-79-log.md` — this file
- `scripts/_probe/parallel-batch-79-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-085 serialize PASSes + 1 Relock**

Pass pin from body-extend-084 Relock: `9eafc9ce0376d389043b0e77ec2c1ff2bc44dda11b4fb8f6449cc4ea811798ac`.
Handlers before consolidate = 611 (H_00..H_604). Next selectors `40 263`.. for H_605.. if all serialize.

PASS list for body-extend-085:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_605 | 0x263 | 0x61 SUB-IMM | 0x50 0x1C0 | `498b87800200004881e8c001000049898780020000c3` (22B) | `2dd291d1df0ff186` |
| H_606 | 0x264 | 0x61 SUB-IMM | 0x51 0x1C0 | `498b87880200004881e8c001000049898788020000c3` (22B) | `162f63e6a4ed8641` |
| H_607 | 0x265 | 0x61 SUB-IMM | 0x52 0x1C0 | `498b87900200004881e8c001000049898790020000c3` (22B) | `649c06ddcb80956d` |
| H_608 | 0x266 | 0x80 LDB | 0x50 0x60 0x1C8 | `498b87000300004881c0c8010000480fb60049898780020000c3` (26B) | `b299fd62cea22ef7` |
| H_609 | 0x267 | 0x80 LDB | 0x51 0x60 0x1C8 | `498b87000300004881c0c8010000480fb60049898788020000c3` (26B) | `18e61721bdda72c3` |
| H_610 | 0x268 | 0x80 LDB | 0x52 0x60 0x1C8 | `498b87000300004881c0c8010000480fb60049898790020000c3` (26B) | `9612ef36d64f34eb` |
| H_611 | 0x269 | 0x62 ADD-IMM | 0x50 0x1C8 | `498b87800200004881c0c801000049898780020000c3` (22B) | `435f20ebb01bbc21` |
| H_612 | 0x26A | 0x62 ADD-IMM | 0x51 0x1C8 | `498b87880200004881c0c801000049898788020000c3` (22B) | `d6e88e4f8c96211e` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-084 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_604.
- If the parent decides to serialize, append H_605.. at selectors `40 263`..:
  - H_605 0x61 SUB-IMM (61 50 1C0) — pin `498b87800200004881e8c001000049898780020000c3`
  - H_606 0x61 SUB-IMM (61 51 1C0) — pin `498b87880200004881e8c001000049898788020000c3`
  - H_607 0x61 SUB-IMM (61 52 1C0) — pin `498b87900200004881e8c001000049898790020000c3`
  - H_608 0x80 LDB (80 50 60 1C8) — pin `498b87000300004881c0c8010000480fb60049898780020000c3`
  - H_609 0x80 LDB (80 51 60 1C8) — pin `498b87000300004881c0c8010000480fb60049898788020000c3`
  - H_610 0x80 LDB (80 52 60 1C8) — pin `498b87000300004881c0c8010000480fb60049898790020000c3`
  - H_611 0x62 ADD-IMM (62 50 1C8) — pin `498b87800200004881c0c801000049898780020000c3`
  - H_612 0x62 ADD-IMM (62 51 1C8) — pin `498b87880200004881c0c801000049898788020000c3`
- Plus 1 Relock after append from pin `9eafc9ce…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-085 serialize PASSes + 1 Relock
