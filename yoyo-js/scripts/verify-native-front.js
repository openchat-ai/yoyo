'use strict';
/**
 * FX-2d: native front (→ parse1) unpatched bytes ≡ emitAll prefix.
 */
const fs = require('fs');
const path = require('path');
const {
  buildCompilerCode,
  S3_WIN32_LOAD_FILE_BODY_SEED,
  S3_WIN32_WRITE_FILE_BODY_SEED,
} = require('../src/platform/yoyo-compiler-emit-code');
const { emitNativeFront } = require('../src/platform/yoyo-compiler-parse1-native-asm');

const seedPath =
  process.argv[2] ||
  path.join(__dirname, '..', '..', 'scripts', '_stage-fullbar-fs2-chain', 'seed_js.exe');

function main() {
  const seedPe = fs.readFileSync(seedPath);
  const dumpPath = path.join(__dirname, '..', 'src', 'platform', 'yoyo-compiler-payload', 'code.bin');
  const golden = fs.readFileSync(dumpPath);
  const { meta } = buildCompilerCode({ seedPe, userCodeLen: 17810 });
  const asm = emitNativeFront(meta, {
    load: S3_WIN32_LOAD_FILE_BODY_SEED,
    write: S3_WIN32_WRITE_FILE_BODY_SEED,
  });
  const n = asm.buf;
  const dump = Buffer.from(golden);
  const code = golden; // coverage denominator
  for (const name of Object.keys(asm.labels)) {
    const at = asm.labels[name];
    if (name.startsWith('jmp_')) {
      if (dump[at] === 0xe9) dump[at + 1] = dump[at + 2] = dump[at + 3] = dump[at + 4] = 0;
    } else if (dump[at] === 0x0f && (dump[at + 1] & 0xf0) === 0x80) {
      // any jcc rel32 site (je/jne/ja/jb/jl/jg/jz/…)
      dump[at + 2] = dump[at + 3] = dump[at + 4] = dump[at + 5] = 0;
    }
  }
  if (asm.extraPatches) {
    for (const [jne1, jne2, jmpE] of asm.extraPatches) {
      for (const at of [jne1, jne2]) {
        if (dump[at] === 0x0f) dump[at + 2] = dump[at + 3] = dump[at + 4] = dump[at + 5] = 0;
      }
      if (dump[jmpE] === 0xe9) dump[jmpE + 1] = dump[jmpE + 2] = dump[jmpE + 3] = dump[jmpE + 4] = 0;
    }
  }
  let mismatch = -1;
  for (let i = 0; i < n.length; i++) {
    if (n[i] !== dump[i]) {
      mismatch = i;
      break;
    }
  }
  console.log(`HB_FX_NATIVE front_bytes=${n.length}`);
  if (mismatch >= 0) {
    console.log(`HB_FX_NATIVE status=RED mismatch_at=${mismatch}`);
    console.log(
      `HB_FX_NATIVE nat=${Buffer.from(n.slice(Math.max(0, mismatch - 4), mismatch + 12)).toString('hex')}`,
    );
    console.log(
      `HB_FX_NATIVE dump=${dump.slice(Math.max(0, mismatch - 4), mismatch + 12).toString('hex')}`,
    );
    process.exit(1);
  }
  console.log('HB_FX_NATIVE status=GREEN front_unpatched_EQUAL');
  const pct = (n.length / code.length) * 100;
  console.log(`HB_FX_NATIVE coverage_pct=${pct.toFixed(1)}`);
  const stubs = require('../src/platform/yoyo-compiler-parse1-emit-stubs');
  const plant = require('../src/platform/yoyo-compiler-parse1-x64-plant');
  const emit2Ok =
    stubs.lineage === 'js_parse1_isa_rt_plant' &&
    typeof plant.emit2Stores === 'function' &&
    JSON.stringify(stubs.TEMPLATES.EMIT2_SET) === JSON.stringify(plant.emit2Stores('set')) &&
    JSON.stringify(stubs.TEMPLATES.EMIT2_NID) === JSON.stringify(plant.emit2Stores('nid'));
  const x6 =
    n.length === code.length && emit2Ok
      ? 'HB_FX_NATIVE x6_native=PRESENT note=emit1_isa_emit2_isa_rt_EQUAL_pack_schedule_historical'
      : n.length === code.length
        ? 'HB_FX_NATIVE x6_native=ABSENT note=emit2_not_isa_rt_wired'
        : 'HB_FX_NATIVE x6_native=ABSENT note=tail_after_front_still_lifted';
  console.log(x6);
  process.exit(0);
}

main();
