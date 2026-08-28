/* Stage 11-B: minimal dlopen trampoline (no CRT).
 * H_00 extracts this ELF + libyoyo_runtime.so, then execve's it.
 * Rebuild: scripts/build-linux-h00-tramp.sh
 */
        .section .text
        .global _start
_start:
        leaq    .Lso(%rip), %rdi
        movl    $1, %esi                 /* RTLD_LAZY */
        call    dlopen@PLT
        testq   %rax, %rax
        jz      .Lfail1
        movq    %rax, %rdi
        leaq    .Lsym(%rip), %rsi
        call    dlsym@PLT
        testq   %rax, %rax
        jz      .Lfail2
        call    *%rax
        movl    %eax, %edi
        movl    $60, %eax                /* sys_exit */
        syscall
.Lfail1:
        movl    $1, %edi
        movl    $60, %eax
        syscall
.Lfail2:
        movl    $2, %edi
        movl    $60, %eax
        syscall

        .section .rodata
.Lso:   .asciz  "./libyoyo_runtime.so"
.Lsym:  .asciz  "yoyo_runtime_selfhost_main"
