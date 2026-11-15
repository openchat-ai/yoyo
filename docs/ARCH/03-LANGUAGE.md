# 03 — 语言表面（应用面）

> 现行成文表面：仓库根 `LANG-SURFACE.md`（随版更新；本文件为架构展开）。

## 程序是什么

一个 YOYO **应用**在 v1.1 常态下是：

1. **文本源** `*.ty`：行导向；可含 `;` 注释；主体为十六进制 token / 助记约定（见样例）。  
2. **链接**：`yoyo.exe link --target=win32 <in.ty> <out.exe>`  
3. **运行**：直接执行 PE；依赖 IAT 导入。

```powershell
& F:\yoyo\yoyo-rust\target\release\yoyo.exe link --target=win32 `
  F:\yoyo\scripts\fixtures\anyprog\hello_console.ty `
  F:\yoyo\scripts\_tmp-hello\hello.exe
```

## 能力面（已门禁 · ANYPROG）

| 能力 | 样例 | 证明 |
|------|------|------|
| 文件读/写/填零 | `cat.ty` `head.ty` `zero.ty` | 字节精确 |
| 控制台 | `hello_console.ty` | stdout |
| argv | `cmdline.ty` | GetCommandLineA |
| 时间 | `tick.ty` | GetTickCount64 |
| 线程 | `thread.ty` | CreateThread |
| 退出 | 各样例 | ExitProcess |

位置：`scripts/fixtures/anyprog/` · 诚实 API：`API-HONEST.md` · 契约：`MODEL-CONTRACT.md`（LLM 写面；回归）。

## 导入面（两档）

| 档 | 集合 | 约束 |
|----|------|------|
| **冷启编译器 A1** | 6：VirtualAlloc, CreateFileA, ReadFile, WriteFile, CloseHandle, ExitProcess | `yoyo_a1_imports.pin`；禁静默膨胀 |
| **应用 PE** | 可至 11+（含 GetStdHandle 等） | 见 API-HONEST；≠ 冷启 A1 |

## Winsock

**OPEN / SCOPE-CUT（v1.1）**：`socket.ty` 保留；WSAStartup 路径宿主 AV；不入绿门禁。闭合属 ≥1.4 候选。

## 与自举源的区别

| | 应用 `.ty` | 自举 `.tyb` / plant |
|--|------------|---------------------|
| 入口 | `yoyo link` | C1 + `input.tyb` |
| 主尺 | 语言 | 回归 |
| 典型目录 | `scripts/fixtures/anyprog` | `yoyo/projects/*.tyb` |

## 更高源语（规划位 · 未激活则勿假装已有）

v1.2 候选：在 hex `.ty` 之上增加更可读层，或诚实 CUT。激活前必须有 SCOPE + 门禁；详见 `YOYO-LANG-PLAN.md`。

## 实现触点（改语言能力时）

- 链接/发射：`yoyo-rust/verifier`（及 pe_link / platform_io）  
- 样例与门禁：`scripts/fixtures/anyprog` · `scripts/stage-anyprog.ps1` · `scripts/stage-lang1.ps1`  
- 表面文档：同步改 `LANG-SURFACE.md`
