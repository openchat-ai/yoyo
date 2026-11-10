'use strict';
/**
 * AUTO-TRUER24: C1 × yoyo-compiler-io-paths-face.tyb → PE hosts input.tyb\\0output.exe\\0.
 * Full yoyo.tyb product remains ABSENT those paths (TRUER23 honesty).
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
const faceTyb = path.join(projects, 'yoyo-compiler-io-paths-face.tyb');
const faceExpect = path.join(projects, 'yoyo-compiler-io-paths-face.expect.hex');
const facePe =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer24', 'io-paths-face', 'output.exe');
const productPe =
  process.argv[3] ||
  path.join(root, 'scripts', '_stage-truer24', 'product', 'output.exe');

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
  if (pin.yoyo_compiler_io_paths_face !== 'PRESENT') {
    console.log('Y24 status=RED reason=pin_io_paths_face');
    process.exit(1);
  }
  // TRUER25+: full product may PRESENT io paths; face gate still requires face PE hit.
  if (!fs.existsSync(faceTyb)) {
    console.log('Y24 status=RED reason=missing_face_tyb');
    process.exit(1);
  }
  const tybSha = crypto.createHash('sha256').update(fs.readFileSync(faceTyb)).digest('hex');
  if (
    pin.yoyo_compiler_io_paths_face_tyb_sha256 &&
    pin.yoyo_compiler_io_paths_face_tyb_sha256 !== tybSha
  ) {
    console.log('Y24 status=RED reason=face_tyb_sha_drift got=' + tybSha.slice(0, 16));
    process.exit(1);
  }
  const exp = Buffer.from(
    fs.readFileSync(faceExpect, 'utf8').trim().replace(/\s+/g, ''),
    'hex',
  );
  if (!exp.equals(NEEDLE)) {
    console.log('Y24 status=RED reason=expect_ne_needle');
    process.exit(1);
  }
  if (!fs.existsSync(facePe)) {
    console.log('Y24 status=RED reason=missing_face_pe');
    process.exit(1);
  }
  const pe = fs.readFileSync(facePe);
  if (pe[0] !== 0x4d || pe[1] !== 0x5a) {
    console.log('Y24 status=RED reason=face_not_mz');
    process.exit(1);
  }
  const text = peText(pe);
  const idx = text.indexOf(NEEDLE);
  if (idx < 0) {
    console.log('Y24 status=RED reason=face_text_miss_io_paths');
    process.exit(1);
  }
  if (!fs.existsSync(productPe)) {
    console.log('Y24 status=RED reason=missing_product');
    process.exit(1);
  }
  const prod = fs.readFileSync(productPe);
  const prodHas =
    prod.indexOf(Buffer.from('input.tyb')) >= 0 &&
    prod.indexOf(Buffer.from('output.exe')) >= 0;
  if (pin.yoyo_product_compiler_io_paths === 'PRESENT' && !prodHas) {
    console.log('Y24 status=RED reason=product_missing_io_paths');
    process.exit(1);
  }
  if (pin.yoyo_product_compiler_io_paths === 'ABSENT' && prodHas) {
    console.log('Y24 status=RED reason=product_unexpected_io_paths');
    process.exit(1);
  }
  console.log('Y24 yoyo_compiler_io_paths_face=PRESENT');
  console.log('Y24 face_text_off=' + idx);
  console.log('Y24 face_pe_bytes=' + pe.length);
  console.log(
    'Y24 yoyo_product_compiler_io_paths=' +
      (pin.yoyo_product_compiler_io_paths || 'ABSENT'),
  );
  console.log('Y24 status=GREEN');
}

main();
