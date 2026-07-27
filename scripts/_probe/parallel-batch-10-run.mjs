/**
 * parallel-batch-10 probe runner — JS via spawnSync + Rust cargo link,
 * strip 1B startup_blob, compare sha256, write .code.hex scratch pins.
 * DO NOT use PowerShell `>` redirect (UTF-16 corruption).
 */
import { spawnSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, '../..');
const GOLDEN = path.join(ROOT, 'yoyo/tests/golden');
const JS_DRIVER = path.join(ROOT, 'scripts/_probe/js-ty2text.mjs');
const RUST_CWD = path.join(ROOT, 'yoyo-rust/verifier');

const picks = [
  {
    name: 'set_deadbeef',
    opcode: '0x30 SET',
    args: 'slot=0x51 imm=0xDEADBEEF',
    expected: '48b8efbeadde0000000049898788020000c3',
  },
  {
    name: 'get_h52_50',
    opcode: '0x60 GET',
    args: '(0x52, 0x50)',
    expected: '498b878002000049898790020000c3',
  },
  {
    name: 'orv_h52',
    opcode: '0x69 ORV',
    args: '(0x52, 0x51)',
    expected: '498b8790020000498b8f880200004809c849898790020000c3',
  },
  {
    name: 'subv_h52',
    opcode: '0x6A SUBV',
    args: '(0x52, 0x51)',
    expected: '498b8790020000498b8f880200004829c849898790020000c3',
  },
  {
    name: 'imul_swap',
    opcode: '0x63 IMUL',
    args: '(0x51, 0x50)',
    expected: '498b8788020000498b8f80020000480fafc149898788020000c3',
  },
  {
    name: 'inc_h51',
    opcode: '0x66 INC',
    args: 'slot=0x51',
    expected: '498b878802000048ffc049898788020000c3',
  },
  {
    name: 'dec_h51',
    opcode: '0x67 DEC',
    args: 'slot=0x51',
    expected: '498b878802000048ffc849898788020000c3',
  },
  {
    name: 'addimm_h51',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x51 imm=0x07',
    expected: '498b87880200004883c00749898788020000c3',
  },
];

function sha16(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex').slice(0, 16);
}
function shaFull(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex');
}

const results = [];

for (const p of picks) {
  const ty = path.join(GOLDEN, `_scratch_${p.name}.ty`);
  const hexOut = path.join(GOLDEN, `_scratch_${p.name}.code.hex`);

  // JS driver via spawnSync (raw binary stdout)
  const js = spawnSync(process.execPath, [JS_DRIVER, ty], {
    encoding: 'buffer',
    cwd: ROOT,
  });
  let jsHex = null;
  let jsSha = null;
  let jsErr = null;
  if (js.status !== 0) {
    jsErr = (js.stderr || js.stdout || Buffer.from('js fail')).toString('utf8');
  } else {
    const code = Buffer.from(js.stdout);
    jsHex = code.toString('hex');
    jsSha = shaFull(code);
    fs.writeFileSync(hexOut, jsHex + '\n');
  }

  // Rust driver
  const rustBin = path.join(GOLDEN, `_scratch_${p.name}.rust.bin`);
  const rust = spawnSync(
    'cargo',
    ['run', '-q', '--bin', 'yoyo', '--', 'link', '--target=stub', ty, rustBin],
    { cwd: RUST_CWD, encoding: 'utf8', shell: true }
  );
  let rustHex = null;
  let rustSha = null;
  let rustErr = null;
  if (rust.status !== 0) {
    rustErr = (rust.stderr || rust.stdout || 'rust fail').toString();
  } else if (!fs.existsSync(rustBin)) {
    rustErr = 'no output bin';
  } else {
    const raw = fs.readFileSync(rustBin);
    // strip 1B startup_blob prefix (0xc3)
    const code = raw.length >= 1 ? raw.subarray(1) : raw;
    rustHex = code.toString('hex');
    rustSha = shaFull(code);
    try { fs.unlinkSync(rustBin); } catch {}
  }

  const expected = p.expected.toLowerCase();
  const jsMatch = jsHex === expected;
  const rustMatch = rustHex === expected;
  const byteEq = jsHex !== null && rustHex !== null && jsHex === rustHex;
  const pass = jsMatch && rustMatch && byteEq;

  results.push({
    ...p,
    expected,
    expectedLen: expected.length / 2,
    jsHex,
    rustHex,
    jsSha,
    rustSha,
    jsSha16: jsSha ? jsSha.slice(0, 16) : null,
    rustSha16: rustSha ? rustSha.slice(0, 16) : null,
    jsMatch,
    rustMatch,
    byteEq,
    pass,
    jsErr,
    rustErr,
  });

  console.log(
    JSON.stringify({
      name: p.name,
      pass,
      byteEq,
      jsHex,
      rustHex,
      jsSha16: jsSha ? jsSha.slice(0, 16) : null,
      rustSha16: rustSha ? rustSha.slice(0, 16) : null,
      jsErr: jsErr ? jsErr.slice(0, 200) : null,
      rustErr: rustErr ? rustErr.slice(0, 400) : null,
    })
  );
}

fs.writeFileSync(
  path.join(ROOT, 'docs/auxdocs/_parallel-batch-10-results.json'),
  JSON.stringify(results, null, 2)
);
const nPass = results.filter((r) => r.pass).length;
const nReject = results.length - nPass;
console.log(`SUMMARY ${nPass} PASS / ${nReject} REJECT`);
