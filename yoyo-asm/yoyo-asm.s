; yoyo-asm.s — ground-truth DDC peer (PROMPT-v3 Appendix B)
; Target: ~500 lines with NASM macros. Linux-first; Windows port later.
; This skeleton implements Layer 1–2 (startup + primitives) and a minimal
; main that exits 0. Full .ty parser/emit is Phase 4d.
;
; scope=primitive-probe — emits isolated opcode byte streams for golden
; compare. NOT an asm compiler peer / NOT C-ddc claim.

BITS 64
DEFAULT REL

%macro emit_byte 1
    mov byte [rdi], %1
    inc rdi
%endmacro

section .text
global _start
_start:
    ; R15 = state base (BSS)
    lea r15, [rel state]
    ; RDI = output buffer
    lea rdi, [rel outbuf]

    ; ── INC S[0x50] + RET ──────────────────────────────────────────
    mov sil, 0x50
    call emit_load_state_rax   ; 49 8B 87 80 02 00 00
    call emit_inc_rax          ; 48 FF C0
    mov sil, 0x50
    call emit_store_state_rax  ; 49 89 87 80 02 00 00
    call emit_ret              ; C3
    ; = 18 bytes @ offset 0
    ; golden: 498b878002000048ffc049898780020000c3

    ; ── DEC S[0x50] + RET ──────────────────────────────────────────
    mov sil, 0x50
    call emit_load_state_rax   ; 49 8B 87 80 02 00 00
    call emit_dec_rax          ; 48 FF C8
    mov sil, 0x50
    call emit_store_state_rax  ; 49 89 87 80 02 00 00
    call emit_ret              ; C3
    ; = 18 bytes @ offset 18
    ; golden: 498b878002000048ffc849898780020000c3

    ; ── SET S[0x50]=0x2A + GET S[0x51]←S[0x50] + RET ──────────────
    ; G01 disk golden (with trailing RET)
    lea rsi, [rel imm_2a]
    call emit_movabs_rax       ; 48 B8 2A 00 00 00 00 00 00 00
    mov sil, 0x50
    call emit_store_state_rax  ; 49 89 87 80 02 00 00
    mov sil, 0x50
    call emit_load_state_rax   ; 49 8B 87 80 02 00 00
    mov sil, 0x51
    call emit_store_state_rax  ; 49 89 87 88 02 00 00
    call emit_ret              ; C3
    ; = 32 bytes @ offset 36
    ; golden: 48b82a0000000000000049898780020000498b878002000049898788020000c3

    ; ── ADDV S[0x50] += S[0x51] ────────────────────────────────────
    ; G02 addv disk golden (NO trailing RET)
    mov sil, 0x50
    call emit_load_state_rax   ; 49 8B 87 80 02 00 00
    mov sil, 0x51
    call emit_load_state_rcx   ; 49 8B 8F 88 02 00 00
    call emit_add_rax_rcx      ; 48 01 C8
    mov sil, 0x50
    call emit_store_state_rax  ; 49 89 87 80 02 00 00
    ; = 24 bytes @ offset 68
    ; golden: 498b8780020000498b8f880200004801c849898780020000

    ; ── ORV S[0x50] |= S[0x51] ─────────────────────────────────────
    ; G02 orv disk golden (NO trailing RET); MUST differ from ADDV
    mov sil, 0x50
    call emit_load_state_rax   ; 49 8B 87 80 02 00 00
    mov sil, 0x51
    call emit_load_state_rcx   ; 49 8B 8F 88 02 00 00
    call emit_or_rax_rcx       ; 48 09 C8
    mov sil, 0x50
    call emit_store_state_rax  ; 49 89 87 80 02 00 00
    ; = 24 bytes @ offset 92
    ; golden: 498b8780020000498b8f880200004809c849898780020000

    ; ── JMP fixture (G-SM-JMP): SET S[50]=0 + RET + JMP H_00 + RET ─
    ; Disk golden includes both handlers; JMP@18 rel32=-23 → target@0
    lea rsi, [rel imm_0]
    call emit_movabs_rax       ; 48 B8 00 00 00 00 00 00 00 00
    mov sil, 0x50
    call emit_store_state_rax  ; 49 89 87 80 02 00 00
    call emit_ret              ; C3
    mov esi, -23               ; rel32 = 0 - (18 + 5)
    call emit_jmp_rel32        ; E9 E9 FF FF FF
    call emit_ret              ; C3
    ; = 24 bytes @ offset 116
    ; golden: 48b8000000000000000049898780020000c3e9e9ffffffc3

    ; ── CALL fixture (G-SM-CALL): SET S[50]=0 + RET + CALL H_00 + RET ─
    ; Disk golden includes both handlers; CALL@18 rel32=-23 → target@0
    ; MUST differ from JMP at byte 18 (E8 vs E9), rest identical.
    lea rsi, [rel imm_0]
    call emit_movabs_rax       ; 48 B8 00 00 00 00 00 00 00 00
    mov sil, 0x50
    call emit_store_state_rax  ; 49 89 87 80 02 00 00
    call emit_ret              ; C3
    mov esi, -23               ; rel32 = 0 - (18 + 5)
    call emit_call_rel32       ; E8 E9 FF FF FF
    call emit_ret              ; C3
    ; = 24 bytes @ offset 140
    ; golden: 48b8000000000000000049898780020000c3e8e9ffffffc3

    ; ── FORWARD CALL+RET fixture (G04): CALL H_02 + RET; H_02: SET + RET
    ; Disk golden includes the same 24-byte compound. CALL@0 rel32=+1 → body@6.
    mov esi, 1               ; rel32 = 6 - (0 + 5)
    call emit_call_rel32     ; E8 01 00 00 00
    call emit_ret             ; C3 (fall-through RET in compound head)
    lea rsi, [rel imm_cc]
    call emit_movabs_rax      ; 48 B8 CC 00 00 00 00 00 00 00
    mov sil, 0x50
    call emit_store_state_rax ; 49 89 87 80 02 00 00
    call emit_ret              ; C3 (callee epilogue)
    ; = 24 bytes @ offset 164
    ; golden: e801000000c348b8cc0000000000000049898780020000c3

    ; ── NOP + RET (G00) — minimal fixture, NO state access ───────────
    ; Disk golden is independent encoding: just 90 c3.
    call emit_nop               ; 90
    call emit_ret               ; c3
    ; = 2 bytes @ offset 188
    ; golden: 90c3

    ; ── RET (G00 suffix) — standalone 1-byte primitive ──────────────
    ; Expected byte is taken from the final byte of the disk G00 golden.
    call emit_ret               ; c3
    ; = 1 byte @ offset 190
    ; golden suffix: c3

    ; ── LDB fixture (G-SM-LDB): H_00 SET S[50]=0 + RET; H_01 LDB(50,60,0) + RET; H_02 RET
    ; Three handlers in one stream; LDB per PROMPT-v3 §4S.3 = load_state(ss,rax) +
    ; (add_imm if oo!=0) + movzx rax,byte[rax] + store_state(dd,rax).
    ; Asm emits the same 38-byte stream from independent primitives:
    ;   SET S[50]=0:  movabs rax,0 (10B) + store_state(0x50) (7B) + ret (1B) = 18B
    ;   LDB S[50]←zx(byte[mem[S[60]+0]]):  load_state(0x60) (7B) + movzx (4B) +
    ;                                       store_state(0x50) (7B) + ret (1B) = 19B
    ;   H_02:         ret (1B)
    lea rsi, [rel imm_0]
    call emit_movabs_rax       ; 48 B8 00 00 00 00 00 00 00 00
    mov sil, 0x50
    call emit_store_state_rax  ; 49 89 87 80 02 00 00
    call emit_ret              ; C3
    mov sil, 0x60
    call emit_load_state_rax   ; 49 8B 87 00 03 00 00
    call emit_movzx_rax_byte_rax ; 48 0F B6 00
    mov sil, 0x50
    call emit_store_state_rax  ; 49 89 87 80 02 00 00
    call emit_ret              ; C3
    call emit_ret              ; C3
    ; = 38 bytes @ offset 191
    ; golden: 48b8000000000000000049898780020000c3498b8700030000480fb60049898780020000c3c3

    ; ── LDB offset=8 fixture: independent add-imm8 encoding, compile-only
    lea rsi, [rel imm_0]
    call emit_movabs_rax
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    mov sil, 0x60
    call emit_load_state_rax
    call emit_add_rax_imm8_8     ; 48 83 C0 08
    call emit_movzx_rax_byte_rax ; 48 0F B6 00
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    call emit_ret
    ; = 42 bytes @ offset 229
    ; golden: 48b8000000000000000049898780020000c3498b87000300004883c008480fb60049898780020000c3c3

    ; ── LDB offset=127 fixture: independent add-imm8(0x7F) encoding — imm8 RIGHT edge
    ; off=127 (0x7F) is the largest signed imm8 value [-128, 127]. Encoder MUST
    ; stay on the imm8 path (48 83 C0 + 1B imm8=0x7F). Independent asm encoding
    ; emits the same 4-byte imm8 primitive from raw bytes.
    lea rsi, [rel imm_0]
    call emit_movabs_rax
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    mov sil, 0x60
    call emit_load_state_rax
    call emit_add_rax_imm8_127    ; 48 83 C0 7F
    call emit_movzx_rax_byte_rax  ; 48 0F B6 00
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    call emit_ret
    ; = 42 bytes @ offset 271
    ; golden: 48b8000000000000000049898780020000c3498b87000300004883c07f480fb60049898780020000c3c3

    ; ── LDB offset=-128 fixture: independent add-imm8(0x80) encoding — imm8 LEFT edge
    ; off=-128 (0x80 as signed int8) is the SMALLEST signed imm8 value [-128, 127].
    ; Encoder MUST stay on the imm8 path (48 83 C0 + 1B imm8=0x80). Independent
    ; asm encoding emits the same 4-byte imm8 primitive from raw bytes. This is
    ; the imm8 LEFT edge; off=-129 forces imm32.
    lea rsi, [rel imm_0]
    call emit_movabs_rax
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    mov sil, 0x60
    call emit_load_state_rax
    call emit_add_rax_imm8_0x80    ; 48 83 C0 80 (signed -128)
    call emit_movzx_rax_byte_rax   ; 48 0F B6 00
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    call emit_ret
    ; = 42 bytes @ offset 313
    ; golden: 48b8000000000000000049898780020000c3498b87000300004883c080480fb60049898780020000c3c3

    ; ── LDB offset=128 fixture: independent add-imm32(0x80) encoding — imm32 LEFT edge
    ; off=128 (0x80) is the FIRST value past the signed imm8 range [-128, 127].
    ; Encoder MUST switch to the imm32 path (48 81 C0 + 4-byte LE imm32=0x80).
    ; If the encoder interprets imm8 as unsigned [0, 255], it would silently
    ; emit imm8 = 0x80 instead. STOP if asm emits 48 83 C0 80 for off=128.
    lea rsi, [rel imm_0]
    call emit_movabs_rax
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    mov sil, 0x60
    call emit_load_state_rax
    call emit_add_rax_imm32_0x80  ; 48 81 C0 80 00 00 00
    call emit_movzx_rax_byte_rax  ; 48 0F B6 00
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    call emit_ret
    ; = 45 bytes @ offset 355
    ; golden: 48b8000000000000000049898780020000c3498b87000300004881c080000000480fb60049898780020000c3c3

    ; ── LDB offset=256 fixture: independent add-imm32 encoding, compile-only
    ; offset=0x100 forces `add rax, imm32` (48 81 C0 + 4-byte LE imm32), NOT
    ; `add rax, imm8` (48 83 C0 + 1 byte). Independent asm encoding emits the
    ; same 7-byte imm32 primitive from raw bytes.
    lea rsi, [rel imm_0]
    call emit_movabs_rax
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    mov sil, 0x60
    call emit_load_state_rax
    call emit_add_rax_imm32_0x100 ; 48 81 C0 00 01 00 00
    call emit_movzx_rax_byte_rax  ; 48 0F B6 00
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    call emit_ret
    ; = 45 bytes @ offset 400
    ; golden: 48b8000000000000000049898780020000c3498b87000300004881c000010000480fb60049898780020000c3c3

    ; ── LDB offset=-129 fixture: independent add-imm32(0xFFFFFF7F) encoding — imm32 LEFT-edge (negative side)
    ; off=-129 (signed) is JUST PAST the signed imm8 range [-128, 127] on the
    ; negative side. Encoder MUST switch to imm32 path (48 81 C0 + 4-byte LE
    ; imm32 = 0xFFFFFF7F, signed -129). Symmetric with off=128 (imm32 LEFT-edge
    ; on positive side) and off=-128 (imm8 LEFT-edge on negative side).
    ; If the encoder silently truncated to imm8 (48 83 C0 7F), it would emit
    ; +127 instead of -129 (wrong sign/magnitude). STOP if so.
    lea rsi, [rel imm_0]
    call emit_movabs_rax
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    mov sil, 0x60
    call emit_load_state_rax
    call emit_add_rax_imm32_m129    ; 48 81 C0 7F FF FF FF (signed -129)
    call emit_movzx_rax_byte_rax    ; 48 0F B6 00
    mov sil, 0x50
    call emit_store_state_rax
    call emit_ret
    call emit_ret
    ; = 45 bytes @ offset 445
    ; golden: 48b8000000000000000049898780020000c3498b87000300004881c07fffffff480fb60049898780020000c3c3

    ; ── emit to stdout ─────────────────────────────────────────────
    call emit_flush

    ; exit(0)
    xor edi, edi
    mov eax, 60
    syscall

