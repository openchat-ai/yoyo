#!/usr/bin/env node
/**
 * golden.js — Appendix F golden runner (G00–G02).
 * Fail-closed: missing fixture / mismatch → exit ≠ 0.
 * Does NOT claim full Appendix F (G03–G06) green.
 */
'use strict';

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { encodeOp, loadState, storeState, movabsRax } = require('../src/platform/encode-x64');

const ROOT = path.resolve(__dirname, '../..');
const GOLDEN_DIR = path.join(ROOT, 'yoyo', 'tests', 'golden');

function parseTy(src) {
  // Mirror yoyo.js M0 seed: bind named slots starting at USER_SLOT_BASE.
  // Single-letter / short names are ambiguous with hex (e.g. 'a' = 0x0A);
  // we follow the same rule as yoyo.js: if the token parses cleanly as hex,
  // treat it as a hex literal. Otherwise bind the next free slot ≥ 0x50.
  //
  // Signed hex literals: tokens starting with `-` followed by hex digits
  // (e.g. `-80`, `-0x80`, `-128`) parse as the signed integer value. This
  // supports tests that exercise the signed-imm8 boundary (e.g. LDB off=-128).
  const lines = [];
  const names = new Map();
  let nextSlot = 0x50;
  const isHex = (t) => /^(0x)?[0-9a-fA-F]+$/.test(t);
  const hex = (t) => parseInt(t.replace(/^0x/i, ''), 16);
  const isSignedHex = (t) => /^-0x[0-9a-fA-F]+$/.test(t) || /^-[0-9a-fA-F]+$/.test(t);
  const signedHex = (t) => -parseInt(t.replace(/^-/, '').replace(/^0x/i, ''), 16);
  const slotOf = (t) => {
    if (isSignedHex(t)) return signedHex(t);
    if (isHex(t)) return hex(t);
    if (names.has(t)) return names.get(t);
    const s = nextSlot++;
    names.set(t, s);
    return s;
  };
  for (const raw of src.split(/\r?\n/)) {
    const line = raw.replace(/[;#].*$/, '').trim();
    if (!line) continue;
    const toks = line.split(/\s+/);
    const op = parseInt(toks[0], 16);
    const args = toks.slice(1).map(slotOf);
    lines.push({ op, args });
  }
  return lines;
}

/**
 * Mirror yoyo.js compile: emit handlers as labels, branches as placeholders,
 * patch rel32 to the matching handler offset. Used by G03/G04 fixtures.
 */
function compileCode(lines) {
  const code = [];
  const labels = new Map();
  const fixups = [];
  // Label ids are full numeric args (not masked to u8). Values ≥0x100 use
  // multi-digit hex tokens (e.g. `40 100`); wrapping via &0xff would collide H_00..
  const labelId = (a) => {
    const hh = a[0];
    if (!Number.isInteger(hh) || hh < 0 || hh > 0xffff) {
      throw new Error('label id out of range: ' + hh);
    }
    return hh;
  };
  for (const { op, args } of lines) {
    if (op === 0x40) {
      labels.set(labelId(args), code.length);
      continue;
    }
    if (op === 0x10 || op === 0x12 || op === 0x13) continue;
    if (op === 0x41 || op === 0x70 || (op >= 0x71 && op <= 0x7a)) {
      const start = code.length;
      code.push(...encodeOp(op, args, true));
      const relAt = op >= 0x71 && op <= 0x7a ? start + 2 : start + 1;
      fixups.push({ relAt, hh: labelId(args) });
      continue;
    }
    code.push(...encodeOp(op, args, false));
  }
  for (const f of fixups) {
    if (!labels.has(f.hh)) throw new Error('undefined label H_' + f.hh.toString(16));
    const rel = labels.get(f.hh) - (f.relAt + 4);
    const b = Buffer.alloc(4); b.writeInt32LE(rel, 0);
    code[f.relAt] = b[0]; code[f.relAt + 1] = b[1];
    code[f.relAt + 2] = b[2]; code[f.relAt + 3] = b[3];
  }
  return Buffer.from(code);
}

function loadExpectedHex(file) {
  const text = fs.readFileSync(file, 'utf8')
    .replace(/;[^\r\n]*/g, '')
    .replace(/\s+/g, '')
    .toLowerCase();
  if (!/^[0-9a-f]*$/.test(text) || text.length % 2 !== 0) {
    throw new Error(`bad expected hex: ${file}`);
  }
  return Buffer.from(text, 'hex');
}

function hexOf(buf) {
  return Buffer.from(buf).toString('hex');
}

function readUtf8(p) {
  return fs.readFileSync(p, 'utf8');
}

/** G00: code MUST be exactly 90 C3 (no undocumented extra nops). */
function checkG00() {
  const tyPath = path.join(GOLDEN_DIR, '00_nop_ret.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', '00_nop_ret.code.hex');
  if (!fs.existsSync(tyPath)) {
    return { id: 'G00', ok: false, detail: `missing fixture ${tyPath}` };
  }
  if (!fs.existsSync(expPath)) {
    return { id: 'G00', ok: false, detail: `missing expected ${expPath}` };
  }
  const expected = loadExpectedHex(expPath);
  if (expected.length < 2 || expected[0] !== 0x90 || expected[expected.length - 1] !== 0xc3) {
    return { id: 'G00', ok: false, detail: 'expected pin must be NOP…RET (90 … C3)' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  if (!got.equals(expected)) {
    return {
      id: 'G00',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  if (!got.includes(Buffer.from([0x90, 0xc3]))) {
    return { id: 'G00', ok: false, detail: 'missing 90 C3 pattern' };
  }
  return { id: 'G00', ok: true, detail: `code=${hexOf(got)}` };
}

/**
 * G01: SET/GET round-trip via self-test harness.
 * Fixture pin + harness: SET store disp ≡ GET load disp for transferred slot.
 */
function checkG01() {
  const tyPath = path.join(GOLDEN_DIR, '01_set_get.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', '01_set_get.code.hex');
  if (!fs.existsSync(tyPath)) {
    return { id: 'G01', ok: false, detail: `missing fixture ${tyPath}` };
  }
  if (!fs.existsSync(expPath)) {
    return { id: 'G01', ok: false, detail: `missing expected ${expPath}` };
  }
  const expected = loadExpectedHex(expPath);
  const got = compileCode(parseTy(readUtf8(tyPath)));
  if (!got.equals(expected)) {
    return {
      id: 'G01',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }

  // Self-test harness: slot 0x50 imm 0x2A → slot 0x51
  const slotSrc = 0x50;
  const slotDst = 0x51;
  const imm = 0x2a;
  const setBytes = Buffer.from([...movabsRax(imm), ...storeState(slotSrc, 0, 0)]);
  const getBytes = Buffer.from([...loadState(slotSrc, 0, 0), ...storeState(slotDst, 0, 0)]);
  const storeSrc = Buffer.from(storeState(slotSrc, 0, 0));
  const loadSrc = Buffer.from(loadState(slotSrc, 0, 0));
  if (!setBytes.subarray(setBytes.length - storeSrc.length).equals(storeSrc)) {
    return { id: 'G01', ok: false, detail: 'SET does not end with store_state(src)' };
  }
  if (!getBytes.subarray(0, loadSrc.length).equals(loadSrc)) {
    return { id: 'G01', ok: false, detail: 'GET does not start with load_state(src)' };
  }
  // Round-trip: store_state(src) and load_state(src) share the same displacement bytes
  const storeDisp = storeSrc.subarray(3);
  const loadDisp = loadSrc.subarray(3);
  if (!storeDisp.equals(loadDisp)) {
    return {
      id: 'G01',
      ok: false,
      detail: `slot disp mismatch store=${hexOf(storeDisp)} load=${hexOf(loadDisp)}`,
    };
  }
  if (setBytes[0] !== 0x48 || setBytes[1] !== 0xb8) {
    return { id: 'G01', ok: false, detail: 'SET missing movabs rax' };
  }
  return { id: 'G01', ok: true, detail: `code=${hexOf(got)} round-trip slot=0x${slotSrc.toString(16)}` };
}

/** G02: ORV ≠ ADDV byte stream for same operands; pins + text.sha256. */
function checkG02() {
  const tyPath = path.join(GOLDEN_DIR, '02_addv_orv.ty');
  const addvPath = path.join(GOLDEN_DIR, 'expected', '02_addv_orv.addv.hex');
  const orvPath = path.join(GOLDEN_DIR, 'expected', '02_addv_orv.orv.hex');
  const shaPath = path.join(GOLDEN_DIR, 'expected', '02_addv_orv.text.sha256');
  for (const p of [tyPath, addvPath, orvPath, shaPath]) {
    if (!fs.existsSync(p)) {
      return { id: 'G02', ok: false, detail: `missing ${p}` };
    }
  }
  const dst = 0x50;
  const src = 0x51;
  const addvGot = Buffer.from(encodeOp(0x68, [dst, src], false));
  const orvGot = Buffer.from(encodeOp(0x69, [dst, src], false));
  const addvExp = loadExpectedHex(addvPath);
  const orvExp = loadExpectedHex(orvPath);
  if (!addvGot.equals(addvExp)) {
    return {
      id: 'G02',
      ok: false,
      detail: `ADDV mismatch: got ${hexOf(addvGot)} want ${hexOf(addvExp)}`,
    };
  }
  if (!orvGot.equals(orvExp)) {
    return {
      id: 'G02',
      ok: false,
      detail: `ORV mismatch: got ${hexOf(orvGot)} want ${hexOf(orvExp)}`,
    };
  }
  if (addvGot.equals(orvGot)) {
    return { id: 'G02', ok: false, detail: 'ORV aliases ADDV (MUST differ)' };
  }
  // Fixture must compile and contain both streams
  const full = compileCode(parseTy(readUtf8(tyPath)));
  if (!full.includes(addvGot) || !full.includes(orvGot)) {
    return { id: 'G02', ok: false, detail: 'fixture code missing ADDV or ORV stream' };
  }
  const text = `addv=${hexOf(addvExp)}\norv=${hexOf(orvExp)}\n`;
  const gotSha = crypto.createHash('sha256').update(text, 'utf8').digest('hex');
  const expSha = readUtf8(shaPath).replace(/\s+/g, '').toLowerCase();
  if (gotSha !== expSha) {
    return {
      id: 'G02',
      ok: false,
      detail: `text.sha256 mismatch: got ${gotSha} want ${expSha}`,
    };
  }
  return {
    id: 'G02',
    ok: true,
    detail: `addv≠orv sha256=${gotSha.slice(0, 16)}…`,
  };
}

/** G03: CMP+JE patches rel32 to labeled handler (Appendix F). */
function checkG03() {
  const tyPath = path.join(GOLDEN_DIR, '03_cmp_je.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', '03_cmp_je.code.hex');
  if (!fs.existsSync(tyPath)) {
    return { id: 'G03', ok: false, detail: `missing fixture ${tyPath}` };
  }
  if (!fs.existsSync(expPath)) {
    return { id: 'G03', ok: false, detail: `missing expected ${expPath}` };
  }
  const expected = loadExpectedHex(expPath);
  const got = compileCode(parseTy(readUtf8(tyPath)));
  if (!got.equals(expected)) {
    return {
      id: 'G03',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  // JE opcode signature: 0F 84 (CC=je)
  const jePattern = Buffer.from([0x0f, 0x84]);
  const jeIdx = got.indexOf(jePattern);
  if (jeIdx < 0) {
    return { id: 'G03', ok: false, detail: 'JE opcode 0F 84 missing in stream' };
  }
  // rel32 lives at jeIdx + 2
  const rel32 = got.readInt32LE(jeIdx + 2);
  // After JE (6 bytes), the rel32 target MUST point inside the emitted buffer.
  const target = jeIdx + 2 + 4 + rel32;
  if (target < 0 || target >= got.length) {
    return {
      id: 'G03',
      ok: false,
      detail: `JE rel32=${rel32} out of range (target=${target}, len=${got.length})`,
    };
  }
  // CMP emit stream (state[0x50], state[0x51]) must appear before JE.
  const cmpStream = Buffer.from(encodeOp(0x65, [0x50, 0x51], false));
  const cmpIdx = got.indexOf(cmpStream);
  if (cmpIdx < 0) {
    return { id: 'G03', ok: false, detail: 'CMP stream missing in fixture' };
  }
  if (cmpIdx > jeIdx) {
    return {
      id: 'G03',
      ok: false,
      detail: `CMP must precede JE (cmp@${cmpIdx}, je@${jeIdx})`,
    };
  }
  return {
    id: 'G03',
    ok: true,
    detail: `je@${jeIdx} rel32=${rel32} target=0x${target.toString(16)} cmp@${cmpIdx}`,
  };
}

/**
 * INC: single op emission at slot 0x50 MUST match disk bytes (Appendix F).
 * Disk: selfhost_min_inc.code.hex = load state[0x50] + inc rax + store + RET.
 */
function checkINC() {
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_inc.code.hex');
  if (!fs.existsSync(expPath)) {
    return { id: 'INC', ok: false, detail: `missing expected ${expPath}` };
  }
  const slot = 0x50;
  const got = Buffer.from([...encodeOp(0x66, [slot]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return {
      id: 'INC',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  // Pin signature: 48 ff c0 = inc rax (REX.W + ff /0 with modrm c0)
  const incSig = Buffer.from([0x48, 0xff, 0xc0]);
  if (!got.includes(incSig)) {
    return { id: 'INC', ok: false, detail: 'inc rax signature 48 ff c0 missing' };
  }
  if (got[got.length - 1] !== 0xc3) {
    return { id: 'INC', ok: false, detail: 'program must terminate with RET (c3)' };
  }
  return {
    id: 'INC',
    ok: true,
    detail: `slot=0x${slot.toString(16)} code=${hexOf(got)}`,
  };
}

/**
 * DEC: single op emission at slot 0x50 MUST match disk bytes (Appendix F).
 * Disk: selfhost_min_dec.code.hex = load state[0x50] + dec rax + store + RET.
 */
function checkDEC() {
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_dec.code.hex');
  if (!fs.existsSync(expPath)) {
    return { id: 'DEC', ok: false, detail: `missing expected ${expPath}` };
  }
  const slot = 0x50;
  const got = Buffer.from([...encodeOp(0x67, [slot]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return {
      id: 'DEC',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  // Pin signature: 48 ff c8 = dec rax (REX.W + ff /1 with modrm c8)
  const decSig = Buffer.from([0x48, 0xff, 0xc8]);
  if (!got.includes(decSig)) {
    return { id: 'DEC', ok: false, detail: 'dec rax signature 48 ff c8 missing' };
  }
  if (got[got.length - 1] !== 0xc3) {
    return { id: 'DEC', ok: false, detail: 'program must terminate with RET (c3)' };
  }
  return {
    id: 'DEC',
    ok: true,
    detail: `slot=0x${slot.toString(16)} code=${hexOf(got)}`,
  };
}

/**
 * ADD-IMM: body-extend-001 single op emission at slot 0x50 with imm=3 MUST
 * match disk bytes (Appendix F). Disk: selfhost_min_add_imm.code.hex =
 * load state[0x50] + add rax,3 + store + RET. Mirrors INC/DEC template.
 *
 * Both peers compose identical bytes via the same x86-64 primitives:
 *   JS:   loadState(0x50) + addImmRax(3) + storeState(0x50) + 0xC3
 *   Rust: emit_add_imm(0x50, 3) + ret()
 *   0x62 = ADD slot, imm (PROMPT Part 4.1 / isa_table.txt).
 * No D-1/D-2 aliasing; 2-arg opcode; both imm8 path (3 ∈ [-128, 127]).
 */
function checkADDIMM() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_add_imm.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_add_imm.code.hex');
  if (!fs.existsSync(tyPath)) {
    return { id: 'ADD-IMM', ok: false, detail: `missing fixture ${tyPath}` };
  }
  if (!fs.existsSync(expPath)) {
    return { id: 'ADD-IMM', ok: false, detail: `missing expected ${expPath}` };
  }
  const slot = 0x50;
  const imm = 3;
  const got = Buffer.from([...encodeOp(0x62, [slot, imm]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return {
      id: 'ADD-IMM',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  // Pin signature: add rax, imm8 = 48 83 c0 03 (REX.W + 83 /0 + imm8)
  const addImmSig = Buffer.from([0x48, 0x83, 0xc0, 0x03]);
  if (!got.includes(addImmSig)) {
    return {
      id: 'ADD-IMM',
      ok: false,
      detail: 'add rax,imm8 0x03 signature 48 83 c0 03 missing',
    };
  }
  // load_state(0x50) must appear (disp32 = 0x280 LE = 80 02 00 00)
  const load50 = Buffer.from([0x49, 0x8b, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (!got.includes(load50)) {
    return {
      id: 'ADD-IMM',
      ok: false,
      detail: 'load_state(0x50,rax) = 49 8b 87 80 02 00 00 missing (disp 0x280 LE)',
    };
  }
  // store_state(0x50) must appear after the add
  const store50 = Buffer.from([0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (!got.includes(store50)) {
    return {
      id: 'ADD-IMM',
      ok: false,
      detail: 'store_state(0x50,rax) = 49 89 87 80 02 00 00 missing (disp 0x280 LE)',
    };
  }
  if (got[got.length - 1] !== 0xc3) {
    return { id: 'ADD-IMM', ok: false, detail: 'program must terminate with RET (c3)' };
  }
  // Cross-check: full fixture compile must equal the encoded stream.
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return {
      id: 'ADD-IMM',
      ok: false,
      detail: `fixture compile mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}`,
    };
  }
  return {
    id: 'ADD-IMM',
    ok: true,
    detail: `slot=0x${slot.toString(16)} imm=${imm} code=${hexOf(got)}`,
  };
}

/**
 * MOVRR: body-extend-003 single op emission, 0x64 independent route (D-2 Phase 2).
 * Both peers compose identical bytes via separate emit paths:
 *   JS:   loadState(src) + storeState(dst) + 0xC3  (encodeOp 0x64 branch)
 *   Rust: emit_movrr(dst, src) + ret()
 *   0x64 = MOVRR dst, src (PROMPT Part 4.1 / isa_table.txt).
 * Same slot-copy semantics as GET (0x60) but routed independently from emit_get.
 */
function checkMOVRR() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_movrr.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_movrr.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'MOVRR', ok: false, detail: 'missing MOVRR fixture or expected pin' };
  }
  const dst = 0x50;
  const src = 0x51;
  const got = Buffer.from([...encodeOp(0x64, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) return { id: 'MOVRR', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  const load51 = Buffer.from([0x49, 0x8b, 0x87, 0x88, 0x02, 0x00, 0x00]);
  const store50 = Buffer.from([0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (!got.includes(load51) || !got.includes(store50) || got[got.length - 1] !== 0xc3) {
    return { id: 'MOVRR', ok: false, detail: 'MOVRR must be load(src)+store(dst)+RET shape' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) return { id: 'MOVRR', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  return { id: 'MOVRR', ok: true, detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}` };
}

/**
 * ORV: body-extend-004 single op emission, 2-arg state[0x50] |= state[0x51].
 * Mirrors MOVRR template but exercises the bitwise-OR path. Both peers
 * compose identical bytes via the same x86-64 primitives:
 *   JS:   loadState(0x50) + loadState(0x51,rcx) + orRegRaxRcx + storeState(0x50) + 0xC3
 *   Rust: emit_orv(0x50, 0x51) + ret()
 *   0x69 = ORV dst, src (PROMPT Part 4.1 / isa_table.txt).
 * No D-1/D-2/D-3/D-4 aliasing; 2-arg opcode; both peers route through or_reg
 * (NOT add_reg — the audit-defect flag in PROMPT Part 4.1 is satisfied).
 */
function checkORV() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_orv.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_orv.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'ORV', ok: false, detail: 'missing ORV fixture or expected pin' };
  }
  const dst = 0x50;
  const src = 0x51;
  const got = Buffer.from([...encodeOp(0x69, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'ORV', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  // Pin signature: or rax,rcx = 48 09 c8 (REX.W + 09 /1 with modrm c8).
  const orSig = Buffer.from([0x48, 0x09, 0xc8]);
  if (!got.includes(orSig)) {
    return { id: 'ORV', ok: false, detail: 'or rax,rcx signature 48 09 c8 missing (audit defect: NOT add_reg)' };
  }
  // load_state(0x50) must appear (disp32 = 0x280 LE = 80 02 00 00)
  const load50 = Buffer.from([0x49, 0x8b, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (!got.includes(load50)) {
    return {
      id: 'ORV',
      ok: false,
      detail: 'load_state(0x50,rax) = 49 8b 87 80 02 00 00 missing (disp 0x280 LE)',
    };
  }
  // load_state(0x51,rcx) must appear (disp32 = 0x288 LE = 88 02 00 00)
  const load51 = Buffer.from([0x49, 0x8b, 0x8f, 0x88, 0x02, 0x00, 0x00]);
  if (!got.includes(load51)) {
    return {
      id: 'ORV',
      ok: false,
      detail: 'load_state(0x51,rcx) = 49 8b 8f 88 02 00 00 missing (disp 0x288 LE)',
    };
  }
  // store_state(0x50) must appear after the or
  const store50 = Buffer.from([0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (!got.includes(store50)) {
    return {
      id: 'ORV',
      ok: false,
      detail: 'store_state(0x50,rax) = 49 89 87 80 02 00 00 missing (disp 0x280 LE)',
    };
  }
  if (got[got.length - 1] !== 0xc3) {
    return { id: 'ORV', ok: false, detail: 'program must terminate with RET (c3)' };
  }
  // Cross-check: full fixture compile must equal the encoded stream.
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return {
      id: 'ORV',
      ok: false,
      detail: `fixture compile mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}`,
    };
  }
  return {
    id: 'ORV',
    ok: true,
    detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}`,
  };
}

/**
 * SUB-IMM: body-extend-002 single op emission at slot 0x50 with imm=3 MUST
 * match disk bytes (Appendix F). Disk: selfhost_min_sub_imm.code.hex =
 * load state[0x50] + sub rax,3 + store + RET. Mirrors ADD-IMM template.
 *
 * Both peers compose identical bytes via the same x86-64 primitives:
 *   JS:   loadState(0x50) + subImmRax(3) + storeState(0x50) + 0xC3
 *   Rust: emit_sub_imm(0x50, 3) + ret()
 *   0x61 = SUB slot, imm (PROMPT Part 4.1 / isa_table.txt).
 * No D-1/D-2 aliasing; 2-arg opcode; both imm8 path (3 ∈ [-128, 127]).
 * Differs from ADD-IMM ONLY at the imm byte (0xe8 vs 0xc0) — ModRM /5 vs /0.
 */
function checkSUBIMM() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_sub_imm.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_sub_imm.code.hex');
  if (!fs.existsSync(tyPath)) {
    return { id: 'SUB-IMM', ok: false, detail: `missing fixture ${tyPath}` };
  }
  if (!fs.existsSync(expPath)) {
    return { id: 'SUB-IMM', ok: false, detail: `missing expected ${expPath}` };
  }
  const slot = 0x50;
  const imm = 3;
  const got = Buffer.from([...encodeOp(0x61, [slot, imm]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return {
      id: 'SUB-IMM',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  // Pin signature: sub rax, imm8 = 48 83 e8 03 (REX.W + 83 /5 + imm8)
  const subImmSig = Buffer.from([0x48, 0x83, 0xe8, 0x03]);
  if (!got.includes(subImmSig)) {
    return {
      id: 'SUB-IMM',
      ok: false,
      detail: 'sub rax,imm8 0x03 signature 48 83 e8 03 missing',
    };
  }
  // load_state(0x50) must appear (disp32 = 0x280 LE = 80 02 00 00)
  const load50 = Buffer.from([0x49, 0x8b, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (!got.includes(load50)) {
    return {
      id: 'SUB-IMM',
      ok: false,
      detail: 'load_state(0x50,rax) = 49 8b 87 80 02 00 00 missing (disp 0x280 LE)',
    };
  }
  // store_state(0x50) must appear after the sub
  const store50 = Buffer.from([0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (!got.includes(store50)) {
    return {
      id: 'SUB-IMM',
      ok: false,
      detail: 'store_state(0x50,rax) = 49 89 87 80 02 00 00 missing (disp 0x280 LE)',
    };
  }
  if (got[got.length - 1] !== 0xc3) {
    return { id: 'SUB-IMM', ok: false, detail: 'program must terminate with RET (c3)' };
  }
  // Cross-check: full fixture compile must equal the encoded stream.
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return {
      id: 'SUB-IMM',
      ok: false,
      detail: `fixture compile mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}`,
    };
  }
  return {
    id: 'SUB-IMM',
    ok: true,
    detail: `slot=0x${slot.toString(16)} imm=${imm} code=${hexOf(got)}`,
  };
}

/**
 * JMP: rel32 branch emission (op 0x70) MUST match disk bytes (Appendix F).
 * Disk: selfhost_min_jmp.code.hex = LABEL H_00 + SET state[0x50]=0 + RET +
 *   JMP H_00 (rel32=-23 → offset 0) + RET.
 * compileCode exercises the rel32 fixup path; encodeOp(0x70,...) alone can't
 * reproduce the 24-byte disk stream (true→5B stub, false→1B ret).
 */
function checkJMP() {
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_jmp.code.hex');
  if (!fs.existsSync(expPath)) {
    return { id: 'JMP', ok: false, detail: `missing expected ${expPath}` };
  }
  const src = '40 00\n20 50 00\nff\n70 00\nff\n';
  const got = compileCode(parseTy(src));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return {
      id: 'JMP',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  // Pin signature: JMP opcode 0xE9 must appear with a patched (non-zero) rel32.
  const jmpIdx = got.indexOf(0xe9);
  if (jmpIdx < 0) {
    return { id: 'JMP', ok: false, detail: 'JMP opcode 0xE9 missing' };
  }
  const rel32 = got.readInt32LE(jmpIdx + 1);
  if (rel32 === 0) {
    return { id: 'JMP', ok: false, detail: 'JMP rel32 must be patched (non-zero)' };
  }
  const target = jmpIdx + 1 + 4 + rel32;
  if (target < 0 || target >= got.length) {
    return {
      id: 'JMP',
      ok: false,
      detail: `JMP rel32=${rel32} out of range (target=${target}, len=${got.length})`,
    };
  }
  if (got[got.length - 1] !== 0xc3) {
    return { id: 'JMP', ok: false, detail: 'program must terminate with RET (c3)' };
  }
  return {
    id: 'JMP',
    ok: true,
    detail: `jmp@${jmpIdx} rel32=${rel32} target=0x${target.toString(16)} code=${hexOf(got)}`,
  };
}

/**
 * LDB: three-handler fixture pinning load_state(0x60,rax) +
 *   movzx rax,byte[rax] + store_state(0x50,rax) per Part §4S.3
 *   (PROMPT-v3 LDB dd ss oo: addr=S[ss]+oo, S[dd]←zx(byte[addr])).
 * Disk: selfhost_min_ldb.code.hex = SET0(0x50) + RET + LDB stream + RET + RET.
 */
function checkLDB() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb.code.hex');
  if (!fs.existsSync(tyPath)) {
    return { id: 'LDB', ok: false, detail: `missing fixture ${tyPath}` };
  }
  if (!fs.existsSync(expPath)) {
    return { id: 'LDB', ok: false, detail: `missing expected ${expPath}` };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return {
      id: 'LDB',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  // Pin signature: movzx rax, byte [rax] = 48 0f b6 00 (REX.W + 0F B6 + modrm 00).
  const movzxSig = Buffer.from([0x48, 0x0f, 0xb6, 0x00]);
  if (!got.includes(movzxSig)) {
    return { id: 'LDB', ok: false, detail: 'movzx rax,byte[rax] signature 48 0f b6 00 missing' };
  }
  // LDB must load state[0x60] (disp 0x300 = 00 03 00 00 LE) before the movzx.
  const load60 = Buffer.from([0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00]);
  if (!got.includes(load60)) {
    return {
      id: 'LDB',
      ok: false,
      detail: 'load_state(0x60,rax) = 49 8b 87 00 03 00 00 missing (disp 0x300 LE)',
    };
  }
  // LDB must store into state[0x50] (disp 0x280 = 80 02 00 00 LE) after the movzx.
  const store50 = Buffer.from([0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (!got.includes(store50)) {
    return {
      id: 'LDB',
      ok: false,
      detail: 'store_state(0x50,rax) = 49 89 87 80 02 00 00 missing (disp 0x280 LE)',
    };
  }
  if (got[got.length - 1] !== 0xc3) {
    return { id: 'LDB', ok: false, detail: 'program must terminate with RET (c3)' };
  }
  return {
    id: 'LDB',
    ok: true,
    detail: `code=${hexOf(got)} len=${got.length}`,
  };
}

function checkLDBoff8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-off8', ok: false, detail: 'missing offset=8 fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-off8', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const sig = Buffer.from([0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
    0x48, 0x83, 0xc0, 0x08, 0x48, 0x0f, 0xb6, 0x00,
    0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (!got.includes(sig)) {
    return { id: 'LDB-off8', ok: false, detail: 'load+add imm8+movzx+store signature missing' };
  }
  return { id: 'LDB-off8', ok: true, detail: `code=${hexOf(got)} len=${got.length}` };
}

/**
 * LDB-OFF8-HANDLER: body-extend-007 H_40 `0x80 LDB dd ss oo` at selector 0x2E
 * with oo=8 (imm8 path). Mirrors checkLDBBODY template but for the CANONICAL
 * HANDLER entry at H_40 (selector 0x2E), distinct from the free-standing
 * selfhost_min_ldb_off8.ty which uses arbitrary H_01. Both peers compose
 * identical bytes via the same x86-64 primitives:
 *   JS:   loadState(0x60,rax) + addImmRax(8) + [0x48,0x0f,0xb6,0x00]
 *         + storeState(0x50,rax) + 0xC3 (23B)
 *   Rust: emit_ldb(0x50, 0x60, 8) which selects add_imm path via
 *         `if oo != 0 { out.extend(add_imm(Reg::Rax, oo as u64)) }`
 *         followed by movzx + store_state + ret() (23B)
 * Pin: 498b87000300004883c008480fb60049898780020000c3 (23B).
 * Disjoint from checkLDBBODY (oo=0, 19B) — exercises the imm8 add_imm
 * code path that bare H_37 does NOT take. Disjoint from checkLDBoff8
 * (free-standing probe at H_01) — H_40 is the canonical handler entry.
 */
function checkLDBOFF8HANDLER() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off8_handler.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off8_handler.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-OFF8-HANDLER', ok: false, detail: 'missing LDB-off8-handler fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-OFF8-HANDLER', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  if (got.length !== 23) {
    return { id: 'LDB-OFF8-HANDLER', ok: false, detail: `expected 23B pin, got ${got.length}B` };
  }
  // Pin signature: load+add imm8+movzx+store+ret slice — 23B total
  const sig = Buffer.from([
    0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) — disp 0x300 LE
    0x48, 0x83, 0xc0, 0x08,                   // add rax, 8 (imm8 path: 8 ∈ [-128, 127])
    0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte [rax]
    0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) — disp 0x280 LE
    0xc3,                                     // ret
  ]);
  if (!got.includes(sig)) {
    return { id: 'LDB-OFF8-HANDLER', ok: false, detail: 'load+add imm8=8+movzx+store+ret 23B signature missing' };
  }
  // Verify the imm8 path is active (NOT imm32): pin absence of `48 81 c0` (imm32 opcode)
  const imm32Opcode = Buffer.from([0x48, 0x81, 0xc0]);
  if (got.includes(imm32Opcode)) {
    return { id: 'LDB-OFF8-HANDLER', ok: false, detail: 'encoder emitted imm32 path for oo=8 (premature imm32 escalation)' };
  }
  return { id: 'LDB-OFF8-HANDLER', ok: true, detail: `selector=0x2E dd=0x50 ss=0x60 oo=8 code=${hexOf(got)} len=${got.length}` };
}

function checkLDBOFF127HANDLER() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off127_handler.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off127_handler.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-OFF127-HANDLER', ok: false, detail: 'missing fixture or expected bytes' };
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || got.length !== 23) return { id: 'LDB-OFF127-HANDLER', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  const imm8 = Buffer.from([0x48, 0x83, 0xc0, 0x7f]);
  const imm32 = Buffer.from([0x48, 0x81, 0xc0, 0x7f, 0x00, 0x00, 0x00]);
  if (!got.includes(imm8) || got.includes(imm32)) return { id: 'LDB-OFF127-HANDLER', ok: false, detail: 'signed-imm8 right-edge assertion failed' };
  return { id: 'LDB-OFF127-HANDLER', ok: true, detail: `selector=0x2F code=${hexOf(got)} len=${got.length}` };
}

/**
 * LDB-OFFM128-HANDLER: body-extend-009 H_42 `0x80 LDB dd ss oo` at selector 0x30
 * with oo=0x50 (positive imm8 byte slot). Mirrors checkLDBOFF127HANDLER
 * template but exercises the LEFT-side imm8 byte (0x50 vs H_41's 0x7f).
 * Both peers compose identical bytes via the same x86-64 primitives:
 *   JS:   loadState(0x60,rax) + addImmRax(0x50) + [0x48,0x0f,0xb6,0x00]
 *         + storeState(0x50,rax) + 0xC3 (23B)
 *   Rust: emit_ldb(0x50, 0x60, 0x50) → load_state(0x60,rax) +
 *         add_imm(rax, 0x50) (imm8 path) + movzx + store_state(0x50,rax)
 *         + ret() (23B)
 * Pin: 498b87000300004883c050480fb60049898780020000c3 (23B).
 * Differs from H_41 ONLY at the imm8 byte (0x50 vs 0x7f). The signed-imm8
 * LEFT-edge signed-token semantic (-128 → imm8 byte 0x80) is covered by
 * the JS-only checkLDBoffm128 probe on selfhost_min_ldb_offm128.ty (Rust
 * ty_parser lacks signed-hex support; emit.rs treats oo as unsigned u16
 * per dispatch contract).
 */
function checkLDBOFFM128HANDLER() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_offm128_handler.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_offm128_handler.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-OFFM128-HANDLER', ok: false, detail: 'missing LDB-offm128-handler fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-OFFM128-HANDLER', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  if (got.length !== 23) {
    return { id: 'LDB-OFFM128-HANDLER', ok: false, detail: `expected 23B pin, got ${got.length}B` };
  }
  // Pin signature: load+add imm8=0x50+movzx+store+ret slice — 23B total
  const sig = Buffer.from([
    0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) — disp 0x300 LE
    0x48, 0x83, 0xc0, 0x50,                   // add rax, 0x50 (imm8 path: 0x50 ∈ [-128, 127] unsigned 80, signed -80 interpretation differs)
    0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte [rax]
    0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) — disp 0x280 LE
    0xc3,                                     // ret
  ]);
  if (!got.includes(sig)) {
    return { id: 'LDB-OFFM128-HANDLER', ok: false, detail: 'load+add imm8=0x50+movzx+store+ret 23B signature missing' };
  }
  // Verify the imm8 path is active (NOT imm32): pin absence of `48 81 c0`.
  const imm32Opcode = Buffer.from([0x48, 0x81, 0xc0]);
  if (got.includes(imm32Opcode)) {
    return { id: 'LDB-OFFM128-HANDLER', ok: false, detail: 'encoder emitted imm32 path for oo=0x50 (premature imm32 escalation)' };
  }
  return { id: 'LDB-OFFM128-HANDLER', ok: true, detail: `selector=0x30 dd=0x50 ss=0x60 oo=0x50 code=${hexOf(got)} len=${got.length}` };
}

/**
 * LDB-OFF64-HANDLER: body-extend-010 H_43 `0x80 LDB dd ss oo` at selector 0x31
 * with oo=0x40 (= 64 decimal; positive imm8 byte). Mirrors checkLDBOFFM128HANDLER
 * template but exercises a fresh imm8 byte (0x40) symmetric to H_42 (0x50).
 * Both peers compose identical bytes via the same x86-64 primitives:
 *   JS:   loadState(0x60,rax) + addImmRax(0x40) + [0x48,0x0f,0xb6,0x00]
 *         + storeState(0x50,rax) + 0xC3 (23B)
 *   Rust: emit_ldb(0x50, 0x60, 0x40) -> load_state(0x60,rax) +
 *         add_imm(rax, 0x40) (imm8 path) + movzx + store_state(0x50,rax)
 *         + ret() (23B)
 * Pin: 498b87000300004883c040480fb60049898780020000c3 (23B).
 * Differs from H_42 ONLY at the imm8 byte (0x40 vs 0x50). Extends the LDB
 * imm8-path byte-coverage matrix to include 0x40 (=64). The signed-imm8
 * LEFT-edge signed-token semantic (-128 -> imm8 byte 0x80) remains covered
 * by the JS-only checkLDBoffm128 probe (Rust ty_parser lacks signed-hex
 * support; emit.rs treats oo as unsigned u16 per dispatch contract).
 */
function checkLDBOFF64HANDLER() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off64_handler.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off64_handler.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-OFF64-HANDLER', ok: false, detail: 'missing LDB-off64-handler fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-OFF64-HANDLER', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  if (got.length !== 23) {
    return { id: 'LDB-OFF64-HANDLER', ok: false, detail: `expected 23B pin, got ${got.length}B` };
  }
  // Pin signature: load+add imm8=0x40+movzx+store+ret slice - 23B total
  const sig = Buffer.from([
    0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) - disp 0x300 LE
    0x48, 0x83, 0xc0, 0x40,                   // add rax, 0x40 (imm8 path: 0x40=64 in [-128, 127])
    0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte [rax]
    0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) - disp 0x280 LE
    0xc3,                                     // ret
  ]);
  if (!got.includes(sig)) {
    return { id: 'LDB-OFF64-HANDLER', ok: false, detail: 'load+add imm8=0x40+movzx+store+ret 23B signature missing' };
  }
  // Verify the imm8 path is active (NOT imm32): pin absence of `48 81 c0`.
  const imm32Opcode = Buffer.from([0x48, 0x81, 0xc0]);
  if (got.includes(imm32Opcode)) {
    return { id: 'LDB-OFF64-HANDLER', ok: false, detail: 'encoder emitted imm32 path for oo=0x40 (premature imm32 escalation)' };
  }
  return { id: 'LDB-OFF64-HANDLER', ok: true, detail: `selector=0x31 dd=0x50 ss=0x60 oo=0x40 code=${hexOf(got)} len=${got.length}` };
}

/**
 * LDB-OFF16-HANDLER: body-extend-011 H_44 `0x80 LDB dd ss oo` at selector 0x32
 * with oo=0x10 (= 16 decimal; positive imm8 byte). Mirrors checkLDBOFF64HANDLER
 * template but exercises a fresh imm8 byte (0x10) at a fresh selector (0x32).
 * Both peers compose identical bytes via the same x86-64 primitives:
 *   JS:   loadState(0x60,rax) + addImmRax(0x10) + [0x48,0x0f,0xb6,0x00]
 *         + storeState(0x50,rax) + 0xC3 (23B)
 *   Rust: emit_ldb(0x50, 0x60, 0x10) -> load_state(0x60,rax) +
 *         add_imm(rax, 0x10) (imm8 path) + movzx + store_state(0x50,rax)
 *         + ret() (23B)
 * Pin: 498b87000300004883c010480fb60049898780020000c3 (23B).
 * Differs from H_43 ONLY at the imm8 byte (0x10 vs 0x40). Extends the LDB
 * imm8-path byte-coverage matrix to include 0x10 (=16). The signed-imm8
 * LEFT-edge signed-token semantic (-128 -> imm8 byte 0x80) remains covered
 * by the JS-only checkLDBoffm128 probe (Rust ty_parser lacks signed-hex
 * support; emit.rs treats oo as unsigned u16 per dispatch contract).
 */
function checkLDBOFF16HANDLER() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off16_handler.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off16_handler.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-OFF16-HANDLER', ok: false, detail: 'missing LDB-off16-handler fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-OFF16-HANDLER', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  if (got.length !== 23) {
    return { id: 'LDB-OFF16-HANDLER', ok: false, detail: `expected 23B pin, got ${got.length}B` };
  }
  // Pin signature: load+add imm8=0x10+movzx+store+ret slice - 23B total
  const sig = Buffer.from([
    0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) - disp 0x300 LE
    0x48, 0x83, 0xc0, 0x10,                   // add rax, 0x10 (imm8 path: 0x10=16 in [-128, 127])
    0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte [rax]
    0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) - disp 0x280 LE
    0xc3,                                     // ret
  ]);
  if (!got.includes(sig)) {
    return { id: 'LDB-OFF16-HANDLER', ok: false, detail: 'load+add imm8=0x10+movzx+store+ret 23B signature missing' };
  }
  // Verify the imm8 path is active (NOT imm32): pin absence of `48 81 c0`.
  const imm32Opcode = Buffer.from([0x48, 0x81, 0xc0]);
  if (got.includes(imm32Opcode)) {
    return { id: 'LDB-OFF16-HANDLER', ok: false, detail: 'encoder emitted imm32 path for oo=0x10 (premature imm32 escalation)' };
  }
  return { id: 'LDB-OFF16-HANDLER', ok: true, detail: `selector=0x32 dd=0x50 ss=0x60 oo=0x10 code=${hexOf(got)} len=${got.length}` };
}

/**
 * LDB-OFF32-HANDLER: body-extend-012 H_45 `0x80 LDB dd ss oo` at selector 0x33
 * with oo=0x20 (= 32 decimal; positive imm8 byte). Mirrors checkLDBOFF16HANDLER
 * template but exercises a fresh imm8 byte (0x20) at a fresh selector (0x33).
 * Both peers compose identical bytes via the same x86-64 primitives:
 *   JS:   loadState(0x60,rax) + addImmRax(0x20) + [0x48,0x0f,0xb6,0x00]
 *         + storeState(0x50,rax) + 0xC3 (23B)
 *   Rust: emit_ldb(0x50, 0x60, 0x20) -> load_state(0x60,rax) +
 *         add_imm(rax, 0x20) (imm8 path) + movzx + store_state(0x50,rax)
 *         + ret() (23B)
 * Pin: 498b87000300004883c020480fb60049898780020000c3 (23B).
 * Differs from H_44 ONLY at the imm8 byte (0x20 vs 0x10). Extends the LDB
 * imm8-path byte-coverage matrix to include 0x20 (=32). The signed-imm8
 * LEFT-edge signed-token semantic (-128 -> imm8 byte 0x80) remains covered
 * by the JS-only checkLDBoffm128 probe (Rust ty_parser lacks signed-hex
 * support; emit.rs treats oo as unsigned u16 per dispatch contract).
 */
function checkLDBOFF32HANDLER() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off32_handler.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off32_handler.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-OFF32-HANDLER', ok: false, detail: 'missing LDB-off32-handler fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-OFF32-HANDLER', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  if (got.length !== 23) {
    return { id: 'LDB-OFF32-HANDLER', ok: false, detail: `expected 23B pin, got ${got.length}B` };
  }
  // Pin signature: load+add imm8=0x20+movzx+store+ret slice - 23B total
  const sig = Buffer.from([
    0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00, // load_state(0x60, rax) - disp 0x300 LE
    0x48, 0x83, 0xc0, 0x20,                   // add rax, 0x20 (imm8 path: 0x20=32 in [-128, 127])
    0x48, 0x0f, 0xb6, 0x00,                   // movzx rax, byte [rax]
    0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00, // store_state(0x50, rax) - disp 0x280 LE
    0xc3,                                     // ret
  ]);
  if (!got.includes(sig)) {
    return { id: 'LDB-OFF32-HANDLER', ok: false, detail: 'load+add imm8=0x20+movzx+store+ret 23B signature missing' };
  }
  // Verify the imm8 path is active (NOT imm32): pin absence of `48 81 c0`.
  const imm32Opcode = Buffer.from([0x48, 0x81, 0xc0]);
  if (got.includes(imm32Opcode)) {
    return { id: 'LDB-OFF32-HANDLER', ok: false, detail: 'encoder emitted imm32 path for oo=0x20 (premature imm32 escalation)' };
  }
  return { id: 'LDB-OFF32-HANDLER', ok: true, detail: `selector=0x33 dd=0x50 ss=0x60 oo=0x20 code=${hexOf(got)} len=${got.length}` };
}

/**
 * LDB-OFF96-HANDLER: body-extend-013 H_46 at selector 0x34 with
 * oo=0x60 (= 96 decimal). Positive imm8 byte at a fresh selector; both
 * peers must emit the four-byte `48 83 c0 60` form rather than imm32.
 */
function checkLDBOFF96HANDLER() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off96_handler.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off96_handler.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-OFF96-HANDLER', ok: false, detail: 'missing LDB-off96-handler fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-OFF96-HANDLER', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  if (got.length !== 23) {
    return { id: 'LDB-OFF96-HANDLER', ok: false, detail: `expected 23B pin, got ${got.length}B` };
  }
  const sig = Buffer.from([
    0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
    0x48, 0x83, 0xc0, 0x60,
    0x48, 0x0f, 0xb6, 0x00,
    0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
    0xc3,
  ]);
  if (!got.includes(sig)) {
    return { id: 'LDB-OFF96-HANDLER', ok: false, detail: 'load+add imm8=0x60+movzx+store+ret 23B signature missing' };
  }
  if (got.includes(Buffer.from([0x48, 0x81, 0xc0]))) {
    return { id: 'LDB-OFF96-HANDLER', ok: false, detail: 'encoder emitted imm32 path for oo=0x60' };
  }
  return { id: 'LDB-OFF96-HANDLER', ok: true, detail: `selector=0x34 dd=0x50 ss=0x60 oo=0x60 code=${hexOf(got)} len=${got.length}` };
}

/**
 * LDB-OFF112-HANDLER: body-extend-014 H_47 at selector 0x35 with
 * oo=0x70 (= 112 decimal). Positive imm8 byte at a fresh selector; both
 * peers must emit the four-byte `48 83 c0 70` form rather than imm32.
 * Mirrors checkLDBOFF96HANDLER template (H_46 oo=0x60) but exercises a
 * fresh imm8 byte (0x70 = 112 decimal) at a fresh selector (0x35).
 */
function checkLDBOFF112HANDLER() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off112_handler.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off112_handler.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-OFF112-HANDLER', ok: false, detail: 'missing LDB-off112-handler fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-OFF112-HANDLER', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  if (got.length !== 23) {
    return { id: 'LDB-OFF112-HANDLER', ok: false, detail: `expected 23B pin, got ${got.length}B` };
  }
  const sig = Buffer.from([
    0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00,
    0x48, 0x83, 0xc0, 0x70,
    0x48, 0x0f, 0xb6, 0x00,
    0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00,
    0xc3,
  ]);
  if (!got.includes(sig)) {
    return { id: 'LDB-OFF112-HANDLER', ok: false, detail: 'load+add imm8=0x70+movzx+store+ret 23B signature missing' };
  }
  if (got.includes(Buffer.from([0x48, 0x81, 0xc0]))) {
    return { id: 'LDB-OFF112-HANDLER', ok: false, detail: 'encoder emitted imm32 path for oo=0x70' };
  }
  return { id: 'LDB-OFF112-HANDLER', ok: true, detail: `selector=0x35 dd=0x50 ss=0x60 oo=0x70 code=${hexOf(got)} len=${got.length}` };
}

/**
 * LDB-off127: offset=127 (0x7F) is the largest signed imm8 value [-128, 127].
 * Encoder MUST stay on the imm8 path (48 83 c0 + 1B imm8=0x7f). This is the
 * right edge of the imm8 range; off=128 forces imm32. JS encoder threshold is
 * signed: `if (imm >= -128 && imm <= 127)`.
 */
function checkLDBoff127() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off127.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off127.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-off127', ok: false, detail: 'missing offset=127 fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-off127', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  // Pin the imm8(0x7f) signature: 48 83 c0 7f
  const imm8Edge = Buffer.from([0x48, 0x83, 0xc0, 0x7f]);
  if (!got.includes(imm8Edge)) {
    return {
      id: 'LDB-off127',
      ok: false,
      detail: 'add rax,imm8 0x7f signature 48 83 c0 7f missing — encoder did NOT stay on imm8 path at the right edge',
    };
  }
  // Make sure the (wrong) imm32 shape does NOT appear in the load→movzx window.
  const loadIdx = got.indexOf(Buffer.from([0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00]));
  const movzxIdx = got.indexOf(Buffer.from([0x48, 0x0f, 0xb6, 0x00]));
  if (loadIdx < 0 || movzxIdx < 0) {
    return { id: 'LDB-off127', ok: false, detail: 'load_state(0x60) or movzx window missing' };
  }
  const win = got.subarray(loadIdx, movzxIdx + 4);
  if (win.includes(Buffer.from([0x48, 0x81, 0xc0]))) {
    return {
      id: 'LDB-off127',
      ok: false,
      detail: 'encoder emitted imm32 path for off=127 (premature imm32 escalation)',
    };
  }
  return {
    id: 'LDB-off127',
    ok: true,
    detail: `code=${hexOf(got)} len=${got.length} imm8-right-edge active`,
  };
}

/**
 * LDB-offm128: offset=-128 (signed) is the SMALLEST signed imm8 value [-128, 127].
 * Encoder MUST stay on the imm8 path (48 83 c0 + 1B imm8=0x80). This is the
 * LEFT edge of the imm8 range; off=-129 forces imm32. JS encoder threshold is
 * signed: `if (imm >= -128 && imm <= 127)`.
 *
 * The .ty file uses `-80` token which parseTy() parses as signed hex literal
 * -128 (matches the imm8 LEFT-edge boundary). Same shape as off8/off127
 * (imm8 path); only the imm8 byte differs (`80` vs `08` vs `7f`).
 */
function checkLDBoffm128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_offm128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_offm128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-offm128', ok: false, detail: 'missing offset=-128 fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-offm128', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  // Pin the imm8(0x80) signature: 48 83 c0 80
  const imm8Edge = Buffer.from([0x48, 0x83, 0xc0, 0x80]);
  if (!got.includes(imm8Edge)) {
    return {
      id: 'LDB-offm128',
      ok: false,
      detail: 'add rax,imm8 0x80 signature 48 83 c0 80 missing — encoder did NOT stay on imm8 path at the left edge',
    };
  }
  // Make sure the (wrong) imm32 shape does NOT appear in the load→movzx window.
  const loadIdx = got.indexOf(Buffer.from([0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00]));
  const movzxIdx = got.indexOf(Buffer.from([0x48, 0x0f, 0xb6, 0x00]));
  if (loadIdx < 0 || movzxIdx < 0) {
    return { id: 'LDB-offm128', ok: false, detail: 'load_state(0x60) or movzx window missing' };
  }
  const win = got.subarray(loadIdx, movzxIdx + 4);
  if (win.includes(Buffer.from([0x48, 0x81, 0xc0]))) {
    return {
      id: 'LDB-offm128',
      ok: false,
      detail: 'encoder emitted imm32 path for off=-128 (premature imm32 escalation on negative side)',
    };
  }
  return {
    id: 'LDB-offm128',
    ok: true,
    detail: `code=${hexOf(got)} len=${got.length} imm8-left-edge active`,
  };
}

/**
 * LDB-offm129: offset=-129 (signed) is JUST PAST the signed imm8 range
 * [-128, 127] on the NEGATIVE side. Encoder MUST switch to imm32 path
 * (48 81 c0 + 4B LE imm32=0xFFFFFF7F). The .ty uses `-81` token which
 * parseTy() parses as signed hex literal -129. Symmetric with LDB-off128
 * (positive imm32 LEFT-edge) and LDB-offm128 (imm8 LEFT-edge on negative
 * side). If the encoder silently truncated to imm8 (48 83 c0 7F), it
 * would emit +127 instead of -129 (wrong sign/magnitude) — STOP if so.
 */
function checkLDBoffm129() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_offm129.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_offm129.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-offm129', ok: false, detail: 'missing offset=-129 fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-offm129', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  // Pin the imm32(0xFFFFFF7F) signature: 48 81 c0 7f ff ff ff
  const imm32NegEdge = Buffer.from([0x48, 0x81, 0xc0, 0x7f, 0xff, 0xff, 0xff]);
  if (!got.includes(imm32NegEdge)) {
    return {
      id: 'LDB-offm129',
      ok: false,
      detail: 'add rax,imm32 0xFFFFFF7F signature 48 81 c0 7f ff ff ff missing — encoder did NOT switch to imm32 on the negative side',
    };
  }
  // Make sure the (wrong) imm8 shape `48 83 c0 7f` (+127) does NOT appear in
  // the load→movzx window. If the encoder silently truncated -129 to imm8
  // = 0x7F, it would emit `48 83 c0 7f` (positive +127) — pin its ABSENCE.
  const loadIdx = got.indexOf(Buffer.from([0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00]));
  const movzxIdx = got.indexOf(Buffer.from([0x48, 0x0f, 0xb6, 0x00]));
  if (loadIdx < 0 || movzxIdx < 0) {
    return { id: 'LDB-offm129', ok: false, detail: 'load_state(0x60) or movzx window missing' };
  }
  const win = got.subarray(loadIdx, movzxIdx + 4);
  if (win.includes(Buffer.from([0x48, 0x83, 0xc0, 0x7f]))) {
    return {
      id: 'LDB-offm129',
      ok: false,
      detail: 'encoder emitted imm8=0x7F (+127) instead of imm32=0xFFFFFF7F (-129) — silent sign/magnitude truncation',
    };
  }
  return {
    id: 'LDB-offm129',
    ok: true,
    detail: `code=${hexOf(got)} len=${got.length} imm32-negative-edge active`,
  };
}

/**
 * LDB-off128: offset=128 (0x80) is the FIRST value past the signed imm8 range
 * [-128, 127]. Encoder MUST switch to imm32 path (48 81 c0 + 4B LE imm32=0x80).
 * If the encoder interprets imm8 as unsigned [0, 255], it would silently emit
 * imm8 = 0x80 instead. STOP if asm emits 48 83 c0 80 for off=128.
 */
function checkLDBoff128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-off128', ok: false, detail: 'missing offset=128 fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-off128', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  // Pin the imm32(0x80) signature: 48 81 c0 80 00 00 00
  const imm32Edge = Buffer.from([0x48, 0x81, 0xc0, 0x80, 0x00, 0x00, 0x00]);
  if (!got.includes(imm32Edge)) {
    return {
      id: 'LDB-off128',
      ok: false,
      detail: 'add rax,imm32 0x80 signature 48 81 c0 80 00 00 00 missing — encoder did NOT switch to imm32 at the left edge',
    };
  }
  // Make sure the (wrong) imm8=0x80 byte does NOT appear as a standalone imm8.
  const loadIdx = got.indexOf(Buffer.from([0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00]));
  const movzxIdx = got.indexOf(Buffer.from([0x48, 0x0f, 0xb6, 0x00]));
  if (loadIdx < 0 || movzxIdx < 0) {
    return { id: 'LDB-off128', ok: false, detail: 'load_state(0x60) or movzx window missing' };
  }
  const win = got.subarray(loadIdx, movzxIdx + 4);
  if (win.includes(Buffer.from([0x48, 0x83, 0xc0, 0x80]))) {
    return {
      id: 'LDB-off128',
      ok: false,
      detail: 'encoder emitted imm8=0x80 instead of imm32=0x80 (unsigned-imm8 path bug)',
    };
  }
  return {
    id: 'LDB-off128',
    ok: true,
    detail: `code=${hexOf(got)} len=${got.length} imm32-left-edge active`,
  };
}

/**
 * LDB-off256: offset=256 (0x100) forces the imm32 path in add rax, imm.
 * Encoder MUST emit 48 81 c0 + 4-byte LE imm32 (48 81 c0 00 01 00 00),
 * NOT 48 83 c0 + 1-byte imm8. If it picks imm8 = 0x00, that's wrong
 * (0 != 256). Compile-only; bytes only.
 */
function checkLDBoff256() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_off256.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_off256.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-off256', ok: false, detail: 'missing offset=256 fixture or expected bytes' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-off256', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  // Pin the imm32 signature: 48 81 c0 00 01 00 00 = add rax, 0x100 (LE imm32).
  const imm32Sig = Buffer.from([0x48, 0x81, 0xc0, 0x00, 0x01, 0x00, 0x00]);
  if (!got.includes(imm32Sig)) {
    return {
      id: 'LDB-off256',
      ok: false,
      detail: 'add rax,imm32 0x100 signature 48 81 c0 00 01 00 00 missing — encoder did NOT take imm32 path',
    };
  }
  // Make sure the (wrong) imm8 byte 0x100 does NOT appear as a standalone imm8.
  // 0x100 mod 256 = 0x00; if the encoder silently dropped to imm8 = 0x00 we'd
  // see `48 83 c0 00` instead. Pin its ABSENCE in the load→movzx window.
  const loadIdx = got.indexOf(Buffer.from([0x49, 0x8b, 0x87, 0x00, 0x03, 0x00, 0x00]));
  const movzxIdx = got.indexOf(Buffer.from([0x48, 0x0f, 0xb6, 0x00]));
  if (loadIdx < 0 || movzxIdx < 0) {
    return { id: 'LDB-off256', ok: false, detail: 'load_state(0x60) or movzx window missing' };
  }
  const win = got.subarray(loadIdx, movzxIdx + 4);
  if (win.includes(Buffer.from([0x48, 0x83, 0xc0, 0x00]))) {
    return {
      id: 'LDB-off256',
      ok: false,
      detail: 'encoder emitted imm8=0x00 instead of imm32=0x100 (imm8 path bug)',
    };
  }
  return {
    id: 'LDB-off256',
    ok: true,
    detail: `code=${hexOf(got)} len=${got.length} imm32-path active`,
  };
}

/**
 * CALLBACK: compile the disk selfhost_min_call fixture and pin its backward
 * CALL H_00 fixup (rel32=-23).
 */
function checkCALLBACK() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_call.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_call.code.hex');
  if (!fs.existsSync(tyPath)) {
    return { id: 'CALLBACK', ok: false, detail: `missing fixture ${tyPath}` };
  }
  if (!fs.existsSync(expPath)) {
    return { id: 'CALLBACK', ok: false, detail: `missing expected ${expPath}` };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return {
      id: 'CALLBACK',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  const callIdx = got.indexOf(0xe8);
  if (callIdx < 0) {
    return { id: 'CALLBACK', ok: false, detail: 'CALL opcode 0xE8 missing' };
  }
  const rel32 = got.readInt32LE(callIdx + 1);
  const target = callIdx + 1 + 4 + rel32;
  if (rel32 !== -23 || target !== 0) {
    return {
      id: 'CALLBACK',
      ok: false,
      detail: `CALL rel32=${rel32} target=${target}; want rel32=-23 target=0`,
    };
  }
  return {
    id: 'CALLBACK',
    ok: true,
    detail: `call@${callIdx} rel32=${rel32} target=0x${target.toString(16)} code=${hexOf(got)}`,
  };
}

/** G04: CALL/RET stack pair (Appendix F). */
function checkG04() {
  const tyPath = path.join(GOLDEN_DIR, '04_call_ret.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', '04_call_ret.code.hex');
  if (!fs.existsSync(tyPath)) {
    return { id: 'G04', ok: false, detail: `missing fixture ${tyPath}` };
  }
  if (!fs.existsSync(expPath)) {
    return { id: 'G04', ok: false, detail: `missing expected ${expPath}` };
  }
  const expected = loadExpectedHex(expPath);
  const got = compileCode(parseTy(readUtf8(tyPath)));
  if (!got.equals(expected)) {
    return {
      id: 'G04',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  // CALL signature: E8 rel32 (5 bytes); rel32 at +1.
  const callIdx = got.indexOf(0xe8);
  if (callIdx < 0) {
    return { id: 'G04', ok: false, detail: 'CALL opcode 0xE8 missing' };
  }
  const callRel = got.readInt32LE(callIdx + 1);
  const callTarget = callIdx + 1 + 4 + callRel;
  if (callTarget < 0 || callTarget >= got.length) {
    return {
      id: 'G04',
      ok: false,
      detail: `CALL rel32=${callRel} out of range (target=${callTarget})`,
    };
  }
  // RET signature: 0xC3 — must appear at least twice (CALL site's fallthrough +
  // callee epilogue).
  const retBytes = Buffer.from([0xc3]);
  let retCount = 0;
  let i = 0;
  while ((i = got.indexOf(retBytes, i)) >= 0) { retCount++; i++; }
  if (retCount < 2) {
    return {
      id: 'G04',
      ok: false,
      detail: `expected ≥2 RET (0xC3) bytes; got ${retCount}`,
    };
  }
  // CALL must precede the first RET in the stream (site ret = fallthrough).
  const firstRet = got.indexOf(retBytes);
  if (firstRet <= callIdx) {
    return {
      id: 'G04',
      ok: false,
      detail: `CALL@${callIdx} must precede first RET@${firstRet}`,
    };
  }
  return {
    id: 'G04',
    ok: true,
    detail: `call@${callIdx} rel32=${callRel} target=0x${callTarget.toString(16)} rets=${retCount}`,
  };
}

/**
 * CALLRET (compound): pin the FORWARD CALL+RET slice in the disk 04_call_ret
 * golden (e8 01 00 00 00 c3, 6 contiguous bytes).
 *
 * Mirror of checkCALLBACK (BACKWARD CALL rel32=-23 → H_00 entry at offset 0).
 * Here the disk fixture 04_call_ret.code.hex exposes a FORWARD compound:
 *   bytes [0..5] = E8 01 00 00 00 C3   (CALL +1 → offset 6 = callee body;
 *                                        byte at offset 5 is the call-site
 *                                        fallthrough RET, never executed at
 *                                        runtime but present in the slice)
 *   bytes [6..24]= callee: movabs rax,0xCC + store state[0x50] + RET
 *
 * This case proves the JS encoder produces the same FORWARD CALL+RET compound
 * as the disk golden. (asm probe today emits only the BACKWARD CALL slice
 * selfhost_min_call.code.hex = e8 e9 ff ff ff c3; that slice is already pinned
 * by checkCALLBACK above. Rust consumes disk bytes directly — no JS-side edit
 * to yoyo-rust/.)
 */
function checkCALLRET() {
  const tyPath = path.join(GOLDEN_DIR, '04_call_ret.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', '04_call_ret.code.hex');
  if (!fs.existsSync(tyPath)) {
    return { id: 'CALLRET', ok: false, detail: `missing fixture ${tyPath}` };
  }
  if (!fs.existsSync(expPath)) {
    return { id: 'CALLRET', ok: false, detail: `missing expected ${expPath}` };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return {
      id: 'CALLRET',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  // CALL must be at offset 0 in this fixture (H_00 is the entry label).
  if (got.length === 0 || got[0] !== 0xe8) {
    return { id: 'CALLRET', ok: false, detail: 'CALL opcode 0xE8 must lead the stream' };
  }
  // Forward CALL: rel32 = +1, target = callIdx + 1 + 4 + rel32 = 6 (start of
  // callee body — the byte after the 6-byte CALL+RET compound).
  const callIdx = 0;
  const rel32 = got.readInt32LE(callIdx + 1);
  const target = callIdx + 1 + 4 + rel32;
  if (rel32 !== 1) {
    return {
      id: 'CALLRET',
      ok: false,
      detail: `FORWARD CALL must use rel32=+1 (vs BACKWARD -23 in CALLBACK); got rel32=${rel32}`,
    };
  }
  if (target !== 6) {
    return {
      id: 'CALLRET',
      ok: false,
      detail: `FORWARD CALL target must be 6 (callee body after compound); got target=${target}`,
    };
  }
  // Compound slice (6 bytes): E8 01 00 00 00 C3 — CALL+RET (call site fallthrough
  // RET; runtime jumps over it into callee).
  const slice = got.subarray(callIdx, callIdx + 6);
  const sliceHex = hexOf(slice);
  const wantSlice = 'e801000000c3';
  if (sliceHex !== wantSlice) {
    return {
      id: 'CALLRET',
      ok: false,
      detail: `CALL+RET compound must be ${wantSlice}; got ${sliceHex}`,
    };
  }
  // Stream must terminate with a RET (callee epilogue at offset got.length-1).
  if (got[got.length - 1] !== 0xc3) {
    return { id: 'CALLRET', ok: false, detail: 'stream must terminate with RET (0xC3)' };
  }
  // Distinguish from CALLBACK: this slice is FORWARD (+1), not BACKWARD (-23).
  // If a regression ever swaps them, the sliceHex pin catches it.
  const backwardSlice = 'e8e9ffffffc3';
  if (sliceHex === backwardSlice) {
    return {
      id: 'CALLRET',
      ok: false,
      detail: `CALL+RET compound matches BACKWARD slice ${backwardSlice}; want FORWARD ${wantSlice}`,
    };
  }
  return {
    id: 'CALLRET',
    ok: true,
    detail: `call@${callIdx} rel32=+1 compound=${sliceHex} target=${target} calleeRet@${got.length - 1}`,
  };
}

/**
 * G05: Named slots ≡ hex rewrite (Appendix F.5 / Part 8 smoke).
 * - multi-letter names bind on first occurrence starting at 0x50
 * - pin verifies the rewrite: bound names produce the same bytes as the
 *   same program with all names replaced by their bound hex slots.
 * - STOCK_GUI_PATH is also compiled (smoke) to ensure named-slot loop
 *   programs don't regress in fixture territory.
 */
function checkG05() {
  const tyPath = path.join(GOLDEN_DIR, '05_named_slots.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', '05_named_slots.code.hex');
  if (!fs.existsSync(tyPath)) {
    return { id: 'G05', ok: false, detail: `missing fixture ${tyPath}` };
  }
  if (!fs.existsSync(expPath)) {
    return { id: 'G05', ok: false, detail: `missing expected ${expPath}` };
  }
  const expected = loadExpectedHex(expPath);
  const got = compileCode(parseTy(readUtf8(tyPath)));
  if (!got.equals(expected)) {
    return {
      id: 'G05',
      ok: false,
      detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}`,
    };
  }
  // Smoke: stock_gui.ty (named-slot loop) must compile without throwing.
  const stockPath = path.join(ROOT, 'yoyo', 'projects', 'stock_gui.ty');
  if (!fs.existsSync(stockPath)) {
    return {
      id: 'G05',
      ok: false,
      detail: `missing stock_gui.ty smoke (${stockPath})`,
    };
  }
  let stockCode;
  try {
    stockCode = compileCode(parseTy(readUtf8(stockPath)));
  } catch (e) {
    return {
      id: 'G05',
      ok: false,
      detail: `stock_gui.ty compile failed: ${e.message}`,
    };
  }
  if (stockCode.length === 0 || stockCode[stockCode.length - 1] !== 0xc3) {
    return {
      id: 'G05',
      ok: false,
      detail: 'stock_gui.ty must end with RET (0xC3)',
    };
  }
  // Pin must contain expected slot displacement bytes for the named bind range
  // (0x280 = 0x50*8, 0x288 = 0x51*8). Hex rewrite proves the bind path.
  for (const disp of [0x80, 0x02, 0x00, 0x00, 0x88, 0x02, 0x00, 0x00]) {
    if (!got.includes(Buffer.from([disp]))) {
      return {
        id: 'G05',
        ok: false,
        detail: `expected slot disp byte 0x${disp.toString(16)} missing`,
      };
    }
  }
  return {
    id: 'G05',
    ok: true,
    detail: `code=${hexOf(got)} stock_gui=${stockCode.length}B`,
  };
}

function checkNOP() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_nop.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_nop.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'NOP', ok: false, detail: 'missing NOP fixture or expected pin' };
  }
  const got = Buffer.from([...encodeOp(0x00, []), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || hexOf(got) !== '90c3') {
    return { id: 'NOP', ok: false, detail: `mismatch: got ${hexOf(got)} want 90c3` };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'NOP', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'NOP', ok: true, detail: `zero-arg code=${hexOf(got)}` };
}

/**
 * RAW-BYTES: body-extend-005 H_33 `0xA1 RAW_BYTES`, 1-arg variadic literal.
 * Probe pin: A1 CC DD → ccdd c3 (3B). Both peers compose identical bytes via
 * the literal-byte variadic emit path. NOT RAW_BYTE 0xA0 filler — opcode 0xA1
 * routes through a real primitive path.
 */
function checkRAWBYTES() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_raw_bytes.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_raw_bytes.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'RAW-BYTES', ok: false, detail: 'missing RAW_BYTES fixture or expected pin' };
  }
  const got = Buffer.from([...encodeOp(0xA1, [0xCC, 0xDD]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'RAW-BYTES', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'RAW-BYTES', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'RAW-BYTES', ok: true, detail: `variadic-literal code=${hexOf(got)}` };
}

/**
 * IMUL: body-extend-005 H_34 `0x63 IMUL`, 2-arg ALU.
 * Pin: 498b8780020000498b8f88020000480fafc149898780020000c3 (26B).
 * Mirrors ORV template but exercises the IMUL primitive (load_state(0x50,rax)
 * + load_state(0x51,rcx) + imul rax,rcx + store_state(0x50,rax) + ret).
 */
function checkIMUL() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_imul.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_imul.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'IMUL', ok: false, detail: 'missing IMUL fixture or expected pin' };
  }
  const dst = 0x50;
  const src = 0x51;
  const got = Buffer.from([...encodeOp(0x63, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'IMUL', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const imulSig = Buffer.from([0x48, 0x0f, 0xaf, 0xc1]);
  if (!got.includes(imulSig)) {
    return { id: 'IMUL', ok: false, detail: 'imul rax,rcx signature 48 0f af c1 missing' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'IMUL', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'IMUL', ok: true, detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}` };
}

/**
 * SUBV: body-extend-005 H_35 `0x6A SUBV`, 2-arg ALU.
 * Pin: 498b8780020000498b8f880200004829c849898780020000c3 (25B).
 * load_state(0x50,rax) + load_state(0x51,rcx) + sub rax,rcx +
 * store_state(0x50,rax) + ret. Differs from IMUL only at byte 16: 48 29 c8
 * (sub rax,rcx) vs 48 0f af c1 (imul rax,rcx).
 */
function checkSUBV() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subv.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subv.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'SUBV', ok: false, detail: 'missing SUBV fixture or expected pin' };
  }
  const dst = 0x50;
  const src = 0x51;
  const got = Buffer.from([...encodeOp(0x6A, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'SUBV', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const subSig = Buffer.from([0x48, 0x29, 0xc8]);
  if (!got.includes(subSig)) {
    return { id: 'SUBV', ok: false, detail: 'sub rax,rcx signature 48 29 c8 missing' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'SUBV', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'SUBV', ok: true, detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}` };
}

/**
 * CMP: body-extend-005 H_36 `0x65 CMP`, 2-arg compare, no store.
 * Pin: 498b8780020000498b8f880200004839c8c3 (18B).
 * load_state(0x50,rax) + load_state(0x51,rcx) + cmp rax,rcx + ret.
 */
function checkCMP() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_cmp.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_cmp.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'CMP', ok: false, detail: 'missing CMP fixture or expected pin' };
  }
  const dst = 0x50;
  const src = 0x51;
  const got = Buffer.from([...encodeOp(0x65, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'CMP', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const cmpSig = Buffer.from([0x48, 0x39, 0xc8]);
  if (!got.includes(cmpSig)) {
    return { id: 'CMP', ok: false, detail: 'cmp rax,rcx signature 48 39 c8 missing' };
  }
  // CMP must NOT store (compare, no store)
  const store50 = Buffer.from([0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (got.includes(store50)) {
    return { id: 'CMP', ok: false, detail: 'CMP must NOT store_state (compare-only, no store)' };
  }
  return { id: 'CMP', ok: true, detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}` };
}

/**
 * LDB-BODY: body-extend-005 H_37 `0x80 LDB`, 3-arg load-byte (dd=0x50 dst,
 * ss=0x60 src, oo=0 offset). Pin: 498b8700030000480fb60049898780020000c3
 * (19B). load_state(0x60,rax) + movzx rax,byte[rax] + store_state(0x50,rax)
 * + ret. Companion to existing checkLDB (which exercises multi-handler
 * fixture territory); this is the canonical-body 3-arg form at oo=0.
 */
function checkLDBBODY() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_body.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_body.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-BODY', ok: false, detail: 'missing LDB-body fixture or expected pin' };
  }
  const got = compileCode(parseTy(readUtf8(tyPath)));
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-BODY', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const movzxSig = Buffer.from([0x48, 0x0f, 0xb6, 0x00]);
  if (!got.includes(movzxSig)) {
    return { id: 'LDB-BODY', ok: false, detail: 'movzx rax,byte[rax] signature 48 0f b6 00 missing' };
  }
  return { id: 'LDB-BODY', ok: true, detail: `dd=0x50 ss=0x60 oo=0 code=${hexOf(got)} len=${got.length}` };
}

/**
 * SET-CONTROL: body-extend-005 H_38 `0x30 SET`, CONTROL: already
 * opcode-covered in yoyo.ty at H_00..H_04. Identical 18B pin to H_00;
 * no regression: existing primitive path persists at the new selector.
 */
function checkSETCONTROL() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_control.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_control.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'SET-CONTROL', ok: false, detail: 'missing SET-CONTROL fixture or expected pin' };
  }
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0x00]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'SET-CONTROL', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'SET-CONTROL', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'SET-CONTROL', ok: true, detail: `control-no-regression code=${hexOf(got)}` };
}

/**
 * GET: body-extend-006 H_39 `0x60 GET`, 2-arg state-slot copy. Mirrors
 * MOVRR template but exercises the GET opcode directly (0x60, not 0x64).
 * Both peers compose identical bytes via separate emit paths:
 *   JS:   loadState(src) + storeState(dst) + 0xC3  (encodeOp 0x60 branch)
 *   Rust: emit_get(dst, src) + ret()
 *   0x60 = GET dst, src (PROMPT Part 4.1 / isa_table.txt).
 * D-2 Phase 2: MOVRR (0x64) routes through emit_movrr; GET (0x60) through emit_get.
 * Pin: 498b878802000049898780020000c3 (15B).
 */
function checkGET() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_get.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_get.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'GET', ok: false, detail: 'missing GET fixture or expected pin' };
  }
  const dst = 0x50;
  const src = 0x51;
  const got = Buffer.from([...encodeOp(0x60, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'GET', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const load51 = Buffer.from([0x49, 0x8b, 0x87, 0x88, 0x02, 0x00, 0x00]);
  const store50 = Buffer.from([0x49, 0x89, 0x87, 0x80, 0x02, 0x00, 0x00]);
  if (!got.includes(load51) || !got.includes(store50) || got[got.length - 1] !== 0xc3) {
    return { id: 'GET', ok: false, detail: 'GET must be load(src,rax)+store(dst,rax)+RET shape' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'GET', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'GET', ok: true, detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}` };
}

/**
 * ADDV-SWAP: body-extend-015 / parallel-batch-09 H_48 `0x68 ADDV` dst=0x51 src=0x50.
 * Pin 25B: 498b8788020000498b8f800200004801c849898788020000c3.
 */
function checkADDVSWAP() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addv_swap.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addv_swap.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'ADDV-SWAP', ok: false, detail: 'missing ADDV-swap fixture or expected pin' };
  }
  const dst = 0x51;
  const src = 0x50;
  const got = Buffer.from([...encodeOp(0x68, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'ADDV-SWAP', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const addSig = Buffer.from([0x48, 0x01, 0xc8]);
  if (!got.includes(addSig)) {
    return { id: 'ADDV-SWAP', ok: false, detail: 'add rax,rcx signature 48 01 c8 missing' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'ADDV-SWAP', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'ADDV-SWAP', ok: true, detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}` };
}

/**
 * ORV-SWAP: body-extend-015 / parallel-batch-09 H_49 `0x69 ORV` dst=0x51 src=0x50.
 * Pin 25B: 498b8788020000498b8f800200004809c849898788020000c3.
 */
function checkORVSWAP() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_orv_swap.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_orv_swap.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'ORV-SWAP', ok: false, detail: 'missing ORV-swap fixture or expected pin' };
  }
  const dst = 0x51;
  const src = 0x50;
  const got = Buffer.from([...encodeOp(0x69, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'ORV-SWAP', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const orSig = Buffer.from([0x48, 0x09, 0xc8]);
  if (!got.includes(orSig)) {
    return { id: 'ORV-SWAP', ok: false, detail: 'or rax,rcx signature 48 09 c8 missing' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'ORV-SWAP', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'ORV-SWAP', ok: true, detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}` };
}

/**
 * SUBV-SWAP: body-extend-015 / parallel-batch-09 H_50 `0x6A SUBV` dst=0x51 src=0x50.
 * Pin 25B: 498b8788020000498b8f800200004829c849898788020000c3.
 */
function checkSUBVSWAP() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subv_swap.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subv_swap.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'SUBV-SWAP', ok: false, detail: 'missing SUBV-swap fixture or expected pin' };
  }
  const dst = 0x51;
  const src = 0x50;
  const got = Buffer.from([...encodeOp(0x6A, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'SUBV-SWAP', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const subSig = Buffer.from([0x48, 0x29, 0xc8]);
  if (!got.includes(subSig)) {
    return { id: 'SUBV-SWAP', ok: false, detail: 'sub rax,rcx signature 48 29 c8 missing' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'SUBV-SWAP', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'SUBV-SWAP', ok: true, detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}` };
}

/**
 * GET-ALT: body-extend-015 / parallel-batch-09 H_51 `0x60 GET` dst=0x51 src=0x52.
 * Pin 15B: 498b879002000049898788020000c3.
 */
function checkGETALT() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_get_alt.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_get_alt.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'GET-ALT', ok: false, detail: 'missing GET-alt fixture or expected pin' };
  }
  const dst = 0x51;
  const src = 0x52;
  const got = Buffer.from([...encodeOp(0x60, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'GET-ALT', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'GET-ALT', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'GET-ALT', ok: true, detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}` };
}

/**
 * ADDV-H52: body-extend-015 / parallel-batch-09 H_52 `0x68 ADDV` dst=0x52 src=0x51.
 * Pin 25B: 498b8790020000498b8f880200004801c849898790020000c3.
 */
function checkADDVH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addv_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addv_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'ADDV-H52', ok: false, detail: 'missing ADDV-h52 fixture or expected pin' };
  }
  const dst = 0x52;
  const src = 0x51;
  const got = Buffer.from([...encodeOp(0x68, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'ADDV-H52', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'ADDV-H52', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'ADDV-H52', ok: true, detail: `dst=0x${dst.toString(16)} src=0x${src.toString(16)} code=${hexOf(got)}` };
}

/**
 * SET-LARGE: body-extend-015 / parallel-batch-09 H_53 `0x30 SET` slot=0x52 imm=0xCAFEBABE.
 * Pin 18B: 48b8bebafeca0000000049898790020000c3.
 */
function checkSETLARGE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_large.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_large.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'SET-LARGE', ok: false, detail: 'missing SET-large fixture or expected pin' };
  }
  const slot = 0x52;
  const imm = 0xCAFEBABE;
  const got = Buffer.from([...encodeOp(0x30, [slot, imm]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'SET-LARGE', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'SET-LARGE', ok: false, detail: `fixture mismatch: got ${hexOf(fixture)} want ${hexOf(expected)}` };
  }
  return { id: 'SET-LARGE', ok: true, detail: `slot=0x${slot.toString(16)} imm=0x${imm.toString(16)} code=${hexOf(got)}` };
}

/** body-extend-016 H_54 ORV 52/51 */
function checkORVH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_orv_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_orv_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ORV-H52', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x69, [0x52, 0x51]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ORV-H52', ok: false, detail: 'mismatch' };
  return { id: 'ORV-H52', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBVH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subv_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subv_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBV-H52', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x6A, [0x52, 0x51]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBV-H52', ok: false, detail: 'mismatch' };
  return { id: 'SUBV-H52', ok: true, detail: `code=${hexOf(got)}` };
}
function checkIMULSWAP() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_imul_swap.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_imul_swap.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'IMUL-SWAP', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x63, [0x51, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'IMUL-SWAP', ok: false, detail: 'mismatch' };
  return { id: 'IMUL-SWAP', ok: true, detail: `code=${hexOf(got)}` };
}
function checkIMULH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_imul_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_imul_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'IMUL-H52', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x63, [0x52, 0x51]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'IMUL-H52', ok: false, detail: 'mismatch' };
  return { id: 'IMUL-H52', ok: true, detail: `code=${hexOf(got)}` };
}
function checkCMPSWAP() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_cmp_swap.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_cmp_swap.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'CMP-SWAP', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x65, [0x51, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'CMP-SWAP', ok: false, detail: 'mismatch' };
  return { id: 'CMP-SWAP', ok: true, detail: `code=${hexOf(got)}` };
}
function checkGETH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_get_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_get_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'GET-H52', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x60, [0x52, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'GET-H52', ok: false, detail: 'mismatch' };
  return { id: 'GET-H52', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSETDEADBEEF() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_deadbeef.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_deadbeef.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-DEADBEEF', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x51, 0xDEADBEEF]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-DEADBEEF', ok: false, detail: 'mismatch' };
  return { id: 'SET-DEADBEEF', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDBDST51() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_dst51.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_dst51.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-DST51', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x08]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-DST51', ok: false, detail: 'mismatch' };
  return { id: 'LDB-DST51', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-017 / parallel-batch-11 H_62 INC slot=0x51 */
function checkINCH51() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_inc_h51.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_inc_h51.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'INC-H51', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x66, [0x51]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'INC-H51', ok: false, detail: 'mismatch' };
  return { id: 'INC-H51', ok: true, detail: `code=${hexOf(got)}` };
}
function checkDECH51() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_dec_h51.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_dec_h51.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'DEC-H51', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x67, [0x51]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'DEC-H51', ok: false, detail: 'mismatch' };
  return { id: 'DEC-H51', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x07]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51', ok: true, detail: `code=${hexOf(got)}` };
}
function checkCMPH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_cmp_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_cmp_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'CMP-H52', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x65, [0x52, 0x51]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'CMP-H52', ok: false, detail: 'mismatch' };
  return { id: 'CMP-H52', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDV5052() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addv_5052.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addv_5052.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDV-5052', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x68, [0x50, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDV-5052', ok: false, detail: 'mismatch' };
  return { id: 'ADDV-5052', ok: true, detail: `code=${hexOf(got)}` };
}
function checkGET5150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_get_5150.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_get_5150.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'GET-5150', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x60, [0x51, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'GET-5150', ok: false, detail: 'mismatch' };
  return { id: 'GET-5150', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET12345678() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_12345678.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_12345678.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-12345678', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0x12345678]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-12345678', ok: false, detail: 'mismatch' };
  return { id: 'SET-12345678', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDBDST52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_dst52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_dst52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-DST52', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x08]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-DST52', ok: false, detail: 'mismatch' };
  return { id: 'LDB-DST52', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-018 / parallel-batch-12 H_70..H_77 */
function checkSUBIMMH51() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x03]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51', ok: true, detail: `code=${hexOf(got)}` };
}
function checkDECH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_dec_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_dec_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'DEC-H52', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x67, [0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'DEC-H52', ok: false, detail: 'mismatch' };
  return { id: 'DEC-H52', ok: true, detail: `code=${hexOf(got)}` };
}
function checkINCH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_inc_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_inc_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'INC-H52', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x66, [0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'INC-H52', ok: false, detail: 'mismatch' };
  return { id: 'INC-H52', ok: true, detail: `code=${hexOf(got)}` };
}
function checkORV5052() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_orv_5052.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_orv_5052.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ORV-5052', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x69, [0x50, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ORV-5052', ok: false, detail: 'mismatch' };
  return { id: 'ORV-5052', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBV5052() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subv_5052.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subv_5052.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBV-5052', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x6A, [0x50, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBV-5052', ok: false, detail: 'mismatch' };
  return { id: 'SUBV-5052', ok: true, detail: `code=${hexOf(got)}` };
}
function checkGET5251() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_get_5251.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_get_5251.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'GET-5251', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x60, [0x52, 0x51]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'GET-5251', ok: false, detail: 'mismatch' };
  return { id: 'GET-5251', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSETF00DBABE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_f00dbabe.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_f00dbabe.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-F00DBABE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0xF00DBABE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-F00DBABE', ok: false, detail: 'mismatch' };
  return { id: 'SET-F00DBABE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkCMP5250() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_cmp_5250.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_cmp_5250.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'CMP-5250', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x65, [0x52, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'CMP-5250', ok: false, detail: 'mismatch' };
  return { id: 'CMP-5250', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-019 / parallel-batch-13 H_78..H_85 */
function checkADDIMMH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x07]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5203() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_03.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_03.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-03', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x03]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-03', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-03', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH510A() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_0a.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_0a.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-0A', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x0A]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-0A', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-0A', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5005() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_05.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_05.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-05', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x05]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-05', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-05', ok: true, detail: `code=${hexOf(got)}` };
}
function checkORV5250() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_orv_5250.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_orv_5250.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ORV-5250', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x69, [0x52, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ORV-5250', ok: false, detail: 'mismatch' };
  return { id: 'ORV-5250', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBV5250() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subv_5250.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subv_5250.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBV-5250', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x6A, [0x52, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBV-5250', ok: false, detail: 'mismatch' };
  return { id: 'SUBV-5250', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDV5152() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addv_5152.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addv_5152.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDV-5152', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x68, [0x51, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDV-5152', ok: false, detail: 'mismatch' };
  return { id: 'ADDV-5152', ok: true, detail: `code=${hexOf(got)}` };
}
function checkIMUL5052() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_imul_5052.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_imul_5052.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'IMUL-5052', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x63, [0x50, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'IMUL-5052', ok: false, detail: 'mismatch' };
  return { id: 'IMUL-5052', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-020 / parallel-batch-14 H_86..H_93 */
function checkSETFEEDFACE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_feedface.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_feedface.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-FEEDFACE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x52, 0xFEEDFACE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-FEEDFACE', ok: false, detail: 'mismatch' };
  return { id: 'SET-FEEDFACE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSETAABBCCDD() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_aabbccdd.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_aabbccdd.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-AABBCCDD', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x51, 0xAABBCCDD]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-AABBCCDD', ok: false, detail: 'mismatch' };
  return { id: 'SET-AABBCCDD', ok: true, detail: `code=${hexOf(got)}` };
}
function checkGET5052() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_get_5052.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_get_5052.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'GET-5052', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x60, [0x50, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'GET-5052', ok: false, detail: 'mismatch' };
  return { id: 'GET-5052', ok: true, detail: `code=${hexOf(got)}` };
}
function checkCMP5052() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_cmp_5052.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_cmp_5052.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'CMP-5052', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x65, [0x50, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'CMP-5052', ok: false, detail: 'mismatch' };
  return { id: 'CMP-5052', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516010() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_10.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_10.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-10', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x10]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-10', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-10', ok: true, detail: `code=${hexOf(got)}` };
}
function checkIMUL5250() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_imul_5250.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_imul_5250.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'IMUL-5250', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x63, [0x52, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'IMUL-5250', ok: false, detail: 'mismatch' };
  return { id: 'IMUL-5250', ok: true, detail: `code=${hexOf(got)}` };
}
function checkORV5152() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_orv_5152.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_orv_5152.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ORV-5152', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x69, [0x51, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ORV-5152', ok: false, detail: 'mismatch' };
  return { id: 'ORV-5152', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH500F() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_0f.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_0f.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-0F', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x0F]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-0F', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-0F', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-021 / parallel-batch-15 H_94..H_101 */
function checkSETBEEFCAFE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_beefcafe.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_beefcafe.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-BEEFCAFE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0xBEEFCAFE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-BEEFCAFE', ok: false, detail: 'mismatch' };
  return { id: 'SET-BEEFCAFE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET11111111() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_11111111.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_11111111.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-11111111', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x52, 0x11111111]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-11111111', ok: false, detail: 'mismatch' };
  return { id: 'SET-11111111', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5008() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_08.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_08.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-08', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x08]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-08', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-08', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH520A() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_0a.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_0a.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-0A', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x0A]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-0A', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-0A', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526010() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_10.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_10.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-10', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x10]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-10', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-10', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506018() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_18.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_18.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-18', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x18]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-18', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-18', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBV5152() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subv_5152.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subv_5152.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBV-5152', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x6A, [0x51, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBV-5152', ok: false, detail: 'mismatch' };
  return { id: 'SUBV-5152', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDV5250() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addv_5250.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addv_5250.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDV-5250', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x68, [0x52, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDV-5250', ok: false, detail: 'mismatch' };
  return { id: 'ADDV-5250', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-022 / parallel-batch-16 H_102..H_109 */
function checkCMP5152() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_cmp_5152.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_cmp_5152.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'CMP-5152', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x65, [0x51, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'CMP-5152', ok: false, detail: 'mismatch' };
  return { id: 'CMP-5152', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516018() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_18.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_18.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-18', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x18]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-18', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-18', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526018() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_18.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_18.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-18', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x18]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-18', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-18', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSETC0FFEE00() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_c0ffee00.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_c0ffee00.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-C0FFEE00', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x51, 0xC0FFEE00]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-C0FFEE00', ok: false, detail: 'mismatch' };
  return { id: 'SET-C0FFEE00', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5208() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_08.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_08.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-08', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x08]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-08', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-08', ok: true, detail: `code=${hexOf(got)}` };
}
function checkIMUL5152() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_imul_5152.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_imul_5152.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'IMUL-5152', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x63, [0x51, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'IMUL-5152', ok: false, detail: 'mismatch' };
  return { id: 'IMUL-5152', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5014() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_14.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_14.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-14', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x14]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-14', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-14', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET50C0FFEE00() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_50_c0ffee00.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_50_c0ffee00.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-50-C0FFEE00', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0xC0FFEE00]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-50-C0FFEE00', ok: false, detail: 'mismatch' };
  return { id: 'SET-50-C0FFEE00', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-023 / parallel-batch-17 H_110..H_117 */
function checkSET52DEADF00D() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_52_deadf00d.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_52_deadf00d.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-52-DEADF00D', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x52, 0xDEADF00D]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-52-DEADF00D', ok: false, detail: 'mismatch' };
  return { id: 'SET-52-DEADF00D', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5114() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_14.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_14.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-14', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x14]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-14', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-14', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH510A() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_0a.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_0a.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-0A', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x0A]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-0A', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-0A', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516020() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_20.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_20.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-20', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x20]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-20', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-20', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526020() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_20.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_20.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-20', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x20]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-20', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-20', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5214() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_14.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_14.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-14', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x14]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-14', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-14', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH500A() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_0a.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_0a.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-0A', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x0A]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-0A', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-0A', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET51DEADF00D() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_51_deadf00d.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_51_deadf00d.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-51-DEADF00D', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x51, 0xDEADF00D]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-51-DEADF00D', ok: false, detail: 'mismatch' };
  return { id: 'SET-51-DEADF00D', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-024 / parallel-batch-18 H_118..H_125 */
function checkSET50FACEFEED() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_50_facefeed.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_50_facefeed.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-50-FACEFEED', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0xFACEFEED]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-50-FACEFEED', ok: false, detail: 'mismatch' };
  return { id: 'SET-50-FACEFEED', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511E() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1e.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1e.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1E', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1E]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1E', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1E', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH520A() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_0a.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_0a.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-0A', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x0A]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-0A', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-0A', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506028() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_28.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_28.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-28', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x28]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-28', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-28', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET52FACEFEED() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_52_facefeed.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_52_facefeed.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-52-FACEFEED', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x52, 0xFACEFEED]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-52-FACEFEED', ok: false, detail: 'mismatch' };
  return { id: 'SET-52-FACEFEED', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501E() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1e.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1e.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1E', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1E]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1E', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1E', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5105() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_05.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_05.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-05', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x05]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-05', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-05', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516028() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_28.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_28.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-28', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x28]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-28', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-28', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-025 / parallel-batch-19 H_126..H_133 */
function checkLDB526028() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_28.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_28.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-28', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x28]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-28', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-28', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506030() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_30.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_30.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-30', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x30]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-30', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-30', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET51BAADF00D() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_51_baadf00d.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_51_baadf00d.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-51-BAADF00D', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x51, 0xBAADF00D]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-51-BAADF00D', ok: false, detail: 'mismatch' };
  return { id: 'SET-51-BAADF00D', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521E() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1e.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1e.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1E', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1E]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1E', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1E', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5014() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_14.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_14.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-14', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x14]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-14', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-14', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516030() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_30.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_30.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-30', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x30]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-30', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-30', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET52BAADF00D() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_52_baadf00d.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_52_baadf00d.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-52-BAADF00D', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x52, 0xBAADF00D]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-52-BAADF00D', ok: false, detail: 'mismatch' };
  return { id: 'SET-52-BAADF00D', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5214() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_14.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_14.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-14', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x14]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-14', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-14', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-026 / parallel-batch-20 H_134..H_141 */
function checkLDB526030() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_30.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_30.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-30', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x30]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-30', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-30', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506038() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_38.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_38.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-38', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x38]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-38', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-38', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET500BADF00D() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_50_0badf00d.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_50_0badf00d.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-50-0BADF00D', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0x0BADF00D]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-50-0BADF00D', ok: false, detail: 'mismatch' };
  return { id: 'SET-50-0BADF00D', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_28.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_28.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-28', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x28]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-28', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-28', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511E() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1e.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1e.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1E', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1E]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1E', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1E', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516038() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_38.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_38.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-38', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x38]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-38', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-38', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5028() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_28.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_28.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-28', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x28]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-28', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-28', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521E() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1e.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1e.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1E', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1E]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1E', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1E', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-027 / parallel-batch-21 H_142..H_149 */
function checkLDB526038() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_38.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_38.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-38', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x38]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-38', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-38', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET51FEEDC0DE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_51_feedc0de.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_51_feedc0de.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-51-FEEDC0DE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x51, 0xFEEDC0DE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-51-FEEDC0DE', ok: false, detail: 'mismatch' };
  return { id: 'SET-51-FEEDC0DE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_28.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_28.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-28', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x28]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-28', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-28', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501E() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1e.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1e.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1E', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1E]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1E', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1E', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516040() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_40.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_40.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-40', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x40]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-40', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-40', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526040() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_40.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_40.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-40', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x40]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-40', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-40', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET52FEEDC0DE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_52_feedc0de.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_52_feedc0de.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-52-FEEDC0DE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x52, 0xFEEDC0DE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-52-FEEDC0DE', ok: false, detail: 'mismatch' };
  return { id: 'SET-52-FEEDC0DE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_28.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_28.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-28', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x28]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-28', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-28', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-028 / parallel-batch-22 H_150..H_157 */
function checkSET50FEEDC0DE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_50_feedc0de.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_50_feedc0de.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-50-FEEDC0DE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0xFEEDC0DE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-50-FEEDC0DE', ok: false, detail: 'mismatch' };
  return { id: 'SET-50-FEEDC0DE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5032() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_32.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_32.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-32', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x32]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-32', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-32', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_28.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_28.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-28', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x28]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-28', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-28', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506048() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_48.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_48.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-48', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x48]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-48', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-48', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516048() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_48.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_48.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-48', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x48]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-48', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-48', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526048() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_48.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_48.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-48', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x48]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-48', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-48', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5132() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_32.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_32.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-32', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x32]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-32', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-32', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5028() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_28.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_28.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-28', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x28]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-28', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-28', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-029 / parallel-batch-23 H_158..H_165 */
function checkLDB516050() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_50.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_50.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-50', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-50', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-50', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526050() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_50.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_50.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-50', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-50', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-50', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET51CAFEF00D() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_51_cafef00d.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_51_cafef00d.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-51-CAFEF00D', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x51, 0xCAFEF00D]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-51-CAFEF00D', ok: false, detail: 'mismatch' };
  return { id: 'SET-51-CAFEF00D', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5232() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_32.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_32.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-32', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x32]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-32', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-32', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5132() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_32.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_32.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-32', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x32]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-32', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-32', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET50CAFEF00D() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_50_cafef00d.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_50_cafef00d.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-50-CAFEF00D', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0xCAFEF00D]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-50-CAFEF00D', ok: false, detail: 'mismatch' };
  return { id: 'SET-50-CAFEF00D', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5232() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_32.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_32.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-32', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x32]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-32', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-32', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH503C() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_3c.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_3c.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-3C', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x3C]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-3C', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-3C', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-030 / parallel-batch-24 H_166..H_173 */
function checkSET52CAFEF00D() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_52_cafef00d.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_52_cafef00d.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-52-CAFEF00D', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x52, 0xCAFEF00D]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-52-CAFEF00D', ok: false, detail: 'mismatch' };
  return { id: 'SET-52-CAFEF00D', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506058() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_58.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_58.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-58', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x58]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-58', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-58', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH513C() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_3c.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_3c.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-3C', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x3C]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-3C', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-3C', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH503C() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_3c.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_3c.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-3C', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x3C]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-3C', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-3C', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526058() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_58.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_58.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-58', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x58]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-58', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-58', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516058() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_58.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_58.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-58', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x58]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-58', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-58', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH523C() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_3c.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_3c.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-3C', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x3C]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-3C', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-3C', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH513C() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_3c.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_3c.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-3C', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x3C]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-3C', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-3C', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-031 / parallel-batch-25 H_174..H_181 */
function checkSET50DEADC0DE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_50_deadc0de.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_50_deadc0de.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-50-DEADC0DE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0xDEADC0DE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-50-DEADC0DE', ok: false, detail: 'mismatch' };
  return { id: 'SET-50-DEADC0DE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516060() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_60.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_60.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-60', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x60]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-60', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-60', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526060() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_60.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_60.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-60', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x60]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-60', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-60', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5040() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_40.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_40.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-40', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x40]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-40', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-40', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_40.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_40.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-40', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x40]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-40', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-40', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5240() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_40.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_40.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-40', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x40]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-40', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-40', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH523C() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_3c.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_3c.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-3C', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x3C]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-3C', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-3C', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET51DEADC0DE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_51_deadc0de.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_51_deadc0de.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-51-DEADC0DE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x51, 0xDEADC0DE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-51-DEADC0DE', ok: false, detail: 'mismatch' };
  return { id: 'SET-51-DEADC0DE', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-032 / parallel-batch-26 H_182..H_189 */
function checkSET52DEADC0DE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_52_deadc0de.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_52_deadc0de.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-52-DEADC0DE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x52, 0xDEADC0DE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-52-DEADC0DE', ok: false, detail: 'mismatch' };
  return { id: 'SET-52-DEADC0DE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506068() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_68.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_68.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-68', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x68]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-68', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-68', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516068() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_68.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_68.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-68', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x68]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-68', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-68', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526068() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_68.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_68.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-68', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x68]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-68', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-68', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5048() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_48.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_48.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-48', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x48]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-48', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-48', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_48.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_48.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-48', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x48]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-48', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-48', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5040() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_40.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_40.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-40', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x40]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-40', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-40', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_40.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_40.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-40', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x40]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-40', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-40', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-033 / parallel-batch-27 H_190..H_197 */
function checkADDIMMH5248() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_48.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_48.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-48', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x48]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-48', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-48', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5240() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_40.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_40.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-40', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x40]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-40', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-40', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516070() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_70.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_70.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-70', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x70]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-70', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-70', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526070() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_70.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_70.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-70', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x70]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-70', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-70', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET50C0DEC0DE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_50_c0dec0de.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_50_c0dec0de.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-50-C0DEC0DE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0xC0DEC0DE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-50-C0DEC0DE', ok: false, detail: 'mismatch' };
  return { id: 'SET-50-C0DEC0DE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5050() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_50.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_50.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-50', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-50', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-50', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_48.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_48.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-48', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x48]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-48', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-48', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_50.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_50.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-50', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-50', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-50', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-034 / parallel-batch-28 H_198..H_205 */
function checkADDIMMH5250() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_50.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_50.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-50', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-50', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-50', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5048() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_48.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_48.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-48', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x48]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-48', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-48', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5248() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_48.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_48.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-48', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x48]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-48', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-48', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506078() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_78.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_78.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-78', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x78]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-78', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-78', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET51C0DEC0DE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_51_c0dec0de.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_51_c0dec0de.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-51-C0DEC0DE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x51, 0xC0DEC0DE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-51-C0DEC0DE', ok: false, detail: 'mismatch' };
  return { id: 'SET-51-C0DEC0DE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5058() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_58.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_58.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-58', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x58]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-58', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-58', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_50.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_50.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-50', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-50', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-50', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516078() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_78.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_78.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-78', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x78]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-78', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-78', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-035 / parallel-batch-29 H_206..H_213 */
function checkADDIMMH5158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_58.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_58.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-58', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x58]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-58', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-58', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5258() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_58.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_58.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-58', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x58]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-58', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-58', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5050() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_50.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_50.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-50', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-50', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-50', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5250() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_50.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_50.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-50', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-50', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-50', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526078() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_78.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_78.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-78', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x78]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-78', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-78', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSET52C0DEC0DE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_52_c0dec0de.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_52_c0dec0de.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-52-C0DEC0DE', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x52, 0xC0DEC0DE]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SET-52-C0DEC0DE', ok: false, detail: 'mismatch' };
  return { id: 'SET-52-C0DEC0DE', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5060() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_60.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_60.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-60', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x60]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-60', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-60', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506080() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_80.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_80.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-80', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x80]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-80', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-80', ok: true, detail: `code=${hexOf(got)}` };
}

/** body-extend-036 / parallel-batch-30 H_214..H_221 */
function checkADDIMMH5160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_60.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_60.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-60', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x60]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-60', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-60', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5260() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_60.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_60.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-60', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x60]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-60', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-60', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5058() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_58.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_58.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-58', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x58]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-58', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-58', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_58.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_58.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-58', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x58]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-58', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-58', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516080() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_80.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_80.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-80', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x80]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-80', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-80', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526080() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_80.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_80.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-80', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x80]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-80', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-80', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5258() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_58.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_58.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-58', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x58]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-58', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-58', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5068() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_68.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_68.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-68', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x68]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-68', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-68', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_68.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_68.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-68', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x68]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-68', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-68', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5268() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_68.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_68.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-68', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x68]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-68', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-68', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5060() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_60.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_60.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-60', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x60]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-60', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-60', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_60.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_60.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-60', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x60]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-60', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-60', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5260() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_60.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_60.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-60', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x60]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-60', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-60', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506088() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_88.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_88.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-88', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x88]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-88', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-88', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516088() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_88.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_88.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-88', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x88]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-88', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-88', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526088() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_88.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_88.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-88', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x88]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-88', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-88', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5070() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_70.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_70.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-70', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x70]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-70', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-70', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_70.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_70.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-70', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x70]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-70', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-70', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5270() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_70.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_70.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-70', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x70]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-70', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-70', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5068() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_68.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_68.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-68', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x68]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-68', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-68', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_68.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_68.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-68', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x68]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-68', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-68', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5268() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_68.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_68.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-68', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x68]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-68', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-68', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506090() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_90.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_90.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-90', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x90]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-90', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-90', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516090() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_90.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_90.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-90', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x90]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-90', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-90', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526090() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_90.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_90.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-90', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x90]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-90', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-90', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5070() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_70.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_70.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-70', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x70]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-70', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-70', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_70.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_70.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-70', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x70]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-70', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-70', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5270() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_70.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_70.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-70', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x70]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-70', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-70', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5078() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_78.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_78.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-78', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x78]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-78', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-78', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_78.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_78.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-78', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x78]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-78', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-78', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5278() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_78.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_78.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-78', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x78]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-78', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-78', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB506098() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_98.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_98.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-98', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x98]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-98', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-98', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB516098() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_98.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_98.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-98', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x98]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-98', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-98', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB526098() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_98.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_98.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-98', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x98]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-98', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-98', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5078() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_78.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_78.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-78', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x78]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-78', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-78', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_78.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_78.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-78', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x78]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-78', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-78', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5278() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_78.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_78.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-78', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x78]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-78', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-78', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5080() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_80.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_80.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-80', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x80]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-80', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-80', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_80.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_80.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-80', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x80]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-80', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-80', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5280() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_80.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_80.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-80', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x80]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-80', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-80', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_a0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_a0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xA0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-A0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_a0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_a0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xA0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-A0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_a0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_a0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xA0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-A0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5080() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_80.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_80.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-80', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x80]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-80', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-80', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_80.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_80.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-80', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x80]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-80', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-80', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5280() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_80.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_80.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-80', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x80]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-80', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-80', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5088() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_88.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_88.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-88', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x88]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-88', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-88', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_88.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_88.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-88', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x88]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-88', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-88', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5288() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_88.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_88.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-88', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x88]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-88', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-88', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5088() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_88.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_88.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-88', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x88]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-88', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-88', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_88.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_88.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-88', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x88]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-88', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-88', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5288() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_88.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_88.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-88', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x88]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-88', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-88', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_a8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_a8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xA8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-A8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_a8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_a8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xA8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-A8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_a8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_a8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xA8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-A8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5090() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_90.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_90.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-90', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x90]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-90', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-90', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_90.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_90.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-90', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x90]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-90', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-90', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5290() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_90.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_90.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-90', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x90]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-90', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-90', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5090() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_90.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_90.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-90', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x90]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-90', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-90', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_90.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_90.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-90', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x90]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-90', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-90', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5290() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_90.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_90.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-90', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x90]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-90', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-90', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_b0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_b0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xB0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-B0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_b0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_b0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xB0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-B0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_b0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_b0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xB0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-B0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5098() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_98.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_98.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-98', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x98]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-98', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-98', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_98.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_98.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-98', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x98]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-98', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-98', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH5298() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_98.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_98.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-98', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x98]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-98', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-98', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5098() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_98.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_98.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-98', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x98]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-98', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-98', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_98.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_98.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-98', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x98]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-98', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-98', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH5298() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_98.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_98.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-98', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x98]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-98', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-98', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_b8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_b8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xB8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-B8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_b8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_b8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xB8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-B8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_b8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_b8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xB8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-B8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_a0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_a0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xA0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-A0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_a0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_a0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xA0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-A0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_a0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_a0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xA0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-A0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_a0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_a0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xA0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-A0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_a0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_a0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xA0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-A0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_a0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_a0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xA0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-A0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_c0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_c0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xC0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-C0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_c0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_c0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xC0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-C0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_c0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_c0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xC0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-C0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_a8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_a8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xA8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-A8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_a8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_a8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xA8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-A8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_a8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_a8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xA8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-A8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_a8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_a8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xA8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-A8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_a8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_a8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xA8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-A8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_a8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_a8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xA8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-A8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_c8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_c8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xC8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-C8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_c8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_c8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xC8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-C8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_c8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_c8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xC8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-C8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_b0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_b0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xB0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-B0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_b0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_b0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xB0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-B0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_b0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_b0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xB0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-B0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_b0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_b0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xB0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-B0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_b0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_b0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xB0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-B0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_b0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_b0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xB0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-B0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_b8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_b8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xB8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-B8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_b8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_b8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xB8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-B8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_b8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_b8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xB8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-B8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_b8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_b8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xB8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-B8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_b8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_b8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xB8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-B8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_b8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_b8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xB8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-B8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_d0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_d0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xD0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-D0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_d0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_d0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xD0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-D0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_d0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_d0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xD0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-D0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_c0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_c0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xC0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-C0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_c0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_c0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xC0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-C0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_c0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_c0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xC0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-C0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_c0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_c0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xC0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-C0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_c0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_c0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xC0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-C0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_c0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_c0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xC0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-C0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_d8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_d8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xD8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-D8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_d8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_d8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xD8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-D8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_d8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_d8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xD8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-D8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_c8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_c8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xC8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-C8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_c8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_c8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xC8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-C8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_c8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_c8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xC8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-C8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_c8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_c8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xC8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-C8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_c8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_c8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xC8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-C8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_c8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_c8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xC8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-C8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_d0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_d0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xD0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-D0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_d0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_d0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xD0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-D0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_d0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_d0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xD0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-D0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_d0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_d0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xD0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-D0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_d0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_d0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xD0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-D0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_d0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_d0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xD0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-D0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_e0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_e0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xE0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-E0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_e0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_e0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xE0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-E0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_e0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_e0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xE0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-E0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_d8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_d8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xD8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-D8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_d8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_d8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xD8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-D8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_d8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_d8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xD8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-D8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_d8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_d8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xD8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-D8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_d8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_d8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xD8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-D8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_d8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_d8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xD8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-D8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_e8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_e8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xE8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-E8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_e8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_e8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xE8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-E8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_e8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_e8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xE8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-E8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_e0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_e0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xE0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-E0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_e0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_e0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xE0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-E0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_e0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_e0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xE0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-E0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_e0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_e0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xE0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-E0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_e0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_e0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xE0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-E0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_e0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_e0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xE0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-E0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_e8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_e8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xE8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-E8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_e8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_e8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xE8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-E8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_e8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_e8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xE8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-E8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_e8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_e8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xE8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-E8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_e8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_e8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xE8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-E8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_e8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_e8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xE8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-E8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_f0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_f0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xF0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-F0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_f0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_f0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xF0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-F0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_f0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_f0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xF0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-F0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_f0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_f0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xF0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-F0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_f0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_f0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xF0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-F0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_f0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_f0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xF0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-F0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_f0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_f0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xF0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-F0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_f0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_f0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xF0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-F0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_f0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_f0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xF0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-F0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_f8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_f8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0xF8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-F8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-F8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_f8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_f8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0xF8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-F8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-F8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_f8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_f8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0xF8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-F8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-F8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_f8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_f8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0xF8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-F8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-F8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_f8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_f8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0xF8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-F8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-F8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_f8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_f8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0xF8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-F8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-F8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_f8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_f8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0xF8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-F8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-F8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_f8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_f8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0xF8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-F8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-F8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_f8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_f8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0xF8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-F8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-F8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060100() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_100.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_100.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-100', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x100]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-100', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-100', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160100() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_100.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_100.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-100', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x100]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-100', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-100', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260100() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_100.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_100.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-100', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x100]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-100', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-100', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50100() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_100.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_100.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-100', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x100]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-100', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-100', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51100() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_100.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_100.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-100', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x100]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-100', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-100', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52100() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_100.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_100.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-100', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x100]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-100', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-100', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50100() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_100.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_100.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-100', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x100]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-100', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-100', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51100() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_100.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_100.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-100', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x100]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-100', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-100', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52100() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_100.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_100.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-100', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x100]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-100', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-100', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060108() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_108.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_108.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-108', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x108]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-108', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-108', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160108() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_108.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_108.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-108', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x108]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-108', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-108', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260108() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_108.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_108.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-108', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x108]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-108', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-108', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50108() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_108.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_108.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-108', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x108]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-108', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-108', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51108() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_108.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_108.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-108', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x108]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-108', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-108', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52108() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_108.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_108.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-108', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x108]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-108', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-108', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50108() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_108.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_108.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-108', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x108]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-108', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-108', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51108() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_108.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_108.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-108', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x108]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-108', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-108', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52108() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_108.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_108.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-108', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x108]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-108', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-108', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060110() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_110.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_110.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-110', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x110]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-110', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-110', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160110() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_110.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_110.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-110', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x110]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-110', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-110', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260110() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_110.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_110.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-110', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x110]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-110', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-110', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50110() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_110.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_110.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-110', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x110]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-110', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-110', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51110() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_110.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_110.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-110', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x110]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-110', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-110', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52110() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_110.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_110.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-110', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x110]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-110', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-110', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50110() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_110.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_110.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-110', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x110]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-110', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-110', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51110() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_110.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_110.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-110', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x110]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-110', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-110', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52110() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_110.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_110.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-110', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x110]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-110', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-110', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060118() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_118.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_118.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-118', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x118]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-118', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-118', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160118() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_118.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_118.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-118', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x118]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-118', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-118', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260118() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_118.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_118.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-118', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x118]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-118', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-118', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50118() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_118.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_118.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-118', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x118]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-118', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-118', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51118() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_118.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_118.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-118', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x118]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-118', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-118', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52118() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_118.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_118.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-118', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x118]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-118', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-118', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50118() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_118.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_118.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-118', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x118]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-118', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-118', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51118() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_118.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_118.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-118', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x118]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-118', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-118', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52118() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_118.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_118.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-118', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x118]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-118', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-118', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060120() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_120.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_120.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-120', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x120]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-120', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-120', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160120() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_120.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_120.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-120', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x120]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-120', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-120', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260120() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_120.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_120.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-120', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x120]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-120', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-120', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50120() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_120.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_120.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-120', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x120]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-120', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-120', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51120() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_120.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_120.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-120', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x120]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-120', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-120', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52120() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_120.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_120.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-120', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x120]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-120', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-120', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50120() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_120.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_120.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-120', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x120]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-120', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-120', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51120() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_120.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_120.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-120', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x120]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-120', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-120', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52120() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_120.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_120.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-120', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x120]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-120', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-120', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-128', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x128]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-128', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-128', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-128', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x128]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-128', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-128', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-128', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x128]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-128', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-128', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-128', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x128]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-128', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-128', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-128', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x128]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-128', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-128', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-128', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x128]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-128', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-128', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-128', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x128]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-128', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-128', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-128', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x128]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-128', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-128', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52128() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_128.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_128.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-128', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x128]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-128', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-128', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060130() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_130.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_130.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-130', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x130]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-130', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-130', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160130() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_130.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_130.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-130', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x130]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-130', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-130', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260130() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_130.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_130.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-130', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x130]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-130', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-130', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50130() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_130.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_130.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-130', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x130]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-130', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-130', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51130() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_130.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_130.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-130', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x130]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-130', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-130', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52130() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_130.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_130.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-130', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x130]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-130', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-130', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50130() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_130.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_130.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-130', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x130]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-130', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-130', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51130() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_130.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_130.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-130', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x130]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-130', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-130', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52130() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_130.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_130.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-130', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x130]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-130', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-130', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060138() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_138.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_138.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-138', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x138]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-138', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-138', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160138() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_138.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_138.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-138', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x138]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-138', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-138', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260138() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_138.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_138.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-138', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x138]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-138', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-138', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50138() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_138.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_138.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-138', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x138]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-138', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-138', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51138() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_138.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_138.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-138', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x138]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-138', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-138', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52138() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_138.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_138.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-138', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x138]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-138', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-138', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50138() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_138.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_138.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-138', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x138]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-138', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-138', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51138() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_138.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_138.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-138', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x138]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-138', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-138', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52138() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_138.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_138.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-138', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x138]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-138', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-138', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_140.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_140.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-140', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x140]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-140', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-140', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_140.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_140.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-140', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x140]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-140', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-140', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_140.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_140.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-140', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x140]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-140', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-140', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_140.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_140.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-140', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x140]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-140', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-140', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_140.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_140.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-140', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x140]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-140', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-140', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_140.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_140.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-140', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x140]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-140', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-140', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_140.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_140.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-140', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x140]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-140', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-140', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_140.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_140.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-140', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x140]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-140', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-140', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52140() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_140.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_140.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-140', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x140]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-140', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-140', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_148.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_148.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-148', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x148]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-148', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-148', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_148.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_148.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-148', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x148]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-148', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-148', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_148.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_148.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-148', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x148]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-148', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-148', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_148.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_148.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-148', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x148]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-148', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-148', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_148.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_148.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-148', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x148]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-148', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-148', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_148.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_148.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-148', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x148]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-148', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-148', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_148.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_148.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-148', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x148]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-148', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-148', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_148.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_148.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-148', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x148]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-148', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-148', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52148() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_148.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_148.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-148', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x148]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-148', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-148', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_150.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_150.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-150', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x150]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-150', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-150', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_150.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_150.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-150', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x150]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-150', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-150', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_150.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_150.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-150', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x150]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-150', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-150', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_150.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_150.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-150', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x150]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-150', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-150', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_150.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_150.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-150', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x150]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-150', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-150', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_150.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_150.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-150', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x150]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-150', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-150', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_150.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_150.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-150', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x150]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-150', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-150', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_150.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_150.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-150', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x150]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-150', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-150', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52150() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_150.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_150.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-150', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x150]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-150', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-150', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_158.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_158.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-158', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x158]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-158', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-158', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_158.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_158.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-158', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x158]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-158', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-158', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_158.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_158.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-158', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x158]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-158', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-158', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_158.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_158.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-158', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x158]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-158', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-158', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_158.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_158.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-158', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x158]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-158', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-158', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_158.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_158.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-158', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x158]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-158', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-158', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_158.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_158.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-158', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x158]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-158', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-158', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_158.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_158.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-158', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x158]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-158', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-158', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52158() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_158.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_158.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-158', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x158]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-158', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-158', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_160.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_160.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-160', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x160]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-160', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-160', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_160.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_160.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-160', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x160]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-160', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-160', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_160.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_160.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-160', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x160]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-160', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-160', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_160.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_160.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-160', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x160]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-160', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-160', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_160.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_160.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-160', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x160]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-160', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-160', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_160.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_160.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-160', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x160]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-160', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-160', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_160.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_160.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-160', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x160]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-160', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-160', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_160.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_160.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-160', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x160]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-160', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-160', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52160() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_160.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_160.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-160', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x160]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-160', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-160', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_168.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_168.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-168', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x168]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-168', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-168', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_168.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_168.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-168', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x168]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-168', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-168', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_168.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_168.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-168', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x168]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-168', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-168', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_168.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_168.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-168', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x168]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-168', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-168', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_168.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_168.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-168', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x168]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-168', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-168', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_168.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_168.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-168', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x168]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-168', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-168', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_168.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_168.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-168', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x168]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-168', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-168', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_168.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_168.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-168', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x168]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-168', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-168', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52168() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_168.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_168.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-168', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x168]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-168', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-168', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_170.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_170.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-170', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x170]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-170', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-170', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_170.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_170.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-170', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x170]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-170', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-170', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_170.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_170.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-170', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x170]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-170', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-170', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_170.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_170.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-170', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x170]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-170', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-170', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_170.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_170.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-170', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x170]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-170', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-170', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_170.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_170.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-170', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x170]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-170', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-170', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_170.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_170.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-170', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x170]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-170', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-170', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_170.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_170.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-170', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x170]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-170', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-170', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52170() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_170.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_170.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-170', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x170]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-170', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-170', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_178.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_178.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-178', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x178]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-178', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-178', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_178.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_178.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-178', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x178]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-178', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-178', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_178.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_178.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-178', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x178]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-178', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-178', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_178.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_178.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-178', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x178]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-178', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-178', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_178.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_178.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-178', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x178]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-178', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-178', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_178.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_178.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-178', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x178]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-178', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-178', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_178.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_178.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-178', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x178]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-178', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-178', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_178.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_178.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-178', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x178]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-178', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-178', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52178() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_178.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_178.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-178', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x178]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-178', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-178', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_180.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_180.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-180', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x180]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-180', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-180', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_180.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_180.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-180', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x180]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-180', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-180', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_180.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_180.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-180', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x180]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-180', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-180', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_180.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_180.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-180', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x180]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-180', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-180', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_180.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_180.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-180', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x180]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-180', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-180', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_180.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_180.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-180', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x180]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-180', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-180', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_180.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_180.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-180', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x180]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-180', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-180', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_180.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_180.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-180', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x180]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-180', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-180', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52180() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_180.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_180.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-180', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x180]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-180', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-180', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_188.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_188.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-188', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x188]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-188', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-188', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_188.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_188.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-188', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x188]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-188', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-188', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_188.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_188.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-188', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x188]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-188', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-188', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_188.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_188.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-188', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x188]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-188', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-188', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_188.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_188.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-188', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x188]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-188', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-188', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_188.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_188.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-188', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x188]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-188', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-188', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_188.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_188.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-188', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x188]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-188', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-188', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_188.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_188.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-188', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x188]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-188', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-188', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52188() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_188.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_188.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-188', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x188]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-188', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-188', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_190.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_190.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-190', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x190]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-190', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-190', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_190.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_190.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-190', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x190]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-190', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-190', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_190.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_190.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-190', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x190]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-190', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-190', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_190.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_190.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-190', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x190]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-190', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-190', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_190.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_190.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-190', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x190]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-190', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-190', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_190.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_190.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-190', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x190]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-190', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-190', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_190.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_190.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-190', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x190]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-190', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-190', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_190.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_190.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-190', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x190]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-190', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-190', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52190() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_190.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_190.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-190', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x190]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-190', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-190', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5060198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_198.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_198.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-198', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x198]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-198', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-198', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5160198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_198.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_198.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-198', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x198]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-198', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-198', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB5260198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_198.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_198.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-198', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x198]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-198', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-198', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH50198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_198.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_198.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-198', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x198]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-198', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-198', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH51198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_198.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_198.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-198', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x198]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-198', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-198', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH52198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_198.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_198.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-198', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x198]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-198', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-198', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH50198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_198.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_198.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-198', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x198]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-198', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-198', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH51198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_198.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_198.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-198', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x198]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-198', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-198', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH52198() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_198.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_198.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-198', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x198]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-198', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-198', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1A0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1A0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1A0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1A0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1A0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1A0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1A0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1A0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1A0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1A0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1A0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1A0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1A0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1A0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1A0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1A0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1A0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1A0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1A0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1A0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1A0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1A0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1A0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1A0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1A0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1A0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1A0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1A0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1A0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1A0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1A0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1A0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521A0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1A0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1A0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1A0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1A0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1A0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1A0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1A8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1A8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1A8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1A8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1A8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1A8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1A8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1A8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1A8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1A8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1A8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1A8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1A8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1A8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1A8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1A8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1A8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1A8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1A8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1A8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1A8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1A8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1A8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1A8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1A8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1A8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1A8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1A8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1A8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1A8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1A8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1A8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521A8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1A8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1A8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1A8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1A8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1A8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1A8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1B0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1B0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1B0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1B0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1B0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1B0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1B0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1B0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1B0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1B0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1B0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1B0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1B0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1B0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1B0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1B0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1B0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1B0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1B0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1B0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1B0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1B0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1B0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1B0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1B0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1B0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1B0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1B0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1B0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1B0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1B0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1B0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521B0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1B0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1B0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1B0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1B0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1B0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1B0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1B8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1B8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1B8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1B8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1B8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1B8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1B8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1B8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1B8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1B8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1B8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1B8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1B8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1B8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1B8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1B8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1B8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1B8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1B8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1B8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1B8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1B8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1B8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1B8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1B8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1B8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1B8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1B8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1B8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1B8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1B8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1B8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521B8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1B8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1B8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1B8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1B8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1B8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1B8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1C0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1C0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1C0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1C0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1C0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1C0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1C0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1C0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1C0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1C0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1C0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1C0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1C0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1C0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1C0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1C0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1C0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1C0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1C0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1C0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1C0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1C0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1C0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1C0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1C0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1C0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1C0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1C0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1C0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1C0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1C0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1C0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521C0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1C0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1C0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1C0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1C0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1C0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1C0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1C8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1C8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1C8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1C8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1C8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1C8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1C8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1C8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1C8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1C8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1C8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1C8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1C8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1C8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1C8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1C8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1C8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1C8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1C8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1C8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1C8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1C8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1C8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1C8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1C8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1C8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1C8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1C8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1C8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1C8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1C8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1C8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521C8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1C8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1C8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1C8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1C8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1C8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1C8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1D0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1D0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1D0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1D0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1D0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1D0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1D0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1D0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1D0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1D0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1D0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1D0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1D0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1D0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1D0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1D0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1D0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1D0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1D0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1D0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1D0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1D0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1D0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1D0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1D0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1D0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1D0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1D0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1D0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1D0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1D0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1D0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521D0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1D0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1D0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1D0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1D0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1D0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1D0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1D8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1D8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1D8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1D8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1D8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1D8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1D8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1D8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1D8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1D8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1D8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1D8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1D8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1D8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1D8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1D8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1D8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1D8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1D8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1D8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1D8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1D8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1D8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1D8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1D8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1D8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1D8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1D8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1D8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1D8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1D8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1D8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521D8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1D8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1D8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1D8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1D8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1D8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1D8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1E0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1E0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1E0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1E0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1E0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1E0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1E0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1E0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1E0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1E0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1E0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1E0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1E0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1E0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1E0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1E0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1E0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1E0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1E0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1E0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1E0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1E0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1E0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1E0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1E0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1E0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1E0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1E0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1E0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1E0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1E0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1E0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521E0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1E0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1E0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1E0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1E0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1E0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1E0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1E8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1E8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1E8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1E8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1E8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1E8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1E8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1E8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1E8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1E8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1E8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1E8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1E8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1E8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1E8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1E8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1E8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1E8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1E8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1E8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1E8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1E8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1E8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1E8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1E8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1E8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1E8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1E8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1E8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1E8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1E8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1E8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH521E8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1E8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1E8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1E8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1E8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1E8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1E8', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB50601F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1F0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1F0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1F0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1F0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB51601F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1F0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1F0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1F0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1F0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkLDB52601F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1F0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1F0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1F0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1F0', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH501F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1F0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1F0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1F0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1F0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH511F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1F0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1F0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1F0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1F0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkADDIMMH521F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1F0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1F0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1F0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1F0', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH501F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1F0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1F0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1F0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1F0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1F0', ok: true, detail: `code=${hexOf(got)}` };
}
function checkSUBIMMH511F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1F0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1F0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1F0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1F0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1F0', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH521F0() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1F0.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1F0.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1F0', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1F0]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1F0', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1F0', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB50601F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_1F8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_1F8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-1F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x1F8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-1F8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-1F8', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB51601F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_1F8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_1F8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-1F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x1F8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-1F8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-1F8', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB52601F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_1F8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_1F8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-1F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x1F8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-1F8', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-1F8', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH501F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_1F8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_1F8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-1F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x1F8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-1F8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-1F8', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH511F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_1F8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_1F8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-1F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x1F8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-1F8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-1F8', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH521F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_1F8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_1F8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-1F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x1F8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-1F8', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-1F8', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH501F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_1F8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_1F8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-1F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x1F8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-1F8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-1F8', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH511F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_1F8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_1F8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-1F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x1F8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-1F8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-1F8', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH521F8() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_1F8.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_1F8.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-1F8', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x1F8]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-1F8', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-1F8', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5060200() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_200.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_200.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-200', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x200]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-200', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-200', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5160200() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_200.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_200.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-200', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x200]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-200', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-200', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5260200() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_200.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_200.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-200', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x200]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-200', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-200', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH50200() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_200.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_200.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-200', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x200]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-200', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-200', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH51200() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_200.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_200.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-200', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x200]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-200', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-200', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH52200() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_200.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_200.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-200', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x200]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-200', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-200', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH50200() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_200.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_200.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-200', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x200]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-200', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-200', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH51200() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_200.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_200.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-200', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x200]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-200', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-200', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH52200() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_200.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_200.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-200', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x200]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-200', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-200', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5060208() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_208.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_208.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-208', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x208]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-208', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-208', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5160208() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_208.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_208.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-208', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x208]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-208', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-208', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5260208() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_208.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_208.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-208', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x208]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-208', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-208', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH50208() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_208.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_208.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-208', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x208]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-208', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-208', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH51208() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_208.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_208.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-208', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x208]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-208', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-208', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH52208() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_208.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_208.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-208', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x208]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-208', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-208', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH50208() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_208.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_208.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-208', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x208]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-208', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-208', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH51208() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_208.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_208.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-208', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x208]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-208', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-208', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH52208() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_208.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_208.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-208', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x208]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-208', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-208', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5060210() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_210.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_210.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-210', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x210]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-210', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-210', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5160210() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_210.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_210.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-210', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x210]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-210', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-210', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5260210() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_210.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_210.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-210', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x210]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-210', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-210', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH50210() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_210.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_210.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-210', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x210]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-210', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-210', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH51210() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_210.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_210.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-210', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x210]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-210', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-210', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH52210() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_210.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_210.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-210', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x210]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-210', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-210', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH50210() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_210.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_210.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-210', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x210]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-210', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-210', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH51210() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_210.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_210.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-210', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x210]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-210', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-210', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH52210() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_210.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_210.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-210', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x210]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-210', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-210', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5060218() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_218.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_218.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-218', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x218]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-218', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-218', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5160218() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_218.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_218.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-218', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x218]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-218', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-218', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5260218() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_218.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_218.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-218', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x218]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-218', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-218', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH50218() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_218.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_218.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-218', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x218]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-218', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-218', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH51218() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_218.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_218.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-218', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x218]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-218', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-218', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH52218() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_218.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_218.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-218', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x218]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-218', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-218', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH50218() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_218.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_218.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-218', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x218]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-218', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-218', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH51218() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_218.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_218.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-218', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x218]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-218', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-218', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH52218() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_218.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_218.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-218', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x218]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-218', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-218', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5060220() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_220.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_220.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-220', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x220]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-220', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-220', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5160220() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_220.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_220.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-220', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x220]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-220', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-220', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5260220() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_220.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_220.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-220', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x220]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-220', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-220', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH50220() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_220.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_220.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-220', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x220]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-220', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-220', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH51220() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_220.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_220.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-220', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x220]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-220', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-220', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH52220() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_220.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_220.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-220', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x220]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-220', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-220', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH50220() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_220.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_220.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-220', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x220]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-220', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-220', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH51220() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_220.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_220.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-220', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x220]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-220', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-220', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH52220() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_220.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_220.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-220', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x220]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-220', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-220', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5060228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_228.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_228.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-228', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x228]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-228', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-228', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5160228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_228.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_228.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-228', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x228]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-228', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-228', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5260228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_228.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_228.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-228', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x228]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-228', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-228', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH50228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_228.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_228.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-228', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x228]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-228', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-228', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH51228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_228.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_228.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-228', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x228]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-228', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-228', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH52228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_228.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_228.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-228', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x228]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-228', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-228', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH50228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_228.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_228.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-228', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x228]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-228', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-228', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH51228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_228.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_228.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-228', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x228]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-228', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-228', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH52228() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_228.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_228.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-228', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x228]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-228', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-228', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5060230() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_230.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_230.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-230', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x230]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-230', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-230', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5160230() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_230.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_230.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-230', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x230]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-230', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-230', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5260230() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_230.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_230.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-230', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x230]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-230', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-230', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH50230() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_230.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_230.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-230', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x230]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-230', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-230', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH51230() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_230.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_230.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-230', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x230]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-230', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-230', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH52230() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_230.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_230.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-230', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x230]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-230', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-230', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH50230() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_230.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_230.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-230', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x230]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-230', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-230', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH51230() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h51_230.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h51_230.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H51-230', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x51, 0x230]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H51-230', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H51-230', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH52230() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h52_230.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h52_230.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H52-230', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x52, 0x230]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H52-230', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H52-230', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5060232() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5060_232.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5060_232.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5060-232', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x50, 0x60, 0x232]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5060-232', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5060-232', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5160232() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5160_232.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5160_232.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5160-232', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x51, 0x60, 0x232]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5160-232', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5160-232', ok: true, detail: `code=${hexOf(got)}` };
}

function checkLDB5260232() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_5260_232.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_5260_232.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'LDB-5260-232', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x80, [0x52, 0x60, 0x232]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'LDB-5260-232', ok: false, detail: 'mismatch' };
  return { id: 'LDB-5260-232', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH50232() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h50_232.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h50_232.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H50-232', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x50, 0x232]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H50-232', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H50-232', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH51232() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h51_232.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h51_232.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H51-232', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x51, 0x232]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H51-232', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H51-232', ok: true, detail: `code=${hexOf(got)}` };
}

function checkADDIMMH52232() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_addimm_h52_232.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_addimm_h52_232.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ADDIMM-H52-232', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x62, [0x52, 0x232]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'ADDIMM-H52-232', ok: false, detail: 'mismatch' };
  return { id: 'ADDIMM-H52-232', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSUBIMMH50232() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subimm_h50_232.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subimm_h50_232.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SUBIMM-H50-232', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x61, [0x50, 0x232]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected)) return { id: 'SUBIMM-H50-232', ok: false, detail: 'mismatch' };
  return { id: 'SUBIMM-H50-232', ok: true, detail: `code=${hexOf(got)}` };
}

function checkMEMCPYDATA() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_memcpy_data_stub.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_memcpy_data_stub.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'MEMCPY-DATA-50-51-40', ok: false, detail: 'missing fixture' };
  const expected = loadExpectedHex(expPath);
  const got = Buffer.from([...encodeOp(0x84, [0x50, 0x51, 0x40]), 0xc3]);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected))
    return { id: 'MEMCPY-DATA-50-51-40', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  return { id: 'MEMCPY-DATA-50-51-40', ok: true, detail: `code=${hexOf(got)}` };
}

function checkMEMCPYSTATE() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_memcpy_state_stub.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_memcpy_state_stub.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'MEMCPY-STATE-50-51-40', ok: false, detail: 'missing fixture' };
  const expected = loadExpectedHex(expPath);
  const got = Buffer.from([...encodeOp(0x85, [0x50, 0x51, 0x40]), 0xc3]);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected))
    return { id: 'MEMCPY-STATE-50-51-40', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  return { id: 'MEMCPY-STATE-50-51-40', ok: true, detail: `code=${hexOf(got)}` };
}

function checkGET6050() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_get_60_50.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_get_60_50.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'GET-60-50', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x60, [0x60, 0x50]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected))
    return { id: 'GET-60-50', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  return { id: 'GET-60-50', ok: true, detail: `code=${hexOf(got)}` };
}

function checkGET5060() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_get_50_60.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_get_50_60.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'GET-50-60', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x60, [0x50, 0x60]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected))
    return { id: 'GET-50-60', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  return { id: 'GET-50-60', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSET50FFF() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_50_fff.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_50_fff.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-50-0xfff', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x50, 0xfff]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected))
    return { id: 'SET-50-0xfff', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  return { id: 'SET-50-0xfff', ok: true, detail: `code=${hexOf(got)}` };
}

function checkSET5110000() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_51_10000.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_51_10000.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'SET-51-0x10000', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x30, [0x51, 0x10000]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected))
    return { id: 'SET-51-0x10000', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  return { id: 'SET-51-0x10000', ok: true, detail: `code=${hexOf(got)}` };
}

function checkORV5062() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_orv_50_62.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_orv_50_62.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'ORV-50-62', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x69, [0x50, 0x62]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected))
    return { id: 'ORV-50-62', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  return { id: 'ORV-50-62', ok: true, detail: `code=${hexOf(got)}` };
}

function checkCMP6052() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_cmp_60_52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_cmp_60_52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) return { id: 'CMP-60-52', ok: false, detail: 'missing fixture' };
  const got = Buffer.from([...encodeOp(0x65, [0x60, 0x52]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected) || !compileCode(parseTy(readUtf8(tyPath))).equals(expected))
    return { id: 'CMP-60-52', ok: false, detail: `mismatch: got ${hexOf(got)} want ${hexOf(expected)}` };
  return { id: 'CMP-60-52', ok: true, detail: `code=${hexOf(got)}` };
}

function main() {
  const cases = [checkG00(), checkG01(), checkG02(), checkG03(), checkG04(), checkG05(), checkINC(), checkDEC(), checkADDIMM(), checkSUBIMM(), checkMOVRR(), checkORV(), checkNOP(), checkRAWBYTES(), checkIMUL(), checkSUBV(), checkCMP(), checkLDBBODY(), checkSETCONTROL(), checkGET(), checkADDVSWAP(), checkORVSWAP(), checkSUBVSWAP(), checkGETALT(), checkADDVH52(), checkSETLARGE(), checkORVH52(), checkSUBVH52(), checkIMULSWAP(), checkIMULH52(), checkCMPSWAP(), checkGETH52(), checkSETDEADBEEF(), checkLDBDST51(), checkINCH51(), checkDECH51(), checkADDIMMH51(), checkCMPH52(), checkADDV5052(), checkGET5150(), checkSET12345678(), checkLDBDST52(), checkSUBIMMH51(), checkDECH52(), checkINCH52(), checkORV5052(), checkSUBV5052(), checkGET5251(), checkSETF00DBABE(), checkCMP5250(), checkADDIMMH52(), checkSUBIMMH5203(), checkADDIMMH510A(), checkSUBIMMH5005(), checkORV5250(), checkSUBV5250(), checkADDV5152(), checkIMUL5052(), checkSETFEEDFACE(), checkSETAABBCCDD(), checkGET5052(), checkCMP5052(), checkLDB516010(), checkIMUL5250(), checkORV5152(), checkADDIMMH500F(), checkSETBEEFCAFE(), checkSET11111111(), checkSUBIMMH5008(), checkADDIMMH520A(), checkLDB526010(), checkLDB506018(), checkSUBV5152(), checkADDV5250(), checkCMP5152(), checkLDB516018(), checkLDB526018(), checkSETC0FFEE00(), checkSUBIMMH5208(), checkIMUL5152(), checkADDIMMH5014(), checkSET50C0FFEE00(), checkSET52DEADF00D(), checkADDIMMH5114(), checkSUBIMMH510A(), checkLDB516020(), checkLDB526020(), checkADDIMMH5214(), checkSUBIMMH500A(), checkSET51DEADF00D(), checkSET50FACEFEED(), checkADDIMMH511E(), checkSUBIMMH520A(), checkLDB506028(), checkSET52FACEFEED(), checkADDIMMH501E(), checkSUBIMMH5105(), checkLDB516028(), checkLDB526028(), checkLDB506030(), checkSET51BAADF00D(), checkADDIMMH521E(), checkSUBIMMH5014(), checkLDB516030(), checkSET52BAADF00D(), checkSUBIMMH5214(), checkLDB526030(), checkLDB506038(), checkSET500BADF00D(), checkADDIMMH5128(), checkSUBIMMH511E(), checkLDB516038(), checkADDIMMH5028(), checkSUBIMMH521E(), checkLDB526038(), checkSET51FEEDC0DE(), checkADDIMMH5228(), checkSUBIMMH501E(), checkLDB516040(), checkLDB526040(), checkSET52FEEDC0DE(), checkSUBIMMH5128(), checkSET50FEEDC0DE(), checkADDIMMH5032(), checkSUBIMMH5228(), checkLDB506048(), checkLDB516048(), checkLDB526048(), checkADDIMMH5132(), checkSUBIMMH5028(), checkLDB516050(), checkLDB526050(), checkSET51CAFEF00D(), checkADDIMMH5232(), checkSUBIMMH5132(), checkSET50CAFEF00D(), checkSUBIMMH5232(), checkADDIMMH503C(), checkSET52CAFEF00D(), checkLDB506058(), checkADDIMMH513C(), checkSUBIMMH503C(), checkLDB526058(), checkLDB516058(), checkADDIMMH523C(), checkSUBIMMH513C(), checkSET50DEADC0DE(), checkLDB516060(), checkLDB526060(), checkADDIMMH5040(), checkADDIMMH5140(), checkADDIMMH5240(), checkSUBIMMH523C(), checkSET51DEADC0DE(), checkSET52DEADC0DE(), checkLDB506068(), checkLDB516068(), checkLDB526068(), checkADDIMMH5048(), checkADDIMMH5148(), checkSUBIMMH5040(), checkSUBIMMH5140(), checkADDIMMH5248(), checkSUBIMMH5240(), checkLDB516070(), checkLDB526070(), checkSET50C0DEC0DE(), checkADDIMMH5050(), checkSUBIMMH5148(), checkADDIMMH5150(), checkADDIMMH5250(), checkSUBIMMH5048(), checkSUBIMMH5248(), checkLDB506078(), checkSET51C0DEC0DE(), checkADDIMMH5058(), checkSUBIMMH5150(), checkLDB516078(), checkADDIMMH5158(), checkADDIMMH5258(), checkSUBIMMH5050(), checkSUBIMMH5250(), checkLDB526078(), checkSET52C0DEC0DE(), checkADDIMMH5060(), checkLDB506080(), checkADDIMMH5160(), checkADDIMMH5260(), checkSUBIMMH5058(), checkSUBIMMH5158(), checkLDB516080(), checkLDB526080(), checkSUBIMMH5258(), checkADDIMMH5068(), checkADDIMMH5168(), checkADDIMMH5268(), checkSUBIMMH5060(), checkSUBIMMH5160(), checkSUBIMMH5260(), checkLDB506088(), checkLDB516088(), checkLDB526088(), checkADDIMMH5070(), checkADDIMMH5170(), checkADDIMMH5270(), checkSUBIMMH5068(), checkSUBIMMH5168(), checkSUBIMMH5268(), checkLDB506090(), checkLDB516090(), checkLDB526090(), checkSUBIMMH5070(), checkSUBIMMH5170(), checkSUBIMMH5270(), checkADDIMMH5078(), checkADDIMMH5178(), checkADDIMMH5278(), checkLDB506098(), checkLDB516098(), checkLDB526098(), checkSUBIMMH5078(), checkSUBIMMH5178(), checkSUBIMMH5278(), checkADDIMMH5080(), checkADDIMMH5180(), checkADDIMMH5280(), checkLDB5060A0(), checkLDB5160A0(), checkLDB5260A0(), checkSUBIMMH5080(), checkSUBIMMH5180(), checkSUBIMMH5280(), checkADDIMMH5088(), checkADDIMMH5188(), checkADDIMMH5288(), checkSUBIMMH5088(), checkSUBIMMH5188(), checkSUBIMMH5288(), checkLDB5060A8(), checkLDB5160A8(), checkLDB5260A8(), checkADDIMMH5090(), checkADDIMMH5190(), checkADDIMMH5290(), checkSUBIMMH5090(), checkSUBIMMH5190(), checkSUBIMMH5290(), checkLDB5060B0(), checkLDB5160B0(), checkLDB5260B0(), checkADDIMMH5098(), checkADDIMMH5198(), checkADDIMMH5298(), checkSUBIMMH5098(), checkSUBIMMH5198(), checkSUBIMMH5298(), checkLDB5060B8(), checkLDB5160B8(), checkLDB5260B8(), checkADDIMMH50A0(), checkADDIMMH51A0(), checkADDIMMH52A0(), checkSUBIMMH50A0(), checkSUBIMMH51A0(), checkSUBIMMH52A0(), checkLDB5060C0(), checkLDB5160C0(), checkLDB5260C0(), checkADDIMMH50A8(), checkADDIMMH51A8(), checkADDIMMH52A8(), checkSUBIMMH50A8(), checkSUBIMMH51A8(), checkSUBIMMH52A8(), checkLDB5060C8(), checkLDB5160C8(), checkLDB5260C8(), checkADDIMMH50B0(), checkADDIMMH51B0(), checkADDIMMH52B0(), checkSUBIMMH50B0(), checkSUBIMMH51B0(), checkSUBIMMH52B0(), checkADDIMMH50B8(), checkADDIMMH51B8(), checkADDIMMH52B8(), checkSUBIMMH50B8(), checkSUBIMMH51B8(), checkSUBIMMH52B8(), checkLDB5060D0(), checkLDB5160D0(), checkLDB5260D0(), checkADDIMMH50C0(), checkADDIMMH51C0(), checkADDIMMH52C0(), checkSUBIMMH50C0(), checkSUBIMMH51C0(), checkSUBIMMH52C0(), checkLDB5060D8(), checkLDB5160D8(), checkLDB5260D8(), checkADDIMMH50C8(), checkADDIMMH51C8(), checkADDIMMH52C8(), checkSUBIMMH50C8(), checkSUBIMMH51C8(), checkSUBIMMH52C8(), checkADDIMMH50D0(), checkADDIMMH51D0(), checkADDIMMH52D0(), checkSUBIMMH50D0(), checkSUBIMMH51D0(), checkSUBIMMH52D0(), checkLDB5060E0(), checkLDB5160E0(), checkLDB5260E0(), checkADDIMMH50D8(), checkADDIMMH51D8(), checkADDIMMH52D8(), checkSUBIMMH50D8(), checkSUBIMMH51D8(), checkSUBIMMH52D8(), checkLDB5060E8(), checkLDB5160E8(), checkLDB5260E8(), checkADDIMMH50E0(), checkADDIMMH51E0(), checkADDIMMH52E0(), checkSUBIMMH50E0(), checkSUBIMMH51E0(), checkSUBIMMH52E0(), checkADDIMMH50E8(), checkADDIMMH51E8(), checkADDIMMH52E8(), checkSUBIMMH50E8(), checkSUBIMMH51E8(), checkSUBIMMH52E8(), checkLDB5060F0(), checkLDB5160F0(), checkLDB5260F0(), checkADDIMMH50F0(), checkADDIMMH51F0(), checkADDIMMH52F0(), checkSUBIMMH50F0(), checkSUBIMMH51F0(), checkSUBIMMH52F0(), checkLDB5060F8(), checkLDB5160F8(), checkLDB5260F8(), checkADDIMMH50F8(), checkADDIMMH51F8(), checkADDIMMH52F8(), checkSUBIMMH50F8(), checkSUBIMMH51F8(), checkSUBIMMH52F8(), checkLDB5060100(), checkLDB5160100(), checkLDB5260100(), checkADDIMMH50100(), checkADDIMMH51100(), checkADDIMMH52100(), checkSUBIMMH50100(), checkSUBIMMH51100(), checkSUBIMMH52100(), checkLDB5060108(), checkLDB5160108(), checkLDB5260108(), checkADDIMMH50108(), checkADDIMMH51108(), checkADDIMMH52108(), checkSUBIMMH50108(), checkSUBIMMH51108(), checkSUBIMMH52108(), checkLDB5060110(), checkLDB5160110(), checkLDB5260110(), checkADDIMMH50110(), checkADDIMMH51110(), checkADDIMMH52110(), checkSUBIMMH50110(), checkSUBIMMH51110(), checkSUBIMMH52110(), checkLDB5060118(), checkLDB5160118(), checkLDB5260118(), checkADDIMMH50118(), checkADDIMMH51118(), checkADDIMMH52118(), checkSUBIMMH50118(), checkSUBIMMH51118(), checkSUBIMMH52118(), checkLDB5060120(), checkLDB5160120(), checkLDB5260120(), checkADDIMMH50120(), checkADDIMMH51120(), checkADDIMMH52120(), checkSUBIMMH50120(), checkSUBIMMH51120(), checkSUBIMMH52120(), checkLDB5060128(), checkLDB5160128(), checkLDB5260128(), checkADDIMMH50128(), checkADDIMMH51128(), checkADDIMMH52128(), checkSUBIMMH50128(), checkSUBIMMH51128(), checkSUBIMMH52128(), checkLDB5060130(), checkLDB5160130(), checkLDB5260130(), checkADDIMMH50130(), checkADDIMMH51130(), checkADDIMMH52130(), checkSUBIMMH50130(), checkSUBIMMH51130(), checkSUBIMMH52130(), checkLDB5060138(), checkLDB5160138(), checkLDB5260138(), checkADDIMMH50138(), checkADDIMMH51138(), checkADDIMMH52138(), checkSUBIMMH50138(), checkSUBIMMH51138(), checkSUBIMMH52138(), checkLDB5060140(), checkLDB5160140(), checkLDB5260140(), checkADDIMMH50140(), checkADDIMMH51140(), checkADDIMMH52140(), checkSUBIMMH50140(), checkSUBIMMH51140(), checkSUBIMMH52140(), checkLDB5060148(), checkLDB5160148(), checkLDB5260148(), checkADDIMMH50148(), checkADDIMMH51148(), checkADDIMMH52148(), checkSUBIMMH50148(), checkSUBIMMH51148(), checkSUBIMMH52148(), checkLDB5060150(), checkLDB5160150(), checkLDB5260150(), checkADDIMMH50150(), checkADDIMMH51150(), checkADDIMMH52150(), checkSUBIMMH50150(), checkSUBIMMH51150(), checkSUBIMMH52150(), checkLDB5060158(), checkLDB5160158(), checkLDB5260158(), checkADDIMMH50158(), checkADDIMMH51158(), checkADDIMMH52158(), checkSUBIMMH50158(), checkSUBIMMH51158(), checkSUBIMMH52158(), checkLDB5060160(), checkLDB5160160(), checkLDB5260160(), checkADDIMMH50160(), checkADDIMMH51160(), checkADDIMMH52160(), checkSUBIMMH50160(), checkSUBIMMH51160(), checkSUBIMMH52160(), checkLDB5060168(), checkLDB5160168(), checkLDB5260168(), checkADDIMMH50168(), checkADDIMMH51168(), checkADDIMMH52168(), checkSUBIMMH50168(), checkSUBIMMH51168(), checkSUBIMMH52168(), checkLDB5060170(), checkLDB5160170(), checkLDB5260170(), checkADDIMMH50170(), checkADDIMMH51170(), checkADDIMMH52170(), checkSUBIMMH50170(), checkSUBIMMH51170(), checkSUBIMMH52170(), checkLDB5060178(), checkLDB5160178(), checkLDB5260178(), checkADDIMMH50178(), checkADDIMMH51178(), checkADDIMMH52178(), checkSUBIMMH50178(), checkSUBIMMH51178(), checkSUBIMMH52178(), checkLDB5060180(), checkLDB5160180(), checkLDB5260180(), checkADDIMMH50180(), checkADDIMMH51180(), checkADDIMMH52180(), checkSUBIMMH50180(), checkSUBIMMH51180(), checkSUBIMMH52180(), checkLDB5060188(), checkLDB5160188(), checkLDB5260188(), checkADDIMMH50188(), checkADDIMMH51188(), checkADDIMMH52188(), checkSUBIMMH50188(), checkSUBIMMH51188(), checkSUBIMMH52188(), checkLDB5060190(), checkLDB5160190(), checkLDB5260190(), checkADDIMMH50190(), checkADDIMMH51190(), checkADDIMMH52190(), checkSUBIMMH50190(), checkSUBIMMH51190(), checkSUBIMMH52190(), checkLDB5060198(), checkLDB5160198(), checkLDB5260198(), checkADDIMMH50198(), checkADDIMMH51198(), checkADDIMMH52198(), checkSUBIMMH50198(), checkSUBIMMH51198(), checkSUBIMMH52198(), checkLDB50601A0(), checkLDB51601A0(), checkLDB52601A0(), checkADDIMMH501A0(), checkADDIMMH511A0(), checkADDIMMH521A0(), checkSUBIMMH501A0(), checkSUBIMMH511A0(), checkSUBIMMH521A0(), checkLDB50601A8(), checkLDB51601A8(), checkLDB52601A8(), checkADDIMMH501A8(), checkADDIMMH511A8(), checkADDIMMH521A8(), checkSUBIMMH501A8(), checkSUBIMMH511A8(), checkSUBIMMH521A8(), checkLDB50601B0(), checkLDB51601B0(), checkLDB52601B0(), checkADDIMMH501B0(), checkADDIMMH511B0(), checkADDIMMH521B0(), checkSUBIMMH501B0(), checkSUBIMMH511B0(), checkSUBIMMH521B0(), checkLDB50601B8(), checkLDB51601B8(), checkLDB52601B8(), checkADDIMMH501B8(), checkADDIMMH511B8(), checkADDIMMH521B8(), checkSUBIMMH501B8(), checkSUBIMMH511B8(), checkSUBIMMH521B8(), checkLDB50601C0(), checkLDB51601C0(), checkLDB52601C0(), checkADDIMMH501C0(), checkADDIMMH511C0(), checkADDIMMH521C0(), checkSUBIMMH501C0(), checkSUBIMMH511C0(), checkSUBIMMH521C0(), checkLDB50601C8(), checkLDB51601C8(), checkLDB52601C8(), checkADDIMMH501C8(), checkADDIMMH511C8(), checkADDIMMH521C8(), checkSUBIMMH501C8(), checkSUBIMMH511C8(), checkSUBIMMH521C8(), checkLDB50601D0(), checkLDB51601D0(), checkLDB52601D0(), checkADDIMMH501D0(), checkADDIMMH511D0(), checkADDIMMH521D0(), checkSUBIMMH501D0(), checkSUBIMMH511D0(), checkSUBIMMH521D0(), checkLDB50601D8(), checkLDB51601D8(), checkLDB52601D8(), checkADDIMMH501D8(), checkADDIMMH511D8(), checkADDIMMH521D8(), checkSUBIMMH501D8(), checkSUBIMMH511D8(), checkSUBIMMH521D8(), checkLDB50601E0(), checkLDB51601E0(), checkLDB52601E0(), checkADDIMMH501E0(), checkADDIMMH511E0(), checkADDIMMH521E0(), checkSUBIMMH501E0(), checkSUBIMMH511E0(), checkSUBIMMH521E0(), checkLDB50601E8(), checkLDB51601E8(), checkLDB52601E8(), checkADDIMMH501E8(), checkADDIMMH511E8(), checkADDIMMH521E8(), checkSUBIMMH501E8(), checkSUBIMMH511E8(), checkSUBIMMH521E8(), checkLDB50601F0(), checkLDB51601F0(), checkLDB52601F0(), checkADDIMMH501F0(), checkADDIMMH511F0(), checkADDIMMH521F0(), checkSUBIMMH501F0(), checkSUBIMMH511F0(), checkSUBIMMH521F0(), checkLDB50601F8(), checkLDB51601F8(), checkLDB52601F8(), checkADDIMMH501F8(), checkADDIMMH511F8(), checkADDIMMH521F8(), checkSUBIMMH501F8(), checkSUBIMMH511F8(), checkSUBIMMH521F8(), checkLDB5060200(), checkLDB5160200(), checkLDB5260200(), checkADDIMMH50200(), checkADDIMMH51200(), checkADDIMMH52200(), checkSUBIMMH50200(), checkSUBIMMH51200(), checkSUBIMMH52200(), checkLDB5060208(), checkLDB5160208(), checkLDB5260208(), checkADDIMMH50208(), checkADDIMMH51208(), checkADDIMMH52208(), checkSUBIMMH50208(), checkSUBIMMH51208(), checkSUBIMMH52208(), checkLDB5060210(), checkLDB5160210(), checkLDB5260210(), checkADDIMMH50210(), checkADDIMMH51210(), checkADDIMMH52210(), checkSUBIMMH50210(), checkSUBIMMH51210(), checkSUBIMMH52210(), checkLDB5060218(), checkLDB5160218(), checkLDB5260218(), checkADDIMMH50218(), checkADDIMMH51218(), checkADDIMMH52218(), checkSUBIMMH50218(), checkSUBIMMH51218(), checkSUBIMMH52218(), checkLDB5060220(), checkLDB5160220(), checkLDB5260220(), checkADDIMMH50220(), checkADDIMMH51220(), checkADDIMMH52220(), checkSUBIMMH50220(), checkSUBIMMH51220(), checkSUBIMMH52220(), checkLDB5060228(), checkLDB5160228(), checkLDB5260228(), checkADDIMMH50228(), checkADDIMMH51228(), checkADDIMMH52228(), checkSUBIMMH50228(), checkSUBIMMH51228(), checkSUBIMMH52228(), checkLDB5060230(), checkLDB5160230(), checkLDB5260230(), checkADDIMMH50230(), checkADDIMMH51230(), checkADDIMMH52230(), checkSUBIMMH50230(), checkSUBIMMH51230(), checkSUBIMMH52230(), checkLDB5060232(), checkLDB5160232(), checkLDB5260232(), checkADDIMMH50232(), checkADDIMMH51232(), checkADDIMMH52232(), checkSUBIMMH50232(), checkJMP(), checkCALLBACK(), checkCALLRET(), checkLDB(), checkLDBoff8(), checkLDBOFF8HANDLER(), checkLDBOFF127HANDLER(), checkLDBOFFM128HANDLER(), checkLDBOFF64HANDLER(), checkLDBOFF16HANDLER(), checkLDBOFF32HANDLER(), checkLDBOFF96HANDLER(), checkLDBOFF112HANDLER(), checkLDBoff127(), checkLDBoff128(), checkLDBoff256(), checkLDBoffm128(), checkLDBoffm129(), checkMEMCPYDATA(), checkMEMCPYSTATE(), checkGET6050(), checkGET5060(), checkSET50FFF(), checkSET5110000(), checkORV5062(), checkCMP6052()];
  let failed = 0;
  for (const c of cases) {
    if (c.ok) console.log(`${c.id} PASS — ${c.detail}`);
    else { failed++; console.error(`${c.id} FAIL — ${c.detail}`); }
  }
  if (cases.length === 0) { console.error('golden: no cases registered (fail-closed)'); process.exit(1); }
  if (failed > 0) { console.error(`golden: ${failed}/${cases.length} failed`); process.exit(1); }
  console.log(`golden: ${cases.length} case(s) ok (016+017+018+019+020+021+022+023+024+025+026+027+028+029+030+031+032+033+034+035+036+037+038+039+040+041+042+043+044+045+046+047+048+049+050+051+052+053+054+055+056+057+058+059+060+061+062+063+064+065+066+067+068+069+070+071+072+073+074+075+076+077+078+079+080+081+082+083+084+085+086+087+088+089+090+091+092+093+094+095+096+097+098+099+100+101 batch-95 reflectors)`);
  process.exit(0);
}

main();
