# parallel-batch-90 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-90-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-095 (pin `aef6d89f…`, handlers = 699, H_685..H_692 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-095 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_692 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x51 imm=0x210 | `498b87880200004881c01002000049898788020000c3` (22) | same | same | Y | `f59a9a17f02eae7c` | `f59a9a17f02eae7c` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x52 imm=0x210 | `498b87900200004881c01002000049898790020000c3` (22) | same | same | Y | `b6abb627bf849fc0` | `b6abb627bf849fc0` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x210 | `498b87800200004881e81002000049898780020000c3` (22) | same | same | Y | `f77af100f9fabd84` | `f77af100f9fabd84` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x51 imm=0x210 | `498b87880200004881e81002000049898788020000c3` (22) | same | same | Y | `dbfd9ece27cb16d9` | `dbfd9ece27cb16d9` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x52 imm=0x210 | `498b87900200004881e81002000049898790020000c3` (22) | same | same | Y | `b4bcf1859605c71c` | `b4bcf1859605c71c` | PASS |
| 6 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x218 | `498b87000300004881c018020000480fb60049898780020000c3` (26) | same | same | Y | `c6cb4e7e1fac02c9` | `c6cb4e7e1fac02c9` | PASS |
| 7 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x218 | `498b87000300004881c018020000480fb60049898788020000c3` (26) | same | same | Y | `6296837a29daedeb` | `6296837a29daedeb` | PASS |
| 8 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x218 | `498b87000300004881c018020000480fb60049898790020000c3` (26) | same | same | Y | `8e68e69170dde74d` | `8e68e69170dde74d` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x51 imm=0x210 — **PASS**

- fixture: `_scratch_addimm_h51_210.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c01002000049898788020000c3`
- js-sha256: `f59a9a17f02eae7c09283f04ad040634e7fda687dda507038344fa9cc758be6c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x52 imm=0x210 — **PASS**

- fixture: `_scratch_addimm_h52_210.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c01002000049898790020000c3`
- js-sha256: `b6abb627bf849fc0a9cebf0c9b09d36e0ca9c67bf705287c2bfc5e25301c690a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x210 — **PASS**

- fixture: `_scratch_subimm_h50_210.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e81002000049898780020000c3`
- js-sha256: `f77af100f9fabd84ef73e82bfcfed4011049214dccf6d64fc50a8931a9015fa2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x51 imm=0x210 — **PASS**

- fixture: `_scratch_subimm_h51_210.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e81002000049898788020000c3`
- js-sha256: `dbfd9ece27cb16d9e60e5a74e1a1bfac06a2ed48396cb7fddae9db85ef6576e8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x52 imm=0x210 — **PASS**

- fixture: `_scratch_subimm_h52_210.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e81002000049898790020000c3`
- js-sha256: `b4bcf1859605c71c1618d398a81d2e3a1fd0f0d47298a2a62af9164f9f7080f9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x50 ss=0x60 oo=0x218 — **PASS**

- fixture: `_scratch_ldb_5060_218.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c018020000480fb60049898780020000c3`
- js-sha256: `c6cb4e7e1fac02c9cea83b983dc954c4f3066cd8a67a026fa8c2b35e92aea8a3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x51 ss=0x60 oo=0x218 — **PASS**

- fixture: `_scratch_ldb_5160_218.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c018020000480fb60049898788020000c3`
- js-sha256: `6296837a29daedeba1df94ff6f0c6173e11264bc97593e06246e6cc71544234b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x52 ss=0x60 oo=0x218 — **PASS**

- fixture: `_scratch_ldb_5260_218.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c018020000480fb60049898790020000c3`
- js-sha256: `8e68e69170dde74dc3221b9ba81b23012c1fd0d5957b564ae1ad73489451dc85`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=51/52 imm=210 (finish deferred 210 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=210 (start 210 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=218 (start 218 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=218 deferred to next batch.
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 2BB`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h51_210.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_210.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_210.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_210.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_210.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_218.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_218.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_218.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-90-log.md` — this file
- `scripts/_probe/parallel-batch-90-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-096 serialize PASSes + 1 Relock**

Pass pin from body-extend-095 Relock: `aef6d89f98ceb7c8d9770950da9a584d7165f7e0d6713fc30c1d3f14c92552ee`.
Handlers before consolidate = 699 (H_00..H_692). Next selectors `40 2BB`.. for H_693.. if all serialize.

PASS list for body-extend-096:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_693 | 0x2BB | 0x62 ADD-IMM | 0x51 0x210 | `498b87880200004881c01002000049898788020000c3` (22B) | `f59a9a17f02eae7c` |
| H_694 | 0x2BC | 0x62 ADD-IMM | 0x52 0x210 | `498b87900200004881c01002000049898790020000c3` (22B) | `b6abb627bf849fc0` |
| H_695 | 0x2BD | 0x61 SUB-IMM | 0x50 0x210 | `498b87800200004881e81002000049898780020000c3` (22B) | `f77af100f9fabd84` |
| H_696 | 0x2BE | 0x61 SUB-IMM | 0x51 0x210 | `498b87880200004881e81002000049898788020000c3` (22B) | `dbfd9ece27cb16d9` |
| H_697 | 0x2BF | 0x61 SUB-IMM | 0x52 0x210 | `498b87900200004881e81002000049898790020000c3` (22B) | `b4bcf1859605c71c` |
| H_698 | 0x2C0 | 0x80 LDB | 0x50 0x60 0x218 | `498b87000300004881c018020000480fb60049898780020000c3` (26B) | `c6cb4e7e1fac02c9` |
| H_699 | 0x2C1 | 0x80 LDB | 0x51 0x60 0x218 | `498b87000300004881c018020000480fb60049898788020000c3` (26B) | `6296837a29daedeb` |
| H_700 | 0x2C2 | 0x80 LDB | 0x52 0x60 0x218 | `498b87000300004881c018020000480fb60049898790020000c3` (26B) | `8e68e69170dde74d` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-095 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_692.
- If the parent decides to serialize, append H_693.. at selectors `40 2BB`..:
  - H_693 0x62 ADD-IMM (62 51 210) — pin `498b87880200004881c01002000049898788020000c3`
  - H_694 0x62 ADD-IMM (62 52 210) — pin `498b87900200004881c01002000049898790020000c3`
  - H_695 0x61 SUB-IMM (61 50 210) — pin `498b87800200004881e81002000049898780020000c3`
  - H_696 0x61 SUB-IMM (61 51 210) — pin `498b87880200004881e81002000049898788020000c3`
  - H_697 0x61 SUB-IMM (61 52 210) — pin `498b87900200004881e81002000049898790020000c3`
  - H_698 0x80 LDB (80 50 60 218) — pin `498b87000300004881c018020000480fb60049898780020000c3`
  - H_699 0x80 LDB (80 51 60 218) — pin `498b87000300004881c018020000480fb60049898788020000c3`
  - H_700 0x80 LDB (80 52 60 218) — pin `498b87000300004881c018020000480fb60049898790020000c3`
- Plus 1 Relock after append from pin `aef6d89f…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: ADD-IMM slot=50/51/52 imm=218; finish 218 ADD/SUB ladder.

## §7. Consolidation handoff

parent next = body-extend-096 serialize PASSes + 1 Relock
