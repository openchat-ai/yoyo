# Stub 审核走读（人或 AI 用这个，别啃生肉）

> `python scripts/gen-stub-disasm.py` → 本文件 + `STUB.disasm.txt`。≠ Thompson。

## 审法（强制）

1. **先读本走读**，再只打开反汇编里带 `; =>` / `; ==========` / `; str=` / `; DATA` 的行
2. CreateFile 路径必须落在允许名单；否则记红旗
3. 禁止出现 `UNKNOWN_IAT`；所有 call 应已标成 6 API 之一

## 允许打开的路径

`code.bin`, `data.bin`, `emit.bins`, `input.tyb`, `output.exe`, `yoyo_compiler.exe`

## CreateFileA 一览

| VA | path | 允许? |
| --- | --- | --- |
| `0x140001044` | input.tyb | **YES** |
| `0x140001404` | output.exe | **YES** |
| `0x1400015a6` | input.tyb | **YES** |
| `0x140001c89` | output.exe | **YES** |
| `0x140001d14` | emit.bins | **YES** |
| `0x140001d83` | yoyo_compiler.exe | **YES** |
| `0x140001e06` | code.bin | **YES** |
| `0x140001e82` | data.bin | **YES** |
| `0x140001f7d` | code.bin | **YES** |
| `0x140002000` | data.bin | **YES** |

## 全部 API 计数

| API | count |
| --- | ---: |
| CloseHandle | 10 |
| CreateFileA | 10 |
| ExitProcess | 3 |
| ReadFile | 5 |
| VirtualAlloc | 6 |
| WriteFile | 4 |

## 审核红旗（生成器）

- （无）路径均在允许名单或未解析；无 UNKNOWN_IAT

## PE 内相关字符串（抽查用）

- `input.tyb`
- `output.exe`
- `SEMHOST1`
- `kernel32H`
- `WriteFiH`
- `ExitPH`
- `kernel32.dll`
- `VirtualAlloc`
- `CreateFileA`
- `ReadFile`
- `WriteFile`
- `CloseHandle`
- `ExitProcess`
- `yoyo_ret_c`
- `yoyo_sr_c`
- `yoyo_tbl_c`
- `yoyo_f10_c`
- `yoyo_multi_c`
- `yoyo_live1_c`
- `yoyo_live2_c`
- `yoyo_emit1_c`
- `yoyo_emit2_c`
- `yoyo_parse1_c`
- `code.bin`
- `data.bin`
- `yoyo_compiler.exe`
- `emit.bins`

## 分场

在 `STUB.disasm.txt` 搜索 `==========` 跳读：ENTRY → 开 input.tyb → Y1PL/TYB → VirtualAlloc/建 PE → 写 output.exe → ExitProcess → 后续 bins/place 变体。

## 诚实

- 标注让 AI/人**抽查面**变小，不是证明无逻辑后门
- 未解析到路径的 CreateFile 需结合前后 `lea`/`mov` 人工看一眼

