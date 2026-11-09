'use strict';
/**
 * HB-FX FX-2d — JS rebuild of parse1 compiler code.bin.
 *
 * Hot path: native CodeAsm builders (parse1-native-asm + emit-stubs + patches).
 * Never opens pe_dll_link.rs / code-recipe.json / STEPS table at runtime.
 */

const { buildCompilerDataPrelude, S3_PARSE1_RET_PE_LEN, S3_H00_DATA_FLOOR, S3_H00_HDR_STARTUP_LEN } =
  require('./yoyo-compiler-emit-prelude');
const { STR_TABLE_OFF } = require('./platform-io');
const { emitNativeFront } = require('./yoyo-compiler-parse1-native-asm');
const { registerParse1Patches, lineage: NATIVE_PATCH_LINEAGE } = require('./yoyo-compiler-parse1-patches');
const { linkPeWin32 } = require('./pe-builder');

const S3_WIN32_LOAD_FILE_BODY_SEED = Buffer.from([
  0x48, 0x83, 0xec, 0x28, 0x49, 0x8d, 0x8f, 0x00, 0x00, 0x01, 0x00, 0xba, 0x00, 0x00, 0x00, 0x80,
  0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9, 0xc7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00, 0x48, 0xc7,
  0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00, 0x48, 0xc7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00,
  0x41, 0xff, 0x97, 0x08, 0x00, 0x00, 0x00, 0x48, 0x89, 0xc3, 0x31, 0xc9, 0xba, 0x00, 0x00, 0x01,
  0x00, 0x41, 0xb8, 0x00, 0x30, 0x00, 0x00, 0x41, 0xb9, 0x04, 0x00, 0x00, 0x00, 0x41, 0xff, 0x97,
  0x00, 0x00, 0x00, 0x00, 0x48, 0x89, 0xc6, 0x48, 0x89, 0xd9, 0x48, 0x89, 0xf2, 0x41, 0xb8, 0x00,
  0x00, 0x01, 0x00, 0x4c, 0x8d, 0x4c, 0x24, 0x20, 0x48, 0xc7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00,
  0x00, 0x41, 0xff, 0x97, 0x10, 0x00, 0x00, 0x00, 0x48, 0x89, 0xd9, 0x41, 0xff, 0x97, 0x20, 0x00,
  0x00, 0x00, 0x48, 0x83, 0xc4, 0x28, 0x48, 0x89, 0xf0, 0x49, 0x89, 0x87, 0x00, 0x00, 0x00, 0x00,
]);
const S3_WIN32_WRITE_FILE_BODY_SEED = Buffer.from([
  0x48, 0x83, 0xec, 0x28, 0x49, 0x8d, 0x8f, 0x00, 0x00, 0x01, 0x00, 0xba, 0x00, 0x00, 0x00, 0x40,
  0x45, 0x31, 0xc0, 0x45, 0x31, 0xc9, 0xc7, 0x44, 0x24, 0x20, 0x02, 0x00, 0x00, 0x00, 0x48, 0xc7,
  0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00, 0x48, 0xc7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00,
  0x41, 0xff, 0x97, 0x08, 0x00, 0x00, 0x00, 0x48, 0x89, 0xc3, 0x49, 0x8b, 0x97, 0x00, 0x00, 0x00,
  0x00, 0x4d, 0x8b, 0x87, 0x00, 0x00, 0x00, 0x00, 0x48, 0x89, 0xd9, 0x4c, 0x8d, 0x4c, 0x24, 0x20,
  0x48, 0xc7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00, 0x41, 0xff, 0x97, 0x18, 0x00, 0x00, 0x00,
  0x48, 0x89, 0xd9, 0x41, 0xff, 0x97, 0x20, 0x00, 0x00, 0x00, 0x48, 0x83, 0xc4, 0x28, 0x48, 0xb8,
  0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x87, 0x00, 0x00, 0x00, 0x00,
]);

const CONSTS = {
  S3_PARSE1_RET_PE_LEN,
  S3_H00_DATA_FLOOR,
  S3_H00_HDR_STARTUP_LEN,
  S3_H00_ALLOC_PAD: 0x10000,
  S3_PARSE1_H00_PE_PTR_OFF: 0x4010,
  S3_PARSE1_TEXT_ZERO: 0x4ff3,
  S3_PARSE1_DATA_FILE_OFF: 0x5400,
  S3_COMPACT_TEXT_PAYLOAD_OFF: 0x40d,
  S3_COMPACT_TEXT_CAP: 0x1f3,
  S3_COMPACT_GATE_TEXT_LEN: 0x1f3,
  S3_COMPACT_DATA_USER_FILE_OFF: 0x720,
  S3_COMPACT_DATA_USER_CAP: 0xee0,
  S3_COMPACT_MID_GROW_CAP: 0xfb3,
  S3_COMPACT_FULL_GROW_CAP: 0x4ff3,
  S3_COMPACT_TMPL_DATA_RP: 0x600,
  S3_COMPACT_TMPL_DATA_RAW: 0x2000,
  S3_COMPACT_TMPL_DATA_VA: 0x2000,
  S3_COMPACT_IAT_PREFIX: 0x120,
  S3_COMPACT_HEADERS: 0x400,
  S3_COMPACT_PE_OPT: 0x98,
  S3_COMPACT_PE_SECT_TEXT: 0x188,
  S3_PARSE1_SCRATCH_LEN: 0x6100,
  S3_PARSE1_HOT_SCRATCH_OFF: 0x4040,
  S3_PARSE1_HOT_CUR_HH_OFF: 0x4044,
  S3_PARSE1_HOT_START_OFF: 0x403c,
  S3_PARSE1_HOT_ENTRIES_OFF: 0x4048,
  S3_PARSE1_HOT_CAP: 1024,
  S1_MINI_DATA_MAX: 384,
  STR_TABLE_OFF,
};

