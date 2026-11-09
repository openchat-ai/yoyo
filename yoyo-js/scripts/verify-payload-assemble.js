'use strict';
/**
 * AUTO-TRUER12: data.bin must EQUAL algorithmic assemble from seeds + YOYO compact.
 * Prefab content SoT = code.bin + ret/setret seed PEs (not the assembled data blob).
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const {
  buildCompilerDataPrelude,
  expandRetPeTextRaw20k,
  S3_PARSE1_RET_PE_LEN,
} = require('../src/platform/yoyo-compiler-emit-prelude');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const seeds = path.join(payload, 'seeds');
const pinPath = path.join(payload, 'payload.pin');

const COMPACT_MARK = Buffer.from('yoyo_compact3_c\0');
const COMPACT_LEN = 9728;
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

function sha16(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex').slice(0, 16);
}

function main() {
  const pin = readPin(pinPath);
  const data = fs.readFileSync(path.join(payload, 'data.bin'));
  const code = fs.readFileSync(path.join(payload, 'code.bin'));
  const retSeed = fs.readFileSync(path.join(seeds, 's3_compile_pe_ret.pe'));
  const setretSeed = fs.readFileSync(path.join(seeds, 's3_compile_pe_setret.pe'));

  const assembled = buildCompilerDataPrelude({});
  if (!assembled.data.equals(data)) {
    console.log('ASM status=RED reason=data_not_assembled_equal');
    process.exit(1);
  }
  console.log(`ASM data_bytes=${data.length} assemble_EQUAL=1 sha=${sha16(data)}`);

  const mz = [];
  for (let i = 0; i < data.length - 1; i++) {
    if (data[i] === 0x4d && data[i + 1] === 0x5a) mz.push(i);
  }
  if (mz.length < 3) {
    console.log(`ASM status=RED reason=mz_count got=${mz.length}`);
    process.exit(1);
  }
  const retExp = data.subarray(mz[0], mz[0] + S3_PARSE1_RET_PE_LEN);
  const derRet = expandRetPeTextRaw20k(retSeed);
  if (!derRet.equals(retExp)) {
    console.log('ASM status=RED reason=ret_expand_DIFF');
    process.exit(1);
  }
  console.log(`ASM ret_pe_off=${mz[0]} ret_expand_EQUAL=1 seed_bytes=${retSeed.length}`);

  const setIn = data.subarray(mz[1], mz[1] + setretSeed.length);
  if (!setIn.equals(setretSeed)) {
    console.log('ASM status=RED reason=setret_seed_DIFF');
    process.exit(1);
  }
  console.log(`ASM setret_off=${mz[1]} setret_seed_EQUAL=1`);

  const mark = data.indexOf(COMPACT_MARK);
  const compact = data.subarray(mark - COMPACT_LEN, mark);
  if (mark < 0 || compact.length !== COMPACT_LEN) {
    console.log('ASM status=RED reason=compact_geometry');
    process.exit(1);
  }
  console.log(`ASM compact_off=${mark - COMPACT_LEN} compact_sot=${pin.compact_sot || 'ABSENT'}`);

  if (pin.ret_pe_sot !== 'seed_expand') {
    console.log('ASM status=RED reason=pin_ret_pe_sot');
    process.exit(1);
  }
  if (pin.setret_sot !== 'seed_bytes') {
    console.log('ASM status=RED reason=pin_setret_sot');
    process.exit(1);
  }
  if (pin.data_sot !== 'assembled') {
    console.log('ASM status=RED reason=pin_data_sot');
    process.exit(1);
  }
  if (pin.prefab_sot !== 'code_plus_seeds') {
    console.log('ASM status=RED reason=pin_prefab_sot');
    process.exit(1);
  }
  if (pin.compact_sot !== 'yoyo_ff_tyb') {
    console.log('ASM status=RED reason=pin_compact_sot');
    process.exit(1);
  }

  const prefabBytes = code.length + retSeed.length + setretSeed.length;
  const parse1Bytes = Math.max(0, code.length - PARSE1_OFF);
  console.log(`ASM prefab_bytes=${prefabBytes} code=${code.length} ret_seed=${retSeed.length} setret_seed=${setretSeed.length}`);
  console.log(`ASM code_place_bytes=${PARSE1_OFF} code_parse1_prefab_bytes=${parse1Bytes}`);
  console.log('ASM data_sot=assembled');
  console.log('ASM ret_pe_sot=seed_expand');
  console.log('ASM setret_sot=seed_bytes');
  console.log('ASM prefab_sot=code_plus_seeds');
  console.log('ASM status=GREEN');
  console.log('ASM doc=AUTO-TRUER12.md');
  console.log('ASM note=data_assembled;parse1_plant_still_prefab;not_Thompson');
  process.exit(0);
}

main();
