'use strict';
/**
 * AUTO-TRUER21: C1 × yoyo.tyb product PE .text hosts all semantic emit-face expect.hex.
 * Usage: node verify-yoyo-product-hosts-sem-faces.js [product.exe]
 */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');
const productPath =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer21', 'product', 'output.exe');

const SLICES = [
  'yoyo-sem-ret',
  'yoyo-sem-nop',
  'yoyo-sem-sub',
  'yoyo-sem-add',
  'yoyo-sem-get',
  'yoyo-sem-set',
  'yoyo-sem-mov',
  'yoyo-sem-cmp',
  'yoyo-sem-inc',
  'yoyo-sem-dec',
  'yoyo-sem-addv',
  'yoyo-sem-orv',
  'yoyo-sem-subv',
  'yoyo-sem-imul',
];

function peText(pe) {
  const e = pe.readUInt32LE(0x3c);
  const sizeOpt = pe.readUInt16LE(e + 20);
  const sect = e + 24 + sizeOpt;
  const rp = pe.readUInt32LE(sect + 20);
  const rs = pe.readUInt32LE(sect + 16);
  return pe.subarray(rp, rp + rs);
}

function main() {
  if (!fs.existsSync(productPath)) {
    console.log('Y21 status=RED reason=missing_product path=' + productPath);
    process.exit(1);
  }
  const pe = fs.readFileSync(productPath);
  if (pe.length < 2 || pe[0] !== 0x4d || pe[1] !== 0x5a) {
    console.log('Y21 status=RED reason=product_not_mz');
    process.exit(1);
  }
  const text = peText(pe);
  let hits = 0;
  for (const name of SLICES) {
    const expPath = path.join(projects, name + '.expect.hex');
    if (!fs.existsSync(expPath)) {
      console.log('Y21 status=RED reason=missing_expect name=' + name);
      process.exit(1);
    }
    const exp = Buffer.from(
      fs.readFileSync(expPath, 'utf8').trim().replace(/\s+/g, ''),
      'hex',
    );
    const idx = text.indexOf(exp);
    if (idx < 0) {
      console.log('Y21 status=RED reason=face_miss name=' + name);
      process.exit(1);
    }
    hits += 1;
    console.log(
      'Y21 face_hit name=' +
        name +
        ' exp_bytes=' +
        exp.length +
        ' text_off=' +
        idx,
    );
  }
  console.log('Y21 product_bytes=' + pe.length);
  console.log('Y21 text_bytes=' + text.length);
  console.log('Y21 sem_faces_hit=' + hits + '/' + SLICES.length);
  console.log('Y21 yoyo_product_hosts_sem_emit_faces=PRESENT');
  console.log('Y21 status=GREEN');
}

main();
