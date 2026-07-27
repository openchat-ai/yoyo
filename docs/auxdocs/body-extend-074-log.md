# body-extend-074 Log · parallel-batch-68 consolidation (H_517..H_524)

> Tag: `body-extend-074-EXPERIMENTAL-batch68-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-68-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `1a6cb44aa28367d2…` → `9243965c886555e9…`.
> **handler count: 523 → 531** (+8 at selectors 0x20B..0x212 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED; 2 deferred from batch-68)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_517 | 0x20B | 0x61 SUB-IMM | 52 170 | 22 | `dad788fae9b7dc6d` |
| H_518 | 0x20C | 0x80 LDB | 50 60 178 | 26 | `88e184b59a6db03c` |
| H_519 | 0x20D | 0x80 LDB | 51 60 178 | 26 | `9ed7c675af239145` |
| H_520 | 0x20E | 0x80 LDB | 52 60 178 | 26 | `acf695cec1340844` |
| H_521 | 0x20F | 0x62 ADD-IMM | 50 178 | 22 | `90d4b604f3d3217f` |
| H_522 | 0x210 | 0x62 ADD-IMM | 51 178 | 22 | `ef600aa63170300a` |
| H_523 | 0x211 | 0x62 ADD-IMM | 52 178 | 22 | `720aa67f69ef0ab9` |
| H_524 | 0x212 | 0x61 SUB-IMM | 50 178 | 22 | `7f477a27dd9d8bb9` |

**REJECTED (not added):** none (batch-68 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 20B`..`40 212` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_517 finishes 170 SUB triad. H_518..H_520 start 178 LDB triad. H_521..H_523 start 178 ADD triad. H_524 starts 178 SUB (slot 51/52 deferred).

**Deferred (not added this beat):** SUB-IMM slot=51/52 imm=178 — suggested for parallel-batch-69.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x178 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
SUB-IMM imm=0x170 uses imm32 → 22B pin.
LDB oo=0x178 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_517..H_524 at selectors 0x20B..0x212 (`40 20B`..`40 212`). Not RAW_BYTE; mirrors H_509..H_516 comment style (body-extend-074 / parallel-batch-68).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{subimm_h52_170,ldb_5060_178,ldb_5160_178,ldb_5260_178,addimm_h50_178,addimm_h51_178,addimm_h52_178,subimm_h50_178}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **515/515 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **523/523 PASS**.
- Full canonical emit: JS=Rust=**11843B** code (was 11655B; +188B); byte-equal **Y**; sha `b00f4338090941fe…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `9243965c886555e9…`; previous chained to `1a6cb44aa28367d2…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=12288; both peers code=11843; hash_a=hash_b=`75e7cf2ac5809b45…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-073 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-68 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_517..H_524 at selectors 0x20B..0x212.
4. Selftest: exact pins PASS (22/26/26/26/22/22/22/22B).
5. Goldens: JS 515/515 and Rust 523/523 PASS; full emit byte-equal Y at 11843B.
6. Lock: Relock once → `9243965c886555e9…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-69: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_517..H_524), writing `parallel-batch-69-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish SUB-IMM 51/52 178, then LDB/ADD/SUB oo=180 ladder, SET/GET fresh, etc. After batch-69 scratches done: parent next = body-extend-075 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-69-SPAWN.md` (no Task tool on this consolidator). Deferred carry: SUB-IMM slot=51/52 imm=178.
