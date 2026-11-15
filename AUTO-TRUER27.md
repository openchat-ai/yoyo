# AUTO-TRUER27 — VirtualAlloc 打开路径面（C1 有 / 产物无）

> 用户授权（2026-11-10）：**继续**（TRUER26 后：relay 缺口 → 打开读循环面）。  
> **不是** Stage 17 / Thompson / 已能打开文件的完整主循环。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-TRUER26.md
NORTH_STAR=钉 C1/code 的 VirtualAlloc open-path face；全量产物 ABSENT；YOYO A1 open-face PE HIT
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **K1** | `yoyo-compiler-open-face.ty(b)` A1 编 VA face | **绿** |
| **K2** | C1 PRESENT / product ABSENT / face PE HIT | **绿** |
| **K3** | pin + `stage-truer27` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-truer27.ps1
# open_va_face=PRESENT · product=ABSENT · face_pe HIT · GREEN
```

## 诚实

- 证明：真正打开读缓冲（VirtualAlloc 0x8000）的机器面在 C1，不在全量产物  
- YOYO 有限面能把该字节编进 PE，但**未**并入 yoyo.tyb，也**未**接上 CreateFile→Read 主循环  
- ≠ Thompson  

---

*2026-11-10 · COMPLETED=1 · GREEN*
