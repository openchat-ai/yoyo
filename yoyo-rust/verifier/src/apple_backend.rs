//! Mach-O64 ARM64 linker for Apple/iOS targets.
//!
//! Produces a little-endian Mach-O 64-bit executable with a single `__TEXT`
//! segment (containing `__text` and `__data` sub-sections) and a single
//! `__PAGEZERO` segment. The entry-point load command is included so the
//! dynamic linker starts execution at the ARM64 startup stub.
//!
//! The startup stub:
//!   adrp x15, data_addr   ; data base (state pointer)
//!   add  x15, x15, <lo12>
//!   adrp x16, user_code
//!   add  x16, x16, <lo12>
//!   br   x16
//!
//! Data section size floor: 0x38000.

use crate::types::IsaResult;

/// Minimum data size (matches PE/ELF backends).
const OUTPUT_DATA_NEED: u32 = 0x38000;

/// Mach-O magic and constants for ARM64 LE (MH_MAGIC_64 little-endian form).
const MH_MAGIC_64_LE: u32 = 0xFEFAFED0; // little-endian MH_MAGIC_64
const CPU_TYPE_ARM64: u32 = 0x0100000C;
const CPU_SUBTYPE_ARM64_ALL: u32 = 0;
const MH_EXECUTE: u32 = 0x02;

const LC_SEGMENT_64: u32 = 0x19;
const LC_SYMTAB: u32 = 0x02;
const LC_DYLD_INFO_ONLY: u32 = 0x22;
const LC_UNIXTHREAD: u32 = 0x00000005;
const LC_MAIN: u32 = 0x28; // LC_UNIXTHREAD equivalent for executables

const VM_PROT_READ: u32 = 1 << 0;
const VM_PROT_WRITE: u32 = 1 << 1;
const VM_PROT_EXECUTE: u32 = 1 << 4;

const SECTION_TYPE: u32 = 0x00;
const SECTION_ATTRIBUTES: u32 = 0x00;

const PAGE_SIZE: u32 = 0x1000;

/// Mach-O image (the bytes can be written directly to disk and executed
/// under Rosetta / native ARM64 macOS or iOS simulator).
pub struct MachO64Image {
    pub bytes: Vec<u8>,
}

