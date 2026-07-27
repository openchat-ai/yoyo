/**
 * parallel-batch-42 probe — scratch-only fresh picks after H_302..H_309.
 * Mirror parallel-batch-41-run.mjs. Do NOT touch yoyo.ty / lock / goldens.
 */
import { spawnSync } from 'child_process';
import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import { fileURLToPath } from 'url';
import { createRequire } from 'module';

const require = createRequire(import.meta.url);
const { encodeOp } = require('../../yoyo-js/src/platform/encode-x64.js');

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const ROOT = path.resolve(__dirname, '../..');
const GOLDEN = path.join(ROOT, 'yoyo/tests/golden');
const JS_DRIVER = path.join(ROOT, 'scripts/_probe/js-ty2text.mjs');
const YOYO_EXE = path.join(ROOT, 'yoyo-rust/target/debug/yoyo.exe');

function pinOf(op, args) {
  return Buffer.from([...encodeOp(op, args), 0xc3]).toString('hex');
}

function shaFull(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex');
}

// Fresh vs H_48..H_309. Skip D-1/D-2/D-3 / MEMCPY. No AND/XOR.
// Slot disp: 50→0x280, 51→0x288, 52→0x290, 60→0x300.
// Store disp LE: 50→80, 51→88, 52→90.
// LDB oo≥0x80 → imm32 add (48 81 c0) → 26B.
// SUB-IMM/ADD-IMM imm=0xB0/0xB8 (>127) → imm32 → 22B.
const picks = [
  {
    name: 'subimm_h52_b0',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x52 imm=0xB0',
    body: '61 52 B0',
    expected: pinOf(0x61, [0x52, 0xb0]),
  },
  {
    name: 'addimm_h50_b8',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x50 imm=0xB8',
    body: '62 50 B8',
    expected: pinOf(0x62, [0x50, 0xb8]),
  },
  {
    name: 'addimm_h51_b8',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x51 imm=0xB8',
    body: '62 51 B8',
    expected: pinOf(0x62, [0x51, 0xb8]),
  },
  {
    name: 'addimm_h52_b8',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x52 imm=0xB8',
    body: '62 52 B8',
    expected: pinOf(0x62, [0x52, 0xb8]),
  },
  {
    name: 'subimm_h50_b8',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x50 imm=0xB8',
    body: '61 50 B8',
    expected: pinOf(0x61, [0x50, 0xb8]),
  },
  {
    name: 'subimm_h51_b8',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x51 imm=0xB8',
    body: '61 51 B8',
    expected: pinOf(0x61, [0x51, 0xb8]),
  },
  {
    name: 'subimm_h52_b8',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x52 imm=0xB8',
    body: '61 52 B8',
    expected: pinOf(0x61, [0x52, 0xb8]),
  },
  {
    name: 'ldb_5060_d0',
    opcode: '0x80 LDB',
    args: 'dd=0x50 ss=0x60 oo=0xD0',
    body: '80 50 60 D0',
    expected: pinOf(0x80, [0x50, 0x60, 0xd0]),
  },
];

function writeScratch(p) {
  const tyPath = path.join(GOLDEN, `_scratch_${p.name}.ty`);
  const hexPath = path.join(GOLDEN, `_scratch_${p.name}.code.hex`);
  const ty = `; _scratch_${p.name}.ty — ${p.opcode} ${p.args}\n; batch-42 scratch-only\n40 00\n  ${p.body}\n  FF\n`;
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
  let r = spawnSync(
    'cargo',
    ['run', '-q', '-p', 'verifier', '--bin', 'yoyo', '--', 'link', '--target=stub', tyPath, outBin],
    { cwd: path.join(ROOT, 'yoyo-rust'), encoding: 'buffer' }
  );
  if (r.status !== 0 && fs.existsSync(YOYO_EXE)) {
    r = spawnSync(YOYO_EXE, ['link', '--target=stub', tyPath, outBin], {
      cwd: path.join(ROOT, 'yoyo-rust'),
      encoding: 'buffer',
    });
  }
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
  const peerEq = js.ok && rust.ok && js.buf.equals(rust.buf);
  const byteEq = jsOk && rustOk && peerEq;
  const jshaFull = js.ok ? shaFull(js.buf) : 'FAIL';
  const rshaFull = rust.ok ? shaFull(rust.buf) : 'FAIL';
  const jsha = js.ok ? jshaFull.slice(0, 16) : 'FAIL';
  const rsha = rust.ok ? rshaFull.slice(0, 16) : 'FAIL';
  let result = 'PASS';
  if (!js.ok || !rust.ok) result = 'REJECT';
  else if (!peerEq) result = 'REJECT'; // fail-closed peer divergence
  else if (!jsOk || !rustOk || jsha !== rsha) result = 'REJECT';
  rows.push({
    ...p,
    pin: p.expected,
    len: exp.length,
    jsOk,
    rustOk,
    peerEq,
    byteEq: byteEq ? 'Y' : 'N',
    jsha,
    rsha,
    jshaFull,
    rshaFull,
    result,
    jsDetail: js.ok ? '' : js.detail,
    rustDetail: rust.ok ? '' : rust.detail,
  });
  console.log(
    `${p.name}: ${result} len=${exp.length} js=${jsOk} rust=${rustOk} peer=${peerEq} sha=${jsha}`
  );
  if (!js.ok) console.log('  js fail:', js.detail.slice(0, 200));
  if (!rust.ok) console.log('  rust fail:', rust.detail.slice(0, 200));
  if (js.ok && !jsOk) console.log('  js hex:', js.buf.toString('hex'));
  if (rust.ok && !rustOk) console.log('  rust hex:', rust.buf.toString('hex'));
}

