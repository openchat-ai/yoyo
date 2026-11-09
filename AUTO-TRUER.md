# AUTO-TRUER — 更真自举（缩信任面）

> 用户授权（2026-11-09）：「更真」— 缩自举路径外来信任。  
> **不是** Stage 17 / Thompson / 空盘创世。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-NO-PLANT-ESCAPE.md + POST-1.0-HOLE-CHECKLIST.md
NORTH_STAR=操作 SoT 收成单一 runnable：fixture PE；bins 仅派生
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列（易→难）

| ID | 项 | 状态 |
| --- | --- | --- |
| **T0** | `verify-c1-fixture-pin.js`；fixture 为唯一冷启根 | **绿** |
| **T1** | 门上先抹 bins → C1 regen（checked-in bins 非独立 SoT） | **绿** |
| **T2** | pin：`bins_sot=fixture_pe` / `trust_root=c1_fixture` | **绿** |
| **T3** | `stage-truer.ps1` → c1-native GREEN | **绿** |

## 验收（2026-11-09）

```powershell
& .\scripts\stage-truer.ps1
# trust_root=c1_fixture · bins_sot=fixture_pe · status=GREEN
```

## 诚实

- 仍有 **一个** checked-in PE 信任根（`scripts/fixtures/yoyo_compiler.exe`）；不是无起源  
- bins 派生 ≠ YOYO 源码改语义  
- ≠ Thompson  

## 续

再「更真」→ **`AUTO-TRUER2` COMPLETED**（seed 只信 C1+tyb）。

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · 续 TRUER2*
