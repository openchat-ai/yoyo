# body-extend-072 SPAWN · consolidate parallel-batch-66

> Continuous queue handoff from parallel-batch-66 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `1f070530a91ca949696f7552fc5d3b3690f00630a191ce25662ee33951314e41` (abbrev `1f070530…`).
> Handlers = 507 (H_00..H_500). Last selectors: 0x1F3..0x1FA = H_493..H_500 (`40 1F3`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-66-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-071-log.md` / `docs/auxdocs/body-extend-071-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-071 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1FB`.. for H_501.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1FB`/`40 1FC` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-072 (serialize + Relock)

Mirror body-extend-071 / body-extend-070 protocol:

1. Hand-author append H_501..H_508 to `yoyo/projects/yoyo.ty` at selectors `40 1FB` .. `40 202` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5160_168,ldb_5260_168,addimm_h50_168,addimm_h51_168,addimm_h52_168,subimm_h50_168,subimm_h51_168,subimm_h52_168}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `1f070530a91ca949696f7552fc5d3b3690f00630a191ce25662ee33951314e41`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-072-log.md`.
7. Auto-spawn parallel-batch-67 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-67-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_501 | 0x1FB | 0x80 LDB | 51 60 168 | `498b87000300004881c068010000480fb60049898788020000c3` (26B) | `71614ed8ee72059f` |
| H_502 | 0x1FC | 0x80 LDB | 52 60 168 | `498b87000300004881c068010000480fb60049898790020000c3` (26B) | `b40ac7b90a6c8cb3` |
| H_503 | 0x1FD | 0x62 ADD-IMM | 50 168 | `498b87800200004881c06801000049898780020000c3` (22B) | `70dcc769354c9c59` |
| H_504 | 0x1FE | 0x62 ADD-IMM | 51 168 | `498b87880200004881c06801000049898788020000c3` (22B) | `ae42aee20a8d8c9f` |
| H_505 | 0x1FF | 0x62 ADD-IMM | 52 168 | `498b87900200004881c06801000049898790020000c3` (22B) | `7109bea20936a27a` |
| H_506 | 0x200 | 0x61 SUB-IMM | 50 168 | `498b87800200004881e86801000049898780020000c3` (22B) | `5b1652dbeda9a005` |
| H_507 | 0x201 | 0x61 SUB-IMM | 51 168 | `498b87880200004881e86801000049898788020000c3` (22B) | `2d56b2a1e2d5c002` |
| H_508 | 0x202 | 0x61 SUB-IMM | 52 168 | `498b87900200004881e86801000049898790020000c3` (22B) | `f442c8a07cbb8382` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x168 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x168 uses imm32 sub (`48 81 e8`) → 22B pins (H_506..H_508); not imm8.
LDB oo=0x168 finishes 168 LDB triad (H_501 dd=51, H_502 dd=52; H_500 already dd=50).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_501 | `71614ed8ee72059f9244f7935dc82742138ff7f26afddb6da756b8105659edb9` |
| H_502 | `b40ac7b90a6c8cb3d4f4c0eddd0bb4be7d544ebe35a847e994a31a07b7751cd7` |
| H_503 | `70dcc769354c9c590aed7416d470b91ce3cc91eabdcc12c2dda567949bd57769` |
| H_504 | `ae42aee20a8d8c9fa873f70fd087a2ff1fa8075b73fddae57eeec7d68d91c6c2` |
| H_505 | `7109bea20936a27a02373f431eaaa99ad49e81fc843b563184f145f1587119fb` |
| H_506 | `5b1652dbeda9a005e3b805181701200116762b682340e405bf20c50cb8be893e` |
| H_507 | `2d56b2a1e2d5c002d0994c309e2b846f88c0ba07f360ded43be97618c02f0a28` |
| H_508 | `f442c8a07cbb83826c533545bd5986c64332a717011034776c69921337416064` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5160_168`, `_scratch_ldb_5260_168`, `_scratch_addimm_h50_168`, `_scratch_addimm_h51_168`,
`_scratch_addimm_h52_168`, `_scratch_subimm_h50_168`, `_scratch_subimm_h51_168`, `_scratch_subimm_h52_168`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1FB`.. for H_501.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
