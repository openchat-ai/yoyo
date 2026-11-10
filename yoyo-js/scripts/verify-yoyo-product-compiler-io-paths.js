'use strict';
/**
 * AUTO-TRUER25: full C1×yoyo.tyb product hosts input.tyb + output.exe path faces.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const pinPath = path.join(
  root,
  'yoyo-js',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'payload.pin',
);
const tybPath = path.join(root, 'yoyo', 'projects', 'yoyo.tyb');
const productPath =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer25', 'product', 'output.exe');
const NEEDLE = Buffer.from('input.tyb\0output.exe\0', 'ascii');

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
  if (pin.yoyo_product_compiler_io_paths !== 'PRESENT') {
    console.log('Y25 status=RED reason=pin_product_io_paths');
    process.exit(1);
  }
  const tyb = fs.readFileSync(tybPath);
  const n = tyb.readUInt16LE(6);
  if (n < 2468) {
    console.log('Y25 status=RED reason=tyb_n_too_small n=' + n);
    process.exit(1);
  }
  // H_788 header at end: 40 01 14 03 …
  const hdrOff = 8 + (2468 - 9) * 8;
  const hdr = tyb.subarray(hdrOff, hdrOff + 8);
  if (hdr[0] !== 0x40 || hdr[2] !== 0x14 || hdr[3] !== 0x03) {
    console.log(
      'Y25 status=RED reason=missing_h788_hdr got=' + hdr.toString('hex'),
    );
    process.exit(1);
  }
  if (!fs.existsSync(productPath)) {
    console.log('Y25 status=RED reason=missing_product');
    process.exit(1);
  }
  const pe = fs.readFileSync(productPath);
  if (pe[0] !== 0x4d || pe[1] !== 0x5a) {
    console.log('Y25 status=RED reason=product_not_mz');
    process.exit(1);
  }
  const text = peText(pe);
  const idx = text.indexOf(NEEDLE);
  if (idx < 0) {
    console.log('Y25 status=RED reason=product_text_miss_io_paths');
    process.exit(1);
  }
  const sha = crypto.createHash('sha256').update(pe).digest('hex');
  if (
    pin.yoyo_product_with_io_paths_sha256 &&
    pin.yoyo_product_with_io_paths_sha256 !== sha
  ) {
    console.log(
      'Y25 status=RED reason=product_sha_drift got=' + sha.slice(0, 16),
    );
    process.exit(1);
  }
  console.log('Y25 yoyo_product_compiler_io_paths=PRESENT');
  console.log('Y25 product_text_off=' + idx);
  console.log('Y25 product_bytes=' + pe.length);
  console.log('Y25 product_sha256_prefix=' + sha.slice(0, 16));
  console.log('Y25 tyb_n=' + n);
  console.log('Y25 status=GREEN');
}

main();
