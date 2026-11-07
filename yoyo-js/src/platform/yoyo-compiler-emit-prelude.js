'use strict';
/**
 * HB-FX FX-2b — JS algorithmic rebuild of parse1 compiler *data*
 * (strings + live2 + RET/setret/compact + H00 bake + field-match PEs).
 *
 * Peer of pe_dll_link::yoyo_compiler_ret_compile_code_data **data** half.
 * With `{ seedPe, fieldMatch:true }` → full data.bin EQUAL dump.
 * code.bin emit still ABSENT (FX-2b-4).
 */

const fs = require('fs');
const path = require('path');
const { linkPeWin32Compact } = require('./pe-builder');
const { encodeOp } = require('./encode-x64');

const SEEDS = path.join(__dirname, 'yoyo-compiler-payload', 'seeds');
const S3_PARSE1_RET_PE_LEN = 0x6400;
const S3_HOST_SET_DEADBEEF = 0xdeadbeef;
const S3_HOST_ADD_IMM = 5;
const S3_HOST_SUB_IMM = 5;

/** Peer of s1_tyb_parser::s3_fixed_rec + s3_fixed10_*_tyb */
const FM_SINGLES = [0x00, 0x60, 0x66, 0x62, 0x67, 0x61, 0x64, 0x65];
const FM_CHAINS = [
  [0x30, 0x62],
  [0x30, 0x60],
  [0x30, 0x62, 0x61],
  [0x00, 0x66, 0x67],
  [0x30, 0x62, 0x61, 0x00],
  [0x30, 0x64],
  [0x30, 0x65, 0x64],
];

function fixedRec(op) {
  const pack = (slot, imm) => {
    const a = Buffer.alloc(6);
    a.writeUInt16LE(slot & 0xffff, 0);
    a.writeUInt32LE(imm >>> 0, 2);
    return a;
  };
  switch (op) {
    case 0xff:
    case 0x00:
      return { op, argc: 0, args: Buffer.alloc(6) };
    case 0x30:
      return { op, argc: 2, args: pack(0, S3_HOST_SET_DEADBEEF) };
    case 0x60:
      return { op, argc: 2, args: pack(0, 1) };
    case 0x66:
    case 0x67:
      return { op, argc: 1, args: Buffer.alloc(6) };
    case 0x62:
      return { op, argc: 2, args: pack(0, S3_HOST_ADD_IMM) };
    case 0x61:
      return { op, argc: 2, args: pack(0, S3_HOST_SUB_IMM) };
    case 0x64:
      return { op, argc: 2, args: pack(1, 0) };
    case 0x65:
      return { op, argc: 2, args: pack(0, 1) };
    default:
      throw new Error(`fixedRec: op 0x${op.toString(16)} outside fixed10`);
  }
}

function encodeArgs(rec) {
  const slot = rec.args.readUInt16LE(0);
  const imm32 = rec.args.readUInt32LE(2);
  const src16 = rec.args.readUInt16LE(2);
  if (rec.op === 0x30 || rec.op === 0x62 || rec.op === 0x61) return [slot, imm32];
  if (rec.op === 0x60 || rec.op === 0x64 || rec.op === 0x65) return [slot, src16];
  if (rec.op === 0x66 || rec.op === 0x67) return [slot];
  return [];
}

function emitFixedOps(ops) {
  const bytes = [];
  for (const op of ops) {
    const rec = fixedRec(op);
    bytes.push(...encodeOp(rec.op, encodeArgs(rec)));
  }
  return Buffer.from(bytes);
}

/** Peer of s3_host_tyb_from_records / s3_fixed10_chain_ret_tyb */
function fixed10RetTyb(ops) {
  const recs = ops.map(fixedRec);
  if (ops[ops.length - 1] !== 0xff) recs.push(fixedRec(0xff));
  const v = Buffer.alloc(8 + recs.length * 8);
  v.write('TYB\0', 0, 4, 'ascii');
  v.writeUInt16LE(0, 4);
  v.writeUInt16LE(recs.length, 6);
  let o = 8;
  for (const r of recs) {
    v[o++] = r.op;
    v[o++] = r.argc;
    r.args.copy(v, o);
    o += 6;
  }
  return v.subarray(0, o);
}

