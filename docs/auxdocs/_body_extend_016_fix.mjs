/**
 * body-extend-016 atomic fix — serialize parallel-batch-10 ALL 8 PASS picks
 * into canonical order H_54..H_61 at selectors 0x3C..0x43.
 */
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');

const HANDLERS = [
  {
    h: 54, sel: '3C', selNum: 0x3c, name: 'orv_h52',
    comment: `; H_54 (body-extend-016 / parallel-batch-10 — 0x69 ORV dst=0x52 src=0x51):
;   High-slot ORV at S[0x52]|=S[0x51] (canonical H_31 uses 69 50 51; H_49 uses 69 51 50).
;   Same shape as H_52 ADDV-h52 except \`or rax,rcx\`=4809c8 at byte 16. Pin 25B:
;   498b8790020000498b8f880200004809c849898790020000c3.
;   Not full self-host / not G06 / not Phase 2 / not freeze.
40 3C
  69 52 51
  FF`,
    pin: '498b8790020000498b8f880200004809c849898790020000c3',
    fixtureLines: [
      '; body-extend-016 / parallel-batch-10 ORV-h52 -- H_54 0x69 ORV dst=0x52 src=0x51 + RET',
      '; High-slot ORV S[0x52] |= S[0x51].',
      '; Pin 25B: 498b8790020000498b8f880200004809c849898790020000c3',
      '40 3C',
      '  69 52 51',
      '  FF',
    ],
  },
  {
    h: 55, sel: '3D', selNum: 0x3d, name: 'subv_h52',
    comment: `; H_55 (body-extend-016 / parallel-batch-10 — 0x6A SUBV dst=0x52 src=0x51):
;   High-slot SUBV at S[0x52]-=S[0x51] (canonical H_35 uses 6A 50 51; H_50 uses 6A 51 50).
;   Same shape as H_54 except \`sub rax,rcx\`=4829c8 at byte 16. Pin 25B:
;   498b8790020000498b8f880200004829c849898790020000c3.
;   Not full self-host / not G06 / not Phase 2 / not freeze.
40 3D
  6A 52 51
  FF`,
    pin: '498b8790020000498b8f880200004829c849898790020000c3',
    fixtureLines: [
      '; body-extend-016 / parallel-batch-10 SUBV-h52 -- H_55 0x6A SUBV dst=0x52 src=0x51 + RET',
      '; High-slot SUBV S[0x52] -= S[0x51].',
      '; Pin 25B: 498b8790020000498b8f880200004829c849898790020000c3',
      '40 3D',
      '  6A 52 51',
      '  FF',
    ],
  },
  {
    h: 56, sel: '3E', selNum: 0x3e, name: 'imul_swap',
    comment: `; H_56 (body-extend-016 / parallel-batch-10 — 0x63 IMUL dst=0x51 src=0x50):
;   Slot-swap IMUL (canonical H_34 uses 63 50 51). load+load+imul+store+ret. Pin 26B:
;   498b8788020000498b8f80020000480fafc149898788020000c3.
;   Not full self-host / not G06 / not Phase 2 / not freeze.
40 3E
  63 51 50
  FF`,
    pin: '498b8788020000498b8f80020000480fafc149898788020000c3',
    fixtureLines: [
      '; body-extend-016 / parallel-batch-10 IMUL-swap -- H_56 0x63 IMUL dst=0x51 src=0x50 + RET',
      '; Slot-swap IMUL S[0x51] *= S[0x50].',
      '; Pin 26B: 498b8788020000498b8f80020000480fafc149898788020000c3',
      '40 3E',
      '  63 51 50',
      '  FF',
    ],
  },
  {
    h: 57, sel: '3F', selNum: 0x3f, name: 'imul_h52',
    comment: `; H_57 (body-extend-016 / parallel-batch-10 — 0x63 IMUL dst=0x52 src=0x51):
;   High-slot IMUL at S[0x52]*=S[0x51]. Pin 26B:
;   498b8790020000498b8f88020000480fafc149898790020000c3.
;   Not full self-host / not G06 / not Phase 2 / not freeze.
40 3F
  63 52 51
  FF`,
    pin: '498b8790020000498b8f88020000480fafc149898790020000c3',
    fixtureLines: [
      '; body-extend-016 / parallel-batch-10 IMUL-h52 -- H_57 0x63 IMUL dst=0x52 src=0x51 + RET',
      '; High-slot IMUL S[0x52] *= S[0x51].',
      '; Pin 26B: 498b8790020000498b8f88020000480fafc149898790020000c3',
      '40 3F',
      '  63 52 51',
      '  FF',
    ],
  },
  {
    h: 58, sel: '40', selNum: 0x40, name: 'cmp_swap',
    comment: `; H_58 (body-extend-016 / parallel-batch-10 — 0x65 CMP a=0x51 b=0x50):
;   Slot-swap CMP (canonical H_36 uses 65 50 51). load+load+cmp+ret (no store). Pin 18B:
;   498b8788020000498b8f800200004839c8c3.
;   Not full self-host / not G06 / not Phase 2 / not freeze.
40 40
  65 51 50
  FF`,
    pin: '498b8788020000498b8f800200004839c8c3',
    fixtureLines: [
      '; body-extend-016 / parallel-batch-10 CMP-swap -- H_58 0x65 CMP a=0x51 b=0x50 + RET',
      '; Slot-swap CMP (no store).',
      '; Pin 18B: 498b8788020000498b8f800200004839c8c3',
      '40 40',
      '  65 51 50',
      '  FF',
    ],
  },
  {
    h: 59, sel: '41', selNum: 0x41, name: 'get_h52',
    comment: `; H_59 (body-extend-016 / parallel-batch-10 — 0x60 GET dst=0x52 src=0x50):
;   High-dst GET (H_39=60 50 51; H_51=60 51 52). load_state(0x50)+store_state(0x52)+ret.
;   Pin: 498b878002000049898790020000c3 (15B).
;   Not full self-host / not G06 / not Phase 2 / not freeze.
40 41
  60 52 50
  FF`,
    pin: '498b878002000049898790020000c3',
    fixtureLines: [
      '; body-extend-016 / parallel-batch-10 GET-h52 -- H_59 0x60 GET dst=0x52 src=0x50 + RET',
      '; High-dst GET S[0x52] = S[0x50].',
      '; Pin 15B: 498b878002000049898790020000c3',
      '40 41',
      '  60 52 50',
      '  FF',
    ],
  },
  {
    h: 60, sel: '42', selNum: 0x42, name: 'set_deadbeef',
    comment: `; H_60 (body-extend-016 / parallel-batch-10 — 0x30 SET slot=0x51 imm=0xDEADBEEF):
;   Non-zero SET imm at slot 0x51 (H_53 uses slot 0x52 / CAFEBABE).
;   movabs rax,0xDEADBEEF + store_state(0x51)+ret.
;   Pin: 48b8efbeadde0000000049898788020000c3 (18B).
;   Not full self-host / not G06 / not Phase 2 / not freeze.
40 42
  30 51 DEADBEEF
  FF`,
    pin: '48b8efbeadde0000000049898788020000c3',
    fixtureLines: [
      '; body-extend-016 / parallel-batch-10 SET-deadbeef -- H_60 0x30 SET slot=0x51 imm=0xDEADBEEF + RET',
      '; Non-zero imm at slot 0x51.',
      '; Pin 18B: 48b8efbeadde0000000049898788020000c3',
      '40 42',
      '  30 51 DEADBEEF',
      '  FF',
    ],
  },
  {
    h: 61, sel: '43', selNum: 0x43, name: 'ldb_dst51',
    comment: `; H_61 (body-extend-016 / parallel-batch-10 — 0x80 LDB dd=0x51 ss=0x60 oo=0x08):
;   LDB with dest slot 0x51 (H_40 uses dd=0x50). Same imm8 path at oo=8. Pin 23B:
;   498b87000300004883c008480fb60049898788020000c3.
;   Not full self-host / not G06 / not Phase 2 / not freeze.
40 43
  80 51 60 08
  FF`,
    pin: '498b87000300004883c008480fb60049898788020000c3',
    fixtureLines: [
      '; body-extend-016 / parallel-batch-10 LDB-dst51 -- H_61 0x80 LDB dd=0x51 ss=0x60 oo=8 + RET',
      '; LDB dest slot 0x51 (H_40 uses dd=0x50).',
      '; Pin 23B: 498b87000300004883c008480fb60049898788020000c3',
      '40 43',
      '  80 51 60 08',
      '  FF',
    ],
  },
];

