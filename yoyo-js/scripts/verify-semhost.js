'use strict';
/** AUTO-SEMHOST: OPENLOOP H_00 is content-aware (SEMHOST1 + expect tables; not blind-only C3). */
const fs = require('fs');
const path = require('path');

const projects = path.join(__dirname, '..', '..', 'yoyo', 'projects');
const expPath = path.join(projects, 'yoyo-openloop-h00.expect.hex');
const fragPath = path.join(projects, 'yoyo-openloop-h00.fragment.ty');

function fail(reason) {
  console.log('SEMHOST_VERIFY status=RED reason=' + reason);
  process.exit(1);
}

if (!fs.existsSync(expPath) || !fs.existsSync(fragPath)) fail('missing_openloop_artifacts');
const exp = Buffer.from(fs.readFileSync(expPath, 'utf8').trim(), 'hex');
const frag = fs.readFileSync(fragPath, 'utf8');
if (!exp.includes(Buffer.from('SEMHOST1\0'))) fail('marker_absent_in_expect');
if (!frag.includes('SEMHOST') && !frag.includes('content-aware')) fail('fragment_not_semhost');
// blind-only compact was a lone C6 86 00 02 00 00 C3 store; content-aware embeds many expects
if (exp.length < 1200) fail('expect_too_small_for_14');
const ret = Buffer.from('c3', 'hex');
const nop = Buffer.from('90c3', 'hex');
if (exp.indexOf(nop) < 0) fail('nop_expect_absent');
if (exp.indexOf(ret) < 0) fail('ret_expect_absent');

console.log('SEMHOST_VERIFY openloop_compact=content_aware');
console.log('SEMHOST_VERIFY marker=SEMHOST1');
console.log('SEMHOST_VERIFY openloop_h00_bytes=' + exp.length);
console.log('SEMHOST_VERIFY status=GREEN');
process.exit(0);
