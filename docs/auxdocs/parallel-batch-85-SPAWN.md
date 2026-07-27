# parallel-batch-85 SPAWN · scratch-only (post body-extend-090)

> Continuous queue handoff from body-extend-090.
> Pin after Relock: `63204ed031f1ad84c28688effab4ef4148b7c9e6277c1a08d68a7067dfe56aa1` (abbrev `63204ed0…`).
> Handlers = 659 (H_00..H_652). Last selectors: 0x28B..0x292 = H_645..H_652 (`40 28B`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-84-log.md` / batch-84 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-090 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x292 are `40 293`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-090 / batch-84: **LDB / ADD-IMM / SUB-IMM imm=1F0** (start next ladder).

## Task: parallel-batch-85 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-85-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_645..H_652).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_652.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-091 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-091 consolidation Task (same chain protocol), passing pin `63204ed0…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-091-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_644 as prior + H_645 LDB 51 60 1E8 · H_646 LDB 52 60 1E8 · H_647 ADD-IMM 50 1E8 · H_648 ADD-IMM 51 1E8 · H_649 ADD-IMM 52 1E8 · H_650 SUB-IMM 50 1E8 · H_651 SUB-IMM 51 1E8 · H_652 SUB-IMM 52 1E8

(Full H_48..H_644 list: see `docs/auxdocs/parallel-batch-84-SPAWN.md` §Already locked plus H_637..H_644; treat that list plus H_645..H_652 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Start deferred ladder:** LDB dd=50/51/52 ss=60 oo=1F0 (start 1F0 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=1F0 (start 1F0 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1F0 (start 1F0 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
