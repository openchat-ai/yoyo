# body-extend-097 Log · parallel-batch-91 consolidation (H_701..H_708)

> Tag: `body-extend-097-EXPERIMENTAL-batch91-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-91-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `0a02f49ed0c94a2d…` → `e6ba7d6cfcbb11da…`.
> **handler count: 707 → 715** (+8 at selectors 0x2C3..0x2CA via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_701 | 0x2C3 | 0x62 ADD-IMM | 50 218 | 22 | `4ab4f6b000bfc170` |
| H_702 | 0x2C4 | 0x62 ADD-IMM | 51 218 | 22 | `f2a5fff94e8993ce` |
| H_703 | 0x2C5 | 0x62 ADD-IMM | 52 218 | 22 | `b8f58bcc6a95b935` |
| H_704 | 0x2C6 | 0x61 SUB-IMM | 50 218 | 22 | `d98dffc59593a3e5` |
| H_705 | 0x2C7 | 0x61 SUB-IMM | 51 218 | 22 | `a2df94a8e97fec79` |
| H_706 | 0x2C8 | 0x61 SUB-IMM | 52 218 | 22 | `501f0c3b69e446a3` |
| H_707 | 0x2C9 | 0x80 LDB | 50 60 220 | 26 | `38dd8dd1ab3ef61c` |
| H_708 | 0x2CA | 0x80 LDB | 51 60 220 | 26 | `6633a1f5ac21e65f` |

**REJECTED (not added):** none (batch-91 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 2C3`..`40 2CA` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_701..H_703 start/finish 218 ADD triad. H_704..H_706 start/finish 218 SUB triad. H_707/H_708 start 220 LDB ladder (LDB 52 220 deferred).

**Deferred (not added this beat):** LDB dd=52 ss=60 oo=220; ADD-IMM slot=50/51/52 imm=220; SUB-IMM slot=50/51/52 imm=220; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked — suggested for parallel-batch-92.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x218 uses imm32 (`48 81 e8` / `48 81 c0`) → 22B pins; not imm8.
LDB oo=0x220 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_701 | `4ab4f6b000bfc170429e5110542aa259ae4323684ecd0111ced31957ca2ed16a` |
| H_702 | `f2a5fff94e8993ce3881609c5b31db4ca87a058fc1737abeae2dacfa6c0e6eea` |
| H_703 | `b8f58bcc6a95b93566f9489f12aa546360d390c7233847963b448dffbc6e5986` |
| H_704 | `d98dffc59593a3e5a92ed2b70d52f837b419c60bf8178c3e645177aa43293294` |
| H_705 | `a2df94a8e97fec797174a0f4370ecf60ca0d2085543ee3c4795b586dbc22bcf3` |
| H_706 | `501f0c3b69e446a3f382e8f726884e6348c763eb6e365ef6dc05e7097aeff88a` |
| H_707 | `38dd8dd1ab3ef61c0576bb5f5c590a0e710f7996c93657f9f915f2546903cdd0` |
| H_708 | `6633a1f5ac21e65f54355ae582c8f2364ba596270e71b1d3f28c2d6cd7bf6503` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_701..H_708 at selectors 0x2C3..0x2CA (`40 2C3`..`40 2CA`). Not RAW_BYTE; mirrors H_693..H_700 comment style (body-extend-097 / parallel-batch-91).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h50_218,addimm_h51_218,addimm_h52_218,subimm_h50_218,subimm_h51_218,subimm_h52_218,ldb_5060_220,ldb_5160_220}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **699/699 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **707/707 PASS**.
- Full canonical emit: JS=Rust=**16139B** code (was 15955B; +184B); byte-equal **Y**; sha `2a5a25c1ed9eaf1e…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `e6ba7d6cfcbb11da…`; previous chained to `0a02f49ed0c94a2d…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=16384; both peers code=16139; hash_a=hash_b=`ae74dd32e5940a33…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-096 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-91 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_701..H_708 at selectors 0x2C3..0x2CA.
4. Selftest: exact pins PASS (22/22/22/22/22/22/26/26B).
5. Goldens: JS 699/699 and Rust 707/707 PASS; full emit byte-equal Y at 16139B.
6. Lock: Relock once → `e6ba7d6cfcbb11da…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-92: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_701..H_708), writing `parallel-batch-92-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB 52 60 220 (finish 220 LDB triad); ADD-IMM 50/51/52 220 (start 220 ADD triad); SUB-IMM 50/51/52 220 (start 220 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder if continuing. After batch-92 scratches done: parent next = body-extend-098 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-92-SPAWN.md` (no Task tool on this consolidator). Deferred carry: LDB 52 220; ADD-IMM/SUB-IMM 50/51/52 220; SET/GET/ORV/SUBV/ADDV/IMUL.
