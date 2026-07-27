# body-extend-099 Log · parallel-batch-93 consolidation (H_717..H_724)

> Tag: `body-extend-099-EXPERIMENTAL-batch93-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-93-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `8d4277255b098dc1…` → `3fc618f9e14a881a…`.
> **handler count: 723 → 731** (+8 at selectors 0x2D3..0x2DA via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_717 | 0x2D3 | 0x80 LDB | 51 60 228 | 26 | `ec662f4d79ff8add` |
| H_718 | 0x2D4 | 0x80 LDB | 52 60 228 | 26 | `0a14cf8c72933615` |
| H_719 | 0x2D5 | 0x62 ADD-IMM | 50 228 | 22 | `308359b06a3c0b71` |
| H_720 | 0x2D6 | 0x62 ADD-IMM | 51 228 | 22 | `30a3548d2b182ab8` |
| H_721 | 0x2D7 | 0x62 ADD-IMM | 52 228 | 22 | `bb5db527c469beec` |
| H_722 | 0x2D8 | 0x61 SUB-IMM | 50 228 | 22 | `f21787f68d23f722` |
| H_723 | 0x2D9 | 0x61 SUB-IMM | 51 228 | 22 | `b4edd744e6cbfd23` |
| H_724 | 0x2DA | 0x61 SUB-IMM | 52 228 | 22 | `a64562f9de393830` |

**REJECTED (not added):** none (batch-93 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 2D3`..`40 2DA` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_717/H_718 finish 228 LDB triad. H_719..H_721 start/finish 228 ADD triad. H_722..H_724 start/finish 228 SUB triad.

**Deferred (not added this beat):** LDB dd=50/51/52 ss=60 oo=230; ADD-IMM / SUB-IMM slot=50/51/52 imm=230; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked — suggested for parallel-batch-94.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x228 uses imm32 (`48 81 e8` / `48 81 c0`) → 22B pins; not imm8.
LDB oo=0x228 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_717 | `ec662f4d79ff8add66c6b0606f5a408b3c485d2361c271118c0eef2d41ed60d3` |
| H_718 | `0a14cf8c7293361575919dd2df7f3ffb4bdb7fa27f1ef29919b7a2b4a3ba149e` |
| H_719 | `308359b06a3c0b715e6575564a5adf581bb2bab054dee4b61ba0b5ab4d8c52d8` |
| H_720 | `30a3548d2b182ab87c5fdd862f32dd467b0bf1c4078df243023e90e6c0c0a874` |
| H_721 | `bb5db527c469beeca3feb4b57ce15971c13ba3a1916646a503c65e9042a608ff` |
| H_722 | `f21787f68d23f722623c13531402795368893b64a61a65a658b8953e26320347` |
| H_723 | `b4edd744e6cbfd23b6f73bc312697cbee7bce3125f544060286caeeccd04cd57` |
| H_724 | `a64562f9de393830d164b6493bb727b27106b09e043cc597c4f75ff11ecababd` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_717..H_724 at selectors 0x2D3..0x2DA (`40 2D3`..`40 2DA`). Not RAW_BYTE; mirrors H_709..H_716 comment style (body-extend-099 / parallel-batch-93).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5160_228,ldb_5260_228,addimm_h50_228,addimm_h51_228,addimm_h52_228,subimm_h50_228,subimm_h51_228,subimm_h52_228}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **715/715 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **723/723 PASS**.
- Full canonical emit: JS=Rust=**16507B** code (was 16323B; +184B); byte-equal **Y**; sha `88f02983c96e43cf…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `3fc618f9e14a881a…`; previous chained to `8d4277255b098dc1…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=16896; both peers code=16507; hash_a=hash_b=`92ac5309cb1e1b58…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-098 measured DIFFER — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-93 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_717..H_724 at selectors 0x2D3..0x2DA.
4. Selftest: exact pins PASS (26/26/22/22/22/22/22/22B).
5. Goldens: JS 715/715 and Rust 723/723 PASS; full emit byte-equal Y at 16507B.
6. Lock: Relock once → `3fc618f9e14a881a…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-94: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_717..H_724), writing `parallel-batch-94-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB 50/51/52 60 230 (start 230 LDB ladder); ADD-IMM 50/51/52 230 (start 230 ADD triad); SUB-IMM 50/51/52 230 (start 230 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder if continuing. After batch-94 scratches done: parent next = body-extend-100 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-94-SPAWN.md` (no Task tool on this consolidator). Deferred carry: LDB/ADD-IMM/SUB-IMM imm=230; SET/GET/ORV/SUBV/ADDV/IMUL.
