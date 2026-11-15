# AUTO-TRUER16 — parse1 语义发射面 batch2（SET/MOV/CMP/INC/DEC）

> 用户授权（2026-11-10）：**继续**（TRUER15 之后）。  
> **不是** Stage 17 / Thompson / 空盘 / 全文 plant 消 A1。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-10
PARENT=AUTO-TRUER15.md
NORTH_STAR=parse1 语义发射面扩到 SET/MOV/CMP/INC/DEC；十切片 C1 expect；A1=0；batch=2
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **B1** | `yoyo-sem-{set,mov,cmp,inc,dec}.ty(b)` + expect（无 A1） | **绿** |
| **B2** | C1×十切片 → MZ 且 `.text` 含 expect | **绿** |
| **B3** | pin `parse1_sem_batch=2`；`stage-truer16` GREEN | **绿** |

## 验收（2026-11-10）

```powershell
& .\scripts\stage-truer16.ps1
# parse1_sem_batch=2 · c1_sem_regen=EXPECT_IN_TEXT · GREEN
```

## 诚实

- batch1（ret/nop/sub/add/get）+ batch2（set/mov/cmp/inc/dec）= 有限发射面语义 YOYO  
- plant 正文两半仍 A1；≠ 全文算法重写  
- ≠ Thompson  

---

*2026-11-10 · ACTIVE=0 · COMPLETED=1 · parse1_sem_batch2*
