# AUTO-SEMNATIVE — 语义编出编译器（打进 code.bin）

> 用户授权（2026-11-10）：**语义编出编译器**（去掉 SEMCOMP 外挂 trampoline 边界）。  
> stock C1 原生认识 Y1PL plant A1 → link 出自己。  
> **不是** Stage 17 / Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMCOMP.md
NORTH_STAR=无 PE poke；stock×T → output==stock；Y1PL 在 code.bin
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **N1** | Y1PL handler bake 进 `code.bin`；回打 plant | **绿** |
| **N2** | place stock（无 install poke）× `yoyo-semcomp-self.tyb` → EQUAL stock | **绿** |
| **N3** | hop2；compact MZ；stage-semnative GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-semnative.ps1
# stock_runs_T=EQUAL · hop2=EQUAL · GREEN
```

## 诚实

- **已真**：编译器本体（code.bin）语义处理 plant A1 tyb 并 link 出自己；门外不再装 trampoline；`stock×T==stock`（hop2 EQUAL）
- **边界**：仍是 plant A1 回收 bins + native link（不是从零写的高级语言源）；stock 入口 ReadFile 仅 0x8000，Y1PL 路径需再读满文件
- ≠ Thompson  

---

*2026-11-10 · COMPLETED=1 · GREEN · Y1PL baked in code.bin*
