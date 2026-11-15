'use strict';
/**
 * AUTO-TRUER41: product hosts entry→connected trampoline face bytes,
 * but PE entry is still NOT that trampoline (entry_wired ABSENT).
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
const trampExpect = path.join(
  projects,
  'yoyo-compiler-open-loop-entry-trampoline-face.expect.hex',
);
const connExpect = path.join(
  projects,
  'yoyo-compiler-open-loop-connected-face.expect.hex',
);
const productPe =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer41', 'product', 'output.exe');

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
  return { text, entryRva, entryOff: entryRva - vaddr };
}

function main() {
  const pin = readPin(pinPath);
  if (pin.yoyo_product_open_loop_entry_trampoline !== 'PRESENT') {
    console.log('Y41 status=RED reason=pin_trampoline');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop_connected !== 'PRESENT') {
    console.log('Y41 status=RED reason=pin_connected');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop_entry_wired !== 'ABSENT') {
    console.log('Y41 status=RED reason=pin_entry_wired');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop !== 'ABSENT') {
    console.log('Y41 status=RED reason=pin_open_loop');
    process.exit(1);
  }
  const tyb = fs.readFileSync(tybPath);
  const n = tyb.readUInt16LE(6);
  if (n < 2546) {
    console.log('Y41 status=RED reason=tyb_n_too_small n=' + n);
    process.exit(1);
  }
  // H_795: 4 ops (40 + 2 A1 + FF)
  const hdrOff = 8 + (2546 - 4) * 8;
  const hdr = tyb.subarray(hdrOff, hdrOff + 8);
  if (hdr[0] !== 0x40 || hdr[2] !== 0x1b || hdr[3] !== 0x03) {
    console.log(
      'Y41 status=RED reason=missing_h795_hdr got=' + hdr.toString('hex'),
    );
    process.exit(1);
  }
  const tramp = Buffer.from(
    fs.readFileSync(trampExpect, 'utf8').trim().replace(/\s+/g, ''),
    'hex',
  );
  const conn = Buffer.from(
    fs.readFileSync(connExpect, 'utf8').trim().replace(/\s+/g, ''),
    'hex',
  );
  if (!fs.existsSync(productPe)) {
    console.log('Y41 status=RED reason=missing_product');
    process.exit(1);
  }
  const pe = fs.readFileSync(productPe);
  const { text, entryRva, entryOff } = peTextAndEntry(pe);
  if (entryOff < 0 || entryOff >= text.length) {
    console.log('Y41 status=RED reason=entry_outside_text');
    process.exit(1);
  }
  const tidx = text.indexOf(tramp);
  if (tidx < 0) {
    console.log('Y41 status=RED reason=product_miss_trampoline');
    process.exit(1);
  }
  const cidx = text.indexOf(conn);
  if (cidx < 0) {
    console.log('Y41 status=RED reason=product_miss_connected');
    process.exit(1);
  }
  const entryHead = text.subarray(entryOff, entryOff + tramp.length);
  if (entryHead.equals(tramp)) {
    console.log('Y41 status=RED reason=trampoline_installed_at_entry');
    process.exit(1);
  }
  const wantRel = (cidx - 5) >>> 0;
  const faceRel = tramp.readUInt32LE(1);
  if (faceRel !== wantRel) {
    console.log(
      'Y41 status=RED reason=trampoline_rel_mismatch face=' +
        faceRel +
        ' want=' +
        wantRel +
        ' connectedOff=' +
        cidx,
    );
    process.exit(1);
  }
  const sha = crypto.createHash('sha256').update(pe).digest('hex');
  if (
    pin.yoyo_product_with_io_paths_sha256 &&
    pin.yoyo_product_with_io_paths_sha256 !== sha
  ) {
    console.log('Y41 status=RED reason=product_sha_drift got=' + sha.slice(0, 16));
    process.exit(1);
  }
  console.log('Y41 yoyo_product_open_loop_entry_trampoline=PRESENT');
  console.log('Y41 yoyo_product_open_loop_connected=PRESENT');
  console.log('Y41 yoyo_product_open_loop_entry_wired=ABSENT');
  console.log('Y41 yoyo_product_open_loop=ABSENT');
  console.log('Y41 entry_rva=0x' + entryRva.toString(16));
  console.log('Y41 entry_text_off=' + entryOff);
  console.log('Y41 connected_text_off=' + cidx);
  console.log('Y41 trampoline_text_off=' + tidx);
  console.log('Y41 trampoline_rel=0x' + faceRel.toString(16));
  console.log('Y41 product_sha256_prefix=' + sha.slice(0, 16));
  console.log('Y41 status=GREEN');
}

main();
