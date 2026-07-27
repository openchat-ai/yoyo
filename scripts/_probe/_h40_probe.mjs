// One-shot probe: run js-ty2text.mjs and print hex + length
import { execFileSync } from 'node:child_process';
const out = execFileSync('node', ['scripts/_probe/js-ty2text.mjs', 'yoyo/tests/golden/_scratch_h40.ty']);
const arr = Array.from(out);
const hex = arr.map(b => b.toString(16).padStart(2, '0')).join('');
console.log('hex=' + hex);
console.log('len=' + arr.length);
