# parallel-batch-77 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-77-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-082 (pin `05a3a9c6…`, handlers = 595, H_581..H_588 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-082 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_588 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x52 imm=0x1B0 | `498b87900200004881e8b001000049898790020000c3` (22) | same | same | Y | `6b09f5d585880e4e` | `6b09f5d585880e4e` | PASS |
| 2 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1B8 | `498b87000300004881c0b8010000480fb60049898780020000c3` (26) | same | same | Y | `991bc7cddb01b0d2` | `991bc7cddb01b0d2` | PASS |
| 3 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1B8 | `498b87000300004881c0b8010000480fb60049898788020000c3` (26) | same | same | Y | `eb823184d5b340f6` | `eb823184d5b340f6` | PASS |
| 4 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1B8 | `498b87000300004881c0b8010000480fb60049898790020000c3` (26) | same | same | Y | `4769bc5c1af2f770` | `4769bc5c1af2f770` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x50 imm=0x1B8 | `498b87800200004881c0b801000049898780020000c3` (22) | same | same | Y | `8670afebb32cc65e` | `8670afebb32cc65e` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x51 imm=0x1B8 | `498b87880200004881c0b801000049898788020000c3` (22) | same | same | Y | `46ee1e357ab8ae14` | `46ee1e357ab8ae14` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x52 imm=0x1B8 | `498b87900200004881c0b801000049898790020000c3` (22) | same | same | Y | `a95def3bbb47b285` | `a95def3bbb47b285` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x50 imm=0x1B8 | `498b87800200004881e8b801000049898780020000c3` (22) | same | same | Y | `ab8ef8aa14a41432` | `ab8ef8aa14a41432` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x52 imm=0x1B0 — **PASS**

- fixture: `_scratch_subimm_h52_1B0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8b001000049898790020000c3`
- js-sha256: `6b09f5d585880e4ec49c5e9c396bb9e39446613f5363674617689869bab33bfd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x50 ss=0x60 oo=0x1B8 — **PASS**

- fixture: `_scratch_ldb_5060_1B8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b8010000480fb60049898780020000c3`
- js-sha256: `991bc7cddb01b0d29dfca5fb319b68256961b1812f00b88ae985882b39c38998`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x51 ss=0x60 oo=0x1B8 — **PASS**

- fixture: `_scratch_ldb_5160_1B8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b8010000480fb60049898788020000c3`
- js-sha256: `eb823184d5b340f60c461778e7ab3ca2948f25a3d027fced281183e2a49913ac`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x52 ss=0x60 oo=0x1B8 — **PASS**

- fixture: `_scratch_ldb_5260_1B8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b8010000480fb60049898790020000c3`
- js-sha256: `4769bc5c1af2f770c1056bfb8170c204314d60ca96624d72323238d6c7ecdca2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x50 imm=0x1B8 — **PASS**

- fixture: `_scratch_addimm_h50_1B8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0b801000049898780020000c3`
- js-sha256: `8670afebb32cc65e32e1ba06b08b165b09792c2cf045673023d42964c261b24a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x51 imm=0x1B8 — **PASS**

- fixture: `_scratch_addimm_h51_1B8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0b801000049898788020000c3`
- js-sha256: `46ee1e357ab8ae141ae1fa9df0aa3354ca8f50bece5207409dbfc33237b3f516`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x52 imm=0x1B8 — **PASS**

- fixture: `_scratch_addimm_h52_1B8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0b801000049898790020000c3`
- js-sha256: `a95def3bbb47b2852075f9b5eaa1b01aa135e2f472516fd07162777a3915252b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x50 imm=0x1B8 — **PASS**

- fixture: `_scratch_subimm_h50_1B8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8b801000049898780020000c3`
- js-sha256: `ab8ef8aa14a41432c50fdfcce3a68657f56c95ef810e7b0fa42ccc36b3374b31`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=52 imm=1B0 (finish deferred 1B0 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=1B8 (start deferred 1B8 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=1B8 (start deferred 1B8 ADD triad; imm32 22B).
- SUB-IMM slot=50 imm=1B8 (start deferred 1B8 SUB triad; slots 51/52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 253`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h52_1B0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_1B8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_1B8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_1B8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1B8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1B8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1B8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1B8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-77-log.md` — this file
- `scripts/_probe/parallel-batch-77-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-083 serialize PASSes + 1 Relock**

Pass pin from body-extend-082 Relock: `05a3a9c6693fa65c20f47a3eab1bc536c5e5fe0a168381faf0cf72330ca58056`.
Handlers before consolidate = 595 (H_00..H_588). Next selectors `40 253`.. for H_589.. if all serialize.

PASS list for body-extend-083:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_589 | 0x253 | 0x61 SUB-IMM | 0x52 0x1B0 | `498b87900200004881e8b001000049898790020000c3` (22B) | `6b09f5d585880e4e` |
| H_590 | 0x254 | 0x80 LDB | 0x50 0x60 0x1B8 | `498b87000300004881c0b8010000480fb60049898780020000c3` (26B) | `991bc7cddb01b0d2` |
| H_591 | 0x255 | 0x80 LDB | 0x51 0x60 0x1B8 | `498b87000300004881c0b8010000480fb60049898788020000c3` (26B) | `eb823184d5b340f6` |
| H_592 | 0x256 | 0x80 LDB | 0x52 0x60 0x1B8 | `498b87000300004881c0b8010000480fb60049898790020000c3` (26B) | `4769bc5c1af2f770` |
| H_593 | 0x257 | 0x62 ADD-IMM | 0x50 0x1B8 | `498b87800200004881c0b801000049898780020000c3` (22B) | `8670afebb32cc65e` |
| H_594 | 0x258 | 0x62 ADD-IMM | 0x51 0x1B8 | `498b87880200004881c0b801000049898788020000c3` (22B) | `46ee1e357ab8ae14` |
| H_595 | 0x259 | 0x62 ADD-IMM | 0x52 0x1B8 | `498b87900200004881c0b801000049898790020000c3` (22B) | `a95def3bbb47b285` |
| H_596 | 0x25A | 0x61 SUB-IMM | 0x50 0x1B8 | `498b87800200004881e8b801000049898780020000c3` (22B) | `ab8ef8aa14a41432` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-082 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_588.
- If the parent decides to serialize, append H_589.. at selectors `40 253`..:
  - H_589 0x61 SUB-IMM (61 52 1B0) — pin `498b87900200004881e8b001000049898790020000c3`
  - H_590 0x80 LDB (80 50 60 1B8) — pin `498b87000300004881c0b8010000480fb60049898780020000c3`
  - H_591 0x80 LDB (80 51 60 1B8) — pin `498b87000300004881c0b8010000480fb60049898788020000c3`
  - H_592 0x80 LDB (80 52 60 1B8) — pin `498b87000300004881c0b8010000480fb60049898790020000c3`
  - H_593 0x62 ADD-IMM (62 50 1B8) — pin `498b87800200004881c0b801000049898780020000c3`
  - H_594 0x62 ADD-IMM (62 51 1B8) — pin `498b87880200004881c0b801000049898788020000c3`
  - H_595 0x62 ADD-IMM (62 52 1B8) — pin `498b87900200004881c0b801000049898790020000c3`
  - H_596 0x61 SUB-IMM (61 50 1B8) — pin `498b87800200004881e8b801000049898780020000c3`
- Plus 1 Relock after append from pin `05a3a9c6…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-083 serialize PASSes + 1 Relock
