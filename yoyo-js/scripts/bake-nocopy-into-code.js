'use strict';
/** AUTO-NOCOPY: rebuild OPENLOOP without copy_self, then SEMREPRO+SEMAV chain. */
const { spawnSync } = require('child_process');
const path = require('path');

const root = path.join(__dirname, '..', '..');

function run(script) {
  const r = spawnSync(process.execPath, [path.join(__dirname, script)], {
    cwd: root,
    encoding: 'utf8',
  });
  process.stdout.write(r.stdout || '');
  process.stderr.write(r.stderr || '');
  if (r.status !== 0) process.exit(r.status || 1);
}

run('bake-semrepro-into-code.js');
run('bake-semav-into-code.js');

const fs = require('fs');
const pinPath = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload', 'payload.pin');
let pin = fs.readFileSync(pinPath, 'utf8');
const frag = fs.readFileSync(
  path.join(root, 'yoyo', 'projects', 'yoyo-openloop-h00.fragment.ty'),
  'utf8'
);
if (frag.includes('yoyo_compiler.exe')) {
  console.log('NOCOPY status=RED reason=fragment_still_has_strSelf');
  process.exit(1);
}
function setPin(pinText, key, val) {
  const re = new RegExp('^' + key + '=.*$', 'm');
  if (re.test(pinText)) return pinText.replace(re, key + '=' + val);
  return pinText.trimEnd() + '\n' + key + '=' + val + '\n';
}
pin = setPin(pin, 'yoyo_nocopy', 'PRESENT');
pin = setPin(pin, 'yoyo_product_selfhost_kind', 'y1pl_try_place_only');
pin = setPin(pin, 'yoyo_nocopy_copy_self', 'ABSENT');
pin = setPin(pin, 'note', 'NOCOPY_no_copy_self;y1pl_only;not_Thompson');
pin = setPin(pin, 'doc', 'AUTO-NOCOPY.md');
fs.writeFileSync(pinPath, pin);
console.log('NOCOPY copy_self=ABSENT');
console.log('NOCOPY selfhost_kind=y1pl_try_place_only');
console.log('NOCOPY status=GREEN');
