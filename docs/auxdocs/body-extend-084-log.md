# body-extend-084 Log · parallel-batch-78 consolidation (H_597..H_604)

> Tag: `body-extend-084-EXPERIMENTAL-batch78-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-78-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `45dff031e2acfa0e…` → `9eafc9ce0376d389…`.
> **handler count: 603 → 611** (+8 at selectors 0x25B..0x262 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_597 | 0x25B | 0x61 SUB-IMM | 51 1B8 | 22 | `e5f22d8e3828fbe4` |
| H_598 | 0x25C | 0x61 SUB-IMM | 52 1B8 | 22 | `fb3630917fc37295` |
| H_599 | 0x25D | 0x80 LDB | 50 60 1C0 | 26 | `8953358138eb317e` |
| H_600 | 0x25E | 0x80 LDB | 51 60 1C0 | 26 | `1759a1345d7af7ee` |
| H_601 | 0x25F | 0x80 LDB | 52 60 1C0 | 26 | `bc6894d42acc6084` |
| H_602 | 0x260 | 0x62 ADD-IMM | 50 1C0 | 22 | `f6926af2f6dc5e89` |
| H_603 | 0x261 | 0x62 ADD-IMM | 51 1C0 | 22 | `8b90b51a7b7d5e6d` |
| H_604 | 0x262 | 0x62 ADD-IMM | 52 1C0 | 22 | `6c82474ed68d4ac8` |

**REJECTED (not added):** none (batch-78 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 25B`..`40 262` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_597/H_598 finish 1B8 SUB triad (H_596 was slot=50). H_599..H_601 start/finish 1C0 LDB triad. H_602..H_604 start/finish 1C0 ADD triad.

**Deferred (not added this beat):** SUB-IMM slot=50/51/52 imm=1C0; LDB 50/51/52 60 1C8; ADD-IMM/SUB-IMM slot=50/51/52 imm=1C8 — suggested for parallel-batch-79.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1C0 / 0x1B8 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1C0 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_597..H_604 at selectors 0x25B..0x262 (`40 25B`..`40 262`). Not RAW_BYTE; mirrors H_589..H_596 comment style (body-extend-084 / parallel-batch-78).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h51_1B8,subimm_h52_1B8,ldb_5060_1C0,ldb_5160_1C0,ldb_5260_1C0,addimm_h50_1C0,addimm_h51_1C0,addimm_h52_1C0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **595/595 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **603/603 PASS**.
- Full canonical emit: JS=Rust=**13711B** code (was 13523B; +188B); byte-equal **Y**; sha `07db208333971457…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `9eafc9ce0376d389…`; previous chained to `45dff031e2acfa0e…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=13824; both peers code=13711; hash_a=hash_b=`fc8105b741f93c68…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-083 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-78 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_597..H_604 at selectors 0x25B..0x262.
4. Selftest: exact pins PASS (22/22/26/26/26/22/22/22B).
5. Goldens: JS 595/595 and Rust 603/603 PASS; full emit byte-equal Y at 13711B.
6. Lock: Relock once → `9eafc9ce0376d389…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-79: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_597..H_604), writing `parallel-batch-79-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: start SUB-IMM 50/51/52 1C0, then LDB 50/51/52 60 1C8, ADD-IMM/SUB-IMM imm=1C8 triad, SET/GET fresh, etc. After batch-79 scratches done: parent next = body-extend-085 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-79-SPAWN.md` (no Task tool on this consolidator). Deferred carry: SUB-IMM 50/51/52 1C0; LDB 50/51/52 60 1C8; ADD-IMM/SUB-IMM slot=50/51/52 imm=1C8.