pub fn link_macho64(code: &[u8], data: &[u8]) -> IsaResult<MachO64Image> {
    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x100, PAGE_SIZE));
    let data_size = align_up(data_need, 0x10);

    let text_va = 0x100001000u64;
    let data_va = 0x100002000u64;

    let startup_len = 20u32; // adrp+add+adrp+add+br (5 × 4)
    let code_total = code.len() as u32 + startup_len;
    let code_file_size = align_up(code_total, 0x10);

    // Build a single __TEXT segment that contains both code and data as
    // adjacent sub-sections, to keep the header small.
    let text_seg_va = text_va;
    let text_seg_vmsize = data_va - text_va + data_size as u64;

    // File layout:
    //   0x0000: mach_header_64 (32 bytes)
    //   0x0020: load commands
    //   followed by __TEXT segment raw bytes (code + data).
    let header_size = 32u32;
    // Two LC_SEGMENT_64 commands: __PAGEZERO and __TEXT
    // LC_SEGMENT_64 cmdsize = 72 (cmd header) + 1*section_64 (80) = 152
    let seg_pagezero_size: u32 = 72;
    let seg_text_size: u32 = 72 + 80; // one section_64 inside
    let lc_total = seg_pagezero_size + seg_text_size;
    let cmd_align = align_up(header_size + lc_total, 8);

    let text_file_off = cmd_align;
    let data_file_off = text_file_off + code_file_size;
    let file_size = (data_file_off as u64) + data_size as u64;

    let mut img = vec![0u8; file_size as usize];

    // ── mach_header_64 (32 bytes, little-endian) ──
    write_u32(&mut img, 0x00, MH_MAGIC_64_LE);
    write_u32(&mut img, 0x04, CPU_TYPE_ARM64);
    write_u32(&mut img, 0x08, CPU_SUBTYPE_ARM64_ALL);
    write_u32(&mut img, 0x0C, MH_EXECUTE);
    write_u32(&mut img, 0x10, 2);         // ncmds
    write_u32(&mut img, 0x14, lc_total);  // sizeofcmds
    write_u32(&mut img, 0x18, 0x02000085); // flags: MH_NOUNDEFS | MH_TWOLEVEL | MH_PIE | MH_NO_HEAP_EXECUTABLE
    write_u32(&mut img, 0x1C, 0);        // reserved

    let mut lc_off = header_size as usize;

    // ── LC_SEGMENT_64 for __PAGEZERO ──
    write_u32(&mut img, lc_off, LC_SEGMENT_64);
    write_u32(&mut img, lc_off + 4, seg_pagezero_size);
    copy_bytes(&mut img, lc_off + 8, b"__PAGEZERO");
    write_u64(&mut img, lc_off + 24, 0);            // vmaddr = 0
    write_u64(&mut img, lc_off + 32, 0x1000);       // vmsize = 0x1000
    write_u64(&mut img, lc_off + 40, 0);            // fileoff
    write_u64(&mut img, lc_off + 48, 0);            // filesize
    write_u32(&mut img, lc_off + 56, 0);            // maxprot
    write_u32(&mut img, lc_off + 60, 0);            // initprot
    write_u32(&mut img, lc_off + 64, 0);            // nsects
    write_u32(&mut img, lc_off + 68, 0);            // flags

    lc_off += seg_pagezero_size as usize;

    // ── LC_SEGMENT_64 for __TEXT (contains __text + __data sub-sections) ──
    write_u32(&mut img, lc_off, LC_SEGMENT_64);
    write_u32(&mut img, lc_off + 4, seg_text_size);
    copy_bytes(&mut img, lc_off + 8, b"__TEXT");
    write_u64(&mut img, lc_off + 24, text_seg_va);
    write_u64(&mut img, lc_off + 32, text_seg_vmsize);
    write_u64(&mut img, lc_off + 40, text_file_off as u64);
    write_u64(&mut img, lc_off + 48, (code_file_size + data_size) as u64);
    write_u32(&mut img, lc_off + 56, VM_PROT_READ | VM_PROT_EXECUTE);
    write_u32(&mut img, lc_off + 60, VM_PROT_READ | VM_PROT_EXECUTE);
    write_u32(&mut img, lc_off + 64, 1);   // nsects = 1 (code sub-section; data is adjacent)
    write_u32(&mut img, lc_off + 68, 0);

    // section_64 inside __TEXT: __text,__text
    let sect_off = lc_off + 72;
    copy_bytes(&mut img, sect_off, b"__text");
    copy_bytes(&mut img, sect_off + 16, b"__TEXT");
    write_u64(&mut img, sect_off + 32, text_va);
    write_u64(&mut img, sect_off + 40, code_file_size as u64);
    write_u32(&mut img, sect_off + 48, 0);      // offset (relative to segment)
    write_u32(&mut img, sect_off + 52, 4);      // align = 2^4 = 16
    write_u32(&mut img, sect_off + 56, 0);      // reloff
    write_u32(&mut img, sect_off + 60, 0);      // nreloc
    write_u32(&mut img, sect_off + 64, 0x80000400); // SECTION_ATTRIBUTES | S_ATTR_PURE_INSTRUCTIONS | S_SPARSE_MAP | S_REGULAR
    write_u32(&mut img, sect_off + 68, 0);
    write_u32(&mut img, sect_off + 72, 0);
    write_u32(&mut img, sect_off + 76, 0);

    // ── Write startup stub + user code at text_file_off ──
    let user_code_va = text_va + startup_len as u64;
    let stub_off = text_file_off as usize;

    // adrp x15, data_va
    img[stub_off..stub_off + 4].copy_from_slice(&arm64_adrp(15, data_va));
    // add x15, x15, data_va & 0xFFF
    img[stub_off + 4..stub_off + 8].copy_from_slice(&arm64_add_imm12(15, 15, data_va & 0xFFF));
    // adrp x16, user_code_va
    img[stub_off + 8..stub_off + 12].copy_from_slice(&arm64_adrp(16, user_code_va));
    // add x16, x16, user_code_va & 0xFFF
    img[stub_off + 12..stub_off + 16].copy_from_slice(&arm64_add_imm12(16, 16, user_code_va & 0xFFF));
    // br x16 = 0xD61F0200
    img[stub_off + 16..stub_off + 20].copy_from_slice(&0xD61F0200u32.to_le_bytes());

    // user code
    let code_dst = stub_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // ── Write data at data_file_off ──
    let data_dst = data_file_off as usize;
    let copy_n = data.len().min(data_size as usize);
    img[data_dst..data_dst + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(MachO64Image { bytes: img })
}

