fp = r"F:\yoyO\yoyO-rust\verifier\src\platform.rs"
with open(fp, "r", encoding="utf-8") as f:
    src = f.read()

# Insert AVR helpers BEFORE avr_ldi
avr_helpers = '''// ===== AVR add/sub/or/inc/dec helpers =====
fn avr_add_rr(rd: u8, rr: u8) -> Vec<u8> {
    let enc: u16 = 0x0C00 | ((rd as u16) << 4) | (rr as u16) | 0x0E;
    enc.to_le_bytes().to_vec()
}
fn avr_sub_rr(rd: u8, rr: u8) -> Vec<u8> {
    let enc: u16 = 0x0C00 | ((rd as u16) << 4) | (rr as u16) | 0x06;
    enc.to_le_bytes().to_vec()
}
fn avr_or_rr(rd: u8, rr: u8) -> Vec<u8> {
    let enc: u16 = 0x0C00 | ((rd as u16) << 4) | (rr as u16) | 0x02;
    enc.to_le_bytes().to_vec()
}
fn avr_inc_r(r: u8) -> Vec<u8> {
    let enc: u16 = 0x9C00 | (r as u16);
    enc.to_le_bytes().to_vec()
}
fn avr_dec_r(r: u8) -> Vec<u8> {
    let enc: u16 = 0x9C00 | (r as u16) | 0x08;
    enc.to_le_bytes().to_vec()
}
fn avr_cp_r(rd: u8, rr: u8) -> Vec<u8> {
    let enc: u16 = 0x0C00 | ((rd as u16) << 4) | (rr as u16) | 0x0A;
    enc.to_le_bytes().to_vec()
}

'''

# Insert SPARC helpers BEFORE sparc_sethi
sparc_helpers = '''// ===== SPARC add/sub/or/sll/srl/mul helpers =====
fn sparc_add(rd: u32, rs1: u32, rs2: u32, imm: u32) -> [u8; 4] {
    if imm == 0 {
        (0x80000000u32 | (rd << 25) | (rs1 << 14) | (rs2 << 19)).to_be_bytes()
    } else {
        (0x80002000u32 | (rd << 25) | (rs1 << 14) | (imm & 0x1FFF)).to_be_bytes()
    }
}
fn sparc_sub(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    (0x80001000u32 | (rd << 25) | (rs1 << 14) | (rs2 << 19)).to_be_bytes()
}
fn sparc_or_rr(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    (0x80000000u32 | (rd << 25) | (rs1 << 14) | (rs2 << 19)).to_be_bytes()
}
fn sparc_mul(rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    (0x9C000000u32 | (rd << 25) | (rs1 << 14) | (rs2 << 19)).to_be_bytes()
}
fn sparc_sll(rd: u32, rs1: u32, _rs2: u32, amt: u32) -> [u8; 4] {
    (0x82000000u32 | (rd << 25) | (rs1 << 14) | (amt & 0x1F)).to_be_bytes()
}
fn sparc_srl(rd: u32, rs1: u32, _rs2: u32, amt: u32) -> [u8; 4] {
    (0x82000000u32 | (rd << 25) | (rs1 << 14) | (amt & 0x1F) | 0x00001000).to_be_bytes()
}
fn sparc_ldub(rd: u32, rs1: u32, imm: u32) -> [u8; 4] {
    (0xC0002100u32 | (rd << 25) | (rs1 << 14) | (imm & 0x1FFF)).to_be_bytes()
}
fn sparc_stb(rd: u32, rs1: u32, imm: u32) -> [u8; 4] {
    (0xC0202100u32 | (rd << 25) | (rs1 << 14) | (imm & 0x1FFF)).to_be_bytes()
}
fn sparc_subcc(_rd: u32, rs1: u32, rs2: u32) -> [u8; 4] {
    (0x80001000u32 | (rs1 << 14) | (rs2 << 19) | 0x00200000).to_be_bytes()
}
fn sparc_li_g1(imm: u32) -> Vec<u8> {
    let mut out = sparc_sethi(SPARC_G1, imm >> 10).to_vec();
    let lo = imm & 0x3FF;
    if lo != 0 { out.extend_from_slice(&sparc_or_imm(SPARC_G1, SPARC_G1, lo)); }
    out
}

'''

# Replace foreign_ calls in AVR block (lines 2652-2685)
old_avr = '''    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }'''

