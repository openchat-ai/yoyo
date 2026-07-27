# parallel-batch-25 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-25-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-030 (pin `9fddb56b…`, handlers = 180, H_166..H_173 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_173 and
> not already present as handlers in current `yoyo.ty` (skipped early
> INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51; skipped H_46 LDB 50 60 60).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x30 SET | slot=0x50 imm=0xDEADC0DE | `48b8dec0adde0000000049898780020000c3` (18) | same | same | Y | `2a769aa9aba9805c` | `2a769aa9aba9805c` | PASS |
| 2 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x60 | `498b87000300004883c060480fb60049898788020000c3` (23) | same | same | Y | `abf0f5b80eb452c0` | `abf0f5b80eb452c0` | PASS |
| 3 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x60 | `498b87000300004883c060480fb60049898790020000c3` (23) | same | same | Y | `24b65657d4e28852` | `24b65657d4e28852` | PASS |
| 4 | 0x62 ADD-IMM | slot=0x50 imm=0x40 | `498b87800200004883c04049898780020000c3` (19) | same | same | Y | `600b3eb1029e26ea` | `600b3eb1029e26ea` | PASS |
| 5 | 0x62 ADD-IMM | slot=0x51 imm=0x40 | `498b87880200004883c04049898788020000c3` (19) | same | same | Y | `ed54fe4ff3d8414c` | `ed54fe4ff3d8414c` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x52 imm=0x40 | `498b87900200004883c04049898790020000c3` (19) | same | same | Y | `e98fc8f93f052ba2` | `e98fc8f93f052ba2` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x52 imm=0x3C | `498b87900200004883e83c49898790020000c3` (19) | same | same | Y | `c57d88a68c708a91` | `c57d88a68c708a91` | PASS |
| 8 | 0x30 SET | slot=0x51 imm=0xDEADC0DE | `48b8dec0adde0000000049898788020000c3` (18) | same | same | Y | `946ee015447d1bab` | `946ee015447d1bab` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x30 SET slot=0x50 imm=0xDEADC0DE — **PASS**

- fixture: `_scratch_set_50_deadc0de.ty` + `.code.hex`
- expected pin (18B): `48b8dec0adde0000000049898780020000c3`
- js-sha256: `2a769aa9aba9805cfbe044a2c7277573c58028335f0105c5c625b1493e824440`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x51 ss=0x60 oo=0x60 — **PASS**

- fixture: `_scratch_ldb_5160_60.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c060480fb60049898788020000c3`
- js-sha256: `abf0f5b80eb452c05ef5f31d0662765533caf1f3554d0958a13714c29345927a`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x52 ss=0x60 oo=0x60 — **PASS**

- fixture: `_scratch_ldb_5260_60.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c060480fb60049898790020000c3`
- js-sha256: `24b65657d4e2885251bc9155a92cef65c2f25d5b0befd78ec4b9d2b54403ab0d`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x62 ADD-IMM slot=0x50 imm=0x40 — **PASS**

- fixture: `_scratch_addimm_h50_40.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c04049898780020000c3`
- js-sha256: `600b3eb1029e26eadd62a6dbb9051e77408a3051c4657f6305c68f4faddb7f92`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x62 ADD-IMM slot=0x51 imm=0x40 — **PASS**

- fixture: `_scratch_addimm_h51_40.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c04049898788020000c3`
- js-sha256: `ed54fe4ff3d8414c6dccbc87bfd8968fac1cd1faf81b700c40891e996e6d1a4e`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x52 imm=0x40 — **PASS**

- fixture: `_scratch_addimm_h52_40.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c04049898790020000c3`
- js-sha256: `e98fc8f93f052ba2ed1f09f1401b0d3210025ce318a6e30a244a244ef3556b48`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x52 imm=0x3C — **PASS**

- fixture: `_scratch_subimm_h52_3c.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e83c49898790020000c3`
- js-sha256: `c57d88a68c708a912b4b1d21fdb37e90ab04745efae589abb3123bbbb6e78122`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x30 SET slot=0x51 imm=0xDEADC0DE — **PASS**

