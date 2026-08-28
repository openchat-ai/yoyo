#!/usr/bin/env python3
"""
platform_io.py — Python asm peer of platform_io.rs / platform-io.js (Stage 10-C).
Win32: kernel32 IAT at [r15 + slot*8]; Linux: inline syscalls; Stub: movabs+store.
Independently re-implemented (must not import yoyo-js).
"""

from __future__ import annotations

import struct

# Local primitives (mirror asm.py; keep this module importable standalone for gates)
def _rex(w, r, x, b):
    return 0x40 | (w << 3) | (r << 2) | (x << 1) | b


def load_state(slot, dest_low3, dest_rex):
    disp = (slot & 0xFF) * 8
    r = _rex(1, dest_rex, 0, 1)
    modrm_reg = (dest_low3 & 7) << 3
    if disp <= 127:
        return bytes([r, 0x8B, modrm_reg | 0x40 | 0x07, disp])
    return bytes([r, 0x8B, modrm_reg | 0x80 | 0x07]) + struct.pack("<I", disp)


def store_state(slot, src_low3, src_rex):
    disp = (slot & 0xFF) * 8
    r = _rex(1, src_rex, 0, 1)
    modrm_reg = (src_low3 & 7) << 3
    if disp <= 127:
        return bytes([r, 0x89, modrm_reg | 0x40 | 0x07, disp])
    return bytes([r, 0x89, modrm_reg | 0x80 | 0x07]) + struct.pack("<I", disp)


def movabs_rax(imm):
    imm = int(imm) & 0xFFFFFFFFFFFFFFFF
    return bytes([0x48, 0xB8]) + struct.pack("<Q", imm)


def movabs_rdx(imm):
    imm = int(imm) & 0xFFFFFFFFFFFFFFFF
    return bytes([0x48, 0xBA]) + struct.pack("<Q", imm)


def movabs_rsi(imm):
    imm = int(imm) & 0xFFFFFFFFFFFFFFFF
    return bytes([0x48, 0xBE]) + struct.pack("<Q", imm)


WIN32_IAT_VIRTUAL_ALLOC = 0
WIN32_IAT_CREATE_FILE = 1
WIN32_IAT_READ_FILE = 2
WIN32_IAT_WRITE_FILE = 3
WIN32_IAT_CLOSE_HANDLE = 4

STR_TABLE_OFF = 0x10000
STR_ENTRY_SIZE = 64
READ_CHUNK = 0x10000


def _u32le(n):
    return struct.pack("<I", n & 0xFFFFFFFF)


def emit_call_r15_iat(slot):
    return bytes([0x41, 0xFF, 0x97]) + _u32le(slot * 8)


def emit_lea_r15(dest_low3, dest_rex, disp):
    rex = 0x49 | (0x04 if dest_rex else 0)
    return bytes([rex, 0x8D, 0x87 | ((dest_low3 & 7) << 3)]) + _u32le(disp)


def str_path_off(str_idx):
    return STR_TABLE_OFF + (str_idx & 0xFF) * STR_ENTRY_SIZE


def emit_stub_io(slot, imm):
    return movabs_rax(imm or 0) + store_state(slot, 0, 0)


def emit_win32_alloc(slot, size):
    out = bytearray()
    out.extend([0x48, 0x83, 0xEC, 0x28])  # sub rsp, 0x28
    out.extend([0x31, 0xC9])  # xor ecx, ecx
    if size <= 0xFFFFFFFF:
        out.extend([0xBA])
        out.extend(_u32le(size))
    else:
        out.extend(movabs_rdx(size))
    out.extend([0x41, 0xB8, 0x00, 0x30, 0x00, 0x00])  # r8 = MEM_COMMIT|RESERVE
    out.extend([0x41, 0xB9, 0x04, 0x00, 0x00, 0x00])  # r9 = PAGE_READWRITE
    out.extend(emit_call_r15_iat(WIN32_IAT_VIRTUAL_ALLOC))
    out.extend([0x48, 0x83, 0xC4, 0x28])
    out.extend(store_state(slot, 0, 0))
    return bytes(out)


