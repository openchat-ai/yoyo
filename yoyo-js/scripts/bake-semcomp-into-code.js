'use strict';
/**
 * AUTO-SEMNATIVE: bake Y1PL plant-A1 handler into code.bin (no PE trampoline poke).
 * At cmp-edi-0x10 site: jmp handler; Y1PL → unpack A1 → write bins → try_place_bins.
 * Stock place C1 then understands yoyo-semcomp-self.tyb natively.
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

const CODE_BIN_OFF = 0x1af;
const DATA_BIN_OFF = 0x1b8;
const TRY_PLACE = 0x963;
const RET_FAST = 0xa1; // cmp edi, 0x10
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

/** rsi=partial file buf (stock only ReadFile 0x8000), edi=nread, r15=IAT+names, rsp 0x78. */
function buildHandler(handlerOff) {
  const parts = [];
  const emit = (...xs) => {
    for (const x of xs) {
      if (typeof x === 'number') parts.push(x);
      else parts.push(...x);
    }
  };
  const fails = [];

  // magic Y1PL? (first 4 bytes already in the 0x8000 window)
  emit(0x81, 0x3e, ...u32(0x4c503159));
  const jneBack = parts.length;
  emit(0x0f, 0x85, ...u32(0));

  // stock scratch is only 0x8000 — re-open input.tyb and ReadFile 1MB for full plant
  emit(0x49, 0x8d, 0x8f, ...u32(0x120)); // input.tyb
  emit(0xba, ...u32(0x80000000));
  emit(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
  emit(0xc7, 0x44, 0x24, 0x20, 3, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 8, 0, 0, 0);
  emit(0x48, 0x83, 0xf8, 0xff);
  const jeFailOpen = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  fails.push(jeFailOpen);
  emit(0x48, 0x89, 0xc3);

  emit(0x31, 0xc9);
  emit(0xba, ...u32(0x100000));
  emit(0x41, 0xb8, 0, 0x30, 0, 0);
  emit(0x41, 0xb9, 4, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0, 0, 0, 0);
  emit(0x48, 0x85, 0xc0);
  const jeFailVa = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  fails.push(jeFailVa);
  emit(0x48, 0x89, 0x44, 0x24, 0x50); // full file buf

  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x8b, 0x54, 0x24, 0x50);
  emit(0x41, 0xb8, ...u32(0x100000));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0x10, 0, 0, 0);
  emit(0x85, 0xc0);
  const jeFailRd = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  fails.push(jeFailRd);
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, 0x20, 0, 0, 0);

  emit(0x48, 0x8b, 0x74, 0x24, 0x50); // rsi = full file
  emit(0x81, 0x3e, ...u32(0x4c503159));
  const jneFailMagic = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  fails.push(jneFailMagic);

  emit(0x8b, 0x4e, 0x04);
  emit(0x89, 0x4c, 0x24, 0x58);
  emit(0x8b, 0x4e, 0x08);
  emit(0x89, 0x4c, 0x24, 0x5c);
  emit(0x48, 0x8d, 0x76, 0x0c);

  emit(0x81, 0x3e, ...u32(0x00425954));
  const jneFail = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  fails.push(jneFail);

  emit(0x31, 0xc9);
  emit(0x8b, 0x54, 0x24, 0x58);
  emit(0x03, 0x54, 0x24, 0x5c);
  emit(0x41, 0xb8, 0, 0x30, 0, 0);
  emit(0x41, 0xb9, 4, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0, 0, 0, 0);
  emit(0x48, 0x85, 0xc0);
  const jeFail = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  fails.push(jeFail);
  emit(0x48, 0x89, 0xc7);
  emit(0x48, 0x89, 0x44, 0x24, 0x60);

  emit(0x48, 0x83, 0xc6, 0x08);
  const loopAt = parts.length;
  emit(0x8a, 0x06);
  emit(0x3c, 0xff);
  const jeDone = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x3c, 0xa1);
  const jneA0 = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  emit(0x8a, 0x46, 0x02);
  emit(0xaa);
  emit(0x80, 0x7e, 0x01, 0x02);
  const jb1 = parts.length;
  emit(0x0f, 0x82, ...u32(0));
  emit(0x8a, 0x46, 0x04);
  emit(0xaa);
  emit(0x80, 0x7e, 0x01, 0x03);
  const jb2 = parts.length;
  emit(0x0f, 0x82, ...u32(0));
  emit(0x8a, 0x46, 0x06);
  emit(0xaa);
  const a1end = parts.length;
  emit(0x48, 0x83, 0xc6, 0x08);
  const jmpL1 = parts.length;
  emit(0xe9, ...u32(0));

  const tryA0 = parts.length;
  emit(0x3c, 0xa0);
  const jneSk = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  emit(0x8a, 0x46, 0x02);
  emit(0xaa);
  emit(0x48, 0x83, 0xc6, 0x08);
  const jmpL2 = parts.length;
  emit(0xe9, ...u32(0));

  const skip = parts.length;
  emit(0x48, 0x83, 0xc6, 0x08);
  const jmpL3 = parts.length;
  emit(0xe9, ...u32(0));

  const doneAt = parts.length;
  function writeBin(pathOff, setupRdx) {
    emit(0x49, 0x8d, 0x8f, ...u32(pathOff));
    emit(0xba, ...u32(0x40000000));
    emit(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
    emit(0xc7, 0x44, 0x24, 0x20, 2, 0, 0, 0);
    emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0, 0, 0);
    emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
    emit(0x41, 0xff, 0x97, 8, 0, 0, 0);
    emit(0x48, 0x83, 0xf8, 0xff);
    const j1 = parts.length;
    emit(0x0f, 0x84, ...u32(0));
    emit(0x48, 0x89, 0xc3);
    setupRdx();
    emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
    emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
    emit(0x48, 0x89, 0xd9);
    emit(0x41, 0xff, 0x97, 0x18, 0, 0, 0);
    emit(0x85, 0xc0);
    const j2 = parts.length;
    emit(0x0f, 0x84, ...u32(0));
    emit(0x48, 0x89, 0xd9);
    emit(0x41, 0xff, 0x97, 0x20, 0, 0, 0);
    return [j1, j2];
  }
  fails.push(
    ...writeBin(CODE_BIN_OFF, () => {
      emit(0x48, 0x8b, 0x54, 0x24, 0x60);
      emit(0x44, 0x8b, 0x44, 0x24, 0x58);
    }),
  );
  fails.push(
    ...writeBin(DATA_BIN_OFF, () => {
      emit(0x48, 0x8b, 0x54, 0x24, 0x60);
      emit(0x8b, 0x44, 0x24, 0x58);
      emit(0x48, 0x01, 0xc2);
      emit(0x44, 0x8b, 0x44, 0x24, 0x5c);
    }),
  );

  const jmpPlace = parts.length;
  emit(0xe9, ...u32(0));

  const failAt = parts.length;
  emit(0xb8, 1, 0, 0, 0);
  emit(0x48, 0x83, 0xc4, 0x78);
  emit(0xc3);

  // back: original cmp edi,0x10; jne 0xdd; jmp 0xaa (TYB check)
  const backAt = parts.length;
  emit(0x83, 0xff, 0x10);
  const jneOrig = parts.length;
  emit(0x0f, 0x85, ...u32(0));
  const jmpCont = parts.length;
  emit(0xe9, ...u32(0));

  const buf = Buffer.from(parts);
  function patchRel(at, targetAbs) {
    const isJcc = buf[at] === 0x0f;
    const immAt = isJcc ? at + 2 : at + 1;
    const next = isJcc ? at + 6 : at + 5;
    buf.writeInt32LE((targetAbs - (handlerOff + next)) | 0, immAt);
  }
  const H = handlerOff;
  patchRel(jneBack, H + backAt);
  patchRel(jeDone, H + doneAt);
  patchRel(jneA0, H + tryA0);
  patchRel(jb1, H + a1end);
  patchRel(jb2, H + a1end);
  patchRel(jmpL1, H + loopAt);
  patchRel(jneSk, H + skip);
  patchRel(jmpL2, H + loopAt);
  patchRel(jmpL3, H + loopAt);
  for (const f of fails) patchRel(f, H + failAt);
  patchRel(jmpPlace, TRY_PLACE);
  patchRel(jneOrig, 0xdd);
  patchRel(jmpCont, 0xaa);
  return buf;
}

