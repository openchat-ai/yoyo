# body-extend-033 Log · parallel-batch-27 consolidation (H_190..H_197)

> Tag: `body-extend-033-EXPERIMENTAL-batch27-consolidation-8` · 2026-07-25 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-27-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `a0cb2642b1b3a3e0…` → `0f0fce9a754e2629…`.
> **handler count: 196 → 204** (+8 at selectors 0xC4..0xCB).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_190 | 0xC4 | 0x62 ADD-IMM | 52 48 | 19 | `87d6ef901773b519` |
| H_191 | 0xC5 | 0x61 SUB-IMM | 52 40 | 19 | `6389a07c533b54d4` |
| H_192 | 0xC6 | 0x80 LDB | 51 60 70 | 23 | `a36507620f4b048d` |
| H_193 | 0xC7 | 0x80 LDB | 52 60 70 | 23 | `29dddd3529790413` |
| H_194 | 0xC8 | 0x30 SET | 50 C0DEC0DE | 18 | `b41a84acb6668560` |
| H_195 | 0xC9 | 0x62 ADD-IMM | 50 50 | 19 | `137444f465f92575` |
| H_196 | 0xCA | 0x61 SUB-IMM | 51 48 | 19 | `29980365da8b1f33` |
| H_197 | 0xCB | 0x62 ADD-IMM | 51 50 | 19 | `c608d7b30f277885` |

**REJECTED (not added):** none (batch-27 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selector `40 C4`..`40 CB` for H_190..H_197 are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_190..H_197 at selectors 0xC4..0xCB. Not RAW_BYTE; mirrors H_182..H_189 comment style (body-extend-033 / parallel-batch-27).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h52_48,subimm_h52_40,ldb_5160_70,ldb_5260_70,set_50_c0dec0de,addimm_h50_50,subimm_h51_48,addimm_h51_50}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **188/188 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **196/196 PASS**.
- Full canonical emit: JS=Rust=**4378B** code (was 4219B; +159B); byte-equal **Y**; sha `722d8fafe9782b7b…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `0f0fce9a754e2629…`; previous chained to `a0cb2642b1b3a3e0…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=4608; both peers code=4378). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion.
- No PROMPT edit, version bump, commit, GREEN claim, or trusted encoder modification.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-27 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; no encoder edits.
3. Hand-author: H_190..H_197 at selectors 0xC4..0xCB.
4. Selftest: exact pins PASS (19/19/23/23/18/19/19/19B).
5. Goldens: JS 188/188 and Rust 196/196 PASS; full emit byte-equal Y at 4378B.
6. Lock: Relock once → `0f0fce9a754e2629…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-28: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_190..H_197), writing `parallel-batch-28-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. After batch-28 scratches done: parent next = body-extend-034 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-28-SPAWN.md` (no Task tool on this consolidator).
