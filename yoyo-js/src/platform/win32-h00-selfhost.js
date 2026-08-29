'use strict';
/**
 * JS peer of yoyo-rust/verifier/src/win32_selfhost.rs (H_00 seed/link path).
 */

const { TEXT_RVA } = require('./platform-config');
const { STR_TABLE_OFF, STR_ENTRY_SIZE } = require('./platform-io');
const {
  genH00ManualMapMain,
  H00_MANUAL_MAP_STUB_LEN,
} = require('./h00-manual-map-peer');

const TEMP_DLL_NAME = Buffer.from('yoyo_rt.dll\0');
const PE_STARTUP_LEN = 13;
const H00_SLOT_LEN = 18;
const SECTION_ALIGN = 0x1000;

function alignUp(v, a) {
  return (v + a - 1) & ~(a - 1);
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
  return genH00ManualMapMain(meta, textRva, codeBaseOff);
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
  if (h00Main.length !== H00_MANUAL_MAP_STUB_LEN) {
    throw new Error(`H_00 stub len ${h00Main.length} != pinned ${H00_MANUAL_MAP_STUB_LEN}`);
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
  H00_MANUAL_MAP_STUB_LEN,
};
