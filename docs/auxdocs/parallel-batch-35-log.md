# parallel-batch-35 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-35-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-040 (pin `a58ead28…`, handlers = 260, H_246..H_253 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-040 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_253 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xA0 | `498b87000300004881c0a0000000480fb60049898780020000c3` (26) | same | same | Y | `4817b8ddf9b52566` | `4817b8ddf9b52566` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xA0 | `498b87000300004881c0a0000000480fb60049898788020000c3` (26) | same | same | Y | `fcf0ba5ffb072ffa` | `fcf0ba5ffb072ffa` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xA0 | `498b87000300004881c0a0000000480fb60049898790020000c3` (26) | same | same | Y | `c6dd95a8ede6bf6a` | `c6dd95a8ede6bf6a` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x80 | `498b87800200004881e88000000049898780020000c3` (22) | same | same | Y | `e0304eea69eed143` | `e0304eea69eed143` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x51 imm=0x80 | `498b87880200004881e88000000049898788020000c3` (22) | same | same | Y | `f76a1690a99750ff` | `f76a1690a99750ff` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x52 imm=0x80 | `498b87900200004881e88000000049898790020000c3` (22) | same | same | Y | `d26957f7354c5ec6` | `d26957f7354c5ec6` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x50 imm=0x88 | `498b87800200004881c08800000049898780020000c3` (22) | same | same | Y | `eabc3ae46677427e` | `eabc3ae46677427e` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x51 imm=0x88 | `498b87880200004881c08800000049898788020000c3` (22) | same | same | Y | `4c60d97a9ae2744d` | `4c60d97a9ae2744d` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x50 ss=0x60 oo=0xA0 — **PASS**

- fixture: `_scratch_ldb_5060_a0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a0000000480fb60049898780020000c3`
- js-sha256: `4817b8ddf9b525669ccf6489dd0795345486ef0399993f7d81a05782b5bd7a0f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0xA0 — **PASS**

- fixture: `_scratch_ldb_5160_a0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a0000000480fb60049898788020000c3`
- js-sha256: `fcf0ba5ffb072ffa95b0eee6acb2338408db86254dbe816f0c684e14996d1dc3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0xA0 — **PASS**

- fixture: `_scratch_ldb_5260_a0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a0000000480fb60049898790020000c3`
- js-sha256: `c6dd95a8ede6bf6a65911146461ffa7f80a0bd1170098ad100478c2aefbe05c2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x80 — **PASS**

- fixture: `_scratch_subimm_h50_80.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e88000000049898780020000c3`
- js-sha256: `e0304eea69eed143e909734feb74ed20316a403855b257e13aaaea5bbcdbc964`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x51 imm=0x80 — **PASS**

- fixture: `_scratch_subimm_h51_80.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e88000000049898788020000c3`
- js-sha256: `f76a1690a99750ff96f35a052519004f469b3b306da7134812d7c6e8d10cf962`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x52 imm=0x80 — **PASS**

- fixture: `_scratch_subimm_h52_80.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e88000000049898790020000c3`
- js-sha256: `d26957f7354c5ec61f02629f0c80401e40f1134c4bfe6295d65cf0502367132a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x50 imm=0x88 — **PASS**

- fixture: `_scratch_addimm_h50_88.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c08800000049898780020000c3`
- js-sha256: `eabc3ae46677427e13e8aa82bd58f288680bdde77a890ce1150f7a167d36d224`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x51 imm=0x88 — **PASS**

- fixture: `_scratch_addimm_h51_88.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c08800000049898788020000c3`
- js-sha256: `4c60d97a9ae2744d33f0d0d8b3689f4537b6f704777093a75a81a3dfa572dcf5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=50/51/52 ss=60 oo=A0 (fresh oo=A0 triad; imm32 26B).
- SUB-IMM slot 50/51/52 imm=80 (complements locked ADD-IMM * 80; imm=0x80 → imm32 sub).
- ADD-IMM slot 50/51 imm=88 (fresh imm=88 start; 0x88>127 → imm32 `48 81 c0` → 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 104`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5060_a0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_a0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_a0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_80.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_80.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_80.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_88.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_88.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-35-log.md` — this file
- `scripts/_probe/parallel-batch-35-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-041 serialize PASSes + 1 Relock**

Pass pin from body-extend-040 Relock: `a58ead289233c42ba1c6e9a84aedb6218176aad27ecd5cbdd0d4659a2e5bc187`.
Handlers before consolidate = 260 (H_00..H_253). Next selectors `40 104`.. for H_254.. if all serialize.

PASS list for body-extend-041:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_254 | 0x104 | 0x80 LDB | 0x50 0x60 0xA0 | `498b87000300004881c0a0000000480fb60049898780020000c3` (26B) | `4817b8ddf9b52566` |
| H_255 | 0x105 | 0x80 LDB | 0x51 0x60 0xA0 | `498b87000300004881c0a0000000480fb60049898788020000c3` (26B) | `fcf0ba5ffb072ffa` |
| H_256 | 0x106 | 0x80 LDB | 0x52 0x60 0xA0 | `498b87000300004881c0a0000000480fb60049898790020000c3` (26B) | `c6dd95a8ede6bf6a` |
| H_257 | 0x107 | 0x61 SUB-IMM | 0x50 0x80 | `498b87800200004881e88000000049898780020000c3` (22B) | `e0304eea69eed143` |
| H_258 | 0x108 | 0x61 SUB-IMM | 0x51 0x80 | `498b87880200004881e88000000049898788020000c3` (22B) | `f76a1690a99750ff` |
| H_259 | 0x109 | 0x61 SUB-IMM | 0x52 0x80 | `498b87900200004881e88000000049898790020000c3` (22B) | `d26957f7354c5ec6` |
| H_260 | 0x10A | 0x62 ADD-IMM | 0x50 0x88 | `498b87800200004881c08800000049898780020000c3` (22B) | `eabc3ae46677427e` |
| H_261 | 0x10B | 0x62 ADD-IMM | 0x51 0x88 | `498b87880200004881c08800000049898788020000c3` (22B) | `4c60d97a9ae2744d` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-040 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_253.
- If the parent decides to serialize, append H_254.. at selectors `40 104`..:
  - H_254 0x80 LDB (80 50 60 A0) — pin `498b87000300004881c0a0000000480fb60049898780020000c3`
  - H_255 0x80 LDB (80 51 60 A0) — pin `498b87000300004881c0a0000000480fb60049898788020000c3`
  - H_256 0x80 LDB (80 52 60 A0) — pin `498b87000300004881c0a0000000480fb60049898790020000c3`
  - H_257 0x61 SUB-IMM (61 50 80) — pin `498b87800200004881e88000000049898780020000c3`
  - H_258 0x61 SUB-IMM (61 51 80) — pin `498b87880200004881e88000000049898788020000c3`
  - H_259 0x61 SUB-IMM (61 52 80) — pin `498b87900200004881e88000000049898790020000c3`
  - H_260 0x62 ADD-IMM (62 50 88) — pin `498b87800200004881c08800000049898780020000c3`
  - H_261 0x62 ADD-IMM (62 51 88) — pin `498b87880200004881c08800000049898788020000c3`
- Plus 1 Relock after append from pin `a58ead28…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-041 serialize PASSes + 1 Relock
