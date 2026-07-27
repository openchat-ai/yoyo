# body-extend-082 Log · parallel-batch-76 consolidation (H_581..H_588)

> Tag: `body-extend-082-EXPERIMENTAL-batch76-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-76-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `267c611dbb648db1…` → `05a3a9c6693fa65c…`.
> **handler count: 587 → 595** (+8 at selectors 0x24B..0x252 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_581 | 0x24B | 0x80 LDB | 50 60 1B0 | 26 | `4a28b7afe67cd9c8` |
| H_582 | 0x24C | 0x80 LDB | 51 60 1B0 | 26 | `bbbb35dd922e35f8` |
| H_583 | 0x24D | 0x80 LDB | 52 60 1B0 | 26 | `114e9beed1fbb101` |
| H_584 | 0x24E | 0x62 ADD-IMM | 50 1B0 | 22 | `449e70ae9ce9bc48` |
| H_585 | 0x24F | 0x62 ADD-IMM | 51 1B0 | 22 | `19cf91fa4836bb0d` |
| H_586 | 0x250 | 0x62 ADD-IMM | 52 1B0 | 22 | `e719980bb34c73f8` |
| H_587 | 0x251 | 0x61 SUB-IMM | 50 1B0 | 22 | `fde148880489e4d4` |
| H_588 | 0x252 | 0x61 SUB-IMM | 51 1B0 | 22 | `2f842240d885a210` |

**REJECTED (not added):** none (batch-76 was 8/8 PASS; MEMCPY opcodes 0x84/0x85 remain out of scope). Selectors `40 24B`..`40 252` are HANDLER labels only — not opcode MEMCPY / RAW_BYTE. Opcode 0x64 MOVRR (D-2) was not emitted. H_581..H_583 start/finish 1B0 LDB triad. H_584..H_586 start/finish 1B0 ADD triad. H_587/H_588 start 1B0 SUB triad (slot=52 deferred).

**Deferred (not added this beat):** SUB-IMM slot=52 imm=1B0; LDB 50/51/52 60 1B8; ADD-IMM/SUB-IMM slot=50/51/52 imm=1B8 — suggested for parallel-batch-77.

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins.
ADD-IMM/SUB-IMM imm=0x1B0 uses imm32 (`48 81 c0` / `48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1B0 uses imm32 → 26B pins.

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_581..H_588 at selectors 0x24B..0x252 (`40 24B`..`40 252`). Not RAW_BYTE; mirrors H_573..H_580 comment style (body-extend-082 / parallel-batch-76).
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{ldb_5060_1B0,ldb_5160_1B0,ldb_5260_1B0,addimm_h50_1B0,addimm_h51_1B0,addimm_h52_1B0,subimm_h50_1B0,subimm_h51_1B0}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` — **579/579 PASS**.
- Rust self_test: 8 `*_slot_check` — **PASS** (`yoyo selftest` ok).
- Rust golden: 8 `check_selfhost_min_*` — **587/587 PASS**.
- Full canonical emit: JS=Rust=**13335B** code (was 13147B; +188B); byte-equal **Y**; sha `3a307a15d3d451e9…`.
- Lock: `verify-yoyo-ty.mjs` PASS at `05a3a9c6693fa65c…`; previous chained to `267c611dbb648db1…`.
- DDC: `verify-selfhost.ps1` ran; PE `.text` DDC **EQUAL** (compared_bytes=13824; both peers code=13335; hash_a=hash_b=`8bf07bc4fcf6543a…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN; no invent-green / no GREEN promotion. (Prior beat body-extend-081 also EQUAL — both honest.)
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-76 (slot/imm/dst variants).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_581..H_588 at selectors 0x24B..0x252.
4. Selftest: exact pins PASS (26/26/26/22/22/22/22/22B).
5. Goldens: JS 579/579 and Rust 587/587 PASS; full emit byte-equal Y at 13335B.
6. Lock: Relock once → `05a3a9c6693fa65c…`.
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat).
8. Commit: none.

## 4. Next default

Auto-spawn parallel-batch-77: 6–8 scratch-only fresh picks (opcodes/slot variants not yet in yoyo.ty after H_581..H_588), writing `parallel-batch-77-log.md`. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. Suggested: finish SUB-IMM 52 1B0, then LDB 50/51/52 60 1B8, ADD-IMM/SUB-IMM imm=1B8 triad, SET/GET fresh, etc. After batch-77 scratches done: parent next = body-extend-083 serialize PASSes + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-77-SPAWN.md` (no Task tool on this consolidator). Deferred carry: SUB-IMM 52 1B0; LDB 50/51/52 60 1B8; ADD-IMM/SUB-IMM slot=50/51/52 imm=1B8.
