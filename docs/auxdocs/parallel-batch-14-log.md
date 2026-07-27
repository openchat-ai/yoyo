# parallel-batch-14 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-14-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-019 (pin `ea348e8b…`, handlers = 92, H_78..H_85 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_85 and
> not already present as handlers in current `yoyo.ty`. Slot/imm/dst
> variations of SET/GET/CMP/LDB/IMUL/ORV/ADD-IMM. Skipped D-1
> 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x30 SET | slot=0x52 imm=0xFEEDFACE | `48b8cefaedfe0000000049898790020000c3` (18) | same | same | Y | `e66d020e76069da7` | `e66d020e76069da7` | PASS |
| 2 | 0x30 SET | slot=0x51 imm=0xAABBCCDD | `48b8ddccbbaa0000000049898788020000c3` (18) | same | same | Y | `2a98933dfb0d8cdd` | `2a98933dfb0d8cdd` | PASS |
| 3 | 0x60 GET | (0x50, 0x52) | `498b879002000049898780020000c3` (15) | same | same | Y | `ce17131dfed4ee14` | `ce17131dfed4ee14` | PASS |
| 4 | 0x65 CMP | (0x50, 0x52) | `498b8780020000498b8f900200004839c8c3` (18) | same | same | Y | `594c4a8e7b724cf5` | `594c4a8e7b724cf5` | PASS |
| 5 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x10 | `498b87000300004883c010480fb60049898788020000c3` (23) | same | same | Y | `d3253d0131cd96d0` | `d3253d0131cd96d0` | PASS |
| 6 | 0x63 IMUL | (0x52, 0x50) | `498b8790020000498b8f80020000480fafc149898790020000c3` (26) | same | same | Y | `ba2a57ad864330da` | `ba2a57ad864330da` | PASS |
| 7 | 0x69 ORV | (0x51, 0x52) | `498b8788020000498b8f900200004809c849898788020000c3` (25) | same | same | Y | `df8b41f4c74b2540` | `df8b41f4c74b2540` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x50 imm=0x0F | `498b87800200004883c00f49898780020000c3` (19) | same | same | Y | `899a90c682241183` | `899a90c682241183` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x30 SET slot=0x52 imm=0xFEEDFACE — **PASS**

- fixture: `_scratch_set_feedface.ty` + `.code.hex`
- expected pin (18B): `48b8cefaedfe0000000049898790020000c3`
- js-sha256: `e66d020e76069da7f2aeb30d6618f61321d1c7b396f15ed6d03a1858326e589e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x30 SET slot=0x51 imm=0xAABBCCDD — **PASS**

- fixture: `_scratch_set_aabbccdd.ty` + `.code.hex`
- expected pin (18B): `48b8ddccbbaa0000000049898788020000c3`
- js-sha256: `2a98933dfb0d8cdda9161df415cb8a9a9635ff1387085d7168024e8869a6688f`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x60 GET (0x50, 0x52) — **PASS**

- fixture: `_scratch_get_5052.ty` + `.code.hex`
- expected pin (15B): `498b879002000049898780020000c3`
- js-sha256: `ce17131dfed4ee14af0697f07f0d04f1a0b667aff33bef843c12e28b08399120`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x65 CMP (0x50, 0x52) — **PASS**

- fixture: `_scratch_cmp_5052.ty` + `.code.hex`
- expected pin (18B): `498b8780020000498b8f900200004839c8c3`
- js-sha256: `594c4a8e7b724cf54fbf766598061808a0fbd5b4965f2b1d858d7c2f2fb68ab0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x51 ss=0x60 oo=0x10 — **PASS**

- fixture: `_scratch_ldb_5160_10.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c010480fb60049898788020000c3`
- js-sha256: `d3253d0131cd96d0f544e7149d58883b77ce84fe3deea780b141e7adabf943af`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x63 IMUL (0x52, 0x50) — **PASS**

- fixture: `_scratch_imul_5250.ty` + `.code.hex`
- expected pin (26B): `498b8790020000498b8f80020000480fafc149898790020000c3`
- js-sha256: `ba2a57ad864330daa4c0158da3ecfb56887a5f08df92b7e86c397ab9d1669f92`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x69 ORV (0x51, 0x52) — **PASS**

