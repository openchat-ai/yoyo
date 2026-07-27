/**
 * Dry smoke for mapPool concurrency (no Relock, no yoyo.ty).
 * Usage: node scripts/_probe/parallel-batch-scratch-lib-smoke.mjs
 */
import { mapPool, MAX_SCRATCH_WORKERS } from './parallel-batch-scratch-lib.mjs';

const items = Array.from({ length: 8 }, (_, i) => i);
let live = 0;
let peak = 0;

const results = await mapPool(items, MAX_SCRATCH_WORKERS, async (i) => {
  live++;
  peak = Math.max(peak, live);
  await new Promise((r) => setTimeout(r, 40));
  live--;
  return i * 2;
});

const ok =
  peak >= 2 &&
  peak <= MAX_SCRATCH_WORKERS &&
  results.length === 8 &&
  results.every((v, i) => v === i * 2);

console.log(`mapPool smoke: peak=${peak} max=${MAX_SCRATCH_WORKERS} order_ok=${results.every((v, i) => v === i * 2)} => ${ok ? 'PASS' : 'FAIL'}`);
process.exit(ok ? 0 : 1);
