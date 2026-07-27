# parallel-batch-37 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-37-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-042 (pin `afceb388…`, handlers = 276, H_262..H_269 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-042 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_269 and
> not already present as handlers in current `yoyo.ty` (except H_269 ADD 50 90 already locked; picks complete 90 triad + SUB 90 + LDB B0).
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x51 imm=0x90 | `498b87880200004881c09000000049898788020000c3` (22) | same | same | Y | `30d80ac5f98d5b91` | `30d80ac5f98d5b91` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x52 imm=0x90 | `498b87900200004881c09000000049898790020000c3` (22) | same | same | Y | `1f4ed4e242ed21b3` | `1f4ed4e242ed21b3` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x90 | `498b87800200004881e89000000049898780020000c3` (22) | same | same | Y | `5108f62107ced6f5` | `5108f62107ced6f5` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x51 imm=0x90 | `498b87880200004881e89000000049898788020000c3` (22) | same | same | Y | `07c48bf0e15bc2fd` | `07c48bf0e15bc2fd` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x52 imm=0x90 | `498b87900200004881e89000000049898790020000c3` (22) | same | same | Y | `ce43fa09ae8fd687` | `ce43fa09ae8fd687` | PASS |
| 6 | 0x80 LDB | dd=0x50 ss=0x60 oo=0xB0 | `498b87000300004881c0b0000000480fb60049898780020000c3` (26) | same | same | Y | `64f22f32cf0fab77` | `64f22f32cf0fab77` | PASS |
| 7 | 0x80 LDB | dd=0x51 ss=0x60 oo=0xB0 | `498b87000300004881c0b0000000480fb60049898788020000c3` (26) | same | same | Y | `8de79951c51e9c4a` | `8de79951c51e9c4a` | PASS |
| 8 | 0x80 LDB | dd=0x52 ss=0x60 oo=0xB0 | `498b87000300004881c0b0000000480fb60049898790020000c3` (26) | same | same | Y | `24662dc0540eff95` | `24662dc0540eff95` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x51 imm=0x90 — **PASS**

- fixture: `_scratch_addimm_h51_90.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c09000000049898788020000c3`
- js-sha256: `30d80ac5f98d5b91b3cdc3f176a37ee810973825a55af337dcae8a95254b1023`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x52 imm=0x90 — **PASS**

- fixture: `_scratch_addimm_h52_90.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c09000000049898790020000c3`
- js-sha256: `1f4ed4e242ed21b3ca4c1cb13bf3cb58db65ef6da56e6e50fd47e52edbcc4953`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x90 — **PASS**

- fixture: `_scratch_subimm_h50_90.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e89000000049898780020000c3`
- js-sha256: `5108f62107ced6f51fcd11dbc43815601e1f26890847c3cfc43a79f180eeb873`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x51 imm=0x90 — **PASS**

- fixture: `_scratch_subimm_h51_90.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e89000000049898788020000c3`
- js-sha256: `07c48bf0e15bc2fd1bb1150b111268d555cca8eec7ed5c71c7bcfcb8df2c5309`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x52 imm=0x90 — **PASS**

- fixture: `_scratch_subimm_h52_90.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e89000000049898790020000c3`
- js-sha256: `ce43fa09ae8fd687347e34b8f70dd0aeeaf4e3d7be6046e6dc282624d7daec66`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x50 ss=0x60 oo=0xB0 — **PASS**

- fixture: `_scratch_ldb_5060_b0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b0000000480fb60049898780020000c3`
- js-sha256: `64f22f32cf0fab77e475d17ef59518340cf5bdf1b03f84eda58bc971fe8ec9d8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x51 ss=0x60 oo=0xB0 — **PASS**

- fixture: `_scratch_ldb_5160_b0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b0000000480fb60049898788020000c3`
- js-sha256: `8de79951c51e9c4af62035f837dbbc6f6f640d0b29ed526b4597d16a4fb58cd8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x52 ss=0x60 oo=0xB0 — **PASS**

