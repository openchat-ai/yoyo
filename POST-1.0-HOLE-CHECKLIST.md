# Post-v1.0 关洞看板（path 2 · OW-RT / S3）

## 北星

v1.0 已毕业（`ACTIVE=0`）。本板 = **OW-RT 长杆日常**（S3）。**禁假 CLOSED** · **禁 invent Stage** · `closed=0 cut=7`。

**默认** 继续 **= S3.691**（690=host slim len 0x46c5 / Δ=+307；下一刀=非 sheep；禁 flip / 禁 imm8-ify fixed10 ADD）。

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

**诚实底线**：CreateProcess≡live-compose；host LDB≡exec；host ADD/SUB **imm32**（fixed10）；M4 flip **禁** while CUT。

### 波段 tip（更早单刀已折）

| 波段 | tip | 一句话 |
| --- | --- | --- |
| **S3.1–630** ✅ | `3809e18` | fixed10→parse1→any-slot→text4KiB |
| **S3.631–638** ✅ | `3227356` | yoyo.tyb child→CreateProcess GREEN |
| **S3.639–655** ✅ | `7f27b15` | compact→mid-grow→**full-grow / yoyo≈27KiB** |
| **S3.656–683** ✅ | — | HOT→H00→**parse1≡exec + stage live** |
| **S3.684–688** ✅ | — | cliff→contract→map→prep→**gates** |
| **S3.689** ✅ | — | host **LDB oo imm8** |
| **S3.690** ✅ | — | host slim **0x46c5**（Δ=+307；ADD imm32 golden） |
| **S3.691** ⏳ | — | **本步**：非 sheep 下一片（禁 flip / 禁 fixed10 ADD imm8） |

**缺口**：host ADD/SUB imm32 golden（Δ=+307）；M4 flip **ABSENT**（CUT）。`ready=false`（M4）。bake=362。

---

## 验收（日常）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- yoyo_parse1_equiv_exec_absent_cliff_pin yoyo_h00_writefile_seed_pin --test-threads=1
```

path 2 回归才跑：`stage17-ow-*` / `stage15-hole-inventory` / `stage16-*`（见 `scripts/`）。

---

## 对 AI

- 继续 → **S3.691**（非 sheep；**禁** flip / raise slim / fixed10 ADD imm8 / densify / 假 CLOSED）。S3.690=slim len。红了记根因停。
- OW-* GREEN ≠ CLOSED；本地先绿；2 次红 CI 停推；AUTO `ACTIVE=0`。

---

*post-v1.0 · 收纳板 · tip S3.691*
