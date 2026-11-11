# STUB.AI — 一页审（优先于反汇编）

> 自动生成。逐项勾完即可交差。≠ Thompson。

## 结论槽

- opens_all_ALLOW: **PASS** (10 CreateFileA)
- no_UNKNOWN_IAT: **PASS**
- no_BAN_substr: **PASS**
- CALLSEQ_pin_EQUAL: **PASS**
- generator_flags: **PASS** (0)

**VERDICT: GREEN**

## 打开序列（合同）

`input.tyb,output.exe,input.tyb,output.exe,emit.bins,yoyo_compiler.exe,code.bin,data.bin,code.bin,data.bin`

## 必读分场（跳读）

- `0x140001000` ENTRY / PE startup
- `0x14000100d` PLACE BODY
- `0x140001044` CreateFileA → input.tyb
- `0x140001074` ReadFile
- `0x14000109a` CHECK magic Y1PL
- `0x1400010bf` CHECK magic TYB
- `0x14000124e` VirtualAlloc
- `0x14000126d` BUILD PE MZ header
- `0x140001404` CreateFileA → output.exe
- `0x140001432` WriteFile
- `0x140001445` ExitProcess
- `0x140001464` DATA string "input.tyb"
- … +5 more in WALKTHROUGH

## 怎么用

1. 看 VERDICT；红则停
2. 扫打开序列是否只含允许名单
3. 需要细节才开 WALKTHROUGH / 带 `; call#` 的 disasm 行

