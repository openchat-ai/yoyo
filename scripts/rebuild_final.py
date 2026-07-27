"""Final rebuild: correct initial 34-handler stub + body-extend log handlers."""
import os, re, glob, hashlib, subprocess, json

ROOT = r'f:\yoyo'
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

# Correct initial 34-handler stub (selectors 0x00..0x21 = H_00..H_2D)
# From PROMPT W-SM section. Each entry: (h, sel, body_line, desc)
stub = [
    (0, 0x00, '20 50 2A', 'SET-UP slot=50 imm=0x2A'),
    (1, 0x01, '60 51 50', 'GET slot=51 slot=50'),
    (2, 0x02, '68 50 51', 'ADDV slot=50 slot=51'),
    (3, 0x03, '30 50 07', 'SET slot=50 imm=7'),
    (4, 0x04, '60 51 50', 'GET slot=51 slot=50'),
    (5, 0x05, 'A0 90', 'RAW_BYTE 0x90'),
    (6, 0x06, 'A0 90 90', 'RAW_BYTE 0x90 0x90'),
    (7, 0x07, 'A0 90 90 90', 'RAW_BYTE 0x90*3'),
    (8, 0x08, 'A0 90 90 90 90', 'RAW_BYTE 0x90*4'),
    (9, 0x09, 'A0 90 90 90 90 90', 'RAW_BYTE 0x90*5'),
    (10, 0x0A, 'A0 90 90 90 90 90 90', 'RAW_BYTE 0x90*6'),
    (11, 0x0B, '66 50', 'INC slot=50'),
    (12, 0x0C, '67 50', 'DEC slot=50'),
    (13, 0x0D, '00', 'NOP'),
    (14, 0x0E, 'A1 CC DD', 'RAW_BYTES CC DD'),
    (15, 0x0F, '63 50 51', 'IMUL slot=50 slot=51'),
    (16, 0x10, '6A 50 51', 'SUBV slot=50 slot=51'),
    (17, 0x11, '65 50 51', 'CMP slot=50 slot=51'),
    (18, 0x12, '80 50 51 00', 'LDB slot=50 slot=51 oo=0'),
    (19, 0x13, '70 00', 'JMP H_00'),
    (20, 0x14, '41 00', 'CALL H_00'),
    (21, 0x15, '71 00', 'JE H_00'),
    (22, 0x16, '72 00', 'JNE H_00'),
    (23, 0x17, '73 00', 'JL H_00'),
    (24, 0x18, '74 00', 'JGE H_00'),
    (25, 0x19, '75 00', 'JLE H_00'),
    (26, 0x1A, '76 00', 'JG H_00'),
    (27, 0x1B, '77 00', 'JB H_00'),
    (28, 0x1C, '78 00', 'JAE H_00'),
    (29, 0x1D, '79 00', 'JBE H_00'),
    (30, 0x1E, '7A 00', 'JA H_00'),
    (31, 0x1F, '50 50 00', 'SET-CONTROL slot=50 imm=0'),
    (32, 0x20, '60 51 50', 'GET slot=51 slot=50'),
    (33, 0x21, '80 51 50 00', 'LDB slot=51 slot=50 oo=0'),
]

# Step 2: Parse body-extend logs for handler additions
AUX = os.path.join(ROOT, 'docs', 'auxdocs')
logs = sorted(glob.glob(os.path.join(AUX, 'body-extend-*-log.md')))
def sort_key(f):
    m = re.search(r'body-extend-(\d+)', os.path.basename(f))
    return int(m.group(1)) if m else 0
logs.sort(key=sort_key)

extended = {}  # h -> {sel, opcode, args, len, sha, body_line}
for logfile in logs:
    with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    # Pattern: | H_048 | 0x22 | 0x62 ADD-IMM | 50 03 | 19 | `498b87...` |
    pattern = r'\|\s*H_(\d+)\s*\|\s*0x([0-9A-Fa-f]+)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*(\d+)\s*\|\s*`([^`]+)`\s*\|'
    for m in re.finditer(pattern, content):
        h, sel, op, args, blen, sha = m.groups()
        h = int(h)
        sel = int(sel, 16)
        if h not in extended:
            # Parse opcode byte from "0x62 ADD-IMM" format
            op_match = re.match(r'0x([0-9A-Fa-f]+)', op.strip())
            op_byte = int(op_match.group(1), 16) if op_match else 0
            # Keep args as-is for body line
            extended[h] = {
                'sel': sel, 'op': op.strip(), 'args': args.strip(),
                'op_byte': op_byte, 'len': int(blen), 'sha': sha.strip(),
                'body': f'{op_byte:02X} {args.strip()}'
            }

# Also parse early logs (body-extend-001..018) for handlers H_2E..H_47
# These have a different format - extract from the log title
early_logs = [l for l in logs if int(re.search(r'(\d+)', os.path.basename(l)).group(1)) <= 18]
for logfile in early_logs:
    with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    # Try to find handler info from first line
    first = content.split('\n')[0]
    m = re.search(r'H_(\d+)\s+0x([0-9A-Fa-f]+)\s+(\w+)', first)
    if m:
        h = int(m.group(1))
        op = int(m.group(2), 16)
        name = m.group(3)
        if h not in extended:
            # Try to find args from the description
            desc_match = re.search(r'slot=0x([0-9A-Fa-f]+)', content)
            if desc_match:
                slot = int(desc_match.group(1), 16)
                extended[h] = {
                    'sel': 0, 'op': f'0x{op:02X} {name}',
                    'args': f'{slot:02X}',
                    'op_byte': op, 'len': 0, 'sha': '',
                    'body': f'{op:02X} {slot:02X}'
                }

