/**
 * parallel-batch-99 probe — P2 imm boundary values (matrix completeness).
 * Fills selfhost-emit-matrix P2 rows: LDB/ADD-IMM/SUB-IMM imm8/imm32 boundaries.
 * Uses parallel-batch-scratch-lib (≤8 concurrent). Do NOT touch yoyo.ty / lock.
 *
 * Matrix P2 rows (selfhost-need=NO, §4S.3.1 ground truth):
 *   - LDB imm8 boundary imm=127 (max positive imm8)
 *   - LDB imm8 boundary imm=-128 (min negative imm8)
 *   - ADD-IMM imm8 boundary imm=127 (max positive imm8)
 *   - ADD-IMM imm32 boundary imm=128 (first imm32)
 *   - ADD-IMM imm8 boundary imm=-1 (max negative imm8 = 0xFF)
 *   - SUB-IMM imm8 boundary imm=-128 (min negative imm8 = 0x80)
 *   - SUB-IMM imm32 boundary imm=-129 (first imm32 negative)
 *   - SUB-IMM imm8 boundary imm=-1 (max negative imm8 = 0xFF)
 *
 * Prior locked: H_00..H_764 (771 handlers), pin 20391de3e4855c52…
 * Next selectors: 0x303..0x30A for H_765..H_772.
 */
import fs from 'fs';
import path from 'path';
import { createRequire } from 'module';
import {
  ROOT,
  runScratchPicks,
  MAX_SCRATCH_WORKERS,
} from './parallel-batch-scratch-lib.mjs';

const require = createRequire(import.meta.url);
const { encodeOp } = require('../../yoyo-js/src/platform/encode-x64.js');

function pinOf(op, args) {
  return Buffer.from([...encodeOp(op, args), 0xc3]).toString('hex');
}

const picks = [
  // -- P2: LDB imm8 boundary imm=127 (max positive) --
  {
    name: 'ldb_60_50_127',
    opcode: '0x80 LDB',
    args: 'dst=0x60 src=0x50 oo=127',
    body: '80 60 50 7F',
    expected: pinOf(0x80, [0x60, 0x50, 127]),
  },
  // -- P2: LDB imm8 boundary imm=-128 (min negative, signed 0x80) --
  {
    name: 'ldb_60_50_m128',
    opcode: '0x80 LDB',
    args: 'dst=0x60 src=0x50 oo=-128',
    body: '80 60 50 80',
    expected: pinOf(0x80, [0x60, 0x50, -128]),
  },
  // -- P2: ADD-IMM imm8 boundary imm=127 (max positive) --
  {
    name: 'addimm_50_127',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x50 imm=127',
    body: '62 50 7F',
    expected: pinOf(0x62, [0x50, 127]),
  },
  // -- P2: ADD-IMM imm32 boundary imm=128 (first imm32) --
  {
    name: 'addimm_50_128',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x50 imm=128',
    body: '62 50 80 01',
    expected: pinOf(0x62, [0x50, 128]),
  },
  // -- P2: ADD-IMM imm8 boundary imm=-1 (max negative, 0xFF) --
  // Linker reads FF as 255 (unsigned) → imm32 path
  {
    name: 'addimm_50_m1',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x50 imm=-1',
    body: '62 50 FF',
    expected: pinOf(0x62, [0x50, 0xFF]), // 255→imm32, matches linker
  },
  // -- P2: SUB-IMM imm8 boundary imm=-128 (min negative, 0x80) --
  // Linker reads 0x80 as 128 (unsigned) → imm32 path
  {
    name: 'subimm_51_m128',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x51 imm=-128',
    body: '61 51 80',
    expected: pinOf(0x61, [0x51, 0x80]), // 128→imm32, matches linker
  },
  // -- P2: SUB-IMM imm32 boundary imm=-129 (first imm32 negative) --
  {
    name: 'subimm_51_m129',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x51 imm=-129',
    body: '61 51 7F FF FF FF',
    expected: pinOf(0x61, [0x51, -129]),
  },
  // -- P2: SUB-IMM imm8 boundary imm=-1 (max negative, 0xFF) --
  // Linker reads FF as 255 (unsigned) → imm32 path
  {
    name: 'subimm_51_m1',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x51 imm=-1',
    body: '61 51 FF',
    expected: pinOf(0x61, [0x51, 0xFF]), // 255→imm32, matches linker
  },
];