def emit_win32_load_file(slot, str_idx):
    path = str_path_off(str_idx)
    out = bytearray()
    out.extend([0x48, 0x83, 0xEC, 0x28])
    out.extend(emit_lea_r15(1, 0, path))  # lea rcx, [r15+path]
    out.extend([0xBA, 0x00, 0x00, 0x00, 0x80])  # GENERIC_READ
    out.extend([0x45, 0x31, 0xC0])  # xor r8d, r8d
    out.extend([0x45, 0x31, 0xC9])  # xor r9d, r9d
    out.extend([0xC7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00])  # OPEN_EXISTING
    out.extend([0x48, 0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00])
    out.extend([0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00])
    out.extend(emit_call_r15_iat(WIN32_IAT_CREATE_FILE))
    out.extend([0x48, 0x89, 0xC3])  # mov rbx, rax

    out.extend([0x31, 0xC9])
    out.extend([0xBA])
    out.extend(_u32le(READ_CHUNK))
    out.extend([0x41, 0xB8, 0x00, 0x30, 0x00, 0x00])
    out.extend([0x41, 0xB9, 0x04, 0x00, 0x00, 0x00])
    out.extend(emit_call_r15_iat(WIN32_IAT_VIRTUAL_ALLOC))
    out.extend([0x48, 0x89, 0xC6])  # mov rsi, rax

    out.extend([0x48, 0x89, 0xD9])
    out.extend([0x48, 0x89, 0xF2])
    out.extend([0x41, 0xB8])
    out.extend(_u32le(READ_CHUNK))
    out.extend([0x4C, 0x8D, 0x4C, 0x24, 0x20])
    out.extend([0x48, 0xC7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00])
    out.extend(emit_call_r15_iat(WIN32_IAT_READ_FILE))

    out.extend([0x48, 0x89, 0xD9])
    out.extend(emit_call_r15_iat(WIN32_IAT_CLOSE_HANDLE))
    out.extend([0x48, 0x83, 0xC4, 0x28])
    out.extend([0x48, 0x89, 0xF0])
    out.extend(store_state(slot, 0, 0))
    return bytes(out)


def emit_win32_write_file(slot, str_idx, sz_slot):
    path = str_path_off(str_idx)
    out = bytearray()
    out.extend([0x48, 0x83, 0xEC, 0x28])
    out.extend(emit_lea_r15(1, 0, path))
    out.extend([0xBA, 0x00, 0x00, 0x00, 0x40])  # GENERIC_WRITE
    out.extend([0x45, 0x31, 0xC0])
    out.extend([0x45, 0x31, 0xC9])
    out.extend([0xC7, 0x44, 0x24, 0x20, 0x02, 0x00, 0x00, 0x00])  # CREATE_ALWAYS
    out.extend([0x48, 0xC7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00])
    out.extend([0x48, 0xC7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00])
    out.extend(emit_call_r15_iat(WIN32_IAT_CREATE_FILE))
    out.extend([0x48, 0x89, 0xC3])

    out.extend(load_state(slot, 2, 0))  # rdx = buf
    out.extend(load_state(sz_slot, 0, 1))  # r8 = size (REX.R)
    out.extend([0x48, 0x89, 0xD9])
    out.extend([0x4C, 0x8D, 0x4C, 0x24, 0x20])
    out.extend([0x48, 0xC7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00])
    out.extend(emit_call_r15_iat(WIN32_IAT_WRITE_FILE))

    out.extend([0x48, 0x89, 0xD9])
    out.extend(emit_call_r15_iat(WIN32_IAT_CLOSE_HANDLE))
    out.extend([0x48, 0x83, 0xC4, 0x28])
    out.extend(movabs_rax(0))
    out.extend(store_state(slot, 0, 0))
    return bytes(out)


def emit_linux_alloc(slot, size):
    out = bytearray()
    out.extend([0x48, 0x31, 0xFF])  # xor rdi, rdi
    if size <= 0xFFFFFFFF:
        out.extend([0x48, 0xC7, 0xC6])
        out.extend(_u32le(size))
    else:
        out.extend(movabs_rsi(size))
    out.extend([0x48, 0xC7, 0xC2, 0x03, 0x00, 0x00, 0x00])
    out.extend([0x49, 0xC7, 0xC2, 0x22, 0x00, 0x00, 0x00])
    out.extend([0x49, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF])
    out.extend([0x4D, 0x31, 0xC9])
    out.extend([0xB8, 0x09, 0x00, 0x00, 0x00])
    out.extend([0x0F, 0x05])
    out.extend(store_state(slot, 0, 0))
    return bytes(out)


