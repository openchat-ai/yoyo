# parallel-batch-92 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-92-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-097 (pin `e6ba7d6c…`, handlers = 715, H_701..H_708 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-097 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_708 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x220 | `498b87000300004881c020020000480fb60049898790020000c3` (26) | same | same | Y | `3fc747bcdb5a7814` | `3fc747bcdb5a7814` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x50 imm=0x220 | `498b87800200004881c02002000049898780020000c3` (22) | same | same | Y | `1bbf4fad113bcab7` | `1bbf4fad113bcab7` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0x220 | `498b87880200004881c02002000049898788020000c3` (22) | same | same | Y | `8504700ade40627c` | `8504700ade40627c` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x52 imm=0x220 | `498b87900200004881c02002000049898790020000c3` (22) | same | same | Y | `c0a102f97c62576f` | `c0a102f97c62576f` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x50 imm=0x220 | `498b87800200004881e82002000049898780020000c3` (22) | same | same | Y | `3a44dbe899e12859` | `3a44dbe899e12859` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x51 imm=0x220 | `498b87880200004881e82002000049898788020000c3` (22) | same | same | Y | `740509fefa4bff85` | `740509fefa4bff85` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0x220 | `498b87900200004881e82002000049898790020000c3` (22) | same | same | Y | `2ae2a9625cac581c` | `2ae2a9625cac581c` | PASS |
| 8 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x228 | `498b87000300004881c028020000480fb60049898780020000c3` (26) | same | same | Y | `9e1963a796211cc1` | `9e1963a796211cc1` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x52 ss=0x60 oo=0x220 — **PASS**

- fixture: `_scratch_ldb_5260_220.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c020020000480fb60049898790020000c3`
- js-sha256: `3fc747bcdb5a781461fd0348cdb022ece3e3a9661b215e684e0233690b5f8f8d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x50 imm=0x220 — **PASS**

- fixture: `_scratch_addimm_h50_220.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c02002000049898780020000c3`
- js-sha256: `1bbf4fad113bcab70b19bba185dc4e67008fe55aa83c534a13df23ecf474b482`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0x220 — **PASS**

- fixture: `_scratch_addimm_h51_220.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c02002000049898788020000c3`
- js-sha256: `8504700ade40627c64cde82afa2ac6385f4194d5798cf34258012669602545fb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x52 imm=0x220 — **PASS**

- fixture: `_scratch_addimm_h52_220.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c02002000049898790020000c3`
- js-sha256: `c0a102f97c62576f4163546150ecd9d07a1dc8ee2c781e1a0cae0c70957e47cb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x50 imm=0x220 — **PASS**

- fixture: `_scratch_subimm_h50_220.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e82002000049898780020000c3`
- js-sha256: `3a44dbe899e12859ab6ac9679f62b181a83f1321ea5328d991efa218669f239e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x51 imm=0x220 — **PASS**

- fixture: `_scratch_subimm_h51_220.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e82002000049898788020000c3`
- js-sha256: `740509fefa4bff8502ceff79f94cfdfc9a24681c1941768c4663818ba95c0279`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0x220 — **PASS**

- fixture: `_scratch_subimm_h52_220.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e82002000049898790020000c3`
- js-sha256: `2ae2a9625cac581c393c14716bf4dcbd8f576e8a0c12c7d6b56671ecc70917bb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x50 ss=0x60 oo=0x228 — **PASS**

- fixture: `_scratch_ldb_5060_228.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c028020000480fb60049898780020000c3`
- js-sha256: `9e1963a796211cc173505fa2bd3c4864753788fffb324d73944ead0e6682c2f2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=52 ss=60 oo=220 (finish deferred 220 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=220 (start deferred 220 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=220 (start 220 SUB triad; imm32 22B).
- LDB dd=50 ss=60 oo=228 (start 228 LDB ladder; imm32 26B; LDB 51/52 228 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 2CB`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5260_220.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_220.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_220.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_220.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_220.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_220.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_220.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_228.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-92-log.md` — this file
- `scripts/_probe/parallel-batch-92-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-098 serialize PASSes + 1 Relock**

Pass pin from body-extend-097 Relock: `e6ba7d6cfcbb11da0a3a63dab93cde597a265934cf95064968d97697c85cd68a`.
Handlers before consolidate = 715 (H_00..H_708). Next selectors `40 2CB`.. for H_709.. if all serialize.

PASS list for body-extend-098:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_709 | 0x2CB | 0x80 LDB | 0x52 0x60 0x220 | `498b87000300004881c020020000480fb60049898790020000c3` (26B) | `3fc747bcdb5a7814` |
| H_710 | 0x2CC | 0x62 ADD-IMM | 0x50 0x220 | `498b87800200004881c02002000049898780020000c3` (22B) | `1bbf4fad113bcab7` |
| H_711 | 0x2CD | 0x62 ADD-IMM | 0x51 0x220 | `498b87880200004881c02002000049898788020000c3` (22B) | `8504700ade40627c` |
| H_712 | 0x2CE | 0x62 ADD-IMM | 0x52 0x220 | `498b87900200004881c02002000049898790020000c3` (22B) | `c0a102f97c62576f` |
| H_713 | 0x2CF | 0x61 SUB-IMM | 0x50 0x220 | `498b87800200004881e82002000049898780020000c3` (22B) | `3a44dbe899e12859` |
| H_714 | 0x2D0 | 0x61 SUB-IMM | 0x51 0x220 | `498b87880200004881e82002000049898788020000c3` (22B) | `740509fefa4bff85` |
| H_715 | 0x2D1 | 0x61 SUB-IMM | 0x52 0x220 | `498b87900200004881e82002000049898790020000c3` (22B) | `2ae2a9625cac581c` |
| H_716 | 0x2D2 | 0x80 LDB | 0x50 0x60 0x228 | `498b87000300004881c028020000480fb60049898780020000c3` (26B) | `9e1963a796211cc1` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-097 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_708.
- If the parent decides to serialize, append H_709.. at selectors `40 2CB`..:
  - H_709 0x80 LDB (80 52 60 220) — pin `498b87000300004881c020020000480fb60049898790020000c3`
  - H_710 0x62 ADD-IMM (62 50 220) — pin `498b87800200004881c02002000049898780020000c3`
  - H_711 0x62 ADD-IMM (62 51 220) — pin `498b87880200004881c02002000049898788020000c3`
  - H_712 0x62 ADD-IMM (62 52 220) — pin `498b87900200004881c02002000049898790020000c3`
  - H_713 0x61 SUB-IMM (61 50 220) — pin `498b87800200004881e82002000049898780020000c3`
  - H_714 0x61 SUB-IMM (61 51 220) — pin `498b87880200004881e82002000049898788020000c3`
  - H_715 0x61 SUB-IMM (61 52 220) — pin `498b87900200004881e82002000049898790020000c3`
  - H_716 0x80 LDB (80 50 60 228) — pin `498b87000300004881c028020000480fb60049898780020000c3`
- Plus 1 Relock after append from pin `e6ba7d6c…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: LDB 51/52 60 228; finish 228 LDB ladder.

## §7. Consolidation handoff

parent next = body-extend-098 serialize PASSes + 1 Relock
