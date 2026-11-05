# Post-v1.0 关洞看板（path 2 · OW-RT / S3）

## 一眼进度

| | |
| --- | --- |
| **tip** | **S3.733** ✅ **E5 清闸设计已命名**（近路=co-place compiler PE；仍 ABSENT） |
| **主线** | S1✅ S2✅ **S3⏳** · S4/S5 未开 |
| **已站住** | E1–E4 · E5 **named** · **E5 clear 仍 ABSENT** |
| **硬闸** | G CLOSED **ABSENT**（oracle≠full compile）· inventory CUT |
| **禁** | 假 CLOSED / densify / invent Stage |
| **近刀** | 705–733 ✅ |
| **git** | tip **S3.733** · bake=362 |

## 下一刀

| tip | 做什么 | 禁 |
| --- | --- | --- |
| **S3.734** | E5 近路：`place_cwd` Off 同目录 co-place `yoyo_compiler.exe`（CreateProcess≡full-compile 证据） | 假 CLOSED · densify |

**缺口**：E5 clear / G CLOSED ABSENT。

## HARD BLOCK（runner-only · 勿当本地回归）

下列 `pe_dll_link` 测试保持 `#[ignore]`——**仅** GitHub `windows-latest` debug 复现；本地同 rustc 多轮不复现。主门禁 = `stage17-ow-rt-yoyo-runtime.ps1`（in-DLL recompile），不依赖它们。

| 测试 | CI 现象 | 说明 |
| --- | --- | --- |
| `yoyo_sidecar_export_compile_success_writes_pe` | `STATUS_ACCESS_VIOLATION` | 旧 export-compile 路径；见 issue #37 |
| `yoyo_sidecar_export_compile_probe_inproc` | `STATUS_STACK_BUFFER_OVERRUN` (`0xC0000409`) | `__fastfail` 绕过 VEH |
| `yoyo_sidecar_export_compile_probe_subproc` | 同上 | 子进程探针同崩 |

临时打开：`cargo test -p verifier --lib -- --include-ignored --exact <name>`。

## 验收

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- yoyo_e5_oracle_to_full_compile_design_pin yoyo_s3_732_human_e1_e2_e4_landing_pin --test-threads=1
```

---

*post-v1.0 · tip S3.733 · next co-place*
