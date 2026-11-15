'use strict';
/** AUTO-TRUER21: pin that YOYO product hosts semantic emit faces. */
const fs = require('fs');
const path = require('path');

const pinPath = path.join(
  __dirname,
  '..',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'payload.pin',
);

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

function main() {
  const pin = readPin(pinPath);
  if (pin.yoyo_product_hosts_sem_emit_faces !== 'PRESENT') {
    console.log('Y21 status=RED reason=pin_yoyo_product_hosts_sem_emit_faces');
    process.exit(1);
  }
  if (pin.parse1_sem_batch !== '3') {
    console.log('Y21 status=RED reason=pin_parse1_sem_batch');
    process.exit(1);
  }
  console.log('Y21 yoyo_product_hosts_sem_emit_faces=PRESENT');
  console.log('Y21 parse1_sem_batch=3');
  console.log('Y21 status=GREEN');
}

main();
