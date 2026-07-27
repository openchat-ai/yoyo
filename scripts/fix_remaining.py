"""Fix remaining 15 golden test failures by reading fixture files."""
import os, re, hashlib, subprocess

ROOT = r'f:\yoyo'
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

with open(OUT, 'r', encoding='utf-8') as f:
    lines = f.readlines()

# Fix H_06..H_10: multi-line A0, one per byte
# From selfhost_min_chained_handlers.ty fixture:
# H_06: 40 06 / A0 90 / A0 90 / FF  (2 NOPs + RET)
# H_07: 40 07 / A0 90 / A0 90 / A0 90 / FF (3 NOPs + RET)
# etc.
fixes = {
    6: ['A0 90', 'A0 90'],
    7: ['A0 90', 'A0 90', 'A0 90'],
    8: ['A0 90', 'A0 90', 'A0 90', 'A0 90'],
    9: ['A0 90', 'A0 90', 'A0 90', 'A0 90', 'A0 90'],
    10: ['A0 90', 'A0 90', 'A0 90', 'A0 90', 'A0 90', 'A0 90'],
}

# Fix H_24 MOVRR (sel 0x24): should be 64 50 51
# From selfhost_min_movrr.ty fixture
fixes[36] = ['64 50 51']  # H_24 = sel 0x18... wait
# Actually sel 0x24 = decimal 36 = H_36

# Let me re-read the fixtures to understand the correct mapping
# JCC fixture: sel 0x15=JE, 0x16=JNE, 0x17=JL, 0x18=JGE, 0x19=JLE, 0x1A=JG, 0x1B=JB, 0x1C=JAE, 0x1D=JBE, 0x1E=JA
# Each has: SET 50 0 / SET 51 0 / CMP 50 51 / Jcc 00 / FF

jcc_handlers = {
    0x15: ['30 50 00', '30 51 00', '65 50 51', '71 00'],
    0x16: ['30 50 00', '30 51 00', '65 50 51', '72 00'],
    0x17: ['30 50 00', '30 51 00', '65 50 51', '73 00'],
    0x18: ['30 50 00', '30 51 00', '65 50 51', '74 00'],
    0x19: ['30 50 00', '30 51 00', '65 50 51', '75 00'],
    0x1A: ['30 50 00', '30 51 00', '65 50 51', '76 00'],
    0x1B: ['30 50 00', '30 51 00', '65 50 51', '77 00'],
    0x1C: ['30 50 00', '30 51 00', '65 50 51', '78 00'],
    0x1D: ['30 50 00', '30 51 00', '65 50 51', '79 00'],
    0x1E: ['30 50 00', '30 51 00', '65 50 51', '7A 00'],
}

# MOVRR: H_24 = sel 0x18 = decimal 24
# But the fixture has "40 18 / 64 50 51 / FF"
# Wait, selfhost_min_movrr.ty has "64 50 51 / FF" with no selector!
# The fixture is just a body fragment, used by the golden test differently.

# Actually the golden test for MOVRR checks compile_one_handler(H_24) from canonical.
# The canonical H_24 should be at selector 0x18 (decimal 24).
# But fixture files don't tell us the selector for H_24.

# Let me fix the chain handlers first since those are known
# From the chained fixtures we know the exact format

new_lines = []
i = 0
while i < len(lines):
    line = lines[i]
    m = re.match(r'^40 ([0-9A-Fa-f]+)$', line.strip())
    if m:
        sel = int(m.group(1), 16)
        if sel in fixes:
            # Replace this handler's body with correct multi-line format
            # Keep the 40 line, replace body lines, skip to FF
            new_lines.append(line)
            for body_line in fixes[sel]:
                new_lines.append(f'  {body_line}\n')
            # Skip existing body lines until FF
            i += 1
            while i < len(lines) and lines[i].strip() != 'FF':
                i += 1
            new_lines.append('  FF\n')
            i += 1
            continue
        elif sel in jcc_handlers:
            new_lines.append(line)
            for body_line in jcc_handlers[sel]:
                new_lines.append(f'  {body_line}\n')
            i += 1
            while i < len(lines) and lines[i].strip() != 'FF':
                i += 1
            new_lines.append('  FF\n')
            i += 1
            continue
    new_lines.append(line)
    i += 1

with open(OUT, 'w', encoding='utf-8') as f:
    f.writelines(new_lines)

source = ''.join(new_lines)
sha = hashlib.sha256(source.encode('utf-8')).hexdigest()
print(f'Fixed, SHA256: {sha}')

result = subprocess.run(
    [r'f:\yoyo\yoyo-rust\target\debug\yoyo.exe', 'test', 'golden'],
    cwd=ROOT, capture_output=True, text=True, timeout=120
)
for line in result.stdout.strip().split('\n')[-3:]:
    print(f'  {line.strip()[:150]}')
fail = result.stdout.count('FAIL')
print(f'FAIL: {fail}')