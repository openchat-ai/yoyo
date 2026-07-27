# parallel-batch-93 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-93-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-098 (pin `8d427725…`, handlers = 723, H_709..H_716 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-098 DDC PE `.text` measured DIFFER this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_716 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x228 | `498b87000300004881c028020000480fb60049898788020000c3` (26) | same | same | Y | `ec662f4d79ff8add` | `ec662f4d79ff8add` | PASS |
| 2 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x228 | `498b87000300004881c028020000480fb60049898790020000c3` (26) | same | same | Y | `0a14cf8c72933615` | `0a14cf8c72933615` | PASS |
| 3 | 0x62 ADD-IMM | slot=0x50 imm=0x228 | `498b87800200004881c02802000049898780020000c3` (22) | same | same | Y | `308359b06a3c0b71` | `308359b06a3c0b71` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x51 imm=0x228 | `498b87880200004881c02802000049898788020000c3` (22) | same | same | Y | `30a3548d2b182ab8` | `30a3548d2b182ab8` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x52 imm=0x228 | `498b87900200004881c02802000049898790020000c3` (22) | same | same | Y | `bb5db527c469beec` | `bb5db527c469beec` | PASS |
| 6 | 0x61 SUB-IMM | slot=0x50 imm=0x228 | `498b87800200004881e82802000049898780020000c3` (22) | same | same | Y | `f21787f68d23f722` | `f21787f68d23f722` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0x228 | `498b87880200004881e82802000049898788020000c3` (22) | same | same | Y | `b4edd744e6cbfd23` | `b4edd744e6cbfd23` | PASS |
| 8 | 0x61 SUB-IMM | slot=0x52 imm=0x228 | `498b87900200004881e82802000049898790020000c3` (22) | same | same | Y | `a64562f9de393830` | `a64562f9de393830` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x51 ss=0x60 oo=0x228 — **PASS**

- fixture: `_scratch_ldb_5160_228.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c028020000480fb60049898788020000c3`
- js-sha256: `ec662f4d79ff8add66c6b0606f5a408b3c485d2361c271118c0eef2d41ed60d3`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x52 ss=0x60 oo=0x228 — **PASS**

- fixture: `_scratch_ldb_5260_228.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c028020000480fb60049898790020000c3`
- js-sha256: `0a14cf8c7293361575919dd2df7f3ffb4bdb7fa27f1ef29919b7a2b4a3ba149e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x62 ADD-IMM slot=0x50 imm=0x228 — **PASS**

- fixture: `_scratch_addimm_h50_228.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c02802000049898780020000c3`
- js-sha256: `308359b06a3c0b715e6575564a5adf581bb2bab054dee4b61ba0b5ab4d8c52d8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x51 imm=0x228 — **PASS**

- fixture: `_scratch_addimm_h51_228.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c02802000049898788020000c3`
- js-sha256: `30a3548d2b182ab87c5fdd862f32dd467b0bf1c4078df243023e90e6c0c0a874`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x52 imm=0x228 — **PASS**

- fixture: `_scratch_addimm_h52_228.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c02802000049898790020000c3`
- js-sha256: `bb5db527c469beeca3feb4b57ce15971c13ba3a1916646a503c65e9042a608ff`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x61 SUB-IMM slot=0x50 imm=0x228 — **PASS**

- fixture: `_scratch_subimm_h50_228.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e82802000049898780020000c3`
- js-sha256: `f21787f68d23f722623c13531402795368893b64a61a65a658b8953e26320347`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0x228 — **PASS**

- fixture: `_scratch_subimm_h51_228.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e82802000049898788020000c3`
- js-sha256: `b4edd744e6cbfd23b6f73bc312697cbee7bce3125f544060286caeeccd04cd57`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x61 SUB-IMM slot=0x52 imm=0x228 — **PASS**

- fixture: `_scratch_subimm_h52_228.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e82802000049898790020000c3`
- js-sha256: `a64562f9de393830d164b6493bb727b27106b09e043cc597c4f75ff11ecababd`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=51/52 ss=60 oo=228 (finish deferred 228 LDB triad; imm32 26B).
- ADD-IMM slot=50/51/52 imm=228 (start deferred 228 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=228 (start 228 SUB triad; imm32 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 2D3`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5160_228.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_228.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_228.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_228.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_228.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_228.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_228.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_228.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-93-log.md` — this file
- `scripts/_probe/parallel-batch-93-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-099 serialize PASSes + 1 Relock**

Pass pin from body-extend-098 Relock: `8d4277255b098dc108295590e42155afd50ffca67fbab34ea1430ef615405136`.
Handlers before consolidate = 723 (H_00..H_716). Next selectors `40 2D3`.. for H_717.. if all serialize.

PASS list for body-extend-099:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_717 | 0x2D3 | 0x80 LDB | 0x51 0x60 0x228 | `498b87000300004881c028020000480fb60049898788020000c3` (26B) | `ec662f4d79ff8add` |
| H_718 | 0x2D4 | 0x80 LDB | 0x52 0x60 0x228 | `498b87000300004881c028020000480fb60049898790020000c3` (26B) | `0a14cf8c72933615` |
| H_719 | 0x2D5 | 0x62 ADD-IMM | 0x50 0x228 | `498b87800200004881c02802000049898780020000c3` (22B) | `308359b06a3c0b71` |
| H_720 | 0x2D6 | 0x62 ADD-IMM | 0x51 0x228 | `498b87880200004881c02802000049898788020000c3` (22B) | `30a3548d2b182ab8` |
| H_721 | 0x2D7 | 0x62 ADD-IMM | 0x52 0x228 | `498b87900200004881c02802000049898790020000c3` (22B) | `bb5db527c469beec` |
| H_722 | 0x2D8 | 0x61 SUB-IMM | 0x50 0x228 | `498b87800200004881e82802000049898780020000c3` (22B) | `f21787f68d23f722` |
| H_723 | 0x2D9 | 0x61 SUB-IMM | 0x51 0x228 | `498b87880200004881e82802000049898788020000c3` (22B) | `b4edd744e6cbfd23` |
| H_724 | 0x2DA | 0x61 SUB-IMM | 0x52 0x228 | `498b87900200004881e82802000049898790020000c3` (22B) | `a64562f9de393830` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-098 DDC PE `.text` DIFFER noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_716.
- If the parent decides to serialize, append H_717.. at selectors `40 2D3`..:
  - H_717 0x80 LDB (80 51 60 228) — pin `498b87000300004881c028020000480fb60049898788020000c3`
  - H_718 0x80 LDB (80 52 60 228) — pin `498b87000300004881c028020000480fb60049898790020000c3`
  - H_719 0x62 ADD-IMM (62 50 228) — pin `498b87800200004881c02802000049898780020000c3`
  - H_720 0x62 ADD-IMM (62 51 228) — pin `498b87880200004881c02802000049898788020000c3`
  - H_721 0x62 ADD-IMM (62 52 228) — pin `498b87900200004881c02802000049898790020000c3`
  - H_722 0x61 SUB-IMM (61 50 228) — pin `498b87800200004881e82802000049898780020000c3`
  - H_723 0x61 SUB-IMM (61 51 228) — pin `498b87880200004881e82802000049898788020000c3`
  - H_724 0x61 SUB-IMM (61 52 228) — pin `498b87900200004881e82802000049898790020000c3`
- Plus 1 Relock after append from pin `8d427725…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: next ladder LDB 50/51/52 60 230; continue 228 ADD/SUB if any REJECT.

## §7. Consolidation handoff

parent next = body-extend-099 serialize PASSes + 1 Relock
