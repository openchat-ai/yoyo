'use strict';
/**
 * FX-2d-7c: x64 snippets planted into mini-PE .text — built from ISA semantics.
 * Store packing (abs64/imm32/…) follows the historical chunking so code.bin EQUAL holds.
 */

const TEXT_OFF = 0x40d;

// --- ISA builders (r15 = state base) ---
const I = {
  nop: () => [0x90],
  ret: () => [0xc3],
  mov_rax_m_r15: (d8) => [0x49, 0x8b, 0x47, d8 & 0xff],
  mov_rcx_m_r15: (d8) => [0x49, 0x8b, 0x4f, d8 & 0xff],
  mov_m_r15_rax: (d8) => [0x49, 0x89, 0x47, d8 & 0xff],
  inc_rax: () => [0x48, 0xff, 0xc0],
  dec_rax: () => [0x48, 0xff, 0xc8],
  add_rax_imm32: (imm) => [0x48, 0x81, 0xc0, imm & 0xff, (imm >> 8) & 0xff, (imm >> 16) & 0xff, (imm >> 24) & 0xff],
  sub_rax_imm32: (imm) => [0x48, 0x81, 0xe8, imm & 0xff, (imm >> 8) & 0xff, (imm >> 16) & 0xff, (imm >> 24) & 0xff],
  cmp_rax_rcx: () => [0x48, 0x39, 0xc8],
  /** movabs rax, imm64 (placeholder 0xdeadbeef) */
  movabs_rax_imm64: (lo32 = 0xdeadbeef, hi32 = 0) => {
    const b = [0x48, 0xb8];
    for (let i = 0; i < 4; i++) b.push((lo32 >> (8 * i)) & 0xff);
    for (let i = 0; i < 4; i++) b.push((hi32 >> (8 * i)) & 0xff);
    return b;
  },
};

function cat(...parts) {
  const o = [];
  for (const p of parts) o.push(...p);
  return o;
}

/** Pack contiguous program into store ops with fixed chunk sizes (EQUAL schedule). */
function pack(program, chunks, base = TEXT_OFF) {
  let i = 0;
  let off = base;
  const stores = [];
  for (const n of chunks) {
    if (i + n > program.length) {
      throw new Error(`pack overrun need=${n} left=${program.length - i}`);
    }
    const bytes = program.slice(i, i + n);
    i += n;
    if (n === 8) stores.push({ op: 'abs64', off, bytes });
    else if (n === 4) stores.push({ op: 'imm32', off, bytes });
    else if (n === 2) stores.push({ op: 'imm16', off, bytes });
    else if (n === 1) stores.push({ op: 'imm8', off, bytes });
    else throw new Error(`bad chunk ${n}`);
    off += n;
  }
  if (i !== program.length) {
    throw new Error(`pack leftover ${program.length - i}`);
  }
  return stores;
}

