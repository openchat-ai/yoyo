"""Rebuild yoyo.ty: handlers in selector order, pad gaps with NOP."""
import os, re, glob, hashlib, subprocess, json

ROOT = r'f:\yoyo'
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

# Step 1: Collect all handlers from body-extend logs + initial stub  
AUX = os.path.join(ROOT, 'docs', 'auxdocs')
# Parse all logs for handler table entries
logs = sorted(glob.glob(os.path.join(AUX, 'body-extend-*-log.md')))

handlers_by_sel = {}  # sel -> body_line

for logfile in logs:
    with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    # Pattern: | H_048 | 0x22 | 0x62 ADD-IMM | 50 03 | 19 | `sha` |
    pattern = r'\|\s*H_(\d+)\s*\|\s*0x([0-9A-Fa-f]+)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*(\d+)\s*\|\s*`([^`]+)`\s*\|'
    for m in re.finditer(pattern, content):
        h, sel_hex, op, args, blen, sha = m.groups()
        sel = int(sel_hex, 16)
        op_match = re.match(r'0x([0-9A-Fa-f]+)', op.strip())
        op_byte = int(op_match.group(1), 16) if op_match else 0
        body = f'{op_byte:02X} {args.strip()}'
        if sel not in handlers_by_sel:
            handlers_by_sel[sel] = body

# Add initial stub
initial_stub = [
    (0x00, '20 50 2A'), (0x01, '60 51 50'), (0x02, '68 50 51'),
    (0x03, '30 50 07'), (0x04, '60 51 50'), (0x05, 'A0 90'),
    (0x06, 'A1 90 90'), (0x07, 'A1 90 90 90'), (0x08, 'A1 90 90 90 90'),
    (0x09, 'A1 90 90 90 90 90'), (0x0A, 'A1 90 90 90 90 90 90'),
    (0x0B, '66 50'), (0x0C, '67 50'), (0x0D, '00'), (0x0E, 'A1 CC DD'),
    (0x0F, '63 50 51'), (0x10, '6A 50 51'), (0x11, '65 50 51'),
    (0x12, '80 50 51 00'), (0x13, '70 00'), (0x14, '41 00'),
    (0x15, '71 00'), (0x16, '72 00'), (0x17, '73 00'), (0x18, '74 00'),
    (0x19, '75 00'), (0x1A, '76 00'), (0x1B, '77 00'), (0x1C, '78 00'),
    (0x1D, '79 00'), (0x1E, '7A 00'), (0x1F, '50 50 00'),
    (0x20, '60 51 50'), (0x21, '80 51 50 00'),
]
for sel, body in initial_stub:
    if sel not in handlers_by_sel:
        handlers_by_sel[sel] = body

# Early body-extend handlers (H_2E..H_47, selectors 0x22..0x3C)
early = [
    (0x22, '62 50 03'), (0x23, '80 51 50 7F'), (0x24, '80 51 50 80'),
    (0x25, '60 51 50'), (0x26, '69 50 51'), (0x27, '6A 50 51'),
    (0x28, '63 50 51'), (0x29, '65 50 51'), (0x2A, '66 51'),
    (0x2B, '67 51'), (0x2C, '30 51 DEADBEEF'), (0x2D, '80 51 50 08'),
    (0x2E, '80 52 50 08'), (0x2F, '68 50 52'), (0x30, '69 50 52'),
    (0x31, '6A 50 52'), (0x32, '63 50 52'), (0x33, '60 52 50'),
    (0x34, '80 50 60 10'), (0x35, '80 51 60 10'), (0x36, '80 52 60 10'),
    (0x37, '62 50 0F'), (0x38, '61 50 08'), (0x39, '62 50 0A'),
    (0x3A, '61 50 05'), (0x3B, '62 51 0A'), (0x3C, '61 51 05'),
]
for sel, body in early:
    if sel not in handlers_by_sel:
        handlers_by_sel[sel] = body

# MEMCPY handlers (H_741/H_742, selectors 0x2EB/0x2EC)
memcpy = [
    (0x2EB, '84 50 51 40'), (0x2EC, '85 50 51 40'),
]
for sel, body in memcpy:
    if sel not in handlers_by_sel:
        handlers_by_sel[sel] = body

# Step 2: Write in selector order, pad gaps with NOP
all_sels = sorted(handlers_by_sel.keys())
print(f'Total selectors: {len(all_sels)}')
print(f'Range: 0x{all_sels[0]:03X}..0x{all_sels[-1]:03X}')

# Check for gaps
gaps = [s for s in range(all_sels[0], all_sels[-1]+1) if s not in handlers_by_sel]
print(f'Gaps ({len(gaps)}): {[f"0x{s:03X}" for s in gaps[:20]]}{"..." if len(gaps)>20 else ""}')

lines = []
lines.append('; yoyo.ty — YOYO v3 compiler body')
lines.append('; RECONSTRUCTED from body-extend logs (selector-order)')
lines.append('; W-START: EXPERIMENTAL · NON-GREEN')
lines.append('')

for sel in range(0, all_sels[-1] + 1):
    if sel in handlers_by_sel:
        body = handlers_by_sel[sel]
    else:
        body = 'A0 90'  # NOP placeholder
    lines.append(f'40 {sel:03X}')
    lines.append(f'  {body}')
    lines.append('  FF')
    lines.append('')

source = '\n'.join(lines) + '\n'
with open(OUT, 'w', encoding='utf-8') as f:
    f.write(source)

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
for line in result.stdout.strip().split('\n')[-3:]:
    print(f'  {line.strip()[:150]}')
fail = result.stdout.count('FAIL')
pass_ = result.stdout.count('PASS')
print(f'PASS: {pass_}, FAIL: {fail}')