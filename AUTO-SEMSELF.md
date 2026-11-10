# AUTO-SEMSELF — 产物语义编 tyb 并再编出自己

> 用户授权（2026-11-10）：**产物能编 tyb 并再编出自己**。  
> 取代 `AUTO-SELFHOST` 的 copy-self 宣称；**不是** Stage 17 / Thompson。

```
ACTIVE=1
COMPLETED=0
PARENT=AUTO-SELFHOST.md
NORTH_STAR=C1×plant-place/parse1.tyb → 回收 code.bin；link→C1'==C1；hop2 仍 EQUAL
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **M1** | C1 语义编 `yoyo-plant-place.tyb` / `yoyo-plant-parse1.tyb`，`.text+13` 回收 code 半片 | **待** |
| **M2** | concat==`code.bin`；cwd bins link → `output.exe`==C1 | **待** |
| **M3** | hop2（C1'）再编两半片 EQUAL；compact-ret 仍出 MZ | **待** |

## 验收

```powershell
& .\scripts\stage-semself.ps1
# plant_code_recover=EQUAL · self_repro=EQUAL · hop2=EQUAL · GREEN
```

## 诚实（必读）

- **已真（本刀）**：C1 对 plant tyb **语义编译**；半片拼接回收 `code.bin`；再 link 出与 C1 字节相等的自己；hop2 仍能编出相同半片 PE  
- **已真**：compact-ret 路径仍是 parse1 语义童子（非 copy-self）  
- **仍假 / 边界**：`data.bin` 仍 pin SoT（本刀不从 tyb 再生 data）；28KB `yoyo.tyb` handler 产物 **不是** 本轨自举载体（SELFHOST copy-self 降级）  
- ≠ Thompson  

---

*2026-11-10 · ACTIVE tip*
