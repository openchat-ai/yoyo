'use strict';
/**
 * BD-4: native peer of pe-builder linkPeWin32 (buildPe + prependWin32IoIat).
 * Emits x64 into CodeAsm — no self-PE punch.
 */
const { prependWin32IoIat } = require('./pe-builder');
const { TEXT_RVA, IMAGE_BASE } = require('./platform-config');

function u32le(n) {
  const b = Buffer.alloc(4);
  b.writeUInt32LE(n >>> 0, 0);
  return [...b];
}

function alignUp(v, a) {
  return (v + a - 1) & ~(a - 1);
}

/** Layout constants matching linkPeWin32(code,data,0x38000). */
function layoutFor(codeLen, userDataLen, dataNeed = 0x38000) {
  const sectionAlign = 0x1000;
  const fileAlign = 0x200;
  const headersRaw = 0x400;
  const startupLen = 13;
  const codeRaw = alignUp(codeLen + startupLen, fileAlign);
  const textRva = TEXT_RVA;
  const textVs = alignUp(codeLen + startupLen, sectionAlign);
  const dataRva = textRva + textVs;
  const fakeUser = Buffer.alloc(userDataLen);
  const prep = prependWin32IoIat(fakeUser, dataRva);
  const iatLen = prep.data.length - userDataLen;
  const extendedLen = prep.data.length;
  const dataVs = Math.max(dataNeed, alignUp(extendedLen + 0x1000, sectionAlign));
  const dataRaw = alignUp(dataVs, fileAlign);
  const sizeOfImage = alignUp(dataRva + dataVs, sectionAlign);
  const peSize = headersRaw + codeRaw + dataRaw;
  return {
    sectionAlign,
    fileAlign,
    headersRaw,
    startupLen,
    codeRaw,
    textRva,
    textVs,
    dataRva,
    dataVs,
    dataRaw,
    sizeOfImage,
    peSize,
    iatLen,
    importDirRva: prep.importDirRva,
    importDirSize: prep.importDirSize,
    codeOff: headersRaw + startupLen,
    dataFileOff: headersRaw + codeRaw,
    userDataOff: headersRaw + codeRaw + iatLen,
    leaDisp: dataRva - (textRva + 7),
    jmpRel: textRva + startupLen - (textRva + 7 + 5),
    imageBase: typeof IMAGE_BASE === 'bigint' ? IMAGE_BASE : BigInt(IMAGE_BASE),
  };
}

function emitMovImm64Rax(asm, imm) {
  const b = Buffer.alloc(8);
  b.writeBigUInt64LE(BigInt(imm), 0);
  asm.raw(0x48, 0xb8, ...b);
}

function emitStoreU8(asm, baseRspOff, fileOff, val) {
  // mov byte [rax+fileOff], val  where rax=[rsp+base]
  asm.raw(0x48, 0x8b, 0x44, 0x24, baseRspOff);
  asm.raw(0xc6, 0x80, ...u32le(fileOff), val & 0xff);
}

function emitStoreU16(asm, baseRspOff, fileOff, val) {
  asm.raw(0x48, 0x8b, 0x44, 0x24, baseRspOff);
  asm.raw(0x66, 0xc7, 0x80, ...u32le(fileOff), val & 0xff, (val >> 8) & 0xff);
}

function emitStoreU32(asm, baseRspOff, fileOff, val) {
  asm.raw(0x48, 0x8b, 0x44, 0x24, baseRspOff);
  asm.raw(0xc7, 0x80, ...u32le(fileOff), ...u32le(val >>> 0));
}

function emitStoreU64(asm, baseRspOff, fileOff, val) {
  emitMovImm64Rax(asm, val);
  asm.raw(0x48, 0x8b, 0x4c, 0x24, baseRspOff); // rcx = pe
  asm.raw(0x48, 0x89, 0x81, ...u32le(fileOff)); // mov [rcx+off], rax
}

