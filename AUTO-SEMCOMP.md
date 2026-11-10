# AUTO-SEMCOMP — 语义编出编译器

> 用户授权（2026-11-10）：**语义编出编译器，继续吧**。  
> 单文件 plant-A1 tyb → 进程内 unpack → link → 编译器。  
> **不是** Stage 17 / Thompson；**不是** Y1SF 映像写回（ONESELF）。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-ONESELF.md
NORTH_STAR=一个 T（Y1PL+plant A1）；一次跑；output.exe == stock C1 编译器
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **C1** | `yoyo-semcomp-self.tyb` = Y1PL + A1(code\|\|data) | **绿** |
| **C2** | 产物 unpack A1 → bins → `try_place_bins` → 编译器 | **绿** |
| **C3** | `output == stock C1`；hop2；compact 仍 MZ | **绿** |

## 验收

```powershell
& .\scripts\stage-semcomp.ps1
# compiler_from_tyb=EQUAL · hop2=EQUAL · GREEN
```

## 诚实（必读）

- **已真**：一个 tyb 里是 **plant A1 语义**；一次跑 **link 出编译器** 且 `== stock C1`（`c525b1cc…`）  
- **边界**：入口仍是 PE `.text` 空隙 trampoline；runner ≠ stock，**输出**才是编译器固定点  
- ≠ ONESELF 映像写回；≠ Thompson  

---

*2026-11-10 · COMPLETED=1 · GREEN · plant A1 → compiler*
