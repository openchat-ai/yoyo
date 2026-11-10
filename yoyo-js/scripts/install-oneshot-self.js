'use strict';
/**
 * AUTO-ONESELF: PE .text-padding trampoline.
 * input.tyb = Y1SF | u32 pe_len | pe_bytes
 * → write pe_bytes to output.exe (P(T)=P).
 * Non-oneshot → original entry prologue.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');

function u32(n) {
  const b = Buffer.alloc(4);
  b.writeUInt32LE(n >>> 0);
  return [...b];
}

function peText(pe) {
  const e = pe.readUInt32LE(0x3c);
  const optSize = pe.readUInt16LE(e + 20);
  const sec = e + 24 + optSize;
  return {
    e,
    entryRva: pe.readUInt32LE(e + 24 + 16),
    va: pe.readUInt32LE(sec + 12),
    vs: pe.readUInt32LE(sec + 8),
    rs: pe.readUInt32LE(sec + 16),
    rp: pe.readUInt32LE(sec + 20),
  };
}

/** Build oneshot+fallback handler. origPrologue = first 11 bytes (sub+lea). */
function buildHandler(handlerRva, codeEntryRva, origPrologue) {
  const parts = [];
  const emit = (...xs) => {
    for (const x of xs) {
      if (typeof x === 'number') parts.push(x);
      else parts.push(...x);
    }
  };

  // Prologue stack (match original)
  emit(0x48, 0x83, 0xec, 0x78);

  // CreateFileA(input.tyb) — path at r15+0x120
  emit(0x49, 0x8d, 0x8f, ...u32(0x120));
  emit(0xba, ...u32(0x80000000));
  emit(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
  emit(0xc7, 0x44, 0x24, 0x20, 3, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 8, 0, 0, 0);
  emit(0x48, 0x83, 0xf8, 0xff);
  const jeFallback1 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xc3);

  // VirtualAlloc 0x80000
  emit(0x31, 0xc9);
  emit(0xba, ...u32(0x80000));
  emit(0x41, 0xb8, 0, 0x30, 0, 0);
  emit(0x41, 0xb9, 4, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0, 0, 0, 0);
  emit(0x48, 0x85, 0xc0);
  const jeFallback2 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xc6);

  // ReadFile
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x89, 0xf2);
  emit(0x41, 0xb8, ...u32(0x80000));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0x10, 0, 0, 0);
  emit(0x85, 0xc0);
  const jeFallback3 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, 0x20, 0, 0, 0);

  // magic Y1SF?
  emit(0x81, 0x3e, ...u32(0x46533159));
  const jneFallback = parts.length;
  emit(0x0f, 0x85, ...u32(0));

  // pe_len = [rsi+4]; save payload ptr rsi+8 at [rsp+0x48]
  emit(0x8b, 0x7e, 0x04);
  emit(0x48, 0x8d, 0x46, 0x08);
  emit(0x48, 0x89, 0x44, 0x24, 0x48);

  // CreateFileA(output.exe) r15+0x12a
  emit(0x49, 0x8d, 0x8f, ...u32(0x12a));
  emit(0xba, ...u32(0x40000000));
  emit(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
  emit(0xc7, 0x44, 0x24, 0x20, 2, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 8, 0, 0, 0);
  emit(0x48, 0x83, 0xf8, 0xff);
  const jeFail = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xc3);

  // WriteFile(h, payload, pe_len, &n, 0)
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x8b, 0x54, 0x24, 0x48);
  emit(0x41, 0x89, 0xf8);
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, 0x18, 0, 0, 0);
  emit(0x85, 0xc0);
  const jeFail2 = parts.length;
  emit(0x0f, 0x84, ...u32(0));
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, 0x20, 0, 0, 0);
  emit(0x31, 0xc0);
  emit(0x48, 0x83, 0xc4, 0x78);
  emit(0xc3);

  const failAt = parts.length;
  emit(0xb8, 1, 0, 0, 0);
  emit(0x48, 0x83, 0xc4, 0x78);
  emit(0xc3);

  // fallback: undo our sub; replay sub+lea (11B); jmp codeEntry+11
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
  patchRel(jeFallback1, handlerRva + fbAt);
  patchRel(jeFallback2, handlerRva + fbAt);
  patchRel(jeFallback3, handlerRva + fbAt);
  patchRel(jneFallback, handlerRva + fbAt);
  patchRel(jeFail, handlerRva + failAt);
  patchRel(jeFail2, handlerRva + failAt);
  patchRel(jmpBack, codeEntryRva + 11);
  return buf;
}