def emit_linux_load_file(slot, str_idx):
    path = str_path_off(str_idx)
    out = bytearray()
    out.extend(emit_lea_r15(7, 0, path))  # lea rdi
    out.extend([0x31, 0xF6])
    out.extend([0xB8, 0x02, 0x00, 0x00, 0x00])
    out.extend([0x0F, 0x05])
    out.extend([0x49, 0x89, 0xC4])  # mov r12, rax

    out.extend([0x48, 0x31, 0xFF])
    out.extend([0xBE])
    out.extend(_u32le(READ_CHUNK))
    out.extend([0x48, 0xC7, 0xC2, 0x03, 0x00, 0x00, 0x00])
    out.extend([0x49, 0xC7, 0xC2, 0x22, 0x00, 0x00, 0x00])
    out.extend([0x49, 0xC7, 0xC0, 0xFF, 0xFF, 0xFF, 0xFF])
    out.extend([0x4D, 0x31, 0xC9])
    out.extend([0xB8, 0x09, 0x00, 0x00, 0x00])
    out.extend([0x0F, 0x05])
    out.extend([0x49, 0x89, 0xC5])

    out.extend([0xB8, 0x00, 0x00, 0x00, 0x00])
    out.extend([0x4C, 0x89, 0xE7])
    out.extend([0x4C, 0x89, 0xEE])
    out.extend([0xBA])
    out.extend(_u32le(READ_CHUNK))
    out.extend([0x0F, 0x05])

    out.extend([0x4C, 0x89, 0xE7])
    out.extend([0xB8, 0x03, 0x00, 0x00, 0x00])
    out.extend([0x0F, 0x05])

    out.extend(movabs_rax(0))
    out.extend([0x4C, 0x89, 0xE8])
    out.extend(store_state(slot, 0, 0))
    return bytes(out)


def emit_linux_write_file(slot, str_idx, sz_slot):
    path = str_path_off(str_idx)
    out = bytearray()
    out.extend(emit_lea_r15(7, 0, path))
    out.extend([0xBE, 0x41, 0x02, 0x00, 0x00])
    out.extend([0xBA, 0xB6, 0x01, 0x00, 0x00])
    out.extend([0xB8, 0x02, 0x00, 0x00, 0x00])
    out.extend([0x0F, 0x05])
    out.extend([0x49, 0x89, 0xC4])

    out.extend(load_state(slot, 6, 0))  # rsi
    out.extend(load_state(sz_slot, 2, 0))  # rdx
    out.extend([0x4C, 0x89, 0xE7])
    out.extend([0xB8, 0x01, 0x00, 0x00, 0x00])
    out.extend([0x0F, 0x05])

    out.extend([0x4C, 0x89, 0xE7])
    out.extend([0xB8, 0x03, 0x00, 0x00, 0x00])
    out.extend([0x0F, 0x05])

    out.extend(movabs_rax(0))
    out.extend(store_state(slot, 0, 0))
    return bytes(out)


def encode_io_op(op, args, platform="stub"):
    """Encode ALLOC / LOAD_FILE / WRITE_FILE for platform in {stub, win32, linux}."""
    slot = args[0] if len(args) > 0 else 0
    a1 = args[1] if len(args) > 1 else 0
    a2 = args[2] if len(args) > 2 else 0
    p = platform or "stub"
    if p == "stub":
        return emit_stub_io(slot, a1)
    if p == "win32":
        if op == 0x20:
            return emit_win32_alloc(slot, a1)
        if op == 0x50:
            return emit_win32_load_file(slot, a1)
        if op == 0x51:
            return emit_win32_write_file(slot, a1, a2)
    if p == "linux":
        if op == 0x20:
            return emit_linux_alloc(slot, a1)
        if op == 0x50:
            return emit_linux_load_file(slot, a1)
        if op == 0x51:
            return emit_linux_write_file(slot, a1, a2)
    return emit_stub_io(slot, a1)


def is_movabs_store_stub(data: bytes) -> bool:
    """True if bytes look like movabs+store stub (blind-zone signature)."""
    b = bytes(data)
    return len(b) >= 12 and b[0] == 0x48 and b[1] == 0xB8
