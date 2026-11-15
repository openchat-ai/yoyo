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

const H00_LEA_SITE = 32;
const H00_RELOAD_R15_SITES = [
  7, 21, 125, 185, 257, 298, 312, 695, 738, 1205, 2492, 2515, 2538, 2561, 2584, 2607, 2630, 2653,
];
const H00_IAT_SITES = [
  [79, 1], // CreateFileA
  [158, 0], // VirtualAlloc (file buffer)
  [234, 2], // ReadFile
  [279, 4], // CloseHandle
  [354, 0], // VirtualAlloc (image)
  [2486, 5], // ExitProcess (export success)
];
/** `c6 05 rel32 imm8` phase probes — must track pe_link::WIN32_IO_H00_SCRATCH_OFF+24. */
const H00_PHASE_SCRATCH_OFF = 0xf8 + 24;
const H00_PHASE_SITES = [
  14, 132, 192, 264, 305, 319, 376, 383, 544, 688, 953, 1058, 1173, 1198, 1521, 1538, 2463,
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
  const phaseTgt = meta.iatRva + H00_PHASE_SCRATCH_OFF;
  for (const at of H00_PHASE_SITES) {
    // c6 05 rel32 imm8 — disp at +2, insn end at +7
    patchRel32(buf, at + 2, textRva + codeBaseOff + at + 7, phaseTgt);
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
