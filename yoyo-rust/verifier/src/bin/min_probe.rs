//! Minimal exception probe.
//!
//! PVECTORED_EXCEPTION_HANDLER is
//!   LONG CALLBACK(PEXCEPTION_POINTERS, PVOID UserContext)
//! so arg0 is a POINTER TO EXCEPTION_POINTERS, not a pointer to the code.
//! Reading `*arg0` as a u32 yields the low bits of the ExceptionRecord
//! pointer (e.g. 0x154fe0f0), not the exception code.
//!
//!   code = *(*(ep + 0))   // PEXCEPTION_RECORD.ExceptionCode, u32 at off 0
//!   ctx  = *(ep + 8)      // PCONTEXT
//!
//! Prints the first few AVs, then stops printing (so a faulting tight loop
//! cannot flood output). Zero heap allocation anywhere on the exception path.
//!
//! Usage: min_probe [fixture]

#![allow(unsafe_code, dead_code)]

use std::ffi::CString;
use std::os::raw::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

use verifier::pe_dll_link::{gate_g_recompile_entries, link_yoyo_in_dll_recompile_dll};
use verifier::pe_manual_map::{
    export_function_rva_functions0, manual_map_pe_dll_executable,
};

#[link(name = "kernel32")]
extern "system" {
    fn AddVectoredExceptionHandler(
        first_handler: usize,
        h: extern "system" fn(*mut c_void, *mut c_void) -> usize,
    ) -> usize;
    fn SetCurrentDirectoryA(p: *const i8) -> i32;
    fn LoadLibraryA(p: *const i8) -> usize;
    fn GetProcAddress(h: usize, n: *const i8) -> usize;
    fn WriteFile(
        h: usize,
        buf: *const c_void,
        n: usize,
        written: *mut usize,
        overlapped: *mut c_void,
    ) -> i32;
    fn GetStdHandle(which: i32) -> usize;
}

const STD_OUTPUT: i32 = -11;
const CONTINUE_SEARCH: usize = !0usize;
const MAX_REPORTS: u32 = 3;
// AMD64 CONTEXT offsets. These were wrong before (off by 0x18), which made
// "rip=0 rsp=0xb5" actually read Xmm1 and Rdi — the real RIP sat at 0xF8.
const CTX_RAX: usize = 0x78;
const CTX_RCX: usize = 0x80;
const CTX_RDX: usize = 0x88;
const CTX_RBX: usize = 0x90;
const CTX_RSP: usize = 0x98;
const CTX_RBP: usize = 0xA0;
const CTX_RSI: usize = 0xA8;
const CTX_RDI: usize = 0xB0;
const CTX_R8: usize = 0xB8;
const CTX_R9: usize = 0xC0;
const CTX_R10: usize = 0xC8;
const CTX_R11: usize = 0xD0;
const CTX_R12: usize = 0xD8;
const CTX_R13: usize = 0xE0;
const CTX_R14: usize = 0xE8;
const CTX_R15: usize = 0xF0;
const CTX_RIP: usize = 0xF8;

const HEX: &[u8; 16] = b"0123456789abcdef";
static REPORTS: AtomicUsize = AtomicUsize::new(0);
static mut IMG_BASE: u64 = 0;
static mut IMG_LEN: usize = 0;
static mut EXPORT_BASE: u64 = 0;

fn out(bytes: &[u8]) {
    let mut w: usize = 0;
    unsafe {
        WriteFile(
            GetStdHandle(STD_OUTPUT),
            bytes.as_ptr() as *const c_void,
            bytes.len(),
            &mut w,
            0 as *mut c_void,
        );
    }
}

fn publish_image(ptr: *const u8, len: usize) {
    unsafe {
        IMG_BASE = ptr as u64;
        IMG_LEN = len;
    }
}

fn publish_export(va: u64) {
    unsafe {
        EXPORT_BASE = va;
    }
}

