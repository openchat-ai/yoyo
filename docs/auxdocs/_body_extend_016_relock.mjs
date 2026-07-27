import fs from 'fs';
import path from 'path';
import crypto from 'crypto';

const ROOT = 'f:/yoyo';

// 1) Rewrite yoyo.ty H_54..H_61
{
  const p = path.join(ROOT, 'yoyo/projects/yoyo.ty');
  let ty = fs.readFileSync(p, 'utf8');
  const idx = ty.indexOf('; H_54 (body-extend-016');
  if (idx < 0) throw new Error('H_54 marker missing');
  const block = fs.readFileSync(
    path.join(ROOT, 'docs/auxdocs/_body_extend_016_handlers.ty.txt'),
    'utf8'
  );
  // Ensure trailing newline
  const out = ty.slice(0, idx) + block.replace(/\s*$/, '\n');
  fs.writeFileSync(p, out);
  const t = fs.readFileSync(p, 'utf8');
  const bodies = [...t.matchAll(/40 (3[C-F]|4[0-3])\r?\n  ([^\n]+)/g)].map((m) =>
    m[0].replace(/\r?\n/g, ' | ')
  );
  console.log('yoyo.ty bodies:\n' + bodies.join('\n'));
  const expect = [
    '40 3C |   69 52 51',
    '40 3D |   6A 52 51',
    '40 3E |   63 51 50',
    '40 3F |   63 52 51',
    '40 40 |   65 51 50',
    '40 41 |   60 52 50',
    '40 42 |   30 51 DEADBEEF',
    '40 43 |   80 51 60 08',
  ];
  for (let i = 0; i < expect.length; i++) {
    if (!bodies[i] || !bodies[i].startsWith(expect[i])) {
      throw new Error('body mismatch at ' + i + ': ' + bodies[i]);
    }
  }
}

// 2) Relock immediately
{
  const tyPath = path.join(ROOT, 'yoyo/projects/yoyo.ty');
  const lockPath = path.join(ROOT, 'yoyo/tests/yoyo.ty.lock');
  const buf = fs.readFileSync(tyPath);
  const sha = crypto.createHash('sha256').update(buf).digest('hex');
  const prev = JSON.parse(fs.readFileSync(lockPath, 'utf8'));
  const lock = {
    sha256: sha,
    previous_sha256: prev.sha256,
    date: '2026-07-25',
    signer: 'bootstrap',
    note:
      'Decision #17 Relock - body-extend-016 / parallel-batch-10 consolidation: H_54..H_61 (ORV-h52/SUBV-h52/IMUL-swap/IMUL-h52/CMP-swap/GET-h52/SET-deadbeef/LDB-dst51; selectors 0x3C..0x43; +8 handlers). EXPERIMENTAL only; no PROMPT edit, version bump, commit, or GREEN promotion.',
  };
  fs.writeFileSync(lockPath, JSON.stringify(lock, null, 4) + '\n');
  console.log('Relocked pin', sha.slice(0, 16) + '…');
  console.log('previous', prev.sha256.slice(0, 16) + '…');
}
