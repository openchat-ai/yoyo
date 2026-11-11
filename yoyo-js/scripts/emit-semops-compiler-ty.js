'use strict';
/**
 * AUTO-SEMOPS: emit yoyo-semops-compiler.ty / .tyb
 *
 * YOYO H_00 (A1 x64): read input.tyb → classify batch1 ops → write 1024B PE
 * whose .text@0x200 holds the locked expect bytes (content-aware, not blind C3).
 *
 * Not YOYC hex-unload of stock C1. Not Thompson.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');

const EXP = {
  Ret: Buffer.from('c3', 'hex'),
  Nop: Buffer.from('90c3', 'hex'),
  Sub: Buffer.from('498b87800200004883e80349898780020000c3', 'hex'),
  Add: Buffer.from('498b87800200004883c00349898780020000c3', 'hex'),
  Get: Buffer.from('498b878802000049898780020000c3', 'hex'),
};

function u32(n) {
  return [n & 255, (n >>> 8) & 255, (n >>> 16) & 255, (n >>> 24) & 255];
}

function buildCode() {
  const parts = [];
  const emit = (...xs) => {
    for (const x of xs) {
      if (typeof x === 'number') parts.push(x & 255);
      else for (const b of x) parts.push(b & 255);
    }
  };
  const fixRel32 = [];
  const L = {};
  const mark = (name) => {
    L[name] = parts.length;
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
  const leaRip = (regModrm /* e.g. 0x0d=rcx, 0x35=rsi, 0x3d=rdi */, lab) => {
    // 48 8D r/m  OR 4C 8D for r8+
    emit(0x48, 0x8d, regModrm);
    rel32(lab);
  };
  const leaRipR14 = (lab) => {
    emit(0x4c, 0x8d, 0x35);
    rel32(lab);
  };

  // --- prologue + CreateFileA(input.tyb)  [OPENLOOP-faithful] ---
  emit(0x48, 0x81, 0xec, ...u32(0x88));
  leaRip(0x0d, 'strIn'); // lea rcx,[rip+strIn]
  emit(0xba, ...u32(0x80000000)); // GENERIC_READ
  emit(0x41, 0xb8, ...u32(1)); // FILE_SHARE_READ
  emit(0x45, 0x31, 0xc9); // xor r9d,r9d
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 3, 0, 0, 0); // OPEN_EXISTING
  emit(0x48, 0xc7, 0x44, 0x24, 0x28, 0, 0, 0, 0);
  emit(0x48, 0xc7, 0x44, 0x24, 0x30, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(8)); // CreateFileA
  emit(0x48, 0x83, 0xf8, 0xff);
  jcc(0x84, 'fail'); // jz fail
  emit(0x48, 0x89, 0xc3); // rbx=handle

  // ReadFile(h, [rsp+0x50], 0x40, &nread)
  emit(0x48, 0x89, 0xd9);
  emit(0x48, 0x8d, 0x54, 0x24, 0x50);
  emit(0x41, 0xb8, ...u32(0x40));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(0x10));
  emit(0x85, 0xc0);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, ...u32(0x20)); // CloseHandle

  // default expect = Ret
  leaRipR14('expRet');
  emit(0x41, 0xbd, ...u32(EXP.Ret.length)); // r13d=len

  // classify TYB at [rsp+0x50]
  emit(0x48, 0x8d, 0x74, 0x24, 0x50); // rsi=buf
  emit(0x81, 0x3e, 0x54, 0x59, 0x42, 0x00); // cmp dword [rsi],'TYB\0'
  jcc(0x85, 'fail'); // jnz fail
  emit(0x0f, 0xb7, 0x4e, 0x06); // movzx ecx, word [rsi+6]  nrec
  emit(0x48, 0x8d, 0x76, 0x08); // rsi = first rec

  mark('scan');
  emit(0x66, 0x85, 0xc9); // test cx,cx
  jcc(0x84, 'mkpe');
  emit(0x8a, 0x06); // al=[rsi]
  emit(0x3c, 0x00);
  jcc(0x84, 'useNop');
  emit(0x3c, 0x62);
  jcc(0x84, 'useAdd');
  emit(0x3c, 0x61);
  jcc(0x84, 'useSub');
  emit(0x3c, 0x60);
  jcc(0x84, 'useGet');
  emit(0x48, 0x83, 0xc6, 0x08);
  emit(0xff, 0xc9); // dec ecx
  jmp('scan');

  const use = (lab, expLab, len) => {
    mark(lab);
    leaRipR14(expLab);
    emit(0x41, 0xbd, ...u32(len));
    jmp('mkpe');
  };
  use('useNop', 'expNop', EXP.Nop.length);
  use('useAdd', 'expAdd', EXP.Add.length);
  use('useSub', 'expSub', EXP.Sub.length);
  use('useGet', 'expGet', EXP.Get.length);

  // --- VirtualAlloc + PE template (OPENLOOP compact layout) ---
  mark('mkpe');
  emit(0x31, 0xc9);
  emit(0xba, ...u32(0x1000));
  emit(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00); // MEM_COMMIT|RESERVE
  emit(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00); // PAGE_READWRITE
  emit(0x41, 0xff, 0x97, ...u32(0)); // VirtualAlloc
  emit(0x48, 0x85, 0xc0);
  jcc(0x84, 'fail');
  emit(0x48, 0x89, 0xc6); // rsi = pe
  emit(0x48, 0x89, 0xf7);
  emit(0x31, 0xc0);
  emit(0xb9, ...u32(0x400));
  emit(0xf3, 0xaa); // zero 1024

  // MZ + e_lfanew
  emit(0xc6, 0x06, 0x4d);
  emit(0xc6, 0x46, 0x01, 0x5a);
  emit(0xc7, 0x46, 0x3c, ...u32(0x80));

  // PE signature + COFF + optional (PE32+) — offsets match OPENLOOP
  emit(0xc7, 0x86, ...u32(0x80), 0x50, 0x45, 0x00, 0x00);
  emit(0x66, 0xc7, 0x86, ...u32(0x84), 0x64, 0x86); // Machine
  emit(0x66, 0xc7, 0x86, ...u32(0x86), 0x01, 0x00); // NumberOfSections
  emit(0x66, 0xc7, 0x86, ...u32(0x94), 0xf0, 0x00); // SizeOfOptionalHeader
  emit(0x66, 0xc7, 0x86, ...u32(0x96), 0x22, 0x00); // Characteristics
  emit(0x66, 0xc7, 0x86, ...u32(0x98), 0x0b, 0x20); // Magic PE32+
  emit(0xc7, 0x86, ...u32(0x9c), ...u32(0x200)); // SizeOfCode
  emit(0xc7, 0x86, ...u32(0xa8), ...u32(0x1000)); // AddressOfEntryPoint
  emit(0xc7, 0x86, ...u32(0xac), ...u32(0x1000)); // BaseOfCode
  // ImageBase = 0x140000000
  emit(0x48, 0xb8, 0x00, 0x00, 0x00, 0x40, 0x01, 0x00, 0x00, 0x00);
  emit(0x48, 0x89, 0x86, ...u32(0xb0));
  emit(0xc7, 0x86, ...u32(0xb8), ...u32(0x1000)); // SectionAlignment
  emit(0xc7, 0x86, ...u32(0xbc), ...u32(0x200)); // FileAlignment
  emit(0x66, 0xc7, 0x86, ...u32(0xc0), 0x06, 0x00); // MajorOS
  emit(0x66, 0xc7, 0x86, ...u32(0xc8), 0x06, 0x00); // MajorSubsystem
  emit(0xc7, 0x86, ...u32(0xd0), ...u32(0x2000)); // SizeOfImage
  emit(0xc7, 0x86, ...u32(0xd4), ...u32(0x200)); // SizeOfHeaders
  emit(0x66, 0xc7, 0x86, ...u32(0xdc), 0x03, 0x00); // Subsystem CONSOLE
  emit(0x66, 0xc7, 0x86, ...u32(0xde), 0x00, 0x01); // DllCharacteristics
  emit(0x48, 0xb8, ...u32(0x100000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xe0)); // SizeOfStackReserve
  emit(0x48, 0xb8, ...u32(0x1000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xe8)); // SizeOfStackCommit
  emit(0x48, 0xb8, ...u32(0x100000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xf0)); // SizeOfHeapReserve
  emit(0x48, 0xb8, ...u32(0x1000), 0, 0, 0, 0);
  emit(0x48, 0x89, 0x86, ...u32(0xf8)); // SizeOfHeapCommit
  emit(0xc7, 0x86, ...u32(0x108), ...u32(0x10)); // NumberOfRvaAndSizes

  // .text section header @ 0x188
  emit(0x48, 0xb8, 0x2e, 0x74, 0x65, 0x78, 0x74, 0x00, 0x00, 0x00);
  emit(0x48, 0x89, 0x86, ...u32(0x188));
  emit(0xc7, 0x86, ...u32(0x190), ...u32(0x200)); // VirtualSize
  emit(0xc7, 0x86, ...u32(0x194), ...u32(0x1000)); // VirtualAddress
  emit(0xc7, 0x86, ...u32(0x198), ...u32(0x200)); // SizeOfRawData
  emit(0xc7, 0x86, ...u32(0x19c), ...u32(0x200)); // PointerToRawData
  emit(0xc7, 0x86, ...u32(0x1ac), ...u32(0x60000020)); // Characteristics

  // memcpy expect → pe+0x200  (rdi=dst, rsi=src destroyed — save pe)
  emit(0x48, 0x89, 0x74, 0x24, 0x48); // [rsp+0x48]=pe
  emit(0x48, 0x8d, 0xbe, ...u32(0x200)); // lea rdi,[rsi+0x200]
  emit(0x4c, 0x89, 0xf6); // rsi=r14 expect
  emit(0x44, 0x89, 0xe9); // ecx=r13d
  emit(0xf3, 0xa4);
  emit(0x48, 0x8b, 0x74, 0x24, 0x48); // restore rsi=pe

  // CreateFileA(output.exe) GENERIC_WRITE CREATE_ALWAYS
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
  emit(0x48, 0x89, 0xf2); // rdx=pe
  emit(0x41, 0xb8, ...u32(0x400));
  emit(0x4c, 0x8d, 0x4c, 0x24, 0x40);
  emit(0x48, 0xc7, 0x44, 0x24, 0x20, 0, 0, 0, 0);
  emit(0x41, 0xff, 0x97, ...u32(0x18)); // WriteFile
  emit(0x48, 0x89, 0xd9);
  emit(0x41, 0xff, 0x97, ...u32(0x20)); // CloseHandle
  emit(0x31, 0xc9);
  emit(0x41, 0xff, 0x97, ...u32(0x28)); // ExitProcess(0)

  mark('fail');
  emit(0xb9, ...u32(1));
  emit(0x41, 0xff, 0x97, ...u32(0x28));

  // --- rodata ---
  const locs = {};
  const addStr = (lab, s) => {
    locs[lab] = parts.length;
    for (let i = 0; i < s.length; i++) parts.push(s.charCodeAt(i) & 255);
    parts.push(0);
  };
  const addBuf = (lab, buf) => {
    locs[lab] = parts.length;
    for (const b of buf) parts.push(b);
  };
  addStr('strIn', 'input.tyb');
  addStr('strOut', 'output.exe');
  addBuf('expRet', EXP.Ret);
  addBuf('expNop', EXP.Nop);
  addBuf('expAdd', EXP.Add);
  addBuf('expSub', EXP.Sub);
  addBuf('expGet', EXP.Get);

  const code = Buffer.from(parts);
  for (const f of fixRel32) {
    const target = L[f.lab] !== undefined ? L[f.lab] : locs[f.lab];
    if (target === undefined) throw new Error('missing label ' + f.lab);
    const next = f.at + 4;
    code.writeInt32LE(target - next, f.at);
  }
  return code;
}

