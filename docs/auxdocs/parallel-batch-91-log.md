# parallel-batch-91 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-91-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-096 (pin `0a02f49e…`, handlers = 707, H_693..H_700 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-096 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_700 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x50 imm=0x218 | `498b87800200004881c01802000049898780020000c3` (22) | same | same | Y | `4ab4f6b000bfc170` | `4ab4f6b000bfc170` | PASS |
| 2 | 0x62 ADD-IMM | slot=0x51 imm=0x218 | `498b87880200004881c01802000049898788020000c3` (22) | same | same | Y | `f2a5fff94e8993ce` | `f2a5fff94e8993ce` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x52 imm=0x218 | `498b87900200004881c01802000049898790020000c3` (22) | same | same | Y | `b8f58bcc6a95b935` | `b8f58bcc6a95b935` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x50 imm=0x218 | `498b87800200004881e81802000049898780020000c3` (22) | same | same | Y | `d98dffc59593a3e5` | `d98dffc59593a3e5` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x51 imm=0x218 | `498b87880200004881e81802000049898788020000c3` (22) | same | same | Y | `a2df94a8e97fec79` | `a2df94a8e97fec79` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x52 imm=0x218 | `498b87900200004881e81802000049898790020000c3` (22) | same | same | Y | `501f0c3b69e446a3` | `501f0c3b69e446a3` | PASS |
| 7 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x220 | `498b87000300004881c020020000480fb60049898780020000c3` (26) | same | same | Y | `38dd8dd1ab3ef61c` | `38dd8dd1ab3ef61c` | PASS |
| 8 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x220 | `498b87000300004881c020020000480fb60049898788020000c3` (26) | same | same | Y | `6633a1f5ac21e65f` | `6633a1f5ac21e65f` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x50 imm=0x218 — **PASS**

- fixture: `_scratch_addimm_h50_218.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c01802000049898780020000c3`
- js-sha256: `4ab4f6b000bfc170429e5110542aa259ae4323684ecd0111ced31957ca2ed16a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x62 ADD-IMM slot=0x51 imm=0x218 — **PASS**

- fixture: `_scratch_addimm_h51_218.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c01802000049898788020000c3`
- js-sha256: `f2a5fff94e8993ce3881609c5b31db4ca87a058fc1737abeae2dacfa6c0e6eea`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x52 imm=0x218 — **PASS**

- fixture: `_scratch_addimm_h52_218.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c01802000049898790020000c3`
- js-sha256: `b8f58bcc6a95b93566f9489f12aa546360d390c7233847963b448dffbc6e5986`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x50 imm=0x218 — **PASS**

- fixture: `_scratch_subimm_h50_218.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e81802000049898780020000c3`
- js-sha256: `d98dffc59593a3e5a92ed2b70d52f837b419c60bf8178c3e645177aa43293294`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x51 imm=0x218 — **PASS**

- fixture: `_scratch_subimm_h51_218.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e81802000049898788020000c3`
- js-sha256: `a2df94a8e97fec797174a0f4370ecf60ca0d2085543ee3c4795b586dbc22bcf3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x52 imm=0x218 — **PASS**

- fixture: `_scratch_subimm_h52_218.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e81802000049898790020000c3`
- js-sha256: `501f0c3b69e446a3f382e8f726884e6348c763eb6e365ef6dc05e7097aeff88a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x50 ss=0x60 oo=0x220 — **PASS**

- fixture: `_scratch_ldb_5060_220.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c020020000480fb60049898780020000c3`
- js-sha256: `38dd8dd1ab3ef61c0576bb5f5c590a0e710f7996c93657f9f915f2546903cdd0`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x80 LDB dd=0x51 ss=0x60 oo=0x220 — **PASS**

- fixture: `_scratch_ldb_5160_220.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c020020000480fb60049898788020000c3`
- js-sha256: `6633a1f5ac21e65f54355ae582c8f2364ba596270e71b1d3f28c2d6cd7bf6503`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=50/51/52 imm=218 (start deferred 218 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=218 (start 218 SUB triad; imm32 22B).
- LDB dd=50/51 ss=60 oo=220 (start 220 LDB ladder; imm32 26B; LDB 52 220 deferred).
- ADD-IMM slot=50/51/52 imm=220 deferred to next batch.
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 2C3`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h50_218.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_218.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_218.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_218.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_218.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_218.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_220.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_220.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-91-log.md` — this file
- `scripts/_probe/parallel-batch-91-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-097 serialize PASSes + 1 Relock**

