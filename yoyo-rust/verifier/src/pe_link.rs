//! Minimal PE32+ linker (PROMPT-v3 Phase 1).
//! Produces a valid Win10-compatible executable wrapping emitted .text + .data.
//! Data section size floor: 0x38000 (Phase 2 root-cause fix).

use crate::types::IsaResult;

/// IMAGE_DOS_HEADER + PE signature offset convention.
const OUTPUT_DATA_NEED: u32 = 0x38000;

pub struct PeImage {
    pub bytes: Vec<u8>,
}

/// Wrap raw x64 code (+ optional data) in a PE32+ image.
/// Entry: sets up R15 → .data (state base), then jumps to `code`.
pub fn link_pe(code: &[u8], data: &[u8]) -> IsaResult<PeImage> {
    let section_align: u32 = 0x1000;
    let file_align: u32 = 0x200;

    let code_raw = align_up(code.len() as u32, file_align);
    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, section_align));
    let data_raw = align_up(data_need, file_align);

    // Layout:
    // 0x0000: DOS + PE headers (1 section-align = 0x1000 file, 0x200 min)
    // text VA 0x1000, data VA 0x1000 + align(code)
    let headers_raw = 0x400u32;
    let text_rva = section_align; // 0x1000
    let text_vs = align_up(code.len() as u32 + 0x40, section_align); // room for startup
    let data_rva = text_rva + text_vs;
    let data_vs = data_need;

    let size_of_image = align_up(data_rva + data_vs, section_align);
    let size_of_headers = headers_raw;

    let mut img = vec![0u8; (headers_raw + code_raw + data_raw) as usize];

    // DOS header
    img[0] = 0x4D;
    img[1] = 0x5A; // MZ
    write_u32(&mut img, 0x3C, 0x80); // e_lfanew

    // PE signature at 0x80
    img[0x80] = b'P';
    img[0x81] = b'E';

    // COFF header
    write_u16(&mut img, 0x84, 0x8664); // Machine AMD64
    write_u16(&mut img, 0x86, 2); // NumberOfSections
    write_u16(&mut img, 0x94, 0xF0); // SizeOfOptionalHeader
    write_u16(&mut img, 0x96, 0x22); // Characteristics: EXECUTABLE | LARGE_ADDRESS_AWARE

    // Optional header (PE32+)
    let opt = 0x98usize;
    write_u16(&mut img, opt, 0x20B); // PE32+
    img[opt + 2] = 1; // MajorLinkerVersion
    write_u32(&mut img, opt + 16, text_rva); // AddressOfEntryPoint = startup in .text
    write_u64(&mut img, opt + 24, 0x140000000); // ImageBase
    write_u32(&mut img, opt + 32, section_align);
    write_u32(&mut img, opt + 36, file_align);
    write_u16(&mut img, opt + 40, 6); // MajorOS
    write_u16(&mut img, opt + 44, 6); // MajorSubsystem
    write_u32(&mut img, opt + 56, size_of_image);
    write_u32(&mut img, opt + 60, size_of_headers);
    write_u16(&mut img, opt + 68, 3); // Subsystem = CONSOLE
    write_u16(&mut img, opt + 70, 0x8160); // DllCharacteristics
    write_u64(&mut img, opt + 72, 0x100000); // Stack Reserve
    write_u64(&mut img, opt + 80, 0x1000); // Stack Commit
    write_u64(&mut img, opt + 88, 0x100000); // Heap Reserve
    write_u64(&mut img, opt + 96, 0x1000); // Heap Commit
    write_u32(&mut img, opt + 108, 16); // NumberOfRvaAndSizes

    // Section .text
    let s1 = 0x98 + 0xF0; // 0x188
    write_name(&mut img, s1, b".text");
    write_u32(&mut img, s1 + 8, text_vs);
    write_u32(&mut img, s1 + 12, text_rva);
    write_u32(&mut img, s1 + 16, code_raw);
    write_u32(&mut img, s1 + 20, headers_raw);
    write_u32(&mut img, s1 + 36, 0x60000020); // CODE | EXECUTE | READ

    // Section .data
    let s2 = s1 + 40;
    write_name(&mut img, s2, b".data");
    write_u32(&mut img, s2 + 8, data_vs);
    write_u32(&mut img, s2 + 12, data_rva);
    write_u32(&mut img, s2 + 16, data_raw);
    write_u32(&mut img, s2 + 20, headers_raw + code_raw);
    write_u32(&mut img, s2 + 36, 0xC0000040); // INIT_DATA | READ | WRITE

    // SizeOfCode / SizeOfInitializedData in optional header
    write_u32(&mut img, opt + 4, code_raw);
    write_u32(&mut img, opt + 8, data_raw);
    write_u32(&mut img, opt + 20, text_rva); // BaseOfCode

    // Build startup at start of .text:
    //   lea r15, [rip + disp]  ; r15 = data base (state)
    //   jmp user_code
    let text_file_off = headers_raw as usize;
    let startup_len = 13u32; // lea r15, [rip+d] (7) + jmp rel32 (5) + align nop

    // lea r15, [rip + disp32]
    // After this 7-byte insn, RIP = text_rva + 7
    // Want r15 = imagebase+data_rva  →  disp = data_rva - (text_rva + 7)
    let lea_disp = data_rva as i32 - (text_rva as i32 + 7);
    img[text_file_off] = 0x4C; // REX.WR
    img[text_file_off + 1] = 0x8D;
    img[text_file_off + 2] = 0x3D; // ModRM: r15, [rip+disp]
    img[text_file_off + 3..text_file_off + 7].copy_from_slice(&lea_disp.to_le_bytes());

    // jmp rel32 to user code (right after startup)
    let jmp_from = text_rva + 7;
    let user_code_rva = text_rva + startup_len;
    let jmp_rel = user_code_rva as i32 - (jmp_from as i32 + 5);
    img[text_file_off + 7] = 0xE9;
    img[text_file_off + 8..text_file_off + 12].copy_from_slice(&jmp_rel.to_le_bytes());
    img[text_file_off + 12] = 0x90; // nop pad → startup_len=13

    // Copy user code
    let code_dst = text_file_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // Copy data
    let data_file_off = (headers_raw + code_raw) as usize;
    let copy_n = data.len().min(data_raw as usize);
    img[data_file_off..data_file_off + copy_n].copy_from_slice(&data[..copy_n]);

    // Entry point = text_rva (startup)
    write_u32(&mut img, opt + 16, text_rva);

    Ok(PeImage { bytes: img })
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

fn write_name(buf: &mut [u8], off: usize, name: &[u8]) {
    let n = name.len().min(8);
    buf[off..off + n].copy_from_slice(&name[..n]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pe_has_mz_and_pe() {
        let pe = link_pe(&[0xC3], &[]).unwrap();
        assert_eq!(&pe.bytes[0..2], b"MZ");
        let lfanew = u32::from_le_bytes(pe.bytes[0x3C..0x40].try_into().unwrap()) as usize;
        assert_eq!(&pe.bytes[lfanew..lfanew + 4], b"PE\0\0");
    }

    #[test]
    fn data_floor_0x38000() {
        let pe = link_pe(&[0xC3], &[]).unwrap();
        // file should be large enough to hold data section raw size
        assert!(pe.bytes.len() > 0x38000);
    }
}
