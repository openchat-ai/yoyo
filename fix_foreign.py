import sys
fp = r"F:\yoyo\yoyo-rust\verifier\src\platform.rs"
with open(fp, "r", encoding="utf-8") as f: src = f.read()

repls = []

# EVM (2 methods)
repls.append(("""    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }""", """    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError { msg: "EVM memcpy_data not implemented".into() })
    }"""))
repls.append(("""    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }""", """    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = src + i;
            let da = dst + i;
            out.push(0x61); out.push(((sa as u16).wrapping_mul(0x20) >> 8) as u8);
            out.push((sa as u16).wrapping_mul(0x20) as u8);
            out.push(0x51);
            out.push(0x61); out.push(((da as u16).wrapping_mul(0x20) >> 8) as u8);
            out.push((da as u16).wrapping_mul(0x20) as u8);
            out.push(0x52);
        }
        Ok(out)
    }"""))

# 8051 (2 methods)
repls.append(("""    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }""", """    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError { msg: "8051 memcpy_data not implemented".into() })
    }"""))
repls.append(("""    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }""", """    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        let d = E8051_STATE_BASE + dst as u8;
        let s = E8051_STATE_BASE + src as u8;
        for _i in 0..n {
            out.extend(e8051_mov_a_direct(s));
            out.extend(e8051_mov_direct_a(d));
        }
        Ok(out)
    }"""))

# M6502 (8 methods)
repls.append(("""    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }""", """    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = M6502_STATE_BASE + dst * 2;
        let sa = M6502_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xAD, slo, shi]); out.push(0x18);
        out.extend_from_slice(&[0xAD, dlo, dhi]);
        out.push(0x69); out.push(slo); out.extend_from_slice(&[0x8D, dlo, dhi]);
        out.extend_from_slice(&[0xAD, dlo.wrapping_add(1), dhi]);
        out.push(0x69); out.push(shi); out.extend_from_slice(&[0x8D, dlo.wrapping_add(1), dhi]);
        Ok(out)
    }"""))
repls.append(("""    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }""", """    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = M6502_STATE_BASE + dst * 2;
        let sa = M6502_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xAD, slo, shi]);
        out.push(0x29); out.push(0xFF); out.push(0x49); out.push(0x00);
        out.extend_from_slice(&[0xAD, dlo, dhi]);
        out.push(0x09); out.push(slo); out.extend_from_slice(&[0x8D, dlo, dhi]);
        out.extend_from_slice(&[0xAD, dlo.wrapping_add(1), dhi]);
        out.push(0x09); out.push(shi); out.extend_from_slice(&[0x8D, dlo.wrapping_add(1), dhi]);
        Ok(out)
    }"""))
repls.append(("""    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }""", """    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = M6502_STATE_BASE + dst * 2;
        let sa = M6502_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xAD, slo, shi]); out.push(0x38);
        out.extend_from_slice(&[0xAD, dlo, dhi]);
        out.push(0xE9); out.push(slo); out.extend_from_slice(&[0x8D, dlo, dhi]);
        out.extend_from_slice(&[0xAD, dlo.wrapping_add(1), dhi]);
        out.push(0xE9); out.push(shi); out.extend_from_slice(&[0x8D, dlo.wrapping_add(1), dhi]);
        Ok(out)
    }"""))
repls.append(("""    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }""", """    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = M6502_STATE_BASE + dst * 2;
        let sa = M6502_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xA2, slo, 0xA0, shi, 0xA9, 0x00, 0x85, dlo, 0x86, dhi]);
        out.extend_from_slice(&[0xAD, slo, shi]); out.push(0xA8);
        out.extend_from_slice(&[0xA5, dlo]); out.push(0x6D); out.push(slo); out.push(shi);
        out.extend_from_slice(&[0x85, dlo]);
        out.extend_from_slice(&[0xA5, dhi]); out.push(0x65); out.push(dlo);
        out.extend_from_slice(&[0x85, dhi]);
        out.extend_from_slice(&[0xA6, slo]); out.push(0xCA);
        out.extend_from_slice(&[0xF0, 0x04, 0xA4, shi]); out.push(0xC8); out.push(0xF0); out.push(0x28);
        out.extend_from_slice(&[0xA5, dlo]); out.push(0x69); out.push(slo);
        out.extend_from_slice(&[0x85, dlo]);
        out.extend_from_slice(&[0xA5, dhi]); out.push(0x65); out.push(dlo);
        out.extend_from_slice(&[0x85, dhi]);
        out.push(0xD0); out.push(0x08);
        out.push(0xC6); out.push(shi); out.push(0xD0); out.push(0x45);
        Ok(out)
    }"""))
