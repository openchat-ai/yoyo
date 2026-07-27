# parallel-batch-29 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-29-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-034 (pin `e531a0a8…`, handlers = 212, H_198..H_205 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-034 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_205 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x51 imm=0x58 | `498b87880200004883c05849898788020000c3` (19) | same | same | Y | `4ff049a8441518ba` | `4ff049a8441518ba` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x52 imm=0x58 | `498b87900200004883c05849898790020000c3` (19) | same | same | Y | `44445f68d85c340f` | `44445f68d85c340f` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x50 | `498b87800200004883e85049898780020000c3` (19) | same | same | Y | `e51df228ac034429` | `e51df228ac034429` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x52 imm=0x50 | `498b87900200004883e85049898790020000c3` (19) | same | same | Y | `1b61da415449f276` | `1b61da415449f276` | PASS |
| 5 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x78 | `498b87000300004883c078480fb60049898790020000c3` (23) | same | same | Y | `f7221a4afaec1410` | `f7221a4afaec1410` | PASS |
| 6 | 0x30 SET | slot=0x52 imm=0xC0DEC0DE | `48b8dec0dec00000000049898790020000c3` (18) | same | same | Y | `20960f8da0f70a8e` | `20960f8da0f70a8e` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x50 imm=0x60 | `498b87800200004883c06049898780020000c3` (19) | same | same | Y | `88c5f7c3de52c972` | `88c5f7c3de52c972` | PASS |
| 8 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x80 | `498b87000300004881c080000000480fb60049898780020000c3` (26) | same | same | Y | `5c4e0e3a942cbe06` | `5c4e0e3a942cbe06` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x51 imm=0x58 — **PASS**

- fixture: `_scratch_addimm_h51_58.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c05849898788020000c3`
- js-sha256: `4ff049a8441518baae98d945eef56610c0acafd2a50bfbd12e247392eef0d0c5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x52 imm=0x58 — **PASS**

- fixture: `_scratch_addimm_h52_58.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c05849898790020000c3`
- js-sha256: `44445f68d85c340f9c168ae79fcfd5182458fd37f62ee76a7f2b576060b0598d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x50 — **PASS**

- fixture: `_scratch_subimm_h50_50.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e85049898780020000c3`
- js-sha256: `e51df228ac03442961671193993852ff712426ecfdc091920f9b81d60381eae5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x52 imm=0x50 — **PASS**

- fixture: `_scratch_subimm_h52_50.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e85049898790020000c3`
- js-sha256: `1b61da415449f276ec05815d2e5feeb54e563d3a3967eb8fef1d5fddf0204b9a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x52 ss=0x60 oo=0x78 — **PASS**

- fixture: `_scratch_ldb_5260_78.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c078480fb60049898790020000c3`
- js-sha256: `f7221a4afaec14106ec5535002b27e4aac7e3c192bfe76d3dc4bf186a8409d9f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x30 SET slot=0x52 imm=0xC0DEC0DE — **PASS**

- fixture: `_scratch_set_52_c0dec0de.ty` + `.code.hex`
- expected pin (18B): `48b8dec0dec00000000049898790020000c3`
- js-sha256: `20960f8da0f70a8efaf8cbd49d4df2683a22cbccb8e45c57be90e1a9fa912a43`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x50 imm=0x60 — **PASS**

- fixture: `_scratch_addimm_h50_60.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c06049898780020000c3`
- js-sha256: `88c5f7c3de52c9726d0ff4aa8d0fc20425af6466b1397e0fa049250ccfd1fbbd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x50 ss=0x60 oo=0x80 — **PASS**

- fixture: `_scratch_ldb_5060_80.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c080000000480fb60049898780020000c3`
- js-sha256: `5c4e0e3a942cbe06bb59e8e0a4b9bb801e02d1ded417a8ac20a1eedac6e9737b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot 51/52 imm=58 (complete imm=58 ADD triad with H_203).
- SUB-IMM slot 50/52 imm=50 (complete SUB imm=50 with H_204).
- LDB dd=52 ss=60 oo=78 (complete oo=78 LDB triad with H_201/H_205).
- SET slot 52 imm=C0DEC0DE (complete C0DEC0DE SET triad with H_194/H_202).
- ADD-IMM slot 50 imm=60 (fresh imm rung above 58).
- LDB dd=50 ss=60 oo=80 (next oo rung above 78).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h51_58.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_58.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_78.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_52_c0dec0de.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_80.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-29-log.md` — this file
- `scripts/_probe/parallel-batch-29-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-035 serialize PASSes + 1 Relock**

Pass pin from body-extend-034 Relock: `e531a0a8962e21ecce4f085df042195b60eb72b69f90d468f04cfeaa9c283588`.
Handlers before consolidate = 212 (H_00..H_205). Next selectors 0xD4.. for H_206.. if all serialize.

PASS list for body-extend-035:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_206 | 0xD4 | 0x62 ADD-IMM | 0x51 0x58 | `498b87880200004883c05849898788020000c3` (19B) | `4ff049a8441518ba` |
| H_207 | 0xD5 | 0x62 ADD-IMM | 0x52 0x58 | `498b87900200004883c05849898790020000c3` (19B) | `44445f68d85c340f` |
| H_208 | 0xD6 | 0x61 SUB-IMM | 0x50 0x50 | `498b87800200004883e85049898780020000c3` (19B) | `e51df228ac034429` |
| H_209 | 0xD7 | 0x61 SUB-IMM | 0x52 0x50 | `498b87900200004883e85049898790020000c3` (19B) | `1b61da415449f276` |
| H_210 | 0xD8 | 0x80 LDB | 0x52 0x60 0x78 | `498b87000300004883c078480fb60049898790020000c3` (23B) | `f7221a4afaec1410` |
| H_211 | 0xD9 | 0x30 SET | 0x52 0xC0DEC0DE | `48b8dec0dec00000000049898790020000c3` (18B) | `20960f8da0f70a8e` |
| H_212 | 0xDA | 0x62 ADD-IMM | 0x50 0x60 | `498b87800200004883c06049898780020000c3` (19B) | `88c5f7c3de52c972` |
| H_213 | 0xDB | 0x80 LDB | 0x50 0x60 0x80 | `498b87000300004881c080000000480fb60049898780020000c3` (26B) | `5c4e0e3a942cbe06` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-034 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_205.
- If the parent decides to serialize, append H_206.. at selectors 0xD4..:
  - H_206 0x62 ADD-IMM (62 51 58) — pin `498b87880200004883c05849898788020000c3`
  - H_207 0x62 ADD-IMM (62 52 58) — pin `498b87900200004883c05849898790020000c3`
  - H_208 0x61 SUB-IMM (61 50 50) — pin `498b87800200004883e85049898780020000c3`
  - H_209 0x61 SUB-IMM (61 52 50) — pin `498b87900200004883e85049898790020000c3`
  - H_210 0x80 LDB (80 52 60 78) — pin `498b87000300004883c078480fb60049898790020000c3`
  - H_211 0x30 SET (30 52 C0DEC0DE) — pin `48b8dec0dec00000000049898790020000c3`
  - H_212 0x62 ADD-IMM (62 50 60) — pin `498b87800200004883c06049898780020000c3`
  - H_213 0x80 LDB (80 50 60 80) — pin `498b87000300004881c080000000480fb60049898780020000c3`
- Plus 1 Relock after append from pin `e531a0a8…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-035 serialize PASSes + 1 Relock
