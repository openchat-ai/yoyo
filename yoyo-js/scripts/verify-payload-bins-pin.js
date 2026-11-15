'use strict';
/**
 * SUPERSEDED on stage path by Assert-YoyoPayloadBinsPin (yoyo hash).
 * AUTO-LESSHOST: kept as optional Node contrast only.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const dir = path.join(__dirname, '..', 'src', 'platform', 'yoyo-compiler-payload');
const pinPath = path.join(dir, 'payload.pin');
const codePath = path.join(dir, 'code.bin');
const dataPath = path.join(dir, 'data.bin');

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
  if (!fs.existsSync(pinPath) || !fs.existsSync(codePath) || !fs.existsSync(dataPath)) {
    console.log('HB_FX_BINS status=RED reason=missing_pin_or_bins');
    process.exit(1);
  }
  const pin = readPin(pinPath);
  const codeSha = sha256file(codePath);
  const dataSha = sha256file(dataPath);
  const codeBytes = fs.statSync(codePath).size;
  const dataBytes = fs.statSync(dataPath).size;

  console.log(`HB_FX_BINS code_bytes=${codeBytes} sha256_prefix=${codeSha.slice(0, 16)}`);
  console.log(`HB_FX_BINS data_bytes=${dataBytes} sha256_prefix=${dataSha.slice(0, 16)}`);
  console.log('HB_FX_BINS node_rebuild_on_gate=ABSENT');
  console.log('HB_FX_BINS policy=pin_locked');
  console.log(`HB_FX_BINS bins_sot=${pin.bins_sot || 'ABSENT'}`);

  if (String(pin.code_bytes) !== String(codeBytes) || String(pin.data_bytes) !== String(dataBytes)) {
    console.log('HB_FX_BINS status=RED reason=size_pin_DIFF');
    process.exit(1);
  }
  if ((pin.code_sha256 || '').toLowerCase() !== codeSha) {
    console.log('HB_FX_BINS status=RED reason=code_sha_pin_DIFF');
    process.exit(1);
  }
  if ((pin.data_sha256 || '').toLowerCase() !== dataSha) {
    console.log('HB_FX_BINS status=RED reason=data_sha_pin_DIFF');
    process.exit(1);
  }
  if (pin.bins_gate_policy !== 'pin_locked') {
    console.log('HB_FX_BINS status=RED reason=bins_gate_policy_not_pin_locked');
    process.exit(1);
  }
  // TRUER4: content SoT is pin-locked bins; executable SoT is place-stub.
  const sotOk = pin.bins_sot === 'fixture_pe' || pin.bins_sot === 'pin_locked_content';
  if (!sotOk) {
    console.log('HB_FX_BINS status=RED reason=bins_sot_invalid');
    process.exit(1);
  }

  if (pin.sidecar_face_in_data === 'ABSENT' || pin.sidecar_name_face === 'ABSENT') {
    const data = fs.readFileSync(dataPath);
    if (data.indexOf(Buffer.from('yoyo_rt.dll')) >= 0 || data.indexOf(Buffer.from('yoyo_no_rt.')) >= 0) {
      console.log('HB_FX_BINS status=RED reason=sidecar_name_face_still_in_data');
      process.exit(1);
    }
    console.log('HB_FX_BINS sidecar_face_in_data=ABSENT');
    console.log('HB_FX_BINS sidecar_name_face=ABSENT');
  }

  console.log('HB_FX_BINS status=GREEN pin_EQUAL');
  console.log('HB_FX_BINS doc=AUTO-TRUER6.md');
  process.exit(0);
}

main();
