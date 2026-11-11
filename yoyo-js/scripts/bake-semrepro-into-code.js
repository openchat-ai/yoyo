'use strict';
/**
 * AUTO-SEMREPRO: rebuild OPENLOOP fragment (Y1PL→place_epi), bake into code.bin.
 * Retarget place_epi final jmp from fail → old H_00 body (end of prefix).
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { spawnSync } = require('child_process');
const { layoutFor } = require('../src/platform/yoyo-compiler-parse1-native-link');
const { packPlant, setPin, OLD_PLACE } = require('./bake-sembed-into-code');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const projects = path.join(root, 'yoyo', 'projects');
const codePath = path.join(payload, 'code.bin');
const dataPath = path.join(payload, 'data.bin');
const pinPath = path.join(payload, 'payload.pin');
const fragPath = path.join(projects, 'yoyo-openloop-h00.fragment.ty');
const expectPath = path.join(projects, 'yoyo-openloop-h00.expect.hex');

const BODY_LEN = 16717;

function unpackFragment(text) {
  const out = [];
  for (const line of text.split(/\r?\n/)) {
    const t = line.trim();
    if (!t || t.startsWith(';') || /^40\s+0$/.test(t) || /^FF$/i.test(t)) continue;
    const m1 = t.match(/^A1\s+([0-9a-fA-F]{2})\s+([0-9a-fA-F]{2})\s+([0-9a-fA-F]{2})$/);
    if (m1) {
      out.push(parseInt(m1[1], 16), parseInt(m1[2], 16), parseInt(m1[3], 16));
      continue;
    }
    const m0 = t.match(/^A0\s+([0-9a-fA-F]{2})$/);
    if (m0) out.push(parseInt(m0[1], 16));
  }
  return Buffer.from(out);
}

function findAll(buf, v) {
  const out = [];
  const want = v >>> 0;
  for (let i = 0; i + 3 < buf.length; i++) {
    if (buf.readUInt32LE(i) === want) out.push(i);
  }
  return out;
}

function replaceImm(buf, oldV, newV, expectedHits) {
  const hits = findAll(buf, oldV);
  if (hits.length !== expectedHits) {
    throw new Error('imm ' + (oldV >>> 0) + ' hits=' + hits.length + ' expect=' + expectedHits);
  }
  for (const s of hits) buf.writeUInt32LE(newV >>> 0, s);
  return hits;
}

function readPrefixLen(pinText, codeLen) {
  for (const key of [
    'yoyo_semrepro_prefix_bytes',
    'yoyo_semplace_prefix_bytes',
    'yoyo_semfrag_prefix_bytes',
    'yoyo_semolp_prefix_bytes',
    'yoyo_sembed_prefix_bytes',
  ]) {
    const m = pinText.match(new RegExp('^\\s*' + key + '=(\\d+)\\s*$', 'm'));
    if (m) return parseInt(m[1], 10);
  }
  return codeLen - BODY_LEN;
}

/** place_epi = add rsp,0x88; jmp rel32 — retarget jmp to old_h00 (= prefix length). */
function retargetPlaceEpiJmp(prefix) {
  const pat = Buffer.from([0x48, 0x81, 0xc4, 0x88, 0x00, 0x00, 0x00, 0xe9]);
  let idx = -1;
  let from = 0;
  while (true) {
    const at = prefix.indexOf(pat, from);
    if (at < 0) break;
    idx = at;
    from = at + 1;
  }
  if (idx < 0) throw new Error('place_epi not found');
  const relAt = idx + 8;
  const next = relAt + 4;
  const oldH00 = prefix.length;
  prefix.writeInt32LE(oldH00 - next, relAt);
  return idx;
}

