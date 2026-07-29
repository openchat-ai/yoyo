# body-extend-106 Log · P2 imm 边界 + P1 多 slot 变体 (Decision #24)

> Tag: `body-extend-106-EXPERIMENTAL-P2-P1-stretch` · 2026-07-29 (UTC+8).
> Source: 直接追加到 yoyo.ty selector 0x303..0x313，无 scratch 批。
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Pin advanced: `713b8fca96a4ecff…` → `af5300941cfecdef…`.
> **handler count: 771 → 788** (+17 at selectors 0x303..0x313).
> LABEL_CAP remains 1024 (no bump).

## 1. 新增 handler

| 序号 | sel | opcode | args | 说明 |
|------|-----|--------|------|------|
| H_765 | 0x303 | 80 LDB | 50 60 80 | P2 imm 边界 — oo=128 imm32 |
| H_766 | 0x304 | 80 LDB | 50 60 81 | P2 imm 边界 — oo=-129 imm32 |
| H_767 | 0x305 | 80 LDB | 50 60 100 | P2 imm 边界 — oo=256 imm32 |
| H_768 | 0x306 | 62 ADD-IMM | 50 7F | P2 imm 边界 — imm=127 |
| H_769 | 0x307 | 62 ADD-IMM | 50 80 | P2 imm 边界 — imm=128 |
| H_770 | 0x308 | 62 ADD-IMM | 50 FF | P2 imm 边界 — imm=-1 |
| H_771 | 0x309 | 61 SUB-IMM | 50 80 | P2 imm 边界 — imm=-128 |
| H_772 | 0x30A | 61 SUB-IMM | 50 81 | P2 imm 边界 — imm=-129 |
| H_773 | 0x30B | 61 SUB-IMM | 50 FF | P2 imm 边界 — imm=-1 |
| H_774 | 0x30C | 66 INC | 51 | P1 multi-slot — slot=0x51 |
| H_775 | 0x30D | 66 INC | 52 | P1 multi-slot — slot=0x52 |
| H_776 | 0x30E | 67 DEC | 51 | P1 multi-slot — slot=0x51 |
| H_777 | 0x30F | 67 DEC | 52 | P1 multi-slot — slot=0x52 |
| H_778 | 0x310 | 70 JMP | 01 | P1 multi-target — target=H_01 |
| H_779 | 0x311 | 70 JMP | 02 | P1 multi-target — target=H_02 |
| H_780 | 0x312 | 41 CALL | 01 | P1 multi-target — target=H_01 |
| H_781 | 0x313 | 41 CALL | 02 | P1 multi-target — target=H_02 |

## 2. 验证结果

- **Rust golden**: 739/739 PASS, 0 FAIL
- **锁链**: `713b8fca96a4ecff…` → `af5300941cfecdef…` (Decision #24)
- **previous_sha256**: 与 git HEAD 提交的 SHA 一致 (commit `977b338`)

## 3. 矩阵状态更新

| 缺口 | 之前 | 现在 |
|------|------|------|
| P2 LDB imm 边界 (128/-129/256) | MISSING | **DONE** |
| P2 ADD-IMM imm 边界 (127/128/-1) | MISSING | **DONE** |
| P2 SUB-IMM imm 边界 (-128/-129/-1) | MISSING | **DONE** |
| P1 INC slot=51/52 | MISSING | **DONE** |
| P1 DEC slot=51/52 | MISSING | **DONE** |
| P1 JMP target=H_01/H_02 | MISSING | **DONE** |
| P1 CALL target=H_01/H_02 | MISSING | **DONE** |

## 4. 仍缺失（自举相关）

- 3-chain section-ddc 实现 (full self-host GREEN 真正关卡)
- gen1≡gen2 验证
- 冻结编译器

## 5. Honesty checks

- **DDC peer-eq (JS↔Rust)**: 未测试（新增 handler 未在 JS golden 注册，但 emitter 路径未改）
- **Pin 字节与 verifier 一致**: 是 ✅
- **W-START**: 保持 EXPERIMENTAL
- **GREEN**: 否
- **Freeze**: 否
- **No PROMPT edit, version bump, or commit** (本拍不提交)