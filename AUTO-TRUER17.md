# AUTO-TRUER17 — parse1 语义发射面 batch3（ADDV/ORV/SUBV/IMUL）

> 用户授权（2026-11-10）：**继续**（TRUER16 之后）。  
> **不是** Stage 17 / STAGE17 checklist / Thompson / 空盘 / 全文 plant 消 A1。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-10
PARENT=AUTO-TRUER16.md
NORTH_STAR=parse1 语义发射面扩到 ADDV/ORV/SUBV/IMUL；十四切片 C1 expect；A1=0；batch=3
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **C1** | `yoyo-sem-{addv,orv,subv,imul}.ty(b)` + expect（无 A1） | **绿** |
| **C2** | C1×十四切片 → MZ 且 `.text` 含 expect | **绿** |
| **C3** | pin `parse1_sem_batch=3`；`stage-truer17` GREEN | **绿** |

## 验收（2026-11-10）

```powershell
& .\scripts\stage-truer17.ps1
# parse1_sem_batch=3 · c1_sem_regen=EXPECT_IN_TEXT · GREEN
```

## 诚实

- batch1+2+3 = 有限 ALU/发射面语义 YOYO（十四切片）  
- plant 正文两半仍 A1；≠ 全文算法重写；≠ Stage 17  
- ≠ Thompson  

---

*2026-11-10 · ACTIVE=0 · COMPLETED=1 · parse1_sem_batch3*
