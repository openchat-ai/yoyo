"""Rebuild: use fixture files as source of truth for correct handler bodies."""
import os, re, glob, hashlib, subprocess

ROOT = r'f:\yoyo'
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

# Parse fixture files for handler bodies
fixtures = sorted(glob.glob(os.path.join(ROOT, 'yoyo/tests/golden', 'selfhost_min_*.ty')))
fixture_sels = {}  # sel -> [body_lines]

for fpath in sorted(fixtures):
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
                stripped = lines[i].strip()
                if stripped == 'FF' or stripped.startswith('40 '):
                    break
                if stripped:
                    body_lines.append(stripped)
                i += 1
            if sel not in fixture_sels:
                fixture_sels[sel] = body_lines
        i += 1

# Parse body-extend logs for additional handlers
AUX = os.path.join(ROOT, 'docs', 'auxdocs')
logs = sorted(glob.glob(os.path.join(AUX, 'body-extend-*-log.md')))
log_sels = {}  # sel -> body_line
for logfile in logs:
    with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    pattern = r'\|\s*H_(\d+)\s*\|\s*0x([0-9A-Fa-f]+)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*(\d+)\s*\|\s*`([^`]+)`\s*\|'
    for m in re.finditer(pattern, content):
        sel = int(m.group(2), 16)
        op = m.group(3).strip()
        args = m.group(4).strip()
        op_match = re.match(r'0x([0-9A-Fa-f]+)', op)
        if op_match and sel not in fixture_sels and sel not in log_sels:
            op_byte = int(op_match.group(1), 16)
            log_sels[sel] = f'{op_byte:02X} {args}'

# Write in selector order
all_sels = sorted(set(fixture_sels.keys()) | set(log_sels.keys()))
max_sel = max(all_sels)
print(f'Fixture selectors: {len(fixture_sels)}, Log selectors: {len(log_sels)}, Max: 0x{max_sel:03X}')

lines = []
lines.append('; yoyo.ty — RECONSTRUCTED from fixtures + logs\n; W-START: EXPERIMENTAL\n')

for sel in range(0, max_sel + 1):
    if sel in fixture_sels:
        bodies = fixture_sels[sel]
    elif sel in log_sels:
        bodies = [log_sels[sel]]
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
pass_ = result.stdout.count('PASS')
print(f'PASS: {pass_}, FAIL: {fail}')