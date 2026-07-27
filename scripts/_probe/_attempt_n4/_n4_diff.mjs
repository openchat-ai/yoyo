#!/usr/bin/env node
// _n4_diff.mjs — D-1 probe diff (JS hex-dump text vs Rust flat binary).
//   JS text   → scripts/_probe/_attempt_n4/js_out.txt (hex-dump format)
//   Rust flat → scripts/_probe/_attempt_n4/rust_out.bin (1B startup + 90B code)
// After stripping the leading 0xC3 stub-startup, both streams should be 90B code.
import fs from 'node:fs';
import path from 'node:path';
import crypto from 'node:crypto';

const ROOT = 'f:/yoyo';
const DIR = path.join(ROOT, 'scripts/_probe/_attempt_n4');

// 1) Load JS hex-dump text → bytes. PowerShell `>` redirected the wrapper
// stdout as UTF-16 LE (BOM + each ASCII char + 0x00), so we detect BOM and
// decode accordingly.
const jsRaw = fs.readFileSync(path.join(DIR, 'js_out.txt'));
let jsTxt;
if (jsRaw[0] === 0xff && jsRaw[1] === 0xfe) {
  jsTxt = jsRaw.slice(2).toString('utf16le');
} else if (jsRaw[0] === 0xef && jsRaw[1] === 0xbb && jsRaw[2] === 0xbf) {
  jsTxt = jsRaw.slice(3).toString('utf8');
} else {
  jsTxt = jsRaw.toString('utf8');
}
console.error('[debug] jsTxt.length=' + jsTxt.length + ' firstLine=' + JSON.stringify(jsTxt.split(/\r?\n/)[0]));
const jsLines = jsTxt.split(/\r?\n/);
const jsHexPieces = [];
let jsLenFromText = null;
for (const ln of jsLines) {
  if (ln.startsWith('len=')) {
    const m = /len=(\d+) sha256=([0-9a-f]+)/.exec(ln);
    if (m) jsLenFromText = parseInt(m[1], 10);
    continue;
  }
  const m = /^[0-9a-f]+:\s*(.*)$/.exec(ln);
  if (m) jsHexPieces.push(m[1].replace(/\s+/g, ''));
}
const jsHex = jsHexPieces.join('');
const jsBuf = Buffer.from(jsHex, 'hex');
const jsSha = crypto.createHash('sha256').update(jsBuf).digest('hex');

// 2) Load Rust flat bin → strip leading 0xC3 (stub startup) → code bytes
const rustRaw = fs.readFileSync(path.join(DIR, 'rust_out.bin'));
const rustCode = rustRaw[0] === 0xC3 ? rustRaw.slice(1) : rustRaw;
const rustSha = crypto.createHash('sha256').update(rustCode).digest('hex');

// 3) Stats
const len = Math.min(jsBuf.length, rustCode.length);
let byteEqual = 0;
let firstDiff = -1;
const diffs = [];
for (let i = 0; i < len; i++) {
  if (jsBuf[i] === rustCode[i]) {
    byteEqual++;
  } else {
    if (firstDiff < 0) firstDiff = i;
    diffs.push({ off: i, js: jsBuf[i], rust: rustCode[i] });
  }
}
const parity = jsBuf.length === rustCode.length;