; ── primitives ────────────────────────────────────────────────────
; emit RET (C3)
emit_ret:
    emit_byte 0xC3
    ret

; emit NOP (90)
emit_nop:
    emit_byte 0x90
    ret

; movabs rax, imm64 — imm at [rsi]
emit_movabs_rax:
    emit_byte 0x48
    emit_byte 0xB8
    mov rax, [rsi]
    mov [rdi], rax
    add rdi, 8
    ret

; load_state slot → rax  (slot in sil)
emit_load_state_rax:
    emit_byte 0x49
    emit_byte 0x8B
    movzx ecx, sil
    shl ecx, 3
    cmp ecx, 127
    ja .ld_disp32
    emit_byte 0x47            ; ModRM: 01 000 111 (disp8, rax, r15)
    mov [rdi], cl
    inc rdi
    ret
.ld_disp32:
    emit_byte 0x87            ; ModRM: 10 000 111 (disp32, rax, r15)
    mov [rdi], ecx
    add rdi, 4
    ret

; load_state slot → rcx  (slot in sil)
emit_load_state_rcx:
    emit_byte 0x49
    emit_byte 0x8B
    movzx ecx, sil
    shl ecx, 3
    cmp ecx, 127
    ja .ldc_disp32
    emit_byte 0x4F            ; ModRM: 01 001 111 (disp8, rcx, r15)
    mov [rdi], cl
    inc rdi
    ret
