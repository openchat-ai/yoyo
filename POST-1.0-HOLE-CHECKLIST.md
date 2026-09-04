# Post-v1.0 关洞负责人看板（path 2 · 缩宿主信任 · 整仓竣工）



## 北星：打破后门魔咒



YOYO v1.0 已毕业（`ACTIVE=0` · `COMPLETED=1`）。**ROADMAP 止于 Stage 16 / v1.0** — 本看板 **不是 Stage 17 功能轨**，而是 post-v1.0 **path 2 关洞**：逐项缩小 OW-* 宿主信任、诚实 CUT/CLOSED、**禁止假 CLOSED**、**禁止 invent 新 Stage 功能**。



> **用途**：用户说 `继续` / `关洞` / `整仓竣工` 时的 post-v1.0 主线（`AUTO_TO_1.0.md` 为 `ACTIVE=0` 时读 **本文件**，勿启 AUTO invent Stage 17）。  

> **范围**：`SCOPE-CUT-v1.0-hole-inventory.md` 七项 disposition 的 **诚实推进**；非 MCU / Morph 主赛道。  

> **基线**：Stage 16 已毕业（2026-08-29）；tag `v1.0.0`；Lock pin `0275802d…`（Decision #25）；Gate C 重测 `HOLE_INVENTORY_V10 status=FINAL` · **closed=0 cut=7**（OW-H00 因 full `.text` DIFF 回 CUT · 禁止假 CLOSED）。  

> **整仓竣工**：语言轨 v1.0 已毕业 ≠ 七洞全 CLOSED。长杆 = **OW-RT YOYO-built runtime**（多月）；REL-FULLTEXT **永不**作毕业 CLOSED；REL-STUBOS 待生产 I/O。



## 🎯 进度总览



```text

[x] A  [x] B  [x] C   →  path 2 里程碑（无 tag）

[x] D  [x] E  [x] F   →  OW-RT DLL emit + origin stub + YOYO-built effect（仍 CUT）

[ ] G                 →  drop Rust sidecar → OW-RT CLOSED

```



> **关于「打钩」**：`- [x]` = 已勾，`- [ ]` = 未勾。Markdown 预览才显示为 checkbox 符号。  

> **脚本名 `stage17-*`** = post-v1.0 **门禁编号**（OW-IAT / OW-RT），**非** ROADMAP Stage 17。



---



## 阻塞



| 项 | 状态 | 说明 |

|----|------|------|

| **with-sidecar manual-map** | ✅ **Gate A 已绿（PR #26 · `f8eb429`）** | no-sidecar fail-closed + with-sidecar GREEN · **OW-IAT 仍 CUT** |

| **整仓竣工长杆** | **OW-RT YOYO-built runtime** | Gate D–F 已绿；G 才可能 CLOSED；**禁止**假 CLOSED |

| **勿做** | — | 勿 fake OW-IAT/OW-RT CLOSED；勿启 `AUTO_TO_1.0 ACTIVE=1`；勿 invent Stage 17 |



---



## 如何打开看板



| 方式 | 操作 |

| ---- | ---- |

| **完整路径** | `F:\yoyo\POST-1.0-HOLE-CHECKLIST.md` |

| **Cursor 内** | `Ctrl+P` → `POST-1.0-HOLE` |

| **洞清单定稿** | `F:\yoyo\SCOPE-CUT-v1.0-hole-inventory.md` |

| **OW-RT spike** | `F:\yoyo\SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md` |

| **v1.0 毕业看板（历史）** | `F:\yoyo\STAGE16_OWNER_CHECKLIST.md`（全绿 · 勿回改） |



相关：`RELEASE-v1.0.md` · `BACKEND_SUPPORT.md` · `AUTO_TO_1.0.md`（`ACTIVE=0` · `COMPLETED=1`）· `.cursor/rules/ci-anti-thrash.mdc`。



---



## 零指令执行（post-v1.0）



| 方式 | 操作 |

| ---- | ---- |

| **触发词** | `继续` / `关洞` / `post-1.0` / `path 2` / `整仓竣工`（**非** `ACTIVE=1` AUTO） |

