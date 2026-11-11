# AUTO-SEMREPRO — 软肋：大输入 Y1PL 语义自举（非 copy_self）

> 用户授权（2026-11-11）：**软肋破障，破不掉不要停**。  
> live 前缀对大 tyb 曾纯 `copy_self`；SEMCOMP 语义在 body。  
> 本轨：Y1PL → place_epi → body try_place；毒化 runner 名仍 `output==stock`。  
> **不是** Stage 17 / Thompson / 消尽 Win32 A1。

```
ACTIVE=0
COMPLETED=1
PARENT=AUTO-SEMPLACE.md
NORTH_STAR=Y1PL→semantic self EQUAL；poison 证明非 copy_self；十四切片仍绿
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **R1** | OPENLOOP：Y1PL→place_epi；重建 fragment；烙进 code.bin | **绿** |
| **R2** | poison runner + Y1PL → output==stock（≠poison） | **绿** |
| **R3** | 十四切片 + place；`stage-semrepro` GREEN | **绿** |

## 验收

```powershell
& .\scripts\stage-semrepro.ps1
# y1pl_semantic=EQUAL · poison_copy_self_impossible=1 · stock_sem_all14 · GREEN
```

## 诚实（完整）

- **已真**：Y1PL 大输入语义 try_place 出 stock；毒化 `yoyo_compiler.exe` 字符串后仍 EQUAL → 非 copy_self  
- **边界**：非 Y1PL 大输入仍 copy_self；runner 写后可能 AV（exit 残）；host 仍 A1 Win32/PE；trust_root 仍 place_stub  
- ≠ Thompson  

---

*2026-11-11 · COMPLETED*
