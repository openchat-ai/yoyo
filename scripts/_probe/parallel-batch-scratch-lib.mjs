/**
 * Shared scratch-only probe pool for parallel-batch-N runners.
 * Concurrent within one batch (≤8). Never Relocks / never writes yoyo.ty.
 *
 * New batches: import from this module; do not copy the old serial for-loop.
 */
import { spawn } from 'child_process';
import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
export const ROOT = path.resolve(__dirname, '../..');
export const GOLDEN = path.join(ROOT, 'yoyo/tests/golden');
export const JS_DRIVER = path.join(ROOT, 'scripts/_probe/js-ty2text.mjs');
export const YOYO_EXE = path.join(ROOT, 'yoyo-rust/target/debug/yoyo.exe');
export const MAX_SCRATCH_WORKERS = 8;

export function shaFull(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex');
}

function spawnCapture(cmd, args, opts = {}) {
  return new Promise((resolve) => {
    const child = spawn(cmd, args, {
      ...opts,
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    const chunksOut = [];
    const chunksErr = [];
    child.stdout.on('data', (d) => chunksOut.push(d));
    child.stderr.on('data', (d) => chunksErr.push(d));
    child.on('error', (err) => {
      resolve({
        status: 1,
        stdout: Buffer.concat(chunksOut),
        stderr: Buffer.from(String(err.message || err)),
      });
    });
    child.on('close', (status) => {
      resolve({
        status: status ?? 1,
        stdout: Buffer.concat(chunksOut),
        stderr: Buffer.concat(chunksErr),
      });
    });
  });
}

/** Bounded worker pool; preserves input order in results. */
export async function mapPool(items, concurrency, fn) {
  const n = Math.max(1, Math.min(concurrency, MAX_SCRATCH_WORKERS, items.length || 1));
  const results = new Array(items.length);
  let next = 0;
  async function worker() {
    while (true) {
      const i = next++;
      if (i >= items.length) return;
      results[i] = await fn(items[i], i);
    }
  }
  await Promise.all(Array.from({ length: Math.min(n, items.length) }, () => worker()));
  return results;
}

export function writeScratch(p, { batchTag = 'scratch' } = {}) {
  const tyPath = path.join(GOLDEN, `_scratch_${p.name}.ty`);
  const hexPath = path.join(GOLDEN, `_scratch_${p.name}.code.hex`);
  const ty = `; _scratch_${p.name}.ty — ${p.opcode} ${p.args}\n; ${batchTag} scratch-only\n40 00\n  ${p.body}\n  FF\n`;
  fs.writeFileSync(tyPath, ty);
  fs.writeFileSync(hexPath, p.expected + '\n');
  return { tyPath, hexPath };
}

async function runJs(tyPath) {
  const r = await spawnCapture(process.execPath, [JS_DRIVER, tyPath]);
  if (r.status !== 0) {
    return { ok: false, detail: (r.stderr.length ? r.stderr : r.stdout).toString() || 'js fail' };
  }
  return { ok: true, buf: r.stdout };
}

async function runRust(tyPath, outBin) {
  // Prefer prebuilt exe so concurrent workers do not serialize on cargo lock.
  if (fs.existsSync(YOYO_EXE)) {
    const r = await spawnCapture(YOYO_EXE, ['link', '--target=stub', tyPath, outBin], {
      cwd: path.join(ROOT, 'yoyo-rust/verifier'),
    });
    if (r.status === 0) {
      const blob = fs.readFileSync(outBin);
      return { ok: true, buf: blob.subarray(1) };
    }
  }
  const r = await spawnCapture(
    'cargo',
    ['run', '-q', '--bin', 'yoyo', '--', 'link', '--target=stub', tyPath, outBin],
    { cwd: path.join(ROOT, 'yoyo-rust/verifier') }
  );
  if (r.status !== 0) {
    return {
      ok: false,
      detail: (r.stderr.length ? r.stderr : r.stdout).toString() || 'rust fail',
    };
  }
  const blob = fs.readFileSync(outBin);
  return { ok: true, buf: blob.subarray(1) };
}

export async function probeOne(p, { batchTag = 'scratch' } = {}) {
  const { tyPath } = writeScratch(p, { batchTag });
  const outBin = path.join(GOLDEN, `_scratch_${p.name}.bin`);
  const [js, rust] = await Promise.all([runJs(tyPath), runRust(tyPath, outBin)]);
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
  else if (!peerEq) result = 'REJECT';
  else if (!jsOk || !rustOk || jsha !== rsha) result = 'REJECT';
  const row = {
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
  };
  console.log(
    `${p.name}: ${result} len=${exp.length} js=${jsOk} rust=${rustOk} peer=${peerEq} sha=${jsha}`
  );
  if (!js.ok) console.log('  js fail:', String(js.detail).slice(0, 200));
  if (!rust.ok) console.log('  rust fail:', String(rust.detail).slice(0, 200));
  if (js.ok && !jsOk) console.log('  js hex:', js.buf.toString('hex'));
  if (rust.ok && !rustOk) console.log('  rust hex:', rust.buf.toString('hex'));
  return row;
}

/**
 * Run independent scratch probes with ≤ MAX_SCRATCH_WORKERS concurrency.
 * Relock / yoyo.ty / lock writes are out of scope — caller must stay consolidator-serial.
 */
export async function runScratchPicks(picks, { concurrency = MAX_SCRATCH_WORKERS, batchTag } = {}) {
  const limit = Math.max(1, Math.min(concurrency, MAX_SCRATCH_WORKERS));
  console.log(`scratch pool: ${picks.length} picks, concurrency=${limit} (max ${MAX_SCRATCH_WORKERS})`);
  return mapPool(picks, limit, (p) => probeOne(p, { batchTag }));
}