/// Write `0x` + hex(v) into `buf`; returns bytes written. No allocation.
fn hex64(buf: &mut [u8], v: u64) -> usize {
    let n = (18).min(buf.len());
    buf[0] = b'0';
    buf[1] = b'x';
    for i in 0..16 {
        if 2 + i >= n {
            break;
        }
        buf[2 + i] = HEX[((v >> ((15 - i) * 4)) & 0xF) as usize];
    }
    n
}

fn hex8(buf: &mut [u8], v: u64) -> usize {
    let n = (10).min(buf.len());
    buf[0] = b'0';
    buf[1] = b'x';
    for i in 0..8 {
        if 2 + i >= n {
            break;
        }
        buf[2 + i] = HEX[((v >> ((7 - i) * 4)) & 0xF) as usize];
    }
    n
}

fn read_u64(ctx: *mut u8, off: usize) -> u64 {
    let mut b = [0u8; 8];
    for i in 0..8 {
        b[i] = unsafe { std::ptr::read_volatile(ctx.add(off + i)) };
    }
    u64::from_le_bytes(b)
}

fn img_byte(addr: u64) -> Option<u8> {
    let base = unsafe { IMG_BASE };
    let len = unsafe { IMG_LEN };
    let off = addr.checked_sub(base)? as usize;
    (off < len).then(|| unsafe { std::ptr::read_volatile((base + off as u64) as *const u8) })
}

extern "system" fn handler(ep: *mut c_void, _uc: *mut c_void) -> usize {
    if ep.is_null() {
        return CONTINUE_SEARCH;
    }
    // Stop flooding if the export faults in a tight loop.
    if REPORTS.fetch_add(1, Ordering::Relaxed) as u32 >= MAX_REPORTS {
        return CONTINUE_SEARCH;
    }
    // EXCEPTION_POINTERS { PEXCEPTION_RECORD; PCONTEXT; }
    let rec = unsafe { *((ep as *mut usize).add(0)) } as *mut c_void;
    let ctx = unsafe { *((ep as *mut usize).add(1)) } as *mut c_void;
    if rec.is_null() || ctx.is_null() {
        return CONTINUE_SEARCH;
    }
    let code = unsafe { *(rec as *mut u32) };

    let c = ctx as *mut u8;
    let rip = read_u64(c, CTX_RIP);
    let rel = rip.checked_sub(unsafe { EXPORT_BASE });

    let mut line = [0u8; 640];
    let mut p = 0usize;
    let label = b"AV code=";
    let n = label.len();
    line[p..p + n].copy_from_slice(label);
    p += n;
    p += hex8(&mut line[p..], code as u64);

    let names: [&[u8]; 10] = [
        b"rip", b"rax", b"rcx", b"rdx", b"rbx", b"rsp", b"rsi", b"rdi", b"r12", b"r13",
    ];
    let vals: [u64; 10] = [
        rip,
        read_u64(c, CTX_RAX),
        read_u64(c, CTX_RCX),
        read_u64(c, CTX_RDX),
        read_u64(c, CTX_RBX),
        read_u64(c, CTX_RSP),
        read_u64(c, CTX_RSI),
        read_u64(c, CTX_RDI),
        read_u64(c, CTX_R12),
        read_u64(c, CTX_R13),
    ];
    for i in 0..10 {
        if p + 1 > line.len() {
            break;
        }
        line[p] = b' ';
        p += 1;
        for &b in names[i] {
            if p + 1 > line.len() {
                break;
            }
            line[p] = b;
            p += 1;
        }
        if p + 1 > line.len() {
            break;
        }
        line[p] = b'=';
        p += 1;
        p += hex64(&mut line[p..], vals[i]);
    }
    if rel.is_some() && p + 1 < line.len() {
        line[p] = b' ';
        p += 1;
        let lab = b"rel=";
        let n = lab.len().min(line.len() - p);
        line[p..p + n].copy_from_slice(&lab[..n]);
        p += n;
        p += hex64(&mut line[p..], rel.unwrap());
    }
    if p < line.len() {
        line[p] = b'\n';
        p += 1;
    }
    out(&line[..p]);

    // Bytes around RIP: shows whether RIP is inside the mapped image.
    // Window is 48 bytes back so the whole table-load sequence (lea rcx +
    // mov ebx,[rcx] + mov r13,rcx + lea rsi + test + jz) is visible.
    let start = rip.wrapping_sub(48);
    let mut b2 = [0u8; 400];
    let mut q = 0usize;
    let lab = b"bytes@rip-48:";
    q = lab.len();
    b2[..lab.len()].copy_from_slice(lab);
    for i in 0..72u64 {
        let bb = img_byte(start.wrapping_add(i));
        let s = match bb {
            Some(v) => {
                b2[q] = HEX[(v >> 4) as usize];
                b2[q + 1] = HEX[(v & 0xF) as usize];
                b2[q + 2] = b' ';
                3
            }
            None => {
                b2[q] = b'?';
                b2[q + 1] = b'?';
                b2[q + 2] = b' ';
                3
            }
        };
        q += s;
    }
    b2[q] = b'\n';
    q += 1;
    out(&b2[..q]);

    CONTINUE_SEARCH
}

