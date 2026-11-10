# AUTO-TRUER15 — parse1 发射面语义 YOYO 化（有限切片）

> 用户授权（2026-11-10）：**下一刀**（TRUER14 之后）。  
> **不是** Stage 17 / Thompson / 空盘 / 全文 15KB plant 消 A1。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-10
PARENT=AUTO-TRUER14.md
NORTH_STAR=parse1 有限发射面（RET/NOP/SUB/ADD/GET）由 YOYO 语义 tyb（00/40/60/61/62/FF）拥有；C1 冷跑 .text 含锁定 expect.hex；A1=0
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **S1** | `yoyo-sem-{ret,nop,sub,add,get}.ty(b)` + `.expect.hex`（无 A1） | **绿** |
| **S2** | C1×五切片 → MZ 且 `.text` 含 expect | **绿** |
| **S3** | pin `parse1_sem_slice_sot=yoyo_semantic_tyb`；`stage-truer15` GREEN | **绿** |

## 验收（2026-11-10）

```powershell
& .\scripts\stage-truer15.ps1
# parse1_sem_slice_sot=yoyo_semantic_tyb · c1_sem_regen=EXPECT_IN_TEXT · GREEN
```

## 诚实

- 本刀钉的是 **parse1 发射算法的有限语义面**（YOYO 源 40/00/60/61/62/FF），不是 A1 装字节  
- `code.bin` plant 正文两半 **仍是** A1 tyb（TRUER14）；全文语义重写未做  
- ≠ Thompson  

---

*2026-11-10 · ACTIVE=0 · COMPLETED=1 · parse1_sem_slice*
