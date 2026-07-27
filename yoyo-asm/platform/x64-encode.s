; x64-encode.s — shared encode macros (included by yoyo-asm.s)
; Kept separate so DDC auditors can review encoding in isolation.
;
; Current primitive-probe beat keeps live encode helpers in yoyo-asm.s
; (INC/DEC/SET/GET/ADDV/ORV). This file remains the audit hook for later
; %include extractions — do not invent ISA here.
