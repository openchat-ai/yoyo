'use strict';
/**
 * AUTO-TRUER15/16/17: C1 × semantic parse1 slices → .text contains locked expect.hex.
 * Usage: node verify-yoyo-semantic-c1-regen.js [_stage-truer17]
 */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');
const stageName = process.argv[2] || '_stage-truer17';
const stage = path.join(root, 'scripts', stageName);

const BATCH1 = [
  'yoyo-sem-ret',
  'yoyo-sem-nop',
  'yoyo-sem-sub',
  'yoyo-sem-add',
  'yoyo-sem-get',
];
const BATCH2 = [
  'yoyo-sem-set',
  'yoyo-sem-mov',
  'yoyo-sem-cmp',
  'yoyo-sem-inc',
  'yoyo-sem-dec',
];
const BATCH3 = [
  'yoyo-sem-addv',
  'yoyo-sem-orv',
  'yoyo-sem-subv',
  'yoyo-sem-imul',
];
// Historical stages only compiled their batch tip.
let SLICES = BATCH1.concat(BATCH2, BATCH3);
if (stageName === '_stage-truer15') SLICES = BATCH1;
else if (stageName === '_stage-truer16') SLICES = BATCH1.concat(BATCH2);

function peText(pe) {
  const e = pe.readUInt32LE(0x3c);
  const sizeOpt = pe.readUInt16LE(e + 20);
  const sect = e + 24 + sizeOpt;
  const rp = pe.readUInt32LE(sect + 20);
  const rs = pe.readUInt32LE(sect + 16);
  return pe.subarray(rp, rp + rs);
}

function main() {
  for (const name of SLICES) {
    const short = name.replace(/^yoyo-sem-/, '');
    const out = path.join(stage, 'sem-' + short, 'output.exe');
    const expPath = path.join(projects, name + '.expect.hex');
    if (!fs.existsSync(out)) {
      console.log(`Y15C1 status=RED reason=missing_pe name=${name} stage=${stageName}`);
      process.exit(1);
    }
    const exp = Buffer.from(
      fs.readFileSync(expPath, 'utf8').trim().replace(/\s+/g, ''),
      'hex',
    );
    const text = peText(fs.readFileSync(out));
    const idx = text.indexOf(exp);
    if (idx < 0) {
      console.log(`Y15C1 status=RED reason=expect_missing name=${name}`);
      console.log(`Y15C1 head=${text.subarray(0, 48).toString('hex')}`);
      process.exit(1);
    }
    const peSize = fs.statSync(out).size;
    console.log(
      `Y15C1 slice=${name} pe_bytes=${peSize} expect_at=${idx} expect_bytes=${exp.length}`,
    );
  }
  console.log(`Y15C1 stage=${stageName}`);
  console.log('Y15C1 c1_sem_regen=EXPECT_IN_TEXT');
  console.log('Y15C1 status=GREEN');
}

main();
