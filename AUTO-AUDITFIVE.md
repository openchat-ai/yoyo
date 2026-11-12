# AUTO-AUDITFIVE — 剩余五路压到 0

> 接 AUDITZERO 之后的「还能动手」五类。能 CLOSED 的必须毒红；不可消的 HARD_BLOCK，禁止假绿。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-AUDITZERO.md
NORTH_STAR=五路均 CLOSED 或 HARD_BLOCK；假 CLOSED 禁
STOP_ONLY=停 | ACTIVE=0 | 五路结算完（含 HARD_BLOCK）
```

| # | 手法 | 结算 | 门禁 |
| --- | --- | --- | --- |
| F1 | 共谋改钉 | **HARD_BLOCK** | 同 AUDITZERO #8；Trusting Trust 残渣，仓内不可真消 0 |
| F2 | 允许 I/O 内语义后门 | **CLOSED** | `stage-plantpoison` + `stage-y1plpin` + `stage-binspoison` 毒红网 |
| F3 | 钉外同名文件 | **CLOSED** | `verify-live-scan-allow-pin.js` 扫 fixtures+payload 基名 |
| F4 | OS/加载器/硬件 | **HARD_BLOCK** | A1 以下 / YOYO TCB 外 |
| F5 | 哈希碰撞/换算法信任 | **HARD_BLOCK** | `STUB.DUALHASH.pin` 抬成本；真破哈希不可工程消 0 |

```powershell
& F:\yoyo\scripts\stage-auditfive.ps1
```

**诚实边界**：F2/F3 工程 CLOSED；F1/F4/F5 不可真 0 → HARD_BLOCK。≠ Thompson-proof。

---

*COMPLETED 2026-11-12 · gate GREEN*
