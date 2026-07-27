# parallel-batch-20 SPAWN · scratch-only (post body-extend-025)

> Continuous queue handoff from body-extend-025.
> Pin after Relock: `e59ddfae905aeea50f440cf46d763e29d869274866bc9b57cb3ab33886716fa2` (abbrev `e59ddfae…`).
> Handlers = 140 (H_00..H_133). Last selectors: 0x84..0x8B = H_126..H_133.
> Source protocol: mirror `docs/auxdocs/parallel-batch-19-log.md` / batch-18 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.

## Task: parallel-batch-20 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-20-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_126..H_133).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels 0x84/0x85 already used by H_126/H_127; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_133.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-026 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-026 consolidation Task (same chain protocol), passing pin `e59ddfae905aeea5…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-026-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48 ADDV 51 50 · H_49 ORV 51 50 · H_50 SUBV 51 50 · H_51 GET 51 52 · H_52 ADDV 52 51 · H_53 SET 52 CAFEBABE · H_54 ORV 52 51 · H_55 SUBV 52 51 · H_56 IMUL 51 50 · H_57 IMUL 52 51 · H_58 CMP 51 50 · H_59 GET 52 50 · H_60 SET 51 DEADBEEF · H_61 LDB 51 60 08 · H_62 INC 51 · H_63 DEC 51 · H_64 ADD-IMM 51 07 · H_65 CMP 52 51 · H_66 ADDV 50 52 · H_67 GET 51 50 · H_68 SET 50 12345678 · H_69 LDB 52 60 08 · H_70 SUB-IMM 51 03 · H_71 DEC 52 · H_72 INC 52 · H_73 ORV 50 52 · H_74 SUBV 50 52 · H_75 GET 52 51 · H_76 SET 50 F00DBABE · H_77 CMP 52 50 · H_78 ADD-IMM 52 07 · H_79 SUB-IMM 52 03 · H_80 ADD-IMM 51 0A · H_81 SUB-IMM 50 05 · H_82 ORV 52 50 · H_83 SUBV 52 50 · H_84 ADDV 51 52 · H_85 IMUL 50 52 · H_86 SET 52 FEEDFACE · H_87 SET 51 AABBCCDD · H_88 GET 50 52 · H_89 CMP 50 52 · H_90 LDB 51 60 10 · H_91 IMUL 52 50 · H_92 ORV 51 52 · H_93 ADD-IMM 50 0F · H_94 SET 50 BEEFCAFE · H_95 SET 52 11111111 · H_96 SUB-IMM 50 08 · H_97 ADD-IMM 52 0A · H_98 LDB 52 60 10 · H_99 LDB 50 60 18 · H_100 SUBV 51 52 · H_101 ADDV 52 50 · H_102 CMP 51 52 · H_103 LDB 51 60 18 · H_104 LDB 52 60 18 · H_105 SET 51 C0FFEE00 · H_106 SUB-IMM 52 08 · H_107 IMUL 51 52 · H_108 ADD-IMM 50 14 · H_109 SET 50 C0FFEE00 · H_110 SET 52 DEADF00D · H_111 ADD-IMM 51 14 · H_112 SUB-IMM 51 0A · H_113 LDB 51 60 20 · H_114 LDB 52 60 20 · H_115 ADD-IMM 52 14 · H_116 SUB-IMM 50 0A · H_117 SET 51 DEADF00D · H_118 SET 50 FACEFEED · H_119 ADD-IMM 51 1E · H_120 SUB-IMM 52 0A · H_121 LDB 50 60 28 · H_122 SET 52 FACEFEED · H_123 ADD-IMM 50 1E · H_124 SUB-IMM 51 05 · H_125 LDB 51 60 28 · H_126 LDB 52 60 28 · H_127 LDB 50 60 30 · H_128 SET 51 BAADF00D · H_129 ADD-IMM 52 1E · H_130 SUB-IMM 50 14 · H_131 LDB 51 60 30 · H_132 SET 52 BAADF00D · H_133 SUB-IMM 52 14

## Suggested fresh pick directions (non-binding)

- SET slot/imm not in locked SET set (e.g. slot 50 fresh imm ≠ FACEFEED/C0FFEE00/…; slot 51/52 ≠ BAADF00D/DEADF00D/…)
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 (pairs not in locked set)
- LDB dd/ss/oo variants (e.g. oo=38; dd=52 oo=30; dd=50 oo=38) ≠ locked LDB set including H_126/H_127/H_131
- ADD-IMM/SUB-IMM fresh (slot,imm); INC 50 / DEC 50
- No AND/XOR. No MEMCPY / D-1 / D-2.