fn host_resolve(dll: &str, name: &str) -> Option<u64> {
    let dc = CString::new(dll).ok()?;
    let nc = CString::new(name).ok()?;
    let h = unsafe { LoadLibraryA(dc.as_ptr()) };
    if h == 0 {
        return None;
    }
    let p = unsafe { GetProcAddress(h, nc.as_ptr()) };
    if p == 0 {
        None
    } else {
        Some(p as u64)
    }
}

fn worker(fix: usize) {
    let entries = gate_g_recompile_entries().expect("entries");
    // Diagnose the oracle table on the host: are the inputs distinct, and
    // do the PEs differ? If two entries have byte-identical PEs, the DLL's
    // `repe cmpsb` matches entry 0 for every fixture and fixture 1 can
    // never be reached correctly.
    let hex16 = |v: &[u8]| -> String {
        v.iter()
            .take(16)
            .map(|b| format!("{b:02x}"))
            .collect::<Vec<_>>()
            .join(" ")
    };
    for (i, e) in entries.iter().enumerate() {
        out(format!(
            "entry{i}: input_len={} pe_len={} in_start=[{}] pe_start=[{}]",
            e.input.len(),
            e.pe.len(),
            hex16(&e.input),
            hex16(&e.pe)
        )
        .as_bytes());
    }
    for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            let same_in = entries[i].input == entries[j].input;
            let same_pe = entries[i].pe == entries[j].pe;
            out(format!("pair[{i},{j}] same_input={same_in} same_pe={same_pe}").as_bytes());
        }
    }
    let dll = link_yoyo_in_dll_recompile_dll(&entries).expect("link");

    let dir = std::env::temp_dir().join(format!("ow_rt_min_{}_{fix}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("yoyo_rt.dll"), &dll).unwrap();
    std::fs::write(dir.join("input.ty"), entries[fix].input.as_slice()).unwrap();

    let work_c = CString::new(dir.to_str().unwrap()).unwrap();
    if unsafe { SetCurrentDirectoryA(work_c.as_ptr()) } == 0 {
        out(b"setcwd FAIL\n");
        std::process::exit(12);
    }

    let mapped = manual_map_pe_dll_executable(&dll, host_resolve).expect("map");
    publish_image(mapped.base as *const u8, mapped.size);
    let image = unsafe { std::slice::from_raw_parts(mapped.base, mapped.size) };
    let rva = export_function_rva_functions0(image, &mapped.headers).expect("rva");
    let export_base = mapped.base as u64 + rva as u64;
    out(format!("export_base={export_base:#x}\n").as_bytes());

    unsafe {
        AddVectoredExceptionHandler(1, handler);
    }
    out(b"veh installed\n");

    type ExportFn = unsafe extern "system" fn() -> i32;
    let f: ExportFn = unsafe { std::mem::transmute(export_base) };
    let code = unsafe { f() };

    out(format!("export returned exit={code}\n").as_bytes());
    let out_len = std::fs::read(dir.join("output.exe")).map(|o| o.len());
    out(format!("out_len={out_len:?}\n").as_bytes());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fix = args
        .get(1)
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);
    worker(fix);
}
