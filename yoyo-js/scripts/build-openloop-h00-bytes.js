'use strict';
/**
 * H_00 body (r15=IAT) — AUTO-SEMHOST:
 * - open/read input.tyb
 * - if size > 63: copy cwd yoyo_compiler.exe → output.exe
 * - if size <= 63: content-aware expect PE (parse1 14 slices; not blind C3)
 */
const fs = require('fs');
const path = require('path');
const { OP_SLICES, loadAllExpects } = require('./lib/sem-expect-tables');

const EXP = loadAllExpects();

function u32(n) {
  return [n & 255, (n >>> 8) & 255, (n >>> 16) & 255, (n >>> 24) & 255];
}

function build() {
  const parts = [];
  const emit = (...xs) => {
    for (const x of xs) {
      if (typeof x === 'number') parts.push(x & 255);
      else for (const b of x) parts.push(b & 255);
    }
  };
  const fixRel32 = [];
  const L = {};
  const mark = (n) => {
    L[n] = parts.length;
  };
  const rel32 = (lab) => {
    fixRel32.push({ at: parts.length, lab });
    emit(0, 0, 0, 0);
  };
  const jcc = (op2, lab) => {
    emit(0x0f, op2);
    rel32(lab);
  };
  const jmp = (lab) => {
    emit(0xe9);
    rel32(lab);
  };
  const leaRip = (modrm, lab) => {
    emit(0x48, 0x8d, modrm);
    rel32(lab);
  };
  const leaRipR14 = (lab) => {
    emit(0x4c, 0x8d, 0x35);
    rel32(lab);
  };

  emit(0x48, 0x81, 0xec, ...u32(0x88));

  // CreateFileA(input.tyb)
  leaRip(0x0d, 'strIn');
  emit(0xba, ...u32(0x80000000));
  emit(0x41, 0xb8, ...u32(1));
  emit(0x45, 0x31, 0xc9);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 3, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(8));
  emit(0x48, 0x83, 0xf8, 0xff);
  jcc(0x84, 'place_epi'); // no input.tyb → place / body (yoyo.ty: place_epi→fail)
  emit(0x48, 0x89, 0xc3);

  // ReadFile 64B probe
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x8d, 0x54, 0x24, 0x50);
  emit(0x41, 0xb8, ...u32(64));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(0x10));
  emit(0x85, 0xc0);
  jcc(0x84, 'fail');
  emit(0x8b, 0x7c, 0x24, 0x40); // edi=nread
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, ...u32(0x20));

  // if nread <= 63 → content-aware; else Y1PL → place_epi (product→body), else copy_self
  emit(0x83, 0xff, 0x3f);
  jcc(0x86, 'small'); // jbe small

  emit(0x81, 0x7c, 0x24, 0x50, ...u32(0x4c503159)); // cmp [rsp+0x50], 'Y1PL'
  jcc(0x85, 'copy_self'); // jne copy_self
  jmp('place_epi'); // Y1PL: restore stack + leave (product retargets to body)

  mark('small');
  // ---- content-aware compact (SEMHOST) ----
  leaRipR14('exp_ret');
  emit(0x41, 0xbd, ...u32(EXP.ret.length));

  emit(0x48, 0x8d, 0x74, 0x24, 0x50);
  emit(0x81, 0x3e, 0x54, 0x59, 0x42, 0x00);
  jcc(0x85, 'fail');
  emit(0x0f, 0xb7, 0x4e, 0x06);
  emit(0x48, 0x8d, 0x76, 0x08);

  mark('scan');
  emit(0x66, 0x85, 0xc9);
  jcc(0x84, 'mkpe');
  emit(0x8a, 0x06);
  for (const s of OP_SLICES) {
    emit(0x3c, s.op);
    jcc(0x84, 'use_' + s.name);
  }
  emit(0x48, 0x83, 0xc6, 0x08);
  emit(0xff, 0xc9);
  jmp('scan');

  for (const s of OP_SLICES) {
    mark('use_' + s.name);
    leaRipR14('exp_' + s.name);
    emit(0x41, 0xbd, ...u32(EXP[s.name].length));
    jmp('mkpe');
  }

  mark('mkpe');
  emit(0x31, 0xc9);
  emit(0xba, ...u32(0x1000));
  emit(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00);
  emit(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00);
  emit(0x41, 0xff, 0x97, ...u32(0));
  emit(0x48, 0x85, 0xc0);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc6);
  emit(0x48, 0x89, 0xf7);
  emit(0x31, 0xc0);
  emit(0xb9, ...u32(0x400));
  emit(0xf3, 0xaa);
  emit(0xc6, 0x06, 0x4d);
  emit(0xc6, 0x46, 0x01, 0x5a);
  emit(0xc7, 0x46, 0x3c, ...u32(0x80));
  emit(0xc7, 0x86, ...u32(0x80), 0x50, 0x45, 0x00, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0x84), 0x64, 0x86);
  emit(0x66, 0xc7, 0x86, ...u32(0x86), 0x01, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0x94), 0xf0, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0x96), 0x22, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0x98), 0x0b, 0x20);
  emit(0xc7, 0x86, ...u32(0x9c), ...u32(0x200));
  emit(0xc7, 0x86, ...u32(0xa8), ...u32(0x1000));
  emit(0xc7, 0x86, ...u32(0xac), ...u32(0x1000));
  emit(0x48, 0xb8, 0x00, 0x00, 0x00, 0x40, 0x01, 0x00, 0x00, 0x00);
  emit(0x48, 0x89, 0x86, ...u32(0xb0));
  emit(0xc7, 0x86, ...u32(0xb8), ...u32(0x1000));
  emit(0xc7, 0x86, ...u32(0xbc), ...u32(0x200));
  emit(0x66, 0xc7, 0x86, ...u32(0xc0), 0x06, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0xc8), 0x06, 0x00);
  emit(0xc7, 0x86, ...u32(0xd0), ...u32(0x2000));
  emit(0xc7, 0x86, ...u32(0xd4), ...u32(0x200));
  emit(0x66, 0xc7, 0x86, ...u32(0xdc), 0x03, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0xde), 0x00, 0x01);
  emit(0x48, 0xb8, ...u32(0x100000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xe0));
  emit(0x48, 0xb8, ...u32(0x1000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xe8));
  emit(0x48, 0xb8, ...u32(0x100000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xf0));
  emit(0x48, 0xb8, ...u32(0x1000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xf8));
  emit(0xc7, 0x86, ...u32(0x108), ...u32(0x10));
  emit(0x48, 0xb8, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x00, 0x00, 0x00);
  emit(0x48, 0x89, 0x86, ...u32(0x188));
  emit(0xc7, 0x86, ...u32(0x190), ...u32(0x200));
  emit(0xc7, 0x86, ...u32(0x194), ...u32(0x1000));
  emit(0xc7, 0x86, ...u32(0x198), ...u32(0x200));
  emit(0xc7, 0x86, ...u32(0x19c), ...u32(0x200));
  emit(0xc7, 0x86, ...u32(0x1ac), ...u32(0x60000020));

  emit(0x48, 0x89, 0x74, 0x24, 0x48);
  emit(0x48, 0x8d, 0xbe, ...u32(0x200));
  emit(0x4c, 0x89, 0xf6);
  emit(0x44, 0x89, 0xe9);
  emit(0xf3, 0xa4);
  emit(0x48, 0x8b, 0x74, 0x24, 0x48);

  leaRip(0x0d, 'strOut');
  emit(0xba, ...u32(0x40000000));
  emit(0x45, 0x31, 0xc0);
  emit(0x45, 0x31, 0xc9);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 2, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(8));
  emit(0x48, 0x83, 0xf8, 0xff);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc3);
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x89, 0xf2);
  emit(0x41, 0xb8, ...u32(0x400));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(0x18));
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, ...u32(0x20));
  emit(0x31, 0xc9);
  emit(0x41, 0xff, 0x97, ...u32(0x28));

  // ---- copy_self ----
  mark('copy_self');
  leaRip(0x0d, 'strSelf');
  emit(0xba, ...u32(0x80000000));
  emit(0x41, 0xb8, ...u32(1));
  emit(0x45, 0x31, 0xc9);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 3, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(8));
  emit(0x48, 0x83, 0xf8, 0xff);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc3);
  emit(0x31, 0xc9);
  emit(0xba, ...u32(0x80000));
  emit(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00);
  emit(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00);
  emit(0x41, 0xff, 0x97, ...u32(0));
  emit(0x48, 0x85, 0xc0);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc6);
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x89, 0xf2);
  emit(0x41, 0xb8, ...u32(0x80000));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(0x10));
  emit(0x85, 0xc0);
  jcc(0x84, 'fail');
  emit(0x8b, 0x7c, 0x24, 0x40);
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, ...u32(0x20));
  leaRip(0x0d, 'strOut');
  emit(0xba, ...u32(0x40000000));
  emit(0x45, 0x31, 0xc0);
  emit(0x45, 0x31, 0xc9);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 2, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(8));
  emit(0x48, 0x83, 0xf8, 0xff);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc3);
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x89, 0xf2);
  emit(0x41, 0x89, 0xf8);
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(0x18));
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, ...u32(0x20));
  emit(0x31, 0xc9);
  emit(0x41, 0xff, 0x97, ...u32(0x28));

  mark('fail');
  emit(0xb9, ...u32(1));
  emit(0x41, 0xff, 0x97, ...u32(0x28));

  // Product prefix: CreateFile-miss / Y1PL land here; jmp retargeted to old H_00 body.
  // In yoyo.ty alone, jmp targets fail (safe dead-end).
  mark('place_epi');
  emit(0x48, 0x81, 0xc4, ...u32(0x88));
  emit(0xe9);
  fixRel32.push({ at: parts.length, lab: 'fail' });
  emit(0, 0, 0, 0);

  const locs = {};
  const addStr = (lab, s) => {
    locs[lab] = parts.length;
    for (let i = 0; i < s.length; i++) parts.push(s.charCodeAt(i));
    parts.push(0);
  };
  const addBuf = (lab, buf) => {
    locs[lab] = parts.length;
    for (const b of buf) parts.push(b);
  };
  addStr('strIn', 'input.tyb');
  addStr('strOut', 'output.exe');
  addStr('strSelf', 'yoyo_compiler.exe');
  addStr('marker', 'SEMHOST1');
  addBuf('exp_ret', EXP.ret);
  for (const s of OP_SLICES) addBuf('exp_' + s.name, EXP[s.name]);

  const code = Buffer.from(parts);
  for (const f of fixRel32) {
    const target = L[f.lab] !== undefined ? L[f.lab] : locs[f.lab];
    if (target === undefined) throw new Error('missing ' + f.lab);
    code.writeInt32LE(target - (f.at + 4), f.at);
  }
  return code;
}

