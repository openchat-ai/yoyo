//! Minimal ELF64 Linux x64 linker (PROMPT-v3 Phase 1).
//! Produces a working Linux x64 executable wrapping emitted .text + .data.
//! Data section size floor: 0x38000 (same as PE backend for consistency).

use crate::types::IsaResult;

/// Minimum data section size (matches PE backend).
const OUTPUT_DATA_NEED: u32 = 0x38000;

pub struct ElfImage {
    pub bytes: Vec<u8>,
}

/// Wrap raw x64 code (+ optional data) in an ELF64 executable image.
/// Entry: sets up R15 -> .data (state base), then jumps to `code`.
pub fn link_elf(code: &[u8], data: &[u8]) -> IsaResult<ElfImage> {
    // ELF64 constants
    const ELF_EHDR_SIZE: u32 = 64; // ELF header size
    const ELF_PHDR_SIZE: u32 = 56; // Program header entry size
    const PAGE_SIZE: u32 = 0x1000;

    // Two PT_LOAD segments: .text and .data
    let phdr_count: u16 = 2;

    // Compute sizes
    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, PAGE_SIZE));
    let data_align = align_up(data_need, PAGE_SIZE);

    // Layout (virtual addresses, base at 0x400000):
    //   ELF header + PHDRs:  file offset 0, VA 0x400000
    //   .text:               file offset page_align(hdr_end), VA 0x401000
    //   .data:               file offset page_align(text_end), VA 0x402000

    let hdr_file_size = align_up(ELF_EHDR_SIZE + phdr_count as u32 * ELF_PHDR_SIZE, PAGE_SIZE);
    let text_va = 0x401000u64;
    let text_file_off = hdr_file_size;
    let text_file_size = align_up(code.len() as u32 + 13, 0x10);
    let text_mem_size = align_up(text_file_size, PAGE_SIZE);

    let data_va = 0x402000u64;
    let data_file_off = text_file_off + text_file_size;
    let data_file_size = data_align;
    let data_mem_size = data_align;

    let total_file_size = (data_file_off + data_file_size) as usize;

    let mut img = vec![0u8; total_file_size];

    // ── ELF Header (offset 0) ──
    // e_ident
    img[0..4].copy_from_slice(b"\x7fELF"); // ELF magic
    img[4] = 2; // ELFCLASS64
    img[5] = 1; // ELFDATA2LSB
    img[6] = 1; // EV_CURRENT
    // bytes 7-15 are zero (padding)

    write_u16(&mut img, 16, 2); // e_type = ET_EXEC
    write_u16(&mut img, 18, 0x3E); // e_machine = EM_X86_64
    write_u32(&mut img, 20, 1); // e_version

    // e_entry = text_va (startup code at start of .text)
    write_u64(&mut img, 24, text_va);

    // e_phoff = program header offset (right after ELF header)
    write_u64(&mut img, 32, ELF_EHDR_SIZE as u64);

    // e_shoff = 0 (no section headers)
    write_u64(&mut img, 40, 0);

    write_u32(&mut img, 48, 0); // e_flags
    write_u16(&mut img, 52, ELF_EHDR_SIZE as u16); // e_ehsize
    write_u16(&mut img, 54, ELF_PHDR_SIZE as u16); // e_phentsize
    write_u16(&mut img, 56, phdr_count); // e_phnum
    write_u16(&mut img, 58, 0); // e_shentsize
    write_u16(&mut img, 60, 0); // e_shnum
    write_u16(&mut img, 62, 0); // e_shstrndx

    // ── Program Headers (offset 64) ──
    // PHDR 0: .text (PT_LOAD)
    let phdr_off = ELF_EHDR_SIZE as usize;
    write_u32(&mut img, phdr_off, 1); // p_type = PT_LOAD
    write_u32(&mut img, phdr_off + 4, 5); // p_flags = PF_R | PF_X
    write_u64(&mut img, phdr_off + 8, text_file_off as u64); // p_offset
    write_u64(&mut img, phdr_off + 16, text_va); // p_vaddr
    write_u64(&mut img, phdr_off + 24, text_va); // p_paddr (same as vaddr)
    write_u64(&mut img, phdr_off + 32, text_mem_size as u64); // p_filesz (in memory)
    write_u64(&mut img, phdr_off + 40, text_mem_size as u64); // p_memsz
    write_u64(&mut img, phdr_off + 48, PAGE_SIZE as u64); // p_align

    // PHDR 1: .data (PT_LOAD)
    let phdr2_off = phdr_off + ELF_PHDR_SIZE as usize;
    write_u32(&mut img, phdr2_off, 1); // p_type = PT_LOAD
    write_u32(&mut img, phdr2_off + 4, 6); // p_flags = PF_R | PF_W
    write_u64(&mut img, phdr2_off + 8, data_file_off as u64); // p_offset
    write_u64(&mut img, phdr2_off + 16, data_va); // p_vaddr
    write_u64(&mut img, phdr2_off + 24, data_va); // p_paddr
    write_u64(&mut img, phdr2_off + 32, data_file_size as u64); // p_filesz
    write_u64(&mut img, phdr2_off + 40, data_mem_size as u64); // p_memsz
    write_u64(&mut img, phdr2_off + 48, PAGE_SIZE as u64); // p_align

    // ── Build startup at start of .text ──
    //   lea r15, [rip + disp]  ; r15 = data base (state)
    //   jmp user_code
    let text_off = text_file_off as usize;
    let startup_len = 13u32; // lea r15, [rip+d] (7) + jmp rel32 (5) + align nop

    // lea r15, [rip + disp32]
    // After this 7-byte insn, RIP = text_va + 7
    // Want r15 = data_va -> disp = data_va - (text_va + 7)
    let lea_disp = data_va as i64 - (text_va as i64 + 7);
    let lea_disp32 = lea_disp as i32;
    img[text_off] = 0x4C; // REX.WR
    img[text_off + 1] = 0x8D;
    img[text_off + 2] = 0x3D; // ModRM: r15, [rip+disp]
    img[text_off + 3..text_off + 7].copy_from_slice(&lea_disp32.to_le_bytes());

    // jmp rel32 to user code (right after startup)
    let jmp_from = text_va + 7;
    let user_code_va = text_va + startup_len as u64;
    let jmp_rel = user_code_va as i64 - (jmp_from as i64 + 5);
    let jmp_rel32 = jmp_rel as i32;
    img[text_off + 7] = 0xE9;
    img[text_off + 8..text_off + 12].copy_from_slice(&jmp_rel32.to_le_bytes());
    img[text_off + 12] = 0x90; // nop pad -> startup_len=13

    // Copy user code
    let code_dst = text_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // Copy data
    let data_off = data_file_off as usize;
    let copy_n = data.len().min(data_file_size as usize);
    img[data_off..data_off + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(ElfImage { bytes: img })
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
    fn elf_has_magic() {
        let elf = link_elf(&[0xC3], &[]).unwrap();
        assert_eq!(&elf.bytes[0..4], b"\x7fELF");
        assert_eq!(elf.bytes[4], 2); // ELFCLASS64
        assert_eq!(elf.bytes[5], 1); // ELFDATA2LSB
    }

    #[test]
    fn elf_header_fields() {
        let elf = link_elf(&[0xC3], &[]).unwrap();
        let e_type = u16::from_le_bytes(elf.bytes[16..18].try_into().unwrap());
        assert_eq!(e_type, 2); // ET_EXEC
        let e_machine = u16::from_le_bytes(elf.bytes[18..20].try_into().unwrap());
        assert_eq!(e_machine, 0x3E); // EM_X86_64
        let e_phnum = u16::from_le_bytes(elf.bytes[56..58].try_into().unwrap());
        assert_eq!(e_phnum, 2); // 2 program headers
    }

    #[test]
    fn elf_has_two_phdrs() {
        let elf = link_elf(&[0xC3], &[]).unwrap();
        let phdr_off = 64usize;
        // First PHDR: .text, flags = PF_R | PF_X (5)
        let flags1 = u32::from_le_bytes(elf.bytes[phdr_off + 4..phdr_off + 8].try_into().unwrap());
        assert_eq!(flags1, 5);
        // Second PHDR: .data, flags = PF_R | PF_W (6)
        let phdr2_off = phdr_off + 56;
        let flags2 = u32::from_le_bytes(elf.bytes[phdr2_off + 4..phdr2_off + 8].try_into().unwrap());
        assert_eq!(flags2, 6);
    }

    #[test]
    fn data_floor_0x38000() {
        let elf = link_elf(&[0xC3], &[]).unwrap();
        // file should be large enough to hold data section raw size
        assert!(elf.bytes.len() > 0x38000);
    }

    #[test]
    fn entry_points_to_text() {
        let elf = link_elf(&[0xC3], &[]).unwrap();
        let e_entry = u64::from_le_bytes(elf.bytes[24..32].try_into().unwrap());
        assert_eq!(e_entry, 0x401000);
    }
}