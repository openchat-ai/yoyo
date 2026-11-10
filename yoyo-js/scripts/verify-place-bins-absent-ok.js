'use strict';
/**
 * AUTO-TRUER19: pin claims tip place does not require checked-in code.bin/data.bin.
 */
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
  if (pin.code_bin_required_on_place !== 'ABSENT') {
    console.log('Y19 status=RED reason=pin_code_bin_required_on_place');
    process.exit(1);
  }
  if (pin.data_bin_required_on_place !== 'ABSENT') {
    console.log('Y19 status=RED reason=pin_data_bin_required_on_place');
    process.exit(1);
  }
  if (pin.place_bins_input !== 'plant_derive_plus_assemble') {
    console.log('Y19 status=RED reason=pin_place_bins_input');
    process.exit(1);
  }
  if (pin.code_authority !== 'yoyo_plant_tyb_halves') {
    console.log('Y19 status=RED reason=pin_code_authority');
    process.exit(1);
  }
  if (pin.data_sot !== 'assembled') {
    console.log('Y19 status=RED reason=pin_data_sot');
    process.exit(1);
  }
  console.log('Y19 code_bin_required_on_place=ABSENT');
  console.log('Y19 data_bin_required_on_place=ABSENT');
  console.log('Y19 place_bins_input=plant_derive_plus_assemble');
  console.log('Y19 status=GREEN');
}

main();
