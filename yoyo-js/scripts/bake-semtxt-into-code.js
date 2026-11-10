'use strict';
/**
 * AUTO-SEMTXT: bake Y1PL + YOYC text parser into code.bin (from 15596 baseline).
 *
 * YOYC text (LF only):
 *   YOYC\n
 *   <8 hex code_len>\n
 *   <8 hex data_len>\n
 *   <contiguous hex pairs of code||data, no whitespace>
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { layoutFor } = require('../src/platform/yoyo-compiler-parse1-native-link');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const projects = path.join(root, 'yoyo', 'projects');
const codePath = path.join(payload, 'code.bin');
const dataPath = path.join(payload, 'data.bin');
const baselinePath = path.join(root, 'scripts', '_stage-semtxt', 'code-15596.bin');

const CODE_BIN_OFF = 0x1af;
const DATA_BIN_OFF = 0x1b8;
const TRY_PLACE = 0x963;
const RET_FAST = 0xa1;
const OLD_CODE_LEN = 15596;

function u32(n) {
  const b = Buffer.alloc(4);
  b.writeUInt32LE(n >>> 0);
  return [...b];
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
      b[2] = bytes[i];
      i += 1;
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

function buildHandler(handlerOff) {
  const parts = [];
  const emit = (...xs) => {
    for (const x of xs) {
      if (typeof x === 'number') parts.push(x);
      else parts.push(...x);
    }
  };
  const fails = [];
  const sites = {};

  // peek Y1PL?
  emit(0x81, 0x3e, ...u32(0x4c503159));
  sites.jneNotY1 = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  sites.jmpReread = parts.length;
  emit(0xe9, ...u32(0));

  sites.tryYoycPeek = parts.length;
  emit(0x81, 0x3e, ...u32(0x43594f59));
  sites.jneBack = parts.length;
  emit(0x0f, 0x85, ...u32(0));

  sites.reread = parts.length;
  emit(0x49, 0x8d, 0x8f, ...u32(0x120));
  emit(0xba, ...u32(0x80000000));
  emit(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
  emit(0xc7, 0x44, 0x24, 0x20, 3, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 8, 0, 0, 0);
  emit(0x48, 0x83, 0xf8, 0xff);
  sites.jeFailOpen = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  fails.push('jeFailOpen');
  emit(0x48, 0x89, 0xc3);

  emit(0x31, 0xc9);
  emit(0xba, ...u32(0x100000));
  emit(0x41, 0xb8, 0, 0x30, 0, 0);
  emit(0x41, 0xb9, 4, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0, 0, 0, 0);
  emit(0x48, 0x85, 0xc0);
  sites.jeFailVa = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  fails.push('jeFailVa');
  emit(0x48, 0x89, 0x44, 0x24, 0x50);

  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x8b, 0x54, 0x24, 0x50);
  emit(0x41, 0xb8, ...u32(0x100000));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0x10, 0, 0, 0);
  emit(0x85, 0xc0);
  sites.jeFailRd = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  fails.push('jeFailRd');
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, 0x20, 0, 0, 0);
  emit(0x48, 0x8b, 0x74, 0x24, 0x50);

  emit(0x81, 0x3e, ...u32(0x4c503159));
  sites.jneTryYoyc = parts.length;
  emit(0x0f, 0x85, ...u32(0));

  // Y1PL
  emit(0x8b, 0x4e, 0x04);
  emit(0x89, 0x4c, 0x24, 0x58);
  emit(0x8b, 0x4e, 0x08);
  emit(0x89, 0x4c, 0x24, 0x5c);
  emit(0x48, 0x8d, 0x76, 0x0c);
  emit(0x81, 0x3e, ...u32(0x00425954));
  sites.jneFailTyb = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  fails.push('jneFailTyb');

  emit(0x31, 0xc9);
  emit(0x8b, 0x54, 0x24, 0x58);
  emit(0x03, 0x54, 0x24, 0x5c);
  emit(0x41, 0xb8, 0, 0x30, 0, 0);
  emit(0x41, 0xb9, 4, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0, 0, 0, 0);
  emit(0x48, 0x85, 0xc0);
  sites.jeFailAlloc = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  fails.push('jeFailAlloc');
  emit(0x48, 0x89, 0xc7);
  emit(0x48, 0x89, 0x44, 0x24, 0x60);
  emit(0x48, 0x83, 0xc6, 0x08);

  sites.loopAt = parts.length;
  emit(0x8a, 0x06);
  emit(0x3c, 0xff);
  sites.jeDone = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x3c, 0xa1);
  sites.jneA0 = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  emit(0x8a, 0x46, 0x02);
  emit(0xaa);
  emit(0x80, 0x7e, 0x01, 0x02);
  sites.jb1 = parts.length;
  emit(0x0f, 0x82, ...u32(0));
  emit(0x8a, 0x46, 0x04);
  emit(0xaa);
  emit(0x80, 0x7e, 0x01, 0x03);
  sites.jb2 = parts.length;
  emit(0x0f, 0x82, ...u32(0));
  emit(0x8a, 0x46, 0x06);
  emit(0xaa);
  sites.a1end = parts.length;
  emit(0x48, 0x83, 0xc6, 0x08);
  sites.jmpL1 = parts.length;
  emit(0xe9, ...u32(0));

  sites.tryA0 = parts.length;
  emit(0x3c, 0xa0);
  sites.jneSk = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  emit(0x8a, 0x46, 0x02);
  emit(0xaa);
  emit(0x48, 0x83, 0xc6, 0x08);
  sites.jmpL2 = parts.length;
  emit(0xe9, ...u32(0));

  sites.skip = parts.length;
  emit(0x48, 0x83, 0xc6, 0x08);
  sites.jmpL3 = parts.length;
  emit(0xe9, ...u32(0));

  // YOYC
  sites.yoycAt = parts.length;
  emit(0x81, 0x3e, ...u32(0x43594f59));
  sites.jneFailYoyc = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  fails.push('jneFailYoyc');
  emit(0x48, 0x83, 0xc6, 0x04);
  emit(0x80, 0x3e, 0x0a);
  sites.jneFailNl0 = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  fails.push('jneFailNl0');
  emit(0x48, 0xff, 0xc6);

  // parseHex8 → eax; clobber edx; advances rsi; on bad → fail
  function emitHex8(tag) {
    emit(0x31, 0xc0);
    emit(0xb9, 8, 0, 0, 0);
    sites[tag + 'Loop'] = parts.length;
    emit(0x0f, 0xb6, 0x16);
    emit(0x48, 0xff, 0xc6);
    emit(0x83, 0xfa, 0x30);
    sites[tag + 'B0'] = parts.length;
    emit(0x0f, 0x8c, ...u32(0));
    fails.push(tag + 'B0');
    emit(0x83, 0xfa, 0x39);
    sites[tag + 'AA'] = parts.length;
    emit(0x0f, 0x8f, ...u32(0));
    emit(0x83, 0xea, 0x30);
    sites[tag + 'J1'] = parts.length;
    emit(0xe9, ...u32(0));
    sites[tag + 'A'] = parts.length;
    emit(0x83, 0xfa, 0x41);
    sites[tag + 'B1'] = parts.length;
    emit(0x0f, 0x8c, ...u32(0));
    fails.push(tag + 'B1');
    emit(0x83, 0xfa, 0x46);
    sites[tag + 'AL'] = parts.length;
    emit(0x0f, 0x8f, ...u32(0));
    emit(0x83, 0xea, 0x37);
    sites[tag + 'J2'] = parts.length;
    emit(0xe9, ...u32(0));
    sites[tag + 'L'] = parts.length;
    emit(0x83, 0xfa, 0x61);
    sites[tag + 'B2'] = parts.length;
    emit(0x0f, 0x8c, ...u32(0));
    fails.push(tag + 'B2');
    emit(0x83, 0xfa, 0x66);
    sites[tag + 'B3'] = parts.length;
    emit(0x0f, 0x8f, ...u32(0));
    fails.push(tag + 'B3');
    emit(0x83, 0xea, 0x57);
    sites[tag + 'Ok'] = parts.length;
    emit(0xc1, 0xe0, 0x04);
    emit(0x01, 0xd0);
    emit(0xe2);
    sites[tag + 'LoopRel'] = parts.length;
    emit(0x00);
  }

  emitHex8('c');
  emit(0x89, 0x44, 0x24, 0x58);
  emit(0x80, 0x3e, 0x0a);
  sites.jneFailNl1 = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  fails.push('jneFailNl1');
  emit(0x48, 0xff, 0xc6);

  emitHex8('d');
  emit(0x89, 0x44, 0x24, 0x5c);
  emit(0x80, 0x3e, 0x0a);
  sites.jneFailNl2 = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  fails.push('jneFailNl2');
  emit(0x48, 0xff, 0xc6);

  emit(0x31, 0xc9);
  emit(0x8b, 0x54, 0x24, 0x58);
  emit(0x03, 0x54, 0x24, 0x5c);
  emit(0x41, 0xb8, 0, 0x30, 0, 0);
  emit(0x41, 0xb9, 4, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0, 0, 0, 0);
  emit(0x48, 0x85, 0xc0);
  sites.jeFailYAlloc = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  fails.push('jeFailYAlloc');
  emit(0x48, 0x89, 0xc7);
  emit(0x48, 0x89, 0x44, 0x24, 0x60);
  emit(0x8b, 0x4c, 0x24, 0x58);
  emit(0x03, 0x4c, 0x24, 0x5c);

  sites.hexBody = parts.length;
  emit(0x85, 0xc9);
  sites.jeBodyDone = parts.length;
  emit(0x0f, 0x84, ...u32(0));

  // nibble1 → bl
  emit(0x0f, 0xb6, 0x06);
  emit(0x48, 0xff, 0xc6);
  emit(0x83, 0xf8, 0x30);
  sites.hb0 = parts.length;
  emit(0x0f, 0x8c, ...u32(0));
  fails.push('hb0');
  emit(0x83, 0xf8, 0x39);
  sites.haA = parts.length;
  emit(0x0f, 0x8f, ...u32(0));
  emit(0x83, 0xe8, 0x30);
  sites.hj1 = parts.length;
  emit(0xe9, ...u32(0));
  sites.hA = parts.length;
  emit(0x83, 0xf8, 0x41);
  sites.hb1 = parts.length;
  emit(0x0f, 0x8c, ...u32(0));
  fails.push('hb1');
  emit(0x83, 0xf8, 0x46);
  sites.haL = parts.length;
  emit(0x0f, 0x8f, ...u32(0));
  emit(0x83, 0xe8, 0x37);
  sites.hj2 = parts.length;
  emit(0xe9, ...u32(0));
  sites.hL = parts.length;
  emit(0x83, 0xf8, 0x61);
  sites.hb2 = parts.length;
  emit(0x0f, 0x8c, ...u32(0));
  fails.push('hb2');
  emit(0x83, 0xf8, 0x66);
  sites.hb3 = parts.length;
  emit(0x0f, 0x8f, ...u32(0));
  fails.push('hb3');
  emit(0x83, 0xe8, 0x57);
  sites.hok1 = parts.length;
  emit(0xc1, 0xe0, 0x04);
  emit(0x88, 0xc3); // bl = hi<<4

  // nibble2
  emit(0x0f, 0xb6, 0x06);
  emit(0x48, 0xff, 0xc6);
  emit(0x83, 0xf8, 0x30);
  sites.hb4 = parts.length;
  emit(0x0f, 0x8c, ...u32(0));
  fails.push('hb4');
  emit(0x83, 0xf8, 0x39);
  sites.haA2 = parts.length;
  emit(0x0f, 0x8f, ...u32(0));
  emit(0x83, 0xe8, 0x30);
  sites.hj3 = parts.length;
  emit(0xe9, ...u32(0));
  sites.hA2 = parts.length;
  emit(0x83, 0xf8, 0x41);
  sites.hb5 = parts.length;
  emit(0x0f, 0x8c, ...u32(0));
  fails.push('hb5');
  emit(0x83, 0xf8, 0x46);
  sites.haL2 = parts.length;
  emit(0x0f, 0x8f, ...u32(0));
  emit(0x83, 0xe8, 0x37);
  sites.hj4 = parts.length;
  emit(0xe9, ...u32(0));
  sites.hL2 = parts.length;
  emit(0x83, 0xf8, 0x61);
  sites.hb6 = parts.length;
  emit(0x0f, 0x8c, ...u32(0));
  fails.push('hb6');
  emit(0x83, 0xf8, 0x66);
  sites.hb7 = parts.length;
  emit(0x0f, 0x8f, ...u32(0));
  fails.push('hb7');
  emit(0x83, 0xe8, 0x57);
  sites.hok2 = parts.length;
  emit(0x08, 0xd8); // al |= bl
  emit(0xaa);
  emit(0xff, 0xc9);
  sites.jmpBody = parts.length;
  emit(0xe9, ...u32(0));

  function writeBin(pathOff, setupRdx) {
    emit(0x49, 0x8d, 0x8f, ...u32(pathOff));
    emit(0xba, ...u32(0x40000000));
    emit(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
    emit(0xc7, 0x44, 0x24, 0x20, 2, 0, 0, 0);
    emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0, 0, 0);
    emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
    emit(0x41, 0xff, 0x97, 8, 0, 0, 0);
    emit(0x48, 0x83, 0xf8, 0xff);
    const j1 = 'w' + pathOff + 'a';
    sites[j1] = parts.length;
    emit(0x0f, 0x84, ...u32(0));
    fails.push(j1);
    emit(0x48, 0x89, 0xc3);
    setupRdx();
    emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
    emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
    emit(0x48, 0x89, 0xd9);
    emit(0x41, 0xff, 0x97, 0x18, 0, 0, 0);
    emit(0x85, 0xc0);
    const j2 = 'w' + pathOff + 'b';
    sites[j2] = parts.length;
    emit(0x0f, 0x84, ...u32(0));
    fails.push(j2);
    emit(0x48, 0x89, 0xd9);
    emit(0x41, 0xff, 0x97, 0x20, 0, 0, 0);
  }

  sites.doneAt = parts.length;
  writeBin(CODE_BIN_OFF, () => {
    emit(0x48, 0x8b, 0x54, 0x24, 0x60);
    emit(0x44, 0x8b, 0x44, 0x24, 0x58);
  });
  writeBin(DATA_BIN_OFF, () => {
    emit(0x48, 0x8b, 0x54, 0x24, 0x60);
    emit(0x8b, 0x44, 0x24, 0x58);
    emit(0x48, 0x01, 0xc2);
    emit(0x44, 0x8b, 0x44, 0x24, 0x5c);
  });
  sites.jmpPlace = parts.length;
  emit(0xe9, ...u32(0));

  sites.failAt = parts.length;
  emit(0xb8, 1, 0, 0, 0);
  emit(0x48, 0x83, 0xc4, 0x78);
  emit(0xc3);

  sites.backAt = parts.length;
  emit(0x83, 0xff, 0x10);
  sites.jneOrig = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  sites.jmpCont = parts.length;
  emit(0xe9, ...u32(0));

  const buf = Buffer.from(parts);
  function patchRel(at, targetAbs) {
    const isJcc = buf[at] === 0x0f;
    const immAt = isJcc ? at + 2 : at + 1;
    const next = isJcc ? at + 6 : at + 5;
    buf.writeInt32LE((targetAbs - (handlerOff + next)) | 0, immAt);
  }
  const H = handlerOff;
  const P = (name, target) => patchRel(sites[name], H + sites[target]);

  P('jneNotY1', 'tryYoycPeek');
  P('jmpReread', 'reread');
  P('jneBack', 'backAt');
  P('jneTryYoyc', 'yoycAt');
  P('jeDone', 'doneAt');
  P('jneA0', 'tryA0');
  P('jb1', 'a1end');
  P('jb2', 'a1end');
  P('jmpL1', 'loopAt');
  P('jneSk', 'skip');
  P('jmpL2', 'loopAt');
  P('jmpL3', 'loopAt');

  for (const tag of ['c', 'd']) {
    P(tag + 'AA', tag + 'A');
    P(tag + 'J1', tag + 'Ok');
    P(tag + 'AL', tag + 'L');
    P(tag + 'J2', tag + 'Ok');
    const loopAt = sites[tag + 'Loop'];
    const relAt = sites[tag + 'LoopRel'];
    buf[relAt] = (loopAt - (relAt + 1)) & 0xff;
  }

  P('jeBodyDone', 'doneAt');
  P('haA', 'hA');
  P('hj1', 'hok1');
  P('haL', 'hL');
  P('hj2', 'hok1');
  P('haA2', 'hA2');
  P('hj3', 'hok2');
  P('haL2', 'hL2');
  P('hj4', 'hok2');
  P('jmpBody', 'hexBody');

  for (const f of fails) patchRel(sites[f], H + sites.failAt);
  patchRel(sites.jmpPlace, TRY_PLACE);
  patchRel(sites.jneOrig, 0xdd);
  patchRel(sites.jmpCont, 0xaa);
  return buf;
}

function main() {
  if (!fs.existsSync(baselinePath)) {
    console.log('SEMTXT status=RED reason=missing_baseline');
    process.exit(1);
  }
  fs.copyFileSync(baselinePath, codePath);
  const oldCode = fs.readFileSync(codePath);
  const data = fs.readFileSync(dataPath);
  if (oldCode.length !== OLD_CODE_LEN) {
    console.log('SEMTXT status=RED reason=baseline_len got=' + oldCode.length);
    process.exit(1);
  }

  const handlerOff = oldCode.length;
  const handler = buildHandler(handlerOff);
  const newCode = Buffer.concat([oldCode, handler]);
  const Lold = layoutFor(OLD_CODE_LEN, data.length, 0x38000);
  const Lnew = layoutFor(newCode.length, data.length, 0x38000);

  const tramp = Buffer.alloc(5);
  tramp[0] = 0xe9;
  tramp.writeInt32LE(handlerOff - (RET_FAST + 5), 1);
  tramp.copy(newCode, RET_FAST);

  function replaceImm(oldV, newV, siteList) {
    for (const s of siteList) {
      if (newCode.readUInt32LE(s) !== oldV) throw new Error('imm mismatch at 0x' + s.toString(16));
      newCode.writeUInt32LE(newV >>> 0, s);
    }
  }
  replaceImm(OLD_CODE_LEN, newCode.length, [0x8b2, 0x98e, 0xa22, 0xa4a, 0x115d]);
  replaceImm(Lold.peSize, Lnew.peSize, [0x7bc, 0x828, 0x850, 0x966, 0xae9, 0x1182]);
  // textVs grows 0x4000→0x5000 — must move dataRva/IAT/lea before codeRaw aliases collide
  replaceImm(Lold.sizeOfImage, Lnew.sizeOfImage, [0xbea]);
  replaceImm(Lold.importDirRva, Lnew.importDirRva, [0xcb8]);
  replaceImm(Lold.textVs, Lnew.textVs, [0xd12, 0xdd4]);
  replaceImm(Lold.leaDisp, Lnew.leaDisp, [0xe08]);
  // IAT blob RVAs (+0x1000) via movabs — before dataRva u32/site patches
  const { prependWin32IoIat } = require('../src/platform/pe-builder');
  const iatOld = prependWin32IoIat(Buffer.alloc(0), Lold.dataRva).data.subarray(0, Lold.iatLen);
  const iatNew = prependWin32IoIat(Buffer.alloc(0), Lnew.dataRva).data.subarray(0, Lnew.iatLen);
  for (let off = 0; off + 8 <= iatOld.length; off += 8) {
    const oldQ = iatOld.readBigUInt64LE(off);
    const newQ = iatNew.readBigUInt64LE(off);
    if (oldQ === newQ) continue;
    let hits = 0;
    for (let i = 0; i <= newCode.length - 10; i++) {
      if (newCode[i] === 0x48 && newCode[i + 1] === 0xb8 && newCode.readBigUInt64LE(i + 2) === oldQ) {
        newCode.writeBigUInt64LE(newQ, i + 2);
        hits++;
      }
    }
    if (hits < 1) {
      let already = 0;
      for (let i = 0; i <= newCode.length - 10; i++) {
        if (newCode[i] === 0x48 && newCode[i + 1] === 0xb8 && newCode.readBigUInt64LE(i + 2) === newQ) already++;
      }
      if (already < 1) throw new Error('iat value off+' + off + ' hits=0');
    }
  }
  replaceImm(Lold.dataRva, Lnew.dataRva, [0xda8]); // section header u32; movabs 0xefb already via IAT
  replaceImm(Lold.codeRaw, Lnew.codeRaw, [0xc8b, 0xd30]);
  replaceImm(Lold.dataFileOff, Lnew.dataFileOff, [0xdc6, 0xe45]);
  replaceImm(Lold.userDataOff, Lnew.userDataOff, [0x928, 0x116b]);
  for (let k = 8; k < Lold.iatLen; k += 8) {
    const ov = (Lold.dataFileOff + k) >>> 0;
    const nv = (Lnew.dataFileOff + k) >>> 0;
    let hits = 0;
    for (let i = 0; i <= newCode.length - 4; i++) {
      if (newCode.readUInt32LE(i) === ov) {
        newCode.writeUInt32LE(nv, i);
        hits++;
      }
    }
    if (hits !== 1) throw new Error('iat off+' + k + ' hits=' + hits);
  }

  fs.writeFileSync(codePath, newCode);
  fs.writeFileSync(path.join(projects, 'yoyo-plant-place.tyb'), packPlant(newCode.subarray(0, 4491)));
  fs.writeFileSync(path.join(projects, 'yoyo-plant-parse1.tyb'), packPlant(newCode.subarray(4491)));
  fs.writeFileSync(path.join(projects, 'yoyo-plant.tyb'), packPlant(newCode));

  const codeSha = crypto.createHash('sha256').update(newCode).digest('hex');
  console.log('SEMTXT code_bytes=' + newCode.length + ' handler=' + handler.length);
  console.log('SEMTXT code_sha256_prefix=' + codeSha.slice(0, 16));
  console.log('SEMTXT peSize ' + Lold.peSize + '→' + Lnew.peSize + ' codeRaw ' + Lold.codeRaw + '→' + Lnew.codeRaw);
  console.log('SEMTXT status=GREEN');
}

main();
