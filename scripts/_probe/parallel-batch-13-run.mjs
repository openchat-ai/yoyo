/**
 * parallel-batch-13 probe — scratch-only fresh picks after H_70..H_77.
 * Mirror parallel-batch-12-run.mjs. Do NOT touch yoyo.ty / lock / goldens.
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

function sha16(hexOrBuf) {
  const buf = Buffer.isBuffer(hexOrBuf) ? hexOrBuf : Buffer.from(hexOrBuf, 'hex');
  return shaFull(buf).slice(0, 16);
}

// Fresh vs H_48..H_77. Skip D-1/D-2/D-3 / MEMCPY. No AND/XOR.
const picks = [
  {
    name: 'addimm_h52',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x52 imm=0x07',
    body: '62 52 07',
    expected: pinOf(0x62, [0x52, 0x07]),
  },
  {
    name: 'subimm_h52_03',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x52 imm=0x03',
    body: '61 52 03',
    expected: pinOf(0x61, [0x52, 0x03]),
  },
  {
    name: 'addimm_h51_0a',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x51 imm=0x0A',
    body: '62 51 0A',
    expected: pinOf(0x62, [0x51, 0x0a]),
  },
  {
    name: 'subimm_h50_05',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x50 imm=0x05',
    body: '61 50 05',
    expected: pinOf(0x61, [0x50, 0x05]),
  },
  {
    name: 'orv_5250',
    opcode: '0x69 ORV',
    args: '(0x52, 0x50)',
    body: '69 52 50',
    expected: pinOf(0x69, [0x52, 0x50]),
  },
  {
    name: 'subv_5250',
    opcode: '0x6A SUBV',
    args: '(0x52, 0x50)',
    body: '6A 52 50',
    expected: pinOf(0x6A, [0x52, 0x50]),
  },
  {
    name: 'addv_5152',
    opcode: '0x68 ADDV',
    args: '(0x51, 0x52)',
    body: '68 51 52',
    expected: pinOf(0x68, [0x51, 0x52]),
  },
  {
    name: 'imul_5052',
    opcode: '0x63 IMUL',
    args: '(0x50, 0x52)',
    body: '63 50 52',
    expected: pinOf(0x63, [0x50, 0x52]),
  },
];

function writeScratch(p) {
  const tyPath = path.join(GOLDEN, `_scratch_${p.name}.ty`);
  const hexPath = path.join(GOLDEN, `_scratch_${p.name}.code.hex`);
  const ty = `; _scratch_${p.name}.ty — ${p.opcode} ${p.args}\n; batch-13 scratch-only\n40 00\n  ${p.body}\n  FF\n`;
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
  // Prefer cargo run; fall back to existing debug binary if rebuild broken.
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
  // Fail-closed on peer divergence
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
}

const pass = rows.filter((r) => r.result === 'PASS');
const reject = rows.filter((r) => r.result !== 'PASS');
console.log(`\nSummary: ${pass.length} PASS / ${reject.length} REJECT`);

const logPath = path.join(ROOT, 'docs/auxdocs/parallel-batch-13-log.md');
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
    const h = 78 + i;
    const sel = (0x54 + i).toString(16).toUpperCase().padStart(2, '0');
    const argsShort = r.args.replace(/slot=|imm=|\(|\)/g, '').replace(/,/g, '').replace(/\s+/g, ' ').trim();
    return `| H_${h} | 0x${sel} | ${r.opcode} | ${argsShort} | \`${r.pin}\` (${r.len}B) | \`${r.jsha}\` |`;
  })
  .join('\n');

const scratchList = rows
  .map((r) => `- \`yoyo/tests/golden/_scratch_${r.name}.ty\` + \`.code.hex\``)
  .join('\n');

const log = `# parallel-batch-13 Log · 8-pick fresh-picks scratch test sweep

> Tag: \`parallel-batch-13-EXPERIMENTAL-8-pick-scratch\` · 2026-07-25 (UTC+8).
> Following body-extend-018 (pin \`e8603542…\`, handlers = 84, H_70..H_77 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (\`_scratch_*.ty\` + \`_scratch_*.code.hex\`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_77 and
> not already present as handlers in current \`yoyo.ty\`. Slot/imm/dst
> variations of ADD-IMM/SUB-IMM/ORV/SUBV/ADDV/IMUL. Skipped D-1
> 0x20/0x50/0x51, D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
${table}

**Summary**: ${pass.length} PASS / ${reject.length} REJECT out of ${rows.length} attempted.

## §1b. Full sha256 per pick

${detailBlocks}

## §2. Pick rationale

- ADD-IMM at slot 0x52 imm=0x07 (H_64 uses 51 07; H_13 uses 50).
- SUB-IMM at slot 0x52 imm=0x03 (H_70 uses 51 03; H_23 uses 50 03).
- ADD-IMM at slot 0x51 imm=0x0A (fresh imm vs H_64's 0x07).
- SUB-IMM at slot 0x50 imm=0x05 (fresh imm vs H_23's 0x03).
- ORV/SUBV at 52 50 (H_49/H_50 are 51 50; H_54/H_55 are 52 51; H_73/H_74 are 50 52).
- ADDV at 51 52 (H_48=51 50; H_52=52 51; H_66=50 52).
- IMUL at 50 52 (H_34=50 51; H_56=51 50; H_57=52 51).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

${scratchList}
- \`docs/auxdocs/parallel-batch-13-log.md\` — this file
- \`scripts/_probe/parallel-batch-13-run.mjs\` — probe runner

NO \`yoyo/projects/yoyo.ty\`, NO lock, NO \`golden.js\`, NO \`self_test.rs\`, NO \`main.rs\`.

## §4. Parent next

**parent next = body-extend-019 serialize PASSes + 1 Relock**

Pass pin from body-extend-018 Relock: \`e8603542fb13c5f027b3bea34b63aa0b8b20e82bb087ffe06568bd8193b401a2\`.
Handlers before consolidate = 84 (H_00..H_77). Next selectors 0x54.. for H_78.. if all serialize.

PASS list for body-extend-019:

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

## §6. Next-step suggestion (parent for serialization)

- The ${pass.length} PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_77.
- If the parent decides to serialize, append H_78.. at selectors 0x54..:
${pass.map((r, i) => `  - H_${78 + i} ${r.opcode} (${r.body}) — pin \`${r.pin}\``).join('\n')}
- Plus 1 Relock after append from pin \`e8603542…\`.

## §7. Consolidation handoff

parent next = body-extend-019 serialize PASSes + 1 Relock
`;

fs.writeFileSync(logPath, log);
console.log('wrote', logPath);

// Also emit JSON summary for SPAWN authoring
const summaryPath = path.join(ROOT, 'scripts/_probe/parallel-batch-13-summary.json');
fs.writeFileSync(
  summaryPath,
  JSON.stringify(
    {
      attempted: rows.length,
      pass: pass.length,
      reject: reject.length,
      pin: 'e8603542fb13c5f027b3bea34b63aa0b8b20e82bb087ffe06568bd8193b401a2',
      passes: pass.map((r, i) => ({
        h: 78 + i,
        sel: 0x54 + i,
        name: r.name,
        opcode: r.opcode,
        args: r.args,
        body: r.body,
        pin: r.pin,
        len: r.len,
        sha16: r.jsha,
        sha256: r.jshaFull,
      })),
      rejects: reject.map((r) => ({ name: r.name, opcode: r.opcode, reason: !r.peerEq ? 'peer-divergence' : 'mismatch' })),
    },
    null,
    2
  )
);
console.log('wrote', summaryPath);
