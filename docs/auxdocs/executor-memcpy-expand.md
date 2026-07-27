# Executor MEMCPY 扩展 + LEA 编码修复 (Decision #22)

> Tag: `executor-memcpy-expand` + `lea-scale-fix` · 2026-07-26 (UTC+8)。
> **W-START: EXPERIMENTAL · NON-GREEN · NOT-FREEZE。**
> Pin (yoyo.ty.lock) advanced: `9011774e…` → `20391de3…` (Decision #22)。
> handler count: 771 (unchanged)。

---

## 1. Executor 扩展 (yoyo-rust/executor/src/cpu.rs)

### 1.1 新增寄存器
- RegId: `Rsi, Rdi` (原只有 Rax, Rcx, R15)
- Cpu struct: `rsi: u64, rdi: u64` 字段
- `map_reg()`: 6→Rsi, 7→Rdi, 14→Rsi (REX.B)
- `low3_reg()`: 6→Rsi, 7→Rdi
- `read_reg()` / `write_reg()`: 处理 Rsi, Rdi

### 1.2 新增 0xFC dispatch
- 单字节 opcode `FC` = rep movsb
- 循环复制 `rcx` 字节从 `[rsi]` → `[rdi]`
- 后 rsi+=rcx, rdi+=rcx, rcx=0
- 使用 mmu read_u8/write_u8 逐字节安全复制

### 1.3 decode_state_modrm 扩展
- reg=6 → Rsi, reg=7 → Rdi (原只认 0/Rax, 1/Rcx)

### 1.4 单元测试
- 8/8 PASS (原有 + 无回归)

---

## 2. LEA 编码修复 (JS+Rust)

### 2.1 问题
MEMCPY_STATE 的 `lea rdi,[r15+rdi*8]` 编码中 scale=8 应为 `(3<<6)` = `0xC0`，但 JS 和 Rust 都写了 `(5<<6)` 在 u8 截断后变成 `0x40`（scale=2）。

### 2.2 修复
- JS `encode-x64.js`: `modrm=0x00→0x04` (SIB), `scale=5→3`
- Rust `assembler.rs`: `modrm=0x00→0x04` (SIB), `scale=5→3`
- 影响: MEMCPY_STATE 的 LEA 字节从 `4d8bc7` → `498b3c` 和 `4d8bc6` → `498b34`

### 2.3 DDC
- JS==Rust==pin EQUAL (17420 bytes)
- golden test: 739/739 PASS

---

## 3. 执行器运行结果

`scripts/_probe/_attempt_n5b/run-locked.cmd`:
- steps: 3（H_00 执行后 halt，未触发 MEMCPY 调用——这是预期行为）
- exit: HALT at 0x1012
- 需要构造一个调用 H_741/H_742 的测试脚本来验证 MEMCPY 执行路径

---

## 4. 矩阵状态

unchanged: DONE 19 / PARTIAL 17 / MISSING 27 / NOT-EMIT 1
selfhost-need=YES 且 (MISSING+PARTIAL): 10 行 (全部 P2 imm 边界, selfhost-need=NO)