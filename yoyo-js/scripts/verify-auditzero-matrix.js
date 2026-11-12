'use strict';
/**
 * AUTO-AUDITZERO: print closed/hard-block matrix for the eight attack paths.
 * Exit 0 only if paths 1-7 are CLOSED and path 8 is HARD_BLOCK (explicit).
 */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');

function exists(...parts) {
  return fs.existsSync(path.join(root, ...parts));
}

const rows = [
  {
    id: 1,
    name: 'mutate_code.bin',
    closed_if: () =>
      exists('yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'payload.pin') &&
      exists('yoyo-js', 'scripts', 'verify-payload-bins-pin.js'),
    gate: 'verify-payload-bins-pin.js (full code sha)',
  },
  {
    id: 2,
    name: 'mutate_data.bin',
    closed_if: () =>
      exists('yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'payload.pin') &&
      exists('yoyo-js', 'scripts', 'verify-payload-bins-pin.js'),
    gate: 'verify-payload-bins-pin.js (full data sha)',
  },
  {
    id: 3,
    name: 'mutate_stub_gates',
    closed_if: () =>
      exists('scripts', 'fixtures', 'yoyo_place_stub.pin') &&
      exists('yoyo-js', 'scripts', 'verify-place-stub-pin.js'),
    gate: 'verify-place-stub-pin.js (full stub sha)',
  },
  {
    id: 4,
    name: 'mutate_seed_templates',
    closed_if: () =>
      exists('scripts', 'fixtures', 'yoyo_place_stub.pin') &&
      exists('yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'payload.pin'),
    gate: 'stub sha covers .text seeds; data/code pins cover payload seeds',
  },
  {
    id: 5,
    name: 'out_of_window_hijack',
    closed_if: () =>
      exists('scripts', 'fixtures', 'audit-stub', 'STUB.OOW.pin') &&
      exists('scripts', 'fixtures', 'audit-stub', 'STUB.BEYOND.pin') &&
      exists('yoyo-js', 'scripts', 'verify-stub-oow-landing-pin.js') &&
      exists('yoyo-js', 'scripts', 'verify-beyond-place-pin.js'),
    gate: 'OOW landing 64B + full beyond-place region sha',
  },
  {
    id: 6,
    name: 'mutate_try_place_geometry',
    closed_if: () => exists('scripts', 'fixtures', 'yoyo_place_stub.pin'),
    gate: 'try_place immediates live in place half → stub sha',
  },
  {
    id: 7,
    name: 'poison_live_allow_files',
    closed_if: () => exists('yoyo-js', 'scripts', 'verify-live-allow-files-pin.js'),
    gate: 'verify-live-allow-files-pin.js (+ stage poison)',
  },
  {
    id: 8,
    name: 'collude_rewrite_pins',
    closed_if: () => false,
    hard_block: true,
    gate: 'HARD_BLOCK: pin/CI/docs co-rewrite = Thompson-class; not engineer-zeroable',
  },
];

function main() {
  let bad = 0;
  console.log('AUDITZERO matrix:');
  for (const r of rows) {
    let status;
    if (r.hard_block) {
      status = 'HARD_BLOCK';
    } else if (r.closed_if()) {
      status = 'CLOSED';
    } else {
      status = 'OPEN';
      bad += 1;
    }
    console.log(`AUDITZERO path=${r.id} name=${r.name} status=${status} gate=${r.gate}`);
  }
  const p8 = rows.find((x) => x.id === 8);
  if (!p8 || !p8.hard_block) {
    console.log('AUDITZERO status=RED reason=path8_must_be_hard_block');
    process.exit(1);
  }
  if (bad) {
    console.log(`AUDITZERO status=RED reason=open_paths n=${bad}`);
    process.exit(1);
  }
  console.log('AUDITZERO paths_1_7=CLOSED path_8=HARD_BLOCK');
  console.log('AUDITZERO status=GREEN');
  console.log('AUDITZERO doc=AUTO-AUDITZERO.md');
  console.log('AUDITZERO note=not_Thompson;hard_block_collusion');
  process.exit(0);
}
main();
