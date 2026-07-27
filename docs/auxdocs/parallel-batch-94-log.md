# parallel-batch-94 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-94-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-099 (pin `3fc618f9…`, handlers = 731, H_717..H_724 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-099 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_724 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x230 | `498b87000300004881c030020000480fb60049898780020000c3` (26) | same | same | Y | `5c33c35f5fd9760b` | `5c33c35f5fd9760b` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x230 | `498b87000300004881c030020000480fb60049898788020000c3` (26) | same | same | Y | `e574a865427adbbc` | `e574a865427adbbc` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x230 | `498b87000300004881c030020000480fb60049898790020000c3` (26) | same | same | Y | `b066434bf619727b` | `b066434bf619727b` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=0x230 | `498b87800200004881c03002000049898780020000c3` (22) | same | same | Y | `a2091c2a78abf623` | `a2091c2a78abf623` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x51 imm=0x230 | `498b87880200004881c03002000049898788020000c3` (22) | same | same | Y | `e5fd2243f67268fd` | `e5fd2243f67268fd` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0x230 | `498b87900200004881c03002000049898790020000c3` (22) | same | same | Y | `7fb8cfe02d18cafc` | `7fb8cfe02d18cafc` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x50 imm=0x230 | `498b87800200004881e83002000049898780020000c3` (22) | same | same | Y | `6057b3357f248ea9` | `6057b3357f248ea9` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0x230 | `498b87880200004881e83002000049898788020000c3` (22) | same | same | Y | `7b44c3ca05a14832` | `7b44c3ca05a14832` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x50 ss=0x60 oo=0x230 — **PASS**

- fixture: `_scratch_ldb_5060_230.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c030020000480fb60049898780020000c3`
- js-sha256: `5c33c35f5fd9760bff127c50a7190077e8179b418de6f54d667aa08bdc69f34b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0x230 — **PASS**

- fixture: `_scratch_ldb_5160_230.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c030020000480fb60049898788020000c3`
- js-sha256: `e574a865427adbbcc9fceca251487fa299ab80135e0abbf0be72094e6d08331b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0x230 — **PASS**

- fixture: `_scratch_ldb_5260_230.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c030020000480fb60049898790020000c3`
- js-sha256: `b066434bf619727bfc38fae08251b8be38f39b2d12d9ae89e23f066de712ce77`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=0x230 — **PASS**

- fixture: `_scratch_addimm_h50_230.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c03002000049898780020000c3`
- js-sha256: `a2091c2a78abf623843bf75666d8bbed15a0959397a01bef4e66efd2fcca4f1a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x51 imm=0x230 — **PASS**

- fixture: `_scratch_addimm_h51_230.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c03002000049898788020000c3`
- js-sha256: `e5fd2243f67268fde53d644b88bec57be0e74d8b0db703b799b9d2d6b6165a18`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0x230 — **PASS**

- fixture: `_scratch_addimm_h52_230.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c03002000049898790020000c3`
- js-sha256: `7fb8cfe02d18cafc02e295bf962c249fa332e1c18499b482aac3a18a6f369905`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x50 imm=0x230 — **PASS**

- fixture: `_scratch_subimm_h50_230.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e83002000049898780020000c3`
- js-sha256: `6057b3357f248ea9b548c111e3b2ce2a2b8b17a8a385e592eda7998f822642f8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0x230 — **PASS**

- fixture: `_scratch_subimm_h51_230.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e83002000049898788020000c3`
- js-sha256: `7b44c3ca05a148325d4d462a1a872b18bf20c1994a1c0b3551bb0ebef49583d7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=50/51/52 ss=60 oo=230 (start deferred 230 LDB ladder; imm32 26B).
- ADD-IMM slot=50/51/52 imm=230 (start deferred 230 ADD triad; imm32 22B).
- SUB-IMM slot=50/51 imm=230 (start 230 SUB triad; imm32 22B; SUB 52 deferred / substitute).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 2DB`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5060_230.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_230.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_230.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_230.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_230.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_230.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_230.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_230.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-94-log.md` — this file
- `scripts/_probe/parallel-batch-94-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-100 serialize PASSes + 1 Relock**

Pass pin from body-extend-099 Relock: `3fc618f9e14a881a91460a8c1be733bade35794eca50282f64c5eb0cabb57021`.
Handlers before consolidate = 731 (H_00..H_724). Next selectors `40 2DB`.. for H_725.. if all serialize.

PASS list for body-extend-100:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_725 | 0x2DB | 0x80 LDB | 0x50 0x60 0x230 | `498b87000300004881c030020000480fb60049898780020000c3` (26B) | `5c33c35f5fd9760b` |
| H_726 | 0x2DC | 0x80 LDB | 0x51 0x60 0x230 | `498b87000300004881c030020000480fb60049898788020000c3` (26B) | `e574a865427adbbc` |
| H_727 | 0x2DD | 0x80 LDB | 0x52 0x60 0x230 | `498b87000300004881c030020000480fb60049898790020000c3` (26B) | `b066434bf619727b` |
| H_728 | 0x2DE | 0x62 ADD-IMM | 0x50 0x230 | `498b87800200004881c03002000049898780020000c3` (22B) | `a2091c2a78abf623` |
| H_729 | 0x2DF | 0x62 ADD-IMM | 0x51 0x230 | `498b87880200004881c03002000049898788020000c3` (22B) | `e5fd2243f67268fd` |
| H_730 | 0x2E0 | 0x62 ADD-IMM | 0x52 0x230 | `498b87900200004881c03002000049898790020000c3` (22B) | `7fb8cfe02d18cafc` |
| H_731 | 0x2E1 | 0x61 SUB-IMM | 0x50 0x230 | `498b87800200004881e83002000049898780020000c3` (22B) | `6057b3357f248ea9` |
| H_732 | 0x2E2 | 0x61 SUB-IMM | 0x51 0x230 | `498b87880200004881e83002000049898788020000c3` (22B) | `7b44c3ca05a14832` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-099 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_724.
- If the parent decides to serialize, append H_725.. at selectors `40 2DB`..:
  - H_725 0x80 LDB (80 50 60 230) — pin `498b87000300004881c030020000480fb60049898780020000c3`
  - H_726 0x80 LDB (80 51 60 230) — pin `498b87000300004881c030020000480fb60049898788020000c3`
  - H_727 0x80 LDB (80 52 60 230) — pin `498b87000300004881c030020000480fb60049898790020000c3`
  - H_728 0x62 ADD-IMM (62 50 230) — pin `498b87800200004881c03002000049898780020000c3`
  - H_729 0x62 ADD-IMM (62 51 230) — pin `498b87880200004881c03002000049898788020000c3`
  - H_730 0x62 ADD-IMM (62 52 230) — pin `498b87900200004881c03002000049898790020000c3`
  - H_731 0x61 SUB-IMM (61 50 230) — pin `498b87800200004881e83002000049898780020000c3`
  - H_732 0x61 SUB-IMM (61 51 230) — pin `498b87880200004881e83002000049898788020000c3`
- Plus 1 Relock after append from pin `3fc618f9…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: SUB-IMM 52 230; SET/GET/ORV/SUBV/ADDV/IMUL fresh; next ladder if continuing.

## §7. Consolidation handoff

parent next = body-extend-100 serialize PASSes + 1 Relock
