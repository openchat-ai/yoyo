'use strict';
/**
 * AUTO-TYBSELF: pack data.bin into YOYO plant tyb chunks (A1×3 + FF).
 * Usage: node pack-plant-data-tyb.js
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const dataPath = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'data.bin');
const projects = path.join(root, 'yoyo', 'projects');
const N = 4;

function packPlant(bytes) {
  const recs = [];
  let i = 0;
  while (i < bytes.length) {
    const left = bytes.length - i;
    const b = Buffer.alloc(8);
    if (left >= 3) {
      b[0] = 0xa1;
      b[1] = 3;
      b[2] = bytes[i];
      b[4] = bytes[i + 1];
      b[6] = bytes[i + 2];
      i += 3;
    } else {
      b[0] = 0xa0;
      b[1] = 1;
      b[2] = bytes[i];
      i += 1;
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

function main() {
  const data = fs.readFileSync(dataPath);
  const chunk = Math.ceil(data.length / N);
  for (let h = 0; h < N; h++) {
    const part = data.subarray(h * chunk, Math.min(data.length, (h + 1) * chunk));
    const tyb = packPlant(part);
    const base = `yoyo-plant-data${h}`;
    fs.writeFileSync(path.join(projects, `${base}.tyb`), tyb);
    fs.writeFileSync(
      path.join(projects, `${base}.ty`),
      `# ${base} — AUTO-TYBSELF\n# data.bin chunk ${h}/${N} (${part.length}B) as A1×3 + FF.\n`,
    );
    const sha = crypto.createHash('sha256').update(tyb).digest('hex');
    console.log(
      `PACK_DATA chunk=${h} payload=${part.length} tyb=${tyb.length} sha256_prefix=${sha.slice(0, 16)}`,
    );
  }
  console.log(`PACK_DATA chunks=${N} data_bytes=${data.length} status=GREEN`);
  process.exit(0);
}

main();
