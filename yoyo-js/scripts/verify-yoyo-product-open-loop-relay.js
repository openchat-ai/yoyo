'use strict';
/**
 * AUTO-OPENLOOP: product open_loop + compiler relay PRESENT.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');
const pinPath = path.join(
  root,
  'yoyo-js',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'payload.pin',
);
const h00Expect = path.join(projects, 'yoyo-openloop-h00.expect.hex');
const productPe =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-openloop', 'product', 'output.exe');
const relayOut =
  process.argv[3] ||
  path.join(root, 'scripts', '_stage-openloop', 'relay', 'output.exe');

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

function peText(pe) {
  const e = pe.readUInt32LE(0x3c);
  const sizeOpt = pe.readUInt16LE(e + 20);
  const sect = e + 24 + sizeOpt;
  const rp = pe.readUInt32LE(sect + 20);
  const rs = pe.readUInt32LE(sect + 16);
  return pe.subarray(rp, rp + rs);
}

function main() {
  const pin = readPin(pinPath);
  if (pin.yoyo_product_open_loop !== 'PRESENT') {
    console.log('OL status=RED reason=pin_open_loop');
    process.exit(1);
  }
  if (pin.yoyo_product_compiler_relay !== 'PRESENT') {
    console.log('OL status=RED reason=pin_relay');
    process.exit(1);
  }
  if (!fs.existsSync(productPe)) {
    console.log('OL status=RED reason=missing_product');
    process.exit(1);
  }
  if (!fs.existsSync(relayOut)) {
    console.log('OL status=RED reason=missing_relay_output');
    process.exit(1);
  }
  const pe = fs.readFileSync(productPe);
  const text = peText(pe);
  const exp = Buffer.from(
    fs.readFileSync(h00Expect, 'utf8').trim().replace(/\s+/g, ''),
    'hex',
  );
  const idx = text.indexOf(exp);
  if (idx < 0) {
    console.log('OL status=RED reason=product_miss_h00_openloop');
    process.exit(1);
  }
  const sha = crypto.createHash('sha256').update(pe).digest('hex');
  if (
    pin.yoyo_product_with_io_paths_sha256 &&
    pin.yoyo_product_with_io_paths_sha256 !== sha
  ) {
    console.log('OL status=RED reason=product_sha_drift got=' + sha.slice(0, 16));
    process.exit(1);
  }
  const out = fs.readFileSync(relayOut);
  if (out.length < 2 || out[0] !== 0x4d || out[1] !== 0x5a) {
    console.log('OL status=RED reason=relay_not_mz');
    process.exit(1);
  }
  console.log('OL yoyo_product_open_loop=PRESENT');
  console.log('OL yoyo_product_compiler_relay=PRESENT');
  console.log('OL h00_text_off=' + idx);
  console.log('OL product_sha256_prefix=' + sha.slice(0, 16));
  console.log('OL relay_bytes=' + out.length);
  console.log('OL status=GREEN');
}

main();
