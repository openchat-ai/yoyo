'use strict';
/**
 * HB-FX FX-2b-4 — JS rebuild of parse1 compiler code.bin.
 *
 * Interprets pe_dll_link::yoyo_compiler_ret_compile_code_data emit body
 * (extend_from_slice + labels + patch_jcc/jmp + extras/seed specials).
 * Does not invoke Rust/cargo; origin = js_parse1_source_interp.
 */

const fs = require('fs');
const path = require('path');
const { buildCompilerDataPrelude, S3_PARSE1_RET_PE_LEN, S3_H00_DATA_FLOOR, S3_H00_HDR_STARTUP_LEN } =
  require('./yoyo-compiler-emit-prelude');
const { STR_TABLE_OFF } = require('./platform-io');

const RUST_EMIT = path.join(
  __dirname,
  '..',
  '..',
  '..',
  'yoyo-rust',
  'verifier',
  'src',
  'pe_dll_link.rs',
);

// Win32 body seeds (peer of s1_tyb_parser constants) — poke sites zeroed.
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

function u32le(n) {
  const b = Buffer.alloc(4);
  b.writeUInt32LE(n >>> 0, 0);
  return [...b];
}

function parseByteArray(inner) {
  const out = [];
  const re = /0x([0-9a-fA-F]+)|(\d+)/g;
  let m;
  while ((m = re.exec(inner))) {
    out.push(m[1] != null ? parseInt(m[1], 16) : parseInt(m[2], 10));
  }
  return out;
}

function evalU32(expr, env) {
  let e = expr.trim();
  e = e.replace(/\.to_le_bytes\(\)\s*$/, '');
  e = e.replace(/\bas u32\b/g, '');
  e = e.replace(/crate::platform_io::STR_TABLE_OFF/g, 'STR_TABLE_OFF');
  e = e.replace(/crate::s1_tyb_parser::S1_MINI_DATA_MAX/g, 'S1_MINI_DATA_MAX');
  e = e.replace(/\.wrapping_add\(([^)]+)\)/g, '+ ($1)');
  // Replace known names with numbers (longest first)
  const names = Object.keys({ ...CONSTS, ...env }).sort((a, b) => b.length - a.length);
  for (const n of names) {
    const v = n in env ? env[n] : CONSTS[n];
    e = e.replace(new RegExp(`\\b${n}\\b`, 'g'), String(v >>> 0));
  }
  if (!/^[\d\s+\-*/%()|<>&^~x]+$/i.test(e.replace(/>>>/g, '').replace(/<</g, ''))) {
    // allow >>> and <<
    if (!/^[\d\s+\-*/%()|<>&^~x]+$/i.test(e.replace(/>>>/g, ' ').replace(/<</g, ' '))) {
      throw new Error(`evalU32 bad expr after subst: ${expr} -> ${e}`);
    }
  }
  // eslint-disable-next-line no-new-func
  const n = Function(`"use strict"; return (${e});`)();
  return n >>> 0;
}

function emitSeedBody(c, seed) {
  let off = 0;
  while (off + 8 <= seed.length) {
    c.push(0x48, 0xb8, ...seed.subarray(off, off + 8));
    if (off === 0) c.push(0x48, 0x89, 0x07);
    else if (off < 0x80) c.push(0x48, 0x89, 0x47, off);
    else c.push(0x48, 0x89, 0x87, ...u32le(off));
    off += 8;
  }
  while (off < seed.length) {
    c.push(0xc6, 0x87, ...u32le(off), seed[off]);
    off += 1;
  }
}

function emitExtrasLoop(c, extras, extraPatches) {
  for (let i = 0; i < extras.length; i++) {
    const [tybOff, tybLen, peOff, peLen] = extras[i];
    c.push(0x81, 0xff, ...u32le(tybLen));
    const jne1 = c.length;
    c.push(0x0f, 0x85, 0, 0, 0, 0);
    c.push(0x50, 0x51, 0x57, 0x56);
    c.push(0xb9, ...u32le(tybLen));
    c.push(0x49, 0x8d, 0xbf, ...u32le(tybOff));
    c.push(0xf3, 0xa6);
    c.push(0x5e, 0x5f, 0x59, 0x58);
    const jne2 = c.length;
    c.push(0x0f, 0x85, 0, 0, 0, 0);
    if (i <= 6) {
      const jmpE = c.length;
      c.push(0xe9, 0, 0, 0, 0);
      extraPatches.push([jne1, jne2, jmpE, i + 2]); // kinds 2..8
      void peOff;
      void peLen;
    } else {
      c.push(0x49, 0x8d, 0x97, ...u32le(peOff));
      c.push(0x41, 0xb8, ...u32le(peLen));
      const jmpW = c.length;
      c.push(0xe9, 0, 0, 0, 0);
      extraPatches.push([jne1, jne2, jmpW, 0]);
    }
  }
}

