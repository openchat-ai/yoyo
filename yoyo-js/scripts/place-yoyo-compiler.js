#!/usr/bin/env node
'use strict';
/**
 * Peer place entry for yoyo_compiler.exe (AUTO-HB-FX).
 *
 * Writes <outdir>/yoyo_compiler.exe via JS `linkPeWin32`.
 * Default: live JS emit of code+data (FX-2b-4 source-interp + data bake).
 * Fallback: peer-local code.bin + data.bin if --from-bins.
 *
 * Hard rules:
 *   - never copy scripts/fixtures/yoyo_compiler.exe
 *   - never shell cargo --place-compiler
 *
 * Usage:
 *   node yoyo-js/scripts/place-yoyo-compiler.js <outdir>
 *   node yoyo-js/scripts/place-yoyo-compiler.js <outdir> --from-bins
 */
const fs = require('fs');
const path = require('path');
const { linkPeWin32 } = require('../src/platform/pe-builder');
const { buildCompilerCode } = require('../src/platform/yoyo-compiler-emit-code');

const PAYLOAD_DIR = path.join(__dirname, '..', 'src', 'platform', 'yoyo-compiler-payload');
const DEFAULT_SEED = path.join(
  __dirname,
  '..',
  '..',
  'scripts',
  '_stage-fullbar-fs2-chain',
  'seed_js.exe',
);

function main() {
  const outDir = process.argv[2];
  const fromBins = process.argv.includes('--from-bins');
  if (!outDir) {
    console.error('usage: node place-yoyo-compiler.js <outdir> [--from-bins]');
    process.exit(2);
  }

  let code;
  let data;
  let origin;

  if (fromBins) {
    const codePath = path.join(PAYLOAD_DIR, 'code.bin');
    const dataPath = path.join(PAYLOAD_DIR, 'data.bin');
    if (!fs.existsSync(codePath) || !fs.existsSync(dataPath)) {
      console.log('HB_FX_PLACE emitter=js_peer');
      console.log('HB_FX_PLACE status=ABSENT');
      console.log('HB_FX_PLACE reason=missing_payload_bins');
      console.log('HB_FX_PLACE doc=AUTO-HB-FX.md');
      process.exit(2);
    }
    code = fs.readFileSync(codePath);
    data = fs.readFileSync(dataPath);
    origin = 'bins_checked_in';
  } else {
    const seedPath = process.env.HB_FX_SEED || DEFAULT_SEED;
    if (!fs.existsSync(seedPath)) {
      console.log('HB_FX_PLACE status=ABSENT');
      console.log(`HB_FX_PLACE reason=missing_seed path=${seedPath}`);
      process.exit(2);
    }
    const seedPe = fs.readFileSync(seedPath);
    ({ code, data } = buildCompilerCode({ seedPe, userCodeLen: 17810 }));
    origin = 'js_parse1_source_interp';
  }

  const dataNeed = 0x38000;
  const pe = linkPeWin32(code, data, [], dataNeed);

  fs.mkdirSync(outDir, { recursive: true });
  const outPath = path.join(outDir, 'yoyo_compiler.exe');
  fs.writeFileSync(outPath, pe);

  console.log('HB_FX_PLACE emitter=js_peer_link');
  console.log(`HB_FX_PLACE payload_origin=${origin}`);
  console.log(`HB_FX_PLACE code_bytes=${code.length} data_bytes=${data.length} pe_bytes=${pe.length}`);
  console.log(`HB_FX_PLACE out=${outPath}`);
  console.log('HB_FX_PLACE status=GREEN');
  console.log('HB_FX_PLACE doc=AUTO-HB-FX.md');
  process.exit(0);
}

main();