# Step 3: Generate the full file
lines = []
def write_handler(h, sel, body, comment=''):
    lines.append(f'; H_{h:03d} — {comment}')
    lines.append(f';   Not full self-host / not G06 / not Phase 2 / not freeze.')
    lines.append(f'40 {sel:03X}')
    lines.append(f'  {body}')
    lines.append('  FF')
    lines.append('')

lines.append('; yoyo.ty — YOYO v3 compiler body (PROMPT-v3 Part 4S.3)')
lines.append(';')
lines.append('; RECONSTRUCTED from PROMPT W-SM stub + body-extend-*-log.md files')
lines.append('; W-START: EXPERIMENTAL · NON-GREEN')
lines.append('')

# Write initial stub
for h, sel, body, desc in stub:
    write_handler(h, sel, body, desc)

# Write extended handlers from logs (sorted by selector)
# Collect all extended handlers, skip those already in stub
extended_sorted = sorted(extended.values(), key=lambda x: x['sel'])
for info in extended_sorted:
    h = [k for k, v in extended.items() if v['sel'] == info['sel']][0]
    if h >= 34:  # Skip if already in stub
        body = info['body']
        comment = f'{info["op"]} {info["args"]}'
        write_handler(h, info['sel'], body, comment)

# Write fixed handlers that were added by body-extend-N but not in logs
# H_2E (sel 0x22): ADD-IMM slot=50 imm=3 (body-extend-001)
# Check if selector 0x22 is already used
all_sels = {info['sel'] for info in extended_sorted}
all_sels |= {sel for _, sel, _, _ in stub}

fixups = [
    (46, 0x22, '62 50 03', 'ADD-IMM slot=50 imm=3'),
    (47, 0x23, '80 51 50 7F', 'LDB dst=51 src=50 oo=127'),
    (48, 0x24, '80 51 50 80', 'LDB dst=51 src=50 oo=-128'),
    (49, 0x25, '60 51 50', 'MOVRR dst=51 src=50'),
    (50, 0x26, '69 50 51', 'ORV slot=50 slot=51'),
    (51, 0x27, '6A 50 51', 'SUBV slot=50 slot=51'),
    (52, 0x28, '63 50 51', 'IMUL slot=50 slot=51'),
    (53, 0x29, '65 50 51', 'CMP slot=50 slot=51'),
    (54, 0x2A, '66 51', 'INC slot=51'),
    (55, 0x2B, '67 51', 'DEC slot=51'),
    (56, 0x2C, '30 51 DEADBEEF', 'SET slot=51 imm=0xDEADBEEF'),
    (57, 0x2D, '80 51 50 08', 'LDB dst=51 src=50 oo=8'),
    (58, 0x2E, '80 52 50 08', 'LDB dst=52 src=50 oo=8'),
    (59, 0x2F, '68 50 52', 'ADDV slot=50 slot=52'),
    (60, 0x30, '69 50 52', 'ORV slot=50 slot=52'),
    (61, 0x31, '6A 50 52', 'SUBV slot=50 slot=52'),
    (62, 0x32, '63 50 52', 'IMUL slot=50 slot=52'),
    (63, 0x33, '60 52 50', 'GET slot=52 slot=50'),
    # H_40..H_47 from body-extend-017/018
    (64, 0x34, '80 50 60 10', 'LDB dst=50 src=60 oo=16'),
    (65, 0x35, '80 51 60 10', 'LDB dst=51 src=60 oo=16'),
    (66, 0x36, '80 52 60 10', 'LDB dst=52 src=60 oo=16'),
    (67, 0x37, '62 50 0F', 'ADD-IMM slot=50 imm=15'),
    (68, 0x38, '61 50 08', 'SUB-IMM slot=50 imm=8'),
    (69, 0x39, '62 50 0A', 'ADD-IMM slot=50 imm=10'),
    (70, 0x3A, '61 50 05', 'SUB-IMM slot=50 imm=5'),
    (71, 0x3B, '62 51 0A', 'ADD-IMM slot=51 imm=10'),
    (72, 0x3C, '61 51 05', 'SUB-IMM slot=51 imm=5'),
]

for h, sel, body, desc in fixups:
    if sel not in all_sels:
        write_handler(h, sel, body, desc)
        all_sels.add(sel)

source = '\n'.join(lines) + '\n'
with open(OUT, 'w', encoding='utf-8') as f:
    f.write(source)

# Count handlers
count = len([l for l in lines if l.startswith('40 ')])
sha = hashlib.sha256(source.encode('utf-8')).hexdigest()
print(f'Written {len(lines)} lines, {count} handlers')
print(f'SHA256: {sha}')

# Run golden test
print('\n=== Running golden test ===')
result = subprocess.run(
    [r'f:\yoyo\yoyo-rust\target\debug\yoyo.exe', 'test', 'golden'],
    cwd=ROOT, capture_output=True, text=True, timeout=120
)
# Print last line
for line in result.stdout.strip().split('\n')[-3:]:
    print(f'  {line.strip()[:150]}')

fail_count = result.stdout.count('FAIL')
pass_count = result.stdout.count('PASS')
print(f'PASS: {pass_count}, FAIL: {fail_count}')