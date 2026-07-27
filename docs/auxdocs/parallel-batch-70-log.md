# parallel-batch-70 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-70-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-075 (pin `69f1bb2f…`, handlers = 539, H_525..H_532 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-075 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_532 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x50 imm=0x180 | `498b87800200004881e88001000049898780020000c3` (22) | same | same | Y | `f31ae79928dbdd81` | `f31ae79928dbdd81` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x51 imm=0x180 | `498b87880200004881e88001000049898788020000c3` (22) | same | same | Y | `050072b4e44aee5c` | `050072b4e44aee5c` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x52 imm=0x180 | `498b87900200004881e88001000049898790020000c3` (22) | same | same | Y | `6ad9c3df1ba66463` | `6ad9c3df1ba66463` | PASS |
| 4 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x188 | `498b87000300004881c088010000480fb60049898780020000c3` (26) | same | same | Y | `18667432b27ded5f` | `18667432b27ded5f` | PASS |
| 5 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x188 | `498b87000300004881c088010000480fb60049898788020000c3` (26) | same | same | Y | `565922cabac58b5a` | `565922cabac58b5a` | PASS |
| 6 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x188 | `498b87000300004881c088010000480fb60049898790020000c3` (26) | same | same | Y | `0ccdef6304b031b3` | `0ccdef6304b031b3` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x50 imm=0x188 | `498b87800200004881c08801000049898780020000c3` (22) | same | same | Y | `b9c2434436452b99` | `b9c2434436452b99` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x51 imm=0x188 | `498b87880200004881c08801000049898788020000c3` (22) | same | same | Y | `4710e829b779fc66` | `4710e829b779fc66` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x50 imm=0x180 — **PASS**

- fixture: `_scratch_subimm_h50_180.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e88001000049898780020000c3`
- js-sha256: `f31ae79928dbdd81951f15879484ce8c3f347dfc9d091d93d5655997fef8a891`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x51 imm=0x180 — **PASS**

- fixture: `_scratch_subimm_h51_180.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e88001000049898788020000c3`
- js-sha256: `050072b4e44aee5c00ee9d299f1fccc170e724256993ea6237244e2c7c6bacc7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x52 imm=0x180 — **PASS**

- fixture: `_scratch_subimm_h52_180.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e88001000049898790020000c3`
- js-sha256: `6ad9c3df1ba6646323eaebc33e665e4cba6212891bc657d9b95e2135585b9ee0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x50 ss=0x60 oo=0x188 — **PASS**

- fixture: `_scratch_ldb_5060_188.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c088010000480fb60049898780020000c3`
- js-sha256: `18667432b27ded5f98f065124b9ae0f537b16849d3cfb3d581f76fe51a551b0e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x51 ss=0x60 oo=0x188 — **PASS**

- fixture: `_scratch_ldb_5160_188.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c088010000480fb60049898788020000c3`
- js-sha256: `565922cabac58b5a3c474b57981d5f7e084af4b6a2e0ae51f4dda2f710206a5f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x52 ss=0x60 oo=0x188 — **PASS**

- fixture: `_scratch_ldb_5260_188.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c088010000480fb60049898790020000c3`
- js-sha256: `0ccdef6304b031b3dcc4752687b7c97f4c254aa7391506f05168f9662d26a17e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x50 imm=0x188 — **PASS**

- fixture: `_scratch_addimm_h50_188.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c08801000049898780020000c3`
- js-sha256: `b9c2434436452b99be0787c38ad6fb9c0679e92c716d4d9a112283396a7ddff3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x51 imm=0x188 — **PASS**

- fixture: `_scratch_addimm_h51_188.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c08801000049898788020000c3`
- js-sha256: `4710e829b779fc660201a66890536c1a7970e81284ab0be822197e187d4e5b31`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=50/51/52 imm=180 (finish deferred 180 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=188 (start 188 LDB triad; imm32 26B).
- ADD-IMM slot=50/51 imm=188 (start 188 ADD triad; imm32 22B; slot=52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 21B`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h50_180.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_180.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_180.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_188.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_188.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_188.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_188.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_188.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-70-log.md` — this file
- `scripts/_probe/parallel-batch-70-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-076 serialize PASSes + 1 Relock**

Pass pin from body-extend-075 Relock: `69f1bb2f223e28673dfb97de72b1305d451313a4865d02e766ed947748a10cf9`.
Handlers before consolidate = 539 (H_00..H_532). Next selectors `40 21B`.. for H_533.. if all serialize.

PASS list for body-extend-076:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_533 | 0x21B | 0x61 SUB-IMM | 0x50 0x180 | `498b87800200004881e88001000049898780020000c3` (22B) | `f31ae79928dbdd81` |
| H_534 | 0x21C | 0x61 SUB-IMM | 0x51 0x180 | `498b87880200004881e88001000049898788020000c3` (22B) | `050072b4e44aee5c` |
| H_535 | 0x21D | 0x61 SUB-IMM | 0x52 0x180 | `498b87900200004881e88001000049898790020000c3` (22B) | `6ad9c3df1ba66463` |
| H_536 | 0x21E | 0x80 LDB | 0x50 0x60 0x188 | `498b87000300004881c088010000480fb60049898780020000c3` (26B) | `18667432b27ded5f` |
| H_537 | 0x21F | 0x80 LDB | 0x51 0x60 0x188 | `498b87000300004881c088010000480fb60049898788020000c3` (26B) | `565922cabac58b5a` |
| H_538 | 0x220 | 0x80 LDB | 0x52 0x60 0x188 | `498b87000300004881c088010000480fb60049898790020000c3` (26B) | `0ccdef6304b031b3` |
| H_539 | 0x221 | 0x62 ADD-IMM | 0x50 0x188 | `498b87800200004881c08801000049898780020000c3` (22B) | `b9c2434436452b99` |
| H_540 | 0x222 | 0x62 ADD-IMM | 0x51 0x188 | `498b87880200004881c08801000049898788020000c3` (22B) | `4710e829b779fc66` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-075 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_532.
- If the parent decides to serialize, append H_533.. at selectors `40 21B`..:
  - H_533 0x61 SUB-IMM (61 50 180) — pin `498b87800200004881e88001000049898780020000c3`
  - H_534 0x61 SUB-IMM (61 51 180) — pin `498b87880200004881e88001000049898788020000c3`
  - H_535 0x61 SUB-IMM (61 52 180) — pin `498b87900200004881e88001000049898790020000c3`
  - H_536 0x80 LDB (80 50 60 188) — pin `498b87000300004881c088010000480fb60049898780020000c3`
  - H_537 0x80 LDB (80 51 60 188) — pin `498b87000300004881c088010000480fb60049898788020000c3`
  - H_538 0x80 LDB (80 52 60 188) — pin `498b87000300004881c088010000480fb60049898790020000c3`
  - H_539 0x62 ADD-IMM (62 50 188) — pin `498b87800200004881c08801000049898780020000c3`
  - H_540 0x62 ADD-IMM (62 51 188) — pin `498b87880200004881c08801000049898788020000c3`
- Plus 1 Relock after append from pin `69f1bb2f…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-076 serialize PASSes + 1 Relock
