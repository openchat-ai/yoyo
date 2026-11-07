#!/usr/bin/env node
'use strict';
/**
 * HB-FX FX-2b-4: JS code.bin emit ≡ Rust dump.
 *
 *   node verify-compiler-code.js [seed.exe]
 */
const fs = require('fs');
const path = require('path');
const { buildCompilerCode } = require('../src/platform/yoyo-compiler-emit-code');

const dumpPath = path.join(
  __dirname,
  '..',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'code.bin',
);

function main() {
  const seedPath =
    process.argv[2] ||
    path.join(__dirname, '..', '..', 'scripts', '_stage-fullbar-fs2-chain', 'seed_js.exe');
  if (!fs.existsSync(dumpPath)) {
    console.log('HB_FX_CODE status=RED reason=missing_code_bin');
    process.exit(1);
  }
  if (!fs.existsSync(seedPath)) {
    console.log(`HB_FX_CODE status=RED reason=missing_seed path=${seedPath}`);
    process.exit(1);
  }
  const dump = fs.readFileSync(dumpPath);
  const seedPe = fs.readFileSync(seedPath);
  let code;
  try {
    ({ code } = buildCompilerCode({ seedPe, userCodeLen: 17810 }));
  } catch (e) {
    console.log(`HB_FX_CODE status=RED reason=emit_throw msg=${e.message}`);
    console.error(e.stack);
    process.exit(2);
  }
  console.log(`HB_FX_CODE js_bytes=${code.length} dump_bytes=${dump.length}`);
  let mismatch = -1;
  const n = Math.min(code.length, dump.length);
  for (let i = 0; i < n; i++) {
    if (code[i] !== dump[i]) {
      mismatch = i;
      break;
    }
  }
  if (mismatch < 0 && code.length !== dump.length) mismatch = n;
  if (mismatch >= 0) {
    console.log(`HB_FX_CODE status=RED mismatch_at=${mismatch}`);
    console.log(
      `HB_FX_CODE js=${code.slice(Math.max(0, mismatch - 4), mismatch + 12).toString('hex')}`,
    );
    console.log(
      `HB_FX_CODE ru=${dump.slice(Math.max(0, mismatch - 4), mismatch + 12).toString('hex')}`,
    );
    process.exit(1);
  }
  console.log('HB_FX_CODE status=GREEN code_bin_FULL_EQUAL');
  console.log('HB_FX_CODE origin=js_parse1_source_interp');
  console.log('HB_FX_CODE doc=AUTO-HB-FX.md');
  process.exit(0);
}

main();
