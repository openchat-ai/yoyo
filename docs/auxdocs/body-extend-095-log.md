# body-extend-095 Log · parallel-batch-89 consolidation (H_685..H_692)

> Tag: `body-extend-095-EXPERIMENTAL-batch89-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-89-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `0ef9611b50021d82…` → `aef6d89f98ceb7c8…`.
> **handler count: 691 → 699** (+8 at selectors 0x2B3..0x2BA via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_685 | 0x2B3 | 0x62 ADD-IMM | 52 208 | 22 | `bb7306a6accdaf1d` |
| H_686 | 0x2B4 | 0x61 SUB-IMM | 50 208 | 22 | `f7711234e1f246db` |
| H_687 | 0x2B5 | 0x61 SUB-IMM | 51 208 | 22 | `71f14163af6727da` |
| H_688 | 0x2B6 | 0x61 SUB-IMM | 52 208 | 22 | `b95b3672e4031732` |
| H_689 | 0x2B7 | 0x80 LDB | 50 60 210 | 26 | `e5d730581fb17e84` |
| H_690 | 0x2B8 | 0x80 LDB | 51 60 210 | 26 | `ebbb4b6905b61aa1` |
| H_691 | 0x2B9 | 0x80 LDB | 52 60 210 | 26 | `62a53f91d97addee` |
| H_692 | 0x2BA | 0x62 ADD-IMM | 50 210 | 22 | `b28afba882b0e6c1` |

**REJECTED (not added):** none (batch-89 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 2B3`..`40 2BA` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_685 finishes 208 ADD triad. H_686..H_688 start/finish 208 SUB triad. H_689..H_691 start/finish 210 LDB triad. H_692 starts 210 ADD triad (ADD 51/52 deferred).

**Deferred (not added this beat):** ADD-IMM slot=51/52 imm=210; SUB-IMM slot=50/51/52 imm=210; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked — suggested for parallel-batch-90.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x208/0x210 uses imm32 (`48 81 e8` / `48 81 c0`) → 22B pins; not imm8.
LDB oo=0x210 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_685 | `bb7306a6accdaf1dd37bfa4c5811e5bed548e84ddf0e5182504d0a09bce5d0e2` |
| H_686 | `f7711234e1f246db5dde5d4b6bbd3c11b32f6a77fe2f884fd2dfcb6249890718` |
| H_687 | `71f14163af6727da4a31441ed23a21bb6a633eee6a4bfadd8308a0dc0ace6137` |
| H_688 | `b95b3672e4031732805ad8afb821479b85f2377415fd62a42e632eb19bce70ef` |
| H_689 | `e5d730581fb17e8481d5891459d7eecc79083d0cc1c554c6459d1e1b4c589e17` |
| H_690 | `ebbb4b6905b61aa16c3a1d05a370e9e1532f824bdbb7b3af8c9c52e360b0d3b9` |
| H_691 | `62a53f91d97addee0e04e77ad9d5f2f3a6de52ce7176a891db4a7e41903cbe04` |
| H_692 | `b28afba882b0e6c1af6d419af5ec0fb99881e922e186e75ea167ec65edfeb6d0` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_685..H_692 at selectors 0x2B3..0x2BA (`40 2B3`..`40 2BA`). Not RAW_BYTE; mirrors H_677..H_684 comment style (body-extend-095 / parallel-batch-89).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h52_208,subimm_h50_208,subimm_h51_208,subimm_h52_208,ldb_5060_210,ldb_5160_210,ldb_5260_210,addimm_h50_210}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **683/683 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **691/691 PASS**.
- Full canonical emit: JS=Rust=**15767B** code (was 15579B; +188B); byte-equal **Y**; sha `0ec083cba268878c…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `aef6d89f98ceb7c8…`; previous chained to `0ef9611b50021d82…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=15872; both peers code=15767; hash_a=hash_b=`e9241aae71ef04d0…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-094 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-89 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_685..H_692 at selectors 0x2B3..0x2BA.
4. Selftest: exact pins PASS (22/22/22/22/26/26/26/22B).
5. Goldens: JS 683/683 and Rust 691/691 PASS; full emit byte-equal Y at 15767B.
6. Lock: Relock once → `aef6d89f98ceb7c8…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-90: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_685..H_692), writing `parallel-batch-90-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: ADD-IMM 51/52 210 (finish 210 ADD triad); SUB-IMM 50/51/52 210 (start 210 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder if continuing. After batch-90 scratches done: parent next = body-extend-096 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-90-SPAWN.md` (no Task tool on this consolidator). Deferred carry: ADD-IMM 51/52 210; SUB-IMM 50/51/52 210; SET/GET/ORV/SUBV/ADDV/IMUL.