Pass pin from body-extend-096 Relock: `0a02f49ed0c94a2df5078022a7737c92d4021cab62c41dcbbfc5bb728f32f29c`.
Handlers before consolidate = 707 (H_00..H_700). Next selectors `40 2C3`.. for H_701.. if all serialize.

PASS list for body-extend-097:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_701 | 0x2C3 | 0x62 ADD-IMM | 0x50 0x218 | `498b87800200004881c01802000049898780020000c3` (22B) | `4ab4f6b000bfc170` |
| H_702 | 0x2C4 | 0x62 ADD-IMM | 0x51 0x218 | `498b87880200004881c01802000049898788020000c3` (22B) | `f2a5fff94e8993ce` |
| H_703 | 0x2C5 | 0x62 ADD-IMM | 0x52 0x218 | `498b87900200004881c01802000049898790020000c3` (22B) | `b8f58bcc6a95b935` |
| H_704 | 0x2C6 | 0x61 SUB-IMM | 0x50 0x218 | `498b87800200004881e81802000049898780020000c3` (22B) | `d98dffc59593a3e5` |
| H_705 | 0x2C7 | 0x61 SUB-IMM | 0x51 0x218 | `498b87880200004881e81802000049898788020000c3` (22B) | `a2df94a8e97fec79` |
| H_706 | 0x2C8 | 0x61 SUB-IMM | 0x52 0x218 | `498b87900200004881e81802000049898790020000c3` (22B) | `501f0c3b69e446a3` |
| H_707 | 0x2C9 | 0x80 LDB | 0x50 0x60 0x220 | `498b87000300004881c020020000480fb60049898780020000c3` (26B) | `38dd8dd1ab3ef61c` |
| H_708 | 0x2CA | 0x80 LDB | 0x51 0x60 0x220 | `498b87000300004881c020020000480fb60049898788020000c3` (26B) | `6633a1f5ac21e65f` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-096 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_700.
- If the parent decides to serialize, append H_701.. at selectors `40 2C3`..:
  - H_701 0x62 ADD-IMM (62 50 218) — pin `498b87800200004881c01802000049898780020000c3`
  - H_702 0x62 ADD-IMM (62 51 218) — pin `498b87880200004881c01802000049898788020000c3`
  - H_703 0x62 ADD-IMM (62 52 218) — pin `498b87900200004881c01802000049898790020000c3`
  - H_704 0x61 SUB-IMM (61 50 218) — pin `498b87800200004881e81802000049898780020000c3`
  - H_705 0x61 SUB-IMM (61 51 218) — pin `498b87880200004881e81802000049898788020000c3`
  - H_706 0x61 SUB-IMM (61 52 218) — pin `498b87900200004881e81802000049898790020000c3`
  - H_707 0x80 LDB (80 50 60 220) — pin `498b87000300004881c020020000480fb60049898780020000c3`
  - H_708 0x80 LDB (80 51 60 220) — pin `498b87000300004881c020020000480fb60049898788020000c3`
- Plus 1 Relock after append from pin `0a02f49e…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: LDB 52 60 220; ADD-IMM slot=50/51/52 imm=220; finish 220 ladder.

## §7. Consolidation handoff

parent next = body-extend-097 serialize PASSes + 1 Relock
