'use strict';
/**
 * AUTO-TRUER6 Z1: critical trust blobs must not contain sidecar dll *name* faces.
 * Checks both historical `yoyo_rt.dll` and TRUER5 placeholder `yoyo_no_rt.`.
 * ≠ Thompson — face absent ≠ H00 manual-map code gone.
 */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');
const needles = [Buffer.from('yoyo_rt.dll'), Buffer.from('yoyo_no_rt.')];
const files = [
  path.join(root, 'scripts', 'fixtures', 'yoyo_place_stub.exe'),
  path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'code.bin'),
  path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'data.bin'),
];

function main() {
  let bad = 0;
  for (const f of files) {
    if (!fs.existsSync(f)) {
      console.log(`NO_SIDECAR_FACE status=RED reason=missing path=${path.relative(root, f)}`);
      process.exit(1);
    }
    const b = fs.readFileSync(f);
    const rel = path.relative(root, f).replace(/\\/g, '/');
    for (const n of needles) {
      const idx = b.indexOf(n);
      console.log(`NO_SIDECAR_FACE path=${rel} face=${n.toString()} off=${idx}`);
      if (idx >= 0) bad++;
    }
  }
  if (bad !== 0) {
    console.log('NO_SIDECAR_FACE status=RED reason=dll_name_face_present');
    process.exit(1);
  }
  console.log('NO_SIDECAR_FACE sidecar_name_face=ABSENT');
  console.log('NO_SIDECAR_FACE status=GREEN');
  console.log('NO_SIDECAR_FACE doc=AUTO-TRUER6.md');
  process.exit(0);
}

main();
