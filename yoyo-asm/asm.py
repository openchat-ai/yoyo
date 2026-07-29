#!/usr/bin/env python3
"""
yoyo-asm.py — Ground-truth DDC peer (3rd independent implementation).
Reads .ty hex tokens → x64 bytes → PE32+ executable.
Independently encodes all 38 opcodes with byte-level equality to JS/Rust peers.

Usage:
    python yoyo-asm/asm.py <input.ty> <output.exe>
    python yoyo-asm/asm.py <input.ty> <output.exe> --target=win32|linux|stub
"""

import struct
import sys
import os
import hashlib


# ── x64 primitive encoders (independent implementation) ──────────────

def rex(w, r, x, b):
    """Build REX prefix byte."""
    return 0x40 | (w << 3) | (r << 2) | (x << 1) | b


def load_state_bytes(slot, dest_low3, dest_rex):
    """mov <dest>, [r15 + slot*8]"""
    disp = (slot & 0xFF) * 8
    r = rex(1, dest_rex, 0, 1)  # W=1, B=1 (R15 base)
    modrm_reg = (dest_low3 & 7) << 3
    if disp <= 127:
        return bytes([r, 0x8B, modrm_reg | 0x40 | 0x07, disp])
    else:
        return bytes([r, 0x8B, modrm_reg | 0x80 | 0x07]) + struct.pack('<I', disp)


def store_state_bytes(slot, src_low3, src_rex):
    """mov [r15 + slot*8], <src>"""
    disp = (slot & 0xFF) * 8
    r = rex(1, src_rex, 0, 1)  # W=1, B=1 (R15 base)
    modrm_reg = (src_low3 & 7) << 3
    if disp <= 127:
        return bytes([r, 0x89, modrm_reg | 0x40 | 0x07, disp])
    else:
        return bytes([r, 0x89, modrm_reg | 0x80 | 0x07]) + struct.pack('<I', disp)


def load_state_rax(slot):
    return load_state_bytes(slot, 0, 0)


def load_state_rcx(slot):
    return load_state_bytes(slot, 1, 0)


def load_state_rsi(slot):
    return load_state_bytes(slot, 6, 0)


def load_state_rdi(slot):
    return load_state_bytes(slot, 7, 0)


def store_state_rax(slot):
    return store_state_bytes(slot, 0, 0)


def movabs_rax(imm):
    """movabs rax, imm64 (10 bytes)"""
    if isinstance(imm, int):
        imm = imm & 0xFFFFFFFFFFFFFFFF
    return bytes([0x48, 0xB8]) + struct.pack('<Q', imm & 0xFFFFFFFFFFFFFFFF)


def add_rax_imm(imm):
    """add rax, imm (signed; imm8 or imm32)"""
    if -128 <= imm <= 127:
        return bytes([0x48, 0x83, 0xC0, imm & 0xFF])
    return bytes([0x48, 0x81, 0xC0]) + struct.pack('<i', imm)


def sub_rax_imm(imm):
    """sub rax, imm (signed; imm8 or imm32)"""
    if -128 <= imm <= 127:
        return bytes([0x48, 0x83, 0xE8, imm & 0xFF])
    return bytes([0x48, 0x81, 0xE8]) + struct.pack('<i', imm)


def add_rax_rcx():
    return bytes([0x48, 0x01, 0xC8])


def sub_rax_rcx():
    return bytes([0x48, 0x29, 0xC8])


def or_rax_rcx():
    return bytes([0x48, 0x09, 0xC8])


def mul_rax_rcx():
    return bytes([0x48, 0x0F, 0xAF, 0xC1])


def cmp_rax_rcx():
    return bytes([0x48, 0x39, 0xC8])


def inc_rax():
    return bytes([0x48, 0xFF, 0xC0])


def dec_rax():
    return bytes([0x48, 0xFF, 0xC8])


def movzx_rax_byte_rax():
    """movzx rax, byte [rax]"""
    return bytes([0x48, 0x0F, 0xB6, 0x00])


def ret_bytes():
    return bytes([0xC3])


def nop():
    return bytes([0x90])


