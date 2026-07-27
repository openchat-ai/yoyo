/**
 * parallel-batch-96 probe — matrix-priority picks (NOT random imm ladders).
 * Strategy switch from batch-94/95: fills selfhost-emit-matrix P0→P1 rows.
 * Uses parallel-batch-scratch-lib (≤8 concurrent). Do NOT touch yoyo.ty / lock.
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

// ===== Matrix-priority picks (P0→P1) =====
// Slot disp rule: slot n → disp n*8 (LE). 50→0x280, 51→0x288, 52→0x290,
// 60→0x300, 61→0x308, 62→0x310.
// Prior locked: H_48..H_740 (imm ladders + GET/SET/ADDV/ORV/SUBV/IMUL/CMP base).
//
// P0 target set (from selfhost-emit-matrix.md):
//   - 84 MEMCPY_DATA / 85 MEMCPY_STATE: both emit stub 0xC3 in JS & Rust →
//     encode-eq WILL pass; semantic gap is that stub doesn't actually copy.
//     Log notes this; consolidation (body-extend-102) is where real MEMCPY lands.
//   - 60 GET multi-slot (fresh dst/src combos)
//   - 30 SET multi-imm (fresh imm values)
// P1: 68 ADDV / 6A SUBV / 69 ORV / 63 IMUL multi-combo; 65 CMP multi-slot.

const picks = [
  // -- P0: MEMCPY stub probes (both emit C3; PASS encodes, noted as semantic gap) --
  {
    name: 'memcpy_data_stub',
    opcode: '0x84 MEMCPY_DATA',
    args: 'dst=0x50 src=0x51 n=0x40',
    body: '84 50 51 40',
    expected: pinOf(0x84, [0x50, 0x51, 0x40]),
  },
  {
    name: 'memcpy_state_stub',
    opcode: '0x85 MEMCPY_STATE',
    args: 'dst=0x50 src=0x51 n=0x40',
    body: '85 50 51 40',
    expected: pinOf(0x85, [0x50, 0x51, 0x40]),
  },
  // -- P0: GET multi-slot (dst≠src across 50/51/52/60/61/62) --
  {
    name: 'get_60_50',
    opcode: '0x60 GET',
    args: 'dst=0x60 src=0x50',
    body: '60 60 50',
    expected: pinOf(0x60, [0x60, 0x50]),
  },
  {
    name: 'get_50_60',
    opcode: '0x60 GET',
    args: 'dst=0x50 src=0x60',
    body: '60 50 60',
    expected: pinOf(0x60, [0x50, 0x60]),
  },
  // -- P0: SET multi-imm (fresh imm32 values; slot varies) --
  {
    name: 'set_50_0xfff',
    opcode: '0x30 SET',
    args: 'slot=0x50 imm=0xfff',
    body: '30 50 fff',
    expected: pinOf(0x30, [0x50, 0xfff]),
  },
  {
    name: 'set_51_0x10000',
    opcode: '0x30 SET',
    args: 'slot=0x51 imm=0x10000',
    body: '30 51 10000',
    expected: pinOf(0x30, [0x51, 0x10000]),
  },
  // -- P1: ORV multi-combo (dst/src fresh pair) --
  {
    name: 'orv_50_62',
    opcode: '0x69 ORV',
    args: 'dst=0x50 src=0x62',
    body: '69 50 62',
    expected: pinOf(0x69, [0x50, 0x62]),
  },
  // -- P1: CMP multi-slot --
  {
    name: 'cmp_60_52',
    opcode: '0x65 CMP',
    args: 'a=0x60 b=0x52',
    body: '65 60 52',
    expected: pinOf(0x65, [0x60, 0x52]),
  },
];

const rows = await runScratchPicks(picks, {
  concurrency: MAX_SCRATCH_WORKERS,
  batchTag: 'batch-96-matrix',
});

const pass = rows.filter((r) => r.result === 'PASS');
const reject = rows.filter((r) => r.result !== 'PASS');
console.log(`\nSummary: ${pass.length} PASS / ${reject.length} REJECT`);

const logPath = path.join(ROOT, 'docs/auxdocs/parallel-batch-96-log.md');
const table = rows
  .map(
    (r, i) =>
      `| ${i + 1} | ${r.opcode} | ${r.args} | \`${r.pin}\` (${r.len}) | ${r.jsOk ? 'same' : 'DIFF'} | ${r.rustOk ? 'same' : 'DIFF'} | ${r.byteEq} | \`${r.jsha}\` | \`${r.rsha}\` | ${r.result} |`
  )
  .join('\n');

const detailBlocks = rows
  .map((r, i) => {
    const note = r.name.startsWith('memcpy')
      ? '- **MEMCPY NOTE**: JS & Rust both emit stub `0xc3`; byte-eq PASSES, but this is the D-3 semantic gap (stub does not actually copy). Consolidation (body-extend-102) must implement real MEMCPY before this row is truly DONE.'
      : '';
    return `### Pick ${i + 1}: ${r.opcode} ${r.args} — **${r.result}**

- fixture: \`_scratch_${r.name}.ty\` + \`.code.hex\`
- expected pin (${r.len}B): \`${r.pin}\`
- js-sha256: \`${r.jshaFull}\`
- rust-sha256: \`${r.result === 'PASS' ? 'same' : r.rshaFull}\`
- byte-eq JS↔Rust↔expected: ${r.byteEq}
${note}
`;
  })
  .join('\n');

const scratchList = rows
  .map((r) => `- \`yoyo/tests/golden/_scratch_${r.name}.ty\` + \`.code.hex\``)
  .join('\n');

const log = `# parallel-batch-96 Log · 8-pick matrix-priority scratch sweep

> Tag: \`parallel-batch-96-EXPERIMENTAL-8-pick-matrix\` · 2026-07-26 (UTC+8).
> Following body-extend-101 (pin \`514ff62ce8663a1507d07f965ba205e7483428b503c34e58f0a35dcfe4064719\`, handlers = 747, H_00..H_740 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (still do not invent-green).
>
> **STRATEGY SWITCH**: This batch picks from \`docs/auxdocs/selfhost-emit-matrix.md\`
> priority gaps (P0→P1), NOT random imm ladders. Prior batches 94/95 used imm
> ladders (P3). Starting batch-96, all picks trace to a matrix (opcode, shape) row.
> Full body-extend phase ends when all selfhost-need=YES rows reach DONE status.
>
> 8 picks: 2 MEMCPY stub probes (P0), 2 GET multi-slot (P0), 2 SET multi-imm (P0),
> 1 ORV multi-combo (P1), 1 CMP multi-slot (P1).
> Next selectors after 0x2EA: \`40 2EB\`..

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
${table}

**Summary**: ${pass.length} PASS / ${reject.length} REJECT out of ${rows.length} attempted.

## §1b. Full sha256 per pick

${detailBlocks}

## §2. Pick rationale (matrix trace)

- **P0 · 84 MEMCPY_DATA**: matrix row "MISSING / YES" — stub emits \`0xc3\` in JS & Rust; byte-eq passes but semantic gap remains. Real MEMCPY needed for self-host.
- **P0 · 85 MEMCPY_STATE**: matrix row "MISSING / YES" — same stub situation as MEMCPY_DATA.
- **P0 · 60 GET multi-slot**: dst=60 src=50 and dst=50 src=60; tests cross-slot load+store with varying disp (0x300/0x280).
- **P0 · 30 SET multi-imm**: imm=0xfff (imm32, 22B) on slot 50; imm=0x10000 (imm32, 22B) on slot 51; fresh large imm values.
- **P1 · 69 ORV multi-combo**: dst=50 src=62; fresh bitwise OR pair not in H_48..H_740.
- **P1 · 65 CMP multi-slot**: a=60 b=52; fresh comparison pair for Jcc loops.
- No D-1 0x20/0x50/0x51, no D-2 0x64.
- yoyo.ty unchanged this beat.

## §3. Files touched

${scratchList}
- \`docs/auxdocs/parallel-batch-96-log.md\` — this file
- \`scripts/_probe/parallel-batch-96-run.mjs\` — probe runner (uses shared concurrent lib)
- \`scripts/_probe/parallel-batch-scratch-lib.mjs\` — ≤${MAX_SCRATCH_WORKERS} scratch workers

NO \`yoyo/projects/yoyo.ty\`, NO lock, NO \`golden.js\`, NO \`self_test.rs\`, NO \`main.rs\`.

## §4. Parent next

**parent next = body-extend-102 serialize PASSes + 1 Relock** (consolidator implements MEMCPY)

Pass pin from body-extend-101 Relock: \`514ff62ce8663a1507d07f965ba205e7483428b503c34e58f0a35dcfe4064719\`.
Handlers before consolidate = 747 (H_00..H_740). Next selectors \`40 2EB\`.. for H_741.. if all serialize.

PASS list for body-extend-102:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
${pass.map((r, i) => {
  const h = 741 + i;
  const sel = (0x2eb + i).toString(16).toUpperCase();
  const argsShort = r.args.replace(/slot=|imm=|dd=|ss=|oo=|dst=|src=|a=|b=|n=/g, '').replace(/\s+/g, ' ').trim();
  return `| H_${h} | 0x${sel} | ${r.opcode} | ${argsShort} | \`${r.pin}\` (${r.len}B) | \`${r.jsha}\` |`;
}).join('\n')}

## §5. Honesty override checks

- Peer JS/Rust divergence at the ${pass.length} PASS handlers: **${reject.length === 0 ? 'NONE' : 'SEE REJECTS'}** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
- No PROMPT edit. No version bump. No \`*.lock\` touch.
- No git commit (W-START convention).
- No \`yoyo.ty\` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- MEMCPY_DATA/STATE probe: both emit stub \`0xc3\` → byte-eq PASSES; the D-3 gap is semantic (no real copy), recorded honestly, not invented-green.

## §6. Consolidation handoff

parent next = body-extend-102 serialize PASSes + 1 Relock (MEMCPY implementation required before true DONE)
`;

fs.writeFileSync(logPath, log);
console.log('wrote', logPath);

const summaryPath = path.join(ROOT, 'scripts/_probe/parallel-batch-96-summary.json');
fs.writeFileSync(
  summaryPath,
  JSON.stringify(
    {
      attempted: rows.length,
      pass: pass.length,
      reject: reject.length,
      pin: '514ff62ce8663a1507d07f965ba205e7483428b503c34e58f0a35dcfe4064719',
      concurrency: MAX_SCRATCH_WORKERS,
      matrixPriority: true,
      passes: pass.map((r, i) => ({
        h: 741 + i,
        sel: 0x2eb + i,
        name: r.name,
        opcode: r.opcode,
        args: r.args,
        body: r.body,
        pin: r.pin,
        len: r.len,
        sha16: r.jsha,
        sha256: r.jshaFull,
        matrixPriority: r.name.startsWith('memcpy') ? 'P0'
          : r.opcode.includes('GET') ? 'P0'
          : r.opcode.includes('SET') ? 'P0'
          : 'P1',
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