function applyExtraPatches(c, labels, extraPatches) {
  const extraStarts = extraPatches.map(([jne1]) => jne1 - 6);
  for (let i = 0; i < extraPatches.length; i++) {
    const [jne1, jne2, jmp, kind] = extraPatches[i];
    const miss =
      i + 1 < extraStarts.length ? extraStarts[i + 1] : labels.parse1_any;
    patchJcc(c, jne1, miss);
    patchJcc(c, jne2, miss);
    if (kind >= 2 && kind <= 8) patchJmp(c, jmp, labels.parse1);
    else if (kind === 1) patchJmp(c, jmp, labels.live2_nop);
    else patchJmp(c, jmp, labels.do_write);
  }
}

function patchJcc(c, at, target) {
  const rel = (target - (at + 6)) | 0;
  c[at + 2] = rel & 0xff;
  c[at + 3] = (rel >>> 8) & 0xff;
  c[at + 4] = (rel >>> 16) & 0xff;
  c[at + 5] = (rel >>> 24) & 0xff;
}

function patchJmp(c, at, target) {
  const rel = (target - (at + 5)) | 0;
  c[at + 1] = rel & 0xff;
  c[at + 2] = (rel >>> 8) & 0xff;
  c[at + 3] = (rel >>> 16) & 0xff;
  c[at + 4] = (rel >>> 24) & 0xff;
}

/**
 * Build parse1 code.bin bytes. Requires seedPe for data meta (same as full data bake).
 */
