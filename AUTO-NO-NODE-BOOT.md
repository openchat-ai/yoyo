# AUTO-NO-NODE-BOOT — 去 Node bootstrap C1

> 用户授权（2026-11-09）：消灭 BOUNDARY 诚实残留「离线 Node place 一次」。  
> **不是** Stage 17 / Thompson / YOYO 吐 bins。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-BOUNDARY.md + AUTO-C1-NATIVE.md
NORTH_STAR=冷启/无预置 C1 时用 pin 锁 fixture，不调用 Node place
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **NB0** | 刷新 `scripts/fixtures/yoyo_compiler.exe` ≡ 当前 bins native-link PE + pin | **绿** |
| **NB1** | `bootstrap-c1-offline.ps1` = fixture copy（禁 Node） | **绿** |
| **NB2** | `place-yoyo-compiler-c1` / yc2 优先 fixture | **绿** |
| **NB3** | 抹掉 `_stage-c1-native` C1 → bootstrap → `stage-c1-native` GREEN | **绿** |

## Checklist

- [x] **NB-0**…**NB-3**
- [x] **NB-DONE** → `COMPLETED=1`

## 验收（2026-11-09）

```powershell
Remove-Item -Force scripts\_stage-c1-native\yoyo_compiler.exe
& .\scripts\bootstrap-c1-offline.ps1   # emitter=fixture_copy · node_place=ABSENT
& .\scripts\stage-c1-native.ps1        # GREEN · peer 0463c39c…
```

## 诚实

- fixture = checked-in C1 PE（与 bins 同世代 pin）；改 bins 后离线 refresh fixture（C1 place 自刷，勿 Node）  
- bins 再生 → **`AUTO-YOYO-BINS` COMPLETED**（C1 extract）  
- ≠ Thompson  

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · node bootstrap ABSENT · 续 YOYO-BINS*
