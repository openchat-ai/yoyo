# Post-v1.0 关洞看板（path 2 · OW-RT / S3）

## 北星

v1.0 已毕业（`ACTIVE=0`）。本板 = **OW-RT 长杆日常**（S3）。**禁假 CLOSED** · **禁 invent Stage** · `closed=0 cut=7`。

**默认** 继续 **= S3.631**（630=child .text≈4KiB；下一刀=text≥18KiB / M4）。

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

**诚实底线**：fixed10 / parse1 可 GREEN ≠ 全 mini-ISA ≠ OW-* CLOSED ≠ StubPlatform 变真 OS。明细 → `git log --grep=S3.`。

### 波段 tip（更早单刀已折）

| 波段 | tip | 一句话 |
| --- | --- | --- |
| **S3.1–521** ✅ | `5dd7651` | fixed10→host compose→DATA；禁折 FIXED10 |
| **S3.522–555** ✅ | `8ea1ee5` | CreateProcess child PE / M4-prep |
| **S3.556–618** ✅ | `7a5f435` | live→parse1≈mini cliff |
| **S3.619–626** ✅ | `972d78a` | ALU+mem any-slot≤255 |
| **S3.627–629** ✅ | `2e7240f` | yoyo.tyb 诊断；label/patch→1024 |
| **S3.630** ✅ | — | child .text raw→0x1000（≈4KiB） |
| **S3.631** ⏳ | — | **本步**：.text≥18KiB / yoyo.tyb / M4 |

**缺口**：yoyo.tyb 需 ~18KiB .text（child≈4KiB）/ M4 flip / general emit_x86 仍 **ABSENT**。`S1_SLIM_MAX_CODE=1024` 禁抬（sheep）。`ready=false`。bake=362。

---

## 验收（日常）

```powershell
cd F:\yoyo\yoyo-rust\verifier
cargo test --lib -- yoyo_compiler_pe_parse1_ --test-threads=1
```

path 2 回归才跑：`stage17-ow-*` / `stage15-hole-inventory` / `stage16-*`（见 `scripts/`）。

---

## 对 AI

- 继续 → **S3.631**（.text≥18KiB；禁 densify / 假 CLOSED / 单抬 slim-cap）。S3.630=text≈4KiB。红了记根因停。
- OW-* GREEN ≠ CLOSED；本地先绿；2 次红 CI 停推；AUTO `ACTIVE=0`。

---

*post-v1.0 · 收纳板 · tip S3.631*
