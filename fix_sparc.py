fp = r"F:\yoyO\yoyO-rust\verifier\src\platform.rs"
with open(fp, "r", encoding="utf-8") as f:
    src = f.read()

# The SPARC block currently has AVR code. Find it between "impl PlatformBackend for SparcPlatform" and "impl PlatformBackend for Riscv32Platform"
start = src.index("impl PlatformBackend for SparcPlatform")
end = src.index("impl PlatformBackend for Riscv32Platform")

# Read the whole SparcPlatform block
sparc_block = src[start:end]
print("Current SPARC block length:", len(sparc_block))

# Find the first "fn emit_add_imm" and the "fn emit_alloc" to isolate the region to replace
ai = sparc_block.index("    fn emit_add_imm")
al = sparc_block.index("    fn emit_alloc")

new_sparc_methods = '''    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = 0x20000u32 + slot as u32 * 4;
        let hi = addr >> 10;
        let lo = addr & 0x3FF;
        let mut out = sparc_sethi(1, hi).to_vec();
        if lo != 0 { out.extend_from_slice(&sparc_or_imm(1, 1, lo)); }
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        if imm <= 0xFFF {
            out.extend_from_slice(&sparc_or_imm(2, 2, imm as u32));
        } else if imm <= 0x3FFFFF {
            out.extend_from_slice(&sparc_sethi(2, imm as u32 >> 10));
            let ll = imm as u32 & 0x3FF;
            if ll != 0 { out.extend_from_slice(&sparc_or_imm(2, 2, ll)); }
        } else {
            out.extend_from_slice(&sparc_li_g2(imm as u32));
        }
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = 0x20000u32 + slot as u32 * 4;
        let hi = addr >> 10;
        let lo = addr & 0x3FF;
        let mut out = sparc_sethi(1, hi).to_vec();
        if lo != 0 { out.extend_from_slice(&sparc_or_imm(1, 1, lo)); }
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        let neg = (-imm as i32) as u32;
        if imm <= 0xFFF {
            out.extend_from_slice(&sparc_add(2, 2, 0, neg));
        } else {
            out.extend_from_slice(&sparc_li_g2(neg));
            out.extend_from_slice(&sparc_add(2, 2, 2));
        }
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = 0x20000u32 + slot as u32 * 4;
        let hi = addr >> 10;
        let lo = addr & 0x3FF;
        let mut out = sparc_sethi(1, hi).to_vec();
        if lo != 0 { out.extend_from_slice(&sparc_or_imm(1, 1, lo)); }
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_add(2, 0, 0, 1));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = 0x20000u32 + slot as u32 * 4;
        let hi = addr >> 10;
        let lo = addr & 0x3FF;
        let mut out = sparc_sethi(1, hi).to_vec();
        if lo != 0 { out.extend_from_slice(&sparc_or_imm(1, 1, lo)); }
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_add(2, 2, 0, 0xFFFFFF));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x20000u32 + src as u32 * 4;
        let da = 0x20000u32 + dst as u32 * 4;
        let mut out = sparc_li_g1(sa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_li_g1(da));
        out.extend_from_slice(&sparc_ld(3, 1, 0));
        out.extend_from_slice(&sparc_add(2, 2, 3));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x20000u32 + src as u32 * 4;
        let da = 0x20000u32 + dst as u32 * 4;
        let mut out = sparc_li_g1(sa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_li_g1(da));
        out.extend_from_slice(&sparc_ld(3, 1, 0));
        out.extend_from_slice(&sparc_or_rr(2, 2, 3));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x20000u32 + src as u32 * 4;
        let da = 0x20000u32 + dst as u32 * 4;
        let mut out = sparc_li_g1(sa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_li_g1(da));
        out.extend_from_slice(&sparc_ld(3, 1, 0));
        out.extend_from_slice(&sparc_sub(2, 3, 2));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x20000u32 + src as u32 * 4;
        let da = 0x20000u32 + dst as u32 * 4;
        let mut out = sparc_li_g1(sa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_li_g1(da));
        out.extend_from_slice(&sparc_ld(3, 1, 0));
        out.extend_from_slice(&sparc_mul(2, 2, 3));
        out.extend_from_slice(&sparc_st(2, 1, 0));
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let aa = 0x20000u32 + a as u32 * 4;
        let ba = 0x20000u32 + b as u32 * 4;
        let mut out = sparc_li_g1(aa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        out.extend_from_slice(&sparc_li_g1(ba));
        out.extend_from_slice(&sparc_ld(3, 1, 0));
        out.extend_from_slice(&sparc_subcc(0, 2, 3));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let sa = 0x20000u32 + ss as u32 * 4;
        let da = 0x20000u32 + dd as u32 * 4;
        let mut out = sparc_li_g1(sa);
        out.extend_from_slice(&sparc_ld(2, 1, 0));
        if oo != 0 { out.extend_from_slice(&sparc_add(2, 2, 0, oo as u32)); }
        out.extend_from_slice(&sparc_ldub(3, 2, 0));
        out.extend_from_slice(&sparc_li_g1(da));
        out.extend_from_slice(&sparc_stb(3, 1, 0));
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError { msg: "SPARC memcpy_data not implemented".into() })
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = 0x20000u32 + (src + i) as u32 * 4;
            let da = 0x20000u32 + (dst + i) as u32 * 4;
            out.extend_from_slice(&sparc_li_g1(sa));
            out.extend_from_slice(&sparc_ld(2, 1, 0));
            out.extend_from_slice(&sparc_li_g1(da));
            out.extend_from_slice(&sparc_st(2, 1, 0));
        }
        Ok(out)
    }
'''

# Replace the region between emit_add_imm and emit_alloc in the SPARC block
sparc_block_fixed = sparc_block[:ai] + new_sparc_methods + sparc_block[al:]
src_fixed = src[:start] + sparc_block_fixed + src[end:]

with open(fp, "w", encoding="utf-8") as f:
    f.write(src_fixed)

# Verify
with open(fp, "r", encoding="utf-8") as f:
    text = f.read()

remaining = 0
for i, line in enumerate(text.split("\n")):
    if "foreign_" in line and "fn foreign_" not in line:
        remaining += 1
        print(f"L{i+1}: {line.strip()}")

# Also check no AVR code leaked into SPARC
sparc_block2 = text[text.index("impl PlatformBackend for SparcPlatform"):text.index("impl PlatformBackend for Riscv32Platform")]
if "AVR_SRAM_BASE" in sparc_block2:
    print("ERROR: AVR code still in SPARC block!")
else:
    print("SPARC block clean (no AVR references)")

print(f"Remaining foreign_ calls: {remaining}")