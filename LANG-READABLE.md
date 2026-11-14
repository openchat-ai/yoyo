# LANG-READABLE（v1.2）

> 纯 hex `.ty` **之上**的官方可读层。展开器：**PowerShell only**（无 Node）。

## 文件

- 源：`*.yoyor`  
- 展开：`scripts/lib/Expand-YoyoReadable.ps1` → `*.ty`  
- 链接：`yoyo link --target=win32`（与 v1.1 相同）

## 语法（1.2）

| 形式 | 含义 |
|------|------|
| `;;` 或 `;` 或 `#` 行 | 注释（可进展开后的 `.ty`） |
| `program <name>` | 元数据（不进映像） |
| `target win32` | 元数据（门禁可校验） |
| `raw <hex…>` 或 `db <hex…>` | 写入 `.ty` 的字节行 |
| 空行 | 忽略 |

**不做**（OUT）：标签、表达式、宏、重定位。

## 例

见 `scripts/fixtures/anyprog/hello_hi.yoyor`。

## 门禁

`& F:\yoyo\scripts\stage-lang2.ps1`

---

*v1.2*
