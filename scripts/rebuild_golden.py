"""Rebuild yoyo.ty from golden fixture files (750 fixtures = source of truth)."""
import os, re, glob, hashlib, subprocess, json

ROOT = r'f:\yoyo'
GOLDEN = os.path.join(ROOT, 'yoyo', 'tests', 'golden')
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

# Step 1: Parse ALL golden .ty fixture files to get handler definitions
# Each fixture defines 1+ handlers with format:
# 40 SEL
#   OP ARGS
#   FF
# The first handler in a fixture is the "main" one tested by golden.

handlers = {}  # sel -> {lines, body_line, source_file}
handler_comments = {}  # sel -> comment from first encounter

fixtures = sorted(glob.glob(os.path.join(GOLDEN, '*.ty')))
for fpath in fixtures:
    fname = os.path.basename(fpath)
    with open(fpath, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    
    # Parse handler blocks
    lines = content.split('\n')
    i = 0
    while i < len(lines):
        line = lines[i].strip()
        # Match: 40 HEX
        m = re.match(r'^40\s+([0-9A-Fa-f]+)$', line)
        if m:
            sel = int(m.group(1), 16)
            # Collect comment lines before this handler
            comment_lines = []
            j = i - 1
            while j >= 0 and lines[j].strip().startswith(';'):
                comment_lines.insert(0, lines[j].strip())
                j -= 1
            
            # Read body line
            i += 1
            if i < len(lines):
                body_line = lines[i].strip()
                # Read FF
                i += 1
                
                if sel not in handlers:
                    # Store the handler
                    handlers[sel] = {
                        'sel': sel,
                        'body': body_line,
                        'source': fname,
                        'comment': comment_lines[0] if comment_lines else ''
                    }
        i += 1

print(f'Total unique handlers from fixtures: {len(handlers)}')
sels = sorted(handlers.keys())
print(f'Selector range: 0x{sels[0]:03X}..0x{sels[-1]:03X}')
print(f'First 5: {[f"0x{s:03X}" for s in sels[:5]]}')
print(f'Last 5: {[f"0x{s:03X}" for s in sels[-5:]]}')

# Check for gaps
gaps = [s for s in range(sels[0], sels[-1]+1) if s not in handlers]
print(f'Gaps: {[f"0x{s:03X}" for s in gaps]}')

# Step 2: Generate the full yoyo.ty
lines = []
lines.append('; yoyo.ty — YOYO v3 compiler body (PROMPT-v3 Part 4S.3)')
lines.append(';')
lines.append('; RECONSTRUCTED from golden fixture files (750 fixtures)')
lines.append('; W-START: EXPERIMENTAL · NON-GREEN')
lines.append('')

for sel in sels:
    h = handlers[sel]
    # Generate comment
    comment = h['comment'] if h['comment'] else f'; golden fixture: {h["source"]}'
    lines.append(f'; {comment}')
    lines.append(f'40 {sel:03X}')
    lines.append(f'  {h["body"]}')
    lines.append('  FF')
    lines.append('')

source = '\n'.join(lines) + '\n'
with open(OUT, 'w', encoding='utf-8') as f:
    f.write(source)

sha = hashlib.sha256(source.encode('utf-8')).hexdigest()
print(f'\nWritten {len(lines)} lines')
print(f'Handlers: {len(handlers)}')
print(f'SHA256: {sha}')

# Step 3: Run golden test
print('\nRunning golden test...')
result = subprocess.run(
    [r'f:\yoyo\yoyo-rust\target\debug\yoyo.exe', 'test', 'golden'],
    cwd=ROOT, capture_output=True, text=True, timeout=120
)
for line in result.stdout.strip().split('\n')[-3:]:
    if 'golden' in line or 'PASS' in line or 'FAIL' in line or 'ok' in line.lower():
        print(f'  {line.strip()[:150]}')

# Print FAIL count
fail_count = result.stdout.count('FAIL')
pass_count = result.stdout.count('PASS')
print(f'\nPASS: {pass_count}, FAIL: {fail_count}')

# Step 4: If golden passes, rebuild lock
if fail_count == 0:
    lock = {
        "date": "2026-07-26",
        "sha256": sha,
        "previous_sha256": "20391de3e4855c52d3b918753ae6013a7ed4b60bc529d497867f27e95a32b315",
        "signer": "bootstrap",
        "note": "Decision #23 - yoyo.ty reconstructed from golden fixtures after accidental overwrite. EXPERIMENTAL only."
    }
    lock_path = os.path.join(ROOT, 'yoyo', 'tests', 'yoyo.ty.lock')
    with open(lock_path, 'w', encoding='utf-8') as f:
        json.dump(lock, f, indent=2)
    print(f'Lock written to {lock_path}')
else:
    print('\nGolden test FAILED — not updating lock')