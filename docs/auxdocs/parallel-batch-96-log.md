# parallel-batch-96 Log · 8-pick matrix-priority scratch sweep

> Tag: `parallel-batch-96-EXPERIMENTAL-8-pick-matrix` · 2026-07-26 (UTC+8).
> Following body-extend-101 (pin `514ff62ce8663a1507d07f965ba205e7483428b503c34e58f0a35dcfe4064719`, handlers = 747, H_00..H_740 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (still do not invent-green).
>
> **STRATEGY SWITCH**: This batch picks from `docs/auxdocs/selfhost-emit-matrix.md`
> priority gaps (P0→P1), NOT random imm ladders. Prior batches 94/95 used imm
> ladders (P3). Starting batch-96, all picks trace to a matrix (opcode, shape) row.
> Full body-extend phase ends when all selfhost-need=YES rows reach DONE status.
>
> 8 picks: 2 MEMCPY stub probes (P0), 2 GET multi-slot (P0), 2 SET multi-imm (P0),
> 1 ORV multi-combo (P1), 1 CMP multi-slot (P1).
> Next selectors after 0x2EA: `40 2EB`..

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x84 MEMCPY_DATA | dst=0x50 src=0x51 n=0x40 | `c3c3` (2) | same | same | Y | `1344fed055987f9e` | `1344fed055987f9e` | PASS |
| 2 | 0x85 MEMCPY_STATE | dst=0x50 src=0x51 n=0x40 | `c3c3` (2) | same | same | Y | `1344fed055987f9e` | `1344fed055987f9e` | PASS |
| 3 | 0x60 GET | dst=0x60 src=0x50 | `498b878002000049898700030000c3` (15) | same | same | Y | `81fbdbb14873c447` | `81fbdbb14873c447` | PASS |
| 4 | 0x60 GET | dst=0x50 src=0x60 | `498b870003000049898780020000c3` (15) | same | same | Y | `236c066a6b5b44ef` | `236c066a6b5b44ef` | PASS |
| 5 | 0x30 SET | slot=0x50 imm=0xfff | `48b8ff0f00000000000049898780020000c3` (18) | same | same | Y | `61697071ff6cd475` | `61697071ff6cd475` | PASS |
| 6 | 0x30 SET | slot=0x51 imm=0x10000 | `48b8000001000000000049898788020000c3` (18) | same | same | Y | `11a103bf4b11cd82` | `11a103bf4b11cd82` | PASS |
| 7 | 0x69 ORV | dst=0x50 src=0x62 | `498b8780020000498b8f100300004809c849898780020000c3` (25) | same | same | Y | `d1ef5ee917509ccc` | `d1ef5ee917509ccc` | PASS |
| 8 | 0x65 CMP | a=0x60 b=0x52 | `498b8700030000498b8f900200004839c8c3` (18) | same | same | Y | `9d5076dd78f13b7f` | `9d5076dd78f13b7f` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x84 MEMCPY_DATA dst=0x50 src=0x51 n=0x40 — **PASS**

- fixture: `_scratch_memcpy_data_stub.ty` + `.code.hex`
- expected pin (2B): `c3c3`
- js-sha256: `1344fed055987f9eb87aef70de1922ae7239d57b2a38fd9fb91a2a3f0b4c497a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y
- **MEMCPY NOTE**: JS & Rust both emit stub `0xc3`; byte-eq PASSES, but this is the D-3 semantic gap (stub does not actually copy). Consolidation (body-extend-102) must implement real MEMCPY before this row is truly DONE.

### Pick 2: 0x85 MEMCPY_STATE dst=0x50 src=0x51 n=0x40 — **PASS**

- fixture: `_scratch_memcpy_state_stub.ty` + `.code.hex`
- expected pin (2B): `c3c3`
- js-sha256: `1344fed055987f9eb87aef70de1922ae7239d57b2a38fd9fb91a2a3f0b4c497a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y
- **MEMCPY NOTE**: JS & Rust both emit stub `0xc3`; byte-eq PASSES, but this is the D-3 semantic gap (stub does not actually copy). Consolidation (body-extend-102) must implement real MEMCPY before this row is truly DONE.

### Pick 3: 0x60 GET dst=0x60 src=0x50 — **PASS**

- fixture: `_scratch_get_60_50.ty` + `.code.hex`
- expected pin (15B): `498b878002000049898700030000c3`
- js-sha256: `81fbdbb14873c447b3b9bc5bd013c689c0ab643218d88c3dce820ce8526f4374`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


### Pick 4: 0x60 GET dst=0x50 src=0x60 — **PASS**

- fixture: `_scratch_get_50_60.ty` + `.code.hex`
- expected pin (15B): `498b870003000049898780020000c3`
- js-sha256: `236c066a6b5b44ef04ddb29402d2d5ef64c905f0eb75a55abb7ad1336384d552`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


### Pick 5: 0x30 SET slot=0x50 imm=0xfff — **PASS**

- fixture: `_scratch_set_50_0xfff.ty` + `.code.hex`
- expected pin (18B): `48b8ff0f00000000000049898780020000c3`
- js-sha256: `61697071ff6cd475f073532bd14da2567d20a87d1090dee1983f599f4eae00ee`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


### Pick 6: 0x30 SET slot=0x51 imm=0x10000 — **PASS**

- fixture: `_scratch_set_51_0x10000.ty` + `.code.hex`
- expected pin (18B): `48b8000001000000000049898788020000c3`
- js-sha256: `11a103bf4b11cd823012f36cd9a66ea86cfbbd1c6e1a0928a394190211582a5c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


### Pick 7: 0x69 ORV dst=0x50 src=0x62 — **PASS**

- fixture: `_scratch_orv_50_62.ty` + `.code.hex`
- expected pin (25B): `498b8780020000498b8f100300004809c849898780020000c3`
- js-sha256: `d1ef5ee917509ccc81e91d54f84d04ae34496c7e8dc6951184f38e8de30ec145`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


### Pick 8: 0x65 CMP a=0x60 b=0x52 — **PASS**

- fixture: `_scratch_cmp_60_52.ty` + `.code.hex`
- expected pin (18B): `498b8700030000498b8f900200004839c8c3`
- js-sha256: `9d5076dd78f13b7f682c2bfb9ff0925be887e072242b21cecdd75671abb8f3a8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y



## §2. Pick rationale (matrix trace)

- **P0 · 84 MEMCPY_DATA**: matrix row "MISSING / YES" — stub emits `0xc3` in JS & Rust; byte-eq passes but semantic gap remains. Real MEMCPY needed for self-host.
- **P0 · 85 MEMCPY_STATE**: matrix row "MISSING / YES" — same stub situation as MEMCPY_DATA.
- **P0 · 60 GET multi-slot**: dst=60 src=50 and dst=50 src=60; tests cross-slot load+store with varying disp (0x300/0x280).
- **P0 · 30 SET multi-imm**: imm=0xfff (imm32, 22B) on slot 50; imm=0x10000 (imm32, 22B) on slot 51; fresh large imm values.
- **P1 · 69 ORV multi-combo**: dst=50 src=62; fresh bitwise OR pair not in H_48..H_740.
- **P1 · 65 CMP multi-slot**: a=60 b=52; fresh comparison pair for Jcc loops.
- No D-1 0x20/0x50/0x51, no D-2 0x64.
- yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_memcpy_data_stub.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_memcpy_state_stub.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_get_60_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_get_50_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_50_0xfff.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_51_0x10000.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_orv_50_62.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_cmp_60_52.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-96-log.md` — this file
- `scripts/_probe/parallel-batch-96-run.mjs` — probe runner (uses shared concurrent lib)
- `scripts/_probe/parallel-batch-scratch-lib.mjs` — ≤8 scratch workers

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-102 serialize PASSes + 1 Relock** (consolidator implements MEMCPY)

Pass pin from body-extend-101 Relock: `514ff62ce8663a1507d07f965ba205e7483428b503c34e58f0a35dcfe4064719`.
Handlers before consolidate = 747 (H_00..H_740). Next selectors `40 2EB`.. for H_741.. if all serialize.

PASS list for body-extend-102:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_741 | 0x2EB | 0x84 MEMCPY_DATA | 0x50 0x51 0x40 | `c3c3` (2B) | `1344fed055987f9e` |
| H_742 | 0x2EC | 0x85 MEMCPY_STATE | 0x50 0x51 0x40 | `c3c3` (2B) | `1344fed055987f9e` |
| H_743 | 0x2ED | 0x60 GET | 0x60 0x50 | `498b878002000049898700030000c3` (15B) | `81fbdbb14873c447` |
| H_744 | 0x2EE | 0x60 GET | 0x50 0x60 | `498b870003000049898780020000c3` (15B) | `236c066a6b5b44ef` |
| H_745 | 0x2EF | 0x30 SET | 0x50 0xfff | `48b8ff0f00000000000049898780020000c3` (18B) | `61697071ff6cd475` |
| H_746 | 0x2F0 | 0x30 SET | 0x51 0x10000 | `48b8000001000000000049898788020000c3` (18B) | `11a103bf4b11cd82` |
| H_747 | 0x2F1 | 0x69 ORV | 0x50 0x62 | `498b8780020000498b8f100300004809c849898780020000c3` (25B) | `d1ef5ee917509ccc` |
| H_748 | 0x2F2 | 0x65 CMP | 0x60 0x52 | `498b8700030000498b8f900200004839c8c3` (18B) | `9d5076dd78f13b7f` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- MEMCPY_DATA/STATE probe: both emit stub `0xc3` → byte-eq PASSES; the D-3 gap is semantic (no real copy), recorded honestly, not invented-green.

## §6. Consolidation handoff

parent next = body-extend-102 serialize PASSes + 1 Relock (MEMCPY implementation required before true DONE)
