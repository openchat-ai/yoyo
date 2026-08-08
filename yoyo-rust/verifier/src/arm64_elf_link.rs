//! Minimal ARM64 (aarch64) ELF64 linker (PROMPT-v3 Android backend).
//!
//! Produces an ARM64 Linux ELF64 executable wrapping emitted .text + .data.
//! The startup stub at the start of .text uses ADRP+ADD pairs to set up
//! x15 → .data base (state pointer) and branches into user code.
//!
//! Data section size floor: 0x38000 (same as PE/ELF x64 backends).

use crate::types::IsaResult;

const OUTPUT_DATA_NEED: u32 = 0x38000;

pub struct ElfArm64Image {
    pub bytes: Vec<u8>,
}

/// Wrap raw ARM64 code (+ optional data) in an ELF64 ARM64 executable.
pub fn link_arm64_elf(code: &[u8], data: &[u8]) -> IsaResult<ElfArm64Image> {
    const ELF_EHDR_SIZE: u32 = 64;
    const ELF_PHDR_SIZE: u32 = 56;
    const PAGE_SIZE: u32 = 0x1000;

    let phdr_count: u16 = 2;

    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, PAGE_SIZE));
    let data_align = align_up(data_need, PAGE_SIZE);

    // Virtual layout (base 0x400000):
    //   .text @ text_va, .data @ data_va
    let text_va = 0x4001000u64;
    let data_va = 0x4002000u64;

    let hdr_file_size = align_up(ELF_EHDR_SIZE + phdr_count as u32 * ELF_PHDR_SIZE, PAGE_SIZE);
    let text_file_off = hdr_file_size as u64;
    // 5 × 4-byte ARM64 instructions: adrp x15 + add x15 (data base),
    // adrp x16 + add x16 (user code VA), br x16.
    let startup_len = 20u32;
    let text_file_size = align_up(code.len() as u32 + startup_len, PAGE_SIZE) as u64;
    let text_mem_size = align_up(text_file_size as u32, PAGE_SIZE) as u64;

    let data_file_off = text_file_off + text_file_size as u64;
    let data_file_size = data_align as u64;
    let data_mem_size = data_align as u64;

    let total_file_size = (data_file_off + data_file_size) as usize;
    let mut img = vec![0u8; total_file_size];

    // ── ELF Header ──
    img[0..4].copy_from_slice(b"\x7fELF");
    img[4] = 2;  // ELFCLASS64
    img[5] = 1;  // ELFDATA2LSB
    img[6] = 1;  // EV_CURRENT

    write_u16(&mut img, 16, 2);              // ET_EXEC
    write_u16(&mut img, 18, 0xB7);           // EM_AARCH64
    write_u32(&mut img, 20, 1);              // e_version
    write_u64(&mut img, 24, text_va);        // e_entry
    write_u64(&mut img, 32, ELF_EHDR_SIZE as u64); // e_phoff
    write_u32(&mut img, 48, 0);              // e_flags
    write_u16(&mut img, 52, ELF_EHDR_SIZE as u16); // e_ehsize
    write_u16(&mut img, 54, ELF_PHDR_SIZE as u16); // e_phentsize
    write_u16(&mut img, 56, phdr_count);     // e_phnum

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

    // ── Startup stub at start of .text ──
    //   adrp x15, data_va          ; page-aligned data base
    //   add  x15, x15, data_va&0xFFF
    //   adrp x16, user_code_va     ; page-aligned user code VA
    //   add  x16, x16, user_code_va&0xFFF
    //   br   x16
    let text_off = text_file_off as usize;
    let user_code_va = text_va + startup_len as u64;

    img[text_off..text_off + 4].copy_from_slice(&arm64_adrp(15, data_va));
    img[text_off + 4..text_off + 8].copy_from_slice(&arm64_add_imm12(15, 15, data_va & 0xFFF));
    img[text_off + 8..text_off + 12].copy_from_slice(&arm64_adrp(16, user_code_va));
    img[text_off + 12..text_off + 16].copy_from_slice(&arm64_add_imm12(16, 16, user_code_va & 0xFFF));
    // br x16 = 0xD61F0200
    img[text_off + 16..text_off + 20].copy_from_slice(&0xD61F0200u32.to_le_bytes());

    // Copy user code
    let code_dst = text_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // Copy data
    let data_off = data_file_off as usize;
    let copy_n = data.len().min(data_file_size as usize);
    img[data_off..data_off + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(ElfArm64Image { bytes: img })
}

/// `adrp rd, <addr>` — PC-relative page number load (absolute, PC assumed 0).
/// imm = ((addr >> 12) & 0x7FFFF) (21-bit signed).
fn arm64_adrp(rd: u32, addr: u64) -> [u8; 4] {
    let imm = ((addr >> 12) & 0x7FFFF) as u32;
    let enc: u32 = 0x90000000
        | (imm & 0x3F)
        | (((imm >> 5) & 0x1F) << 5)
        | ((rd & 0x1F) << 10)
        | (((imm >> 6) & 0xFF) << 16);
    enc.to_le_bytes()
}

/// `add rd, rn, imm12` (signed 12-bit).
fn arm64_add_imm12(rd: u32, rn: u32, imm: u64) -> [u8; 4] {
    let imm12 = (imm & 0xFFF) as u32;
    let enc: u32 = 0x91000000
        | (imm12 << 10)
        | ((rn & 0x1F) << 5)
        | (rd & 0x1F);
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
    fn arm64_elf_has_magic() {
        let elf = link_arm64_elf(&[0x1F, 0x20, 0x03, 0xD5], &[]).unwrap();
        assert_eq!(&elf.bytes[0..4], b"\x7fELF");
        assert_eq!(elf.bytes[4], 2);
        assert_eq!(elf.bytes[5], 1);
        let e_machine = u16::from_le_bytes(elf.bytes[18..20].try_into().unwrap());
        assert_eq!(e_machine, 0xB7); // EM_AARCH64
    }

    #[test]
    fn arm64_elf_entry_and_phnum() {
        let elf = link_arm64_elf(&[0x1F, 0x20, 0x03, 0xD5], &[]).unwrap();
        let e_entry = u64::from_le_bytes(elf.bytes[24..32].try_into().unwrap());
        assert_eq!(e_entry, 0x4001000);
        let e_phnum = u16::from_le_bytes(elf.bytes[56..58].try_into().unwrap());
        assert_eq!(e_phnum, 2);
    }

    #[test]
    fn arm64_data_floor_0x38000() {
        let elf = link_arm64_elf(&[0x1F, 0x20, 0x03, 0xD5], &[]).unwrap();
        assert!(elf.bytes.len() > 0x38000);
    }
}
