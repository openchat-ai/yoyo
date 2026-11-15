'use strict';
/**
 * AUTO-SEMLARGE: rebuild SEMBED prefix so TYB size>63 jumps old H_00,
 * then re-align try_place PE geometry to the new total code length.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { layoutFor } = require('../src/platform/yoyo-compiler-parse1-native-link');
const { buildPrefix, packPlant, setPin, OLD_PLACE } = require('./bake-sembed-into-code');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const projects = path.join(root, 'yoyo', 'projects');
const codePath = path.join(payload, 'code.bin');
const dataPath = path.join(payload, 'data.bin');
const pinPath = path.join(payload, 'payload.pin');

const BODY_LEN = 16717;

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

function readPrefixLen(pinText, codeLen) {
  const m = pinText.match(/^\s*yoyo_sembed_prefix_bytes=(\d+)\s*$/m);
  if (m) return parseInt(m[1], 10);
  return codeLen - BODY_LEN;
}

function main() {
  let pin = fs.readFileSync(pinPath, 'utf8');
  const code = Buffer.from(fs.readFileSync(codePath));
  const data = fs.readFileSync(dataPath);
  if (!code.includes(Buffer.from('SEMHOST1'))) {
    console.log('SEMLARGE status=RED reason=missing_SEMBED_marker');
    process.exit(1);
  }

  const oldPrefixLen = readPrefixLen(pin, code.length);
  if (code.length !== oldPrefixLen + BODY_LEN) {
    console.log(
      'SEMLARGE status=RED reason=len_mismatch code=' +
        code.length +
        ' prefix=' +
        oldPrefixLen
    );
    process.exit(1);
  }

  const body = Buffer.from(code.subarray(oldPrefixLen));
  // Body still carries geometry for previous total length — revert to BODY_LEN layout.
  const Lcur = layoutFor(code.length, data.length, 0x38000);
  const Lbody = layoutFor(BODY_LEN, data.length, 0x38000);
  if (findAll(body, code.length).length === 5) {
    replaceImm(body, code.length, BODY_LEN, 5);
    replaceImm(body, Lcur.peSize, Lbody.peSize, 6);
    replaceImm(body, Lcur.codeRaw, Lbody.codeRaw, 2);
    replaceImm(body, Lcur.dataFileOff, Lbody.dataFileOff, 2);
    replaceImm(body, Lcur.userDataOff, Lbody.userDataOff, 2);
    for (let k = 8; k < Lcur.iatLen; k += 8) {
      replaceImm(body, (Lcur.dataFileOff + k) >>> 0, (Lbody.dataFileOff + k) >>> 0, 1);
    }
  } else if (findAll(body, BODY_LEN).length !== 5) {
    console.log('SEMLARGE status=RED reason=body_codeLen_imm_missing');
    process.exit(1);
  }

  const prefix = buildPrefix();
  const newCode = Buffer.concat([prefix, body]);
  const Lnew = layoutFor(newCode.length, data.length, 0x38000);

  replaceImm(newCode, BODY_LEN, newCode.length, 5);
  replaceImm(newCode, Lbody.peSize, Lnew.peSize, 6);
  replaceImm(newCode, Lbody.codeRaw, Lnew.codeRaw, 2);
  replaceImm(newCode, Lbody.dataFileOff, Lnew.dataFileOff, 2);
  replaceImm(newCode, Lbody.userDataOff, Lnew.userDataOff, 2);
  for (let k = 8; k < Lbody.iatLen; k += 8) {
    replaceImm(
      newCode,
      (Lbody.dataFileOff + k) >>> 0,
      (Lnew.dataFileOff + k) >>> 0,
      1
    );
  }

  const placeLen = prefix.length + OLD_PLACE;
  const place = newCode.subarray(0, placeLen);
  const parse1 = newCode.subarray(placeLen);

  fs.writeFileSync(codePath, newCode);
  fs.writeFileSync(path.join(projects, 'yoyo-plant-place.tyb'), packPlant(place));
  fs.writeFileSync(path.join(projects, 'yoyo-plant-parse1.tyb'), packPlant(parse1));
  fs.writeFileSync(path.join(projects, 'yoyo-plant.tyb'), packPlant(newCode));

  const codeSha = crypto.createHash('sha256').update(newCode).digest('hex');
  const placeSha = crypto
    .createHash('sha256')
    .update(fs.readFileSync(path.join(projects, 'yoyo-plant-place.tyb')))
    .digest('hex');
  const parse1Sha = crypto
    .createHash('sha256')
    .update(fs.readFileSync(path.join(projects, 'yoyo-plant-parse1.tyb')))
    .digest('hex');

  pin = setPin(pin, 'code_bytes', String(newCode.length));
  pin = setPin(pin, 'code_sha256', codeSha);
  pin = setPin(pin, 'code_sha256_prefix', codeSha.slice(0, 16));
  pin = setPin(pin, 'code_place_bytes', String(placeLen));
  pin = setPin(pin, 'code_parse1_bytes', String(parse1.length));
  pin = setPin(pin, 'code_plant_place_sha256', placeSha);
  pin = setPin(pin, 'code_plant_parse1_sha256', parse1Sha);
  pin = setPin(pin, 'yoyo_sembed_prefix_bytes', String(prefix.length));
  pin = setPin(pin, 'yoyo_semlarge', 'PRESENT');
  pin = setPin(pin, 'yoyo_semgeom', 'PRESENT');
  pin = setPin(pin, 'yoyo_semgeom_pe_size', String(Lnew.peSize));
  pin = setPin(pin, 'yoyo_semgeom_code_raw', String(Lnew.codeRaw));
  pin = setPin(pin, 'note', 'SEMLARGE_large_copy_self;geom_aligned;not_Thompson');
  pin = setPin(pin, 'doc', 'AUTO-SEMLARGE.md');
  fs.writeFileSync(pinPath, pin);

  console.log('SEMLARGE prefix_bytes ' + oldPrefixLen + '→' + prefix.length);
  console.log('SEMLARGE code_bytes ' + code.length + '→' + newCode.length);
  console.log('SEMLARGE peSize=' + Lnew.peSize + ' codeRaw=' + Lnew.codeRaw);
  console.log('SEMLARGE status=GREEN');
}

main();