| **单轨** | A→B→C→D→E→F→G；一项 per tick；本地验绿再勾 |

| **AUTO** | `ACTIVE=0` → **停**；读本看板，**不** invent Stage 17 |

| **CI** | gate 不是 debugger；WIP 用 `[skip ci]`；同 PR 连续 2 次红全量 CI → 停推改本地 |



**下一项** = **G**（续）— 生产去 Rust sidecar → OW-RT CLOSED（本 tick 仅 in-DLL recompile 切片 · 生产默认仍 Rust · **仍 CUT**）。



---



## 约束



0. **打破后门魔咒（北星）** — 每项须说明如何缩小宿主信任或诚实 CUT。

1. **诚实 disposition** — CUT 项不得标 CLOSED；OW-IAT/OW-RT 在 sidecar `yoyo_rt.dll` / Rust runtime 仍在时 **必 CUT**。

2. **绿了才勾** — 未跑验收命令不勾 `[x]`。

3. **v1.0 不退化** — 勾任一项前 stage16-v09-regress 或等价不得红。

4. **非里程碑 WIP 不 push** — path 2 无 tag/release；候选 fix 本地绿 → 一次 push。

5. **ROADMAP 终站** — Stage 16 已毕业；本看板 **不是** Stage 17。



---



## 洞清单映射（SCOPE-CUT v1.0 FINAL）+ 整仓竣工要求



| ID | Disposition | 看板门 | 整仓竣工要求（CUT→CLOSED） |

|----|-------------|--------|---------------------------|

| **OW-H00** | **CUT** | C | full `.text` three-peer **EQUAL** + body EQUAL（Gate A 后 DIFF · 勿假 CLOSED） |

| **OW-STUB** | CUT | A/C | `stub_tail_nonzero==0`（需去 stub / 并入可比窗） |

| **OW-RT** | CUT | **D→G** | **YOYO-built** runtime；无 Rust `yoyo_rt.dll` / `.so` 宿主信任（长杆） |

| **OW-IAT** | CUT | A/C→G+ | 无 `yoyo_rt.dll` sidecar 标记（依赖 OW-RT 去 sidecar 或内嵌 YOYO runtime） |

| **OW-SEED** | CUT | C | 非 Rust `yoyo.exe` 发射路径证据 |

| **REL-FULLTEXT** | CUT | C | **设计上不毕业 CLOSED**（DIFF→CUT；EQUAL 仅 PARTIAL） |

| **REL-STUBOS** | CUT | C | Plan9/FreeBSD/Haiku/Serenity **生产 I/O**（非本长杆优先） |



---



## 关洞三门（A / B / C）+ 整仓竣工（D→G）



### 待做 / 已勾



- [x] **基线（历史）：OW-H00 曾 CLOSED** — 曾 three-peer EQUAL · **`72c27c9f`** / 18944 B · **Gate C 重测后回 CUT**（full DIFF）· 勿假 CLOSED



- [x] **A：Win OW-IAT wire-up smoke GREEN** — **2026-09-02 · PR #26 · `f8eb429` · CI [33662626655](https://github.com/openchat-ai/yoyo/actions/runs/33662626655)** · **本地复验 2026-09-03 tip `64a78d9` GREEN**  

  - **验收**：`& .\scripts\stage17-ow-iat-wireup.ps1` exit 0 ✅  

  - **诚实状态**：OW-IAT **仍 CUT**（sidecar + kernel32 I/O）；全脚本 GREEN **≠ CLOSED**



- [x] **B：Linux OW-IAT / tramp 回归不退化** — **2026-09-02 · 同 CI run · `f8eb429`**  

  - **验收**：`stage10-linux-pure-m4.sh` — `H_00 chain gen1→gen4: GREEN` ✅  

  - **诚实状态**：OW-IAT **仍 CUT**（dlopen + ld.so libc + cwd sidecar `.so`）



- [x] **C：洞清单 sync + BACKEND_SUPPORT 诚实状态** — **2026-09-03 · Gate C honest sync**  

  - **验收**：`stage16-scope-cut-finalize` + `stage15-hole-inventory` -SkipBuild · `closed=0 cut=7` ✅  

  - **诚实状态**：七项 CUT；smoke GREEN ≠ CLOSED



