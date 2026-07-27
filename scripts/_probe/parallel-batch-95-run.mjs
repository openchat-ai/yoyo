/**
 * parallel-batch-95 probe — scratch-only fresh picks after H_725..H_732.
 * Template for batch 96+: use parallel-batch-scratch-lib (≤8 concurrent).
 * Do NOT touch yoyo.ty / lock / goldens. Relock stays consolidator-serial.
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

// Fresh vs H_48..H_732. Skip D-1/D-2/D-3 / MEMCPY. No AND/XOR.
// Slot disp: 50→0x280, 51→0x288, 52→0x290, 60→0x300.
// Store disp LE: 50→80, 51→88, 52→90.
// Finish deferred SUB 52 230; start 232 LDB + ADD + SUB ladders.
const picks = [
  {
    name: 'subimm_h52_230',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x52 imm=0x230',
    body: '61 52 230',
    expected: pinOf(0x61, [0x52, 0x230]),
  },
  {
    name: 'ldb_5060_232',
    opcode: '0x80 LDB',
    args: 'dd=0x50 ss=0x60 oo=0x232',
    body: '80 50 60 232',
    expected: pinOf(0x80, [0x50, 0x60, 0x232]),
  },
  {
    name: 'ldb_5160_232',
    opcode: '0x80 LDB',
    args: 'dd=0x51 ss=0x60 oo=0x232',
    body: '80 51 60 232',
    expected: pinOf(0x80, [0x51, 0x60, 0x232]),
  },
  {
    name: 'ldb_5260_232',
    opcode: '0x80 LDB',
    args: 'dd=0x52 ss=0x60 oo=0x232',
    body: '80 52 60 232',
    expected: pinOf(0x80, [0x52, 0x60, 0x232]),
  },
  {
    name: 'addimm_h50_232',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x50 imm=0x232',
    body: '62 50 232',
    expected: pinOf(0x62, [0x50, 0x232]),
  },
  {
    name: 'addimm_h51_232',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x51 imm=0x232',
    body: '62 51 232',
    expected: pinOf(0x62, [0x51, 0x232]),
  },
  {
    name: 'addimm_h52_232',
    opcode: '0x62 ADD-IMM',
    args: 'slot=0x52 imm=0x232',
    body: '62 52 232',
    expected: pinOf(0x62, [0x52, 0x232]),
  },
  {
    name: 'subimm_h50_232',
    opcode: '0x61 SUB-IMM',
    args: 'slot=0x50 imm=0x232',
    body: '61 50 232',
    expected: pinOf(0x61, [0x50, 0x232]),
  },
];

const rows = await runScratchPicks(picks, {
  concurrency: MAX_SCRATCH_WORKERS,
  batchTag: 'batch-95',
});

const pass = rows.filter((r) => r.result === 'PASS');
const reject = rows.filter((r) => r.result !== 'PASS');
console.log(`\nSummary: ${pass.length} PASS / ${reject.length} REJECT`);

const logPath = path.join(ROOT, 'docs/auxdocs/parallel-batch-95-log.md');
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

const H0 = 733;
const SEL0 = 0x2e3;
const PIN = '7c07906496a7af9cbaec74b5590ec3677117ced6c36241823bd69b6a4ff1ae51';

const passTable = pass
  .map((r, i) => {
    const h = H0 + i;
    const sel = (SEL0 + i).toString(16).toUpperCase();
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

const deferredNote =
  'SUB-IMM 51/52 232 (finish 232 SUB triad); SET/GET/ORV/SUBV/ADDV/IMUL fresh; next ladder if continuing.';

const log = `# parallel-batch-95 Log · 8-pick fresh-picks scratch test sweep

> Tag: \`parallel-batch-95-EXPERIMENTAL-8-pick-scratch\` · 2026-07-26 (UTC+8).
> Following body-extend-100 (pin \`7c079064…\`, handlers = 739, H_725..H_732 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (body-extend-100 DDC PE \`.text\` measured EQUAL this beat — still do not invent-green).
> Parent-driven 8-pick scratch sweep; each writes its own scratch
> fixture (\`_scratch_*.ty\` + \`_scratch_*.code.hex\`). This log aggregates
> the per-handler results below.
> All picks below are FRESH — not duplicate of H_48–H_732 and
> not already present as handlers in current \`yoyo.ty\`.
> Slot/imm/dst variations of ADD-IMM/SUB-IMM/LDB. Skipped D-1 0x20/0x50/0x51,
> D-2 0x64, D-3 0x84/0x85. No AND/XOR.
> Runner: \`parallel-batch-scratch-lib.mjs\` pool ≤${MAX_SCRATCH_WORKERS} (no Relock).

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
${table}

**Summary**: ${pass.length} PASS / ${reject.length} REJECT out of ${rows.length} attempted.

## §1b. Full sha256 per pick

${detailBlocks}

## §2. Pick rationale

- SUB-IMM slot=52 imm=230 (finish deferred 230 SUB triad; imm32 22B).
- LDB dd=50/51/52 ss=60 oo=232 (start 232 LDB ladder; imm32 26B).
- ADD-IMM slot=50/51/52 imm=232 (start 232 ADD triad; imm32 22B).
- SUB-IMM slot=50 imm=232 (start 232 SUB triad; imm32 22B; SUB 51/52 deferred).
- No MEMCPY / D-1 / D-2. yoyo.ty unchanged this beat.
- Next HANDLER selectors after consolidate: \`40 2E3\`.. (label-width A landed; multi-digit hex OK).

## §3. Files touched

${scratchList}
- \`docs/auxdocs/parallel-batch-95-log.md\` — this file
- \`scripts/_probe/parallel-batch-95-run.mjs\` — probe runner (uses shared concurrent lib)
- \`scripts/_probe/parallel-batch-scratch-lib.mjs\` — ≤${MAX_SCRATCH_WORKERS} scratch workers

NO \`yoyo/projects/yoyo.ty\`, NO lock, NO \`golden.js\`, NO \`self_test.rs\`, NO \`main.rs\`.

## §4. Parent next

**parent next = body-extend-101 serialize PASSes + 1 Relock**

Pass pin from body-extend-100 Relock: \`${PIN}\`.
Handlers before consolidate = 739 (H_00..H_732). Next selectors \`40 2E3\`.. for H_733.. if all serialize.

PASS list for body-extend-101:

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
- body-extend-100 DDC PE \`.text\` EQUAL noted — still EXPERIMENTAL · NON-GREEN; no invent-green.

## §6. Next-step suggestion (parent for serialization)

- The ${pass.length} PASS handlers exercise well-tested primitive paths with
  fresh slot/imm/dst combinations not in H_48..H_732.
- If the parent decides to serialize, append H_733.. at selectors \`40 2E3\`..:
${pass.map((r, i) => `  - H_${H0 + i} ${r.opcode} (${r.body}) — pin \`${r.pin}\``).join('\n')}
- Plus 1 Relock after append from pin \`7c079064…\`.
- Slot store disp cross-check: 50→80, 51→88, 52→90 (disp32 LE in pin).
- Deferred remainder: ${deferredNote}

## §7. Consolidation handoff

parent next = body-extend-101 serialize PASSes + 1 Relock
`;

fs.writeFileSync(logPath, log);
console.log('wrote', logPath);

const summaryPath = path.join(ROOT, 'scripts/_probe/parallel-batch-95-summary.json');
fs.writeFileSync(
  summaryPath,
  JSON.stringify(
    {
      attempted: rows.length,
      pass: pass.length,
      reject: reject.length,
      pin: PIN,
      concurrency: MAX_SCRATCH_WORKERS,
      passes: pass.map((r, i) => ({
        h: H0 + i,
        sel: SEL0 + i,
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
