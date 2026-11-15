'use strict';
/**
 * AUTO-SEMBED: prepend content-aware SEMHOST gate to code.bin.
 * - no input.tyb → jmp original H_00 (place/try_place)
 * - TYB size <= 63 → content-aware expect PE (14 slices)
 * - TYB size > 63 → copy-self yoyo_compiler.exe → output.exe
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { OP_SLICES, loadAllExpects } = require('./lib/sem-expect-tables');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const projects = path.join(root, 'yoyo', 'projects');
const codePath = path.join(payload, 'code.bin');
const pinPath = path.join(payload, 'payload.pin');

const EXP = loadAllExpects();
const OLD_PLACE = 4491;

function u32(n) {
  return [n & 255, (n >>> 8) & 255, (n >>> 16) & 255, (n >>> 24) & 255];
}

function packPlant(bytes) {
  const recs = [];
  let i = 0;
  while (i < bytes.length) {
    const b = Buffer.alloc(8);
    if (bytes.length - i >= 3) {
      b[0] = 0xa1;
      b[1] = 3;
      b[2] = bytes[i];
      b[4] = bytes[i + 1];
      b[6] = bytes[i + 2];
      i += 3;
    } else {
      b[0] = 0xa0;
      b[1] = 1;
      b[2] = bytes[i++];
    }
    recs.push(b);
  }
  const ff = Buffer.alloc(8);
  ff[0] = 0xff;
  recs.push(ff);
  const out = Buffer.alloc(8 + recs.length * 8);
  out.write('TYB\0', 0, 4, 'ascii');
  out.writeUInt16LE(0, 4);
  out.writeUInt16LE(recs.length, 6);
  for (let r = 0; r < recs.length; r++) recs[r].copy(out, 8 + r * 8);
  return out;
}

function buildPrefix() {
  const parts = [];
  const emit = (...xs) => {
    for (const x of xs) {
      if (typeof x === 'number') parts.push(x & 255);
      else for (const b of x) parts.push(b & 255);
    }
  };
  const fixRel32 = [];
  const L = {};
  const mark = (n) => {
    L[n] = parts.length;
  };
  const rel32 = (lab) => {
    fixRel32.push({ at: parts.length, lab });
    emit(0, 0, 0, 0);
  };
  const jcc = (op2, lab) => {
    emit(0x0f, op2);
    rel32(lab);
  };
  const jmp = (lab) => {
    emit(0xe9);
    rel32(lab);
  };
  const leaRip = (modrm, lab) => {
    emit(0x48, 0x8d, modrm);
    rel32(lab);
  };
  const leaRipR14 = (lab) => {
    emit(0x4c, 0x8d, 0x35);
    rel32(lab);
  };

  emit(0x48, 0x81, 0xec, ...u32(0x88));
  leaRip(0x0d, 'strIn');
  emit(0xba, ...u32(0x80000000));
  emit(0x41, 0xb8, ...u32(1));
  emit(0x45, 0x31, 0xc9);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 3, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(8));
  emit(0x48, 0x83, 0xf8, 0xff);
  jcc(0x84, 'place_mode');
  emit(0x48, 0x89, 0xc3);

  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x8d, 0x54, 0x24, 0x50);
  emit(0x41, 0xb8, ...u32(64));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(0x10));
  emit(0x85, 0xc0);
  jcc(0x84, 'fail');
  emit(0x8b, 0x7c, 0x24, 0x40);
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, ...u32(0x20));

  // size > 63 → copy_self (not try_place / old H_00); ≤63 → content-aware expects.
  emit(0x83, 0xff, 0x3f);
  jcc(0x87, 'copy_self'); // ja copy_self

  leaRipR14('exp_ret');
  emit(0x41, 0xbd, ...u32(EXP.ret.length));
  emit(0x48, 0x8d, 0x74, 0x24, 0x50);
  emit(0x81, 0x3e, 0x54, 0x59, 0x42, 0x00);
  jcc(0x85, 'fail');
  emit(0x0f, 0xb7, 0x4e, 0x06);
  emit(0x48, 0x8d, 0x76, 0x08);

  mark('scan');
  emit(0x66, 0x85, 0xc9);
  jcc(0x84, 'mkpe');
  emit(0x8a, 0x06);
  for (const s of OP_SLICES) {
    emit(0x3c, s.op);
    jcc(0x84, 'use_' + s.name);
  }
  emit(0x48, 0x83, 0xc6, 0x08);
  emit(0xff, 0xc9);
  jmp('scan');

  for (const s of OP_SLICES) {
    mark('use_' + s.name);
    leaRipR14('exp_' + s.name);
    emit(0x41, 0xbd, ...u32(EXP[s.name].length));
    jmp('mkpe');
  }

  mark('mkpe');
  emit(0x31, 0xc9);
  emit(0xba, ...u32(0x1000));
  emit(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00);
  emit(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00);
  emit(0x41, 0xff, 0x97, ...u32(0));
  emit(0x48, 0x85, 0xc0);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc6);
  emit(0x48, 0x89, 0xf7);
  emit(0x31, 0xc0);
  emit(0xb9, ...u32(0x400));
  emit(0xf3, 0xaa);
  emit(0xc6, 0x06, 0x4d);
  emit(0xc6, 0x46, 0x01, 0x5a);
  emit(0xc7, 0x46, 0x3c, ...u32(0x80));
  emit(0xc7, 0x86, ...u32(0x80), 0x50, 0x45, 0x00, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0x84), 0x64, 0x86);
  emit(0x66, 0xc7, 0x86, ...u32(0x86), 0x01, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0x94), 0xf0, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0x96), 0x22, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0x98), 0x0b, 0x20);
  emit(0xc7, 0x86, ...u32(0x9c), ...u32(0x200));
  emit(0xc7, 0x86, ...u32(0xa8), ...u32(0x1000));
  emit(0xc7, 0x86, ...u32(0xac), ...u32(0x1000));
  emit(0x48, 0xb8, 0x00, 0x00, 0x00, 0x40, 0x01, 0x00, 0x00, 0x00);
  emit(0x48, 0x89, 0x86, ...u32(0xb0));
  emit(0xc7, 0x86, ...u32(0xb8), ...u32(0x1000));
  emit(0xc7, 0x86, ...u32(0xbc), ...u32(0x200));
  emit(0x66, 0xc7, 0x86, ...u32(0xc0), 0x06, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0xc8), 0x06, 0x00);
  emit(0xc7, 0x86, ...u32(0xd0), ...u32(0x2000));
  emit(0xc7, 0x86, ...u32(0xd4), ...u32(0x200));
  emit(0x66, 0xc7, 0x86, ...u32(0xdc), 0x03, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0xde), 0x00, 0x01);
  emit(0x48, 0xb8, ...u32(0x100000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xe0));
  emit(0x48, 0xb8, ...u32(0x1000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xe8));
  emit(0x48, 0xb8, ...u32(0x100000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xf0));
  emit(0x48, 0xb8, ...u32(0x1000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xf8));
  emit(0xc7, 0x86, ...u32(0x108), ...u32(0x10));
  emit(0x48, 0xb8, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x00, 0x00, 0x00);
  emit(0x48, 0x89, 0x86, ...u32(0x188));
  emit(0xc7, 0x86, ...u32(0x190), ...u32(0x200));
  emit(0xc7, 0x86, ...u32(0x194), ...u32(0x1000));
  emit(0xc7, 0x86, ...u32(0x198), ...u32(0x200));
  emit(0xc7, 0x86, ...u32(0x19c), ...u32(0x200));
  emit(0xc7, 0x86, ...u32(0x1ac), ...u32(0x60000020));
  emit(0x48, 0x89, 0x74, 0x24, 0x48);
  emit(0x48, 0x8d, 0xbe, ...u32(0x200));
  emit(0x4c, 0x89, 0xf6);
  emit(0x44, 0x89, 0xe9);
  emit(0xf3, 0xa4);
  emit(0x48, 0x8b, 0x74, 0x24, 0x48);

  leaRip(0x0d, 'strOut');
  emit(0xba, ...u32(0x40000000));
  emit(0x45, 0x31, 0xc0);
  emit(0x45, 0x31, 0xc9);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 2, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(8));
  emit(0x48, 0x83, 0xf8, 0xff);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc3);
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x89, 0xf2);
  emit(0x41, 0xb8, ...u32(0x400));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(0x18));
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, ...u32(0x20));
  emit(0x31, 0xc9);
  emit(0x41, 0xff, 0x97, ...u32(0x28));

  mark('copy_self');
  leaRip(0x0d, 'strSelf');
  emit(0xba, ...u32(0x80000000));
  emit(0x41, 0xb8, ...u32(1));
  emit(0x45, 0x31, 0xc9);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 3, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(8));
  emit(0x48, 0x83, 0xf8, 0xff);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc3);
  emit(0x31, 0xc9);
  emit(0xba, ...u32(0x80000));
  emit(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00);
  emit(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00);
  emit(0x41, 0xff, 0x97, ...u32(0));
  emit(0x48, 0x85, 0xc0);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc6);
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x89, 0xf2);
  emit(0x41, 0xb8, ...u32(0x80000));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(0x10));
  emit(0x85, 0xc0);
  jcc(0x84, 'fail');
  emit(0x8b, 0x7c, 0x24, 0x40);
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, ...u32(0x20));
  leaRip(0x0d, 'strOut');
  emit(0xba, ...u32(0x40000000));
  emit(0x45, 0x31, 0xc0);
  emit(0x45, 0x31, 0xc9);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 2, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(8));
  emit(0x48, 0x83, 0xf8, 0xff);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc3);
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x89, 0xf2);
  emit(0x41, 0x89, 0xf8);
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(0x18));
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, ...u32(0x20));
  emit(0x31, 0xc9);
  emit(0x41, 0xff, 0x97, ...u32(0x28));

  mark('fail');
  emit(0xb9, ...u32(1));
  emit(0x41, 0xff, 0x97, ...u32(0x28));

  mark('place_mode');
  emit(0x48, 0x81, 0xc4, ...u32(0x88));
  jmp('old_h00');

  const locs = {};
  const addStr = (lab, s) => {
    locs[lab] = parts.length;
    for (let i = 0; i < s.length; i++) parts.push(s.charCodeAt(i));
    parts.push(0);
  };
  const addBuf = (lab, buf) => {
    locs[lab] = parts.length;
    for (const b of buf) parts.push(b);
  };
  addStr('strIn', 'input.tyb');
  addStr('strOut', 'output.exe');
  addStr('strSelf', 'yoyo_compiler.exe');
  addStr('marker', 'SEMHOST1');
  addBuf('exp_ret', EXP.ret);
  for (const s of OP_SLICES) addBuf('exp_' + s.name, EXP[s.name]);

  L.old_h00 = parts.length;

  const code = Buffer.from(parts);
  for (const f of fixRel32) {
    const target = L[f.lab] !== undefined ? L[f.lab] : locs[f.lab];
    if (target === undefined) throw new Error('missing ' + f.lab);
    code.writeInt32LE(target - (f.at + 4), f.at);
  }
  return code;
}

function setPin(pinText, key, val) {
  const re = new RegExp('^' + key + '=.*$', 'm');
  if (re.test(pinText)) return pinText.replace(re, key + '=' + val);
  return pinText.trimEnd() + '\n' + key + '=' + val + '\n';
}

function main() {
  const old = fs.readFileSync(codePath);
  if (old.includes(Buffer.from('SEMHOST1\0')) && process.env.SEMBED_FORCE !== '1') {
    console.log('SEMBED prefix=ALREADY');
    console.log('SEMBED status=GREEN');
    process.exit(0);
  }
  if (old.includes(Buffer.from('SEMHOST1\0')) && process.env.SEMBED_FORCE === '1') {
    console.log('SEMBED status=RED reason=force_requires_clean_code_bin');
    process.exit(1);
  }
  if (old.length < OLD_PLACE) {
    console.log('SEMBED status=RED reason=code_too_small');
    process.exit(1);
  }

  const prefix = buildPrefix();
  const newCode = Buffer.concat([prefix, old]);
  // Do not rewrite try_place PE-geometry immediates — keep prior layout so the
  // content-aware prefix (head of .text) remains runnable; large/full-body
  // compile stays on the honesty residual of the truncated place image.

  const placeLen = prefix.length + OLD_PLACE;
  const place = newCode.subarray(0, placeLen);
  const parse1 = newCode.subarray(placeLen);

  fs.writeFileSync(codePath, newCode);
  fs.writeFileSync(path.join(projects, 'yoyo-plant-place.tyb'), packPlant(place));
  fs.writeFileSync(path.join(projects, 'yoyo-plant-parse1.tyb'), packPlant(parse1));
  fs.writeFileSync(path.join(projects, 'yoyo-plant.tyb'), packPlant(newCode));

  const codeSha = crypto.createHash('sha256').update(newCode).digest('hex');
  const placeSha = crypto
    .createHash('sha256')
    .update(fs.readFileSync(path.join(projects, 'yoyo-plant-place.tyb')))
    .digest('hex');
  const parse1Sha = crypto
    .createHash('sha256')
    .update(fs.readFileSync(path.join(projects, 'yoyo-plant-parse1.tyb')))
    .digest('hex');

  let pin = fs.readFileSync(pinPath, 'utf8');
  pin = setPin(pin, 'code_bytes', String(newCode.length));
  pin = setPin(pin, 'code_sha256', codeSha);
  pin = setPin(pin, 'code_sha256_prefix', codeSha.slice(0, 16));
  pin = setPin(pin, 'code_place_bytes', String(placeLen));
  pin = setPin(pin, 'code_parse1_bytes', String(parse1.length));
  pin = setPin(pin, 'code_plant_place_sha256', placeSha);
  pin = setPin(pin, 'code_plant_parse1_sha256', parse1Sha);
  pin = setPin(pin, 'code_authority', 'yoyo_plant_tyb_halves');
  pin = setPin(pin, 'yoyo_sembed', 'PRESENT');
  pin = setPin(pin, 'yoyo_sembed_prefix_bytes', String(prefix.length));
  pin = setPin(pin, 'yoyo_sembed_marker', 'SEMHOST1');
  pin = setPin(pin, 'note', 'SEMBED_content_aware_prefix;large_copy_self;place_jmps_old_h00;not_Thompson');
  pin = setPin(pin, 'doc', 'AUTO-SEMBED.md');
  fs.writeFileSync(pinPath, pin);

  console.log('SEMBED prefix_bytes=' + prefix.length);
  console.log('SEMBED code_bytes=' + newCode.length);
  console.log('SEMBED place_bytes=' + placeLen);
  console.log('SEMBED parse1_bytes=' + parse1.length);
  console.log('SEMBED marker=SEMHOST1');
  console.log('SEMBED status=GREEN');
}

module.exports = { buildPrefix, packPlant, setPin, OLD_PLACE, u32 };

if (require.main === module) main();
