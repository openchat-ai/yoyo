/**
 * parallel-batch-97 probe — matrix-priority picks (P1 focus).
 * Strategy: fills selfhost-emit-matrix P1 remaining rows (YES+MISSING/PARTIAL).
 * Skips MEMCPY (P0) — requires real impl, not scratch bytes.
 * Uses parallel-batch-scratch-lib (≤8 concurrent). Do NOT touch yoyo.ty / lock.
 *
 * Matrix remaining YES+MISSING/PARTIAL (12 rows):
 *   - 84 MEMCPY_DATA / 85 MEMCPY_STATE: PARTIAL (stub=C3) — SKIP this batch
 *   - 68 ADDV / 6A SUBV / 69 ORV / 63 IMUL: multi-combo P1
 *   - 65 CMP: multi-slot P1 (1 more, after H_748)
 *   - 66 INC / 67 DEC: multi-slot P1 (only 3 handlers each)
 *
 * Prior locked: H_00..H_748 (755 handlers).
 * Next selectors: 0x2F3..0x2FA for H_749..H_756.
 *
 * Slot disp: n → n*8 LE. 50→0x280, 51→0x288, 52→0x290, 60→0x300, 61→0x308, 62→0x310.
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
  // -- P1: 68 ADDV multi-combo — fresh dst/src pairs (25B each) --
  // 6 permutations of 3 slots: 6 already exist. Pick 2 new combos:
  {
    name: 'addv_52_50',
    opcode: '0x68 ADDV',
    args: 'dst=0x52 src=0x50',
    body: '68 52 50',
    expected: pinOf(0x68, [0x52, 0x50]),
  },
  {
    name: 'addv_50_51',
    opcode: '0x68 ADDV',
    args: 'dst=0x50 src=0x51',
    body: '68 50 51',
    expected: pinOf(0x68, [0x50, 0x51]),
  },
  // -- P1: 6A SUBV multi-combo --
  {
    name: 'subv_60_61',
    opcode: '0x6A SUBV',
    args: 'dst=0x60 src=0x61',
    body: '6A 60 61',
    expected: pinOf(0x6a, [0x60, 0x61]),
  },
  {
    name: 'subv_61_62',
    opcode: '0x6A SUBV',
    args: 'dst=0x61 src=0x62',
    body: '6A 61 62',
    expected: pinOf(0x6a, [0x61, 0x62]),
  },
  // -- P1: 65 CMP multi-slot — after H_748 (a=0x60 b=0x52) --
  {
    name: 'cmp_62_60',
    opcode: '0x65 CMP',
    args: 'a=0x62 b=0x60',
    body: '65 62 60',
    expected: pinOf(0x65, [0x62, 0x60]),
  },
  // -- P1: 66 INC multi-slot (only 3 handlers) --
  {
    name: 'inc_60',
    opcode: '0x66 INC',
    args: 'slot=0x60',
    body: '66 60',
    expected: pinOf(0x66, [0x60]),
  },
  {
    name: 'inc_61',
    opcode: '0x66 INC',
    args: 'slot=0x61',
    body: '66 61',
    expected: pinOf(0x66, [0x61]),
  },
  // -- P1: 67 DEC multi-slot (only 3 handlers) --
  {
    name: 'dec_60',
    opcode: '0x67 DEC',
    args: 'slot=0x60',
    body: '67 60',
    expected: pinOf(0x67, [0x60]),
  },
];

const rows = await runScratchPicks(picks, {
  concurrency: MAX_SCRATCH_WORKERS,
  batchTag: 'batch-97-matrix',
});

const pass = rows.filter((r) => r.result === 'PASS');
const reject = rows.filter((r) => r.result !== 'PASS');
console.log(`\nSummary: ${pass.length} PASS / ${reject.length} REJECT`);

const logPath = path.join(ROOT, 'docs/auxdocs/parallel-batch-97-log.md');
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

const priority = (name) => {
  if (name.startsWith('addv') || name.startsWith('subv')) return 'P1';
  if (name.startsWith('cmp') || name.startsWith('inc') || name.startsWith('dec')) return 'P1';
  return 'P1';
};

const log = `# parallel-batch-97 Log · 8-pick matrix-priority scratch sweep (P1)

> Tag: \`parallel-batch-97-EXPERIMENTAL-8-pick-matrix-p1\` · 2026-07-26 (UTC+8).
> Following body-extend-102 (pin \`6532ea809c58c7a9…\`, handlers = 755, H_00..H_748 locked).
> W-START: **EXPERIMENTAL · NON-GREEN**.
>
> Picks from \`docs/auxdocs/selfhost-emit-matrix.md\` priority gaps (P1).
> MEMCPY (P0) skipped — requires real impl, not scratch bytes.
> 8 picks: 2 ADDV multi-combo + 2 SUBV multi-combo + 1 CMP multi-slot + 2 INC multi-slot + 1 DEC multi-slot.
> Next selectors after 0x2F2: \`40 2F3\`..

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
${table}

**Summary**: ${pass.length} PASS / ${reject.length} REJECT out of ${rows.length} attempted.

## §1b. Full sha256 per pick

${detailBlocks}

## §2. Pick rationale (matrix trace)

- **P1 · 68 ADDV multi-combo**: dst=52 src=50 and dst=50 src=51; fresh slot permutations for self-host arithmetic loops.
- **P1 · 6A SUBV multi-combo**: dst=60 src=61 and dst=61 src=62; fresh high-slot subtraction pairs.
- **P1 · 65 CMP multi-slot**: a=62 b=60; extends after H_748 (a=60 b=52) for Jcc self-host condition coverage.
- **P1 · 66 INC multi-slot**: slot=60 and slot=61; existing handlers only on 50/51/52; extends loop-counter range.
- **P1 · 67 DEC multi-slot**: slot=60; extends loop-counter decrement range.
- MEMCPY_DATA/STATE (P0) skipped — real implementation required before true DONE; not scratch bytes.
- No D-1 0x20/0x50/0x51 body use as primary opcodes. No D-2 0x64.
- yoyo.ty unchanged this beat.

## §3. Files touched

${scratchList}
- \`docs/auxdocs/parallel-batch-97-log.md\` — this file
- \`scripts/_probe/parallel-batch-97-run.mjs\` — probe runner (uses shared concurrent lib)
- \`scripts/_probe/parallel-batch-scratch-lib.mjs\` — ≤${MAX_SCRATCH_WORKERS} scratch workers

NO \`yoyo/projects/yoyo.ty\`, NO lock, NO \`golden.js\`, NO \`self_test.rs\`, NO \`main.rs\`.

## §4. Parent next

**parent next = body-extend-103 serialize PASSes + 1 Relock** (consolidator)

Pass pin from body-extend-102 Relock: \`6532ea809c58c7a9\` (abbrev).
Handlers before consolidate = 755 (H_00..H_748). Next selectors \`40 2F3\`.. for H_749.. if all serialize.

PASS list for body-extend-103:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
${pass.map((r, i) => {
  const h = 749 + i;
  const sel = (0x2f3 + i).toString(16).toUpperCase();
  const argsShort = r.args.replace(/slot=|imm=|dst=|src=|a=|b=|n=/g, '').replace(/\s+/g, ' ').trim();
  return `| H_${h} | 0x${sel} | ${r.opcode} | ${argsShort} | \`${r.pin}\` (${r.len}B) | \`${r.jsha}\` |`;
}).join('\n')}

## §5. Honesty override checks

- Peer JS/Rust divergence at the ${pass.length} PASS handlers: **${reject.length === 0 ? 'NONE' : 'SEE REJECTS'}** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
- No PROMPT edit. No version bump. No \`*.lock\` touch.
- No git commit (W-START convention).
- No \`yoyo.ty\` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.

## §6. Consolidation handoff

parent next = body-extend-103 serialize PASSes + 1 Relock
`;

fs.writeFileSync(logPath, log);
console.log('wrote', logPath);

const summaryPath = path.join(ROOT, 'scripts/_probe/parallel-batch-97-summary.json');
fs.writeFileSync(
  summaryPath,
  JSON.stringify(
    {
      attempted: rows.length,
      pass: pass.length,
      reject: reject.length,
      pin: '6532ea809c58c7a9',
      concurrency: MAX_SCRATCH_WORKERS,
      matrixPriority: true,
      passes: pass.map((r, i) => ({
        h: 749 + i,
        sel: 0x2f3 + i,
        name: r.name,
        opcode: r.opcode,
        args: r.args,
        body: r.body,
        pin: r.pin,
        len: r.len,
        sha16: r.jsha,
        sha256: r.jshaFull,
        matrixPriority: priority(r.name),
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
