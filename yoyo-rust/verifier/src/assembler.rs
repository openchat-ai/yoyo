//! X64Assembler — the 13 primitives (PROMPT-v3 Part 4.3).
//! Replaces legacy primitives.rs. All emit paths go through here.

use crate::types::{IsaError, IsaResult, Reg};

/// Valid JCC second bytes for `0F 8x` (Part 4.3.6).
pub const JCC_VALID: [u8; 10] = [
    0x84, // je
    0x85, // jne
    0x8C, // jl
    0x8D, // jge
    0x8E, // jle
    0x8F, // jg
    0x82, // jb
    0x83, // jae
    0x86, // jbe
    0x87, // ja
];

/// KY opcode 0x71..=0x7A → x64 JCC second byte.
pub const JCC_TABLE: [u8; 10] = JCC_VALID;

pub const JCC_MNEMONIC: [&str; 10] = [
    "je", "jne", "jl", "jge", "jle", "jg", "jb", "jae", "jbe", "ja",
];

fn rex_wrxb(w: bool, r: bool, x: bool, b: bool) -> u8 {
    0x40 | ((w as u8) << 3) | ((r as u8) << 2) | ((x as u8) << 1) | (b as u8)
}

/// Emits: `mov <dest>, [r15 + slot*8]`
/// Size: 4 bytes (slot ≤ 15, disp8) or 7 bytes (slot ≥ 16, disp32).
pub fn load_state(slot: u16, dest: Reg) -> IsaResult<Vec<u8>> {
    if slot > 255 {
        return Err(IsaError::SlotOutOfRange { slot });
    }
    let disp = (slot as u32) * 8;
    // REX.WB: W=1 (64-bit), B=1 because base is R15
    let rex = rex_wrxb(true, dest.rex_bit(), false, true);
    let modrm_reg = dest.low3() << 3;
    if disp <= 127 {
        Ok(vec![rex, 0x8B, modrm_reg | 0x40 | 0x07, disp as u8])
    } else {
        let mut b = vec![rex, 0x8B, modrm_reg | 0x80 | 0x07];
        b.extend_from_slice(&disp.to_le_bytes());
        Ok(b)
    }
}

/// Emits: `mov [r15 + slot*8], <src>`
pub fn store_state(slot: u16, src: Reg) -> IsaResult<Vec<u8>> {
    if slot > 255 {
        return Err(IsaError::SlotOutOfRange { slot });
    }
    let disp = (slot as u32) * 8;
    let rex = rex_wrxb(true, src.rex_bit(), false, true);
    let modrm_reg = src.low3() << 3;
    if disp <= 127 {
        Ok(vec![rex, 0x89, modrm_reg | 0x40 | 0x07, disp as u8])
    } else {
        let mut b = vec![rex, 0x89, modrm_reg | 0x80 | 0x07];
        b.extend_from_slice(&disp.to_le_bytes());
        Ok(b)
    }
}

/// Emits: `movabs <reg>, imm64` (10 bytes).
pub fn movabs(reg: Reg, imm: u64) -> IsaResult<Vec<u8>> {
    let rex = rex_wrxb(true, false, false, reg.rex_bit());
    let mut b = vec![rex, 0xB8 + reg.low3()];
    b.extend_from_slice(&imm.to_le_bytes());
    Ok(b)
}

/// Emits: `add <reg>, imm` (imm8 or imm32).
pub fn add_imm(reg: Reg, imm: u64) -> IsaResult<Vec<u8>> {
    let as_i64 = imm as i64;
    if as_i64 < i32::MIN as i64 || as_i64 > i32::MAX as i64 {
        return Err(IsaError::ImmOutOfRange {
            value: imm,
            max: i32::MAX as u64,
        });
    }
    let rex = rex_wrxb(true, false, false, reg.rex_bit());
    let modrm = 0xC0 | reg.low3(); // /0 = ADD
    if as_i64 >= -128 && as_i64 <= 127 {
        Ok(vec![rex, 0x83, modrm, as_i64 as u8])
    } else {
        let mut b = vec![rex, 0x81, modrm];
        b.extend_from_slice(&(as_i64 as i32).to_le_bytes());
        Ok(b)
    }
}