- [x] **D：OW-RT PE DLL emit spike** — **2026-09-04 · 整仓竣工第一实质步**  

  - **验收**：`& .\scripts\stage17-ow-rt-yoyo-runtime.ps1` exit 0 · `cargo test -p verifier pe_dll_link` ✅  

  - **产物**：`pe_dll_link.rs` · `SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md`  

  - **诚实状态**：`yoyo_built=ABSENT` · Rust sidecar **PRESENT** · **OW-RT 仍 CUT**  

  - **信任链**：可发射 ordinal-0=`yoyo_runtime_selfhost_main` 的 PE32+ DLL（H_00 同契约）；为 YOYO-built 铺路 · **≠ CLOSED**



- [x] **E：YOYO-origin stub 填 export body** — **2026-09-04 · 整仓竣工**  

  - **验收**：`& .\scripts\stage17-ow-rt-yoyo-runtime.ps1` exit 0 · `yoyo_origin_export=PRESENT` ✅  

  - **产物**：`ow_rt_yoyo_origin_exit2.ty` → RAW_BYTES+RET = `B8 02 00 00 00 C3` · `pe_dll_link` 用 YOYO-origin body  

  - **诚实状态**：`yoyo_built=ABSENT` · Rust sidecar **PRESENT** · **OW-RT 仍 CUT**  

  - **信任链**：export `.text` 经 YOYO emit；DLL 壳仍 Rust · **≠ CLOSED**



- [x] **F：YOYO-built read→compile→write 效应** — **2026-09-04 · 整仓竣工**  

  - **验收**：& .\scripts\stage17-ow-rt-yoyo-runtime.ps1 exit 0 · yoyo_built_effect=PRESENT · exits 0/1/2/3 ✅  

  - **产物**：pe_dll_link::yoyo_built_runtime_effect · fixture selfhost_min_nop.ty  

  - **诚实状态**：yoyo_built=EFFECT · Rust sidecar **PRESENT** · **OW-RT 仍 CUT**  

  - **信任链**：YOYO seed/link 路径完成 R→C→W（无 LoadLibrary）；生产仍 Rust sidecar · **≠ CLOSED**



- [ ] **G：生产去 Rust sidecar → OW-RT CLOSED 证据**  

  - **目标**：H_00 只用 YOYO-built；inventory fail-closed `disposition=CLOSED`  

  - **验收**：`stage16-scope-cut-finalize` 含 OW-RT CLOSED；无 `yoyo_rt.dll` Rust 宿主信任  

  - **依赖**：E+F；随后才可能推进 OW-IAT CLOSED  

  - **本 tick 切片（仍 CUT · 勿勾 CLOSED）**：`yoyo_sidecar_in_dll_recompile` + `emit-rt-sidecar --in-dll-recompile` · call-time `ReadFile` + 多条目 YOYO oracle（input→PE）· gate `yoyo_in_dll_recompile=PRESENT` · `yoyo_built=IN_DLL_RECOMPILE` · `gate_g_slice=in_dll_recompile` · production_default=RUST · oracle **≠** 完整 YOYO in-DLL 编译器  

  - **Win 本地**（云无 Win）：`& .\scripts\stage17-ow-rt-yoyo-runtime.ps1`；in-dll-recompile exit 0；两 fixture 不同 PE；H_00+YOYO no-input **exit=2**（可 NOT_STABLE）  

  - **Linux/云**：`bash scripts/stage17-ow-rt-yoyo-runtime.sh`（unit + in-dll-recompile）



### path 2 里程碑（A+B+C 全绿 · 无 tag）



**完成：2026-09-03** · master tip **`11a2cea`**（PR #27）· 观测：七项 **CUT**（`closed=0 cut=7`）· OW-IAT smoke GREEN ≠ CLOSED · stub **2673** · DLL **158720** · **无 tag / GitHub Release**（v1.0 已毕业）。



### 整仓竣工进度（D→G · 无假 CLOSED）



