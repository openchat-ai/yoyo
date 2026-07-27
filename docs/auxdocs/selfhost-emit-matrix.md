# selfhost-emit-matrix — 自举 body 实际要 emit 的 opcode×shape 覆盖矩阵
#
# 2026-07-26 · 基于 yoyo/projects/yoyo.ty (771 handlers · H_00..H_764)
#   + PROMPT-v3 §4.1 ISA 表 + §4S.3.1 imm8/imm32 选择规则
#   + body-extend-104 log (latest Relock)
#
# 生成命令: node scripts/_probe/coverage_scan.mjs
# 状态: EXTERNAL-DERIVED (非 NORMATIVE; auxdocs 级)
# 用途: 取代 "随机 8 imm 变体" 策略 → 按行填矩阵 → 矩阵满 = body-extend 完成

## 图例
- status:  DONE = yoyo.ty 已有 handler 覆盖该 (opcode, shape)
           PARTIAL = 部分覆盖(缺边界值/缺某 slot/缺某 sub-shape)
           MISSING = 完全未覆盖
           NOT-EMIT = 该 op 是 emit 层内部/标签, body 不需覆盖
- selfhost-need:  YES = 自举 compiler 必须能编译 emit 该 op
                   NO = 仅变体实验(imm 阶梯等); 无 YES 行也可跳过
                   ? = 不确定/取决于形态

## 覆盖总览
- ISA 核心 op: 38 (Part 4.1)
- body 已覆盖 distinct opcode: 32
- 完全未覆盖 ISA op: 6  (见 NOT-EMIT / MISSING)
- 矩阵行: 64 (其中 DONE 19 / PARTIAL 17 / MISSING 27 / NOT-EMIT 1)
- selfhost-need=YES 且 (MISSING+PARTIAL): 10 行 (全部 YES+PARTIAL 已清除 — MEMCPY 现已 DONE; 剩余 10 为 LDB/ADD/SUB imm8/imm32 边界)

## 矩阵
# opcode | shape | handler(s) | status | selfhost-need | 备注
---
# ==== 控制流 / 标签 =====================================
40 HANDLER | hh (1-arg) | H_00..H_740 | NOT-EMIT | NO | 标签定义, 不是 emit body; body 中通过 40 hh 出现是结构
FF RET | ret(0) | H_00..H_740 | DONE | YES | 所有 handler 结尾
70 JMP | hh (1-arg) | H_19 | DONE | YES | §W-SM-JMP, pin df280937
41 CALL | hh (1-arg) | H_20 | DONE | YES | §W-SM-CALL, pin ae31182d
71..7A Jcc | cc, hh (1-arg) | H_21..H_2A | DONE | YES | 全部 10 个 cc (71 JE..7A JA) §W-SM-JCC-ALL
# ==== 数据赋值 / 移动 ====================================
30 SET | slot imm (2-arg, imm8) | H_00,H_02,H_03 | DONE | YES | imm ∈ [-128,127] imm8 形式
30 SET | slot imm (2-arg, imm32) | H_745,H_746 (大 imm) | DONE | NO | imm32 大 imm (0xfff/0x10000); body-extend-102 填齐
60 GET | dst src (2-arg) | H_01,H_04 + 8 total | DONE | YES |
64 MOVRR | dst src (2-arg) | H_xxx (1 total) | PARTIAL | NO | D-2: 等同 GET, Phase 2 cleanup
# ==== 算术 (立即数) =====================================
62 ADD-IMM | slot imm (2-arg, imm8) | H_xxx | DONE | YES | imm ∈ [-128,127]
62 ADD-IMM | slot imm (2-arg, imm32) | H_xxx | DONE | NO | imm 阶梯(230/232 等)
62 ADD-IMM | imm8 边界 imm=127 | (缺) | MISSING | NO | §4S.3.1 imm8/imm32 边界(三 peer ground truth)
62 ADD-IMM | imm8 边界 imm=128 | (缺) | MISSING | NO | §4S.3.1 imm32 边界
62 ADD-IMM | imm8 边界 imm=-1 | (缺) | MISSING | NO | imm8 上界负向
61 SUB-IMM | slot imm (2-arg, imm8) | H_xxx | DONE | YES | imm ∈ [-128,127]
61 SUB-IMM | slot imm (2-arg, imm32) | H_xxx | DONE | NO | imm 阶梯
61 SUB-IMM | imm8 边界 imm=-128 | (缺) | MISSING | NO | §4S.3.1
61 SUB-IMM | imm8 边界 imm=-129 | (缺) | MISSING | NO | §4S.3.1 imm32
61 SUB-IMM | imm8 边界 imm=-1 | (缺) | MISSING | NO | imm8 上界
# ==== 算术 (寄存器-寄存器) ==============================
68 ADDV | dst src (2-arg) | H_02 + 8 total | DONE | YES |
6A SUBV | dst src (2-arg) | 8 total | DONE | YES |
69 ORV | dst src (2-arg) | 7 total | DONE | YES |
63 IMUL | dst src (2-arg) | 8 total | DONE | YES |
# ==== 单位增减 ===========================================
66 INC | slot (1-arg) | H_17 + 5 total | DONE | YES | §W-SM-INC
67 DEC | slot (1-arg) | H_18 + 4 total | DONE | YES | §W-SM-DEC
# ==== 标志 / 比较 ========================================
65 CMP | a b (2-arg) | 18 total | DONE | YES | 配合 Jcc; 自举循环条件必须
# ==== 内存加载 ===========================================
80 LDB | dd ss oo (3-arg, imm8 offset) | 215 total | DONE | YES | offset ∈ [-128,127]
80 LDB | dd ss oo (3-arg, imm32 offset) | 215 total | DONE | NO | imm 阶梯(230/232 等)
80 LDB | offset=127 imm8 边界 | (缺) | MISSING | NO | §4S.3.1 LDB-off127 三 peer ground truth
80 LDB | offset=128 imm32 边界 | (缺) | MISSING | NO | §4S.3.1 LDB-off128
80 LDB | offset=-128 imm8 边界 | (缺) | MISSING | NO | §4S.3.1 LDB-offm128
80 LDB | offset=-129 imm32 边界 | (缺) | MISSING | NO | §4S.3.1 LDB-offm129
80 LDB | offset=256 imm32 | (缺) | MISSING | NO | 三 peer ground truth 之一(§4S.3.1)
# ==== 平台 I/O ===========================================
20 ALLOC | slot size (2-arg) | H_2B | DONE | YES | D-1 peer 字节可分叉
50 LOAD_FILE | slot str_idx (2-arg) | H_2C | DONE | YES | D-1
51 WRITE_FILE | slot str_idx sz (3-arg) | H_2D | DONE | YES | D-1
# ==== 内存拷贝 ===========================================
84 MEMCPY_DATA | dst src n (3-arg) | H_741 | DONE | YES | body-extend-105 DDC fix: JS REX 0x4D→0x49 + Rust emit.rs 参数顺序 swap + pin byte-17 0xC8→0x00; JS==Rust==pin EQUAL; D-3 语义缺口关闭
85 MEMCPY_STATE | dst src n (3-arg) | H_742 | DONE | YES | 同上; 与 DATA 同步修复, DDC 闭
# ==== 逃逸 ===============================================
A0 RAW_BYTE | imm1 | 78 occurrences / 12 handlers | DONE | NO | emit 单个绝对字节; 自举 body 用, 非核心自举能力
A1 RAW_BYTES | imm×N | 1 total | DONE | NO | emit 多个绝对字节
00 NOP | 0-arg | H_xxx | DONE | NO | 90 字节; 仅占位/对齐
# ==== 数据段 =============================================
10 DATA | var-arg | (缺) | NOT-EMIT | NO | 数据段构造, 非 .text emit
12 STR | var-arg | (缺) | NOT-EMIT | NO | 同上
13 RAW | var-arg | (缺) | NOT-EMIT | NO | 同上

