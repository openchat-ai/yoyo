//! Minimal PE32 (32-bit x86) linker (PROMPT-v3 Phase 1).
//! Produces a valid Win32 x86 executable wrapping emitted .text + .data.

use crate::types::IsaResult;

const OUTPUT_DATA_NEED: u32 = 0x38000;

pub struct Pe32Image {
    pub bytes: Vec<u8>,
}

/// Wrap raw x86 code (+ optional data) in a PE32 image.
/// Entry: sets up EDI → .data (state base), then jumps to `code`.
pub fn link_x86(code: &[u8], data: &[u8]) -> IsaResult<Pe32Image> {
    let section_align: u32 = 0x1000;
    let file_align: u32 = 0x200;

    let code_raw = align_up(code.len() as u32, file_align);
    let data_need = OUTPUT_DATA_NEED.max(align_up(data.len() as u32 + 0x1000, section_align));
    let data_raw = align_up(data_need, file_align);

    // Layout:
    // 0x0000: DOS + PE headers
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
    write_u16(&mut img, 0x84, 0x014C); // Machine IMAGE_FILE_MACHINE_I386
    write_u16(&mut img, 0x86, 2); // NumberOfSections
    // Optional header size for PE32:
    //   standard fields 28 + data dirs 16*8=128 → 0x78,
    //   but we declare 0xF0 to match PE64 convention used elsewhere.
    write_u16(&mut img, 0x94, 0xE0); // SizeOfOptionalHeader
    write_u16(&mut img, 0x96, 0x22); // Characteristics

    // Optional header (PE32) — addresses are u32.
    let opt = 0x98usize;
    write_u16(&mut img, opt, 0x10B); // PE32 magic
    img[opt + 2] = 1; // MajorLinkerVersion
    write_u32(&mut img, opt + 4, code_raw); // SizeOfCode
    write_u32(&mut img, opt + 8, data_raw); // SizeOfInitializedData
    // opt+12: SizeOfUninitializedData (0)
    write_u32(&mut img, opt + 16, text_rva); // AddressOfEntryPoint (set below, set to text_rva)
    write_u32(&mut img, opt + 20, text_rva); // BaseOfCode
    write_u32(&mut img, opt + 24, data_rva); // BaseOfData (only present in PE32)
    write_u32(&mut img, opt + 28, 0x00400000); // ImageBase
    write_u32(&mut img, opt + 32, section_align);
    write_u32(&mut img, opt + 36, file_align);
    write_u16(&mut img, opt + 40, 6); // MajorOS
    write_u16(&mut img, opt + 44, 6); // MajorSubsystem
    write_u32(&mut img, opt + 56, size_of_image);
    write_u32(&mut img, opt + 60, size_of_headers);
    write_u16(&mut img, opt + 68, 3); // Subsystem = CONSOLE
    write_u16(&mut img, opt + 70, 0x8160); // DllCharacteristics
    write_u32(&mut img, opt + 72, 0x100000); // Stack Reserve (PE32 uses u32)
    write_u32(&mut img, opt + 76, 0x1000); // Stack Commit
    write_u32(&mut img, opt + 80, 0x100000); // Heap Reserve
    write_u32(&mut img, opt + 84, 0x1000); // Heap Commit
    write_u32(&mut img, opt + 96, 16); // NumberOfRvaAndSizes

    // Section .text
    let s1 = 0x98 + 0xE0; // 0x178 (after PE32 optional header)
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

    // Build startup at start of .text:
    //   mov edi, imm32  ; EDI = data VA (state base) — 5 bytes
    //   jmp rel32       ; jump to user code — 5 bytes
    //   nop nop         ; pad to 12
    let text_file_off = headers_raw as usize;
    let startup_len = 12u32;

    let data_va = 0x00400000u32 + data_rva;
    img[text_file_off] = 0xBF; // mov edi, imm32
    img[text_file_off + 1..text_file_off + 5].copy_from_slice(&data_va.to_le_bytes());

    let jmp_from = text_rva + 5;
    let user_code_rva = text_rva + startup_len;
    let jmp_rel = user_code_rva as i32 - (jmp_from as i32 + 5);
    img[text_file_off + 5] = 0xE9;
    img[text_file_off + 6..text_file_off + 10].copy_from_slice(&jmp_rel.to_le_bytes());
    img[text_file_off + 10] = 0x90; // nop
    img[text_file_off + 11] = 0x90; // nop

    // Copy user code
    let code_dst = text_file_off + startup_len as usize;
    img[code_dst..code_dst + code.len()].copy_from_slice(code);

    // Copy data
    let data_file_off = (headers_raw + code_raw) as usize;
    let copy_n = data.len().min(data_raw as usize);
    img[data_file_off..data_file_off + copy_n].copy_from_slice(&data[..copy_n]);

    // Entry point = text_rva (startup)
    write_u32(&mut img, opt + 16, text_rva);

    Ok(Pe32Image { bytes: img })
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

fn write_name(buf: &mut [u8], off: usize, name: &[u8]) {
    let n = name.len().min(8);
    buf[off..off + n].copy_from_slice(&name[..n]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pe32_has_mz_and_pe() {
        let pe = link_x86(&[0xC3], &[]).unwrap();
        assert_eq!(&pe.bytes[0..2], b"MZ");
        let lfanew = u32::from_le_bytes(pe.bytes[0x3C..0x40].try_into().unwrap()) as usize;
        assert_eq!(&pe.bytes[lfanew..lfanew + 4], b"PE\0\0");
    }

    #[test]
    fn pe32_magic_and_machine() {
        let pe = link_x86(&[0xC3], &[]).unwrap();
        assert_eq!(
            u16::from_le_bytes(pe.bytes[0x98..0x9A].try_into().unwrap()),
            0x10B,
        );
        assert_eq!(
            u16::from_le_bytes(pe.bytes[0x84..0x86].try_into().unwrap()),
            0x014C,
        );
    }

    #[test]
    fn pe32_startup_edi_and_jmp() {
        let pe = link_x86(&[0xC3], &[]).unwrap();
        // startup starts at file offset 0x400 (headers_raw)
        assert_eq!(pe.bytes[0x400], 0xBF); // mov edi, imm32
        assert_eq!(pe.bytes[0x405], 0xE9); // jmp rel32
    }

    #[test]
    fn data_floor_0x38000() {
        let pe = link_x86(&[0xC3], &[]).unwrap();
        assert!(pe.bytes.len() > 0x38000);
    }
}
