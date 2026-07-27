//! libyoyo — platform syscall abstraction (PROMPT-v3 Part 7.6 / Appendix A).
//!
//! C ABI exports (cdylib):
//!   libyoyo_alloc, libyoyo_free, libyoyo_open, libyoyo_read,
//!   libyoyo_write, libyoyo_close, libyoyo_exit, libyoyo_print, libyoyo_time

#![allow(clippy::missing_safety_doc)]

use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

/// Allocate `size` bytes. Returns pointer or null on failure.
#[no_mangle]
pub unsafe extern "C" fn libyoyo_alloc(size: u64) -> *mut u8 {
    if size == 0 || size > isize::MAX as u64 {
        return std::ptr::null_mut();
    }
    let layout = std::alloc::Layout::from_size_align(size as usize, 16)
        .unwrap_or(std::alloc::Layout::from_size_align(1, 1).unwrap());
    std::alloc::alloc_zeroed(layout)
}

#[no_mangle]
pub unsafe extern "C" fn libyoyo_free(ptr: *mut u8, size: u64) {
    if ptr.is_null() || size == 0 {
        return;
    }
    if let Ok(layout) = std::alloc::Layout::from_size_align(size as usize, 16) {
        std::alloc::dealloc(ptr, layout);
    }
}

/// Open path (NUL-terminated) for read/write. Returns fd-like handle index, or -1.
/// Simplified: returns a heap Box<File> pointer cast to i64.
#[no_mangle]
pub unsafe extern "C" fn libyoyo_open(path: *const u8) -> i32 {
    if path.is_null() {
        return -1;
    }
    let cstr = std::ffi::CStr::from_ptr(path as *const i8);
    let Ok(s) = cstr.to_str() else {
        return -1;
    };
    match OpenOptions::new().read(true).write(true).create(true).open(s) {
        Ok(f) => {
            let boxed = Box::new(f);
            Box::into_raw(boxed) as i32
        }
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn libyoyo_read(fd: i32, buf: *mut u8, len: u64) -> i64 {
    if fd == 0 || buf.is_null() {
        return -1;
    }
    let file = &mut *(fd as *mut std::fs::File);
    let slice = std::slice::from_raw_parts_mut(buf, len as usize);
    match file.read(slice) {
        Ok(n) => n as i64,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn libyoyo_write(fd: i32, buf: *const u8, len: u64) -> i64 {
    if buf.is_null() {
        return -1;
    }
    let slice = std::slice::from_raw_parts(buf, len as usize);
    if fd == 1 || fd == 2 {
        // stdout/stderr
        let mut out = if fd == 1 {
            Box::new(std::io::stdout()) as Box<dyn Write>
        } else {
            Box::new(std::io::stderr()) as Box<dyn Write>
        };
        return match out.write(slice) {
            Ok(n) => n as i64,
            Err(_) => -1,
        };
    }
    if fd == 0 {
        return -1;
    }
    let file = &mut *(fd as *mut std::fs::File);
    match file.write(slice) {
        Ok(n) => n as i64,
        Err(_) => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn libyoyo_close(fd: i32) {
    if fd <= 2 {
        return;
    }
    let _ = Box::from_raw(fd as *mut std::fs::File);
}

#[no_mangle]
pub unsafe extern "C" fn libyoyo_exit(code: i32) -> ! {
    std::process::exit(code);
}

#[no_mangle]
pub unsafe extern "C" fn libyoyo_print(s: *const u8) {
    if s.is_null() {
        return;
    }
    let cstr = std::ffi::CStr::from_ptr(s as *const i8);
    if let Ok(text) = cstr.to_str() {
        print!("{text}");
        let _ = std::io::stdout().flush();
    }
}

#[no_mangle]
pub unsafe extern "C" fn libyoyo_time() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn alloc_free() {
        unsafe {
            let p = libyoyo_alloc(64);
            assert!(!p.is_null());
            libyoyo_free(p, 64);
        }
    }

    #[test]
    fn time_nonzero() {
        unsafe {
            assert!(libyoyo_time() > 0);
        }
    }

    #[test]
    fn print_ok() {
        let s = CString::new("libyoyo_ok\n").unwrap();
        unsafe {
            libyoyo_print(s.as_ptr() as *const u8);
        }
    }
}
