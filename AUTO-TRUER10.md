# AUTO-TRUER10 — 少信预制（切 FM · bins 为唯一内容 SoT）

> 用户授权（2026-11-09）：**只信自己、不信任何预制字节，继续肝**。  
> **不是** Stage 17 / Thompson / 空盘创世（零字节创世做不到；本轨把预制面砍到 bins）。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-TRUER9.md
NORTH_STAR=切除 field-match 预制尾；stub=派生；内容 SoT=bins；stage 绿
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **P1** | 物理切除 `data.bin` compact3 之后 FM 尾 | **绿** `180864→42608`（−138256） |
| **P2** | FM cmpsb 长度→0；place 尺寸合同同步 | **绿** |
| **P3** | 门禁：`prefab_sot=bins` · stub derived · `stage-truer10` GREEN | **绿** |

## 验收（2026-11-09）

```powershell
& .\scripts\stage-truer10.ps1
# prefab_sot=bins · data_bytes=42608 · bins_total=58204 · GREEN
```

## 诚实

- 内容预制 SoT = **bins only**（~58KB）；stub 由 bins 派生（门禁重建 EQUAL）  
- 语义输入 = `yoyo.tyb`  
- **仍有** bins 预制字节；空盘零字节 = 做不到 / ≠ Thompson  
- PE 映像仍受 `0x38000` data floor 撑到 ~246KB（布局地板，不是内容 SoT）  

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · prefab_bins_only*
