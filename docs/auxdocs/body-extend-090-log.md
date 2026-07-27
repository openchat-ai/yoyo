# body-extend-090 Log · parallel-batch-84 consolidation (H_645..H_652)

> Tag: `body-extend-090-EXPERIMENTAL-batch84-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-84-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `e8500277650750c5…` → `63204ed031f1ad84…`.
> **handler count: 651 → 659** (+8 at selectors 0x28B..0x292 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_645 | 0x28B | 0x80 LDB | 51 60 1E8 | 26 | `ba62e4ad2c2e56ee` |
| H_646 | 0x28C | 0x80 LDB | 52 60 1E8 | 26 | `aac7a387b001d803` |
| H_647 | 0x28D | 0x62 ADD-IMM | 50 1E8 | 22 | `a63c229b97189c94` |
| H_648 | 0x28E | 0x62 ADD-IMM | 51 1E8 | 22 | `356a1a0b3408f7f6` |
| H_649 | 0x28F | 0x62 ADD-IMM | 52 1E8 | 22 | `ea596d905acbddb7` |
| H_650 | 0x290 | 0x61 SUB-IMM | 50 1E8 | 22 | `0e13aa7197e06d20` |
| H_651 | 0x291 | 0x61 SUB-IMM | 51 1E8 | 22 | `58e9756f847685f3` |
| H_652 | 0x292 | 0x61 SUB-IMM | 52 1E8 | 22 | `eeda72c92f5324fc` |

**REJECTED (not added):** none (batch-84 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 28B`..`40 292` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_645/H_646 finish 1E8 LDB triad. H_647..H_649 start/finish 1E8 ADD triad. H_650..H_652 start/finish 1E8 SUB triad.

**Deferred (not added this beat):** LDB / ADD-IMM / SUB-IMM imm=1F0 ladder — suggested for parallel-batch-85.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1E8 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1E8 uses imm32 → 26B pins.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_645 | `ba62e4ad2c2e56ee2ffdfc86fb5d52b43bc7ff65642a4246282f010fbdd9d5d1` |
| H_646 | `aac7a387b001d803071588118024c4b3edd529e4996f70f642c645e5d2eeed22` |
| H_647 | `a63c229b97189c942fd07bdd4622bcfcc67f550f5e4fe7972808180865b7ed9f` |
| H_648 | `356a1a0b3408f7f686339abad6a21ef6d856e7db3c340818c548755f60751813` |
| H_649 | `ea596d905acbddb77450f2f693618792308c05549e59eeeae5b4d04cdb102a04` |
| H_650 | `0e13aa7197e06d2067d67e5ce88f977dd7c9dc1746ef126e65a69268df08d635` |
| H_651 | `58e9756f847685f381c05f00f272297a49fba2942d372b4fea3c875df5fbed2f` |
| H_652 | `eeda72c92f5324fcc96b121d2202021424f924135b4b7baa7cbc96156e26585e` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_645..H_652 at selectors 0x28B..0x292 (`40 28B`..`40 292`). Not RAW_BYTE; mirrors H_637..H_644 comment style (body-extend-090 / parallel-batch-84).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5160_1E8,ldb_5260_1E8,addimm_h50_1E8,addimm_h51_1E8,addimm_h52_1E8,subimm_h50_1E8,subimm_h51_1E8,subimm_h52_1E8}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **643/643 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **651/651 PASS**.
- Full canonical emit: JS=Rust=**14827B** code (was 14643B; +184B); byte-equal **Y**; sha `40f726ea2f4f59dd…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `63204ed031f1ad84…`; previous chained to `e8500277650750c5…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=14848; both peers code=14827; hash_a=hash_b=`6ae39260712572a2…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-089 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-84 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_645..H_652 at selectors 0x28B..0x292.
4. Selftest: exact pins PASS (26/26/22/22/22/22/22/22B).
5. Goldens: JS 643/643 and Rust 651/651 PASS; full emit byte-equal Y at 14827B.
6. Lock: Relock once → `63204ed031f1ad84…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-85: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_645..H_652), writing `parallel-batch-85-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: start LDB/ADD-IMM/SUB-IMM imm=1F0 ladder; SET/GET fresh, etc. After batch-85 scratches done: parent next = body-extend-091 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-85-SPAWN.md` (no Task tool on this consolidator). Deferred carry: LDB/ADD-IMM/SUB-IMM imm=1F0.
