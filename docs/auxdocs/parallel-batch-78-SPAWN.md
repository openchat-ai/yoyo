# parallel-batch-78 SPAWN · scratch-only (post body-extend-083)

> Continuous queue handoff from body-extend-083.
> Pin after Relock: `45dff031e2acfa0ee40a932a4bca8709747e45bb1ac19f622fe0c477c4fe4a44` (abbrev `45dff031…`).
> Handlers = 603 (H_00..H_596). Last selectors: 0x253..0x25A = H_589..H_596 (`40 253`.. via label-width A; LABEL_CAP=1024).
> Source protocol: mirror `docs/auxdocs/parallel-batch-77-log.md` / batch-77 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-083 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.
> Next selectors after 0x25A are `40 25B`.. — HANDLER labels only (multi-digit hex tokens; do not wrap).
> Deferred from body-extend-083 / batch-77: **SUB-IMM slot=51/52 imm=1B8** (finish 1B8 SUB triad); **LDB 50/51/52 60 1C0** (start 1C0 LDB triad); **ADD-IMM/SUB-IMM slot=50/51/52 imm=1C0** (start 1C0 ADD/SUB triads).

## Task: parallel-batch-78 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-78-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_589..H_596).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_596.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main / label-width peers.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-084 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-084 consolidation Task (same chain protocol), passing pin `45dff031…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-084-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48..H_588 as prior + H_589 SUB-IMM 52 1B0 · H_590 LDB 50 60 1B8 · H_591 LDB 51 60 1B8 · H_592 LDB 52 60 1B8 · H_593 ADD-IMM 50 1B8 · H_594 ADD-IMM 51 1B8 · H_595 ADD-IMM 52 1B8 · H_596 SUB-IMM 50 1B8

(Full H_48..H_588 list: see `docs/auxdocs/parallel-batch-77-SPAWN.md` §Already locked plus H_581..H_588; treat that list plus H_589..H_596 as occupied.)

## Suggested fresh pick directions (non-binding)

- **Finish deferred:** SUB-IMM slot=51/52 imm=1B8 (finish 1B8 SUB triad; imm32 22B)
- **Start deferred:** LDB dd=50/51/52 ss=60 oo=1C0 (start 1C0 LDB triad; imm32 26B)
- **Start deferred:** ADD-IMM slot=50/51/52 imm=1C0 (start 1C0 ADD triad; imm32 22B)
- **Start deferred:** SUB-IMM slot=50/51/52 imm=1C0 (start 1C0 SUB triad; imm32 22B)
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