console.error('=== N4 synth-d1.ty diff ===');
console.error('JS   bytes=' + jsBuf.length + ' sha256=' + jsSha);
console.error('Rust bytes=' + rustCode.length + ' sha256=' + rustSha + ' (stripped 1B startup)');
console.error('len(text)=' + jsLenFromText);
console.error('len-parity=' + parity);
console.error('byte-equal=' + byteEqual + '/' + len + '  (' + (len ? (100 * byteEqual / len).toFixed(2) : '0') + '%)');
console.error('first-diff-offset=' + (firstDiff < 0 ? 'none' : '0x' + firstDiff.toString(16) + ' (' + firstDiff + ')'));
if (diffs.length) {
  console.error('total-diff-bytes=' + diffs.length);
  const grouped = [];
  let runStart = diffs[0].off, runLast = diffs[0].off;
  for (let i = 1; i < diffs.length; i++) {
    if (diffs[i].off === runLast + 1) {
      runLast = diffs[i].off;
    } else {
      grouped.push([runStart, runLast]);
      runStart = diffs[i].off;
      runLast = diffs[i].off;
    }
  }
  grouped.push([runStart, runLast]);
  console.error('diff-runs: ' + grouped.map(([a, b]) => `0x${a.toString(16)}-0x${b.toString(16)}(len=${b - a + 1})`).join(', '));
  for (const [a, b] of grouped) {
    const sliceJs = jsBuf.slice(a, b + 1).toString('hex');
    const sliceRust = rustCode.slice(a, b + 1).toString('hex');
    console.error(`  0x${a.toString(16)}-0x${b.toString(16)}:`);
    console.error(`    JS   = ${sliceJs.match(/.{2}/g).join(' ')}`);
    console.error(`    Rust = ${sliceRust.match(/.{2}/g).join(' ')}`);
  }
}

// 4) Decode the handlers in JS stream — find each "C3" terminator
// and group the preceding bytes into 18B handler blocks. Then re-run same on
// Rust stream. Map each block to its offset range.
function splitHandlers(buf) {
  const blocks = [];
  let cur = 0;
  for (let i = 0; i < buf.length; i++) {
    if (buf[i] === 0xc3) {
      blocks.push([cur, i]);
      cur = i + 1;
    }
  }
  return blocks;
}
const jsBlocks = splitHandlers(jsBuf);
const rustBlocks = splitHandlers(rustCode);
console.error('\nJS handler blocks: ' + jsBlocks.map(([a, b]) => `0x${a.toString(16)}-0x${b.toString(16)}(${b - a + 1}B)`).join(', '));
console.error('Rust handler blocks: ' + rustBlocks.map(([a, b]) => `0x${a.toString(16)}-0x${b.toString(16)}(${b - a + 1}B)`).join(', '));

if (jsBlocks.length !== rustBlocks.length) {
  console.error('!! handler count differs (js=' + jsBlocks.length + ' rust=' + rustBlocks.length + ')');
} else {
  for (let h = 0; h < jsBlocks.length; h++) {
    const [ja, jb] = jsBlocks[h];
    const [ra, rb] = rustBlocks[h];
    const jsSlice = jsBuf.slice(ja, jb + 1);
    const rsSlice = rustCode.slice(ra, rb + 1);
    const eq = jsSlice.equals(rsSlice);
    const headHex = jsSlice.slice(0, 5).toString('hex'); // first 5 bytes = op + modrm + disp[0..2]
    console.error(`H_${h.toString(16).padStart(2, '0')}: js=0x${ja.toString(16)}-0x${jb.toString(16)} rust=0x${ra.toString(16)}-0x${rb.toString(16)} len=${jb - ja + 1} ${eq ? 'EQUAL' : 'DIFF'} head=${headHex.match(/.{2}/g).join(' ')}`);
  }
}

// 5) Save pure code binaries for inspection
fs.writeFileSync(path.join(DIR, 'js_code.bin'), jsBuf);
fs.writeFileSync(path.join(DIR, 'rust_code.bin'), rustCode);
console.error('\n[wrote] js_code.bin rust_code.bin');

if (firstDiff < 0 && parity) {
  console.error('\nVERDICT: perfect byte-equal (D-1 did not trigger on this synth)');
} else if (parity && firstDiff >= 0) {
  console.error('\nVERDICT: byte-equal breaks at 0x' + firstDiff.toString(16));
} else {
  console.error('\nVERDICT: length mismatch (js=' + jsBuf.length + ' rust=' + rustCode.length + ')');
}