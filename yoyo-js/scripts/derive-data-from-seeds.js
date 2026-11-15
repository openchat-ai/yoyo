'use strict';
/**
 * AUTO-TRUER19: derive data.bin from seeds + YOYO compact assemble.
 * Usage: node derive-data-from-seeds.js <out-data.bin>
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { buildCompilerDataPrelude } = require('../src/platform/yoyo-compiler-emit-prelude');

const out = process.argv[2];
if (!out) {
  console.log('DERIVE_DATA status=RED reason=missing_out_path');
  process.exit(1);
}

const { data } = buildCompilerDataPrelude({});
fs.mkdirSync(path.dirname(path.resolve(out)), { recursive: true });
fs.writeFileSync(out, data);
const sha = crypto.createHash('sha256').update(data).digest('hex');
console.log(`DERIVE_DATA data_bytes=${data.length} sha256_prefix=${sha.slice(0, 16)}`);
console.log('DERIVE_DATA data_authority=assembled_seeds_plus_yoyo_compact');
console.log('DERIVE_DATA status=GREEN');
process.exit(0);
