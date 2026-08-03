//! Minimal LoongArch LA64 ELF64 linker (LE).
//!
//! Produces a working LoongArch ELF64 executable wrapping emitted
//! .text + .data. No startup preamble — code starts directly at entry.
//! Entry VA = 0x120000000, data VA = 0x120010000.
//!
//! Data section size floor: 0x38000.

use crate::types::IsaResult;

const OUTPUT_DATA_NEED: u32 = 0x38000;
const PAGE_SIZE: u32 = 0x1000;
const ELF_EHDR_SIZE: u32 = 64;
const ELF_PHDR_SIZE: u32 = 56;

pub struct ElfLoongArchImage {
    pub bytes: Vec<u8>,
}

/// Wrap raw code (+ optional data) in a LoongArch ELF64 executable.
pub fn link_loongarch_elf(code: &[u8], data: &[u8]) -> IsaResult<ElfLoongArchImage> {
    let phdr_count: u16 = 2;

    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, PAGE_SIZE));
    let data_align = align_up(data_need, PAGE_SIZE);

    // Virtual layout:
    //   .text @ 0x120000000, .data @ 0x120010000
    let text_va = 0x120000000u64;
    let data_va = 0x120010000u64;

    let hdr_file_size = align_up(ELF_EHDR_SIZE + phdr_count as u32 * ELF_PHDR_SIZE, PAGE_SIZE);
    let text_file_off = hdr_file_size as u64;
    let text_file_size = align_up(code.len() as u32, PAGE_SIZE) as u64;
    let text_mem_size = text_file_size;

    let data_file_off = text_file_off + text_file_size;
    let data_file_size = data_align as u64;
    let data_mem_size = data_align as u64;

    let total_file_size = (data_file_off + data_file_size) as usize;
    let mut img = vec![0u8; total_file_size];

    // ── ELF Header ──
    img[0..4].copy_from_slice(b"\x7fELF");
    img[4] = 2;  // ELFCLASS64
    img[5] = 1;  // ELFDATA2LSB
    img[6] = 1;  // EV_CURRENT

    write_u16(&mut img, 16, 2);                   // ET_EXEC
    write_u16(&mut img, 18, 0x102);               // EM_LOONGARCH
    write_u32(&mut img, 20, 1);                   // e_version
    write_u64(&mut img, 24, text_va);             // e_entry
    write_u64(&mut img, 32, ELF_EHDR_SIZE as u64); // e_phoff
    write_u32(&mut img, 48, 0);                   // e_flags
    write_u16(&mut img, 52, ELF_EHDR_SIZE as u16); // e_ehsize
    write_u16(&mut img, 54, ELF_PHDR_SIZE as u16); // e_phentsize
    write_u16(&mut img, 56, phdr_count);          // e_phnum

    // ── Program header 0: .text ──
    let phdr_off = ELF_EHDR_SIZE as usize;
    write_u32(&mut img, phdr_off, 1);               // PT_LOAD
    write_u32(&mut img, phdr_off + 4, 5);           // PF_R | PF_X
    write_u64(&mut img, phdr_off + 8, text_file_off);
    write_u64(&mut img, phdr_off + 16, text_va);
    write_u64(&mut img, phdr_off + 24, text_va);
    write_u64(&mut img, phdr_off + 32, text_file_size);
    write_u64(&mut img, phdr_off + 40, text_mem_size);
    write_u64(&mut img, phdr_off + 48, PAGE_SIZE as u64);

    // ── Program header 1: .data ──
    let phdr2_off = phdr_off + ELF_PHDR_SIZE as usize;
    write_u32(&mut img, phdr2_off, 1);              // PT_LOAD
    write_u32(&mut img, phdr2_off + 4, 6);          // PF_R | PF_W
    write_u64(&mut img, phdr2_off + 8, data_file_off);
    write_u64(&mut img, phdr2_off + 16, data_va);
    write_u64(&mut img, phdr2_off + 24, data_va);
    write_u64(&mut img, phdr2_off + 32, data_file_size);
    write_u64(&mut img, phdr2_off + 40, data_mem_size);
    write_u64(&mut img, phdr2_off + 48, PAGE_SIZE as u64);

    // ── Copy user code at start of .text (no startup preamble) ──
    let text_off = text_file_off as usize;
    img[text_off..text_off + code.len()].copy_from_slice(code);

    // ── Copy data ──
    let data_off = data_file_off as usize;
    let copy_n = data.len().min(data_file_size as usize);
    img[data_off..data_off + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(ElfLoongArchImage { bytes: img })
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
    fn loongarch_elf_has_magic() {
        let elf = link_loongarch_elf(&[0x00, 0x00, 0x00, 0x00], &[]).unwrap();
        assert_eq!(&elf.bytes[0..4], b"\x7fELF");
        assert_eq!(elf.bytes[4], 2); // ELFCLASS64
        assert_eq!(elf.bytes[5], 1); // ELFDATA2LSB
        let e_machine = u16::from_le_bytes(elf.bytes[18..20].try_into().unwrap());
        assert_eq!(e_machine, 0x102); // EM_LOONGARCH
    }

    #[test]
    fn loongarch_elf_entry_and_phnum() {
        let elf = link_loongarch_elf(&[0x00, 0x00, 0x00, 0x00], &[]).unwrap();
        let e_entry = u64::from_le_bytes(elf.bytes[24..32].try_into().unwrap());
        assert_eq!(e_entry, 0x120000000);
        let e_phnum = u16::from_le_bytes(elf.bytes[56..58].try_into().unwrap());
        assert_eq!(e_phnum, 2);
    }

    #[test]
    fn loongarch_data_floor() {
        let elf = link_loongarch_elf(&[0x00, 0x00, 0x00, 0x00], &[]).unwrap();
        assert!(elf.bytes.len() > 0x38000);
    }
}