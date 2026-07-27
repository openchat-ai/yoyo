# parallel-batch-45 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-45-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-050 (pin `1566906f…`, handlers = 340, H_326..H_333 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-050 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_333 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x52 imm=0xC8 | `498b87900200004881e8c800000049898790020000c3` (22) | same | same | Y | `3b32f5875666e837` | `3b32f5875666e837` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x50 imm=0xD0 | `498b87800200004881c0d000000049898780020000c3` (22) | same | same | Y | `5cdff426638d0c76` | `5cdff426638d0c76` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0xD0 | `498b87880200004881c0d000000049898788020000c3` (22) | same | same | Y | `a4c8fb5e23221fc9` | `a4c8fb5e23221fc9` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0xD0 | `498b87900200004881c0d000000049898790020000c3` (22) | same | same | Y | `d3a3f45f884525f8` | `d3a3f45f884525f8` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x50 imm=0xD0 | `498b87800200004881e8d000000049898780020000c3` (22) | same | same | Y | `308c801c542d857b` | `308c801c542d857b` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x51 imm=0xD0 | `498b87880200004881e8d000000049898788020000c3` (22) | same | same | Y | `744b3918b3f5fe8e` | `744b3918b3f5fe8e` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0xD0 | `498b87900200004881e8d000000049898790020000c3` (22) | same | same | Y | `ee26c6478e1bedb5` | `ee26c6478e1bedb5` | PASS |
| 8 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xE0 | `498b87000300004881c0e0000000480fb60049898780020000c3` (26) | same | same | Y | `3fcfa899104fe81a` | `3fcfa899104fe81a` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x52 imm=0xC8 — **PASS**

- fixture: `_scratch_subimm_h52_c8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8c800000049898790020000c3`
- js-sha256: `3b32f5875666e8373a280b3a1286f8992ed1cb91c944ea5802da691c198b7d97`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x50 imm=0xD0 — **PASS**

- fixture: `_scratch_addimm_h50_d0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0d000000049898780020000c3`
- js-sha256: `5cdff426638d0c76397e38d765e16b075ace43f3a67fe9fd20066a191c4812a5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0xD0 — **PASS**

- fixture: `_scratch_addimm_h51_d0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0d000000049898788020000c3`
- js-sha256: `a4c8fb5e23221fc91addf13af5b7196ac39981cfae5baac16dc2b9b66302dbc5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0xD0 — **PASS**

- fixture: `_scratch_addimm_h52_d0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0d000000049898790020000c3`
- js-sha256: `d3a3f45f884525f8096effcf3cc1607acf843190ee2f698c79969773d1b3922e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x50 imm=0xD0 — **PASS**

- fixture: `_scratch_subimm_h50_d0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8d000000049898780020000c3`
- js-sha256: `308c801c542d857beb0fc1ac66260fd6386d87e7e0fe477cf557953a6d2f596c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x51 imm=0xD0 — **PASS**

- fixture: `_scratch_subimm_h51_d0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8d000000049898788020000c3`
- js-sha256: `744b3918b3f5fe8e5d42fbfcb1a7c33fca66ebfdcc44c3b056bef1a0c9561b1c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0xD0 — **PASS**

- fixture: `_scratch_subimm_h52_d0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8d000000049898790020000c3`
- js-sha256: `ee26c6478e1bedb558571013d8552266352879f1d00151658d7d5f5d1aae4e79`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x50 ss=0x60 oo=0xE0 — **PASS**

- fixture: `_scratch_ldb_5060_e0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e0000000480fb60049898780020000c3`
- js-sha256: `3fcfa899104fe81aacb435e9380b6862db53b37e9bf2bc607a9c458a64cedbd3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=52 imm=C8 (finish C8 SUB triad after H_332/H_333; imm32 22B).
- ADD-IMM slot=50/51/52 imm=D0 (fresh imm after C8; imm32 22B).
- SUB-IMM slot=50/51/52 imm=D0 (complements ADD-IMM * D0; imm32 22B).
- LDB dd=50 ss=60 oo=E0 (next oo after D8 triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 154`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h52_c8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_d0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_d0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_d0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_d0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_d0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_d0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_e0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-45-log.md` — this file
- `scripts/_probe/parallel-batch-45-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-051 serialize PASSes + 1 Relock**

Pass pin from body-extend-050 Relock: `1566906f85667e97cb5701b0d3ba8fdd806e893b1982fa3ad11a1138efb8adfe`.
Handlers before consolidate = 340 (H_00..H_333). Next selectors `40 154`.. for H_334.. if all serialize.

PASS list for body-extend-051:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_334 | 0x154 | 0x61 SUB-IMM | 0x52 0xC8 | `498b87900200004881e8c800000049898790020000c3` (22B) | `3b32f5875666e837` |
| H_335 | 0x155 | 0x62 ADD-IMM | 0x50 0xD0 | `498b87800200004881c0d000000049898780020000c3` (22B) | `5cdff426638d0c76` |
| H_336 | 0x156 | 0x62 ADD-IMM | 0x51 0xD0 | `498b87880200004881c0d000000049898788020000c3` (22B) | `a4c8fb5e23221fc9` |
| H_337 | 0x157 | 0x62 ADD-IMM | 0x52 0xD0 | `498b87900200004881c0d000000049898790020000c3` (22B) | `d3a3f45f884525f8` |
| H_338 | 0x158 | 0x61 SUB-IMM | 0x50 0xD0 | `498b87800200004881e8d000000049898780020000c3` (22B) | `308c801c542d857b` |
| H_339 | 0x159 | 0x61 SUB-IMM | 0x51 0xD0 | `498b87880200004881e8d000000049898788020000c3` (22B) | `744b3918b3f5fe8e` |
| H_340 | 0x15A | 0x61 SUB-IMM | 0x52 0xD0 | `498b87900200004881e8d000000049898790020000c3` (22B) | `ee26c6478e1bedb5` |
| H_341 | 0x15B | 0x80 LDB | 0x50 0x60 0xE0 | `498b87000300004881c0e0000000480fb60049898780020000c3` (26B) | `3fcfa899104fe81a` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-050 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_333.
- If the parent decides to serialize, append H_334.. at selectors `40 154`..:
  - H_334 0x61 SUB-IMM (61 52 C8) — pin `498b87900200004881e8c800000049898790020000c3`
  - H_335 0x62 ADD-IMM (62 50 D0) — pin `498b87800200004881c0d000000049898780020000c3`
  - H_336 0x62 ADD-IMM (62 51 D0) — pin `498b87880200004881c0d000000049898788020000c3`
  - H_337 0x62 ADD-IMM (62 52 D0) — pin `498b87900200004881c0d000000049898790020000c3`
  - H_338 0x61 SUB-IMM (61 50 D0) — pin `498b87800200004881e8d000000049898780020000c3`
  - H_339 0x61 SUB-IMM (61 51 D0) — pin `498b87880200004881e8d000000049898788020000c3`
  - H_340 0x61 SUB-IMM (61 52 D0) — pin `498b87900200004881e8d000000049898790020000c3`
  - H_341 0x80 LDB (80 50 60 E0) — pin `498b87000300004881c0e0000000480fb60049898780020000c3`
- Plus 1 Relock after append from pin `1566906f…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-051 serialize PASSes + 1 Relock
