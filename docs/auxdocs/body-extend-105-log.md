# body-extend-105 Log · 真实 MEMCPY_DATA / MEMCPY_STATE 实现 (Decision #20 + Fix)

> Tag: `body-extend-105-EXPERIMENTAL-real-memcpy` · 2026-07-26 (UTC+8)。
> 来源：子代理执行 "MEMCPY real implementation" + 后续 DDC 修平。
> **W-START: EXPERIMENTAL · NON-GREEN · NOT-FREEZE。**
> Pin (yoyo.ty.lock) advanced: `1f04fa7a3e6a9c62…` → `9011774e90305f64…` (Decision #21)。
> handler count: 771 (unchanged; H_741/H_742 已存在，仅 body 字节更新)。
> LABEL_CAP: 1024 (unchanged)。

---

## 1. 目标

替换 H_741 (0x84 MEMCPY_DATA) / H_742 (0x85 MEMCPY_STATE) 的 stub `C3`，使用 canonical `rep movsb` emit。
- H_741 args: `dst=0x50 src=0x51 n=0x40` → 22B
- H_742 args: `dst=0x50 src=0x51 n=0x40` → 36B

---

## 2. 修复的三道伤

### 2.1 JS `loadState` REX.R 位 (encode-x64.js)
- 原: `destRex=1` 硬编码 → REX 0x4D (RDI/RSI 强制 REX.R=1)
- Rust: `dest.rex_bit()` 对 RSI/RDI 返回 FALSE → REX 0x49
- 修: MEMCPY 路径 `destRex=0` 匹配 Rust → 0x49
- 影响: MEMCPY_DATA + MEMCPY_STATE 中所有 loadState 调用

### 2.2 JS `leaR15Scale8` REX.R 位 (encode-x64.js)
- 原: `rexR=1` → LEA REX 0x4D
- Rust `emit_lea_r15_scale8`: `rex_wrxb(true, false, false, true)` → 0x49
- 修: lea 调用 `rexR=0` 匹配 Rust → 0x49
- 影响: MEMCPY_STATE 中两个 LEA 指令

### 2.3 Rust `emit.rs:127` 参数顺序 (emit.rs)
- 原: `emit_memcpy_data(a(0), a(1), a(2))` — TIR args=[dst,src,n] 但 assembler 签名是 `(src,dst,n)`
- 修: `emit_memcpy_data(a(1), a(0), a(2))` — 交换 src/dst 对齐签名
- 影响: MEMCPY_DATA + MEMCPY_STATE 的 Rust 编译产出

### 2.4 yoyo.ty H_741 pin 注释 byte-17
- 原: `498b8fc8020000fc` (byte 17 = 0xC8, mod=11/rax 直接寻址)
- 修: `498b8f00020000fc` (byte 17 = 0x00, mod=10/r15+disp32=0x200)
- 影响: 注释对齐 verifier 实际产出

---

## 3. DDC 结果: **EQUAL** ✅

| 比较 | 结果 |
|------|------|
| JS encoder == Rust verifier `.text` | **EQUAL** |
| Rust verifier == yoyo.ty pin 注释 | **EQUAL** |
| full binary (17420 bytes) | JS==Rust==pin 三方一致 |
| golden test | 739/739 PASS |

### 三向字节

| handler | 字节 (hex) |
|---------|-----------|
| MEMCPY_DATA (22B) | `498bb788020000498bbf80020000498b8f00020000fc` |
| MEMCPY_STATE (36B) | `498bbf80020000498b387f000000498bb788020000498b3077000000498b8f00020000fc` |

---

## 4. Lock 状态

- `yoyo/tests/yoyo.ty.lock`: `sha256 = 9011774e90305f64…` (previous `1f04fa7a3e6a9c62…`)
- Decision #21 Relock。链自 body-extend-105 Decision #20 pin。
- `verify-yoyo-ty.mjs`: ✅ PASS

---

## 5. 矩阵状态

| opcode | handler | 之前 | 现在 | 说明 |
|--------|---------|------|------|------|
| 84 MEMCPY_DATA | H_741 | PARTIAL | **DONE** | 真实 `rep movsb` 字节 + DDC EQUAL |
| 85 MEMCPY_STATE | H_742 | PARTIAL | **DONE** | 同上 |

矩阵 flips: DONE 17→19, PARTIAL 19→17。selfhost-need=YES 且 (MISSING+PARTIAL): 12→10 行。

---

## 6. Honesty checks

- **DDC peer-eq (JS↔Rust)**: EQUAL ✅ — D-3 语义缺口关闭
- **Pin 字节与 verifier 一致**: 是 ✅
- **W-START**: 保持 EXPERIMENTAL。
- **GREEN**: 否 (仍需 full self-host 3-chain + 执行器验证)。
- **Freeze**: 否。
- **No PROMPT edit, no version bump, no commit**.
- **Lock 已更新**: Decision #21。

---

## 7. 下一步

矩阵 selfhost-need=YES 行除 LDB/ADD/SUB imm8/imm32 边界外已全部 DONE。下一拍可选:
- (a) 填 LDB/ADD/SUB imm 边界 (P2, selfhost-need=NO, 非自举必需)
- (b) 切回自举体执行器验证 (N5 executor)
- (c) 暂停