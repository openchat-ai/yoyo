'use strict';
/**
 * AUTO-TRUER31: open-loop quartet honesty.
 * C1 hosts VA+CreateFile+ReadFile+CloseHandle faces; full product hosts none.
 * Pin: yoyo_product_open_loop=ABSENT (≠ wired open-loop / Thompson).
 */
const fs = require('fs');
const path = require('path');

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
const productPe =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer31', 'product', 'output.exe');
const c1Pe =
  process.argv[3] ||
  path.join(root, 'scripts', '_stage-truer31', 'place-gate', 'yoyo_compiler.exe');

const FACES = [
  ['va', 'yoyo-compiler-open-face.expect.hex'],
  ['createfile', 'yoyo-compiler-createfile-face.expect.hex'],
  ['readfile', 'yoyo-compiler-readfile-face.expect.hex'],
  ['closehandle', 'yoyo-compiler-closehandle-face.expect.hex'],
];

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

function loadExpect(name) {
  return Buffer.from(
    fs.readFileSync(path.join(projects, name), 'utf8').trim().replace(/\s+/g, ''),
    'hex',
  );
}

function main() {
  const pin = readPin(pinPath);
  if (pin.yoyo_compiler_open_loop_faces !== 'PRESENT') {
    console.log('Y31 status=RED reason=pin_open_loop_faces');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop !== 'ABSENT') {
    console.log('Y31 status=RED reason=pin_product_open_loop');
    process.exit(1);
  }
  if (!fs.existsSync(c1Pe) || !fs.existsSync(productPe)) {
    console.log('Y31 status=RED reason=missing_pe');
    process.exit(1);
  }
  const code = fs.readFileSync(codePath);
  const c1Text = peText(fs.readFileSync(c1Pe));
  const prodText = peText(fs.readFileSync(productPe));
  for (const [label, file] of FACES) {
    const exp = loadExpect(file);
    if (code.indexOf(exp) < 0) {
      console.log('Y31 status=RED reason=code_miss_' + label);
      process.exit(1);
    }
    if (c1Text.indexOf(exp) < 0) {
      console.log('Y31 status=RED reason=c1_miss_' + label);
      process.exit(1);
    }
    if (prodText.indexOf(exp) >= 0) {
      console.log('Y31 status=RED reason=product_has_' + label);
      process.exit(1);
    }
    console.log('Y31 face_' + label + '=C1_HIT_PRODUCT_MISS');
  }
  console.log('Y31 yoyo_compiler_open_loop_faces=PRESENT');
  console.log('Y31 yoyo_product_open_loop=ABSENT');
  console.log('Y31 status=GREEN');
}

main();
