'use strict';
/**
 * AUTO-SEMOPS: SoT is YOYO .ty (A1 handlers), not YOYC magic text unload of stock bins.
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

// YOYC stock text starts with magic token as SoT body — ban that shape, not the word in comments.
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

console.log('SEMOPS_SRC src_kind=yoyo_ty_not_yoyc');
console.log('SEMOPS_SRC tyb_sha256_prefix=' + sha.slice(0, 16));
console.log('SEMOPS_SRC status=GREEN');
process.exit(0);
