//! Minimal RISC-V RV32 ELF32 linker (LE).
//!
//! Produces a working RISC-V 32-bit ELF32 executable wrapping emitted
//! .text + .data. No startup preamble — code starts directly at entry.
//! Entry VA = 0x8001000, data VA = 0x8002000.
//!
//! Data section size floor: 0x38000.

use crate::types::IsaResult;

const OUTPUT_DATA_NEED: u32 = 0x38000;
const PAGE_SIZE: u32 = 0x1000;
// ELF32 header = 52, program header entry = 32.
const ELF_EHDR_SIZE: u32 = 52;
const ELF_PHDR_SIZE: u32 = 32;

pub struct ElfRiscv32Image {
    pub bytes: Vec<u8>,
}

/// Wrap raw code (+ optional data) in a RISC-V RV32 ELF32 executable.
pub fn link_riscv32_elf(code: &[u8], data: &[u8]) -> IsaResult<ElfRiscv32Image> {
    let phdr_count: u16 = 2;

    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, PAGE_SIZE));
    let data_align = align_up(data_need, PAGE_SIZE);

    // Virtual layout:
    //   .text @ 0x8001000, .data @ 0x8002000
    let text_va = 0x8001000u32;
    let data_va = 0x8002000u32;

    let hdr_file_size = align_up(ELF_EHDR_SIZE + phdr_count as u32 * ELF_PHDR_SIZE, PAGE_SIZE);
    let text_file_off = hdr_file_size as u32;
    let text_file_size = align_up(code.len() as u32 + 16, PAGE_SIZE);
    let text_mem_size = text_file_size;

    let data_file_off = text_file_off + text_file_size;
    let data_file_size = data_align;
    let data_mem_size = data_align;

    let total_file_size = (data_file_off + data_file_size) as usize;
    let mut img = vec![0u8; total_file_size];

    // ── ELF Header (little-endian) ──
    img[0..4].copy_from_slice(b"\x7fELF");
    img[4] = 1; // ELFCLASS32
    img[5] = 1; // ELFDATA2LSB
    img[6] = 1; // EV_CURRENT

    write_u16(&mut img, 16, 2);               // ET_EXEC
    write_u16(&mut img, 18, 0xF3);            // EM_RISCV
    write_u32(&mut img, 20, 1);               // e_version
    write_u32(&mut img, 24, text_va);         // e_entry
    write_u32(&mut img, 28, ELF_EHDR_SIZE);   // e_phoff
    // e_shoff (byte 32) = 0
    write_u32(&mut img, 36, 0);               // e_flags
    write_u16(&mut img, 40, ELF_EHDR_SIZE as u16); // e_ehsize
    write_u16(&mut img, 42, ELF_PHDR_SIZE as u16); // e_phentsize
    write_u16(&mut img, 44, phdr_count);      // e_phnum

    // ── Program header 0: .text ──
    let phdr_off = ELF_EHDR_SIZE as usize;
    write_u32(&mut img, phdr_off, 1);           // PT_LOAD
    write_u32(&mut img, phdr_off + 4, text_file_off); // p_offset
    write_u32(&mut img, phdr_off + 8, text_va); // p_vaddr
    write_u32(&mut img, phdr_off + 12, text_va); // p_paddr
    write_u32(&mut img, phdr_off + 16, text_file_size); // p_filesz
    write_u32(&mut img, phdr_off + 20, text_mem_size); // p_memsz
    write_u32(&mut img, phdr_off + 24, 0x00000005); // p_flags = PF_R | PF_X
    write_u32(&mut img, phdr_off + 28, PAGE_SIZE); // p_align

    // ── Program header 1: .data ──
    let phdr2_off = phdr_off + ELF_PHDR_SIZE as usize;
    write_u32(&mut img, phdr2_off, 1);            // PT_LOAD
    write_u32(&mut img, phdr2_off + 4, data_file_off); // p_offset
    write_u32(&mut img, phdr2_off + 8, data_va); // p_vaddr
    write_u32(&mut img, phdr2_off + 12, data_va); // p_paddr
    write_u32(&mut img, phdr2_off + 16, data_file_size); // p_filesz
    write_u32(&mut img, phdr2_off + 20, data_mem_size); // p_memsz
    write_u32(&mut img, phdr2_off + 24, 0x00000006); // p_flags = PF_R | PF_W
    write_u32(&mut img, phdr2_off + 28, PAGE_SIZE); // p_align

    // ── Startup stub at start of .text ──
    //   auipc x5, <page of data_va>
    //   addi  x5, x5, <lo12 of data_va>
    //   jal   x0, <offset to user code>
    //   NOP pad (4 bytes) -> 16 bytes
    let text_off = text_file_off as usize;
    let startup_len = 16u32;
    let user_code_va = text_va + startup_len;
    // auipc x5, imm20
    img[text_off..text_off + 4].copy_from_slice(&riscv_auipc(5, data_va as u64));
    // addi x5, x5, lo12(data_va)
    img[text_off + 4..text_off + 8].copy_from_slice(&riscv_addi(5, 5, data_va & 0xFFF));
    // jal x0, imm20 — PC-relative to user code
    let jal_target = (user_code_va as i32) - ((text_va + 8) as i32);
    img[text_off + 8..text_off + 12].copy_from_slice(&riscv_jal(0, jal_target));
    // NOP pad
    img[text_off + 12..text_off + 16].copy_from_slice(&[0x13, 0x00, 0x00, 0x00]);
    // Copy user code after startup
    let code_dst = text_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // ── Copy data ──
    let data_off = data_file_off as usize;
    let copy_n = data.len().min(data_file_size as usize);
    img[data_off..data_off + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(ElfRiscv32Image { bytes: img })
}

