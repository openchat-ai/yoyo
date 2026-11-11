'use strict';
/**
 * AUTO-PLACEPIN: fail-closed if placed C1 drifts from pin.
 *   node yoyo-js/scripts/verify-place-c1-pin.js <c1.exe>
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const pinPath = path.join(root, 'scripts', 'fixtures', 'yoyo_place_c1.pin');
const c1 = process.argv[2];

function readPin(p) {
  const o = Object.create(null);
  for (const line of fs.readFileSync(p, 'utf8').split(/\r?\n/)) {
    if (!line || line.startsWith('#')) continue;
    const i = line.indexOf('=');
    if (i < 0) continue;
    o[line.slice(0, i).trim()] = line.slice(i + 1).trim();
  }
  return o;
}

if (!c1 || !fs.existsSync(c1) || !fs.existsSync(pinPath)) {
  console.log('PLACE_C1 status=RED reason=missing_c1_or_pin');
  process.exit(1);
}
const pin = readPin(pinPath);
const buf = fs.readFileSync(c1);
const sha = crypto.createHash('sha256').update(buf).digest('hex');
console.log(`PLACE_C1 bytes=${buf.length} sha256_prefix=${sha.slice(0, 16)}`);
if (String(pin.bytes) !== String(buf.length) || (pin.sha256 || '').toLowerCase() !== sha) {
  console.log('PLACE_C1 status=RED reason=pin_DIFF');
  process.exit(1);
}
if (pin.trust_root !== 'place_stub') {
  console.log('PLACE_C1 status=RED reason=trust_root');
  process.exit(1);
}
console.log('PLACE_C1 status=GREEN pin_EQUAL');
console.log('PLACE_C1 doc=AUTO-PLACEPIN.md');
process.exit(0);
