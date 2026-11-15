'use strict';
/**
 * AUTO-TRUER3: fail-closed if fixtures/yoyo_rt.dll drifts from yoyo_rt.pin.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const fx = path.join(root, 'scripts', 'fixtures', 'yoyo_rt.dll');
const pinPath = path.join(root, 'scripts', 'fixtures', 'yoyo_rt.pin');

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
    console.log('YOYO_RT_FIXTURE status=RED reason=missing_fixture_or_pin');
    process.exit(1);
  }
  const pin = readPin(pinPath);
  const sha = sha256file(fx);
  const bytes = fs.statSync(fx).size;
  console.log(`YOYO_RT_FIXTURE bytes=${bytes} sha256_prefix=${sha.slice(0, 16)}`);
  console.log('YOYO_RT_FIXTURE cargo_emit_rt=ABSENT');

  if (String(pin.bytes) !== String(bytes)) {
    console.log('YOYO_RT_FIXTURE status=RED reason=size_pin_DIFF');
    process.exit(1);
  }
  if ((pin.sha256 || '').toLowerCase() !== sha) {
    console.log('YOYO_RT_FIXTURE status=RED reason=sha_pin_DIFF');
    process.exit(1);
  }
  // TRUER5: demoted from BOOTSTRAP_FIXTURE → CONTRAST_ONLY (not critical SoT).
  if (pin.disposition !== 'CONTRAST_ONLY' && pin.disposition !== 'BOOTSTRAP_FIXTURE') {
    console.log('YOYO_RT_FIXTURE status=RED reason=disposition');
    process.exit(1);
  }
  if (pin.cargo_emit_rt !== 'ABSENT') {
    console.log('YOYO_RT_FIXTURE status=RED reason=cargo_emit_rt_not_ABSENT');
    process.exit(1);
  }

  console.log('YOYO_RT_FIXTURE status=GREEN pin_EQUAL');
  console.log('YOYO_RT_FIXTURE doc=AUTO-TRUER3.md');
  process.exit(0);
}

main();
