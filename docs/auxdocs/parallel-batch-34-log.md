# parallel-batch-34 Log · 8-pick fresh-picks scratch test sweep

> Tag: `parallel-batch-34-EXPERIMENTAL-8-pick-scratch` · 2026-07-25 (UTC+8).
> Following body-extend-039 (pin `cc64da68…`, handlers = 252, H_238..H_245 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-039 DDC PE `.text` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (`_scratch_*.ty` + `_scratch_*.code.hex`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_245 and
> not already present as handlers in current `yoyo.ty`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
| 1 | 0x80 LDB | dd=0x51 ss=0x60 oo=0x98 | `498b87000300004881c098000000480fb60049898788020000c3` (26) | same | same | Y | `3c0d495ee6537c54` | `3c0d495ee6537c54` | PASS |
| 2 | 0x80 LDB | dd=0x52 ss=0x60 oo=0x98 | `498b87000300004881c098000000480fb60049898790020000c3` (26) | same | same | Y | `08b6a771b863baeb` | `08b6a771b863baeb` | PASS |
| 3 | 0x61 SUB-IMM | slot=0x50 imm=0x78 | `498b87800200004883e87849898780020000c3` (19) | same | same | Y | `15fb68e82133705a` | `15fb68e82133705a` | PASS |
| 4 | 0x61 SUB-IMM | slot=0x51 imm=0x78 | `498b87880200004883e87849898788020000c3` (19) | same | same | Y | `362f4b6c5b190470` | `362f4b6c5b190470` | PASS |
| 5 | 0x61 SUB-IMM | slot=0x52 imm=0x78 | `498b87900200004883e87849898790020000c3` (19) | same | same | Y | `1d069becb63d59dd` | `1d069becb63d59dd` | PASS |
| 6 | 0x62 ADD-IMM | slot=0x50 imm=0x80 | `498b87800200004881c08000000049898780020000c3` (22) | same | same | Y | `483e67e06faf0c03` | `483e67e06faf0c03` | PASS |
| 7 | 0x62 ADD-IMM | slot=0x51 imm=0x80 | `498b87880200004881c08000000049898788020000c3` (22) | same | same | Y | `3ce4b6b0b760a9ba` | `3ce4b6b0b760a9ba` | PASS |
| 8 | 0x62 ADD-IMM | slot=0x52 imm=0x80 | `498b87900200004881c08000000049898790020000c3` (22) | same | same | Y | `fd2b59647a997f33` | `fd2b59647a997f33` | PASS |

**Summary**: 8 PASS / 0 REJECT out of 8 attempted.

## §1b. Full sha256 per pick

### Pick 1: 0x80 LDB dd=0x51 ss=0x60 oo=0x98 — **PASS**

- fixture: `_scratch_ldb_5160_98.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c098000000480fb60049898788020000c3`
- js-sha256: `3c0d495ee6537c545e0c9883db84a4ef2e7d96f4db95fc3be5f6e7be68cd88c1`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 2: 0x80 LDB dd=0x52 ss=0x60 oo=0x98 — **PASS**

- fixture: `_scratch_ldb_5260_98.ty` + `.code.hex`
- expected pin (26B): `498b87000300004881c098000000480fb60049898790020000c3`
- js-sha256: `08b6a771b863baebe49eb4b818ba6eb40575669b1dc65367b0b7eaaac2abbde8`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 3: 0x61 SUB-IMM slot=0x50 imm=0x78 — **PASS**

- fixture: `_scratch_subimm_h50_78.ty` + `.code.hex`
- expected pin (19B): `498b87800200004883e87849898780020000c3`
- js-sha256: `15fb68e82133705a6ffa5de8b82ee48a2d56df43c045f98334fcadbfd6be15c9`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 4: 0x61 SUB-IMM slot=0x51 imm=0x78 — **PASS**

- fixture: `_scratch_subimm_h51_78.ty` + `.code.hex`
- expected pin (19B): `498b87880200004883e87849898788020000c3`
- js-sha256: `362f4b6c5b190470bdcb4d893d7e3b4e0553a065e237961fa15fdfc1a8f2fe17`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 5: 0x61 SUB-IMM slot=0x52 imm=0x78 — **PASS**

- fixture: `_scratch_subimm_h52_78.ty` + `.code.hex`
- expected pin (19B): `498b87900200004883e87849898790020000c3`
- js-sha256: `1d069becb63d59ddf346ae7eb0d4540bce021cccb0817181c9000993f7ddff04`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 6: 0x62 ADD-IMM slot=0x50 imm=0x80 — **PASS**

- fixture: `_scratch_addimm_h50_80.ty` + `.code.hex`
- expected pin (22B): `498b87800200004881c08000000049898780020000c3`
- js-sha256: `483e67e06faf0c0321f2f9a7a9c0d76d9fda2837f36b63c927a12069ef27ef78`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 7: 0x62 ADD-IMM slot=0x51 imm=0x80 — **PASS**

- fixture: `_scratch_addimm_h51_80.ty` + `.code.hex`
- expected pin (22B): `498b87880200004881c08000000049898788020000c3`
- js-sha256: `3ce4b6b0b760a9ba7c98df6eac28f20c1ef745ea0b3b8fdced62fd9b26f0bac2`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y

### Pick 8: 0x62 ADD-IMM slot=0x52 imm=0x80 — **PASS**

- fixture: `_scratch_addimm_h52_80.ty` + `.code.hex`
- expected pin (22B): `498b87900200004881c08000000049898790020000c3`
- js-sha256: `fd2b59647a997f333a00b14ab0e9497355357b43ece8c23e87763376de7fa27c`
- rust-sha256: `same`
- byte-eq JS↔Rust↔expected: Y


## §2. Pick rationale

- LDB dd=51/52 ss=60 oo=98 (complete oo=98 triad; imm32 26B; H_245 already has dd=50).
- SUB-IMM slot 50/51/52 imm=78 (fresh SUB imm=78 triad; complements locked ADD-IMM * 78).
- ADD-IMM slot 50/51/52 imm=80 (fresh ADD imm=0x80 triad; 0x80>127 → imm32 `48 81 c0` → 22B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Note: H_250.. need selectors past `40 FF` (current label map is `args[0]&0xff`); consolidator must widen or otherwise assign without colliding H_00..

## §3. Files touched

- `yoyo/tests/golden/_scratch_ldb_5160_98.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_ldb_5260_98.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h50_78.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h51_78.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_subimm_h52_78.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h50_80.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h51_80.ty` + `.code.hex`
- `yoyo/tests/golden/_scratch_addimm_h52_80.ty` + `.code.hex`
- `docs/auxdocs/parallel-batch-34-log.md` — this file
- `scripts/_probe/parallel-batch-34-run.mjs` — probe runner

NO `yoyo/projects/yoyo.ty`, NO lock, NO `golden.js`, NO `self_test.rs`, NO `main.rs`.

## §4. Parent next

**parent next = body-extend-040 serialize PASSes + 1 Relock**

Pass pin from body-extend-039 Relock: `cc64da680d967e6b17ebc9767a74dd670251df82f614d0c72df1240126c05642`.
Handlers before consolidate = 252 (H_00..H_245). Next selectors 0xFC.. for H_246.. if all serialize.
(0xFF RET is opcode namespace only — HANDLER labels OK. H_250.. past `40 FF` need label-width widen — see SPAWN.)

PASS list for body-extend-040:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_246 | 0xFC | 0x80 LDB | 0x51 0x60 0x98 | `498b87000300004881c098000000480fb60049898788020000c3` (26B) | `3c0d495ee6537c54` |
| H_247 | 0xFD | 0x80 LDB | 0x52 0x60 0x98 | `498b87000300004881c098000000480fb60049898790020000c3` (26B) | `08b6a771b863baeb` |
| H_248 | 0xFE | 0x61 SUB-IMM | 0x50 0x78 | `498b87800200004883e87849898780020000c3` (19B) | `15fb68e82133705a` |
| H_249 | 0xFF | 0x61 SUB-IMM | 0x51 0x78 | `498b87880200004883e87849898788020000c3` (19B) | `362f4b6c5b190470` |
| H_250 | 0x100 | 0x61 SUB-IMM | 0x52 0x78 | `498b87900200004883e87849898790020000c3` (19B) | `1d069becb63d59dd` |
| H_251 | 0x101 | 0x62 ADD-IMM | 0x50 0x80 | `498b87800200004881c08000000049898780020000c3` (22B) | `483e67e06faf0c03` |
| H_252 | 0x102 | 0x62 ADD-IMM | 0x51 0x80 | `498b87880200004881c08000000049898788020000c3` (22B) | `3ce4b6b0b760a9ba` |
| H_253 | 0x103 | 0x62 ADD-IMM | 0x52 0x80 | `498b87900200004881c08000000049898790020000c3` (22B) | `fd2b59647a997f33` |

## §5. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No `*.lock` touch.
- No git commit (W-START convention).
- No `yoyo.ty` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-039 DDC PE `.text` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The 8 PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_245.
- If the parent decides to serialize, append H_246.. at selectors 0xFC..:
  - H_246 0x80 LDB (80 51 60 98) — pin `498b87000300004881c098000000480fb60049898788020000c3`
  - H_247 0x80 LDB (80 52 60 98) — pin `498b87000300004881c098000000480fb60049898790020000c3`
  - H_248 0x61 SUB-IMM (61 50 78) — pin `498b87800200004883e87849898780020000c3`
  - H_249 0x61 SUB-IMM (61 51 78) — pin `498b87880200004883e87849898788020000c3`
  - H_250 0x61 SUB-IMM (61 52 78) — pin `498b87900200004883e87849898790020000c3`
  - H_251 0x62 ADD-IMM (62 50 80) — pin `498b87800200004881c08000000049898780020000c3`
  - H_252 0x62 ADD-IMM (62 51 80) — pin `498b87880200004881c08000000049898788020000c3`
  - H_253 0x62 ADD-IMM (62 52 80) — pin `498b87900200004881c08000000049898790020000c3`
- Plus 1 Relock after append from pin `cc64da68…`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-040 serialize PASSes + 1 Relock
