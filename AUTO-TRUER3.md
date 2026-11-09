# AUTO-TRUER3 — 更真³（钉死 YOYO sidecar · 禁 cargo emit-rt）

> 用户授权（2026-11-09）：再「更真」。  
> 发现：H00 链仍吃 1.8MB `yoyo_rt.dll`；`critical_pe_roots=1` 漏计。  
> **不是** Stage 17 / Thompson / 空盘创世。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-TRUER2.md
NORTH_STAR=sidecar 进 fixtures pin；门上 cargo emit-rt-sidecar=ABSENT
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **V0** | `scripts/fixtures/yoyo_rt.dll` + pin | **绿** |
| **V1** | fullbar 只从 fixture 取 sidecar；禁 cargo fallback | **绿** |
| **V2** | `critical_trust_blobs=2`（compiler PE + rt dll） | **绿** |
| **V3** | `stage-truer` GREEN · `cargo_emit_rt=ABSENT` | **绿** |

## 验收（2026-11-09）

```powershell
& .\scripts\stage-truer.ps1
# sidecar_sot=fixture · cargo_emit_rt=ABSENT · critical_trust_blobs=2
```

## 诚实

- 信任面**更正**为 2 个 pin 锁 blob：`yoyo_compiler.exe` + `yoyo_rt.dll`  
- 去掉的是门上 **cargo emit-rt**；未消灭 H_00 对 sidecar 的需求  
- ≠ Thompson  

## 续

再「更真」→ **`AUTO-TRUER4` COMPLETED**（stub TCB + sidecar 离关键）。

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · 续 TRUER4*
