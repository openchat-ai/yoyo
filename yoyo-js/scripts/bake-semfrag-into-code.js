'use strict';
/**
 * AUTO-SEMFRAG: live prefix core from yoyo-openloop-h00.fragment.ty (YOYO IR).
 * expect.hex is cross-check only — not the bake input.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { layoutFor } = require('../src/platform/yoyo-compiler-parse1-native-link');
const { packPlant, setPin, OLD_PLACE } = require('./bake-sembed-into-code');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const projects = path.join(root, 'yoyo', 'projects');
const codePath = path.join(payload, 'code.bin');
const dataPath = path.join(payload, 'data.bin');
const pinPath = path.join(payload, 'payload.pin');
const fragPath = path.join(projects, 'yoyo-openloop-h00.fragment.ty');
const expectPath = path.join(projects, 'yoyo-openloop-h00.expect.hex');

const BODY_LEN = 16717;

function u32(n) {
  return [n & 255, (n >>> 8) & 255, (n >>> 16) & 255, (n >>> 24) & 255];
}

function unpackFragment(text) {
  const out = [];
  for (const line of text.split(/\r?\n/)) {
    const t = line.trim();
    if (!t || t.startsWith(';') || /^40\s+0$/.test(t) || /^FF$/i.test(t)) continue;
    const m1 = t.match(/^A1\s+([0-9a-fA-F]{2})\s+([0-9a-fA-F]{2})\s+([0-9a-fA-F]{2})$/);
    if (m1) {
      out.push(parseInt(m1[1], 16), parseInt(m1[2], 16), parseInt(m1[3], 16));
      continue;
    }
    const m0 = t.match(/^A0\s+([0-9a-fA-F]{2})$/);
    if (m0) {
      out.push(parseInt(m0[1], 16));
      continue;
    }
  }
  return Buffer.from(out);
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

function readPrefixLen(pinText, codeLen) {
  for (const key of ['yoyo_semfrag_prefix_bytes', 'yoyo_semolp_prefix_bytes', 'yoyo_sembed_prefix_bytes']) {
    const m = pinText.match(new RegExp('^\\s*' + key + '=(\\d+)\\s*$', 'm'));
    if (m) return parseInt(m[1], 10);
  }
  return codeLen - BODY_LEN;
}

function patchCreateFileFailTo(buf, placeModeOff) {
  const pat = Buffer.from([0x48, 0x83, 0xf8, 0xff, 0x0f, 0x84]);
  const idx = buf.indexOf(pat);
  if (idx < 0) throw new Error('CreateFile fail jcc not found');
  const relAt = idx + 6;
  const next = relAt + 4;
  buf.writeInt32LE(placeModeOff - next, relAt);
  return idx;
}

function buildPlaceModeEpilogue(prefixCoreLen) {
  const parts = [];
  parts.push(0x48, 0x81, 0xc4, ...u32(0x88));
  const jmpAt = parts.length;
  parts.push(0xe9, 0, 0, 0, 0);
  const epi = Buffer.from(parts);
  const oldH00 = prefixCoreLen + epi.length;
  const rel = oldH00 - (prefixCoreLen + jmpAt + 5);
  epi.writeInt32LE(rel, jmpAt + 1);
  return epi;
}

function main() {
  let pin = fs.readFileSync(pinPath, 'utf8');
  const code = Buffer.from(fs.readFileSync(codePath));
  const data = fs.readFileSync(dataPath);
  const core = unpackFragment(fs.readFileSync(fragPath, 'utf8'));
  const expect = Buffer.from(
    fs.readFileSync(expectPath, 'utf8').trim().replace(/\s+/g, ''),
    'hex'
  );
  if (!core.equals(expect)) {
    console.log('SEMFRAG status=RED reason=fragment_ne_expect');
    process.exit(1);
  }
  if (!core.includes(Buffer.from('SEMHOST1'))) {
    console.log('SEMFRAG status=RED reason=fragment_missing_SEMHOST1');
    process.exit(1);
  }

  const oldPrefixLen = readPrefixLen(pin, code.length);
  if (code.length !== oldPrefixLen + BODY_LEN) {
    console.log(
      'SEMFRAG status=RED reason=len_mismatch code=' +
        code.length +
        ' prefix=' +
        oldPrefixLen
    );
    process.exit(1);
  }

  const body = Buffer.from(code.subarray(oldPrefixLen));
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
    console.log('SEMFRAG status=RED reason=body_codeLen_imm_missing');
    process.exit(1);
  }

  const epi = buildPlaceModeEpilogue(core.length);
  const prefix = Buffer.concat([Buffer.from(core), epi]);
  patchCreateFileFailTo(prefix, core.length);

  const newCode = Buffer.concat([prefix, body]);
  const Lnew = layoutFor(newCode.length, data.length, 0x38000);
  if (Lbody.textVs !== Lnew.textVs) {
    console.log('SEMFRAG status=RED reason=textVs_changed');
    process.exit(1);
  }

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
  fs.writeFileSync(codePath, newCode);
  fs.writeFileSync(path.join(projects, 'yoyo-plant-place.tyb'), packPlant(newCode.subarray(0, placeLen)));
  fs.writeFileSync(path.join(projects, 'yoyo-plant-parse1.tyb'), packPlant(newCode.subarray(placeLen)));
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
  pin = setPin(pin, 'code_parse1_bytes', String(newCode.length - placeLen));
  pin = setPin(pin, 'code_plant_place_sha256', placeSha);
  pin = setPin(pin, 'code_plant_parse1_sha256', parse1Sha);
  pin = setPin(pin, 'yoyo_sembed_prefix_bytes', String(prefix.length));
  pin = setPin(pin, 'yoyo_semfrag', 'PRESENT');
  pin = setPin(pin, 'yoyo_semfrag_prefix_bytes', String(prefix.length));
  pin = setPin(pin, 'yoyo_semfrag_core_bytes', String(core.length));
  pin = setPin(pin, 'yoyo_semfrag_prefix_sot', 'yoyo-openloop-h00.fragment.ty');
  pin = setPin(pin, 'yoyo_semfrag_hex_cross', 'EQUAL');
  pin = setPin(pin, 'yoyo_semolp_node_emit_expects', 'ABSENT');
  pin = setPin(pin, 'yoyo_semgeom', 'PRESENT');
  pin = setPin(pin, 'yoyo_semgeom_pe_size', String(Lnew.peSize));
  pin = setPin(pin, 'yoyo_semgeom_code_raw', String(Lnew.codeRaw));
  pin = setPin(pin, 'note', 'SEMFRAG_fragment_ty_sot;place_mode_epilogue;not_Thompson');
  pin = setPin(pin, 'doc', 'AUTO-SEMFRAG.md');
  fs.writeFileSync(pinPath, pin);

  console.log('SEMFRAG core_bytes=' + core.length);
  console.log('SEMFRAG prefix_bytes=' + prefix.length);
  console.log('SEMFRAG code_bytes=' + newCode.length);
  console.log('SEMFRAG prefix_sot=yoyo-openloop-h00.fragment.ty');
  console.log('SEMFRAG hex_cross=EQUAL');
  console.log('SEMFRAG node_emit_expects=ABSENT');
  console.log('SEMFRAG status=GREEN');
}

main();