- fixture: `_scratch_orv_5152.ty` + `.code.hex`
- expected pin (25B): `498b8788020000498b8f900200004809c849898788020000c3`
- js-sha256: `df8b41f4c74b2540fd7aa2f3437d35001f0d4753344f5157bf836cd26c24864b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x50 imm=0x0F — **PASS**

- fixture: `_scratch_addimm_h50_0f.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c00f49898780020000c3`
- js-sha256: `899a90c68224118374482202720a205f23b7e1cb0dd41f92127a4303f7c7a4ca`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SET at slot 0x52 imm=FEEDFACE (H_53 uses CAFEBABE; H_68/H_76 are slot 50).
- SET at slot 0x51 imm=AABBCCDD (H_60 uses DEADBEEF at 51).
- GET at 50←52 (H_51=51 52; H_59=52 50; H_67=51 50; H_75=52 51).
- CMP at 50 52 (H_58=51 50; H_65=52 51; H_77=52 50).
- LDB dd=51 ss=60 oo=10 (H_61=51 60 08; H_69=52 60 08).
- IMUL at 52 50 (H_56=51 50; H_57=52 51; H_85=50 52).
- ORV at 51 52 (H_49=51 50; H_54=52 51; H_73=50 52; H_82=52 50).
- ADD-IMM at slot 50 imm=0F (H_13/H_64/H_78/H_80 use other slot/imm).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_set_feedface.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_aabbccdd.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_get_5052.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_cmp_5052.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_10.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_imul_5250.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_orv_5152.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_0f.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-14-log.md` — this file
- `scripts/_probe/parallel-batch-14-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-020 serialize PASSes + 1 Relock**

Pass pin from body-extend-019 Relock: `ea348e8b7a43f285121c1755b572a87940a50432ef5d0482be6ecc3c575a98bd`.
Handlers before consolidate = 92 (H_00..H_85). Next selectors 0x5C.. for H_86.. if all serialize.

PASS list for body-extend-020:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_86 | 0x5C | 0x30 SET | 0x52 0xFEEDFACE | `48b8cefaedfe0000000049898790020000c3` (18B) | `e66d020e76069da7` |
| H_87 | 0x5D | 0x30 SET | 0x51 0xAABBCCDD | `48b8ddccbbaa0000000049898788020000c3` (18B) | `2a98933dfb0d8cdd` |
| H_88 | 0x5E | 0x60 GET | 0x50 0x52 | `498b879002000049898780020000c3` (15B) | `ce17131dfed4ee14` |
| H_89 | 0x5F | 0x65 CMP | 0x50 0x52 | `498b8780020000498b8f900200004839c8c3` (18B) | `594c4a8e7b724cf5` |
| H_90 | 0x60 | 0x80 LDB | 0x51 0x60 0x10 | `498b87000300004883c010480fb60049898788020000c3` (23B) | `d3253d0131cd96d0` |
| H_91 | 0x61 | 0x63 IMUL | 0x52 0x50 | `498b8790020000498b8f80020000480fafc149898790020000c3` (26B) | `ba2a57ad864330da` |
| H_92 | 0x62 | 0x69 ORV | 0x51 0x52 | `498b8788020000498b8f900200004809c849898788020000c3` (25B) | `df8b41f4c74b2540` |
| H_93 | 0x63 | 0x62 ADD-IMM | 0x50 0x0F | `498b87800200004883c00f49898780020000c3` (19B) | `899a90c682241183` |

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
  fresh slot/imm/dst combinations not in H_48..H_85.
- If the parent decides to serialize, append H_86.. at selectors 0x5C..:
  - H_86 0x30 SET (30 52 FEEDFACE) — pin `48b8cefaedfe0000000049898790020000c3`
  - H_87 0x30 SET (30 51 AABBCCDD) — pin `48b8ddccbbaa0000000049898788020000c3`
  - H_88 0x60 GET (60 50 52) — pin `498b879002000049898780020000c3`
  - H_89 0x65 CMP (65 50 52) — pin `498b8780020000498b8f900200004839c8c3`
  - H_90 0x80 LDB (80 51 60 10) — pin `498b87000300004883c010480fb60049898788020000c3`
  - H_91 0x63 IMUL (63 52 50) — pin `498b8790020000498b8f80020000480fafc149898790020000c3`
  - H_92 0x69 ORV (69 51 52) — pin `498b8788020000498b8f900200004809c849898788020000c3`
  - H_93 0x62 ADD-IMM (62 50 0F) — pin `498b87800200004883c00f49898780020000c3`
- Plus 1 Relock after append from pin `ea348e8b…`.

## §7. Consolidation handoff

parent next = body-extend-020 serialize PASSes + 1 Relock