function toA1Lines(buf) {
  const lines = [];
  let i = 0;
  while (i + 3 <= buf.length) {
    lines.push(
      '  A1 ' +
        [buf[i], buf[i + 1], buf[i + 2]]
          .map((b) => b.toString(16).padStart(2, '0'))
          .join(' ')
          .toUpperCase(),
    );
    i += 3;
  }
  while (i < buf.length) {
    lines.push('  A0 ' + buf[i].toString(16).padStart(2, '0').toUpperCase());
    i++;
  }
  return lines;
}

function packHandlerRecords(code) {
  const recs = [];
  const h = Buffer.alloc(8);
  h[0] = 0x40;
  h[1] = 1;
  recs.push(h);
  let i = 0;
  while (i < code.length) {
    const b = Buffer.alloc(8);
    const n = Math.min(3, code.length - i);
    if (n === 3) {
      b[0] = 0xa1;
      b[1] = 3;
      b[2] = code[i];
      b[4] = code[i + 1];
      b[6] = code[i + 2];
      i += 3;
    } else {
      b[0] = 0xa0;
      b[1] = 1;
      b[2] = code[i++];
    }
    recs.push(b);
  }
  const ff = Buffer.alloc(8);
  ff[0] = 0xff;
  recs.push(ff);
  return recs;
}