function compileTybToPeChild(opsWithRet) {
  const code = emitFixedOps(opsWithRet);
  return linkPeWin32Compact(code, Buffer.alloc(0), []);
}

function pad16(buf) {
  const n = (16 - (buf.length % 16)) % 16;
  if (!n) return buf;
  return Buffer.concat([buf, Buffer.alloc(n, 0)]);
}

/** Peer of s1_tyb_parser::s3_expand_ret_pe_text_raw20k */
function expandRetPeTextRaw20k(seed) {
  if (seed.length < 0x1a00) throw new Error('ret seed short');
  const OLD_TEXT_RS = 0x600;
  const NEW_TEXT_RS = 0x5000;
  const OLD_DATA_RP = 0xa00;
  const NEW_DATA_RP = 0x5400;
  const OLD_DATA_VA = 0x2000;
  const NEW_DATA_VA = 0x6000;
  const DATA_RS = 0x1000;
  const DATA_VS = 0x1000;
  const RVA_DELTA = NEW_DATA_VA - OLD_DATA_VA;
  const pe = Buffer.alloc(NEW_DATA_RP + DATA_RS, 0);
  seed.copy(pe, 0, 0, 0x400);
  seed.copy(pe, 0x400, 0x400, 0x400 + OLD_TEXT_RS);
  seed.copy(pe, NEW_DATA_RP, OLD_DATA_RP, OLD_DATA_RP + DATA_RS);
  const eLfanew = pe.readUInt32LE(0x3c);
  const opt = eLfanew + 24;
  pe.writeUInt32LE(NEW_TEXT_RS, opt + 4);
  pe.writeUInt32LE(NEW_DATA_VA + DATA_VS, opt + 56);
  const impRva = pe.readUInt32LE(opt + 120);
  if (impRva !== 0) pe.writeUInt32LE(impRva + RVA_DELTA, opt + 120);
  const sizeOpt = pe.readUInt16LE(eLfanew + 20);
  const sect = eLfanew + 24 + sizeOpt;
  pe.writeUInt32LE(NEW_TEXT_RS, sect + 8);
  pe.writeUInt32LE(NEW_TEXT_RS, sect + 16);
  pe.writeUInt32LE(NEW_DATA_VA, sect + 40 + 12);
  pe.writeUInt32LE(NEW_DATA_RP, sect + 40 + 20);
  for (let i = 0; i + 3 < DATA_RS; i += 4) {
    const off = NEW_DATA_RP + i;
    const v = pe.readUInt32LE(off);
    if (v >= OLD_DATA_VA && v < OLD_DATA_VA + DATA_VS) {
      pe.writeUInt32LE(v + RVA_DELTA, off);
    }
  }
  if (pe.length !== S3_PARSE1_RET_PE_LEN) {
    throw new Error(`expanded RET PE len ${pe.length} != ${S3_PARSE1_RET_PE_LEN}`);
  }
  return pe;
}

function pushLive2(chunks, bytes) {
  let b = Buffer.from(bytes);
  chunks.push(b);
  const pad = (16 - (b.length % 16)) % 16;
  if (pad) chunks.push(Buffer.alloc(pad, 0));
}

const S3_H00_HDR_STARTUP_LEN = 0x40d;
const S3_H00_DATA_FLOOR = 0x38000;

/**
 * Append H00 hdr/main/data slices from a YOYO seed PE (JS or Rust linked).
 * Peer of S3.674–678 bake in yoyo_compiler_ret_compile_code_data.
 */