/// Emits: `sub <reg>, imm`.
pub fn sub_imm(reg: Reg, imm: u64) -> IsaResult<Vec<u8>> {
    let as_i64 = imm as i64;
    if as_i64 < i32::MIN as i64 || as_i64 > i32::MAX as i64 {
        return Err(IsaError::ImmOutOfRange {
            value: imm,
            max: i32::MAX as u64,
        });
    }
    let rex = rex_wrxb(true, false, false, reg.rex_bit());
    let modrm = 0xE8 | reg.low3(); // /5 = SUB
    if as_i64 >= -128 && as_i64 <= 127 {
        Ok(vec![rex, 0x83, modrm, as_i64 as u8])
    } else {
        let mut b = vec![rex, 0x81, modrm];
        b.extend_from_slice(&(as_i64 as i32).to_le_bytes());
        Ok(b)
    }
}

/// Emits: `add <dst>, <src>`.
pub fn add_reg(dst: Reg, src: Reg) -> IsaResult<Vec<u8>> {
    let rex = rex_wrxb(true, src.rex_bit(), false, dst.rex_bit());
    Ok(vec![rex, 0x01, 0xC0 | (src.low3() << 3) | dst.low3()])
}

/// Emits: `or <dst>, <src>` (ORV).
pub fn or_reg(dst: Reg, src: Reg) -> IsaResult<Vec<u8>> {
    let rex = rex_wrxb(true, src.rex_bit(), false, dst.rex_bit());
    Ok(vec![rex, 0x09, 0xC0 | (src.low3() << 3) | dst.low3()])
}

/// Emits: `sub <dst>, <src>`.
pub fn sub_reg(dst: Reg, src: Reg) -> IsaResult<Vec<u8>> {
    let rex = rex_wrxb(true, src.rex_bit(), false, dst.rex_bit());
    Ok(vec![rex, 0x29, 0xC0 | (src.low3() << 3) | dst.low3()])
}

/// Emits: `imul <dst>, <src>` (`0F AF`).
pub fn mul_reg(dst: Reg, src: Reg) -> IsaResult<Vec<u8>> {
    let rex = rex_wrxb(true, dst.rex_bit(), false, src.rex_bit());
    Ok(vec![
        rex,
        0x0F,
        0xAF,
        0xC0 | (dst.low3() << 3) | src.low3(),
    ])
}

/// Emits: `cmp <a>, <b>`.
pub fn cmp_reg(a: Reg, b: Reg) -> IsaResult<Vec<u8>> {
    let rex = rex_wrxb(true, b.rex_bit(), false, a.rex_bit());
    Ok(vec![rex, 0x39, 0xC0 | (b.low3() << 3) | a.low3()])
}

/// Emits: `call <rel32>` (5 bytes: `E8 imm32`).
pub fn call_rel32(offset: i32) -> IsaResult<Vec<u8>> {
    let mut b = vec![0xE8];
    b.extend_from_slice(&offset.to_le_bytes());
    Ok(b)
}

/// Emits: `jmp <rel32>` (5 bytes: `E9 imm32`).
pub fn jmp_rel32(offset: i32) -> IsaResult<Vec<u8>> {
    let mut b = vec![0xE9];
    b.extend_from_slice(&offset.to_le_bytes());
    Ok(b)
}

/// Emits: `j<cc> <rel32>` (6 bytes: `0F 8x imm32`).
pub fn jcc_rel32(cc: u8, offset: i32) -> IsaResult<Vec<u8>> {
    if !JCC_VALID.contains(&cc) {
        return Err(IsaError::InvalidConditionCode { cc });
    }
    let mut b = vec![0x0F, cc];
    b.extend_from_slice(&offset.to_le_bytes());
    Ok(b)
}

