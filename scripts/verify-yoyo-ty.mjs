#!/usr/bin/env node
/**
 * scripts/verify-yoyo-ty.mjs — Decision #13 lockdown check.
 * Compares sha256 of yoyo/projects/yoyo.ty against yoyo/tests/yoyo.ty.lock
 */
import { createHash } from 'crypto';
import { readFileSync, existsSync } from 'fs';
import { dirname, join } from 'path';
import { fileURLToPath } from 'url';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const tyPath = join(root, 'yoyo', 'projects', 'yoyo.ty');
const lockPath = join(root, 'yoyo', 'tests', 'yoyo.ty.lock');

if (!existsSync(tyPath) || !existsSync(lockPath)) {
  console.error('missing yoyo.ty or lock file');
  process.exit(1);
}

const actual = createHash('sha256').update(readFileSync(tyPath)).digest('hex');
const lock = JSON.parse(readFileSync(lockPath, 'utf8'));
const expected = lock.sha256;

if (actual !== expected) {
  console.error('✗ yoyo.ty hash mismatch');
  console.error('  expected:', expected);
  console.error('  actual:  ', actual);
  process.exit(1);
}
console.log('✓ yoyo.ty lockdown OK', actual.slice(0, 16) + '…');
console.log('  signed:', lock.signer, lock.date);
