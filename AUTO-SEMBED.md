# AUTO-SEMBED — 内容感知烙进 live product

> 用户授权（2026-11-11）：**继续破，没成功不许停**。  
> SEMHOST 已改 `yoyo.ty` OPENLOOP，但 **place/code.bin 主路径仍无内容感知**。  
> 本轨：在 `code.bin` 入口前缀烙 SEMHOST 闸——有输入则内容感知（十四切片 / 大输入默认 ret-PE）；无输入 jmp 原 H_00（place）。  
> **不是** Stage 17 / Thompson / 消尽 Win32 A1。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMHOST.md
NORTH_STAR=stock(code.bin)×十四 sem→expect；SEMHOST1∈stock；large→MZ
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **B1** | bake 前缀进 code.bin + plant-place；pin 同步 | **绿** |
| **B2** | place stock×十四 expect EQUAL；marker∈stock | **绿** |
| **B3** | 大输入仍 MZ；`stage-sembed` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-sembed.ps1
# stock_sem_all14=EQUAL · SEMHOST1=PRESENT · large_still_mz · GREEN
```

## 诚实（完整）

- **已真**：live product 入口对语义输入内容感知（`SEMHOST1`∈stock；十四 expect；place 仍走原 H_00）  
- **边界**：try_place 仍用旧 PE 几何（code.bin 变长后 `.text` 截断尾部）；大输入默认 ret-PE 而非全文编译；host 仍 A1 Win32/PE  
- ≠ Thompson  

---

*2026-11-11 · COMPLETED*