def jmp_rel32(offset):
    """jmp rel32 (E9 imm32)"""
    return bytes([0xE9]) + struct.pack('<i', offset)


def call_rel32(offset):
    """call rel32 (E8 imm32)"""
    return bytes([0xE8]) + struct.pack('<i', offset)


# JCC second bytes (matching JS: 0x71 -> 0x84, 0x72 -> 0x85, ..., 0x7A -> 0x87)
JCC_MAP = {
    0x71: 0x84,  # JE
    0x72: 0x85,  # JNE
    0x73: 0x8C,  # JL
    0x74: 0x8D,  # JGE
    0x75: 0x8E,  # JLE
    0x76: 0x8F,  # JG
    0x77: 0x82,  # JB
    0x78: 0x83,  # JAE
    0x79: 0x86,  # JBE
    0x7A: 0x87,  # JA
}


def jcc_rel32(jcc_byte, offset):
    """j<cc> rel32 (0F 8x imm32)"""
    return bytes([0x0F, JCC_MAP[jcc_byte]]) + struct.pack('<i', offset)


def lea_r15_scale8(reg_low3):
    """lea <reg>, [r15 + <reg>*8] (7 bytes)"""
    rex_byte = 0x40 | 8 | 1  # W=1, R=0, X=0, B=1 (R15 base)
    modrm = (reg_low3 << 3) | 0x04  # mod=00, rm=100 (SIB)
    sib = (3 << 6) | ((reg_low3 & 7) << 3) | 7  # scale=8, index=reg, base=R15
    return bytes([rex_byte, 0x8B, modrm, sib, 0x00, 0x00, 0x00])


# ── Opcode → bytes encoder (matches JS encodeOp + Rust emit_one) ─────

def encode_op(op, args, branch_placeholder=False):
    """
    Encode one TIR instruction to x64 bytes.
    args: list of ints (already resolved from hex/names).
    branch_placeholder: True for forward branches (emit placeholder rel32).
    """
    a = lambda i: args[i] if i < len(args) else 0

    if op == 0xFF:  # RET
        return ret_bytes()

    if op == 0x00:  # NOP
        return nop()

    if op == 0xA0:  # RAW_BYTE
        return bytes([a(0) & 0xFF])

    if op == 0xA1:  # RAW_BYTES
        return bytes([x & 0xFF for x in args])

    if op == 0x41:  # CALL
        return call_rel32(0) if branch_placeholder else ret_bytes()

    if op == 0x70:  # JMP
        return jmp_rel32(0) if branch_placeholder else ret_bytes()

    if 0x71 <= op <= 0x7A:  # Jcc
        return jcc_rel32(op, 0) if branch_placeholder else ret_bytes()

    if op == 0x30:  # SET slot imm
        # movabs rax, imm + store_state slot, rax
        return movabs_rax(a(1)) + store_state_rax(a(0))

    if op == 0x20:  # ALLOC slot size
        # movabs rax, size + store_state slot, rax
        return movabs_rax(a(1)) + store_state_rax(a(0))

    if op == 0x50:  # LOAD_FILE slot str_idx
        return movabs_rax(a(1)) + store_state_rax(a(0))

    if op == 0x51:  # WRITE_FILE slot str_idx sz
        return movabs_rax(a(1)) + store_state_rax(a(0))

    if op == 0x60 or op == 0x64:  # GET / MOVRR dst src
        return load_state_rax(a(1)) + store_state_rax(a(0))

    if op == 0x61:  # SUB slot imm
        return load_state_rax(a(0)) + sub_rax_imm(a(1)) + store_state_rax(a(0))

    if op == 0x62:  # ADD slot imm
        return load_state_rax(a(0)) + add_rax_imm(a(1)) + store_state_rax(a(0))

    if op == 0x63:  # IMUL dst src
        return load_state_rax(a(0)) + load_state_rcx(a(1)) + mul_rax_rcx() + store_state_rax(a(0))

    if op == 0x65:  # CMP a b
        return load_state_rax(a(0)) + load_state_rcx(a(1)) + cmp_rax_rcx()

    if op == 0x66:  # INC slot
        return load_state_rax(a(0)) + inc_rax() + store_state_rax(a(0))

    if op == 0x67:  # DEC slot
        return load_state_rax(a(0)) + dec_rax() + store_state_rax(a(0))

    if op == 0x68:  # ADDV dst src
        return load_state_rax(a(0)) + load_state_rcx(a(1)) + add_rax_rcx() + store_state_rax(a(0))

    if op == 0x69:  # ORV dst src
        return load_state_rax(a(0)) + load_state_rcx(a(1)) + or_rax_rcx() + store_state_rax(a(0))

    if op == 0x6A:  # SUBV dst src
        return load_state_rax(a(0)) + load_state_rcx(a(1)) + sub_rax_rcx() + store_state_rax(a(0))

    if op == 0x80:  # LDB dd ss oo
        out = [load_state_rax(a(1))]
        if a(2):
            out.append(add_rax_imm(a(2)))
        out.append(movzx_rax_byte_rax())
        out.append(store_state_rax(a(0)))
        return b''.join(out)

    if op == 0x84:  # MEMCPY_DATA dst src n
        return load_state_rsi(a(1)) + load_state_rdi(a(0)) + load_state_rcx(a(2)) + bytes([0xFC])

    if op == 0x85:  # MEMCPY_STATE dst src n
        return (load_state_rdi(a(0)) + lea_r15_scale8(7) +
                load_state_rsi(a(1)) + lea_r15_scale8(6) +
                load_state_rcx(a(2)) + bytes([0xFC]))

    if op == 0x40:  # HANDLER (label) — handled by caller
        return b''

    if op in (0x10, 0x12, 0x13):  # DATA, STR, RAW — data section
        return b''

    return nop()  # fallback


