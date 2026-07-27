# parallel-batch-88 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-88-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-093 (pin `04656bbb…`, handlers = 683, H_669..H_676 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-093 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_676 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x50 imm=0x200 | `498b87800200004881e80002000049898780020000c3` (22) | same | same | Y | `616e435fa3303d6d` | `616e435fa3303d6d` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x51 imm=0x200 | `498b87880200004881e80002000049898788020000c3` (22) | same | same | Y | `c68ac43f8d46d532` | `c68ac43f8d46d532` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x52 imm=0x200 | `498b87900200004881e80002000049898790020000c3` (22) | same | same | Y | `aa5d87726f97aedf` | `aa5d87726f97aedf` | PASS |
| 4 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x208 | `498b87000300004881c008020000480fb60049898780020000c3` (26) | same | same | Y | `454561f22b4cd018` | `454561f22b4cd018` | PASS |
| 5 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x208 | `498b87000300004881c008020000480fb60049898788020000c3` (26) | same | same | Y | `4d6d099ee46ef004` | `4d6d099ee46ef004` | PASS |
| 6 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x208 | `498b87000300004881c008020000480fb60049898790020000c3` (26) | same | same | Y | `49ede9483394add3` | `49ede9483394add3` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x50 imm=0x208 | `498b87800200004881c00802000049898780020000c3` (22) | same | same | Y | `20c12c152bbba594` | `20c12c152bbba594` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x51 imm=0x208 | `498b87880200004881c00802000049898788020000c3` (22) | same | same | Y | `612703982c8eadbb` | `612703982c8eadbb` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x50 imm=0x200 — **PASS**

- fixture: `_scratch_subimm_h50_200.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e80002000049898780020000c3`
- js-sha256: `616e435fa3303d6d6ba0710790f2689e99f6628131c1c135d4560e43a12ce990`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x51 imm=0x200 — **PASS**

- fixture: `_scratch_subimm_h51_200.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e80002000049898788020000c3`
- js-sha256: `c68ac43f8d46d532ba9c6f4d1d060cc3b879145e6a2e6770f015e6145d763379`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x52 imm=0x200 — **PASS**

- fixture: `_scratch_subimm_h52_200.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e80002000049898790020000c3`
- js-sha256: `aa5d87726f97aedfbd932d90d43047686e049ef1ac3a86a8492b02739b852c73`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x50 ss=0x60 oo=0x208 — **PASS**

- fixture: `_scratch_ldb_5060_208.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c008020000480fb60049898780020000c3`
- js-sha256: `454561f22b4cd018ef79befab6dd2911e4dfb00566eeabd5111866aea8ff8895`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x51 ss=0x60 oo=0x208 — **PASS**

- fixture: `_scratch_ldb_5160_208.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c008020000480fb60049898788020000c3`
- js-sha256: `4d6d099ee46ef0045a4eb3e81c5b58b73a4a8ad82b907f9192f1368a33112139`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x52 ss=0x60 oo=0x208 — **PASS**

- fixture: `_scratch_ldb_5260_208.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c008020000480fb60049898790020000c3`
- js-sha256: `49ede9483394add3545ffd850a337cf4e2a608953a0b1db5a7bbce046b8ea331`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x50 imm=0x208 — **PASS**

- fixture: `_scratch_addimm_h50_208.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c00802000049898780020000c3`
- js-sha256: `20c12c152bbba59406e5c82303bc3ccd3ddc945fdf57984d2226d57c16426da0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x51 imm=0x208 — **PASS**

- fixture: `_scratch_addimm_h51_208.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c00802000049898788020000c3`
- js-sha256: `612703982c8eadbb83922b686ef84d5dd929cde146cc8e77328b01241092d313`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=50/51/52 imm=200 (finish deferred 200 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=208 (start 208 LDB triad; imm32 26B).
- ADD-IMM slot=50/51 imm=208 (start 208 ADD triad; imm32 22B).
- ADD-IMM slot=52 imm=208 + SUB-IMM slot=50/51/52 imm=208 deferred to next batch.
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 2AB`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h50_200.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_200.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_200.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_208.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_208.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_208.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_208.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_208.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-88-log.md` — this file
- `scripts/_probe/parallel-batch-88-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-094 serialize PASSes + 1 Relock**

Pass pin from body-extend-093 Relock: `04656bbbbb152b5402bd76daa324a51a7f68477df3b3615827ef88aa2907978b`.
Handlers before consolidate = 683 (H_00..H_676). Next selectors `40 2AB`.. for H_677.. if all serialize.

PASS list for body-extend-094:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_677 | 0x2AB | 0x61 SUB-IMM | 0x50 0x200 | `498b87800200004881e80002000049898780020000c3` (22B) | `616e435fa3303d6d` |
| H_678 | 0x2AC | 0x61 SUB-IMM | 0x51 0x200 | `498b87880200004881e80002000049898788020000c3` (22B) | `c68ac43f8d46d532` |
| H_679 | 0x2AD | 0x61 SUB-IMM | 0x52 0x200 | `498b87900200004881e80002000049898790020000c3` (22B) | `aa5d87726f97aedf` |
| H_680 | 0x2AE | 0x80 LDB | 0x50 0x60 0x208 | `498b87000300004881c008020000480fb60049898780020000c3` (26B) | `454561f22b4cd018` |
| H_681 | 0x2AF | 0x80 LDB | 0x51 0x60 0x208 | `498b87000300004881c008020000480fb60049898788020000c3` (26B) | `4d6d099ee46ef004` |
| H_682 | 0x2B0 | 0x80 LDB | 0x52 0x60 0x208 | `498b87000300004881c008020000480fb60049898790020000c3` (26B) | `49ede9483394add3` |
| H_683 | 0x2B1 | 0x62 ADD-IMM | 0x50 0x208 | `498b87800200004881c00802000049898780020000c3` (22B) | `20c12c152bbba594` |
| H_684 | 0x2B2 | 0x62 ADD-IMM | 0x51 0x208 | `498b87880200004881c00802000049898788020000c3` (22B) | `612703982c8eadbb` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-093 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_676.
- If the parent decides to serialize, append H_677.. at selectors `40 2AB`..:
  - H_677 0x61 SUB-IMM (61 50 200) — pin `498b87800200004881e80002000049898780020000c3`
  - H_678 0x61 SUB-IMM (61 51 200) — pin `498b87880200004881e80002000049898788020000c3`
  - H_679 0x61 SUB-IMM (61 52 200) — pin `498b87900200004881e80002000049898790020000c3`
  - H_680 0x80 LDB (80 50 60 208) — pin `498b87000300004881c008020000480fb60049898780020000c3`
  - H_681 0x80 LDB (80 51 60 208) — pin `498b87000300004881c008020000480fb60049898788020000c3`
  - H_682 0x80 LDB (80 52 60 208) — pin `498b87000300004881c008020000480fb60049898790020000c3`
  - H_683 0x62 ADD-IMM (62 50 208) — pin `498b87800200004881c00802000049898780020000c3`
  - H_684 0x62 ADD-IMM (62 51 208) — pin `498b87880200004881c00802000049898788020000c3`
- Plus 1 Relock after append from pin `04656bbb…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: ADD-IMM slot=52 imm=208; SUB-IMM slot=50/51/52 imm=208; finish 208 ADD/SUB ladder.

## §7. Consolidation handoff

parent next = body-extend-094 serialize PASSes + 1 Relock
