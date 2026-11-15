# AUTO-TRUSTLIM — 少信 + 有毒必被抓，到极限才停

> 用户授权（2026-11-11）：**少信，有毒必被抓，没达到极限不要停**。  
> 判据：`TRUST-SURFACE-v1.md`。禁空盘/Stage17/Thompson 宣称。

```
ACTIVE=0
COMPLETED=1
PARENT=TRUST-SURFACE-v1.md
NORTH_STAR=stub面到工程极限；冷启观测上投毒必RED
STOP_ONLY=停 | ACTIVE=0 | hard-block双极限 | COMPLETED=1
```

## 队列

| ID | 轨 | 项 | 状态 |
| --- | --- | --- | --- |
| **L0** | 探测 | stub code 最小 place 长度 | **5876唯一绿** |
| **P1** | 检测 | STUBPOISON | **绿** `47acc46` |
| **S1** | 少信 | STUBMIN | **绿** `1e90bce` |
| **P2** | 检测 | PLACEPIN | **绿** `c795d8c` |
| **P3** | 检测 | BINSPOISON | **绿** |
| **L9** | 结算 | 双极限到齐 | **绿** |

## 极限判定（停）

| 轨 | 到了吗 | 证据 |
| --- | --- | --- |
| **少信（stub `.text`）** | **是** | STUBMIN：再短 1B place 红；data floor 不缩 PE |
| **检测（冷启观测）** | **是** | linker DDC + stub毒 + place产物pin + bins毒 → 皆必 RED |
| **仍 hard-block** | stub 存在本身 · 内容 pin 体量 · Win32/PE A1 · 共谋/观测外 | 再下=空盘/宣称证明（禁） |

---

*2026-11-11 · COMPLETED · ≠ Thompson*
