'use strict';
/**
 * AUTO-TRUER28: CreateFileA open-path face.
 * C1/code PRESENT; full yoyo.tyb product ABSENT; YOYO createfile-face PE HIT.
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
const codePath = path.join(
  root,
  'yoyo-js',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'code.bin',
);
const faceTyb = path.join(projects, 'yoyo-compiler-createfile-face.tyb');
const faceExpect = path.join(projects, 'yoyo-compiler-createfile-face.expect.hex');
const facePe =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer28', 'createfile-face', 'output.exe');
const productPe =
  process.argv[3] ||
  path.join(root, 'scripts', '_stage-truer28', 'product', 'output.exe');
const c1Pe =
  process.argv[4] ||
  path.join(root, 'scripts', '_stage-truer28', 'place-gate', 'yoyo_compiler.exe');

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
  if (pin.yoyo_compiler_createfile_face !== 'PRESENT') {
    console.log('Y28 status=RED reason=pin_createfile_face');
    process.exit(1);
  }
  if (pin.yoyo_product_createfile_face !== 'ABSENT') {
    console.log('Y28 status=RED reason=pin_product_createfile');
    process.exit(1);
  }
  const exp = Buffer.from(
    fs.readFileSync(faceExpect, 'utf8').trim().replace(/\s+/g, ''),
    'hex',
  );
  const tyb = fs.readFileSync(faceTyb);
  const tybSha = crypto.createHash('sha256').update(tyb).digest('hex');
  if (
    pin.yoyo_compiler_createfile_face_tyb_sha256 &&
    pin.yoyo_compiler_createfile_face_tyb_sha256 !== tybSha
  ) {
    console.log('Y28 status=RED reason=tyb_sha_drift got=' + tybSha.slice(0, 16));
    process.exit(1);
  }
  const code = fs.readFileSync(codePath);
  if (code.indexOf(exp) < 0) {
    console.log('Y28 status=RED reason=code_miss_createfile_face');
    process.exit(1);
  }
  if (!fs.existsSync(c1Pe)) {
    console.log('Y28 status=RED reason=missing_c1');
    process.exit(1);
  }
  const c1Text = peText(fs.readFileSync(c1Pe));
  if (c1Text.indexOf(exp) < 0) {
    console.log('Y28 status=RED reason=c1_miss_createfile_face');
    process.exit(1);
  }
  if (!fs.existsSync(productPe)) {
    console.log('Y28 status=RED reason=missing_product');
    process.exit(1);
  }
  const prodText = peText(fs.readFileSync(productPe));
  if (prodText.indexOf(exp) >= 0) {
    console.log('Y28 status=RED reason=product_unexpected_createfile_face');
    process.exit(1);
  }
  if (!fs.existsSync(facePe)) {
    console.log('Y28 status=RED reason=missing_face_pe');
    process.exit(1);
  }
  const faceText = peText(fs.readFileSync(facePe));
  const idx = faceText.indexOf(exp);
  if (idx < 0) {
    console.log('Y28 status=RED reason=face_pe_miss');
    process.exit(1);
  }
  console.log('Y28 yoyo_compiler_createfile_face=PRESENT');
  console.log('Y28 yoyo_product_createfile_face=ABSENT');
  console.log('Y28 face_text_off=' + idx);
  console.log('Y28 status=GREEN');
}

main();
