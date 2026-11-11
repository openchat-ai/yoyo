# AUTO-TRUSTLIM — 少信 + 有毒必被抓，到极限才停

> 用户授权（2026-11-11）：**少信，有毒必被抓，没达到极限不要停**。  
> 判据：`TRUST-SURFACE-v1.md`。禁空盘/Stage17/Thompson 宣称。

```
ACTIVE=1
COMPLETED=0
PARENT=TRUST-SURFACE-v1.md
NORTH_STAR=stub面到工程极限；冷启观测上投毒必RED
STOP_ONLY=停 | ACTIVE=0 | hard-block双极限 | COMPLETED=1
```

## 队列

| ID | 轨 | 项 | 状态 |
| --- | --- | --- | --- |
| **L0** | 探测 | stub code 最小 place 长度 | 5876=唯一绿（5875红） |
| **P1** | 检测 | STUBPOISON：毒 stub→pin/DDC 必 RED | 待 |
| **S1** | 少信 | STUBMIN：门禁钉死 min code_place=5876 | 待 |
| **P2** | 检测 | PLACEPIN：stub×bins→C1 sha pin | 待 |
| **L9** | 结算 | 双极限到齐 → COMPLETED | 待 |

## 极限定义（到了才停）

- **少信**：place 所需 stub `.text` 不可再短；PE 几何不可再靠 floor 缩小；stub/A1/内容pin 不可空盘消除  
- **检测**：冷启 linker + place 产物在既定观测下毒必 RED；共谋/观测外仍不可抓  

---

*ACTIVE=1*
