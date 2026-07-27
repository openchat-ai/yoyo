# parallel-batch-38 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-38-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-043 (pin `113decd0…`, handlers = 284, H_270..H_277 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-043 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_277 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x50 imm=0x98 | `498b87800200004881c09800000049898780020000c3` (22) | same | same | Y | `13b9014e066c9897` | `13b9014e066c9897` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x51 imm=0x98 | `498b87880200004881c09800000049898788020000c3` (22) | same | same | Y | `eaf423344be083bb` | `eaf423344be083bb` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x52 imm=0x98 | `498b87900200004881c09800000049898790020000c3` (22) | same | same | Y | `0374f755088d14c3` | `0374f755088d14c3` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x98 | `498b87800200004881e89800000049898780020000c3` (22) | same | same | Y | `39737d6b950d19d4` | `39737d6b950d19d4` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x51 imm=0x98 | `498b87880200004881e89800000049898788020000c3` (22) | same | same | Y | `7dd6789e588e0525` | `7dd6789e588e0525` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x52 imm=0x98 | `498b87900200004881e89800000049898790020000c3` (22) | same | same | Y | `4df6f69f74da2e8d` | `4df6f69f74da2e8d` | PASS |
| 7 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xB8 | `498b87000300004881c0b8000000480fb60049898780020000c3` (26) | same | same | Y | `c0d9668174c58dd0` | `c0d9668174c58dd0` | PASS |
| 8 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xB8 | `498b87000300004881c0b8000000480fb60049898788020000c3` (26) | same | same | Y | `0e4180bb03065699` | `0e4180bb03065699` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x50 imm=0x98 — **PASS**

- fixture: `_scratch_addimm_h50_98.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c09800000049898780020000c3`
- js-sha256: `13b9014e066c98976376ee2850cb7d3fc2e6bd0d3ef80bbbd8a9cd13073e161d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x51 imm=0x98 — **PASS**

- fixture: `_scratch_addimm_h51_98.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c09800000049898788020000c3`
- js-sha256: `eaf423344be083bbb8984ea6223837dbfcae32483079cd2e5df726fa4bd5d54e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x52 imm=0x98 — **PASS**

- fixture: `_scratch_addimm_h52_98.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c09800000049898790020000c3`
- js-sha256: `0374f755088d14c3b6ad6c142a967885cadce8c9b9089f1bac6176163ae8a547`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x98 — **PASS**

- fixture: `_scratch_subimm_h50_98.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e89800000049898780020000c3`
- js-sha256: `39737d6b950d19d41d864cb081f21bb99f092d6264a976bcafff2bf604469148`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x51 imm=0x98 — **PASS**

- fixture: `_scratch_subimm_h51_98.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e89800000049898788020000c3`
- js-sha256: `7dd6789e588e052568acffd479f315f11b111ef6b2d017c8053e4b9bfd53a4e9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x52 imm=0x98 — **PASS**

- fixture: `_scratch_subimm_h52_98.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e89800000049898790020000c3`
- js-sha256: `4df6f69f74da2e8d20a961321a4d177483303f8324be356adb8e4905a322c54e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x50 ss=0x60 oo=0xB8 — **PASS**

- fixture: `_scratch_ldb_5060_b8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b8000000480fb60049898780020000c3`
- js-sha256: `c0d9668174c58dd0043d62f032c5e053d16b39f223de538506fe4a12e5e7ab4f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x51 ss=0x60 oo=0xB8 — **PASS**

- fixture: `_scratch_ldb_5160_b8.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b8000000480fb60049898788020000c3`
- js-sha256: `0e4180bb03065699dda791bd44881918b2db0e4467e62f0216505871c4f58873`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=50/51/52 imm=98 (fresh imm after 90; imm32 22B).
- SUB-IMM slot 50/51/52 imm=98 (complements ADD-IMM * 98; imm=0x98 → imm32 sub).
- LDB dd=50/51 ss=60 oo=B8 (fresh oo=B8 pair; imm32 26B; triad finish deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 11C`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h50_98.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_98.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_98.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_98.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_98.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_98.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_b8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_b8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-38-log.md` — this file
- `scripts/_probe/parallel-batch-38-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-044 serialize PASSes + 1 Relock**

Pass pin from body-extend-043 Relock: `113decd0cbfa7a1106ae3f17f82ba7b6a135c8ad6a3b579b7c30978ffb96d7a0`.
Handlers before consolidate = 284 (H_00..H_277). Next selectors `40 11C`.. for H_278.. if all serialize.

PASS list for body-extend-044:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_278 | 0x11C | 0x62 ADD-IMM | 0x50 0x98 | `498b87800200004881c09800000049898780020000c3` (22B) | `13b9014e066c9897` |
| H_279 | 0x11D | 0x62 ADD-IMM | 0x51 0x98 | `498b87880200004881c09800000049898788020000c3` (22B) | `eaf423344be083bb` |
| H_280 | 0x11E | 0x62 ADD-IMM | 0x52 0x98 | `498b87900200004881c09800000049898790020000c3` (22B) | `0374f755088d14c3` |
| H_281 | 0x11F | 0x61 SUB-IMM | 0x50 0x98 | `498b87800200004881e89800000049898780020000c3` (22B) | `39737d6b950d19d4` |
| H_282 | 0x120 | 0x61 SUB-IMM | 0x51 0x98 | `498b87880200004881e89800000049898788020000c3` (22B) | `7dd6789e588e0525` |
| H_283 | 0x121 | 0x61 SUB-IMM | 0x52 0x98 | `498b87900200004881e89800000049898790020000c3` (22B) | `4df6f69f74da2e8d` |
| H_284 | 0x122 | 0x80 LDB | 0x50 0x60 0xB8 | `498b87000300004881c0b8000000480fb60049898780020000c3` (26B) | `c0d9668174c58dd0` |
| H_285 | 0x123 | 0x80 LDB | 0x51 0x60 0xB8 | `498b87000300004881c0b8000000480fb60049898788020000c3` (26B) | `0e4180bb03065699` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-043 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_277.
- If the parent decides to serialize, append H_278.. at selectors `40 11C`..:
  - H_278 0x62 ADD-IMM (62 50 98) — pin `498b87800200004881c09800000049898780020000c3`
  - H_279 0x62 ADD-IMM (62 51 98) — pin `498b87880200004881c09800000049898788020000c3`
  - H_280 0x62 ADD-IMM (62 52 98) — pin `498b87900200004881c09800000049898790020000c3`
  - H_281 0x61 SUB-IMM (61 50 98) — pin `498b87800200004881e89800000049898780020000c3`
  - H_282 0x61 SUB-IMM (61 51 98) — pin `498b87880200004881e89800000049898788020000c3`
  - H_283 0x61 SUB-IMM (61 52 98) — pin `498b87900200004881e89800000049898790020000c3`
  - H_284 0x80 LDB (80 50 60 B8) — pin `498b87000300004881c0b8000000480fb60049898780020000c3`
  - H_285 0x80 LDB (80 51 60 B8) — pin `498b87000300004881c0b8000000480fb60049898788020000c3`
- Plus 1 Relock after append from pin `113decd0…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-044 serialize PASSes + 1 Relock
