//! Minimal MIPS big-endian ELF32 linker (PROMPT-v3 Phase 1).
//!
//! Produces a working MIPS BE Linux ELF32 executable wrapping emitted
//! .text + .data. Big-endian byte order throughout (e_ident[5]=2).
//!
//! The startup stub at the start of .text uses `lui` + `ori` to set up
//! t8 (r24) pointed at the .data base (state pointer), then `j`s into
//! user code. All multi-byte constants are big-endian.
//!
//! Data section size floor: 0x38000 (same as other backends).

use crate::types::IsaResult;

const OUTPUT_DATA_NEED: u32 = 0x38000;
const PAGE_SIZE: u32 = 0x1000;
// ELF32 header = 52, program header entry = 32.
const ELF_EHDR_SIZE: u32 = 52;
const ELF_PHDR_SIZE: u32 = 32;

pub struct ElfMipsImage {
    pub bytes: Vec<u8>,
}

/// Wrap raw code (+ optional data) in a MIPS BE ELF32 executable.
pub fn link_mips_elf(code: &[u8], data: &[u8]) -> IsaResult<ElfMipsImage> {
    let phdr_count: u16 = 2;

    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, PAGE_SIZE));
    let data_align = align_up(data_need, PAGE_SIZE);

    // Virtual layout:
    //   .text @ 0x4001000, .data @ 0x4002000
    let text_va = 0x4001000u32;
    let data_va = 0x4002000u32;

    let hdr_file_size = align_up(ELF_EHDR_SIZE + phdr_count as u32 * ELF_PHDR_SIZE, PAGE_SIZE);
    let text_file_off = hdr_file_size as u32;
    // 4 × 4-byte MIPS insns: lui t8, hi(data_va); ori t8, t8, lo(data_va);
    // j <user_code>; nop
    let startup_len = 16u32;
    let text_file_size = align_up(code.len() as u32 + startup_len, PAGE_SIZE);
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
    write_u16_be(&mut img, 18, 0x08);            // EM_MIPS
    write_u32_be(&mut img, 20, 1);               // e_version
    write_u32_be(&mut img, 24, text_va);         // e_entry
    write_u32_be(&mut img, 28, ELF_EHDR_SIZE);   // e_phoff
    write_u32_be(&mut img, 36, 0);               // e_flags (MIPS specific flags at byte 36+)
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
    write_u32_be(&mut img, phdr_off + 24, 0x00000005); // p_flags = PF_R | PF_X (BE)
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

    // ── Startup stub at start of .text ──
    //   lui t8, hi16(data_va)        ; load upper 16 bits of data VA
    //   ori t8, t8, lo16(data_va)   ; load lower 16 bits
    //   j <user_code_offset>        ; jump to user code
    //   nop                          ; delay slot
    let text_off = text_file_off as usize;
    let user_code_va = text_va + startup_len;

    // lui t8, hi16(data_va) — 0x3C180000 | (hi16 << 16)
    let data_hi = (data_va >> 16) & 0xFFFF;
    let lui_enc: u32 = 0x3C180000 | (data_hi << 16);
    img[text_off..text_off + 4].copy_from_slice(&lui_enc.to_be_bytes());

    // ori t8, t8, lo16(data_va) — 0x37180000 | lo16
    let data_lo = data_va & 0xFFFF;
    let ori_enc: u32 = 0x37180000 | data_lo;
    img[text_off + 4..text_off + 8].copy_from_slice(&ori_enc.to_be_bytes());

    // j <user_code> — 0x08000000 | (user_code_va >> 2)
    // MIPS j is region-relative (top 4 bits of PC), but user_code is in same region
    let j_enc: u32 = 0x08000000 | ((user_code_va >> 2) & 0x3FFFFFF);
    img[text_off + 8..text_off + 12].copy_from_slice(&j_enc.to_be_bytes());

    // nop (delay slot)
    img[text_off + 12..text_off + 16].copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    // Copy user code
    let code_dst = text_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // ── Copy data ──
    let data_off = data_file_off as usize;
    let copy_n = data.len().min(data_file_size as usize);
    img[data_off..data_off + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(ElfMipsImage { bytes: img })
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
    fn mips_elf_has_magic() {
        let elf = link_mips_elf(&[0x00, 0x00, 0x00, 0x00], &[]).unwrap();
        assert_eq!(&elf.bytes[0..4], b"\x7fELF");
        assert_eq!(elf.bytes[4], 1); // ELFCLASS32
        assert_eq!(elf.bytes[5], 2); // ELFDATA2MSB
    }

    #[test]
    fn mips_elf_machine_and_entry() {
        let elf = link_mips_elf(&[0x00, 0x00, 0x00, 0x00], &[]).unwrap();
        let e_machine = u16::from_be_bytes(elf.bytes[18..20].try_into().unwrap());
        assert_eq!(e_machine, 0x08); // EM_MIPS
        let e_entry = u32::from_be_bytes(elf.bytes[24..28].try_into().unwrap());
        assert_eq!(e_entry, 0x4001000);
        let e_phnum = u16::from_be_bytes(elf.bytes[44..46].try_into().unwrap());
        assert_eq!(e_phnum, 2);
    }

    #[test]
    fn mips_data_floor() {
        let elf = link_mips_elf(&[0x00, 0x00, 0x00, 0x00], &[]).unwrap();
        assert!(elf.bytes.len() > 0x38000);
    }
}
