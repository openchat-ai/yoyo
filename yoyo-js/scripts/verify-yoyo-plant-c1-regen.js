'use strict';
/** AUTO-TRUER14: C1×plant halves PE text (strip FF→C3) concat EQUAL code.bin */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');
const code = fs.readFileSync(
  path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'code.bin'),
);
const PARSE1 = 4491;
const placePe = path.join(root, 'scripts', '_stage-truer14', 'regen-place', 'output.exe');
const parse1Pe = path.join(root, 'scripts', '_stage-truer14', 'regen-parse1', 'output.exe');

function extract(pePath, want) {
  const pe = fs.readFileSync(pePath);
  const withC3 = Buffer.concat([want, Buffer.from([0xc3])]);
  let i = pe.indexOf(withC3);
  if (i >= 0) return pe.subarray(i, i + want.length);
  i = pe.indexOf(want);
  if (i < 0) throw new Error(`blob missing in ${pePath}`);
  return pe.subarray(i, i + want.length);
}

const a = extract(placePe, code.subarray(0, PARSE1));
const b = extract(parse1Pe, code.subarray(PARSE1));
const rebuilt = Buffer.concat([a, b]);
console.log(`Y14_REGEN place=${a.length} parse1=${b.length} total=${rebuilt.length}`);
if (!rebuilt.equals(code)) {
  console.log('Y14_REGEN status=RED reason=c1_halves_ne_code');
  process.exit(1);
}
console.log('Y14_REGEN c1_halves_EQUAL=1');
console.log('Y14_REGEN status=GREEN');
console.log('Y14_REGEN doc=AUTO-TRUER14.md');
process.exit(0);
