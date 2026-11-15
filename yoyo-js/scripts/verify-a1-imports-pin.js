'use strict';
/**
 * AUTO-A1PIN: fail-closed if PE kernel32 import set ≠ pin.
 *   node yoyo-js/scripts/verify-a1-imports-pin.js <pe.exe>
 */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');
const pinPath = path.join(root, 'scripts', 'fixtures', 'yoyo_a1_imports.pin');
const pePath = process.argv[2];

function readPin(p) {
  const o = Object.create(null);
  for (const line of fs.readFileSync(p, 'utf8').split(/\r?\n/)) {
    if (!line || line.startsWith('#')) continue;
    const i = line.indexOf('=');
    if (i < 0) continue;
    o[line.slice(0, i).trim()] = line.slice(i + 1).trim();
  }
  return o;
}

function kernel32Funcs(buf) {
  const lfanew = buf.readUInt32LE(0x3c);
  const opt = lfanew + 24;
  const magic = buf.readUInt16LE(opt);
  const ddOff = magic === 0x20b ? opt + 112 : opt + 96;
  const impRva = buf.readUInt32LE(ddOff + 8);
  const soh = buf.readUInt16LE(lfanew + 20);
  const sec = lfanew + 24 + soh;
  const nsec = buf.readUInt16LE(lfanew + 6);
  function rvaToRaw(rva) {
    for (let i = 0; i < nsec; i++) {
      const s = sec + i * 40;
      const vr = buf.readUInt32LE(s + 12);
      const vs = buf.readUInt32LE(s + 8);
      const raw = buf.readUInt32LE(s + 20);
      const rs = buf.readUInt32LE(s + 16);
      if (rva >= vr && rva < vr + Math.max(vs, rs)) return raw + (rva - vr);
    }
    return -1;
  }
  const out = [];
  let desc = rvaToRaw(impRva);
  for (;;) {
    const nameRva = buf.readUInt32LE(desc + 12);
    const iatRva = buf.readUInt32LE(desc + 16);
    const iltRva = buf.readUInt32LE(desc);
    if (!nameRva && !iltRva && !iatRva) break;
    let dll = '';
    const nr = rvaToRaw(nameRva);
    for (let i = nr; buf[i]; i++) dll += String.fromCharCode(buf[i]);
    if (dll.toLowerCase() === 'kernel32.dll') {
      let th = rvaToRaw(iltRva || iatRva);
      for (;;) {
        const entry = buf.readBigUInt64LE(th);
        if (entry === 0n) break;
        if ((entry & 0x8000000000000000n) === 0n) {
          const hn = rvaToRaw(Number(entry & 0xffffffffn));
          let nm = '';
          for (let i = hn + 2; buf[i]; i++) nm += String.fromCharCode(buf[i]);
          out.push(nm);
        }
        th += 8;
      }
    }
    desc += 20;
  }
  return out;
}

if (!pePath || !fs.existsSync(pePath) || !fs.existsSync(pinPath)) {
  console.log('A1PIN status=RED reason=missing_pe_or_pin');
  process.exit(1);
}
const pin = readPin(pinPath);
const expect = (pin.funcs || '').split(',').map((s) => s.trim()).filter(Boolean);
const got = kernel32Funcs(fs.readFileSync(pePath));
console.log(`A1PIN pe=${path.basename(pePath)} funcs=${got.join(',')}`);
if (got.length !== expect.length || got.some((f, i) => f !== expect[i])) {
  console.log(`A1PIN status=RED reason=import_DIFF expect=${expect.join(',')}`);
  process.exit(1);
}
console.log('A1PIN status=GREEN pin_EQUAL');
console.log('A1PIN doc=AUTO-A1PIN.md');
process.exit(0);
