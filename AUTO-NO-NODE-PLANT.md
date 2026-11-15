# AUTO-NO-NODE-PLANT — 去 Node ISA plant

> 用户授权（2026-11-09）：消灭「改/再生 bins 靠 Node rebuild-compiler-payload」操作残留。  
> **不是** Stage 17 / Thompson / 把 JS emit 源码 magically 变成 YOYO 语义编辑器。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-YOYO-BINS.md + AUTO-NO-NODE-BOOT.md
NORTH_STAR=bins↔PE 再生闭环零 Node plant；Node rebuild 非 SoT（默认拒绝）
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **NP0** | SoT = fixture PE ↔ C1 emit/place；写明 plant 历史 | **绿** |
| **NP1** | `rebuild-compiler-payload.js` / live `place` 默认拒绝（需 `YOYO_ALLOW_NODE_PLANT=1`） | **绿** |
| **NP2** | `regen-payload-from-c1.ps1`：无 Node 从 fixture 再生 bins+pin | **绿** |
| **NP3** | 抹 bins → C1 regen ≡ 原 pin；`node_plant=ABSENT` | **绿** |

## Checklist

- [x] **NP-0**…**NP-3**
- [x] **NP-DONE** → `COMPLETED=1`

## 验收（2026-11-09）

```powershell
& .\scripts\stage-no-node-plant.ps1   # GREEN · regen_after_wipe=EQUAL
& .\scripts\stage-c1-native.ps1       # node_plant=ABSENT
```

## 诚实

- 再生 = C1 抽 PE / link bins  
- 显式 escape → **`AUTO-NO-PLANT-ESCAPE` COMPLETED**（`escape=ABSENT`）  
- `emit-*.js` 仅考古；≠ Thompson  

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · 续 NO-PLANT-ESCAPE*
