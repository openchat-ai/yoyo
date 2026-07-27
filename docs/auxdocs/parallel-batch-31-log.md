# parallel-batch-31 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-31-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-036 (pin `3bf549a6…`, handlers = 228, H_214..H_221 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-036 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_221 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x51 imm=0x68 | `498b87880200004883c06849898788020000c3` (19) | same | same | Y | `b2f72feaae60803e` | `b2f72feaae60803e` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x52 imm=0x68 | `498b87900200004883c06849898790020000c3` (19) | same | same | Y | `7819936ee9d0c007` | `7819936ee9d0c007` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x60 | `498b87800200004883e86049898780020000c3` (19) | same | same | Y | `140f19aded02db3b` | `140f19aded02db3b` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x51 imm=0x60 | `498b87880200004883e86049898788020000c3` (19) | same | same | Y | `17f59cbf3cc854a2` | `17f59cbf3cc854a2` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x52 imm=0x60 | `498b87900200004883e86049898790020000c3` (19) | same | same | Y | `af095c6f5e0afc0b` | `af095c6f5e0afc0b` | PASS |
| 6 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x88 | `498b87000300004881c088000000480fb60049898780020000c3` (26) | same | same | Y | `5edbd7f24b9a903a` | `5edbd7f24b9a903a` | PASS |
| 7 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x88 | `498b87000300004881c088000000480fb60049898788020000c3` (26) | same | same | Y | `3bee10754f19b9d5` | `3bee10754f19b9d5` | PASS |
| 8 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x88 | `498b87000300004881c088000000480fb60049898790020000c3` (26) | same | same | Y | `74c53973c0c6f552` | `74c53973c0c6f552` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x51 imm=0x68 — **PASS**

- fixture: `_scratch_addimm_h51_68.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c06849898788020000c3`
- js-sha256: `b2f72feaae60803e16768a53b93b44880403f9d1e1a34f24d81c6c745259cfec`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x52 imm=0x68 — **PASS**

- fixture: `_scratch_addimm_h52_68.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c06849898790020000c3`
- js-sha256: `7819936ee9d0c007938b5a6cb91c8126b28a3fc78f520f144d7d05275bf7a71a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x60 — **PASS**

- fixture: `_scratch_subimm_h50_60.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e86049898780020000c3`
- js-sha256: `140f19aded02db3bd2fe0ca338c60ebd0dabad07f1f6c8a97d6cc6230034b825`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x51 imm=0x60 — **PASS**

- fixture: `_scratch_subimm_h51_60.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e86049898788020000c3`
- js-sha256: `17f59cbf3cc854a2a5f4e58863e09535a12c16203d6fa8d73ef61511504fd537`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x52 imm=0x60 — **PASS**

- fixture: `_scratch_subimm_h52_60.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e86049898790020000c3`
- js-sha256: `af095c6f5e0afc0ba0fabafa9b4bf6b287acca562d811694b8b5c2e8e8e87fd8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x50 ss=0x60 oo=0x88 — **PASS**

- fixture: `_scratch_ldb_5060_88.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c088000000480fb60049898780020000c3`
- js-sha256: `5edbd7f24b9a903a7ffc7898018a1624bb79299314c68aea52fd6eb6068b2aba`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x51 ss=0x60 oo=0x88 — **PASS**

- fixture: `_scratch_ldb_5160_88.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c088000000480fb60049898788020000c3`
- js-sha256: `3bee10754f19b9d5d05f54a85541cf63a9d8eefcea658906415157d0bc582972`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x52 ss=0x60 oo=0x88 — **PASS**

- fixture: `_scratch_ldb_5260_88.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c088000000480fb60049898790020000c3`
- js-sha256: `74c53973c0c6f552ab8faa8f418161edbdcbb9d2217c9a55bd63e0e0553b88a3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot 51/52 imm=68 (complete imm=68 ADD triad with H_221).
- SUB-IMM slot 50/51/52 imm=60 (fresh SUB imm=60 triad).
- LDB dd=50/51/52 ss=60 oo=88 (fresh oo=88 LDB triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h51_68.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_68.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_88.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_88.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_88.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-31-log.md` — this file
- `scripts/_probe/parallel-batch-31-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-037 serialize PASSes + 1 Relock**

Pass pin from body-extend-036 Relock: `3bf549a652a2746e26d16216f3c3d1e6c8c65a6b6403472091240f753d1545ec`.
Handlers before consolidate = 228 (H_00..H_221). Next selectors 0xE4.. for H_222.. if all serialize.

PASS list for body-extend-037:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_222 | 0xE4 | 0x62 ADD-IMM | 0x51 0x68 | `498b87880200004883c06849898788020000c3` (19B) | `b2f72feaae60803e` |
| H_223 | 0xE5 | 0x62 ADD-IMM | 0x52 0x68 | `498b87900200004883c06849898790020000c3` (19B) | `7819936ee9d0c007` |
| H_224 | 0xE6 | 0x61 SUB-IMM | 0x50 0x60 | `498b87800200004883e86049898780020000c3` (19B) | `140f19aded02db3b` |
| H_225 | 0xE7 | 0x61 SUB-IMM | 0x51 0x60 | `498b87880200004883e86049898788020000c3` (19B) | `17f59cbf3cc854a2` |
| H_226 | 0xE8 | 0x61 SUB-IMM | 0x52 0x60 | `498b87900200004883e86049898790020000c3` (19B) | `af095c6f5e0afc0b` |
| H_227 | 0xE9 | 0x80 LDB | 0x50 0x60 0x88 | `498b87000300004881c088000000480fb60049898780020000c3` (26B) | `5edbd7f24b9a903a` |
| H_228 | 0xEA | 0x80 LDB | 0x51 0x60 0x88 | `498b87000300004881c088000000480fb60049898788020000c3` (26B) | `3bee10754f19b9d5` |
| H_229 | 0xEB | 0x80 LDB | 0x52 0x60 0x88 | `498b87000300004881c088000000480fb60049898790020000c3` (26B) | `74c53973c0c6f552` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-036 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_221.
- If the parent decides to serialize, append H_222.. at selectors 0xE4..:
  - H_222 0x62 ADD-IMM (62 51 68) — pin `498b87880200004883c06849898788020000c3`
  - H_223 0x62 ADD-IMM (62 52 68) — pin `498b87900200004883c06849898790020000c3`
  - H_224 0x61 SUB-IMM (61 50 60) — pin `498b87800200004883e86049898780020000c3`
  - H_225 0x61 SUB-IMM (61 51 60) — pin `498b87880200004883e86049898788020000c3`
  - H_226 0x61 SUB-IMM (61 52 60) — pin `498b87900200004883e86049898790020000c3`
  - H_227 0x80 LDB (80 50 60 88) — pin `498b87000300004881c088000000480fb60049898780020000c3`
  - H_228 0x80 LDB (80 51 60 88) — pin `498b87000300004881c088000000480fb60049898788020000c3`
  - H_229 0x80 LDB (80 52 60 88) — pin `498b87000300004881c088000000480fb60049898790020000c3`
- Plus 1 Relock after append from pin `3bf549a6…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-037 serialize PASSes + 1 Relock
