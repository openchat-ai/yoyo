//! Minimal ELF64 Linux x64 linker (PROMPT-v3 Phase 1).
//! Produces a working Linux x64 executable wrapping emitted .text + .data.
//! Data section size floor: 0x38000 (same as PE backend for consistency).
//! Stage 10-B: optional H_00 runtime path (extract embedded .so + trampoline).

use crate::linux_selfhost;
use crate::platform_io;
use crate::types::{IsaError, IsaResult};

/// Minimum data section size (matches PE backend).
const OUTPUT_DATA_NEED: u32 = 0x38000;
const ELF_STARTUP_LEN: u32 = 13;

pub struct ElfImage {
    pub bytes: Vec<u8>,
}

/// Wrap raw x64 code (+ optional data) in an ELF64 executable image.
/// Entry: sets up R15 -> .data (state base), then jumps to `code`.
pub fn link_elf(code: &[u8], data: &[u8]) -> IsaResult<ElfImage> {
    link_elf_impl(code, data)
}

/// Linux link with optional H_00 in-process selfhost (Stage 10-B).
/// When `handler_offsets` contains H_20/H_21, patch H_00 to extract embedded
/// runtime `.so` + trampoline and `execve` (ELF entry stays lea r15 → H_00).
pub fn link_elf_linux(
    code: &[u8],
    data: &[u8],
    handler_offsets: &[(u16, u32, u32)],
) -> IsaResult<ElfImage> {
    if should_h00_selfhost(handler_offsets) {
        let so = linux_selfhost::runtime_so_bytes()?;
        let tramp = linux_selfhost::trampoline_bytes();
        link_elf_h00_runtime(code, data, &so, tramp)
    } else {
        link_elf_impl(code, data)
    }
}

fn should_h00_selfhost(handler_offsets: &[(u16, u32, u32)]) -> bool {
    let has_load = handler_offsets.iter().any(|(h, _, _)| *h == 0x20);
    let has_write = handler_offsets.iter().any(|(h, _, _)| *h == 0x21);
    has_load && has_write
}

/// Stage 10-B: gen1 H_00 pure runtime — patch entry, embed strings + .so + trampoline.
pub fn link_elf_h00_runtime(
    code: &[u8],
    data: &[u8],
    so_bytes: &[u8],
    tramp_bytes: &[u8],
) -> IsaResult<ElfImage> {
    let mut code = code.to_vec();
    if code.len() < 18 {
        return Err(IsaError::PlatformError {
            msg: "H_00 selfhost: code too short for entry patch".into(),
        });
    }

    let with_strings = embed_string_table(data);
    let (extended, meta) =
        linux_selfhost::append_h00_runtime_data(&with_strings, so_bytes, tramp_bytes)?;
    let h00_main = linux_selfhost::gen_h00_selfhost_main(&meta);
    let main_user_off = code.len() as u32;
    code.extend_from_slice(&h00_main);

    // Patch H_00 (user code offset 0): JMP h00_main (not CALL).
    let rel = main_user_off as i32 - 5;
    code[0] = 0xE9;
    code[1..5].copy_from_slice(&rel.to_le_bytes());
    for i in 5..18 {
        code[i] = 0x90;
    }

    link_elf_impl(&code, &extended)
}

/// Embed default selfhost paths at r15+STR_TABLE_OFF (platform_io layout).
fn embed_string_table(user_data: &[u8]) -> Vec<u8> {
    let table_off = platform_io::STR_TABLE_OFF as usize;
    let need = table_off + platform_io::STR_ENTRY_SIZE as usize * 3;
    let mut blob = user_data.to_vec();
    if blob.len() < need {
        blob.resize(need, 0);
    }
    write_cstr_entry(&mut blob, table_off, b"input.tyb");
    write_cstr_entry(
        &mut blob,
        table_off + platform_io::STR_ENTRY_SIZE as usize,
        b"input.ky",
    );
    write_cstr_entry(
        &mut blob,
        table_off + platform_io::STR_ENTRY_SIZE as usize * 2,
        b"output.elf",
    );
    blob
}

fn write_cstr_entry(blob: &mut [u8], base: usize, s: &[u8]) {
    let n = s.len().min(platform_io::STR_ENTRY_SIZE as usize - 1);
    blob[base..base + n].copy_from_slice(&s[..n]);
    blob[base + n] = 0;
}