.ldc_disp32:
    emit_byte 0x8F            ; ModRM: 10 001 111 (disp32, rcx, r15)
    mov [rdi], ecx
    add rdi, 4
    ret

; store_state rax → slot (slot in sil)
emit_store_state_rax:
    emit_byte 0x49
    emit_byte 0x89
    movzx ecx, sil
    shl ecx, 3
    cmp ecx, 127
    ja .st_disp32
    emit_byte 0x47            ; ModRM: 01 000 111 (disp8, rax, r15)
    mov [rdi], cl
    inc rdi
    ret
.st_disp32:
    emit_byte 0x87            ; ModRM: 10 000 111 (disp32, rax, r15)
    mov [rdi], ecx
    add rdi, 4
    ret

; inc rax
emit_inc_rax:
    emit_byte 0x48
    emit_byte 0xFF
    emit_byte 0xC0
    ret

; movzx rax, byte [rax]  (used by LDB; REX.W + 0F B6 + modrm=00)
emit_movzx_rax_byte_rax:
    emit_byte 0x48
    emit_byte 0x0F
    emit_byte 0xB6
    emit_byte 0x00
    ret

; add rax, imm8 8 — independent LDB offset fixture
emit_add_rax_imm8_8:
    emit_byte 0x48
    emit_byte 0x83
    emit_byte 0xC0
    emit_byte 0x08
    ret

