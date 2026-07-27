# body-extend-083 SPAWN · consolidate parallel-batch-77

> Continuous queue handoff from parallel-batch-77 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `05a3a9c6693fa65c20f47a3eab1bc536c5e5fe0a168381faf0cf72330ca58056` (abbrev `05a3a9c6…`).
> Handlers = 595 (H_00..H_588). Last selectors: 0x24B..0x252 = H_581..H_588 (`40 24B`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-77-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-082-log.md` / `docs/auxdocs/body-extend-082-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-082 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 253`.. for H_589.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 253`/`40 254` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-083 (serialize + Relock)

Mirror body-extend-082 / body-extend-081 protocol:

1. Hand-author append H_589..H_596 to `yoyo/projects/yoyo.ty` at selectors `40 253` .. `40 25A` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h52_1B0,ldb_5060_1B8,ldb_5160_1B8,ldb_5260_1B8,addimm_h50_1B8,addimm_h51_1B8,addimm_h52_1B8,subimm_h50_1B8}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `05a3a9c6693fa65c20f47a3eab1bc536c5e5fe0a168381faf0cf72330ca58056`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-083-log.md`.
7. Auto-spawn parallel-batch-78 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-78-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_589 | 0x253 | 0x61 SUB-IMM | 52 1B0 | `498b87900200004881e8b001000049898790020000c3` (22B) | `6b09f5d585880e4e` |
| H_590 | 0x254 | 0x80 LDB | 50 60 1B8 | `498b87000300004881c0b8010000480fb60049898780020000c3` (26B) | `991bc7cddb01b0d2` |
| H_591 | 0x255 | 0x80 LDB | 51 60 1B8 | `498b87000300004881c0b8010000480fb60049898788020000c3` (26B) | `eb823184d5b340f6` |
| H_592 | 0x256 | 0x80 LDB | 52 60 1B8 | `498b87000300004881c0b8010000480fb60049898790020000c3` (26B) | `4769bc5c1af2f770` |
| H_593 | 0x257 | 0x62 ADD-IMM | 50 1B8 | `498b87800200004881c0b801000049898780020000c3` (22B) | `8670afebb32cc65e` |
| H_594 | 0x258 | 0x62 ADD-IMM | 51 1B8 | `498b87880200004881c0b801000049898788020000c3` (22B) | `46ee1e357ab8ae14` |
| H_595 | 0x259 | 0x62 ADD-IMM | 52 1B8 | `498b87900200004881c0b801000049898790020000c3` (22B) | `a95def3bbb47b285` |
| H_596 | 0x25A | 0x61 SUB-IMM | 50 1B8 | `498b87800200004881e8b801000049898780020000c3` (22B) | `ab8ef8aa14a41432` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1B8 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1B0/0x1B8 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
SUB-IMM slot=52 imm=1B0 finishes 1B0 SUB triad (H_589).
LDB oo=0x1B8 starts 1B8 LDB triad (H_590/H_591/H_592 dd=50/51/52).
ADD-IMM slot=50/51/52 imm=1B8 starts deferred 1B8 ADD triad (H_593/H_594/H_595).
SUB-IMM slot=50 imm=1B8 starts deferred 1B8 SUB triad (H_596; slots 51/52 deferred).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_589 | `6b09f5d585880e4ec49c5e9c396bb9e39446613f5363674617689869bab33bfd` |
| H_590 | `991bc7cddb01b0d29dfca5fb319b68256961b1812f00b88ae985882b39c38998` |
| H_591 | `eb823184d5b340f60c461778e7ab3ca2948f25a3d027fced281183e2a49913ac` |
| H_592 | `4769bc5c1af2f770c1056bfb8170c204314d60ca96624d72323238d6c7ecdca2` |
| H_593 | `8670afebb32cc65e32e1ba06b08b165b09792c2cf045673023d42964c261b24a` |
| H_594 | `46ee1e357ab8ae141ae1fa9df0aa3354ca8f50bece5207409dbfc33237b3f516` |
| H_595 | `a95def3bbb47b2852075f9b5eaa1b01aa135e2f472516fd07162777a3915252b` |
| H_596 | `ab8ef8aa14a41432c50fdfcce3a68657f56c95ef810e7b0fa42ccc36b3374b31` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h52_1B0`, `_scratch_ldb_5060_1B8`, `_scratch_ldb_5160_1B8`, `_scratch_ldb_5260_1B8`,
`_scratch_addimm_h50_1B8`, `_scratch_addimm_h51_1B8`, `_scratch_addimm_h52_1B8`, `_scratch_subimm_h50_1B8`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 253`.. for H_589.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-78

- SUB-IMM slot=51/52 imm=1B8 (finish 1B8 SUB triad)
- LDB 50/51/52 60 1C0 (start 1C0 LDB triad) if ladder continues
- ADD-IMM/SUB-IMM slot=50/51/52 imm=1C0 (start 1C0 ADD/SUB triads)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
