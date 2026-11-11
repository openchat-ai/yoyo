'use strict';
/**
 * AUTO-SEMOPS/SEMFULL/SEMHOST: SoT is pinned YOYO .ty/.tyb (not YOYC; not gate Node rewrite).
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const projects = path.join(root, 'yoyo', 'projects');
const tyPath = path.join(projects, 'yoyo-semops-compiler.ty');
const tybPath = path.join(projects, 'yoyo-semops-compiler.tyb');
const pinPath = path.join(
  root,
  'yoyo-js',
  'src',
  'platform',
  'yoyo-compiler-payload',
  'semops.pin',
);

function fail(reason) {
  console.log('SEMOPS_SRC status=RED reason=' + reason);
  process.exit(1);
}

if (!fs.existsSync(tyPath) || !fs.existsSync(tybPath)) fail('missing_ty_or_tyb');

const ty = fs.readFileSync(tyPath, 'utf8');
const tyb = fs.readFileSync(tybPath);

const bodyLines = ty.split(/\r?\n/).filter((l) => l.trim() && !l.trim().startsWith(';'));
if (bodyLines.length && /^YOYC\b/.test(bodyLines[0].trim())) fail('ty_is_yoyc');
if (!/^\s*40\s+0\s*$/m.test(ty)) fail('ty_missing_handler_40_0');
if (!/^\s*A1\s+/m.test(ty)) fail('ty_missing_A1');
if (tyb.length < 16 || tyb.slice(0, 4).toString('ascii') !== 'TYB\0') fail('tyb_magic');
const sha = crypto.createHash('sha256').update(tyb).digest('hex');
if (!fs.existsSync(pinPath)) fail('missing_semops_pin');
const pin = {};
for (const line of fs.readFileSync(pinPath, 'utf8').split(/\r?\n/)) {
  const m = line.match(/^\s*([^=]+)=(.*)$/);
  if (m) pin[m[1].trim()] = m[2].trim();
}
if (pin.tyb_sha256 !== sha) fail('tyb_sha_DIFF');
if (pin.src_kind !== 'yoyo_ty_not_yoyc') fail('src_kind_pin');
if (pin.parse1_sem_slices !== '14') fail('slices_not_14');
if (pin.gate_node_emit !== 'ABSENT') fail('gate_node_emit');
if (pin.host_a1_marker !== 'SEMHOST1') fail('host_marker');
const face = parseInt(pin.host_a1_face_bytes, 10);
const max = parseInt(pin.host_a1_face_max, 10);
if (!(face > 0) || !(max > 0) || face > max) fail('host_a1_face_budget');

console.log('SEMOPS_SRC src_kind=yoyo_ty_not_yoyc');
console.log('SEMOPS_SRC tyb_sha256_prefix=' + sha.slice(0, 16));
console.log('SEMOPS_SRC parse1_sem_slices=14');
console.log('SEMOPS_SRC gate_node_emit=ABSENT');
console.log('SEMOPS_SRC host_a1_face=PINNED bytes=' + face);
console.log('SEMOPS_SRC status=GREEN');
process.exit(0);
