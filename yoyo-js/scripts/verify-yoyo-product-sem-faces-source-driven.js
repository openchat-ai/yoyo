'use strict';
/**
 * AUTO-TRUER22: product sem faces are source-driven by yoyo.tyb.
 * Mutate locked IMUL record (63 02 50 00 51…) → 6A; recompiled product must MISS imul face.
 * Usage: node verify-yoyo-product-sem-faces-source-driven.js <base.exe> <mut.exe>
 */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');
const pinPath = path.join(
  __dirname,
  '..',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'payload.pin',
);
const tybPath = path.join(projects, 'yoyo.tyb');
const basePath = process.argv[2];
const mutPath = process.argv[3];

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
    fs
      .readFileSync(path.join(projects, name + '.expect.hex'), 'utf8')
      .trim()
      .replace(/\s+/g, ''),
    'hex',
  );
}

function main() {
  if (!basePath || !mutPath) {
    console.log('Y22 status=RED reason=usage_need_base_mut_exe');
    process.exit(1);
  }
  const pin = readPin(pinPath);
  if (pin.yoyo_product_sem_faces_source_driven !== 'PRESENT') {
    console.log('Y22 status=RED reason=pin_source_driven');
    process.exit(1);
  }
  const mutOff = parseInt(pin.yoyo_product_sem_face_mut_off, 10);
  const mutPat = Buffer.from(pin.yoyo_product_sem_face_mut_pat, 'hex');
  const mutFrom = parseInt(pin.yoyo_product_sem_face_mut_from, 16);
  const mutTo = parseInt(pin.yoyo_product_sem_face_mut_to, 16);
  const missName = pin.yoyo_product_sem_face_mut_miss || 'yoyo-sem-imul';
  if (!Number.isFinite(mutOff) || mutPat.length < 1) {
    console.log('Y22 status=RED reason=pin_mut_fields');
    process.exit(1);
  }

  const tyb = fs.readFileSync(tybPath);
  if (tyb.length < mutOff + mutPat.length) {
    console.log('Y22 status=RED reason=tyb_too_small');
    process.exit(1);
  }
  const at = tyb.subarray(mutOff, mutOff + mutPat.length);
  if (!at.equals(mutPat)) {
    console.log(
      'Y22 status=RED reason=tyb_mut_pat_drift off=' +
        mutOff +
        ' got=' +
        at.toString('hex'),
    );
    process.exit(1);
  }
  if (mutPat[0] !== mutFrom) {
    console.log('Y22 status=RED reason=mut_from_mismatch');
    process.exit(1);
  }
  console.log(
    'Y22 tyb_mut_anchor off=' +
      mutOff +
      ' pat=' +
      mutPat.toString('hex') +
      ' from=' +
      mutFrom.toString(16) +
      ' to=' +
      mutTo.toString(16),
  );

  if (!fs.existsSync(basePath) || !fs.existsSync(mutPath)) {
    console.log('Y22 status=RED reason=missing_pe');
    process.exit(1);
  }
  const baseText = peText(fs.readFileSync(basePath));
  const mutText = peText(fs.readFileSync(mutPath));
  const missExp = loadExpect(missName);
  const baseIdx = baseText.indexOf(missExp);
  const mutIdx = mutText.indexOf(missExp);
  if (baseIdx < 0) {
    console.log('Y22 status=RED reason=base_miss_face name=' + missName);
    process.exit(1);
  }
  if (mutIdx >= 0) {
    console.log(
      'Y22 status=RED reason=mut_still_has_face name=' +
        missName +
        ' text_off=' +
        mutIdx,
    );
    process.exit(1);
  }
  console.log(
    'Y22 base_face_hit name=' + missName + ' text_off=' + baseIdx,
  );
  console.log('Y22 mut_face_miss name=' + missName);
  console.log('Y22 yoyo_product_sem_faces_source_driven=PRESENT');
  console.log('Y22 status=GREEN');
}

main();
