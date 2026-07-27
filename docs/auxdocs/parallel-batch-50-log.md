# parallel-batch-50 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-50-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-055 (pin `fba1f97e…`, handlers = 380, H_366..H_373 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-055 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_373 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xF8 | `498b87000300004881c0f8000000480fb60049898780020000c3` (26) | same | same | Y | `58d6062a26266dd7` | `58d6062a26266dd7` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xF8 | `498b87000300004881c0f8000000480fb60049898788020000c3` (26) | same | same | Y | `03ca25f17de5059c` | `03ca25f17de5059c` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xF8 | `498b87000300004881c0f8000000480fb60049898790020000c3` (26) | same | same | Y | `a94d6b39ac0bfbcd` | `a94d6b39ac0bfbcd` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=0xF8 | `498b87800200004881c0f800000049898780020000c3` (22) | same | same | Y | `5179a4fbad6d4cda` | `5179a4fbad6d4cda` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x51 imm=0xF8 | `498b87880200004881c0f800000049898788020000c3` (22) | same | same | Y | `4670b7c563c506d0` | `4670b7c563c506d0` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0xF8 | `498b87900200004881c0f800000049898790020000c3` (22) | same | same | Y | `c84a511509fceff1` | `c84a511509fceff1` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0xF8 | `498b87800200004881e8f800000049898780020000c3` (22) | same | same | Y | `9ffb9228f48ec264` | `9ffb9228f48ec264` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0xF8 | `498b87880200004881e8f800000049898788020000c3` (22) | same | same | Y | `dbb8d1ae964b7218` | `dbb8d1ae964b7218` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x50 ss=0x60 oo=0xF8 — **PASS**

- fixture: `_scratch_ldb_5060_f8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f8000000480fb60049898780020000c3`
- js-sha256: `58d6062a26266dd726f7fe903ed872a53097e76cb8c1646e361a639a6bc8ac20`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0xF8 — **PASS**

- fixture: `_scratch_ldb_5160_f8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f8000000480fb60049898788020000c3`
- js-sha256: `03ca25f17de5059cc8205e34937baaee8eadf73082fa32ecc97870ffc23b752f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0xF8 — **PASS**

- fixture: `_scratch_ldb_5260_f8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0f8000000480fb60049898790020000c3`
- js-sha256: `a94d6b39ac0bfbcd810dcbbff961594e565bbdb2effa1269e6779871594477bd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=0xF8 — **PASS**

- fixture: `_scratch_addimm_h50_f8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0f800000049898780020000c3`
- js-sha256: `5179a4fbad6d4cdae7c1b54b9ce95ba981f453ce5a2be75ba5e38d4effcb2b0e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x51 imm=0xF8 — **PASS**

- fixture: `_scratch_addimm_h51_f8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0f800000049898788020000c3`
- js-sha256: `4670b7c563c506d01ede9f962190ca8badc5cf631854d12917401214a61a2d0c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0xF8 — **PASS**

- fixture: `_scratch_addimm_h52_f8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0f800000049898790020000c3`
- js-sha256: `c84a511509fceff17e958e241f8f0dab1ad2241b221e8043b0dd2dd04b2afc99`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0xF8 — **PASS**

- fixture: `_scratch_subimm_h50_f8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8f800000049898780020000c3`
- js-sha256: `9ffb9228f48ec264d8e4dfac651be3427eb69c82ccdb4ff9cc99b69892f6946a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0xF8 — **PASS**

- fixture: `_scratch_subimm_h51_f8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8f800000049898788020000c3`
- js-sha256: `dbb8d1ae964b7218ed4aeb5d56e58a15797ee4527effb61b24f0755fb8751939`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=50/51/52 ss=60 oo=F8 (next oo after F0 triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=F8 (fresh imm after F0; imm32 22B).
- SUB-IMM slot=50/51 imm=F8 (complements ADD-IMM * F8; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 17C`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5060_f8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_f8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_f8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_f8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_f8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_f8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_f8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_f8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-50-log.md` — this file
- `scripts/_probe/parallel-batch-50-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-056 serialize PASSes + 1 Relock**

Pass pin from body-extend-055 Relock: `fba1f97e01a9ef7e6285451fe34b6b52a972caf99ae81f93518563d7eb1ec442`.
Handlers before consolidate = 380 (H_00..H_373). Next selectors `40 17C`.. for H_374.. if all serialize.

PASS list for body-extend-056:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_374 | 0x17C | 0x80 LDB | 0x50 0x60 0xF8 | `498b87000300004881c0f8000000480fb60049898780020000c3` (26B) | `58d6062a26266dd7` |
| H_375 | 0x17D | 0x80 LDB | 0x51 0x60 0xF8 | `498b87000300004881c0f8000000480fb60049898788020000c3` (26B) | `03ca25f17de5059c` |
| H_376 | 0x17E | 0x80 LDB | 0x52 0x60 0xF8 | `498b87000300004881c0f8000000480fb60049898790020000c3` (26B) | `a94d6b39ac0bfbcd` |
| H_377 | 0x17F | 0x62 ADD-IMM | 0x50 0xF8 | `498b87800200004881c0f800000049898780020000c3` (22B) | `5179a4fbad6d4cda` |
| H_378 | 0x180 | 0x62 ADD-IMM | 0x51 0xF8 | `498b87880200004881c0f800000049898788020000c3` (22B) | `4670b7c563c506d0` |
| H_379 | 0x181 | 0x62 ADD-IMM | 0x52 0xF8 | `498b87900200004881c0f800000049898790020000c3` (22B) | `c84a511509fceff1` |
| H_380 | 0x182 | 0x61 SUB-IMM | 0x50 0xF8 | `498b87800200004881e8f800000049898780020000c3` (22B) | `9ffb9228f48ec264` |
| H_381 | 0x183 | 0x61 SUB-IMM | 0x51 0xF8 | `498b87880200004881e8f800000049898788020000c3` (22B) | `dbb8d1ae964b7218` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-055 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_373.
- If the parent decides to serialize, append H_374.. at selectors `40 17C`..:
  - H_374 0x80 LDB (80 50 60 F8) — pin `498b87000300004881c0f8000000480fb60049898780020000c3`
  - H_375 0x80 LDB (80 51 60 F8) — pin `498b87000300004881c0f8000000480fb60049898788020000c3`
  - H_376 0x80 LDB (80 52 60 F8) — pin `498b87000300004881c0f8000000480fb60049898790020000c3`
  - H_377 0x62 ADD-IMM (62 50 F8) — pin `498b87800200004881c0f800000049898780020000c3`
  - H_378 0x62 ADD-IMM (62 51 F8) — pin `498b87880200004881c0f800000049898788020000c3`
  - H_379 0x62 ADD-IMM (62 52 F8) — pin `498b87900200004881c0f800000049898790020000c3`
  - H_380 0x61 SUB-IMM (61 50 F8) — pin `498b87800200004881e8f800000049898780020000c3`
  - H_381 0x61 SUB-IMM (61 51 F8) — pin `498b87880200004881e8f800000049898788020000c3`
- Plus 1 Relock after append from pin `fba1f97e…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-056 serialize PASSes + 1 Relock
