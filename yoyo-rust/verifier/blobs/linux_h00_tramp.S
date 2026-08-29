/* Stage 11-B / post-v1.0 OW-IAT: minimal dlopen trampoline (no CRT).
 * H_00 extracts this ELF + cwd sidecar libyoyo_runtime.so, then execve's it.
 * Contract: dlopen("./libyoyo_runtime.so") → in-process ELF dyn sym walk for
 *   yoyo_runtime_selfhost_main (no dlsym — mirrors Win PE export walk).
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
        /* glibc dlopen handle == struct link_map*; l_addr at offset 0 = load base */
        movq    (%rax), %rbp

        /* ELF magic */
        cmpl    $0x464c457f, (%rbp)      /* 0x7f 'E' 'L' 'F' */
        jne     .Lfail2
        cmpb    $2, 4(%rbp)              /* EI_CLASS ELFCLASS64 */
        jne     .Lfail2
        cmpw    $0x3e, 0x12(%rbp)          /* e_machine EM_X86_64 */
        jne     .Lfail2

        movzwl  0x36(%rbp), %ecx          /* e_phentsize */
        movzwl  0x38(%rbp), %edx          /* e_phnum */
        movq    0x20(%rbp), %r8           /* e_phoff */
        leaq    (%rbp,%r8), %r9           /* first Phdr */
        xorl    %r10d, %r10d
.Lph_loop:
        cmpl    %r10d, %edx
        jbe     .Lfail2
        cmpl    $2, (%r9)                 /* PT_DYNAMIC */
        je      .Lph_found
        addq    %rcx, %r9
        incl    %r10d
        jmp     .Lph_loop
.Lph_found:
        movq    16(%r9), %r11             /* p_vaddr */
        leaq    (%rbp,%r11), %r12         /* dynamic */
        movq    40(%r9), %r13             /* p_memsz */
        leaq    (%r12,%r13), %r14         /* dynamic end */

        xorl    %r15d, %r15d              /* symtab off */
        xorl    %ebx, %ebx                /* strtab off */
.Ldyn_loop:
        cmpq    %r12, %r14
        jbe     .Lfail2
        movq    (%r12), %rax              /* d_tag */
        testq   %rax, %rax
        jz      .Ldyn_done
        cmpq    $6, %rax                  /* DT_SYMTAB */
        je      .Ldyn_sym
        cmpq    $5, %rax                  /* DT_STRTAB */
        je      .Ldyn_str
        addq    $16, %r12
        jmp     .Ldyn_loop
.Ldyn_sym:
        movq    8(%r12), %r15
        addq    $16, %r12
        jmp     .Ldyn_loop
.Ldyn_str:
        movq    8(%r12), %rbx
        addq    $16, %r12
        jmp     .Ldyn_loop
.Ldyn_done:
        testq   %r15, %r15
        jz      .Lfail2
        testq   %rbx, %rbx
        jz      .Lfail2
        /* Relocated DT_SYMTAB/DT_STRTAB are absolute VAs — do not add load base */
        movq    %r15, %r12                /* symtab */
        movq    %rbx, %r13                /* strtab */
        leaq    .Lsym(%rip), %r14         /* needle */

        xorl    %ebx, %ebx                /* sym index */
.Lsym_loop:
        cmpl    $128, %ebx
        jae     .Lfail2
        movl    %ebx, %eax
        shlq    $3, %rax                  /* *8 */
        leaq    (%rax,%rax,2), %rax       /* *24 = sizeof(Elf64_Sym) */
        leaq    (%r12,%rax), %rsi         /* sym */
        movzbl  4(%rsi), %eax             /* st_info */
        movl    %eax, %edx
        andl    $0xf, %edx                /* STT_* */
        cmpl    $2, %edx                  /* STT_FUNC */
        jne     .Lsym_next
        shrl    $4, %eax
        cmpl    $1, %eax                  /* STB_GLOBAL */
        je      .Lsym_name
        cmpl    $2, %eax                  /* STB_WEAK */
        jne     .Lsym_next
.Lsym_name:
        movl    (%rsi), %eax              /* st_name */
        leaq    (%r13,%rax), %rdi
        movq    %r14, %rsi
.Lstrcmp:
        movzbl  (%rdi), %eax
        movzbl  (%rsi), %edx
        cmpb    %al, %dl
        jne     .Lsym_next
        testb   %al, %al
        jz      .Lsym_found
        incq    %rdi
        incq    %rsi
        jmp     .Lstrcmp
.Lsym_found:
        movl    %ebx, %eax
        shlq    $3, %rax
        leaq    (%rax,%rax,2), %rax       /* index * 24 */
        addq    %r12, %rax                /* sym */
        movq    8(%rax), %rax             /* st_value */
        addq    %rbp, %rax
        call    *%rax
        movl    %eax, %edi
        movl    $60, %eax                 /* sys_exit */
        syscall
.Lsym_next:
        incl    %ebx
        jmp     .Lsym_loop

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
