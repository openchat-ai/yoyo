# body-extend-096 Log · parallel-batch-90 consolidation (H_693..H_700)

> Tag: `body-extend-096-EXPERIMENTAL-batch90-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-90-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `aef6d89f98ceb7c8…` → `0a02f49ed0c94a2d…`.
> **handler count: 699 → 707** (+8 at selectors 0x2BB..0x2C2 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_693 | 0x2BB | 0x62 ADD-IMM | 51 210 | 22 | `f59a9a17f02eae7c` |
| H_694 | 0x2BC | 0x62 ADD-IMM | 52 210 | 22 | `b6abb627bf849fc0` |
| H_695 | 0x2BD | 0x61 SUB-IMM | 50 210 | 22 | `f77af100f9fabd84` |
| H_696 | 0x2BE | 0x61 SUB-IMM | 51 210 | 22 | `dbfd9ece27cb16d9` |
| H_697 | 0x2BF | 0x61 SUB-IMM | 52 210 | 22 | `b4bcf1859605c71c` |
| H_698 | 0x2C0 | 0x80 LDB | 50 60 218 | 26 | `c6cb4e7e1fac02c9` |
| H_699 | 0x2C1 | 0x80 LDB | 51 60 218 | 26 | `6296837a29daedeb` |
| H_700 | 0x2C2 | 0x80 LDB | 52 60 218 | 26 | `8e68e69170dde74d` |

**REJECTED (not added):** none (batch-90 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 2BB`..`40 2C2` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_693/H_694 finish 210 ADD triad. H_695..H_697 start/finish 210 SUB triad. H_698..H_700 start/finish 218 LDB triad.

**Deferred (not added this beat):** ADD-IMM slot=50/51/52 imm=218; SUB-IMM slot=50/51/52 imm=218; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked — suggested for parallel-batch-91.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x210 uses imm32 (`48 81 e8` / `48 81 c0`) → 22B pins; not imm8.
LDB oo=0x218 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_693 | `f59a9a17f02eae7c09283f04ad040634e7fda687dda507038344fa9cc758be6c` |
| H_694 | `b6abb627bf849fc0a9cebf0c9b09d36e0ca9c67bf705287c2bfc5e25301c690a` |
| H_695 | `f77af100f9fabd84ef73e82bfcfed4011049214dccf6d64fc50a8931a9015fa2` |
| H_696 | `dbfd9ece27cb16d9e60e5a74e1a1bfac06a2ed48396cb7fddae9db85ef6576e8` |
| H_697 | `b4bcf1859605c71c1618d398a81d2e3a1fd0f0d47298a2a62af9164f9f7080f9` |
| H_698 | `c6cb4e7e1fac02c9cea83b983dc954c4f3066cd8a67a026fa8c2b35e92aea8a3` |
| H_699 | `6296837a29daedeba1df94ff6f0c6173e11264bc97593e06246e6cc71544234b` |
| H_700 | `8e68e69170dde74dc3221b9ba81b23012c1fd0d5957b564ae1ad73489451dc85` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_693..H_700 at selectors 0x2BB..0x2C2 (`40 2BB`..`40 2C2`). Not RAW_BYTE; mirrors H_685..H_692 comment style (body-extend-096 / parallel-batch-90).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{addimm_h51_210,addimm_h52_210,subimm_h50_210,subimm_h51_210,subimm_h52_210,ldb_5060_218,ldb_5160_218,ldb_5260_218}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **691/691 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **699/699 PASS**.
- Full canonical emit: JS=Rust=**15955B** code (was 15767B; +188B); byte-equal **Y**; sha `1c7b55e4eed6df28…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `0a02f49ed0c94a2d…`; previous chained to `aef6d89f98ceb7c8…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=16384; both peers code=15955; hash_a=hash_b=`910e889e6d393f6d…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-095 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-90 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_693..H_700 at selectors 0x2BB..0x2C2.
4. Selftest: exact pins PASS (22/22/22/22/22/26/26/26B).
5. Goldens: JS 691/691 and Rust 699/699 PASS; full emit byte-equal Y at 15955B.
6. Lock: Relock once → `0a02f49ed0c94a2d…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-91: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_693..H_700), writing `parallel-batch-91-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: ADD-IMM 50/51/52 218 (start 218 ADD triad); SUB-IMM 50/51/52 218 (start 218 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder if continuing. After batch-91 scratches done: parent next = body-extend-097 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-91-SPAWN.md` (no Task tool on this consolidator). Deferred carry: ADD-IMM/SUB-IMM 50/51/52 218; SET/GET/ORV/SUBV/ADDV/IMUL.
