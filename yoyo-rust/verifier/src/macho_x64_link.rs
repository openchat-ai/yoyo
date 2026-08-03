//! Mach-O64 x64 (Intel macOS) linker.
//!
//! Produces a little-endian Mach-O 64-bit executable for CPU_TYPE_X86_64
//! with two __PAGEZERO / __TEXT segments.
//!
//! The startup stub (like Linux x64):
//!   lea r15, [rip + disp]   ; r15 = data base
//!   jmp user_code
//!   (padded to 16 bytes)
//!
//! Exit: Darwin x64 syscall — mov rax, 0x20000003; mov edi, code; syscall.
//!
//! Data section size floor: 0x38000.

use crate::types::IsaResult;

const OUTPUT_DATA_NEED: u32 = 0x38000;

const MH_MAGIC_64_LE: u32 = 0xFEFAFED0;
const CPU_TYPE_X86_64: u32 = 0x01000007;
const CPU_SUBTYPE_X86_64_ALL: u32 = 0x03;
const MH_EXECUTE: u32 = 0x02;

const LC_SEGMENT_64: u32 = 0x19;

const VM_PROT_READ: u32 = 1 << 0;
const VM_PROT_WRITE: u32 = 1 << 1;
const VM_PROT_EXECUTE: u32 = 1 << 4;

const PAGE_SIZE: u32 = 0x1000;

pub struct MachO64Image {
    pub bytes: Vec<u8>,
}

pub fn link_macho_x64(code: &[u8], data: &[u8]) -> IsaResult<MachO64Image> {
    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x100, PAGE_SIZE));
    let data_size = align_up(data_need, 0x10);

    let text_va: u64 = 0x100001000;
    let data_va: u64 = 0x100002000;

    // x64 startup: lea r15,[rip+disp] (7) + jmp rel32 (5) + nop pad (4) = 16
    let startup_len = 16u32;
    let code_total = code.len() as u32 + startup_len;
    let code_file_size = align_up(code_total, 0x10);

    let text_seg_va = text_va;
    let text_seg_vmsize = data_va - text_va + data_size as u64;

    let header_size = 32u32;
    let seg_pagezero_size: u32 = 72;
    let seg_text_size: u32 = 72 + 80;
    let lc_total = seg_pagezero_size + seg_text_size;
    let cmd_align = align_up(header_size + lc_total, 8);

    let text_file_off = cmd_align;
    let data_file_off = text_file_off + code_file_size;
    let file_size = (data_file_off as u64) + data_size as u64;

    let mut img = vec![0u8; file_size as usize];

    // ── mach_header_64 ──
    write_u32(&mut img, 0x00, MH_MAGIC_64_LE);
    write_u32(&mut img, 0x04, CPU_TYPE_X86_64);
    write_u32(&mut img, 0x08, CPU_SUBTYPE_X86_64_ALL);
    write_u32(&mut img, 0x0C, MH_EXECUTE);
    write_u32(&mut img, 0x10, 2);
    write_u32(&mut img, 0x14, lc_total);
    write_u32(&mut img, 0x18, 0x02000085);
    write_u32(&mut img, 0x1C, 0);

    let mut lc_off = header_size as usize;

    // ── LC_SEGMENT_64: __PAGEZERO ──
    write_u32(&mut img, lc_off, LC_SEGMENT_64);
    write_u32(&mut img, lc_off + 4, seg_pagezero_size);
    copy_bytes(&mut img, lc_off + 8, b"__PAGEZERO");
    write_u64(&mut img, lc_off + 24, 0);
    write_u64(&mut img, lc_off + 32, 0x1000);
    write_u64(&mut img, lc_off + 40, 0);
    write_u64(&mut img, lc_off + 48, 0);
    write_u32(&mut img, lc_off + 56, 0);
    write_u32(&mut img, lc_off + 60, 0);
    write_u32(&mut img, lc_off + 64, 0);
    write_u32(&mut img, lc_off + 68, 0);

    lc_off += seg_pagezero_size as usize;

    // ── LC_SEGMENT_64: __TEXT ──
    write_u32(&mut img, lc_off, LC_SEGMENT_64);
    write_u32(&mut img, lc_off + 4, seg_text_size);
    copy_bytes(&mut img, lc_off + 8, b"__TEXT");
    write_u64(&mut img, lc_off + 24, text_seg_va);
    write_u64(&mut img, lc_off + 32, text_seg_vmsize);
    write_u64(&mut img, lc_off + 40, text_file_off as u64);
    write_u64(&mut img, lc_off + 48, (code_file_size + data_size) as u64);
    write_u32(&mut img, lc_off + 56, VM_PROT_READ | VM_PROT_EXECUTE);
    write_u32(&mut img, lc_off + 60, VM_PROT_READ | VM_PROT_EXECUTE);
    write_u32(&mut img, lc_off + 64, 1);
    write_u32(&mut img, lc_off + 68, 0);

    let sect_off = lc_off + 72;
    copy_bytes(&mut img, sect_off, b"__text");
    copy_bytes(&mut img, sect_off + 16, b"__TEXT");
    write_u64(&mut img, sect_off + 32, text_va);
    write_u64(&mut img, sect_off + 40, code_file_size as u64);
    write_u32(&mut img, sect_off + 48, 0);
    write_u32(&mut img, sect_off + 52, 4);
    write_u32(&mut img, sect_off + 56, 0);
    write_u32(&mut img, sect_off + 60, 0);
    write_u32(&mut img, sect_off + 64, 0x80000400);
    write_u32(&mut img, sect_off + 68, 0);
    write_u32(&mut img, sect_off + 72, 0);
    write_u32(&mut img, sect_off + 76, 0);

    // ── Startup stub at text_file_off ──
    let user_code_va = text_va + startup_len as u64;
    let stub_off = text_file_off as usize;

    // lea r15, [rip + disp32]
    // REX.WR = 0x4C; ModRM r15 [rip+disp] = 0x3D
    let lea_disp = data_va as i64 - (text_va as i64 + 7);
    let lea_disp32 = lea_disp as i32;
    img[stub_off] = 0x4C;
    img[stub_off + 1] = 0x8D;
    img[stub_off + 2] = 0x3D;
    img[stub_off + 3..stub_off + 7].copy_from_slice(&lea_disp32.to_le_bytes());

    // jmp rel32 to user code
    let jmp_from = text_va + 7;
    let jmp_rel = user_code_va as i64 - (jmp_from as i64 + 5);
    let jmp_rel32 = jmp_rel as i32;
    img[stub_off + 7] = 0xE9;
    img[stub_off + 8..stub_off + 12].copy_from_slice(&jmp_rel32.to_le_bytes());
    // nop pad to 16
    for i in 12..16 {
        img[stub_off + i] = 0x90;
    }

    // Copy user code
    let code_dst = stub_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // Copy data
    let data_dst = data_file_off as usize;
    let copy_n = data.len().min(data_size as usize);
    img[data_dst..data_dst + copy_n].copy_from_slice(&data[..copy_n]);

    Ok(MachO64Image { bytes: img })
}

