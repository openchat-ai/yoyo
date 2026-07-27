"""Rebuild yoyo.ty from golden fixtures (source of truth) + body-extend logs."""
import os, re, glob, hashlib, subprocess, json

ROOT = r'f:\yoyo'
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

# Step 1: Parse ALL golden fixture .ty files for selector→body mapping
fixtures = glob.glob(os.path.join(ROOT, 'yoyo/tests/golden', '*.ty'))
fixture_map = {}  # sel -> body_line
for fpath in fixtures:
    with open(fpath, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    lines = content.split('\n')
    i = 0
    while i < len(lines):
        m = re.match(r'^40\s+([0-9A-Fa-f]+)$', lines[i].strip())
        if m:
            sel = int(m.group(1), 16)
            if i + 1 < len(lines):
                body = lines[i+1].strip()
                if sel not in fixture_map:
                    fixture_map[sel] = body
            i += 2
        i += 1

print(f'Fixtures: {len(fixture_map)} unique selectors')
sels = sorted(fixture_map.keys())
print(f'  Range: 0x{sels[0]:03X}..0x{sels[-1]:03X}')

# Step 2: Parse body-extend logs for additional handlers
AUX = os.path.join(ROOT, 'docs', 'auxdocs')
logs = sorted(glob.glob(os.path.join(AUX, 'body-extend-*-log.md')))
log_map = {}  # sel -> body_line
for logfile in logs:
    with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    pattern = r'\|\s*H_(\d+)\s*\|\s*0x([0-9A-Fa-f]+)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*(\d+)\s*\|\s*`([^`]+)`\s*\|'
    for m in re.finditer(pattern, content):
        h, sel_hex, op, args, blen, sha = m.groups()
        sel = int(sel_hex, 16)
        op_match = re.match(r'0x([0-9A-Fa-f]+)', op.strip())
        if op_match:
            op_byte = int(op_match.group(1), 16)
            if sel not in log_map and sel not in fixture_map:
                log_map[sel] = f'{op_byte:02X} {args.strip()}'

print(f'From logs (not in fixtures): {len(log_map)} selectors')

# Step 3: Merge: fixture data wins, then log data, then NOP padding
all_sels = set(fixture_map.keys()) | set(log_map.keys())
max_sel = max(all_sels)
print(f'Max selector: 0x{max_sel:03X}')

# Write in selector order
lines = []
lines.append('; yoyo.ty — YOYO v3 compiler body')
lines.append('; RECONSTRUCTED from golden fixtures + body-extend logs')
lines.append('; W-START: EXPERIMENTAL · NON-GREEN')
lines.append('')

for sel in range(0, max_sel + 1):
    if sel in fixture_map:
        body = fixture_map[sel]
    elif sel in log_map:
        body = log_map[sel]
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
    if 'golden' in line or 'FAIL' in line or 'ok' in line.lower():
        print(f'  {line.strip()[:150]}')
fail = result.stdout.count('FAIL')
pass_ = result.stdout.count('PASS')
print(f'PASS: {pass_}, FAIL: {fail}')