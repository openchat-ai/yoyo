# body-extend-080 Log · parallel-batch-74 consolidation (H_565..H_572)

> Tag: `body-extend-080-EXPERIMENTAL-batch74-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-74-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `0e5b612c7e4882a1…` → `e255cd93a26ec455…`.
> **handler count: 571 → 579** (+8 at selectors 0x23B..0x242 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_565 | 0x23B | 0x80 LDB | 52 60 1A0 | 26 | `5492824be268600b` |
| H_566 | 0x23C | 0x62 ADD-IMM | 50 1A0 | 22 | `d6c054ff35b9b724` |
| H_567 | 0x23D | 0x62 ADD-IMM | 51 1A0 | 22 | `5db8f3bc0d22ae9b` |
| H_568 | 0x23E | 0x62 ADD-IMM | 52 1A0 | 22 | `88b0244979ff3341` |
| H_569 | 0x23F | 0x61 SUB-IMM | 50 1A0 | 22 | `f7a21b3a8775eaaa` |
| H_570 | 0x240 | 0x61 SUB-IMM | 51 1A0 | 22 | `47d4190d9e3f6f16` |
| H_571 | 0x241 | 0x61 SUB-IMM | 52 1A0 | 22 | `131a705e499f8031` |
| H_572 | 0x242 | 0x80 LDB | 50 60 1A8 | 26 | `c7b2148d29e6d1e4` |

**REJECTED (not added):** none (batch-74 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 23B`..`40 242` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_565 finishes 1A0 LDB triad. H_566..H_568 finish 1A0 ADD triad. H_569..H_571 finish 1A0 SUB triad. H_572 starts 1A8 LDB triad (dd=51/52 deferred).

**Deferred (not added this beat):** LDB 51/52 60 1A8; ADD-IMM/SUB-IMM slot=50/51/52 imm=1A8 — suggested for parallel-batch-75.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1A0 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1A0 / 0x1A8 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_565..H_572 at selectors 0x23B..0x242 (`40 23B`..`40 242`). Not RAW_BYTE; mirrors H_557..H_564 comment style (body-extend-080 / parallel-batch-74).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5260_1A0,addimm_h50_1A0,addimm_h51_1A0,addimm_h52_1A0,subimm_h50_1A0,subimm_h51_1A0,subimm_h52_1A0,ldb_5060_1A8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **563/563 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **571/571 PASS**.
- Full canonical emit: JS=Rust=**12963B** code (was 12779B; +184B); byte-equal **Y**; sha `77742f96e391071d…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `e255cd93a26ec455…`; previous chained to `0e5b612c7e4882a1…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=13312; both peers code=12963; hash_a=hash_b=`48a1770369491cdb…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-079 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-74 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_565..H_572 at selectors 0x23B..0x242.
4. Selftest: exact pins PASS (26/22/22/22/22/22/22/26B).
5. Goldens: JS 563/563 and Rust 571/571 PASS; full emit byte-equal Y at 12963B.
6. Lock: Relock once → `e255cd93a26ec455…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-75: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_565..H_572), writing `parallel-batch-75-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish LDB 51/52 60 1A8, then ADD-IMM/SUB-IMM imm=1A8 triad, SET/GET fresh, etc. After batch-75 scratches done: parent next = body-extend-081 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-75-SPAWN.md` (no Task tool on this consolidator). Deferred carry: LDB 51/52 60 1A8; ADD-IMM/SUB-IMM slot=50/51/52 imm=1A8.
