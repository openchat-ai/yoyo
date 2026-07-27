# body-extend-036 SPAWN · consolidate parallel-batch-30

> Continuous queue handoff from parallel-batch-30 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `23f42236c6097a13e83a15c861d51845fbe1da64eadfabdb95fdeaca3ebe55f5` (abbrev `23f42236…`).
> Handlers = 220 (H_00..H_213). Last selectors: 0xD4..0xDB = H_206..H_213.
> Source: `docs/auxdocs/parallel-batch-30-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-035-log.md` / `docs/auxdocs/body-extend-035-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-035 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.

## Task: body-extend-036 (serialize + Relock)

Mirror body-extend-035 / body-extend-034 protocol:

1. Hand-author append H_214..H_221 to `yoyo/projects/yoyo.ty` at selectors `40 DC` .. `40 E3`.
2. Promote fixtures from `_scratch_{addimm_h51_60,addimm_h52_60,subimm_h50_58,subimm_h51_58,ldb_5160_80,ldb_5260_80,subimm_h52_58,addimm_h50_68}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `23f42236c6097a13…`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-036-log.md`.
7. Auto-spawn parallel-batch-31 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-31-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_214 | 0xDC | 0x62 ADD-IMM | 51 60 | `498b87880200004883c06049898788020000c3` (19B) | `2e762fb2ad102e6a` |
| H_215 | 0xDD | 0x62 ADD-IMM | 52 60 | `498b87900200004883c06049898790020000c3` (19B) | `efec3943630fb998` |
| H_216 | 0xDE | 0x61 SUB-IMM | 50 58 | `498b87800200004883e85849898780020000c3` (19B) | `a7d41e13060d56b7` |
| H_217 | 0xDF | 0x61 SUB-IMM | 51 58 | `498b87880200004883e85849898788020000c3` (19B) | `d9559da92e31429b` |
| H_218 | 0xE0 | 0x80 LDB | 51 60 80 | `498b87000300004881c080000000480fb60049898788020000c3` (26B) | `f39364a89ec6b361` |
| H_219 | 0xE1 | 0x80 LDB | 52 60 80 | `498b87000300004881c080000000480fb60049898790020000c3` (26B) | `d239426ce0456ebf` |
| H_220 | 0xE2 | 0x61 SUB-IMM | 52 58 | `498b87900200004883e85849898790020000c3` (19B) | `155b83f538845515` |
| H_221 | 0xE3 | 0x62 ADD-IMM | 50 68 | `498b87800200004883c06849898780020000c3` (19B) | `8390493232f90387` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
LDB oo≥0x80 uses imm32 add (`48 81 c0`) → 26B pins (H_218/H_219).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_214 | `2e762fb2ad102e6a117754a9757b83ebefa2cb33e8cfad7e6b056dae64ad9318` |
| H_215 | `efec3943630fb9988eace80e5c0f18753a72a78871ce173cd33206c6183373e6` |
| H_216 | `a7d41e13060d56b759bad08c82f4da839255df32a27a2cc3c1ed5ed226486e6b` |
| H_217 | `d9559da92e31429bc2418d7e71c0a2f5f70a1dca17a0fa500ecf51cb8b321fbe` |
| H_218 | `f39364a89ec6b3616fc726561a0d03a9a6f220876e3a72d4db83828c20bb7444` |
| H_219 | `d239426ce0456ebf0b535f72a28e26996c9270146443a10c43912a878fb56a50` |
| H_220 | `155b83f5388455152e75d9abd12c7dca908df8b055de59218f9c4125699c9688` |
| H_221 | `8390493232f90387c309b4a307b8d14f63885109c65f0701f48cf41746a6f415` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_addimm_h51_60`, `_scratch_addimm_h52_60`, `_scratch_subimm_h50_58`, `_scratch_subimm_h51_58`,
`_scratch_ldb_5160_80`, `_scratch_ldb_5260_80`, `_scratch_subimm_h52_58`, `_scratch_addimm_h50_68`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 DC`.. for H_214.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