function main() {
  const inPe = process.argv[2];
  const outPe = process.argv[3] || inPe;
  if (!inPe || !fs.existsSync(inPe)) {
    console.log('ONESELF status=RED reason=missing_pe');
    process.exit(1);
  }
  let pe = Buffer.from(fs.readFileSync(inPe));
  let t = peText(pe);
  const codeEntryRva = t.va + 0x0d; // startup 13 bytes then code.bin
  const codeEntryOff = t.rp + 0x0d;
  if (pe[codeEntryOff] === 0xe9) {
    console.log('ONESELF status=RED reason=already_patched');
    process.exit(1);
  }
  const origPrologue = [...pe.subarray(codeEntryOff, codeEntryOff + 11)];

  // Grow .text raw by one file-align if needed (shift .data in file)
  const used = 13 + 15596;
  const needPad = 400; // oneshot handler budget
  if (t.rs - used < needPad) {
    const grow = 512;
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
    newPe.writeUInt32LE(newTextRs, sec + 16); // .text SizeOfRawData
    newPe.writeUInt32LE(newDataRp, dataSec + 20); // .data PointerToRawData
    // SizeOfImage unchanged if vs already covers; optional SizeOfInitializedData
    pe = newPe;
    t = peText(pe);
    console.log('ONESELF text_raw_grow=' + grow + ' new_rs=' + t.rs);
  }

  const handlerOff = t.rp + used;
  const handlerRva = t.va + used;
  const pad = t.rs - used;
  const handler = buildHandler(handlerRva, codeEntryRva, origPrologue);
  if (handler.length > pad) {
    console.log('ONESELF status=RED reason=handler_too_big need=' + handler.length + ' pad=' + pad);
    process.exit(1);
  }
  handler.copy(pe, handlerOff);

  // trampoline at code entry
  pe[codeEntryOff] = 0xe9;
  pe.writeInt32LE(handlerRva - (codeEntryRva + 5), codeEntryOff + 1);

  fs.mkdirSync(path.dirname(path.resolve(outPe)), { recursive: true });
  fs.writeFileSync(outPe, pe);

  // Build oneshot tyb embedding this PE
  const oneshot = Buffer.alloc(8 + pe.length);
  oneshot.write('Y1SF', 0, 4, 'ascii');
  oneshot.writeUInt32LE(pe.length, 4);
  pe.copy(oneshot, 8);
  fs.writeFileSync(path.join(projects, 'yoyo-oneshot-self.tyb'), oneshot);
  fs.writeFileSync(
    path.join(projects, 'yoyo-oneshot-self.ty'),
    '# yoyo-oneshot-self — AUTO-ONESELF\n# Y1SF | pe_len | pe_bytes  → P(T)=P\n',
  );

  const sha = crypto.createHash('sha256').update(pe).digest('hex');
  const tsha = crypto.createHash('sha256').update(oneshot).digest('hex');
  console.log('ONESELF pe_bytes=' + pe.length + ' sha256_prefix=' + sha.slice(0, 16));
  console.log('ONESELF handler_len=' + handler.length + ' pad=' + pad);
  console.log('ONESELF oneshot_bytes=' + oneshot.length + ' sha256_prefix=' + tsha.slice(0, 16));
  console.log('ONESELF out=' + outPe);
  console.log('ONESELF status=GREEN');
  process.exit(0);
}

main();
