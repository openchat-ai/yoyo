# Stub 审核走读（人或 AI 用这个，别啃生肉）

> 生成：`STUB.AI.md` → 本文件 → **`STUB.SKELETON.txt`** → 必要时全量 disasm。≠ Thompson。

## 审法（强制）

1. **先读 `STUB.AI.md`**
2. 细节优先 **`STUB.SKELETON.txt`**（无假解码；长门/PE 拼装已 FOLD）
3. 全量 `STUB.disasm.txt` 仅在 FOLD 边界需要时打开
4. CreateFile ⊆ 允许名单；无 UNKNOWN_IAT；无 BAN；CALLSEQ EQUAL

## 允许打开的路径

`code.bin`, `data.bin`, `emit.bins`, `input.tyb`, `output.exe`, `yoyo_compiler.exe`

## CreateFileA 一览

| # | VA | path | 允许? |
| --- | --- | --- | --- |
| 1 | `0x140001044` | input.tyb | **YES** |
| 5 | `0x140001404` | output.exe | **YES** |
| 10 | `0x1400015a6` | input.tyb | **YES** |
| 14 | `0x140001c89` | output.exe | **YES** |
| 18 | `0x140001d14` | emit.bins | **YES** |
| 21 | `0x140001d83` | yoyo_compiler.exe | **YES** |
| 24 | `0x140001e06` | code.bin | **YES** |
| 27 | `0x140001e82` | data.bin | **YES** |
| 33 | `0x140001f7d` | code.bin | **YES** |
| 36 | `0x140002000` | data.bin | **YES** |

## 全部 API 时间线（合同）

| # | VA | API | path/size |
| --- | --- | --- | --- |
| 1 | `0x140001044` | CreateFileA | input.tyb |
| 2 | `0x140001074` | ReadFile | n=0x40 |
| 3 | `0x14000108a` | CloseHandle |  |
| 4 | `0x14000124e` | VirtualAlloc | size=0x1000 |
| 5 | `0x140001404` | CreateFileA | output.exe |
| 6 | `0x140001432` | WriteFile | n=0x400 |
| 7 | `0x14000143c` | CloseHandle |  |
| 8 | `0x140001445` | ExitProcess |  |
| 9 | `0x140001451` | ExitProcess |  |
| 10 | `0x1400015a6` | CreateFileA | input.tyb |
| 11 | `0x1400015cd` | VirtualAlloc | size=0x8000 |
| 12 | `0x1400015fa` | ReadFile | n=0x8000 |
| 13 | `0x140001610` | CloseHandle |  |
| 14 | `0x140001c89` | CreateFileA | output.exe |
| 15 | `0x140001cb4` | WriteFile |  |
| 16 | `0x140001cc6` | CloseHandle |  |
| 17 | `0x140001ccf` | ExitProcess |  |
| 18 | `0x140001d14` | CreateFileA | emit.bins |
| 19 | `0x140001d28` | CloseHandle |  |
| 20 | `0x140001d42` | VirtualAlloc | size=0x3cc00 |
| 21 | `0x140001d83` | CreateFileA | yoyo_compiler.exe |
| 22 | `0x140001db3` | ReadFile | n=0x3cc00 |
| 23 | `0x140001dd3` | CloseHandle |  |
| 24 | `0x140001e06` | CreateFileA | code.bin |
| 25 | `0x140001e3d` | WriteFile | n=0x46b6 |
| 26 | `0x140001e4f` | CloseHandle |  |
| 27 | `0x140001e82` | CreateFileA | data.bin |
| 28 | `0x140001eb9` | WriteFile | n=0xa670 |
| 29 | `0x140001ecb` | CloseHandle |  |
| 30 | `0x140001eec` | VirtualAlloc | size=0x3cc00 |
| 31 | `0x140001f14` | VirtualAlloc | size=0x46b6 |
| 32 | `0x140001f3c` | VirtualAlloc | size=0xa670 |
| 33 | `0x140001f7d` | CreateFileA | code.bin |
| 34 | `0x140001fad` | ReadFile | n=0x46b6 |
| 35 | `0x140001fcd` | CloseHandle |  |
| 36 | `0x140002000` | CreateFileA | data.bin |
| 37 | `0x140002030` | ReadFile | n=0xa670 |
| 38 | `0x140002050` | CloseHandle |  |

## API 计数

| API | count |
| --- | ---: |
| CloseHandle | 10 |
| CreateFileA | 10 |
| ExitProcess | 3 |
| ReadFile | 5 |
| VirtualAlloc | 6 |
| WriteFile | 4 |

## 分场索引

| VA | phase |
| --- | --- |
| `0x140001000` | ENTRY / PE startup |
| `0x14000100d` | PLACE BODY |
| `0x140001044` | CreateFileA → input.tyb |
| `0x140001074` | ReadFile |
| `0x14000109a` | CHECK magic Y1PL |
| `0x1400010bf` | CHECK magic TYB |
| `0x14000124e` | VirtualAlloc |
| `0x14000126d` | BUILD PE MZ header |
| `0x140001404` | CreateFileA → output.exe |
| `0x140001432` | WriteFile |
| `0x140001445` | ExitProcess |
| `0x140001464` | DATA string "input.tyb | output.exe | SEMHOST1" |
| `0x140001482` | EMBEDDED seed/table (not cold I/O path) |
| `0x140001576` | H_00 / open-loop frame (sub rsp,0x78) |
| `0x140001d14` | CreateFileA → emit.bins |
| `0x140001d83` | CreateFileA → yoyo_compiler.exe |
| `0x140001e06` | CreateFileA → code.bin |
| `0x140001e82` | CreateFileA → data.bin |

## FOLD 摘要（消歧义）

- `; FOLD opcode/size-gates 0x1400010de..0x140001146  (26 insns — skipped as noise)`
- `; FOLD seed/table helpers 0x140001482..0x140001576  (244 insns — skipped as noise)`
- `; FOLD opcode/size-gates 0x14000164e..0x140001b12  (239 insns — skipped as noise)`
- `; FOLD opcode/size-gates 0x140001b14..0x140001b3f  (14 insns — skipped as noise)`
- `; FOLD opcode/size-gates 0x140001b41..0x140001b6c  (14 insns — skipped as noise)`
- `; FOLD opcode/size-gates 0x140001b6e..0x140001b99  (14 insns — skipped as noise)`
- `; FOLD opcode/size-gates 0x140001b9b..0x140001bc6  (14 insns — skipped as noise)`
- `; FOLD opcode/size-gates 0x140001bc8..0x140001bf3  (14 insns — skipped as noise)`
- `; FOLD opcode/size-gates 0x140001bf5..0x140001c20  (14 insns — skipped as noise)`
- `; FOLD opcode/size-gates 0x140001c22..0x140001c52  (12 insns — skipped as noise)`
- `; FOLD PE-field/IAT stores 0x140002065..0x1400026c6  (230 insns — skipped as noise)`

## 审核红旗（生成器）

- （无）路径允许 · 无 UNKNOWN_IAT · 无 BAN · CALLSEQ EQUAL · 无假解码

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

## 诚实

- FOLD/DATA 消歧义 ≠ 逻辑无后门证明
- bins / plant 仍非「读完」对象

