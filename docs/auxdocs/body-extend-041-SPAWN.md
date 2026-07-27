# body-extend-041 SPAWN · consolidate parallel-batch-35

> Continuous queue handoff from parallel-batch-35 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `a58ead289233c42ba1c6e9a84aedb6218176aad27ecd5cbdd0d4659a2e5bc187` (abbrev `a58ead28…`).
> Handlers = 260 (H_00..H_253). Last selectors: 0xFC..0xFF = H_246..H_249; 0x100..0x103 = H_250..H_253 (`40 100`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-35-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-040-log.md` / `docs/auxdocs/body-extend-040-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-040 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 104`.. for H_254.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).

## Task: body-extend-041 (serialize + Relock)

Mirror body-extend-040 / body-extend-039 protocol:

1. Hand-author append H_254..H_261 to `yoyo/projects/yoyo.ty` at selectors `40 104` .. `40 10B` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5060_a0,ldb_5160_a0,ldb_5260_a0,subimm_h50_80,subimm_h51_80,subimm_h52_80,addimm_h50_88,addimm_h51_88}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `a58ead289233c42b…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-041-log.md`.
7. Auto-spawn parallel-batch-36 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-36-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_254 | 0x104 | 0x80 LDB | 50 60 A0 | `498b87000300004881c0a0000000480fb60049898780020000c3` (26B) | `4817b8ddf9b52566` |
| H_255 | 0x105 | 0x80 LDB | 51 60 A0 | `498b87000300004881c0a0000000480fb60049898788020000c3` (26B) | `fcf0ba5ffb072ffa` |
| H_256 | 0x106 | 0x80 LDB | 52 60 A0 | `498b87000300004881c0a0000000480fb60049898790020000c3` (26B) | `c6dd95a8ede6bf6a` |
| H_257 | 0x107 | 0x61 SUB-IMM | 50 80 | `498b87800200004881e88000000049898780020000c3` (22B) | `e0304eea69eed143` |
| H_258 | 0x108 | 0x61 SUB-IMM | 51 80 | `498b87880200004881e88000000049898788020000c3` (22B) | `f76a1690a99750ff` |
| H_259 | 0x109 | 0x61 SUB-IMM | 52 80 | `498b87900200004881e88000000049898790020000c3` (22B) | `d26957f7354c5ec6` |
| H_260 | 0x10A | 0x62 ADD-IMM | 50 88 | `498b87800200004881c08800000049898780020000c3` (22B) | `eabc3ae46677427e` |
| H_261 | 0x10B | 0x62 ADD-IMM | 51 88 | `498b87880200004881c08800000049898788020000c3` (22B) | `4c60d97a9ae2744d` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_254..H_256).
SUB-IMM imm=0x80 uses imm32 sub (`48 81 e8`) → 22B pins (H_257..H_259); not imm8.
ADD-IMM imm=0x88 uses imm32 add (`48 81 c0`) → 22B pins (H_260..H_261); not imm8.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_254 | `4817b8ddf9b525669ccf6489dd0795345486ef0399993f7d81a05782b5bd7a0f` |
| H_255 | `fcf0ba5ffb072ffa95b0eee6acb2338408db86254dbe816f0c684e14996d1dc3` |
| H_256 | `c6dd95a8ede6bf6a65911146461ffa7f80a0bd1170098ad100478c2aefbe05c2` |
| H_257 | `e0304eea69eed143e909734feb74ed20316a403855b257e13aaaea5bbcdbc964` |
| H_258 | `f76a1690a99750ff96f35a052519004f469b3b306da7134812d7c6e8d10cf962` |
| H_259 | `d26957f7354c5ec61f02629f0c80401e40f1134c4bfe6295d65cf0502367132a` |
| H_260 | `eabc3ae46677427e13e8aa82bd58f288680bdde77a890ce1150f7a167d36d224` |
| H_261 | `4c60d97a9ae2744d33f0d0d8b3689f4537b6f704777093a75a81a3dfa572dcf5` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5060_a0`, `_scratch_ldb_5160_a0`, `_scratch_ldb_5260_a0`, `_scratch_subimm_h50_80`,
`_scratch_subimm_h51_80`, `_scratch_subimm_h52_80`, `_scratch_addimm_h50_88`, `_scratch_addimm_h51_88`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 104`.. for H_254.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