function buildCompilerCode(opts = {}) {
  if (!opts.seedPe) throw new Error('buildCompilerCode requires seedPe');
  const built = buildCompilerDataPrelude({
    seedPe: opts.seedPe,
    userCodeLen: opts.userCodeLen != null ? opts.userCodeLen : 17810,
    fieldMatch: true,
  });
  const meta = built.meta;
  const env = { ...meta };
  // derived lets used in emit body
  env.tmpl_text_off = meta.compact_tmpl_off + CONSTS.S3_COMPACT_HEADERS;
  env.tmpl_data_off = meta.compact_tmpl_off + CONSTS.S3_COMPACT_TMPL_DATA_RP;

  const src = fs.readFileSync(RUST_EMIT, 'utf8').split(/\r?\n/);
  // Find `let mut c = Vec::new();` after yoyo_compiler_ret_compile_code_data
  let start = -1;
  let end = -1;
  for (let i = 0; i < src.length; i++) {
    if (src[i].includes('fn yoyo_compiler_ret_compile_code_data')) {
      for (let j = i; j < src.length; j++) {
        if (src[j].includes('let mut c = Vec::new()')) {
          start = j;
          break;
        }
      }
      break;
    }
  }
  if (start < 0) throw new Error('code emit start not found');
  for (let i = start; i < src.length; i++) {
    if (/^\s*Ok\(\(c, data\)\)\s*;?\s*$/.test(src[i])) {
      end = i;
      break;
    }
  }
  if (end < 0) throw new Error('code emit end not found');

  const c = [];
  const labels = Object.create(null);
  const patches = []; // {kind:'jcc'|'jmp', atName, toName} or absolute later
  const extraPatches = [];
  let i = start + 1;
  let mode = 'normal'; // normal | extras | load_seed | write_seed | patch_extras

  const pushBytes = (arr) => {
    for (const b of arr) c.push(b & 0xff);
  };

  while (i < end) {
    let line = src[i].trim();
    // strip line comments (naive)
    const cmt = line.indexOf('//');
    if (cmt === 0) {
      i++;
      continue;
    }
    if (cmt > 0) line = line.slice(0, cmt).trim();
    if (!line) {
      i++;
      continue;
    }

    // extras loop entry
    if (line.includes('let mut extra_patches') || line.includes('for (i, &(tyb_off')) {
      // skip to for loop body start
      while (i < end && !src[i].includes('for (i, &(tyb_off')) i++;
      emitExtrasLoop(c, meta.extras, extraPatches);
      // skip until after the for-loop closing brace that precedes parse1_any
      while (i < end && !src[i].includes('let parse1_any = c.len()')) i++;
      continue;
    }

    // load/write seed blocks — expand then skip to post-block poke extend
    if (line.includes('S3_WIN32_LOAD_FILE_BODY_SEED')) {
      emitSeedBody(c, S3_WIN32_LOAD_FILE_BODY_SEED);
      while (i < end && !src[i].includes('0x8B, 0x44, 0x24, 0x5C')) i++;
      continue; // reprocess poke extend
    }
    if (line.includes('S3_WIN32_WRITE_FILE_BODY_SEED')) {
      emitSeedBody(c, S3_WIN32_WRITE_FILE_BODY_SEED);
      while (i < end && !src[i].includes('0x8B, 0x44, 0x24, 0x5C')) i++;
      continue;
    }

    // patch helper defs — skip
    if (line.startsWith('let patch_jcc') || line.startsWith('let patch_jmp')) {
      while (i < end && src[i].trim() !== '};') i++;
      i++;
      continue;
    }

    // extra_patches apply section
    if (line.includes('let mut extra_starts')) {
      // record marker; actual apply after all named patches collected — do at end
      while (i < end && !src[i].includes('patch_jcc(&mut c, jne_fail_any_tyb')) i++;
      // stop before jne_fail_any — those are normal patches below
      continue;
    }

    // label alias
    let m;
    if ((m = line.match(/^let (\w+) = (\w+);\s*$/))) {
      if (labels[m[2]] != null) labels[m[1]] = labels[m[2]];
      else env[m[1]] = env[m[2]];
      i++;
      continue;
    }

    // let name = c.len();
    if ((m = line.match(/^let (_\w+|\w+) = c\.len\(\);\s*$/))) {
      labels[m[1]] = c.length;
      i++;
      continue;
    }

    // let _ = ...
    if (line.startsWith('let _ ')) {
      i++;
      continue;
    }

    // derived offs: let tmpl_text_off = ...
    if ((m = line.match(/^let (\w+) = (.+);\s*$/))) {
      try {
        env[m[1]] = evalU32(m[2], env);
      } catch (_) {
        /* ignore complex */
      }
      i++;
      continue;
    }

    // patch_jcc / patch_jmp
    if ((m = line.match(/^patch_(jcc|jmp)\(&mut c,\s*(\w+),\s*(\w+)\);\s*$/))) {
      patches.push({ kind: m[1], at: m[2], to: m[3] });
      i++;
      continue;
    }

    // extend_from_slice — may be multiline
    if (line.includes('c.extend_from_slice(')) {
      let blob = line;
      while (!blob.includes(');') && i + 1 < end) {
        i++;
        blob += ' ' + src[i].trim();
      }
      if ((m = blob.match(/c\.extend_from_slice\(&\[([\s\S]*?)\]\s*\)/))) {
        pushBytes(parseByteArray(m[1]));
      } else if ((m = blob.match(/c\.extend_from_slice\(&([\s\S]+?)\)\s*;/))) {
        const arg = m[1].trim();
        if (arg.includes('to_le_bytes') || arg.includes('as u32') || /^[A-Za-z_]/.test(arg)) {
          pushBytes(u32le(evalU32(arg, env)));
        } else {
          throw new Error(`unhandled extend: ${blob.slice(0, 120)}`);
        }
      } else {
        throw new Error(`parse extend fail: ${blob.slice(0, 120)}`);
      }
      i++;
      continue;
    }

    // skip braces / lone tokens
    if (line === '{' || line === '}' || line === '};') {
      i++;
      continue;
    }

    throw new Error(`unhandled emit line ${i + 1}: ${line}`);
  }

  // Apply named patches
  for (const p of patches) {
    const at = labels[p.at];
    const to = labels[p.to];
    if (at == null) throw new Error(`missing patch at label ${p.at}`);
    if (to == null) throw new Error(`missing patch to label ${p.to}`);
    if (p.kind === 'jcc') patchJcc(c, at, to);
    else patchJmp(c, at, to);
  }
  applyExtraPatches(c, labels, extraPatches);

  return {
    code: Buffer.from(c),
    data: built.data,
    meta,
    labels,
  };
}

module.exports = {
  buildCompilerCode,
  CONSTS,
};
