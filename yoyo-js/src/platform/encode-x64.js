'use strict';
/**
 * encode-x64.js — JS portion of platform-emit (entity 6).
 * Mirrors yoyo-rust assembler primitives for the 22 executor opcodes.
 *
 * Platform I/O (0x20/0x50/0x51): default `stub` (movabs+store) for golden;
 * production PE path sets `win32` via setEmitPlatform (Stage 9-B).
 */

/** @type {'stub'|'win32'|'linux'} */
let emitPlatform = 'stub';

function setEmitPlatform(p) {
  if (p !== 'stub' && p !== 'win32' && p !== 'linux') {
    throw new Error('emitPlatform must be stub|win32|linux');
  }
  emitPlatform = p;
}

function getEmitPlatform() {
  return emitPlatform;
}

function rex(w, r, x, b) {
  return 0x40 | (w << 3) | (r << 2) | (x << 1) | b;
}

function loadState(slot, destLow3, destRex) {
  const disp = (slot & 0xff) * 8;
  const r = rex(1, destRex, 0, 1);
  const modrmReg = (destLow3 & 7) << 3;
  if (disp <= 127) return [r, 0x8b, modrmReg | 0x40 | 0x07, disp];
  const d = Buffer.alloc(4); d.writeUInt32LE(disp, 0);
  return [r, 0x8b, modrmReg | 0x80 | 0x07, ...d];
}

function storeState(slot, srcLow3, srcRex) {
  const disp = (slot & 0xff) * 8;
  const r = rex(1, srcRex, 0, 1);
  const modrmReg = (srcLow3 & 7) << 3;
  if (disp <= 127) return [r, 0x89, modrmReg | 0x40 | 0x07, disp];
  const d = Buffer.alloc(4); d.writeUInt32LE(disp, 0);
  return [r, 0x89, modrmReg | 0x80 | 0x07, ...d];
}

function movabsRax(imm) {
  const buf = Buffer.alloc(8);
  buf.writeBigUInt64LE(BigInt.asUintN(64, BigInt(imm)), 0);
  return [0x48, 0xb8, ...buf];
}

function addRegRaxRcx() { return [0x48, 0x01, 0xc8]; }
function orRegRaxRcx() { return [0x48, 0x09, 0xc8]; }
function subRegRaxRcx() { return [0x48, 0x29, 0xc8]; }
function mulRegRaxRcx() { return [0x48, 0x0f, 0xaf, 0xc1]; }
function cmpRegRaxRcx() { return [0x48, 0x39, 0xc8]; }
function incRax() { return [0x48, 0xff, 0xc0]; }
function decRax() { return [0x48, 0xff, 0xc8]; }
function addImmRax(imm) {
  if (imm >= -128 && imm <= 127) return [0x48, 0x83, 0xc0, imm & 0xff];
  const b = Buffer.alloc(4); b.writeInt32LE(imm, 0);
  return [0x48, 0x81, 0xc0, ...b];
}
function subImmRax(imm) {
  if (imm >= -128 && imm <= 127) return [0x48, 0x83, 0xe8, imm & 0xff];
  const b = Buffer.alloc(4); b.writeInt32LE(imm, 0);
  return [0x48, 0x81, 0xe8, ...b];
}

const JCC = {
  0x71: 0x84, 0x72: 0x85, 0x73: 0x8c, 0x74: 0x8d, 0x75: 0x8e,
  0x76: 0x8f, 0x77: 0x82, 0x78: 0x83, 0x79: 0x86, 0x7a: 0x87,
};

