'use strict';
/**
 * AUTO-AUDITLIVE: pin place_stub OUT_OF_WINDOW landing bytes in full code.bin.
 * Also: STUB.disasm.txt must list exactly one OUT_OF_WINDOW tgt (contract).
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const pinPath = path.join(root, 'scripts', 'fixtures', 'audit-stub', 'STUB.OOW.pin');
const codePath = process.argv[2]
  ? path.resolve(process.argv[2])
  : path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'code.bin');
const disasmPath = path.join(root, 'scripts', 'fixtures', 'audit-stub', 'STUB.disasm.txt');

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

function main() {
  if (!fs.existsSync(pinPath)) {
    console.log('AUDITLIVE status=RED reason=missing_oow_pin');
    process.exit(1);
  }
  if (!fs.existsSync(codePath)) {
    console.log('AUDITLIVE status=RED reason=missing_code_bin');
    process.exit(1);
  }
  const pin = readPin(pinPath);
  const code = fs.readFileSync(codePath);
  const off = parseInt(pin.code_bin_off, 16) || parseInt(pin.code_bin_off, 10);
  const win = parseInt(pin.window_bytes, 10);
  if (!(off >= 0) || off + win > code.length) {
    console.log(`AUDITLIVE status=RED reason=oow_off_oob off=0x${off.toString(16)} len=${code.length}`);
    process.exit(1);
  }
  const slice = code.subarray(off, off + win);
  const sha = crypto.createHash('sha256').update(slice).digest('hex');
  console.log(`AUDITLIVE oow_va=${pin.oow_va} code_bin_off=0x${off.toString(16)} window=${win}`);
  console.log(`AUDITLIVE landing_sha256_prefix=${sha.slice(0, 16)}`);

  if ((pin.landing_sha256 || '').toLowerCase() !== sha) {
    console.log('AUDITLIVE status=RED reason=landing_sha_DIFF');
    console.log(`AUDITLIVE want=${pin.landing_sha256}`);
    console.log(`AUDITLIVE got=${sha}`);
    process.exit(1);
  }

  // Disasm contract: exactly one OUT_OF_WINDOW with expected tgt
  if (fs.existsSync(disasmPath)) {
    const text = fs.readFileSync(disasmPath, 'utf8');
    const re = /OUT_OF_WINDOW tgt=(0x[0-9a-fA-F]+)/g;
    const tgts = [];
    let m;
    while ((m = re.exec(text))) tgts.push(m[1].toLowerCase());
    const wantCount = parseInt(pin.expect_oow_count_in_disasm || '1', 10);
    const wantTgt = (pin.expect_oow_tgt || '').toLowerCase();
    console.log(`AUDITLIVE disasm_oow_count=${tgts.length}`);
    if (tgts.length !== wantCount) {
      console.log(`AUDITLIVE status=RED reason=oow_count want=${wantCount} got=${tgts.length}`);
      process.exit(1);
    }
    if (wantTgt && tgts.some((t) => t !== wantTgt)) {
      console.log(`AUDITLIVE status=RED reason=oow_tgt_unexpected ${tgts.join(',')}`);
      process.exit(1);
    }
  } else {
    console.log('AUDITLIVE note=disasm_absent_skip_count');
  }

  console.log('AUDITLIVE oow=EQUAL');
  console.log('AUDITLIVE status=GREEN');
  console.log('AUDITLIVE doc=AUTO-AUDITLIVE.md');
  process.exit(0);
}

main();
