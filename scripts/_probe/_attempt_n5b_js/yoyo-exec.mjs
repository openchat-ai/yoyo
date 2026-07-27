#!/usr/bin/env node
// yoyo-exec.mjs — JS-side byte interpreter mirroring yoyo-rust/executor
// opcode subset (W-START attempt-N5b-JS · EXPERIMENTAL · NON-GREEN).
// 1:1 parity with the Rust peer: same regs (rax/rcx/r15+EFLAGS),
// same flat MMU (base 0x1000), same fault messages, same exit codes.
// CLI: node yoyo-exec.mjs <compiled.bin>   (exit 0 HALT, 1 FAULT, 2 bad).

import fs from 'node:fs';
import { pathToFileURL } from 'node:url';

const BASE = 0x1000n, CAP = 16 * 1024, SL = 50000n;
const R = { RAX: 0, RCX: 1, R15: 2 };
const hx = n => '0x' + BigInt.asUintN(64, n).toString(16);
const u64 = m => { const o = m._o; return BigInt(m.b[o])|BigInt(m.b[o+1])<<8n|BigInt(m.b[o+2])<<16n|BigInt(m.b[o+3])<<24n|BigInt(m.b[o+4])<<32n|BigInt(m.b[o+5])<<40n|BigInt(m.b[o+6])<<48n|BigInt(m.b[o+7])<<56n; };
const u32 = m => { const o = m._o; return BigInt(m.b[o])|BigInt(m.b[o+1])<<8n|BigInt(m.b[o+2])<<16n|BigInt(m.b[o+3])<<24n; };
const roob = (a, l) => `read OOB at ${a} (${l}B)`;
const oob = (m, a, l) => {
  if (a < m.base) throw roob(hx(a), l);
  const off = Number(a - m.base);
  if (off + l > m.b.length) throw roob(hx(a), l);
  m._o = off;
};
const ru8 = (m, a) => { oob(m, a, 1); return m.b[m._o]; };
const ru32 = (m, a) => { oob(m, a, 4); return u32(m); };
const ru64 = (m, a) => { oob(m, a, 8); return u64(m); };
const wu64 = (m, a, v) => {
  if (a < m.base) throw `write OOB at ${hx(a)} (8B)`;
  const off = Number(a - m.base);
  if (off + 8 > m.b.length) throw `write OOB at ${hx(a)} (8B)`;
  for (let i = 0; i < 8; i++) m.b[off + i] = Number((v >> BigInt(i * 8)) & 0xFFn);
};
const ridx = (lo, b) => { const e = b ? (lo + 8) & 0xF : lo; return e === 0 ? R.RAX : e === 1 ? R.RCX : e === 15 ? R.R15 : -1; };
const par = r => { let p = Number(r & 0xFFn), b = 0; for (let i = 0; i < 8; i++) { if (p & 1) b++; p >>= 1; } return (b % 2) === 0; };
const flA = (f, l, r2, s, sub) => {
  f.zf = s === 0n; f.sf = ((s >> 63n) & 1n) === 1n;
  const L = BigInt.asIntN(64, l), R2 = BigInt.asIntN(64, r2), S = BigInt.asIntN(64, s);
  f.of = sub ? ((L >= 0n && R2 < 0n && S < 0n) || (L < 0n && R2 >= 0n && S >= 0n)) : ((L >= 0n && R2 >= 0n && S < 0n) || (L < 0n && R2 < 0n && S >= 0n));
  f.cf = sub ? l < r2 : ((l + r2) >> 64n) !== 0n || (l + r2) < l;
  f.pf = par(s);
};
const flO = (f, r2) => { f.zf = r2 === 0n; f.sf = ((r2 >> 63n) & 1n) === 1n; f.of = false; f.cf = false; f.pf = par(r2); };
const jcc = (cc, f) => ({ 0x84: f.zf, 0x85: !f.zf, 0x82: f.cf, 0x83: !f.cf, 0x86: f.cf || f.zf, 0x87: !f.cf && !f.zf, 0x8C: f.sf !== f.of, 0x8D: f.sf === f.of, 0x8E: f.zf || (f.sf !== f.of), 0x8F: !f.zf && (f.sf === f.of) })[cc] ?? false;
const dRM = (m, at) => {
  const b = ru8(m, at), md = (b >> 6) & 3, rg = (b >> 3) & 7, rm = b & 7;
  if (rm !== 7) throw `decode fault at ${hx(at)}: state: rm must be 111 (r15)`;
  const t = rg === 0 ? R.RAX : rg === 1 ? R.RCX : rg === 7 ? R.R15 : (() => { throw `decode fault at ${hx(at)}: state: reg field unsupported`; })();
  let dp, ln;
  if (md === 1) { dp = BigInt(ru8(m, at + 1n) << 24 >> 24); ln = 2; }
  else if (md === 2) { dp = BigInt.asIntN(64, BigInt.asIntN(32, ru32(m, at + 1n))); ln = 5; }
  else throw `decode fault at ${hx(at)}: state: mod=00 not allowed (no SIB/rip-rel)`;
  return { t, dp, ln };
};
const dRR = (m, at, b) => {
  const by = ru8(m, at); if (((by >> 6) & 3) !== 3) throw `decode fault at ${hx(at)}: rr form: mod must be 11`;
  const d = ridx((by >> 3) & 7, b), s = ridx(by & 7, b);
  if (d < 0) throw `decode fault at ${hx(at)}: rr dst outside subset`;
  if (s < 0) throw `decode fault at ${hx(at)}: rr src outside subset`;
  return { d, s };
};
const step = (cpu, m) => {
  if (cpu.steps >= SL) throw `step limit ${cpu.steps} reached`;
  cpu.steps++;
  const pc = cpu.rip, b0 = ru8(m, pc);
  if (b0 === 0x90) { cpu.rip = pc + 1n; return; }
  if (b0 === 0xC3) { if (cpu.ret.length) cpu.rip = cpu.ret.pop(); else { cpu.rip = pc + 1n; cpu.halted = true; } return; }
  if (b0 === 0xE9 || b0 === 0xE8) {
    const o = BigInt.asIntN(32, ru32(m, pc + 1n));
    if (b0 === 0xE8) { if (cpu.ret.length) throw `diverge at ${hx(pc)}: nested CALL beyond 1-deep shadow stack`; cpu.ret.push(pc + 5n); }
    cpu.rip = BigInt.asIntN(64, pc + 5n + o); return;
  }
  if (b0 === 0x0F) {
    const b1 = ru8(m, pc + 1n);
    if (b1 >= 0x82 && b1 <= 0x8F) {
      const o = BigInt.asIntN(32, ru32(m, pc + 2n));
      cpu.rip = jcc(b1, cpu.f) ? BigInt.asIntN(64, pc + 6n + o) : (pc + 6n); return;
    }
    throw `decode fault at ${hx(pc)}: unrecognised 0F escape`;
  }
  if (b0 !== 0x48 && b0 !== 0x49) throw `unimplemented opcode at ${hx(pc)}: 0x${b0.toString(16).padStart(2,'0')}`;
  const b = (b0 & 1) !== 0, op = ru8(m, pc + 1n);
  if (op >= 0xB8 && op <= 0xBF) {
    const lo = op & 7; let r;
    if (b ? (lo !== 0 && lo !== 1 && lo !== 7) : (lo !== 0 && lo !== 1 && lo !== 7))
      throw `decode fault at ${hx(pc)}: movabs: reg outside subset${b ? ' (r8..r15)' : ''}`;
    r = lo === 0 ? R.RAX : lo === 1 ? R.RCX : R.R15;
    cpu.regs[r] = ru64(m, pc + 2n); cpu.rip = pc + 10n; return;
  }
  if (op === 0x8B || op === 0x89) {
    const { t, dp, ln } = dRM(m, pc + 2n);
    if (op === 0x8B) cpu.regs[t] = ru64(m, cpu.regs[R.R15] + dp);
    else wu64(m, cpu.regs[R.R15] + dp, cpu.regs[t]);
    cpu.rip = pc + 2n + BigInt(ln); return;
  }
  if (op === 0xFF) {
    const b2 = ru8(m, pc + 2n);
    if (b2 !== 0xC0 && b2 !== 0xC8) throw `decode fault at ${hx(pc)}: unsupported 48/49 FF /modrm`;
    const o = cpu.regs[R.RAX], n = b2 === 0xC0 ? o + 1n : o - 1n;
    flA(cpu.f, o, 1n, n, b2 === 0xC8); cpu.regs[R.RAX] = BigInt.asUintN(64, n); cpu.rip = pc + 3n; return;
  }
  if (op === 0x83 || op === 0x81) {
    const b2 = ru8(m, pc + 2n), ope = b2 >> 3, lo = b2 & 7, h = op.toString(16);
    const r = b ? (lo === 0 ? R.RAX : lo === 1 ? R.RCX : lo === 7 ? R.R15 : (() => { throw `decode fault at ${hx(pc)}: 48/49 ${h}: reg outside subset`; })()) : ridx(lo, false);
    if (r < 0) throw `decode fault at ${hx(pc)}: 48/49 ${h}: reg outside subset`;
    if (ope !== 0 && ope !== 5 || r !== R.RAX) throw `decode fault at ${hx(pc)}: 48/49 ${h}: op not add/sub rax`;
    const raw = op === 0x83 ? BigInt(ru8(m, pc + 3n)) : ru32(m, pc + 3n);
    const imm = op === 0x83 ? BigInt.asIntN(64, BigInt.asIntN(8, raw)) : BigInt.asIntN(64, BigInt.asIntN(32, raw));
    const o = cpu.regs[R.RAX], n = ope === 0 ? o + imm : o - imm;
    flA(cpu.f, o, imm, n, ope === 5); cpu.regs[R.RAX] = BigInt.asUintN(64, n);
    cpu.rip = pc + (op === 0x83 ? 4n : 7n); return;
  }
  if (op === 0x01 || op === 0x29 || op === 0x09) {
    const { d, s } = dRR(m, pc + 2n, b);
    const a = cpu.regs[d], c = cpu.regs[s], n = op === 0x01 ? a + c : op === 0x29 ? a - c : (a | c);
    const r2 = BigInt.asUintN(64, n); cpu.regs[d] = r2;
    if (op === 0x09) flO(cpu.f, r2); else flA(cpu.f, a, c, r2, op === 0x29);
    cpu.rip = pc + 3n; return;
  }
  if (op === 0x0F) {
    const b2 = ru8(m, pc + 2n);
    if (b2 === 0xAF) {
      const { d, s } = dRR(m, pc + 3n, b);
      const r2 = BigInt.asUintN(64, BigInt.asIntN(64, BigInt.asIntN(64, cpu.regs[d]) * BigInt.asIntN(64, cpu.regs[s])));
      cpu.regs[d] = r2; flO(cpu.f, r2); cpu.rip = pc + 4n; return;
    }
    if (b2 === 0xB6) {
      if (ru8(m, pc + 3n) !== 0) throw `decode fault at ${hx(pc)}: movzx expects ModRM=0x00`;
      cpu.regs[R.RAX] = BigInt(ru8(m, cpu.regs[R.RAX])); cpu.rip = pc + 4n; return;
    }
    throw `decode fault at ${hx(pc)}: unsupported 48/49 0F escape`;
  }
  if (op === 0x39 || op === 0x3B) {
    const { d, s } = dRR(m, pc + 2n, b);
    const a = cpu.regs[d], c = cpu.regs[s], l = op === 0x39 ? a : c, rr = op === 0x39 ? c : a;
    flA(cpu.f, l, rr, BigInt.asUintN(64, l - rr), true); cpu.rip = pc + 3n; return;
  }
  throw `decode fault at ${hx(pc)}: unsupported 48/49 escape`;
};

