# body-extend-098 Log · parallel-batch-92 consolidation (H_709..H_716)

> Tag: `body-extend-098-EXPERIMENTAL-batch92-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-92-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `e6ba7d6cfcbb11da…` → `8d4277255b098dc1…`.
> **handler count: 715 → 723** (+8 at selectors 0x2CB..0x2D2 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_709 | 0x2CB | 0x80 LDB | 52 60 220 | 26 | `3fc747bcdb5a7814` |
| H_710 | 0x2CC | 0x62 ADD-IMM | 50 220 | 22 | `1bbf4fad113bcab7` |
| H_711 | 0x2CD | 0x62 ADD-IMM | 51 220 | 22 | `8504700ade40627c` |
| H_712 | 0x2CE | 0x62 ADD-IMM | 52 220 | 22 | `c0a102f97c62576f` |
| H_713 | 0x2CF | 0x61 SUB-IMM | 50 220 | 22 | `3a44dbe899e12859` |
| H_714 | 0x2D0 | 0x61 SUB-IMM | 51 220 | 22 | `740509fefa4bff85` |
| H_715 | 0x2D1 | 0x61 SUB-IMM | 52 220 | 22 | `2ae2a9625cac581c` |
| H_716 | 0x2D2 | 0x80 LDB | 50 60 228 | 26 | `9e1963a796211cc1` |

**REJECTED (not added):** none (batch-92 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 2CB`..`40 2D2` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_709 finishes 220 LDB triad. H_710..H_712 start/finish 220 ADD triad. H_713..H_715 start/finish 220 SUB triad. H_716 starts 228 LDB ladder (LDB 51/52 228 deferred).

**Deferred (not added this beat):** LDB dd=51/52 ss=60 oo=228; ADD-IMM / SUB-IMM slot=50/51/52 imm=228; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked — suggested for parallel-batch-93.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x220 uses imm32 (`48 81 e8` / `48 81 c0`) → 22B pins; not imm8.
LDB oo=0x220/0x228 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_709 | `3fc747bcdb5a781461fd0348cdb022ece3e3a9661b215e684e0233690b5f8f8d` |
| H_710 | `1bbf4fad113bcab70b19bba185dc4e67008fe55aa83c534a13df23ecf474b482` |
| H_711 | `8504700ade40627c64cde82afa2ac6385f4194d5798cf34258012669602545fb` |
| H_712 | `c0a102f97c62576f4163546150ecd9d07a1dc8ee2c781e1a0cae0c70957e47cb` |
| H_713 | `3a44dbe899e12859ab6ac9679f62b181a83f1321ea5328d991efa218669f239e` |
| H_714 | `740509fefa4bff8502ceff79f94cfdfc9a24681c1941768c4663818ba95c0279` |
| H_715 | `2ae2a9625cac581c393c14716bf4dcbd8f576e8a0c12c7d6b56671ecc70917bb` |
| H_716 | `9e1963a796211cc173505fa2bd3c4864753788fffb324d73944ead0e6682c2f2` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_709..H_716 at selectors 0x2CB..0x2D2 (`40 2CB`..`40 2D2`). Not RAW_BYTE; mirrors H_701..H_708 comment style (body-extend-098 / parallel-batch-92).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5260_220,addimm_h50_220,addimm_h51_220,addimm_h52_220,subimm_h50_220,subimm_h51_220,subimm_h52_220,ldb_5060_228}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **707/707 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **715/715 PASS**.
- Full canonical emit: JS=Rust=**16323B** code (was 16139B; +184B); byte-equal **Y**; sha `fee9f381d9845fb7…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `8d4277255b098dc1…`; previous chained to `e6ba7d6cfcbb11da…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **DIFFER** (compared_bytes=16384; both peers code=16323; hash_a=`2279071b5452acf6…` hash_b=`640ece2057b48d75…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-097 measured EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-92 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_709..H_716 at selectors 0x2CB..0x2D2.
4. Selftest: exact pins PASS (26/22/22/22/22/22/22/26B).
5. Goldens: JS 707/707 and Rust 715/715 PASS; full emit byte-equal Y at 16323B.
6. Lock: Relock once → `8d4277255b098dc1…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` DIFFER this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-93: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_709..H_716), writing `parallel-batch-93-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: LDB 51/52 60 228 (finish 228 LDB triad); ADD-IMM 50/51/52 228 (start 228 ADD triad); SUB-IMM 50/51/52 228 (start 228 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder if continuing. After batch-93 scratches done: parent next = body-extend-099 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-93-SPAWN.md` (no Task tool on this consolidator). Deferred carry: LDB 51/52 228; ADD-IMM/SUB-IMM 50/51/52 228; SET/GET/ORV/SUBV/ADDV/IMUL.
