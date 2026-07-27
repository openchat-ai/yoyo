// _attempt_n3_diff.mjs — per-op masked section diff
import fs from 'node:fs';
import crypto from 'node:crypto';

const js = fs.readFileSync('f:/yoyo/scripts/_probe/js_yoyoty_code.bin');
const rust = fs.readFileSync('f:/yoyo/scripts/_probe/rust_yoyoty_code.bin');

// Sanity: full-stream equality
console.error('=== full-stream compare ===');
console.error('js   bytes=' + js.length + ' sha256=' + crypto.createHash('sha256').update(js).digest('hex'));
console.error('rust bytes=' + rust.length + ' sha256=' + crypto.createHash('sha256').update(rust).digest('hex'));
console.error('byte-equal-all=' + js.equals(rust));

// Per-op signatures (canonical x64 per N1/N2/three-peer-compare):
//   INC slot[0x50]: 49 8b 87 80 02 00 00 48 ff c0 49 89 87 80 02 00 00 c3 (18B, ends with C3 RET)
//   DEC slot[0x50]: 49 8b 87 80 02 00 00 48 ff c8 49 89 87 80 02 00 00 c3 (18B, ends with C3 RET)
//   JMP rel32:     e9 ?? ?? ?? ?? c3 (6B, ends with C3 RET)
//
// Greedy opcode scan: for each opcode, find next occurrence of its 4B head after
// `start`, then scan forward for the trailing c3 (RET) to bracket the span.

function findOpSpans(buf, headBytes, terminator = 0xc3) {
  // Returns array of [start, endInclusive] byte-offset spans in buf that begin
  // with headBytes and end with `terminator` (or have terminator as last byte).
  const spans = [];
  let i = 0;
  while (i + headBytes.length <= buf.length) {
    let match = true;
    for (let k = 0; k < headBytes.length; k++) {
      if (buf[i + k] !== headBytes[k]) { match = false; break; }
    }
    if (!match) { i++; continue; }
    // Find trailing terminator (RET 0xC3) within a sensible window (≤ 32B).
    let end = -1;
    for (let j = i + headBytes.length; j < Math.min(i + 32, buf.length); j++) {
      if (buf[j] === terminator) { end = j; break; }
    }
    if (end < 0) { i++; continue; }
    spans.push([i, end]);
    i = end + 1;
  }
  return spans;
}

const INC_HEAD = Buffer.from([0x49, 0x8b, 0x87, 0x80, 0x02, 0x00, 0x00, 0x48, 0xff, 0xc0]);
const DEC_HEAD = Buffer.from([0x49, 0x8b, 0x87, 0x80, 0x02, 0x00, 0x00, 0x48, 0xff, 0xc8]);
const JMP_HEAD = Buffer.from([0xe9]);

const incJs = findOpSpans(js, INC_HEAD);
const incRust = findOpSpans(rust, INC_HEAD);
const decJs = findOpSpans(js, DEC_HEAD);
const decRust = findOpSpans(rust, DEC_HEAD);
const jmpJs = findOpSpans(js, JMP_HEAD);
const jmpRust = findOpSpans(rust, JMP_HEAD);

