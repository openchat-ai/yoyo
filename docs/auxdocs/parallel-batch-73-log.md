# parallel-batch-73 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-73-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-078 (pin `4c42576d…`, handlers = 563, H_549..H_556 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-078 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_556 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x50 imm=0x198 | `498b87800200004881c09801000049898780020000c3` (22) | same | same | Y | `b9a1454084d99711` | `b9a1454084d99711` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x51 imm=0x198 | `498b87880200004881c09801000049898788020000c3` (22) | same | same | Y | `6dfea21cc077f979` | `6dfea21cc077f979` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x52 imm=0x198 | `498b87900200004881c09801000049898790020000c3` (22) | same | same | Y | `b4bced2f75175884` | `b4bced2f75175884` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x198 | `498b87800200004881e89801000049898780020000c3` (22) | same | same | Y | `7dca7636d1845a95` | `7dca7636d1845a95` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x51 imm=0x198 | `498b87880200004881e89801000049898788020000c3` (22) | same | same | Y | `5b1facdbbae86c25` | `5b1facdbbae86c25` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x52 imm=0x198 | `498b87900200004881e89801000049898790020000c3` (22) | same | same | Y | `3b46829def05556b` | `3b46829def05556b` | PASS |
| 7 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x1A0 | `498b87000300004881c0a0010000480fb60049898780020000c3` (26) | same | same | Y | `bcf7781865161f65` | `bcf7781865161f65` | PASS |
| 8 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x1A0 | `498b87000300004881c0a0010000480fb60049898788020000c3` (26) | same | same | Y | `55cd34d122a07524` | `55cd34d122a07524` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x50 imm=0x198 — **PASS**

- fixture: `_scratch_addimm_h50_198.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c09801000049898780020000c3`
- js-sha256: `b9a1454084d99711e863c60d89ef428ac9c1000ac459788d80ae9b81237e9a8d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x51 imm=0x198 — **PASS**

- fixture: `_scratch_addimm_h51_198.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c09801000049898788020000c3`
- js-sha256: `6dfea21cc077f97993970fba0b515ea60b4a8dd2713f7742970c1846130b496c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x52 imm=0x198 — **PASS**

- fixture: `_scratch_addimm_h52_198.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c09801000049898790020000c3`
- js-sha256: `b4bced2f75175884c281ebe07167b1c07f4c22ed228efd262cf82c72003aace7`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x198 — **PASS**

- fixture: `_scratch_subimm_h50_198.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e89801000049898780020000c3`
- js-sha256: `7dca7636d1845a95121362d02954825a9ac14150259681115527e9b451cd7b69`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x51 imm=0x198 — **PASS**

- fixture: `_scratch_subimm_h51_198.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e89801000049898788020000c3`
- js-sha256: `5b1facdbbae86c25f64f38daf14816bb07bc7424cf3996f41a5f0b02baf132e4`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x52 imm=0x198 — **PASS**

- fixture: `_scratch_subimm_h52_198.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e89801000049898790020000c3`
- js-sha256: `3b46829def05556bb9655db0f7ca21419879296a60f941c45f8f476e9e6b8cfc`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x50 ss=0x60 oo=0x1A0 — **PASS**

- fixture: `_scratch_ldb_5060_1A0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a0010000480fb60049898780020000c3`
- js-sha256: `bcf7781865161f6502cc7b701f7de3ee7650446a480df69236b456e2cbe63530`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x51 ss=0x60 oo=0x1A0 — **PASS**

- fixture: `_scratch_ldb_5160_1A0.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c0a0010000480fb60049898788020000c3`
- js-sha256: `55cd34d122a075243914eb017cd595cb0a6c5281fb1c2d2ff2e6f816d0057416`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=50/51/52 imm=198 (start deferred 198 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=198 (start deferred 198 SUB triad; imm32 22B).
- LDB dd=50/51 ss=60 oo=1A0 (start 1A0 LDB triad; imm32 26B; leave dd=52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 233`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h50_198.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_198.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_198.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_198.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_198.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_198.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_1A0.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_1A0.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-73-log.md` — this file
- `scripts/_probe/parallel-batch-73-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-079 serialize PASSes + 1 Relock**

Pass pin from body-extend-078 Relock: `4c42576df4f80a8d3f4e57074fb4fc081bc16d37c9638b9fd0659ddae86fd42b`.
Handlers before consolidate = 563 (H_00..H_556). Next selectors `40 233`.. for H_557.. if all serialize.

PASS list for body-extend-079:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_557 | 0x233 | 0x62 ADD-IMM | 0x50 0x198 | `498b87800200004881c09801000049898780020000c3` (22B) | `b9a1454084d99711` |
| H_558 | 0x234 | 0x62 ADD-IMM | 0x51 0x198 | `498b87880200004881c09801000049898788020000c3` (22B) | `6dfea21cc077f979` |
| H_559 | 0x235 | 0x62 ADD-IMM | 0x52 0x198 | `498b87900200004881c09801000049898790020000c3` (22B) | `b4bced2f75175884` |
| H_560 | 0x236 | 0x61 SUB-IMM | 0x50 0x198 | `498b87800200004881e89801000049898780020000c3` (22B) | `7dca7636d1845a95` |
| H_561 | 0x237 | 0x61 SUB-IMM | 0x51 0x198 | `498b87880200004881e89801000049898788020000c3` (22B) | `5b1facdbbae86c25` |
| H_562 | 0x238 | 0x61 SUB-IMM | 0x52 0x198 | `498b87900200004881e89801000049898790020000c3` (22B) | `3b46829def05556b` |
| H_563 | 0x239 | 0x80 LDB | 0x50 0x60 0x1A0 | `498b87000300004881c0a0010000480fb60049898780020000c3` (26B) | `bcf7781865161f65` |
| H_564 | 0x23A | 0x80 LDB | 0x51 0x60 0x1A0 | `498b87000300004881c0a0010000480fb60049898788020000c3` (26B) | `55cd34d122a07524` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-078 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_556.
- If the parent decides to serialize, append H_557.. at selectors `40 233`..:
  - H_557 0x62 ADD-IMM (62 50 198) — pin `498b87800200004881c09801000049898780020000c3`
  - H_558 0x62 ADD-IMM (62 51 198) — pin `498b87880200004881c09801000049898788020000c3`
  - H_559 0x62 ADD-IMM (62 52 198) — pin `498b87900200004881c09801000049898790020000c3`
  - H_560 0x61 SUB-IMM (61 50 198) — pin `498b87800200004881e89801000049898780020000c3`
  - H_561 0x61 SUB-IMM (61 51 198) — pin `498b87880200004881e89801000049898788020000c3`
  - H_562 0x61 SUB-IMM (61 52 198) — pin `498b87900200004881e89801000049898790020000c3`
  - H_563 0x80 LDB (80 50 60 1A0) — pin `498b87000300004881c0a0010000480fb60049898780020000c3`
  - H_564 0x80 LDB (80 51 60 1A0) — pin `498b87000300004881c0a0010000480fb60049898788020000c3`
- Plus 1 Relock after append from pin `4c42576d…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-079 serialize PASSes + 1 Relock
