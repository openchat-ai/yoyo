//! ARM32 (32-bit ARM, Android EABI) ELF32 linker.
//!
//! Produces a 32-bit ELF executable (e_machine = EM_ARM = 0x28, data2LSB)
//! with two PT_LOAD segments: .text and .data. Entry at 0x8001000,
//! data at 0x8002000.
//!
//! No startup preamble — code written at entry directly (like MIPS).
//! Data section size floor: 0x38000.

use crate::types::IsaResult;

/// Minimum data section size (matches other backends).
const OUTPUT_DATA_NEED: u32 = 0x38000;

/// 32-bit ELF constants.
const ELF_EHDR_SIZE: u32 = 52; // ELF32 ehdr is 52 bytes (ident 16 + rest 36)
const ELF_PHDR_SIZE: u32 = 32; // program header entry size (Elf32_Phdr)
const PAGE_SIZE: u32 = 0x1000;

/// ARM32 image.
pub struct Arm32ElfImage {
    pub bytes: Vec<u8>,
}

/// Wrap raw x64-ish code (+ optional data) in an ELF32 ARM executable.
/// Entry: 0x8001000 (.text), data VA 0x8002000.
pub fn link_arm32_elf(code: &[u8], data: &[u8]) -> IsaResult<Arm32ElfImage> {
    let phdr_count: u16 = 2;

    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, PAGE_SIZE));
    let data_align = align_up(data_need, PAGE_SIZE);

    // Layout (virtual addresses, ARM32 base 0x8000000):
    let text_va: u32 = 0x8001000;
    let data_va: u32 = 0x8002000;

    let hdr_file_size = align_up(ELF_EHDR_SIZE + phdr_count as u32 * ELF_PHDR_SIZE, PAGE_SIZE);
    let text_file_off = hdr_file_size;
    let startup_len = 16u32;
    let text_file_size = align_up(code.len() as u32 + startup_len, 0x10);
    let text_mem_size = align_up(text_file_size, PAGE_SIZE);

    let data_file_off = text_file_off + text_file_size;
    let data_file_size = data_align;
    let data_mem_size = data_align;

    let total_file_size = (data_file_off + data_file_size) as usize;

    let mut img = vec![0u8; total_file_size];

    // ── ELF Header (offset 0, 52 bytes) ──
    img[0..4].copy_from_slice(b"\x7fELF");
    img[4] = 1; // ELFCLASS32
    img[5] = 1; // ELFDATA2LSB
    img[6] = 1; // EV_CURRENT
    // bytes 7-15 zero

    write_u16(&mut img, 16, 2); // e_type = ET_EXEC
    write_u16(&mut img, 18, 0x28); // e_machine = EM_ARM
    write_u32(&mut img, 20, 1); // e_version

    write_u32(&mut img, 24, text_va); // e_entry (32-bit)
    write_u32(&mut img, 28, ELF_EHDR_SIZE); // e_phoff (32-bit)
    write_u32(&mut img, 32, 0); // e_shoff = 0
    write_u32(&mut img, 36, 0); // e_flags

    write_u16(&mut img, 40, ELF_EHDR_SIZE as u16); // e_ehsize
    write_u16(&mut img, 42, ELF_PHDR_SIZE as u16); // e_phentsize
    write_u16(&mut img, 44, phdr_count); // e_phnum
    write_u16(&mut img, 46, 0); // e_shentsize
    write_u16(&mut img, 48, 0); // e_shnum
    write_u16(&mut img, 50, 0); // e_shstrndx

    // ── Program Headers (offset 52) ──
    // PHDR 0: .text (PT_LOAD)
    let phdr_off = ELF_EHDR_SIZE as usize;
    write_u32(&mut img, phdr_off, 1); // p_type = PT_LOAD
    write_u32(&mut img, phdr_off + 4, text_file_off); // p_offset
    write_u32(&mut img, phdr_off + 8, text_va); // p_vaddr
    write_u32(&mut img, phdr_off + 12, text_va); // p_paddr
    write_u32(&mut img, phdr_off + 16, text_file_size); // p_filesz
    write_u32(&mut img, phdr_off + 20, text_mem_size); // p_memsz
    write_u32(&mut img, phdr_off + 24, 5); // p_flags = PF_R | PF_X
    write_u32(&mut img, phdr_off + 28, PAGE_SIZE); // p_align

    // PHDR 1: .data (PT_LOAD)
    let phdr2_off = phdr_off + ELF_PHDR_SIZE as usize;
    write_u32(&mut img, phdr2_off, 1); // p_type = PT_LOAD
    write_u32(&mut img, phdr2_off + 4, data_file_off); // p_offset
    write_u32(&mut img, phdr2_off + 8, data_va); // p_vaddr
    write_u32(&mut img, phdr2_off + 12, data_va); // p_paddr
    write_u32(&mut img, phdr2_off + 16, data_file_size); // p_filesz
    write_u32(&mut img, phdr2_off + 20, data_mem_size); // p_memsz
    write_u32(&mut img, phdr2_off + 24, 6); // p_flags = PF_R | PF_W
    write_u32(&mut img, phdr2_off + 28, PAGE_SIZE); // p_align

    // ── Build startup at start of .text ──
    //   movw r8, #lo16(data_va)
    //   movt r8, #hi16(data_va)
    //   b <user_code>
    //   NOP (4 bytes)
    let code_dst = (text_file_off + startup_len) as usize;
    let text_off = text_file_off as usize;
    // movw r8, lo16(data_va)
    {
        let lo16 = data_va & 0xFFFF;
        let enc: u32 = 0xE3000000 | (8 << 12) | lo16;
        img[text_off..text_off + 4].copy_from_slice(&enc.to_le_bytes());
    }
    // movt r8, hi16(data_va)
    {
        let hi16 = data_va >> 16;
        let enc: u32 = 0xE3400000 | (8 << 12) | hi16;
        img[text_off + 4..text_off + 8].copy_from_slice(&enc.to_le_bytes());
    }
    // b <user_code>
    {
        let user_code_va = text_va + startup_len;
        let offset = (user_code_va as i32) - ((text_va + 8) as i32);
        let imm24 = ((offset >> 2) & 0xFFFFFF) as u32;
        let enc: u32 = 0xEA000000 | imm24;
        img[text_off + 8..text_off + 12].copy_from_slice(&enc.to_le_bytes());
    }
    // NOP
    img[text_off + 12..text_off + 16].copy_from_slice(&[0x00, 0x00, 0xA0, 0xE1]);
    // Copy user code
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // ── Copy data ──
    let data_off = data_file_off as usize;
    let copy_n = data.len().min(data_file_size as usize);
    img[data_off..data_off + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(Arm32ElfImage { bytes: img })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arm32_has_elf32_magic() {
        let img = link_arm32_elf(&[0xE1, 0xA0, 0x00, 0x00], &[]).unwrap();
        assert_eq!(&img.bytes[0..4], b"\x7fELF");
        assert_eq!(img.bytes[4], 1); // ELFCLASS32
        assert_eq!(img.bytes[5], 1); // ELFDATA2LSB
    }

    #[test]
    fn arm32_header_fields() {
        let img = link_arm32_elf(&[0xE1, 0xA0, 0x00, 0x00], &[]).unwrap();
        let e_type = u16::from_le_bytes(img.bytes[16..18].try_into().unwrap());
        assert_eq!(e_type, 2); // ET_EXEC
        let e_machine = u16::from_le_bytes(img.bytes[18..20].try_into().unwrap());
        assert_eq!(e_machine, 0x28); // EM_ARM
        let e_phnum = u16::from_le_bytes(img.bytes[44..46].try_into().unwrap());
        assert_eq!(e_phnum, 2);
    }

    #[test]
    fn arm32_entry_and_sizes() {
        let img = link_arm32_elf(&[0xE1, 0xA0, 0x00, 0x00], &[]).unwrap();
        let e_entry = u32::from_le_bytes(img.bytes[24..28].try_into().unwrap());
        assert_eq!(e_entry, 0x8001000);
        let e_phoff = u32::from_le_bytes(img.bytes[28..32].try_into().unwrap());
        assert_eq!(e_phoff, ELF_EHDR_SIZE);
        assert!(img.bytes.len() > 0x38000);
    }

    #[test]
    fn arm32_two_phdrs() {
        let img = link_arm32_elf(&[0xE1, 0xA0, 0x00, 0x00], &[]).unwrap();
        let phdr_off = ELF_EHDR_SIZE as usize;
        let text_flags = u32::from_le_bytes(img.bytes[phdr_off + 24..phdr_off + 28].try_into().unwrap());
        assert_eq!(text_flags, 5); // PF_R | PF_X
        let data_flags =
            u32::from_le_bytes(img.bytes[phdr_off + 32 + 24..phdr_off + 32 + 28].try_into().unwrap());
        assert_eq!(data_flags, 6); // PF_R | PF_W
    }
}
