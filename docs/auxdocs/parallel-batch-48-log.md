# parallel-batch-48 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-48-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-053 (pin `86485f48…`, handlers = 364, H_350..H_357 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-053 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_357 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x52 imm=0xE0 | `498b87900200004881e8e000000049898790020000c3` (22) | same | same | Y | `7986c4bc9ebed8c6` | `7986c4bc9ebed8c6` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x50 imm=0xE8 | `498b87800200004881c0e800000049898780020000c3` (22) | same | same | Y | `51760cec223058e1` | `51760cec223058e1` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0xE8 | `498b87880200004881c0e800000049898788020000c3` (22) | same | same | Y | `75755148da277056` | `75755148da277056` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0xE8 | `498b87900200004881c0e800000049898790020000c3` (22) | same | same | Y | `e8d397ad24fcfa8c` | `e8d397ad24fcfa8c` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x50 imm=0xE8 | `498b87800200004881e8e800000049898780020000c3` (22) | same | same | Y | `45dace9bedbf51e3` | `45dace9bedbf51e3` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x51 imm=0xE8 | `498b87880200004881e8e800000049898788020000c3` (22) | same | same | Y | `ce05fadbd17ed30f` | `ce05fadbd17ed30f` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0xE8 | `498b87900200004881e8e800000049898790020000c3` (22) | same | same | Y | `87083a564ea9a2de` | `87083a564ea9a2de` | PASS |
| 8 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xF0 | `498b87000300004881c0f0000000480fb60049898780020000c3` (26) | same | same | Y | `a8241e1de5be2a76` | `a8241e1de5be2a76` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x52 imm=0xE0 — **PASS**

- fixture: `_scratch_subimm_h52_e0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8e000000049898790020000c3`
- js-sha256: `7986c4bc9ebed8c615b5512585c1c715369f0e06406186b9e26de02550c5087b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x50 imm=0xE8 — **PASS**

- fixture: `_scratch_addimm_h50_e8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0e800000049898780020000c3`
- js-sha256: `51760cec223058e1e2efa167c7c6e97cd9c880e38a09616a625aed8010d6bd7a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0xE8 — **PASS**

- fixture: `_scratch_addimm_h51_e8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0e800000049898788020000c3`
- js-sha256: `75755148da27705618ea6e348d9f98e107a5c70c9be4ab5442b4275e80d059cb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0xE8 — **PASS**

- fixture: `_scratch_addimm_h52_e8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0e800000049898790020000c3`
- js-sha256: `e8d397ad24fcfa8c05d2981b1e71db11a7183c30e2b30f49a39c20ed51c6565e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x50 imm=0xE8 — **PASS**

- fixture: `_scratch_subimm_h50_e8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8e800000049898780020000c3`
- js-sha256: `45dace9bedbf51e3a8f01a2676a43b1ebbd7c807513786231cb31af596d29c12`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x51 imm=0xE8 — **PASS**

- fixture: `_scratch_subimm_h51_e8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8e800000049898788020000c3`
- js-sha256: `ce05fadbd17ed30f59808e6379fb6e21f94ab2cff74e67c0b9ae7ed5914c4a98`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0xE8 — **PASS**

- fixture: `_scratch_subimm_h52_e8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8e800000049898790020000c3`
- js-sha256: `87083a564ea9a2ded7da403c8650e858a8971f03af2bf780607b959c6faef966`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x50 ss=0x60 oo=0xF0 — **PASS**

- fixture: `_scratch_ldb_5060_f0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f0000000480fb60049898780020000c3`
- js-sha256: `a8241e1de5be2a76725f8966555ade3cbf05387853d23d3b4a5d3f5dd601caf2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=52 imm=E0 (finish E0 triad after H_356/H_357; imm32 22B).
- ADD-IMM slot=50/51/52 imm=E8 (fresh imm after E0; imm32 22B).
- SUB-IMM slot=50/51/52 imm=E8 (complements ADD-IMM * E8; imm32 22B).
- LDB dd=50 ss=60 oo=F0 (next oo after E8; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 16C`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h52_e0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_e8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_e8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_e8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_e8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_e8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_e8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_f0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-48-log.md` — this file
- `scripts/_probe/parallel-batch-48-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-054 serialize PASSes + 1 Relock**

Pass pin from body-extend-053 Relock: `86485f4822e891c4f11dbc5f181c43dc3f23d7ed779b61831f2426f2329e504d`.
Handlers before consolidate = 364 (H_00..H_357). Next selectors `40 16C`.. for H_358.. if all serialize.

PASS list for body-extend-054:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_358 | 0x16C | 0x61 SUB-IMM | 0x52 0xE0 | `498b87900200004881e8e000000049898790020000c3` (22B) | `7986c4bc9ebed8c6` |
| H_359 | 0x16D | 0x62 ADD-IMM | 0x50 0xE8 | `498b87800200004881c0e800000049898780020000c3` (22B) | `51760cec223058e1` |
| H_360 | 0x16E | 0x62 ADD-IMM | 0x51 0xE8 | `498b87880200004881c0e800000049898788020000c3` (22B) | `75755148da277056` |
| H_361 | 0x16F | 0x62 ADD-IMM | 0x52 0xE8 | `498b87900200004881c0e800000049898790020000c3` (22B) | `e8d397ad24fcfa8c` |
| H_362 | 0x170 | 0x61 SUB-IMM | 0x50 0xE8 | `498b87800200004881e8e800000049898780020000c3` (22B) | `45dace9bedbf51e3` |
| H_363 | 0x171 | 0x61 SUB-IMM | 0x51 0xE8 | `498b87880200004881e8e800000049898788020000c3` (22B) | `ce05fadbd17ed30f` |
| H_364 | 0x172 | 0x61 SUB-IMM | 0x52 0xE8 | `498b87900200004881e8e800000049898790020000c3` (22B) | `87083a564ea9a2de` |
| H_365 | 0x173 | 0x80 LDB | 0x50 0x60 0xF0 | `498b87000300004881c0f0000000480fb60049898780020000c3` (26B) | `a8241e1de5be2a76` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-053 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_357.
- If the parent decides to serialize, append H_358.. at selectors `40 16C`..:
  - H_358 0x61 SUB-IMM (61 52 E0) — pin `498b87900200004881e8e000000049898790020000c3`
  - H_359 0x62 ADD-IMM (62 50 E8) — pin `498b87800200004881c0e800000049898780020000c3`
  - H_360 0x62 ADD-IMM (62 51 E8) — pin `498b87880200004881c0e800000049898788020000c3`
  - H_361 0x62 ADD-IMM (62 52 E8) — pin `498b87900200004881c0e800000049898790020000c3`
  - H_362 0x61 SUB-IMM (61 50 E8) — pin `498b87800200004881e8e800000049898780020000c3`
  - H_363 0x61 SUB-IMM (61 51 E8) — pin `498b87880200004881e8e800000049898788020000c3`
  - H_364 0x61 SUB-IMM (61 52 E8) — pin `498b87900200004881e8e800000049898790020000c3`
  - H_365 0x80 LDB (80 50 60 F0) — pin `498b87000300004881c0f0000000480fb60049898780020000c3`
- Plus 1 Relock after append from pin `86485f48…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-054 serialize PASSes + 1 Relock
