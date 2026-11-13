'use strict';
/**
 * SUPERSEDED on stage path by scripts/lib/Assert-YoyoFilePin.ps1 (yoyo hash).
 * AUTO-LESSHOST: kept as optional Node contrast only.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const fx = path.join(root, 'scripts', 'fixtures', 'yoyo_place_stub.exe');
const pinPath = path.join(root, 'scripts', 'fixtures', 'yoyo_place_stub.pin');

function sha256file(p) {
  return crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
}
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

function main() {
  if (!fs.existsSync(fx) || !fs.existsSync(pinPath)) {
    console.log('PLACE_STUB status=RED reason=missing_fixture_or_pin');
    process.exit(1);
  }
  const pin = readPin(pinPath);
  const sha = sha256file(fx);
  const bytes = fs.statSync(fx).size;
  console.log(`PLACE_STUB bytes=${bytes} sha256_prefix=${sha.slice(0, 16)}`);
  console.log('PLACE_STUB trust_root=place_stub');

  if (String(pin.bytes) !== String(bytes) || (pin.sha256 || '').toLowerCase() !== sha) {
    console.log('PLACE_STUB status=RED reason=pin_DIFF');
    process.exit(1);
  }
  if (pin.trust_root !== 'place_stub') {
    console.log('PLACE_STUB status=RED reason=trust_root');
    process.exit(1);
  }
  console.log('PLACE_STUB status=GREEN pin_EQUAL');
  console.log('PLACE_STUB doc=AUTO-TRUER4.md');
  process.exit(0);
}

main();
