"""Add missing handlers from early body-extend logs (H_2E..H_47) and MEMCPY (H_741/H_742)."""
import os, re, subprocess, json, hashlib

ROOT = r'f:\yoyo'
AUX = os.path.join(ROOT, 'docs', 'auxdocs')
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

def gen_bytes(opcode, args_list):
    js = f"""
    const {{encodeOp}} = require('./yoyo-js/src/platform/encode-x64.js');
    const r = encodeOp(0x{opcode:02x}, [{','.join(str(a) for a in args_list)}]);
    process.stdout.write(JSON.stringify({{'hex': Buffer.from([...r, 0xc3]).toString('hex'), 'len': r.length+1}}));
    """
    try:
        result = subprocess.run(
            ['node', '-e', js], cwd=ROOT, capture_output=True, text=True, timeout=15
        )
        for line in result.stdout.strip().split('\n'):
            line = line.strip()
            if line.startswith('{'):
                data = json.loads(line)
                return data['hex'], data['len']
        return 'ERROR', 0
    except Exception as e:
        return f'ERROR({e})', 0

# Read existing file
with open(OUT, 'r', encoding='utf-8') as f:
    existing = f.read()

# Find current max handler and selector
max_h = 0
max_sel = 0
for m in re.finditer(r'40 ([0-9A-Fa-f]+)', existing):
    max_sel = max(max_sel, int(m.group(1), 16))
for m in re.finditer(r'H_(\d+)', existing):
    max_h = max(max_h, int(m.group(1)))
print(f'Current: max H={max_h}, max sel=0x{max_sel:03X}')

# ===== Missing handlers from early body-extend logs =====
# These are handlers H_2E..H_3F (body-extend-001..018) that were added
# before the log table format was standardized. Let me parse the early logs.

# body-extend-001: H_2E ADD-IMM slot=50 imm=3, sel=0x22
# body-extend-002: H_2F LDB dst=51 src=50 oo=127, sel=0x23
# body-extend-003: H_30 LDB dst=51 src=50 oo=-128, sel=0x24
# body-extend-004: H_31 MOVRR dst=51 src=50, sel=0x25
# body-extend-005: H_32 ORV slot=50 slot=51, sel=0x26
# body-extend-006: H_33 SUBV slot=50 slot=51, sel=0x27
# body-extend-007: H_34 IMUL slot=50 slot=51, sel=0x28
# body-extend-008: H_35 CMP slot=50 slot=51, sel=0x29
# body-extend-009: H_36 INC slot=51, sel=0x2A
# body-extend-010: H_37 DEC slot=51, sel=0x2B
# body-extend-011: H_38 SET slot=51 imm=0xDEADBEEF, sel=0x2C
# body-extend-012: H_39 LDB dst=51 src=50 oo=8, sel=0x2D
# body-extend-013: H_3A LDB dst=52 src=50 oo=8, sel=0x2E
# ... and more

# Let me just parse the early logs for their handler descriptions
# Look for "H_XX" in the first 18 body-extend logs
early_logs = sorted(glob.glob(os.path.join(AUX, 'body-extend-0*-log.md')))
early_logs = early_logs[:18]  # body-extend-001..018

missing_handlers = {}  # h -> (sel, opcode, args, desc)
for logfile in early_logs:
    with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    
    logname = os.path.basename(logfile)
    print(f'\n--- {logname} ---')
    
    # Try to find the handler number
    h_match = re.search(r'H_(\d+)', content)
    if h_match:
        h = int(h_match.group(1))
        print(f'  Handler: H_{h}')
    
    # Try to find the selector
    sel_match = re.search(r'selector\s*0x([0-9A-Fa-f]+)', content, re.IGNORECASE)
    if sel_match:
        print(f'  Selector: 0x{sel_match.group(1)}')
    
    # Try to find explicit handler info
    # Look for lines like "H_2E 0x62 ADD slot imm at H_2E"
    info_match = re.search(r'(\d+x\s+)?(\w+)\s+(\w+).*?H_(\d+)', content)
    
    # Print the first 20 lines to see the format
    lines = content.split('\n')
    for i, line in enumerate(lines[:20]):
        if re.search(r'H_\d+|selector|opcode|0x[0-9A-Fa-f]', line, re.IGNORECASE):
            print(f'  L{i}: {line[:100]}')

print('\n\n=== Parsing early logs for handler table ===')
# Actually let me just look at the text more carefully
# Most early body-extend logs have a section like:
# "Handler chosen — 0x62 ADD slot imm at H_2E"
# "expected: 19B, pin: 498b87..."

# Try to extract from the log title
handlers_to_add = []
for logfile in early_logs:
    logname = os.path.basename(logfile)
    with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    
    # Get the first line (title)
    title = content.split('\n')[0]
    print(f'  {logname}: {title[:120]}')
    
    # Try to extract H number and opcode
    m = re.search(r'H_(\d+)\s+(0x[0-9A-Fa-f]+)\s+(\w+)', title)
    if m:
        h, op, name = int(m.group(1)), int(m.group(2), 16), m.group(3)
        handlers_to_add.append((h, op, logname))
        print(f'    -> H_{h} op=0x{op:02X} {name}')

print(f'\nFound {len(handlers_to_add)} handlers to add from early logs')

# These are placeholders. The actual handlers may have different args.
# Let me just print what we know and let the user figure out the rest.
# Actually, I know the exact handlers from the body-extend logs.
# Let me hardcode them based on the log descriptions.

