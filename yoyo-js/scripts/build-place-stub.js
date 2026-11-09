'use strict';
/**
 * AUTO-TRUER4: build place/emit-only stub PE from checked-in bins (no ISA plant).
 * Truncates code.bin before parse1 body; keeps string-table prefix of data.bin.
 *
 *   node yoyo-js/scripts/build-place-stub.js
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { linkPeWin32 } = require('../src/platform/pe-builder');

const root = path.join(__dirname, '..', '..');
const payload = path.join(__dirname, '..', 'src', 'platform', 'yoyo-compiler-payload');
const outDir = path.join(root, 'scripts', 'fixtures');

// parse1 label offset from last native emit (prefix includes place/emit).
const PARSE1_OFF = 4491;
const DATA_PREFIX = 0x800;
const DATA_NEED = 0x2000;

const codeFull = fs.readFileSync(path.join(payload, 'code.bin'));
const dataFull = fs.readFileSync(path.join(payload, 'data.bin'));
if (codeFull.length < PARSE1_OFF) {
  console.log('PLACE_STUB status=RED reason=code_too_short');
  process.exit(1);
}
const code = codeFull.slice(0, PARSE1_OFF);
const data = dataFull.slice(0, DATA_PREFIX);
const pe = linkPeWin32(code, data, [], DATA_NEED);

fs.mkdirSync(outDir, { recursive: true });
const outPe = path.join(outDir, 'yoyo_place_stub.exe');
fs.writeFileSync(outPe, pe);
const sha = crypto.createHash('sha256').update(pe).digest('hex');
const pin = [
  '# Cold-start executable SoT (AUTO-TRUER4). Place/emit only — no parse1 body.',
  '# Full compiler PE is derived: stub × bins → place.',
  '# ≠ Thompson.',
  'name=yoyo_place_stub.exe',
  `bytes=${pe.length}`,
  `sha256=${sha}`,
  `sha256_prefix=${sha.slice(0, 16)}`,
  'path=scripts/fixtures/yoyo_place_stub.exe',
  `code_trunc=${PARSE1_OFF}`,
  `data_prefix=${DATA_PREFIX}`,
  'disposition=BOOTSTRAP_FIXTURE',
  'trust_root=place_stub',
  'role=cold_start_executable_sot',
  'doc=AUTO-TRUER4.md',
  '',
].join('\n');
fs.writeFileSync(path.join(outDir, 'yoyo_place_stub.pin'), pin);

console.log(`PLACE_STUB bytes=${pe.length} sha256_prefix=${sha.slice(0, 16)}`);
console.log(`PLACE_STUB full_pe_bytes=${linkPeWin32(codeFull, dataFull, [], 0x38000).length}`);
console.log('PLACE_STUB node_isa_plant=ABSENT');
console.log('PLACE_STUB status=GREEN');
console.log('PLACE_STUB doc=AUTO-TRUER4.md');
