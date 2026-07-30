#!/usr/bin/env python3
"""
ty2tyb.py — Convert .ty to 8-byte-record binary .tyb.

Design principle: each .ty line → exactly 8 bytes, fitting one state slot.
Format: [opcode:1][argc:1][arg0:2][arg1:2][arg2:2]

Label:  40 hh,0    →  [0x40][hh_hi][hh_lo][0][0][0][0][0]
Record: op,args    →  [op][argc][a0][a1][a2][0][0][0]

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
    [op:1][argc:1][arg0:2][arg1:2][arg2:2]
    """
    a = args + [0] * (3 - len(args))
    return struct.pack('<BBHHH', op & 0xFF, len(args), a[0] & 0xFFFF, a[1] & 0xFFFF, a[2] & 0xFFFF)

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