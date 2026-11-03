# Post-v1.0 关洞看板（path 2 · OW-RT / S3）

## 北星

v1.0 已毕业（`ACTIVE=0`）。本板 = **OW-RT 长杆日常**（S3）。**禁假 CLOSED** · **禁 invent Stage** · `closed=0 cut=7`。

**默认** 继续 **= S3.687**（686=M4/live map 诚实刷新；下一刀=M4 production flip prep / place_cwd 仍 Rust）。

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

**诚实底线**：CreateProcess(compiler)≡live-compose；densify=sheep；M4 flip = place_cwd 默认仍 Rust + inventory CUT。

### 波段 tip（更早单刀已折）

| 波段 | tip | 一句话 |
| --- | --- | --- |
| **S3.1–630** ✅ | `3809e18` | fixed10→parse1→any-slot→text4KiB |
| **S3.631–638** ✅ | `3227356` | yoyo.tyb child→CreateProcess GREEN |
| **S3.639–655** ✅ | `7f27b15` | compact→mid-grow→**full-grow / yoyo≈27KiB** |
| **S3.656–683** ✅ | — | HOT→H00→**parse1≡exec + stage live** |
| **S3.684** ✅ | — | export-local live emit **cliff** |
| **S3.685** ✅ | — | **CreateProcess-as-contract**（A） |
| **S3.686** ✅ | — | M4/live map **诚实刷新** |
| **S3.687** ⏳ | — | **本步**：M4 flip prep（place_cwd 默认 Rust） |

**缺口**：M4 production flip **ABSENT**（default=Rust；inventory CUT）。`ready=false`（M4）。bake=362。

---

## 验收（日常）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- yoyo_m4_live_compose_map_refresh_pin yoyo_h00_writefile_seed_pin --test-threads=1
```

path 2 回归才跑：`stage17-ow-*` / `stage15-hole-inventory` / `stage16-*`（见 `scripts/`）。

---

## 对 AI

- 继续 → **S3.687**（M4 flip prep；禁 densify / 假 CLOSED / bake-sole-seed-WF）。S3.686=map refresh。红了记根因停。
- OW-* GREEN ≠ CLOSED；本地先绿；2 次红 CI 停推；AUTO `ACTIVE=0`。

---

*post-v1.0 · 收纳板 · tip S3.687*
