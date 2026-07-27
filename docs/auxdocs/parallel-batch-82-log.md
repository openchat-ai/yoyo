# parallel-batch-82 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-82-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-087 (pin `db550629…`, handlers = 635, H_621..H_628 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-087 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_628 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x50 imm=0x1D8 | `498b87800200004881c0d801000049898780020000c3` (22) | same | same | Y | `985fc739129b28e5` | `985fc739129b28e5` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x51 imm=0x1D8 | `498b87880200004881c0d801000049898788020000c3` (22) | same | same | Y | `529c91e6cee0c610` | `529c91e6cee0c610` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x52 imm=0x1D8 | `498b87900200004881c0d801000049898790020000c3` (22) | same | same | Y | `11e5e0737f59a060` | `11e5e0737f59a060` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x1D8 | `498b87800200004881e8d801000049898780020000c3` (22) | same | same | Y | `a387a1d628c84d7e` | `a387a1d628c84d7e` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x51 imm=0x1D8 | `498b87880200004881e8d801000049898788020000c3` (22) | same | same | Y | `f7f546cac9fd3bab` | `f7f546cac9fd3bab` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x52 imm=0x1D8 | `498b87900200004881e8d801000049898790020000c3` (22) | same | same | Y | `9f9c8525bbf76801` | `9f9c8525bbf76801` | PASS |
| 7 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1E0 | `498b87000300004881c0e0010000480fb60049898780020000c3` (26) | same | same | Y | `54ae10749db49954` | `54ae10749db49954` | PASS |
| 8 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1E0 | `498b87000300004881c0e0010000480fb60049898788020000c3` (26) | same | same | Y | `fab08f3c3976d127` | `fab08f3c3976d127` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x50 imm=0x1D8 — **PASS**

- fixture: `_scratch_addimm_h50_1D8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0d801000049898780020000c3`
- js-sha256: `985fc739129b28e5206f7a44af2242038d962dfab346dc8508df252fec254a18`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x51 imm=0x1D8 — **PASS**

- fixture: `_scratch_addimm_h51_1D8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0d801000049898788020000c3`
- js-sha256: `529c91e6cee0c6109a96db3ce2e03499601a8842698be17f37b7ae849b66ceb7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x52 imm=0x1D8 — **PASS**

- fixture: `_scratch_addimm_h52_1D8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0d801000049898790020000c3`
- js-sha256: `11e5e0737f59a0604afe87cbec770b18cd8c096583d1007d45c2129506c8dccd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x1D8 — **PASS**

- fixture: `_scratch_subimm_h50_1D8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8d801000049898780020000c3`
- js-sha256: `a387a1d628c84d7eb3f1149a46c9d3568e2c7fa70e19b289f1fdc4d11735d875`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x51 imm=0x1D8 — **PASS**

- fixture: `_scratch_subimm_h51_1D8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8d801000049898788020000c3`
- js-sha256: `f7f546cac9fd3bab9ba41979b7477e535b189d1c59de5113ebea881f368f0c42`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x52 imm=0x1D8 — **PASS**

- fixture: `_scratch_subimm_h52_1D8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8d801000049898790020000c3`
- js-sha256: `9f9c8525bbf768014398e291dffe27d2523d9350d2b10d71c990bfafd40ff3bd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x50 ss=0x60 oo=0x1E0 — **PASS**

- fixture: `_scratch_ldb_5060_1E0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e0010000480fb60049898780020000c3`
- js-sha256: `54ae10749db49954ff3e0f998b123304cb8bdc5d88a0b54a728d95efb27044e9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x51 ss=0x60 oo=0x1E0 — **PASS**

- fixture: `_scratch_ldb_5160_1E0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e0010000480fb60049898788020000c3`
- js-sha256: `fab08f3c3976d127fad45fef564cdb0b17def5621b4350fcc26df85fd86693d0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=50/51/52 imm=1D8 (start deferred 1D8 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=1D8 (start deferred 1D8 SUB triad; imm32 22B).
- LDB dd=50/51 ss=60 oo=1E0 (start deferred 1E0 LDB triad; imm32 26B; LDB 52 1E0 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 27B`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h50_1D8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1D8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1D8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1D8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1D8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1D8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_1E0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_1E0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-82-log.md` — this file
- `scripts/_probe/parallel-batch-82-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-088 serialize PASSes + 1 Relock**

Pass pin from body-extend-087 Relock: `db550629db78a974cd83bec8db879fec415cd6fe37c94b35f57ce10a6917010d`.
Handlers before consolidate = 635 (H_00..H_628). Next selectors `40 27B`.. for H_629.. if all serialize.

PASS list for body-extend-088:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_629 | 0x27B | 0x62 ADD-IMM | 0x50 0x1D8 | `498b87800200004881c0d801000049898780020000c3` (22B) | `985fc739129b28e5` |
| H_630 | 0x27C | 0x62 ADD-IMM | 0x51 0x1D8 | `498b87880200004881c0d801000049898788020000c3` (22B) | `529c91e6cee0c610` |
| H_631 | 0x27D | 0x62 ADD-IMM | 0x52 0x1D8 | `498b87900200004881c0d801000049898790020000c3` (22B) | `11e5e0737f59a060` |
| H_632 | 0x27E | 0x61 SUB-IMM | 0x50 0x1D8 | `498b87800200004881e8d801000049898780020000c3` (22B) | `a387a1d628c84d7e` |
| H_633 | 0x27F | 0x61 SUB-IMM | 0x51 0x1D8 | `498b87880200004881e8d801000049898788020000c3` (22B) | `f7f546cac9fd3bab` |
| H_634 | 0x280 | 0x61 SUB-IMM | 0x52 0x1D8 | `498b87900200004881e8d801000049898790020000c3` (22B) | `9f9c8525bbf76801` |
| H_635 | 0x281 | 0x80 LDB | 0x50 0x60 0x1E0 | `498b87000300004881c0e0010000480fb60049898780020000c3` (26B) | `54ae10749db49954` |
| H_636 | 0x282 | 0x80 LDB | 0x51 0x60 0x1E0 | `498b87000300004881c0e0010000480fb60049898788020000c3` (26B) | `fab08f3c3976d127` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-087 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_628.
- If the parent decides to serialize, append H_629.. at selectors `40 27B`..:
  - H_629 0x62 ADD-IMM (62 50 1D8) — pin `498b87800200004881c0d801000049898780020000c3`
  - H_630 0x62 ADD-IMM (62 51 1D8) — pin `498b87880200004881c0d801000049898788020000c3`
  - H_631 0x62 ADD-IMM (62 52 1D8) — pin `498b87900200004881c0d801000049898790020000c3`
  - H_632 0x61 SUB-IMM (61 50 1D8) — pin `498b87800200004881e8d801000049898780020000c3`
  - H_633 0x61 SUB-IMM (61 51 1D8) — pin `498b87880200004881e8d801000049898788020000c3`
  - H_634 0x61 SUB-IMM (61 52 1D8) — pin `498b87900200004881e8d801000049898790020000c3`
  - H_635 0x80 LDB (80 50 60 1E0) — pin `498b87000300004881c0e0010000480fb60049898780020000c3`
  - H_636 0x80 LDB (80 51 60 1E0) — pin `498b87000300004881c0e0010000480fb60049898788020000c3`
- Plus 1 Relock after append from pin `db550629…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-088 serialize PASSes + 1 Relock
