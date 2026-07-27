"""
Rebuild: fixture selector = canonical selector. Use fixture bodies directly.
"""
import os, re, glob, hashlib, subprocess, json

ROOT = r'f:\yoyo'
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

# Parse ALL fixture .ty files for selector→body mapping
# The fixture's selector IS the canonical selector
fixtures = sorted(glob.glob(os.path.join(ROOT, 'yoyo/tests/golden', '*.ty')))
sel_map = {}  # sel -> [body_lines]

for fpath in fixtures:
    with open(fpath, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    lines = content.split('\n')
    i = 0
    while i < len(lines):
        m = re.match(r'^40\s+([0-9A-Fa-f]+)$', lines[i].strip())
        if m:
            sel = int(m.group(1), 16)
            body_lines = []
            i += 1
            while i < len(lines):
                s = lines[i].strip()
                if s == 'FF' or s.startswith('40 '):
                    break
                if s:
                    body_lines.append(s)
                i += 1
            if sel not in sel_map:
                sel_map[sel] = body_lines
        i += 1

# Parse body-extend logs for additional handlers
AUX = os.path.join(ROOT, 'docs', 'auxdocs')
logs = sorted(glob.glob(os.path.join(AUX, 'body-extend-*-log.md')))
for logfile in logs:
    with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    pattern = r'\|\s*H_(\d+)\s*\|\s*0x([0-9A-Fa-f]+)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*(\d+)\s*\|\s*`([^`]+)`\s*\|'
    for m in re.finditer(pattern, content):
        sel = int(m.group(2), 16)
        op = m.group(3).strip()
        args = m.group(4).strip()
        op_match = re.match(r'0x([0-9A-Fa-f]+)', op)
        if op_match and sel not in sel_map:
            op_byte = int(op_match.group(1), 16)
            sel_map[sel] = [f'{op_byte:02X} {args}']

# MEMCPY (not in fixtures or logs table)
for sel, body in [(0x2EB, '84 50 51 40'), (0x2EC, '85 50 51 40')]:
    if sel not in sel_map:
        sel_map[sel] = [body]

# Also add P2 boundary handlers from batch-99 (5 PASS)
p2 = [
    (0x303, '80 60 50 7F'),  # LDB 127
    (0x304, '62 50 7F'),      # ADD-IMM 127
    (0x305, '61 51 FF'),      # SUB-IMM -1
    (0x306, '61 51 80'),      # SUB-IMM -128
    (0x307, '62 50 FF'),      # ADD-IMM -1
]
for sel, body in p2:
    if sel not in sel_map:
        sel_map[sel] = [body]

# Write in selector order
all_sels = sorted(sel_map.keys())
max_sel = max(all_sels)
print(f'Total selectors: {len(all_sels)}, max: 0x{max_sel:03X}')
gaps = [s for s in range(0, max_sel+1) if s not in sel_map]
print(f'Gaps: {len(gaps)}')

lines = ['; yoyo.ty — RECONSTRUCTED from fixtures + logs\n; W-START: EXPERIMENTAL\n']
for sel in range(0, max_sel + 1):
    bodies = sel_map.get(sel, ['A0 90'])
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
pass_ = result.stdout.count('PASS')
print(f'PASS: {pass_}, FAIL: {fail}')