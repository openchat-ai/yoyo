# LANG-HI — 最可读极（v1.3）

> 与纯 `.ty` hex **对立**的一极。源中**零**机器码。

## 语法

| 语句 | 含义 |
|------|------|
| `program <name>` | 程序名 |
| `target win32` | 目标 |
| `print "…"` | 写 stdout（v1.3 表内串：`hi\n`、`ok\n`） |
| `exit 0` | 退出 |
| `#` / `;` / `;;` | 注释 |

**非法**：`raw`、`db`、任何 hex 字节行 → 展开器硬失败。

## 工具

`scripts/lib/Expand-YoyoHi.ps1`（PowerShell only）

## 样例

`scripts/fixtures/anyprog/hello_hi.yoyo`

## 门禁

`& F:\yoyo\scripts\stage-lang3.ps1`
