import os, re, glob

ROOT = r'f:\yoyo'
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

# Read current yoyo.ty
with open(OUT, 'r', encoding='utf-8', errors='replace') as f:
    content = f.read()
lines = content.split('\n')

# Build selector->body from current file
sel_map = {}
i = 0
while i < len(lines):
    s = lines[i].strip()
    m = re.match(r'^40\s+([0-9A-Fa-f]+)$', s)
    if m:
        sel = m.group(1)
        bodies = []
        i += 1
        while i < len(lines):
            s2 = lines[i].strip()
            if s2 == 'FF' or s2.startswith('40 '):
                break
            if s2:
                bodies.append(s2)
            i += 1
        sel_map[sel] = bodies
    i += 1

print('=== Diagnosing FAIL selectors ===')
print(f'H_06 (0x6) body: {sel_map.get("6", "?")}')
print(f'H_18 (0x18) body: {sel_map.get("18", "?")}')
print(f'H_24 (0x18) body: {sel_map.get("24", "?")}')
print(f'H_3D (0x3D) body: {sel_map.get("3D", "?")}')
print(f'H_46 (0x46) body: {sel_map.get("46", "?")}')
print(f'H_4E (0x4E) body: {sel_map.get("4E", "?")}')

# Find chained2 fixture
for fpath in sorted(glob.glob(os.path.join(ROOT, 'yoyo/tests/golden', 'selfhost_min_*.ty'))):
    fname = os.path.basename(fpath)
    if 'chained' in fname and 'chained2' in fname:
        with open(fpath, 'r', encoding='utf-8', errors='replace') as f:
            print(f'\n=== {fname} ===')
            print(f.read())

# Find movrr fixture content
for fpath in sorted(glob.glob(os.path.join(ROOT, 'yoyo/tests/golden', 'selfhost_min_movrr.ty'))):
    with open(fpath, 'r', encoding='utf-8', errors='replace') as f:
        print('\n=== selfhost_min_movrr.ty ===')
        print(f.read())
