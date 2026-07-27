# body-extend-102 Log · parallel-batch-96 consolidation (H_741..H_748)

> Tag: `body-extend-102-EXPERIMENTAL-batch96-consolidation-8` · 2026-07-26 (UTC+8).
> Source: `docs/auxdocs/parallel-batch-96-log.md` (8 PASS / 0 REJECT).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `514ff62ce8663a15…` → `6532ea809c58c7a9…`.
> **handler count: 747 → 755** (+8 at selectors 0x2EB..0x2F2 via label-width A).
> LABEL_CAP remains 1024 (no bump this beat).
> Matrix strategy (first batch under matrix-priority; replaces imm-ladder).

## 1. Consolidated picks (ALL 8 — no REJECTED)

| H | sel | opcode | args | pin (B) | sha256 (16) |
|---|-----|--------|------|---------|-------------|
| H_741 | 0x2EB | 0x84 MEMCPY_DATA | 50 51 40 | 2 (stub) | `1344fed055987f9e` |
| H_742 | 0x2EC | 0x85 MEMCPY_STATE | 50 51 40 | 2 (stub) | `1344fed055987f9e` |
| H_743 | 0x2ED | 0x60 GET | 60 50 | 15 | `81fbdbb14873c447` |
| H_744 | 0x2EE | 0x60 GET | 50 60 | 15 | `236c066a6b5b44ef` |
| H_745 | 0x2EF | 0x30 SET | 50 0xfff | 18 | `61697071ff6cd475` |
| H_746 | 0x2F0 | 0x30 SET | 51 0x10000 | 18 | `11a103bf4b11cd82` |
| H_747 | 0x2F1 | 0x69 ORV | 50 62 | 25 | `d1ef5ee917509ccc` |
| H_748 | 0x2F2 | 0x65 CMP | 60 52 | 18 | `9d5076dd78f13b7f` |

**REJECTED (not added):** none (batch-96 was 8/8 PASS).

**MEMCPY honesty record (P0 semantic gap):**
- H_741 (MEMCPY_DATA) and H_742 (MEMCPY_STATE) both emit stub `0xc3` in JS `encodeOp` and Rust `compile.rs`. Byte-eq PASSES (2B pins identical across peers), but this is the **D-3 semantic gap**: the stub does not actually copy. Real MEMCPY (load src/store dst loop) still TODO. Matrix rows 84 MEMCPY_DATA / 85 MEMCPY_STATE remain **MISSING** for true DONE; logged here as **PARTIAL** (byte-pass, semantics not implemented). No invent-green.

**Operand encoding notes:**
- GET dst=60 src=50: `498b8780020000` (load slot-50 disp 0x280) + `49898700030000` (store slot-60 disp 0x300) + `c3` → 15B.
- GET dst=50 src=60: load disp 0x300, store disp 0x280 → 15B.
- SET slot=50 imm=0xfff: `48b8ff0f000000000000` (movabs rax imm32) + store disp 0x280 + `c3` → 18B.
- SET slot=51 imm=0x10000: `48b80000010000000000` + store disp 0x300 + `c3` → 18B.
- ORV dst=50 src=62: load slot-50 + load slot-62 + `or rax,r9` + store slot-50 → 25B.
- CMP a=60 b=52: load slot-60 + load slot-52 + `cmp rax,r9` → 18B.
- Store-disp check (slot→disp low): 50→80, 51→88, 52→90 — verified in all non-MEMCPY pins.

Full sha256 (PASS pins):
| H | sha256 |
|---|--------|
| H_741 | `1344fed055987f9eb87aef70de1922ae7239d57b2a38fd9fb91a2a3f0b4c497a` |
| H_742 | `1344fed055987f9eb87aef70de1922ae7239d57b2a38fd9fb91a2a3f0b4c497a` |
| H_743 | `81fbdbb14873c447b3b9bc5bd013c689c0ab643218d88c3dce820ce8526f4374` |
| H_744 | `236c066a6b5b44ef04ddb29402d2d5ef64c905f0eb75a55abb7ad1336384d552` |
| H_745 | `61697071ff6cd475f073532bd14da2567d20a87d1090dee1983f599f4eae00ee` |
| H_746 | `11a103bf4b11cd823012f36cd9a66ea86cfbbd1c6e1a0928a394190211582a5c` |
| H_747 | `d1ef5ee917509ccc81e91d54f84d04ae34496c7e8dc6951184f38e8de30ec145` |
| H_748 | `9d5076dd78f13b7f682c2bfb9ff0925be887e072242b21cecdd75671abb8f3a8` |

## 2. Execution record

