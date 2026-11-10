# AUTO-TRUER30 — CloseHandle 打开路径面（C1 有 / 产物无）

> 用户授权（2026-11-10）：**继续**（TRUER29 后：ReadFile 面 → CloseHandle 面）。  
> **不是** Stage 17 / Thompson / 已接上完整读写主循环。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-TRUER29.md
NORTH_STAR=钉 C1/code 的 CloseHandle open-path face；全量产物 ABSENT；YOYO A1 face PE HIT
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **N1** | `yoyo-compiler-closehandle-face.ty(b)` A1 编 CloseHandle face | **绿** |
| **N2** | C1 PRESENT / product ABSENT / face PE HIT | **绿** |
| **N3** | pin + `stage-truer30` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-truer30.ps1
# closehandle_face=PRESENT · product=ABSENT · face_pe HIT · GREEN
```

## 诚实

- 证明：真正 CloseHandle（call [r15+0x20]）机器面在 C1，不在全量产物  
- YOYO 有限面能把该字节编进 PE，但**未**并入 yoyo.tyb；开环 VA→CreateFile→ReadFile→Close 仍未进产物  
- ≠ Thompson  

---

*2026-11-10 · COMPLETED=1 · GREEN*
