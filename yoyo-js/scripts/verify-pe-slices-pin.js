'use strict';
/**
 * TRUSTLIM4: fail-closed if PE hdr[0x200] / .data[0x100] IAT window ≠ pin.
 *   node yoyo-js/scripts/verify-pe-slices-pin.js <pe.exe> <slices.pin>
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

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

function windows(buf) {
  const lfanew = buf.readUInt32LE(0x3c);
  const opt = lfanew + 24;
  const soh = buf.readUInt16LE(lfanew + 20);
  const sec = lfanew + 24 + soh;
  const nsec = buf.readUInt16LE(lfanew + 6);
  let dataRaw = -1;
  for (let i = 0; i < nsec; i++) {
    const s = sec + i * 40;
    if (buf.slice(s, s + 5).toString('ascii').startsWith('.data')) {
      dataRaw = buf.readUInt32LE(s + 20);
    }
  }
  if (dataRaw < 0) throw new Error('no .data');
  const entry = buf.readUInt32LE(opt + 16);
  const hdr = buf.slice(0, 0x200);
  const iat = buf.slice(dataRaw, dataRaw + 0x100);
  return {
    bytes: buf.length,
    entry,
    dataRaw,
    hdrSha: crypto.createHash('sha256').update(hdr).digest('hex'),
    iatSha: crypto.createHash('sha256').update(iat).digest('hex'),
  };
}

const pePath = process.argv[2];
const pinPath = process.argv[3];
if (!pePath || !pinPath || !fs.existsSync(pePath) || !fs.existsSync(pinPath)) {
  console.log('SLICEPIN status=RED reason=missing_pe_or_pin');
  process.exit(1);
}
const pin = readPin(pinPath);
const got = windows(fs.readFileSync(pePath));
console.log(
  `SLICEPIN pe=${path.basename(pePath)} bytes=${got.bytes} entry=${got.entry} hdr=${got.hdrSha.slice(0, 16)} iat=${got.iatSha.slice(0, 16)}`,
);
if (String(pin.bytes) !== String(got.bytes)) {
  console.log('SLICEPIN status=RED reason=bytes_DIFF');
  process.exit(1);
}
if (String(pin.entry_rva) !== String(got.entry)) {
  console.log('SLICEPIN status=RED reason=entry_DIFF');
  process.exit(1);
}
if ((pin.hdr_sha256 || '').toLowerCase() !== got.hdrSha) {
  console.log('SLICEPIN status=RED reason=hdr_DIFF');
  process.exit(1);
}
if ((pin.iat_sha256 || '').toLowerCase() !== got.iatSha) {
  console.log('SLICEPIN status=RED reason=iat_DIFF');
  process.exit(1);
}
console.log('SLICEPIN status=GREEN pin_EQUAL');
console.log('SLICEPIN doc=AUTO-TRUSTLIM4.md');
process.exit(0);