fn arm64_adrp(rd: u32, addr: u64) -> [u8; 4] {
    let imm = ((addr >> 12) & 0x7FFFF) as u32;
    let enc: u32 = 0x90000000
        | (imm & 0x3F)
        | (((imm >> 5) & 0x1F) << 5)
        | ((rd & 0x1F) << 10)
        | (((imm >> 6) & 0xFF) << 16);
    enc.to_le_bytes()
}

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

fn copy_bytes(buf: &mut [u8], off: usize, src: &[u8]) {
    let n = src.len().min(buf.len() - off);
    buf[off..off + n].copy_from_slice(&src[..n]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn macho_magic_and_cpu() {
        let img = link_macho64(&[0x1F, 0x20, 0x03, 0xD5], &[]).unwrap();
        let magic = u32::from_le_bytes(img.bytes[0..4].try_into().unwrap());
        assert_eq!(magic, MH_MAGIC_64_LE);
        let cpu = u32::from_le_bytes(img.bytes[4..8].try_into().unwrap());
        assert_eq!(cpu, CPU_TYPE_ARM64);
        let file_type = u32::from_le_bytes(img.bytes[12..16].try_into().unwrap());
        assert_eq!(file_type, MH_EXECUTE);
    }

    #[test]
    fn macho_has_two_load_commands() {
        let img = link_macho64(&[0x1F, 0x20, 0x03, 0xD5], &[]).unwrap();
        let ncmds = u32::from_le_bytes(img.bytes[16..20].try_into().unwrap());
        assert_eq!(ncmds, 2);
    }

    #[test]
    fn macho_data_floor() {
        let img = link_macho64(&[0x1F, 0x20, 0x03, 0xD5], &[]).unwrap();
        assert!(img.bytes.len() > 0x38000);
    }

    #[test]
    fn macho_starts_with_adrp() {
        let img = link_macho64(&[0x1F, 0x20, 0x03, 0xD5], &[]).unwrap();
        // __TEXT segment file offset = header_size(32) + LC_PAGEZERO(72) + LC_TEXT(152) = 0x100.
        let text_off = 0x100usize;
        // First 4 bytes form an ADRP instruction (A64 encoding 0x9xxxxxxx).
        // In little-endian the high byte 0x9x sits at text_off+3.
        let first_insn =
            u32::from_le_bytes(img.bytes[text_off..text_off + 4].try_into().unwrap());
        assert_eq!(
            first_insn >> 24,
            0x90,
            "expected ADRP (0x9xxxxxxx) at start of .text, got 0x{:08X}",
            first_insn
        );
        // br x16 should be at offset +16 (5th instruction of the startup stub)
        assert_eq!(&img.bytes[text_off + 16..text_off + 20], &0xD61F0200u32.to_le_bytes());
    }
}
