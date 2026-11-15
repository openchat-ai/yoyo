# AUTO-BOUNDARY — 诚实边界逐项消灭（易→难）

> 用户授权（2026-11-09）：从简到难收残留。  
> **不是** Stage 17 / Thompson / 无界 tip。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-C1-NATIVE.md + AUTO-YOYO-CLOSED.md
NORTH_STAR=门上与再生路径逐步去掉宿主/捷径残留
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 易→难队列

| ID | 项 | 难度 | 状态 |
| --- | --- | --- | --- |
| **B0** | 文档诚实注同步 | 易 | **绿** |
| **B1** | 门上禁止 Node place fallback | 易 | **绿** |
| **B2** | Cold-start 无现场 asm（checked-in seed fixture） | 中 | **绿** |
| **B3** | bins 门上 pin 锁 hash；无 Node rebuild | 难 | **绿**（离线仍可 `rebuild-compiler-payload.js`） |
| **B4** | C1 真 `linkPeWin32`（替 punch） | 最难 | **绿** |

## Checklist

- [x] **BD-0**…**BD-3**  
- [x] **BD-4**：native link 替换固定布局 punch；C₂≡C₁；geometry 不靠 self-PE 拷贝  
- [x] **BD-DONE**：B0–B4 全绿 → `COMPLETED=1`

## 验收（2026-11-09）

```powershell
node yoyo-js\scripts\verify-payload-bins-pin.js   # GREEN pin_EQUAL
& .\scripts\stage-c1-native.ps1                   # C1_NATIVE status=GREEN
```

- C1 place ≡ Node `linkPeWin32`（sha `0463c39c1023aa0f` · 446464）
- yc2 C₂≡C₁；fullbar fixedpoint EQUAL；`node_place_on_gate=ABSENT`

## 诚实

- B3 = **pin 锁**；再生 → **`AUTO-YOYO-BINS`**（C1 抽 self PE）  
- B4 = C1 内算法化 `linkPeWin32` peer  
- Node bootstrap → **`AUTO-NO-NODE-BOOT`** fixture copy  
- Node plant → **`AUTO-NO-NODE-PLANT` COMPLETED**（默认拒绝）  
- ≠ Thompson  

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · 续 NO-NODE-BOOT + YOYO-BINS + NO-NODE-PLANT*
