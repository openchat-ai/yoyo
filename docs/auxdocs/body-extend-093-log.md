# body-extend-093 Log · parallel-batch-87 consolidation (H_669..H_676)

> Tag: `body-extend-093-EXPERIMENTAL-batch87-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-87-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `1991af8484d67ec1…` → `04656bbbbb152b54…`.
> **handler count: 675 → 683** (+8 at selectors 0x2A3..0x2AA via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_669 | 0x2A3 | 0x61 SUB-IMM | 51 1F8 | 22 | `cd8053ace6652cd9` |
| H_670 | 0x2A4 | 0x61 SUB-IMM | 52 1F8 | 22 | `512b7b4c08728ca7` |
| H_671 | 0x2A5 | 0x80 LDB | 50 60 200 | 26 | `8ef97152f880c8bf` |
| H_672 | 0x2A6 | 0x80 LDB | 51 60 200 | 26 | `ae88f23839b7ed37` |
| H_673 | 0x2A7 | 0x80 LDB | 52 60 200 | 26 | `623de62f88220d56` |
| H_674 | 0x2A8 | 0x62 ADD-IMM | 50 200 | 22 | `cba55979366f2bab` |
| H_675 | 0x2A9 | 0x62 ADD-IMM | 51 200 | 22 | `d48330be708021e4` |
| H_676 | 0x2AA | 0x62 ADD-IMM | 52 200 | 22 | `563af54479f67bd3` |

**REJECTED (not added):** none (batch-87 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 2A3`..`40 2AA` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_669/H_670 finish 1F8 SUB triad. H_671..H_673 start/finish 200 LDB triad. H_674..H_676 start/finish 200 ADD triad.

**Deferred (not added this beat):** SUB-IMM slot=50/51/52 imm=200; SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked — suggested for parallel-batch-88.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1F8/0x200 uses imm32 (`48 81 e8` / `48 81 c0`) → 22B pins; not imm8.
LDB oo=0x200 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_669 | `cd8053ace6652cd9f67f1021123bdc5222b2a7a5e4dec6586639ae6d3bbb95d4` |
| H_670 | `512b7b4c08728ca7793738789ea4129b0a3982166e627307e6ad89d2b009f471` |
| H_671 | `8ef97152f880c8bf58ccff2b1e71f0d5607d5659d0fb45acda46d9f3dfc13490` |
| H_672 | `ae88f23839b7ed37a87ae8ca78f67b76185a4b74f88574f6a84689f8d40bea2c` |
| H_673 | `623de62f88220d56cb2e73f2807b3a7503641858552eb732a5e5cef420bd803d` |
| H_674 | `cba55979366f2bab0d63b00bca48823c7d7b80965c4c2f5ac4691cd11977ea07` |
| H_675 | `d48330be708021e46e3d010b4b35804437f8e844842cf428ac1bf7fa6f7f5348` |
| H_676 | `563af54479f67bd3329f812c6f699999bbfb34097e119ae3ab3f39516c6a022f` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_669..H_676 at selectors 0x2A3..0x2AA (`40 2A3`..`40 2AA`). Not RAW_BYTE; mirrors H_661..H_668 comment style (body-extend-093 / parallel-batch-87).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h51_1F8,subimm_h52_1F8,ldb_5060_200,ldb_5160_200,ldb_5260_200,addimm_h50_200,addimm_h51_200,addimm_h52_200}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **667/667 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **675/675 PASS**.
- Full canonical emit: JS=Rust=**15391B** code (was 15203B; +188B); byte-equal **Y**; sha `e6177229b657100f…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `04656bbbbb152b54…`; previous chained to `1991af8484d67ec1…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=15872; both peers code=15391; hash_a=hash_b=`fa9e36c557873cfe…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-092 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-87 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_669..H_676 at selectors 0x2A3..0x2AA.
4. Selftest: exact pins PASS (22/22/26/26/26/22/22/22B).
5. Goldens: JS 667/667 and Rust 675/675 PASS; full emit byte-equal Y at 15391B.
6. Lock: Relock once → `04656bbbbb152b54…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-88: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_669..H_676), writing `parallel-batch-88-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: SUB-IMM 50/51/52 200 (start/finish 200 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh if not locked; next ladder if continuing. After batch-88 scratches done: parent next = body-extend-094 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-88-SPAWN.md` (no Task tool on this consolidator). Deferred carry: SUB-IMM 50/51/52 200; SET/GET/ORV/SUBV/ADDV/IMUL.
