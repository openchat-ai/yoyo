# parallel-batch-20 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-20-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-025 (pin `e59ddfae…`, handlers = 140, H_126..H_133 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_133 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x30 | `498b87000300004883c030480fb60049898790020000c3` (23) | same | same | Y | `b24f11cd6c12dc39` | `b24f11cd6c12dc39` | PASS |
| 2 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x38 | `498b87000300004883c038480fb60049898780020000c3` (23) | same | same | Y | `f97682dbb19b0928` | `f97682dbb19b0928` | PASS |
| 3 | 0x30 SET | slot=0x50 imm=0x0BADF00D | `48b80df0ad0b0000000049898780020000c3` (18) | same | same | Y | `5753e9efa883ecb9` | `5753e9efa883ecb9` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x51 imm=0x28 | `498b87880200004883c02849898788020000c3` (19) | same | same | Y | `87a17504336759cb` | `87a17504336759cb` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x51 imm=0x1E | `498b87880200004883e81e49898788020000c3` (19) | same | same | Y | `d28f48426b980e60` | `d28f48426b980e60` | PASS |
| 6 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x38 | `498b87000300004883c038480fb60049898788020000c3` (23) | same | same | Y | `7595918efb0d5e8e` | `7595918efb0d5e8e` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x50 imm=0x28 | `498b87800200004883c02849898780020000c3` (19) | same | same | Y | `7da4341eb02983a9` | `7da4341eb02983a9` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x52 imm=0x1E | `498b87900200004883e81e49898790020000c3` (19) | same | same | Y | `5e4e1c6e05df64c6` | `5e4e1c6e05df64c6` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x52 ss=0x60 oo=0x30 — **PASS**

- fixture: `_scratch_ldb_5260_30.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c030480fb60049898790020000c3`
- js-sha256: `b24f11cd6c12dc39e53b95fe2f4c91ed236a0b1356621974059da28144730c9f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x50 ss=0x60 oo=0x38 — **PASS**

- fixture: `_scratch_ldb_5060_38.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c038480fb60049898780020000c3`
- js-sha256: `f97682dbb19b0928bedacc0627817aec3eb17873860d92916d8b7c0d562f18e8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x30 SET slot=0x50 imm=0x0BADF00D — **PASS**

- fixture: `_scratch_set_50_0badf00d.ty` + `.code.hex`
- expected pin (18B): `48b80df0ad0b0000000049898780020000c3`
- js-sha256: `5753e9efa883ecb9ffcf006ef1e00df90f7b646be15211d10ac3fbc2a4b53b1e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x51 imm=0x28 — **PASS**

- fixture: `_scratch_addimm_h51_28.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c02849898788020000c3`
- js-sha256: `87a17504336759cb8c7789840f722dcbe7c26388d69baaeeef66d99ea3b83b6d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x51 imm=0x1E — **PASS**

- fixture: `_scratch_subimm_h51_1e.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e81e49898788020000c3`
- js-sha256: `d28f48426b980e60499ad1b9c06d9fa6214a3d88cbffb9994824fb1ca455b1d7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x51 ss=0x60 oo=0x38 — **PASS**

- fixture: `_scratch_ldb_5160_38.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c038480fb60049898788020000c3`
- js-sha256: `7595918efb0d5e8e3fa4424e32c35d87881c69c3eff6b7cb1b86e69febbff4e5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x50 imm=0x28 — **PASS**

- fixture: `_scratch_addimm_h50_28.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c02849898780020000c3`
- js-sha256: `7da4341eb02983a9e5f19adb1ef2a7bbcbd481ff6ae3b4bf5e37966738f86618`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x52 imm=0x1E — **PASS**

