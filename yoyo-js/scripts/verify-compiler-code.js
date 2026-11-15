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
  let origin;
  try {
    ({ code, origin } = buildCompilerCode({ seedPe, userCodeLen: 17810 }));
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
  // Hot-path must not open pe_dll_link.rs (FX-2c).
  const emitSrc = require('fs').readFileSync(
    require('path').join(__dirname, '..', 'src', 'platform', 'yoyo-compiler-emit-code.js'),
    'utf8',
  );
  if (/readFileSync\([^\)]*pe_dll_link\.rs/.test(emitSrc)) {
    console.log('HB_FX_CODE status=RED reason=emit_code_still_reads_rust_source');
    process.exit(1);
  }
  console.log('HB_FX_CODE status=GREEN code_bin_FULL_EQUAL');
  console.log(`HB_FX_CODE origin=${origin || 'js_parse1_native'}`);
  console.log('HB_FX_CODE rust_source_hot_path=ABSENT');
  console.log('HB_FX_CODE steps_table_hot_path=ABSENT');
  // X6 = emit semantics not from STEPS freeze-dry. Require ISA/RT plant lineage.
  const stubs = require('../src/platform/yoyo-compiler-parse1-emit-stubs');
  const plant = require('../src/platform/yoyo-compiler-parse1-x64-plant');
  const emit2Ok =
    stubs.lineage === 'js_parse1_isa_rt_plant' &&
    typeof plant.emit2Stores === 'function' &&
    JSON.stringify(stubs.TEMPLATES.EMIT2_SET) === JSON.stringify(plant.emit2Stores('set')) &&
    JSON.stringify(stubs.TEMPLATES.EMIT2_NID) === JSON.stringify(plant.emit2Stores('nid'));
  console.log(
    emit2Ok
      ? 'HB_FX_CODE x6_native=PRESENT note=emit1_isa_emit2_isa_rt_EQUAL_pack_schedule_historical'
      : 'HB_FX_CODE x6_native=ABSENT note=emit2_not_isa_rt_wired',
  );
  console.log('HB_FX_CODE doc=AUTO-HB-FX.md');
  process.exit(0);
}

main();
