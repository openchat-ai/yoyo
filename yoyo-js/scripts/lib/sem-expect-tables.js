'use strict';
/** Shared parse1 sem expect tables (TRUER15–17 / SEMFULL / SEMHOST). */
const fs = require('fs');
const path = require('path');

const projects = path.join(__dirname, '..', '..', '..', 'yoyo', 'projects');

function loadExp(name) {
  return Buffer.from(
    fs
      .readFileSync(path.join(projects, 'yoyo-sem-' + name + '.expect.hex'), 'utf8')
      .trim()
      .replace(/\s+/g, ''),
    'hex',
  );
}

const OP_SLICES = [
  { op: 0x00, name: 'nop' },
  { op: 0x30, name: 'set' },
  { op: 0x60, name: 'get' },
  { op: 0x61, name: 'sub' },
  { op: 0x62, name: 'add' },
  { op: 0x63, name: 'imul' },
  { op: 0x64, name: 'mov' },
  { op: 0x65, name: 'cmp' },
  { op: 0x66, name: 'inc' },
  { op: 0x67, name: 'dec' },
  { op: 0x68, name: 'addv' },
  { op: 0x69, name: 'orv' },
  { op: 0x6a, name: 'subv' },
];

function loadAllExpects() {
  const EXP = { ret: loadExp('ret') };
  for (const s of OP_SLICES) EXP[s.name] = loadExp(s.name);
  return EXP;
}

module.exports = { OP_SLICES, loadExp, loadAllExpects, projects };
