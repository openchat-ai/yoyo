#!/usr/bin/env node
'use strict';
/**
 * HB-FX FX-2b: verify JS data emit ≡ Rust dump prefix / full.
 *
 * Usage:
 *   node verify-compiler-data-prelude.js
 *   node verify-compiler-data-prelude.js --with-h00 [seed.exe]
 *   node verify-compiler-data-prelude.js --with-field-match [seed.exe]
 */
const fs = require('fs');
const path = require('path');
const { buildCompilerDataPrelude } = require('../src/platform/yoyo-compiler-emit-prelude');

const dumpPath = path.join(
  __dirname,
  '..',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'data.bin',
);

function main() {
  if (!fs.existsSync(dumpPath)) {
    console.log('HB_FX_PRELUDE status=RED reason=missing_data_bin');
    process.exit(1);
  }
  const dump = fs.readFileSync(dumpPath);
  const args = process.argv.slice(2);
  const withFieldMatch = args.includes('--with-field-match');
  const withH00 = withFieldMatch || args.includes('--with-h00');
  let seedPath = path.join(
    __dirname,
    '..',
    '..',
    'scripts',
    '_stage-fullbar-fs2-chain',
    'seed_js.exe',
  );
  const flag = withFieldMatch ? '--with-field-match' : '--with-h00';
  const idx = args.indexOf(flag);
  if (idx >= 0 && args[idx + 1] && !args[idx + 1].startsWith('-')) {
    seedPath = args[idx + 1];
  }
  const opts = {};
  if (withH00) {
    if (!fs.existsSync(seedPath)) {
      console.log(`HB_FX_PRELUDE status=RED reason=missing_seed path=${seedPath}`);
      process.exit(1);
    }
    opts.seedPe = fs.readFileSync(seedPath);
    opts.userCodeLen = 17810;
  }
  if (withFieldMatch) opts.fieldMatch = true;

  const { data, compactLen, retPeLen, h00Len, fieldMatchLen } = buildCompilerDataPrelude(opts);
  const n = data.length;
  let mismatch = -1;
  for (let i = 0; i < n; i++) {
    if (i >= dump.length || data[i] !== dump[i]) {
      mismatch = i;
      break;
    }
  }
  console.log(
    `HB_FX_PRELUDE prelude_bytes=${n} dump_bytes=${dump.length} with_h00=${withH00} field_match=${withFieldMatch}`,
  );
  console.log(
    `HB_FX_PRELUDE ret_pe_len=${retPeLen} compact_len=${compactLen} h00_len=${h00Len || 0} fm_len=${fieldMatchLen || 0}`,
  );
  if (mismatch >= 0) {
    console.log(`HB_FX_PRELUDE status=RED mismatch_at=${mismatch}`);
    console.log(
      `HB_FX_PRELUDE js=${data.slice(Math.max(0, mismatch - 4), mismatch + 8).toString('hex')}`,
    );
    console.log(
      `HB_FX_PRELUDE ru=${dump.slice(Math.max(0, mismatch - 4), mismatch + 8).toString('hex')}`,
    );
    process.exit(1);
  }
  const full = withFieldMatch && n === dump.length;
  console.log(
    full
      ? 'HB_FX_PRELUDE status=GREEN data_bin_FULL_EQUAL'
      : 'HB_FX_PRELUDE status=GREEN prefix_EQUAL',
  );
  console.log('HB_FX_PRELUDE doc=AUTO-HB-FX.md');
  process.exit(0);
}

main();
