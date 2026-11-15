# AUTO-ONESELF — 一个产物 × 一个 tyb → 自己

> 用户授权（2026-11-10）：**一个产物编一个 tyb 就再长出自己**。  
> 方程：`P(T)=P`（一次 CreateProcess）。  
> **不是** Stage 17 / Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-TYBSELF.md
NORTH_STAR=单文件 T；单次跑 P；output.exe 字节等于 P
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **O1** | place C1 → 安装 oneshot 入口；打 `yoyo-oneshot-self.tyb` | **绿** |
| **O2** | `P × T` 一次 → `output.exe == P` | **绿** |
| **O3** | hop2；compact-ret 仍走原路径 MZ | **绿** |

## 验收

```powershell
& .\scripts\stage-oneself.ps1
# oneshot_self=EQUAL · hop2=EQUAL · GREEN
```

## 诚实（必读）

- **已真**：一个产物、一个 tyb、一次跑，`output.exe` 字节等于产物；hop2 EQUAL  
- **边界**：T = `Y1SF` 内嵌 P 映像；入口为 PE `.text` 空隙 trampoline（不是 plant 语义「编出编译器逻辑」）  
- 多片 plant TYBSELF 仍在；本刀补的是单文件方程  
- ≠ Thompson  

---

> **2026-11-10 续刀**：语义编出编译器改走 `AUTO-SEMCOMP.md`。

*2026-11-10 · COMPLETED=1 · GREEN · P(T)=P*
