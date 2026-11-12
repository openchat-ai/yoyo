# AUTO-ANYPROG — 第一档「能写程序」

> post-v1.0 能力轴。抛开 Thompson / 空盘。≠ 回改 AUDIT* / 自举 COMPLETED。

```
ACTIVE=0
COMPLETED=1
PARENT=POST-1.0-HOLE-CHECKLIST.md
NORTH_STAR=Win32 三小工具门禁绿 + API 诚实表；A1 不静默膨胀
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## 成功尺（第一档）

| ID | 工具 | 期望 |
| --- | --- | --- |
| T1 | `cat.ty` | `input.tyb`="hello" → `output.exe`="hello"；exit 0 |
| T2 | `head.ty` | 同输入 → `output.exe`="hel"；exit 0 |
| T3 | `zero.ty` | ALLOC+WRITE → `output.exe` 4 零字节；exit 0 |

门禁：`scripts/stage-anyprog.ps1`  
API 表：`scripts/fixtures/anyprog/API-HONEST.md`  
退出例程：`scripts/fixtures/anyprog/stdlib_exit.inc.ty`

## 诚实

- 控制台 = 查 `output.exe`（非 GetStdHandle）
- A1 仍 6 名；ExitProcess 已在面内，经 RAW 调用
- pe_link 非 H00：`embed_string_table_r15` 于 IAT 之后
- ≠ Thompson；≠「已替代 C」

## Checklist

- [x] **AP-0** 文档 + API 诚实表
- [x] **AP-1** pe_link 非 H00 路径嵌入 str 表（r15+0x10000）
- [x] **AP-2** 三工具 + stage 绿
- [x] **AP-DONE** COMPLETED=1

---

*COMPLETED 2026-11-12 · gate GREEN*