; add rax, imm8 127 (0x7F) — independent LDB off127 boundary probe
; 48 83 C0 7F (REX.W + 83 /0 imm8) — largest signed imm8 value
emit_add_rax_imm8_127:
    emit_byte 0x48
    emit_byte 0x83
    emit_byte 0xC0
    emit_byte 0x7F
    ret

; add rax, imm8 -128 (0x80) — independent LDB offm128 boundary probe
; 48 83 C0 80 (REX.W + 83 /0 imm8=0x80) — smallest signed imm8 value
emit_add_rax_imm8_0x80:
    emit_byte 0x48
    emit_byte 0x83
    emit_byte 0xC0
    emit_byte 0x80
    ret

; add rax, imm32 0x80 — independent LDB off128 boundary probe
; 48 81 C0 80 00 00 00 (REX.W + 81 /0 + 4B LE imm32) — first value past
; the signed imm8 range [-128, 127]
emit_add_rax_imm32_0x80:
    emit_byte 0x48
    emit_byte 0x81
    emit_byte 0xC0
    emit_byte 0x80
    emit_byte 0x00
    emit_byte 0x00
    emit_byte 0x00
    ret

; add rax, imm32 0x100 — independent LDB offset=256 fixture
; 48 81 C0 00 01 00 00 (REX.W + opcode 81 /0 = ADD rax, imm32 LE)
emit_add_rax_imm32_0x100:
    emit_byte 0x48
    emit_byte 0x81
    emit_byte 0xC0
    emit_byte 0x00
    emit_byte 0x01
    emit_byte 0x00
    emit_byte 0x00
    ret

