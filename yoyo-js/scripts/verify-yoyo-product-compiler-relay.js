'use strict';
/** AUTO-TRUER26: product hosts io paths but cannot relay-compile (no output.exe). */
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
  if (pin.yoyo_product_compiler_io_paths !== 'PRESENT') {
    console.log('Y26 status=RED reason=pin_io_paths_not_present');
    process.exit(1);
  }
  if (pin.yoyo_product_compiler_relay !== 'ABSENT') {
    console.log('Y26 status=RED reason=pin_relay');
    process.exit(1);
  }
  const relayDir =
    process.argv[2] ||
    path.join(root, 'scripts', '_stage-truer26', 'relay');
  const out = path.join(relayDir, 'output.exe');
  if (fs.existsSync(out)) {
    console.log('Y26 status=RED reason=relay_produced_output');
    process.exit(1);
  }
  const marker = path.join(relayDir, '_relay_attempted');
  if (!fs.existsSync(marker)) {
    console.log('Y26 status=RED reason=missing_relay_attempt_marker');
    process.exit(1);
  }
  console.log('Y26 yoyo_product_compiler_io_paths=PRESENT');
  console.log('Y26 yoyo_product_compiler_relay=ABSENT');
  console.log('Y26 status=GREEN');
}

main();
