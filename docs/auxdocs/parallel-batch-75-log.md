# parallel-batch-75 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-75-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-080 (pin `e255cd93…`, handlers = 579, H_565..H_572 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-080 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_572 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1A8 | `498b87000300004881c0a8010000480fb60049898788020000c3` (26) | same | same | Y | `fbea55b03005c5a5` | `fbea55b03005c5a5` | PASS |
| 2 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1A8 | `498b87000300004881c0a8010000480fb60049898790020000c3` (26) | same | same | Y | `7db0bd86b3e802a1` | `7db0bd86b3e802a1` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x50 imm=0x1A8 | `498b87800200004881c0a801000049898780020000c3` (22) | same | same | Y | `5a3272ce14feca9a` | `5a3272ce14feca9a` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x51 imm=0x1A8 | `498b87880200004881c0a801000049898788020000c3` (22) | same | same | Y | `6aecaccb918f42df` | `6aecaccb918f42df` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x52 imm=0x1A8 | `498b87900200004881c0a801000049898790020000c3` (22) | same | same | Y | `f2ea24f19b1f387c` | `f2ea24f19b1f387c` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x50 imm=0x1A8 | `498b87800200004881e8a801000049898780020000c3` (22) | same | same | Y | `2a655dd4d2adee0c` | `2a655dd4d2adee0c` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0x1A8 | `498b87880200004881e8a801000049898788020000c3` (22) | same | same | Y | `44c2fed0d54d8b28` | `44c2fed0d54d8b28` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x52 imm=0x1A8 | `498b87900200004881e8a801000049898790020000c3` (22) | same | same | Y | `2c5130704cf19491` | `2c5130704cf19491` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x51 ss=0x60 oo=0x1A8 — **PASS**

- fixture: `_scratch_ldb_5160_1A8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a8010000480fb60049898788020000c3`
- js-sha256: `fbea55b03005c5a5e5f040c713fda4d857774e5abe360051f5996da6e30bfd82`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x52 ss=0x60 oo=0x1A8 — **PASS**

- fixture: `_scratch_ldb_5260_1A8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a8010000480fb60049898790020000c3`
- js-sha256: `7db0bd86b3e802a19bff232db71fde4967ec75a1969973f1b9841101faa934be`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x50 imm=0x1A8 — **PASS**

- fixture: `_scratch_addimm_h50_1A8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0a801000049898780020000c3`
- js-sha256: `5a3272ce14feca9acc6662aa72a89d53c9afabc861ba1a14f92752976384a6a8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x51 imm=0x1A8 — **PASS**

- fixture: `_scratch_addimm_h51_1A8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0a801000049898788020000c3`
- js-sha256: `6aecaccb918f42dfee3967683d0e1b30b8740a0d4a14a936c518a5fcd91cafc9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x52 imm=0x1A8 — **PASS**

- fixture: `_scratch_addimm_h52_1A8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0a801000049898790020000c3`
- js-sha256: `f2ea24f19b1f387c5b5d415f109609bc51a9f3f1982a083fa4e3206a60ea9483`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x50 imm=0x1A8 — **PASS**

- fixture: `_scratch_subimm_h50_1A8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8a801000049898780020000c3`
- js-sha256: `2a655dd4d2adee0c5ec4c24070e01fdd7668bc56fb85738917c511dc703ead4b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0x1A8 — **PASS**

- fixture: `_scratch_subimm_h51_1A8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8a801000049898788020000c3`
- js-sha256: `44c2fed0d54d8b28b537faff3f8aa11ef7ea078c50930a5bf143336291a4a767`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x52 imm=0x1A8 — **PASS**

- fixture: `_scratch_subimm_h52_1A8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8a801000049898790020000c3`
- js-sha256: `2c5130704cf19491ce73bb03a34a507ef0c97d712f177935760f956f92bcd6d8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=51/52 ss=60 oo=1A8 (finish deferred 1A8 LDB triad; imm32 26B; H_572 already dd=50).
- ADD-IMM slot=50/51/52 imm=1A8 (start deferred 1A8 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=1A8 (start deferred 1A8 SUB triad; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 243`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5160_1A8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_1A8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1A8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1A8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1A8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1A8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1A8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1A8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-75-log.md` — this file
- `scripts/_probe/parallel-batch-75-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-081 serialize PASSes + 1 Relock**

Pass pin from body-extend-080 Relock: `e255cd93a26ec455cc4def0ceb38c1cfc93bcb1ec7476f9e57ecd062d1be065a`.
Handlers before consolidate = 579 (H_00..H_572). Next selectors `40 243`.. for H_573.. if all serialize.

PASS list for body-extend-081:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_573 | 0x243 | 0x80 LDB | 0x51 0x60 0x1A8 | `498b87000300004881c0a8010000480fb60049898788020000c3` (26B) | `fbea55b03005c5a5` |
| H_574 | 0x244 | 0x80 LDB | 0x52 0x60 0x1A8 | `498b87000300004881c0a8010000480fb60049898790020000c3` (26B) | `7db0bd86b3e802a1` |
| H_575 | 0x245 | 0x62 ADD-IMM | 0x50 0x1A8 | `498b87800200004881c0a801000049898780020000c3` (22B) | `5a3272ce14feca9a` |
| H_576 | 0x246 | 0x62 ADD-IMM | 0x51 0x1A8 | `498b87880200004881c0a801000049898788020000c3` (22B) | `6aecaccb918f42df` |
| H_577 | 0x247 | 0x62 ADD-IMM | 0x52 0x1A8 | `498b87900200004881c0a801000049898790020000c3` (22B) | `f2ea24f19b1f387c` |
| H_578 | 0x248 | 0x61 SUB-IMM | 0x50 0x1A8 | `498b87800200004881e8a801000049898780020000c3` (22B) | `2a655dd4d2adee0c` |
| H_579 | 0x249 | 0x61 SUB-IMM | 0x51 0x1A8 | `498b87880200004881e8a801000049898788020000c3` (22B) | `44c2fed0d54d8b28` |
| H_580 | 0x24A | 0x61 SUB-IMM | 0x52 0x1A8 | `498b87900200004881e8a801000049898790020000c3` (22B) | `2c5130704cf19491` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-080 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_572.
- If the parent decides to serialize, append H_573.. at selectors `40 243`..:
  - H_573 0x80 LDB (80 51 60 1A8) — pin `498b87000300004881c0a8010000480fb60049898788020000c3`
  - H_574 0x80 LDB (80 52 60 1A8) — pin `498b87000300004881c0a8010000480fb60049898790020000c3`
  - H_575 0x62 ADD-IMM (62 50 1A8) — pin `498b87800200004881c0a801000049898780020000c3`
  - H_576 0x62 ADD-IMM (62 51 1A8) — pin `498b87880200004881c0a801000049898788020000c3`
  - H_577 0x62 ADD-IMM (62 52 1A8) — pin `498b87900200004881c0a801000049898790020000c3`
  - H_578 0x61 SUB-IMM (61 50 1A8) — pin `498b87800200004881e8a801000049898780020000c3`
  - H_579 0x61 SUB-IMM (61 51 1A8) — pin `498b87880200004881e8a801000049898788020000c3`
  - H_580 0x61 SUB-IMM (61 52 1A8) — pin `498b87900200004881e8a801000049898790020000c3`
- Plus 1 Relock after append from pin `e255cd93…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-081 serialize PASSes + 1 Relock