function emitCopyIatBlob(asm, peRspOff, dataFileOff, iatBuf) {
  // rax = pe + dataFileOff; copy iatBuf via movabs+store
  let off = 0;
  while (off + 8 <= iatBuf.length) {
    emitMovImm64Rax(asm, iatBuf.readBigUInt64LE(off));
    asm.raw(0x48, 0x8b, 0x4c, 0x24, peRspOff);
    asm.raw(0x48, 0x89, 0x81, ...u32le(dataFileOff + off));
    off += 8;
  }
  while (off < iatBuf.length) {
    emitStoreU8(asm, peRspOff, dataFileOff + off, iatBuf[off]);
    off += 1;
  }
}

/**
 * Replace punch path: link code.bin+data.bin → PE in memory → jmp do_write.
 */
function emitNativeLinkFromBins(asm, meta) {
  const codeLen = (meta.place && meta.place.code_len) || 13474;
  const dataLen = (meta.place && meta.place.data_len) || 414016; // TRUER7 trimmed default
  const L = layoutFor(codeLen, dataLen, 0x38000);
  // Build IAT-only prefix (user zeros) for embedding.
  const iatOnly = prependWin32IoIat(Buffer.alloc(0), L.dataRva).data;
  // prependWin32IoIat(empty) gives only IAT pad without user — length should be iatLen
  const iatBlob = iatOnly.length === L.iatLen ? iatOnly : iatOnly.slice(0, L.iatLen);

  const PE = 0x58;
  const CODE_BUF = 0x60;
  const DATA_BUF = 0x68;
  // need more stack — we're in 0x78 frame; 0x58/0x60/0x68 fit (0x70+ would be tight with 0x78)

  function readExact(pathOff, destRspOff, size, failSite) {
    asm.raw(0x49, 0x8d, 0x8f, ...u32le(pathOff));
    asm.raw(0xba, 0x00, 0x00, 0x00, 0x80);
    asm.raw(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
    asm.raw(0xc7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00);
    asm.raw(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00);
    asm.raw(0x48, 0xc7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00);
    asm.raw(0x41, 0xff, 0x97, 0x08, 0x00, 0x00, 0x00);
    asm.raw(0x48, 0x83, 0xf8, 0xff);
    asm.jccSite(failSite, 0x84);
    asm.raw(0x48, 0x89, 0xc3);
    asm.raw(0x48, 0x8b, 0x54, 0x24, destRspOff); // rdx = dest buf
    asm.raw(0x41, 0xb8, ...u32le(size));
    asm.raw(0x4c, 0x8d, 0x4c, 0x24, 0x40);
    asm.raw(0x48, 0xc7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00);
    asm.raw(0x48, 0x89, 0xd9);
    asm.raw(0x41, 0xff, 0x97, 0x10, 0x00, 0x00, 0x00);
    asm.raw(0x85, 0xc0);
    asm.jccSite(failSite + '_rd', 0x84);
    asm.raw(0x81, 0x7c, 0x24, 0x40, ...u32le(size));
    asm.jccSite(failSite + '_sz', 0x85);
    asm.raw(0x48, 0x89, 0xd9);
    asm.raw(0x41, 0xff, 0x97, 0x20, 0x00, 0x00, 0x00);
  }

  function alloc(size, saveOff, failSite) {
    asm.raw(0x31, 0xc9);
    asm.raw(0xba, ...u32le(size));
    asm.raw(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00);
    asm.raw(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00);
    asm.raw(0x41, 0xff, 0x97, 0x00, 0x00, 0x00, 0x00);
    asm.raw(0x48, 0x85, 0xc0);
    asm.jccSite(failSite, 0x84);
    asm.raw(0x48, 0x89, 0x44, 0x24, saveOff);
  }

  asm.lab('try_place_bins');

  alloc(L.peSize, PE, 'je_place_fail');
  alloc(codeLen, CODE_BUF, 'je_place_fail_codealloc');
  alloc(dataLen, DATA_BUF, 'je_place_fail_dataalloc');

  readExact(meta.code_bin_off, CODE_BUF, codeLen, 'je_place_fail_code');
  readExact(meta.data_bin_off, DATA_BUF, dataLen, 'je_place_fail_data');

  // zero PE
  asm.raw(0x48, 0x8b, 0x7c, 0x24, PE); // rdi = pe
  asm.raw(0x31, 0xc0);
  asm.raw(0xb9, ...u32le(L.peSize));
  asm.raw(0xf3, 0xaa);

  // MZ / PE headers (peer of buildPe)
  emitStoreU8(asm, PE, 0x00, 0x4d);
  emitStoreU8(asm, PE, 0x01, 0x5a);
  emitStoreU32(asm, PE, 0x3c, 0x80);
  emitStoreU8(asm, PE, 0x80, 0x50);
  emitStoreU8(asm, PE, 0x81, 0x45);
  emitStoreU16(asm, PE, 0x84, 0x8664);
  emitStoreU16(asm, PE, 0x86, 2);
  emitStoreU16(asm, PE, 0x94, 0xf0);
  emitStoreU16(asm, PE, 0x96, 0x22);

  const opt = 0x98;
  emitStoreU16(asm, PE, opt, 0x20b);
  emitStoreU8(asm, PE, opt + 2, 1);
  emitStoreU32(asm, PE, opt + 16, L.textRva);
  emitStoreU64(asm, PE, opt + 24, L.imageBase);
  emitStoreU32(asm, PE, opt + 32, L.sectionAlign);
  emitStoreU32(asm, PE, opt + 36, L.fileAlign);
  emitStoreU16(asm, PE, opt + 40, 6);
  emitStoreU16(asm, PE, opt + 48, 6);
  emitStoreU32(asm, PE, opt + 56, L.sizeOfImage);
  emitStoreU32(asm, PE, opt + 60, L.headersRaw);
  emitStoreU16(asm, PE, opt + 68, 3);
  emitStoreU16(asm, PE, opt + 70, 0x0100);
  emitStoreU64(asm, PE, opt + 72, 0x100000n);
  emitStoreU64(asm, PE, opt + 80, 0x1000n);
  emitStoreU64(asm, PE, opt + 88, 0x100000n);
  emitStoreU64(asm, PE, opt + 96, 0x1000n);
  emitStoreU32(asm, PE, opt + 108, 16);
  emitStoreU32(asm, PE, opt + 4, L.codeRaw);
  emitStoreU32(asm, PE, opt + 8, L.dataRaw);
  emitStoreU32(asm, PE, opt + 20, L.textRva);
  emitStoreU32(asm, PE, opt + 120, L.importDirRva);
  emitStoreU32(asm, PE, opt + 124, L.importDirSize);

  const s1 = opt + 0xf0;
  // .text
  emitStoreU8(asm, PE, s1 + 0, 0x2e);
  emitStoreU8(asm, PE, s1 + 1, 0x74);
  emitStoreU8(asm, PE, s1 + 2, 0x65);
  emitStoreU8(asm, PE, s1 + 3, 0x78);
  emitStoreU8(asm, PE, s1 + 4, 0x74);
  emitStoreU32(asm, PE, s1 + 8, L.textVs);
  emitStoreU32(asm, PE, s1 + 12, L.textRva);
  emitStoreU32(asm, PE, s1 + 16, L.codeRaw);
  emitStoreU32(asm, PE, s1 + 20, L.headersRaw);
  emitStoreU32(asm, PE, s1 + 36, 0x60000020);

  const s2 = s1 + 40;
  emitStoreU8(asm, PE, s2 + 0, 0x2e);
  emitStoreU8(asm, PE, s2 + 1, 0x64);
  emitStoreU8(asm, PE, s2 + 2, 0x61);
  emitStoreU8(asm, PE, s2 + 3, 0x74);
  emitStoreU8(asm, PE, s2 + 4, 0x61);
  emitStoreU32(asm, PE, s2 + 8, L.dataVs);
  emitStoreU32(asm, PE, s2 + 12, L.dataRva);
  emitStoreU32(asm, PE, s2 + 16, L.dataRaw);
  emitStoreU32(asm, PE, s2 + 20, L.headersRaw + L.codeRaw);
  emitStoreU32(asm, PE, s2 + 36, 0xc0000040);

  // startup at headersRaw
  const textOff = L.headersRaw;
  emitStoreU8(asm, PE, textOff + 0, 0x4c);
  emitStoreU8(asm, PE, textOff + 1, 0x8d);
  emitStoreU8(asm, PE, textOff + 2, 0x3d);
  emitStoreU32(asm, PE, textOff + 3, L.leaDisp >>> 0);
  emitStoreU8(asm, PE, textOff + 7, 0xe9);
  emitStoreU32(asm, PE, textOff + 8, L.jmpRel >>> 0);
  emitStoreU8(asm, PE, textOff + 12, 0x90);

  // IAT blob at dataFileOff
  emitCopyIatBlob(asm, PE, L.dataFileOff, iatBlob);

  // memcpy code.bin → pe+codeOff
  asm.raw(0x48, 0x8b, 0x7c, 0x24, PE);
  asm.raw(0x48, 0x81, 0xc7, ...u32le(L.codeOff));
  asm.raw(0x48, 0x8b, 0x74, 0x24, CODE_BUF);
  asm.raw(0xb9, ...u32le(codeLen));
  asm.raw(0xf3, 0xa4);

  // memcpy data.bin → pe+userDataOff
  asm.raw(0x48, 0x8b, 0x7c, 0x24, PE);
  asm.raw(0x48, 0x81, 0xc7, ...u32le(L.userDataOff));
  asm.raw(0x48, 0x8b, 0x74, 0x24, DATA_BUF);
  asm.raw(0xb9, ...u32le(dataLen));
  asm.raw(0xf3, 0xa4);

  asm.raw(0x48, 0x8b, 0x54, 0x24, PE);
  asm.raw(0x41, 0xb8, ...u32le(L.peSize));
  asm.jmpSite('jmp_place_to_write');
  return { layout: L, iatLen: iatBlob.length };
}

/**
 * YB: if emit.bins sentinel opens → read self PE → write code.bin + data.bin → exit 0.
 * Else fall through to try_place_bins.
 */
function emitNativeEmitBinsFromSelf(asm, meta) {
  const p = meta.place || {};
  const PE_SIZE = p.pe_size || 438784; // TRUER7 trimmed default
  const CODE_OFF = p.code_off || 0x40d;
  const CODE_LEN = p.code_len || 15099;
  const DATA_OFF = p.data_off || 0x4120;
  const DATA_LEN = p.data_len || 414016; // TRUER7 trimmed default
  const PE = 0x58;

  function writeSlice(pathOff, fileOff, size, failSite) {
    // CreateFileA(path, GENERIC_WRITE, 0,0, CREATE_ALWAYS, NORMAL, 0)
    asm.raw(0x49, 0x8d, 0x8f, ...u32le(pathOff));
    asm.raw(0xba, 0x00, 0x00, 0x00, 0x40); // GENERIC_WRITE
    asm.raw(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
    asm.raw(0xc7, 0x44, 0x24, 0x20, 0x02, 0x00, 0x00, 0x00); // CREATE_ALWAYS
    asm.raw(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00);
    asm.raw(0x48, 0xc7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00);
    asm.raw(0x41, 0xff, 0x97, 0x08, 0x00, 0x00, 0x00);
    asm.raw(0x48, 0x83, 0xf8, 0xff);
    asm.jccSite(failSite, 0x84);
    asm.raw(0x48, 0x89, 0xc3); // rbx = handle
    asm.raw(0x48, 0x8b, 0x54, 0x24, PE);
    asm.raw(0x48, 0x81, 0xc2, ...u32le(fileOff)); // rdx = pe+off
    asm.raw(0x41, 0xb8, ...u32le(size)); // r8d
    asm.raw(0x4c, 0x8d, 0x4c, 0x24, 0x40); // r9 = &written
    asm.raw(0x48, 0xc7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00);
    asm.raw(0x48, 0x89, 0xd9); // rcx = handle
    asm.raw(0x41, 0xff, 0x97, 0x18, 0x00, 0x00, 0x00); // WriteFile
    asm.raw(0x85, 0xc0);
    asm.jccSite(failSite, 0x84);
    asm.raw(0x48, 0x89, 0xd9);
    asm.raw(0x41, 0xff, 0x97, 0x20, 0x00, 0x00, 0x00); // CloseHandle
  }

  asm.lab('try_emit_bins');
  // Open sentinel emit.bins (OPEN_EXISTING)
  asm.raw(0x49, 0x8d, 0x8f, ...u32le(meta.emit_bins_off));
  asm.raw(0xba, 0x00, 0x00, 0x00, 0x80);
  asm.raw(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
  asm.raw(0xc7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00);
  asm.raw(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00);
  asm.raw(0x48, 0xc7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00);
  asm.raw(0x41, 0xff, 0x97, 0x08, 0x00, 0x00, 0x00);
  asm.raw(0x48, 0x83, 0xf8, 0xff);
  asm.jccSite('je_emit_no_sentinel', 0x84);
  asm.raw(0x48, 0x89, 0xc1);
  asm.raw(0x41, 0xff, 0x97, 0x20, 0x00, 0x00, 0x00); // CloseHandle sentinel

  // VirtualAlloc PE_SIZE
  asm.raw(0x31, 0xc9);
  asm.raw(0xba, ...u32le(PE_SIZE));
  asm.raw(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00);
  asm.raw(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00);
  asm.raw(0x41, 0xff, 0x97, 0x00, 0x00, 0x00, 0x00);
  asm.raw(0x48, 0x85, 0xc0);
  asm.jccSite('je_emit_fail', 0x84);
  asm.raw(0x48, 0x89, 0x44, 0x24, PE);

  // Read self PE
  asm.raw(0x49, 0x8d, 0x8f, ...u32le(meta.self_pe_off));
  asm.raw(0xba, 0x00, 0x00, 0x00, 0x80);
  asm.raw(0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9);
  asm.raw(0xc7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00);
  asm.raw(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00);
  asm.raw(0x48, 0xc7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00);
  asm.raw(0x41, 0xff, 0x97, 0x08, 0x00, 0x00, 0x00);
  asm.raw(0x48, 0x83, 0xf8, 0xff);
  asm.jccSite('je_emit_fail', 0x84);
  asm.raw(0x48, 0x89, 0xc3);
  asm.raw(0x48, 0x8b, 0x54, 0x24, PE);
  asm.raw(0x41, 0xb8, ...u32le(PE_SIZE));
  asm.raw(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  asm.raw(0x48, 0xc7, 0x44, 0x24, 0x20, 0x00, 0x00, 0x00, 0x00);
  asm.raw(0x48, 0x89, 0xd9);
  asm.raw(0x41, 0xff, 0x97, 0x10, 0x00, 0x00, 0x00);
  asm.raw(0x85, 0xc0);
  asm.jccSite('je_emit_fail_rd', 0x84);
  asm.raw(0x81, 0x7c, 0x24, 0x40, ...u32le(PE_SIZE));
  asm.jccSite('je_emit_fail_sz', 0x85);
  asm.raw(0x48, 0x89, 0xd9);
  asm.raw(0x41, 0xff, 0x97, 0x20, 0x00, 0x00, 0x00);

  writeSlice(meta.code_bin_off, CODE_OFF, CODE_LEN, 'je_emit_fail_wcode');
  writeSlice(meta.data_bin_off, DATA_OFF, DATA_LEN, 'je_emit_fail_wdata');

  // success exit 0
  asm.raw(0x31, 0xc0);
  asm.raw(0x48, 0x83, 0xc4, 0x78);
  asm.raw(0xc3);
  return asm;
}

module.exports = { emitNativeLinkFromBins, emitNativeEmitBinsFromSelf, layoutFor };
