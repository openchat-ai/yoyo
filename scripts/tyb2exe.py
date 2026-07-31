#!/usr/bin/env python3
"""
tyb2exe.py — Paper-tape compiler: reads .tyb, emits .exe.

Architecture: pure paper tape machine.
  .tyb (8B records) → dispatch → emit table → .exe

No parser, no offsets, no pointers. Just sequential read → dispatch.
"""
import struct, os, sys, hashlib

# ── Import emit table from asm.py ───────────────────────────────
sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', 'yoyo-asm'))
from asm import encode_op, build_pe, ret_bytes, nop

# ── .tyb format ─────────────────────────────────────────────────
# [magic:4] = TYB\0
# [entry_hh:2][rec_cnt:2]  — header
# [op:1][argc:1][a0:2][a1:2][a2:2]  × rec_cnt  — records
# [rec_idx:4][label_hh:2]  × fixup_cnt — fixup table (after records)
# [data_bytes]  — data section

# ── Paper tape compiler ──────────────────────────────────────────
def compile_tyb(tyb_data):
    """Compile .tyb to code + data + fixups."""
    assert tyb_data[:4] == b'TYB\0', 'Not a valid .tyb file'
    
    entry_hh, rec_cnt = struct.unpack_from('<HH', tyb_data, 4)
    records = []
    labels = {}
    data_bytes = bytearray()
    fixups = []
    
    # Parse records
    off = 8  # header size
    for i in range(rec_cnt):
        if off + 8 > len(tyb_data):
            break
        op, argc, a0, a1, a2 = struct.unpack_from('<BBHHH', tyb_data, off)
        args = [a0, a1, a2][:argc]
        records.append((op, args))
        off += 8
    
    # Rest is fixup table + data
    # Each fixup entry: [rec_idx:4][label_hh:2] = 6 bytes
    # Remaining bytes after fixups are data
    # We know the fixup count from the records that have branches
    fixup_raw = tyb_data[off:]
    fixup_entries = []
    fixup_data = bytearray()
    
    # Parse fixups: we know which records are branches
    fixup_idx = 0
    for i, (op, args) in enumerate(records):
        is_branch = (op == 0x41 or op == 0x70 or (0x71 <= op <= 0x7A))
        if is_branch and args:
            if fixup_idx * 6 + 6 <= len(fixup_raw):
                rec_idx, hh = struct.unpack_from('<IH', fixup_raw, fixup_idx * 6)
                fixups.append((rec_idx, hh))
                fixup_idx += 1
                labels[hh] = 0  # placeholder, will resolve
    
    # Remaining bytes after fixups are data
    if fixup_idx * 6 < len(fixup_raw):
        fixup_data = bytearray(fixup_raw[fixup_idx * 6:])
    
    # ── Pass 1: emit with placeholder rel32 ──────────────────────
    code = bytearray()
    label_offsets = {}
    record_fixups = []  # [(code_offset, rel_at, hh)]
    
    for i, (op, args) in enumerate(records):
        if op == 0x40:  # label
            label_offsets[args[0]] = len(code)
            continue
        
        is_branch = (op == 0x41 or op == 0x70 or (0x71 <= op <= 0x7A))
        if is_branch:
            start = len(code)
            bytes_out = encode_op(op, args, branch_placeholder=True)
            code.extend(bytes_out)
            rel_at = start + 2 if (0x71 <= op <= 0x7A) else start + 1
            record_fixups.append((start, rel_at, args[0]))
            continue
        
        code.extend(encode_op(op, args, branch_placeholder=False))
    
    # ── Pass 2: resolve fixups ───────────────────────────────────
    for start, rel_at, hh in record_fixups:
        if hh not in label_offsets:
            print(f"  warning: undefined label H_{hh:04X}, using 0")
            continue
        target = label_offsets[hh]
        rel = target - (rel_at + 4)
        struct.pack_into('<i', code, rel_at, rel)
    
    return bytes(code), bytes(fixup_data), label_offsets, entry_hh

def main():
    in_file = os.path.join(os.path.dirname(__file__), '..', 'yoyo', 'projects', 'yoyo.tyb')
    out_file = os.path.join(os.path.dirname(__file__), '..', 'yoyo-asm', 'build', 'M_tyb.exe')
    
    os.makedirs(os.path.dirname(out_file), exist_ok=True)
    
    with open(in_file, 'rb') as f:
        tyb_data = f.read()
    
    code, data, labels, entry_hh = compile_tyb(tyb_data)
    pe = build_pe(code, data)
    
    with open(out_file, 'wb') as f:
        f.write(pe)
    
    # Compute SHA256
    sha = hashlib.sha256(pe).hexdigest()
    
    print(f"tyb2exe: {in_file} → {out_file}")
    print(f"  Records: {(len(tyb_data) - 8) // 8}")
    print(f"  Code: {len(code)} bytes")
    print(f"  Data: {len(data)} bytes")
    print(f"  PE: {len(pe)} bytes")
    print(f"  SHA256: {sha}")
    
    # Verify against reference
    ref_path = os.path.join(os.path.dirname(__file__), '..', 'yoyo-js', 'build', 'M_js.exe')
    if os.path.exists(ref_path):
        with open(ref_path, 'rb') as f:
            ref = f.read()
        ref_sha = hashlib.sha256(ref).hexdigest()
        if sha == ref_sha:
            print(f"  ✅ DDC: EQUAL (matches JS reference)")
        else:
            print(f"  ❌ DDC: DIFFER")
            print(f"     JS: {ref_sha}")
    
    # Also verify against Rust
    ref_path2 = os.path.join(os.path.dirname(__file__), '..', 'yoyo-rust', 'build', 'M_rust.exe')
    if os.path.exists(ref_path2):
        with open(ref_path2, 'rb') as f:
            ref2 = f.read()
        ref2_sha = hashlib.sha256(ref2).hexdigest()
        if sha == ref2_sha:
            print(f"  ✅ DDC: EQUAL (matches Rust reference)")

if __name__ == '__main__':
    main()