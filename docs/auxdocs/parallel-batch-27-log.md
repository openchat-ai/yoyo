# parallel-batch-27 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-27-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-032 (pin `a0cb2642…`, handlers = 196, H_182..H_189 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-032 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_189 and
> not already present as handlers in current `yoyo.ty` (skipped H_47 LDB 50 60 70).
> Slot/imm/dst variations of SET/ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x62 ADD-IMM | slot=0x52 imm=0x48 | `498b87900200004883c04849898790020000c3` (19) | same | same | Y | `87d6ef901773b519` | `87d6ef901773b519` | PASS |
| 2 | 0x61 SUB-IMM | slot=0x52 imm=0x40 | `498b87900200004883e84049898790020000c3` (19) | same | same | Y | `6389a07c533b54d4` | `6389a07c533b54d4` | PASS |
| 3 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x70 | `498b87000300004883c070480fb60049898788020000c3` (23) | same | same | Y | `a36507620f4b048d` | `a36507620f4b048d` | PASS |
| 4 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x70 | `498b87000300004883c070480fb60049898790020000c3` (23) | same | same | Y | `29dddd3529790413` | `29dddd3529790413` | PASS |
| 5 | 0x30 SET | slot=0x50 imm=0xC0DEC0DE | `48b8dec0dec00000000049898780020000c3` (18) | same | same | Y | `b41a84acb6668560` | `b41a84acb6668560` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x50 imm=0x50 | `498b87800200004883c05049898780020000c3` (19) | same | same | Y | `137444f465f92575` | `137444f465f92575` | PASS |
| 7 | 0x61 SUB-IMM | slot=0x51 imm=0x48 | `498b87880200004883e84849898788020000c3` (19) | same | same | Y | `29980365da8b1f33` | `29980365da8b1f33` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x51 imm=0x50 | `498b87880200004883c05049898788020000c3` (19) | same | same | Y | `c608d7b30f277885` | `c608d7b30f277885` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x62 ADD-IMM slot=0x52 imm=0x48 — **PASS**

- fixture: `_scratch_addimm_h52_48.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883c04849898790020000c3`
- js-sha256: `87d6ef901773b519a500a91f6252d58150b3acfa7555c0a934daaaa42a69a1b1`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x61 SUB-IMM slot=0x52 imm=0x40 — **PASS**

- fixture: `_scratch_subimm_h52_40.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e84049898790020000c3`
- js-sha256: `6389a07c533b54d4fcc1d87bda8720d1c10b1abcf2479d9816436e5624ebd1f5`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x80 LDB dd=0x51 ss=0x60 oo=0x70 — **PASS**

- fixture: `_scratch_ldb_5160_70.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c070480fb60049898788020000c3`
- js-sha256: `a36507620f4b048d5f4453fe55ee6766a951709a6cba30dca050399a81745031`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x80 LDB dd=0x52 ss=0x60 oo=0x70 — **PASS**

- fixture: `_scratch_ldb_5260_70.ty` + `.code.hex`
- expected pin (23B): `498b87000300004883c070480fb60049898790020000c3`
- js-sha256: `29dddd3529790413b7ca1825690551d66a12c5630b3604113995fc61b97c3969`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x30 SET slot=0x50 imm=0xC0DEC0DE — **PASS**

- fixture: `_scratch_set_50_c0dec0de.ty` + `.code.hex`
- expected pin (18B): `48b8dec0dec00000000049898780020000c3`
- js-sha256: `b41a84acb6668560c2a0889fe0a5502765fe1b67fa208494af74dc6e8a75779b`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x50 imm=0x50 — **PASS**

