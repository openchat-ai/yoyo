# body-extend-079 Log · parallel-batch-73 consolidation (H_557..H_564)

> Tag: `body-extend-079-EXPERIMENTAL-batch73-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-73-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `4c42576df4f80a8d…` → `0e5b612c7e4882a1…`.
> **handler count: 563 → 571** (+8 at selectors 0x233..0x23A via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_557 | 0x233 | 0x62 ADD-IMM | 50 198 | 22 | `b9a1454084d99711` |
| H_558 | 0x234 | 0x62 ADD-IMM | 51 198 | 22 | `6dfea21cc077f979` |
| H_559 | 0x235 | 0x62 ADD-IMM | 52 198 | 22 | `b4bced2f75175884` |
| H_560 | 0x236 | 0x61 SUB-IMM | 50 198 | 22 | `7dca7636d1845a95` |
| H_561 | 0x237 | 0x61 SUB-IMM | 51 198 | 22 | `5b1facdbbae86c25` |
| H_562 | 0x238 | 0x61 SUB-IMM | 52 198 | 22 | `3b46829def05556b` |
| H_563 | 0x239 | 0x80 LDB | 50 60 1A0 | 26 | `bcf7781865161f65` |
| H_564 | 0x23A | 0x80 LDB | 51 60 1A0 | 26 | `55cd34d122a07524` |

**REJECTED (not added):** none (batch-73 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 233`..`40 23A` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_557..H_559 finish/start 198 ADD triad. H_560..H_562 finish 198 SUB triad. H_563/H_564 start 1A0 LDB triad (dd=52 deferred).

**Deferred (not added this beat):** LDB 52 60 1A0; ADD-IMM/SUB-IMM slot=50/51/52 imm=1A0 — suggested for parallel-batch-74.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x198 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1A0 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_557..H_564 at selectors 0x233..0x23A (`40 233`..`40 23A`). Not RAW_BYTE; mirrors H_549..H_556 comment style (body-extend-079 / parallel-batch-73).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h50_198,addimm_h51_198,addimm_h52_198,subimm_h50_198,subimm_h51_198,subimm_h52_198,ldb_5060_1A0,ldb_5160_1A0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **555/555 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **563/563 PASS**.
- Full canonical emit: JS=Rust=**12779B** code (was 12595B; +184B); byte-equal **Y**; sha `357afd268bdbacb5…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `0e5b612c7e4882a1…`; previous chained to `4c42576df4f80a8d…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=12800; both peers code=12779; hash_a=hash_b=`d0c490907b40fe20…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-078 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-73 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_557..H_564 at selectors 0x233..0x23A.
4. Selftest: exact pins PASS (22/22/22/22/22/22/26/26B).
5. Goldens: JS 555/555 and Rust 563/563 PASS; full emit byte-equal Y at 12779B.
6. Lock: Relock once → `0e5b612c7e4882a1…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-74: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_557..H_564), writing `parallel-batch-74-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish LDB 52 60 1A0, then ADD-IMM/SUB-IMM imm=1A0 triad, SET/GET fresh, etc. After batch-74 scratches done: parent next = body-extend-080 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-74-SPAWN.md` (no Task tool on this consolidator). Deferred carry: LDB 52 60 1A0; ADD-IMM/SUB-IMM slot=50/51/52 imm=1A0.
