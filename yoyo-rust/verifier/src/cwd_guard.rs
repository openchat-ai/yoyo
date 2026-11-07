// Shared process-global lock for tests that mutate the current working directory.
//
// Written as plain comments, not rustdoc: any 4-space-indented example here
// would be compiled as a doctest by `cargo test --doc`, which turns a
// documentation typo into a build failure.

// Why this exists
//
// `SetCurrentDirectoryA` / `std::env::set_current_dir` change process-global
// state, not per-thread state. Two modules each kept their own `Mutex<()>`
// (pe_dll_link::CWD_PROBE_LOCK, pe_manual_map::MANUAL_MAP_SMOKE_CWD_LOCK).
// Those two locks do not exclude each other, so under cargo's parallel test
// runner thread A could chdir into dir X while thread B concurrently chdir'd
// into dir Y. A then read B's input.tyb and the sidecar export returned the
// wrong exit code -- "fixture 0 should compile via baked oracle" and its
// siblings.
//
// The failure is deterministic under parallelism and invisible under CI's
// --test-threads=1, which serializes every test. That is exactly how it
// escaped every local gate and every CI run on master since PR #33. One
// shared lock for every cwd mutation closes the race regardless of thread
// count.

// Scope
//
// Tests only; no production code touches this. TEST_CWD_LOCK is pub(crate)
// rather than #[cfg(test)] so the shared definition does not trip dead_code
// when the crate is compiled without the `test` cfg.

/// Process-global lock serializing every test that changes the current working
/// directory. Hold it across the *whole* chdir-restore window, not just the
/// syscall itself.
pub(crate) static TEST_CWD_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Acquire [`TEST_CWD_LOCK`], recovering from a poisoned (panicked-while-held)
/// lock so one test's panic cannot wedge every other cwd test forever.
#[cfg(windows)]
pub(crate) fn test_cwd_lock() -> std::sync::MutexGuard<'static, ()> {
    TEST_CWD_LOCK.lock().unwrap_or_else(|e| e.into_inner())
}
