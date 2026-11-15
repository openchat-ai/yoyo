'use strict';
/**
 * AUTO-SEMCOMP: PE trampoline — one plant-A1 tyb → unpack → bins → try_place_bins
 * → output.exe == stock compiler (semantic 编出编译器).
 *
 * input.tyb = Y1PL | u32 code_len | u32 data_len | TYB(A1 packing of code||data)
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');

const CODE_BIN_OFF = 0x1af;
const DATA_BIN_OFF = 0x1b8;
const TRY_PLACE_OFF_IN_CODE = 0x963; // code.bin offset of try_place_bins

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

function peText(pe) {
  const e = pe.readUInt32LE(0x3c);
  const optSize = pe.readUInt16LE(e + 20);
  const sec = e + 24 + optSize;
  return {
    e,
    va: pe.readUInt32LE(sec + 12),
    vs: pe.readUInt32LE(sec + 8),
    rs: pe.readUInt32LE(sec + 16),
    rp: pe.readUInt32LE(sec + 20),
  };
}

function buildHandler(handlerRva, codeEntryRva, origPrologue, tryPlaceRva) {
  const parts = [];
  const emit = (...xs) => {
    for (const x of xs) {
      if (typeof x === 'number') parts.push(x);
      else parts.push(...x);
    }
  };

  emit(0x48, 0x83, 0xec, 0x78);

  // open input.tyb
  emit(0x49, 0x8d, 0x8f, ...u32(0x120));
  emit(0xba, ...u32(0x80000000));
  emit(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
  emit(0xc7, 0x44, 0x24, 0x20, 3, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 8, 0, 0, 0);
  emit(0x48, 0x83, 0xf8, 0xff);
  const jeFb1 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xc3);

  // VirtualAlloc 1MB scratch
  emit(0x31, 0xc9);
  emit(0xba, ...u32(0x100000));
  emit(0x41, 0xb8, 0, 0x30, 0, 0);
  emit(0x41, 0xb9, 4, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0, 0, 0, 0);
  emit(0x48, 0x85, 0xc0);
  const jeFb2 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0x44, 0x24, 0x50); // [rsp+0x50] = file buf

  // ReadFile
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x8b, 0x54, 0x24, 0x50);
  emit(0x41, 0xb8, ...u32(0x100000));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0x10, 0, 0, 0);
  emit(0x85, 0xc0);
  const jeFb3 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, 0x20, 0, 0, 0);

  // rsi = file buf; magic Y1PL?
  emit(0x48, 0x8b, 0x74, 0x24, 0x50);
  emit(0x81, 0x3e, ...u32(0x4c503159)); // 'Y1PL' LE
  const jneFb = parts.length;
  emit(0x0f, 0x85, ...u32(0));

  // code_len=[rsi+4] data_len=[rsi+8]; plant tyb at rsi+12
  emit(0x8b, 0x4e, 0x04);
  emit(0x89, 0x4c, 0x24, 0x58); // save code_len
  emit(0x8b, 0x4e, 0x08);
  emit(0x89, 0x4c, 0x24, 0x5c); // save data_len
  emit(0x48, 0x8d, 0x76, 0x0c); // rsi = plant TYB

  // check TYB magic
  emit(0x81, 0x3e, ...u32(0x00425954)); // 'TYB\0'
  const jneFbTyb = parts.length;
  emit(0x0f, 0x85, ...u32(0));

  // alloc dest for unpacked bins
  emit(0x31, 0xc9);
  emit(0x8b, 0x54, 0x24, 0x58);
  emit(0x03, 0x54, 0x24, 0x5c); // edx = code+data
  emit(0x41, 0xb8, 0, 0x30, 0, 0);
  emit(0x41, 0xb9, 4, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0, 0, 0, 0);
  emit(0x48, 0x85, 0xc0);
  const jeFail = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xc7); // rdi = dest
  emit(0x48, 0x89, 0x44, 0x24, 0x60); // save dest base

  // A1 unpack: rsi records, rdi dest
  emit(0x48, 0x83, 0xc6, 0x08); // skip TYB header
  const loopAt = parts.length;
  emit(0x8a, 0x06); // al = op
  emit(0x3c, 0xff);
  const jeDone = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x3c, 0xa1);
  const jneTryA0 = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  // A1: argc at [rsi+1]
  emit(0x8a, 0x46, 0x02);
  emit(0xaa); // stosb
  emit(0x80, 0x7e, 0x01, 0x02);
  const jbA1end = parts.length;
  emit(0x0f, 0x82, ...u32(0)); // jb skip2
  emit(0x8a, 0x46, 0x04);
  emit(0xaa);
  emit(0x80, 0x7e, 0x01, 0x03);
  const jbA1end2 = parts.length;
  emit(0x0f, 0x82, ...u32(0));
  emit(0x8a, 0x46, 0x06);
  emit(0xaa);
  const a1end = parts.length;
  emit(0x48, 0x83, 0xc6, 0x08);
  const jmpLoop1 = parts.length;
  emit(0xe9, ...u32(0));

  const tryA0 = parts.length;
  emit(0x3c, 0xa0);
  const jneSkip = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  emit(0x8a, 0x46, 0x02);
  emit(0xaa);
  emit(0x48, 0x83, 0xc6, 0x08);
  const jmpLoop2 = parts.length;
  emit(0xe9, ...u32(0));

  const skipRec = parts.length;
  emit(0x48, 0x83, 0xc6, 0x08);
  const jmpLoop3 = parts.length;
  emit(0xe9, ...u32(0));

  const doneAt = parts.length;
  // write code.bin from dest base, len code_len
  emit(0x49, 0x8d, 0x8f, ...u32(CODE_BIN_OFF));
  emit(0xba, ...u32(0x40000000));
  emit(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
  emit(0xc7, 0x44, 0x24, 0x20, 2, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 8, 0, 0, 0);
  emit(0x48, 0x83, 0xf8, 0xff);
  const jeFailW1 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xc3);
  emit(0x48, 0x8b, 0x54, 0x24, 0x60);
  emit(0x44, 0x8b, 0x44, 0x24, 0x58);
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, 0x18, 0, 0, 0);
  emit(0x85, 0xc0);
  const jeFailW2 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, 0x20, 0, 0, 0);

  // write data.bin
  emit(0x49, 0x8d, 0x8f, ...u32(DATA_BIN_OFF));
  emit(0xba, ...u32(0x40000000));
  emit(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
  emit(0xc7, 0x44, 0x24, 0x20, 2, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 8, 0, 0, 0);
  emit(0x48, 0x83, 0xf8, 0xff);
  const jeFailW3 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xc3);
  emit(0x48, 0x8b, 0x54, 0x24, 0x60);
  emit(0x8b, 0x44, 0x24, 0x58);
  emit(0x48, 0x01, 0xc2); // rdx = dest+code_len
  emit(0x44, 0x8b, 0x44, 0x24, 0x5c);
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, 0x18, 0, 0, 0);
  emit(0x85, 0xc0);
  const jeFailW4 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, 0x20, 0, 0, 0);

  // stack still 0x78; try_place expects that frame
  const jmpPlace = parts.length;
  emit(0xe9, ...u32(0));

  const failAt = parts.length;
  emit(0xb8, 1, 0, 0, 0);
  emit(0x48, 0x83, 0xc4, 0x78);
  emit(0xc3);

  const fbAt = parts.length;
  emit(0x48, 0x83, 0xc4, 0x78);
  emit(...origPrologue);
  const jmpBack = parts.length;
  emit(0xe9, ...u32(0));

  const buf = Buffer.from(parts);
  function patchRel(at, targetRva) {
    const isJcc = buf[at] === 0x0f;
    const immAt = isJcc ? at + 2 : at + 1;
    const nextRva = handlerRva + (isJcc ? at + 6 : at + 5);
    buf.writeInt32LE((targetRva - nextRva) | 0, immAt);
  }
  const H = handlerRva;
  patchRel(jeFb1, H + fbAt);
  patchRel(jeFb2, H + fbAt);
  patchRel(jeFb3, H + fbAt);
  patchRel(jneFb, H + fbAt);
  patchRel(jneFbTyb, H + fbAt);
  patchRel(jeFail, H + failAt);
  patchRel(jeDone, H + doneAt);
  patchRel(jneTryA0, H + tryA0);
  patchRel(jbA1end, H + a1end);
  patchRel(jbA1end2, H + a1end);
  patchRel(jmpLoop1, H + loopAt);
  patchRel(jneSkip, H + skipRec);
  patchRel(jmpLoop2, H + loopAt);
  patchRel(jmpLoop3, H + loopAt);
  patchRel(jeFailW1, H + failAt);
  patchRel(jeFailW2, H + failAt);
  patchRel(jeFailW3, H + failAt);
  patchRel(jeFailW4, H + failAt);
  patchRel(jmpPlace, tryPlaceRva);
  patchRel(jmpBack, codeEntryRva + 11);
  return buf;
}

function main() {
  const inPe = process.argv[2];
  const outPe = process.argv[3] || inPe;
  if (!inPe || !fs.existsSync(inPe)) {
    console.log('SEMCOMP status=RED reason=missing_pe');
    process.exit(1);
  }

  const code = fs.readFileSync(path.join(payload, 'code.bin'));
  const data = fs.readFileSync(path.join(payload, 'data.bin'));
  const plant = packPlant(Buffer.concat([code, data]));
  const oneshot = Buffer.alloc(12 + plant.length);
  oneshot.write('Y1PL', 0, 4, 'ascii');
  oneshot.writeUInt32LE(code.length, 4);
  oneshot.writeUInt32LE(data.length, 8);
  plant.copy(oneshot, 12);
  fs.writeFileSync(path.join(projects, 'yoyo-semcomp-self.tyb'), oneshot);
  fs.writeFileSync(
    path.join(projects, 'yoyo-semcomp-self.ty'),
    '# yoyo-semcomp-self — AUTO-SEMCOMP\n# Y1PL | code_len | data_len | plant TYB(A1 of code||data)\n',
  );

  let pe = Buffer.from(fs.readFileSync(inPe));
  let t = peText(pe);
  const codeEntryRva = t.va + 0x0d;
  const codeEntryOff = t.rp + 0x0d;
  if (pe[codeEntryOff] === 0xe9) {
    console.log('SEMCOMP status=RED reason=already_patched');
    process.exit(1);
  }
  const origPrologue = [...pe.subarray(codeEntryOff, codeEntryOff + 11)];
  const tryPlaceRva = codeEntryRva + TRY_PLACE_OFF_IN_CODE;

  const used = 13 + 15596;
  const needPad = 700;
  if (t.rs - used < needPad) {
    const grow = 1024;
    const optSize = pe.readUInt16LE(t.e + 20);
    const sec = t.e + 24 + optSize;
    const dataSec = sec + 40;
    const oldDataRp = pe.readUInt32LE(dataSec + 20);
    const dataRs = pe.readUInt32LE(dataSec + 16);
    const dataSlice = Buffer.from(pe.subarray(oldDataRp, oldDataRp + dataRs));
    const newTextRs = t.rs + grow;
    const newDataRp = t.rp + newTextRs;
    const newPe = Buffer.alloc(newDataRp + dataRs, 0);
    pe.copy(newPe, 0, 0, t.rp + t.rs);
    dataSlice.copy(newPe, newDataRp);
    newPe.writeUInt32LE(newTextRs, sec + 16);
    newPe.writeUInt32LE(newDataRp, dataSec + 20);
    pe = newPe;
    t = peText(pe);
    console.log('SEMCOMP text_raw_grow=' + grow + ' new_rs=' + t.rs);
  }

  const handlerOff = t.rp + used;
  const handlerRva = t.va + used;
  const pad = t.rs - used;
  const handler = buildHandler(handlerRva, codeEntryRva, origPrologue, tryPlaceRva);
  if (handler.length > pad) {
    console.log('SEMCOMP status=RED reason=handler_too_big need=' + handler.length + ' pad=' + pad);
    process.exit(1);
  }
  handler.copy(pe, handlerOff);
  pe[codeEntryOff] = 0xe9;
  pe.writeInt32LE(handlerRva - (codeEntryRva + 5), codeEntryOff + 1);

  fs.mkdirSync(path.dirname(path.resolve(outPe)), { recursive: true });
  fs.writeFileSync(outPe, pe);

  const sha = crypto.createHash('sha256').update(pe).digest('hex');
  const tsha = crypto.createHash('sha256').update(oneshot).digest('hex');
  console.log('SEMCOMP pe_bytes=' + pe.length + ' sha256_prefix=' + sha.slice(0, 16));
  console.log('SEMCOMP handler_len=' + handler.length + ' pad=' + pad);
  console.log('SEMCOMP oneshot_bytes=' + oneshot.length + ' sha256_prefix=' + tsha.slice(0, 16));
  console.log('SEMCOMP plant_tyb_bytes=' + plant.length);
  console.log('SEMCOMP code_bytes=' + code.length + ' data_bytes=' + data.length);
  console.log('SEMCOMP out=' + outPe);
  console.log('SEMCOMP status=GREEN');
  process.exit(0);
}

main();