function writeUtf8(p, s) {
  fs.writeFileSync(p, s, 'utf8');
}

// 1) yoyo.ty — replace from H_54 comment through EOF with correct block
{
  const tyPath = path.join(ROOT, 'yoyo/projects/yoyo.ty');
  let ty = fs.readFileSync(tyPath, 'utf8');
  const marker = '; H_54 (body-extend-016';
  const idx = ty.indexOf(marker);
  if (idx < 0) throw new Error('yoyo.ty H_54 marker missing');
  // keep through end of H_53 (before H_54)
  const head = ty.slice(0, idx);
  const block = HANDLERS.map((h) => h.comment).join('\n\n') + '\n';
  writeUtf8(tyPath, head + block);
  console.log('yoyo.ty H_54..H_61 rewritten');
}

// 2) fixtures
{
  const gdir = path.join(ROOT, 'yoyo/tests/golden');
  const edir = path.join(gdir, 'expected');
  for (const h of HANDLERS) {
    writeUtf8(path.join(gdir, `selfhost_min_${h.name}.ty`), h.fixtureLines.join('\n') + '\n');
    writeUtf8(path.join(edir, `selfhost_min_${h.name}.code.hex`), h.pin);
  }
  console.log('8 fixtures + expected pins written');
}

// 3) golden.js — replace body-extend-016 check block + cases
{
  const gPath = path.join(ROOT, 'yoyo-js/scripts/golden.js');
  let g = fs.readFileSync(gPath, 'utf8');
  const start = g.indexOf('/** body-extend-016 H_54');
  const start2 = g.indexOf('/**\n * ORV-H52: body-extend-016');
  const start3 = g.indexOf('function checkSETDEADBEEF()');
  let cut = -1;
  if (start >= 0) cut = start;
  else if (start2 >= 0) cut = start2;
  else if (start3 >= 0) {
    // find doc comment or blank before function after SET-LARGE
    cut = g.lastIndexOf('\n', start3);
    // prefer after checkSETLARGE closing
    const afterLarge = g.indexOf("return { id: 'SET-LARGE'");
    const afterLargeEnd = g.indexOf('\n}\n', afterLarge);
    if (afterLargeEnd > 0) cut = afterLargeEnd + 3;
  }
  if (cut < 0) throw new Error('golden.js cut point missing');
  const mainIdx = g.indexOf('\nfunction main() {', cut);
  if (mainIdx < 0) throw new Error('golden.js main missing');
  const afterMain = g.indexOf('\nmain();\n', mainIdx);
  if (afterMain < 0) throw new Error('golden.js main() call missing');

  const checks = `/**
 * ORV-H52: body-extend-016 / parallel-batch-10 H_54 \`0x69 ORV\` dst=0x52 src=0x51.
 * Pin 25B: 498b8790020000498b8f880200004809c849898790020000c3.
 */
function checkORVH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_orv_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_orv_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'ORV-H52', ok: false, detail: 'missing ORV-h52 fixture or expected pin' };
  }
  const dst = 0x52;
  const src = 0x51;
  const got = Buffer.from([...encodeOp(0x69, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'ORV-H52', ok: false, detail: \`mismatch: got \${hexOf(got)} want \${hexOf(expected)}\` };
  }
  const orSig = Buffer.from([0x48, 0x09, 0xc8]);
  if (!got.includes(orSig)) {
    return { id: 'ORV-H52', ok: false, detail: 'or rax,rcx signature 48 09 c8 missing' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'ORV-H52', ok: false, detail: \`fixture mismatch: got \${hexOf(fixture)} want \${hexOf(expected)}\` };
  }
  return { id: 'ORV-H52', ok: true, detail: \`dst=0x\${dst.toString(16)} src=0x\${src.toString(16)} code=\${hexOf(got)}\` };
}

/**
 * SUBV-H52: body-extend-016 / parallel-batch-10 H_55 \`0x6A SUBV\` dst=0x52 src=0x51.
 * Pin 25B: 498b8790020000498b8f880200004829c849898790020000c3.
 */
function checkSUBVH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_subv_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_subv_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'SUBV-H52', ok: false, detail: 'missing SUBV-h52 fixture or expected pin' };
  }
  const dst = 0x52;
  const src = 0x51;
  const got = Buffer.from([...encodeOp(0x6A, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'SUBV-H52', ok: false, detail: \`mismatch: got \${hexOf(got)} want \${hexOf(expected)}\` };
  }
  const subSig = Buffer.from([0x48, 0x29, 0xc8]);
  if (!got.includes(subSig)) {
    return { id: 'SUBV-H52', ok: false, detail: 'sub rax,rcx signature 48 29 c8 missing' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'SUBV-H52', ok: false, detail: \`fixture mismatch: got \${hexOf(fixture)} want \${hexOf(expected)}\` };
  }
  return { id: 'SUBV-H52', ok: true, detail: \`dst=0x\${dst.toString(16)} src=0x\${src.toString(16)} code=\${hexOf(got)}\` };
}

/**
 * IMUL-SWAP: body-extend-016 / parallel-batch-10 H_56 \`0x63 IMUL\` dst=0x51 src=0x50.
 * Pin 26B: 498b8788020000498b8f80020000480fafc149898788020000c3.
 */
function checkIMULSWAP() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_imul_swap.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_imul_swap.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'IMUL-SWAP', ok: false, detail: 'missing IMUL-swap fixture or expected pin' };
  }
  const dst = 0x51;
  const src = 0x50;
  const got = Buffer.from([...encodeOp(0x63, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'IMUL-SWAP', ok: false, detail: \`mismatch: got \${hexOf(got)} want \${hexOf(expected)}\` };
  }
  const imulSig = Buffer.from([0x48, 0x0f, 0xaf, 0xc1]);
  if (!got.includes(imulSig)) {
    return { id: 'IMUL-SWAP', ok: false, detail: 'imul rax,rcx signature 48 0f af c1 missing' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'IMUL-SWAP', ok: false, detail: \`fixture mismatch: got \${hexOf(fixture)} want \${hexOf(expected)}\` };
  }
  return { id: 'IMUL-SWAP', ok: true, detail: \`dst=0x\${dst.toString(16)} src=0x\${src.toString(16)} code=\${hexOf(got)}\` };
}

/**
 * IMUL-H52: body-extend-016 / parallel-batch-10 H_57 \`0x63 IMUL\` dst=0x52 src=0x51.
 * Pin 26B: 498b8790020000498b8f88020000480fafc149898790020000c3.
 */
function checkIMULH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_imul_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_imul_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'IMUL-H52', ok: false, detail: 'missing IMUL-h52 fixture or expected pin' };
  }
  const dst = 0x52;
  const src = 0x51;
  const got = Buffer.from([...encodeOp(0x63, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'IMUL-H52', ok: false, detail: \`mismatch: got \${hexOf(got)} want \${hexOf(expected)}\` };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'IMUL-H52', ok: false, detail: \`fixture mismatch: got \${hexOf(fixture)} want \${hexOf(expected)}\` };
  }
  return { id: 'IMUL-H52', ok: true, detail: \`dst=0x\${dst.toString(16)} src=0x\${src.toString(16)} code=\${hexOf(got)}\` };
}

/**
 * CMP-SWAP: body-extend-016 / parallel-batch-10 H_58 \`0x65 CMP\` a=0x51 b=0x50.
 * Pin 18B: 498b8788020000498b8f800200004839c8c3.
 */
function checkCMPSWAP() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_cmp_swap.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_cmp_swap.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'CMP-SWAP', ok: false, detail: 'missing CMP-swap fixture or expected pin' };
  }
  const a = 0x51;
  const b = 0x50;
  const got = Buffer.from([...encodeOp(0x65, [a, b]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'CMP-SWAP', ok: false, detail: \`mismatch: got \${hexOf(got)} want \${hexOf(expected)}\` };
  }
  const cmpSig = Buffer.from([0x48, 0x39, 0xc8]);
  if (!got.includes(cmpSig)) {
    return { id: 'CMP-SWAP', ok: false, detail: 'cmp rax,rcx signature 48 39 c8 missing' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'CMP-SWAP', ok: false, detail: \`fixture mismatch: got \${hexOf(fixture)} want \${hexOf(expected)}\` };
  }
  return { id: 'CMP-SWAP', ok: true, detail: \`a=0x\${a.toString(16)} b=0x\${b.toString(16)} code=\${hexOf(got)}\` };
}

/**
 * GET-H52: body-extend-016 / parallel-batch-10 H_59 \`0x60 GET\` dst=0x52 src=0x50.
 * Pin 15B: 498b878002000049898790020000c3.
 */
function checkGETH52() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_get_h52.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_get_h52.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'GET-H52', ok: false, detail: 'missing GET-h52 fixture or expected pin' };
  }
  const dst = 0x52;
  const src = 0x50;
  const got = Buffer.from([...encodeOp(0x60, [dst, src]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'GET-H52', ok: false, detail: \`mismatch: got \${hexOf(got)} want \${hexOf(expected)}\` };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'GET-H52', ok: false, detail: \`fixture mismatch: got \${hexOf(fixture)} want \${hexOf(expected)}\` };
  }
  return { id: 'GET-H52', ok: true, detail: \`dst=0x\${dst.toString(16)} src=0x\${src.toString(16)} code=\${hexOf(got)}\` };
}

/**
 * SET-DEADBEEF: body-extend-016 / parallel-batch-10 H_60 \`0x30 SET\` slot=0x51 imm=0xDEADBEEF.
 * Pin 18B: 48b8efbeadde0000000049898788020000c3.
 */
function checkSETDEADBEEF() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_set_deadbeef.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_set_deadbeef.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'SET-DEADBEEF', ok: false, detail: 'missing SET-deadbeef fixture or expected pin' };
  }
  const slot = 0x51;
  const imm = 0xDEADBEEF;
  const got = Buffer.from([...encodeOp(0x30, [slot, imm]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'SET-DEADBEEF', ok: false, detail: \`mismatch: got \${hexOf(got)} want \${hexOf(expected)}\` };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'SET-DEADBEEF', ok: false, detail: \`fixture mismatch: got \${hexOf(fixture)} want \${hexOf(expected)}\` };
  }
  return { id: 'SET-DEADBEEF', ok: true, detail: \`slot=0x\${slot.toString(16)} imm=0x\${imm.toString(16)} code=\${hexOf(got)}\` };
}

/**
 * LDB-DST51: body-extend-016 / parallel-batch-10 H_61 \`0x80 LDB\` dd=0x51 ss=0x60 oo=8.
 * Pin 23B: 498b87000300004883c008480fb60049898788020000c3.
 */
function checkLDBDST51() {
  const tyPath = path.join(GOLDEN_DIR, 'selfhost_min_ldb_dst51.ty');
  const expPath = path.join(GOLDEN_DIR, 'expected', 'selfhost_min_ldb_dst51.code.hex');
  if (!fs.existsSync(tyPath) || !fs.existsSync(expPath)) {
    return { id: 'LDB-DST51', ok: false, detail: 'missing LDB-dst51 fixture or expected pin' };
  }
  const dd = 0x51;
  const ss = 0x60;
  const oo = 8;
  const got = Buffer.from([...encodeOp(0x80, [dd, ss, oo]), 0xc3]);
  const expected = loadExpectedHex(expPath);
  if (!got.equals(expected)) {
    return { id: 'LDB-DST51', ok: false, detail: \`mismatch: got \${hexOf(got)} want \${hexOf(expected)}\` };
  }
  const addSig = Buffer.from([0x48, 0x83, 0xc0, 0x08]);
  if (!got.includes(addSig)) {
    return { id: 'LDB-DST51', ok: false, detail: 'add rax,imm8 signature 48 83 c0 08 missing' };
  }
  const fixture = compileCode(parseTy(readUtf8(tyPath)));
  if (!fixture.equals(expected)) {
    return { id: 'LDB-DST51', ok: false, detail: \`fixture mismatch: got \${hexOf(fixture)} want \${hexOf(expected)}\` };
  }
  return { id: 'LDB-DST51', ok: true, detail: \`dd=0x\${dd.toString(16)} ss=0x\${ss.toString(16)} oo=\${oo} code=\${hexOf(got)}\` };
}

function main() {
  const cases = [checkG00(), checkG01(), checkG02(), checkG03(), checkG04(), checkG05(), checkINC(), checkDEC(), checkADDIMM(), checkSUBIMM(), checkMOVRR(), checkORV(), checkNOP(), checkRAWBYTES(), checkIMUL(), checkSUBV(), checkCMP(), checkLDBBODY(), checkSETCONTROL(), checkGET(), checkADDVSWAP(), checkORVSWAP(), checkSUBVSWAP(), checkGETALT(), checkADDVH52(), checkSETLARGE(), checkORVH52(), checkSUBVH52(), checkIMULSWAP(), checkIMULH52(), checkCMPSWAP(), checkGETH52(), checkSETDEADBEEF(), checkLDBDST51(), checkJMP(), checkCALLBACK(), checkCALLRET(), checkLDB(), checkLDBoff8(), checkLDBOFF8HANDLER(), checkLDBOFF127HANDLER(), checkLDBOFFM128HANDLER(), checkLDBOFF64HANDLER(), checkLDBOFF16HANDLER(), checkLDBOFF32HANDLER(), checkLDBOFF96HANDLER(), checkLDBOFF112HANDLER(), checkLDBoff127(), checkLDBoff128(), checkLDBoff256(), checkLDBoffm128(), checkLDBoffm129()];
  let failed = 0;
  for (const c of cases) {
    if (c.ok) {
      console.log(\`\${c.id} PASS — \${c.detail}\`);
    } else {
      failed++;
      console.error(\`\${c.id} FAIL — \${c.detail}\`);
    }
  }
  if (cases.length === 0) {
    console.error('golden: no cases registered (fail-closed)');
    process.exit(1);
  }
  if (failed > 0) {
    console.error(\`golden: \${failed}/\${cases.length} failed\`);
    process.exit(1);
  }
  console.log(\`golden: \${cases.length} case(s) ok (G00–G05 + INC + DEC + ADD-IMM + SUB-IMM + MOVRR + ORV + NOP + RAW-BYTES + IMUL + SUBV + CMP + LDB-BODY + SET-CONTROL + GET + ADDV-SWAP + ORV-SWAP + SUBV-SWAP + GET-ALT + ADDV-H52 + SET-LARGE + ORV-H52 + SUBV-H52 + IMUL-SWAP + IMUL-H52 + CMP-SWAP + GET-H52 + SET-DEADBEEF + LDB-DST51 + LDB-OFF8-HANDLER + LDB-OFF127-HANDLER + LDB-OFFM128-HANDLER + LDB-OFF64-HANDLER + LDB-OFF16-HANDLER + LDB-OFF32-HANDLER + LDB-OFF96-HANDLER + LDB-OFF112-HANDLER + JMP + CALLBACK + CALLRET + LDB + LDB-off8 + LDB-off127 + LDB-off128 + LDB-off256 + LDB-offm128 + LDB-offm129 — primitive probes only)\`);
  process.exit(0);
}

main();
`;
  // keep everything before cut (through SET-LARGE function end)
  const head = g.slice(0, cut).replace(/\s+$/, '\n\n');
  writeUtf8(gPath, head + checks);
  console.log('golden.js rewritten for 8 batch-10 checks (52 cases)');
}

console.log('DONE atomic fix (ty+fixtures+js). Rust files still need verify.');
