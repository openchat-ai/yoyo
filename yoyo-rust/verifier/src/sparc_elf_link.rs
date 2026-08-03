//! Minimal SPARC v8 ELF32 big-endian linker.
//!
//! Produces a working SPARC v8 ELF32 executable wrapping emitted
//! .text + .data. ALL multi-byte values are big-endian.
//! No startup preamble — code starts directly at entry.
//! Entry VA = 0x10000, data VA = 0x20000.
//!
//! Data section size floor: 0x38000.

use crate::types::IsaResult;

const OUTPUT_DATA_NEED: u32 = 0x38000;
const PAGE_SIZE: u32 = 0x1000;
// ELF32 header = 52, program header entry = 32.
const ELF_EHDR_SIZE: u32 = 52;
const ELF_PHDR_SIZE: u32 = 32;

pub struct ElfSparcImage {
    pub bytes: Vec<u8>,
}

/// Wrap raw code (+ optional data) in a SPARC v8 ELF32 big-endian executable.
pub fn link_sparc_elf(code: &[u8], data: &[u8]) -> IsaResult<ElfSparcImage> {
    let phdr_count: u16 = 2;

    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, PAGE_SIZE));
    let data_align = align_up(data_need, PAGE_SIZE);

    // Virtual layout:
    //   .text @ 0x10000, .data @ 0x20000
    let text_va = 0x10000u32;
    let data_va = 0x20000u32;

    let hdr_file_size = align_up(ELF_EHDR_SIZE + phdr_count as u32 * ELF_PHDR_SIZE, PAGE_SIZE);
    let text_file_off = hdr_file_size as u32;
    let text_file_size = align_up(code.len() as u32, PAGE_SIZE);
    let text_mem_size = text_file_size;

    let data_file_off = text_file_off + text_file_size;
    let data_file_size = data_align;
    let data_mem_size = data_align;

    let total_file_size = (data_file_off + data_file_size) as usize;
    let mut img = vec![0u8; total_file_size];

    // ── ELF Header (big-endian) ──
    img[0..4].copy_from_slice(b"\x7fELF");
    img[4] = 1; // ELFCLASS32
    img[5] = 2; // ELFDATA2MSB (big-endian)
    img[6] = 1; // EV_CURRENT

    write_u16_be(&mut img, 16, 2);               // ET_EXEC
    write_u16_be(&mut img, 18, 0x02);            // EM_SPARC
    write_u32_be(&mut img, 20, 1);               // e_version
    write_u32_be(&mut img, 24, text_va);         // e_entry
    write_u32_be(&mut img, 28, ELF_EHDR_SIZE);   // e_phoff
    // e_shoff (byte 32) = 0
    write_u32_be(&mut img, 36, 0);               // e_flags
    write_u16_be(&mut img, 40, ELF_EHDR_SIZE as u16); // e_ehsize
    write_u16_be(&mut img, 42, ELF_PHDR_SIZE as u16); // e_phentsize
    write_u16_be(&mut img, 44, phdr_count);      // e_phnum

    // ── Program header 0: .text ──
    let phdr_off = ELF_EHDR_SIZE as usize;
    write_u32_be(&mut img, phdr_off, 1);           // PT_LOAD
    write_u32_be(&mut img, phdr_off + 4, text_file_off); // p_offset
    write_u32_be(&mut img, phdr_off + 8, text_va); // p_vaddr
    write_u32_be(&mut img, phdr_off + 12, text_va); // p_paddr
    write_u32_be(&mut img, phdr_off + 16, text_file_size); // p_filesz
    write_u32_be(&mut img, phdr_off + 20, text_mem_size); // p_memsz
    write_u32_be(&mut img, phdr_off + 24, 0x00000005); // p_flags = PF_R | PF_X
    write_u32_be(&mut img, phdr_off + 28, PAGE_SIZE); // p_align

    // ── Program header 1: .data ──
    let phdr2_off = phdr_off + ELF_PHDR_SIZE as usize;
    write_u32_be(&mut img, phdr2_off, 1);            // PT_LOAD
    write_u32_be(&mut img, phdr2_off + 4, data_file_off); // p_offset
    write_u32_be(&mut img, phdr2_off + 8, data_va); // p_vaddr
    write_u32_be(&mut img, phdr2_off + 12, data_va); // p_paddr
    write_u32_be(&mut img, phdr2_off + 16, data_file_size); // p_filesz
    write_u32_be(&mut img, phdr2_off + 20, data_mem_size); // p_memsz
    write_u32_be(&mut img, phdr2_off + 24, 0x00000006); // p_flags = PF_R | PF_W
    write_u32_be(&mut img, phdr2_off + 28, PAGE_SIZE); // p_align

    // ── Copy user code at start of .text (no startup preamble) ──
    let text_off = text_file_off as usize;
    img[text_off..text_off + code.len()].copy_from_slice(code);

    // ── Copy data ──
    let data_off = data_file_off as usize;
    let copy_n = data.len().min(data_file_size as usize);
    img[data_off..data_off + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(ElfSparcImage { bytes: img })
}

fn align_up(v: u32, a: u32) -> u32 {
    (v + a - 1) & !(a - 1)
}

fn write_u16_be(buf: &mut [u8], off: usize, v: u16) {
    buf[off..off + 2].copy_from_slice(&v.to_be_bytes());
}

fn write_u32_be(buf: &mut [u8], off: usize, v: u32) {
    buf[off..off + 4].copy_from_slice(&v.to_be_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparc_elf_has_magic() {
        let elf = link_sparc_elf(&[0x01, 0x00, 0x00, 0x00], &[]).unwrap();
        assert_eq!(&elf.bytes[0..4], b"\x7fELF");
        assert_eq!(elf.bytes[4], 1); // ELFCLASS32
        assert_eq!(elf.bytes[5], 2); // ELFDATA2MSB
    }

    #[test]
    fn sparc_elf_machine_and_entry() {
        let elf = link_sparc_elf(&[0x01, 0x00, 0x00, 0x00], &[]).unwrap();
        let e_machine = u16::from_be_bytes(elf.bytes[18..20].try_into().unwrap());
        assert_eq!(e_machine, 0x02); // EM_SPARC
        let e_entry = u32::from_be_bytes(elf.bytes[24..28].try_into().unwrap());
        assert_eq!(e_entry, 0x10000);
        let e_phnum = u16::from_be_bytes(elf.bytes[44..46].try_into().unwrap());
        assert_eq!(e_phnum, 2);
    }

    #[test]
    fn sparc_data_floor() {
        let elf = link_sparc_elf(&[0x01, 0x00, 0x00, 0x00], &[]).unwrap();
        assert!(elf.bytes.len() > 0x38000);
    }
}