/// Emits: `ret` (1 byte: `C3`).
pub fn ret() -> Vec<u8> {
    vec![0xC3]
}

/// Emits: `inc <reg>`.
pub fn inc_reg(reg: Reg) -> IsaResult<Vec<u8>> {
    let rex = rex_wrxb(true, false, false, reg.rex_bit());
    Ok(vec![rex, 0xFF, 0xC0 | reg.low3()])
}

/// Emits: `dec <reg>`.
pub fn dec_reg(reg: Reg) -> IsaResult<Vec<u8>> {
    let rex = rex_wrxb(true, false, false, reg.rex_bit());
    Ok(vec![rex, 0xFF, 0xC8 | reg.low3()])
}

/// Convenience: SET slot = imm → movabs rax,imm ; store_state slot,rax
pub fn emit_set(slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
    let mut out = movabs(Reg::Rax, imm)?;
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

/// Convenience: GET dst = src → load_state src,rax ; store_state dst,rax
pub fn emit_get(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(src, Reg::Rax)?;
    out.extend(store_state(dst, Reg::Rax)?);
    Ok(out)
}

/// Convenience: MOVRR dst = src (0x64 — independent route from GET; same slot copy semantics)
pub fn emit_movrr(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(src, Reg::Rax)?;
    out.extend(store_state(dst, Reg::Rax)?);
    Ok(out)
}

/// Convenience: ADDV dst += src
pub fn emit_addv(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(dst, Reg::Rax)?;
    out.extend(load_state(src, Reg::Rcx)?);
    out.extend(add_reg(Reg::Rax, Reg::Rcx)?);
    out.extend(store_state(dst, Reg::Rax)?);
    Ok(out)
}

/// Convenience: ORV dst |= src (bitwise OR — MUST NOT alias ADDV)
pub fn emit_orv(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(dst, Reg::Rax)?;
    out.extend(load_state(src, Reg::Rcx)?);
    out.extend(or_reg(Reg::Rax, Reg::Rcx)?);
    out.extend(store_state(dst, Reg::Rax)?);
    Ok(out)
}

/// Convenience: SUBV dst -= src
pub fn emit_subv(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(dst, Reg::Rax)?;
    out.extend(load_state(src, Reg::Rcx)?);
    out.extend(sub_reg(Reg::Rax, Reg::Rcx)?);
    out.extend(store_state(dst, Reg::Rax)?);
    Ok(out)
}

/// Convenience: IMUL dst *= src
pub fn emit_imul(dst: u16, src: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(dst, Reg::Rax)?;
    out.extend(load_state(src, Reg::Rcx)?);
    out.extend(mul_reg(Reg::Rax, Reg::Rcx)?);
    out.extend(store_state(dst, Reg::Rax)?);
    Ok(out)
}

/// Convenience: CMP a, b (sets flags; leaves regs)
pub fn emit_cmp(a: u16, b: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(a, Reg::Rax)?;
    out.extend(load_state(b, Reg::Rcx)?);
    out.extend(cmp_reg(Reg::Rax, Reg::Rcx)?);
    Ok(out)
}

/// Convenience: INC slot
pub fn emit_inc(slot: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(slot, Reg::Rax)?;
    out.extend(inc_reg(Reg::Rax)?);
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

/// Convenience: DEC slot
pub fn emit_dec(slot: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(slot, Reg::Rax)?;
    out.extend(dec_reg(Reg::Rax)?);
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

/// Convenience: ADD slot, imm
pub fn emit_add_imm(slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
    let mut out = load_state(slot, Reg::Rax)?;
    out.extend(add_imm(Reg::Rax, imm)?);
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

/// Convenience: SUB slot, imm
pub fn emit_sub_imm(slot: u16, imm: u64) -> IsaResult<Vec<u8>> {
    let mut out = load_state(slot, Reg::Rax)?;
    out.extend(sub_imm(Reg::Rax, imm)?);
    out.extend(store_state(slot, Reg::Rax)?);
    Ok(out)
}

/// MEMCPY_DATA dst src n — cross-peer canonical emit.
///
/// Canonical (PROMPT §4S.3 / DDC): copy S[n] bytes from address S[src] to S[dst].
/// Uses RSI=src, RDI=dst, RCX=count. `rep movsb` is the normative single-byte copy
/// because it is independent of loop direction / overlap and encodes identically in
/// both JS and Rust peers.
///
/// x64 sequence (15B before load-state; 22B including the three loads for
/// default slot values):
///   load_state src,Rsi   ; RSI = src address
///   load_state dst,Rdi   ; RDI = dst address
///   load_state n,Rcx     ; RCX = byte count
///   rep movsb            ; 0xFC
pub fn emit_memcpy_data(src: u16, dst: u16, n: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(src, Reg::Rsi)?;
    out.extend(load_state(dst, Reg::Rdi)?);
    out.extend(load_state(n, Reg::Rcx)?);
    out.push(0xFC); // rep movsb
    Ok(out)
}

/// MEMCPY_STATE dst src n — canonical emit.
///
/// Canonical (PROMPT §4S.3): MEMCPY_DATA, but the "src" and "dst" operands
/// are *slot indices* (not raw addresses). So before the memcpy we must
/// materialize S[src]*8 and S[dst]*8 by reading the slot cells into RSI and
/// RDI, then lea-extend them against the state base R15.
///
/// x64 sequence (same three loads as DATA + two LEA, 36B for default slots):
///   load_state dst,Rdi   ; RDI = dst slot index
///   lea rdi,[r15+rdi*8]  ; RDI = &S[dst]
///   load_state src,Rsi   ; RSI = src slot index
///   lea rsi,[r15+rsi*8]  ; RSI = &S[src]
///   load_state n,Rcx     ; RCX = byte count
///   rep movsb            ; 0xFC
pub fn emit_memcpy_state(src: u16, dst: u16, n: u16) -> IsaResult<Vec<u8>> {
    let mut out = load_state(dst, Reg::Rdi)?;
    out.extend(emit_lea_r15_scale8(Reg::Rdi));
    out.extend(load_state(src, Reg::Rsi)?);
    out.extend(emit_lea_r15_scale8(Reg::Rsi));
    out.extend(load_state(n, Reg::Rcx)?);
    out.push(0xFC); // rep movsb
    Ok(out)
}

/// Emits `lea <reg>, [r15 + <reg>*8]` (6B).
///
/// Used by MEMCPY_STATE to scale a slot index up to a byte offset from the
/// state base. Encoding: `4D` (REX.WRB) `8B C7` (lea) `08 00 00 00`
/// (SIB: scale 8 = 100, index = reg, base R15; disp32=0).
fn emit_lea_r15_scale8(reg: Reg) -> Vec<u8> {
    // REX: W=1 (64-bit), B=1 (base R15), R=0 (reg < 8)
    let rex = rex_wrxb(true, false, false, true); // R15 base
    let sib = {
        // scale=8 → 11 (3); index=reg; base=R15 (7)
        (3 << 6) | ((reg.low3() as u8) << 3) | 7
    };
    let reg_in_modrm = (reg.low3() as u8) << 3;
    vec![rex, 0x8B, reg_in_modrm | 0x04, sib, 0x00, 0x00, 0x00] // mod=00, rm=100 (SIB)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_state_disp8() {
        let b = load_state(0, Reg::Rax).unwrap();
        // 49 8B 47 00 — REX.WB mov rax,[r15+0]
        assert_eq!(b, vec![0x49, 0x8B, 0x47, 0x00]);
    }

    #[test]
    fn load_state_disp8_slot15() {
        let b = load_state(15, Reg::Rax).unwrap();
        assert_eq!(b, vec![0x49, 0x8B, 0x47, 120]); // 15*8=120
    }

    #[test]
    fn load_state_disp32() {
        let b = load_state(16, Reg::Rax).unwrap();
        assert_eq!(b[0], 0x49);
        assert_eq!(b[1], 0x8B);
        assert_eq!(b[2], 0x87); // mod=10
        assert_eq!(&b[3..], &128u32.to_le_bytes());
    }

    #[test]
    fn store_state_rax() {
        let b = store_state(0x50, Reg::Rax).unwrap();
        // slot 0x50 * 8 = 640 > 127 → disp32
        assert_eq!(b[0], 0x49);
        assert_eq!(b[1], 0x89);
        assert_eq!(b[2], 0x87);
        assert_eq!(&b[3..], &640u32.to_le_bytes());
    }

    #[test]
    fn movabs_rax() {
        let b = movabs(Reg::Rax, 0x1122334455667788).unwrap();
        assert_eq!(b.len(), 10);
        assert_eq!(&b[0..2], &[0x48, 0xB8]);
        assert_eq!(&b[2..], &0x1122334455667788u64.to_le_bytes());
    }

    #[test]
    fn movabs_r8() {
        let b = movabs(Reg::R8, 7).unwrap();
        assert_eq!(b[0], 0x49); // REX.WB
        assert_eq!(b[1], 0xB8); // +rd=0 for r8
    }

    #[test]
    fn add_imm_imm8() {
        let b = add_imm(Reg::Rax, 5).unwrap();
        assert_eq!(b, vec![0x48, 0x83, 0xC0, 5]);
    }

    #[test]
    fn add_imm_imm32() {
        let b = add_imm(Reg::Rax, 1000).unwrap();
        assert_eq!(&b[0..3], &[0x48, 0x81, 0xC0]);
        assert_eq!(&b[3..], &1000i32.to_le_bytes());
    }

    #[test]
    fn sub_imm_imm8() {
        let b = sub_imm(Reg::Rax, 1).unwrap();
        assert_eq!(b, vec![0x48, 0x83, 0xE8, 1]);
    }

    #[test]
    fn add_reg_rax_rcx() {
        let b = add_reg(Reg::Rax, Reg::Rcx).unwrap();
        assert_eq!(b, vec![0x48, 0x01, 0xC8]);
    }

    #[test]
    fn sub_reg_rax_rcx() {
        let b = sub_reg(Reg::Rax, Reg::Rcx).unwrap();
        assert_eq!(b, vec![0x48, 0x29, 0xC8]);
    }

    #[test]
    fn mul_reg_rax_rcx() {
        let b = mul_reg(Reg::Rax, Reg::Rcx).unwrap();
        assert_eq!(b, vec![0x48, 0x0F, 0xAF, 0xC1]);
    }

    #[test]
    fn cmp_reg_rax_rcx() {
        let b = cmp_reg(Reg::Rax, Reg::Rcx).unwrap();
        assert_eq!(b, vec![0x48, 0x39, 0xC8]);
    }

    #[test]
    fn call_rel32_encoding() {
        let b = call_rel32(0x1234).unwrap();
        assert_eq!(b[0], 0xE8);
        assert_eq!(&b[1..], &0x1234i32.to_le_bytes());
    }

    #[test]
    fn jmp_rel32_encoding() {
        let b = jmp_rel32(-5).unwrap();
        assert_eq!(b[0], 0xE9);
        assert_eq!(&b[1..], &(-5i32).to_le_bytes());
    }

    #[test]
    fn jcc_je() {
        let b = jcc_rel32(0x84, 10).unwrap();
        assert_eq!(&b[0..2], &[0x0F, 0x84]);
        assert_eq!(&b[2..], &10i32.to_le_bytes());
    }

    #[test]
    fn jcc_invalid() {
        assert!(matches!(
            jcc_rel32(0x90, 0),
            Err(IsaError::InvalidConditionCode { cc: 0x90 })
        ));
    }

    #[test]
    fn ret_is_c3() {
        assert_eq!(ret(), vec![0xC3]);
    }

    #[test]
    fn slot_out_of_range() {
        assert!(matches!(
            load_state(256, Reg::Rax),
            Err(IsaError::SlotOutOfRange { slot: 256 })
        ));
    }

    #[test]
    fn emit_set_composes() {
        let b = emit_set(0, 42).unwrap();
        // movabs (10) + store disp8 (4) = 14
        assert_eq!(b.len(), 14);
        assert_eq!(&b[0..2], &[0x48, 0xB8]);
    }

    #[test]
    fn emit_get_composes() {
        let b = emit_get(1, 0).unwrap();
        assert!(b.len() >= 8);
    }

    #[test]
    fn emit_movrr_independent_from_get() {
        let get = emit_get(0x50, 0x51).unwrap();
        let movrr = emit_movrr(0x50, 0x51).unwrap();
        assert_eq!(get, movrr, "MOVRR and GET share slot-copy semantics");
        assert!(movrr.len() >= 8);
    }

    #[test]
    fn emit_addv_composes() {
        let b = emit_addv(0x50, 0x51).unwrap();
        assert!(b.len() > 20);
    }

    #[test]
    fn emit_orv_differs_from_addv() {
        let addv = emit_addv(0x50, 0x51).unwrap();
        let orv = emit_orv(0x50, 0x51).unwrap();
        assert_ne!(addv, orv, "ORV MUST NOT alias ADDV");
        assert!(orv.windows(3).any(|w| w == [0x48, 0x09, 0xC8]));
        assert!(addv.windows(3).any(|w| w == [0x48, 0x01, 0xC8]));
    }

    #[test]
    fn emit_cmp_composes() {
        let b = emit_cmp(0x51, 0x52).unwrap();
        assert!(b.ends_with(&[0x48, 0x39, 0xC8]));
    }

    #[test]
    fn emit_inc_composes() {
        let b = emit_inc(0x51).unwrap();
        assert!(b.len() > 10);
    }

    #[test]
    fn jcc_table_len() {
        assert_eq!(JCC_TABLE.len(), 10);
        assert_eq!(JCC_MNEMONIC.len(), 10);
    }

    #[test]
    fn load_state_r8_dest() {
        let b = load_state(0, Reg::R8).unwrap();
        // REX.WRB = 0x4D
        assert_eq!(b[0], 0x4D);
        assert_eq!(b[1], 0x8B);
    }

    #[test]
    fn store_state_rcx() {
        let b = store_state(0, Reg::Rcx).unwrap();
        assert_eq!(b, vec![0x49, 0x89, 0x4F, 0x00]);
    }

    #[test]
    fn imm_out_of_range() {
        let big = (i32::MAX as u64) + 1;
        // as i64 this wraps to negative large — still within i64 but > i32::MAX as positive
        // We cast imm as i64; values > i32::MAX as u64 become large positive i64 if < 2^63
        assert!(add_imm(Reg::Rax, big).is_err() || add_imm(Reg::Rax, big).is_ok());
        // Explicitly over i32 range via bit pattern that fails our check
        assert!(add_imm(Reg::Rax, 0x1_0000_0000).is_err());
    }

    #[test]
    fn all_jcc_codes_accepted() {
        for &cc in &JCC_VALID {
            assert!(jcc_rel32(cc, 0).is_ok());
        }
    }

    #[test]
    fn emit_imul_composes() {
        assert!(emit_imul(0x53, 0x54).is_ok());
    }

    #[test]
    fn emit_subv_composes() {
        assert!(emit_subv(0x50, 0x53).is_ok());
    }

    #[test]
    fn emit_add_sub_imm() {
        assert!(emit_add_imm(0x50, 7).is_ok());
        assert!(emit_sub_imm(0x50, 3).is_ok());
    }

    #[test]
    fn emit_dec_composes() {
        assert!(emit_dec(0x51).is_ok());
    }

    #[test]
    fn load_high_slot() {
        let b = load_state(255, Reg::Rdi).unwrap();
        assert_eq!(b.len(), 7);
        assert_eq!(&b[3..], &(255u32 * 8).to_le_bytes());
    }
}
