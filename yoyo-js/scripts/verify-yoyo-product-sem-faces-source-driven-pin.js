'use strict';
/** AUTO-TRUER22: pin product sem faces source-driven. */
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
    console.log('Y22 status=RED reason=pin_hosts_faces');
    process.exit(1);
  }
  if (pin.yoyo_product_sem_faces_source_driven !== 'PRESENT') {
    console.log('Y22 status=RED reason=pin_source_driven');
    process.exit(1);
  }
  if (pin.yoyo_product_sem_face_mut_pat !== '6302500051000000') {
    console.log('Y22 status=RED reason=pin_mut_pat');
    process.exit(1);
  }
  if (pin.yoyo_product_sem_face_mut_off !== '1736') {
    console.log('Y22 status=RED reason=pin_mut_off');
    process.exit(1);
  }
  if (pin.yoyo_product_sem_face_mut_from !== '63') {
    console.log('Y22 status=RED reason=pin_mut_from');
    process.exit(1);
  }
  if (pin.yoyo_product_sem_face_mut_to !== '6a') {
    console.log('Y22 status=RED reason=pin_mut_to');
    process.exit(1);
  }
  if (pin.yoyo_product_sem_face_mut_miss !== 'yoyo-sem-imul') {
    console.log('Y22 status=RED reason=pin_mut_miss');
    process.exit(1);
  }
  console.log('Y22 yoyo_product_sem_faces_source_driven=PRESENT');
  console.log('Y22 status=GREEN');
}

main();
