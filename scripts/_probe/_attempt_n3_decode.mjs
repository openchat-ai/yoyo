// extract_compare.mjs — convert hex-dump text to bytes; compare JS vs Rust code
import fs from 'node:fs';
import crypto from 'node:crypto';

const txt = fs.readFileSync('f:/yoyo/scripts/_probe/js_yoyoty_text.txt', 'utf8');
const lines = txt.split(/\r?\n/);
const hexPieces = [];
for (const ln of lines) {
  if (ln.startsWith('len=')) continue;
  // skip empty or no-prefix lines
  const m = /^[0-9a-f]+:\s*(.*)$/.exec(ln);
  if (m) hexPieces.push(m[1].replace(/\s+/g, ''));
}
const jsHex = hexPieces.join('');
const jsBuf = Buffer.from(jsHex, 'hex');

const rustRaw = fs.readFileSync('f:/yoyo/scripts/_probe/rust_yoyoty_text.bin');
const rustCode = rustRaw.slice(1); // skip stub startup c3

console.error('js bytes=' + jsBuf.length + ' sha256=' + crypto.createHash('sha256').update(jsBuf).digest('hex'));
console.error('rust code bytes=' + rustCode.length + ' sha256=' + crypto.createHash('sha256').update(rustCode).digest('hex'));
console.error('byte-equal-all=' + jsBuf.equals(rustCode));

fs.writeFileSync('f:/yoyo/scripts/_probe/js_yoyoty_code.bin', jsBuf);
fs.writeFileSync('f:/yoyo/scripts/_probe/rust_yoyoty_code.bin', rustCode);