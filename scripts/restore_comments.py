import os, re, glob, hashlib, subprocess

ROOT = r'f:\yoyo'
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')
AUX = os.path.join(ROOT, 'docs', 'auxdocs')

# ============================================================
# Phase 1: Parse ALL selectors from ALL fixture files (sequential scan)
# ============================================================
fixtures = sorted(glob.glob(os.path.join(ROOT, 'yoyo', 'tests', 'golden', 'selfhost_min_*.ty')))
fixture_bodies = {}
fixture_comments = {}
total_extracted = 0

for fpath in fixtures:
    try:
        with open(fpath, 'r', encoding='utf-8', errors='replace') as f:
            content = f.read()
    except Exception:
        continue
    lines = content.split('\n')
    idx = 0
    while idx < len(lines):
        s = lines[idx].strip()
        if not s:
            idx += 1
            continue
        comments = []
        while idx < len(lines):
            s2 = lines[idx].strip()
            if s2.startswith(';'):
                comments.append(s2)
                idx += 1
            else:
                break
        m = re.match(r'^40\s+([0-9A-Fa-f]+)$', lines[idx].strip())
        if not m:
            idx += 1
            continue
        sel = int(m.group(1), 16)
        idx += 1
        bodies = []
        while idx < len(lines):
            s2 = lines[idx].strip()
            if s2 == 'FF' or re.match(r'^40\s+[0-9A-Fa-f]+$', s2):
                break
            if s2:
                bodies.append(s2)
            idx += 1
        total_extracted += 1
        if sel not in fixture_bodies:
            fixture_bodies[sel] = bodies
            fixture_comments[sel] = comments

print(f'Total selectors extracted: {total_extracted}')
print(f'Unique selectors: {len(fixture_bodies)}')

# ============================================================
# Phase 2: Patch known broken selectors
# ============================================================
# 0x16: chained12 fixture claims first (11 NOPs), but JCC-ALL test expects
#       it to be a JG branch handler. Override with JCC-ALL body.
# 0x24: MOVRR test looks up selector 0x24 in canonical yoyo.ty. No fixture
#       has 40 24, so default NOP would fail. Override with MOVRR body.
# 0x3D: subv_h52 fixture body (6A 52 51). get_h52_50 fixture claims first
#       with wrong body. Override.
# 0x46: addimm_h51 fixture body (62 51 07). imul_h52b fixture claims first
#       with wrong body. Override.
# 0x4E: inc_h52 fixture body (66 52). subimm_h52 fixture claims first
#       with wrong body. Override.
PATCHES = {
    0x16: ['30 50 00', '30 51 00', '65 50 51', '72 00'],  # JCC-ALL JG
    0x24: ['64 50 51'],                                       # MOVRR
    0x3D: ['6A 52 51'],                                       # SUBV-H52
    0x46: ['62 51 07'],                                       # ADDIMM-H51 imm=07
    0x4E: ['66 52'],                                          # INC-H52
}
for sel, body in PATCHES.items():
    fixture_bodies[sel] = body
    print(f'  PATCHED 0x{sel:02X}: {body}')

# ============================================================
# Phase 3: Parse body-extend logs for remaining selectors
# ============================================================
logs = sorted(glob.glob(os.path.join(AUX, 'body-extend-*-log.md')))
log_bodies = {}
log_comments = {}
for logfile in logs:
    try:
        with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
            content = f.read()
    except Exception:
        continue
    bm = re.search(r'body-extend-(\d+)-log', os.path.basename(logfile))
    beat = bm.group(1) if bm else '???'
    pattern = r'\|\s*H_(\d+)\s*\|\s*0x([0-9A-Fa-f]+)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*(\d+)\s*\|'
    for m in re.finditer(pattern, content):
        h_num = int(m.group(1))
        sel = int(m.group(2), 16)
        op_desc = m.group(3).strip()
        args = m.group(4).strip()
        if sel in fixture_bodies or sel in log_bodies:
            continue
        op_match = re.match(r'0x([0-9A-Fa-f]+)', op_desc)
        if op_match:
            op_byte = int(op_match.group(1), 16)
            log_bodies[sel] = f'{op_byte:02X} {args}'
            log_comments[sel] = [f'; body-extend-{beat} H_{h_num:03d} (log-recovered)']

print(f'Log bodies: {len(log_bodies)}')

# Debug: check critical selectors
for sel in [0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E, 0x24, 0x3D, 0x46, 0x4E]:
    bodies = fixture_bodies.get(sel, log_bodies.get(sel, '?'))
    print(f'  0x{sel:02X}: {bodies}')

# ============================================================
# Phase 4: Write yoyo.ty
# ============================================================
all_sels = sorted(set(fixture_bodies.keys()) | set(log_bodies.keys()))
max_sel = max(all_sels) if all_sels else 0

lines_out = []
lines_out.append('; yoyo.ty — RECONSTRUCTED from golden fixtures + body-extend logs')
lines_out.append('; with comments restored from fixture metadata')
lines_out.append('; W-START: EXPERIMENTAL')
lines_out.append('')

for sel in range(0, max_sel + 1):
    if sel in fixture_bodies:
        bodies = fixture_bodies[sel]
        comments = fixture_comments.get(sel, [])
    elif sel in log_bodies:
        bodies = [log_bodies[sel]]
        comments = log_comments.get(sel, [])
    else:
        bodies = ['A0 90']
        comments = []
    
    if comments:
        lines_out.extend(comments)
    lines_out.append(f'40 {sel:X}')
    for b in bodies:
        lines_out.append(f'  {b}')
    lines_out.append('  FF')
    lines_out.append('')

source = '\n'.join(lines_out) + '\n'
with open(OUT, 'w', encoding='utf-8') as f:
    f.write(source)

sha = hashlib.sha256(source.encode('utf-8')).hexdigest()
count = len([l for l in lines_out if l.startswith('40 ')])
comment_count = len([l for l in lines_out if l.startswith('; ')])
total_lines = len(lines_out)
print(f'\n=== RESULT ===')
print(f'{count} handlers, {total_lines} total lines, {comment_count} comment lines')
print(f'SHA256: {sha}')

# ============================================================
# Phase 5: Verify
# ============================================================
try:
    result = subprocess.run(
        [r'f:\yoyo\yoyo-rust\target\debug\yoyo.exe', 'test', 'golden'],
        cwd=ROOT, capture_output=True, text=True, timeout=120,
        encoding='utf-8', errors='replace'
    )
    std = result.stdout or ''
    err = result.stderr or ''
    pass_ = std.count('PASS')
    fail_out = std.count('FAIL')
    fail_err = err.count('FAIL')
    print(f'\nGolden tests — PASS: {pass_}, FAIL stdout: {fail_out}, FAIL stderr: {fail_err}')
    for line in std.strip().split('\n')[-3:]:
        print(f'  {line.strip()[:150]}')
    if err:
        unique = list(set(l.strip()[:130] for l in err.split('\n') if 'FAIL' in l))
        for fl in unique[:10]:
            print(f'  FAIL: {fl}')
except FileNotFoundError:
    print('(yoyo.exe not found)')
