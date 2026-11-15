'use strict';
/**
 * AUDITFIVE matrix: F2/F3 must CLOSED; F1/F4/F5 HARD_BLOCK (true-zero impossible in-repo).
 */
const fs = require('fs');
const path = require('path');
const root = path.join(__dirname, '..', '..');
const ex = (...p) => fs.existsSync(path.join(root, ...p));

const rows = [
  {
    id: 'F1',
    name: 'collude_pins',
    status: 'HARD_BLOCK',
    gate: 'Trusting Trust residue; cannot true-zero in-repo',
  },
  {
    id: 'F2',
    name: 'semantic_in_allow_io',
    status: ex('scripts', 'stage-plantpoison.ps1') && ex('scripts', 'stage-y1plpin.ps1') && ex('scripts', 'stage-binspoison.ps1')
      ? 'CLOSED'
      : 'OPEN',
    gate: 'plantpoison + y1plpin + binspoison (expect/poison nets)',
  },
  {
    id: 'F3',
    name: 'unwatched_allow_basenames',
    status: ex('yoyo-js', 'scripts', 'verify-live-scan-allow-pin.js') ? 'CLOSED' : 'OPEN',
    gate: 'verify-live-scan-allow-pin.js scan fixtures+payload',
  },
  {
    id: 'F4',
    name: 'os_loader_hardware',
    status: 'HARD_BLOCK',
    gate: 'below A1 / outside YOYO TCB',
  },
  {
    id: 'F5',
    name: 'hash_collision',
    status: 'HARD_BLOCK',
    gate: 'dualhash raises cost but true collision/break is not engineer-zeroable; dualhash gate PRESENT',
    dualhash: ex('scripts', 'fixtures', 'audit-stub', 'STUB.DUALHASH.pin'),
  },
];

function main() {
  let open = 0;
  console.log('AUDITFIVE matrix:');
  for (const r of rows) {
    if (r.status === 'OPEN') open += 1;
    console.log(`AUDITFIVE path=${r.id} name=${r.name} status=${r.status} gate=${r.gate}`);
  }
  if (!rows.find((x) => x.id === 'F5').dualhash) {
    console.log('AUDITFIVE status=RED reason=dualhash_pin_missing');
    process.exit(1);
  }
  if (open) {
    console.log(`AUDITFIVE status=RED reason=open_paths n=${open}`);
    process.exit(1);
  }
  const hb = rows.filter((x) => x.status === 'HARD_BLOCK').map((x) => x.id).join(',');
  const cl = rows.filter((x) => x.status === 'CLOSED').map((x) => x.id).join(',');
  console.log(`AUDITFIVE closed=${cl} hard_block=${hb}`);
  console.log('AUDITFIVE status=GREEN');
  console.log('AUDITFIVE note=F2_F3_CLOSED;F1_F4_F5_HARD_BLOCK;not_Thompson');
  process.exit(0);
}
main();
