"""Fix: H_06..H_10 use A1 (RAW_BYTES) not A0 (RAW_BYTE)."""
import os, re, hashlib

OUT = r'f:\yoyo\yoyo\projects\yoyo.ty'

with open(OUT, 'r', encoding='utf-8') as f:
    lines = f.readlines()

# Fix H_06..H_10: A0 → A1
for i in range(len(lines)):
    stripped = lines[i].strip()
    # H_06 (sel 0x06): A0 90 90 → A1 90 90
    # H_07 (sel 0x07): A0 90 90 90 → A1 90 90 90  
    # H_08 (sel 0x08): A0 90 90 90 90 → A1 90 90 90 90
    # H_09 (sel 0x09): A0 90 90 90 90 90 → A1 90 90 90 90 90
    # H_10 (sel 0x0A): A0 90 90 90 90 90 90 → A1 90 90 90 90 90 90
    if stripped.startswith('A0 ') and stripped.count(' ') > 1:
        lines[i] = lines[i].replace('A0', 'A1', 1)

with open(OUT, 'w', encoding='utf-8') as f:
    f.writelines(lines)

source = ''.join(lines)
sha = hashlib.sha256(source.encode('utf-8')).hexdigest()
print(f'Fixed, SHA256: {sha}')