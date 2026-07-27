'use strict';
/**
 * pe-builder.js — PE32+ wrapper (JS peer of yoyo-rust/pe_link.rs).
 * Data section size floor = OUTPUT_DATA_NEED (0x38000) — Phase 2 fix.
 */

const { TEXT_RVA, IMAGE_BASE } = require('./platform-config');

function alignUp(v, a) { return (v + a - 1) & ~(a - 1); }

function buildPe(code, data, dataNeed) {
  const sectionAlign = 0x1000;
  const fileAlign = 0x200;
  const headersRaw = 0x400;
  const startupLen = 13;
  const codeRaw = alignUp(code.length + startupLen, fileAlign);
  const dataVs = Math.max(dataNeed, alignUp(data.length + 0x1000, sectionAlign));
  const dataRaw = alignUp(dataVs, fileAlign);
  const textRva = TEXT_RVA;
  const textVs = alignUp(code.length + startupLen, sectionAlign);
  const dataRva = textRva + textVs;
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
  img.writeUInt16LE(6, opt + 44);
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
  // lea r15, [rip+disp]
  const leaDisp = dataRva - (textRva + 7);
  img[textOff] = 0x4c; img[textOff + 1] = 0x8d; img[textOff + 2] = 0x3d;
  img.writeInt32LE(leaDisp, textOff + 3);
  // jmp to user code
  const jmpRel = (textRva + startupLen) - (textRva + 7 + 5);
  img[textOff + 7] = 0xe9;
  img.writeInt32LE(jmpRel, textOff + 8);
  img[textOff + 12] = 0x90;
  code.copy(img, textOff + startupLen);
  data.copy(img, headersRaw + codeRaw, 0, Math.min(data.length, dataRaw));
  return img;
}

module.exports = { buildPe };
