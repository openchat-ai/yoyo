#!/usr/bin/env node
// hex2bin.mjs — read a hex dump from stdin (output of js-ty2text.mjs)
// and emit raw bytes to a file (path in argv[2]).
//
// Accepts lines like:
//   "0000: 48 b8 00 00 00 00 00 00 00 00 49 89 87 80 02 00"
// Skips lines like:
//   "len=36 sha256=..."
//   "rgit loaded"
//
// We write the output to a file path argument (not stdout) because
// PowerShell `>` redirects use UTF-16 LE which corrupts the binary
// payload.

import fs from 'node:fs';

const outPath = process.argv[2];
if (!outPath) {
  process.stderr.write('usage: node hex2bin.mjs <out.bin>  < hex-dump-on-stdin\n');
  process.exit(2);
}

const raw = fs.readFileSync(0); // stdin (raw bytes)
let text;
if (raw.length >= 2 && raw[0] === 0xff && raw[1] === 0xfe) {
  text = raw.subarray(2).toString('utf16le');
} else if (raw.length >= 3 && raw[0] === 0xef && raw[1] === 0xbb && raw[2] === 0xbf) {
  text = raw.subarray(3).toString('utf8');
} else {
  text = raw.toString('utf8');
}

const out = [];
for (const line of text.split(/\r?\n/)) {
  const m = line.match(/^[0-9a-f]+:\s+([0-9a-f ]+)$/i);
  if (!m) continue;
  for (const tok of m[1].trim().split(/\s+/)) {
    if (tok.length === 2) out.push(parseInt(tok, 16));
  }
}
fs.writeFileSync(outPath, Buffer.from(out));
process.stdout.write(`hex2bin: wrote ${out.length} bytes to ${outPath}\n`);