# ── .ty parser ──────────────────────────────────────────────────────

def parse_hex_u64(s):
    """Parse hex token (with or without 0x prefix)."""
    if s.startswith('0x') or s.startswith('0X'):
        s = s[2:]
    return int(s, 16)


def looks_like_hex(s):
    t = s[2:] if (s.startswith('0x') or s.startswith('0X')) else s
    return len(t) > 0 and all(c in '0123456789abcdefABCDEF' for c in t)


class NameTable:
    """Named slot resolver (matches JS/Rust behavior)."""
    USER_SLOT_BASE = 0x50

    def __init__(self):
        self.names = {}
        self.next_slot = self.USER_SLOT_BASE

    def resolve(self, tok):
        if looks_like_hex(tok):
            return parse_hex_u64(tok)
        if tok in self.names:
            return self.names[tok]
        slot = self.next_slot
        self.next_slot += 1
        self.names[tok] = slot
        return slot


def parse_ty(source):
    """Parse .ty source into list of (opcode, args) tuples."""
    lines = []
    names = NameTable()
    for raw in source.splitlines():
        # Strip comments
        line = raw.split(';')[0].split('#')[0].strip()
        if not line:
            continue
        if line.upper() in ('LAYOUT', 'END_LAYOUT'):
            continue
        toks = line.split()
        op = parse_hex_u64(toks[0])
        args = [names.resolve(t) for t in toks[1:]]
        lines.append((op, args))
    return lines


# ── Compiler: two-pass emit + fixup ─────────────────────────────────

def compile_lines(lines):
    """
    Compile parsed .ty lines to code bytes + data bytes.
    Two-pass: emit with placeholder rel32, then fixup.
    """
    code = bytearray()
    data = bytearray()
    labels = {}
    fixups = []  # (rel_at, hh)

    for op, args in lines:
        if op == 0x40:  # HANDLER hh — label definition
            hh = args[0]
            if hh > 0xFFFF:
                raise ValueError(f"Handler id out of range: {hh}")
            labels[hh] = len(code)
            continue

        if op in (0x10, 0x12, 0x13):  # DATA / STR / RAW — data payload
            for a in args:
                data.append(a & 0xFF)
            continue

        if op == 0x41 or op == 0x70 or (0x71 <= op <= 0x7A):
            # Branch: emit placeholder, record fixup
            start = len(code)
            bytes_out = encode_op(op, args, branch_placeholder=True)
            code.extend(bytes_out)
            rel_at = start + 2 if (0x71 <= op <= 0x7A) else start + 1
            hh = args[0]
            fixups.append((rel_at, hh))
            continue

        # Regular opcode
        bytes_out = encode_op(op, args, branch_placeholder=False)
        code.extend(bytes_out)

    # Pass 2: fixup rel32
    for rel_at, hh in fixups:
        if hh not in labels:
            raise ValueError(f"Undefined label H_{hh:04X}")
        target = labels[hh]
        rel = target - (rel_at + 4)
        struct.pack_into('<i', code, rel_at, rel)

    return bytes(code), bytes(data), labels


