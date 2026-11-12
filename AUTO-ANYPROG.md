# AUTO-ANYPROG — 「能写程序」能力轴终点（Win32 第一成）

> post-v1.0。抛开 Thompson / 空盘。≠ 回改 AUDIT* / 自举 COMPLETED。

```
ACTIVE=0
COMPLETED=1
PARENT=POST-1.0-HOLE-CHECKLIST.md
NORTH_STAR=文件+控制台+argv+时间+线程门禁绿；冷启 A1 不膨胀；Winsock 诚实 OPEN
STOP_ONLY=停 | ACTIVE=0 | COMPLETED=1 | hard-block
```

## 成功尺

| ID | 工具 | 状态 |
| --- | --- | --- |
| T1–T3 | cat / head / zero | **GREEN** |
| T4 | hello_console（GetStdHandle） | **GREEN** |
| T5 | cmdline（GetCommandLineA） | **GREEN** |
| T6 | tick（GetTickCount64） | **GREEN** |
| T7 | thread（CreateThread） | **GREEN** |
| T8 | socket（Winsock） | **OPEN**（WSAStartup host AV） |

门禁：`scripts/stage-anyprog.ps1`  
API：`scripts/fixtures/anyprog/API-HONEST.md`

## 诚实

- 应用 PE IAT=11；冷启 A1PIN 仍=6  
- 控制台=真 stdout；≠ 假文件 stand-in（T1–T3 仍用 output.exe）  
- Winsock OPEN ≠ 假绿  
- ≠ Thompson；≠「已替代 C」；更高层源语另开刀  

## Checklist

- [x] AP-0 文档 + API 诚实表  
- [x] AP-1 pe_link 应用面 / str 表  
- [x] AP-2 三文件工具  
- [x] AP-3 控制台 + argv + tick  
- [x] AP-4 线程  
- [x] AP-DONE COMPLETED=1（Winsock OPEN 写明）  

---

*COMPLETED 2026-11-12 · gate GREEN · winsock OPEN*