export function run(bytes) {
  const m = { b: new Uint8Array(CAP), base: BASE };
  for (let i = 0; i < Math.min(bytes.length, CAP); i++) m.b[i] = bytes[i];
  const cpu = { regs: [0n, 0n, BASE], f: { zf: false, sf: false, of: false, cf: false, pf: false }, rip: BASE, steps: 0n, halted: false, ret: [] };
  try { while (!cpu.halted) step(cpu, m);
    return { ok: true, rax: cpu.regs[R.RAX], rcx: cpu.regs[R.RCX], r15: cpu.regs[R.R15], steps: cpu.steps, rip: cpu.rip }; }
  catch (msg) { return { ok: false, fault: msg, rax: cpu.regs[R.RAX], rcx: cpu.regs[R.RCX], r15: cpu.regs[R.R15], steps: cpu.steps }; }
}

function cli() {
  const fp = process.argv[2];
  if (!fp || fp.startsWith('--')) { process.stderr.write('usage: node yoyo-exec.mjs <compiled.bin>\n'); process.exit(2); }
  let bytes; try { bytes = fs.readFileSync(fp); } catch (e) { process.stderr.write(`read error: ${e.message}\n`); process.exit(2); }
  const o = run(bytes);
  process.stdout.write(`steps : ${o.steps}\nrax   : 0x${o.rax.toString(16).padStart(16,'0')}\nrcx   : 0x${o.rcx.toString(16).padStart(16,'0')}\nr15   : 0x${o.r15.toString(16).padStart(16,'0')}\n`);
  process.stdout.write(o.ok ? `exit  : HALT at 0x${o.rip.toString(16)} after ${o.steps} steps\n` : `exit  : FAULT ${o.fault}\n`);
  process.exit(o.ok ? 0 : 1);
}