// --- emit1 planted programs (slot0 / slot1 = disp 0 / 8) ---
const PROG = {
  nop: () => cat(I.nop(), I.ret()),
  // mov rax,[r15]; inc rax; mov [r15],rax; ret
  inc: () => cat(I.mov_rax_m_r15(0), I.inc_rax(), I.mov_m_r15_rax(0), I.ret()),
  dec: () => cat(I.mov_rax_m_r15(0), I.dec_rax(), I.mov_m_r15_rax(0), I.ret()),
  // mov rax,[r15+8]; mov [r15],rax; ret
  get: () => cat(I.mov_rax_m_r15(8), I.mov_m_r15_rax(0), I.ret()),
  // mov rax,[r15]; add rax,5; mov [r15],rax; ret
  add: () => cat(I.mov_rax_m_r15(0), I.add_rax_imm32(5), I.mov_m_r15_rax(0), I.ret()),
  sub: () => cat(I.mov_rax_m_r15(0), I.sub_rax_imm32(5), I.mov_m_r15_rax(0), I.ret()),
  // movabs rax,imm; mov [r15],rax; ret
  set: () => cat(I.movabs_rax_imm64(), I.mov_m_r15_rax(0), I.ret()),
  // mov rax,[r15]; mov [r15+8],rax; ret
  mov: () => cat(I.mov_rax_m_r15(0), I.mov_m_r15_rax(8), I.ret()),
  // mov rax,[r15]; mov rcx,[r15+8]; cmp rax,rcx; ret
  cmp: () => cat(I.mov_rax_m_r15(0), I.mov_rcx_m_r15(8), I.cmp_rax_rcx(), I.ret()),
  // set imm; mov [r15],rax; mov rax,[r15]; add rax,5; mov [r15],rax; ret
  set_add: () =>
    cat(
      I.movabs_rax_imm64(),
      I.mov_m_r15_rax(0),
      I.mov_rax_m_r15(0),
      I.add_rax_imm32(5),
      I.mov_m_r15_rax(0),
      I.ret(),
    ),
  // set; mov [r15],rax; mov rax,[r15+8]; mov [r15],rax; ret
  set_get: () =>
    cat(I.movabs_rax_imm64(), I.mov_m_r15_rax(0), I.mov_rax_m_r15(8), I.mov_m_r15_rax(0), I.ret()),
  // set; store; load; add5; store; load; sub5; store; ret
  set_add_sub: () =>
    cat(
      I.movabs_rax_imm64(),
      I.mov_m_r15_rax(0),
      I.mov_rax_m_r15(0),
      I.add_rax_imm32(5),
      I.mov_m_r15_rax(0),
      I.mov_rax_m_r15(0),
      I.sub_rax_imm32(5),
      I.mov_m_r15_rax(0),
      I.ret(),
    ),
  // nop; load; inc; store; load; dec; store; ret
  nop_inc_dec: () =>
    cat(
      I.nop(),
      I.mov_rax_m_r15(0),
      I.inc_rax(),
      I.mov_m_r15_rax(0),
      I.mov_rax_m_r15(0),
      I.dec_rax(),
      I.mov_m_r15_rax(0),
      I.ret(),
    ),
  // set_add_sub but ret → nop;ret
  sa_sub_nop: () =>
    cat(
      I.movabs_rax_imm64(),
      I.mov_m_r15_rax(0),
      I.mov_rax_m_r15(0),
      I.add_rax_imm32(5),
      I.mov_m_r15_rax(0),
      I.mov_rax_m_r15(0),
      I.sub_rax_imm32(5),
      I.mov_m_r15_rax(0),
      I.nop(),
      I.ret(),
    ),
  // set; store; load slot0; store slot8; ret
  set_mov: () =>
    cat(I.movabs_rax_imm64(), I.mov_m_r15_rax(0), I.mov_rax_m_r15(0), I.mov_m_r15_rax(8), I.ret()),
  // set; store; load0; load8→rcx; cmp; load0; store8; ret
  set_cmp_mov: () =>
    cat(
      I.movabs_rax_imm64(),
      I.mov_m_r15_rax(0),
      I.mov_rax_m_r15(0),
      I.mov_rcx_m_r15(8),
      I.cmp_rax_rcx(),
      I.mov_rax_m_r15(0),
      I.mov_m_r15_rax(8),
      I.ret(),
    ),
};

/** Historical EQUAL chunk schedules for emit1 contiguous plants. */
const CHUNKS = {
  nop: [1, 1],
  inc: [8, 4],
  dec: [8, 4],
  get: [8, 1],
  add: [8, 8],
  sub: [8, 8],
  set: [8, 4, 2, 1],
  mov: [8, 1],
  cmp: [8, 4],
  set_add: [8, 8, 8, 4, 2],
  set_get: [8, 8, 4, 2, 1],
  set_add_sub: [8, 8, 8, 8, 8, 4, 1],
  nop_inc_dec: [8, 8, 8],
  sa_sub_nop: [8, 8, 8, 8, 8, 4, 2],
  set_mov: [8, 8, 4, 2, 1],
  set_cmp_mov: [8, 8, 8, 8, 2],
};

function emit1Stores(name) {
  const prog = PROG[name];
  const chunks = CHUNKS[name];
  if (!prog || !chunks) throw new Error(`unknown emit1 ${name}`);
  return pack(prog(), chunks);
}

// --- emit2 runtime plant helpers (host-side x64 that patches the PE) ---
const RT = {
  reload_rsi_saved: () => [0x48, 0x8b, 0x74, 0x24, 0x50], // mov rsi,[rsp+0x50]
  load_eax_tyb_u32: (disp) => [0x8b, 0x46, disp & 0xff], // mov eax,[rsi+disp]
  store_rax_pe: (off) => [0x48, 0x89, 0x83, off & 0xff, (off >> 8) & 0xff, (off >> 16) & 0xff, (off >> 24) & 0xff],
  store_eax_pe: (off) => [0x89, 0x83, off & 0xff, (off >> 8) & 0xff, (off >> 16) & 0xff, (off >> 24) & 0xff],
  loadzx_eax_tyb_u16: (disp) => [0x0f, 0xb7, 0x46, disp & 0xff],
  shl_eax_3: () => [0xc1, 0xe0, 0x03],
  store_al_pe: (off) => [0x88, 0x83, off & 0xff, (off >> 8) & 0xff, (off >> 16) & 0xff, (off >> 24) & 0xff],
};

