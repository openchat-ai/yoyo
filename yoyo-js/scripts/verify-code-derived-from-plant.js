'use strict';
/**
 * AUTO-TRUER18: code.bin is derived cache of YOYO plant tyb halves.
 * Authority = plant tyb hashes; checked-in code.bin must EQUAL derive.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { spawnSync } = require('child_process');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const pinPath = path.join(payload, 'payload.pin');
const codePath = path.join(payload, 'code.bin');
const placeTyb = path.join(root, 'yoyo', 'projects', 'yoyo-plant-place.tyb');
const parse1Tyb = path.join(root, 'yoyo', 'projects', 'yoyo-plant-parse1.tyb');

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

function sha256file(p) {
  return crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
}

function main() {
  const pin = readPin(pinPath);
  if (pin.code_authority !== 'yoyo_plant_tyb_halves') {
    console.log('Y18 status=RED reason=pin_code_authority');
    process.exit(1);
  }
  if (pin.code_bin_role !== 'derived_from_plant_tyb') {
    console.log('Y18 status=RED reason=pin_code_bin_role');
    process.exit(1);
  }

  const placeSha = sha256file(placeTyb);
  const parse1Sha = sha256file(parse1Tyb);
  if ((pin.code_plant_place_sha256 || '').toLowerCase() !== placeSha) {
    console.log('Y18 status=RED reason=place_tyb_sha_DIFF');
    process.exit(1);
  }
  if ((pin.code_plant_parse1_sha256 || '').toLowerCase() !== parse1Sha) {
    console.log('Y18 status=RED reason=parse1_tyb_sha_DIFF');
    process.exit(1);
  }

  const tmp = path.join(root, 'scripts', '_stage-truer18', '_derive_check_code.bin');
  fs.mkdirSync(path.dirname(tmp), { recursive: true });
  const r = spawnSync(
    process.execPath,
    [path.join(root, 'yoyo-js', 'scripts', 'derive-code-from-plant-tyb.js'), tmp],
    { encoding: 'utf8' },
  );
  if (r.status !== 0) {
    console.log('Y18 status=RED reason=derive_failed');
    console.log(r.stdout || '');
    console.log(r.stderr || '');
    process.exit(1);
  }
  const derived = fs.readFileSync(tmp);
  const checked = fs.readFileSync(codePath);
  if (!derived.equals(checked)) {
    console.log('Y18 status=RED reason=checked_code_ne_plant_derive');
    process.exit(1);
  }
  const codeSha = crypto.createHash('sha256').update(checked).digest('hex');
  if ((pin.code_sha256 || '').toLowerCase() !== codeSha) {
    console.log('Y18 status=RED reason=code_sha_pin_DIFF');
    process.exit(1);
  }

  console.log(`Y18 place_tyb_sha=${placeSha.slice(0, 16)}`);
  console.log(`Y18 parse1_tyb_sha=${parse1Sha.slice(0, 16)}`);
  console.log(`Y18 code_bytes=${checked.length} derive_EQUAL=1`);
  console.log('Y18 code_authority=yoyo_plant_tyb_halves');
  console.log('Y18 code_bin_role=derived_from_plant_tyb');
  console.log('Y18 status=GREEN');
  console.log('Y18 note=place_input_from_plant_derive;code_bin_is_cache;not_Thompson');
  process.exit(0);
}

main();
