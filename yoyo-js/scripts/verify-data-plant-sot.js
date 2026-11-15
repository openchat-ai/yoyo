'use strict';
/**
 * TRUSTLIM3 / DATAPLANT: data.bin ≡ concat(unpack plant-data0..3).
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const projects = path.join(root, 'yoyo', 'projects');
const pinPath = path.join(payload, 'payload.pin');

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
      throw new Error(`unexpected plant op 0x${op.toString(16)}`);
    }
  }
  return Buffer.from(out);
}

function main() {
  const pin = readPin(pinPath);
  const data = fs.readFileSync(path.join(payload, 'data.bin'));
  const n = parseInt(pin.data_plant_chunks || '4', 10);
  const parts = [];
  for (let h = 0; h < n; h++) {
    const tybPath = path.join(projects, `yoyo-plant-data${h}.tyb`);
    const tyb = fs.readFileSync(tybPath);
    const part = unpackPlantTyb(tyb);
    const wantSha = pin[`data_plant_tyb${h}_sha256`];
    if (wantSha) {
      const got = crypto.createHash('sha256').update(tyb).digest('hex');
      if (got !== wantSha.toLowerCase()) {
        console.log(`DATAPLANT status=RED reason=tyb${h}_sha_DIFF`);
        process.exit(1);
      }
    }
    parts.push(part);
    console.log(`DATAPLANT chunk=${h} bytes=${part.length}`);
  }
  const joined = Buffer.concat(parts);
  if (!joined.equals(data)) {
    console.log(
      `DATAPLANT status=RED reason=unpack_ne_data got=${joined.length} want=${data.length}`,
    );
    process.exit(1);
  }
  console.log(
    `DATAPLANT unpack_EQUAL=1 data_sha=${crypto.createHash('sha256').update(data).digest('hex').slice(0, 16)}`,
  );
  console.log('DATAPLANT status=GREEN');
  console.log('DATAPLANT doc=AUTO-DATAPLANT.md');
  process.exit(0);
}

main();
