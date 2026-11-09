'use strict';
/**
 * AUTO-TRUER5: fail-closed if fallthrough fixture PE drifts from pin.
 * Critical cold-start SoT = place-stub; this PE is FALLTHROUGH_ONLY.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const fx = path.join(root, 'scripts', 'fixtures', 'yoyo_compiler.exe');
const pinPath = path.join(root, 'scripts', 'fixtures', 'yoyo_compiler.pin');

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
    console.log('C1_FIXTURE status=RED reason=missing_fixture_or_pin');
    process.exit(1);
  }
  const pin = readPin(pinPath);
  const sha = sha256file(fx);
  const bytes = fs.statSync(fx).size;
  console.log(`C1_FIXTURE bytes=${bytes} sha256_prefix=${sha.slice(0, 16)}`);
  console.log(`C1_FIXTURE trust_root=${pin.trust_root || 'ABSENT'}`);
  console.log('C1_FIXTURE node_plant=ABSENT');
  console.log(`C1_FIXTURE disposition=${pin.disposition || 'ABSENT'}`);

  if (String(pin.bytes) !== String(bytes)) {
    console.log('C1_FIXTURE status=RED reason=size_pin_DIFF');
    process.exit(1);
  }
  if ((pin.sha256 || '').toLowerCase() !== sha) {
    console.log('C1_FIXTURE status=RED reason=sha_pin_DIFF');
    process.exit(1);
  }
  if (pin.disposition !== 'FALLTHROUGH_ONLY' && pin.disposition !== 'BOOTSTRAP_FIXTURE') {
    console.log('C1_FIXTURE status=RED reason=disposition');
    process.exit(1);
  }

  console.log('C1_FIXTURE status=GREEN pin_EQUAL');
  console.log('C1_FIXTURE doc=AUTO-TRUER5.md');
  process.exit(0);
}

main();
