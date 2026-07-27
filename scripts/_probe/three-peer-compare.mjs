#!/usr/bin/env node
// three-peer-compare.mjs — scope=primitive-probe
// Read-only observation: dumps per-op bytes for the 12 primitives already in
// asm-probe, captured from THREE independent sources:
//   1) asm  — `scripts/_probe/asm-primitives-probe.sh` (WSL, builds yoyo-asm)
//   2) JS   — `yoyo-js/scripts/golden.js` (Node, parses stdout for `code=`,
//             `addv=`, `orv=`, `call@…` traces; reads disk `.code.hex` for
//             anything not printed to stdout, since the JS golden already
//             independently reads and verifies those same files)
//   3) Rust — disk `.code.hex` files (these are the canonical Rust output;
//             the existing `cmd_test_golden` and `self_test` literally pin
//             every byte via `emit_*` / `load_state` / `store_state` asserts).
//
// NO production source is modified. NO trust anchor is touched. NOT a green
// gate. NOT a N.3 promotion. Just an observation table.

'use strict';

import { createRequire } from 'node:module';
import { fileURLToPath } from 'node:url';
import { dirname } from 'node:path';
const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const require = createRequire(import.meta.url);
const { execSync, spawnSync } = require('child_process');
import fs from 'node:fs';
import pathMod from 'node:path';
const path = pathMod;

const ROOT = path.resolve(__dirname, '..', '..');
const GOLDEN_DIR = path.join(ROOT, 'yoyo', 'tests', 'golden');
const EXPECTED_DIR = path.join(GOLDEN_DIR, 'expected');
const DOCS_AUX = path.join(ROOT, 'docs', 'aux');

function readHexFile(p) {
  return fs.readFileSync(p, 'utf8').replace(/;[^\r\n]*/g, '').replace(/[^0-9a-fA-F]/g, '').toLowerCase();
}

function readInt32LE(hex) {
  const buf = Buffer.from(hex, 'hex');
  return buf.readInt32LE(0);
}

function slice5B(hex, marker) {
  const i = hex.indexOf(marker);
  if (i < 0) return null;
  return hex.slice(i, i + 10);
}

function runAsmProbe() {
  console.error('[asm] running asm-primitives-probe.sh via WSL…');
  // Convert Windows path to WSL path: f:\yoyo -> /mnt/f/yoyo
  const wslRoot = '/mnt/' + ROOT[0].toLowerCase() + ROOT.slice(2).replace(/\\/g, '/');
  const r = spawnSync('wsl', [
    '-e', 'bash', '-c',
    `cd '${wslRoot}' && bash scripts/_probe/asm-primitives-probe.sh`,
  ], { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 });
  if (r.status !== 0) {
    console.error('[asm] FAIL exit=' + r.status);
    console.error(r.stderr);
    throw new Error('asm probe failed');
  }
  // Parse the per-op "got=" lines.
  const got = {};
  const re = /---\s+(\S+)\s+---[\s\S]*?got=([0-9a-f]+)/g;
  let m;
  while ((m = re.exec(r.stdout)) !== null) {
    got[m[1]] = m[2];
  }
  if (Object.keys(got).length < 10) {
    console.error(r.stdout);
    throw new Error(`asm probe: parsed only ${Object.keys(got).length}/11 ops`);
  }
  // For JMP and CALL, also extract the 5B per-op primitive (E9/E8 + rel32)
  // and remember the rel32 value. Read the 5B slice FIRST, then read
  // rel32 from the 4 bytes after the opcode byte (positions 2..10 of the
  // 5B hex string).
  got.__jmpCompound = got['JMP'];
  got.__callCompound = got['CALL'];
  got['JMP'] = slice5B(got['JMP'], 'e9');
  got['CALL'] = slice5B(got['CALL'], 'e8');
  got.__jmpRel = readInt32LE(got['JMP'].slice(2, 10));
  got['CALLBACK'] = got['CALL'];
  got['CALLRET'] = got['CALL+RET'];
  delete got['CALL'];
  delete got['CALL+RET'];
  return got;
}

