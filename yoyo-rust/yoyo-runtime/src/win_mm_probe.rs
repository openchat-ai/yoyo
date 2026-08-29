//! Stage17 manual-map smoke probe without Rust std/CRT (DllMain skipped on map).

use std::os::raw::c_void;

const GENERIC_WRITE: u32 = 0x4000_0000;
const CREATE_ALWAYS: u32 = 2;
const FILE_ATTRIBUTE_NORMAL: u32 = 0x80;
const INVALID_HANDLE_VALUE: isize = -1;

/// When `YOYO_MM_SMOKE_PROBE` is set, write `output.exe` via resolved kernel32 IAT.
pub fn run_if_env_set() -> Option<i32> {
    unsafe {
        #[link(name = "kernel32")]
        extern "system" {
            fn GetEnvironmentVariableA(name: *const i8, buf: *mut i8, n: u32) -> u32;
            fn CreateFileA(
                lpFileName: *const i8,
                dwDesiredAccess: u32,
                dwShareMode: u32,
                lpSecurityAttributes: *mut c_void,
                dwCreationDisposition: u32,
                dwFlagsAndAttributes: u32,
                hTemplateFile: *mut c_void,
            ) -> *mut c_void;
            fn WriteFile(
                hFile: *mut c_void,
                lpBuffer: *const u8,
                nNumberOfBytesToWrite: u32,
                lpNumberOfBytesWritten: *mut u32,
                lpOverlapped: *mut c_void,
            ) -> i32;
            fn CloseHandle(hObject: *mut c_void) -> i32;
        }

        let mut buf = [0i8; 1];
        if GetEnvironmentVariableA(
            b"YOYO_MM_SMOKE_PROBE\0".as_ptr() as *const i8,
            buf.as_mut_ptr(),
            1,
        ) == 0
        {
            return None;
        }

        let path = b"output.exe\0";
        let h = CreateFileA(
            path.as_ptr() as *const i8,
            GENERIC_WRITE,
            0,
            std::ptr::null_mut(),
            CREATE_ALWAYS,
            FILE_ATTRIBUTE_NORMAL,
            std::ptr::null_mut(),
        );
        if h as isize == INVALID_HANDLE_VALUE {
            return Some(10);
        }
        let data = b"probe";
        let mut written = 0u32;
        let ok = WriteFile(
            h,
            data.as_ptr(),
            data.len() as u32,
            &mut written,
            std::ptr::null_mut(),
        );
        CloseHandle(h);
        if ok == 0 || written != data.len() as u32 {
            return Some(11);
        }
        Some(0)
    }
}
