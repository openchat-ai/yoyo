'use strict';
/** AUTO-AUDITZERO: full code.bin[place:] hash — OOW landing is a subset. */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const pinPath = path.join(root, 'scripts', 'fixtures', 'audit-stub', 'STUB.BEYOND.pin');
const codePath = process.argv[2]
  ? path.resolve(process.argv[2])
  : path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'code.bin');

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
  const pin = readPin(pinPath);
  const code = fs.readFileSync(codePath);
  const place = parseInt(pin.code_place_bytes, 10);
  const wantLen = parseInt(pin.beyond_bytes, 10);
  const beyond = code.subarray(place);
  if (beyond.length !== wantLen) {
    console.log(`AUDITZERO status=RED reason=beyond_len want=${wantLen} got=${beyond.length}`);
    process.exit(1);
  }
  const sha = crypto.createHash('sha256').update(beyond).digest('hex');
  console.log(`AUDITZERO beyond_bytes=${beyond.length} sha256_prefix=${sha.slice(0, 16)}`);
  if ((pin.beyond_sha256 || '').toLowerCase() !== sha) {
    console.log('AUDITZERO status=RED reason=beyond_sha_DIFF');
    process.exit(1);
  }
  console.log('AUDITZERO beyond=EQUAL');
  console.log('AUDITZERO status=GREEN');
  process.exit(0);
}
main();
