//! Minimal RISC-V RV64 ELF64 linker (PROMPT-v3 Phase 1).
//!
//! Produces a working RISC-V Linux ELF64 executable wrapping emitted
//! .text + .data. The startup stub at the start of .text uses an
//! auipc/addi/jal sequence to set up x5 (t0) -> .data base (state pointer),
//! then jumps to user code.
//!
//! Data section size floor: 0x38000 (same as other ELF64 backends).

use crate::types::IsaResult;

const OUTPUT_DATA_NEED: u32 = 0x38000;
const PAGE_SIZE: u32 = 0x1000;
const ELF_EHDR_SIZE: u32 = 64;
const ELF_PHDR_SIZE: u32 = 56;

pub struct ElfRiscvImage {
    pub bytes: Vec<u8>,
}

/// Wrap raw code (+ optional data) in a RISC-V RV64 ELF64 executable.
pub fn link_riscv_elf(code: &[u8], data: &[u8]) -> IsaResult<ElfRiscvImage> {
    let phdr_count: u16 = 2;

    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, PAGE_SIZE));
    let data_align = align_up(data_need, PAGE_SIZE);

    // Virtual layout:
    //   .text @ 0x1001000, .data @ 0x1002000
    let text_va = 0x1001000u64;
    let data_va = 0x1002000u64;

    let hdr_file_size = align_up(ELF_EHDR_SIZE + phdr_count as u32 * ELF_PHDR_SIZE, PAGE_SIZE);
    let text_file_off = hdr_file_size as u64;
    // 3 × 4-byte RISC-V insns: auipc x1, data_page; addi x1, x1, lo12(data_va); jal x0, code
    let startup_len = 16u32; // 12 + 4 nop pad
    let text_file_size = align_up(code.len() as u32 + startup_len, PAGE_SIZE) as u64;
    let text_mem_size = align_up(text_file_size as u32, PAGE_SIZE) as u64;

    let data_file_off = text_file_off + text_file_size;
    let data_file_size = data_align as u64;
    let data_mem_size = data_align as u64;

    let total_file_size = (data_file_off + data_file_size) as usize;
    let mut img = vec![0u8; total_file_size];

    // ── ELF Header ──
    img[0..4].copy_from_slice(b"\x7fELF");
    img[4] = 2; // ELFCLASS64
    img[5] = 1; // ELFDATA2LSB
    img[6] = 1; // EV_CURRENT

    write_u16(&mut img, 16, 2);               // ET_EXEC
    write_u16(&mut img, 18, 0xF3);            // EM_RISCV
    write_u32(&mut img, 20, 1);               // e_version
    write_u64(&mut img, 24, text_va);         // e_entry
    write_u64(&mut img, 32, ELF_EHDR_SIZE as u64); // e_phoff
    write_u32(&mut img, 48, 0);               // e_flags
    write_u16(&mut img, 52, ELF_EHDR_SIZE as u16); // e_ehsize
    write_u16(&mut img, 54, ELF_PHDR_SIZE as u16); // e_phentsize
    write_u16(&mut img, 56, phdr_count);      // e_phnum

    // ── Program header 0: .text ──
    let phdr_off = ELF_EHDR_SIZE as usize;
    write_u32(&mut img, phdr_off, 1);
    write_u32(&mut img, phdr_off + 4, 5); // PF_R | PF_X
    write_u64(&mut img, phdr_off + 8, text_file_off);
    write_u64(&mut img, phdr_off + 16, text_va);
    write_u64(&mut img, phdr_off + 24, text_va);
    write_u64(&mut img, phdr_off + 32, text_file_size);
    write_u64(&mut img, phdr_off + 40, text_mem_size);
    write_u64(&mut img, phdr_off + 48, PAGE_SIZE as u64);

    // ── Program header 1: .data ──
    let phdr2_off = phdr_off + ELF_PHDR_SIZE as usize;
    write_u32(&mut img, phdr2_off, 1);
    write_u32(&mut img, phdr2_off + 4, 6); // PF_R | PF_W
    write_u64(&mut img, phdr2_off + 8, data_file_off);
    write_u64(&mut img, phdr2_off + 16, data_va);
    write_u64(&mut img, phdr2_off + 24, data_va);
    write_u64(&mut img, phdr2_off + 32, data_file_size);
    write_u64(&mut img, phdr2_off + 40, data_mem_size);
    write_u64(&mut img, phdr2_off + 48, PAGE_SIZE as u64);

    // ── Startup stub at start of .text ──
    //   auipc x5, <page of data_va>
    //   addi  x5, x5, <lo12 of data_va>
    //   jal   x0, <offset to user code>
    //   (pad with NOPs to 16 bytes)
    let text_off = text_file_off as usize;
    let user_code_va = text_va + startup_len as u64;

    // auipc x5, imm20 — imm20 = data_va >> 12
    img[text_off..text_off + 4].copy_from_slice(&riscv_auipc(5, data_va));
    // addi x5, x5, lo12(data_va)
    img[text_off + 4..text_off + 8].copy_from_slice(&riscv_addi(5, 5, data_va as i64 as i32));
    // jal x0, imm20 — imm20 = PC-relative to user code
    // After addi, PC = text_va + 8; target = user_code_va
    let jal_target = (user_code_va as i64) - ((text_va + 8) as i64);
    img[text_off + 8..text_off + 12].copy_from_slice(&riscv_jal(0, jal_target as i32));
    // NOP pad (4 bytes)
    img[text_off + 12..text_off + 16].copy_from_slice(&[0x13, 0x00, 0x00, 0x00]);

    // Copy user code
    let code_dst = text_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // Copy data
    let data_off = data_file_off as usize;
    let copy_n = data.len().min(data_file_size as usize);
    img[data_off..data_off + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(ElfRiscvImage { bytes: img })
}