fn link_elf_impl(code: &[u8], data: &[u8]) -> IsaResult<ElfImage> {
    const ELF_EHDR_SIZE: u32 = 64;
    const ELF_PHDR_SIZE: u32 = 56;
    const PAGE_SIZE: u32 = 0x1000;

    let phdr_count: u16 = 2;

    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, PAGE_SIZE));
    let data_align = align_up(data_need, PAGE_SIZE);

    let hdr_file_size = align_up(ELF_EHDR_SIZE + phdr_count as u32 * ELF_PHDR_SIZE, PAGE_SIZE);
    let text_va = 0x401000u64;
    let text_file_off = hdr_file_size;
    let text_file_size = align_up(code.len() as u32 + ELF_STARTUP_LEN, 0x10);
    let text_mem_size = align_up(text_file_size, PAGE_SIZE);

    // Stage 10-B: .data after .text; file offset MUST be page-aligned and satisfy
    // p_offset ≡ p_vaddr (mod PAGE). Using text_mem_size (not text_file_size) avoids
    // overlapping PT_LOAD file ranges and kernel execve EINVAL / SIGSEGV.
    let data_va = text_va + text_mem_size as u64;
    let data_file_off = text_file_off + text_mem_size;
    let data_file_size = data_align;
    let data_mem_size = data_align;

    let total_file_size = (data_file_off + data_file_size) as usize;

    let mut img = vec![0u8; total_file_size];

    img[0..4].copy_from_slice(b"\x7fELF");
    img[4] = 2;
    img[5] = 1;
    img[6] = 1;

    write_u16(&mut img, 16, 2);
    write_u16(&mut img, 18, 0x3E);
    write_u32(&mut img, 20, 1);
    write_u64(&mut img, 24, text_va);
    write_u64(&mut img, 32, ELF_EHDR_SIZE as u64);
    write_u64(&mut img, 40, 0);
    write_u32(&mut img, 48, 0);
    write_u16(&mut img, 52, ELF_EHDR_SIZE as u16);
    write_u16(&mut img, 54, ELF_PHDR_SIZE as u16);
    write_u16(&mut img, 56, phdr_count);
    write_u16(&mut img, 58, 0);
    write_u16(&mut img, 60, 0);
    write_u16(&mut img, 62, 0);

    let phdr_off = ELF_EHDR_SIZE as usize;
    write_u32(&mut img, phdr_off, 1);
    write_u32(&mut img, phdr_off + 4, 5);
    write_u64(&mut img, phdr_off + 8, text_file_off as u64);
    write_u64(&mut img, phdr_off + 16, text_va);
    write_u64(&mut img, phdr_off + 24, text_va);
    write_u64(&mut img, phdr_off + 32, text_mem_size as u64);
    write_u64(&mut img, phdr_off + 40, text_mem_size as u64);
    write_u64(&mut img, phdr_off + 48, PAGE_SIZE as u64);

    let phdr2_off = phdr_off + ELF_PHDR_SIZE as usize;
    write_u32(&mut img, phdr2_off, 1);
    write_u32(&mut img, phdr2_off + 4, 6);
    write_u64(&mut img, phdr2_off + 8, data_file_off as u64);
    write_u64(&mut img, phdr2_off + 16, data_va);
    write_u64(&mut img, phdr2_off + 24, data_va);
    write_u64(&mut img, phdr2_off + 32, data_file_size as u64);
    write_u64(&mut img, phdr2_off + 40, data_mem_size as u64);
    write_u64(&mut img, phdr2_off + 48, PAGE_SIZE as u64);

    let text_off = text_file_off as usize;
    let lea_disp = data_va as i64 - (text_va as i64 + 7);
    let lea_disp32 = lea_disp as i32;
    img[text_off] = 0x4C;
    img[text_off + 1] = 0x8D;
    img[text_off + 2] = 0x3D;
    img[text_off + 3..text_off + 7].copy_from_slice(&lea_disp32.to_le_bytes());

    let jmp_from = text_va + 7;
    let user_code_va = text_va + ELF_STARTUP_LEN as u64;
    let jmp_rel = user_code_va as i64 - (jmp_from as i64 + 5);
    let jmp_rel32 = jmp_rel as i32;
    img[text_off + 7] = 0xE9;
    img[text_off + 8..text_off + 12].copy_from_slice(&jmp_rel32.to_le_bytes());
    img[text_off + 12] = 0x90;

    let code_dst = text_off + ELF_STARTUP_LEN as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

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
        assert_eq!(elf.bytes[4], 2);
        assert_eq!(elf.bytes[5], 1);
    }

    #[test]
    fn elf_header_fields() {
        let elf = link_elf(&[0xC3], &[]).unwrap();
        let e_type = u16::from_le_bytes(elf.bytes[16..18].try_into().unwrap());
        assert_eq!(e_type, 2);
        let e_machine = u16::from_le_bytes(elf.bytes[18..20].try_into().unwrap());
        assert_eq!(e_machine, 0x3E);
        let e_phnum = u16::from_le_bytes(elf.bytes[56..58].try_into().unwrap());
        assert_eq!(e_phnum, 2);
    }

    #[test]
    fn elf_has_two_phdrs() {
        let elf = link_elf(&[0xC3], &[]).unwrap();
        let phdr_off = 64usize;
        let flags1 = u32::from_le_bytes(elf.bytes[phdr_off + 4..phdr_off + 8].try_into().unwrap());
        assert_eq!(flags1, 5);
        let phdr2_off = phdr_off + 56;
        let flags2 =
            u32::from_le_bytes(elf.bytes[phdr2_off + 4..phdr2_off + 8].try_into().unwrap());
        assert_eq!(flags2, 6);
    }

    #[test]
    fn data_floor_0x38000() {
        let elf = link_elf(&[0xC3], &[]).unwrap();
        assert!(elf.bytes.len() > 0x38000);
    }

    #[test]
    fn entry_points_to_text() {
        let elf = link_elf(&[0xC3], &[]).unwrap();
        let e_entry = u64::from_le_bytes(elf.bytes[24..32].try_into().unwrap());
        assert_eq!(e_entry, 0x401000);
    }

    #[test]
    fn data_va_follows_text() {
        // Large code forces multi-page .text; .data must not overlap.
        let code = vec![0x90u8; 0x4500];
        let elf = link_elf(&code, &[]).unwrap();
        let phdr_off = 64usize;
        let text_memsz =
            u64::from_le_bytes(elf.bytes[phdr_off + 40..phdr_off + 48].try_into().unwrap());
        let data_va =
            u64::from_le_bytes(elf.bytes[phdr_off + 56 + 16..phdr_off + 56 + 24].try_into().unwrap());
        let data_off =
            u64::from_le_bytes(elf.bytes[phdr_off + 56 + 8..phdr_off + 56 + 16].try_into().unwrap());
        assert_eq!(data_va, 0x401000 + text_memsz);
        assert_eq!(data_off % 0x1000, data_va % 0x1000, "PT_LOAD congruence");
    }
}
