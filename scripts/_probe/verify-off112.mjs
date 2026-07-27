// Independent 23B pin verification using js-ty2text.mjs via child_process
// execFileSync pattern to avoid PowerShell UTF-16 mangling.
import { execFileSync } from 'child_process';
import { readFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..', '..');
const probe = join(root, 'scripts', '_probe', 'js-ty2text.mjs');
const fixture = join(root, 'yoyo', 'tests', 'golden', 'selfhost_min_ldb_off112_handler.ty');
const expectedHex = readFileSync(join(root, 'yoyo', 'tests', 'golden', 'expected', 'selfhost_min_ldb_off112_handler.code.hex'), 'utf8')
  .replace(/;[^\r\n]*/g, '').replace(/\s+/g, '').toLowerCase();

const got = execFileSync('node', [probe, fixture], { stdio: ['ignore', 'pipe', 'pipe'] });
const gotHex = Buffer.from(got).toString('hex');
const ok = gotHex === expectedHex;

console.log(`expected: ${expectedHex}`);
console.log(`got:      ${gotHex}`);
console.log(`len(got)=${Buffer.from(got).length}`);
console.log(`match:    ${ok ? 'YES' : 'NO'}`);
if (!ok) process.exit(1);