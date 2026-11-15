# AUTO-TRUER8 — 拆 H00（清 bake 载荷 · 门禁无 stub）

> 用户授权（2026-11-09）：**拆 H00**。  
> **不是** Stage 17 / Thompson / 空盘创世。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-TRUER7.md
NORTH_STAR=关键 data 中 H00 bake 载荷清零；无 manual-map stub 模板；C1×tyb/stage 绿
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **H1** | 清 `data.bin` 内 H00 hdr/main/data_floor 载荷（保留标记/布局） | **绿**（−2356 非零字节 → 0） |
| **H2** | 门禁：manual-map stub 模板 ABSENT；bake 载荷全零 | **绿** |
| **H3** | place/emit/seed/semantic/fullbar/`stage-truer8` GREEN | **绿** |

## 验收（2026-11-09）

```powershell
& .\scripts\stage-truer8.ps1
# h00_bake_payload=ZEROED · h00_manual_map_stub=ABSENT · status=GREEN
```

## 诚实

- bake 载荷已清；标记字符串仍在；布局长度未砍  
- parse1 里 memcpy H00 的机器码仍在（拷的是零）  
- 种子入口短 slot jmp 可能仍在；≠ 删光一切 H00 形迹  
- 对 Rust H00 种子 body = DIFF（contrast_only，预期）  
- ≠ Thompson  

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · h00_bake_zeroed*
