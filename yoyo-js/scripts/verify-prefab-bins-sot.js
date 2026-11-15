'use strict';
/**
 * AUTO-TRUER11: content prefab SoT = bins minus YOYO-derived compact;
 * stub must match rebuild from bins; field-match / h00 markers absent;
 * data capped after compact3 (compact bytes present but compact_sot=yoyo_ff_tyb).
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { linkPeWin32 } = require('../src/platform/pe-builder');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const stubPath = path.join(root, 'scripts', 'fixtures', 'yoyo_place_stub.exe');
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

function main() {
  const pin = readPin(pinPath);
  const code = fs.readFileSync(path.join(payload, 'code.bin'));
  const data = fs.readFileSync(path.join(payload, 'data.bin'));
  const stub = fs.readFileSync(stubPath);

  console.log(`PREFAB data_bytes=${data.length} code_bytes=${code.length} stub_bytes=${stub.length}`);
  console.log(`PREFAB bins_total=${code.length + data.length}`);

  const prefabOk =
    pin.prefab_sot === 'bins' ||
    pin.prefab_sot === 'bins_minus_compact' ||
    pin.prefab_sot === 'code_plus_seeds' ||
    pin.prefab_sot === 'yoyo_plant_tyb_plus_seeds';
  if (!prefabOk) {
    console.log('PREFAB status=RED reason=prefab_sot');
    process.exit(1);
  }
  if (pin.field_match_bake !== 'EXCISED') {
    console.log('PREFAB status=RED reason=fm_not_excised');
    process.exit(1);
  }
  if (data.indexOf(Buffer.from('yoyo_h00_')) >= 0) {
    console.log('PREFAB status=RED reason=h00_mark');
    process.exit(1);
  }
  const c3 = data.indexOf(Buffer.from('yoyo_compact3_c\0'));
  if (c3 < 0) {
    console.log('PREFAB status=RED reason=missing_compact3');
    process.exit(1);
  }
  const expectEnd = c3 + 'yoyo_compact3_c\0'.length;
  if (data.length !== expectEnd) {
    console.log(`PREFAB status=RED reason=tail_after_compact3 got=${data.length} want=${expectEnd}`);
    process.exit(1);
  }

  // stub = derived from bins (same recipe as build-place-stub.js)
  const PARSE1_OFF = 4491;
  const DATA_PREFIX = 0x800;
  const DATA_NEED = 0x2000;
  const rebuilt = linkPeWin32(code.subarray(0, PARSE1_OFF), data.subarray(0, DATA_PREFIX), [], DATA_NEED);
  const a = crypto.createHash('sha256').update(stub).digest('hex');
  const b = crypto.createHash('sha256').update(rebuilt).digest('hex');
  console.log(`PREFAB stub_sha=${a.slice(0, 16)} rebuilt_sha=${b.slice(0, 16)}`);
  if (a !== b || stub.length !== rebuilt.length) {
    console.log('PREFAB status=RED reason=stub_not_derived_from_bins');
    process.exit(1);
  }

  console.log(`PREFAB prefab_sot=${pin.prefab_sot}`);
  console.log('PREFAB stub_role=derived_from_bins');
  console.log('PREFAB field_match_bake=EXCISED');
  if (pin.compact_sot) console.log(`PREFAB compact_sot=${pin.compact_sot}`);
  if (pin.code_lineage) console.log(`PREFAB code_lineage=${pin.code_lineage}`);
  console.log('PREFAB status=GREEN');
  console.log('PREFAB doc=AUTO-TRUER11.md');
  console.log('PREFAB note=compact_yoyo_derived;code_plant_still_prefab;not_Thompson');
  process.exit(0);
}

main();
