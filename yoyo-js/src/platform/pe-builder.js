'use strict';
/**
 * pe-builder.js — PE32+ wrapper (JS peer of yoyo-rust/pe_link.rs).
 * Data section size floor = OUTPUT_DATA_NEED (0x38000) — Phase 2 fix.
 * Stage 9-B: prepend kernel32 IAT at r15+0 for real Win32 I/O emit.
 */

const { TEXT_RVA, IMAGE_BASE } = require('./platform-config');
const { KERNEL32_IO_FUNCS } = require('./platform-io');

function alignUp(v, a) { return (v + a - 1) & ~(a - 1); }

/**
 * Prepend kernel32 IAT blob at start of .data (r15+0), matching pe_link.rs.
 * Returns { data, importDirRva, importDirSize }.
 */
function prependWin32IoIat(userData, dataRva) {
  const n = KERNEL32_IO_FUNCS.length;
  const descSize = 40;
  const kernel32Name = Buffer.from('kernel32.dll\0');
  const iatSlotsOff = 0;

  const hintNames = KERNEL32_IO_FUNCS.map((name) => {
    const nm = Buffer.from(name + '\0');
    const pad = (nm.length + 2) % 2 === 1 ? 1 : 0;
    const hn = Buffer.alloc(2 + nm.length + pad);
    hn.writeUInt16LE(0, 0);
    nm.copy(hn, 2);
    return hn;
  });

  const descOff = (n + 1) * 8;
  const kernOff = descOff + descSize;
  const hnStart = kernOff + kernel32Name.length;
  let hnOff = hnStart;
  const hnRvas = [];
  for (const hn of hintNames) {
    hnRvas.push(dataRva + hnOff);
    hnOff += hn.length;
  }
  const iltOff = hnOff;
  const headerEnd = iltOff + (n + 1) * 8;
  const pad = alignUp(headerEnd, 16);
  const blob = Buffer.alloc(pad + userData.length, 0);

  blob.writeUInt32LE(dataRva + iltOff, descOff);
  blob.writeUInt32LE(dataRva + kernOff, descOff + 12);
  blob.writeUInt32LE(dataRva + iatSlotsOff, descOff + 16);
  kernel32Name.copy(blob, kernOff);

  let off = hnStart;
  for (const hn of hintNames) {
    hn.copy(blob, off);
    off += hn.length;
  }
  for (let i = 0; i < hnRvas.length; i++) {
    blob.writeBigUInt64LE(BigInt(hnRvas[i]), iltOff + i * 8);
    blob.writeBigUInt64LE(BigInt(hnRvas[i]), iatSlotsOff + i * 8);
  }
  userData.copy(blob, pad);
  return { data: blob, importDirRva: dataRva + descOff, importDirSize: descSize };
}

function buildPe(code, data, dataNeed) {
  const sectionAlign = 0x1000;
  const fileAlign = 0x200;
  const headersRaw = 0x400;
  const startupLen = 13;
  const codeRaw = alignUp(code.length + startupLen, fileAlign);
  const textRva = TEXT_RVA;
  const textVs = alignUp(code.length + startupLen, sectionAlign);
  const dataRva = textRva + textVs;

  const userData = Buffer.isBuffer(data) ? data : Buffer.from(data || []);
  const { data: extended, importDirRva, importDirSize } = prependWin32IoIat(userData, dataRva);

  const dataVs = Math.max(dataNeed, alignUp(extended.length + 0x1000, sectionAlign));
  const dataRaw = alignUp(dataVs, fileAlign);
  const sizeOfImage = alignUp(dataRva + dataVs, sectionAlign);

  const img = Buffer.alloc(headersRaw + codeRaw + dataRaw, 0);
  img.write('MZ', 0);
  img.writeUInt32LE(0x80, 0x3c);
  img.write('PE\0\0', 0x80);
  img.writeUInt16LE(0x8664, 0x84);
  img.writeUInt16LE(2, 0x86);
  img.writeUInt16LE(0xf0, 0x94);
  img.writeUInt16LE(0x22, 0x96);

  const opt = 0x98;
  img.writeUInt16LE(0x20b, opt);
  img[opt + 2] = 1;
  img.writeUInt32LE(textRva, opt + 16);
  img.writeBigUInt64LE(IMAGE_BASE, opt + 24);
  img.writeUInt32LE(sectionAlign, opt + 32);
  img.writeUInt32LE(fileAlign, opt + 36);
  img.writeUInt16LE(6, opt + 40);
  img.writeUInt16LE(0, opt + 42);
  img.writeUInt16LE(0, opt + 44);
  img.writeUInt16LE(0, opt + 46);
  img.writeUInt16LE(6, opt + 48);
  img.writeUInt16LE(0, opt + 50);
  img.writeUInt32LE(sizeOfImage, opt + 56);
  img.writeUInt32LE(headersRaw, opt + 60);
  img.writeUInt16LE(3, opt + 68);
  img.writeUInt16LE(0x8160, opt + 70);
  img.writeBigUInt64LE(0x100000n, opt + 72);
  img.writeBigUInt64LE(0x1000n, opt + 80);
  img.writeBigUInt64LE(0x100000n, opt + 88);
  img.writeBigUInt64LE(0x1000n, opt + 96);
  img.writeUInt32LE(16, opt + 108);
  img.writeUInt32LE(codeRaw, opt + 4);
  img.writeUInt32LE(dataRaw, opt + 8);
  img.writeUInt32LE(textRva, opt + 20);
  // DataDirectory[1] = Import Table
  img.writeUInt32LE(importDirRva, opt + 120);
  img.writeUInt32LE(importDirSize, opt + 124);

  const s1 = opt + 0xf0;
  img.write('.text\0\0\0', s1);
  img.writeUInt32LE(textVs, s1 + 8);
  img.writeUInt32LE(textRva, s1 + 12);
  img.writeUInt32LE(codeRaw, s1 + 16);
  img.writeUInt32LE(headersRaw, s1 + 20);
  img.writeUInt32LE(0x60000020, s1 + 36);

  const s2 = s1 + 40;
  img.write('.data\0\0\0', s2);
  img.writeUInt32LE(dataVs, s2 + 8);
  img.writeUInt32LE(dataRva, s2 + 12);
  img.writeUInt32LE(dataRaw, s2 + 16);
  img.writeUInt32LE(headersRaw + codeRaw, s2 + 20);
  img.writeUInt32LE(0xc0000040, s2 + 36);

  const textOff = headersRaw;
  const leaDisp = dataRva - (textRva + 7);
  img[textOff] = 0x4c; img[textOff + 1] = 0x8d; img[textOff + 2] = 0x3d;
  img.writeInt32LE(leaDisp, textOff + 3);
  const jmpRel = (textRva + startupLen) - (textRva + 7 + 5);
  img[textOff + 7] = 0xe9;
  img.writeInt32LE(jmpRel, textOff + 8);
  img[textOff + 12] = 0x90;
  code.copy(img, textOff + startupLen);
  extended.copy(img, headersRaw + codeRaw, 0, Math.min(extended.length, dataRaw));
  return img;
}

module.exports = { buildPe, prependWin32IoIat };
