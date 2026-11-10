'use strict';
/**
 * AUTO-TRUER18: derive code.bin bytes from YOYO plant tyb halves (A1×3+FF).
 * Usage:
 *   node derive-code-from-plant-tyb.js [out-code.bin]
 * Default out: stdout is hex length/sha only; with path, writes file.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const placeTyb = path.join(root, 'yoyo', 'projects', 'yoyo-plant-place.tyb');
const parse1Tyb = path.join(root, 'yoyo', 'projects', 'yoyo-plant-parse1.tyb');
const PARSE1_OFF = 4491;

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

function main() {
  const place = unpackPlantTyb(fs.readFileSync(placeTyb));
  const parse1 = unpackPlantTyb(fs.readFileSync(parse1Tyb));
  if (place.length !== PARSE1_OFF) {
    console.log(`DERIVE status=RED reason=place_len got=${place.length}`);
    process.exit(1);
  }
  const code = Buffer.concat([place, parse1]);
  const sha = crypto.createHash('sha256').update(code).digest('hex');
  const out = process.argv[2];
  if (out) {
    fs.mkdirSync(path.dirname(path.resolve(out)), { recursive: true });
    fs.writeFileSync(out, code);
  }
  console.log(`DERIVE code_bytes=${code.length} sha256_prefix=${sha.slice(0, 16)}`);
  console.log('DERIVE code_authority=yoyo_plant_tyb_halves');
  console.log('DERIVE status=GREEN');
  process.exit(0);
}

main();
