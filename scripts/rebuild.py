"""Rebuild yoyo.ty from scratch: initial stub + body-extend logs."""
import os, re, glob, subprocess, json, hashlib

ROOT = r'f:\yoyo'
AUX = os.path.join(ROOT, 'docs', 'auxdocs')
OUT = os.path.join(ROOT, 'yoyo', 'projects', 'yoyo.ty')

# ===== Step 1: Initial stub from PROMPT (34 handlers, H_00..H_2D) =====
# These are the canonical W-SM stubs. Each handler is:
# H_NNN (desc)
# selector
#   opcode args
#   FF

initial = [
    # H_00: SET-UP slot=50 imm=0x2A (movabs rax,0x2A; store [r15+0x200])
    # H_01: GET slot=51 slot=50 (load [r15+0x200]→rax; store rax→[r15+0x208])
    # H_02: ADDV slot=50 slot=51 (load+add+store)
    # H_03: SET slot=50 imm=7 (movabs rax,7; store)
    # H_04: GET slot=51 slot=50 (load+store)
    # H_05..H_10: RAW_BYTE chain (90 c3, 90 90 c3, ...)
    # H_11: INC slot=50 (load+inc+store)
    # H_12: DEC slot=50 (load+dec+store)
    # H_13: NOP (90 c3)
    # H_14: RAW_BYTES [cc dd]
    # H_15: IMUL slot=50 slot=51
    # H_16: SUBV slot=50 slot=51
    # H_17: CMP slot=50 slot=51
    # H_18: LDB slot=50 slot=51
    # H_19: JMP H_00
    # H_20: CALL H_00
    # H_21: JE H_00
    # H_22..H_2A: 9 Jcc to H_00
    # H_2B: SET-CONTROL imm=0
    # H_2C: GET slot=51 slot=50
    # H_2D: LDB slot=51 slot=50

    # I'll generate the actual bytes from the JS encoder for each
]

# Generate bytes using encodeOp
def gen_bytes(opcode, args):
    """Returns (hex_string, length) for a handler body + ret."""
    js = f"""
    const {{encodeOp}} = require('./yoyo-js/src/platform/encode-x64.js');
    const r = encodeOp(0x{opcode:02x}, [{','.join(str(a) for a in args)}]);
    const r2 = [...r, 0xc3];
    process.stdout.write(JSON.stringify({{'hex': Buffer.from(r2).toString('hex'), 'len': r2.length}}));
    """
    try:
        result = subprocess.run(
            ['node', '-e', js], cwd=ROOT, capture_output=True, text=True, timeout=15
        )
        data = json.loads(result.stdout.strip())
        return data['hex'], data['len']
    except Exception as e:
        return f'ERROR({e})', 0

# Initial stub definitions (from PROMPT W-SM section)
# Format: (h, sel, opcode_byte, args_list, desc)
stub_defs = [
    (0, 0x00, 0x20, [0x50, 0x2A], 'SET-UP slot=50 imm=0x2A'),
    (1, 0x01, 0x60, [0x51, 0x50], 'GET slot=51 slot=50'),
    (2, 0x02, 0x68, [0x50, 0x51], 'ADDV slot=50 slot=51'),
    (3, 0x03, 0x30, [0x50, 7], 'SET slot=50 imm=7'),
    (4, 0x04, 0x60, [0x51, 0x50], 'GET slot=51 slot=50'),
    (5, 0x05, 0xA0, [0x90], 'RAW_BYTE 0x90 (NOP)'),
    (6, 0x06, 0xA0, [0x90, 0x90], 'RAW_BYTE 0x90 0x90'),
    (7, 0x07, 0xA0, [0x90, 0x90, 0x90], 'RAW_BYTE 0x90 0x90 0x90'),
    (8, 0x08, 0xA0, [0x90, 0x90, 0x90, 0x90], 'RAW_BYTE 0x90*4'),
    (9, 0x09, 0xA0, [0x90, 0x90, 0x90, 0x90, 0x90], 'RAW_BYTE 0x90*5'),
    (10, 0x0A, 0xA0, [0x90]*6, 'RAW_BYTE 0x90*6'),
    (11, 0x0B, 0x66, [0x50], 'INC slot=50'),
    (12, 0x0C, 0x67, [0x50], 'DEC slot=50'),
    (13, 0x0D, 0x00, [], 'NOP'),
    (14, 0x0E, 0xA1, [0xCC, 0xDD], 'RAW_BYTES cc dd'),
    (15, 0x0F, 0x63, [0x50, 0x51], 'IMUL slot=50 slot=51'),
    (16, 0x10, 0x6A, [0x50, 0x51], 'SUBV slot=50 slot=51'),
    (17, 0x11, 0x65, [0x50, 0x51], 'CMP slot=50 slot=51'),
    (18, 0x12, 0x80, [0x50, 0x51, 0x00], 'LDB slot=50 slot=51 oo=0'),
    (19, 0x13, 0x70, [0x00], 'JMP H_00'),
    (20, 0x14, 0x41, [0x00], 'CALL H_00'),
    # H_21..H_2A: 10 Jcc (0x71..0x7A)
    (21, 0x15, 0x71, [0x00], 'JE H_00'),
    (22, 0x16, 0x72, [0x00], 'JNE H_00'),
    (23, 0x17, 0x73, [0x00], 'JL H_00'),
    (24, 0x18, 0x74, [0x00], 'JGE H_00'),
    (25, 0x19, 0x75, [0x00], 'JLE H_00'),
    (26, 0x1A, 0x76, [0x00], 'JG H_00'),
    (27, 0x1B, 0x77, [0x00], 'JB H_00'),
    (28, 0x1C, 0x78, [0x00], 'JAE H_00'),
    (29, 0x1D, 0x79, [0x00], 'JBE H_00'),
    (30, 0x1E, 0x7A, [0x00], 'JA H_00'),
    (31, 0x1F, 0x50, [0x00], 'SET-CONTROL imm=0'),
    (32, 0x20, 0x60, [0x51, 0x50], 'GET slot=51 slot=50'),
    (33, 0x21, 0x80, [0x51, 0x50, 0x00], 'LDB slot=51 slot=50 oo=0'),
]