- fixture: `_scratch_set_51_deadc0de.ty` + `.code.hex`
- expected pin (18B): `48b8dec0adde0000000049898788020000c3`
- js-sha256: `946ee015447d1bab70ed0740cb54b4b764be36fbcba10168acd72ada7d360b75`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- SET at slot 50/51 imm=DEADC0DE (fresh imm; locked SET imms exclude DEADC0DE).
- LDB dd=51/52 ss=60 oo=60 (H_46=50 60 60 only; fresh dst slots).
- ADD-IMM at slot 50/51/52 imm=40 (no imm=40 ADD-IMM locked; max was 3C).
- SUB-IMM at slot 52 imm=3C (H_169/H_173=50/51 3C; no 52 3C).
- Skipped suggested INC/DEC 50, GET/ORV/ADDV/SUBV 50 51, IMUL 50 51 (early), LDB 50 60 60 (H_46).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_set_50_deadc0de.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_60.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_40.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_40.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_40.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_3c.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_51_deadc0de.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-25-log.md` — this file
- `scripts/_probe/parallel-batch-25-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-031 serialize PASSes + 1 Relock**

Pass pin from body-extend-030 Relock: `9fddb56b31ab513c92e4435193619de1193f4ea543bbb4b2a239531eeefae0ea`.
Handlers before consolidate = 180 (H_00..H_173). Next selectors 0xB4.. for H_174.. if all serialize.

PASS list for body-extend-031:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_174 | 0xB4 | 0x30 SET | 0x50 0xDEADC0DE | `48b8dec0adde0000000049898780020000c3` (18B) | `2a769aa9aba9805c` |
| H_175 | 0xB5 | 0x80 LDB | 0x51 0x60 0x60 | `498b87000300004883c060480fb60049898788020000c3` (23B) | `abf0f5b80eb452c0` |
| H_176 | 0xB6 | 0x80 LDB | 0x52 0x60 0x60 | `498b87000300004883c060480fb60049898790020000c3` (23B) | `24b65657d4e28852` |
| H_177 | 0xB7 | 0x62 ADD-IMM | 0x50 0x40 | `498b87800200004883c04049898780020000c3` (19B) | `600b3eb1029e26ea` |
| H_178 | 0xB8 | 0x62 ADD-IMM | 0x51 0x40 | `498b87880200004883c04049898788020000c3` (19B) | `ed54fe4ff3d8414c` |
| H_179 | 0xB9 | 0x62 ADD-IMM | 0x52 0x40 | `498b87900200004883c04049898790020000c3` (19B) | `e98fc8f93f052ba2` |
| H_180 | 0xBA | 0x61 SUB-IMM | 0x52 0x3C | `498b87900200004883e83c49898790020000c3` (19B) | `c57d88a68c708a91` |
| H_181 | 0xBB | 0x30 SET | 0x51 0xDEADC0DE | `48b8dec0adde0000000049898788020000c3` (18B) | `946ee015447d1bab` |

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
  fresh slot/imm/dst combinations not in H_48..H_173.
- If the parent decides to serialize, append H_174.. at selectors 0xB4..:
  - H_174 0x30 SET (30 50 DEADC0DE) — pin `48b8dec0adde0000000049898780020000c3`
  - H_175 0x80 LDB (80 51 60 60) — pin `498b87000300004883c060480fb60049898788020000c3`
  - H_176 0x80 LDB (80 52 60 60) — pin `498b87000300004883c060480fb60049898790020000c3`
  - H_177 0x62 ADD-IMM (62 50 40) — pin `498b87800200004883c04049898780020000c3`
  - H_178 0x62 ADD-IMM (62 51 40) — pin `498b87880200004883c04049898788020000c3`
  - H_179 0x62 ADD-IMM (62 52 40) — pin `498b87900200004883c04049898790020000c3`
  - H_180 0x61 SUB-IMM (61 52 3C) — pin `498b87900200004883e83c49898790020000c3`
  - H_181 0x30 SET (30 51 DEADC0DE) — pin `48b8dec0adde0000000049898788020000c3`
- Plus 1 Relock after append from pin `9fddb56b…`.

## §7. Consolidation handoff

parent next = body-extend-031 serialize PASSes + 1 Relock
