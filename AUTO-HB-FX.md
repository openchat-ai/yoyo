# AUTO-HB-FX — peer 现场吐 `yoyo_compiler.exe`

> 补 REAL 诚实残留 HB-FX。  
> **禁止**把 `scripts/fixtures/yoyo_compiler.exe` 拷贝说成 peer emit。  
> **禁止**链上 `cargo … --place-compiler` 冒充 peer。  
> **禁止** invent Stage 17；**禁止** Thompson 宣称。

```
ACTIVE=0
COMPLETED=1
COMPLETED_AT=2026-11-07
PARENT=AUTO-REAL-FIXEDPOINT.md (HB-FX)
NORTH_STAR=CreateProcess 子进程 yoyo_compiler 由 JS/asm/YOYO peer 现场物化；链上 emitter≠fixture_copy≠rust_place
STOP_ONLY=停 | ACTIVE=0 | hard-block | COMPLETED=1
```


## DONE 谓词（全绿才 COMPLETED=1）

| ID | 谓词 |
| --- | --- |
| X1 | **peer place API**：`node yoyo-js/scripts/place-yoyo-compiler.js <dir>` 写出 `yoyo_compiler.exe`，**不读** fixture，**不** cargo place |
| X2 | 产物能跑 Gate I / FULLBAR H00 链：CreateProcess 子进程 exit=0 且产出 ≡ 对照不动点 |
| X3 | FULLBAR / Gate I `emitter=js_peer_link`（≠ `fixture_copy`） |
| X4 | payload pin 记 `chain_emitter=js_peer_link` + sha/size；`payload_origin≠rust_parse1_emit` |

## 悬崖 / 楔子

| 层 | 状态 |
| --- | --- |
| 链上 fixture_copy / cargo place | **已去掉**（Gate I / FULLBAR / HB-FX 门禁） |
| JS `linkPeWin32` 链上 place | **GREEN** · 产物 ≡ fixture `e2a9c566…` |
| parse1 **算法** emit 在 JS | **GREEN** · data bake + code source-interp ≡ dump；origin=`js_parse1_source_interp` |

## Checklist

- [x] **FX-0**（2026-11-07）：悬崖钉死  
- [x] **FX-1** place 入口  
- [x] **FX-2a**（2026-11-07）：JS link payload → PE ≡ fixture `e2a9c566…`；`stage-hb-fx-peer-compiler.ps1` GREEN  
- [x] **FX-2b-1**（2026-11-07）：JS `buildCompilerDataPrelude` ≡ dump 前 **42560** B（live2+RET expand+compact）  
- [x] **FX-2b-2**（2026-11-07）：+ H00 hdr/main/data bake from JS seed ≡ dump 前 **275712** B  
- [x] **FX-2b-3**（2026-11-07）：+ field-match 8 singles + 7 CHAINS → **data.bin FULL EQUAL**（421920 B）  
- [x] **FX-2b-4**（2026-11-07）：**code.bin FULL EQUAL**（13011 B）via JS source-interp of `yoyo_compiler_ret_compile_code_data` emit body；place 默认 live emit；origin=`js_parse1_source_interp`  
- [x] **FX-3**（2026-11-07）：Gate I + FULLBAR `js_peer_link` 绿  
- [x] **FX-DONE**（2026-11-07）：`stage-hb-fx-peer-compiler.ps1` GREEN · place live emit · vs_fixture EQUAL `e2a9c566…` · origin=`js_parse1_source_interp` · `COMPLETED=1`  

> **诚实**：code emit = JS **source-interp** of `pe_dll_link.rs` parse1 body（读 Rust **源** recipe，不 cargo / 不 dump）；手写逐行 JS port 非毕业必须。≠ Thompson。

## 触发

`HB-FX` / `继续` / `继续 HB-FX` / `下一刀` → 下一未勾 FX-\*。

---

*2026-11-07 · FX-2b-4 code source-interp · ≠ Thompson*