/// Encode RISC-V `auipc rd, imm20` (upper 20 bits of target address).
fn riscv_auipc(rd: u32, addr: u64) -> [u8; 4] {
    let imm20 = ((addr >> 12) & 0xFFFFF) as u32;
    let enc: u32 = 0x000000F7 | (imm20 << 12) | (rd << 7);
    enc.to_le_bytes()
}

/// Encode RISC-V `addi rd, rs1, imm12` (signed 12-bit).
fn riscv_addi(rd: u32, rs1: u32, imm12: i32) -> [u8; 4] {
    let imm12 = (imm12 & 0xFFF) as u32;
    let enc: u32 = 0x00000013 | (imm12 << 20) | (rs1 << 15) | (rd << 7);
    enc.to_le_bytes()
}

/// Encode RISC-V `jal rd, imm20` (signed PC-relative 20-bit).
fn riscv_jal(rd: u32, imm20: i32) -> [u8; 4] {
    let imm20 = imm20 as u32;
    // imm[20|10:1|11|12:19] encoding for jal U-type
    let b0 = ((imm20 >> 0) & 0x1) << 31;
    let b1 = ((imm20 >> 1) & 0x1FF) << 21;
    let b2 = ((imm20 >> 11) & 0x1) << 20;
    let b3 = ((imm20 >> 12) & 0xFF) << 12;
    let enc: u32 = 0x0000006F | b0 | b1 | b2 | b3 | (rd << 7);
    enc.to_le_bytes()
}

fn align_up(v: u32, a: u32) -> u32 {
    (v + a - 1) & !(a - 1)
}

fn write_u16(buf: &mut [u8], off: usize, v: u16) {
    buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
}

fn write_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

fn write_u64(buf: &mut [u8], off: usize, v: u64) {
    buf[off..off + 8].copy_from_slice(&v.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn riscv_elf_has_magic() {
        let elf = link_riscv_elf(&[0x13, 0x00, 0x00, 0x00], &[]).unwrap();
        assert_eq!(&elf.bytes[0..4], b"\x7fELF");
        assert_eq!(elf.bytes[4], 2); // ELFCLASS64
        assert_eq!(elf.bytes[5], 1); // ELFDATA2LSB
        let e_machine = u16::from_le_bytes(elf.bytes[18..20].try_into().unwrap());
        assert_eq!(e_machine, 0xF3); // EM_RISCV
    }

    #[test]
    fn riscv_elf_entry_and_phnum() {
        let elf = link_riscv_elf(&[0x13, 0x00, 0x00, 0x00], &[]).unwrap();
        let e_entry = u64::from_le_bytes(elf.bytes[24..32].try_into().unwrap());
        assert_eq!(e_entry, 0x1001000);
        let e_phnum = u16::from_le_bytes(elf.bytes[56..58].try_into().unwrap());
        assert_eq!(e_phnum, 2);
    }

    #[test]
    fn riscv_data_floor() {
        let elf = link_riscv_elf(&[0x13, 0x00, 0x00, 0x00], &[]).unwrap();
        assert!(elf.bytes.len() > 0x38000);
    }
}