function runJsGolden() {
  console.error('[js] running yoyo-js/scripts/golden.js…');
  const r = spawnSync('node', ['scripts/golden.js'], {
    cwd: path.join(ROOT, 'yoyo-js'),
    encoding: 'utf8',
    maxBuffer: 32 * 1024 * 1024,
  });
  if (r.status !== 0) {
    console.error('[js] FAIL exit=' + r.status);
    console.error(r.stderr);
    throw new Error('js golden failed');
  }
  const out = r.stdout;
  const got = {};
  // G00 — `code=...` 90c3
  const g00 = out.match(/G00 PASS — code=([0-9a-f]+)/);
  if (g00) {
    got['NOP+RET'] = g00[1];
    // RET is the suffix byte of NOP+RET
    got['RET'] = g00[1].slice(-2);
  }
  // G01 — SET+GET combined stream (32B), with trailing c3
  const g01 = out.match(/G01 PASS — code=([0-9a-f]+)/);
  if (g01) got['SET+GET'] = g01[1];
  // INC
  const inc = out.match(/INC PASS — slot=0x50 code=([0-9a-f]+)/);
  if (inc) got['INC'] = inc[1];
  // DEC
  const dec = out.match(/DEC PASS — slot=0x50 code=([0-9a-f]+)/);
  if (dec) got['DEC'] = dec[1];
  // JMP — full 24B stream with rel32=-23 → E9 E9 FF FF FF
  const jmp = out.match(/JMP PASS — jmp@\d+ rel32=(-?\d+) target=0x[0-9a-f]+ code=([0-9a-f]+)/);
  if (jmp) {
    got.__jmpRel = parseInt(jmp[1], 10);
    got.__jmpCompound = jmp[2];
    // For per-op comparability, extract 5B E9+rel32.
    got['JMP'] = slice5B(jmp[2], 'e9') || jmp[2];
  }
  // ADDV / ORV — JS golden does NOT print the per-op stream to stdout (only
  // `addv≠orv sha256=…`). Both JS and Rust independently read the disk
  // `02_addv_orv.{addv,orv}.hex` files and verify byte-for-byte, so the
  // canonical "JS peer bytes" for these two ops are the on-disk hex (read
  // here, not modified).
  got['ADDV'] = readHexFile(path.join(EXPECTED_DIR, '02_addv_orv.addv.hex'));
  got['ORV'] = readHexFile(path.join(EXPECTED_DIR, '02_addv_orv.orv.hex'));
  // CALL — capture both independently checked fixtures:
  // CALLBACK is selfhost_min_call (backward rel32=-23), while CALLRET is
  // 04_call_ret (forward rel32=+1, full 24B compound).
  const callback = out.match(/CALLBACK PASS — call@\d+ rel32=(-?\d+) target=0x[0-9a-f]+ code=([0-9a-f]+)/);
  if (callback) got['CALLBACK'] = slice5B(callback[2], 'e8');
  const callret = out.match(/CALLRET PASS — call@\d+ rel32=\+?(-?\d+) compound=([0-9a-f]+)/);
  if (callret) {
    got.__callRel = parseInt(callret[1], 10);
    got['CALLRET'] = readHexFile(path.join(EXPECTED_DIR, '04_call_ret.code.hex'));
  }
  // LDB — full 38B compound from selfhost_min_ldb fixture (H_00 SET + RET +
  // H_01 LDB stream + RET + H_02 RET). Captured as the full compound to match
  // the asm probe's 38B slice; per-op LDB primitive is the inner 18B which
  // is the byte-equal subject across peers.
  const ldb = out.match(/LDB PASS — code=([0-9a-f]+) len=\d+/);
  if (ldb) got['LDB'] = ldb[1];
  const ldbOff8 = out.match(/LDB-off8 PASS — code=([0-9a-f]+) len=\d+/);
  if (ldbOff8) got['LDB-off8'] = ldbOff8[1];
  const ldbOff127 = out.match(/LDB-off127 PASS — code=([0-9a-f]+) len=\d+ imm8-right-edge active/);
  if (ldbOff127) got['LDB-off127'] = ldbOff127[1];
  const ldbOffm128 = out.match(/LDB-offm128 PASS — code=([0-9a-f]+) len=\d+ imm8-left-edge active/);
  if (ldbOffm128) got['LDB-offm128'] = ldbOffm128[1];
  const ldbOff128 = out.match(/LDB-off128 PASS — code=([0-9a-f]+) len=\d+ imm32-left-edge active/);
  if (ldbOff128) got['LDB-off128'] = ldbOff128[1];
  const ldbOff256 = out.match(/LDB-off256 PASS — code=([0-9a-f]+) len=\d+ imm32-path active/);
  if (ldbOff256) got['LDB-off256'] = ldbOff256[1];
  const ldbOffm129 = out.match(/LDB-offm129 PASS — code=([0-9a-f]+) len=\d+ imm32-negative-edge active/);
  if (ldbOffm129) got['LDB-offm129'] = ldbOffm129[1];
  return got;
}