**Gate D 完成：2026-09-04** · `pe_dll_link` + gate GREEN · **仍 cut=7**。  

**Gate E 完成：2026-09-04** · YOYO-origin export stub · `yoyo_origin_export=PRESENT` · **仍 CUT**。  

**Gate F 完成：2026-09-04** · YOYO-built R→C→W effect · `yoyo_built=EFFECT` · **仍 CUT**。  

**Gate G 切片（未勾）：2026-09-04** · in-DLL recompile · `yoyo_in_dll_recompile=PRESENT` · `yoyo_built=IN_DLL_RECOMPILE` · production_default=RUST · oracle ≠ 完整编译器 · **仍 CUT** · **G 仍 `[ ]`**。



---



## 验收命令



```powershell

cd F:\yoyo



# Gate A — Win OW-IAT wire-up（post-v1.0 门禁；脚本名 stage17 ≠ ROADMAP Stage 17）

& .\scripts\stage17-ow-iat-wireup.ps1



# Gate B — Linux tramp / sidecar 回归

& wsl -e bash /mnt/f/yoyo/scripts/stage10-linux-pure-m4.sh



# Gate C — 洞清单 FINAL + Stage 15-A 不退化

& .\scripts\stage16-scope-cut-finalize.ps1 -SkipBuild

& .\scripts\stage15-hole-inventory.ps1 -SkipBuild



# Gate D — OW-RT PE DLL emit spike（整仓竣工）

& .\scripts\stage17-ow-rt-yoyo-runtime.ps1



# v1.0 全回归（post-v1.0 修洞前/后 sanity）

& .\scripts\stage16-v09-regress.ps1 -SkipBuild



# 日常 DDC

cd F:\yoyo\yoyo-rust\verifier

cargo run -- test ddc

```



> **注**：`stage17-*.ps1` 文件名沿用 post-v1.0 门禁编号；**不**表示 ROADMAP 存在 Stage 17。



---



## 对 AI 说什么（复制粘贴话术）



### 任务 D — OW-RT DLL emit（已勾）



```text

Post-v1.0 整仓竣工 Gate D：pe_dll_link PE32+ DLL emit spike。

验收：& .\scripts\stage17-ow-rt-yoyo-runtime.ps1 exit 0。

约束：yoyo_built=ABSENT；OW-RT 仍 CUT；勿假 CLOSED。

```



### 任务 E — YOYO-origin stub export



```text

Post-v1.0 整仓竣工 Gate E：YOYO-origin stub 填 pe_dll_link export body。

目标：固定 exit probe；门禁 yoyo_origin_export=PRESENT；仍 CUT。

约束：本地先绿；不替换生产 sidecar 则禁止 OW-RT CLOSED。

```



---



## 负责人原则



0. **path 2 = 关洞，不是新功能轨** — 只缩 OW-* / 诚实 CUT；禁止 invent ROADMAP 外能力。

1. **OW-H00 勿假 CLOSED** — Gate C 重测 full `.text` DIFF → **CUT**；slot 对齐 ≠ CLOSED。

2. **OW-IAT / OW-RT GREEN ≠ CLOSED** — sidecar / Rust runtime 仍在则必 CUT。

3. **整仓竣工长杆诚实** — YOYO-built runtime 多月；Gate D 只是发射基础设施。

4. **CI anti-thrash** — 本地 smoke 先绿；连续 2 次红 CI → 停推。

5. **AUTO 停手** — `ACTIVE=0` · `COMPLETED=1`；用户 `继续/关洞/整仓竣工` 才读本看板 tick。



---



*创建：2026-08-31 · v1.0 毕业后 · post-v1.0 path 2 关洞 · 模板对齐 STAGE16_OWNER_CHECKLIST.md*



**当前分支诚实快照（2026-09-04 · Gate G 切片）：** path 2 A–F · G 切片 in-DLL recompile · `closed=0 cut=7` · OW-RT **CUT**（`yoyo_in_dll_recompile=PRESENT` · `yoyo_built=IN_DLL_RECOMPILE` · Rust production default PRESENT · oracle ≠ 完整 YOYO 编译器）· **G 仍 `[ ]`** · **无 tag**


