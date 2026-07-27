/**
 * parallel-batch-11 probe — scratch-only fresh picks after H_54..H_61.
 * Mirror parallel-batch-10-run.mjs. Do NOT touch yoyo.ty / lock / goldens.
 */
import { spawnSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import { fileURLToPath } from 'url';
import { encodeOp } from '../../yoyo-js/src/platform/encode-x64.js';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, '../..');
const GOLDEN = path.join(ROOT, 'yoyo/tests/golden');
const JS_DRIVER = path.join(ROOT, 'scripts/_probe/js-ty2text.mjs');
const RUST_CWD = path.join(ROOT, 'yoyo-rust/verifier');

function pinOf(op, args) {
  return Buffer.from([...encodeOp(op, args), 0xc3]).toString('hex');
}

const picks = [
  {
    name: 'inc_h51',
    opcode: '0x66 INC',
    args: 'slot=0x51',
    body: '66 51',
    expected: pinOf(0x66, [0x51]),
  },
  {
    name: 'dec_h51',
    opcode: '0x67 DEC',
    args: 'slot=0x51',
    body: '67 51',
    expected: pinOf(0x67, [0x51]),
  },
  {
    name: 'addimm_h51',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x51 imm=0x07',
    body: '62 51 07',
    expected: pinOf(0x62, [0x51, 0x07]),
  },
  {
    name: 'cmp_h52',
    opcode: '0x65 CMP',
    args: '(0x52, 0x51)',
    body: '65 52 51',
    expected: pinOf(0x65, [0x52, 0x51]),
  },
  {
    name: 'addv_5052',
    opcode: '0x68 ADDV',
    args: '(0x50, 0x52)',
    body: '68 50 52',
    expected: pinOf(0x68, [0x50, 0x52]),
  },
  {
    name: 'get_5150',
    opcode: '0x60 GET',
    args: '(0x51, 0x50)',
    body: '60 51 50',
    expected: pinOf(0x60, [0x51, 0x50]),
  },
  {
    name: 'set_12345678',
    opcode: '0x30 SET',
    args: 'slot=0x50 imm=0x12345678',
    body: '30 50 12345678',
    expected: pinOf(0x30, [0x50, 0x12345678]),
  },
  {
    name: 'ldb_dst52',
    opcode: '0x80 LDB',
    args: '(0x52, 0x60, 0x08)',
    body: '80 52 60 08',
    expected: pinOf(0x80, [0x52, 0x60, 0x08]),
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
  const tySrc = `; _scratch_${p.name}.ty — ${p.opcode} ${p.args}\n; batch-11 scratch-only\n40 00\n  ${p.body}\n  FF\n`;
  fs.writeFileSync(ty, tySrc);

  const js = spawnSync('node', [JS_DRIVER, ty], { encoding: 'buffer' });
  if (js.status !== 0) {
    results.push({ ...p, result: 'REJECT', detail: 'js fail ' + (js.stderr?.toString() || '') });
    continue;
  }
  const jsCode = js.stdout;

  const rustBin = path.join(GOLDEN, `_scratch_${p.name}.bin`);
  const rust = spawnSync(
    'cargo',
    ['run', '-q', '-p', 'verifier', '--bin', 'yoyo', '--', 'link', '--target=stub', ty, rustBin],
    { cwd: path.join(ROOT, 'yoyo-rust'), encoding: 'buffer' }
  );
  if (rust.status !== 0) {
    results.push({ ...p, result: 'REJECT', detail: 'rust fail ' + (rust.stderr?.toString() || '') });
    continue;
  }
  const rustBlob = fs.readFileSync(rustBin);
  const rustCode = rustBlob.subarray(1); // strip 1B startup

  const jsHex = jsCode.toString('hex');
  const rustHex = rustCode.toString('hex');
  const byteEq = jsHex === rustHex && jsHex === p.expected;
  fs.writeFileSync(hexOut, p.expected);

  results.push({
    ...p,
    jsHex,
    rustHex,
    jsSha: sha16(jsCode),
    rustSha: sha16(rustCode),
    fullSha: shaFull(jsCode),
    byteEq,
    len: jsCode.length,
    result: byteEq ? 'PASS' : 'REJECT',
  });
  console.log(
    `${p.name}: ${byteEq ? 'PASS' : 'REJECT'} ${jsCode.length}B js=${jsHex.slice(0, 24)}… sha=${sha16(jsCode)}`
  );
}

const pass = results.filter((r) => r.result === 'PASS').length;
const reject = results.filter((r) => r.result === 'REJECT').length;
console.log(`\nSummary: ${pass} PASS / ${reject} REJECT`);

fs.writeFileSync(
  path.join(ROOT, 'docs/auxdocs/_parallel_batch_11_results.json'),
  JSON.stringify(results, null, 2)
);
