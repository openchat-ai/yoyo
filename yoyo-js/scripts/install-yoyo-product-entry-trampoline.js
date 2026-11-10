'use strict';
/**
 * AUTO-TRUER42: poke product PE .text entry with jmp-rel32 to connected face.
 * Post-emit install (not plant/A1 rewrite). Does not claim full open_loop.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');
const connExpect = path.join(
  projects,
  'yoyo-compiler-open-loop-connected-face.expect.hex',
);
const inPe =
  process.argv[2] ||
  path.join(root, 'scripts', '_stage-truer42', 'product', 'output.exe');
const outPe =
  process.argv[3] ||
  path.join(root, 'scripts', '_stage-truer42', 'installed', 'output.exe');

function peLayout(pe) {
  const e = pe.readUInt32LE(0x3c);
  const sizeOpt = pe.readUInt16LE(e + 20);
  const entryRva = pe.readUInt32LE(e + 24 + 16);
  const sect = e + 24 + sizeOpt;
  const vaddr = pe.readUInt32LE(sect + 12);
  const rp = pe.readUInt32LE(sect + 20);
  const rs = pe.readUInt32LE(sect + 16);
  return { e, entryRva, vaddr, rp, rs, entryOff: entryRva - vaddr };
}

function main() {
  if (!fs.existsSync(inPe)) {
    console.log('I42 status=RED reason=missing_input');
    process.exit(1);
  }
  const pe = Buffer.from(fs.readFileSync(inPe));
  const { entryOff, rp, rs } = peLayout(pe);
  if (entryOff < 0 || entryOff + 5 > rs) {
    console.log('I42 status=RED reason=entry_oob');
    process.exit(1);
  }
  const text = pe.subarray(rp, rp + rs);
  const conn = Buffer.from(
    fs.readFileSync(connExpect, 'utf8').trim().replace(/\s+/g, ''),
    'hex',
  );
  const cidx = text.indexOf(conn);
  if (cidx < 0) {
    console.log('I42 status=RED reason=missing_connected');
    process.exit(1);
  }
  const rel = (cidx - (entryOff + 5)) | 0;
  if (rel !== cidx - 5 || entryOff !== 0) {
    // still support entryOff!=0; rel is always target - (entry+5)
  }
  const tramp = Buffer.alloc(5);
  tramp[0] = 0xe9;
  tramp.writeInt32LE(rel, 1);
  const before = Buffer.from(pe.subarray(rp + entryOff, rp + entryOff + 5));
  if (before.equals(tramp)) {
    console.log('I42 status=RED reason=already_installed');
    process.exit(1);
  }
  tramp.copy(pe, rp + entryOff);
  fs.mkdirSync(path.dirname(outPe), { recursive: true });
  fs.writeFileSync(outPe, pe);
  const sha = crypto.createHash('sha256').update(pe).digest('hex');
  console.log('I42 installed_entry_off=' + entryOff);
  console.log('I42 connected_text_off=' + cidx);
  console.log('I42 trampoline=' + tramp.toString('hex'));
  console.log('I42 before=' + before.toString('hex'));
  console.log('I42 out=' + outPe);
  console.log('I42 sha256_prefix=' + sha.slice(0, 16));
  console.log('I42 sha256=' + sha);
  console.log('I42 status=GREEN');
}

main();
