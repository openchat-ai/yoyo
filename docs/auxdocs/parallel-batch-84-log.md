# parallel-batch-84 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-84-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-089 (pin `e8500277…`, handlers = 651, H_637..H_644 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-089 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_644 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1E8 | `498b87000300004881c0e8010000480fb60049898788020000c3` (26) | same | same | Y | `ba62e4ad2c2e56ee` | `ba62e4ad2c2e56ee` | PASS |
| 2 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1E8 | `498b87000300004881c0e8010000480fb60049898790020000c3` (26) | same | same | Y | `aac7a387b001d803` | `aac7a387b001d803` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x50 imm=0x1E8 | `498b87800200004881c0e801000049898780020000c3` (22) | same | same | Y | `a63c229b97189c94` | `a63c229b97189c94` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x51 imm=0x1E8 | `498b87880200004881c0e801000049898788020000c3` (22) | same | same | Y | `356a1a0b3408f7f6` | `356a1a0b3408f7f6` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x52 imm=0x1E8 | `498b87900200004881c0e801000049898790020000c3` (22) | same | same | Y | `ea596d905acbddb7` | `ea596d905acbddb7` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x50 imm=0x1E8 | `498b87800200004881e8e801000049898780020000c3` (22) | same | same | Y | `0e13aa7197e06d20` | `0e13aa7197e06d20` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0x1E8 | `498b87880200004881e8e801000049898788020000c3` (22) | same | same | Y | `58e9756f847685f3` | `58e9756f847685f3` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x52 imm=0x1E8 | `498b87900200004881e8e801000049898790020000c3` (22) | same | same | Y | `eeda72c92f5324fc` | `eeda72c92f5324fc` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x51 ss=0x60 oo=0x1E8 — **PASS**

- fixture: `_scratch_ldb_5160_1E8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e8010000480fb60049898788020000c3`
- js-sha256: `ba62e4ad2c2e56ee2ffdfc86fb5d52b43bc7ff65642a4246282f010fbdd9d5d1`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x52 ss=0x60 oo=0x1E8 — **PASS**

- fixture: `_scratch_ldb_5260_1E8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0e8010000480fb60049898790020000c3`
- js-sha256: `aac7a387b001d803071588118024c4b3edd529e4996f70f642c645e5d2eeed22`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x50 imm=0x1E8 — **PASS**

- fixture: `_scratch_addimm_h50_1E8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0e801000049898780020000c3`
- js-sha256: `a63c229b97189c942fd07bdd4622bcfcc67f550f5e4fe7972808180865b7ed9f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x51 imm=0x1E8 — **PASS**

- fixture: `_scratch_addimm_h51_1E8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0e801000049898788020000c3`
- js-sha256: `356a1a0b3408f7f686339abad6a21ef6d856e7db3c340818c548755f60751813`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x52 imm=0x1E8 — **PASS**

- fixture: `_scratch_addimm_h52_1E8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0e801000049898790020000c3`
- js-sha256: `ea596d905acbddb77450f2f693618792308c05549e59eeeae5b4d04cdb102a04`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x50 imm=0x1E8 — **PASS**

- fixture: `_scratch_subimm_h50_1E8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8e801000049898780020000c3`
- js-sha256: `0e13aa7197e06d2067d67e5ce88f977dd7c9dc1746ef126e65a69268df08d635`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0x1E8 — **PASS**

- fixture: `_scratch_subimm_h51_1E8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8e801000049898788020000c3`
- js-sha256: `58e9756f847685f381c05f00f272297a49fba2942d372b4fea3c875df5fbed2f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x52 imm=0x1E8 — **PASS**

- fixture: `_scratch_subimm_h52_1E8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8e801000049898790020000c3`
- js-sha256: `eeda72c92f5324fcc96b121d2202021424f924135b4b7baa7cbc96156e26585e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=51/52 ss=60 oo=1E8 (finish deferred 1E8 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=1E8 (start deferred 1E8 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=1E8 (start deferred 1E8 SUB triad; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 28B`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5160_1E8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_1E8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1E8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1E8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1E8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_1E8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1E8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1E8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-84-log.md` — this file
- `scripts/_probe/parallel-batch-84-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-090 serialize PASSes + 1 Relock**

Pass pin from body-extend-089 Relock: `e8500277650750c55bc94ec1a9c5e0277367daa257b09371e33f569a8d46c129`.
Handlers before consolidate = 651 (H_00..H_644). Next selectors `40 28B`.. for H_645.. if all serialize.

PASS list for body-extend-090:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_645 | 0x28B | 0x80 LDB | 0x51 0x60 0x1E8 | `498b87000300004881c0e8010000480fb60049898788020000c3` (26B) | `ba62e4ad2c2e56ee` |
| H_646 | 0x28C | 0x80 LDB | 0x52 0x60 0x1E8 | `498b87000300004881c0e8010000480fb60049898790020000c3` (26B) | `aac7a387b001d803` |
| H_647 | 0x28D | 0x62 ADD-IMM | 0x50 0x1E8 | `498b87800200004881c0e801000049898780020000c3` (22B) | `a63c229b97189c94` |
| H_648 | 0x28E | 0x62 ADD-IMM | 0x51 0x1E8 | `498b87880200004881c0e801000049898788020000c3` (22B) | `356a1a0b3408f7f6` |
| H_649 | 0x28F | 0x62 ADD-IMM | 0x52 0x1E8 | `498b87900200004881c0e801000049898790020000c3` (22B) | `ea596d905acbddb7` |
| H_650 | 0x290 | 0x61 SUB-IMM | 0x50 0x1E8 | `498b87800200004881e8e801000049898780020000c3` (22B) | `0e13aa7197e06d20` |
| H_651 | 0x291 | 0x61 SUB-IMM | 0x51 0x1E8 | `498b87880200004881e8e801000049898788020000c3` (22B) | `58e9756f847685f3` |
| H_652 | 0x292 | 0x61 SUB-IMM | 0x52 0x1E8 | `498b87900200004881e8e801000049898790020000c3` (22B) | `eeda72c92f5324fc` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-089 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_644.
- If the parent decides to serialize, append H_645.. at selectors `40 28B`..:
  - H_645 0x80 LDB (80 51 60 1E8) — pin `498b87000300004881c0e8010000480fb60049898788020000c3`
  - H_646 0x80 LDB (80 52 60 1E8) — pin `498b87000300004881c0e8010000480fb60049898790020000c3`
  - H_647 0x62 ADD-IMM (62 50 1E8) — pin `498b87800200004881c0e801000049898780020000c3`
  - H_648 0x62 ADD-IMM (62 51 1E8) — pin `498b87880200004881c0e801000049898788020000c3`
  - H_649 0x62 ADD-IMM (62 52 1E8) — pin `498b87900200004881c0e801000049898790020000c3`
  - H_650 0x61 SUB-IMM (61 50 1E8) — pin `498b87800200004881e8e801000049898780020000c3`
  - H_651 0x61 SUB-IMM (61 51 1E8) — pin `498b87880200004881e8e801000049898788020000c3`
  - H_652 0x61 SUB-IMM (61 52 1E8) — pin `498b87900200004881e8e801000049898790020000c3`
- Plus 1 Relock after append from pin `e8500277…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-090 serialize PASSes + 1 Relock
