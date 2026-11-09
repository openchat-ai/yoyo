# AUTO-TRUER2 — 更真²（去掉第二条 PE 信任根）

> 用户授权（2026-11-09）：再「更真」— 关键路径只留 C1 fixture PE + tyb。  
> **不是** Stage 17 / Thompson / 空盘创世。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-TRUER.md
NORTH_STAR=H00 冷启 seed 只信 C1+tyb；seed_yoyo_win32 降为可选 contrast
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **U0** | fullbar：chain seed = C1+tyb 产物；不要求 seed fixture EQUAL | **绿** |
| **U1** | `seed_asm` = copy(`seed_js`)；`cold_start_asm_live=ABSENT` 仍立 | **绿** |
| **U2** | seed fixture 仅 contrast（缺席/DIFF 不红）· pin `contrast_only` | **绿** |
| **U3** | `stage-truer` GREEN · `seed_sot=c1_tyb` · `critical_pe_roots=1` | **绿** |

## 验收（2026-11-09）

```powershell
& .\scripts\stage-truer.ps1
# seed_sot=c1_tyb · critical_pe_roots=1 · seed_fixture_required=ABSENT
```

## 诚实

- 关键 PE 信任根仍是 **yoyo_compiler fixture**（+ tyb 输入）  
- seed_yoyo_win32 / Rust = 检测对照，非 ancestry SoT  
- ≠ Thompson  

## 续

再「更真」→ **`AUTO-TRUER3` COMPLETED**（sidecar fixture · 禁 cargo emit-rt）。

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · 续 TRUER3*