; add rax, imm32 -129 (0xFFFFFF7F signed) — independent LDB offm129 boundary probe
; 48 81 C0 7F FF FF FF (REX.W + opcode 81 /0 + 4B LE imm32 = signed -129)
; imm32 LEFT-edge on the NEGATIVE side; -128 stays on imm8 path (48 83 C0 80)
emit_add_rax_imm32_m129:
    emit_byte 0x48
    emit_byte 0x81
    emit_byte 0xC0
    emit_byte 0x7F
    emit_byte 0xFF
    emit_byte 0xFF
    emit_byte 0xFF
    ret

; dec rax
emit_dec_rax:
    emit_byte 0x48
    emit_byte 0xFF
    emit_byte 0xC8
    ret

; add rax, rcx
emit_add_rax_rcx:
    emit_byte 0x48
    emit_byte 0x01
    emit_byte 0xC8
    ret

; or rax, rcx  (ORV; MUST NOT alias add)
emit_or_rax_rcx:
    emit_byte 0x48
    emit_byte 0x09
    emit_byte 0xC8
    ret

; jmp rel32 — signed offset in esi (E9 imm32)
emit_jmp_rel32:
    emit_byte 0xE9
    mov [rdi], esi
    add rdi, 4
    ret

; call rel32 — signed offset in esi (E8 imm32); identical shape to jmp rel32
emit_call_rel32:
    emit_byte 0xE8
    mov [rdi], esi
    add rdi, 4
    ret

