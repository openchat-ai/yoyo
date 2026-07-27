# parallel-batch-30 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-30-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-035 (pin `23f42236…`, handlers = 220, H_206..H_213 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-035 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_213 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x51 imm=0x60 | `498b87880200004883c06049898788020000c3` (19) | same | same | Y | `2e762fb2ad102e6a` | `2e762fb2ad102e6a` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x52 imm=0x60 | `498b87900200004883c06049898790020000c3` (19) | same | same | Y | `efec3943630fb998` | `efec3943630fb998` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x58 | `498b87800200004883e85849898780020000c3` (19) | same | same | Y | `a7d41e13060d56b7` | `a7d41e13060d56b7` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x51 imm=0x58 | `498b87880200004883e85849898788020000c3` (19) | same | same | Y | `d9559da92e31429b` | `d9559da92e31429b` | PASS |
| 5 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x80 | `498b87000300004881c080000000480fb60049898788020000c3` (26) | same | same | Y | `f39364a89ec6b361` | `f39364a89ec6b361` | PASS |
| 6 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x80 | `498b87000300004881c080000000480fb60049898790020000c3` (26) | same | same | Y | `d239426ce0456ebf` | `d239426ce0456ebf` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0x58 | `498b87900200004883e85849898790020000c3` (19) | same | same | Y | `155b83f538845515` | `155b83f538845515` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x50 imm=0x68 | `498b87800200004883c06849898780020000c3` (19) | same | same | Y | `8390493232f90387` | `8390493232f90387` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x51 imm=0x60 — **PASS**

- fixture: `_scratch_addimm_h51_60.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c06049898788020000c3`
- js-sha256: `2e762fb2ad102e6a117754a9757b83ebefa2cb33e8cfad7e6b056dae64ad9318`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x52 imm=0x60 — **PASS**

- fixture: `_scratch_addimm_h52_60.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c06049898790020000c3`
- js-sha256: `efec3943630fb9988eace80e5c0f18753a72a78871ce173cd33206c6183373e6`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x58 — **PASS**

- fixture: `_scratch_subimm_h50_58.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e85849898780020000c3`
- js-sha256: `a7d41e13060d56b759bad08c82f4da839255df32a27a2cc3c1ed5ed226486e6b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x51 imm=0x58 — **PASS**

- fixture: `_scratch_subimm_h51_58.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e85849898788020000c3`
- js-sha256: `d9559da92e31429bc2418d7e71c0a2f5f70a1dca17a0fa500ecf51cb8b321fbe`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x51 ss=0x60 oo=0x80 — **PASS**

- fixture: `_scratch_ldb_5160_80.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c080000000480fb60049898788020000c3`
- js-sha256: `f39364a89ec6b3616fc726561a0d03a9a6f220876e3a72d4db83828c20bb7444`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x52 ss=0x60 oo=0x80 — **PASS**

- fixture: `_scratch_ldb_5260_80.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c080000000480fb60049898790020000c3`
- js-sha256: `d239426ce0456ebf0b535f72a28e26996c9270146443a10c43912a878fb56a50`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0x58 — **PASS**

- fixture: `_scratch_subimm_h52_58.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e85849898790020000c3`
- js-sha256: `155b83f5388455152e75d9abd12c7dca908df8b055de59218f9c4125699c9688`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x50 imm=0x68 — **PASS**

- fixture: `_scratch_addimm_h50_68.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c06849898780020000c3`
- js-sha256: `8390493232f90387c309b4a307b8d14f63885109c65f0701f48cf41746a6f415`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot 51/52 imm=60 (complete imm=60 ADD triad with H_212).
- SUB-IMM slot 50/51/52 imm=58 (fresh SUB imm=58 triad).
- LDB dd=51/52 ss=60 oo=80 (complete oo=80 LDB triad with H_213; imm32 26B).
- ADD-IMM slot 50 imm=68 (fresh imm rung above 60).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h51_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_58.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_58.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_80.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_80.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_58.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_68.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-30-log.md` — this file
- `scripts/_probe/parallel-batch-30-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-036 serialize PASSes + 1 Relock**

Pass pin from body-extend-035 Relock: `23f42236c6097a13e83a15c861d51845fbe1da64eadfabdb95fdeaca3ebe55f5`.
Handlers before consolidate = 220 (H_00..H_213). Next selectors 0xDC.. for H_214.. if all serialize.

PASS list for body-extend-036:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_214 | 0xDC | 0x62 ADD-IMM | 0x51 0x60 | `498b87880200004883c06049898788020000c3` (19B) | `2e762fb2ad102e6a` |
| H_215 | 0xDD | 0x62 ADD-IMM | 0x52 0x60 | `498b87900200004883c06049898790020000c3` (19B) | `efec3943630fb998` |
| H_216 | 0xDE | 0x61 SUB-IMM | 0x50 0x58 | `498b87800200004883e85849898780020000c3` (19B) | `a7d41e13060d56b7` |
| H_217 | 0xDF | 0x61 SUB-IMM | 0x51 0x58 | `498b87880200004883e85849898788020000c3` (19B) | `d9559da92e31429b` |
| H_218 | 0xE0 | 0x80 LDB | 0x51 0x60 0x80 | `498b87000300004881c080000000480fb60049898788020000c3` (26B) | `f39364a89ec6b361` |
| H_219 | 0xE1 | 0x80 LDB | 0x52 0x60 0x80 | `498b87000300004881c080000000480fb60049898790020000c3` (26B) | `d239426ce0456ebf` |
| H_220 | 0xE2 | 0x61 SUB-IMM | 0x52 0x58 | `498b87900200004883e85849898790020000c3` (19B) | `155b83f538845515` |
| H_221 | 0xE3 | 0x62 ADD-IMM | 0x50 0x68 | `498b87800200004883c06849898780020000c3` (19B) | `8390493232f90387` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-035 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_213.
- If the parent decides to serialize, append H_214.. at selectors 0xDC..:
  - H_214 0x62 ADD-IMM (62 51 60) — pin `498b87880200004883c06049898788020000c3`
  - H_215 0x62 ADD-IMM (62 52 60) — pin `498b87900200004883c06049898790020000c3`
  - H_216 0x61 SUB-IMM (61 50 58) — pin `498b87800200004883e85849898780020000c3`
  - H_217 0x61 SUB-IMM (61 51 58) — pin `498b87880200004883e85849898788020000c3`
  - H_218 0x80 LDB (80 51 60 80) — pin `498b87000300004881c080000000480fb60049898788020000c3`
  - H_219 0x80 LDB (80 52 60 80) — pin `498b87000300004881c080000000480fb60049898790020000c3`
  - H_220 0x61 SUB-IMM (61 52 58) — pin `498b87900200004883e85849898790020000c3`
  - H_221 0x62 ADD-IMM (62 50 68) — pin `498b87800200004883c06849898780020000c3`
- Plus 1 Relock after append from pin `23f42236…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-036 serialize PASSes + 1 Relock