repls.append(("""    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }""", """    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let aa = M6502_STATE_BASE + a * 2;
        let [blo, bhi] = (M6502_STATE_BASE + b * 2).to_le_bytes();
        let mut out = m6502_lda_addr(aa);
        out.extend_from_slice(&[0xC9, blo]);
        out.push(0xE0); out.push(bhi);
        Ok(out)
    }"""))
repls.append(("""    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }""", """    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let sa = M6502_STATE_BASE + ss * 2;
        let da = M6502_STATE_BASE + dd * 2;
        let [slo, shi] = sa.to_le_bytes();
        let [dlo, dhi] = da.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0xAD, slo, shi]);
        out.push(0x69); out.push(oo as u8);
        out.extend_from_slice(&[0x8D, dlo, dhi]);
        Ok(out)
    }"""))
repls.append(("""    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }""", """    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError { msg: "M6502 memcpy_data not implemented".into() })
    }"""))
repls.append(("""    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }""", """    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = M6502_STATE_BASE + (src + i) * 2;
            let da = M6502_STATE_BASE + (dst + i) * 2;
            let [slo, shi] = sa.to_le_bytes();
            let [dlo, dhi] = da.to_le_bytes();
            out.extend_from_slice(&[0xAD, slo, shi, 0x8D, dlo, dhi]);
        }
        Ok(out)
    }"""))

# Z80 (8 methods)
repls.append(("""    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_addv(dst, src)
    }""", """    fn emit_addv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = Z80_STATE_BASE + dst * 2;
        let sa = Z80_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0x01, slo, shi]);
        out.extend_from_slice(&[0x2A, dlo, dhi]);
        out.push(0x09);
        out.extend_from_slice(&[0x22, dlo, dhi]);
        Ok(out)
    }"""))
repls.append(("""    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_orv(dst, src)
    }""", """    fn emit_orv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = Z80_STATE_BASE + dst * 2;
        let sa = Z80_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0x3A, slo, shi, 0xB6, 0xFF]);
        out.extend_from_slice(&[0x2A, dlo, dhi, 0xB0]);
        out.extend_from_slice(&[0x77, 0x22, dlo, dhi]);
        out.extend_from_slice(&[0x3A, slo, shi, 0x7F, 0xB4]);
        out.extend_from_slice(&[0x77, 0x22, dlo, dhi]);
        Ok(out)
    }"""))
repls.append(("""    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_subv(dst, src)
    }""", """    fn emit_subv(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = Z80_STATE_BASE + dst * 2;
        let sa = Z80_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0x01, slo, shi, 0x3E, slo, 0xD6, slo]);
        out.extend_from_slice(&[0x6F, 0x3A, dlo, dhi, 0x7D, 0xD6, slo]);
        out.extend_from_slice(&[0x6F, 0x22, dlo, dhi]);
        out.extend_from_slice(&[0x3A, slo, shi, 0x3E, shi, 0x7C, 0x8F, 0xD6, shi]);
        out.extend_from_slice(&[0x67, 0x22, dlo, dhi]);
        Ok(out)
    }"""))
