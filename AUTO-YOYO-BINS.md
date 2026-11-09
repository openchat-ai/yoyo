# AUTO-YOYO-BINS — YOYO/C1 吐 bins

> 用户授权（2026-11-09）：消灭「bins 只能 Node rebuild」残留。  
> **不是** Stage 17 / Thompson / ISA plant 改源仍可有离线 Node 一次。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-BOUNDARY.md + AUTO-NO-NODE-BOOT.md
NORTH_STAR=C1 进程从 peer PE 抽出 code.bin+data.bin；门上再生无 Node rebuild
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **YB0** | C1：`emit.bins` 哨兵 → 读 self PE → 写 `code.bin`/`data.bin` | **绿** |
| **YB1** | `emit-bins-c1.ps1`；产物 ≡ pin | **绿** |
| **YB2** | 门：bins 再生 = C1 emit（`yoyo_bins_node_rebuild=ABSENT`） | **绿** |
| **YB3** | `stage-c1-native` GREEN + emit EQUAL | **绿** |

## Checklist

- [x] **YB-0**…**YB-3**
- [x] **YB-DONE** → `COMPLETED=1`

## 验收（2026-11-09）

```powershell
& .\scripts\emit-bins-c1.ps1          # EQUAL pin · node_rebuild=ABSENT
& .\scripts\stage-c1-native.ps1       # yoyo_bins_emit=EQUAL · peer 5da5716a…
```

## 诚实

- 本轨 = **C1 从 peer PE 抽出/再生** bins  
- Node plant 操作残留 → **`AUTO-NO-NODE-PLANT` COMPLETED**（默认拒绝；regen=C1）  
- ≠ Thompson  

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · 续 NO-NODE-PLANT*