- fixture: `_scratch_subimm_h52_1e.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e81e49898790020000c3`
- js-sha256: `5e4e1c6e05df64c6714a71ff811564502a11d8a703f037528d754b5b9c3906b4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=52 ss=60 oo=30 (H_126=52 60 28; H_69/H_98/H_104/H_114 other oo at 52; H_131=51 60 30).
- LDB dd=50 ss=60 oo=38 (H_99=50 60 18; H_121=50 60 28; H_127=50 60 30).
- SET at slot 50 imm=0BADF00D (H_68 12345678; H_76 F00DBABE; H_94 BEEFCAFE; H_109 C0FFEE00; H_118 FACEFEED).
- ADD-IMM at slot 51 imm=28 (H_64=51 07; H_80=51 0A; H_111=51 14; H_119=51 1E).
- SUB-IMM at slot 51 imm=1E (H_70=51 03; H_112=51 0A; H_124=51 05).
- LDB dd=51 ss=60 oo=38 (H_61/H_90/H_103/H_113/H_125/H_131 other oo at 51).
- ADD-IMM at slot 50 imm=28 (H_93=50 0F; H_108=50 14; H_123=50 1E).
- SUB-IMM at slot 52 imm=1E (H_79=52 03; H_106=52 08; H_120=52 0A; H_133=52 14).
- Skipped suggested INC/DEC 50 (H_17/H_18), GET/ORV/ADDV/SUBV 50 51 (early), IMUL 50 51 (H_34).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5260_30.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_38.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_50_0badf00d.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_28.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_1e.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_38.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_28.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1e.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-20-log.md` — this file
- `scripts/_probe/parallel-batch-20-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-026 serialize PASSes + 1 Relock**

Pass pin from body-extend-025 Relock: `e59ddfae905aeea50f440cf46d763e29d869274866bc9b57cb3ab33886716fa2`.
Handlers before consolidate = 140 (H_00..H_133). Next selectors 0x8C.. for H_134.. if all serialize.

PASS list for body-extend-026:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_134 | 0x8C | 0x80 LDB | 0x52 0x60 0x30 | `498b87000300004883c030480fb60049898790020000c3` (23B) | `b24f11cd6c12dc39` |
| H_135 | 0x8D | 0x80 LDB | 0x50 0x60 0x38 | `498b87000300004883c038480fb60049898780020000c3` (23B) | `f97682dbb19b0928` |
| H_136 | 0x8E | 0x30 SET | 0x50 0x0BADF00D | `48b80df0ad0b0000000049898780020000c3` (18B) | `5753e9efa883ecb9` |
| H_137 | 0x8F | 0x62 ADD-IMM | 0x51 0x28 | `498b87880200004883c02849898788020000c3` (19B) | `87a17504336759cb` |
| H_138 | 0x90 | 0x61 SUB-IMM | 0x51 0x1E | `498b87880200004883e81e49898788020000c3` (19B) | `d28f48426b980e60` |
| H_139 | 0x91 | 0x80 LDB | 0x51 0x60 0x38 | `498b87000300004883c038480fb60049898788020000c3` (23B) | `7595918efb0d5e8e` |
| H_140 | 0x92 | 0x62 ADD-IMM | 0x50 0x28 | `498b87800200004883c02849898780020000c3` (19B) | `7da4341eb02983a9` |
| H_141 | 0x93 | 0x61 SUB-IMM | 0x52 0x1E | `498b87900200004883e81e49898790020000c3` (19B) | `5e4e1c6e05df64c6` |

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
  fresh slot/imm/dst combinations not in H_48..H_133.
- If the parent decides to serialize, append H_134.. at selectors 0x8C..:
  - H_134 0x80 LDB (80 52 60 30) — pin `498b87000300004883c030480fb60049898790020000c3`
  - H_135 0x80 LDB (80 50 60 38) — pin `498b87000300004883c038480fb60049898780020000c3`
  - H_136 0x30 SET (30 50 0BADF00D) — pin `48b80df0ad0b0000000049898780020000c3`
  - H_137 0x62 ADD-IMM (62 51 28) — pin `498b87880200004883c02849898788020000c3`
  - H_138 0x61 SUB-IMM (61 51 1E) — pin `498b87880200004883e81e49898788020000c3`
  - H_139 0x80 LDB (80 51 60 38) — pin `498b87000300004883c038480fb60049898788020000c3`
  - H_140 0x62 ADD-IMM (62 50 28) — pin `498b87800200004883c02849898780020000c3`
  - H_141 0x61 SUB-IMM (61 52 1E) — pin `498b87900200004883e81e49898790020000c3`
- Plus 1 Relock after append from pin `e59ddfae…`.

## §7. Consolidation handoff

parent next = body-extend-026 serialize PASSes + 1 Relock