repls.append(("""    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        foreign_imul(dst, src)
    }""", """    fn emit_imul(&mut self, dst: u16, src: u16) -> IsaResult<Vec<u8>> {
        let da = Z80_STATE_BASE + dst * 2;
        let sa = Z80_STATE_BASE + src * 2;
        let [dlo, dhi] = da.to_le_bytes();
        let [slo, shi] = sa.to_le_bytes();
        let mut out = Vec::new();
        out.extend_from_slice(&[0x2A, dlo, dhi]);
        out.extend_from_slice(&[0x3E, dlo, 0x8F, 0x3E, dhi, 0x87]);
        out.extend_from_slice(&[0x22, dlo, dhi]);
        out.extend_from_slice(&[0x01, slo, shi]);
        out.extend_from_slice(&[0x2E, slo, 0x26, shi, 0x3E, 0x00]);
        out.extend_from_slice(&[0x32, dlo, dhi, 0x32, dlo.wrapping_add(1), dhi]);
        out.extend_from_slice(&[0x2A, dlo, dhi, 0x89]);
        out.extend_from_slice(&[0x22, dlo, dhi]);
        out.extend_from_slice(&[0xCB, 0x6F]);
        out.extend_from_slice(&[0x28, 0x05, 0xCB, 0xCF]);
        out.extend_from_slice(&[0x28, 0x2B]);
        out.extend_from_slice(&[0x2A, dlo, dhi, 0x89]);
        out.extend_from_slice(&[0x22, dlo, dhi, 0x05]);
        out.extend_from_slice(&[0xCB, 0x6F]);
        out.extend_from_slice(&[0x20, 0x05]);
        out.extend_from_slice(&[0xCB, 0xCF, 0x20, 0x2B]);
        out.extend_from_slice(&[0xCB, 0x5F]);
        out.extend_from_slice(&[0x28, 0x2B]);
        out.push(0x23);
        out.push(0xCA); out.push(0x2E);
        Ok(out)
    }"""))
repls.append(("""    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        foreign_cmp(a, b)
    }""", """    fn emit_cmp(&mut self, a: u16, b: u16) -> IsaResult<Vec<u8>> {
        let aa = Z80_STATE_BASE + a * 2;
        let ba = Z80_STATE_BASE + b * 2;
        let [alo, ahi] = aa.to_le_bytes();
        let [blo, bhi] = ba.to_le_bytes();
        let mut out = vec![0x2A, alo, ahi];
        out.push(0xB8); out.push(blo);
        out.push(0xB9); out.push(bhi);
        Ok(out)
    }"""))
repls.append(("""    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        foreign_ldb(dd, ss, oo)
    }""", """    fn emit_ldb(&mut self, dd: u16, ss: u16, oo: u16) -> IsaResult<Vec<u8>> {
        let sa = Z80_STATE_BASE + ss * 2;
        let da = Z80_STATE_BASE + dd * 2;
        let [slo, shi] = sa.to_le_bytes();
        let [dlo, dhi] = da.to_le_bytes();
        let mut out = vec![0x2A, slo, shi];
        out.push(0x86); out.push(oo as u8);
        out.extend_from_slice(&[0x77, 0x22, dlo, dhi]);
        Ok(out)
    }"""))
repls.append(("""    fn emit_memcpy_data(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_data(dst, src, n)
    }""", """    fn emit_memcpy_data(&mut self, _dst: u16, _src: u16, _n: u16) -> IsaResult<Vec<u8>> {
        Err(IsaError::PlatformError { msg: "Z80 memcpy_data not implemented".into() })
    }"""))
repls.append(("""    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        foreign_memcpy_state(dst, src, n)
    }""", """    fn emit_memcpy_state(&mut self, dst: u16, src: u16, n: u16) -> IsaResult<Vec<u8>> {
        let mut out = Vec::new();
        for i in 0..n {
            let sa = Z80_STATE_BASE + (src + i) * 2;
            let da = Z80_STATE_BASE + (dst + i) * 2;
            let [slo, shi] = sa.to_le_bytes();
            let [dlo, dhi] = da.to_le_bytes();
            out.extend_from_slice(&[0x2A, slo, shi, 0x22, dlo, dhi]);
        }
        Ok(out)
    }"""))

with open(fp, "w", encoding="utf-8") as f: f.write(src)

# Check
lines = src.split('\n')
foreign_calls = []
for i, line in enumerate(lines):
    if 'foreign_' in line and 'fn foreign_' not in line:
        foreign_calls.append((i+1, line.strip()))

print(f"Remaining foreign_ calls in body: {len(foreign_calls)}")
for ln, txt in foreign_calls[:20]:
    print(f"  L{ln}: {txt}")
