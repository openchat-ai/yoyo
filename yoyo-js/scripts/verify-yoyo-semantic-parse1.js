'use strict';
/**
 * AUTO-TRUER15: finite parse1 emit faces owned as YOYO semantic tyb
 * (40/00/60/61/62/FF — no A1 byte-bag), with locked expect.hex payloads.
 */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');
const pinPath = path.join(
  root,
  'yoyo-js',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'payload.pin',
);

const SLICES = [
  { name: 'yoyo-sem-ret', allow: new Set([0xff]) },
  { name: 'yoyo-sem-nop', allow: new Set([0x40, 0x00, 0xff]) },
  { name: 'yoyo-sem-sub', allow: new Set([0x40, 0x61, 0xff]) },
  { name: 'yoyo-sem-add', allow: new Set([0x40, 0x62, 0xff]) },
  { name: 'yoyo-sem-get', allow: new Set([0x40, 0x60, 0xff]) },
];

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

function parseTyb(buf) {
  if (buf.length < 8 || buf.toString('ascii', 0, 3) !== 'TYB') {
    throw new Error('bad tyb magic');
  }
  const n = buf.readUInt16LE(6);
  const ops = [];
  let o = 8;
  for (let i = 0; i < n; i++) {
    const op = buf[o];
    const argc = buf[o + 1];
    ops.push({ op, argc });
    o += 8;
  }
  return ops;
}

function main() {
  const pin = readPin(pinPath);
  if (pin.parse1_sem_slice_sot !== 'yoyo_semantic_tyb') {
    console.log('Y15 status=RED reason=pin_parse1_sem_slice_sot');
    process.exit(1);
  }

  let a1 = 0;
  for (const s of SLICES) {
    const tybPath = path.join(projects, s.name + '.tyb');
    const tyPath = path.join(projects, s.name + '.ty');
    const expPath = path.join(projects, s.name + '.expect.hex');
    if (!fs.existsSync(tybPath) || !fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
      console.log(`Y15 status=RED reason=missing_slice name=${s.name}`);
      process.exit(1);
    }
    const tyb = fs.readFileSync(tybPath);
    const ops = parseTyb(tyb);
    for (const op of ops) {
      if (op.op === 0xa1) a1++;
      if (!s.allow.has(op.op)) {
        console.log(
          `Y15 status=RED reason=non_semantic_op name=${s.name} op=0x${op.op
            .toString(16)
            .padStart(2, '0')}`,
        );
        process.exit(1);
      }
    }
    if (ops.length === 0 || ops[ops.length - 1].op !== 0xff) {
      console.log(`Y15 status=RED reason=no_trailing_ff name=${s.name}`);
      process.exit(1);
    }
    const exp = fs.readFileSync(expPath, 'utf8').trim().toLowerCase();
    if (!/^[0-9a-f]+$/.test(exp) || exp.length < 2 || exp.length % 2) {
      console.log(`Y15 status=RED reason=bad_expect name=${s.name}`);
      process.exit(1);
    }
    console.log(
      `Y15 slice=${s.name} ops=${ops.length} expect_bytes=${exp.length / 2} tyb=${tyb.length}`,
    );
  }

  if (a1 !== 0) {
    console.log(`Y15 status=RED reason=a1_present count=${a1}`);
    process.exit(1);
  }

  console.log('Y15 parse1_sem_slice_sot=yoyo_semantic_tyb');
  console.log('Y15 a1_in_sem_slices=0');
  console.log('Y15 slices=ret,nop,sub,add,get');
  console.log('Y15 status=GREEN');
}

main();
