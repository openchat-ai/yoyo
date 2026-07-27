// One-shot: emit yoyo.ty via JS M0 + dump hex + length
import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import fs from 'node:fs';
const out = execFileSync('node', ['scripts/_probe/js-ty2text.mjs', 'yoyo/projects/yoyo.ty']);
const arr = Array.from(out);
const hex = arr.map(b => b.toString(16).padStart(2, '0')).join('');
const sha = createHash('sha256').update(Buffer.from(arr)).digest('hex');
fs.writeFileSync('scripts/_probe/js_yoyoty_h40.code.bin', out);
console.log('js_yoyoty_h40:');
console.log('  len=' + arr.length);
console.log('  hex_prefix=' + hex.slice(0, 64) + '...');
console.log('  sha256=' + sha);
console.log('  full_hex=' + hex);
