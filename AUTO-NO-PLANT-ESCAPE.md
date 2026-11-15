# AUTO-NO-PLANT-ESCAPE — 去掉 Node plant 显式 escape

> 用户授权（2026-11-09）：消灭 `YOYO_ALLOW_NODE_PLANT` 后门。  
> **不是** Stage 17 / Thompson。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-NO-NODE-PLANT.md
NORTH_STAR=Node ISA plant 永久拒绝；无 env escape
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **NE0** | `rebuild-compiler-payload.js` 无条件拒绝（删 plant 体） | **绿** |
| **NE1** | `place-yoyo-compiler.js` 仅 `--from-bins`；无 live plant 分支 | **绿** |
| **NE2** | 门：即使 `YOYO_ALLOW_NODE_PLANT=1` 仍 ABSENT/`escape=ABSENT` | **绿** |

## 验收（2026-11-09）

```powershell
$env:YOYO_ALLOW_NODE_PLANT='1'; & .\scripts\stage-no-node-plant.ps1
# rebuild_even_with_allow=REFUSED · escape=ABSENT · regen_after_wipe=EQUAL
```

## 诚实

- JS `emit-*.js` 可仍在树内作考古；**无**受支持入口调用  
- 再生唯一路径：C1 fixture emit/place  
- ≠ Thompson  

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · escape=ABSENT*
