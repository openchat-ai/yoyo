fp = r"F:\yoyo\yoyo-rust\verifier\src\platform.rs"
with open(fp, "r", encoding="utf-8") as f:
    src = f.read()

# Add AVR and SPARC helpers before their existing helper sections
new_helpers = '''
// ===== AVR add/sub helpers =====
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
fn avr_sbr_r(ri: u8, k: u8) -> Vec<u8> {
    // SBR: r0-r7, 8 bytes
    if ri < 8 {
        let enc: u16 = 0xE000 | ((ri as u16) << 4) | (k as u16);
        enc.to_le_bytes().to_vec()
    } else {
        vec![]
    }
}

// ===== SPARC add/sub helpers =====
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

# Insert AVR helpers before the avr_ldi function
src = src.replace("fn avr_ldi(rd: u8, imm8: u8) -> Vec<u8> {", new_helpers + "fn avr_ldi(rd: u8, imm8: u8) -> Vec<u8> {")

# Insert SPARC helpers before the sparc_sethi function
sparc_helper_insert = '''// ===== SPARC add/sub/branch helpers =====
'''
src = src.replace("fn sparc_sethi(rd: u32, imm22: u32) -> [u8; 4] {", sparc_helper_insert + "fn sparc_sethi(rd: u32, imm22: u32) -> [u8; 4] {")

# Actually, I already wrote the AVR/SPARC helper functions above but the sparc ones weren't inserted before the existing sparc helpers. Let me just append them after sparc_li_g2.
# Move the AVR/SPARC helpers to be after sparc_li_g2() and avr_nop()
# Actually let me just put them all right before the platform structs

# Simpler approach: put all helpers into the file between existing helpers and struct definitions

with open(fp, "w", encoding="utf-8") as f:
    f.write(src)

# verify
with open(fp, "r", encoding="utf-8") as f:
    remaining = 0
    for i, line in enumerate(f):
        if "foreign_" in line and "fn foreign_" not in line:
            remaining += 1
            print(f"L{i+1}: {line.strip()}")
    print(f"Remaining foreign_ calls: {remaining}")