- fixture: `_scratch_ldb_5260_b0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0b0000000480fb60049898790020000c3`
- js-sha256: `24662dc0540eff95f240064e796f6ea1a35fbd9814241e9f76b3f7c7780a1384`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=51/52 imm=90 (complete ADD 90 triad with H_269; imm32 22B).
- SUB-IMM slot 50/51/52 imm=90 (complements ADD-IMM * 90; imm=0x90 → imm32 sub).
- LDB dd=50/51/52 ss=60 oo=B0 (fresh oo=B0 triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 114`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h51_90.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_90.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_90.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_90.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_90.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_b0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_b0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_b0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-37-log.md` — this file
- `scripts/_probe/parallel-batch-37-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-043 serialize PASSes + 1 Relock**

Pass pin from body-extend-042 Relock: `afceb388015dd4a7e7a2de16a109eb8649189bb28471d021bb4b82eeaa9d1311`.
Handlers before consolidate = 276 (H_00..H_269). Next selectors `40 114`.. for H_270.. if all serialize.

PASS list for body-extend-043:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_270 | 0x114 | 0x62 ADD-IMM | 0x51 0x90 | `498b87880200004881c09000000049898788020000c3` (22B) | `30d80ac5f98d5b91` |
| H_271 | 0x115 | 0x62 ADD-IMM | 0x52 0x90 | `498b87900200004881c09000000049898790020000c3` (22B) | `1f4ed4e242ed21b3` |
| H_272 | 0x116 | 0x61 SUB-IMM | 0x50 0x90 | `498b87800200004881e89000000049898780020000c3` (22B) | `5108f62107ced6f5` |
| H_273 | 0x117 | 0x61 SUB-IMM | 0x51 0x90 | `498b87880200004881e89000000049898788020000c3` (22B) | `07c48bf0e15bc2fd` |
| H_274 | 0x118 | 0x61 SUB-IMM | 0x52 0x90 | `498b87900200004881e89000000049898790020000c3` (22B) | `ce43fa09ae8fd687` |
| H_275 | 0x119 | 0x80 LDB | 0x50 0x60 0xB0 | `498b87000300004881c0b0000000480fb60049898780020000c3` (26B) | `64f22f32cf0fab77` |
| H_276 | 0x11A | 0x80 LDB | 0x51 0x60 0xB0 | `498b87000300004881c0b0000000480fb60049898788020000c3` (26B) | `8de79951c51e9c4a` |
| H_277 | 0x11B | 0x80 LDB | 0x52 0x60 0xB0 | `498b87000300004881c0b0000000480fb60049898790020000c3` (26B) | `24662dc0540eff95` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-042 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_269.
- If the parent decides to serialize, append H_270.. at selectors `40 114`..:
  - H_270 0x62 ADD-IMM (62 51 90) — pin `498b87880200004881c09000000049898788020000c3`
  - H_271 0x62 ADD-IMM (62 52 90) — pin `498b87900200004881c09000000049898790020000c3`
  - H_272 0x61 SUB-IMM (61 50 90) — pin `498b87800200004881e89000000049898780020000c3`
  - H_273 0x61 SUB-IMM (61 51 90) — pin `498b87880200004881e89000000049898788020000c3`
  - H_274 0x61 SUB-IMM (61 52 90) — pin `498b87900200004881e89000000049898790020000c3`
  - H_275 0x80 LDB (80 50 60 B0) — pin `498b87000300004881c0b0000000480fb60049898780020000c3`
  - H_276 0x80 LDB (80 51 60 B0) — pin `498b87000300004881c0b0000000480fb60049898788020000c3`
  - H_277 0x80 LDB (80 52 60 B0) — pin `498b87000300004881c0b0000000480fb60049898790020000c3`
- Plus 1 Relock after append from pin `afceb388…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-043 serialize PASSes + 1 Relock
