# 04 — 工具链

## 主链（应用 · 语言主尺）

| 组件 | 路径 | 产出 |
|------|------|------|
| Verifier / linker | `yoyo-rust/` crate `verifier` | `yoyo-rust/target/release/yoyo.exe` |
| 命令 | `yoyo link --target=win32\|linux\|…` | PE/ELF/… |
| 后端矩阵 | `BACKEND_SUPPORT.md` | 多 target；**语言主尺默认 Win32** |

构建：

```powershell
Push-Location F:\yoyo\yoyo-rust
cargo build --release -p verifier
Pop-Location
```

DDC 回归：`cd yoyo-rust/verifier; cargo run -- test ddc`

## 自举链（地基）

| 组件 | 路径 | 说明 |
|------|------|------|
| Payload bins | `yoyo-js/src/platform/yoyo-compiler-payload/{code,data}.bin` + `payload.pin` | pin-locked |
| Place | `scripts/place-yoyo-compiler-c1.ps1` | C1 `try_place_bins`；**拒绝 Node auto-derive** |
| 产品 pin | `scripts/fixtures/yoyo_place_c1.pin` | `trust_root=c1_product_pin` |
| Bootstrap 文件 | `scripts/fixtures/yoyo_compiler.exe` | 仅 offline bit-identical；非信任身份 |
| Stub pin | `scripts/fixtures/yoyo_place_stub.exe` + `.pin` | TRUER4 几何；非 cold 产品 |

## 历史同伴（附录）

| 链 | 路径 | 现状 |
|----|------|------|
| JS | `yoyo-js/` | 门上 place/verify **ABSENT**；可作对照 |
| asm | `yoyo-asm/` | peer I/O 历史；非语言主入口 |

## Agent 选型铁律

1. **写应用 / 验语言** → Rust `yoyo link`  
2. **验自举** → C1 place 脚本 + lesshostx；不要重新引入 Node place  
3. **改 bins** → 必须更新 `payload.pin` 并跑相关 pin/place 门禁  
4. 二进制是否更新 → 查时间戳 / `yoyo hash` / `Get-FileHash`（见 AGENTS 经验）
