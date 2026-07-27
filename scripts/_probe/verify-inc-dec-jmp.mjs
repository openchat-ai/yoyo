#!/usr/bin/env node
/**
 * verify-inc-dec-jmp.mjs — VerifyBeforeClaim probe (transient, do NOT commit).
 * Probes JS-side encodeOp for INC (0x66) / DEC (0x67) / JMP (0x70).
 * No edits to yoyo.js / encode-x64.js / goldens / locks.
 */
'use strict';
import { createRequire } from 'node:module';
const require = createRequire(import.meta.url);
import path from 'node:path';
import { fileURLToPath } from 'node:url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const root = path.resolve(__dirname, '..', '..');
const { encodeOp } = require(path.join(root, 'yoyo-js/src/platform/encode-x64'));

function hex(b) {
  return Buffer.from(b).toString('hex').match(/.{1,2}/g).join(' ');
}

// Expected (per prior scan):
//   0x66 INC [0x50]  -> 48 8B 87 80 02 00 00 48 FF C0 49 89 87 80 02 00 00 C3
//   0x67 DEC [0x50]  -> 48 8B 87 80 02 00 00 48 FF C8 49 89 87 80 02 00 00 C3
//   0x70 JMP rel32   -> initial emit = E9 00 00 00 00 (rel32 patched by compile())
const cases = [
  { name: '0x66 INC [0x50]', op: 0x66, args: [0x50], branch: false,
    expect: '49 8b 87 80 02 00 00 48 ff c0 49 89 87 80 02 00 00' },
  { name: '0x67 DEC [0x50]', op: 0x67, args: [0x50], branch: false,
    expect: '49 8b 87 80 02 00 00 48 ff c8 49 89 87 80 02 00 00' },
  { name: '0x70 JMP rel32 (placeholder)', op: 0x70, args: [0x00], branch: true,
    expect: 'e9 00 00 00 00' },
];

let allPass = true;
const rows = [];
for (const c of cases) {
  const got = encodeOp(c.op, c.args, c.branch);
  const gotHex = hex(got).toLowerCase();
  const pass = gotHex === c.expect;
  if (!pass) allPass = false;
  rows.push({ name: c.name, op: c.op.toString(16), pass, got: gotHex, expect: c.expect });
}

console.log('=== VerifyBeforeClaim: JS encodeOp for INC/DEC/JMP ===');
for (const r of rows) {
  console.log(`${r.pass ? 'PASS' : 'FAIL'}  op=0x${r.op}  ${r.name}`);
  console.log(`        got   : ${r.got}`);
  console.log(`        expect: ${r.expect}`);
}
console.log(`=== ${allPass ? 'ALL PASS' : 'SOME FAIL'} ===`);
process.exit(allPass ? 0 : 1);