function report(label, jsSpans, rustSpans, headHex, opName) {
  console.error(`\n=== ${label} (head=${headHex}) ===`);
  console.error(`js   spans=[${jsSpans.map(s => `${s[0].toString(16)}-${s[1].toString(16)}`).join(', ')}] (${jsSpans.length})`);
  console.error(`rust spans=[${rustSpans.map(s => `${s[0].toString(16)}-${s[1].toString(16)}`).join(', ')}] (${rustSpans.length})`);
  if (jsSpans.length !== rustSpans.length) {
    console.error(`!! ${opName}: count differs (js=${jsSpans.length} rust=${rustSpans.length})`);
    return { ok: false, reason: 'count-differs' };
  }
  let allEqual = true;
  const rows = [];
  for (let i = 0; i < jsSpans.length; i++) {
    const [js0, js1] = jsSpans[i];
    const [rs0, rs1] = rustSpans[i];
    if (js0 !== rs0 || js1 !== rs1) {
      console.error(`!! ${opName}#${i}: offset-range differs (js=[${js0.toString(16)},${js1.toString(16)}] vs rust=[${rs0.toString(16)},${rs1.toString(16)}])`);
      allEqual = false;
      rows.push({ idx: i, status: 'OFFSET_DIFF', js: [js0, js1], rust: [rs0, rs1] });
      continue;
    }
    const len = js1 - js0 + 1;
    const jsSlice = js.slice(js0, js1 + 1);
    const rsSlice = rust.slice(rs0, rs1 + 1);
    const equal = jsSlice.equals(rsSlice);
    const first8 = jsSlice.subarray(0, 4).toString('hex');
    const last8 = jsSlice.subarray(len - 4).toString('hex');
    rows.push({ idx: i, status: equal ? 'EQUAL' : 'BYTE_DIFF', first8, last8, len });
    if (!equal) allEqual = false;
    console.error(`  ${opName}#${i}: span=[${js0.toString(16)},${js1.toString(16)}] len=${len} first8=${first8} last8=${last8} ${equal ? 'EQUAL' : 'BYTE_DIFF'}`);
    if (!equal) {
      const diffs = [];
      for (let k = 0; k < jsSlice.length; k++) if (jsSlice[k] !== rsSlice[k]) diffs.push([k, jsSlice[k], rsSlice[k]]);
      console.error('    diff offsets (rel): ' + diffs.map(d => `[${d[0]}] ${d[1].toString(16)} vs ${d[2].toString(16)}`).join(' '));
    }
  }
  return { ok: allEqual, rows };
}

const incRes = report('INC', incJs, incRust, INC_HEAD.toString('hex'), 'INC');
const decRes = report('DEC', decJs, decRust, DEC_HEAD.toString('hex'), 'DEC');
const jmpRes = report('JMP', jmpJs, jmpRust, JMP_HEAD.toString('hex'), 'JMP');

// Mask anything not inside INC/DEC/JMP spans, plus D-1 ops (`0x20/0x50/0x51`),
// then diff. Note: D-1 ops in the SLOT-by-name path are not standalone opcodes
// (they are slot addresses); the actual emitting ops for slot addressing are
// MOV r64, [r15+disp32] (load_state) and MOV [r15+disp32], r64 (store_state)
// already covered by SET/GET shape. We still mask the 4B of slot disp inside
// each store/load as a courtesy, but the slot disp bytes are part of the
// canonical SET/GET shape and identical across peers in this locked stub.
const mask = Buffer.alloc(js.length, 0x00);
// Unmask: all bytes inside INC/DEC/JMP spans
function unmaskSpans(spans) {
  for (const [a, b] of spans) {
    for (let i = a; i <= b; i++) mask[i] = 0xff;
  }
}
unmaskSpans(incJs);
unmaskSpans(incRust); // same offsets
unmaskSpans(decJs);
unmaskSpans(decRust);
unmaskSpans(jmpJs);
unmaskSpans(jmpRust);

const maskedJs = Buffer.from(js).map((b, i) => b & mask[i]);
const maskedRust = Buffer.from(rust).map((b, i) => b & mask[i]);
const equalMasked = maskedJs.equals(maskedRust);
let byteEqual = 0;
let total = 0;
for (let i = 0; i < js.length; i++) {
  total++;
  if (js[i] === rust[i]) byteEqual++;
}
console.error('\n=== summary ===');
console.error(`total bytes compared: ${total}`);
console.error(`byte-equal rate:      ${(byteEqual / total * 100).toFixed(2)}% (${byteEqual}/${total})`);
console.error(`masked INC/DEC/JMP regions byte-equal: ${equalMasked}`);
console.error(`INC verdict: ${incRes.ok ? 'EQUAL' : 'DIFF'}`);
console.error(`DEC verdict: ${decRes.ok ? 'EQUAL' : 'DIFF'}`);
console.error(`JMP verdict: ${jmpRes.ok ? 'EQUAL' : 'DIFF'}`);