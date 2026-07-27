# parallel-batch-71 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-71-EXPERIMENTAL-8-pick-scratch` · 2026-07-26 (UTC+8).
> Following body-extend-076 (pin `ebbc6d76…`, handlers = 547, H_533..H_540 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-076 DDC PE `.text` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_540 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x52 imm=0x188 | `498b87900200004881c08801000049898790020000c3` (22) | same | same | Y | `ef2cfed790c9d301` | `ef2cfed790c9d301` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x50 imm=0x188 | `498b87800200004881e88801000049898780020000c3` (22) | same | same | Y | `4576822a906e44b8` | `4576822a906e44b8` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x51 imm=0x188 | `498b87880200004881e88801000049898788020000c3` (22) | same | same | Y | `6c36bec002d9aa7d` | `6c36bec002d9aa7d` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x52 imm=0x188 | `498b87900200004881e88801000049898790020000c3` (22) | same | same | Y | `c77a089b4ef783bb` | `c77a089b4ef783bb` | PASS |
| 5 | 0x80 LDB | dd=0x50 ss=0x60 oo=0x190 | `498b87000300004881c090010000480fb60049898780020000c3` (26) | same | same | Y | `e4ad649adfa675bd` | `e4ad649adfa675bd` | PASS |
| 6 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x190 | `498b87000300004881c090010000480fb60049898788020000c3` (26) | same | same | Y | `251c22877545c901` | `251c22877545c901` | PASS |
| 7 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x190 | `498b87000300004881c090010000480fb60049898790020000c3` (26) | same | same | Y | `21f0254d615d4969` | `21f0254d615d4969` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x50 imm=0x190 | `498b87800200004881c09001000049898780020000c3` (22) | same | same | Y | `0b1729a7a8c31cb9` | `0b1729a7a8c31cb9` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x52 imm=0x188 — **PASS**

- fixture: `_scratch_addimm_h52_188.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c08801000049898790020000c3`
- js-sha256: `ef2cfed790c9d301a5e0e1ab548235f8151a62bb286305c852eb8a574a858dc2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x50 imm=0x188 — **PASS**

- fixture: `_scratch_subimm_h50_188.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881e88801000049898780020000c3`
- js-sha256: `4576822a906e44b809a53bc7a47d6f3fed4fe69032568c8eda7f87908db4d346`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x51 imm=0x188 — **PASS**

- fixture: `_scratch_subimm_h51_188.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881e88801000049898788020000c3`
- js-sha256: `6c36bec002d9aa7d47823bae9b798b4dc708bf86beb972c41a859b8a0306e6e2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x52 imm=0x188 — **PASS**

- fixture: `_scratch_subimm_h52_188.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881e88801000049898790020000c3`
- js-sha256: `c77a089b4ef783bb9bd65accaf84e7c7c8c93a11feb6eaf13367e292569b410d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x80 LDB dd=0x50 ss=0x60 oo=0x190 — **PASS**

- fixture: `_scratch_ldb_5060_190.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c090010000480fb60049898780020000c3`
- js-sha256: `e4ad649adfa675bdf2ebda458ed06c9a0467812a402f3d0ebcf72a1de375467d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x80 LDB dd=0x51 ss=0x60 oo=0x190 — **PASS**

- fixture: `_scratch_ldb_5160_190.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c090010000480fb60049898788020000c3`
- js-sha256: `251c22877545c901900760bd670ecc3c4cb5859fe25dd5a312a1dc18c86bb180`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x80 LDB dd=0x52 ss=0x60 oo=0x190 — **PASS**

- fixture: `_scratch_ldb_5260_190.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c090010000480fb60049898790020000c3`
- js-sha256: `21f0254d615d4969cfeabc75fd1bed32d0186f70199fa80476540ef9edd1fcb2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x50 imm=0x190 — **PASS**

- fixture: `_scratch_addimm_h50_190.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c09001000049898780020000c3`
- js-sha256: `0b1729a7a8c31cb9d196acf729c90a88f1fb4eec35c56ed3180ed11842146c58`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM slot=52 imm=188 (finish deferred 188 ADD triad; imm32 22B).
- SUB-IMM slot=50/51/52 imm=188 (start 188 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=190 (start 190 LDB triad; imm32 26B).
- ADD-IMM slot=50 imm=190 (start 190 ADD triad; imm32 22B; slots 51/52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: `40 223`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h52_188.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_188.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_188.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_188.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5060_190.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_190.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_190.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_190.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-71-log.md` — this file
- `scripts/_probe/parallel-batch-71-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-077 serialize PASSes + 1 Relock**

Pass pin from body-extend-076 Relock: `ebbc6d765fcc0fcdc045848e93a3839d47ffdf287646adb781170a66d80690be`.
Handlers before consolidate = 547 (H_00..H_540). Next selectors `40 223`.. for H_541.. if all serialize.

PASS list for body-extend-077:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_541 | 0x223 | 0x62 ADD-IMM | 0x52 0x188 | `498b87900200004881c08801000049898790020000c3` (22B) | `ef2cfed790c9d301` |
| H_542 | 0x224 | 0x61 SUB-IMM | 0x50 0x188 | `498b87800200004881e88801000049898780020000c3` (22B) | `4576822a906e44b8` |
| H_543 | 0x225 | 0x61 SUB-IMM | 0x51 0x188 | `498b87880200004881e88801000049898788020000c3` (22B) | `6c36bec002d9aa7d` |
| H_544 | 0x226 | 0x61 SUB-IMM | 0x52 0x188 | `498b87900200004881e88801000049898790020000c3` (22B) | `c77a089b4ef783bb` |
| H_545 | 0x227 | 0x80 LDB | 0x50 0x60 0x190 | `498b87000300004881c090010000480fb60049898780020000c3` (26B) | `e4ad649adfa675bd` |
| H_546 | 0x228 | 0x80 LDB | 0x51 0x60 0x190 | `498b87000300004881c090010000480fb60049898788020000c3` (26B) | `251c22877545c901` |
| H_547 | 0x229 | 0x80 LDB | 0x52 0x60 0x190 | `498b87000300004881c090010000480fb60049898790020000c3` (26B) | `21f0254d615d4969` |
| H_548 | 0x22A | 0x62 ADD-IMM | 0x50 0x190 | `498b87800200004881c09001000049898780020000c3` (22B) | `0b1729a7a8c31cb9` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-076 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_540.
- If the parent decides to serialize, append H_541.. at selectors `40 223`..:
  - H_541 0x62 ADD-IMM (62 52 188) — pin `498b87900200004881c08801000049898790020000c3`
  - H_542 0x61 SUB-IMM (61 50 188) — pin `498b87800200004881e88801000049898780020000c3`
  - H_543 0x61 SUB-IMM (61 51 188) — pin `498b87880200004881e88801000049898788020000c3`
  - H_544 0x61 SUB-IMM (61 52 188) — pin `498b87900200004881e88801000049898790020000c3`
  - H_545 0x80 LDB (80 50 60 190) — pin `498b87000300004881c090010000480fb60049898780020000c3`
  - H_546 0x80 LDB (80 51 60 190) — pin `498b87000300004881c090010000480fb60049898788020000c3`
  - H_547 0x80 LDB (80 52 60 190) — pin `498b87000300004881c090010000480fb60049898790020000c3`
  - H_548 0x62 ADD-IMM (62 50 190) — pin `498b87800200004881c09001000049898780020000c3`
- Plus 1 Relock after append from pin `ebbc6d76…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-077 serialize PASSes + 1 Relock
