# parallel-batch-32 SPAWN · scratch-only (post body-extend-037)

> Continuous queue handoff from body-extend-037.
> Pin after Relock: `39d454a194359d1d682b0638381fa14cbdec617e707f26b2b2405e05be7f9ede` (abbrev `39d454a1…`).
> Handlers = 236 (H_00..H_229). Last selectors: 0xE4..0xEB = H_222..H_229.
> Source protocol: mirror `docs/auxdocs/parallel-batch-31-log.md` / batch-30 shape.
> No Task tool available on consolidator — this SPAWN is the handoff artifact.
> NOTE: body-extend-037 DDC PE `.text` measured EQUAL this beat — still EXPERIMENTAL · NON-GREEN; do not invent-green.

## Task: parallel-batch-32 (scratch-only)

1. Write `docs/auxdocs/parallel-batch-32-log.md`.
2. Pick 6–8 NEW scratch variants NOT already handlers in `yoyo/projects/yoyo.ty` (after H_222..H_229).
3. Skip D-1 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85 (MEMCPY opcodes — selector labels may reuse digits; do not emit MEMCPY body opcodes).
4. Prefer fresh slot/imm/alt-slot of INC/DEC/ADD-IMM/SUB-IMM/GET/SET/ADDV/ORV/SUBV/IMUL/CMP/LDB not duplicating H_48–H_229.
5. Scratch-only: do NOT touch yoyo.ty / lock / golden.js / self_test / main.
6. Each pick: write `_scratch_*.ty` + `_scratch_*.code.hex`; verify JS↔Rust byte-eq; record PASS/REJECT in log.
7. After scratches done: append "parent next = body-extend-038 serialize PASSes + 1 Relock" and **immediately spawn** body-extend-038 consolidation Task (same chain protocol), passing pin `39d454a194359d1d…` and the PASS list. If no Task tool: write `docs/auxdocs/body-extend-038-SPAWN.md`.
8. EXPERIMENTAL. No PROMPT edit. No git commit. No invent-green.

## Already locked (do not duplicate)

