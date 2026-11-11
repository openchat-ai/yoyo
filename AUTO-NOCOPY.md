# AUTO-NOCOPY — 软肋：废除 live copy_self 兜底

> 用户授权（2026-11-11）：**软肋破障，破不掉不要停**。  
> SEMAV 后非 Y1PL 大输入仍走 copy_self。  
> 本轨：OPENLOOP 删除 copy_self；大输入非 Y1PL → fail；Y1PL 语义+exit0 仍绿。  
> **不是** Stage 17 / Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMAV.md
NORTH_STAR=copy_self=ABSENT；pad80→fail；Y1PL EQUAL exit0；十四切片
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **N1** | OPENLOOP 去 copy_self；重烙+SEMAV | **绿** |
| **N2** | pad80 fail；Y1PL poison EQUAL exit0 | **绿** |
| **N3** | `stage-nocopy` GREEN | **绿** |

## 诚实（完整）

- **已真**：live OPENLOOP 无 copy_self；`selfhost_kind=y1pl_try_place_only`  
- **边界 / hard-block 候选**：trust_root 仍 place_stub；host 仍 Win32/PE A1（SEMHOST 钉）  
- ≠ Thompson  

---

*2026-11-11 · COMPLETED*
