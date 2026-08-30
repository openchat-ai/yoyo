'use strict';
/**
 * JS peer of yoyo-rust/verifier/src/h00_manual_map_wireup.rs (manual-map H_00 stub).
 * Template synced from `cargo test -p verifier manual_map_main_pinned -- --nocapture`.
 */

const fs = require('fs');
const path = require('path');

const CANONICAL_TEXT_RVA = 0x1000;
const CANONICAL_CODE_BASE_OFF = 17_823;
const CANONICAL_IAT_RVA = 0x20_000;

const H00_MANUAL_MAP_STUB_TEMPLATE_HEX = fs
  .readFileSync(path.join(__dirname, 'h00-manual-map-stub.hex'), 'utf8')
  .trim();

const H00_MANUAL_MAP_STUB_LEN = H00_MANUAL_MAP_STUB_TEMPLATE_HEX.length / 2;

function patchRel32(buf, dispOff, from, to) {
  buf.writeInt32LE(to - from, dispOff);
}

const H00_LEA_SITE = 39;
const H00_RELOAD_R15_SITES = [
  14, 28, 132, 192, 255, 296, 310, 714, 1011, 2106, 2129, 2152, 2175, 2198, 2221, 2244, 2267,
];
const H00_IAT_SITES = [
  [86, 1], // CreateFileA
  [165, 0], // VirtualAlloc (file buffer)
  [232, 2], // ReadFile
  [277, 4], // CloseHandle
  [352, 0], // VirtualAlloc (image)
  [2100, 5], // ExitProcess (export success)
];

function rebaseManualMapStub(buf, textRva, codeBaseOff, meta) {
  patchRel32(
    buf,
    H00_LEA_SITE + 3,
    textRva + codeBaseOff + H00_LEA_SITE + 7,
    meta.tempNameRva,
  );
  for (const at of H00_RELOAD_R15_SITES) {
    patchRel32(buf, at + 3, textRva + codeBaseOff + at + 7, meta.iatRva);
  }
  for (const [at, slot] of H00_IAT_SITES) {
    patchRel32(
      buf,
      at + 2,
      textRva + codeBaseOff + at + 6,
      meta.iatRva + slot * 8,
    );
  }
}

function genH00ManualMapMain(meta, textRva, codeBaseOff) {
  const buf = Buffer.from(H00_MANUAL_MAP_STUB_TEMPLATE_HEX, 'hex');
  if (buf.length !== H00_MANUAL_MAP_STUB_LEN) {
    throw new Error(`H_00 manual-map template len ${buf.length} != ${H00_MANUAL_MAP_STUB_LEN}`);
  }
  rebaseManualMapStub(buf, textRva, codeBaseOff, meta);
  return buf;
}

module.exports = {
  genH00ManualMapMain,
  H00_MANUAL_MAP_STUB_LEN,
  CANONICAL_CODE_BASE_OFF,
};