function raw(bytes) {
  return { op: 'raw', bytes };
}
function rtr(fn) {
  return raw(typeof fn === 'function' ? fn() : fn);
}
function imm8(off, b) {
  return { op: 'imm8', off, bytes: [b & 0xff] };
}
function imm16(off, lo, hi) {
  return { op: 'imm16', off, bytes: [lo & 0xff, hi & 0xff] };
}
function imm32(off, bytes4) {
  return { op: 'imm32', off, bytes: bytes4.slice(0, 4) };
}
/** Plant first n bytes of an insn as imm32/imm8 stores (EQUAL hole layout). */
function plantBytes(off, bytes) {
  const out = [];
  let o = off;
  let i = 0;
  while (i < bytes.length) {
    const left = bytes.length - i;
    if (left >= 4 && (left === 4 || left > 4)) {
      // prefer imm32 when ≥4 remaining and we want EQUAL-style; caller passes exact slices
    }
    break;
  }
  // Use exact multi-store only via caller; helper for single imm32 plant:
  if (bytes.length === 4) return [imm32(off, bytes)];
  if (bytes.length === 2) return [imm16(off, bytes[0], bytes[1])];
  if (bytes.length === 1) return [imm8(off, bytes[0])];
  throw new Error('plantBytes: use explicit slices for len=' + bytes.length);
}

/**
 * emit2: plant PE .text skeleton (ISA) then RT-patch immediates/slot disp from tyb.
 * Layout holes match historical EQUAL (imm field / disp8 patch sites).
 */
