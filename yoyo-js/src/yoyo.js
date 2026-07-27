#!/usr/bin/env node
/**
 * yoyo.js — M0 seed compiler (PROMPT-v3 trust anchor, ≤162 lines of logic).
 * Reads .ty hex tokens → emits x64 → wraps PE32+.
 * Platform bytes live in ./platform/* (not in this file).
 */
'use strict';
const fs = require('fs');
const path = require('path');
const { encodeOp } = require('./platform/encode-x64');
const { buildPe } = require('./platform/pe-builder');
const { OUTPUT_DATA_NEED } = require('./platform/platform-config');

function parseTy(src) {
  const lines = [];
  const names = new Map();
  let nextSlot = 0x50;
  const isHex = (t) => /^(0x)?[0-9a-fA-F]+$/.test(t);
  const hex = (t) => parseInt(t.replace(/^0x/i, ''), 16);
  const slotOf = (t) => {
    if (isHex(t)) return hex(t);
    if (names.has(t)) return names.get(t);
    const s = nextSlot++;
    names.set(t, s);
    return s;
  };
  for (const raw of src.split(/\r?\n/)) {
    let line = raw.replace(/[;#].*$/, '').trim();
    if (!line || /^LAYOUT$/i.test(line) || /^END_LAYOUT$/i.test(line)) continue;
    const toks = line.split(/\s+/);
    const op = hex(toks[0]);
    const args = toks.slice(1).map(slotOf);
    lines.push({ op, args });
  }
  return lines;
}

function compile(lines) {
  const code = [];
  const data = [];
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
    if (op === 0x10 || op === 0x12 || op === 0x13) {
      for (const a of args) data.push(a & 0xff);
      continue;
    }
    if (op === 0x41 || op === 0x70 || (op >= 0x71 && op <= 0x7a)) {
      const start = code.length;
      const bytes = encodeOp(op, args, true);
      code.push(...bytes);
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
  return { code: Buffer.from(code), data: Buffer.from(data), labels };
}

function main() {
  const [,, inFile, outFile] = process.argv;
  if (!inFile || !outFile) {
    console.error('usage: node yoyo.js <input.ty> <output.exe>');
    process.exit(2);
  }
  const src = fs.readFileSync(inFile, 'utf8');
  const { code, data } = compile(parseTy(src));
  const pe = buildPe(code, data, OUTPUT_DATA_NEED);
  fs.mkdirSync(path.dirname(path.resolve(outFile)), { recursive: true });
  fs.writeFileSync(outFile, pe);
  console.log(`M0: ${inFile} → ${outFile} (${pe.length} bytes, code=${code.length}, dataFloor=0x${OUTPUT_DATA_NEED.toString(16)})`);
}

main();