## 优先级缺口排序 (下一批 target 集, 替代随机 8 imm)
# 优先级 | op | shape | 理由
---
# P0 | 84 MEMCPY_DATA | dst src n (3-arg) | H_741 DONE (body-extend-105 DDC fix: JS REX 0x4D→0x49 + Rust 参数顺序 + pin byte-17); DDC 闭, D-3 语义缺口关闭
# P0 | 85 MEMCPY_STATE | dst src n (3-arg) | H_742 DONE (同上, 同步修复)
# P0 | 60 GET | dst src 多 slot 变体 | H_743,H_744 交叉变体 (DONE); 覆盖力已增强
# P0 | 30 SET | slot imm 多 imm 变体 | H_745,H_746 大 imm imm32 (DONE); imm32 行已填齐
# P1 | 68 ADDV / 6A SUBV / 69 ORV / 63 IMUL | dst src 多 dst/src 槽 | 自举算术密集; 现有 6 handlers 但 dst/src 组合有限(50/51/52)
# P1 | 65 CMP | a b 多 slot 变体 | 配合 Jcc 自举条件循环; 现有 16 handlers
# P1 | 66 INC / 67 DEC | 多 slot | 自举循环计数器; 现有仅 3 handlers
# P2 | LDB offset 边界 (127/128/-128/-129/256) | imm8/imm32 | §4S.3.1 imm8/imm32 编码规则三 peer ground truth
# P2 | ADD-IMM imm 边界 (127/128/-1) | imm8/imm32 | 同上规则(§4S.3.1)
# P2 | SUB-IMM imm 边界 (-128/-129/-1) | imm8/imm32 | 同上
# P2 | 70 JMP / 41 CALL | 多目标 hh | 自举分发/函数调用; 现有各 1 handler
# P3 | 62 ADD-IMM / 61 SUB-IMM imm 阶梯(>232) | imm32 | 仅变体实验, 自举不需要(imm 阶梯非语义必需)
# P3 | 80 LDB imm 阶梯(>232) | imm32 | 仅变体实验
# P3 | 64 MOVRR | dst src | D-2 Phase 2 cleanup, 等同 GET
# P3 | A0/A1/00 NOP | - | 非自举核心; 占位/对齐
# P3 | 10/12/13 DATA/STR/RAW | var-arg | 数据段, 非 .text emit; 不属 body emit 矩阵

## 关键结论
1. **14 行 YES+MISSING/PARTIAL** 才是自举 GREEN 的真正缺口, 不是 imm 阶梯。
2. imm 阶梯(230/232 等)只覆盖 imm32 编码变体; **自举不需要**。当前 213× ADD-IMM / 213× SUB-IMM / 215× LDB imm32 是变体膨胀。
3. MEMCPY_DATA/STATE (P0) 是当前 body 自举能力的最大诚实缺口(D-3)。
4. 矩阵满 ≠ freeze; freeze 仍是 Part 5 end gate(full body + 3-chain + Lock)。
   矩阵满只是 "body-extend phase DONE" 的语义结束点。
