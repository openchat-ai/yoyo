# parallel-batch-95 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-95-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-100 (pin `7c079064…`, handlers = 739, H_725..H_732 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-100 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_732 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.
> Runner: `parallel-batch-scratch-lib.mjs` pool ≤8 (no Relock).

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x52 imm=0x230 | `498b87900200004881e83002000049898790020000c3` (22) | same | same | Y | `5a2ce924b1a66050` | `5a2ce924b1a66050` | PASS |
| 2 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x232 | `498b87000300004881c032020000480fb60049898780020000c3` (26) | same | same | Y | `2c8b3aa576062c39` | `2c8b3aa576062c39` | PASS |
| 3 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x232 | `498b87000300004881c032020000480fb60049898788020000c3` (26) | same | same | Y | `d935a5d3f24953e7` | `d935a5d3f24953e7` | PASS |
| 4 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x232 | `498b87000300004881c032020000480fb60049898790020000c3` (26) | same | same | Y | `1d9a2681b4fac7a1` | `1d9a2681b4fac7a1` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x50 imm=0x232 | `498b87800200004881c03202000049898780020000c3` (22) | same | same | Y | `da80cde8ed742a1c` | `da80cde8ed742a1c` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x51 imm=0x232 | `498b87880200004881c03202000049898788020000c3` (22) | same | same | Y | `4aa3b5563616b6a6` | `4aa3b5563616b6a6` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x52 imm=0x232 | `498b87900200004881c03202000049898790020000c3` (22) | same | same | Y | `f9199c6bd9783045` | `f9199c6bd9783045` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x50 imm=0x232 | `498b87800200004881e83202000049898780020000c3` (22) | same | same | Y | `922bcb642443cdc9` | `922bcb642443cdc9` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x52 imm=0x230 — **PASS**

- fixture: `_scratch_subimm_h52_230.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e83002000049898790020000c3`
- js-sha256: `5a2ce924b1a66050cd8317c147e86ed49e8277cc463083ae9a8c0eb691989b89`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x50 ss=0x60 oo=0x232 — **PASS**

- fixture: `_scratch_ldb_5060_232.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c032020000480fb60049898780020000c3`
- js-sha256: `2c8b3aa576062c3900b06a28cef3c8d5505960f829c94454bb8154c3e33eccf3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x51 ss=0x60 oo=0x232 — **PASS**

- fixture: `_scratch_ldb_5160_232.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c032020000480fb60049898788020000c3`
- js-sha256: `d935a5d3f24953e7037800a6a859243d8d5e12c711fd4ea0105a13617016acb2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x52 ss=0x60 oo=0x232 — **PASS**

- fixture: `_scratch_ldb_5260_232.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c032020000480fb60049898790020000c3`
- js-sha256: `1d9a2681b4fac7a1dfc3d43209e67426fda3041dee1bcfc1c51f3433838f73da`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x50 imm=0x232 — **PASS**

- fixture: `_scratch_addimm_h50_232.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c03202000049898780020000c3`
- js-sha256: `da80cde8ed742a1c98a87f2e0e0c0f69e62d1cf12b4cb73fe03d51b8c2a2e3eb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x51 imm=0x232 — **PASS**

- fixture: `_scratch_addimm_h51_232.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c03202000049898788020000c3`
- js-sha256: `4aa3b5563616b6a6dbbab36788b7117488fc12b67a0cc851ddb3cccc6a4671cd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x52 imm=0x232 — **PASS**

- fixture: `_scratch_addimm_h52_232.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c03202000049898790020000c3`
- js-sha256: `f9199c6bd9783045ccd6c049dbc65401650062d1a850a6269d18a6fc35617d89`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x50 imm=0x232 — **PASS**