function encodeOp(op, args, branchPlaceholder) {
  const a = (i) => (args[i] || 0);
  if (op === 0xff) return [0xc3];
  if (op === 0x00) return [0x90];
  if (op === 0xa0) return [a(0) & 0xff];
  if (op === 0xa1) return args.map((x) => x & 0xff);
  if (op === 0x41 || op === 0x70) {
    return branchPlaceholder ? (op === 0x41 ? [0xe8, 0, 0, 0, 0] : [0xe9, 0, 0, 0, 0]) : [0xc3];
  }
  if (op >= 0x71 && op <= 0x7a) {
    return branchPlaceholder ? [0x0f, JCC[op], 0, 0, 0, 0] : [0/c3];
  }
  if (op === 0x30) {
    return [...movabsRax(a(1)), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x60) {
    return [...loadState(a(1), 0, 0), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x64) {
    return [...loadState(a(1), 0, 0), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x61) {
    return [...loadState(a(0), 0, 0), ...subImmRax(a(1)), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x62) {
    return [...loadState(a(0), 0, 0), ...addImmRax(a(1)), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x63) {
    return [...loadState(a(0), 0, 0), ...loadState(a(1), 1, 0), ...mulRegRaxRcx(), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x65) {
    return [...loadState(a(0), 0, 0), ...loadState(a(1), 1, 0), ...cmpRegRaxRcx()];
  }
  if (op === 0x66) {
    return [...loadState(a(0), 0, 0), ...incRax(), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x67) {
    return [...loadState(a(0), 0, 0), ...decRax(), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x68) {
    return [...loadState(a(0), 0, 0), ...loadState(a(1), 1, 0), ...addRegRaxRcx(), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x69) {
    return [...loadState(a(0), 0, 0), ...loadState(a(1), 1, 0), ...orRegRaxRcx(), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x6a) {
    return [...loadState(a(0), 0, 0), ...loadState(a(1), 1, 0), ...subRegRaxRcx(), ...storeState(a(0), 0, 0)];
  }
  if (op === 0x80) {
    const out = [...loadState(a(1), 0, 0)];
    if (a(2)) out.push(...addImmRax(a(2)));
    out.push(0x48, 0x0f, 0xb6, 0x00);
    out.push(...storeState(a(0), 0, 0));
    return out;
  }
  if (op === 0x20 || op === 0x50 || op === 0x51) {
    // Lazy require avoids cycle with platform-io.js (uses loadState/storeState).
    const { encodeIoOp } = require('./platform-io');
    return encodeIoOp(op, args, emitPlatform);
  }
  // MEMCPY_DATA: canonical `rep movsb` (cross-peer DDC).
  // Load src→RSI, dst→RDI, n→RCX; rep movsb = 0xFC.
  // REX matches Rust verifier loadState: W=1, X=0, B=1 → 0x49
  // (RSI/RDI via modrm reg field low3=6/7, no REX.R).
  if (op === 0x84) {
    return [
      ...loadState(a(1), 6, 0), // RSI = src
      ...loadState(a(0), 7, 0), // RDI = dst
      ...loadState(a(2), 1, 0), // RCX = count
      0xfc,
    ];
  }
  // MEMCPY_STATE: same as DATA but src/dst are *slot indices*, so scale to
  // byte-address with `lea reg,[r15+reg*8]` before the copy.
  // Order mirrors Rust: dst→RDI (lea scale) then src→RSI (lea scale) then n→RCX.
  if (op === 0x85) {
    return [
      ...loadState(a(0), 7, 0),  // RDI = dst slot index
      ...leaR15Scale8(7, 0),     // RDI = &S[dst]
      ...loadState(a(1), 6, 0),  // RSI = src slot index
      ...leaR15Scale8(6, 0),     // RSI = &S[src]
      ...loadState(a(2), 1, 0),  // RCX = byte count
      0xfc,
    ];
  }
  return [0x90];
}

/**
 * Emits `lea <reg>, [r15 + <reg>*8]` (7B).
 */
function leaR15Scale8(regLow3, rexR) {
  const r = regLow3 & 7;
  const rex = 0x40 | 8 | (rexR ? 4 : 0) | 1; // W=1, R=rexR, X=0, B=1 (R15 base)
  const modrm = (r << 3) | 0x04;         // mod=00, rm=100 (SIB)
  const sib = (3 << 6) | ((r & 7) << 3) | 7; // scale=8 (11), index=r, base=111 (R15)
  return [rex, 0x8b, modrm, sib, 0x00, 0x00, 0x00]; // disp32 = 0
}

module.exports = {
  encodeOp,
  loadState,
  storeState,
  movabsRax,
  setEmitPlatform,
  getEmitPlatform,
};