fn align_up(v: u32, a: u32) -> u32 {
    (v + a - 1) & !(a - 1)
}

fn riscv_auipc(rd: u32, addr: u64) -> [u8; 4] {
    let imm20 = ((addr >> 12) & 0xFFFFF) as u32;
    let enc: u32 = 0x000000F7 | (imm20 << 12) | (rd << 7);
    enc.to_le_bytes()
}

fn riscv_addi(rd: u32, rs1: u32, imm12: u32) -> [u8; 4] {
    let imm12 = imm12 & 0xFFF;
    let enc: u32 = 0x00000013 | (imm12 << 20) | (rs1 << 15) | (rd << 7);
    enc.to_le_bytes()
}

fn riscv_jal(rd: u32, imm20: i32) -> [u8; 4] {
    let imm20 = imm20 as u32;
    let b0 = ((imm20 >> 0) & 0x1) << 31;
    let b1 = ((imm20 >> 1) & 0x1FF) << 21;
    let b2 = ((imm20 >> 11) & 0x1) << 20;
    let b3 = ((imm20 >> 12) & 0xFF) << 12;
    let enc: u32 = 0x0000006F | b0 | b1 | b2 | b3 | (rd << 7);
    enc.to_le_bytes()
}

fn write_u16(buf: &mut [u8], off: usize, v: u16) {
    buf[off..off + 2].copy_from_slice(&v.to_le_bytes());
}

fn write_u32(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn riscv32_elf_has_magic() {
        let elf = link_riscv32_elf(&[0x13, 0x00, 0x00, 0x00], &[]).unwrap();
        assert_eq!(&elf.bytes[0..4], b"\x7fELF");
        assert_eq!(elf.bytes[4], 1); // ELFCLASS32
        assert_eq!(elf.bytes[5], 1); // ELFDATA2LSB
        let e_machine = u16::from_le_bytes(elf.bytes[18..20].try_into().unwrap());
        assert_eq!(e_machine, 0xF3); // EM_RISCV
    }

    #[test]
    fn riscv32_elf_machine_and_entry() {
        let elf = link_riscv32_elf(&[0x13, 0x00, 0x00, 0x00], &[]).unwrap();
        let e_machine = u16::from_le_bytes(elf.bytes[18..20].try_into().unwrap());
        assert_eq!(e_machine, 0xF3);
        let e_entry = u32::from_le_bytes(elf.bytes[24..28].try_into().unwrap());
        assert_eq!(e_entry, 0x8001000);
        let e_phnum = u16::from_le_bytes(elf.bytes[44..46].try_into().unwrap());
        assert_eq!(e_phnum, 2);
    }

    #[test]
    fn riscv32_data_floor() {
        let elf = link_riscv32_elf(&[0x13, 0x00, 0x00, 0x00], &[]).unwrap();
        assert!(elf.bytes.len() > 0x38000);
    }
}