# AUTO-TRUER28 — CreateFileA 打开路径面（C1 有 / 产物无）

> 用户授权（2026-11-10）：**继续**（TRUER27 后：VA 面 → CreateFile 面）。  
> **不是** Stage 17 / Thompson / 已接上读循环的完整打开主路径。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-TRUER27.md
NORTH_STAR=钉 C1/code 的 CreateFileA open-path face；全量产物 ABSENT；YOYO A1 face PE HIT
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **L1** | `yoyo-compiler-createfile-face.ty(b)` A1 编 CreateFile face | **绿** |
| **L2** | C1 PRESENT / product ABSENT / face PE HIT | **绿** |
| **L3** | pin + `stage-truer28` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-truer28.ps1
# createfile_face=PRESENT · product=ABSENT · face_pe HIT · GREEN
```

## 诚实

- 证明：真正 CreateFileA（GENERIC_READ + call [r15+0x10]）机器面在 C1，不在全量产物  
- YOYO 有限面能把该字节编进 PE，但**未**并入 yoyo.tyb，也**未**接上 ReadFile 主循环  
- ≠ Thompson  

---

*2026-11-10 · COMPLETED=1 · GREEN*