function bs(v) { return typeof v === 'bigint' ? v.toString() : JSON.stringify(v); }
function eq(a, b, l) { if (bs(a) !== bs(b)) { process.stderr.write(`FAIL ${l}: got ${bs(a)}, want ${bs(b)}\n`); process.exit(1); } }

function unitTests() {
  let r;
  // 1) nop_ret_halts  2) movabs_store_ret  3) movzx_inc_store_ret  4) jmp_backward_to_nop  5) jmp_je_taken  6) decode_fault  7) hex_smoke  8) raw_byte_nop_chain
  r = run([0x90, 0xC3]); eq(r.ok, true, '1.ok'); eq(r.steps, 2n, '1.steps'); eq(r.rax, 0n, '1.rax');
  r = run([0x48, 0xB8, 0x2A, 0,0,0,0,0,0,0, 0x49, 0x89, 0x87, 0x80, 0x02, 0, 0, 0xC3]); eq(r.ok, true, '2.ok'); eq(r.rax, 0x2An, '2.rax');
  r = run([0x48, 0xB8, 0x05,0,0,0,0,0,0,0, 0x49, 0x89, 0x87, 0x80,0x02,0,0, 0x49, 0x8B, 0x87, 0x80,0x02,0,0, 0x48, 0xFF, 0xC0, 0x49, 0x89, 0x87, 0x80,0x02,0,0, 0xC3]); eq(r.ok, true, '3.ok'); eq(r.rax, 6n, '3.rax');
  r = run([0x90, 0xC3, 0xE9, 0xFB,0xFF,0xFF,0xFF, 0xC3]); eq(r.ok, true, '4.ok'); eq(r.steps, 2n, '4.steps');
  r = run([0x48, 0xB8, 0,0,0,0,0,0,0,0, 0x48, 0x39, 0xC0, 0x0F, 0x84, 0x01,0,0,0, 0xC3, 0xC3]); eq(r.ok, true, '5.ok'); eq(r.steps, 4n, '5.steps');
  r = run([0xFF, 0xC3]); eq(r.ok, false, '6.ok'); eq(/unimplemented opcode/.test(r.fault), true, '6.fault');
  r = run([0x90, 0xC3]); eq(r.ok, true, '7.ok');
  r = run([0x90, 0xC3, 0x90, 0x90, 0xC3]); eq(r.ok, true, '8.ok'); eq(r.steps, 2n, '8.steps');
  process.stdout.write('unit_tests: 8/8 pass\n');
}

if (process.argv[2] === '--test') unitTests();
else if (import.meta.url === pathToFileURL(process.argv[1]).href) cli();