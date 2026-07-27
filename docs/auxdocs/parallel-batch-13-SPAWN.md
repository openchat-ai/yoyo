# parallel-batch-13 SPAWN · scratch-only (post body-extend-018)

> Continuous queue handoff from body-extend-018.
> Pin after Relock: `e8603542fb13c5f027b3bea34b63aa0b8b20e82bb087ffe06568bd8193b401a2` (abbrev `e8603542…`).
> Handlers = 84 (H_00..H_77). Last selectors: 0x4C..0x53 = H_70..H_77.
> Source protocol: mirror `docs/auxdocs/parallel-batch-12-log.md` / batch-11 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.

## Task: parallel-batch-13 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-13-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_70..H_77).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_77.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-019 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-019 consolidation Task (same chain protocol), passing pin `e8603542fb13c5f0…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-019-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48 ADDV 51 50 · H_49 ORV 51 50 · H_50 SUBV 51 50 · H_51 GET 51 52 · H_52 ADDV 52 51 · H_53 SET 52 CAFEBABE · H_54 ORV 52 51 · H_55 SUBV 52 51 · H_56 IMUL 51 50 · H_57 IMUL 52 51 · H_58 CMP 51 50 · H_59 GET 52 50 · H_60 SET 51 DEADBEEF · H_61 LDB 51 60 08 · H_62 INC 51 · H_63 DEC 51 · H_64 ADD-IMM 51 07 · H_65 CMP 52 51 · H_66 ADDV 50 52 · H_67 GET 51 50 · H_68 SET 50 12345678 · H_69 LDB 52 60 08 · H_70 SUB-IMM 51 03 · H_71 DEC 52 · H_72 INC 52 · H_73 ORV 50 52 · H_74 SUBV 50 52 · H_75 GET 52 51 · H_76 SET 50 F00DBABE · H_77 CMP 52 50
