# parallel-batch-69 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-69-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-074 (pin `9243965c…`, handlers = 531, H_517..H_524 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-074 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_524 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x51 imm=0x178 | `498b87880200004881e87801000049898788020000c3` (22) | same | same | Y | `c18f2917305b68fa` | `c18f2917305b68fa` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x52 imm=0x178 | `498b87900200004881e87801000049898790020000c3` (22) | same | same | Y | `29f631d8a2fd2ed7` | `29f631d8a2fd2ed7` | PASS |
| 3 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x180 | `498b87000300004881c080010000480fb60049898780020000c3` (26) | same | same | Y | `b83050617eb70487` | `b83050617eb70487` | PASS |
| 4 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x180 | `498b87000300004881c080010000480fb60049898788020000c3` (26) | same | same | Y | `8905cf5ed3ca338f` | `8905cf5ed3ca338f` | PASS |
| 5 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x180 | `498b87000300004881c080010000480fb60049898790020000c3` (26) | same | same | Y | `c958b80396d606de` | `c958b80396d606de` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x50 imm=0x180 | `498b87800200004881c08001000049898780020000c3` (22) | same | same | Y | `1c96efa23061fbf4` | `1c96efa23061fbf4` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x51 imm=0x180 | `498b87880200004881c08001000049898788020000c3` (22) | same | same | Y | `8732710ac0cc4d60` | `8732710ac0cc4d60` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x52 imm=0x180 | `498b87900200004881c08001000049898790020000c3` (22) | same | same | Y | `b32b4364c0efbe04` | `b32b4364c0efbe04` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x51 imm=0x178 — **PASS**

- fixture: `_scratch_subimm_h51_178.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e87801000049898788020000c3`
- js-sha256: `c18f2917305b68faf3ecee09ce6d76e94a17b19bc14fafdc604b2ae3d1fa8aaf`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x52 imm=0x178 — **PASS**

- fixture: `_scratch_subimm_h52_178.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e87801000049898790020000c3`
- js-sha256: `29f631d8a2fd2ed7623889a6357eeac2bdf4327d41dc274dd257293f917a01e5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x50 ss=0x60 oo=0x180 — **PASS**

- fixture: `_scratch_ldb_5060_180.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c080010000480fb60049898780020000c3`
- js-sha256: `b83050617eb70487ee68e88992b6b35c8863ed9424cde661db3f4d51ebc8fb36`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x51 ss=0x60 oo=0x180 — **PASS**

- fixture: `_scratch_ldb_5160_180.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c080010000480fb60049898788020000c3`
- js-sha256: `8905cf5ed3ca338ffc6c372b6ab744ec6dd06850be653613669b8320543ecd9b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x52 ss=0x60 oo=0x180 — **PASS**

- fixture: `_scratch_ldb_5260_180.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c080010000480fb60049898790020000c3`
- js-sha256: `c958b80396d606de3757923cfb2dea5e473ff1337ad4a674e9ba06f408b5268b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x50 imm=0x180 — **PASS**

- fixture: `_scratch_addimm_h50_180.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c08001000049898780020000c3`
- js-sha256: `1c96efa23061fbf4a12fbd69e1e45d5e250cd8f825074da7f8b82f938a348106`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x51 imm=0x180 — **PASS**

- fixture: `_scratch_addimm_h51_180.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c08001000049898788020000c3`
- js-sha256: `8732710ac0cc4d604d003e0d4b83bf8fa16ebc22c512b3bb2d9a2a4132fece6a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x52 imm=0x180 — **PASS**

- fixture: `_scratch_addimm_h52_180.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c08001000049898790020000c3`
- js-sha256: `b32b4364c0efbe042ce84930704039fd1849e30c1755dd1e73fc2136376b8c09`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=51/52 imm=178 (finish deferred 178 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=180 (start 180 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=180 (start 180 ADD triad; imm32 22B).
- SUB-IMM 50/51/52 imm=180 deferred to a later scratch batch.
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 213`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h51_178.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_178.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_180.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_180.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_180.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_180.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_180.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_180.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-69-log.md` — this file
- `scripts/_probe/parallel-batch-69-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-075 serialize PASSes + 1 Relock**

Pass pin from body-extend-074 Relock: `9243965c886555e99575615e4637331b6c2a49573d50ec183fb616c3ae3d2d98`.
Handlers before consolidate = 531 (H_00..H_524). Next selectors `40 213`.. for H_525.. if all serialize.

PASS list for body-extend-075:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_525 | 0x213 | 0x61 SUB-IMM | 0x51 0x178 | `498b87880200004881e87801000049898788020000c3` (22B) | `c18f2917305b68fa` |
| H_526 | 0x214 | 0x61 SUB-IMM | 0x52 0x178 | `498b87900200004881e87801000049898790020000c3` (22B) | `29f631d8a2fd2ed7` |
| H_527 | 0x215 | 0x80 LDB | 0x50 0x60 0x180 | `498b87000300004881c080010000480fb60049898780020000c3` (26B) | `b83050617eb70487` |
| H_528 | 0x216 | 0x80 LDB | 0x51 0x60 0x180 | `498b87000300004881c080010000480fb60049898788020000c3` (26B) | `8905cf5ed3ca338f` |
| H_529 | 0x217 | 0x80 LDB | 0x52 0x60 0x180 | `498b87000300004881c080010000480fb60049898790020000c3` (26B) | `c958b80396d606de` |
| H_530 | 0x218 | 0x62 ADD-IMM | 0x50 0x180 | `498b87800200004881c08001000049898780020000c3` (22B) | `1c96efa23061fbf4` |
| H_531 | 0x219 | 0x62 ADD-IMM | 0x51 0x180 | `498b87880200004881c08001000049898788020000c3` (22B) | `8732710ac0cc4d60` |
| H_532 | 0x21A | 0x62 ADD-IMM | 0x52 0x180 | `498b87900200004881c08001000049898790020000c3` (22B) | `b32b4364c0efbe04` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-074 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_524.
- If the parent decides to serialize, append H_525.. at selectors `40 213`..:
  - H_525 0x61 SUB-IMM (61 51 178) — pin `498b87880200004881e87801000049898788020000c3`
  - H_526 0x61 SUB-IMM (61 52 178) — pin `498b87900200004881e87801000049898790020000c3`
  - H_527 0x80 LDB (80 50 60 180) — pin `498b87000300004881c080010000480fb60049898780020000c3`
  - H_528 0x80 LDB (80 51 60 180) — pin `498b87000300004881c080010000480fb60049898788020000c3`
  - H_529 0x80 LDB (80 52 60 180) — pin `498b87000300004881c080010000480fb60049898790020000c3`
  - H_530 0x62 ADD-IMM (62 50 180) — pin `498b87800200004881c08001000049898780020000c3`
  - H_531 0x62 ADD-IMM (62 51 180) — pin `498b87880200004881c08001000049898788020000c3`
  - H_532 0x62 ADD-IMM (62 52 180) — pin `498b87900200004881c08001000049898790020000c3`
- Plus 1 Relock after append from pin `9243965c…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-075 serialize PASSes + 1 Relock
