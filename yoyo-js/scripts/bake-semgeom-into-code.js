'use strict';
/**
 * AUTO-SEMGEOM: patch try_place PE-geometry immediates so place embeds full code.bin.
 * textVs/sizeOfImage/dataRva unchanged (prefix grew file raw only).
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { layoutFor } = require('../src/platform/yoyo-compiler-parse1-native-link');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const projects = path.join(root, 'yoyo', 'projects');
const codePath = path.join(payload, 'code.bin');
const dataPath = path.join(payload, 'data.bin');
const pinPath = path.join(payload, 'payload.pin');

const OLD_CODE_LEN = 16717;
const PLACE_BODY = 4491;

function packPlant(bytes) {
  const recs = [];
  let i = 0;
  while (i < bytes.length) {
    const b = Buffer.alloc(8);
    if (bytes.length - i >= 3) {
      b[0] = 0xa1;
      b[1] = 3;
      b[2] = bytes[i];
      b[4] = bytes[i + 1];
      b[6] = bytes[i + 2];
      i += 3;
    } else {
      b[0] = 0xa0;
      b[1] = 1;
      b[2] = bytes[i++];
    }
    recs.push(b);
  }
  const ff = Buffer.alloc(8);
  ff[0] = 0xff;
  recs.push(ff);
  const out = Buffer.alloc(8 + recs.length * 8);
  out.write('TYB\0', 0, 4, 'ascii');
  out.writeUInt16LE(0, 4);
  out.writeUInt16LE(recs.length, 6);
  for (let r = 0; r < recs.length; r++) recs[r].copy(out, 8 + r * 8);
  return out;
}

function setPin(pinText, key, val) {
  const re = new RegExp('^' + key + '=.*$', 'm');
  if (re.test(pinText)) return pinText.replace(re, key + '=' + val);
  return pinText.trimEnd() + '\n' + key + '=' + val + '\n';
}

function findAll(buf, v) {
  const out = [];
  const want = v >>> 0;
  for (let i = 0; i + 3 < buf.length; i++) {
    if (buf.readUInt32LE(i) === want) out.push(i);
  }
  return out;
}

function replaceImm(buf, oldV, newV, expectedHits) {
  const hits = findAll(buf, oldV);
  if (hits.length !== expectedHits) {
    throw new Error(
      'imm ' + (oldV >>> 0) + ' hits=' + hits.length + ' expect=' + expectedHits
    );
  }
  for (const s of hits) buf.writeUInt32LE(newV >>> 0, s);
  return hits;
}

function main() {
  const code = Buffer.from(fs.readFileSync(codePath));
  const data = fs.readFileSync(dataPath);
  if (!code.includes(Buffer.from('SEMHOST1'))) {
    console.log('SEMGEOM status=RED reason=missing_SEMBED_marker');
    process.exit(1);
  }
  if (code.length !== 18074) {
    console.log('SEMGEOM status=RED reason=unexpected_code_len got=' + code.length);
    process.exit(1);
  }

  const Lold = layoutFor(OLD_CODE_LEN, data.length, 0x38000);
  const Lnew = layoutFor(code.length, data.length, 0x38000);
  if (Lold.textVs !== Lnew.textVs || Lold.dataRva !== Lnew.dataRva) {
    console.log('SEMGEOM status=RED reason=textVs_or_dataRva_changed');
    process.exit(1);
  }

  // Already patched?
  if (findAll(code, Lnew.peSize).length >= 6 && findAll(code, OLD_CODE_LEN).length === 0) {
    console.log('SEMGEOM already=PATCHED peSize=' + Lnew.peSize + ' codeRaw=' + Lnew.codeRaw);
    console.log('SEMGEOM status=GREEN');
    process.exit(0);
  }

  replaceImm(code, OLD_CODE_LEN, code.length, 5);
  replaceImm(code, Lold.peSize, Lnew.peSize, 6);
  replaceImm(code, Lold.codeRaw, Lnew.codeRaw, 2);
  replaceImm(code, Lold.dataFileOff, Lnew.dataFileOff, 2);
  replaceImm(code, Lold.userDataOff, Lnew.userDataOff, 2);
  for (let k = 8; k < Lold.iatLen; k += 8) {
    const ov = (Lold.dataFileOff + k) >>> 0;
    const nv = (Lnew.dataFileOff + k) >>> 0;
    replaceImm(code, ov, nv, 1);
  }

  const prefix = code.length - OLD_CODE_LEN;
  const placeLen = prefix + PLACE_BODY;
  const place = code.subarray(0, placeLen);
  const parse1 = code.subarray(placeLen);

  fs.writeFileSync(codePath, code);
  fs.writeFileSync(path.join(projects, 'yoyo-plant-place.tyb'), packPlant(place));
  fs.writeFileSync(path.join(projects, 'yoyo-plant-parse1.tyb'), packPlant(parse1));
  fs.writeFileSync(path.join(projects, 'yoyo-plant.tyb'), packPlant(code));

  const codeSha = crypto.createHash('sha256').update(code).digest('hex');
  const placeSha = crypto
    .createHash('sha256')
    .update(fs.readFileSync(path.join(projects, 'yoyo-plant-place.tyb')))
    .digest('hex');
  const parse1Sha = crypto
    .createHash('sha256')
    .update(fs.readFileSync(path.join(projects, 'yoyo-plant-parse1.tyb')))
    .digest('hex');

  let pin = fs.readFileSync(pinPath, 'utf8');
  pin = setPin(pin, 'code_bytes', String(code.length));
  pin = setPin(pin, 'code_sha256', codeSha);
  pin = setPin(pin, 'code_sha256_prefix', codeSha.slice(0, 16));
  pin = setPin(pin, 'code_place_bytes', String(placeLen));
  pin = setPin(pin, 'code_parse1_bytes', String(parse1.length));
  pin = setPin(pin, 'code_plant_place_sha256', placeSha);
  pin = setPin(pin, 'code_plant_parse1_sha256', parse1Sha);
  pin = setPin(pin, 'yoyo_semgeom', 'PRESENT');
  pin = setPin(pin, 'yoyo_semgeom_pe_size', String(Lnew.peSize));
  pin = setPin(pin, 'yoyo_semgeom_code_raw', String(Lnew.codeRaw));
  pin = setPin(pin, 'note', 'SEMGEOM_place_pe_geom_aligned;large_defaults_ret_pe;not_Thompson');
  pin = setPin(pin, 'doc', 'AUTO-SEMGEOM.md');
  fs.writeFileSync(pinPath, pin);

  console.log('SEMGEOM peSize ' + Lold.peSize + '→' + Lnew.peSize);
  console.log('SEMGEOM codeRaw ' + Lold.codeRaw + '→' + Lnew.codeRaw);
  console.log('SEMGEOM dataFileOff ' + Lold.dataFileOff + '→' + Lnew.dataFileOff);
  console.log('SEMGEOM code_bytes=' + code.length);
  console.log('SEMGEOM status=GREEN');
}

main();
