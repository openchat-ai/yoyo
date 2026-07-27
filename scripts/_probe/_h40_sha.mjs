// One-shot: compute sha256 of yoyo.ty
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
const root = 'f:/yoyo';
const buf = readFileSync(`${root}/yoyo/projects/yoyo.ty`);
const sha = createHash('sha256').update(buf).digest('hex');
console.log('sha256 = ' + sha);
