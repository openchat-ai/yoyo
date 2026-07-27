# body-extend-084 SPAWN · consolidate parallel-batch-78

> Continuous queue handoff from parallel-batch-78 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `45dff031e2acfa0ee40a932a4bca8709747e45bb1ac19f622fe0c477c4fe4a44` (abbrev `45dff031…`).
> Handlers = 603 (H_00..H_596). Last selectors: 0x253..0x25A = H_589..H_596 (`40 253`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-78-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-083-log.md` / `docs/auxdocs/body-extend-083-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-083 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 25B`.. for H_597.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 25B`/`40 25C` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-084 (serialize + Relock)

Mirror body-extend-083 / body-extend-082 protocol:

1. Hand-author append H_597..H_604 to `yoyo/projects/yoyo.ty` at selectors `40 25B` .. `40 262` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{subimm_h51_1B8,subimm_h52_1B8,ldb_5060_1C0,ldb_5160_1C0,ldb_5260_1C0,addimm_h50_1C0,addimm_h51_1C0,addimm_h52_1C0}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `45dff031e2acfa0ee40a932a4bca8709747e45bb1ac19f622fe0c477c4fe4a44`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-084-log.md`.
7. Auto-spawn parallel-batch-79 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-79-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_597 | 0x25B | 0x61 SUB-IMM | 51 1B8 | `498b87880200004881e8b801000049898788020000c3` (22B) | `e5f22d8e3828fbe4` |
| H_598 | 0x25C | 0x61 SUB-IMM | 52 1B8 | `498b87900200004881e8b801000049898790020000c3` (22B) | `fb3630917fc37295` |
| H_599 | 0x25D | 0x80 LDB | 50 60 1C0 | `498b87000300004881c0c0010000480fb60049898780020000c3` (26B) | `8953358138eb317e` |
| H_600 | 0x25E | 0x80 LDB | 51 60 1C0 | `498b87000300004881c0c0010000480fb60049898788020000c3` (26B) | `1759a1345d7af7ee` |
| H_601 | 0x25F | 0x80 LDB | 52 60 1C0 | `498b87000300004881c0c0010000480fb60049898790020000c3` (26B) | `bc6894d42acc6084` |
| H_602 | 0x260 | 0x62 ADD-IMM | 50 1C0 | `498b87800200004881c0c001000049898780020000c3` (22B) | `f6926af2f6dc5e89` |
| H_603 | 0x261 | 0x62 ADD-IMM | 51 1C0 | `498b87880200004881c0c001000049898788020000c3` (22B) | `8b90b51a7b7d5e6d` |
| H_604 | 0x262 | 0x62 ADD-IMM | 52 1C0 | `498b87900200004881c0c001000049898790020000c3` (22B) | `6c82474ed68d4ac8` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x1C0 uses imm32 add (`48 81 c0`) → 22B pins; not imm8.
SUB-IMM imm=0x1B8 uses imm32 sub (`48 81 e8`) → 22B pins; not imm8.
LDB oo=0x1C0 uses imm32 add (`48 81 c0`) → 26B pins.
SUB-IMM slot=51/52 imm=1B8 finishes 1B8 SUB triad (H_597/H_598; H_596 was slot=50).
LDB oo=0x1C0 starts 1C0 LDB triad (H_599/H_600/H_601 dd=50/51/52).
ADD-IMM slot=50/51/52 imm=1C0 starts deferred 1C0 ADD triad (H_602/H_603/H_604).

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_597 | `e5f22d8e3828fbe4596e012f550ab1fed4821b512a354463e287515bac9fe59e` |
| H_598 | `fb3630917fc37295b4f565e189083d072380118d3b3ad0fa070b19766670ecdc` |
| H_599 | `8953358138eb317e0172f330f7236e90798bf1191696a8264469afa9d88b36d7` |
| H_600 | `1759a1345d7af7eec3afd651f38cebca88cc9a916bc9d3b6b1ee70a0d52b19b2` |
| H_601 | `bc6894d42acc60846b7e08a7cf763c8f45a0050405f0d701595409a20282f747` |
| H_602 | `f6926af2f6dc5e890d94e44468a25abd66f381457c950d1212c821a739ffccc8` |
| H_603 | `8b90b51a7b7d5e6d75c25bed4669184293c74de28c60533001c10ad7ebc57f15` |
| H_604 | `6c82474ed68d4ac89252251890ac19f0f64bd257c2bbf3f8ab053d9c888e1210` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_subimm_h51_1B8`, `_scratch_subimm_h52_1B8`, `_scratch_ldb_5060_1C0`, `_scratch_ldb_5160_1C0`,
`_scratch_ldb_5260_1C0`, `_scratch_addimm_h50_1C0`, `_scratch_addimm_h51_1C0`, `_scratch_addimm_h52_1C0`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 25B`.. for H_597.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).

### Deferred for parallel-batch-79

- SUB-IMM slot=50/51/52 imm=1C0 (start 1C0 SUB triad)
- LDB 50/51/52 60 1C8 (start 1C8 LDB triad) if ladder continues
- ADD-IMM/SUB-IMM slot=50/51/52 imm=1C8 (start 1C8 ADD/SUB triads)
- SET / GET / ORV / SUBV / ADDV / IMUL fresh if not locked