- fixture: `_scratch_addimm_h50_50.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883c05049898780020000c3`
- js-sha256: `137444f465f92575826ca9341c5ad44108cc7e01f6179c6ffff4280dfc863df9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x61 SUB-IMM slot=0x51 imm=0x48 — **PASS**

- fixture: `_scratch_subimm_h51_48.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e84849898788020000c3`
- js-sha256: `29980365da8b1f33a028126169fd44c31b9de2f57460e0194cd7271930e3e217`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x51 imm=0x50 — **PASS**

- fixture: `_scratch_addimm_h51_50.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883c05049898788020000c3`
- js-sha256: `c608d7b30f27788517113f3812ba2fbfd8d0a38fe144921406789a2667eccd17`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- ADD-IMM / SUB-IMM triad complete: slot 52 imm=48 ADD, slot 52 imm=40 SUB (H_186..H_189 covered 50/51).
- LDB dd=51/52 ss=60 oo=70 (H_47 is LDB 50 60 70 only; fresh dst).
- SET slot 50 imm=C0DEC0DE (fresh imm not in locked SET set).
- ADD-IMM slot 50/51 imm=50 (fresh imm rung above 48).
- SUB-IMM slot 51 imm=48 (fresh SUB-IMM imm=48; ADD-IMM 48 already locked at 50/51).
- Skipped LDB 50 60 70 (H_47 early). No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

- `yoyo/tests/golden/_scratch_addimm_h52_48.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_40.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5160_70.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_70.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_set_50_c0dec0de.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_50.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_48.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_50.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-27-log.md` — this file
- `scripts/_probe/parallel-batch-27-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-033 serialize PASSes + 1 Relock**

Pass pin from body-extend-032 Relock: `a0cb2642b1b3a3e03be8b82602ae26da1234e8f88170f4c49d836a84caed429d`.
Handlers before consolidate = 196 (H_00..H_189). Next selectors 0xC4.. for H_190.. if all serialize.

PASS list for body-extend-033:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_190 | 0xC4 | 0x62 ADD-IMM | 0x52 0x48 | `498b87900200004883c04849898790020000c3` (19B) | `87d6ef901773b519` |
| H_191 | 0xC5 | 0x61 SUB-IMM | 0x52 0x40 | `498b87900200004883e84049898790020000c3` (19B) | `6389a07c533b54d4` |
| H_192 | 0xC6 | 0x80 LDB | 0x51 0x60 0x70 | `498b87000300004883c070480fb60049898788020000c3` (23B) | `a36507620f4b048d` |
| H_193 | 0xC7 | 0x80 LDB | 0x52 0x60 0x70 | `498b87000300004883c070480fb60049898790020000c3` (23B) | `29dddd3529790413` |
| H_194 | 0xC8 | 0x30 SET | 0x50 0xC0DEC0DE | `48b8dec0dec00000000049898780020000c3` (18B) | `b41a84acb6668560` |
| H_195 | 0xC9 | 0x62 ADD-IMM | 0x50 0x50 | `498b87800200004883c05049898780020000c3` (19B) | `137444f465f92575` |
| H_196 | 0xCA | 0x61 SUB-IMM | 0x51 0x48 | `498b87880200004883e84849898788020000c3` (19B) | `29980365da8b1f33` |
| H_197 | 0xCB | 0x62 ADD-IMM | 0x51 0x50 | `498b87880200004883c05049898788020000c3` (19B) | `c608d7b30f277885` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-032 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_189.
- If the parent decides to serialize, append H_190.. at selectors 0xC4..:
  - H_190 0x62 ADD-IMM (62 52 48) — pin `498b87900200004883c04849898790020000c3`
  - H_191 0x61 SUB-IMM (61 52 40) — pin `498b87900200004883e84049898790020000c3`
  - H_192 0x80 LDB (80 51 60 70) — pin `498b87000300004883c070480fb60049898788020000c3`
  - H_193 0x80 LDB (80 52 60 70) — pin `498b87000300004883c070480fb60049898790020000c3`
  - H_194 0x30 SET (30 50 C0DEC0DE) — pin `48b8dec0dec00000000049898780020000c3`
  - H_195 0x62 ADD-IMM (62 50 50) — pin `498b87800200004883c05049898780020000c3`
  - H_196 0x61 SUB-IMM (61 51 48) — pin `498b87880200004883e84849898788020000c3`
  - H_197 0x62 ADD-IMM (62 51 50) — pin `498b87880200004883c05049898788020000c3`
- Plus 1 Relock after append from pin `a0cb2642…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-033 serialize PASSes + 1 Relock
