'use strict';
/**
 * AUTO-AUDITLIVE: if allow-list live files exist at known paths, hash ≡ fixture pins.
 * Missing path = skip (not RED). Present + wrong hash = RED.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');

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

const checks = [
  {
    name: 'payload_code.bin',
    file: path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'code.bin'),
    pin: path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'payload.pin'),
    key: 'code_sha256',
  },
  {
    name: 'payload_data.bin',
    file: path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'data.bin'),
    pin: path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'payload.pin'),
    key: 'data_sha256',
  },
  {
    name: 'fixture_yoyo_compiler.exe',
    file: path.join(root, 'scripts', 'fixtures', 'yoyo_compiler.exe'),
    pin: path.join(root, 'scripts', 'fixtures', 'yoyo_compiler.pin'),
    key: 'sha256',
  },
];

function main() {
  let checked = 0;
  for (const c of checks) {
    if (!fs.existsSync(c.file)) {
      console.log(`AUDITLIVE live=${c.name} status=SKIP reason=absent`);
      continue;
    }
    if (!fs.existsSync(c.pin)) {
      console.log(`AUDITLIVE status=RED reason=missing_pin_for_${c.name}`);
      process.exit(1);
    }
    const pin = readPin(c.pin);
    const want = (pin[c.key] || '').toLowerCase();
    const got = sha256file(c.file).toLowerCase();
    checked += 1;
    console.log(`AUDITLIVE live=${c.name} sha256_prefix=${got.slice(0, 16)}`);
    if (!want || want !== got) {
      console.log(`AUDITLIVE status=RED reason=live_sha_DIFF name=${c.name}`);
      console.log(`AUDITLIVE want=${want}`);
      console.log(`AUDITLIVE got=${got}`);
      process.exit(1);
    }
    // optional size
    if (pin.bytes && String(pin.bytes) !== String(fs.statSync(c.file).size)) {
      console.log(`AUDITLIVE status=RED reason=live_bytes_DIFF name=${c.name}`);
      process.exit(1);
    }
    if (c.key === 'code_sha256' && pin.code_bytes && String(pin.code_bytes) !== String(fs.statSync(c.file).size)) {
      console.log(`AUDITLIVE status=RED reason=code_bytes_DIFF`);
      process.exit(1);
    }
    if (c.key === 'data_sha256' && pin.data_bytes && String(pin.data_bytes) !== String(fs.statSync(c.file).size)) {
      console.log(`AUDITLIVE status=RED reason=data_bytes_DIFF`);
      process.exit(1);
    }
    console.log(`AUDITLIVE live=${c.name} status=EQUAL`);
  }
  if (checked < 1) {
    console.log('AUDITLIVE status=RED reason=no_live_files_checked');
    process.exit(1);
  }
  console.log(`AUDITLIVE live_checked=${checked}`);
  console.log('AUDITLIVE live=GREEN');
  console.log('AUDITLIVE status=GREEN');
  console.log('AUDITLIVE doc=AUTO-AUDITLIVE.md');
  process.exit(0);
}

main();
