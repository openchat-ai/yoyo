# parallel-batch-24 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-24-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-029 (pin `1dd82346…`, handlers = 172, H_158..H_165 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_165 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51; skipped H_165 ADD-IMM 50 3C).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x30 SET | slot=0x52 imm=0xCAFEF00D | `48b80df0feca0000000049898790020000c3` (18) | same | same | Y | `1d191b40e1afa7fb` | `1d191b40e1afa7fb` | PASS |
| 2 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x58 | `498b87000300004883c058480fb60049898780020000c3` (23) | same | same | Y | `79fc958e25bf6b1a` | `79fc958e25bf6b1a` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x51 imm=0x3C | `498b87880200004883c03c49898788020000c3` (19) | same | same | Y | `4aa8dc968083160f` | `4aa8dc968083160f` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x3C | `498b87800200004883e83c49898780020000c3` (19) | same | same | Y | `2a63a066b3ef82ab` | `2a63a066b3ef82ab` | PASS |
| 5 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x58 | `498b87000300004883c058480fb60049898790020000c3` (23) | same | same | Y | `7b4f4bc7fe9fb608` | `7b4f4bc7fe9fb608` | PASS |
| 6 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x58 | `498b87000300004883c058480fb60049898788020000c3` (23) | same | same | Y | `53655a866d4eb1b9` | `53655a866d4eb1b9` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x52 imm=0x3C | `498b87900200004883c03c49898790020000c3` (19) | same | same | Y | `4025f950cb9d1906` | `4025f950cb9d1906` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x51 imm=0x3C | `498b87880200004883e83c49898788020000c3` (19) | same | same | Y | `a436ca73806b6293` | `a436ca73806b6293` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x30 SET slot=0x52 imm=0xCAFEF00D — **PASS**

- fixture: `_scratch_set_52_cafef00d.ty` + `.code.hex`
- expected pin (18B): `48b80df0feca0000000049898790020000c3`
- js-sha256: `1d191b40e1afa7fb4ee86538e903aac8f80bf8a7a946122f76304bbfcfcc6abb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x50 ss=0x60 oo=0x58 — **PASS**

- fixture: `_scratch_ldb_5060_58.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c058480fb60049898780020000c3`
- js-sha256: `79fc958e25bf6b1a90cbf9009fae8a4fd8f9884aa759ba912883d978b05c1a2f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x51 imm=0x3C — **PASS**

- fixture: `_scratch_addimm_h51_3c.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c03c49898788020000c3`
- js-sha256: `4aa8dc968083160f5970bb3f4ee1bd13df09b7a45d82d9eda083824fc0ba27d4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x3C — **PASS**

- fixture: `_scratch_subimm_h50_3c.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e83c49898780020000c3`
- js-sha256: `2a63a066b3ef82ab1ef3d8ddd432f270d866abb3cc9ed7b1f84ba089677b053c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x52 ss=0x60 oo=0x58 — **PASS**

- fixture: `_scratch_ldb_5260_58.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c058480fb60049898790020000c3`
- js-sha256: `7b4f4bc7fe9fb608f91d2fd3066f23f0c683be8a196616f7bc1ea0c7e15604e8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x51 ss=0x60 oo=0x58 — **PASS**

- fixture: `_scratch_ldb_5160_58.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c058480fb60049898788020000c3`
- js-sha256: `53655a866d4eb1b95c0917caa6b70cb551bc9558d7b48068c5f62d53b793478d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x52 imm=0x3C — **PASS**

- fixture: `_scratch_addimm_h52_3c.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c03c49898790020000c3`
- js-sha256: `4025f950cb9d19064c054fa767e552ae3433e7f24140b8dc2b358a03605bc090`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x51 imm=0x3C — **PASS**