H_48 ADDV 51 50 · H_49 ORV 51 50 · H_50 SUBV 51 50 · H_51 GET 51 52 · H_52 ADDV 52 51 · H_53 SET 52 CAFEBABE · H_54 ORV 52 51 · H_55 SUBV 52 51 · H_56 IMUL 51 50 · H_57 IMUL 52 51 · H_58 CMP 51 50 · H_59 GET 52 50 · H_60 SET 51 DEADBEEF · H_61 LDB 51 60 08 · H_62 INC 51 · H_63 DEC 51 · H_64 ADD-IMM 51 07 · H_65 CMP 52 51 · H_66 ADDV 50 52 · H_67 GET 51 50 · H_68 SET 50 12345678 · H_69 LDB 52 60 08 · H_70 SUB-IMM 51 03 · H_71 DEC 52 · H_72 INC 52 · H_73 ORV 50 52 · H_74 SUBV 50 52 · H_75 GET 52 51 · H_76 SET 50 F00DBABE · H_77 CMP 52 50 · H_78 ADD-IMM 52 07 · H_79 SUB-IMM 52 03 · H_80 ADD-IMM 51 0A · H_81 SUB-IMM 50 05 · H_82 ORV 52 50 · H_83 SUBV 52 50 · H_84 ADDV 51 52 · H_85 IMUL 50 52 · H_86 SET 52 FEEDFACE · H_87 SET 51 AABBCCDD · H_88 GET 50 52 · H_89 CMP 50 52 · H_90 LDB 51 60 10 · H_91 IMUL 52 50 · H_92 ORV 51 52 · H_93 ADD-IMM 50 0F · H_94 SET 50 BEEFCAFE · H_95 SET 52 11111111 · H_96 SUB-IMM 50 08 · H_97 ADD-IMM 52 0A · H_98 LDB 52 60 10 · H_99 LDB 50 60 18 · H_100 SUBV 51 52 · H_101 ADDV 52 50 · H_102 CMP 51 52 · H_103 LDB 51 60 18 · H_104 LDB 52 60 18 · H_105 SET 51 C0FFEE00 · H_106 SUB-IMM 52 08 · H_107 IMUL 51 52 · H_108 ADD-IMM 50 14 · H_109 SET 50 C0FFEE00 · H_110 SET 52 DEADF00D · H_111 ADD-IMM 51 14 · H_112 SUB-IMM 51 0A · H_113 LDB 51 60 20 · H_114 LDB 52 60 20 · H_115 ADD-IMM 52 14 · H_116 SUB-IMM 50 0A · H_117 SET 51 DEADF00D · H_118 SET 50 FACEFEED · H_119 ADD-IMM 51 1E · H_120 SUB-IMM 52 0A · H_121 LDB 50 60 28 · H_122 SET 52 FACEFEED · H_123 ADD-IMM 50 1E · H_124 SUB-IMM 51 05 · H_125 LDB 51 60 28 · H_126 LDB 52 60 28 · H_127 LDB 50 60 30 · H_128 SET 51 BAADF00D · H_129 ADD-IMM 52 1E · H_130 SUB-IMM 50 14 · H_131 LDB 51 60 30 · H_132 SET 52 BAADF00D · H_133 SUB-IMM 52 14 · H_134 LDB 52 60 30 · H_135 LDB 50 60 38 · H_136 SET 50 0BADF00D · H_137 ADD-IMM 51 28 · H_138 SUB-IMM 51 1E · H_139 LDB 51 60 38 · H_140 ADD-IMM 50 28 · H_141 SUB-IMM 52 1E · H_142 LDB 52 60 38 · H_143 SET 51 FEEDC0DE · H_144 ADD-IMM 52 28 · H_145 SUB-IMM 50 1E · H_146 LDB 51 60 40 · H_147 LDB 52 60 40 · H_148 SET 52 FEEDC0DE · H_149 SUB-IMM 51 28 · H_150 SET 50 FEEDC0DE · H_151 ADD-IMM 50 32 · H_152 SUB-IMM 52 28 · H_153 LDB 50 60 48 · H_154 LDB 51 60 48 · H_155 LDB 52 60 48 · H_156 ADD-IMM 51 32 · H_157 SUB-IMM 50 28 · H_158 LDB 51 60 50 · H_159 LDB 52 60 50 · H_160 SET 51 CAFEF00D · H_161 ADD-IMM 52 32 · H_162 SUB-IMM 51 32 · H_163 SET 50 CAFEF00D · H_164 SUB-IMM 52 32 · H_165 ADD-IMM 50 3C · H_166 SET 52 CAFEF00D · H_167 LDB 50 60 58 · H_168 ADD-IMM 51 3C · H_169 SUB-IMM 50 3C · H_170 LDB 52 60 58 · H_171 LDB 51 60 58 · H_172 ADD-IMM 52 3C · H_173 SUB-IMM 51 3C · H_174 SET 50 DEADC0DE · H_175 LDB 51 60 60 · H_176 LDB 52 60 60 · H_177 ADD-IMM 50 40 · H_178 ADD-IMM 51 40 · H_179 ADD-IMM 52 40 · H_180 SUB-IMM 52 3C · H_181 SET 51 DEADC0DE · H_182 SET 52 DEADC0DE · H_183 LDB 50 60 68 · H_184 LDB 51 60 68 · H_185 LDB 52 60 68 · H_186 ADD-IMM 50 48 · H_187 ADD-IMM 51 48 · H_188 SUB-IMM 50 40 · H_189 SUB-IMM 51 40 · H_190 ADD-IMM 52 48 · H_191 SUB-IMM 52 40 · H_192 LDB 51 60 70 · H_193 LDB 52 60 70 · H_194 SET 50 C0DEC0DE · H_195 ADD-IMM 50 50 · H_196 SUB-IMM 51 48 · H_197 ADD-IMM 51 50 · H_198 ADD-IMM 52 50 · H_199 SUB-IMM 50 48 · H_200 SUB-IMM 52 48 · H_201 LDB 50 60 78 · H_202 SET 51 C0DEC0DE · H_203 ADD-IMM 50 58 · H_204 SUB-IMM 51 50 · H_205 LDB 51 60 78 · H_206 ADD-IMM 51 58 · H_207 ADD-IMM 52 58 · H_208 SUB-IMM 50 50 · H_209 SUB-IMM 52 50 · H_210 LDB 52 60 78 · H_211 SET 52 C0DEC0DE · H_212 ADD-IMM 50 60 · H_213 LDB 50 60 80 · H_214 ADD-IMM 51 60 · H_215 ADD-IMM 52 60 · H_216 SUB-IMM 50 58 · H_217 SUB-IMM 51 58 · H_218 LDB 51 60 80 · H_219 LDB 52 60 80 · H_220 SUB-IMM 52 58 · H_221 ADD-IMM 50 68 · H_222 ADD-IMM 51 68 · H_223 ADD-IMM 52 68 · H_224 SUB-IMM 50 60 · H_225 SUB-IMM 51 60 · H_226 SUB-IMM 52 60 · H_227 LDB 50 60 88 · H_228 LDB 51 60 88 · H_229 LDB 52 60 88

## Suggested fresh pick directions (non-binding)

- ADD-IMM / SUB-IMM fresh imm=70 triad (slots 50/51/52)
- LDB oo=90 next rung (dd=50/51/52 ss=60; expect imm32 26B)
- SUB-IMM imm=68 triad if not locked
- SET fresh imm / alt slot if not locked
- GET 50 51 · ORV 50 51 · SUBV 50 51 · ADDV 50 51 · IMUL 50 51 if not early-locked
- No AND/XOR. No MEMCPY / D-1 / D-2.
