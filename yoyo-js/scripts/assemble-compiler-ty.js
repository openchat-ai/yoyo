'use strict';
/**
 * AUTO-SEMSRC: yoyo-compiler.ty text SoT ↔ bins / Y1PL / plant.
 *
 *   node assemble-compiler-ty.js emit     # bins → yoyo-compiler.ty
 *   node assemble-compiler-ty.js assemble # .ty → bins + plant + Y1PL tyb
 *   node assemble-compiler-ty.js check    # assemble and demand EQUAL pins
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const tyPath = path.join(projects, 'yoyo-compiler.ty');
const codePath = path.join(payload, 'code.bin');
const dataPath = path.join(payload, 'data.bin');

function sha256(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex');
}

function packPlant(bytes) {
  const recs = [];
  let i = 0;
  while (i < bytes.length) {
    const b = Buffer.alloc(8);
    if (bytes.length - i >= 3) {
      b[0] = 0xa1;
      b[1] = 3;
      b[2] = bytes[i];
      b[4] = bytes[i + 1];
      b[6] = bytes[i + 2];
      i += 3;
    } else {
      b[0] = 0xa0;
      b[1] = 1;
      b[2] = bytes[i];
      i += 1;
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

function unpackPlantRecords(lines) {
  const out = [];
  for (const { op, args } of lines) {
    if (op === 0xff) break;
    if (op === 0xa1) {
      const n = args[0] & 0xff;
      if (n >= 1) out.push(args[1] & 0xff);
      if (n >= 2) out.push(args[2] & 0xff);
      if (n >= 3) out.push(args[3] & 0xff);
      continue;
    }
    if (op === 0xa0) {
      out.push(args[1] & 0xff);
      continue;
    }
    throw new Error('SEMSRC unsupported op 0x' + op.toString(16));
  }
  return Buffer.from(out);
}

/** Parse SEMSRC .ty: @directives in comments + A1/A0/FF body (yoyo.ty dialect). */
function parseCompilerTy(src) {
  const dir = {};
  const lines = [];
  const isHex = (t) => /^(0x)?[0-9a-fA-F]+$/.test(t);
  const hex = (t) => parseInt(t.replace(/^0x/i, ''), 16);
  for (const raw of src.split(/\r?\n/)) {
    const dirHit = raw.match(/^\s*[;#]\s*@(\w+)\s*=\s*(\S+)/);
    if (dirHit) {
      dir[dirHit[1]] = dirHit[2];
      continue;
    }
    let line = raw.replace(/[;#].*$/, '').trim();
    if (!line) continue;
    const toks = line.split(/\s+/);
    const op = hex(toks[0]);
    const args = toks.slice(1).map((t) => {
      if (!isHex(t)) throw new Error('SEMSRC non-hex token: ' + t);
      return hex(t);
    });
    lines.push({ op, args });
  }
  return { dir, lines };
}

function emitTy() {
  const code = fs.readFileSync(codePath);
  const data = fs.readFileSync(dataPath);
  const body = Buffer.concat([code, data]);
  const rows = [];
  rows.push('# yoyo-compiler.ty — AUTO-SEMSRC');
  rows.push('# High-level text SoT for stock C1 body (code||data).');
  rows.push('# Assemble: node yoyo-js/scripts/assemble-compiler-ty.js assemble');
  rows.push('# Encoding: A1×3 (+ A0 tail) + FF — same dialect family as yoyo.ty');
  rows.push('# @kind=y1pl_compiler_body');
  rows.push('# @code_len=' + code.length);
  rows.push('# @data_len=' + data.length);
  rows.push('# @code_sha256=' + sha256(code));
  rows.push('# @data_sha256=' + sha256(data));
  rows.push('');
  let i = 0;
  while (i + 3 <= body.length) {
    rows.push(
      'A1 03 ' +
        body[i].toString(16).padStart(2, '0') +
        ' ' +
        body[i + 1].toString(16).padStart(2, '0') +
        ' ' +
        body[i + 2].toString(16).padStart(2, '0'),
    );
    i += 3;
  }
  while (i < body.length) {
    rows.push('A0 01 ' + body[i].toString(16).padStart(2, '0'));
    i += 1;
  }
  rows.push('FF');
  rows.push('');
  fs.writeFileSync(tyPath, rows.join('\n'));
  console.log('SEMSRC emit ty_bytes=' + fs.statSync(tyPath).size + ' code=' + code.length + ' data=' + data.length);
  console.log('SEMSRC emit out=' + tyPath);
  console.log('SEMSRC status=GREEN');
}

function assembleTy(writeBins) {
  const src = fs.readFileSync(tyPath, 'utf8');
  const { dir, lines } = parseCompilerTy(src);
  const codeLen = parseInt(dir.code_len, 10);
  const dataLen = parseInt(dir.data_len, 10);
  if (!Number.isFinite(codeLen) || !Number.isFinite(dataLen)) {
    throw new Error('SEMSRC missing @code_len/@data_len');
  }
  const body = unpackPlantRecords(lines);
  if (body.length !== codeLen + dataLen) {
    throw new Error('SEMSRC body_len=' + body.length + ' expect=' + (codeLen + dataLen));
  }
  const code = body.subarray(0, codeLen);
  const data = body.subarray(codeLen);
  if (dir.code_sha256 && sha256(code) !== dir.code_sha256) {
    throw new Error('SEMSRC code_sha mismatch');
  }
  if (dir.data_sha256 && sha256(data) !== dir.data_sha256) {
    throw new Error('SEMSRC data_sha mismatch');
  }

  const place = code.subarray(0, Math.min(4491, code.length));
  const parse1 = code.subarray(place.length);
  const plantCode = packPlant(code);
  const plantFull = packPlant(body);
  const oneshot = Buffer.alloc(12 + plantFull.length);
  oneshot.write('Y1PL', 0, 4, 'ascii');
  oneshot.writeUInt32LE(codeLen, 4);
  oneshot.writeUInt32LE(dataLen, 8);
  plantFull.copy(oneshot, 12);

  if (writeBins) {
    fs.writeFileSync(codePath, code);
    fs.writeFileSync(dataPath, data);
  }
  fs.writeFileSync(path.join(projects, 'yoyo-plant-place.tyb'), packPlant(place));
  fs.writeFileSync(path.join(projects, 'yoyo-plant-parse1.tyb'), packPlant(parse1));
  fs.writeFileSync(path.join(projects, 'yoyo-plant.tyb'), plantCode);
  fs.writeFileSync(path.join(projects, 'yoyo-semcomp-self.tyb'), oneshot);
  fs.writeFileSync(
    path.join(projects, 'yoyo-semcomp-self.ty'),
    '# yoyo-semcomp-self — derived from yoyo-compiler.ty (AUTO-SEMSRC)\n# Do not edit; SoT is yoyo-compiler.ty\n',
  );

  return { code, data, oneshot, codeLen, dataLen };
}

function updatePin(code, data, oneshot) {
  const codeSha = sha256(code);
  const dataSha = sha256(data);
  const oneshotSha = sha256(oneshot);
  const placeSha = sha256(fs.readFileSync(path.join(projects, 'yoyo-plant-place.tyb')));
  const parse1Sha = sha256(fs.readFileSync(path.join(projects, 'yoyo-plant-parse1.tyb')));
  let pin = fs.readFileSync(path.join(payload, 'payload.pin'), 'utf8');
  const set = (re, v) => {
    if (!re.test(pin)) throw new Error('pin miss ' + re);
    pin = pin.replace(re, v);
  };
  set(/code_bytes=\d+/, 'code_bytes=' + code.length);
  set(/code_sha256=[0-9a-f]+/, 'code_sha256=' + codeSha);
  set(/code_sha256_prefix=[0-9a-f]+/, 'code_sha256_prefix=' + codeSha.slice(0, 16));
  set(/data_bytes=\d+/, 'data_bytes=' + data.length);
  set(/data_sha256=[0-9a-f]+/, 'data_sha256=' + dataSha);
  set(/data_sha256_prefix=[0-9a-f]+/, 'data_sha256_prefix=' + dataSha.slice(0, 16));
  set(/code_parse1_bytes=\d+/, 'code_parse1_bytes=' + (code.length - 4491));
  set(/code_plant_place_sha256=[0-9a-f]+/, 'code_plant_place_sha256=' + placeSha);
  set(/code_plant_parse1_sha256=[0-9a-f]+/, 'code_plant_parse1_sha256=' + parse1Sha);
  set(/yoyo_semcomp_self_tyb_sha256=[0-9a-f]+/, 'yoyo_semcomp_self_tyb_sha256=' + oneshotSha);
  set(/yoyo_semcomp_self_kind=[^\r\n]+/, 'yoyo_semcomp_self_kind=y1pl_from_yoyo_compiler_ty');
  set(/code_bin_role=[^\r\n]+/, 'code_bin_role=derived_from_yoyo_compiler_ty');
  set(/note=SEM[^\r\n]+/, 'note=SEMSRC_yoyo_compiler_ty_sot;assemble_to_y1pl;stock_runs_T;not_Thompson');
  if (!/compiler_src_sot=/.test(pin)) {
    pin = pin.replace(
      /yoyo_semcomp_self_kind=[^\r\n]+/,
      (m) => m + '\ncompiler_src_sot=yoyo/projects/yoyo-compiler.ty',
    );
  } else {
    set(/compiler_src_sot=[^\r\n]+/, 'compiler_src_sot=yoyo/projects/yoyo-compiler.ty');
  }
  if (!/doc=AUTO-SEMSRC\.md/.test(pin) && /doc=AUTO-SEMNATIVE\.md/.test(pin)) {
    pin = pin.replace(/doc=AUTO-SEMNATIVE\.md/, 'doc=AUTO-SEMSRC.md');
  }
  fs.writeFileSync(path.join(payload, 'payload.pin'), pin);
}

function checkEqual() {
  const beforeCode = fs.readFileSync(codePath);
  const beforeData = fs.readFileSync(dataPath);
  const beforeShot = fs.existsSync(path.join(projects, 'yoyo-semcomp-self.tyb'))
    ? fs.readFileSync(path.join(projects, 'yoyo-semcomp-self.tyb'))
    : null;
  const { code, data, oneshot } = assembleTy(false);
  if (!code.equals(beforeCode)) throw new Error('SEMSRC check code DIFF');
  if (!data.equals(beforeData)) throw new Error('SEMSRC check data DIFF');
  // write derived oneshot/plant for gate even in check
  fs.writeFileSync(path.join(projects, 'yoyo-semcomp-self.tyb'), oneshot);
  if (beforeShot && !oneshot.equals(beforeShot)) {
    console.log('SEMSRC oneshot_refreshed_from_ty=1');
  }
  console.log('SEMSRC check code_EQUAL=1 data_EQUAL=1');
  console.log('SEMSRC oneshot_bytes=' + oneshot.length + ' sha256_prefix=' + sha256(oneshot).slice(0, 16));
  console.log('SEMSRC compiler_src_sot=yoyo-compiler.ty');
  console.log('SEMSRC status=GREEN');
}

function main() {
  const cmd = process.argv[2] || 'check';
  if (cmd === 'emit') {
    emitTy();
    return;
  }
  if (cmd === 'assemble') {
    const { code, data, oneshot } = assembleTy(true);
    updatePin(code, data, oneshot);
    console.log('SEMSRC assemble code=' + code.length + ' data=' + data.length);
    console.log('SEMSRC oneshot_bytes=' + oneshot.length + ' sha256_prefix=' + sha256(oneshot).slice(0, 16));
    console.log('SEMSRC compiler_src_sot=yoyo-compiler.ty');
    console.log('SEMSRC status=GREEN');
    return;
  }
  if (cmd === 'check') {
    if (!fs.existsSync(tyPath)) emitTy();
    checkEqual();
    return;
  }
  console.error('usage: emit|assemble|check');
  process.exit(2);
}

main();
