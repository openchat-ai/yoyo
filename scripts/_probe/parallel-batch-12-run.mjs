/**
 * parallel-batch-12 probe — scratch-only fresh picks after H_62..H_69.
 * Mirror parallel-batch-11-run.mjs. Do NOT touch yoyo.ty / lock / goldens.
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

function sha16(hex) {
  return crypto.createHash('sha256').update(Buffer.from(hex, 'hex')).digest('hex').slice(0, 16);
}

// Fresh vs H_48..H_69: SUBIMM-h51, DEC-h52, INC-h52, ORV-5052, SUBV-5052,
// GET-5251, SET-f00dbabe, CMP-5250. Skip D-1/D-2/D-3 / MEMCPY.
const picks = [
  {
    name: 'subimm_h51',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x51 imm=0x03',
    body: '61 51 03',
    expected: pinOf(0x61, [0x51, 0x03]),
  },
  {
    name: 'dec_h52',
    opcode: '0x67 DEC',
    args: 'slot=0x52',
    body: '67 52',
    expected: pinOf(0x67, [0x52]),
  },
  {
    name: 'inc_h52',
    opcode: '0x66 INC',
    args: 'slot=0x52',
    body: '66 52',
    expected: pinOf(0x66, [0x52]),
  },
  {
    name: 'orv_5052',
    opcode: '0x69 ORV',
    args: '(0x50, 0x52)',
    body: '69 50 52',
    expected: pinOf(0x69, [0x50, 0x52]),
  },
  {
    name: 'subv_5052',
    opcode: '0x6A SUBV',
    args: '(0x50, 0x52)',
    body: '6A 50 52',
    expected: pinOf(0x6A, [0x50, 0x52]),
  },
  {
    name: 'get_5251',
    opcode: '0x60 GET',
    args: '(0x52, 0x51)',
    body: '60 52 51',
    expected: pinOf(0x60, [0x52, 0x51]),
  },
  {
    name: 'set_f00dbabe',
    opcode: '0x30 SET',
    args: 'slot=0x50 imm=0xF00DBABE',
    body: '30 50 F00DBABE',
    expected: pinOf(0x30, [0x50, 0xF00DBABE]),
  },
  {
    name: 'cmp_5250',
    opcode: '0x65 CMP',
    args: '(0x52, 0x50)',
    body: '65 52 50',
    expected: pinOf(0x65, [0x52, 0x50]),
  },
];

function writeScratch(p) {
  const tyPath = path.join(GOLDEN, `_scratch_${p.name}.ty`);
  const hexPath = path.join(GOLDEN, `_scratch_${p.name}.code.hex`);
  const ty = `; _scratch_${p.name}.ty — ${p.opcode} ${p.args}\n; batch-12 scratch-only\n40 00\n  ${p.body}\n  FF\n`;
  fs.writeFileSync(tyPath, ty);
  fs.writeFileSync(hexPath, p.expected + '\n');
  return { tyPath, hexPath };
}

function runJs(tyPath) {
  const r = spawnSync('node', [JS_DRIVER, tyPath], { encoding: 'buffer' });
  if (r.status !== 0) {
    return { ok: false, detail: (r.stderr || r.stdout || Buffer.from('js fail')).toString() };
  }
  return { ok: true, buf: r.stdout };
}

function runRust(tyPath, outBin) {
  const r = spawnSync(
    'cargo',
    ['run', '-q', '-p', 'verifier', '--bin', 'yoyo', '--', 'link', '--target=stub', tyPath, outBin],
    { cwd: path.join(ROOT, 'yoyo-rust'), encoding: 'buffer' }
  );
  if (r.status !== 0) {
    return { ok: false, detail: (r.stderr || r.stdout || Buffer.from('rust fail')).toString() };
  }
  const blob = fs.readFileSync(outBin);
  // strip 1B startup_blob (0xc3)
  return { ok: true, buf: blob.subarray(1) };
}

const rows = [];
for (const p of picks) {
  const { tyPath } = writeScratch(p);
  const outBin = path.join(GOLDEN, `_scratch_${p.name}.bin`);
  const js = runJs(tyPath);
  const rust = runRust(tyPath, outBin);
  const exp = Buffer.from(p.expected, 'hex');
  const jsOk = js.ok && js.buf.equals(exp);
  const rustOk = rust.ok && rust.buf.equals(exp);
  const byteEq = jsOk && rustOk && js.buf.equals(rust.buf);
  const jsha = js.ok ? sha16(js.buf.toString('hex')) : 'FAIL';
  const rsha = rust.ok ? sha16(rust.buf.toString('hex')) : 'FAIL';
  const result = byteEq && jsha === rsha ? 'PASS' : 'REJECT';
  rows.push({
    ...p,
    pin: p.expected,
    len: exp.length,
    jsOk,
    rustOk,
    byteEq: byteEq ? 'Y' : 'N',
    jsha,
    rsha,
    result,
  });
  console.log(
    `${p.name}: ${result} len=${exp.length} js=${jsOk} rust=${rustOk} sha=${jsha}`
  );
}

const pass = rows.filter((r) => r.result === 'PASS');
const reject = rows.filter((r) => r.result !== 'PASS');
console.log(`\nSummary: ${pass.length} PASS / ${reject.length} REJECT`);

const logPath = path.join(ROOT, 'docs/auxdocs/parallel-batch-12-log.md');
const table = rows
  .map(
    (r, i) =>
      `| ${i + 1} | ${r.opcode} | ${r.args} | \`${r.pin}\` (${r.len}) | ${r.jsOk ? 'same' : 'DIFF'} | ${r.rustOk ? 'same' : 'DIFF'} | ${r.byteEq} | \`${r.jsha}\` | \`${r.rsha}\` | ${r.result} |`
  )
  .join('\n');

const passTable = pass
  .map((r, i) => {
    const h = 70 + i;
    const sel = (0x4c + i).toString(16).toUpperCase().padStart(2, '0');
    return `| H_${h} | 0x${sel} | ${r.opcode} | ${r.args.replace(/slot=|imm=|\(|\)/g, '').trim()} | \`${r.pin}\` (${r.len}B) | \`${r.jsha}\` |`;
  })
  .join('\n');

const log = `# parallel-batch-12 Log · 8-pick fresh-picks scratch test sweep

> Tag: \`parallel-batch-12-EXPERIMENTAL-8-pick-scratch\` · 2026-07-25 (UTC+8).
> Following body-extend-017 (pin \`d1d92927…\`, handlers ≈ 76, H_62..H_69 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (\`_scratch_*.ty\` + \`_scratch_*.code.hex\`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_69 and
> not already present as handlers in current \`yoyo.ty\`. Slot/imm/dst
> variations of SUB-IMM/DEC/INC/ORV/SUBV/GET/SET/CMP. Skipped D-1
> 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
${table}

**Summary**: ${pass.length} PASS / ${reject.length} REJECT out of ${rows.length} attempted.

## §2. Pick rationale

- SUB-IMM at slot 0x51 imm=0x03 (canonical uses slot 0x50; not in H_62..H_69).
- DEC/INC at slot 0x52 (H_62/H_63 use 0x51; H_11/H_12 use 0x50).
- ORV/SUBV at 50 52 (canonical pairs differ; ADDV-5052 is H_66 but ORV/SUBV-5052 absent).
- GET at 52 51 (H_39/H_51/H_59/H_67 cover other pairs).
- SET at slot 0x50 imm=0xF00DBABE (distinct from CAFEBABE/DEADBEEF/12345678).
- CMP at 52 50 (H_36/H_58/H_65 cover other pairs).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

${pass.map((r) => `- \`yoyo/tests/golden/_scratch_${r.name}.ty\` + \`.code.hex\``).join('\n')}
- \`docs/auxdocs/parallel-batch-12-log.md\` — this file
- \`scripts/_probe/parallel-batch-12-run.mjs\` — probe runner

NO \`yoyo/projects/yoyo.ty\`, NO lock, NO \`golden.js\`, NO \`self_test.rs\`, NO \`main.rs\`.

## §4. Parent next

**parent next = body-extend-018 serialize PASSes + 1 Relock**

Pass pin from body-extend-017 Relock: \`d1d92927a66b19ae2ca5b8f13861a58b956da81a969944943c0d68f03104986c\`.
Handlers before consolidate ≈ 76 (H_00..H_69). Next selectors 0x4C.. for H_70.. if all serialize.

PASS list for body-extend-018:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
${passTable}
`;

fs.writeFileSync(logPath, log);
console.log('wrote', logPath);