; ── sub rax, rcx (48 29 C8)
emit_sub_rax_rcx:
    emit_byte 0x48
    emit_byte 0x29
    emit_byte 0xC8
    ret

; ── imul rax, rcx (48 0F AF C1)
emit_mul_rax_rcx:
    emit_byte 0x48
    emit_byte 0x0F
    emit_byte 0xAF
    emit_byte 0xC1
    ret

; ── cmp rax, rcx (48 39 C8)
emit_cmp_rax_rcx:
    emit_byte 0x48
    emit_byte 0x39
    emit_byte 0xC8
    ret

; ── add rax, imm (signed; imm8 or imm32)
; imm64 value in rsi (low 32 bits for imm32 path)
emit_add_rax_imm:
    mov eax, esi               ; sign-extend through eax
    cmp eax, -128
    jl .add_imm32
    cmp eax, 127
    jg .add_imm32
    ; imm8 path: 48 83 C0 ib
    emit_byte 0x48
    emit_byte 0x83
    emit_byte 0xC0
    mov [rdi], sil
    inc rdi
    ret
.add_imm32:
    ; imm32 path: 48 81 C0 id
    emit_byte 0x48
    emit_byte 0x81
    emit_byte 0xC0
    mov [rdi], esi
    add rdi, 4
    ret

; ── sub rax, imm (signed; imm8 or imm32)
; imm64 value in rsi
emit_sub_rax_imm:
    mov eax, esi
    cmp eax, -128
    jl .sub_imm32
    cmp eax, 127
    jg .sub_imm32
    ; imm8 path: 48 83 E8 ib
    emit_byte 0x48
    emit_byte 0x83
    emit_byte 0xE8
    mov [rdi], sil
    inc rdi
    ret
.sub_imm32:
    ; imm32 path: 48 81 E8 id
    emit_byte 0x48
    emit_byte 0x81
    emit_byte 0xE8
    mov [rdi], esi
    add rdi, 4
    ret

; ── load_state slot → rsi (slot in sil)
emit_load_state_rsi:
    emit_byte 0x49
    emit_byte 0x8B
    movzx ecx, sil
    shl ecx, 3
    cmp ecx, 127
    ja .lds_disp32
    emit_byte 0x77            ; ModRM: 01 110 111 (disp8, rsi, r15)
    mov [rdi], cl
    inc rdi
    ret
.lds_disp32:
    emit_byte 0xB7            ; ModRM: 10 110 111 (disp32, rsi, r15)
    mov [rdi], ecx
    add rdi, 4
    ret

; ── load_state slot → rdi (slot in sil)
emit_load_state_rdi:
    emit_byte 0x49
    emit_byte 0x8B
    movzx ecx, sil
    shl ecx, 3
    cmp ecx, 127
    ja .ldd_disp32
    emit_byte 0x7F            ; ModRM: 01 111 111 (disp8, rdi, r15)
    mov [rdi], cl
    inc rdi
    ret
.ldd_disp32:
    emit_byte 0xBF            ; ModRM: 10 111 111 (disp32, rdi, r15)
    mov [rdi], ecx
    add rdi, 4
    ret

; ── store_state rsi → slot (slot in sil)
emit_store_state_rsi:
    emit_byte 0x49
    emit_byte 0x89
    movzx ecx, sil
    shl ecx, 3
    cmp ecx, 127
    ja .sts_disp32
    emit_byte 0x77            ; ModRM: 01 110 111 (disp8, rsi, r15)
    mov [rdi], cl
    inc rdi
    ret
.sts_disp32:
    emit_byte 0xB7            ; ModRM: 10 110 111 (disp32, rsi, r15)
    mov [rdi], ecx
    add rdi, 4
    ret

