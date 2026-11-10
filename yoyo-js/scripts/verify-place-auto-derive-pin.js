'use strict';
/** AUTO-TRUER20: place auto-derives bins when checked-in payload absent. */
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
  if (pin.place_auto_derive_on_absent !== 'PRESENT') {
    console.log('Y20 status=RED reason=pin_place_auto_derive_on_absent');
    process.exit(1);
  }
  if (pin.place_bins_input !== 'plant_derive_plus_assemble') {
    console.log('Y20 status=RED reason=pin_place_bins_input');
    process.exit(1);
  }
  console.log('Y20 place_auto_derive_on_absent=PRESENT');
  console.log('Y20 place_bins_input=plant_derive_plus_assemble');
  console.log('Y20 status=GREEN');
}

main();
