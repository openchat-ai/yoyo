# AUTO-TRUER29 — ReadFile 打开路径面（C1 有 / 产物无）

> 用户授权（2026-11-10）：**继续**（TRUER28 后：CreateFile 面 → ReadFile 面）。  
> **不是** Stage 17 / Thompson / 已接上完整读写主循环。

```
ACTIVE=1
COMPLETED=0
PARENT=AUTO-TRUER28.md
NORTH_STAR=钉 C1/code 的 ReadFile open-path face；全量产物 ABSENT；YOYO A1 face PE HIT
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **M1** | `yoyo-compiler-readfile-face.ty(b)` A1 编 ReadFile face | 待 |
| **M2** | C1 PRESENT / product ABSENT / face PE HIT | 待 |
| **M3** | pin + `stage-truer29` GREEN | 待 |

## 验收

```powershell
& .\scripts\stage-truer29.ps1
# readfile_face=PRESENT · product=ABSENT · face_pe HIT · GREEN
```

## 诚实

- 证明：真正 ReadFile（call [r15+0x18]）机器面在 C1，不在全量产物  
- YOYO 有限面能把该字节编进 PE，但**未**并入 yoyo.tyb，也**未**接上写回/编译主循环  
- ≠ Thompson  

---

*2026-11-10 · ACTIVE=1*
