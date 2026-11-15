'use strict';
/**
 * AUTO-TRUER9: H00 must be gone from critical bins — no yoyo_h00_ markers,
 * no manual-map stub template prefix in stub/code/data.
 */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');
const files = [
  path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'data.bin'),
  path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'code.bin'),
  path.join(root, 'scripts', 'fixtures', 'yoyo_place_stub.exe'),
];
const hexPath = path.join(root, 'yoyo-js', 'src', 'platform', 'h00-manual-map-stub.hex');

function main() {
  const hex = fs.readFileSync(hexPath, 'utf8').replace(/[^0-9a-fA-F]/g, '');
  const tmpl = Buffer.from(hex, 'hex').subarray(0, 32);
  const needle = Buffer.from('yoyo_h00_');
  let bad = 0;
  for (const f of files) {
    const b = fs.readFileSync(f);
    const rel = path.relative(root, f).replace(/\\/g, '/');
    const m = b.indexOf(needle);
    const t = b.indexOf(tmpl);
    console.log(`H00_CLEAN path=${rel} yoyo_h00_=${m} tmpl32=${t}`);
    if (m >= 0 || t >= 0) bad++;
  }
  const dataLen = fs.statSync(files[0]).size;
  console.log(`H00_CLEAN data_bytes=${dataLen}`);
  // TRUER9 excised H00; TRUER10 further cut FM — anything >= prior H00-era size is regress.
  if (dataLen >= 180864) {
    console.log('H00_CLEAN status=RED reason=data_not_shrunk_after_excise');
    process.exit(1);
  }
  if (bad !== 0) {
    console.log('H00_CLEAN status=RED');
    process.exit(1);
  }
  console.log('H00_CLEAN h00_markers=ABSENT');
  console.log('H00_CLEAN h00_bake=EXCISED');
  console.log('H00_CLEAN h00_manual_map_stub=ABSENT');
  console.log('H00_CLEAN status=GREEN');
  console.log('H00_CLEAN doc=AUTO-TRUER9.md');
  process.exit(0);
}

main();
