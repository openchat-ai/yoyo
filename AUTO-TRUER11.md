# AUTO-TRUER11 — code/compact YOYO 化

> 用户授权（2026-11-09）：**再往下把 code/compact 也 YOYO 化**。  
> **不是** Stage 17 / Thompson / 空盘创世 / 删掉 `yoyo.ty(b)`。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-TRUER10.md
NORTH_STAR=compact 模板由 YOYO FF tyb 派生；code.bin 血统钉死为 C1 PE .text 提取；预制面去掉 compact
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **Y1** | `yoyo-compact-ret.tyb`（单 `FF`）为 compact SoT；bake EQUAL 派生 PE | **绿** |
| **Y2** | `code.bin` 血统 = C1 fixture `.text+13` 提取（门禁钉死） | **绿** |
| **Y3** | pin：`compact_sot=yoyo_ff_tyb` · `code_lineage=c1_pe_text_plus13` · `prefab_sot=bins_minus_compact` | **绿** |
| **Y4** | `stage-truer11` GREEN（含 C1×compact-ret 冷跑 → MZ + `.text` 含 `C3`） | **绿** |

## 验收（2026-11-09）

```powershell
& .\scripts\stage-truer11.ps1
# compact_sot=yoyo_ff_tyb · code_lineage=c1_pe_text_plus13 · prefab_sot=bins_minus_compact · GREEN
```

## 诚实

- compact（9728B）= YOYO `FF`→`C3` + compact link，**不再**算内容预制 SoT  
- `code.bin` **仍是** parse1 机器 plant 预制；本轨钉的是 **血统=C1 PE 提取**，**不是**把 15KB plant 改写成 YOYO 源  
- YOYO 源仍在：`yoyo.ty` / `yoyo.tyb` / 新增 `yoyo-compact-ret.ty(b)`  
- 内容预制还剩：`code.bin`（15596）+ data 头至 compact 前（32864）≈ 48KB  
- ≠ 空盘 / Thompson  

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · compact_yoyo + code_lineage*