new_avr = '''    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = AVR_SRAM_BASE + slot * 2;
        let mut out = avr_lds(16, addr);
        out.extend(avr_ldi(18, imm as u8));
        out.extend(avr_add_rr(16, 18));
        out.extend(avr_sts(addr, 16));
        Ok(out)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        let addr = AVR_SRAM_BASE + slot * 2;
        let mut out = avr_lds(16, addr);
        out.extend(avr_ldi(18, imm as u8));
        out.extend(avr_sub_rr(16, 18));
        out.extend(avr_sts(addr, 16));
        Ok(out)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = AVR_SRAM_BASE + slot * 2;
        let mut out = avr_lds(16, addr);
        out.extend(avr_inc_r(16));
        out.extend(avr_sts(addr, 16));
        Ok(out)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        let addr = AVR_SRAM_BASE + slot * 2;
        let mut out = avr_lds(16, addr);
        out.extend(avr_dec_r(16));
        out.extend(avr_sts(addr, 16));
        Ok(out)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = AVR_SRAM_BASE + src * 2;
        let da = AVR_SRAM_BASE + dst * 2;
        let mut out = avr_lds(16, sa);
        out.extend(avr_lds(17, da));
        out.extend(avr_add_rr(17, 16));
        out.extend(avr_sts(da, 17));
        Ok(out)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = AVR_SRAM_BASE + src * 2;
        let da = AVR_SRAM_BASE + dst * 2;
        let mut out = avr_lds(16, sa);
        out.extend(avr_lds(17, da));
        out.extend(avr_or_rr(17, 16));
        out.extend(avr_sts(da, 17));
        Ok(out)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = AVR_SRAM_BASE + src * 2;
        let da = AVR_SRAM_BASE + dst * 2;
        let mut out = avr_lds(16, sa);
        out.extend(avr_lds(17, da));
        out.extend(avr_sub_rr(17, 16));
        out.extend(avr_sts(da, 17));
        Ok(out)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let sa = AVR_SRAM_BASE + src * 2;
        let da = AVR_SRAM_BASE + dst * 2;
        let [slo, shi] = sa.to_le_bytes();
        let [dlo, dhi] = da.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xE0, shi, 0x93, 0x24]);
        out.extend_from_slice(&[0xE0, slo, 0x91, 0x24, 0x91, 0x26]);
        out.extend_from_slice(&[0xE0, dhi, 0x93, 0x2C]);
        out.extend_from_slice(&[0xE0, dlo, 0x91, 0x2C, 0x91, 0x2C]);
        out.extend_from_slice(&[0x90, 0x22, dlo, dhi]);
        out.extend_from_slice(&[0x93, 0x22, 0xE0, 0x00, 0x94, 0x22]);
        out.extend_from_slice(&[0x94, 0x24, 0xE0, 0x00, 0x94, 0x2C]);
        out.extend_from_slice(&[0x94, 0x24, 0x93, 0x26]);
        out.extend_from_slice(&[0xE0, 0x00, 0x94, 0x22, 0x94, 0x24]);
        out.extend_from_slice(&[0x93, 0x22, 0x93, 0x24]);
        out.push(0x91);
        out.push(0xF0);
        out.push(0x02);
        Ok(out)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let aa = AVR_SRAM_BASE + a * 2;
        let ba = AVR_SRAM_BASE + b * 2;
        let mut out = avr_lds(16, aa);
        out.extend(avr_lds(17, ba));
        out.extend(avr_cp_r(16, 17));
        Ok(out)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let sa = AVR_SRAM_BASE + ss * 2;
        let da = AVR_SRAM_BASE + dd * 2;
        let [slo, shi] = sa.to_le_bytes();
        let [dlo, dhi] = da.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xE0, shi, 0x93, 0x24]);
        out.extend_from_slice(&[0xE0, slo, 0x91, 0x24, 0x91, 0x26]);
        out.extend_from_slice(&[0x91, 0xF0, oo as u8]);
        out.extend_from_slice(&[0x93, 0x26]);
        out.extend_from_slice(&[0xE0, shi, 0x93, 0x2C]);
        out.extend_from_slice(&[0xE0, dhi, 0x93, 0x24]);
        out.extend_from_slice(&[0xE0, dlo, 0x91, 0x24, 0x91, 0x2C]);
        out.extend_from_slice(&[0x93, 0x2C]);
        out.extend_from_slice(&[0x90, 0x26, dlo, dhi]);
        Ok(out)
    }
    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError { msg: "AVR memcpy_data not implemented".into() })
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = AVR_SRAM_BASE + (src + i) * 2;
            let da = AVR_SRAM_BASE + (dst + i) * 2;
            let [slo, shi] = sa.to_le_bytes();
            let [dlo, dhi] = da.to_le_bytes();
            out.extend_from_slice(&[0x90, 0x20, slo, shi, 0x93, 0x20, dlo, dhi]);
        }
        Ok(out)
    }'''

if old_avr not in src:
    print("AVR block NOT FOUND!")
else:
    src = src.replace(old_avr, new_avr)

# SPARC block
old_sparc = '''    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_add_imm(slot, imm)
    }
    fn emit_sub_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
        foreign_sub_imm(slot, imm)
    }
    fn emit_inc(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_inc(slot)
    }
    fn emit_dec(&mut self, slot: u16) -> IsaResult<Vec<u8>> {
        foreign_dec(slot)
    }
    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }
    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }
    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }
    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }
    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }
    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }
    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }
    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }'''

new_sparc = '''    fn emit_add_imm(&mut self, slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
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
    }'''

if old_sparc not in src:
    print("SPARC block NOT FOUND!")
else:
    src = src.replace(old_sparc, new_sparc)

# Insert AVR helpers
src = src.replace("fn avr_ldi(rd: u8, imm8: u8) -> Vec<u8> {", avr_helpers + "fn avr_ldi(rd: u8, imm8: u8) -> Vec<u8> {")

# Insert SPARC helpers
src = src.replace("fn sparc_sethi(rd: u32, imm22: u32) -> [u8; 4] {", sparc_helpers + "fn sparc_sethi(rd: u32, imm22: u32) -> [u8; 4] {")

with open(fp, "w", encoding="utf-8") as f:
    f.write(src)

with open(fp, "r", encoding="utf-8") as f:
    remaining = 0
    for i, line in enumerate(f):
        if "foreign_" in line and "fn foreign_" not in line:
            remaining += 1
            print(f"L{i+1}: {line.strip()}")
    print(f"Remaining foreign_ calls: {remaining}")