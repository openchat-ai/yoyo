'use strict';
/**
 * AUTO-TRUER40: product hosts connected face, but PE entry is NOT that face.
 * Pins yoyo_product_open_loop_entry_wired=ABSENT (literal ≠ default entry).
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
const faceExpect = path.join(
  projects,
  'yoyo-compiler-open-loop-connected-face.expect.hex',
);
const productPe =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer40', 'product', 'output.exe');

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

function peTextAndEntry(pe) {
  const e = pe.readUInt32LE(0x3c);
  const sizeOpt = pe.readUInt16LE(e + 20);
  const entryRva = pe.readUInt32LE(e + 24 + 16);
  const sect = e + 24 + sizeOpt;
  const vaddr = pe.readUInt32LE(sect + 12);
  const rp = pe.readUInt32LE(sect + 20);
  const rs = pe.readUInt32LE(sect + 16);
  const text = pe.subarray(rp, rp + rs);
  const entryOff = entryRva - vaddr;
  return { text, entryRva, entryOff, vaddr };
}

function main() {
  const pin = readPin(pinPath);
  if (pin.yoyo_product_open_loop_connected !== 'PRESENT') {
    console.log('Y40 status=RED reason=pin_connected');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop_entry_wired !== 'ABSENT') {
    console.log('Y40 status=RED reason=pin_entry_wired');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop !== 'ABSENT') {
    console.log('Y40 status=RED reason=pin_open_loop');
    process.exit(1);
  }
  const exp = Buffer.from(
    fs.readFileSync(faceExpect, 'utf8').trim().replace(/\s+/g, ''),
    'hex',
  );
  if (!fs.existsSync(productPe)) {
    console.log('Y40 status=RED reason=missing_product');
    process.exit(1);
  }
  const pe = fs.readFileSync(productPe);
  const { text, entryRva, entryOff } = peTextAndEntry(pe);
  if (entryOff < 0 || entryOff >= text.length) {
    console.log(
      'Y40 status=RED reason=entry_outside_text entryOff=' + entryOff,
    );
    process.exit(1);
  }
  const idx = text.indexOf(exp);
  if (idx < 0) {
    console.log('Y40 status=RED reason=product_miss_connected');
    process.exit(1);
  }
  const entryInConnected = entryOff >= idx && entryOff < idx + exp.length;
  if (entryInConnected) {
    console.log(
      'Y40 status=RED reason=entry_lands_in_connected entryOff=' +
        entryOff +
        ' connectedOff=' +
        idx,
    );
    process.exit(1);
  }
  const entryHead = text.subarray(entryOff, entryOff + Math.min(16, exp.length));
  const connHead = exp.subarray(0, entryHead.length);
  if (entryHead.equals(connHead)) {
    console.log('Y40 status=RED reason=entry_bytes_match_connected_head');
    process.exit(1);
  }
  const sha = crypto.createHash('sha256').update(pe).digest('hex');
  if (
    pin.yoyo_product_with_io_paths_sha256 &&
    pin.yoyo_product_with_io_paths_sha256 !== sha
  ) {
    console.log('Y40 status=RED reason=product_sha_drift got=' + sha.slice(0, 16));
    process.exit(1);
  }
  console.log('Y40 yoyo_product_open_loop_connected=PRESENT');
  console.log('Y40 yoyo_product_open_loop_entry_wired=ABSENT');
  console.log('Y40 yoyo_product_open_loop=ABSENT');
  console.log('Y40 entry_rva=0x' + entryRva.toString(16));
  console.log('Y40 entry_text_off=' + entryOff);
  console.log('Y40 connected_text_off=' + idx);
  console.log('Y40 product_sha256_prefix=' + sha.slice(0, 16));
  console.log('Y40 status=GREEN');
}

main();