function main() {
  const built = spawnSync(
    process.execPath,
    [path.join(__dirname, 'build-openloop-h00-bytes.js')],
    { cwd: root, encoding: 'utf8' }
  );
  if (built.status !== 0) {
    console.log(built.stdout || '');
    console.log(built.stderr || '');
    console.log('SEMREPRO status=RED reason=rebuild_openloop');
    process.exit(1);
  }
  console.log((built.stdout || '').trim());

  let pin = fs.readFileSync(pinPath, 'utf8');
  const code = Buffer.from(fs.readFileSync(codePath));
  const data = fs.readFileSync(dataPath);
  const core = unpackFragment(fs.readFileSync(fragPath, 'utf8'));
  const expect = Buffer.from(
    fs.readFileSync(expectPath, 'utf8').trim().replace(/\s+/g, ''),
    'hex'
  );
  if (!core.equals(expect)) {
    console.log('SEMREPRO status=RED reason=fragment_ne_expect');
    process.exit(1);
  }
  if (!core.includes(Buffer.from('SEMHOST1'))) {
    console.log('SEMREPRO status=RED reason=missing_SEMHOST1');
    process.exit(1);
  }
  // Must contain Y1PL imm and place_epi
  if (core.indexOf(Buffer.from([0x81, 0x7c, 0x24, 0x50, 0x59, 0x31, 0x50, 0x4c])) < 0) {
    console.log('SEMREPRO status=RED reason=missing_Y1PL_cmp');
    process.exit(1);
  }

  const oldPrefixLen = readPrefixLen(pin, code.length);
  if (code.length !== oldPrefixLen + BODY_LEN) {
    console.log('SEMREPRO status=RED reason=len_mismatch');
    process.exit(1);
  }

  const body = Buffer.from(code.subarray(oldPrefixLen));
  const Lcur = layoutFor(code.length, data.length, 0x38000);
  const Lbody = layoutFor(BODY_LEN, data.length, 0x38000);
  if (findAll(body, code.length).length === 5) {
    replaceImm(body, code.length, BODY_LEN, 5);
    replaceImm(body, Lcur.peSize, Lbody.peSize, 6);
    replaceImm(body, Lcur.codeRaw, Lbody.codeRaw, 2);
    replaceImm(body, Lcur.dataFileOff, Lbody.dataFileOff, 2);
    replaceImm(body, Lcur.userDataOff, Lbody.userDataOff, 2);
    for (let k = 8; k < Lcur.iatLen; k += 8) {
      replaceImm(body, (Lcur.dataFileOff + k) >>> 0, (Lbody.dataFileOff + k) >>> 0, 1);
    }
  } else if (findAll(body, BODY_LEN).length !== 5) {
    console.log('SEMREPRO status=RED reason=body_codeLen_imm_missing');
    process.exit(1);
  }

  const prefix = Buffer.from(core);
  const epiAt = retargetPlaceEpiJmp(prefix);
  const newCode = Buffer.concat([prefix, body]);
  // Growing prefix may need larger PE geom
  const Lnew = layoutFor(newCode.length, data.length, 0x38000);
  if (Lnew.textVs !== Lbody.textVs) {
    console.log('SEMREPRO status=RED reason=textVs_changed need=' + Lnew.textVs);
    process.exit(1);
  }

  replaceImm(newCode, BODY_LEN, newCode.length, 5);
  replaceImm(newCode, Lbody.peSize, Lnew.peSize, 6);
  replaceImm(newCode, Lbody.codeRaw, Lnew.codeRaw, 2);
  replaceImm(newCode, Lbody.dataFileOff, Lnew.dataFileOff, 2);
  replaceImm(newCode, Lbody.userDataOff, Lnew.userDataOff, 2);
  for (let k = 8; k < Lbody.iatLen; k += 8) {
    replaceImm(
      newCode,
      (Lbody.dataFileOff + k) >>> 0,
      (Lnew.dataFileOff + k) >>> 0,
      1
    );
  }

  // Re-fix place_epi jmp after geom? jmp is relative inside prefix — old_h00 still prefix.length. OK.

  const placeLen = prefix.length + OLD_PLACE;
  fs.writeFileSync(codePath, newCode);
  fs.writeFileSync(path.join(projects, 'yoyo-plant-place.tyb'), packPlant(newCode.subarray(0, placeLen)));
  fs.writeFileSync(path.join(projects, 'yoyo-plant-parse1.tyb'), packPlant(newCode.subarray(placeLen)));
  fs.writeFileSync(path.join(projects, 'yoyo-plant.tyb'), packPlant(newCode));

  const codeSha = crypto.createHash('sha256').update(newCode).digest('hex');
  pin = setPin(pin, 'code_bytes', String(newCode.length));
  pin = setPin(pin, 'code_sha256', codeSha);
  pin = setPin(pin, 'code_sha256_prefix', codeSha.slice(0, 16));
  pin = setPin(pin, 'code_place_bytes', String(placeLen));
  pin = setPin(pin, 'code_parse1_bytes', String(newCode.length - placeLen));
  pin = setPin(
    pin,
    'code_plant_place_sha256',
    crypto
      .createHash('sha256')
      .update(fs.readFileSync(path.join(projects, 'yoyo-plant-place.tyb')))
      .digest('hex')
  );
  pin = setPin(
    pin,
    'code_plant_parse1_sha256',
    crypto
      .createHash('sha256')
      .update(fs.readFileSync(path.join(projects, 'yoyo-plant-parse1.tyb')))
      .digest('hex')
  );
  pin = setPin(pin, 'yoyo_sembed_prefix_bytes', String(prefix.length));
  pin = setPin(pin, 'yoyo_semrepro', 'PRESENT');
  pin = setPin(pin, 'yoyo_semrepro_prefix_bytes', String(prefix.length));
  pin = setPin(pin, 'yoyo_semrepro_place_epi_at', String(epiAt));
  pin = setPin(pin, 'yoyo_semrepro_y1pl_to_body', 'PRESENT');
  pin = setPin(pin, 'yoyo_product_selfhost_kind', 'y1pl_try_place_on_large_else_copy_self');
  pin = setPin(pin, 'yoyo_semgeom_pe_size', String(Lnew.peSize));
  pin = setPin(pin, 'yoyo_semgeom_code_raw', String(Lnew.codeRaw));
  // Refresh Y1PL self tyb to match live code||data (old tyb was body-only 16717).
  const blob = Buffer.concat([newCode, data]);
  const plant = packPlant(blob);
  const oneshot = Buffer.alloc(12 + plant.length);
  oneshot.write('Y1PL', 0, 4, 'ascii');
  oneshot.writeUInt32LE(newCode.length, 4);
  oneshot.writeUInt32LE(data.length, 8);
  plant.copy(oneshot, 12);
  fs.writeFileSync(path.join(projects, 'yoyo-semcomp-self.tyb'), oneshot);
  const tsha = crypto.createHash('sha256').update(oneshot).digest('hex');
  pin = setPin(pin, 'yoyo_semcomp_self_tyb_sha256', tsha);
  pin = setPin(pin, 'yoyo_semcomp_self_kind', 'y1pl_from_live_code_bin');

  pin = setPin(pin, 'note', 'SEMREPRO_y1pl_to_body;copy_self_fallback;exit_av_after_write;not_Thompson');
  pin = setPin(pin, 'doc', 'AUTO-SEMREPRO.md');
  fs.writeFileSync(pinPath, pin);

  console.log('SEMREPRO prefix_bytes=' + prefix.length + ' (was ' + oldPrefixLen + ')');
  console.log('SEMREPRO code_bytes=' + newCode.length);
  console.log('SEMREPRO place_epi_at=' + epiAt);
  console.log('SEMREPRO peSize=' + Lnew.peSize + ' codeRaw=' + Lnew.codeRaw);
  console.log('SEMREPRO y1pl_to_body=PRESENT');
  console.log('SEMREPRO semcomp_tyb_sha256_prefix=' + tsha.slice(0, 16));
  console.log('SEMREPRO status=GREEN');
}

main();
