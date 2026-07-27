# parallel-batch-87 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-87-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-092 (pin `1991af84…`, handlers = 675, H_661..H_668 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-092 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_668 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x51 imm=0x1F8 | `498b87880200004881e8f801000049898788020000c3` (22) | same | same | Y | `cd8053ace6652cd9` | `cd8053ace6652cd9` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x52 imm=0x1F8 | `498b87900200004881e8f801000049898790020000c3` (22) | same | same | Y | `512b7b4c08728ca7` | `512b7b4c08728ca7` | PASS |
| 3 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x200 | `498b87000300004881c000020000480fb60049898780020000c3` (26) | same | same | Y | `8ef97152f880c8bf` | `8ef97152f880c8bf` | PASS |
| 4 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x200 | `498b87000300004881c000020000480fb60049898788020000c3` (26) | same | same | Y | `ae88f23839b7ed37` | `ae88f23839b7ed37` | PASS |
| 5 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x200 | `498b87000300004881c000020000480fb60049898790020000c3` (26) | same | same | Y | `623de62f88220d56` | `623de62f88220d56` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x50 imm=0x200 | `498b87800200004881c00002000049898780020000c3` (22) | same | same | Y | `cba55979366f2bab` | `cba55979366f2bab` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x51 imm=0x200 | `498b87880200004881c00002000049898788020000c3` (22) | same | same | Y | `d48330be708021e4` | `d48330be708021e4` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x52 imm=0x200 | `498b87900200004881c00002000049898790020000c3` (22) | same | same | Y | `563af54479f67bd3` | `563af54479f67bd3` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x51 imm=0x1F8 — **PASS**

- fixture: `_scratch_subimm_h51_1F8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8f801000049898788020000c3`
- js-sha256: `cd8053ace6652cd9f67f1021123bdc5222b2a7a5e4dec6586639ae6d3bbb95d4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x52 imm=0x1F8 — **PASS**

- fixture: `_scratch_subimm_h52_1F8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8f801000049898790020000c3`
- js-sha256: `512b7b4c08728ca7793738789ea4129b0a3982166e627307e6ad89d2b009f471`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x50 ss=0x60 oo=0x200 — **PASS**

- fixture: `_scratch_ldb_5060_200.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c000020000480fb60049898780020000c3`
- js-sha256: `8ef97152f880c8bf58ccff2b1e71f0d5607d5659d0fb45acda46d9f3dfc13490`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x51 ss=0x60 oo=0x200 — **PASS**

- fixture: `_scratch_ldb_5160_200.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c000020000480fb60049898788020000c3`
- js-sha256: `ae88f23839b7ed37a87ae8ca78f67b76185a4b74f88574f6a84689f8d40bea2c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x52 ss=0x60 oo=0x200 — **PASS**

- fixture: `_scratch_ldb_5260_200.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c000020000480fb60049898790020000c3`
- js-sha256: `623de62f88220d56cb2e73f2807b3a7503641858552eb732a5e5cef420bd803d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x50 imm=0x200 — **PASS**

- fixture: `_scratch_addimm_h50_200.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c00002000049898780020000c3`
- js-sha256: `cba55979366f2bab0d63b00bca48823c7d7b80965c4c2f5ac4691cd11977ea07`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x51 imm=0x200 — **PASS**

- fixture: `_scratch_addimm_h51_200.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c00002000049898788020000c3`
- js-sha256: `d48330be708021e46e3d010b4b35804437f8e844842cf428ac1bf7fa6f7f5348`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x52 imm=0x200 — **PASS**

- fixture: `_scratch_addimm_h52_200.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c00002000049898790020000c3`
- js-sha256: `563af54479f67bd3329f812c6f699999bbfb34097e119ae3ab3f39516c6a022f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=51/52 imm=1F8 (finish deferred 1F8 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=200 (start 200 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=200 (start 200 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=200 deferred to next batch.
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 2A3`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h51_1F8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1F8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_200.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_200.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_200.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_200.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_200.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_200.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-87-log.md` — this file
- `scripts/_probe/parallel-batch-87-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-093 serialize PASSes + 1 Relock**

Pass pin from body-extend-092 Relock: `1991af8484d67ec19980bf14771d523d332f85c9974e1da09d45496baf46ebb5`.
Handlers before consolidate = 675 (H_00..H_668). Next selectors `40 2A3`.. for H_669.. if all serialize.

PASS list for body-extend-093:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_669 | 0x2A3 | 0x61 SUB-IMM | 0x51 0x1F8 | `498b87880200004881e8f801000049898788020000c3` (22B) | `cd8053ace6652cd9` |
| H_670 | 0x2A4 | 0x61 SUB-IMM | 0x52 0x1F8 | `498b87900200004881e8f801000049898790020000c3` (22B) | `512b7b4c08728ca7` |
| H_671 | 0x2A5 | 0x80 LDB | 0x50 0x60 0x200 | `498b87000300004881c000020000480fb60049898780020000c3` (26B) | `8ef97152f880c8bf` |
| H_672 | 0x2A6 | 0x80 LDB | 0x51 0x60 0x200 | `498b87000300004881c000020000480fb60049898788020000c3` (26B) | `ae88f23839b7ed37` |
| H_673 | 0x2A7 | 0x80 LDB | 0x52 0x60 0x200 | `498b87000300004881c000020000480fb60049898790020000c3` (26B) | `623de62f88220d56` |
| H_674 | 0x2A8 | 0x62 ADD-IMM | 0x50 0x200 | `498b87800200004881c00002000049898780020000c3` (22B) | `cba55979366f2bab` |
| H_675 | 0x2A9 | 0x62 ADD-IMM | 0x51 0x200 | `498b87880200004881c00002000049898788020000c3` (22B) | `d48330be708021e4` |
| H_676 | 0x2AA | 0x62 ADD-IMM | 0x52 0x200 | `498b87900200004881c00002000049898790020000c3` (22B) | `563af54479f67bd3` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-092 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_668.
- If the parent decides to serialize, append H_669.. at selectors `40 2A3`..:
  - H_669 0x61 SUB-IMM (61 51 1F8) — pin `498b87880200004881e8f801000049898788020000c3`
  - H_670 0x61 SUB-IMM (61 52 1F8) — pin `498b87900200004881e8f801000049898790020000c3`
  - H_671 0x80 LDB (80 50 60 200) — pin `498b87000300004881c000020000480fb60049898780020000c3`
  - H_672 0x80 LDB (80 51 60 200) — pin `498b87000300004881c000020000480fb60049898788020000c3`
  - H_673 0x80 LDB (80 52 60 200) — pin `498b87000300004881c000020000480fb60049898790020000c3`
  - H_674 0x62 ADD-IMM (62 50 200) — pin `498b87800200004881c00002000049898780020000c3`
  - H_675 0x62 ADD-IMM (62 51 200) — pin `498b87880200004881c00002000049898788020000c3`
  - H_676 0x62 ADD-IMM (62 52 200) — pin `498b87900200004881c00002000049898790020000c3`
- Plus 1 Relock after append from pin `1991af84…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: SUB-IMM slot=50/51/52 imm=200 (finish/start 200 SUB triad).

## §7. Consolidation handoff

parent next = body-extend-093 serialize PASSes + 1 Relock
