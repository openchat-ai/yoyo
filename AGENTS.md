# YOYO 编译器项目经验

> 自动迁移自 ~/.config/opencode/AGENTS.md (yoyo 调试/自举/调试工具)。编辑请改此文件或原全局文件。

## "Pre-existing bug" 调查：先用最小测试隔离，再判断是代码还是用法 (2026-06-30)

### 背景
给 yoyo.exe 加 `0x82` (jl) / `0x83` (jg) opcode 后，跑自托管测试 `yoyo.exe test.ty out.exe` 挂死 10s+。
第一印象：yoyo.exe 自托管有 pre-existing bug。
实际：完全误诊。三个"bug"都是误报。

### 三个误诊
1. **"yoyo-gen.js non-deterministic"** — bootstrap-check 报两次 hash 不同 (2E17C76C vs 9B833F47)
   - 真相：yoyo-gen.js **是确定性的**。误报来自 PowerShell `Start-Process | Out-Null` pipe 在 `&` 脚本里行为异常
   - 修复：用 `& node ...` 替代复杂 Start-Process
2. **"yoyo.exe hangs on test-hello.ty"** — 跑 10s+ 不退出，无 stdout/stderr
   - 真相：yoyo.exe **硬编码读 `input.ky` 同目录**（`50 0A 00` + 字符串表 `"input.ky"`），**忽略所有 CLI 参数**
   - 当 `input.ky` 不存在 → 打开失败 → 状态错乱 → spin loop
   - 修复方法：`Copy-Item test.ty input.ky; .\yoyo.exe`，**不传参数**
3. **"yoyo.exe 包含 0x82/0x83"** — 用 yoyo.exe 编完 output.exe 含 0 个 `0F 8C/0F 8F`
   - 真相：yoyo.exe 是 **6/28 旧 build**（DAF2D309），**没有我加的 opcode**！git tracked 的二进制在 6/28 commit 后没重新生成
   - 修复：先 `node yoyo.js yoyo.ty yoyo-new.exe`，**得到带新 opcode 的 yoyo.exe**，再编译测试

### 教训
- **不要凭"我跑了命令"判断 bug** — 用 `Get-FileHash` / `Get-Item` 验证**实际产物**的时间戳和大小
- **永远做最小复现**：先 `Copy-Item X.ty input.ky; .\yoyo.exe` (零参数)，看 output.exe 是否生成
- **PowerShell pipe 不可靠**：`Start-Process | Out-Null` 经常不返回 ExitCode 或报类型转换错。用 `&` 直接调用更稳
- **git tracked 的二进制不代表最新**：yoyo.exe 在 git 里，但 yoyo.ty 改了 → 必须重新 `node yoyo.js yoyo.ty yoyo.exe` → commit 新的
- **APPS.md/spec 标"Stage 2 DONE"不代表当前能跑**——文档会过时，要看 LastWriteTime

### 检查清单（自托管测试前必做）
1. `Get-FileHash build/yoyo.exe` 时间戳对吗？
2. `Copy-Item test.ty input.ky` 准备输入？
3. `& ./build/yoyo.exe` 跑（**不传参数**）
4. `Get-Item output.exe` 看生成时间
5. 反汇编 output.exe 验证 sentinel x86 字节存在？

### yoyo.exe 当前限制（Phase 1 设计）
- **不接受 CLI 参数**——硬编码 `input.ky` → `output.exe`
- **无 stdout/stderr 输出**——yoyo.ty 没有 print 指令（Phase 1 没 `32` write 之外的 I/O）
- **挂死时 CPU spin，内存稳定**——不是内存泄漏，是 busy loop

---

## yoy0 v0.4 实验（2026-07-16）— 区别于 yoyo.exe

> **重要**：yoy0（小写 yoy0，`yoy0/projects/yoy0.ty`）和 yoyo（大写 yoyo，`yoyo/projects/yoyo.ty`）是**两个不同项目**。上面 yoyo.exe 限制**不适用** yoy0 v0.4。

### yoy0 v0.4 vs yoyo.exe 关键差异

| 维度 | yoyo.exe（yoy0 大项目） | yoy0 v0.4（小项目实验） |
|------|------------------------|------------------------|
| CLI 参数 | ❌ 硬编码 `input.ky` → `output.exe` | ✅ argv-aware（GetCommandLineA → rdi=&save_slot → H_00） |
| 状态存储 | BSS（r15-based） | **栈**（`sub rsp, 0x1000; lea r15, [rsp+0x800]`） |
| H_50/H_51 | string table 查路径 | **rsi-arg**（路径从 RSI 寄存器，状态用 r12/r13/r14） |
| H_50 返回 | state[slot] = contentBuf | **rax = contentBuf, rdx = fileSize** |
| 测试方法 | `Copy-Item X.ty input.ky; .\yoyo.exe` | `.\yoy0-v0.4.exe input.ty output.exe` |

