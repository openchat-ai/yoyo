"""
Correct rebuild: initial stub + body-extend logs for selector mapping.
Fixture files are test inputs, NOT canonical selector mapping.
"""
import os, re, glob, hashlib, subprocess, json

ROOT = r'f:\yoyo'
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

# Initial stub selectors: from PROMPT W-SM section
# Each handler maps to a specific selector by its handler number.
# compile_one_handler(src, hh) searches for `40 hh` label.
initial = {
    # h -> (sel, body_lines)
    # H_00 SET-UP: sel 0x00
    # H_01 GET: sel 0x01
    # ... etc
    0: (0x00, ['20 50 2A']),
    1: (0x01, ['60 51 50']),
    2: (0x02, ['68 50 51']),
    3: (0x03, ['30 50 07']),
    4: (0x04, ['60 51 50']),
    5: (0x05, ['A0 90']),
    6: (0x06, ['A0 90', 'A0 90']),
    7: (0x07, ['A0 90', 'A0 90', 'A0 90']),
    8: (0x08, ['A0 90', 'A0 90', 'A0 90', 'A0 90']),
    9: (0x09, ['A0 90', 'A0 90', 'A0 90', 'A0 90', 'A0 90']),
    10: (0x0A, ['A0 90', 'A0 90', 'A0 90', 'A0 90', 'A0 90', 'A0 90']),
    11: (0x0B, ['66 50']),
    12: (0x0C, ['67 50']),
    13: (0x0D, ['00']),
    14: (0x0E, ['A1 CC DD']),
    15: (0x0F, ['63 50 51']),
    16: (0x10, ['6A 50 51']),
    17: (0x11, ['65 50 51']),
    18: (0x12, ['80 50 51 00']),
    19: (0x13, ['70 00']),
    20: (0x14, ['41 00']),
    # H_21..H_2A: 10 Jcc (0x71..0x7A)
    21: (0x15, ['30 50 00', '30 51 00', '65 50 51', '71 00']),
    22: (0x16, ['30 50 00', '30 51 00', '65 50 51', '72 00']),
    23: (0x17, ['30 50 00', '30 51 00', '65 50 51', '73 00']),
    24: (0x18, ['30 50 00', '30 51 00', '65 50 51', '74 00']),
    25: (0x19, ['30 50 00', '30 51 00', '65 50 51', '75 00']),
    26: (0x1A, ['30 50 00', '30 51 00', '65 50 51', '76 00']),
    27: (0x1B, ['30 50 00', '30 51 00', '65 50 51', '77 00']),
    28: (0x1C, ['30 50 00', '30 51 00', '65 50 51', '78 00']),
    29: (0x1D, ['30 50 00', '30 51 00', '65 50 51', '79 00']),
    30: (0x1E, ['30 50 00', '30 51 00', '65 50 51', '7A 00']),
    # H_2B SET-CONTROL, H_2C GET, H_2D LDB
    31: (0x1F, ['50 50 00']),
    32: (0x20, ['60 51 50']),
    33: (0x21, ['80 51 50 00']),
    # H_2E ADD-IMM (body-extend-001)
    34: (0x22, ['62 50 03']),
    # H_2F LDB oo=127 (body-extend-002)
    35: (0x23, ['80 51 50 7F']),
    # H_30 LDB oo=-128 (body-extend-003)
    36: (0x24, ['80 51 50 80']),
}

# Parse body-extend logs for additional handlers
AUX = os.path.join(ROOT, 'docs', 'auxdocs')
logs = sorted(glob.glob(os.path.join(AUX, 'body-extend-*-log.md')))
extended = {}  # sel -> body_line
for logfile in logs:
    with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    # Pattern: | H_048 | 0x22 | 0x62 ADD-IMM | 50 03 | 19 | `sha` |
    pattern = r'\|\s*H_(\d+)\s*\|\s*0x([0-9A-Fa-f]+)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*(\d+)\s*\|\s*`([^`]+)`\s*\|'
    for m in re.finditer(pattern, content):
        h, sel_hex, op, args, blen, sha = m.groups()
        sel = int(sel_hex, 16)
        op_match = re.match(r'0x([0-9A-Fa-f]+)', op.strip())
        if op_match and sel not in extended:
            op_byte = int(op_match.group(1), 16)
            extended[sel] = f'{op_byte:02X} {args.strip()}'

