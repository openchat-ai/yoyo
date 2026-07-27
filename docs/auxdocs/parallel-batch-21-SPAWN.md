# parallel-batch-21 SPAWN · scratch-only (post body-extend-026)

> Continuous queue handoff from body-extend-026.
> Pin after Relock: `6c42f38cd61a0603f8892cbfdf36ab3966be5f894ce6a053c403d014507a6cc7` (abbrev `6c42f38c…`).
> Handlers = 148 (H_00..H_141). Last selectors: 0x8C..0x93 = H_134..H_141.
> Source protocol: mirror `docs/auxdocs/parallel-batch-20-log.md` / batch-19 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.

## Task: parallel-batch-21 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-21-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_134..H_141).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels 0x84/0x85 already used by H_126/H_127; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_141.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-027 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-027 consolidation Task (same chain protocol), passing pin `6c42f38cd61a0603…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-027-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48 ADDV 51 50 · H_49 ORV 51 50 · H_50 SUBV 51 50 · H_51 GET 51 52 · H_52 ADDV 52 51 · H_53 SET 52 CAFEBABE · H_54 ORV 52 51 · H_55 SUBV 52 51 · H_56 IMUL 51 50 · H_57 IMUL 52 51 · H_58 CMP 51 50 · H_59 GET 52 50 · H_60 SET 51 DEADBEEF · H_61 LDB 51 60 08 · H_62 INC 51 · H_63 DEC 51 · H_64 ADD-IMM 51 07 · H_65 CMP 52 51 · H_66 ADDV 50 52 · H_67 GET 51 50 · H_68 SET 50 12345678 · H_69 LDB 52 60 08 · H_70 SUB-IMM 51 03 · H_71 DEC 52 · H_72 INC 52 · H_73 ORV 50 52 · H_74 SUBV 50 52 · H_75 GET 52 51 · H_76 SET 50 F00DBABE · H_77 CMP 52 50 · H_78 ADD-IMM 52 07 · H_79 SUB-IMM 52 03 · H_80 ADD-IMM 51 0A · H_81 SUB-IMM 50 05 · H_82 ORV 52 50 · H_83 SUBV 52 50 · H_84 ADDV 51 52 · H_85 IMUL 50 52 · H_86 SET 52 FEEDFACE · H_87 SET 51 AABBCCDD · H_88 GET 50 52 · H_89 CMP 50 52 · H_90 LDB 51 60 10 · H_91 IMUL 52 50 · H_92 ORV 51 52 · H_93 ADD-IMM 50 0F · H_94 SET 50 BEEFCAFE · H_95 SET 52 11111111 · H_96 SUB-IMM 50 08 · H_97 ADD-IMM 52 0A · H_98 LDB 52 60 10 · H_99 LDB 50 60 18 · H_100 SUBV 51 52 · H_101 ADDV 52 50 · H_102 CMP 51 52 · H_103 LDB 51 60 18 · H_104 LDB 52 60 18 · H_105 SET 51 C0FFEE00 · H_106 SUB-IMM 52 08 · H_107 IMUL 51 52 · H_108 ADD-IMM 50 14 · H_109 SET 50 C0FFEE00 · H_110 SET 52 DEADF00D · H_111 ADD-IMM 51 14 · H_112 SUB-IMM 51 0A · H_113 LDB 51 60 20 · H_114 LDB 52 60 20 · H_115 ADD-IMM 52 14 · H_116 SUB-IMM 50 0A · H_117 SET 51 DEADF00D · H_118 SET 50 FACEFEED · H_119 ADD-IMM 51 1E · H_120 SUB-IMM 52 0A · H_121 LDB 50 60 28 · H_122 SET 52 FACEFEED · H_123 ADD-IMM 50 1E · H_124 SUB-IMM 51 05 · H_125 LDB 51 60 28 · H_126 LDB 52 60 28 · H_127 LDB 50 60 30 · H_128 SET 51 BAADF00D · H_129 ADD-IMM 52 1E · H_130 SUB-IMM 50 14 · H_131 LDB 51 60 30 · H_132 SET 52 BAADF00D · H_133 SUB-IMM 52 14 · H_134 LDB 52 60 30 · H_135 LDB 50 60 38 · H_136 SET 50 0BADF00D · H_137 ADD-IMM 51 28 · H_138 SUB-IMM 51 1E · H_139 LDB 51 60 38 · H_140 ADD-IMM 50 28 · H_141 SUB-IMM 52 1E

## Suggested fresh pick directions (non-binding)

- SET slot/imm not in locked SET set (e.g. slot 51/52 fresh imm ≠ BAADF00D/DEADF00D/…; slot 50 ≠ 0BADF00D/FACEFEED/…)
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 (pairs not in locked set)
- LDB dd/ss/oo variants (e.g. oo=40; dd=52 oo=38; dd=50 oo=40) ≠ locked LDB set including H_134/H_135/H_139
- ADD-IMM/SUB-IMM fresh (slot,imm) e.g. slot 52 imm=28; slot 50/51 imm=32; INC 50 / DEC 50
- No AND/XOR. No MEMCPY / D-1 / D-2.
