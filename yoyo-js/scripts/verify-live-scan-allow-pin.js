'use strict';
/**
 * AUTO-AUDITFIVE F3: scan known roots for allow-list basenames;
 * if present, hash must match fixture pins (catches sidecars outside old fixed paths).
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

const payloadPin = readPin(path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'payload.pin'));
const stubPin = readPin(path.join(root, 'scripts', 'fixtures', 'yoyo_place_stub.pin'));
const compPin = readPin(path.join(root, 'scripts', 'fixtures', 'yoyo_compiler.pin'));

const EXPECT = {
  'code.bin': (payloadPin.code_sha256 || '').toLowerCase(),
  'data.bin': (payloadPin.data_sha256 || '').toLowerCase(),
  'yoyo_place_stub.exe': (stubPin.sha256 || '').toLowerCase(),
  'yoyo_compiler.exe': (compPin.sha256 || '').toLowerCase(),
};

const SCAN_ROOTS = [
  path.join(root, 'scripts', 'fixtures'),
  path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload'),
];

function walk(dir, out) {
  if (!fs.existsSync(dir)) return;
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, ent.name);
    if (ent.isDirectory()) {
      if (ent.name === 'node_modules' || ent.name === '.git' || ent.name.startsWith('_tmp') || ent.name.startsWith('_stage')) {
        continue;
      }
      walk(p, out);
    } else if (EXPECT[ent.name]) {
      out.push(p);
    }
  }
}

function main() {
  const found = [];
  for (const r of SCAN_ROOTS) walk(r, found);
  let n = 0;
  for (const f of found) {
    const base = path.basename(f);
    const want = EXPECT[base];
    const got = sha256file(f).toLowerCase();
    n += 1;
    console.log(`AUDITFIVE scan=${f.replace(/\\/g, '/')} sha256_prefix=${got.slice(0, 16)}`);
    if (got !== want) {
      console.log(`AUDITFIVE status=RED reason=scan_sha_DIFF file=${base}`);
      console.log(`AUDITFIVE want=${want}`);
      console.log(`AUDITFIVE got=${got}`);
      process.exit(1);
    }
  }
  if (n < 4) {
    // expect at least payload code+data + stub + compiler under fixtures/payload
    console.log(`AUDITFIVE status=RED reason=scan_too_few n=${n}`);
    process.exit(1);
  }
  console.log(`AUDITFIVE scan_checked=${n}`);
  console.log('AUDITFIVE scan=GREEN');
  console.log('AUDITFIVE status=GREEN');
  process.exit(0);
}
main();