; ── store_state rdi → slot (slot in sil)
emit_store_state_rdi:
    emit_byte 0x49
    emit_byte 0x89
    movzx ecx, sil
    shl ecx, 3
    cmp ecx, 127
    ja .std_disp32
    emit_byte 0x7F            ; ModRM: 01 111 111 (disp8, rdi, r15)
    mov [rdi], cl
    inc rdi
    ret
.std_disp32:
    emit_byte 0xBF            ; ModRM: 10 111 111 (disp32, rdi, r15)
    mov [rdi], ecx
    add rdi, 4
    ret

; ── lea <reg>, [r15 + <reg>*8]  (7 bytes)
; reg=rdi (sil=7) or reg=rsi (sil=6)
; ModRM: 00 0rr 100 (SIB), SIB: 11 rrr 111
; REX.WRB = 0x4D, then 0x8B, modrm, sib, disp32=0
emit_lea_r15_scale8_rdi:
    emit_byte 0x4D            ; REX.WRB
    emit_byte 0x8B            ; lea
    emit_byte 0x3C            ; modrm: 00 111 100 (rdi, SIB)
    emit_byte 0x3F            ; SIB: 11 111 111 (scale=8, index=rdi, base=r15)
    emit_byte 0x00
    emit_byte 0x00
    emit_byte 0x00
    ret

emit_lea_r15_scale8_rsi:
    emit_byte 0x4D            ; REX.WRB
    emit_byte 0x8B            ; lea
    emit_byte 0x34            ; modrm: 00 110 100 (rsi, SIB)
    emit_byte 0x37            ; SIB: 11 110 111 (scale=8, index=rsi, base=r15)
    emit_byte 0x00
    emit_byte 0x00
    emit_byte 0x00
    ret

; ── JCC rel32 primitives (0F 8x + 4B imm32)
; signed offset in esi
%macro jcc_primitive 2
    emit_byte 0x0F
    emit_byte %1
    mov [rdi], esi
    add rdi, 4
    ret
%endmacro

emit_je_rel32:
    jcc_primitive 0x84
emit_jne_rel32:
    jcc_primitive 0x85
emit_jl_rel32:
    jcc_primitive 0x8C
emit_jge_rel32:
    jcc_primitive 0x8D
emit_jle_rel32:
    jcc_primitive 0x8E
emit_jg_rel32:
    jcc_primitive 0x8F
emit_jb_rel32:
    jcc_primitive 0x82
emit_jae_rel32:
    jcc_primitive 0x83
emit_jbe_rel32:
    jcc_primitive 0x86
emit_ja_rel32:
    jcc_primitive 0x87

; ── MEMCPY_DATA dst src n — rep movsb (stub: see Python peer asm.py)
; The Python DDC peer (asm.py) provides the canonical implementation.
; NASM probe uses the same byte sequences via direct emit_* calls.
emit_memcpy_data:
    ; Full implementation deferred to Python DDC peer (asm.py)
    ; To encode: load_state(RSI=src) + load_state(RDI=dst) + load_state(RCX=n) + FC
    emit_byte 0xFC
    ret

; ── MEMCPY_STATE dst src n — slot indices, lea-scale + rep movsb (stub)
emit_memcpy_state:
    ; Full implementation deferred to Python DDC peer (asm.py)
    emit_byte 0xFC
    ret

; flush outbuf[0 .. rdi-outbuf) to stdout (raw bytes)
emit_flush:
    push rcx
    push rdi
    push rsi
    lea rsi, [rel outbuf]
    mov rdx, rdi
    sub rdx, rsi               ; length = rdi - outbuf
    mov edi, 1                 ; fd=stdout
    mov eax, 1                 ; sys_write
    syscall
    pop rsi
    pop rdi
    pop rcx
    ret

section .data
align 8
imm_2a: dq 0x2A
imm_0:  dq 0x00
imm_cc: dq 0xCC

section .bss
align 8
state: resq 256
outbuf: resb 4096
