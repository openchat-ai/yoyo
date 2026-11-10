# AUTO-TRUER20 — place 工具缺席 bins 时自动派生

> 用户授权（2026-11-10）：**继续**（TRUER19 换轴加深）。  
> **不是** Stage 17 / Thompson / 空盘 / batch4。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-10
PARENT=AUTO-TRUER19.md
NORTH_STAR=place-yoyo-compiler-c1 在 payload bins 缺席时自动 plant+seeds 派生；无 -CodeBin/-DataBin 仍能 place→编 FF
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **D1** | `place-yoyo-compiler-c1.ps1` auto-derive on absent | **绿** |
| **D2** | 隐藏 bins + 无 override place → FF | **绿** |
| **D3** | pin `place_auto_derive_on_absent=PRESENT`；`stage-truer20` GREEN | **绿** |

## 验收（2026-11-10）

```powershell
& .\scripts\stage-truer20.ps1
# place_auto_derive_on_absent=PRESENT · hide_place_no_overrides=GREEN · GREEN
```

## 诚实

- 把 TRUER19 的「可缺席」收进 **默认 place 工具**，不再只靠 stage 手传 -CodeBin  
- plant 正文仍 A1；产物仍非编译器  
- ≠ Thompson  

---

*2026-11-10 · ACTIVE=0 · COMPLETED=1 · place_auto_derive*
