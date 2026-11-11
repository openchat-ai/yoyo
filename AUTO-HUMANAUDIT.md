# AUTO-HUMANAUDIT — 人/AI 可抽查审核包

> 生肉反汇编不可审完；本轨交付 **走读 + 标注反汇编**。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=TRUST-SURFACE-v1.md
NORTH_STAR=STUB.WALKTHROUGH + 标注 disasm；CreateFile 路径全解析且允许名单
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1
```

## 产物

| 文件 | 用途 |
| --- | --- |
| `scripts/fixtures/audit-stub/STUB.WALKTHROUGH.md` | **优先**：API 表、CreateFile 路径、红旗 |
| `scripts/fixtures/audit-stub/STUB.disasm.txt` | `; => API` / `==========` / `DATA` 标注 |
| `stage-humanaudit.ps1` | pin + 再生 + CreateFile FLAG fail-closed |

## 诚实

- 抽查面变小 ≠ 人/AI 读穿逻辑证明无后门  

---

*2026-11-12 · annotated walkthrough*
