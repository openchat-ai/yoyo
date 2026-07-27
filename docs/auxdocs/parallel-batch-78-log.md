# parallel-batch-78 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-78-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-083 (pin `45dff031…`, handlers = 603, H_589..H_596 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-083 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_596 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x61 SUB-IMM | slot=0x51 imm=0x1B8 | `498b87880200004881e8b801000049898788020000c3` (22) | same | same | Y | `e5f22d8e3828fbe4` | `e5f22d8e3828fbe4` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x52 imm=0x1B8 | `498b87900200004881e8b801000049898790020000c3` (22) | same | same | Y | `fb3630917fc37295` | `fb3630917fc37295` | PASS |
| 3 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1C0 | `498b87000300004881c0c0010000480fb60049898780020000c3` (26) | same | same | Y | `8953358138eb317e` | `8953358138eb317e` | PASS |
| 4 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1C0 | `498b87000300004881c0c0010000480fb60049898788020000c3` (26) | same | same | Y | `1759a1345d7af7ee` | `1759a1345d7af7ee` | PASS |
| 5 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x1C0 | `498b87000300004881c0c0010000480fb60049898790020000c3` (26) | same | same | Y | `bc6894d42acc6084` | `bc6894d42acc6084` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x50 imm=0x1C0 | `498b87800200004881c0c001000049898780020000c3` (22) | same | same | Y | `f6926af2f6dc5e89` | `f6926af2f6dc5e89` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x51 imm=0x1C0 | `498b87880200004881c0c001000049898788020000c3` (22) | same | same | Y | `8b90b51a7b7d5e6d` | `8b90b51a7b7d5e6d` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x52 imm=0x1C0 | `498b87900200004881c0c001000049898790020000c3` (22) | same | same | Y | `6c82474ed68d4ac8` | `6c82474ed68d4ac8` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x61 SUB-IMM slot=0x51 imm=0x1B8 — **PASS**

- fixture: `_scratch_subimm_h51_1B8.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e8b801000049898788020000c3`
- js-sha256: `e5f22d8e3828fbe4596e012f550ab1fed4821b512a354463e287515bac9fe59e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x52 imm=0x1B8 — **PASS**

- fixture: `_scratch_subimm_h52_1B8.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e8b801000049898790020000c3`
- js-sha256: `fb3630917fc37295b4f565e189083d072380118d3b3ad0fa070b19766670ecdc`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x50 ss=0x60 oo=0x1C0 — **PASS**

- fixture: `_scratch_ldb_5060_1C0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c0010000480fb60049898780020000c3`
- js-sha256: `8953358138eb317e0172f330f7236e90798bf1191696a8264469afa9d88b36d7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x51 ss=0x60 oo=0x1C0 — **PASS**

- fixture: `_scratch_ldb_5160_1C0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c0010000480fb60049898788020000c3`
- js-sha256: `1759a1345d7af7eec3afd651f38cebca88cc9a916bc9d3b6b1ee70a0d52b19b2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x52 ss=0x60 oo=0x1C0 — **PASS**

- fixture: `_scratch_ldb_5260_1C0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0c0010000480fb60049898790020000c3`
- js-sha256: `bc6894d42acc60846b7e08a7cf763c8f45a0050405f0d701595409a20282f747`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x50 imm=0x1C0 — **PASS**

- fixture: `_scratch_addimm_h50_1C0.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c0c001000049898780020000c3`
- js-sha256: `f6926af2f6dc5e890d94e44468a25abd66f381457c950d1212c821a739ffccc8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x51 imm=0x1C0 — **PASS**

- fixture: `_scratch_addimm_h51_1C0.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c0c001000049898788020000c3`
- js-sha256: `8b90b51a7b7d5e6d75c25bed4669184293c74de28c60533001c10ad7ebc57f15`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x52 imm=0x1C0 — **PASS**

- fixture: `_scratch_addimm_h52_1C0.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c0c001000049898790020000c3`
- js-sha256: `6c82474ed68d4ac89252251890ac19f0f64bd257c2bbf3f8ab053d9c888e1210`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SUB-IMM slot=51/52 imm=1B8 (finish deferred 1B8 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=1C0 (start deferred 1C0 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=1C0 (start deferred 1C0 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=1C0 deferred to next batch.
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 25B`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_subimm_h51_1B8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_1B8.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_1C0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_1C0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_1C0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_1C0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_1C0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_1C0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-78-log.md` — this file
- `scripts/_probe/parallel-batch-78-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-084 serialize PASSes + 1 Relock**

Pass pin from body-extend-083 Relock: `45dff031e2acfa0ee40a932a4bca8709747e45bb1ac19f622fe0c477c4fe4a44`.
Handlers before consolidate = 603 (H_00..H_596). Next selectors `40 25B`.. for H_597.. if all serialize.

PASS list for body-extend-084:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_597 | 0x25B | 0x61 SUB-IMM | 0x51 0x1B8 | `498b87880200004881e8b801000049898788020000c3` (22B) | `e5f22d8e3828fbe4` |
| H_598 | 0x25C | 0x61 SUB-IMM | 0x52 0x1B8 | `498b87900200004881e8b801000049898790020000c3` (22B) | `fb3630917fc37295` |
| H_599 | 0x25D | 0x80 LDB | 0x50 0x60 0x1C0 | `498b87000300004881c0c0010000480fb60049898780020000c3` (26B) | `8953358138eb317e` |
| H_600 | 0x25E | 0x80 LDB | 0x51 0x60 0x1C0 | `498b87000300004881c0c0010000480fb60049898788020000c3` (26B) | `1759a1345d7af7ee` |
| H_601 | 0x25F | 0x80 LDB | 0x52 0x60 0x1C0 | `498b87000300004881c0c0010000480fb60049898790020000c3` (26B) | `bc6894d42acc6084` |
| H_602 | 0x260 | 0x62 ADD-IMM | 0x50 0x1C0 | `498b87800200004881c0c001000049898780020000c3` (22B) | `f6926af2f6dc5e89` |
| H_603 | 0x261 | 0x62 ADD-IMM | 0x51 0x1C0 | `498b87880200004881c0c001000049898788020000c3` (22B) | `8b90b51a7b7d5e6d` |
| H_604 | 0x262 | 0x62 ADD-IMM | 0x52 0x1C0 | `498b87900200004881c0c001000049898790020000c3` (22B) | `6c82474ed68d4ac8` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-083 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_596.
- If the parent decides to serialize, append H_597.. at selectors `40 25B`..:
  - H_597 0x61 SUB-IMM (61 51 1B8) — pin `498b87880200004881e8b801000049898788020000c3`
  - H_598 0x61 SUB-IMM (61 52 1B8) — pin `498b87900200004881e8b801000049898790020000c3`
  - H_599 0x80 LDB (80 50 60 1C0) — pin `498b87000300004881c0c0010000480fb60049898780020000c3`
  - H_600 0x80 LDB (80 51 60 1C0) — pin `498b87000300004881c0c0010000480fb60049898788020000c3`
  - H_601 0x80 LDB (80 52 60 1C0) — pin `498b87000300004881c0c0010000480fb60049898790020000c3`
  - H_602 0x62 ADD-IMM (62 50 1C0) — pin `498b87800200004881c0c001000049898780020000c3`
  - H_603 0x62 ADD-IMM (62 51 1C0) — pin `498b87880200004881c0c001000049898788020000c3`
  - H_604 0x62 ADD-IMM (62 52 1C0) — pin `498b87900200004881c0c001000049898790020000c3`
- Plus 1 Relock after append from pin `45dff031…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-084 serialize PASSes + 1 Relock
