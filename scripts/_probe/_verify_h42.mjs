// Mirror golden.js parseTy (with signed hex support) + encodeOp, compile H_42 standalone
import { encodeOp } from '../../yoyo-js/src/platform/encode-x64.js';

function parseTy(src) {
  const lines = [];
  const names = new Map();
  let nextSlot = 0x50;
  const isHex = (t) => /^(0x)?[0-9a-fA-F]+$/.test(t);
  const hex = (t) => parseInt(t.replace(/^0x/i, ''), 16);
  const isSignedHex = (t) => /^-0x[0-9a-fA-F]+$/.test(t) || /^-[0-9a-fA-F]+$/.test(t);
  const signedHex = (t) => -parseInt(t.replace(/^-/, '').replace(/^0x/i, ''), 16);
  const slotOf = (t) => {
    if (isSignedHex(t)) return signedHex(t);
    if (isHex(t)) return hex(t);
    if (names.has(t)) return names.get(t);
    const s = nextSlot++;
    names.set(t, s);
    return s;
  };
  for (const raw of src.split(/\r?\n/)) {
    const line = raw.replace(/[;#].*$/, '').trim();
    if (!line) continue;
    const toks = line.split(/\s+/);
    const op = parseInt(toks[0], 16);
    const args = toks.slice(1).map(slotOf);
    lines.push({ op, args });
  }
  return lines;
}

const src = `40 30
  80 50 60 -80
  FF
`;
const lines = parseTy(src);
console.log('Parsed lines:');
for (const l of lines) console.log('  ', JSON.stringify(l));

const code = [];
const labels = new Map();
for (const { op, args } of lines) {
  if (op === 0x40) {
    labels.set(args[0] & 0xff, code.length);
    continue;
  }
  code.push(...encodeOp(op, args, false));
}

const hex = Buffer.from(code).toString('hex');
const expected = '498b87000300004883c080480fb60049898780020000c3';
console.log('Compiled H_42 bytes:', hex, 'len=' + code.length);
console.log('Expected:           ', expected, 'len=23');
console.log('Match:', hex === expected);