# ── PE32+ builder (independent implementation) ───────────────────────

def align_up(v, alignment):
    return (v + alignment - 1) & ~(alignment - 1)


def build_pe(code, data, data_need=0x38000):
    """
    Build PE32+ image matching JS/Rust peer output.
    Layout:
      - DOS header + PE signature + COFF + Optional header
      - .text section (startup + user code)
      - .data section
    """
    section_align = 0x1000
    file_align = 0x200
    headers_raw = 0x400
    startup_len = 13  # lea r15 (7) + jmp rel32 (5) + nop pad (1)

    code_raw = align_up(len(code) + startup_len, file_align)
    data_vs = max(data_need, align_up(len(data) + 0x1000, section_align))
    data_raw = align_up(data_vs, file_align)

    text_rva = 0x1000
    text_vs = align_up(len(code) + startup_len, section_align)
    data_rva = text_rva + text_vs
    size_of_image = align_up(data_rva + data_vs, section_align)

    img = bytearray(headers_raw + code_raw + data_raw)

    # DOS header
    img[0:2] = b'MZ'
    struct.pack_into('<I', img, 0x3C, 0x80)  # e_lfanew

    # PE signature
    img[0x80:0x84] = b'PE\x00\x00'

    # COFF header
    struct.pack_into('<H', img, 0x84, 0x8664)  # Machine AMD64
    struct.pack_into('<H', img, 0x86, 2)       # NumberOfSections
    struct.pack_into('<H', img, 0x94, 0xF0)    # SizeOfOptionalHeader
    struct.pack_into('<H', img, 0x96, 0x22)    # Characteristics

    # Optional header (PE32+)
    opt = 0x98
    struct.pack_into('<H', img, opt, 0x20B)    # PE32+ magic
    img[opt + 2] = 1                           # MajorLinkerVersion
    struct.pack_into('<I', img, opt + 16, text_rva)  # AddressOfEntryPoint
    struct.pack_into('<Q', img, opt + 24, 0x140000000)  # ImageBase
    struct.pack_into('<I', img, opt + 32, section_align)
    struct.pack_into('<I', img, opt + 36, file_align)
    struct.pack_into('<H', img, opt + 40, 6)   # MajorOS
    struct.pack_into('<H', img, opt + 44, 6)   # MajorSubsystem
    struct.pack_into('<I', img, opt + 56, size_of_image)
    struct.pack_into('<I', img, opt + 60, headers_raw)
    struct.pack_into('<H', img, opt + 68, 3)   # Subsystem = CONSOLE
    struct.pack_into('<H', img, opt + 70, 0x8160)  # DllCharacteristics
    struct.pack_into('<Q', img, opt + 72, 0x100000)  # Stack Reserve
    struct.pack_into('<Q', img, opt + 80, 0x1000)    # Stack Commit
    struct.pack_into('<Q', img, opt + 88, 0x100000)  # Heap Reserve
    struct.pack_into('<Q', img, opt + 96, 0x1000)    # Heap Commit
    struct.pack_into('<I', img, opt + 108, 16)  # NumberOfRvaAndSizes

    # SizeOfCode / SizeOfInitializedData
    struct.pack_into('<I', img, opt + 4, code_raw)
    struct.pack_into('<I', img, opt + 8, data_raw)
    struct.pack_into('<I', img, opt + 20, text_rva)  # BaseOfCode

    # Section .text
    s1 = 0x98 + 0xF0  # 0x188
    img[s1:s1 + 8] = b'.text\x00\x00\x00'
    struct.pack_into('<I', img, s1 + 8, text_vs)    # VirtualSize
    struct.pack_into('<I', img, s1 + 12, text_rva)  # VirtualAddress
    struct.pack_into('<I', img, s1 + 16, code_raw)  # SizeOfRawData
    struct.pack_into('<I', img, s1 + 20, headers_raw)  # PointerToRawData
    struct.pack_into('<I', img, s1 + 36, 0x60000020)  # Characteristics

    # Section .data
    s2 = s1 + 40
    img[s2:s2 + 8] = b'.data\x00\x00\x00'
    struct.pack_into('<I', img, s2 + 8, data_vs)    # VirtualSize
    struct.pack_into('<I', img, s2 + 12, data_rva)  # VirtualAddress
    struct.pack_into('<I', img, s2 + 16, data_raw)  # SizeOfRawData
    struct.pack_into('<I', img, s2 + 20, headers_raw + code_raw)  # PointerToRawData
    struct.pack_into('<I', img, s2 + 36, 0xC0000040)  # Characteristics

    # Startup code at start of .text
    text_off = headers_raw

    # lea r15, [rip + disp32]  (7 bytes)
    # After this insn, RIP = text_rva + 7
    # Want r15 = imagebase + data_rva → disp = data_rva - (text_rva + 7)
    lea_disp = data_rva - (text_rva + 7)
    img[text_off] = 0x4C
    img[text_off + 1] = 0x8D
    img[text_off + 2] = 0x3D  # ModRM: r15, [rip+disp]
    struct.pack_into('<i', img, text_off + 3, lea_disp)

    # jmp rel32 to user code (after startup)
    jmp_from = text_rva + 7
    user_code_rva = text_rva + startup_len
    jmp_rel = user_code_rva - (jmp_from + 5)
    img[text_off + 7] = 0xE9
    struct.pack_into('<i', img, text_off + 8, jmp_rel)
    img[text_off + 12] = 0x90  # nop pad → startup_len = 13

    # Copy user code
    code_dst = text_off + startup_len
    img[code_dst:code_dst + len(code)] = code

    # Copy data
    data_off = headers_raw + code_raw
    copy_n = min(len(data), data_raw)
    img[data_off:data_off + copy_n] = data[:copy_n]

    return bytes(img)


