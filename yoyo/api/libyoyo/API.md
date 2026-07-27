# libyoyo API surface

| Function | Signature |
|----------|-----------|
| `libyoyo_alloc` | `(size: u64) -> *u8` |
| `libyoyo_free` | `(ptr: *u8, size: u64)` |
| `libyoyo_open` | `(path: *u8) -> i32` |
| `libyoyo_read` | `(fd, buf: *u8, len) -> i64` |
| `libyoyo_write` | `(fd, buf: *u8, len) -> i64` |
| `libyoyo_close` | `(fd: i32)` |
| `libyoyo_exit` | `(code: i32)` |
| `libyoyo_print` | `(s: *u8)` |
| `libyoyo_time` | `() -> u64` |

Naming rule: snake_case, `libyoyo_` prefix. Implementations: win32 DLL / linux SO / baremetal `.a`.