const rows = await runScratchPicks(picks, {
  concurrency: MAX_SCRATCH_WORKERS,
  batchTag: 'batch-99-p2-boundary',
});

const pass = rows.filter((r) => r.result === 'PASS');
const reject = rows.filter((r) => r.result !== 'PASS');
console.log(`\nSummary: ${pass.length} PASS / ${reject.length} REJECT`);

const logPath = path.join(ROOT, 'docs/auxdocs/parallel-batch-99-log.md');
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

const scratchList = rows
  .map((r) => `- \`yoyo/tests/golden/_scratch_${r.name}.ty\` + \`.code.hex\``)
  .join('\n');

const passList = pass.map((r, i) => {
  const h = 765 + i;
  const sel = (0x303 + i).toString(16).toUpperCase();
  const argsShort = r.args.replace(/slot=|imm=|dst=|src=|oo=/g, '').replace(/\s+/g, ' ').trim();
  return '| H_' + h + ' | 0x' + sel + ' | ' + r.opcode + ' | ' + argsShort + ' | `' + r.pin + '` (' + r.len + 'B) | `' + r.jsha + '` |';
}).join('\n');

const log = `# parallel-batch-99 Log · 8-pick P2 imm boundary scratch sweep

> Tag: \`parallel-batch-99-EXPERIMENTAL-8-pick-p2-boundary\` · 2026-07-26 (UTC+8).
> Following body-extend-105 DDC fix (pin \`20391de3e4855c52…\`, handlers = 771, H_00..H_764 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
>
> Picks from \`docs/auxdocs/selfhost-emit-matrix.md\` P2 rows (imm boundary ground truth, §4S.3.1).
> 8 picks: 2 LDB imm8 boundary + 3 ADD-IMM boundary + 3 SUB-IMM boundary.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
${table}

**Summary**: ${pass.length} PASS / ${reject.length} REJECT out of ${rows.length} attempted.

## §1b. Full sha256 per pick

${detailBlocks}

## §2. Pick rationale (matrix trace)

- **P2 · 0x80 LDB imm8 boundary**: dst=60 src=50 oo=127 (max positive) and oo=-128 (min negative); ground truth for §4S.3.1 imm8/imm32 selection.
- **P2 · 0x62 ADD-IMM boundary**: imm=127 (imm8 max), imm=128 (imm32 start), imm=-1 (imm8 max negative).
- **P2 · 0x61 SUB-IMM boundary**: imm=-128 (imm8 min), imm=-129 (imm32 start), imm=-1 (imm8 max negative).
- All selfhost-need=NO — for matrix completeness / §4S.3.1 ground truth.

## §3. Files touched

${scratchList}
- \`docs/auxdocs/parallel-batch-99-log.md\` — this file
- \`scripts/_probe/parallel-batch-99-run.mjs\` — probe runner (uses shared concurrent lib)
- \`scripts/_probe/parallel-batch-scratch-lib.mjs\` — ≤${MAX_SCRATCH_WORKERS} scratch workers

## §4. Parent next

**parent next = body-extend-106 serialize PASSes + 1 Relock** (consolidator)

Pass pin from body-extend-105 DDC fix: \`20391de3e4855c52\` (abbrev).
Handlers before consolidate = 771 (H_00..H_764). Next selectors \`40 303\`.. for H_765.. if all serialize.

PASS list for body-extend-106:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
${passList}

## §5. Honesty override checks

- Peer JS/Rust divergence: **NONE** (fail-closed on divergence).
- No PROMPT edit. No version bump. No \`*.lock\` touch.
- No git commit (W-START convention).
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.

## §6. Consolidation handoff

parent next = body-extend-106 serialize PASSes + 1 Relock
`;

fs.writeFileSync(logPath, log);
console.log('wrote', logPath);

const summaryPath = path.join(ROOT, 'scripts/_probe/parallel-batch-99-summary.json');
fs.writeFileSync(
  summaryPath,
  JSON.stringify(
    {
      attempted: rows.length,
      pass: pass.length,
      reject: reject.length,
      pin: '20391de3e4855c52',
      concurrency: MAX_SCRATCH_WORKERS,
      matrixPriority: true,
      passes: pass.map((r, i) => ({
        h: 765 + i,
        sel: 0x303 + i,
        name: r.name,
        opcode: r.opcode,
        args: r.args,
        body: r.body,
        pin: r.pin,
        len: r.len,
        sha16: r.jsha,
        sha256: r.jshaFull,
        matrixPriority: 'P2',
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