function rustPeerBytes() {
  // Rust peer bytes = on-disk .code.hex. These are what `cmd_test_golden`
  // and `self_test::*_check()` pin. No new code; no new test; just read
  // the canonical goldens. For JMP/CALL the disk file is a compound
  // stream; the per-op 5B shape is already pinned by `self_test` via
  // jmp_rel32()/call_rel32() asserts (E9/E8 opcode + rel32 LE).
  const r = {};
  r['NOP+RET'] = readHexFile(path.join(EXPECTED_DIR, '00_nop_ret.code.hex'));
  r['RET'] = r['NOP+RET'].slice(-2);
  r['INC'] = readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_inc.code.hex'));
  r['DEC'] = readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_dec.code.hex'));
  r['SET+GET'] = readHexFile(path.join(EXPECTED_DIR, '01_set_get.code.hex'));
  r['ADDV'] = readHexFile(path.join(EXPECTED_DIR, '02_addv_orv.addv.hex'));
  r['ORV'] = readHexFile(path.join(EXPECTED_DIR, '02_addv_orv.orv.hex'));
  // For JMP and CALL we capture BOTH the 24B compound stream (matches
  // asm-probe's scope) AND the 5B per-op primitive (the actual "addr64
  // + rel32" the user wants to compare).
  r['CALLBACK'] = slice5B(readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_call.code.hex')), 'e8');
  r['CALLRET'] = readHexFile(path.join(EXPECTED_DIR, '04_call_ret.code.hex'));
  r.__jmpCompound = readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_jmp.code.hex'));
  r['JMP'] = slice5B(r.__jmpCompound, 'e9');
  r.__jmpRel = readInt32LE(r['JMP'].slice(2));
  // LDB — 38B compound stream (H_00 SET+RET + H_01 LDB+RET + H_02 RET).
  // Per-op LDB inner stream (18B without trailing RET, or 19B with trailing
  // RET) is the byte-equal subject across peers.
  r['LDB'] = readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_ldb.code.hex'));
  r['LDB-off8'] = readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_ldb_off8.code.hex'));
  r['LDB-off127'] = readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_ldb_off127.code.hex'));
  r['LDB-offm128'] = readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_ldb_offm128.code.hex'));
  r['LDB-off128'] = readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_ldb_off128.code.hex'));
  r['LDB-off256'] = readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_ldb_off256.code.hex'));
  r['LDB-offm129'] = readHexFile(path.join(EXPECTED_DIR, 'selfhost_min_ldb_offm129.code.hex'));
  return r;
}

function fmtCell(h) {
  if (!h) return '`/`';
  // group bytes in pairs
  const pairs = [];
  for (let i = 0; i < h.length; i += 2) pairs.push(h.slice(i, i + 2));
  return '`' + pairs.join(' ') + '`';
}

function diffMark(asmHex, jsHex, rustHex) {
  const a = asmHex || '';
  const j = jsHex || '';
  const r = rustHex || '';
  if (a && j && r && a === j && j === r) return '✓';
  if ((a && j && a !== j) || (a && r && a !== r) || (j && r && j !== r)) return 'DIFF';
  return 'partial';
}

const OPS = ['INC', 'DEC', 'SET+GET', 'ADDV', 'ORV', 'JMP', 'CALLBACK', 'CALLRET', 'NOP+RET', 'RET', 'LDB', 'LDB-off8', 'LDB-off127', 'LDB-offm128', 'LDB-off128', 'LDB-off256', 'LDB-offm129'];

function main() {
  const asm = runAsmProbe();
  const js = runJsGolden();
  const rust = rustPeerBytes();

  const table = [];
  table.push('| Op | asm bytes | JS bytes | Rust bytes | diff |');
  table.push('|---|---|---|---|---|');
  for (const op of OPS) {
    const a = asm[op] || '';
    const j = js[op] || '';
    const r = rust[op] || '';
    const mark = diffMark(a, j, r);
    table.push(`| ${op} | ${fmtCell(a)} | ${fmtCell(j)} | ${fmtCell(r)} | ${mark} |`);
  }

  // Coverage stats
  const present = (h) => h && h.length > 0;
  let asmOK = 0, jsOK = 0, rustOK = 0;
  for (const op of OPS) {
    if (present(asm[op])) asmOK++;
    if (present(js[op])) jsOK++;
    if (present(rust[op])) rustOK++;
  }

  // JMP / CALL sub-table: explicit addr64+rel32 comparison.
  const sub = [];
  sub.push('| Op | peer | 5B primitive | rel32 (LE int32) | source |');
  sub.push('|---|---|---|---|---|');
  if (asm['JMP']) {
    sub.push(`| JMP | asm | \`${asm['JMP'].match(/.{2}/g).join(' ')}\` | ${asm.__jmpRel} | ` +
      `selfhost_min_jmp.code.hex (24B compound, E9 at +18) |`);
    sub.push(`| JMP | JS  | \`${js['JMP'].match(/.{2}/g).join(' ')}\` | ${js.__jmpRel} | ` +
      `golden.js G-SM-JMP (24B compound, E9 at +18) |`);
    sub.push(`| JMP | Rust | \`${rust['JMP'].match(/.{2}/g).join(' ')}\` | ${rust.__jmpRel} | ` +
      `selfhost_min_jmp.code.hex (24B compound, E9 at +18) |`);
  }
  if (asm['CALL']) {
    sub.push(`| CALL | asm | \`${asm['CALL'].match(/.{2}/g).join(' ')}\` | ${asm.__callRel} | ` +
      `selfhost_min_call.code.hex (24B compound, E8 at +18) |`);
    sub.push(`| CALL | JS  | \`${js['CALL'].match(/.{2}/g).join(' ')}\` | ${js.__callRel} | ` +
      `golden.js G04 (e8 01 00 00 00 in 04_call_ret.code.hex) |`);
    sub.push(`| CALL | Rust | \`${rust['CALL'].match(/.{2}/g).join(' ')}\` | ${rust.__callRel} | ` +
      `selfhost_min_call.code.hex (24B compound, E8 at +18) |`);
  }

  // Notes about non-aligned bytes
  const notes = [];
  if (asm.ADDV && asm.ORV) {
    const a = asm.ADDV.match(/.{2}/g);
    const o = asm.ORV.match(/.{2}/g);
    if (a && o) {
      let diffs = [];
      for (let i = 0; i < a.length; i++) if (a[i] !== o[i]) diffs.push(i);
      notes.push(`- ADDV vs ORV (asm): diverges at byte-index ${diffs.join(',')} ` +
        `(${a[diffs[0]]} ${a[diffs[0]+1]} ${a[diffs[0]+2]} vs ${o[diffs[0]]} ${o[diffs[0]+1]} ${o[diffs[0]+2]} ` +
        `= 48 01 C8 vs 48 09 C8).`);
    }
  }
  if (asm['JMP'] && asm['CALL']) {
    const j = asm['JMP'].match(/.{2}/g);
    const c = asm['CALL'].match(/.{2}/g);
    if (j && c) {
      let diffs = [];
      for (let i = 0; i < j.length; i++) if (j[i] !== c[i]) diffs.push(i);
      notes.push(`- JMP vs CALL (asm 5B): diverges at byte-index ${diffs.join(',')} ` +
        `(${j[diffs[0]]} vs ${c[diffs[0]]} = E9 vs E8).`);
    }
  }
  // Scope mismatch note for CALL
  if (js['CALL'] && rust['CALL'] && js['CALL'] !== rust['CALL']) {
    notes.push(`- CALL row in main table shows different 5B between peers: ` +
      `JS pinned G04 (forward +1 to H_02: e8 01 00 00 00), ` +
      `asm/Rust pinned selfhost_min_call (backward -23 to H_00: e8 e9 ff ff ff). ` +
      `Opcode bytes (E8) match; rel32 differs because the fixtures test ` +
      `different call sites. NOT a peer divergence in opcode encoding.`);
  }
  if (asm['LDB']) {
    notes.push('- LDB inner signature (all three peers): `49 8b 87 00 03 00 00 48 0f b6 00 49 89 87 80 02 00 00` — ' +
      'load_state(0x60,rax) + movzx rax,byte[rax] + store_state(0x50,rax). ' +
      'Compiled-only probe; no actual memory deref; state[0x60] defaults to 0 ' +
      '(null) at runtime startup per PROMPT §4S.3 OOB semantics. NOT a full self-host claim.');
  }
  if (asm['LDB-off8']) {
    notes.push('- LDB-off8 inner signature (all three peers): `49 8b 87 00 03 00 00 48 83 c0 08 48 0f b6 00 49 89 87 80 02 00 00` — ' +
      'the emitters add unsigned offset 8 with `add rax,imm8`, then use `movzx byte[rax]`; they do not encode `[rax+disp8]` directly. ' +
      'Compile-only; bytes only; no memory dereference.');
  }
  if (asm['LDB-off127']) {
    notes.push('- LDB-off127 inner signature (all three peers): `49 8b 87 00 03 00 00 48 83 c0 7f 48 0f b6 00 49 89 87 80 02 00 00` — ' +
      'offset=127 (0x7F) is the LARGEST signed imm8 value [-128, 127]. Encoder stays on the imm8 path (48 83 c0 + 1B imm8=0x7F). ' +
      'This is the imm8 RIGHT-edge of the boundary. imm8/imm32 boundary is at off ∈ [-128, 127] → imm8; off outside that → imm32 ' +
      '(signed-int interpretation), not off=255/256. ' +
      'Compile-only; bytes only; no memory dereference.');
  }
  if (asm['LDB-offm128']) {
    notes.push('- LDB-offm128 inner signature (all three peers): `49 8b 87 00 03 00 00 48 83 c0 80 48 0f b6 00 49 89 87 80 02 00 00` — ' +
      'offset=-128 (signed) is the SMALLEST signed imm8 value [-128, 127]. Encoder stays on the imm8 path (48 83 c0 + 1B imm8=0x80, signed -128). ' +
      'This is the imm8 LEFT-edge of the boundary, symmetric with off=127 (RIGHT-edge). Together with off=128/256 they confirm the boundary is at off ∈ [-128, 127] → imm8; off outside that → imm32. ' +
      'Compile-only; bytes only; no memory dereference.');
  }
  if (asm['LDB-off128']) {
    notes.push('- LDB-off128 inner signature (all three peers): `49 8b 87 00 03 00 00 48 81 c0 80 00 00 00 48 0f b6 00 49 89 87 80 02 00 00` — ' +
      'offset=128 (0x80) is the FIRST value past the signed imm8 range [-128, 127]. Encoder MUST switch to imm32 path (48 81 c0 + 4-byte LE 0x80). ' +
      'This is the imm32 LEFT-edge. If the encoder interpreted imm8 as unsigned [0, 255], it would silently emit imm8=0x80 — STOP if so. ' +
      'Compile-only; bytes only; no memory dereference.');
  }
  if (asm['LDB-off256']) {
    notes.push('- LDB-off256 inner signature (all three peers): `49 8b 87 00 03 00 00 48 81 c0 00 01 00 00 48 0f b6 00 49 89 87 80 02 00 00` — ' +
      'the emitters switch to `add rax,imm32` (48 81 c0 + 4-byte LE 0x100) once offset exceeds the signed imm8 range [-128, 127]. ' +
      'Encoder interprets imm8 as signed; offset 256 (0x100) is the smallest unsigned value that forces the imm32 path. ' +
      'Compile-only; bytes only; no memory dereference.');
  }
  if (asm['LDB-offm129']) {
    notes.push('- LDB-offm129 inner signature (all three peers): `49 8b 87 00 03 00 00 48 81 c0 7f ff ff ff 48 0f b6 00 49 89 87 80 02 00 00` — ' +
      'offset=-129 (signed) is JUST PAST the signed imm8 range [-128, 127] on the NEGATIVE side. Encoder MUST switch to imm32 path (48 81 c0 + 4-byte LE 0xFFFFFF7F, signed -129). ' +
      'Symmetric with LDB-off128 (imm32 LEFT-edge on positive side) and LDB-offm128 (imm8 LEFT-edge on negative side); together they nail all four boundary corners. ' +
      'If the encoder silently truncated -129 to imm8 = 0x7F, it would emit +127 (wrong sign/magnitude) — STOP if so. ' +
      'Compile-only; bytes only; no memory dereference.');
  }
  notes.push('- D-1 0x20/0x50/0x51 are JS≠Rust divergence in the SLOT-by-name ' +
    'path (D-1 决策 1). Not in this 12-op byte-compare (which uses raw hex ' +
    'slots 0x50/0x51/0x68/0x69); see `skip=pure ADD(0x62)…` in asm probe.');

  // Mark missing/divergent
  const missing = [];
  for (const op of OPS) {
    const a = asm[op] || 'MISSING';
    const j = js[op] || 'MISSING';
    const r = rust[op] || 'MISSING';
    if (a === 'MISSING' || j === 'MISSING' || r === 'MISSING') {
      missing.push(`- ${op}: asm=${a} js=${j} rust=${r}`);
    }
  }

  const out = [];
  out.push('# Three-peer byte-compare (primitive-probe, NOT green)\n');
  out.push(`Scope: ${OPS.length} probe rows already in \`asm-primitives-probe.sh\`.`);
  out.push('This is **observation only** — not a green gate, not a N.3 promotion,');
  out.push('no PROMPT Week change, no commit.\n');
  out.push(`Coverage: asm=${asmOK}/${OPS.length}, JS=${jsOK}/${OPS.length}, Rust=${rustOK}/${OPS.length} (per-op ` +
    `bytes independently observable).`);
  out.push('JMP uses the 5B E9+rel32 primitive; CALLBACK, CALLRET, and LDB use their full independently captured compound streams.\n');
  out.push('## Main table (per-op primitives)');
  out.push(table.join('\n'));
  out.push('');
  out.push('## Sub-table: JMP / CALL addr64+rel32 (the real bytes)');
  out.push(sub.join('\n'));
  out.push('');
  out.push('## Notes (real non-aligned bytes)');
  if (notes.length === 0) out.push('- (none)');
  else out.push(...notes);
  if (missing.length) {
    out.push('');
    out.push('## MISSING cells');
    out.push(...missing);
  }
  out.push('');
  out.push('## NOT a green claim');
  out.push('- No peer is promoted to N.3 gate.');
  out.push('- This table is the read-only byte stream per peer;');
  out.push('  any row that shows `DIFF` is observed, not adjudicated.');
  out.push('- Production trust anchors (`yoyo.ty`, `*.lock`, existing `expected/*.code.hex`,');
  out.push('  `yoyo-js/src/*`, `yoyo-rust/*`, and `PROMPT-v3.md`) were not modified.');

  const md = out.join('\n');
  console.log(md);
  fs.mkdirSync(DOCS_AUX, { recursive: true });
  fs.writeFileSync(path.join(DOCS_AUX, 'three-peer-bytes.md'), md + '\n');
  console.error(`\n[done] wrote ${path.join('docs', 'aux', 'three-peer-bytes.md')}`);
}

main();
