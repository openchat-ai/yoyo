# SCOPE-CUT — v0.8 outside-window bytes（Stage 14-A 草案 · post-v1.0 OW-RT sync）

> **Status:** ACTIVE（草案 · 可机器验收）  
> **Gate:** `scripts/stage14-outside-window-scope-cut.ps1`  
> **Rule:** 本文件是 **诚实裁剪**，不是失败。不得用 selfhost-body EQUAL 假装 full `.text` EQUAL。

**Post-v1.0 OW-RT：** Win H_00 **无 exact embed**；sidecar `yoyo_rt.dll`；DLL **141312**。仍 CUT。
**Post-v1.0 OW-IAT wire-up（PR #8 · 2026-08-29）：** H_00 **manual-map** stub（CreateFile/Read/VirtualAlloc + `pe_manual_map`）；PEB `LoadLibraryA` **DROPPED**；JS hex template lockstep。**仍 CUT**（sidecar `yoyo_rt.dll` + kernel32 file I/O）。
**Post-v1.0 OW-H00 CLOSED（JS IAT sync）：** JS `KERNEL32_IO_FUNCS` 6-func 对齐 Rust；**three_peer_full=EQUAL** · **`72c27c9f`** · **18944** B。**OW-H00 CLOSED**。

---

## 为何 CUT

三 peer 在 **selfhost-body 窗**（PE startup + post-H_00 shared handlers）已 **EQUAL**（≥17013 B；观测 **17805** B）。  
full `.text` 仍 **DIFF**，因为窗 **外** 仍有宿主 / 发射面字节。若不把窗外钉成 SCOPE-CUT，验收会落入「整段 DIFF 但窗外仍算绿」的盲区。

**DDC = detection bar，不是 Thompson 证明。**

---

## CUT 清单（窗外 · 诚实剩余）

| ID | 区域 | 在哪 | 观测 / 钉 | v0.8 态度 |
|----|------|------|-----------|-----------|
| **OW-H00** | H_00 entry slot（18 B）+ manual-map stub | PE `.text` emit 头 | three-peer full `.text` **EQUAL** · body window **EQUAL** | **CLOSED** — `72c27c9f` / 18944 B |
| **OW-STUB** | H_00 manual-map stub tail | `.text` emit 后 | `stub_tail_nonzero` 钉 **[40, 950]**；观测 **905** B | **CUT** — 窗外 stub 仍非零 |
| **OW-RT** | Sidecar Rust `yoyo_runtime.dll` | cwd sidecar（非 PE `.data` embed） | size **≤150000**；观测 **141312**；**no exact embed** | **CUT** — 不在 gen12 / body 窗 |
| **OW-IAT** | CreateFile/Read/VirtualAlloc + sidecar | PE IAT + stub 调用 | **无** PEB LoadLibraryA；**仍** `yoyo_rt.dll` sidecar + kernel32 I/O | **CUT** — 非 YOYO-built loader；sidecar smoke pending |
| **OW-SEED** | Seed 仍由 Rust `yoyo.exe` 发射 | host CLI | Stage 13/15：emitter+seed hash + path=h00；不消除 | **CUT** — 不骗 v0.8；继续诚实 |

**可比绿窗（非 CUT）：** selfhost-body / `yoyo test body-ddc` / gen12·fullbody 788-handler 窗。

---

## 禁止宣称

- ❌ 「full `.text` three-peer EQUAL」作为 v0.8-A 毕业话术（窗仍 DIFF 时）
- ❌ 「窗外已关」若 OW-\* 钉仍 ACTIVE
- ❌ Thompson-proof / 已消灭 Rust runtime
- ❌ 把「无 exact embed」说成 OW-RT CLOSED

允许：✅ 「selfhost-body EQUAL + 窗外 SCOPE-CUT 已机器钉住」

---

## 机器验收（exit 0）

```powershell
cd F:\yoyo
.\scripts\stage14-outside-window-scope-cut.ps1
# 或已有 release 二进制：
.\scripts\stage14-outside-window-scope-cut.ps1 -SkipBuild
```

Gate 必须同时：

1. **可比窗绿** — 委托 `stage12-selfhost-body-section-ddc.ps1 -SkipBuild`（三 peer body EQUAL + stub floor）
2. **窗外可观测且有顶** — stub 区间、runtime.dll 天花板、LoadLibrary 宿主标记、seed PE 天花板；**无 exact embed**
3. **诚实 full `.text`** — JS↔Rust full section-ddc：**DIFF → SCOPE-CUT ACTIVE**；若意外 EQUAL → 认 H_00/stub 可能已收口，**仍**要求 OW-RT/OW-IAT 钉住
4. **打印** `SCOPE_CUT status=…` 一行，便于 RELEASE / 看板引用

---

## 与 Stage 14 / v0.8 关系

Stage 14-A = 把窗外从「口头诚实」变成 **可脚本钉的 SCOPE-CUT 草案**。  
毕业不要求关完 OW-\*；要求 **钉住 + 可比窗不退化**。

---

## 观测基线（2026-08-29 · post JS IAT sync · three-peer EQUAL）

| Monitor | Value |
|---------|-------|
| selfhost-body compared | **17805** B EQUAL |
| full `.text` JS↔Rust↔asm | **EQUAL** · **`72c27c9f`** |
| three_peer_full | **EQUAL** |
| stub_tail_nonzero (JS=Rust) | **905**（pin [40, 950]） |
| gen12 / fullbody `.text` | SHA **`72c27c9f`** · compared **18944** B |
| runtime.dll | **141312**（no exact embed；sidecar） |
| seed PE (Rust link) | **249344**（≤270000） |
| OW-H00 | **CLOSED** |
| OW-IAT | manual-map **WIRED**；PEB LoadLibrary **DROPPED**；**仍 CUT** |
| Gate | `stage14-outside-window-scope-cut.ps1 -SkipBuild` · body-ddc · gen12 GREEN |

---

*Stage 14-A · 打破后门魔咒：窗外从盲区变成 SCOPE-CUT · post-v1.0 OW-RT sidecar + OW-IAT no-GPA sync*
