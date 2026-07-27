# parallel-batch-40 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-40-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-045 (pin `8c80a6fa…`, handlers = 300, H_286..H_293 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-045 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_293 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xC0 | `498b87000300004881c0c0000000480fb60049898788020000c3` (26) | same | same | Y | `452adbaebbd767ae` | `452adbaebbd767ae` | PASS |
| 2 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xC0 | `498b87000300004881c0c0000000480fb60049898790020000c3` (26) | same | same | Y | `766e4e7e953a3e88` | `766e4e7e953a3e88` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x50 imm=0xA8 | `498b87800200004881c0a800000049898780020000c3` (22) | same | same | Y | `6fb232e091ad8e33` | `6fb232e091ad8e33` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x51 imm=0xA8 | `498b87880200004881c0a800000049898788020000c3` (22) | same | same | Y | `0eac0a774b9d0193` | `0eac0a774b9d0193` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x52 imm=0xA8 | `498b87900200004881c0a800000049898790020000c3` (22) | same | same | Y | `1acbcee68dee9520` | `1acbcee68dee9520` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x50 imm=0xA8 | `498b87800200004881e8a800000049898780020000c3` (22) | same | same | Y | `f1d0cdaaa848cd64` | `f1d0cdaaa848cd64` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0xA8 | `498b87880200004881e8a800000049898788020000c3` (22) | same | same | Y | `446a3deafbac2416` | `446a3deafbac2416` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x52 imm=0xA8 | `498b87900200004881e8a800000049898790020000c3` (22) | same | same | Y | `254705f23c21fb17` | `254705f23c21fb17` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x51 ss=0x60 oo=0xC0 — **PASS**

- fixture: `_scratch_ldb_5160_c0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c0000000480fb60049898788020000c3`
- js-sha256: `452adbaebbd767aeb96d51b4ee2f91aab3a6da4d566bdecd875b2dae8e88624f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x52 ss=0x60 oo=0xC0 — **PASS**

- fixture: `_scratch_ldb_5260_c0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c0000000480fb60049898790020000c3`
- js-sha256: `766e4e7e953a3e88b74d40e47edb12de908d14326001e1f95ec1eb8ca413174f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x50 imm=0xA8 — **PASS**

- fixture: `_scratch_addimm_h50_a8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0a800000049898780020000c3`
- js-sha256: `6fb232e091ad8e3345bf7f59cc06614258634daa24a17fb196b6c5bd3187d1e5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x51 imm=0xA8 — **PASS**

- fixture: `_scratch_addimm_h51_a8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0a800000049898788020000c3`
- js-sha256: `0eac0a774b9d01933218fb2e52b3493e302865857756c8d49ec0005d2728bb65`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x52 imm=0xA8 — **PASS**

- fixture: `_scratch_addimm_h52_a8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0a800000049898790020000c3`
- js-sha256: `1acbcee68dee9520a277ffc40faddcee9de1599e4ff9232c115a38c8ac45b06f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x50 imm=0xA8 — **PASS**

- fixture: `_scratch_subimm_h50_a8.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e8a800000049898780020000c3`
- js-sha256: `f1d0cdaaa848cd641cecf15959fa70b1421ea7b141fee07751570ab1c4604152`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0xA8 — **PASS**

- fixture: `_scratch_subimm_h51_a8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8a800000049898788020000c3`
- js-sha256: `446a3deafbac2416e875e75fe8792542ee68cb12d88eb6a428474692e6ef20bd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x52 imm=0xA8 — **PASS**

- fixture: `_scratch_subimm_h52_a8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8a800000049898790020000c3`
- js-sha256: `254705f23c21fb17cc3b4a40523f0888fb075124c8c861d7f9615d21d6e78e23`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=51/52 ss=60 oo=C0 (finish C0 triad after H_293=50; imm32 26B).
- ADD-IMM slot=50/51/52 imm=A8 (fresh imm after A0; imm32 22B).
- SUB-IMM slot 50/51/52 imm=A8 (complements ADD-IMM * A8; imm=0xA8 → imm32 sub).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 12C`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5160_c0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_c0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_a8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_a8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_a8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_a8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_a8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_a8.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-40-log.md` — this file
- `scripts/_probe/parallel-batch-40-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-046 serialize PASSes + 1 Relock**

Pass pin from body-extend-045 Relock: `8c80a6fa783440b2ef724beb1860f295c81cde46c53f35d0cdcc40ff8798519c`.
Handlers before consolidate = 300 (H_00..H_293). Next selectors `40 12C`.. for H_294.. if all serialize.

PASS list for body-extend-046:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_294 | 0x12C | 0x80 LDB | 0x51 0x60 0xC0 | `498b87000300004881c0c0000000480fb60049898788020000c3` (26B) | `452adbaebbd767ae` |
| H_295 | 0x12D | 0x80 LDB | 0x52 0x60 0xC0 | `498b87000300004881c0c0000000480fb60049898790020000c3` (26B) | `766e4e7e953a3e88` |
| H_296 | 0x12E | 0x62 ADD-IMM | 0x50 0xA8 | `498b87800200004881c0a800000049898780020000c3` (22B) | `6fb232e091ad8e33` |
| H_297 | 0x12F | 0x62 ADD-IMM | 0x51 0xA8 | `498b87880200004881c0a800000049898788020000c3` (22B) | `0eac0a774b9d0193` |
| H_298 | 0x130 | 0x62 ADD-IMM | 0x52 0xA8 | `498b87900200004881c0a800000049898790020000c3` (22B) | `1acbcee68dee9520` |
| H_299 | 0x131 | 0x61 SUB-IMM | 0x50 0xA8 | `498b87800200004881e8a800000049898780020000c3` (22B) | `f1d0cdaaa848cd64` |
| H_300 | 0x132 | 0x61 SUB-IMM | 0x51 0xA8 | `498b87880200004881e8a800000049898788020000c3` (22B) | `446a3deafbac2416` |
| H_301 | 0x133 | 0x61 SUB-IMM | 0x52 0xA8 | `498b87900200004881e8a800000049898790020000c3` (22B) | `254705f23c21fb17` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-045 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_293.
- If the parent decides to serialize, append H_294.. at selectors `40 12C`..:
  - H_294 0x80 LDB (80 51 60 C0) — pin `498b87000300004881c0c0000000480fb60049898788020000c3`
  - H_295 0x80 LDB (80 52 60 C0) — pin `498b87000300004881c0c0000000480fb60049898790020000c3`
  - H_296 0x62 ADD-IMM (62 50 A8) — pin `498b87800200004881c0a800000049898780020000c3`
  - H_297 0x62 ADD-IMM (62 51 A8) — pin `498b87880200004881c0a800000049898788020000c3`
  - H_298 0x62 ADD-IMM (62 52 A8) — pin `498b87900200004881c0a800000049898790020000c3`
  - H_299 0x61 SUB-IMM (61 50 A8) — pin `498b87800200004881e8a800000049898780020000c3`
  - H_300 0x61 SUB-IMM (61 51 A8) — pin `498b87880200004881e8a800000049898788020000c3`
  - H_301 0x61 SUB-IMM (61 52 A8) — pin `498b87900200004881e8a800000049898790020000c3`
- Plus 1 Relock after append from pin `8c80a6fa…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-046 serialize PASSes + 1 Relock
