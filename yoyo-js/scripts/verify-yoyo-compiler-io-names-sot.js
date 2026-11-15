'use strict';
/**
 * AUTO-TRUER23: compiler I/O name table SoT = YOYO A1 tyb (not JS literals).
 * Also: C1/data PRESENT input.tyb+output.exe; product ABSENT those paths.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const {
  loadCompilerIoNames,
  COMPILER_IO_NAMES_TYB,
  buildCompilerDataPrelude,
} = require('../src/platform/yoyo-compiler-emit-prelude');

const root = path.join(__dirname, '..', '..');
const pinPath = path.join(
  root,
  'yoyo-js',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'payload.pin',
);
const dataPath = path.join(
  root,
  'yoyo-js',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'data.bin',
);
const c1Path =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer23', 'place-gate', 'yoyo_compiler.exe');
const productPath =
  process.argv[3] ||
  path.join(root, 'scripts', '_stage-truer23', 'product', 'output.exe');

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
  if (pin.compiler_io_names_sot !== 'yoyo_compiler_io_names_tyb') {
    console.log('Y23 status=RED reason=pin_compiler_io_names_sot');
    process.exit(1);
  }
  // TRUER25+: full yoyo.tyb product hosts io paths (was ABSENT through TRUER24).
  if (
    pin.yoyo_product_compiler_io_paths !== 'PRESENT' &&
    pin.yoyo_product_compiler_io_paths !== 'ABSENT'
  ) {
    console.log('Y23 status=RED reason=pin_product_io');
    process.exit(1);
  }
  if (!fs.existsSync(COMPILER_IO_NAMES_TYB)) {
    console.log('Y23 status=RED reason=missing_io_names_tyb');
    process.exit(1);
  }
  const tyb = fs.readFileSync(COMPILER_IO_NAMES_TYB);
  const sha = crypto.createHash('sha256').update(tyb).digest('hex');
  if (pin.compiler_io_names_tyb_sha256 && pin.compiler_io_names_tyb_sha256 !== sha) {
    console.log(
      'Y23 status=RED reason=tyb_sha_drift got=' + sha.slice(0, 16),
    );
    process.exit(1);
  }
  const { raw, names } = loadCompilerIoNames();
  if (raw.length !== 189 || names.length !== 16) {
    console.log(
      'Y23 status=RED reason=io_names_shape raw=' +
        raw.length +
        ' names=' +
        names.length,
    );
    process.exit(1);
  }
  if (names[0] !== 'input.tyb\0' || names[1] !== 'output.exe\0') {
    console.log('Y23 status=RED reason=io_names_head');
    process.exit(1);
  }
  const data = fs.readFileSync(dataPath);
  const assembled = buildCompilerDataPrelude({});
  if (!assembled.data.equals(data)) {
    console.log('Y23 status=RED reason=assemble_not_equal');
    process.exit(1);
  }
  if (data.indexOf(Buffer.from('input.tyb\0')) !== 0) {
    console.log('Y23 status=RED reason=data_missing_input_tyb');
    process.exit(1);
  }
  if (data.indexOf(Buffer.from('output.exe\0')) !== 10) {
    console.log('Y23 status=RED reason=data_missing_output_exe');
    process.exit(1);
  }
  if (!fs.existsSync(c1Path)) {
    console.log('Y23 status=RED reason=missing_c1 path=' + c1Path);
    process.exit(1);
  }
  const c1 = fs.readFileSync(c1Path);
  if (c1.indexOf(Buffer.from('input.tyb\0')) < 0) {
    console.log('Y23 status=RED reason=c1_missing_input_tyb');
    process.exit(1);
  }
  if (c1.indexOf(Buffer.from('output.exe\0')) < 0) {
    console.log('Y23 status=RED reason=c1_missing_output_exe');
    process.exit(1);
  }
  if (!fs.existsSync(productPath)) {
    console.log('Y23 status=RED reason=missing_product');
    process.exit(1);
  }
  const prod = fs.readFileSync(productPath);
  const prodHas =
    prod.indexOf(Buffer.from('input.tyb')) >= 0 &&
    prod.indexOf(Buffer.from('output.exe')) >= 0;
  if (pin.yoyo_product_compiler_io_paths === 'PRESENT' && !prodHas) {
    console.log('Y23 status=RED reason=product_missing_io_paths');
    process.exit(1);
  }
  if (pin.yoyo_product_compiler_io_paths === 'ABSENT' && prodHas) {
    console.log('Y23 status=RED reason=product_unexpected_io_paths');
    process.exit(1);
  }
  console.log('Y23 compiler_io_names_sot=yoyo_compiler_io_names_tyb');
  console.log('Y23 compiler_io_names_tyb_sha256_prefix=' + sha.slice(0, 16));
  console.log('Y23 io_names_bytes=189 names=16');
  console.log('Y23 c1_io_paths=PRESENT');
  console.log(
    'Y23 yoyo_product_compiler_io_paths=' + pin.yoyo_product_compiler_io_paths,
  );
  console.log('Y23 assemble_EQUAL=1');
  console.log('Y23 status=GREEN');
}

main();
