'use strict';
/**
 * platform-io.js — JS peer of yoyo-rust/verifier/src/platform_io.rs (Stage 9-B).
 * Win32: kernel32 IAT at [r15 + slot*8]; Linux: inline syscalls; Stub: movabs+store.
 */

const { loadState, storeState, movabsRax } = require('./encode-x64');

const WIN32_IAT_VIRTUAL_ALLOC = 0;
const WIN32_IAT_CREATE_FILE = 1;
const WIN32_IAT_READ_FILE = 2;
const WIN32_IAT_WRITE_FILE = 3;
const WIN32_IAT_CLOSE_HANDLE = 4;

const STR_TABLE_OFF = 0x10000;
const STR_ENTRY_SIZE = 64;
const READ_CHUNK = 0x10000;

/** Kernel32 imports prepended at r15+0 (pe-builder / pe_link). */
const KERNEL32_IO_FUNCS = [
  'VirtualAlloc',
  'CreateFileA',
  'ReadFile',
  'WriteFile',
  'CloseHandle',
  // Deeper OW-IAT: dropped LoadLibraryA from IAT — H_00 manual-map resolves via PEB (Rust peer).
  'ExitProcess',
];

function u32le(n) {
  const b = Buffer.alloc(4);
  b.writeUInt32LE(n >>> 0, 0);
  return [...b];
}

function emitCallR15Iat(slot) {
  return [0x41, 0xff, 0x97, ...u32le(slot * 8)];
}

function emitLeaR15(destLow3, destRex, disp) {
  const rex = 0x49 | (destRex ? 0x04 : 0);
  return [rex, 0x8d, 0x87 | ((destLow3 & 7) << 3), ...u32le(disp)];
}

function strPathOff(strIdx) {
  return STR_TABLE_OFF + (strIdx & 0xff) * STR_ENTRY_SIZE;
}

function emitStubIo(slot, imm) {
  return [...movabsRax(imm || 0), ...storeState(slot, 0, 0)];
}

function emitWin32Alloc(slot, size) {
  const out = [];
  out.push(0x48, 0x83, 0xec, 0x28); // sub rsp, 0x28
  out.push(0x31, 0xc9); // xor ecx, ecx
  if (size <= 0xffffffff) {
    out.push(0xba, ...u32le(size));
  } else {
    // movabs rdx, size
    const buf = Buffer.alloc(8);
    buf.writeBigUInt64LE(BigInt.asUintN(64, BigInt(size)), 0);
    out.push(0x48, 0xba, ...buf);
  }
  out.push(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00); // r8 = MEM_COMMIT|RESERVE
  out.push(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00); // r9 = PAGE_READWRITE
  out.push(...emitCallR15Iat(WIN32_IAT_VIRTUAL_ALLOC));
  out.push(0x48, 0x83, 0xc4, 0x28);
  out.push(...storeState(slot, 0, 0));
  return out;
}

function emitWin32LoadFile(slot, strIdx) {
  const path = strPathOff(strIdx);
  const out = [];
  out.push(0x48, 0x83, 0xec, 0x28);
  out.push(...emitLeaR15(1, 0, path)); // lea rcx, [r15+path]
  out.push(0xba, 0x00, 0x00, 0x00, 0x80); // GENERIC_READ
  out.push(0x45, 0x31, 0xc0); // xor r8d, r8d
  out.push(0x45, 0x31, 0xc9); // xor r9d, r9d
  out.push(0xc7, 0x44, 0x24, 0x20, 0x03, 0x00, 0x00, 0x00); // OPEN_EXISTING
  out.push(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00);
  out.push(0x48, 0xc7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00);
  out.push(...emitCallR15Iat(WIN32_IAT_CREATE_FILE));
  out.push(0x48, 0x89, 0xc3); // mov rbx, rax

  out.push(0x31, 0xc9);
  out.push(0xba, ...u32le(READ_CHUNK));
  out.push(0x41, 0xb8, 0x00, 0x30, 0x00, 0x00);
  out.push(0x41, 0xb9, 0x04, 0x00, 0x00, 0x00);
  out.push(...emitCallR15Iat(WIN32_IAT_VIRTUAL_ALLOC));
  out.push(0x48, 0x89, 0xc6); // mov rsi, rax

  out.push(0x48, 0x89, 0xd9);
  out.push(0x48, 0x89, 0xf2);
  out.push(0x41, 0xb8, ...u32le(READ_CHUNK));
  out.push(0x4c, 0x8d, 0x4c, 0x24, 0x20);
  out.push(0x48, 0xc7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00);
  out.push(...emitCallR15Iat(WIN32_IAT_READ_FILE));

  out.push(0x48, 0x89, 0xd9);
  out.push(...emitCallR15Iat(WIN32_IAT_CLOSE_HANDLE));
  out.push(0x48, 0x83, 0xc4, 0x28);
  out.push(0x48, 0x89, 0xf0);
  out.push(...storeState(slot, 0, 0));
  return out;
}

