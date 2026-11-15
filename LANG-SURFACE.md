# YOYO 语言表面（v1.3 · 独立语言 · 两极）

> **两极政策（用户 2026-11-16）**：要么**完全不可读**，要么**最可读**。  
> 禁止中间态（`raw A0 48` 包装）冒充「可读语言」。  
> 主尺仍是**独立语言**（写/编/跑），不是信任修辞。

## 两极

| 极 | 文件 | 谁写 | 链路 |
|----|------|------|------|
| **不可读** | `.ty` 纯 hex | 机器 / 钉扎 / 自举 | `yoyo link --target=win32` |
| **最可读** | `.yoyo` | 人 | `Expand-YoyoHi` → `.ty` → `yoyo link` |

```text
# 最可读
hello_hi.yoyo  →  Expand-YoyoHi  →  .ty  →  yoyo link  →  .exe

# 完全不可读
hello_console.ty  →  yoyo link  →  .exe
```

### `.yoyo`（最可读 · v1.3）

```text
program hello_hi
target win32
print "hi\n"
exit 0
```

- **禁止**源文件出现 `raw` / `db` / 十六进制字节行  
- 说明：`LANG-HI.md`  
- 门禁：`stage-lang3.ps1`

### `.ty`（不可读）

- 既有 ANYPROG hex 样例  
- 门禁：`stage-lang1` / `stage-anyprog`

### 中间态（非主尺）

- `.yoyor` + `raw hex`（v1.2）= **降级**：可留作展开实验回归，**不是**人读语言表面。

## 最小能力（不可读极 · 已门禁）

| 能力 | 证明 |
|------|------|
| 文件读写 | cat / head / zero |
| 控制台 | hello_console · **及** `.yoyo` print |
| argv / 时间 / 线程 | cmdline / tick / thread |

冷启编译器 A1 仍钉 6 名；应用 IAT 见 `API-HONEST.md`。

## 诚实边界

- `.yoyo` 1.3 = `print`+`exit` 最小最可读，不是完整高级语言  
- Winsock 仍 OPEN/CUT  
- ≠ 替代 C/Rust；≠ Thompson-proof  

---

*v1.3 · 两极表面*
