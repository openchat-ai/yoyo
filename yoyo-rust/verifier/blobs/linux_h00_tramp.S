/* Stage 11-B / post-v1.0 OW-IAT: syscall-only ELF manual-map trampoline.
 *
 * Production source: linux_h00_tramp_mmap.c (rebuild via scripts/build-linux-h00-tramp.sh).
 * Replaces dlopen with open/read/mmap + in-process PT_LOAD map (mirrors Win pe_manual_map).
 *
 * Contract: load cwd ./libyoyo_runtime.so via syscalls (no dlopen/libdl/libc NEEDED),
 *   then in-process ELF dyn sym walk for yoyo_runtime_selfhost_main (no dlsym).
 *
 * This .S file is kept as the historical dlopen reference only; the committed
 * linux_h00_tramp.elf blob is built from linux_h00_tramp_mmap.c.
 */
#if 0
/* --- historical dlopen path (pre mmap wire-up) --- */
        .section .text
        .global _start
_start:
        leaq    .Lso(%rip), %rdi
        movl    $1, %esi
        call    dlopen@PLT
        /* ... sym walk ... */
#endif