# ===== Step 2: Parse body-extend logs for handler additions =====
logs = sorted(glob.glob(os.path.join(AUX, 'body-extend-*-log.md')))
def sort_key(f):
    m = re.search(r'body-extend-(\d+)', os.path.basename(f))
    return int(m.group(1)) if m else 0
logs.sort(key=sort_key)

# Parse each log for handler additions
extended = {}  # h -> {sel, opcode, args, len, sha}
for logfile in logs:
    with open(logfile, 'r', encoding='utf-8', errors='replace') as f:
        content = f.read()
    # Pattern: | H_717 | 0x2D3 | 0x80 LDB | 51 60 228 | 26 | `ec662f4d79ff8add` |
    pattern = r'\|\s*H_(\d+)\s*\|\s*0x([0-9A-Fa-f]+)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*(\d+)\s*\|\s*`([^`]+)`\s*\|'
    for m in re.finditer(pattern, content):
        h, sel, op, args, blen, sha = m.groups()
        h = int(h)
        if h not in extended:
            extended[h] = {
                'sel': int(sel, 16), 'op': op.strip(), 'args': args.strip(),
                'len': int(blen), 'sha': sha.strip(), 'log': os.path.basename(logfile)
            }

# ===== Step 3: Generate the full file =====
lines = []
def add(h, sel, opcode_byte, args_list, desc_str, pin_hex='', pin_len=0):
    global lines
    lines.append(f'; H_{h:03d} — {desc_str}')
    if pin_hex:
        lines.append(f';   Pin {pin_len}B: {pin_hex}.')
    lines.append(f';   Not full self-host / not G06 / not Phase 2 / not freeze.')
    lines.append(f'40 {sel:03X}')
    # Format body: opcode_byte hex args
    if opcode_byte == 0xA0:
        # RAW_BYTE: body is just the raw bytes
        args_hex = ' '.join(f'{a:02X}' for a in args_list)
        lines.append(f'  A0 {args_hex}')
    elif opcode_byte == 0xA1:
        args_hex = ' '.join(f'{a:02X}' for a in args_list)
        lines.append(f'  A1 {args_hex}')
    elif opcode_byte == 0x00:
        lines.append('  00')  # NOP
    else:
        # For JMP/CALL/Jcc, args are handler indexes
        # For others, args are hex values
        if opcode_byte in (0x70, 0x41):  # JMP, CALL
            target_h = args_list[0]
            args_hex = f'{target_h:02X}'
        elif 0x71 <= opcode_byte <= 0x7A:  # Jcc
            target_h = args_list[0]
            args_hex = f'{target_h:02X}'
        elif opcode_byte == 0x50:  # SET-CONTROL
            args_hex = f'{args_list[0]:02X}'
        else:
            args_hex = ' '.join(f'{a:02X}' for a in args_list)
        lines.append(f'  {opcode_byte:02X} {args_hex}')
    lines.append('  FF')
    lines.append('')

# Header
lines.append('; yoyo.ty — YOYO v3 compiler body (PROMPT-v3 Part 4S.3)')
lines.append(';')
lines.append('; AUTO-RECONSTRUCTED from body-extend-*-log.md files')
lines.append('; W-START: EXPERIMENTAL · NON-GREEN')
lines.append('')

# Add initial stub handlers
for h, sel, op, args, desc in stub_defs:
    hex_bytes, blen = gen_bytes(op, args)
    add(h, sel, op, args, desc, hex_bytes, blen)

# Add extended handlers from logs
# Sort by handler number
for h in sorted(extended.keys()):
    info = extended[h]
    # Parse opcode and args for gen_bytes
    op_match = re.match(r'0x([0-9A-Fa-f]+)\s+(\w+)', info['op'])
    if not op_match:
        continue
    opcode = int(op_match.group(1), 16)
    args_list = []
    for x in info['args'].split():
        try:
            args_list.append(int(x, 10))
        except ValueError:
            args_list.append(int(x, 16))
    desc = f'{info["op"]} {info["args"]}'
    hex_bytes, blen = gen_bytes(opcode, args_list)
    add(h, info['sel'], opcode, args_list, desc, hex_bytes, blen)

# Write output
source = '\n'.join(lines) + '\n'
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, 'w', encoding='utf-8') as f:
    f.write(source)

sha = hashlib.sha256(source.encode('utf-8')).hexdigest()
print(f'Written {len(lines)} lines')
print(f'Handlers: stub={len(stub_defs)} + extended={len(extended)} = {len(stub_defs)+len(extended)}')
print(f'SHA256: {sha}')
print(f'Expected: 20391de3e4855c52d3b918753ae6013a7ed4b60bc529d497867f27e95a32b315')
print(f'Match: {sha == "20391de3e4855c52d3b918753ae6013a7ed4b60bc529d497867f27e95a32b315"}')