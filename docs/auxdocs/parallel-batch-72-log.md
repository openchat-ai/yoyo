# parallel-batch-72 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-72-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-077 (pin `97ce84a2…`, handlers = 555, H_541..H_548 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-077 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_548 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x51 imm=0x190 | `498b87880200004881c09001000049898788020000c3` (22) | same | same | Y | `5248421affee5c66` | `5248421affee5c66` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x52 imm=0x190 | `498b87900200004881c09001000049898790020000c3` (22) | same | same | Y | `648351f8db48af34` | `648351f8db48af34` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x190 | `498b87800200004881e89001000049898780020000c3` (22) | same | same | Y | `f7e06d035b717d9d` | `f7e06d035b717d9d` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x51 imm=0x190 | `498b87880200004881e89001000049898788020000c3` (22) | same | same | Y | `489b9cd85b80cad9` | `489b9cd85b80cad9` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x52 imm=0x190 | `498b87900200004881e89001000049898790020000c3` (22) | same | same | Y | `0535305934d986e2` | `0535305934d986e2` | PASS |
| 6 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x198 | `498b87000300004881c098010000480fb60049898780020000c3` (26) | same | same | Y | `f68f3fdd889f57db` | `f68f3fdd889f57db` | PASS |
| 7 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x198 | `498b87000300004881c098010000480fb60049898788020000c3` (26) | same | same | Y | `1fd1cefc37ee2f6a` | `1fd1cefc37ee2f6a` | PASS |
| 8 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x198 | `498b87000300004881c098010000480fb60049898790020000c3` (26) | same | same | Y | `84e2d29d21835c65` | `84e2d29d21835c65` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x51 imm=0x190 — **PASS**

- fixture: `_scratch_addimm_h51_190.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c09001000049898788020000c3`
- js-sha256: `5248421affee5c6657036a849aa23a1e5dfc1fe09c3c78d6ba06947038c8fccb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x52 imm=0x190 — **PASS**

- fixture: `_scratch_addimm_h52_190.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c09001000049898790020000c3`
- js-sha256: `648351f8db48af34920624da1b6b8d4df396537d997f5d58ddcb7aaa1c9dfe13`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x190 — **PASS**

- fixture: `_scratch_subimm_h50_190.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e89001000049898780020000c3`
- js-sha256: `f7e06d035b717d9d783242b3bd9592372b879c5e7a3430e5fec96e4651567ef0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x51 imm=0x190 — **PASS**

- fixture: `_scratch_subimm_h51_190.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e89001000049898788020000c3`
- js-sha256: `489b9cd85b80cad9d028bde2952f8ea33130dce8590b16075ac241d7ba5db55e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x52 imm=0x190 — **PASS**

- fixture: `_scratch_subimm_h52_190.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e89001000049898790020000c3`
- js-sha256: `0535305934d986e2de06cc9fba0add950c71f2de2df62c86f61efec012dce7b7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x50 ss=0x60 oo=0x198 — **PASS**

- fixture: `_scratch_ldb_5060_198.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c098010000480fb60049898780020000c3`
- js-sha256: `f68f3fdd889f57db0bd956d924302f6e0b15d18498ec45b3ba3ad6c3b0f2637a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x51 ss=0x60 oo=0x198 — **PASS**

- fixture: `_scratch_ldb_5160_198.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c098010000480fb60049898788020000c3`
- js-sha256: `1fd1cefc37ee2f6a1243b5a4886c08ecaf023e56b8d3e08babd762e4ba45fd3b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x52 ss=0x60 oo=0x198 — **PASS**

- fixture: `_scratch_ldb_5260_198.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c098010000480fb60049898790020000c3`
- js-sha256: `84e2d29d21835c6502cff74e89e6fdc5c3bec404c1e20379cc6eb1af9d7cb26d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=51/52 imm=190 (finish deferred 190 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=190 (start 190 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=198 (start 198 LDB triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 22B`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h51_190.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_190.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_190.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_190.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_190.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_198.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_198.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_198.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-72-log.md` — this file
- `scripts/_probe/parallel-batch-72-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-078 serialize PASSes + 1 Relock**

Pass pin from body-extend-077 Relock: `97ce84a29adb8c400408d7fec9d2d58a820766a61c18068b1b61eac59946e2b0`.
Handlers before consolidate = 555 (H_00..H_548). Next selectors `40 22B`.. for H_549.. if all serialize.

PASS list for body-extend-078:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_549 | 0x22B | 0x62 ADD-IMM | 0x51 0x190 | `498b87880200004881c09001000049898788020000c3` (22B) | `5248421affee5c66` |
| H_550 | 0x22C | 0x62 ADD-IMM | 0x52 0x190 | `498b87900200004881c09001000049898790020000c3` (22B) | `648351f8db48af34` |
| H_551 | 0x22D | 0x61 SUB-IMM | 0x50 0x190 | `498b87800200004881e89001000049898780020000c3` (22B) | `f7e06d035b717d9d` |
| H_552 | 0x22E | 0x61 SUB-IMM | 0x51 0x190 | `498b87880200004881e89001000049898788020000c3` (22B) | `489b9cd85b80cad9` |
| H_553 | 0x22F | 0x61 SUB-IMM | 0x52 0x190 | `498b87900200004881e89001000049898790020000c3` (22B) | `0535305934d986e2` |
| H_554 | 0x230 | 0x80 LDB | 0x50 0x60 0x198 | `498b87000300004881c098010000480fb60049898780020000c3` (26B) | `f68f3fdd889f57db` |
| H_555 | 0x231 | 0x80 LDB | 0x51 0x60 0x198 | `498b87000300004881c098010000480fb60049898788020000c3` (26B) | `1fd1cefc37ee2f6a` |
| H_556 | 0x232 | 0x80 LDB | 0x52 0x60 0x198 | `498b87000300004881c098010000480fb60049898790020000c3` (26B) | `84e2d29d21835c65` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-077 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_548.
- If the parent decides to serialize, append H_549.. at selectors `40 22B`..:
  - H_549 0x62 ADD-IMM (62 51 190) — pin `498b87880200004881c09001000049898788020000c3`
  - H_550 0x62 ADD-IMM (62 52 190) — pin `498b87900200004881c09001000049898790020000c3`
  - H_551 0x61 SUB-IMM (61 50 190) — pin `498b87800200004881e89001000049898780020000c3`
  - H_552 0x61 SUB-IMM (61 51 190) — pin `498b87880200004881e89001000049898788020000c3`
  - H_553 0x61 SUB-IMM (61 52 190) — pin `498b87900200004881e89001000049898790020000c3`
  - H_554 0x80 LDB (80 50 60 198) — pin `498b87000300004881c098010000480fb60049898780020000c3`
  - H_555 0x80 LDB (80 51 60 198) — pin `498b87000300004881c098010000480fb60049898788020000c3`
  - H_556 0x80 LDB (80 52 60 198) — pin `498b87000300004881c098010000480fb60049898790020000c3`
- Plus 1 Relock after append from pin `97ce84a2…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-078 serialize PASSes + 1 Relock
