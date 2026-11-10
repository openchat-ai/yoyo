'use strict';
/**
 * AUTO-TRUER14: code.bin SoT = YOYO plant tyb halves (A1×3 + FF).
 * Unpack EQUAL; optional C1 regen checked by stage script.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const placeTyb = path.join(root, 'yoyo', 'projects', 'yoyo-plant-place.tyb');
const parse1Tyb = path.join(root, 'yoyo', 'projects', 'yoyo-plant-parse1.tyb');
const fullTyb = path.join(root, 'yoyo', 'projects', 'yoyo-plant.tyb');
const pinPath = path.join(payload, 'payload.pin');
const PARSE1_OFF = 4491;

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

function unpackPlantTyb(buf) {
  if (buf.length < 8 || buf.toString('ascii', 0, 3) !== 'TYB') {
    throw new Error('bad plant tyb magic');
  }
  const n = buf.readUInt16LE(6);
  const out = [];
  for (let i = 0; i < n; i++) {
    const o = 8 + i * 8;
    if (o + 8 > buf.length) throw new Error('plant tyb truncated');
    const op = buf[o];
    const argc = buf[o + 1];
    if (op === 0xff) break;
    if (op === 0xa0) {
      if (argc >= 1) out.push(buf[o + 2]);
    } else if (op === 0xa1) {
      if (argc >= 1) out.push(buf[o + 2]);
      if (argc >= 2) out.push(buf[o + 4]);
      if (argc >= 3) out.push(buf[o + 6]);
    } else {
      throw new Error(`unexpected plant op 0x${op.toString(16)} at rec ${i}`);
    }
  }
  return Buffer.from(out);
}

function sha16(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex').slice(0, 16);
}

function main() {
  const pin = readPin(pinPath);
  const code = fs.readFileSync(path.join(payload, 'code.bin'));
  const placeBuf = fs.readFileSync(placeTyb);
  const parse1Buf = fs.readFileSync(parse1Tyb);
  const fullBuf = fs.readFileSync(fullTyb);

  const place = unpackPlantTyb(placeBuf);
  const parse1 = unpackPlantTyb(parse1Buf);
  const full = unpackPlantTyb(fullBuf);
  const joined = Buffer.concat([place, parse1]);

  console.log(`Y14 place_tyb_bytes=${placeBuf.length} place_bytes=${place.length}`);
  console.log(`Y14 parse1_tyb_bytes=${parse1Buf.length} parse1_bytes=${parse1.length}`);
  console.log(`Y14 full_tyb_bytes=${fullBuf.length} full_bytes=${full.length}`);
  console.log(`Y14 code_bytes=${code.length} code_sha=${sha16(code)}`);

  if (place.length !== PARSE1_OFF) {
    console.log(`Y14 status=RED reason=place_len got=${place.length} want=${PARSE1_OFF}`);
    process.exit(1);
  }
  if (!joined.equals(code)) {
    console.log('Y14 status=RED reason=halves_unpack_ne_code');
    process.exit(1);
  }
  if (!full.equals(code)) {
    console.log('Y14 status=RED reason=full_unpack_ne_code');
    process.exit(1);
  }
  if (pin.code_sot !== 'yoyo_plant_tyb_halves') {
    console.log('Y14 status=RED reason=pin_code_sot');
    process.exit(1);
  }
  if (pin.prefab_sot !== 'yoyo_plant_tyb_plus_seeds') {
    console.log('Y14 status=RED reason=pin_prefab_sot');
    process.exit(1);
  }

  console.log('Y14 code_sot=yoyo_plant_tyb_halves');
  console.log('Y14 prefab_sot=yoyo_plant_tyb_plus_seeds');
  console.log('Y14 unpack_EQUAL=1');
  console.log('Y14 status=GREEN');
  console.log('Y14 doc=AUTO-TRUER14.md');
  console.log('Y14 note=plant_yoyo_ir_owned;not_semantic_rewrite;not_Thompson');
  process.exit(0);
}

main();
