'use strict';
/** AUTO-AUDITFIVE: sha256+sha512 for code/data/stub/compiler/beyond. */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const pinPath = path.join(root, 'scripts', 'fixtures', 'audit-stub', 'STUB.DUALHASH.pin');

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
function digests(buf) {
  return {
    sha256: crypto.createHash('sha256').update(buf).digest('hex'),
    sha512: crypto.createHash('sha512').update(buf).digest('hex'),
  };
}
function check(name, buf, want256, want512) {
  const g = digests(buf);
  console.log(`AUDITFIVE dual=${name} sha256_prefix=${g.sha256.slice(0, 16)} sha512_prefix=${g.sha512.slice(0, 16)}`);
  if (g.sha256 !== (want256 || '').toLowerCase() || g.sha512 !== (want512 || '').toLowerCase()) {
    console.log(`AUDITFIVE status=RED reason=dualhash_DIFF name=${name}`);
    process.exit(1);
  }
}

function main() {
  const pin = readPin(pinPath);
  const code = fs.readFileSync(path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'code.bin'));
  const data = fs.readFileSync(path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'data.bin'));
  const stub = fs.readFileSync(path.join(root, 'scripts', 'fixtures', 'yoyo_place_stub.exe'));
  const comp = fs.readFileSync(path.join(root, 'scripts', 'fixtures', 'yoyo_compiler.exe'));
  const place = parseInt(pin.code_place_bytes, 10);
  check('code', code, pin.code_sha256, pin.code_sha512);
  check('data', data, pin.data_sha256, pin.data_sha512);
  check('stub', stub, pin.stub_sha256, pin.stub_sha512);
  check('compiler', comp, pin.compiler_sha256, pin.compiler_sha512);
  check('beyond', code.subarray(place), pin.beyond_sha256, pin.beyond_sha512);
  console.log('AUDITFIVE dualhash=EQUAL');
  console.log('AUDITFIVE status=GREEN');
  process.exit(0);
}
main();