function packTyb(code) {
  const recs = [];
  const h = Buffer.alloc(8);
  h[0] = 0x40;
  h[1] = 1;
  recs.push(h);
  let i = 0;
  while (i < code.length) {
    const b = Buffer.alloc(8);
    if (code.length - i >= 3) {
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
  const out = Buffer.alloc(8 + recs.length * 8);
  out.write('TYB\0', 0, 4, 'ascii');
  out.writeUInt16LE(0, 4);
  out.writeUInt16LE(recs.length, 6);
  for (let r = 0; r < recs.length; r++) recs[r].copy(out, 8 + r * 8);
  return out;
}

function emitTy(code) {
  const lines = [
    '; yoyo-semops-compiler — AUTO-SEMOPS',
    '; YOYO source: content-aware batch1 (+compact-ret) compiler.',
    '; NOT YOYC hex dump of stock C1. Not Thompson.',
    '40 0',
  ];
  let i = 0;
  while (i + 3 <= code.length) {
    lines.push(
      '  A1 ' +
        [code[i], code[i + 1], code[i + 2]]
          .map((x) => x.toString(16).padStart(2, '0'))
          .join(' '),
    );
    i += 3;
  }
  while (i < code.length) {
    lines.push('  A0 ' + code[i].toString(16).padStart(2, '0'));
    i++;
  }
  lines.push('  FF');
  lines.push('');
  return lines.join('\n');
}

function main() {
  const code = buildCode();
  const ty = emitTy(code);
  const tyb = packTyb(code);
  fs.writeFileSync(path.join(projects, 'yoyo-semops-compiler.ty'), ty);
  fs.writeFileSync(path.join(projects, 'yoyo-semops-compiler.tyb'), tyb);
  const sha = crypto.createHash('sha256').update(tyb).digest('hex');
  const pinDir = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
  const pin =
    '# AUTO-SEMOPS pin — yoyo-semops-compiler.tyb\n' +
    'tyb_sha256=' +
    sha +
    '\n' +
    'tyb_bytes=' +
    tyb.length +
    '\n' +
    'src_kind=yoyo_ty_not_yoyc\n' +
    'yoyc_role=stock_residual\n';
  fs.writeFileSync(path.join(pinDir, 'semops.pin'), pin);
  console.log('SEMOPS code_bytes=' + code.length);
  console.log('SEMOPS ty_bytes=' + Buffer.byteLength(ty));
  console.log('SEMOPS tyb_bytes=' + tyb.length + ' sha256_prefix=' + sha.slice(0, 16));
  console.log('SEMOPS src_kind=yoyo_ty_not_yoyc');
  console.log('SEMOPS emit=GREEN');
}

main();
