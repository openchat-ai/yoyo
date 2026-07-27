"""Fix: use 40 HH format without leading zeros (verifier parser expects no padding)."""
import os, re, hashlib

OUT = r'f:\yoyo\yoyo\projects\yoyo.ty'

with open(OUT, 'r', encoding='utf-8') as f:
    content = f.read()

# Replace 40 XXX with 40 XX (no leading zeros, hex uppercase)
def fix_sel(m):
    sel = int(m.group(1), 16)
    return f'40 {sel:X}'

content = re.sub(r'40 ([0-9A-Fa-f]{3,})', fix_sel, content)

# Also fix body args that are decimal (like 0xDEADBEEF in body)
# The body line should have hex args

with open(OUT, 'w', encoding='utf-8') as f:
    f.write(content)

sha = hashlib.sha256(content.encode('utf-8')).hexdigest()
print(f'Fixed, SHA256: {sha}')

# Run golden
import subprocess
result = subprocess.run(
    [r'f:\yoyo\yoyo-rust\target\debug\yoyo.exe', 'test', 'golden'],
    cwd=r'f:\yoyo', capture_output=True, text=True, timeout=120
)
for line in result.stdout.strip().split('\n')[-3:]:
    print(f'  {line.strip()[:150]}')
fail = result.stdout.count('FAIL')
print(f'FAIL: {fail}')