function emitWin32WriteFile(slot, strIdx, szSlot) {
  const path = strPathOff(strIdx);
  const out = [];
  out.push(0x48, 0x83, 0xec, 0x28);
  out.push(...emitLeaR15(1, 0, path));
  out.push(0xba, 0x00, 0x00, 0x00, 0x40); // GENERIC_WRITE
  out.push(0x45, 0x31, 0xc0);
  out.push(0x45, 0x31, 0xc9);
  out.push(0xc7, 0x44, 0x24, 0x20, 0x02, 0x00, 0x00, 0x00); // CREATE_ALWAYS
  out.push(0x48, 0xc7, 0x44, 0x24, 0x28, 0x80, 0x00, 0x00, 0x00);
  out.push(0x48, 0xc7, 0x44, 0x24, 0x30, 0x00, 0x00, 0x00, 0x00);
  out.push(...emitCallR15Iat(WIN32_IAT_CREATE_FILE));
  out.push(0x48, 0x89, 0xc3);

  out.push(...loadState(slot, 2, 0)); // rdx = buf
  out.push(...loadState(szSlot, 0, 1)); // r8 = size (REX.R)
  out.push(0x48, 0x89, 0xd9);
  out.push(0x4c, 0x8d, 0x4c, 0x24, 0x20);
  out.push(0x48, 0xc7, 0x44, 0x24, 0x28, 0x00, 0x00, 0x00, 0x00);
  out.push(...emitCallR15Iat(WIN32_IAT_WRITE_FILE));

  out.push(0x48, 0x89, 0xd9);
  out.push(...emitCallR15Iat(WIN32_IAT_CLOSE_HANDLE));
  out.push(0x48, 0x83, 0xc4, 0x28);
  out.push(...movabsRax(0));
  out.push(...storeState(slot, 0, 0));
  return out;
}

function emitLinuxAlloc(slot, size) {
  const out = [];
  out.push(0x48, 0x31, 0xff); // xor rdi, rdi
  if (size <= 0xffffffff) {
    out.push(0x48, 0xc7, 0xc6, ...u32le(size));
  } else {
    const buf = Buffer.alloc(8);
    buf.writeBigUInt64LE(BigInt.asUintN(64, BigInt(size)), 0);
    out.push(0x48, 0xbe, ...buf);
  }
  out.push(0x48, 0xc7, 0xc2, 0x03, 0x00, 0x00, 0x00);
  out.push(0x49, 0xc7, 0xc2, 0x22, 0x00, 0x00, 0x00);
  out.push(0x49, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff);
  out.push(0x4d, 0x31, 0xc9);
  out.push(0xb8, 0x09, 0x00, 0x00, 0x00);
  out.push(0x0f, 0x05);
  out.push(...storeState(slot, 0, 0));
  return out;
}