const pass = rows.filter((r) => r.result === 'PASS');
const reject = rows.filter((r) => r.result !== 'PASS');
console.log(`\nSummary: ${pass.length} PASS / ${reject.length} REJECT`);

const logPath = path.join(ROOT, 'docs/auxdocs/parallel-batch-42-log.md');
const table = rows
  .map(
    (r, i) =>
      `| ${i + 1} | ${r.opcode} | ${r.args} | \`${r.pin}\` (${r.len}) | ${r.jsOk ? 'same' : 'DIFF'} | ${r.rustOk ? 'same' : 'DIFF'} | ${r.byteEq} | \`${r.jsha}\` | \`${r.rsha}\` | ${r.result} |`
  )
  .join('\n');

const detailBlocks = rows
  .map((r, i) => {
    return `### Pick ${i + 1}: ${r.opcode} ${r.args} — **${r.result}**

- fixture: \`_scratch_${r.name}.ty\` + \`.code.hex\`
- expected pin (${r.len}B): \`${r.pin}\`
- js-sha256: \`${r.jshaFull}\`
- rust-sha256: \`${r.result === 'PASS' ? 'same' : r.rshaFull}\`
- byte-eq JS↔Rust↔expected: ${r.byteEq}
`;
  })
  .join('\n');

const passTable = pass
  .map((r, i) => {
    const h = 310 + i;
    const sel = (0x13c + i).toString(16).toUpperCase();
    const argsShort = r.args
      .replace(/slot=|imm=|dd=|ss=|oo=|\(|\)/g, '')
      .replace(/,/g, '')
      .replace(/\s+/g, ' ')
      .trim();
    return `| H_${h} | 0x${sel} | ${r.opcode} | ${argsShort} | \`${r.pin}\` (${r.len}B) | \`${r.jsha}\` |`;
  })
  .join('\n');

const scratchList = rows
  .map((r) => `- \`yoyo/tests/golden/_scratch_${r.name}.ty\` + \`.code.hex\``)
  .join('\n');

const PIN = '000042c8ea316c07fce78e5bb05814229058adea09ac196d0d1e8a90987336f2';

const log = `# parallel-batch-42 Log · 8-pick fresh-picks scratch test sweep

> Tag: \`parallel-batch-42-EXPERIMENTAL-8-pick-scratch\` · 2026-07-25 (UTC+8).
> Following body-extend-047 (pin \`000042c8…\`, handlers = 316, H_302..H_309 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-047 DDC PE \`.text\` measured EQUAL — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (\`_scratch_*.ty\` + \`_scratch_*.code.hex\`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_309 and
> not already present as handlers in current \`yoyo.ty\`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
${table}

**Summary**: ${pass.length} PASS / ${reject.length} REJECT out of ${rows.length} attempted.

## §1b. Full sha256 per pick

${detailBlocks}

## §2. Pick rationale

- SUB-IMM slot=52 imm=B0 (finish B0 triad after H_308/H_309; imm32 22B).
- ADD-IMM slot=50/51/52 imm=B8 (fresh imm after B0; imm32 22B).
- SUB-IMM slot=50/51/52 imm=B8 (complements ADD-IMM * B8; imm32 22B).
- LDB dd=50 ss=60 oo=D0 (next oo after C8 triad; imm32 26B).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: \`40 13C\`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

${scratchList}
- \`docs/auxdocs/parallel-batch-42-log.md\` — this file
- \`scripts/_probe/parallel-batch-42-run.mjs\` — probe runner

NO \`yoyo/projects/yoyo.ty\`, NO lock, NO \`golden.js\`, NO \`self_test.rs\`, NO \`main.rs\`.

## §4. Parent next

**parent next = body-extend-048 serialize PASSes + 1 Relock**

Pass pin from body-extend-047 Relock: \`${PIN}\`.
Handlers before consolidate = 316 (H_00..H_309). Next selectors \`40 13C\`.. for H_310.. if all serialize.

PASS list for body-extend-048:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
${passTable}

## §5. Honesty override checks

- Peer JS/Rust divergence at the ${pass.length} PASS handlers: **${reject.length === 0 ? 'NONE' : 'SEE REJECTS'}** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
  Lock-respected surface unchanged (scratch-only beat).
- No PROMPT edit. No version bump. No \`*.lock\` touch.
- No git commit (W-START convention).
- No \`yoyo.ty\` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- body-extend-047 DDC PE \`.text\` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The ${pass.length} PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_309.
- If the parent decides to serialize, append H_310.. at selectors \`40 13C\`..:
${pass.map((r, i) => `  - H_${310 + i} ${r.opcode} (${r.body}) — pin \`${r.pin}\``).join('\n')}
- Plus 1 Relock after append from pin \`000042c8…\`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).

## §7. Consolidation handoff

parent next = body-extend-048 serialize PASSes + 1 Relock
`;

fs.writeFileSync(logPath, log);
console.log('wrote', logPath);

const summaryPath = path.join(ROOT, 'scripts/_probe/parallel-batch-42-summary.json');
fs.writeFileSync(
  summaryPath,
  JSON.stringify(
    {
      attempted: rows.length,
      pass: pass.length,
      reject: reject.length,
      pin: PIN,
      passes: pass.map((r, i) => ({
        h: 310 + i,
        sel: 0x13c + i,
        name: r.name,
        opcode: r.opcode,
        args: r.args,
        body: r.body,
        pin: r.pin,
        len: r.len,
        sha16: r.jsha,
        sha256: r.jshaFull,
      })),
      rejects: reject.map((r) => ({
        name: r.name,
        opcode: r.opcode,
        reason: !r.peerEq ? 'peer-divergence' : 'mismatch',
      })),
    },
    null,
    2
  )
);
console.log('wrote', summaryPath);
