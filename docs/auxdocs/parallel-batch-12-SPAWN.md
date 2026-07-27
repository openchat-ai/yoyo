# parallel-batch-12 SPAWN · scratch-only (post body-extend-017)

> Continuous queue handoff from body-extend-017.
> Pin after Relock: `e879ce4bccbec29b1863265b1dad70c110b2abf74a48efce23ce9fcd403c1088` (abbrev `e879ce4b…`).
> Handlers = 75 (H_00..H_68). Last selectors: 0x44..0x4A = H_62..H_68.
> Source protocol: mirror `docs/auxdocs/parallel-batch-11-log.md` / batch-10 shape.

## Task: parallel-batch-12 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-12-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_62..H_68).
3. Skip MEMCPY 0x84/0x85; LDB oo matrix H_37/H_40..H_47.
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/SUBV/CMP not duplicating H_48–H_68.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-018 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-018 consolidation Task (same chain protocol), passing pin `e879ce4bccbec29b…` and the PASS list.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48 ADDV 51 50 · H_49 ORV 51 50 · H_50 SUBV 51 50 · H_51 GET 51 52 · H_52 ADDV 52 51 · H_53 SET 52 CAFEBABE · H_54 ORV 52 51 · H_55 SUBV 52 51 · H_56 IMUL 51 50 · H_57 IMUL 52 51 · H_58 CMP 51 50 · H_59 GET 52 50 · H_60 SET 51 DEADBEEF · H_61 LDB 51 60 08 · H_62 SUB-IMM 51 03 · H_63 CMP 52 51 · H_64 ADDV 50 52 · H_65 ORV 50 52 · H_66 GET 52 51 · H_67 SET 50 F00DBABE · H_68 INC 52