const EMIT2 = {
  // movabs rax,imm64(hole@+2); mov [r15],rax; ret — fill imm from tyb+0xc
  set: () => [
    imm8(TEXT_OFF, 0x48),
    imm8(TEXT_OFF + 1, 0xb8),
    rtr(RT.reload_rsi_saved),
    rtr(() => RT.load_eax_tyb_u32(0x0c)),
    rtr(() => RT.store_rax_pe(TEXT_OFF + 2)),
    imm32(TEXT_OFF + 0x0a, I.mov_m_r15_rax(0)),
    imm8(TEXT_OFF + 0x0e, 0xc3),
  ],
  // mov rax,[r15]; add rax,imm32(hole); mov [r15],rax; ret
  add: () => [
    imm32(TEXT_OFF, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 4, I.add_rax_imm32(0).slice(0, 4)),
    rtr(RT.reload_rsi_saved),
    rtr(() => RT.load_eax_tyb_u32(0x0c)),
    rtr(() => RT.store_eax_pe(TEXT_OFF + 7)),
    imm32(TEXT_OFF + 0x0b, I.mov_m_r15_rax(0)),
    imm8(TEXT_OFF + 0x0f, 0xc3),
  ],
  sub: () => [
    imm32(TEXT_OFF, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 4, I.sub_rax_imm32(0).slice(0, 4)),
    rtr(RT.reload_rsi_saved),
    rtr(() => RT.load_eax_tyb_u32(0x0c)),
    rtr(() => RT.store_eax_pe(TEXT_OFF + 7)),
    imm32(TEXT_OFF + 0x0b, I.mov_m_r15_rax(0)),
    imm8(TEXT_OFF + 0x0f, 0xc3),
  ],
  // mov rax,[r15+disp_src]; mov [r15+disp_dst],rax; ret — patch both disp8 from tyb u16<<3
  getmov: () => [
    imm32(TEXT_OFF, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 4, I.mov_m_r15_rax(0)),
    rtr(RT.reload_rsi_saved),
    rtr(() => RT.loadzx_eax_tyb_u16(0x0c)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 3)), // src disp in mov rax,[r15+d]
    rtr(() => RT.loadzx_eax_tyb_u16(0x0a)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 7)), // dst disp in mov [r15+d],rax
    imm8(TEXT_OFF + 8, 0xc3),
  ],
  // mov rax,[r15+da]; mov rcx,[r15+db]; cmp rax,rcx; ret — patch da/db
  cmp: () => [
    imm32(TEXT_OFF, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 4, I.mov_rcx_m_r15(0)),
    imm32(TEXT_OFF + 8, cat(I.cmp_rax_rcx(), I.ret())),
    rtr(RT.reload_rsi_saved),
    rtr(() => RT.loadzx_eax_tyb_u16(0x0a)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 3)),
    rtr(() => RT.loadzx_eax_tyb_u16(0x0c)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 7)),
  ],
  // mov rax,[r15+d]; inc rax; mov [r15+d],eax; ret — same disp both (tyb+0xa)
  inc: () => [
    imm32(TEXT_OFF, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 4, cat(I.inc_rax(), [0x49])), // 48 ff c0 49 — leading REX of store
    imm32(TEXT_OFF + 8, [0x89, 0x47, 0x00, 0xc3]), // mov [rdi/r15+0],eax; ret (REX already planted)
    rtr(RT.reload_rsi_saved),
    rtr(() => RT.loadzx_eax_tyb_u16(0x0a)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 3)),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x0a)),
  ],
  dec: () => [
    imm32(TEXT_OFF, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 4, cat(I.dec_rax(), [0x49])),
    imm32(TEXT_OFF + 8, [0x89, 0x47, 0x00, 0xc3]),
    rtr(RT.reload_rsi_saved),
    rtr(() => RT.loadzx_eax_tyb_u16(0x0a)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 3)),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x0a)),
  ],
  // set(imm from tyb+0xc); mov [r15],rax; mov rax,[r15]; add rax,imm(tyb+0x14); mov [r15],rax; ret
  set_add: () => [
    imm8(TEXT_OFF, 0x48),
    imm8(TEXT_OFF + 1, 0xb8),
    rtr(RT.reload_rsi_saved),
    rtr(() => RT.load_eax_tyb_u32(0x0c)),
    rtr(() => RT.store_rax_pe(TEXT_OFF + 2)),
    imm32(TEXT_OFF + 0x0a, I.mov_m_r15_rax(0)),
    imm32(TEXT_OFF + 0x0e, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 0x12, I.add_rax_imm32(0).slice(0, 4)),
    rtr(() => RT.load_eax_tyb_u32(0x14)),
    rtr(() => RT.store_eax_pe(TEXT_OFF + 0x15)),
    imm32(TEXT_OFF + 0x19, I.mov_m_r15_rax(0)),
    imm8(TEXT_OFF + 0x1d, 0xc3),
  ],
  // set; store; mov rax,[r15+ds]; mov [r15+dd],rax; ret — patch ds/dd
  set_get: () => [
    imm8(TEXT_OFF, 0x48),
    imm8(TEXT_OFF + 1, 0xb8),
    rtr(RT.reload_rsi_saved),
    rtr(() => RT.load_eax_tyb_u32(0x0c)),
    rtr(() => RT.store_rax_pe(TEXT_OFF + 2)),
    imm32(TEXT_OFF + 0x0a, I.mov_m_r15_rax(0)),
    imm32(TEXT_OFF + 0x0e, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 0x12, I.mov_m_r15_rax(0)),
    imm8(TEXT_OFF + 0x16, 0xc3),
    rtr(() => RT.loadzx_eax_tyb_u16(0x14)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x11)),
    rtr(() => RT.loadzx_eax_tyb_u16(0x12)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x15)),
  ],
  // set; store; load a; load b→rcx; cmp; load; store — multi disp patches (EQUAL order)
  scm: () => [
    rtr(RT.reload_rsi_saved),
    imm8(TEXT_OFF, 0x48),
    imm8(TEXT_OFF + 1, 0xb8),
    rtr(() => RT.load_eax_tyb_u32(0x0c)),
    rtr(() => RT.store_rax_pe(TEXT_OFF + 2)),
    imm32(TEXT_OFF + 0x0a, I.mov_m_r15_rax(0)),
    imm32(TEXT_OFF + 0x0e, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 0x12, I.mov_rcx_m_r15(0)),
    imm32(TEXT_OFF + 0x16, cat(I.cmp_rax_rcx(), [0x49])),
    imm32(TEXT_OFF + 0x1a, [0x8b, 0x47, 0x00, 0x49]),
    imm32(TEXT_OFF + 0x1e, [0x89, 0x47, 0x00, 0xc3]),
    imm8(TEXT_OFF + 0x19, 0x49), // REX for mov rax,[r15] after cmp
    rtr(() => RT.loadzx_eax_tyb_u16(0x12)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x11)),
    rtr(() => RT.loadzx_eax_tyb_u16(0x14)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x15)),
    rtr(() => RT.loadzx_eax_tyb_u16(0x1c)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x1c)),
    rtr(() => RT.loadzx_eax_tyb_u16(0x1a)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x20)),
  ],
  // set; +add imm; +sub imm; ret
  sas: () => [
    rtr(RT.reload_rsi_saved),
    imm8(TEXT_OFF, 0x48),
    imm8(TEXT_OFF + 1, 0xb8),
    rtr(() => RT.load_eax_tyb_u32(0x0c)),
    rtr(() => RT.store_rax_pe(TEXT_OFF + 2)),
    imm32(TEXT_OFF + 0x0a, I.mov_m_r15_rax(0)),
    imm32(TEXT_OFF + 0x0e, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 0x12, I.add_rax_imm32(0).slice(0, 4)),
    rtr(() => RT.load_eax_tyb_u32(0x14)),
    rtr(() => RT.store_eax_pe(TEXT_OFF + 0x15)),
    imm32(TEXT_OFF + 0x19, I.mov_m_r15_rax(0)),
    imm32(TEXT_OFF + 0x1d, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 0x21, I.sub_rax_imm32(0).slice(0, 4)),
    rtr(() => RT.load_eax_tyb_u32(0x1c)),
    rtr(() => RT.store_eax_pe(TEXT_OFF + 0x24)),
    imm32(TEXT_OFF + 0x28, I.mov_m_r15_rax(0)),
    imm8(TEXT_OFF + 0x2c, 0xc3),
  ],
  // sas + nop before ret
  sasn: () => [
    rtr(RT.reload_rsi_saved),
    imm8(TEXT_OFF, 0x48),
    imm8(TEXT_OFF + 1, 0xb8),
    rtr(() => RT.load_eax_tyb_u32(0x0c)),
    rtr(() => RT.store_rax_pe(TEXT_OFF + 2)),
    imm32(TEXT_OFF + 0x0a, I.mov_m_r15_rax(0)),
    imm32(TEXT_OFF + 0x0e, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 0x12, I.add_rax_imm32(0).slice(0, 4)),
    rtr(() => RT.load_eax_tyb_u32(0x14)),
    rtr(() => RT.store_eax_pe(TEXT_OFF + 0x15)),
    imm32(TEXT_OFF + 0x19, I.mov_m_r15_rax(0)),
    imm32(TEXT_OFF + 0x1d, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 0x21, I.sub_rax_imm32(0).slice(0, 4)),
    rtr(() => RT.load_eax_tyb_u32(0x1c)),
    rtr(() => RT.store_eax_pe(TEXT_OFF + 0x24)),
    imm32(TEXT_OFF + 0x28, I.mov_m_r15_rax(0)),
    imm8(TEXT_OFF + 0x2c, 0x90),
    imm8(TEXT_OFF + 0x2d, 0xc3),
  ],
  // nop; inc slot; dec slot — patch two slot disp pairs from tyb
  nid: () => [
    rtr(RT.reload_rsi_saved),
    imm8(TEXT_OFF, 0x90),
    imm32(TEXT_OFF + 1, I.mov_rax_m_r15(0)),
    imm32(TEXT_OFF + 5, cat(I.inc_rax(), [0x49])),
    imm32(TEXT_OFF + 9, [0x89, 0x47, 0x00, 0x49]),
    imm32(TEXT_OFF + 0x0d, [0x8b, 0x47, 0x00, 0x48]),
    imm32(TEXT_OFF + 0x11, [0xff, 0xc8, 0x49, 0x89]),
    imm16(TEXT_OFF + 0x15, 0x47, 0x00),
    imm8(TEXT_OFF + 0x17, 0xc3),
    rtr(() => RT.loadzx_eax_tyb_u16(0x12)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 4)),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x0b)),
    rtr(() => RT.loadzx_eax_tyb_u16(0x1a)),
    rtr(RT.shl_eax_3),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x0f)),
    rtr(() => RT.store_al_pe(TEXT_OFF + 0x16)),
  ],
};

function emit2Stores(name) {
  const fn = EMIT2[name];
  if (!fn) throw new Error(`unknown emit2 ${name}`);
  return fn();
}

module.exports = {
  I,
  RT,
  TEXT_OFF,
  pack,
  PROG,
  CHUNKS,
  EMIT2,
  emit1Stores,
  emit2Stores,
  raw,
  rtr,
  imm8,
  imm16,
  imm32,
  cat,
};