function main() {
  const oldCode = fs.readFileSync(codePath);
  const data = fs.readFileSync(dataPath);
  if (oldCode.length !== OLD_CODE_LEN) {
    console.log('SEMNATIVE status=RED reason=code_len got=' + oldCode.length);
    process.exit(1);
  }
  if (oldCode[RET_FAST] === 0xe9) {
    console.log('SEMNATIVE status=RED reason=already_baked');
    process.exit(1);
  }

  const handlerOff = oldCode.length;
  const handler = buildHandler(handlerOff);
  const newCode = Buffer.concat([oldCode, handler]);
  const Lold = layoutFor(OLD_CODE_LEN, data.length, 0x38000);
  const Lnew = layoutFor(newCode.length, data.length, 0x38000);

  // trampoline at RET_FAST
  const tramp = Buffer.alloc(5);
  tramp[0] = 0xe9;
  tramp.writeInt32LE(handlerOff - (RET_FAST + 5), 1);
  tramp.copy(newCode, RET_FAST);

  function replaceImm(oldV, newV, sites) {
    for (const s of sites) {
      if (newCode.readUInt32LE(s) !== oldV) {
        throw new Error('imm mismatch at 0x' + s.toString(16));
      }
      newCode.writeUInt32LE(newV >>> 0, s);
    }
  }
  replaceImm(OLD_CODE_LEN, newCode.length, [0x8b2, 0x98e, 0xa22, 0xa4a, 0x115d]);
  replaceImm(Lold.peSize, Lnew.peSize, [0x7bc, 0x828, 0x850, 0x966, 0xae9, 0x1182]);
  replaceImm(Lold.codeRaw, Lnew.codeRaw, [0xc8b, 0xd30]);
  replaceImm(Lold.dataFileOff, Lnew.dataFileOff, [0xdc6, 0xe45]);
  replaceImm(Lold.userDataOff, Lnew.userDataOff, [0x928, 0x116b]);
  // emitCopyIatBlob stores each qword at dataFileOff+k — must bump every imm, not only +0
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

  // plant halves: place 4491 | rest (parse1 grows)
  const place = newCode.subarray(0, 4491);
  const parse1 = newCode.subarray(4491);
  fs.writeFileSync(path.join(projects, 'yoyo-plant-place.tyb'), packPlant(place));
  fs.writeFileSync(path.join(projects, 'yoyo-plant-parse1.tyb'), packPlant(parse1));
  fs.writeFileSync(path.join(projects, 'yoyo-plant.tyb'), packPlant(newCode));

  const plant = packPlant(Buffer.concat([newCode, data]));
  const oneshot = Buffer.alloc(12 + plant.length);
  oneshot.write('Y1PL', 0, 4, 'ascii');
  oneshot.writeUInt32LE(newCode.length, 4);
  oneshot.writeUInt32LE(data.length, 8);
  plant.copy(oneshot, 12);
  fs.writeFileSync(path.join(projects, 'yoyo-semcomp-self.tyb'), oneshot);
  fs.writeFileSync(
    path.join(projects, 'yoyo-semcomp-self.ty'),
    '# yoyo-semcomp-self — AUTO-SEMNATIVE\n# Y1PL | code_len | data_len | A1(code||data) — baked into code.bin\n',
  );

  const codeSha = crypto.createHash('sha256').update(newCode).digest('hex');
  const placeSha = crypto.createHash('sha256').update(fs.readFileSync(path.join(projects, 'yoyo-plant-place.tyb'))).digest('hex');
  const parse1Sha = crypto.createHash('sha256').update(fs.readFileSync(path.join(projects, 'yoyo-plant-parse1.tyb'))).digest('hex');
  const oneshotSha = crypto.createHash('sha256').update(oneshot).digest('hex');

  let pin = fs.readFileSync(path.join(payload, 'payload.pin'), 'utf8');
  pin = pin.replace(/code_bytes=\d+/, 'code_bytes=' + newCode.length);
  pin = pin.replace(/code_sha256=[0-9a-f]+/, 'code_sha256=' + codeSha);
  pin = pin.replace(/code_sha256_prefix=[0-9a-f]+/, 'code_sha256_prefix=' + codeSha.slice(0, 16));
  pin = pin.replace(/code_parse1_bytes=\d+/, 'code_parse1_bytes=' + parse1.length);
  pin = pin.replace(/code_plant_place_sha256=[0-9a-f]+/, 'code_plant_place_sha256=' + placeSha);
  pin = pin.replace(/code_plant_parse1_sha256=[0-9a-f]+/, 'code_plant_parse1_sha256=' + parse1Sha);
  pin = pin.replace(/yoyo_semcomp_self_tyb_sha256=[0-9a-f]+/, 'yoyo_semcomp_self_tyb_sha256=' + oneshotSha);
  pin = pin.replace(
    /yoyo_semcomp_self_kind=[^\r\n]+/,
    'yoyo_semcomp_self_kind=y1pl_baked_in_code_bin_native',
  );
  pin = pin.replace(
    /note=SEMCOMP_[^\r\n]+/,
    'note=SEMNATIVE_y1pl_baked_in_code_bin;stock_runs_T;no_pe_trampoline_poke;not_Thompson',
  );
  pin = pin.replace(/doc=AUTO-SEMCOMP.md/, 'doc=AUTO-SEMNATIVE.md');
  fs.writeFileSync(path.join(payload, 'payload.pin'), pin);

  console.log('SEMNATIVE code_bytes=' + newCode.length + ' handler=' + handler.length);
  console.log('SEMNATIVE code_sha256_prefix=' + codeSha.slice(0, 16));
  console.log('SEMNATIVE peSize ' + Lold.peSize + '→' + Lnew.peSize + ' codeRaw ' + Lold.codeRaw + '→' + Lnew.codeRaw);
  console.log('SEMNATIVE oneshot_bytes=' + oneshot.length + ' sha256_prefix=' + oneshotSha.slice(0, 16));
  console.log('SEMNATIVE status=GREEN');
  process.exit(0);
}

main();