- Canonical append: `yoyo/projects/yoyo.ty` H_741..H_748 at selectors 0x2EB..0x2F2 (`40 2EB`..`40 2F2`). Not RAW_BYTE.
- Fixtures promoted under `yoyo/tests/golden/selfhost_min_{memcpy_data_stub,memcpy_state_stub,get_60_50,get_50_60,set_50_fff,set_51_10000,orv_50_62,cmp_60_52}.ty` + `expected/*.code.hex` (hex-only; scratch/log pins).
- JS: 8 checkX in `golden.js` (checkMEMCPYDATA_STUB, checkMEMCPYSTATE_STUB, checkGET6050, checkGET5060, checkSET50FFF, checkSET5110000, checkORV5062, checkCMP6052) — **739/739 PASS** (prev 731 → 739).
- Rust verifier: **57/57 PASS** (`cargo test` in verifier crate).
- Full canonical emit: JS=Rust=**16996B** code (was 16883B; +113B = 2+2+15+15+18+18+25+18). Byte-equal **Y** (DDC `diff` command).
- Lock: `verify-yoyo-ty.mjs` PASS at `6532ea809c58c7a9…`; previous chained to `514ff62ce8663a15…`.
- DDC: `verify-selfhost.ps1` completed. PE `.text` **EQUAL** (compared_bytes=17408; both code=16996; hash_a=hash_b=`c9e21a449b40e4e5…`). Recorded honestly — still EXPERIMENTAL · NON-GREEN.
- **MEMCPY semantic caveat:** H_741/H_742 stub=C3 passed DDC byte-eq, but real copy is still TODO. P0 gap remains for semantic correctness. Matrix rows 84/85 stay MISSING until real MEMCPY impl.
- No PROMPT edit, version bump, commit, or GREEN claim.

## 3. Lock protocol (8 steps)

1. Pick: 8 PASS from parallel-batch-96 (2 MEMCPY stub probes + 2 GET multi-slot + 2 SET multi-imm + 1 ORV + 1 CMP).
2. Encoder: existing JS/Rust paths retained; LABEL_CAP=1024 unchanged.
3. Hand-author: H_741..H_748 at selectors 0x2EB..0x2F2 in `yoyo/projects/yoyo.ty`.
4. Selftest: exact pins PASS (2/2/15/15/18/18/25/18B).
5. Goldens: JS 739/739 and Rust 57/57 PASS; full emit byte-equal Y at 16996B.
6. Lock: Relock once → `6532ea809c58c7a9…` (previous `514ff62ce8663a15…`).
7. DDC: `verify-selfhost.ps1` completed (PE `.text` EQUAL this beat; 17408 compared).
8. Commit: none.

## 4. Matrix status update

Rows flipped:
| opcode | shape | prev | new | note |
|--------|-------|------|-----|------|
| 60 GET | dst src 多 slot | DONE | DONE (unchanged) | H_743/H_744 交叉变体 +2 handler; DONE 行代表力更强 |
| 30 SET | slot imm 多 imm | DONE | DONE (unchanged) | H_745/H_746 imm32 大 imm; DONE 行代表力更强 |
| 69 ORV | dst src 多 dst/src | DONE | DONE (unchanged) | H_747 dst=50 src=62 新增组合 |
| 65 CMP | a b 多 slot | DONE | DONE (unchanged) | H_748 a=60 b=52 新增组合 |
| 84 MEMCPY_DATA | dst src n | MISSING | PARTIAL | stub=C3 字节通过; 真实拷贝仍 TODO (P0 语义缺口) |
| 85 MEMCPY_STATE | dst src n | MISSING | PARTIAL | 同上 |

**Note:** GET/SET/ORV/CMP rows were already DONE before this beat; H_743..H_748 **strengthen** coverage (more slot/imm combinations) but do not flip status. The only status changes are MEMCPY_DATA/STATE: MISSING → PARTIAL (byte-pass, not semantically DONE).

**Matrix coverage (updated):** DONE 16 / PARTIAL 19 (+2 MEMCPY) / MISSING 28 (-2 MEMCPY). selfhost-need=YES + MISSING: 12 (was 14).

## 5. Next default

Auto-spawn parallel-batch-97: fresh picks from remaining P0/P1/P2 matrix rows (YES+MISSING/PARTIAL = 12 rows). Suggested:
- **P0 first**: real MEMCPY_DATA/STATE implementation (the two PARTIAL rows; replace stub C3). This closes the D-3 semantic gap. After real impl, re-run batch-96 fixtures to confirm pins change from `c3c3` to real copy bytes, still PASS.
- **P1**: 68 ADDV / 6A SUBV / 63 IMUL 更多 dst/src 组合 (现有 6 handler 但组合有限); 66 INC / 67 DEC 多 slot (仅 3 handler).
- **P2**: LDB offset 边界 (127/128/-128/-129/256) 等 imm8/imm32 边界.

After batch-97 done: parent next = body-extend-103 serialize + 1 Relock. Handoff: `docs/auxdocs/parallel-batch-97-SPAWN.md`.

## 6. Honesty override checks

- Peer JS/Rust divergence at the 8 PASS handlers: **NONE** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
- No PROMPT edit. No version bump. No `*.lock` touch before consolidate; lock updated atomically at step 6.
- No git commit (W-START convention).
- MEMCPY_DATA/STATE: both emit stub `0xc3` → byte-eq PASSES DDC; D-3 semantic gap recorded honestly, not invented-green. Matrix rows stay PARTIAL, not DONE.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
