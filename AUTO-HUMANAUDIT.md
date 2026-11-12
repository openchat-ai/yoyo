# AUTO-HUMANAUDIT — 人/AI 可抽查审核包

> 生肉反汇编不可审完；本轨交付 **走读 + 标注反汇编**。深化见 **AUTO-AUDITMAX**。≠ Thompson。

```
ACTIVE=0
COMPLETED=1
PARENT=TRUST-SURFACE-v1.md
CHILD=AUTO-AUDITMAX.md
NORTH_STAR=STUB.WALKTHROUGH + 标注 disasm；CreateFile 路径全解析且允许名单
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1
```

## 产物

| 文件 | 用途 |
| --- | --- |
| `STUB.AI.md` | 一页 VERDICT（优先） |
| `STUB.WALKTHROUGH.md` | API 时间线、CreateFile、分场 |
| `STUB.SKELETON.txt` | **细节优先**（AUDITCLR 消歧义） |
| `STUB.disasm.txt` | 全量但已 DATA/FOLD/OOW |
| `STUB.CALLSEQ.pin` | 合同 pin |
| `stage-humanaudit.ps1` | pin + 再生 + FLAG/VERDICT fail-closed |

## 诚实

- 抽查面变小 ≠ 人/AI 读穿逻辑证明无后门  

---

*2026-11-12 · HUMANAUDIT + AUDITMAX*
