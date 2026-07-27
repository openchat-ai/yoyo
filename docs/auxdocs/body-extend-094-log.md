# body-extend-094 Log · parallel-batch-88 consolidation (H_677..H_684)

> Tag: `body-extend-094-EXPERIMENTAL-batch88-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-88-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `04656bbbbb152b54…` → `0ef9611b50021d82…`.
> **handler count: 683 → 691** (+8 at selectors 0x2AB..0x2B2 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_677 | 0x2AB | 0x61 SUB-IMM | 50 200 | 22 | `616e435fa3303d6d` |
| H_678 | 0x2AC | 0x61 SUB-IMM | 51 200 | 22 | `c68ac43f8d46d532` |
| H_679 | 0x2AD | 0x61 SUB-IMM | 52 200 | 22 | `aa5d87726f97aedf` |
| H_680 | 0x2AE | 0x80 LDB | 50 60 208 | 26 | `454561f22b4cd018` |
| H_681 | 0x2AF | 0x80 LDB | 51 60 208 | 26 | `4d6d099ee46ef004` |
| H_682 | 0x2B0 | 0x80 LDB | 52 60 208 | 26 | `49ede9483394add3` |
| H_683 | 0x2B1 | 0x62 ADD-IMM | 50 208 | 22 | `20c12c152bbba594` |
| H_684 | 0x2B2 | 0x62 ADD-IMM | 51 208 | 22 | `612703982c8eadbb` |

**REJECTED (not added):** none (batch-88 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 2AB`..`40 2B2` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_677..H_679 finish 200 SUB triad. H_680..H_682 start/finish 208 LDB triad. H_683/H_684 start 208 ADD triad (ADD 52 deferred).

**Deferred (not added this beat):** ADD-IMM slot=52 imm=208; SUB-IMM slot=50/51/52 imm=208; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked — suggested for parallel-batch-89.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x200/0x208 uses imm32 (`48 81 e8` / `48 81 c0`) → 22B pins; not imm8.
LDB oo=0x208 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_677 | `616e435fa3303d6d6ba0710790f2689e99f6628131c1c135d4560e43a12ce990` |
| H_678 | `c68ac43f8d46d532ba9c6f4d1d060cc3b879145e6a2e6770f015e6145d763379` |
| H_679 | `aa5d87726f97aedfbd932d90d43047686e049ef1ac3a86a8492b02739b852c73` |
| H_680 | `454561f22b4cd018ef79befab6dd2911e4dfb00566eeabd5111866aea8ff8895` |
| H_681 | `4d6d099ee46ef0045a4eb3e81c5b58b73a4a8ad82b907f9192f1368a33112139` |
| H_682 | `49ede9483394add3545ffd850a337cf4e2a608953a0b1db5a7bbce046b8ea331` |
| H_683 | `20c12c152bbba59406e5c82303bc3ccd3ddc945fdf57984d2226d57c16426da0` |
| H_684 | `612703982c8eadbb83922b686ef84d5dd929cde146cc8e77328b01241092d313` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_677..H_684 at selectors 0x2AB..0x2B2 (`40 2AB`..`40 2B2`). Not RAW_BYTE; mirrors H_669..H_676 comment style (body-extend-094 / parallel-batch-88).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h50_200,subimm_h51_200,subimm_h52_200,ldb_5060_208,ldb_5160_208,ldb_5260_208,addimm_h50_208,addimm_h51_208}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **675/675 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **683/683 PASS**.
- Full canonical emit: JS=Rust=**15579B** code (was 15391B; +188B); byte-equal **Y**; sha `dff16552cf3fae56…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `0ef9611b50021d82…`; previous chained to `04656bbbbb152b54…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=15872; both peers code=15579; hash_a=hash_b=`4395a342e58e7534…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-093 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-88 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_677..H_684 at selectors 0x2AB..0x2B2.
4. Selftest: exact pins PASS (22/22/22/26/26/26/22/22B).
5. Goldens: JS 675/675 and Rust 683/683 PASS; full emit byte-equal Y at 15579B.
6. Lock: Relock once → `0ef9611b50021d82…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-89: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_677..H_684), writing `parallel-batch-89-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: ADD-IMM 52 208 (finish 208 ADD triad); SUB-IMM 50/51/52 208 (start 208 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder if continuing. After batch-89 scratches done: parent next = body-extend-095 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-89-SPAWN.md` (no Task tool on this consolidator). Deferred carry: ADD-IMM 52 208; SUB-IMM 50/51/52 208; SET/GET/ORV/SUBV/ADDV/IMUL.
