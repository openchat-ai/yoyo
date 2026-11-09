# AUTO-TRUER7 — 真收 data（截尾零 + 同步 place 尺寸合同）

> 用户授权（2026-11-09）：**真收 data**。  
> **不是** Stage 17 / Thompson / 空盘创世。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-09
PARENT=AUTO-TRUER6.md
NORTH_STAR=data.bin 去掉尾零；code place 合同同步；stub/place/emit/seed 绿
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **D1** | 截 `data.bin` 尾零（16 对齐） | **绿** `421968→414016`（−7952） |
| **D2** | 同步 `code.bin` place 尺寸常量 | **绿**（dword 合同补丁；非 Node ISA plant） |
| **D3** | stub pin + place/emit EQUAL + face + semantic + fullbar | **绿** |
| **D4** | `stage-truer7` GREEN | **绿** |

## 验收（2026-11-09）

```powershell
& .\scripts\stage-truer7.ps1
# data_bytes=414016 · full_c1_pe_bytes=438784 · status=GREEN
```

## 诚实

- TRUER6 挡点：只截 data、不改 code → place exit=2  
- 本轨：截尾零 **并** 补丁 `data_len`/`peSize`/`dataVs`/`sizeOfImage`  
- 中段 data 空洞未压；≠ 空盘 / Thompson  

---

*2026-11-09 · ACTIVE=0 · COMPLETED=1 · data_414016 · pe_438784*
