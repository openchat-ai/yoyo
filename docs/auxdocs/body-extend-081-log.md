# body-extend-081 Log · parallel-batch-75 consolidation (H_573..H_580)

> Tag: `body-extend-081-EXPERIMENTAL-batch75-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-75-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `e255cd93a26ec455…` → `267c611dbb648db1…`.
> **handler count: 579 → 587** (+8 at selectors 0x243..0x24A via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_573 | 0x243 | 0x80 LDB | 51 60 1A8 | 26 | `fbea55b03005c5a5` |
| H_574 | 0x244 | 0x80 LDB | 52 60 1A8 | 26 | `7db0bd86b3e802a1` |
| H_575 | 0x245 | 0x62 ADD-IMM | 50 1A8 | 22 | `5a3272ce14feca9a` |
| H_576 | 0x246 | 0x62 ADD-IMM | 51 1A8 | 22 | `6aecaccb918f42df` |
| H_577 | 0x247 | 0x62 ADD-IMM | 52 1A8 | 22 | `f2ea24f19b1f387c` |
| H_578 | 0x248 | 0x61 SUB-IMM | 50 1A8 | 22 | `2a655dd4d2adee0c` |
| H_579 | 0x249 | 0x61 SUB-IMM | 51 1A8 | 22 | `44c2fed0d54d8b28` |
| H_580 | 0x24A | 0x61 SUB-IMM | 52 1A8 | 22 | `2c5130704cf19491` |

**REJECTED (not added):** none (batch-75 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 243`..`40 24A` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_573/H_574 finish 1A8 LDB triad. H_575..H_577 start/finish 1A8 ADD triad. H_578..H_580 start/finish 1A8 SUB triad.

**Deferred (not added this beat):** LDB 50/51/52 60 1B0; ADD-IMM/SUB-IMM slot=50/51/52 imm=1B0 — suggested for parallel-batch-76.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1A8 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1A8 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_573..H_580 at selectors 0x243..0x24A (`40 243`..`40 24A`). Not RAW_BYTE; mirrors H_565..H_572 comment style (body-extend-081 / parallel-batch-75).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5160_1A8,ldb_5260_1A8,addimm_h50_1A8,addimm_h51_1A8,addimm_h52_1A8,subimm_h50_1A8,subimm_h51_1A8,subimm_h52_1A8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **571/571 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **579/579 PASS**.
- Full canonical emit: JS=Rust=**13147B** code (was 12963B; +184B); byte-equal **Y**; sha `6e6d22e2a68b78c2…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `267c611dbb648db1…`; previous chained to `e255cd93a26ec455…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=13312; both peers code=13147; hash_a=hash_b=`b37788f93d0dc516…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-080 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-75 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_573..H_580 at selectors 0x243..0x24A.
4. Selftest: exact pins PASS (26/26/22/22/22/22/22/22B).
5. Goldens: JS 571/571 and Rust 579/579 PASS; full emit byte-equal Y at 13147B.
6. Lock: Relock once → `267c611dbb648db1…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-76: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_573..H_580), writing `parallel-batch-76-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: start LDB 50/51/52 60 1B0, then ADD-IMM/SUB-IMM imm=1B0 triad, SET/GET fresh, etc. After batch-76 scratches done: parent next = body-extend-082 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-76-SPAWN.md` (no Task tool on this consolidator). Deferred carry: LDB 50/51/52 60 1B0; ADD-IMM/SUB-IMM slot=50/51/52 imm=1B0.
