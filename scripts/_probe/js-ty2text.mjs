// Compile yoyo.ty via JS M0 and emit raw x64 bytes
import fs from 'fs';
import { encodeOp, loadState, storeState, movabsRax } from '../../yoyo-js/src/platform/encode-x64.js';

// Mirror yoyo.js M0 parser
function parseTy(src) {
  const lines = [];
  const names = new Map();
  let nextSlot = 0x50;
  const isHex = (t) => /^(0x)?[0-9a-fA-F]+$/.test(t);
  const hex = (t) => parseInt(t.replace(/^0x/i, ''), 16);
  const slotOf = (t) => {
    if (isHex(t)) return hex(t);
    if (names.has(t)) return names.get(t);
    const s = nextSlot++;
    names.set(t, s);
    return s;
  };
  for (const raw of src.split(/\r?\n/)) {
    let line = raw.replace(/[;#].*$/, '').trim();
    if (!line || /^LAYOUT$/i.test(line) || /^END_LAYOUT$/i.test(line)) continue;
    const toks = line.split(/\s+/);
    if (toks.length === 0 || !toks[0]) continue;
    const op = hex(toks[0]);
    const args = toks.slice(1).map(slotOf);
    lines.push({ op, args });
  }
  return lines;
}

function compile(lines) {
  const code = [];
  const data = [];
  const labels = new Map();
  const fixups = [];
  // multi-digit hex tokens (e.g. `40 100`); wrapping via &0xff would collide H_00..
  const labelId = (a) => {
    const hh = Number(a[0]);
    if (!Number.isInteger(hh) || hh < 0 || hh > 0xffff) {
      throw new Error('label id out of range: ' + a[0]);
    }
    return hh;
  };
  for (const { op, args } of lines) {
    if (op === 0x40) {
      labels.set(labelId(args), code.length);
      continue;
    }
    if (op === 0x10 || op === 0x12 || op === 0x13) {
      for (const a of args) data.push(a & 0xff);
      continue;
    }
    if (op === 0x41 || op === 0x70 || (op >= 0x71 && op <= 0x7a)) {
      const start = code.length;
      const bytes = encodeOp(op, args, true);
      code.push(...bytes);
      const relAt = op >= 0x71 && op <= 0x7a ? start + 2 : start + 1;
      fixups.push({ relAt, hh: labelId(args) });
      continue;
    }
    code.push(...encodeOp(op, args, false));
  }
  for (const f of fixups) {
    if (!labels.has(f.hh)) throw new Error('undefined label H_' + f.hh.toString(16));
    const rel = labels.get(f.hh) - (f.relAt + 4);
    const b = Buffer.alloc(4); b.writeInt32LE(rel, 0);
    code[f.relAt] = b[0]; code[f.relAt + 1] = b[1];
    code[f.relAt + 2] = b[2]; code[f.relAt + 3] = b[3];
  }
  return { code: Buffer.from(code), data: Buffer.from(data), labels };
}

const src = fs.readFileSync(process.argv[2], 'utf8');
const { code } = compile(parseTy(src));
process.stdout.write(code);
