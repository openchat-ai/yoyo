#!/usr/bin/env python3
"""
ty2tyb.py — Convert .ty to 8-byte-record binary .tyb.

Design principle: each .ty line → exactly 8 bytes, fitting one state slot.
Format: [opcode:1][argc:1][arg0:4][arg1:2]

a0 = 4 bytes (u32) — covers all imm values.
a1 = 2 bytes (u16) — covers arg2.
For 3-arg instructions: a1 high byte = arg1, low byte = arg2 (both ≤ 0xFF).

Label:  40 hh,0    →  [0x40][1][hh:4][0:2]
Record: op,args    →  [op][argc][a0:4][a1:2]

The .tyb is a flat byte array. The YOYO compiler framework reads
it sequentially: MEMCPY_DATA copies 8 bytes → slot → dispatch.

No variable offsets. No pointer arithmetic. Pure paper-tape machine.
"""
import struct, os, hashlib

def parse_ty(source):
    """Parse .ty: returns (records, labels, data_bytes, fixups)."""
    records = []
    labels = {}
    data_bytes = bytearray()
    fixups = []
    
    for raw in source.splitlines():
        line = raw.split(';')[0].split('#')[0].strip()
        if not line:
            continue
        if line.upper() in ('LAYOUT', 'END_LAYOUT'):
            continue
        toks = line.split()
        op = int(toks[0], 16)
        args = [int(t, 16) for t in toks[1:]]
        
        if op == 0x40:
            labels[args[0]] = len(records)
            records.append((op, args))  # emit label record too
            continue
        if op in (0x10, 0x12, 0x13):
            data_bytes.extend(args)
            continue
        
        is_branch = (op == 0x41 or op == 0x70 or (0x71 <= op <= 0x7A))
        if is_branch and args:
            fixups.append((len(records), args[0]))
        
        records.append((op, args))
    
    return records, labels, bytes(data_bytes), fixups

def pack_record(op, args):
    """Pack opcode + args into 8 bytes.
    argc=0: [op:1][0][pad:6]
    argc=1: [op:1][1][a0:4][pad:2]
    argc=2: [op:1][2][a0:4][a1:2]  — a0=imm(32b), a1=slot(16b)
    argc=3: [op:1][3][a0:2][a1:2][a2:2]  — 3×16b
    """
    a = args + [0] * (3 - len(args))
    n = len(args)
    if n == 0:
        return struct.pack('<BBxxxxxx', op & 0xFF, 0)
    elif n == 1:
        return struct.pack('<BBIxx', op & 0xFF, 1, a[0] & 0xFFFFFFFF)
    elif n == 2:
        return struct.pack('<BBHI', op & 0xFF, 2, a[0] & 0xFFFF, a[1] & 0xFFFFFFFF)
    else:  # n == 3
        return struct.pack('<BBHHH', op & 0xFF, 3, a[0] & 0xFFFF, a[1] & 0xFFFF, a[2] & 0xFFFF)

def build_tyb(records, labels, data_bytes, fixups):
    """Build .tyb: header + records + fixups + data."""
    buf = bytearray()
    # Magic + header: 8 bytes
    # [magic:4][entry_hh:2][rec_cnt:2]
    entry_hh = min(labels.keys()) if labels else 0
    buf.extend(b'TYB\0')
    buf.extend(struct.pack('<HH', entry_hh, len(records)))
    
    # Records: 8 bytes each
    for op, args in records:
        buf.extend(pack_record(op, args))
    
    # Fixup table: 6 bytes each
    # [rec_idx:4][label_hh:2]
    for rec_idx, hh in fixups:
        buf.extend(struct.pack('<IH', rec_idx, hh))
    
    # Data bytes
    buf.extend(data_bytes)
    return bytes(buf)

def main():
    in_file = os.path.join(os.path.dirname(__file__), '..', 'yoyo', 'projects', 'yoyo.ty')
    out_file = os.path.join(os.path.dirname(__file__), '..', 'yoyo', 'projects', 'yoyo.tyb')
    
    with open(in_file, 'r', encoding='utf-8') as f:
        source = f.read()
    
    records, labels, data_bytes, fixups = parse_ty(source)
    tyb = build_tyb(records, labels, data_bytes, fixups)
    
    os.makedirs(os.path.dirname(out_file) or '.', exist_ok=True)
    with open(out_file, 'wb') as f:
        f.write(tyb)
    
    print(f"ty2tyb: {in_file} → {out_file}")
    print(f"  Records: {len(records)} × 8B = {len(records)*8} bytes")
    print(f"  Labels: {len(labels)}")
    print(f"  Fixups: {len(fixups)}")
    print(f"  Data: {len(data_bytes)} bytes")
    print(f"  .tyb size: {len(tyb)} bytes")
    print(f"  SHA256: {hashlib.sha256(tyb).hexdigest()}")

if __name__ == '__main__':
    main()