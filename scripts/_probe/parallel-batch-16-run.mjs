/**
 * parallel-batch-16 probe — scratch-only fresh picks after H_94..H_101.
 * Mirror parallel-batch-15-run.mjs. Do NOT touch yoyo.ty / lock / goldens.
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

// Fresh vs H_48..H_101. Skip early-dup INC/DEC 50, GET/ORV/SUBV 50 51.
// Skip D-1/D-2/D-3 / MEMCPY. No AND/XOR.
const picks = [
  {
    name: 'cmp_5152',
    opcode: '0x65 CMP',
    args: '(0x51, 0x52)',
    body: '65 51 52',
    expected: pinOf(0x65, [0x51, 0x52]),
  },
  {
    name: 'ldb_5160_18',
    opcode: '0x80 LDB',
    args: 'dd=0x51 ss=0x60 oo=0x18',
    body: '80 51 60 18',
    expected: pinOf(0x80, [0x51, 0x60, 0x18]),
  },
  {
    name: 'ldb_5260_18',
    opcode: '0x80 LDB',
    args: 'dd=0x52 ss=0x60 oo=0x18',
    body: '80 52 60 18',
    expected: pinOf(0x80, [0x52, 0x60, 0x18]),
  },
  {
    name: 'set_c0ffee00',
    opcode: '0x30 SET',
    args: 'slot=0x51 imm=0xC0FFEE00',
    body: '30 51 C0FFEE00',
    expected: pinOf(0x30, [0x51, 0xc0ffee00]),
  },
  {
    name: 'subimm_h52_08',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x52 imm=0x08',
    body: '61 52 08',
    expected: pinOf(0x61, [0x52, 0x08]),
  },
  {
    name: 'imul_5152',
    opcode: '0x63 IMUL',
    args: '(0x51, 0x52)',
    body: '63 51 52',
    expected: pinOf(0x63, [0x51, 0x52]),
  },
  {
    name: 'addimm_h50_14',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x50 imm=0x14',
    body: '62 50 14',
    expected: pinOf(0x62, [0x50, 0x14]),
  },
  {
    name: 'set_50_c0ffee00',
    opcode: '0x30 SET',
    args: 'slot=0x50 imm=0xC0FFEE00',
    body: '30 50 C0FFEE00',
    expected: pinOf(0x30, [0x50, 0xc0ffee00]),
  },
];

function writeScratch(p) {
  const tyPath = path.join(GOLDEN, `_scratch_${p.name}.ty`);
  const hexPath = path.join(GOLDEN, `_scratch_${p.name}.code.hex`);
  const ty = `; _scratch_${p.name}.ty — ${p.opcode} ${p.args}\n; batch-16 scratch-only\n40 00\n  ${p.body}\n  FF\n`;
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

const logPath = path.join(ROOT, 'docs/auxdocs/parallel-batch-16-log.md');
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
    const h = 102 + i;
    const sel = (0x6c + i).toString(16).toUpperCase().padStart(2, '0');
    const argsShort = r.args.replace(/slot=|imm=|dd=|ss=|oo=|\(|\)/g, '').replace(/,/g, '').replace(/\s+/g, ' ').trim();
    return `| H_${h} | 0x${sel} | ${r.opcode} | ${argsShort} | \`${r.pin}\` (${r.len}B) | \`${r.jsha}\` |`;
  })
  .join('\n');

const scratchList = rows
  .map((r) => `- \`yoyo/tests/golden/_scratch_${r.name}.ty\` + \`.code.hex\``)
  .join('\n');

const PIN = '07eee98cb95446f2c277bcd78b211e43e7b274f583ac02392691dfc1b204cd0a';

const log = `# parallel-batch-16 Log · 8-pick fresh-picks scratch test sweep

> Tag: \`parallel-batch-16-EXPERIMENTAL-8-pick-scratch\` · 2026-07-25 (UTC+8).
> Following body-extend-021 (pin \`07eee98c…\`, handlers = 108, H_94..H_101 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (\`_scratch_*.ty\` + \`_scratch_*.code.hex\`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_101 and
> not already present as handlers in current \`yoyo.ty\` (skipped early
> INC/DEC 50, GET/ORV/SUBV 50 51). Slot/imm/dst variations of
> CMP/LDB/SET/SUB-IMM/IMUL/ADD-IMM. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
${table}

**Summary**: ${pass.length} PASS / ${reject.length} REJECT out of ${rows.length} attempted.

## §1b. Full sha256 per pick

${detailBlocks}

## §2. Pick rationale

- CMP at 51 52 (H_58=51 50; H_65=52 51; H_77=52 50; H_89=50 52).
- LDB dd=51 ss=60 oo=18 (H_61=51 60 08; H_90=51 60 10; H_99=50 60 18).
- LDB dd=52 ss=60 oo=18 (H_69=52 60 08; H_98=52 60 10).
- SET at slot 51 imm=C0FFEE00 (H_60 DEADBEEF; H_87 AABBCCDD).
- SUB-IMM at slot 52 imm=08 (H_79=52 03; H_96=50 08; H_70=51 03).
- IMUL at 51 52 (H_56=51 50; H_57=52 51; H_85=50 52; H_91=52 50).
- ADD-IMM at slot 50 imm=14 (H_93=50 0F; H_64=51 07; H_78=52 07; H_80=51 0A; H_97=52 0A).
- SET at slot 50 imm=C0FFEE00 (H_68/H_76/H_94 other imm at 50; replaces early-dup ORV/SUBV 50 51).
- Skipped suggested INC/DEC 50 (H_17/H_18), GET/ORV/SUBV 50 51 (early handlers).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.

## §3. Files touched

${scratchList}
- \`docs/auxdocs/parallel-batch-16-log.md\` — this file
- \`scripts/_probe/parallel-batch-16-run.mjs\` — probe runner

NO \`yoyo/projects/yoyo.ty\`, NO lock, NO \`golden.js\`, NO \`self_test.rs\`, NO \`main.rs\`.

## §4. Parent next

**parent next = body-extend-022 serialize PASSes + 1 Relock**

Pass pin from body-extend-021 Relock: \`${PIN}\`.
Handlers before consolidate = 108 (H_00..H_101). Next selectors 0x6C.. for H_102.. if all serialize.

PASS list for body-extend-022:

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
  fresh slot/imm/dst combinations not in H_48..H_101.
- If the parent decides to serialize, append H_102.. at selectors 0x6C..:
${pass.map((r, i) => `  - H_${102 + i} ${r.opcode} (${r.body}) — pin \`${r.pin}\``).join('\n')}
- Plus 1 Relock after append from pin \`07eee98c…\`.

## §7. Consolidation handoff

parent next = body-extend-022 serialize PASSes + 1 Relock
`;

fs.writeFileSync(logPath, log);
console.log('wrote', logPath);

const summaryPath = path.join(ROOT, 'scripts/_probe/parallel-batch-16-summary.json');
fs.writeFileSync(
  summaryPath,
  JSON.stringify(
    {
      attempted: rows.length,
      pass: pass.length,
      reject: reject.length,
      pin: PIN,
      passes: pass.map((r, i) => ({
        h: 102 + i,
        sel: 0x6c + i,
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
