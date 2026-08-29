'use strict';
/**
 * JS peer of yoyo-rust/verifier/src/win32_selfhost.rs (H_00 seed/link path).
 */

const { TEXT_RVA } = require('./platform-config');
const { STR_TABLE_OFF, STR_ENTRY_SIZE } = require('./platform-io');

const TEMP_DLL_NAME = Buffer.from('yoyo_rt.dll\0');
const IAT_LOADLIBRARY = 5;
const IAT_EXIT_PROCESS = 6;
const PE_STARTUP_LEN = 13;
const H00_SLOT_LEN = 18;
const H00_MAIN_STUB_LEN = 71; // pinned to Rust PE AddressOfFunctions[0] export resolve stub
const SECTION_ALIGN = 0x1000;

function alignUp(v, a) {
  return (v + a - 1) & ~(a - 1);
}

function patchRel32(buf, dispOff, from, to) {
  buf.writeInt32LE(to - from, dispOff);
}

function fixRipDisp(buf, dispOff, textRva, codeBaseOff, insnEnd, targetRva) {
  const next = textRva + codeBaseOff + insnEnd;
  patchRel32(buf, dispOff, next, targetRva);
}

function emitCallIatMerged(buf, textRva, codeBaseOff, iatRva, slot) {
  const at = buf.length;
  const chunk = Buffer.alloc(6);
  chunk[0] = 0xff;
  chunk[1] = 0x15;
  const nextRva = textRva + codeBaseOff + at + 6;
  patchRel32(chunk, 2, nextRva, iatRva + slot * 8);
  return Buffer.concat([buf, chunk]);
}

function writeCstrEntry(blob, base, s) {
  const n = Math.min(s.length, STR_ENTRY_SIZE - 1);
  s.copy(blob, base, 0, n);
  blob[base + n] = 0;
}

function embedStringTable(userData) {
  const tableOff = STR_TABLE_OFF;
  const need = tableOff + STR_ENTRY_SIZE * 3;
  const blob = Buffer.alloc(Math.max(userData.length, need), 0);
  userData.copy(blob);
  writeCstrEntry(blob, tableOff, Buffer.from('input.tyb'));
  writeCstrEntry(blob, tableOff + STR_ENTRY_SIZE, Buffer.from('input.ky'));
  writeCstrEntry(blob, tableOff + STR_ENTRY_SIZE * 2, Buffer.from('output.exe'));
  return blob;
}

function appendH00RuntimeData(userData, dataRva) {
  let blob = Buffer.from(userData);
  while (blob.length % 16 !== 0) blob = Buffer.concat([blob, Buffer.from([0])]);
  const base = blob.length;
  blob = Buffer.concat([blob, TEMP_DLL_NAME]);
  while (blob.length % 16 !== 0) blob = Buffer.concat([blob, Buffer.from([0])]);
  return {
    data: blob,
    meta: {
      tempNameRva: dataRva + base,
      iatRva: dataRva,
    },
  };
}

function genH00SelfhostMain(meta, textRva, mainUserOff) {
  const codeBaseOff = PE_STARTUP_LEN + mainUserOff;
  let c = Buffer.alloc(0);

  c = Buffer.concat([c, Buffer.from([0x53])]);
  c = Buffer.concat([c, Buffer.from([0x48, 0x83, 0xec, 0x28])]);

  const leaTemp = c.length;
  c = Buffer.concat([c, Buffer.from([0x48, 0x8d, 0x0d, 0, 0, 0, 0])]);
  c = emitCallIatMerged(c, textRva, codeBaseOff, meta.iatRva, IAT_LOADLIBRARY);
  c = Buffer.concat([c, Buffer.from([0x48, 0x85, 0xc0])]);
  const jzFail = c.length;
  c = Buffer.concat([c, Buffer.from([0x74, 0x00])]);
  c = Buffer.concat([c, Buffer.from([0x48, 0x89, 0xc3])]);

  c = Buffer.concat([c, Buffer.from([0x8b, 0x43, 0x3c])]);
  c = Buffer.concat([c, Buffer.from([0x8b, 0x84, 0x03, 0x88, 0x00, 0x00, 0x00])]);
  c = Buffer.concat([c, Buffer.from([0x48, 0x01, 0xd8])]);
  c = Buffer.concat([c, Buffer.from([0x8b, 0x40, 0x1c])]);
  c = Buffer.concat([c, Buffer.from([0x48, 0x01, 0xd8])]);
  c = Buffer.concat([c, Buffer.from([0x8b, 0x00])]);
  c = Buffer.concat([c, Buffer.from([0x48, 0x01, 0xd8])]);
  c = Buffer.concat([c, Buffer.from([0xff, 0xd0])]);
  c = Buffer.concat([c, Buffer.from([0x89, 0xc1])]);
  c = emitCallIatMerged(c, textRva, codeBaseOff, meta.iatRva, IAT_EXIT_PROCESS);

  const fail = c.length;
  c[jzFail + 1] = fail - (jzFail + 2);
  c = Buffer.concat([c, Buffer.from([0xb9, 0x01, 0x00, 0x00, 0x00])]);
  c = emitCallIatMerged(c, textRva, codeBaseOff, meta.iatRva, IAT_EXIT_PROCESS);

  fixRipDisp(c, leaTemp + 3, textRva, codeBaseOff, leaTemp + 7, meta.tempNameRva);
  return c;
}

function shouldH00Selfhost(handlerOffsets) {
  return handlerOffsets.some(([h]) => h === 0x20) && handlerOffsets.some(([h]) => h === 0x21);
}

function handlerOff(handlerOffsets, hh) {
  const row = handlerOffsets.find(([h]) => h === hh);
  return row ? row[1] : null;
}

function linkPeH00Runtime(code, data, handlerOffsets) {
  if (code.length < H00_SLOT_LEN) throw new Error('H_00 selfhost: code too short');
  if (!handlerOff(handlerOffsets, 0x20)) throw new Error('H_00 selfhost: missing H_20');

  const outCode = Buffer.from(code);
  const mainUserOff = outCode.length;
  const textRva = TEXT_RVA;

  const probe = genH00SelfhostMain({ tempNameRva: 0, iatRva: 0 }, textRva, mainUserOff);
  const textVs = alignUp(PE_STARTUP_LEN + outCode.length + probe.length + 0x40, SECTION_ALIGN);
  const dataRva = SECTION_ALIGN + textVs;

  const withStrings = embedStringTable(data);
  const { prependWin32IoIat } = require('./pe-builder');
  const prep = prependWin32IoIat(withStrings, dataRva);
  const { data: extended, meta } = appendH00RuntimeData(prep.data, dataRva);

  const h00Main = genH00SelfhostMain(meta, textRva, mainUserOff);
  if (h00Main.length !== H00_MAIN_STUB_LEN) {
    throw new Error(`H_00 stub len ${h00Main.length} != pinned ${H00_MAIN_STUB_LEN}`);
  }
  const linked = Buffer.concat([outCode, h00Main]);
  linked[0] = 0xe9;
  linked.writeInt32LE(mainUserOff - 5, 1);
  for (let i = 5; i < H00_SLOT_LEN; i++) linked[i] = 0x90;

  return {
    code: linked,
    data: extended,
    importDirRva: prep.importDirRva,
    importDirSize: prep.importDirSize,
  };
}

module.exports = {
  shouldH00Selfhost,
  linkPeH00Runtime,
  PE_STARTUP_LEN,
  H00_SLOT_LEN,
};
