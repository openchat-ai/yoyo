'use strict';
/**
 * AUTO-TRUER11: compact PE template + its .text payload are YOYO-sourced (FF→C3);
 * code.bin lineage = C1 PE .text @ +13 (extract), not a second invent.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { linkPeWin32Compact } = require('../src/platform/pe-builder');
const { encodeOp } = require('../src/platform/encode-x64');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const tybPath = path.join(root, 'yoyo', 'projects', 'yoyo-compact-ret.tyb');
const c1Path = path.join(root, 'scripts', 'fixtures', 'yoyo_compiler.exe');
const pinPath = path.join(payload, 'payload.pin');

const COMPACT_MARK = Buffer.from('yoyo_compact3_c\0');
const COMPACT_LEN = 9728;
const CODE_TEXT_DELTA = 13;

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
    const op = buf[o++];
    const argc = buf[o++];
    const args = [];
    const slot = buf.readUInt16LE(o);
    const imm = buf.readUInt32LE(o + 2);
    o += 6;
    if (argc >= 1) args.push(slot);
    if (argc >= 2) args.push(imm);
    ops.push({ op, argc, args });
  }
  return ops;
}

function peTextSlice(pe) {
  const e = pe.readUInt32LE(0x3c);
  const sizeOpt = pe.readUInt16LE(e + 20);
  const sect = e + 24 + sizeOpt;
  const textRp = pe.readUInt32LE(sect + 20);
  const textRs = pe.readUInt32LE(sect + 16);
  return { textRp, textRs, text: pe.subarray(textRp, textRp + textRs) };
}

function main() {
  const pin = readPin(pinPath);
  const data = fs.readFileSync(path.join(payload, 'data.bin'));
  const code = fs.readFileSync(path.join(payload, 'code.bin'));
  const tyb = fs.readFileSync(tybPath);

  console.log(`Y11 tyb_bytes=${tyb.length} path=yoyo/projects/yoyo-compact-ret.tyb`);

  const ops = parseTyb(tyb);
  if (ops.length !== 1 || ops[0].op !== 0xff) {
    console.log('Y11 status=RED reason=tyb_not_single_ff');
    process.exit(1);
  }

  const enc = Buffer.from(encodeOp(0xff, []));
  if (!enc.equals(Buffer.from([0xc3]))) {
    console.log('Y11 status=RED reason=ff_encode_not_c3');
    process.exit(1);
  }

  const derived = linkPeWin32Compact(enc, Buffer.alloc(0), []);
  const mark = data.indexOf(COMPACT_MARK);
  if (mark < 0) {
    console.log('Y11 status=RED reason=missing_compact3_mark');
    process.exit(1);
  }
  const compactOff = mark - COMPACT_LEN;
  if (compactOff < 0 || derived.length !== COMPACT_LEN) {
    console.log(`Y11 status=RED reason=compact_geometry off=${compactOff} der=${derived.length}`);
    process.exit(1);
  }
  const baked = data.subarray(compactOff, mark);
  const derSha = crypto.createHash('sha256').update(derived).digest('hex');
  const bakeSha = crypto.createHash('sha256').update(baked).digest('hex');
  console.log(`Y11 compact_off=${compactOff} compact_len=${COMPACT_LEN}`);
  console.log(`Y11 compact_der_sha=${derSha.slice(0, 16)} bake_sha=${bakeSha.slice(0, 16)}`);
  if (!derived.equals(baked)) {
    console.log('Y11 status=RED reason=compact_not_yoyo_ff_derived');
    process.exit(1);
  }

  // parse1 S3_COMPACT_TEXT_PAYLOAD_OFF = 0x40d (IAT/prefix then YOYO FF→C3)
  const TEXT_PAYLOAD_OFF = 0x40d;
  if (derived[TEXT_PAYLOAD_OFF] !== 0xc3) {
    console.log(
      `Y11 status=RED reason=compact_text_not_c3 at=${TEXT_PAYLOAD_OFF} b=${derived[TEXT_PAYLOAD_OFF]}`,
    );
    process.exit(1);
  }
  console.log(`Y11 compact_text_off=${TEXT_PAYLOAD_OFF} compact_text=C3 yoyo_op=FF`);

  // code.bin lineage: C1 fixture PE .text @ +13
  if (!fs.existsSync(c1Path)) {
    console.log('Y11 status=RED reason=missing_c1_fixture');
    process.exit(1);
  }
  const pe = fs.readFileSync(c1Path);
  const { textRp: peTextRp, text } = peTextSlice(pe);
  const fromPe = pe.subarray(peTextRp + CODE_TEXT_DELTA, peTextRp + CODE_TEXT_DELTA + code.length);
  const codeSha = crypto.createHash('sha256').update(code).digest('hex');
  const peSha = crypto.createHash('sha256').update(fromPe).digest('hex');
  console.log(`Y11 code_bytes=${code.length} code_sha=${codeSha.slice(0, 16)} pe_text_sha=${peSha.slice(0, 16)}`);
  if (!fromPe.equals(code)) {
    console.log('Y11 status=RED reason=code_not_c1_text_extract');
    process.exit(1);
  }

  if (pin.compact_sot !== 'yoyo_ff_tyb') {
    console.log('Y11 status=RED reason=pin_compact_sot');
    process.exit(1);
  }
  if (pin.code_lineage !== 'c1_pe_text_plus13') {
    console.log('Y11 status=RED reason=pin_code_lineage');
    process.exit(1);
  }
  if (
    pin.prefab_sot !== 'bins_minus_compact' &&
    pin.prefab_sot !== 'code_plus_seeds' &&
    pin.prefab_sot !== 'yoyo_plant_tyb_plus_seeds'
  ) {
    console.log('Y11 status=RED reason=pin_prefab_sot');
    process.exit(1);
  }

  const prefabBytes = code.length + compactOff;
  console.log(`Y11 prefab_bytes_excl_compact=${prefabBytes} compact_derived=${COMPACT_LEN}`);
  console.log('Y11 compact_sot=yoyo_ff_tyb');
  console.log('Y11 code_lineage=c1_pe_text_plus13');
  console.log(`Y11 prefab_sot=${pin.prefab_sot}`);
  console.log('Y11 status=GREEN');
  console.log('Y11 doc=AUTO-TRUER11.md');
  console.log('Y11 note=compact_yoyo;code_extract_lineage;plant_body_still_prefab;not_Thompson');
  process.exit(0);
}

main();