function appendH00BakeFromSeed(dataBuf, seedPe, userCodeLen) {
  const chunks = [dataBuf];
  const pe = Buffer.isBuffer(seedPe) ? seedPe : Buffer.from(seedPe);
  if (pe.length < S3_H00_HDR_STARTUP_LEN || pe[0] !== 0x4d || pe[1] !== 0x5a) {
    throw new Error('H00 seed PE too short or not MZ');
  }
  chunks.push(pe.subarray(0, S3_H00_HDR_STARTUP_LEN));
  chunks.push(Buffer.from('yoyo_h00_hdr_c\0', 'ascii'));

  let len = chunks.reduce((a, c) => a + c.length, 0);
  if (len % 16) chunks.push(Buffer.alloc(16 - (len % 16), 0));

  const eLfanew = pe.readUInt32LE(0x3c);
  const optSz = pe.readUInt16LE(eLfanew + 20);
  const sect = eLfanew + 24 + optSz;
  const textRp = pe.readUInt32LE(sect + 20);
  const dataRp = pe.readUInt32LE(sect + 40 + 20);
  const h00MainFile = textRp + 13 + userCodeLen;
  // Stub length pinned by manual-map peer (2676); derive from seed if longer pack.
  const H00_STUB_LEN = 2676;
  if (pe.length < h00MainFile + H00_STUB_LEN) {
    throw new Error(`seed missing h00_main slice need=${h00MainFile + H00_STUB_LEN} got=${pe.length}`);
  }
  chunks.push(pe.subarray(h00MainFile, h00MainFile + H00_STUB_LEN));
  chunks.push(Buffer.from('yoyo_h00_main_c\0', 'ascii'));

  len = chunks.reduce((a, c) => a + c.length, 0);
  if (len % 16) chunks.push(Buffer.alloc(16 - (len % 16), 0));

  if (pe.length < dataRp + S3_H00_DATA_FLOOR) {
    throw new Error('seed missing h00 .data floor');
  }
  chunks.push(pe.subarray(dataRp, dataRp + S3_H00_DATA_FLOOR));
  chunks.push(Buffer.from('yoyo_h00_data_c\0', 'ascii'));

  return Buffer.concat(chunks);
}

/**
 * Append field-match single-op PEs + CHAINS (tyb+pe).
 * Peer of S3.557–560 / S3.551–555 bake in yoyo_compiler_ret_compile_code_data.
 * Completes data.bin when appended after H00.
 * Returns { data, extras } where extras = CHAINS (tyb_rel, tyb_len, pe_rel, pe_len).
 */
function appendFieldMatchBake(dataBuf) {
  const chunks = [dataBuf];
  const extras = [];
  const curLen = () => chunks.reduce((a, c) => a + c.length, 0);
  const pushAlign16 = () => {
    const len = curLen();
    if (len % 16) chunks.push(Buffer.alloc(16 - (len % 16), 0));
  };

  for (const op of FM_SINGLES) {
    pushAlign16();
    chunks.push(compileTybToPeChild([op, 0xff]));
  }

  for (const ops of FM_CHAINS) {
    pushAlign16();
    const tyb = fixed10RetTyb(ops);
    const tybRel = curLen();
    chunks.push(tyb);
    pushAlign16();
    const pe = compileTybToPeChild([...ops, 0xff]);
    const peRel = curLen();
    chunks.push(pe);
    extras.push([tybRel, tyb.length, peRel, pe.length]);
  }

  return { data: Buffer.concat(chunks), extras };
}

/**
 * Build data.bin prefix through compact tmpl + yoyo_compact3_c marker.
 * Optional seedPe extends through H00 hdr/main/data bake.
 * Optional fieldMatch (requires seedPe) completes full data.bin.
 */
