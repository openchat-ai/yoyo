'use strict';
/**
 * AUTO-TRUER42: product PE entry hosts trampoline jmp to connected.
 * Full open_loop / relay remain ABSENT (connected ≠ open-read-emit).
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
const connExpect = path.join(
  projects,
  'yoyo-compiler-open-loop-connected-face.expect.hex',
);
const productPe =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer42', 'installed', 'output.exe');

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
  if (pin.yoyo_product_open_loop_entry_trampoline_installed !== 'PRESENT') {
    console.log('Y42 status=RED reason=pin_installed');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop_entry_wired !== 'PRESENT') {
    console.log('Y42 status=RED reason=pin_entry_wired');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop_connected !== 'PRESENT') {
    console.log('Y42 status=RED reason=pin_connected');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop !== 'ABSENT') {
    console.log('Y42 status=RED reason=pin_open_loop');
    process.exit(1);
  }
  if (!fs.existsSync(productPe)) {
    console.log('Y42 status=RED reason=missing_product');
    process.exit(1);
  }
  const pe = fs.readFileSync(productPe);
  const { text, entryRva, entryOff } = peTextAndEntry(pe);
  if (entryOff < 0 || entryOff + 5 > text.length) {
    console.log('Y42 status=RED reason=entry_oob');
    process.exit(1);
  }
  const conn = Buffer.from(
    fs.readFileSync(connExpect, 'utf8').trim().replace(/\s+/g, ''),
    'hex',
  );
  const cidx = text.indexOf(conn);
  if (cidx < 0) {
    console.log('Y42 status=RED reason=product_miss_connected');
    process.exit(1);
  }
  const head = text.subarray(entryOff, entryOff + 5);
  if (head[0] !== 0xe9) {
    console.log('Y42 status=RED reason=entry_not_jmp got=' + head.toString('hex'));
    process.exit(1);
  }
  const rel = head.readInt32LE(1);
  const tgt = entryOff + 5 + rel;
  if (tgt !== cidx) {
    console.log(
      'Y42 status=RED reason=jmp_miss_connected tgt=' + tgt + ' connectedOff=' + cidx,
    );
    process.exit(1);
  }
  const sha = crypto.createHash('sha256').update(pe).digest('hex');
  if (
    pin.yoyo_product_entry_trampoline_installed_sha256 &&
    pin.yoyo_product_entry_trampoline_installed_sha256 !== sha
  ) {
    console.log('Y42 status=RED reason=installed_sha_drift got=' + sha.slice(0, 16));
    process.exit(1);
  }
  console.log('Y42 yoyo_product_open_loop_entry_trampoline_installed=PRESENT');
  console.log('Y42 yoyo_product_open_loop_entry_wired=PRESENT');
  console.log('Y42 yoyo_product_open_loop_connected=PRESENT');
  console.log('Y42 yoyo_product_open_loop=ABSENT');
  console.log('Y42 entry_rva=0x' + entryRva.toString(16));
  console.log('Y42 entry_text_off=' + entryOff);
  console.log('Y42 entry_head=' + head.toString('hex'));
  console.log('Y42 connected_text_off=' + cidx);
  console.log('Y42 product_sha256_prefix=' + sha.slice(0, 16));
  console.log('Y42 status=GREEN');
}

main();
