# parallel-batch-28 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-28-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-033 (pin `0f0fce9a…`, handlers = 204, H_190..H_197 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-033 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_197 and
> not already present as handlers in current `yoyo.ty` (skipped H_47 LDB 50 60 70).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x52 imm=0x50 | `498b87900200004883c05049898790020000c3` (19) | same | same | Y | `684324dfa8a4c08b` | `684324dfa8a4c08b` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x50 imm=0x48 | `498b87800200004883e84849898780020000c3` (19) | same | same | Y | `5f68485aac429a89` | `5f68485aac429a89` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x52 imm=0x48 | `498b87900200004883e84849898790020000c3` (19) | same | same | Y | `d3786a374b0a48db` | `d3786a374b0a48db` | PASS |
| 4 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x78 | `498b87000300004883c078480fb60049898780020000c3` (23) | same | same | Y | `431d73b2dfe3fbd1` | `431d73b2dfe3fbd1` | PASS |
| 5 | 0x30 SET | slot=0x51 imm=0xC0DEC0DE | `48b8dec0dec00000000049898788020000c3` (18) | same | same | Y | `8b80a408a82bd068` | `8b80a408a82bd068` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x50 imm=0x58 | `498b87800200004883c05849898780020000c3` (19) | same | same | Y | `84fd334ba8eecae0` | `84fd334ba8eecae0` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0x50 | `498b87880200004883e85049898788020000c3` (19) | same | same | Y | `3eba365fe5dedefd` | `3eba365fe5dedefd` | PASS |
| 8 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x78 | `498b87000300004883c078480fb60049898788020000c3` (23) | same | same | Y | `ed2e4285f92ea9f6` | `ed2e4285f92ea9f6` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x52 imm=0x50 — **PASS**

- fixture: `_scratch_addimm_h52_50.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c05049898790020000c3`
- js-sha256: `684324dfa8a4c08be9943b0b80b73507cf9116256b66132d1beca41fc12dd7ee`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x50 imm=0x48 — **PASS**

- fixture: `_scratch_subimm_h50_48.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e84849898780020000c3`
- js-sha256: `5f68485aac429a893e71839cb7376422ddb8bdb740a2f5c4d9d2f75844ec2ec9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x52 imm=0x48 — **PASS**

- fixture: `_scratch_subimm_h52_48.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e84849898790020000c3`
- js-sha256: `d3786a374b0a48dbd7385ab5965bdcc6e5b74bdc42823b40cc3cfef30e25b36e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x50 ss=0x60 oo=0x78 — **PASS**

- fixture: `_scratch_ldb_5060_78.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c078480fb60049898780020000c3`
- js-sha256: `431d73b2dfe3fbd18b4f5aefb72090288ff85a6ad0a182e86419c70af2ecd2ec`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x30 SET slot=0x51 imm=0xC0DEC0DE — **PASS**

- fixture: `_scratch_set_51_c0dec0de.ty` + `.code.hex`
- expected pin (18B): `48b8dec0dec00000000049898788020000c3`
- js-sha256: `8b80a408a82bd06813705de9302ef1b7467a026b445611a93a18de7fc8d6a488`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x50 imm=0x58 — **PASS**

- fixture: `_scratch_addimm_h50_58.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c05849898780020000c3`
- js-sha256: `84fd334ba8eecae0c74fed633f37dafdb241c87091eef0403fb1df04501c6060`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0x50 — **PASS**

- fixture: `_scratch_subimm_h51_50.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e85049898788020000c3`
- js-sha256: `3eba365fe5dedefd45b965fd031f7d08f94f28834b389c93061a42487534e466`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x51 ss=0x60 oo=0x78 — **PASS**

- fixture: `_scratch_ldb_5160_78.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c078480fb60049898788020000c3`
- js-sha256: `ed2e4285f92ea9f658d7123d18212b7bcabc0235a2a473295930a90860fb0b04`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot 52 imm=50 (complete imm=50 ADD triad with H_195/H_197).
- SUB-IMM slot 50/52 imm=48 (fresh SUB imm=48; H_196 covered slot 51).
- LDB dd=50/51 ss=60 oo=78 (next rung above oo=70; skipped H_47 LDB 50 60 70).
- SET slot 51 imm=C0DEC0DE (H_194 locked slot 50 only).
- ADD-IMM slot 50 imm=58 (fresh imm rung above 50).
- SUB-IMM slot 51 imm=50 (fresh SUB imm=50).
- Skipped LDB 50 60 70 (H_47 early). No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h52_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_48.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_48.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_78.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_51_c0dec0de.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_58.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_78.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-28-log.md` — this file
- `scripts/_probe/parallel-batch-28-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-034 serialize PASSes + 1 Relock**

Pass pin from body-extend-033 Relock: `0f0fce9a754e262914c8e2a78ca2558bd8af31ab0d532339f49018c2354cdac2`.
Handlers before consolidate = 204 (H_00..H_197). Next selectors 0xCC.. for H_198.. if all serialize.

PASS list for body-extend-034:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_198 | 0xCC | 0x62 ADD-IMM | 0x52 0x50 | `498b87900200004883c05049898790020000c3` (19B) | `684324dfa8a4c08b` |
| H_199 | 0xCD | 0x61 SUB-IMM | 0x50 0x48 | `498b87800200004883e84849898780020000c3` (19B) | `5f68485aac429a89` |
| H_200 | 0xCE | 0x61 SUB-IMM | 0x52 0x48 | `498b87900200004883e84849898790020000c3` (19B) | `d3786a374b0a48db` |
| H_201 | 0xCF | 0x80 LDB | 0x50 0x60 0x78 | `498b87000300004883c078480fb60049898780020000c3` (23B) | `431d73b2dfe3fbd1` |
| H_202 | 0xD0 | 0x30 SET | 0x51 0xC0DEC0DE | `48b8dec0dec00000000049898788020000c3` (18B) | `8b80a408a82bd068` |
| H_203 | 0xD1 | 0x62 ADD-IMM | 0x50 0x58 | `498b87800200004883c05849898780020000c3` (19B) | `84fd334ba8eecae0` |
| H_204 | 0xD2 | 0x61 SUB-IMM | 0x51 0x50 | `498b87880200004883e85049898788020000c3` (19B) | `3eba365fe5dedefd` |
| H_205 | 0xD3 | 0x80 LDB | 0x51 0x60 0x78 | `498b87000300004883c078480fb60049898788020000c3` (23B) | `ed2e4285f92ea9f6` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-033 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_197.
- If the parent decides to serialize, append H_198.. at selectors 0xCC..:
  - H_198 0x62 ADD-IMM (62 52 50) — pin `498b87900200004883c05049898790020000c3`
  - H_199 0x61 SUB-IMM (61 50 48) — pin `498b87800200004883e84849898780020000c3`
  - H_200 0x61 SUB-IMM (61 52 48) — pin `498b87900200004883e84849898790020000c3`
  - H_201 0x80 LDB (80 50 60 78) — pin `498b87000300004883c078480fb60049898780020000c3`
  - H_202 0x30 SET (30 51 C0DEC0DE) — pin `48b8dec0dec00000000049898788020000c3`
  - H_203 0x62 ADD-IMM (62 50 58) — pin `498b87800200004883c05849898780020000c3`
  - H_204 0x61 SUB-IMM (61 51 50) — pin `498b87880200004883e85049898788020000c3`
  - H_205 0x80 LDB (80 51 60 78) — pin `498b87000300004883c078480fb60049898788020000c3`
- Plus 1 Relock after append from pin `0f0fce9a…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-034 serialize PASSes + 1 Relock
