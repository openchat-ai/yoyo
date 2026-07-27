# body-extend-064 SPAWN · consolidate parallel-batch-58

> Continuous queue handoff from parallel-batch-58 (scratch-only complete: 8 PASS / 0 REJECT).
> Current pin: `f4fa77a59520fda01683d3ceffe44de6886ba77752450ffbb0947e0ba15f0d96` (abbrev `f4fa77a5…`).
> Handlers = 443 (H_00..H_436). Last selectors: 0x1B4..0x1BA = H_430..H_436 (`40 1B4`.. via label-width A).
> Source: `docs/auxdocs/parallel-batch-58-log.md`.
> Prior consolidation: `docs/auxdocs/body-extend-063-log.md` / `docs/auxdocs/body-extend-063-SPAWN.md`.
> No Task tool available on scratch batch — this SPAWN is the handoff artifact.
> NOTE: body-extend-063 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors: `40 1BB`.. for H_437.. (label-width A already landed — multi-digit hex tokens OK; do not wrap).
> Selector `40 1BB`/`40 1BC` are HANDLER labels only — do NOT emit MEMCPY opcodes 0x84/0x85 in body.

## Task: body-extend-064 (serialize + Relock)

Mirror body-extend-063 / body-extend-062 protocol:

1. Hand-author append H_437..H_444 to `yoyo/projects/yoyo.ty` at selectors `40 1BB` .. `40 1C2` (multi-digit hex; do not wrap).
2. Promote fixtures from `_scratch_{ldb_5060_130,ldb_5160_130,ldb_5260_130,addimm_h50_130,addimm_h51_130,addimm_h52_130,subimm_h50_130,subimm_h51_130}` → `selfhost_min_*` + expected hex.
3. JS goldens + Rust self_test + Rust golden (expect prior+8 JS/Rust counts).
4. Verify + Relock once chaining from `f4fa77a59520fda01683d3ceffe44de6886ba77752450ffbb0947e0ba15f0d96`.
5. DDC via `verify-selfhost.ps1` (expect possible PE VirtualSize DIFFER; do not invent-green).
6. Write `docs/auxdocs/body-extend-064-log.md`.
7. Auto-spawn parallel-batch-59 scratch-only (continuous queue), or write `docs/auxdocs/parallel-batch-59-SPAWN.md` if no Task tool.

### PASS picks (ALL 8)

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
| H_437 | 0x1BB | 0x80 LDB | 50 60 130 | `498b87000300004881c030010000480fb60049898780020000c3` (26B) | `31e8129afecd8ba8` |
| H_438 | 0x1BC | 0x80 LDB | 51 60 130 | `498b87000300004881c030010000480fb60049898788020000c3` (26B) | `d0ba625ab36e77ee` |
| H_439 | 0x1BD | 0x80 LDB | 52 60 130 | `498b87000300004881c030010000480fb60049898790020000c3` (26B) | `addb80d146c8758b` |
| H_440 | 0x1BE | 0x62 ADD-IMM | 50 130 | `498b87800200004881c03001000049898780020000c3` (22B) | `dd2d08fe3b6bdad6` |
| H_441 | 0x1BF | 0x62 ADD-IMM | 51 130 | `498b87880200004881c03001000049898788020000c3` (22B) | `e7e0160df815fc7d` |
| H_442 | 0x1C0 | 0x62 ADD-IMM | 52 130 | `498b87900200004881c03001000049898790020000c3` (22B) | `ca98b1cd15714881` |
| H_443 | 0x1C1 | 0x61 SUB-IMM | 50 130 | `498b87800200004881e83001000049898780020000c3` (22B) | `c505da6e0e035cb4` |
| H_444 | 0x1C2 | 0x61 SUB-IMM | 51 130 | `498b87880200004881e83001000049898788020000c3` (22B) | `cfe5afe593eb6bf8` |

Store-disp check (slot→disp low byte): 50→80, 51→88, 52→90 — verified in all 8 pins above.
ADD-IMM imm=0x130 uses imm32 add (`48 81 c0`) → 22B pins (H_440..H_442); not imm8.
SUB-IMM imm=0x130 uses imm32 sub (`48 81 e8`) → 22B pins (H_443..H_444); not imm8.
LDB oo=0x130 uses imm32 add (`48 81 c0`) → 26B pins (H_437..H_439); starts 130 LDB triad.

Full sha256 (PASS pins):

| H | sha256 |
|---|--------|
| H_437 | `31e8129afecd8ba85a3d891940c5ddb399c390af72c855d53c8c5aa089b884de` |
| H_438 | `d0ba625ab36e77eefecf996a76d1a732510dce8824ddb07a5094ff4ebc56bd92` |
| H_439 | `addb80d146c8758ba56d2ce41ec4006e60a2a8f133de06d8e400f7a1eb4e1bfa` |
| H_440 | `dd2d08fe3b6bdad6401573da7bfeb58ec7e9829594d20006ecc0f94fb6998414` |
| H_441 | `e7e0160df815fc7df5cbdcc96dec33306970d8732d213baf52ee80eb81d7b5c2` |
| H_442 | `ca98b1cd157148817fb766b0d42e123e30f61adf56c3bbe1a1dde634d3d9e039` |
| H_443 | `c505da6e0e035cb495ca102022e0d4fede53f91954f9eb25d8d1d688e7132d9d` |
| H_444 | `cfe5afe593eb6bf8209a9d4ddfb1ae279997a30ffe4e12854dc17e3d0fa519ca` |

Scratch sources under `yoyo/tests/golden/`:
`_scratch_ldb_5060_130`, `_scratch_ldb_5160_130`, `_scratch_ldb_5260_130`, `_scratch_addimm_h50_130`,
`_scratch_addimm_h51_130`, `_scratch_addimm_h52_130`, `_scratch_subimm_h50_130`, `_scratch_subimm_h51_130`.

EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green. Do NOT touch MEMCPY opcode 0x84/0x85
(selector `40 1BB`.. for H_437.. is fine — label namespace, not opcode).
Do NOT touch D-1 0x20/0x50/0x51, D-2 0x64 (opcode).