# ── CLI ──────────────────────────────────────────────────────────────

def main():
    args = sys.argv[1:]
    if len(args) < 2:
        print(f"Usage: {sys.argv[0]} <input.ty> <output.exe> [--target=win32|linux|stub]",
              file=sys.stderr)
        sys.exit(2)

    in_file = args[0]
    out_file = args[1]
    target = "win32"
    for a in args[2:]:
        if a.startswith("--target="):
            target = a.split("=", 1)[1]

    if not os.path.exists(in_file):
        print(f"error: input file not found: {in_file}", file=sys.stderr)
        sys.exit(1)

    with open(in_file, 'r', encoding='utf-8') as f:
        src = f.read()

    lines = parse_ty(src)
    code, data, labels = compile_lines(lines)

    if target == "win32":
        pe = build_pe(code, data)
        os.makedirs(os.path.dirname(os.path.abspath(out_file)) or '.', exist_ok=True)
        with open(out_file, 'wb') as f:
            f.write(pe)
        print(f"yoyo-asm: {in_file} → {out_file} ({len(pe)} bytes, code={len(code)}, data={len(data)})")
    elif target == "stub":
        os.makedirs(os.path.dirname(os.path.abspath(out_file)) or '.', exist_ok=True)
        with open(out_file, 'wb') as f:
            f.write(code)
        print(f"yoyo-asm: {in_file} → {out_file} ({len(code)} bytes, stub)")
    else:
        print(f"error: target '{target}' not yet supported", file=sys.stderr)
        sys.exit(1)

    # Compute entry handler
    entry_hh = None
    for hh, offset in sorted(labels.items(), key=lambda x: x[1]):
        entry_hh = hh
        break
    if entry_hh is not None:
        print(f"  entry H_{entry_hh:04X}, {len(labels)} handlers")


if __name__ == '__main__':
    main()