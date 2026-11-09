#!/usr/bin/env node
'use strict';
/**
 * Offline bins→PE link helper (NOT operational SoT).
 * Operational: scripts/place-yoyo-compiler-c1.ps1
 *
 * Only --from-bins. Live JS ISA plant = RETIRED (AUTO-NO-PLANT-ESCAPE).
 * No YOYO_ALLOW_NODE_PLANT escape.
 *
 *   node yoyo-js/scripts/place-yoyo-compiler.js <outdir> --from-bins
 */
const fs = require('fs');
const path = require('path');
const { linkPeWin32 } = require('../src/platform/pe-builder');

const PAYLOAD_DIR = path.join(__dirname, '..', 'src', 'platform', 'yoyo-compiler-payload');

function main() {
  const outDir = process.argv[2];
  const fromBins = process.argv.includes('--from-bins');
  if (!outDir) {
    console.error('usage: node place-yoyo-compiler.js <outdir> --from-bins');
    process.exit(2);
  }
  if (!fromBins) {
    console.log('HB_FX_PLACE status=ABSENT');
    console.log('HB_FX_PLACE reason=node_plant_refused');
    console.log('HB_FX_PLACE hint=use --from-bins or place-yoyo-compiler-c1.ps1');
    console.log('HB_FX_PLACE escape=ABSENT');
    console.log('HB_FX_PLACE doc=AUTO-NO-PLANT-ESCAPE.md');
    process.exit(2);
  }

  const codePath = path.join(PAYLOAD_DIR, 'code.bin');
  const dataPath = path.join(PAYLOAD_DIR, 'data.bin');
  if (!fs.existsSync(codePath) || !fs.existsSync(dataPath)) {
    console.log('HB_FX_PLACE emitter=js_peer');
    console.log('HB_FX_PLACE status=ABSENT');
    console.log('HB_FX_PLACE reason=missing_payload_bins');
    console.log('HB_FX_PLACE doc=AUTO-NO-PLANT-ESCAPE.md');
    process.exit(2);
  }
  const code = fs.readFileSync(codePath);
  const data = fs.readFileSync(dataPath);
  const pe = linkPeWin32(code, data, [], 0x38000);

  fs.mkdirSync(outDir, { recursive: true });
  const outPath = path.join(outDir, 'yoyo_compiler.exe');
  fs.writeFileSync(outPath, pe);

  console.log('HB_FX_PLACE emitter=js_peer_link_from_bins');
  console.log('HB_FX_PLACE payload_origin=bins_checked_in');
  console.log('HB_FX_PLACE node_plant=ABSENT');
  console.log('HB_FX_PLACE escape=ABSENT');
  console.log(
    `HB_FX_PLACE code_bytes=${code.length} data_bytes=${data.length} pe_bytes=${pe.length}`,
  );
  console.log(`HB_FX_PLACE out=${outPath}`);
  console.log('HB_FX_PLACE status=GREEN');
  console.log('HB_FX_PLACE doc=AUTO-NO-PLANT-ESCAPE.md');
  process.exit(0);
}

main();
