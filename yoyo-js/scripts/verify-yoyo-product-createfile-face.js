'use strict';
/**
 * AUTO-TRUER33: full yoyo.tyb product hosts CreateFileA open-path face.
 * ReadFile/CloseHandle remain ABSENT (open_loop still ABSENT).
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
const tybPath = path.join(projects, 'yoyo.tyb');
const faceExpect = path.join(projects, 'yoyo-compiler-createfile-face.expect.hex');
const productPe =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer33', 'product', 'output.exe');

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
  if (pin.yoyo_product_createfile_face !== 'PRESENT') {
    console.log('Y33 status=RED reason=pin_product_createfile');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop !== 'ABSENT') {
    console.log('Y33 status=RED reason=pin_open_loop_not_absent');
    process.exit(1);
  }
  const tyb = fs.readFileSync(tybPath);
  const n = tyb.readUInt16LE(6);
  if (n < 2490) {
    console.log('Y33 status=RED reason=tyb_n_too_small n=' + n);
    process.exit(1);
  }
  // H_790 header: 40 01 16 03 …
  const hdrOff = 8 + (2490 - 11) * 8;
  const hdr = tyb.subarray(hdrOff, hdrOff + 8);
  if (hdr[0] !== 0x40 || hdr[2] !== 0x16 || hdr[3] !== 0x03) {
    console.log(
      'Y33 status=RED reason=missing_h790_hdr got=' + hdr.toString('hex'),
    );
    process.exit(1);
  }
  const exp = Buffer.from(
    fs.readFileSync(faceExpect, 'utf8').trim().replace(/\s+/g, ''),
    'hex',
  );
  if (!fs.existsSync(productPe)) {
    console.log('Y33 status=RED reason=missing_product');
    process.exit(1);
  }
  const pe = fs.readFileSync(productPe);
  const text = peText(pe);
  const idx = text.indexOf(exp);
  if (idx < 0) {
    console.log('Y33 status=RED reason=product_miss_createfile_face');
    process.exit(1);
  }
  const sha = crypto.createHash('sha256').update(pe).digest('hex');
  if (
    pin.yoyo_product_with_io_paths_sha256 &&
    pin.yoyo_product_with_io_paths_sha256 !== sha
  ) {
    console.log('Y33 status=RED reason=product_sha_drift got=' + sha.slice(0, 16));
    process.exit(1);
  }
  console.log('Y33 yoyo_product_createfile_face=PRESENT');
  console.log('Y33 yoyo_product_open_loop=ABSENT');
  console.log('Y33 product_text_off=' + idx);
  console.log('Y33 product_sha256_prefix=' + sha.slice(0, 16));
  console.log('Y33 status=GREEN');
}

main();
