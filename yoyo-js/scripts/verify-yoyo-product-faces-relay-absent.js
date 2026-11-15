'use strict';
/**
 * AUTO-TRUER36: product hosts all open-loop faces but cannot relay-compile.
 */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');
const pinPath = path.join(
  root,
  'yoyo-js',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'payload.pin',
);
const relayDir =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer36', 'relay');

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
  if (pin.yoyo_product_open_loop_faces !== 'PRESENT') {
    console.log('Y36 status=RED reason=pin_faces');
    process.exit(1);
  }
  if (pin.yoyo_product_open_loop !== 'ABSENT') {
    console.log('Y36 status=RED reason=pin_open_loop');
    process.exit(1);
  }
  if (pin.yoyo_product_compiler_relay !== 'ABSENT') {
    console.log('Y36 status=RED reason=pin_relay');
    process.exit(1);
  }
  const out = path.join(relayDir, 'output.exe');
  if (fs.existsSync(out)) {
    console.log('Y36 status=RED reason=unexpected_relay_output');
    process.exit(1);
  }
  console.log('Y36 yoyo_product_open_loop_faces=PRESENT');
  console.log('Y36 yoyo_product_open_loop=ABSENT');
  console.log('Y36 yoyo_product_compiler_relay=ABSENT');
  console.log('Y36 status=GREEN');
}

main();