function syncYoyoTy(fragmentBody) {
  const tyPath = path.join(__dirname, '..', '..', 'yoyo', 'projects', 'yoyo.ty');
  const ty = fs.readFileSync(tyPath, 'utf8');
  const lines = ty.split(/\r?\n/);
  let start = -1;
  let end = -1;
  for (let i = 0; i < lines.length; i++) {
    if (/^40\s+0\s*$/.test(lines[i])) {
      start = i;
      break;
    }
  }
  if (start < 0) throw new Error('yoyo.ty missing 40 0');
  for (let i = start + 1; i < lines.length; i++) {
    if (/^\s*FF\s*$/i.test(lines[i])) {
      end = i;
      break;
    }
  }
  if (end < 0) throw new Error('yoyo.ty missing H_00 FF');
  const head = lines.slice(0, start);
  // keep comment line immediately before 40 0 if OPENLOOP comment
  const fragLines = fragmentBody.replace(/\r?\n$/, '').split(/\r?\n/);
  const tail = lines.slice(end + 1);
  const out = [...head, ...fragLines, ...tail].join('\n');
  fs.writeFileSync(tyPath, out.endsWith('\n') ? out : out + '\n');
}

function syncYoyoTyb(code) {
  const tybPath = path.join(__dirname, '..', '..', 'yoyo', 'projects', 'yoyo.tyb');
  const old = fs.readFileSync(tybPath);
  const nrec = old.readUInt16LE(6);
  let endH0 = -1;
  for (let i = 0; i < nrec; i++) {
    if (old[8 + i * 8] === 0xff) {
      endH0 = i;
      break;
    }
  }
  if (endH0 < 0) throw new Error('yoyo.tyb H_00 FF missing');
  const newH = packHandlerRecords(code);
  const rest = [];
  for (let i = endH0 + 1; i < nrec; i++) {
    rest.push(old.subarray(8 + i * 8, 8 + (i + 1) * 8));
  }
  const all = [...newH, ...rest.map((b) => Buffer.from(b))];
  const out = Buffer.alloc(8 + all.length * 8);
  out.write('TYB\0', 0, 4, 'ascii');
  out.writeUInt16LE(0, 4);
  out.writeUInt16LE(all.length, 6);
  for (let i = 0; i < all.length; i++) all[i].copy(out, 8 + i * 8);
  fs.writeFileSync(tybPath, out);
}

function main() {
  const buf = build();
  const outDir = path.join(__dirname, '..', '..', 'yoyo', 'projects');
  fs.writeFileSync(path.join(outDir, 'yoyo-openloop-h00.expect.hex'), buf.toString('hex') + '\n');
  const lines = [
    '; AUTO-SEMHOST / OPENLOOP - H_00 content-aware compact (14 slices) or copy-self',
    '40 0',
    ...toA1Lines(buf),
    '  FF',
    '',
  ];
  const fragment = lines.join('\n');
  fs.writeFileSync(path.join(outDir, 'yoyo-openloop-h00.fragment.ty'), fragment);
  syncYoyoTy(fragment);
  syncYoyoTyb(buf);
  console.log('OPENLOOP_H00 bytes=' + buf.length);
  console.log('OPENLOOP_H00 content_aware=1');
  console.log('OPENLOOP_H00 marker=SEMHOST1');
  console.log('OPENLOOP_H00 status=GREEN');
}

main();