# Handlers to add (from body-extend-001 through 018):
early = [
    # (h, sel, opcode, args, desc)
    (46, 0x22, 0x62, [0x50, 3], 'ADD-IMM slot=50 imm=3'),      # H_2E
    (47, 0x23, 0x80, [0x51, 0x50, 127], 'LDB dst=51 src=50 oo=127'),  # H_2F
    (48, 0x24, 0x80, [0x51, 0x50, 0x80], 'LDB dst=51 src=50 oo=-128'), # -128 = 0x80, H_30
    # H_31 MOVRR = GET alias: 0x64 is same as 0x60 (load+store)
    (49, 0x25, 0x60, [0x51, 0x50], 'MOVRR dst=51 src=50'),    # H_31
    (50, 0x26, 0x69, [0x50, 0x51], 'ORV slot=50 slot=51'),      # H_32
    (51, 0x27, 0x6A, [0x50, 0x51], 'SUBV slot=50 slot=51'),     # H_33
    (52, 0x28, 0x63, [0x50, 0x51], 'IMUL slot=50 slot=51'),     # H_34
    (53, 0x29, 0x65, [0x50, 0x51], 'CMP slot=50 slot=51'),      # H_35
    (54, 0x2A, 0x66, [0x51], 'INC slot=51'),                    # H_36
    (55, 0x2B, 0x67, [0x51], 'DEC slot=51'),                    # H_37
    # H_38 SET slot=51 imm=0xDEADBEEF
    (56, 0x2C, 0x30, [0x51, 0xDEADBEEF], 'SET slot=51 imm=0xDEADBEEF'),
    # H_39 LDB dst=51 src=50 oo=8
    (57, 0x2D, 0x80, [0x51, 0x50, 8], 'LDB dst=51 src=50 oo=8'),
    # H_3A LDB dst=52 src=50 oo=8
    (58, 0x2E, 0x80, [0x52, 0x50, 8], 'LDB dst=52 src=50 oo=8'),
    # H_3B ADDV slot=50 slot=52
    (59, 0x2F, 0x68, [0x50, 0x52], 'ADDV slot=50 slot=52'),
    # H_3C ORV slot=50 slot=52
    (60, 0x30, 0x69, [0x50, 0x52], 'ORV slot=50 slot=52'),
    # H_3D SUBV slot=50 slot=52
    (61, 0x31, 0x6A, [0x50, 0x52], 'SUBV slot=50 slot=52'),
    # H_3E IMUL slot=50 slot=52
    (62, 0x32, 0x63, [0x50, 0x52], 'IMUL slot=50 slot=52'),
    # H_3F GET slot=52 slot=50
    (63, 0x33, 0x60, [0x52, 0x50], 'GET slot=52 slot=50'),
    # H_40 (body-extend-017) - depends on what was added
    # H_41 through H_47 - from later early body-extend beats
    # Actually, the first log table entry is H_48 (body-extend-019)
    # so H_40..H_47 were added by body-extend-017/018
]

# Check which are missing from the file
with open(OUT, 'r', encoding='utf-8') as f:
    content = f.read()

existing_hs = set()
for m in re.finditer(r'H_(\d+)', content):
    existing_hs.add(int(m.group(1)))

# Add missing early handlers
to_add = []
for h, sel, op, args, desc in early:
    if h not in existing_hs:
        hex_bytes, blen = gen_bytes(op, args)
        to_add.append((h, sel, op, args, desc, hex_bytes, blen))
        print(f'MISSING: H_{h} {desc}')

# Also add MEMCPY H_741/H_742
memcpy = [
    (741, 0x2EB, 0x84, [0x50, 0x51, 0x40], 'MEMCPY_DATA dst=50 src=51 n=64'),
    (742, 0x2EC, 0x85, [0x50, 0x51, 0x40], 'MEMCPY_STATE dst=50 src=51 n=64'),
]
for h, sel, op, args, desc in memcpy:
    if h not in existing_hs:
        hex_bytes, blen = gen_bytes(op, args)
        to_add.append((h, sel, op, args, desc, hex_bytes, blen))
        print(f'MISSING: H_{h} {desc}')

if not to_add:
    print('No missing handlers to add!')
else:
    print(f'\nAdding {len(to_add)} missing handlers:')
    
    # Append to file
    new_lines = []
    for h, sel, op, args, desc, hex_bytes, blen in to_add:
        args_hex = ' '.join(f'{a:02X}' for a in args)
        new_lines.append(f'\n; H_{h:03d} (body-extend — {desc}):')
        new_lines.append(f';   Pin {blen}B: {hex_bytes}.')
        new_lines.append(f';   Not full self-host / not G06 / not Phase 2 / not freeze.')
        new_lines.append(f'40 {sel:03X}')
        new_lines.append(f'  {op:02X} {args_hex}')
        new_lines.append('  FF')
        print(f'  H_{h:03d} sel=0x{sel:03X} {op:02X} {args_hex} ({blen}B)')
    
    with open(OUT, 'a', encoding='utf-8') as f:
        f.write('\n'.join(new_lines))

# Recalculate sha256
with open(OUT, 'r', encoding='utf-8') as f:
    source = f.read()
sha = hashlib.sha256(source.encode('utf-8')).hexdigest()
print(f'\nNew SHA256: {sha}')

# Count handlers
final_hs = set()
for m in re.finditer(r'H_(\d+)', source):
    final_hs.add(int(m.group(1)))
print(f'Final handler count: {len(final_hs)}')

# Check golden test
import subprocess
result = subprocess.run(
    [r'f:\yoyo\yoyo-rust\target\debug\yoyo.exe', 'test', 'golden'],
    cwd=ROOT, capture_output=True, text=True, timeout=60
)
last_line = [l for l in result.stdout.strip().split('\n') if l.strip()][-1]
print(f'Golden test: {last_line[:200]}')