fn align_up(v: u32, a: u32) -> u32 {
    (v + a - 1) & !(a - 1)
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
    fn macho_x64_magic_and_cpu() {
        let img = link_macho_x64(&[0xC3], &[]).unwrap();
        let magic = u32::from_le_bytes(img.bytes[0..4].try_into().unwrap());
        assert_eq!(magic, MH_MAGIC_64_LE);
        let cpu = u32::from_le_bytes(img.bytes[4..8].try_into().unwrap());
        assert_eq!(cpu, CPU_TYPE_X86_64);
        let subtype = u32::from_le_bytes(img.bytes[8..12].try_into().unwrap());
        assert_eq!(subtype, CPU_SUBTYPE_X86_64_ALL);
    }

    #[test]
    fn macho_x64_two_load_commands() {
        let img = link_macho_x64(&[0xC3], &[]).unwrap();
        let ncmds = u32::from_le_bytes(img.bytes[16..20].try_into().unwrap());
        assert_eq!(ncmds, 2);
    }

    #[test]
    fn macho_x64_data_floor() {
        let img = link_macho_x64(&[0xC3], &[]).unwrap();
        assert!(img.bytes.len() > 0x38000);
    }

    #[test]
    fn macho_x64_startup_lea_r15() {
        let img = link_macho_x64(&[0xC3], &[]).unwrap();
        // text_file_off = 0x100 (aligned)
        let text_off = 0x100usize;
        // First byte should be REX.WR = 0x4C
        assert_eq!(img.bytes[text_off], 0x4C);
        assert_eq!(img.bytes[text_off + 1], 0x8D);
        assert_eq!(img.bytes[text_off + 2], 0x3D); // ModRM
        // JMP at +7
        assert_eq!(img.bytes[text_off + 7], 0xE9);
    }
}
