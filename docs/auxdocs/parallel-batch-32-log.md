# parallel-batch-32 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-32-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-037 (pin `39d454a1…`, handlers = 236, H_222..H_229 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-037 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_229 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x50 imm=0x70 | `498b87800200004883c07049898780020000c3` (19) | same | same | Y | `fd00d3aaf8d154fd` | `fd00d3aaf8d154fd` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x51 imm=0x70 | `498b87880200004883c07049898788020000c3` (19) | same | same | Y | `0b9f43d82535758d` | `0b9f43d82535758d` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x52 imm=0x70 | `498b87900200004883c07049898790020000c3` (19) | same | same | Y | `a84dbb9e54bc5205` | `a84dbb9e54bc5205` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x68 | `498b87800200004883e86849898780020000c3` (19) | same | same | Y | `310e437ef9fb3edd` | `310e437ef9fb3edd` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x51 imm=0x68 | `498b87880200004883e86849898788020000c3` (19) | same | same | Y | `9bb82476b37c5941` | `9bb82476b37c5941` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x52 imm=0x68 | `498b87900200004883e86849898790020000c3` (19) | same | same | Y | `0f9edc3307cfe318` | `0f9edc3307cfe318` | PASS |
| 7 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x90 | `498b87000300004881c090000000480fb60049898780020000c3` (26) | same | same | Y | `19191871913c0878` | `19191871913c0878` | PASS |
| 8 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x90 | `498b87000300004881c090000000480fb60049898788020000c3` (26) | same | same | Y | `7571ee40b3a097be` | `7571ee40b3a097be` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x50 imm=0x70 — **PASS**

- fixture: `_scratch_addimm_h50_70.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c07049898780020000c3`
- js-sha256: `fd00d3aaf8d154fd67813dce47a22b94d7b873f88a54a6187373bbd345b317fd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x51 imm=0x70 — **PASS**

- fixture: `_scratch_addimm_h51_70.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c07049898788020000c3`
- js-sha256: `0b9f43d82535758d3016e610890cb6662aaea3b3e9daa768569cd9f5a98fad78`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x52 imm=0x70 — **PASS**

- fixture: `_scratch_addimm_h52_70.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c07049898790020000c3`
- js-sha256: `a84dbb9e54bc52054cba7e051f37be51ba73cb5e14a796e2d7e561b2a3082ab3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x68 — **PASS**

- fixture: `_scratch_subimm_h50_68.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e86849898780020000c3`
- js-sha256: `310e437ef9fb3eddfc216b6eb9c48ca3926cefd6a4bd9796427519f8d0f6d1de`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x51 imm=0x68 — **PASS**

- fixture: `_scratch_subimm_h51_68.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e86849898788020000c3`
- js-sha256: `9bb82476b37c59414d4485fb8128f88e2677c25ebae95a9bf85edfe8eca78c37`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x52 imm=0x68 — **PASS**

- fixture: `_scratch_subimm_h52_68.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e86849898790020000c3`
- js-sha256: `0f9edc3307cfe318c714a83ef611b8fff5e11f64103836bbbd9cf7d7dd53b132`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x50 ss=0x60 oo=0x90 — **PASS**

- fixture: `_scratch_ldb_5060_90.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c090000000480fb60049898780020000c3`
- js-sha256: `19191871913c0878f8e01a7aaaf640a795bfb030f435257a444c57d5c3c3c87f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x51 ss=0x60 oo=0x90 — **PASS**

- fixture: `_scratch_ldb_5160_90.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c090000000480fb60049898788020000c3`
- js-sha256: `7571ee40b3a097be8f9903019305214612c61ce5dcb4100285340fe9c63084b0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot 50/51/52 imm=70 (fresh ADD imm=70 triad).
- SUB-IMM slot 50/51/52 imm=68 (fresh SUB imm=68 triad; complements locked ADD-IMM * 68).
- LDB dd=50/51 ss=60 oo=90 (fresh oo=90 LDB pair; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h50_70.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_70.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_70.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_68.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_68.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_68.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_90.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_90.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-32-log.md` — this file
- `scripts/_probe/parallel-batch-32-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-038 serialize PASSes + 1 Relock**

Pass pin from body-extend-037 Relock: `39d454a194359d1d682b0638381fa14cbdec617e707f26b2b2405e05be7f9ede`.
Handlers before consolidate = 236 (H_00..H_229). Next selectors 0xEC.. for H_230.. if all serialize.

PASS list for body-extend-038:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_230 | 0xEC | 0x62 ADD-IMM | 0x50 0x70 | `498b87800200004883c07049898780020000c3` (19B) | `fd00d3aaf8d154fd` |
| H_231 | 0xED | 0x62 ADD-IMM | 0x51 0x70 | `498b87880200004883c07049898788020000c3` (19B) | `0b9f43d82535758d` |
| H_232 | 0xEE | 0x62 ADD-IMM | 0x52 0x70 | `498b87900200004883c07049898790020000c3` (19B) | `a84dbb9e54bc5205` |
| H_233 | 0xEF | 0x61 SUB-IMM | 0x50 0x68 | `498b87800200004883e86849898780020000c3` (19B) | `310e437ef9fb3edd` |
| H_234 | 0xF0 | 0x61 SUB-IMM | 0x51 0x68 | `498b87880200004883e86849898788020000c3` (19B) | `9bb82476b37c5941` |
| H_235 | 0xF1 | 0x61 SUB-IMM | 0x52 0x68 | `498b87900200004883e86849898790020000c3` (19B) | `0f9edc3307cfe318` |
| H_236 | 0xF2 | 0x80 LDB | 0x50 0x60 0x90 | `498b87000300004881c090000000480fb60049898780020000c3` (26B) | `19191871913c0878` |
| H_237 | 0xF3 | 0x80 LDB | 0x51 0x60 0x90 | `498b87000300004881c090000000480fb60049898788020000c3` (26B) | `7571ee40b3a097be` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-037 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_229.
- If the parent decides to serialize, append H_230.. at selectors 0xEC..:
  - H_230 0x62 ADD-IMM (62 50 70) — pin `498b87800200004883c07049898780020000c3`
  - H_231 0x62 ADD-IMM (62 51 70) — pin `498b87880200004883c07049898788020000c3`
  - H_232 0x62 ADD-IMM (62 52 70) — pin `498b87900200004883c07049898790020000c3`
  - H_233 0x61 SUB-IMM (61 50 68) — pin `498b87800200004883e86849898780020000c3`
  - H_234 0x61 SUB-IMM (61 51 68) — pin `498b87880200004883e86849898788020000c3`
  - H_235 0x61 SUB-IMM (61 52 68) — pin `498b87900200004883e86849898790020000c3`
  - H_236 0x80 LDB (80 50 60 90) — pin `498b87000300004881c090000000480fb60049898780020000c3`
  - H_237 0x80 LDB (80 51 60 90) — pin `498b87000300004881c090000000480fb60049898788020000c3`
- Plus 1 Relock after append from pin `39d454a1…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-038 serialize PASSes + 1 Relock
