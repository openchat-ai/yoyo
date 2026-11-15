# AUTO-TRUER23 — 编译器 I/O 名表 YOYO SoT

> 用户授权（2026-11-10）：**继续**（TRUER22 后换轴：编译器 I/O 面归属）。  
> **不是** Stage 17 / Thompson / 空盘 / 产物 face 再削。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-10
PARENT=AUTO-TRUER22.md
NORTH_STAR=data.bin 名表（input.tyb/output.exe…）从 JS 字面量迁到 yoyo-compiler-io-names.tyb（A1）；钉 compiler_io_names_sot；产物仍 ABSENT 这些路径
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```

## 队列

| ID | 项 | 状态 |
| --- | --- | --- |
| **G1** | `yoyo-compiler-io-names.tyb` A1 打包 189B 名表 | **绿** |
| **G2** | prelude 从 tyb unpack，不再硬编码 JS 字符串；assemble EQUAL | **绿** |
| **G3** | C1 PRESENT / product ABSENT；pin + `stage-truer23` GREEN | **绿** |

## 验收（2026-11-10）

```powershell
& .\scripts\stage-truer23.ps1
# compiler_io_names_sot=yoyo_compiler_io_names_tyb · product_io=ABSENT · GREEN
```

## 诚实

- 编译器打开 `input.tyb` 的**路径字符串**现由 YOYO 项目 tyb 权威，不再藏在 JS 字面量  
- 名表仍是 A1 字面（与 plant 同形），**不是**语义算法重写  
- 产物仍无这些路径 → 仍不能当编译器接力  
- ≠ Thompson  

---

*2026-11-10 · ACTIVE=0 · COMPLETED=1 · compiler_io_names_yoyo_sot*