# Build the file: every selector from 0 to max_sel
# Use initial stub first, then extended map
max_sel = max(
    max((s for v in initial.values() for s in [v[0]]), default=0),
    max(extended.keys(), default=0)
)
sel_map = {}

# Fill from initial stub
for h, (sel, body_lines) in initial.items():
    sel_map[sel] = body_lines

# Fill from extended logs (overrides initial if same selector)
for sel, body_line in extended.items():
    if sel not in sel_map:
        sel_map[sel] = [body_line]

# Early body-extend handlers (H_31..H_47)
early = [
    (0x25, '64 50 51'),      # H_31 MOVRR (0x64 = GET alias)
    (0x26, '69 50 51'),      # H_32 ORV
    (0x27, '6A 50 51'),      # H_33 SUBV
    (0x28, '63 50 51'),      # H_34 IMUL
    (0x29, '65 50 51'),      # H_35 CMP
    (0x2A, '66 51'),         # H_36 INC slot=51
    (0x2B, '67 51'),         # H_37 DEC slot=51
    (0x2C, '30 51 DEADBEEF'),# H_38 SET slot=51 imm=0xDEADBEEF
    (0x2D, '80 51 50 08'),   # H_39 LDB dst=51 src=50 oo=8
    (0x2E, '80 52 50 08'),   # H_3A LDB dst=52 src=50 oo=8
    (0x2F, '68 50 52'),      # H_3B ADDV slot=50 slot=52
    (0x30, '69 50 52'),      # H_3C ORV slot=50 slot=52
    (0x31, '6A 50 52'),      # H_3D SUBV slot=50 slot=52
    (0x32, '63 50 52'),      # H_3E IMUL slot=50 slot=52
    (0x33, '60 52 50'),      # H_3F GET slot=52 slot=50
    (0x34, '80 50 60 10'),   # H_40 LDB dst=50 src=60 oo=16
    (0x35, '80 51 60 10'),   # H_41 LDB dst=51 src=60 oo=16
    (0x36, '80 52 60 10'),   # H_42 LDB dst=52 src=60 oo=16
    (0x37, '62 50 0F'),      # H_43 ADD-IMM slot=50 imm=15
    (0x38, '61 50 08'),      # H_44 SUB-IMM slot=50 imm=8
    (0x39, '62 50 0A'),      # H_45 ADD-IMM slot=50 imm=10
    (0x3A, '61 50 05'),      # H_46 SUB-IMM slot=50 imm=5
    (0x3B, '62 51 0A'),      # H_47 ADD-IMM slot=51 imm=10
    (0x3C, '61 51 05'),      # H_48 SUB-IMM slot=51 imm=5
    # H_49 SUBV slot=51 slot=52 (body-extend-019, sel 0x3D)
    # H_4A... from logs
]
for sel, body in early:
    if sel not in sel_map:
        sel_map[sel] = [body]

# MEMCPY
memcpy = [(0x2EB, '84 50 51 40'), (0x2EC, '85 50 51 40')]
for sel, body in memcpy:
    if sel not in sel_map:
        sel_map[sel] = [body]

# Write
all_sels = sorted(sel_map.keys())
print(f'Total selectors: {len(all_sels)}, max: 0x{all_sels[-1]:03X}')

lines = ['; yoyo.ty — RECONSTRUCTED\n; W-START: EXPERIMENTAL\n']
for sel in range(0, all_sels[-1] + 1):
    if sel in sel_map:
        bodies = sel_map[sel]
    else:
        bodies = ['A0 90']
    lines.append(f'40 {sel:X}')
    for b in bodies:
        lines.append(f'  {b}')
    lines.append('  FF')
    lines.append('')

source = '\n'.join(lines) + '\n'
with open(OUT, 'w', encoding='utf-8') as f:
    f.write(source)

count = len([l for l in lines if l.startswith('40 ')])
sha = hashlib.sha256(source.encode('utf-8')).hexdigest()
print(f'{count} handlers, SHA256: {sha}')

result = subprocess.run(
    [r'f:\yoyo\yoyo-rust\target\debug\yoyo.exe', 'test', 'golden'],
    cwd=ROOT, capture_output=True, text=True, timeout=120
)
for line in result.stdout.strip().split('\n')[-3:]:
    print(f'  {line.strip()[:150]}')
fail = result.stdout.count('FAIL')
print(f'FAIL: {fail}')