- fixture: `_scratch_subimm_h51_3c.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e83c49898788020000c3`
- js-sha256: `a436ca73806b62935c3e08485447df162fd9ed4000f4cffa0a1144a8113ccfdc`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SET at slot 52 imm=CAFEF00D (H_160/H_163=51/50 CAFEF00D; H_53=52 CAFEBABE).
- LDB dd=50/51/52 ss=60 oo=58 (locked LDB oo through 50 at 50/51/52; H_158/H_159 oo=50).
- ADD-IMM at slot 51/52 imm=3C (H_165=50 3C; no 51/52 3C locked).
- SUB-IMM at slot 50/51 imm=3C (no imm=3C SUB-IMM locked; H_164=52 32).
- Skipped suggested INC/DEC 50 (H_17/H_18), GET/ORV/ADDV/SUBV 50 51 (early), IMUL 50 51 (H_34), ADD-IMM 50 3C (H_165).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_set_52_cafef00d.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_58.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_3c.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_3c.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_58.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_58.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_3c.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_3c.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-24-log.md` — this file
- `scripts/_probe/parallel-batch-24-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-030 serialize PASSes + 1 Relock**

Pass pin from body-extend-029 Relock: `1dd8234623853194c8b159ddd7635c2cf8d83d2195cbe480b63c7335b10ea7c2`.
Handlers before consolidate = 172 (H_00..H_165). Next selectors 0xAC.. for H_166.. if all serialize.

PASS list for body-extend-030:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_166 | 0xAC | 0x30 SET | 0x52 0xCAFEF00D | `48b80df0feca0000000049898790020000c3` (18B) | `1d191b40e1afa7fb` |
| H_167 | 0xAD | 0x80 LDB | 0x50 0x60 0x58 | `498b87000300004883c058480fb60049898780020000c3` (23B) | `79fc958e25bf6b1a` |
| H_168 | 0xAE | 0x62 ADD-IMM | 0x51 0x3C | `498b87880200004883c03c49898788020000c3` (19B) | `4aa8dc968083160f` |
| H_169 | 0xAF | 0x61 SUB-IMM | 0x50 0x3C | `498b87800200004883e83c49898780020000c3` (19B) | `2a63a066b3ef82ab` |
| H_170 | 0xB0 | 0x80 LDB | 0x52 0x60 0x58 | `498b87000300004883c058480fb60049898790020000c3` (23B) | `7b4f4bc7fe9fb608` |
| H_171 | 0xB1 | 0x80 LDB | 0x51 0x60 0x58 | `498b87000300004883c058480fb60049898788020000c3` (23B) | `53655a866d4eb1b9` |
| H_172 | 0xB2 | 0x62 ADD-IMM | 0x52 0x3C | `498b87900200004883c03c49898790020000c3` (19B) | `4025f950cb9d1906` |
| H_173 | 0xB3 | 0x61 SUB-IMM | 0x51 0x3C | `498b87880200004883e83c49898788020000c3` (19B) | `a436ca73806b6293` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_165.
- If the parent decides to serialize, append H_166.. at selectors 0xAC..:
  - H_166 0x30 SET (30 52 CAFEF00D) — pin `48b80df0feca0000000049898790020000c3`
  - H_167 0x80 LDB (80 50 60 58) — pin `498b87000300004883c058480fb60049898780020000c3`
  - H_168 0x62 ADD-IMM (62 51 3C) — pin `498b87880200004883c03c49898788020000c3`
  - H_169 0x61 SUB-IMM (61 50 3C) — pin `498b87800200004883e83c49898780020000c3`
  - H_170 0x80 LDB (80 52 60 58) — pin `498b87000300004883c058480fb60049898790020000c3`
  - H_171 0x80 LDB (80 51 60 58) — pin `498b87000300004883c058480fb60049898788020000c3`
  - H_172 0x62 ADD-IMM (62 52 3C) — pin `498b87900200004883c03c49898790020000c3`
  - H_173 0x61 SUB-IMM (61 51 3C) — pin `498b87880200004883e83c49898788020000c3`
- Plus 1 Relock after append from pin `1dd82346…`.

## §7. Consolidation handoff

parent next = body-extend-030 serialize PASSes + 1 Relock
