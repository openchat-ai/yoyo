# Post-v1.0 关洞看板（path 2 · OW-RT / S3）

## 北星

v1.0 已毕业（`ACTIVE=0`）。本板 = **OW-RT 长杆日常**（S3）。**禁假 CLOSED** · **禁 invent Stage** · `closed=0 cut=7`。

**默认** 继续 **= S3.685**（684=export-local live emit cliff；下一刀=dissect 非 densify 路径 / 禁把 parse1 塞进 DLL）。

| 阻塞 | 说明 |
| --- | --- |
| **长杆** | OW-RT → S3（去 Rust sidecar） |
| **勿做** | fake CLOSED · `AUTO ACTIVE=1` · invent Stage · 定点/oracle 当 CLOSED · CI 当调试器 |

Disposition / 洞表 → `SCOPE-CUT-v1.0-hole-inventory.md`。门禁明细 → `git log`。

---

## S3 主线

| | |
| --- | --- |
| **S1** ✅ | in-DLL mini codegen |
| **S2** ✅ | slim 真 I/O · StubPlatform **仍 stub** |
| **S3** ⏳ | child parse1 扩 ISA · 真自举不排假工期 |
| **S4/S5** | 全 ISA / 形式化 · 地图 |

**诚实底线**：child CreateProcess≡seed via live ≠ export-local emit ≠ M4。parse1≡exec；host emit_x86 仍 slim。

### 波段 tip（更早单刀已折）

| 波段 | tip | 一句话 |
| --- | --- | --- |
| **S3.1–630** ✅ | `3809e18` | fixed10→parse1→any-slot→text4KiB |
| **S3.631–638** ✅ | `3227356` | yoyo.tyb child→CreateProcess GREEN |
| **S3.639–655** ✅ | `7f27b15` | compact→mid-grow→**full-grow / yoyo≈27KiB** |
| **S3.656–679** ✅ | — | HOT→…→**H00 WriteFile≡seed**（bake text） |
| **S3.680** ✅ | — | parse1≡exec **cliff** |
| **S3.681** ✅ | — | parse1 **Win32 I/O** |
| **S3.682** ✅ | — | parse1 **ADD/SUB imm8** |
| **S3.683** ✅ | — | LDB oo imm8 + **stage live** / 去 bake |
| **S3.684** ✅ | — | **export-local live emit cliff** |
| **S3.685** ⏳ | — | **本步**：dissect 非 densify 路径（≠ 塞 parse1 进 DLL） |

**缺口**：export-local live emit **ABSENT**；M4 **ABSENT**。child≡seed via live；parse1≡exec ready。`ready=false`（M4）。bake=362。

---

## 验收（日常）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- yoyo_indll_export_live_emit_absent_cliff_pin yoyo_h00_writefile_seed_pin --test-threads=1
```

path 2 回归才跑：`stage17-ow-*` / `stage15-hole-inventory` / `stage16-*`（见 `scripts/`）。

---

## 对 AI

- 继续 → **S3.685**（dissect export-local 非 densify；禁 densify / 假 CLOSED / bake-sole-seed-WF）。S3.684=cliff。红了记根因停。
- OW-* GREEN ≠ CLOSED；本地先绿；2 次红 CI 停推；AUTO `ACTIVE=0`。

---

*post-v1.0 · 收纳板 · tip S3.685*