### yoy0 v0.4 调试教训（2026-07-16）

#### 1. 永远在三层拆解里找 x64 字节错（用 REX prefix 验证）
yoy0.ty v0.4 H_50 写成 `4C 89 F4` —— 看起来是 "mov r12, rsi"，但 REX byte `4C = 0100 1100`（R=1, B=0）跟 ModRM `F4 = 11 110 100` 配合是 `mov rsp, r14`（覆盖栈指针，必崩）。
- **正确 H_50 (mov rsi, r12)**：`4C 89 E6` (REX.R=1, B=0; ModRM E6 = reg=100, r/m=110)
- **正确 H_51 (mov rsi, r13)**：`4C 89 EE` (REX.R=1, B=0; ModRM EE = reg=101, r/m=110). **BUG ALERT**: Pre-2026-07-16 had `B6` (Mod=10 → memory with disp32, Reg=110=R14) which decoded as `mov [rsi+disp32], r14` — caused 0xC0000005 AV when the junk disp32 bytes (next instruction's `49 89 D0`) formed an unmapped address.
- **检查清单**：看到 `49 89 F4` 别直接照搬到别的段——REX 字节不同，ModRM 含义就不同

#### 2. 工具链差异：rust 链接受 6-tokens `00 00 50 00 00 00`，js 链 validator 拒绝
yoy0.js 编译器有严格 validator（`OP_MIN_ARGS[0x50] = 2`），6 tokens 是 "1 extra arg" 编译错误。
yoy0-rust 链没这限制，直接接受 6 tokens。
- **修 .ty 字节错时，先用 rust 链验证**（编译快、错误信息清楚），再考虑 js 链

#### 3. argv parser 必须检查 null，不能只检查 space
原 .ty 写 argv[0] scan 只检查 `0x20`（空格），不检查 `0x00`（null）。`yoy0-v0.4-rs.exe` 没有空格 → 死循环越界读 OS heap → STATUS_ACCESS_VIOLATION。
- **每个字符串 scan 循环都要**：`cmp [rsi], 0x20; je .null; cmp [rsi], 0; je .end; inc rsi; jmp .scan`

#### 4. jz/jcc disp 是从**下一条指令**起算，不是从本指令起算
原 .ty 写 `jz .copy_done; 74 02`，但 `.copy_done` 在偏移 16（jz 偏移 6，2 字节，next 偏移 8，target 偏移 16），disp 应是 `0x08` 不是 `0x02`。`0x02` 让 jz 跳到 `inc rsi`（循环没退出）→ 死循环越界。
- **检查清单**：写 jcc 后，从 jcc 的 next_instr_addr 重算 disp = target - next_instr_addr

#### 5. yoy0-rust disasm 命令有 bug（2026-07-16）
`yoyo.exe disasm <file.exe>` 报 `cannot read disasm: 系统找不到指定的文件` 即使文件存在。
- **workaround**：用 `yoyo.exe decode <file.ty>` 看 .ty → .text 字节映射（每个 source line → byte offset），从 source line 反推 .text 内容
- **真正的反汇编**：暂用 hexdump + 手动 x64 解码（用 REX prefix + ModRM 表）

#### 6. 工具链选型原则
按用户规则"v0.4 没调通之前不要用 js 和 ASM"：
- **yoy0 项目用 rust 链**（`yoyo-rust/target/release/yoyo.exe link`）
- **不用 js 链**（`yoyo-js` 工具链，validator 严格 + 6-tokens 不接受）
- **不用 asm 链**（`yoyo-asm` 工具链，PE linker 还在 WIP）

#### 7. 三个常被误诊的"老铁律"不适用 yoy0 v0.4
- ❌ "R15 = BSS state base" → v0.4 改**栈**
- ❌ "H_50 load input.ky" → v0.4 改 **rsi-arg**
- ❌ "3-chain DDC byte-equal" → v0.4 还不能 self-host，DDC 不可达
详见 `docs/yoy0-iron-rules.md` 末尾"v0.4 偏离"section 和 `experiments/007-v0.4-argv-self-host/STATUS.md`。

### 检查清单（yoy0 v0.4 测试前必做）
1. `cargo build --release -p verifier`（如果 yoy0-rust 源码改了）？
2. **用 rust 链 link**：`& yoyo-rust\target\release\yoyo.exe link <file.ty> <out.exe>`（不用 js/asm 链）
3. 验证 .exe 字节：hexdump 找 H_50/H_51 入口字节（4C 89 E6 / 4C 89 B6）
4. `.\yoy0-v0.4.exe input.ty output.exe` 跑（**传 CLI 参数**！不是 `Copy-Item input.ky`）
5. exit code 0 + output.exe 存在 = pass
6. exit code 0xC0000005 = AV，**用三层拆解找根因**（不要直接重试）

---

## 自托管编译器调试：永远检查二进制是否真的更新了

### 背景
调试 `ky-compiler` 自托管编译器时，`31 01` selector 始终不工作，折腾了几个小时。

### 根因
`ky-compiler.exe` 是**过时的旧二进制**。修改 `build-selfhost.js` 后：
1. `node build-selfhost.js` 更新了 `ky-compiler.ky`（源码）
2. 但**没有重新运行** `node ky-compiler.js projects/ky-compiler.ky` 来生成新的 `ky-compiler.exe`
3. 后续所有测试用的都是旧 `.exe`，导致改了什么都没用

### 教训
- **修改编译器源码后，必须重新编译编译器本身**，否则所有调试都是白费功夫
- 自托管编译器的调试链路是：`build-selfhost.js` → `.ky` → 用 `ky-compiler.js` **重新编译** → 新的 `.exe`
- 每当怀疑 fixup 或 emit 逻辑有问题时，**第一件事**是确认 `.exe` 的时间戳和 SizeOfCode
- 不要假设构建脚本输出了最新的二进制——**用 `ls -l` 或查看文件时间戳确认**
- 在 `.ky` 中插入 sentinel（如 `81 00 01 1020`）后，如果反汇编找不到对应 x86 模式，几乎可以确定二进制是旧的

---

## yoyo 字节码断言:不要在 .exe 字节流里找 yoyo opcode (2026-06-30)

### 背景
v2 e2e 第一次跑 div_loop 和 3way_branch 失败 "缺 0x69/0x83",我以为模型没学对。检查后**是我的断言写错了**——yoyo 字节码在 .exe 里被 yoyo.js 编译成 **x64 机器码**,**yoyo opcode (0x82/0x83) 在 .exe 里不会出现**。

### 正确的对应关系
| yoyo | x64 字节序列 |
|------|-------------|
| `82` (jl) | `0F 8C` |
| `83` (jg) | `0F 8F` |
| `70` (jmp) | `E9` |
| `71` (je) | `0F 84` |
| `41` (call) | `E8` |
| `FF` (ret) | `C3` |
| `FF` 也用作字符串终止 | `00`(null) |

### 教训
- **e2e 断言要在两层**:
  - **第一层**:yoyo 字节码文本(模型输出)是否含 pattern 关键 opcodes(`82/83/65` 等)
  - **第二层**:编译后的 .exe 字节流是否含对应的 **x86/x64 字节序列**(`0F 8C/0F 8F/E9/E8` 等)
- **不是所有 yoyo 模式都能在 .exe 字节流断言**:SMA 等纯算术模式编译成 `add/mov` 等多字节指令,光靠字节序列 grep 难定位
- **可定位的模式**:有 jmp/jcc/call 的模式最易断言 — jl(0x82)→`0F 8C`, jg(0x83)→`0F 8F`, call(0x41)→`E8`

### 推荐 e2e 断言模式
```python
PATTERN_YOYO = {
    "div_loop":    {0x30, 0x65, 0x82, 0x69, 0x66, 0x70, 0x40, 0xFF},  # 模型输出
    "3way_branch": {0x65, 0x82, 0x83, 0x70, 0x40, 0xFF},
    "orchestrator":{0x40, 0x41, 0xFF},
    "sma":         {0x30, 0x60, 0x68, 0xFF},
}
PATTERN_X64 = {
    "div_loop":    [b"\x0f\x8c"],            # jl
    "3way_branch": [b"\x0f\x8c", b"\x0f\x8f"],  # jl + jg
    "orchestrator":[b"\xe8"],                # call
    "sma":         [],                       # 纯算术,无 jmp 字节序列可断言
}
```

---

## Linux 自举链路：gen2.elf → gen3.elf 挂死调查 (2026-07-06)

### 背景
yoyo.js（Node.js）编译 yoyo.ty → gen2.elf（266KB，正常工作）。gen2.elf 编译 input.ky（即 yoyo.ty）→ gen3.elf（155KB）。但 gen3.elf 运行时挂死（CPU 100%，无输出）。

### 关键事实
- **gen2.elf 与 gen3_direct.elf（yoyo.js 直接输出）哈希完全一致** — yoyo.js 编译是确定性的
- **代码段都是精确 32KB** — gen3.elf 与 gen3_direct.elf 代码段大小相同
- **数据段尺寸差异大**：gen3.elf 数据段 = 118KB vs gen3_direct.elf = 229KB
- **代码段内容从 vaddr 0x403141 开始分歧** — gen2.elf 发出的 x64 代码比 yoyo.js 大（因为总是用 disp32 而非 disp8）

### 已证伪的假设
**disp8/disp32 编码错误假设**：怀疑 gen2.elf 对 state_id ≥ 16 错误使用 signed disp8 导致读取错误内存。
- **验证**：扫描 gen3.elf 全部 32768 字节代码段，零个 `49 8B 47 XX`（disp8 stGet）或 `49 89 47 XX`（disp8 stPut）目标 slot≥16
- 两个编译器都正确使用 disp8（slot 0-15）和 disp32（slot 16+）。gen2.elf 只是始终用 disp32（更大编码），语义正确。

### 根因：数据段尺寸不匹配（已修复 2026-07-06）
yoyo-gen.js 的 `OUTPUT_DATA_NEED` 算出 0x1D000，但 yoyo.js 的 `finish()` 固定分配 0x38000。
gen2.elf 用 0x1D000 打补丁到输出 ELF 头 → gen3.elf 数据段仅 118KB → 运行时溢出挂死。

**修复**（`F:\yoyo-ide\src\yoyo-gen.js` lines 124-156）：
1. `OUTPUT_DATA_NEED` 加上 yoyo.js `finish()` 的固定分配大小（`0x10000 + STATE_BUF_OFF + 0x20000 = 0x38000`）
2. `TPL_BLOB_DATA = OUTPUT_DATA_NEED`，嵌入式模板自带正确数据段大小
3. 删除 ELF/PE 头补丁代码（`SET(0x53, TPL_DATA_SIZE); 55 47 53`），因为模板已正确

**状态**：Linux 已修（gen2.elf 数据段 0x38000 ✓）。Windows M2→M3 仍有独立 bug（AV 崩溃）。

### 教训
- **disp8/disp32 编码差异不一定是 bug** — 大编码格式语义相同，不影响功能
- **代码段大小相同但数据段差 2x 是强烈信号** — 数据段布局差异比代码段编码差异更可能是根因
- **"看起来像"的解释必须经过二进制验证** — 我是扫描了全部 32KB 代码段的 disp8 模式才确认假设是错的
- **三层拆解的第三条落地了**: 源码推测 → TIR 验证 → 机器码验证。disp8 假设在机器码层被彻底反驳

### 检查清单（怀疑自举编译器差异时）
1. 对比 hash：gen2.elf == gen3_direct.elf？
2. 对比数据段大小（程序头的 p_filesz）
3. 扫描代码段找 disp8/disp32 模式验证编码差异
4. 对比数据段前 4KB 查看 handler table 布局
5. 检查 fixup 表条目数是否匹配 yoyo.ty 中的分支/调用指令数

---

## Windows 自举链路（2026-07-06）

### 测试流程
```powershell
node src/yoyo-gen.js --target=win              # 生成 yoyo.ty（615KB）
node src/yoyo.js --target=win projects/yoyo.ty build\yoyo.exe  # gen1（317440 字节）
Copy-Item projects/yoyo.ty input.ky            # 准备自举输入
.\build\yoyo.exe                               # M1→M2
Copy-Item output.exe gen2-m2.exe              # 保存 M2
.\gen2-m2.exe                                  # M2→M3
```

### 结果
- **✅ M1→M2（yoyo.js → yoyo.exe → output.exe）**：通过。gen2 = 264192 字节，退出码 0
- **❌ M2→M3（gen2.exe 编译 input.ky）**：失败。gen2.exe 立即 `STATUS_ACCESS_VIOLATION (0xC0000005)`，未产出 output.exe

### 与 Linux 的差异
| 平台 | gen1→gen2 | gen2→gen3 | 失败表现 |
|------|-----------|-----------|---------|
| Linux | ✅ gen2.elf=266KB | ✅ 已修复（数据段 0x38000） | gen3→gen4 全链路通过 |
| Windows | ✅ output.exe | ❌ gen2 自身 AV 崩溃 | 0xC0000005，崩溃地址 0x121C |

### 根因
**Linux 已修**：数据段尺寸不匹配，yoyo-gen.js 的 `OUTPUT_DATA_NEED` 加上 0x38000 地板后解决。
**Windows 未找到**：gen2 在 `rep movsb` 处崩溃（偏移 0x121C），count 值 0x12E800 应为 0x1E800。
- 排除 overlay 构建 bug（scanner/emitter 路径也一样崩溃）
- 排除模板/缓冲区溢出（已分配 0x60000）
- 问题出在 SCANNER 读取 hex 值 `1e800` 时产生错误值
- 确切根因待查

---

# Windows 调试工具速记
# Windows 调试工具速记

## 项目路径
- 项目根: `F:\yoyo-org`
- 当前测试: `F:\yoyo-org\yoyo-test\`
- yoyo 源码: `F:\yoyo-org\yoyo-js\projects\yoyo.ty`
- 调试脚本: `F:\yoyo-org\yoyo-rust\dbg_*.txt`
- 历史调试 session: `F:\yoyo-org\yoyo-rust\cdb_*.txt`
- yoy0 项目: `F:\yoy0\projects\` (通常不存在,大写 yoyo 是主项目)
- yoy0-rust 工具链: `F:\yoy0-rust\target\release\yoyo.exe`

## WinDbg Preview 已装 (2026-07-17)

### cdbX64.exe 别名 (推荐用这个)
```
C:\Users\Administrator\AppData\Local\Microsoft\WindowsApps\cdbX64.exe
```
这是 App Execution Alias, 直接 `& $cdbPath ...` 可用。

### 实际 cdb.exe (WinDbg AppX)
```
C:\Program Files\WindowsApps\Microsoft.WinDbg_1.2606.22001.0_x64__8wekyb3d8bbwe\amd64\cdb.exe
```
版本: 10.0.29617.1000

### WinDbg GUI (WinDbgX)
```
C:\Users\Administrator\AppData\Local\Microsoft\WindowsApps\WinDbgX.exe
```
是 0 字节 reparse alias, 实际是 `DbgX.Shell.exe` in WindowsApps package

### ProcDump
```
C:\Users\Administrator\procdump\procdump64.exe
```
版本: 12.01

## WER 状态
- LocalDumps 已配 (DumpType=2=full, DumpCount=10)
- 默认输出: `%LOCALAPPDATA%\CrashDumps\*.dmp`
- 解析 Event Log: `Get-WinEvent -LogName Application -ProviderName "Application Error"`

## cdb 常用命令
```
g            # go
sxd av       # 不要拦截 AV
sxd *        # 不要拦截任何异常
sxi -c2 av   # 记录但不拦截
k / kb       # 栈 (kb 含前 3 参数)
r            # 寄存器
r rip        # 看 RIP
.ecxr        # 切到 exception context (dump 模式必用)
!analyze -v  # 自动分析异常
~0s          # 切线程
dqs <addr>   # 8 字节 quad dump
u <addr>     # 反汇编
bp <addr>    # 设断点
.sympath srv*  # 装符号
.reload /f      # 强 reload 符号
```

## 启动示例

### 启动 gen2.exe 让它在异常处停下
```powershell
& "C:\Users\Administrator\AppData\Local\Microsoft\WindowsApps\cdbX64.exe" `
   "F:\yoyo-org\yoyo-test\gen2.exe" `
   -c "sxd av; g"
```

### 用脚本驱动
```powershell
& "C:\Users\Administrator\AppData\Local\Microsoft\WindowsApps\cdbX64.exe" `
   -cf "F:\yoyo-org\yoyo-rust\dbg_av.txt" `
   "F:\yoyo-org\yoyo-test\gen2.exe" `
   -c "g"
```

### 解析现有 dump
```powershell
& "C:\Users\Administrator\AppData\Local\Microsoft\WindowsApps\cdbX64.exe" `
   -z "$env:LOCALAPPDATA\CrashDumps\gen2.exe.XXXX.dmp" `
   -c ".ecxr; r; k; !analyze -v; q"
```

## ProcDump 自动捕获
```powershell
& "C:\Users\Administrator\procdump\procdump64.exe" `
   -ma -e 1 -f c0000005 `
   "F:\yoyo-org\yoyo-test\gen2.exe" `
   "$env:LOCALAPPDATA\CrashDumps\gen2-crash.dmp"
```

## 教训 (2026-07-20)
- **永远不要在 PowerShell 直接搜 `cdb.exe`** — 它在 WinDbg AppX, 不在 Windows Kits。
- **路径里有空格 + WindowsApps 路径** — 必须用 `& "path..."` 调用运算符。
- **REPRODUCE-WINDBG-INFO**: 如果之前的项目运行过 cdb/kscript, 查 `F:\yoyo-org\yoyo-rust\cdb_*.txt` 找版本和路径。