function buildCompilerDataPrelude(opts = {}) {
  const retSeed = fs.readFileSync(path.join(SEEDS, 's3_compile_pe_ret.pe'));
  const setretSeed = fs.readFileSync(path.join(SEEDS, 's3_compile_pe_setret.pe'));

  const chunks = [];
  const names = [
    'input.tyb\0',
    'output.exe\0',
    'yoyo_ret_c\0',
    'yoyo_sr_c\0',
    'yoyo_tbl_c\0',
    'yoyo_f10_c\0',
    'yoyo_multi_c\0',
    'yoyo_live1_c\0',
    'yoyo_live2_c\0',
    'yoyo_emit1_c\0',
    'yoyo_emit2_c\0',
    'yoyo_parse1_c\0',
  ];
  for (const s of names) chunks.push(Buffer.from(s, 'ascii'));

  // align 16 before live2
  let len = chunks.reduce((a, c) => a + c.length, 0);
  if (len % 16) chunks.push(Buffer.alloc(16 - (len % 16), 0));

  pushLive2(chunks, [0x90, 0xc3]);
  pushLive2(chunks, [0x49, 0x8b, 0x47, 0x00, 0x48, 0xff, 0xc0, 0x49, 0x89, 0x47, 0x00, 0xc3]);
  pushLive2(chunks, [0x49, 0x8b, 0x47, 0x00, 0x48, 0xff, 0xc8, 0x49, 0x89, 0x47, 0x00, 0xc3]);
  pushLive2(chunks, [0x49, 0x8b, 0x47, 0x08, 0x49, 0x89, 0x47, 0x00, 0xc3]);
  pushLive2(chunks, [
    0x49, 0x8b, 0x47, 0x00, 0x48, 0x81, 0xc0, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0xc3,
  ]);
  pushLive2(chunks, [
    0x49, 0x8b, 0x47, 0x00, 0x48, 0x81, 0xe8, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0xc3,
  ]);
  pushLive2(chunks, [
    0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0xc3,
  ]);
  pushLive2(chunks, [0x49, 0x8b, 0x47, 0x00, 0x49, 0x89, 0x47, 0x08, 0xc3]);
  pushLive2(chunks, [0x49, 0x8b, 0x47, 0x00, 0x49, 0x8b, 0x4f, 0x08, 0x48, 0x39, 0xc8, 0xc3]);
  pushLive2(chunks, [
    0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x49, 0x8b,
    0x47, 0x00, 0x48, 0x81, 0xc0, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0xc3,
  ]);
  pushLive2(chunks, [
    0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x49, 0x8b,
    0x47, 0x08, 0x49, 0x89, 0x47, 0x00, 0xc3,
  ]);
  pushLive2(chunks, [
    0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x49, 0x8b,
    0x47, 0x00, 0x48, 0x81, 0xc0, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x49, 0x8b, 0x47,
    0x00, 0x48, 0x81, 0xe8, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0xc3,
  ]);
  pushLive2(chunks, [
    0x90, 0x49, 0x8b, 0x47, 0x00, 0x48, 0xff, 0xc0, 0x49, 0x89, 0x47, 0x00, 0x49, 0x8b, 0x47, 0x00,
    0x48, 0xff, 0xc8, 0x49, 0x89, 0x47, 0x00, 0xc3,
  ]);
  pushLive2(chunks, [
    0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x49, 0x8b,
    0x47, 0x00, 0x49, 0x89, 0x47, 0x08, 0xc3,
  ]);
  pushLive2(chunks, [
    0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x49, 0x8b,
    0x47, 0x00, 0x48, 0x81, 0xc0, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x49, 0x8b, 0x47,
    0x00, 0x48, 0x81, 0xe8, 0x05, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x90, 0xc3,
  ]);
  pushLive2(chunks, [
    0x48, 0xb8, 0xef, 0xbe, 0xad, 0xde, 0x00, 0x00, 0x00, 0x00, 0x49, 0x89, 0x47, 0x00, 0x49, 0x8b,
    0x47, 0x00, 0x49, 0x8b, 0x4f, 0x08, 0x48, 0x39, 0xc8, 0x49, 0x8b, 0x47, 0x00, 0x49, 0x89, 0x47,
    0x08, 0xc3,
  ]);

  const retExpanded = expandRetPeTextRaw20k(retSeed);
  chunks.push(retExpanded);
  chunks.push(setretSeed);

  len = chunks.reduce((a, c) => a + c.length, 0);
  if (len % 16) chunks.push(Buffer.alloc(16 - (len % 16), 0));

  const compactTmpl = linkPeWin32Compact(Buffer.from([0xc3]), Buffer.alloc(0), []);
  chunks.push(compactTmpl);
  chunks.push(Buffer.from('yoyo_compact3_c\0', 'ascii'));

  const inputRel = 0;
  const outputRel = Buffer.from('input.tyb\0', 'ascii').length;
  let data = Buffer.concat(chunks);
  // First MZ = expanded RET PE (0x6400); then setret; then compact tmpl.
  let retPeRel = -1;
  for (let i = 0; i < data.length - 1; i++) {
    if (data[i] === 0x4d && data[i + 1] === 0x5a) {
      retPeRel = i;
      break;
    }
  }
  if (retPeRel < 0) throw new Error('ret PE MZ missing in prelude');
  const setretPeRel = retPeRel + S3_PARSE1_RET_PE_LEN;
  const compactMark = data.indexOf(Buffer.from('yoyo_compact3_c\0'));
  const compactTmplRel = compactMark - compactTmpl.length;

  let h00Len = 0;
  let fieldMatchLen = 0;
  let extras = [];
  let h00HdrRel = 0;
  let h00MainRel = 0;
  let h00DataRel = 0;
  let h00MainLen = 2676;
  if (opts.seedPe) {
    const userCodeLen = opts.userCodeLen != null ? opts.userCodeLen : 17810;
    const before = data.length;
    data = appendH00BakeFromSeed(data, opts.seedPe, userCodeLen);
    h00Len = data.length - before;
    const hdrMark = data.indexOf(Buffer.from('yoyo_h00_hdr_c\0'));
    const mainMark = data.indexOf(Buffer.from('yoyo_h00_main_c\0'));
    h00HdrRel = hdrMark - S3_H00_HDR_STARTUP_LEN;
    let p = hdrMark + 'yoyo_h00_hdr_c\0'.length;
    if (p % 16) p += 16 - (p % 16);
    h00MainRel = p;
    h00MainLen = mainMark - h00MainRel;
    p = mainMark + 'yoyo_h00_main_c\0'.length;
    if (p % 16) p += 16 - (p % 16);
    h00DataRel = p;
  }
  if (opts.fieldMatch) {
    if (!opts.seedPe) {
      throw new Error('fieldMatch requires seedPe (H00 bake precedes FM)');
    }
    const before = data.length;
    const fm = appendFieldMatchBake(data);
    data = fm.data;
    extras = fm.extras;
    fieldMatchLen = data.length - before;
  }
  const USER = 0x120;
  return {
    data,
    compactLen: compactTmpl.length,
    retPeLen: retExpanded.length,
    h00Len,
    fieldMatchLen,
    extras,
    meta: {
      input_off: USER + inputRel,
      output_off: USER + outputRel,
      ret_pe_off: USER + retPeRel,
      setret_pe_off: USER + setretPeRel,
      compact_tmpl_off: USER + compactTmplRel,
      compact_tmpl_len: compactTmpl.length,
      seed_len: S3_PARSE1_RET_PE_LEN,
      h00_hdr_off: USER + h00HdrRel,
      h00_hdr_len: S3_H00_HDR_STARTUP_LEN,
      h00_main_off: USER + h00MainRel,
      h00_main_len: h00MainLen,
      h00_data_off: USER + h00DataRel,
      h00_data_len: S3_H00_DATA_FLOOR,
      extras: extras.map(([tr, tl, pr, pl]) => [USER + tr, tl, USER + pr, pl]),
    },
  };
}

module.exports = {
  expandRetPeTextRaw20k,
  buildCompilerDataPrelude,
  appendH00BakeFromSeed,
  appendFieldMatchBake,
  compileTybToPeChild,
  fixed10RetTyb,
  S3_PARSE1_RET_PE_LEN,
  S3_H00_HDR_STARTUP_LEN,
  S3_H00_DATA_FLOOR,
  FM_SINGLES,
  FM_CHAINS,
};
