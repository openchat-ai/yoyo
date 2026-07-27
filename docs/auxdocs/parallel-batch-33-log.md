# parallel-batch-33 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-33-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-038 (pin `aa95228f…`, handlers = 244, H_230..H_237 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-038 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_237 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x90 | `498b87000300004881c090000000480fb60049898790020000c3` (26) | same | same | Y | `515d9290ccd5b51f` | `515d9290ccd5b51f` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x50 imm=0x70 | `498b87800200004883e87049898780020000c3` (19) | same | same | Y | `864bf0ef8581dfff` | `864bf0ef8581dfff` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x51 imm=0x70 | `498b87880200004883e87049898788020000c3` (19) | same | same | Y | `29334b7d85f1f4df` | `29334b7d85f1f4df` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x52 imm=0x70 | `498b87900200004883e87049898790020000c3` (19) | same | same | Y | `ab68fcd1813d0252` | `ab68fcd1813d0252` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x50 imm=0x78 | `498b87800200004883c07849898780020000c3` (19) | same | same | Y | `abb251d39c0c52c4` | `abb251d39c0c52c4` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x51 imm=0x78 | `498b87880200004883c07849898788020000c3` (19) | same | same | Y | `b981458127112570` | `b981458127112570` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x52 imm=0x78 | `498b87900200004883c07849898790020000c3` (19) | same | same | Y | `dfdb811b3af776d0` | `dfdb811b3af776d0` | PASS |
| 8 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x98 | `498b87000300004881c098000000480fb60049898780020000c3` (26) | same | same | Y | `20ef671052bbdb81` | `20ef671052bbdb81` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x52 ss=0x60 oo=0x90 — **PASS**

- fixture: `_scratch_ldb_5260_90.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c090000000480fb60049898790020000c3`
- js-sha256: `515d9290ccd5b51faf05da65936bf6be35120ef7b2d019d002e6814b5ee9a861`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x50 imm=0x70 — **PASS**

- fixture: `_scratch_subimm_h50_70.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e87049898780020000c3`
- js-sha256: `864bf0ef8581dfff3a39e838bdd20e7d3b7568060ef9de1e826a8b64e8dc2cf3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x51 imm=0x70 — **PASS**

- fixture: `_scratch_subimm_h51_70.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e87049898788020000c3`
- js-sha256: `29334b7d85f1f4df1380dc899730d972a6fc88c1012335f2aa6030130747397d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x52 imm=0x70 — **PASS**

- fixture: `_scratch_subimm_h52_70.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e87049898790020000c3`
- js-sha256: `ab68fcd1813d0252dbf73dfbe57376d310f5534bcedec86ed8101216de85ddbb`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x50 imm=0x78 — **PASS**

- fixture: `_scratch_addimm_h50_78.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c07849898780020000c3`
- js-sha256: `abb251d39c0c52c4c8dd2dbe49864c6b66ec987716af864b7a5047013bf3d68e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x51 imm=0x78 — **PASS**

- fixture: `_scratch_addimm_h51_78.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c07849898788020000c3`
- js-sha256: `b9814581271125709b7ac1e5908dfca420301c4cdf2e99a5cb5e91bd1dc763cc`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x52 imm=0x78 — **PASS**

- fixture: `_scratch_addimm_h52_78.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c07849898790020000c3`
- js-sha256: `dfdb811b3af776d091dc51f9bbfcf093e207f5dc0aec404df9a98096300a7f30`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x50 ss=0x60 oo=0x98 — **PASS**

- fixture: `_scratch_ldb_5060_98.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c098000000480fb60049898780020000c3`
- js-sha256: `20ef671052bbdb815e3dcdba6aa72d4f01dade01043449c151f3baf16c8376bf`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=52 ss=60 oo=90 (complete oo=90 triad; imm32 26B).
- SUB-IMM slot 50/51/52 imm=70 (fresh SUB imm=70 triad; complements locked ADD-IMM * 70).
- ADD-IMM slot 50/51/52 imm=78 (fresh ADD imm=78 triad).
- LDB dd=50 ss=60 oo=98 (start oo=98 rung; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5260_90.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_70.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_70.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_70.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_78.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_78.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_78.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_98.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-33-log.md` — this file
- `scripts/_probe/parallel-batch-33-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-039 serialize PASSes + 1 Relock**

Pass pin from body-extend-038 Relock: `aa95228f49b6131c88315b4af43e02b76e8c67070322eab4c200944e839a99fa`.
Handlers before consolidate = 244 (H_00..H_237). Next selectors 0xF4.. for H_238.. if all serialize.

PASS list for body-extend-039:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_238 | 0xF4 | 0x80 LDB | 0x52 0x60 0x90 | `498b87000300004881c090000000480fb60049898790020000c3` (26B) | `515d9290ccd5b51f` |
| H_239 | 0xF5 | 0x61 SUB-IMM | 0x50 0x70 | `498b87800200004883e87049898780020000c3` (19B) | `864bf0ef8581dfff` |
| H_240 | 0xF6 | 0x61 SUB-IMM | 0x51 0x70 | `498b87880200004883e87049898788020000c3` (19B) | `29334b7d85f1f4df` |
| H_241 | 0xF7 | 0x61 SUB-IMM | 0x52 0x70 | `498b87900200004883e87049898790020000c3` (19B) | `ab68fcd1813d0252` |
| H_242 | 0xF8 | 0x62 ADD-IMM | 0x50 0x78 | `498b87800200004883c07849898780020000c3` (19B) | `abb251d39c0c52c4` |
| H_243 | 0xF9 | 0x62 ADD-IMM | 0x51 0x78 | `498b87880200004883c07849898788020000c3` (19B) | `b981458127112570` |
| H_244 | 0xFA | 0x62 ADD-IMM | 0x52 0x78 | `498b87900200004883c07849898790020000c3` (19B) | `dfdb811b3af776d0` |
| H_245 | 0xFB | 0x80 LDB | 0x50 0x60 0x98 | `498b87000300004881c098000000480fb60049898780020000c3` (26B) | `20ef671052bbdb81` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-038 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_237.
- If the parent decides to serialize, append H_238.. at selectors 0xF4..:
  - H_238 0x80 LDB (80 52 60 90) — pin `498b87000300004881c090000000480fb60049898790020000c3`
  - H_239 0x61 SUB-IMM (61 50 70) — pin `498b87800200004883e87049898780020000c3`
  - H_240 0x61 SUB-IMM (61 51 70) — pin `498b87880200004883e87049898788020000c3`
  - H_241 0x61 SUB-IMM (61 52 70) — pin `498b87900200004883e87049898790020000c3`
  - H_242 0x62 ADD-IMM (62 50 78) — pin `498b87800200004883c07849898780020000c3`
  - H_243 0x62 ADD-IMM (62 51 78) — pin `498b87880200004883c07849898788020000c3`
  - H_244 0x62 ADD-IMM (62 52 78) — pin `498b87900200004883c07849898790020000c3`
  - H_245 0x80 LDB (80 50 60 98) — pin `498b87000300004881c098000000480fb60049898780020000c3`
- Plus 1 Relock after append from pin `aa95228f…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-039 serialize PASSes + 1 Relock
