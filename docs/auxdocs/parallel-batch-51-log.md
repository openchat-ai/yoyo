# parallel-batch-51 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-51-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-056 (pin `824207c6…`, handlers = 388, H_374..H_381 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-056 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_381 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x52 imm=0xF8 | `498b87900200004881e8f800000049898790020000c3` (22) | same | same | Y | `69b7068d45f8bf5d` | `69b7068d45f8bf5d` | PASS |
| 2 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x100 | `498b87000300004881c000010000480fb60049898780020000c3` (26) | same | same | Y | `435a012fe7d4460d` | `435a012fe7d4460d` | PASS |
| 3 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x100 | `498b87000300004881c000010000480fb60049898788020000c3` (26) | same | same | Y | `efcb4fa1a01828f3` | `efcb4fa1a01828f3` | PASS |
| 4 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x100 | `498b87000300004881c000010000480fb60049898790020000c3` (26) | same | same | Y | `a26708edf890025c` | `a26708edf890025c` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x50 imm=0x100 | `498b87800200004881c00001000049898780020000c3` (22) | same | same | Y | `220b570f6901c757` | `220b570f6901c757` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x51 imm=0x100 | `498b87880200004881c00001000049898788020000c3` (22) | same | same | Y | `2bca9f9743f2fb78` | `2bca9f9743f2fb78` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x52 imm=0x100 | `498b87900200004881c00001000049898790020000c3` (22) | same | same | Y | `6f99edae6e28e2a6` | `6f99edae6e28e2a6` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x50 imm=0x100 | `498b87800200004881e80001000049898780020000c3` (22) | same | same | Y | `a89c3aeffbbddb04` | `a89c3aeffbbddb04` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x52 imm=0xF8 — **PASS**

- fixture: `_scratch_subimm_h52_f8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8f800000049898790020000c3`
- js-sha256: `69b7068d45f8bf5d56a1e5bd830dde9f4cd9778f5817a9c7a8784f4385ce4ff7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x50 ss=0x60 oo=0x100 — **PASS**

- fixture: `_scratch_ldb_5060_100.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c000010000480fb60049898780020000c3`
- js-sha256: `435a012fe7d4460d49e5cebc9808b62429de0ade9256ee8f08064b4519bfa22e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x51 ss=0x60 oo=0x100 — **PASS**

- fixture: `_scratch_ldb_5160_100.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c000010000480fb60049898788020000c3`
- js-sha256: `efcb4fa1a01828f3d962df2b84ec47c48ee8192c945b455e60e78bb11b16bd14`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x52 ss=0x60 oo=0x100 — **PASS**

- fixture: `_scratch_ldb_5260_100.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c000010000480fb60049898790020000c3`
- js-sha256: `a26708edf890025cee4cdf25c62650d88ed29141f6092408f6d3056b32060303`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x50 imm=0x100 — **PASS**

- fixture: `_scratch_addimm_h50_100.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c00001000049898780020000c3`
- js-sha256: `220b570f6901c7579bf2b076c8d91301acf3569c6d2ef852c244cfb547170de9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x51 imm=0x100 — **PASS**

- fixture: `_scratch_addimm_h51_100.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c00001000049898788020000c3`
- js-sha256: `2bca9f9743f2fb7814418f7214f35ced43c2277247d18be3b4d5204cb8aa89ec`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x52 imm=0x100 — **PASS**

- fixture: `_scratch_addimm_h52_100.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c00001000049898790020000c3`
- js-sha256: `6f99edae6e28e2a68d3a497fa92492ba1d6d6c4f5afde100b4a8fa5e0f03eae8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x50 imm=0x100 — **PASS**

- fixture: `_scratch_subimm_h50_100.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e80001000049898780020000c3`
- js-sha256: `a89c3aeffbbddb04d1a96a36f68c5c10de1d43961651ddcac70c6037cc35191b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=52 imm=F8 (finish F8 SUB triad after H_380/H_381; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=100 (next oo after F8 triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=100 (fresh imm after F8; imm32 22B).
- SUB-IMM slot=50 imm=100 (start SUB * 100; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 184`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h52_f8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_100.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_100.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_100.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_100.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_100.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_100.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_100.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-51-log.md` — this file
- `scripts/_probe/parallel-batch-51-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-057 serialize PASSes + 1 Relock**

Pass pin from body-extend-056 Relock: `824207c608fe5d03e4bd1c3bca1f33aec844dd62f4bc66ca4a6877364538314b`.
Handlers before consolidate = 388 (H_00..H_381). Next selectors `40 184`.. for H_382.. if all serialize.

PASS list for body-extend-057:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_382 | 0x184 | 0x61 SUB-IMM | 0x52 0xF8 | `498b87900200004881e8f800000049898790020000c3` (22B) | `69b7068d45f8bf5d` |
| H_383 | 0x185 | 0x80 LDB | 0x50 0x60 0x100 | `498b87000300004881c000010000480fb60049898780020000c3` (26B) | `435a012fe7d4460d` |
| H_384 | 0x186 | 0x80 LDB | 0x51 0x60 0x100 | `498b87000300004881c000010000480fb60049898788020000c3` (26B) | `efcb4fa1a01828f3` |
| H_385 | 0x187 | 0x80 LDB | 0x52 0x60 0x100 | `498b87000300004881c000010000480fb60049898790020000c3` (26B) | `a26708edf890025c` |
| H_386 | 0x188 | 0x62 ADD-IMM | 0x50 0x100 | `498b87800200004881c00001000049898780020000c3` (22B) | `220b570f6901c757` |
| H_387 | 0x189 | 0x62 ADD-IMM | 0x51 0x100 | `498b87880200004881c00001000049898788020000c3` (22B) | `2bca9f9743f2fb78` |
| H_388 | 0x18A | 0x62 ADD-IMM | 0x52 0x100 | `498b87900200004881c00001000049898790020000c3` (22B) | `6f99edae6e28e2a6` |
| H_389 | 0x18B | 0x61 SUB-IMM | 0x50 0x100 | `498b87800200004881e80001000049898780020000c3` (22B) | `a89c3aeffbbddb04` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-056 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_381.
- If the parent decides to serialize, append H_382.. at selectors `40 184`..:
  - H_382 0x61 SUB-IMM (61 52 F8) — pin `498b87900200004881e8f800000049898790020000c3`
  - H_383 0x80 LDB (80 50 60 100) — pin `498b87000300004881c000010000480fb60049898780020000c3`
  - H_384 0x80 LDB (80 51 60 100) — pin `498b87000300004881c000010000480fb60049898788020000c3`
  - H_385 0x80 LDB (80 52 60 100) — pin `498b87000300004881c000010000480fb60049898790020000c3`
  - H_386 0x62 ADD-IMM (62 50 100) — pin `498b87800200004881c00001000049898780020000c3`
  - H_387 0x62 ADD-IMM (62 51 100) — pin `498b87880200004881c00001000049898788020000c3`
  - H_388 0x62 ADD-IMM (62 52 100) — pin `498b87900200004881c00001000049898790020000c3`
  - H_389 0x61 SUB-IMM (61 50 100) — pin `498b87800200004881e80001000049898780020000c3`
- Plus 1 Relock after append from pin `824207c6…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-057 serialize PASSes + 1 Relock