- fixture: `_scratch_subimm_h50_232.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e83202000049898780020000c3`
- js-sha256: `922bcb642443cdc9d80af6a993395e5d90bb69b7b4e59539c7b5dd327a22ce2a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=52 imm=230 (finish deferred 230 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=232 (start 232 LDB ladder; imm32 26B).
- ADD-IMM slot=50/51/52 imm=232 (start 232 ADD triad; imm32 22B).
- SUB-IMM slot=50 imm=232 (start 232 SUB triad; imm32 22B; SUB 51/52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 2E3`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h52_230.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_232.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_232.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_232.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_232.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_232.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_232.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_232.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-95-log.md` — this file
- `scripts/_probe/parallel-batch-95-run.mjs` — probe runner (uses shared concurrent lib)
- `scripts/_probe/parallel-batch-scratch-lib.mjs` — ≤8 scratch workers

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-101 serialize PASSes + 1 Relock**

Pass pin from body-extend-100 Relock: `7c07906496a7af9cbaec74b5590ec3677117ced6c36241823bd69b6a4ff1ae51`.
Handlers before consolidate = 739 (H_00..H_732). Next selectors `40 2E3`.. for H_733.. if all serialize.

PASS list for body-extend-101:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_733 | 0x2E3 | 0x61 SUB-IMM | 0x52 0x230 | `498b87900200004881e83002000049898790020000c3` (22B) | `5a2ce924b1a66050` |
| H_734 | 0x2E4 | 0x80 LDB | 0x50 0x60 0x232 | `498b87000300004881c032020000480fb60049898780020000c3` (26B) | `2c8b3aa576062c39` |
| H_735 | 0x2E5 | 0x80 LDB | 0x51 0x60 0x232 | `498b87000300004881c032020000480fb60049898788020000c3` (26B) | `d935a5d3f24953e7` |
| H_736 | 0x2E6 | 0x80 LDB | 0x52 0x60 0x232 | `498b87000300004881c032020000480fb60049898790020000c3` (26B) | `1d9a2681b4fac7a1` |
| H_737 | 0x2E7 | 0x62 ADD-IMM | 0x50 0x232 | `498b87800200004881c03202000049898780020000c3` (22B) | `da80cde8ed742a1c` |
| H_738 | 0x2E8 | 0x62 ADD-IMM | 0x51 0x232 | `498b87880200004881c03202000049898788020000c3` (22B) | `4aa3b5563616b6a6` |
| H_739 | 0x2E9 | 0x62 ADD-IMM | 0x52 0x232 | `498b87900200004881c03202000049898790020000c3` (22B) | `f9199c6bd9783045` |
| H_740 | 0x2EA | 0x61 SUB-IMM | 0x50 0x232 | `498b87800200004881e83202000049898780020000c3` (22B) | `922bcb642443cdc9` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-100 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_732.
- If the parent decides to serialize, append H_733.. at selectors `40 2E3`..:
  - H_733 0x61 SUB-IMM (61 52 230) — pin `498b87900200004881e83002000049898790020000c3`
  - H_734 0x80 LDB (80 50 60 232) — pin `498b87000300004881c032020000480fb60049898780020000c3`
  - H_735 0x80 LDB (80 51 60 232) — pin `498b87000300004881c032020000480fb60049898788020000c3`
  - H_736 0x80 LDB (80 52 60 232) — pin `498b87000300004881c032020000480fb60049898790020000c3`
  - H_737 0x62 ADD-IMM (62 50 232) — pin `498b87800200004881c03202000049898780020000c3`
  - H_738 0x62 ADD-IMM (62 51 232) — pin `498b87880200004881c03202000049898788020000c3`
  - H_739 0x62 ADD-IMM (62 52 232) — pin `498b87900200004881c03202000049898790020000c3`
  - H_740 0x61 SUB-IMM (61 50 232) — pin `498b87800200004881e83202000049898780020000c3`
- Plus 1 Relock after append from pin `7c079064…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: SUB-IMM 51/52 232 (finish 232 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh; next ladder if continuing.

## §7. Consolidation handoff

parent next = body-extend-101 serialize PASSes + 1 Relock