function emitLinuxLoadFile(slot, strIdx) {
  const path = strPathOff(strIdx);
  const out = [];
  out.push(...emitLeaR15(7, 0, path)); // lea rdi
  out.push(0x31, 0xf6);
  out.push(0xb8, 0x02, 0x00, 0x00, 0x00);
  out.push(0x0f, 0x05);
  out.push(0x49, 0x89, 0xc4); // mov r12, rax

  out.push(0x48, 0x31, 0xff);
  out.push(0xbe, ...u32le(READ_CHUNK));
  out.push(0x48, 0xc7, 0xc2, 0x03, 0x00, 0x00, 0x00);
  out.push(0x49, 0xc7, 0xc2, 0x22, 0x00, 0x00, 0x00);
  out.push(0x49, 0xc7, 0xc0, 0xff, 0xff, 0xff, 0xff);
  out.push(0x4d, 0x31, 0xc9);
  out.push(0xb8, 0x09, 0x00, 0x00, 0x00);
  out.push(0x0f, 0x05);
  out.push(0x49, 0x89, 0xc5);

  out.push(0xb8, 0x00, 0x00, 0x00, 0x00);
  out.push(0x4c, 0x89, 0xe7);
  out.push(0x4c, 0x89, 0xee);
  out.push(0xba, ...u32le(READ_CHUNK));
  out.push(0x0f, 0x05);

  out.push(0x4c, 0x89, 0xe7);
  out.push(0xb8, 0x03, 0x00, 0x00, 0x00);
  out.push(0x0f, 0x05);

  out.push(...movabsRax(0));
  out.push(0x4c, 0x89, 0xe8);
  out.push(...storeState(slot, 0, 0));
  return out;
}

function emitLinuxWriteFile(slot, strIdx, szSlot) {
  const path = strPathOff(strIdx);
  const out = [];
  out.push(...emitLeaR15(7, 0, path));
  out.push(0xbe, 0x41, 0x02, 0x00, 0x00);
  out.push(0xba, 0xb6, 0x01, 0x00, 0x00);
  out.push(0xb8, 0x02, 0x00, 0x00, 0x00);
  out.push(0x0f, 0x05);
  out.push(0x49, 0x89, 0xc4);

  out.push(...loadState(slot, 6, 0)); // rsi
  out.push(...loadState(szSlot, 2, 0)); // rdx
  out.push(0x4c, 0x89, 0xe7);
  out.push(0xb8, 0x01, 0x00, 0x00, 0x00);
  out.push(0x0f, 0x05);

  out.push(0x4c, 0x89, 0xe7);
  out.push(0xb8, 0x03, 0x00, 0x00, 0x00);
  out.push(0x0f, 0x05);

  out.push(...movabsRax(0));
  out.push(...storeState(slot, 0, 0));
  return out;
}

/**
 * Encode ALLOC / LOAD_FILE / WRITE_FILE for a platform.
 * @param {number} op 0x20 | 0x50 | 0x51
 * @param {number[]} args
 * @param {'stub'|'win32'|'linux'} platform
 */
function encodeIoOp(op, args, platform) {
  const slot = args[0] || 0;
  const a1 = args[1] || 0;
  const a2 = args[2] || 0;
  const p = platform || 'stub';
  if (p === 'stub') {
    return emitStubIo(slot, a1);
  }
  if (p === 'win32') {
    if (op === 0x20) return emitWin32Alloc(slot, a1);
    if (op === 0x50) return emitWin32LoadFile(slot, a1);
    if (op === 0x51) return emitWin32WriteFile(slot, a1, a2);
  }
  if (p === 'linux') {
    if (op === 0x20) return emitLinuxAlloc(slot, a1);
    if (op === 0x50) return emitLinuxLoadFile(slot, a1);
    if (op === 0x51) return emitLinuxWriteFile(slot, a1, a2);
  }
  return emitStubIo(slot, a1);
}

/** True if bytes look like movabs+store stub (blind-zone signature). */
function isMovabsStoreStub(bytes) {
  const b = Buffer.from(bytes);
  return b.length >= 12 && b[0] === 0x48 && b[1] === 0xb8;
}

module.exports = {
  encodeIoOp,
  emitWin32Alloc,
  emitWin32LoadFile,
  emitWin32WriteFile,
  emitLinuxAlloc,
  emitLinuxLoadFile,
  emitLinuxWriteFile,
  emitStubIo,
  isMovabsStoreStub,
  KERNEL32_IO_FUNCS,
  WIN32_IAT_VIRTUAL_ALLOC,
  STR_TABLE_OFF,
  STR_ENTRY_SIZE,
};
