/**
 * parallel-batch-98 probe — matrix-priority picks (P1 focus).
 * Fresh dst/src/slot combos for ADDV/SUBV/IMUL/ORV/CMP/INC/DEC.
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

// ===== P1 fresh combos (matrix priority) =====
// Avoid combos already in H_00..H_756:
//   ADDV existing: 50/51,51/50,52/51,50/52,51/52,52/50,60/61,50/51dup
//   SUBV existing: 50/51,51/50,52/51,50/52,52/50,60/61,61/62
//   IMUL existing: 50/51,51/50,52/51,50/52,52/50,51/52
//   ORV existing: 50/51,51/50,52/51,50/52,52/50,50/62
//   CMP existing: 50/51,51/50,52/51,52/50,50/52,60/52,62/60
//   INC existing: 50,51,52,60,61   DEC existing: 50,51,52,60
// 8 picks: 2 ADDV + 2 SUBV + 2 IMUL + 1 ORV + 1 CMP (multi-slot P1)

const picks = [
  // -- P1: ADDV fresh dst/src high-slot pairs --
  {
    name: 'addv_60_52',
    opcode: '0x68 ADDV',
    args: 'dst=0x60 src=0x52',
    body: '68 60 52',
    expected: pinOf(0x68, [0x60, 0x52]),
  },
  {
    name: 'addv_62_50',
    opcode: '0x68 ADDV',
    args: 'dst=0x62 src=0x50',
    body: '68 62 50',
    expected: pinOf(0x68, [0x62, 0x50]),
  },
  // -- P1: SUBV fresh high-slot pairs --
  {
    name: 'subv_62_60',
    opcode: '0x6A SUBV',
    args: 'dst=0x62 src=0x60',
    body: '6A 62 60',
    expected: pinOf(0x6A, [0x62, 0x60]),
  },
  {
    name: 'subv_62_50',
    opcode: '0x6A SUBV',
    args: 'dst=0x62 src=0x50',
    body: '6A 62 50',
    expected: pinOf(0x6A, [0x62, 0x50]),
  },
  // -- P1: IMUL fresh dst/src pairs (≥60/61/62 slots) --
  {
    name: 'imul_60_62',
    opcode: '0x63 IMUL',
    args: 'dst=0x60 src=0x62',
    body: '63 60 62',
    expected: pinOf(0x63, [0x60, 0x62]),
  },
  {
    name: 'imul_62_61',
    opcode: '0x63 IMUL',
    args: 'dst=0x62 src=0x61',
    body: '63 62 61',
    expected: pinOf(0x63, [0x62, 0x61]),
  },
  // -- P1: ORV fresh pair --
  {
    name: 'orv_60_62',
    opcode: '0x69 ORV',
    args: 'dst=0x60 src=0x62',
    body: '69 60 62',
    expected: pinOf(0x69, [0x60, 0x62]),
  },
  // -- P1: CMP fresh pair --
  {
    name: 'cmp_61_60',
    opcode: '0x65 CMP',
    args: 'a=0x61 b=0x60',
    body: '65 61 60',
    expected: pinOf(0x65, [0x61, 0x60]),
  },
];

const rows = await runScratchPicks(picks, {
  concurrency: MAX_SCRATCH_WORKERS,
  batchTag: 'batch-98-matrix',
});

const pass = rows.filter((r) => r.result === 'PASS');
const reject = rows.filter((r) => r.result !== 'PASS');
console.log(`\nSummary: ${pass.length} PASS / ${reject.length} REJECT`);

const logPath = path.join(ROOT, 'docs/auxdocs/parallel-batch-98-log.md');
const table = rows
  .map(
    (r, i) =>
      `| ${i + 1} | ${r.opcode} | ${r.args} | \`${r.pin}\` (${r.len}) | ${r.jsOk ? 'same' : 'DIFF'} | ${r.rustOk ? 'same' : 'DIFF'} | ${r.byteEq} | \`${r.jsha}\` | \`${r.rsha}\` | ${r.result} |`
  )
  .join('\n');

const detailBlocks = rows
  .map(
    (r, i) =>
      `### Pick ${i + 1}: ${r.opcode} ${r.args} — **${r.result}**

- fixture: \`_scratch_${r.name}.ty\` + \`.code.hex\`
- expected pin (${r.len}B): \`${r.pin}\`
- js-sha256: \`${r.jshaFull}\`
- rust-sha256: \`${r.result === 'PASS' ? 'same' : r.rshaFull}\`
- byte-eq JS↔Rust↔expected: ${r.byteEq}
`
  )
  .join('\n');

const scratchList = rows
  .map(
    (r) => `- \`yoyo/tests/golden/_scratch_${r.name}.ty\` + \`.code.hex\``
  )
  .join('\n');

const log = `# parallel-batch-98 Log · 8-pick matrix-priority scratch sweep (P1 focus)

> Tag: \`parallel-batch-98-EXPERIMENTAL-8-pick-P1\` · 2026-07-26 (UTC+8).
> Following body-extend-103 (pin \`82709dac80fafbbf75421ea1e1b3493a4249f107f85115bfa0509f2d8cf11653\`, handlers = 763, H_00..H_756 locked).
> W-START: **EXPERIMENTAL · NON-GREEN** (still do not invent-green).
>
> **STRATEGY**: matrix-priority picks from \`docs/auxdocs/selfhost-emit-matrix.md\` P1 rows —
> fresh dst/src/slot combos for ADDV/SUBV/IMUL/ORV/CMP (P1). Skips MEMCPY (needs real impl),
> skips imm ladders (P3, not selfhost-need).
>
> 8 picks: 2 ADDV + 2 SUBV + 2 IMUL + 1 ORV + 1 CMP — all multi-slot high-slot (≥0x60) pairs.
> Next selectors after 0x2FA: \`40 2FB\`..
>
> MEMCPY_DATA/STATE (P0) remain PARTIAL (stub=C3). INC/DEC expanded in batch-97; this beat
> prioritizes ADDV/SUBV/IMUL/CMP/ORV coverage.

## §1. Per-handler results (parent-verified by direct JS+Rust driver invocation)

| pick | opcode | args | expected (B) | js-actual | rust-actual | byte-eq | js-sha256 (16) | rust-sha256 (16) | result |
|------|--------|------|--------------|-----------|-------------|---------|----------------|------------------|--------|
${table}

**Summary**: ${pass.length} PASS / ${reject.length} REJECT out of ${rows.length} attempted.

## §1b. Full sha256 per pick

${detailBlocks}

## §2. Pick rationale (matrix trace)

- **P1 · 68 ADDV (2 picks)**: dst=60 src=52; dst=62 src=50 — high-slot pairs filling slots ≥0x60.
- **P1 · 6A SUBV (2 picks)**: dst=62 src=60; dst=62 src=50 — same rationale.
- **P1 · 63 IMUL (2 picks)**: dst=60 src=62; dst=62 src=61 — high-slot multiply pairs.
- **P1 · 69 ORV (1 pick)**: dst=60 src=62 — fresh bitwise OR pair.
- **P1 · 65 CMP (1 pick)**: a=61 b=60 — fresh comparison pair for Jcc loops.
- No D-1 0x20/0x50/0x51, no D-2 0x64. No MEMCPY (deferred, needs real impl). No imm ladders (P3).
- yoyo.ty unchanged this beat.

## §3. Files touched

${scratchList}
- \`docs/auxdocs/parallel-batch-98-log.md\` — this file
- \`scripts/_probe/parallel-batch-98-run.mjs\` — probe runner (uses shared concurrent lib)
- \`scripts/_probe/parallel-batch-scratch-lib.mjs\` — ≤${MAX_SCRATCH_WORKERS} scratch workers

NO \`yoyo/projects/yoyo.ty\`, NO lock, NO \`golden.js\`, NO \`self_test.rs\`, NO \`main.rs\`.

## §4. Parent next

**parent next = body-extend-104 serialize PASSes + 1 Relock** (consolidator adds H_757..H_764).

Pass pin from body-extend-103 Relock: \`82709dac80fafbbf75421ea1e1b3493a4249f107f85115bfa0509f2d8cf11653\`.
Handlers before consolidate = 763 (H_00..H_756). Next selectors \`40 2FB\`.. for H_757.. if all serialize.

PASS list for body-extend-104:

| next H | sel | opcode | args | pin hex | sha16 |
|--------|-----|--------|------|---------|-------|
${pass.map((r, i) => {
  const h = 757 + i;
  const sel = (0x2fb + i).toString(16).toUpperCase();
  const argsShort = r.args
    .replace(/slot=|imm=|dd=|ss=|oo=|dst=|src=|a=|b=|n=/g, '')
    .replace(/\s+/g, ' ')
    .trim();
  return `| H_${h} | 0x${sel} | ${r.opcode} | ${argsShort} | \`${r.pin}\` (${r.len}B) | \`${r.jsha}\` |`;
}).join('\n')}

## §5. Honesty override checks

- Peer JS/Rust divergence at the ${pass.length} PASS handlers: **${reject.length === 0 ? 'NONE' : 'SEE REJECTS'}** (fail-closed on divergence).
- Lock Protocol step 1 (compile) failure: **NONE** on PASS picks.
- No PROMPT edit. No version bump. No \`*.lock\` touch.
- No git commit (W-START convention).
- No \`yoyo.ty\` modification, no selector added at canonical surface.
- W-START row stays EXPERIMENTAL; W-START red-list unchanged.
- MEMCPY_DATA/STATE (P0) remain PARTIAL (stub=C3) — semantic gap, not invented-green.

## §6. Consolidation handoff

parent next = body-extend-104 serialize PASSes + 1 Relock (INC/DEC slots ≥0x62 may need next beat if 8 picks saturated)
`;

fs.writeFileSync(logPath, log);
console.log('wrote', logPath);

const summaryPath = path.join(ROOT, 'scripts/_probe/parallel-batch-98-summary.json');
fs.writeFileSync(
  summaryPath,
  JSON.stringify(
    {
      attempted: rows.length,
      pass: pass.length,
      reject: reject.length,
      pin: '82709dac80fafbbf75421ea1e1b3493a4249f107f85115bfa0509f2d8cf11653',
      concurrency: MAX_SCRATCH_WORKERS,
      matrixPriority: true,
      passes: pass.map((r, i) => ({
        h: 757 + i,
        sel: 0x2fb + i,
        name: r.name,
        opcode: r.opcode,
        args: r.args,
        body: r.body,
        pin: r.pin,
        len: r.len,
        sha16: r.jsha,
        sha256: r.jshaFull,
        matrixPriority: 'P1',
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
