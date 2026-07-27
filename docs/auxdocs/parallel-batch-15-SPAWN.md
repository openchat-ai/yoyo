# parallel-batch-15 SPAWN · scratch-only (post body-extend-020)

> Continuous queue handoff from body-extend-020.
> Pin after Relock: `c922e4d482e1f82e939d24a790483b1b35e791d864e6adf3c26fe49e2dbe2ce1` (abbrev `c922e4d4…`).
> Handlers = 100 (H_00..H_93). Last selectors: 0x5C..0x63 = H_86..H_93.
> Source protocol: mirror `docs/auxdocs/parallel-batch-14-log.md` / batch-13 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.

## Task: parallel-batch-15 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-15-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_86..H_93).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_93.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-021 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-021 consolidation Task (same chain protocol), passing pin `c922e4d482e1f82e…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-021-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48 ADDV 51 50 · H_49 ORV 51 50 · H_50 SUBV 51 50 · H_51 GET 51 52 · H_52 ADDV 52 51 · H_53 SET 52 CAFEBABE · H_54 ORV 52 51 · H_55 SUBV 52 51 · H_56 IMUL 51 50 · H_57 IMUL 52 51 · H_58 CMP 51 50 · H_59 GET 52 50 · H_60 SET 51 DEADBEEF · H_61 LDB 51 60 08 · H_62 INC 51 · H_63 DEC 51 · H_64 ADD-IMM 51 07 · H_65 CMP 52 51 · H_66 ADDV 50 52 · H_67 GET 51 50 · H_68 SET 50 12345678 · H_69 LDB 52 60 08 · H_70 SUB-IMM 51 03 · H_71 DEC 52 · H_72 INC 52 · H_73 ORV 50 52 · H_74 SUBV 50 52 · H_75 GET 52 51 · H_76 SET 50 F00DBABE · H_77 CMP 52 50 · H_78 ADD-IMM 52 07 · H_79 SUB-IMM 52 03 · H_80 ADD-IMM 51 0A · H_81 SUB-IMM 50 05 · H_82 ORV 52 50 · H_83 SUBV 52 50 · H_84 ADDV 51 52 · H_85 IMUL 50 52 · H_86 SET 52 FEEDFACE · H_87 SET 51 AABBCCDD · H_88 GET 50 52 · H_89 CMP 50 52 · H_90 LDB 51 60 10 · H_91 IMUL 52 50 · H_92 ORV 51 52 · H_93 ADD-IMM 50 0F

## Suggested fresh pick directions (non-binding)

- SET slot/imm not in {CAFEBABE@52, DEADBEEF@51, 12345678@50, F00DBABE@50, FEEDFACE@52, AABBCCDD@51}
- GET/CMP pairs not in locked (dst,src) set
- LDB dd/ss/oo variants (e.g. dd=52 oo=10, or oo=18/20) ≠ H_61/H_69/H_90
- ADDV/SUBV/ORV/IMUL remaining slot permutations
- ADD-IMM/SUB-IMM fresh (slot,imm) pairs; INC/DEC if any slot left
- No AND/XOR. No MEMCPY / D-1 / D-2.
