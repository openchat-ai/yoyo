# AUTO-TRUER12 — data 装配派生（少信预制）

> 用户授权（2026-11-09）：`继续`（TRUER11 之后）。  
> **不是** Stage 17 / Thompson / 空盘 / 全文 YOYO 重写 parse1 plant。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-TRUER11.md
NORTH_STAR=data.bin = 种子+算法+YOYO compact 装配 EQUAL；内容预制收成 code + ret/setret seeds
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **A1** | RET 区 = `expand(ret seed)` EQUAL；`ret_pe_sot=seed_expand` | **绿** |
| **A2** | setret 区 = seed 文件 EQUAL；compact 仍 YOYO FF | **绿** |
| **A3** | 全量 `buildCompilerDataPrelude` EQUAL `data.bin`；`prefab_sot=code_plus_seeds` | **绿** |
| **A4** | `stage-truer12` GREEN | **绿** |

## 验收（2026-11-09）

```powershell
& .\scripts\stage-truer12.ps1
# data_sot=assembled · prefab_sot=code_plus_seeds · prefab_bytes=28908 · GREEN
```

## 诚实

- `data.bin` = **装配产物**（不再独立内容 SoT）  
- 内容预制 = `code.bin`（15596）+ ret/setret seeds（6656×2）= **28908**  
- 其中 parse1 plant 正文 ≈ **11105**（code 截断 place@4491 之后）仍是机器预制  
- ≠ Thompson / 空盘  

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · data_assembled*