function patchJccBuf(buf, at, target) {
  const rel = (target - (at + 6)) | 0;
  buf[at + 2] = rel & 0xff;
  buf[at + 3] = (rel >>> 8) & 0xff;
  buf[at + 4] = (rel >>> 16) & 0xff;
  buf[at + 5] = (rel >>> 24) & 0xff;
}
function patchJmpBuf(buf, at, target) {
  const rel = (target - (at + 5)) | 0;
  buf[at + 1] = rel & 0xff;
  buf[at + 2] = (rel >>> 8) & 0xff;
  buf[at + 3] = (rel >>> 16) & 0xff;
  buf[at + 4] = (rel >>> 24) & 0xff;
}

/**
 * Full native CodeAsm emit (parse1 body + emit stubs) + fixup patches.
 */
function emitNativeGrowing(meta, seeds) {
  const asm = emitNativeFront(meta, seeds);
  registerParse1Patches(asm);
  asm.applyPatches();
  const buf = asm.buf;
  const labels = asm.labels;
  const extraPatches = asm.extraPatches || [];
  const extraStarts = extraPatches.map(([jne1]) => jne1 - 6);
  for (let i = 0; i < extraPatches.length; i++) {
    const [jne1, jne2, jmp, kind] = extraPatches[i];
    const miss = i + 1 < extraStarts.length ? extraStarts[i + 1] : labels.parse1_any;
    patchJccBuf(buf, jne1, miss);
    patchJccBuf(buf, jne2, miss);
    if (kind >= 2 && kind <= 8) patchJmpBuf(buf, jmp, labels.parse1);
    else if (kind === 1) patchJmpBuf(buf, jmp, labels.live2_nop);
    else patchJmpBuf(buf, jmp, labels.do_write);
  }
  return { code: Buffer.from(buf), labels, asm };
}

function measurePlaceLayout(code, data) {
  const pe = linkPeWin32(code, data, [], 0x38000);
  const find = (hay, needle) => {
    outer: for (let i = 0; i <= hay.length - needle.length; i++) {
      for (let j = 0; j < Math.min(64, needle.length); j++) {
        if (hay[i + j] !== needle[j]) continue outer;
      }
      for (let j = 0; j < needle.length; j++) {
        if (hay[i + j] !== needle[j]) continue outer;
      }
      return i;
    }
    return -1;
  };
  const code_off = find(pe, code);
  const data_off = find(pe, data);
  if (code_off < 0 || data_off < 0) {
    throw new Error(`place layout locate fail code_off=${code_off} data_off=${data_off}`);
  }
  return {
    pe_size: pe.length,
    code_off,
    code_len: code.length,
    data_off,
    data_len: data.length,
  };
}

/**
 * Build parse1 code.bin — native hot path only (no STEPS / recipe / emit-all).
 * Two-pass: nail meta.place sizes for BD-4 native linkPeWin32 geometry.
 */
function buildCompilerCode(opts = {}) {
  if (!opts.seedPe) throw new Error('buildCompilerCode requires seedPe');
  const built = buildCompilerDataPrelude({
    seedPe: opts.seedPe,
    userCodeLen: opts.userCodeLen != null ? opts.userCodeLen : 17810,
    fieldMatch: true,
  });
  const seeds = {
    load: S3_WIN32_LOAD_FILE_BODY_SEED,
    write: S3_WIN32_WRITE_FILE_BODY_SEED,
  };
  let { code, labels } = emitNativeGrowing(built.meta, seeds);
  built.meta.place = measurePlaceLayout(code, built.data);
  ({ code, labels } = emitNativeGrowing(built.meta, seeds));
  // Confirm stable after second pass.
  const place2 = measurePlaceLayout(code, built.data);
  if (
    place2.pe_size !== built.meta.place.pe_size ||
    place2.code_off !== built.meta.place.code_off ||
    place2.data_off !== built.meta.place.data_off ||
    place2.code_len !== code.length
  ) {
    built.meta.place = place2;
    ({ code, labels } = emitNativeGrowing(built.meta, seeds));
  }
  return {
    code,
    data: built.data,
    meta: built.meta,
    labels,
    origin: 'js_parse1_native',
    lineage: NATIVE_PATCH_LINEAGE || 'js_parse1_native',
    place: built.meta.place,
  };
}

module.exports = {
  buildCompilerCode,
  emitNativeGrowing,
  CONSTS,
  S3_WIN32_LOAD_FILE_BODY_SEED,
  S3_WIN32_WRITE_FILE_BODY_SEED,
};
