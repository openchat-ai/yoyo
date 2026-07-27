# parallel-batch-86 SPAWN · scratch-only (post body-extend-091)

> Continuous queue handoff from body-extend-091.
> Pin after Relock: `339bd482ae784eb8a80f7176ef5d7c6f3c90b0e491b08c6103512860ab5b918a` (abbrev `339bd482…`).
> Handlers = 667 (H_00..H_660). Last selectors: 0x293..0x29A = H_653..H_660 (`40 293`..`40 29A` via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-85-log.md` / batch-85 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-091 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x29A are `40 29B`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-091 / batch-85: **SUB-IMM slot=52 imm=1F0** (finish 1F0 SUB triad); **SET / GET / ORV / SUBV / ADDV / IMUL** fresh if not locked; next ladder if continuing.

## Task: parallel-batch-86 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-86-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_653..H_660).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_660.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-092 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-092 consolidation Task (same chain protocol), passing pin `339bd482…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-092-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_652 as prior + H_653 LDB 50 60 1F0 · H_654 LDB 51 60 1F0 · H_655 LDB 52 60 1F0 · H_656 ADD-IMM 50 1F0 · H_657 ADD-IMM 51 1F0 · H_658 ADD-IMM 52 1F0 · H_659 SUB-IMM 50 1F0 · H_660 SUB-IMM 51 1F0

(Full H_48..H_652 list: see `docs/auxdocs/parallel-batch-85-SPAWN.md` §Already locked plus H_645..H_652; treat that list plus H_653..H_660 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** SUB-IMM slot=52 imm=1F0 (finish 1F0 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- Next ladder if continuing (e.g. LDB/ADD-IMM/SUB-IMM imm beyond 1F0) — only fresh, not duplicates
- No AND/XOR. No MEMCPY / D-1 / D-2.
