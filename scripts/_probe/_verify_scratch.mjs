// Verify all 7 scratch fixtures for body-extend-005
import fs from 'fs';
import path from 'path';
import { execSync } from 'child_process';
import crypto from 'crypto';
import { encodeOp, loadState, storeState, movabsRax } from '../../yoyo-js/src/platform/encode-x64.js';

const ROOT = path.resolve('.');
const GOLDEN_DIR = path.join(ROOT, 'yoyo', 'tests', 'golden');

const picks = [
  { hh: '33', name: 'RAW_BYTES', opcode: 0xA1, args: [0xCC, 0xDD], expectedHex: 'ccddc3' },
  { hh: '34', name: 'IMUL',      opcode: 0x63, args: [0x50, 0x51], expectedHex: '498b8780020000498b8f88020000480fafc149898780020000c3' },
  { hh: '35', name: 'SUBV',      opcode: 0x6A, args: [0x50, 0x51], expectedHex: '498b8780020000498b8f880200004829c849898780020000c3' },
  { hh: '36', name: 'CMP',       opcode: 0x65, args: [0x50, 0x51], expectedHex: '498b8780020000498b8f880200004839c8c3' },
  { hh: '37', name: 'LDB',       opcode: 0x80, args: [0x50, 0x60, 0], expectedHex: '498b8700030000480fb60049898780020000c3' },
  { hh: '38', name: 'SET',       opcode: 0x30, args: [0x50, 0],   expectedHex: '48b8000000000000000049898780020000c3' },
  { hh: '39', name: 'GET',       opcode: 0x60, args: [0x50, 0x51], expectedHex: '498b878802000049898780020000c3' },
];

function jsEncodeForPick(p) {
  const bytes = encodeOp(p.opcode, p.args, false);
  bytes.push(0xC3);
  return Buffer.from(bytes);
}

function jsEncodeFromFixture(fixturePath) {
  const out = execSync(`node "scripts\\_probe\\js-ty2text.mjs" "${fixturePath}"`, { cwd: ROOT });
  return Buffer.from(out);
}

function shaHex(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex');
}

function rustEmit(fixturePath) {
  const exePath = path.join(ROOT, 'yoyo-rust', 'target', 'release', 'yoyo.exe');
  const outPath = path.join(ROOT, 'yoyo-js', 'build', `scratch_${path.basename(fixturePath, '.ty')}_stub.bin`);
  const cmd = `"${exePath}" link --target=stub "${fixturePath}" "${outPath}"`;
  const stdout = execSync(cmd, { cwd: ROOT, encoding: 'utf8', shell: 'cmd.exe' });
  return { stdout };
}

function rustCodeOnly(fixturePath) {
  const outPath = path.join(ROOT, 'yoyo-js', 'build', `scratch_${path.basename(fixturePath, '.ty')}_stub.bin`);
  if (!fs.existsSync(outPath)) return null;
  const blob = fs.readFileSync(outPath);
  if (blob.length === 0) return null;
  // stub startup = [0xC3] (1 byte); skip it
  return blob.subarray(1);
}

let allPass = true;
const summary = [];

for (const p of picks) {
  const fixturePath = path.join(GOLDEN_DIR, `_scratch_h${p.hh}.ty`);
  const expPath = path.join(GOLDEN_DIR, 'expected', `_scratch_h${p.hh}.code.hex`);
  const expectedHex = fs.readFileSync(expPath, 'utf8').trim();
  const expected = Buffer.from(expectedHex, 'hex');

  // JS direct encodeOp
  const jsBytes = jsEncodeForPick(p);
  const jsOk = jsBytes.equals(expected);
  const jsSha = shaHex(jsBytes);

  // JS via fixture compile (js-ty2text.mjs)
  const jsFixtureBytes = jsEncodeFromFixture(fixturePath);
  let jsFixtureSlice = null;
  for (let i = 0; i < jsFixtureBytes.length; i++) {
    if (jsFixtureBytes[i] === expected[0]) {
      let m = true;
      for (let j = 0; j < expected.length && i + j < jsFixtureBytes.length; j++) {
        if (jsFixtureBytes[i + j] !== expected[j]) { m = false; break; }
      }
      if (m) { jsFixtureSlice = jsFixtureBytes.subarray(i, i + expected.length); break; }
    }
  }
  const jsFixtureOk = jsFixtureSlice && jsFixtureSlice.equals(expected);

  // Rust via link --target=stub
  let rustBytes = null;
  let rustSha = null;
  let rustOk = false;
  try {
    rustEmit(fixturePath);
    rustBytes = rustCodeOnly(fixturePath);
    if (rustBytes) {
      rustOk = rustBytes.equals(expected);
      rustSha = shaHex(rustBytes);
    }
  } catch (e) {
    // skip
  }

  const allPeerOk = jsOk && jsFixtureOk && rustOk;
  if (!allPeerOk) allPass = false;
  summary.push({
    hh: p.hh,
    name: p.name,
    expectedHex,
    jsOk, jsFixtureOk, rustOk,
    jsSha, rustSha,
    len: expected.length,
  });
}

console.log(JSON.stringify(summary, null, 2));
console.log(`\nALL_PASS: ${allPass}`);
process.exit(allPass ? 0 : 1);