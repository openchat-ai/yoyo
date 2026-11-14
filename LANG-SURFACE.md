# YOYO 语言表面（v1.1）

> 独立语言第一成：人（或模型）怎么写、怎么编、怎么跑。  
> 自举/信任检测见 post-v1.0 附录；此处主尺 = **能写程序**。

## 是什么

YOYO 程序：

| 层 | 形式 | 链路 |
|----|------|------|
| **可读（v1.2）** | `.yoyor` | Expand-YoyoReadable → `.ty` → `yoyo link` |
| **规范映像源（v1.1）** | `.ty` hex | `yoyo link --target=win32` → PE |

```text
foo.yoyor → Expand-YoyoReadable → foo.ty → yoyo.exe link --target=win32 foo.exe → .\foo.exe
foo.ty    → yoyo.exe link --target=win32 foo.ty foo.exe → .\foo.exe
```

可读层说明：`LANG-READABLE.md`。编译器自举仍用 `.tyb` + C1 place（v1.0）。

## 最小能力（已有门禁）

| 能力 | 证明 |
|------|------|
| 文件读写 | anyprog：cat / head / zero |
| 控制台 stdout | hello_console |
| 命令行 | cmdline（GetCommandLineA） |
| 时间 | tick |
| 线程 | thread |
| 退出 | ExitProcess |

冷启编译器 A1 仍钉 6 名；应用 PE 可多导入（见 `scripts/fixtures/anyprog/API-HONEST.md`）。

## 样例位置

- 应用样例：`scripts/fixtures/anyprog/*.ty`  
- 门禁：`scripts/stage-anyprog.ps1` · `scripts/stage-lang1.ps1`  
- 写面契约（LLM/精确体）：`scripts/fixtures/anyprog/MODEL-CONTRACT.md`（回归，非本版主叙事）

## 诚实边界

- **不是**完整标准库 / 跨平台产品语言（Linux app peer 未作 1.1 必达）  
- Winsock：**OPEN**（见 ANYPROG）；1.1 门 D 决定绿或 CUT  
- ≠ 宣称替代 C/Rust；≠ Thompson-proof  

## 下一版可能（非本版承诺）

模块/库面、套接字闭合、非 Win32 应用 peer、符号/标签——另「定」v1.3+。

---

*v1.2 · LANG-SURFACE（含 READABLE）*
