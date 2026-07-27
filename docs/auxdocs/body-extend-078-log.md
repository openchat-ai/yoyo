# body-extend-078 Log · parallel-batch-72 consolidation (H_549..H_556)

> Tag: `body-extend-078-EXPERIMENTAL-batch72-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-72-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `97ce84a29adb8c40…` → `4c42576df4f80a8d…`.
> **handler count: 555 → 563** (+8 at selectors 0x22B..0x232 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_549 | 0x22B | 0x62 ADD-IMM | 51 190 | 22 | `5248421affee5c66` |
| H_550 | 0x22C | 0x62 ADD-IMM | 52 190 | 22 | `648351f8db48af34` |
| H_551 | 0x22D | 0x61 SUB-IMM | 50 190 | 22 | `f7e06d035b717d9d` |
| H_552 | 0x22E | 0x61 SUB-IMM | 51 190 | 22 | `489b9cd85b80cad9` |
| H_553 | 0x22F | 0x61 SUB-IMM | 52 190 | 22 | `0535305934d986e2` |
| H_554 | 0x230 | 0x80 LDB | 50 60 198 | 26 | `f68f3fdd889f57db` |
| H_555 | 0x231 | 0x80 LDB | 51 60 198 | 26 | `1fd1cefc37ee2f6a` |
| H_556 | 0x232 | 0x80 LDB | 52 60 198 | 26 | `84e2d29d21835c65` |

**REJECTED (not added):** none (batch-72 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 22B`..`40 232` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_549/H_550 finish 190 ADD triad. H_551..H_553 finish 190 SUB triad. H_554..H_556 start 198 LDB triad.

**Deferred (not added this beat):** ADD-IMM/SUB-IMM slot=50/51/52 imm=198 — suggested for parallel-batch-73.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x190 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x198 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_549..H_556 at selectors 0x22B..0x232 (`40 22B`..`40 232`). Not RAW_BYTE; mirrors H_541..H_548 comment style (body-extend-078 / parallel-batch-72).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h51_190,addimm_h52_190,subimm_h50_190,subimm_h51_190,subimm_h52_190,ldb_5060_198,ldb_5160_198,ldb_5260_198}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **547/547 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **555/555 PASS**.
- Full canonical emit: JS=Rust=**12595B** code (was 12407B; +188B); byte-equal **Y**; sha `07d2c2e7dd9c85ec…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `4c42576df4f80a8d…`; previous chained to `97ce84a29adb8c40…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=12800; both peers code=12595; hash_a=hash_b=`7c73b624210e67d6…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-077 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-72 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_549..H_556 at selectors 0x22B..0x232.
4. Selftest: exact pins PASS (22/22/22/22/22/26/26/26B).
5. Goldens: JS 547/547 and Rust 555/555 PASS; full emit byte-equal Y at 12595B.
6. Lock: Relock once → `4c42576df4f80a8d…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-73: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_549..H_556), writing `parallel-batch-73-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: ADD-IMM/SUB-IMM imm=198 triad (still open), then LDB/ADD oo=1A0 ladder, SET/GET fresh, etc. After batch-73 scratches done: parent next = body-extend-079 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-73-SPAWN.md` (no Task tool on this consolidator). Deferred carry: ADD-IMM/SUB-IMM slot=50/51/52 imm=198.
