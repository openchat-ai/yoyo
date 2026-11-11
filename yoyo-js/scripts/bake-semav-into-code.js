'use strict';
/**
 * AUTO-SEMAV: write-success ret → ExitProcess(0); pack fail2/fail3 to same 27-byte window.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { layoutFor } = require('../src/platform/yoyo-compiler-parse1-native-link');
const { packPlant, setPin, OLD_PLACE } = require('./bake-sembed-into-code');

const root = path.join(__dirname, '..', '..');
const payload = path.join(root, 'yoyo-js', 'src', 'platform', 'yoyo-compiler-payload');
const projects = path.join(root, 'yoyo', 'projects');
const codePath = path.join(payload, 'code.bin');
const dataPath = path.join(payload, 'data.bin');
const pinPath = path.join(payload, 'payload.pin');

function main() {
  let pin = fs.readFileSync(pinPath, 'utf8');
  const code = Buffer.from(fs.readFileSync(codePath));
  const data = fs.readFileSync(dataPath);

  const pattern = Buffer.from([
    0x31, 0xc0, 0x48, 0x83, 0xc4, 0x78, 0xc3, 0xb8, 0x02, 0x00, 0x00, 0x00, 0x48, 0x83, 0xc4, 0x78,
    0xc3, 0xb8, 0x03, 0x00, 0x00, 0x00, 0x48, 0x83, 0xc4, 0x78, 0xc3,
  ]);
  const at = code.indexOf(pattern);
  if (at < 0) {
    if (
      code.includes(
        Buffer.from([0x31, 0xc9, 0x41, 0xff, 0x97, 0x28, 0x00, 0x00, 0x00, 0xb8, 0x02, 0x00, 0x00, 0x00, 0xeb])
      )
    ) {
      console.log('SEMAV already=PATCHED');
      console.log('SEMAV status=GREEN');
      process.exit(0);
    }
    console.log('SEMAV status=RED reason=pattern_missing');
    process.exit(1);
  }

  const oldFail2 = at + 7;
  const oldFail3 = at + 17;
  const newFail2 = at + 9;
  const newFail3 = at + 16;

  const retarget = [];
  for (let i = Math.max(0, at - 400); i < at; i++) {
    if (code[i] === 0x0f && (code[i + 1] === 0x84 || code[i + 1] === 0x85)) {
      const next = i + 6;
      const tgt = next + code.readInt32LE(i + 2);
      if (tgt === oldFail2 || tgt === oldFail3) retarget.push({ i, tgt });
    }
  }

  // 27-byte window:
  // ExitProcess(0) [9] | mov eax,2; jmp fail_epi [7] | mov eax,3 [5] | add rsp; ret [5] | nop [1]
  const packed = Buffer.from([
    0x31, 0xc9, 0x41, 0xff, 0x97, 0x28, 0x00, 0x00, 0x00, // ExitProcess(0)
    0xb8, 0x02, 0x00, 0x00, 0x00, 0xeb, 0x05, // mov eax,2; jmp +5 → epi
    0xb8, 0x03, 0x00, 0x00, 0x00, // mov eax,3
    0x48, 0x83, 0xc4, 0x78, 0xc3, // fail epi
    0x90,
  ]);
  if (packed.length !== 27) throw new Error('packed len');
  packed.copy(code, at);

  for (const j of retarget) {
    const next = j.i + 6;
    const nt = j.tgt === oldFail2 ? newFail2 : newFail3;
    code.writeInt32LE(nt - next, j.i + 2);
  }

  const pref = parseInt(
    (pin.match(/^\s*yoyo_semrepro_prefix_bytes=(\d+)\s*$/m) ||
      pin.match(/^\s*yoyo_sembed_prefix_bytes=(\d+)\s*$/m))[1],
    10
  );
  const placeLen = pref + OLD_PLACE;
  fs.writeFileSync(codePath, code);
  fs.writeFileSync(path.join(projects, 'yoyo-plant-place.tyb'), packPlant(code.subarray(0, placeLen)));
  fs.writeFileSync(path.join(projects, 'yoyo-plant-parse1.tyb'), packPlant(code.subarray(placeLen)));
  fs.writeFileSync(path.join(projects, 'yoyo-plant.tyb'), packPlant(code));

  const plant = packPlant(Buffer.concat([code, data]));
  const oneshot = Buffer.alloc(12 + plant.length);
  oneshot.write('Y1PL', 0, 4, 'ascii');
  oneshot.writeUInt32LE(code.length, 4);
  oneshot.writeUInt32LE(data.length, 8);
  plant.copy(oneshot, 12);
  fs.writeFileSync(path.join(projects, 'yoyo-semcomp-self.tyb'), oneshot);

  const codeSha = crypto.createHash('sha256').update(code).digest('hex');
  pin = setPin(pin, 'code_sha256', codeSha);
  pin = setPin(pin, 'code_sha256_prefix', codeSha.slice(0, 16));
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
  pin = setPin(pin, 'yoyo_semcomp_self_tyb_sha256', crypto.createHash('sha256').update(oneshot).digest('hex'));
  pin = setPin(pin, 'yoyo_semav', 'PRESENT');
  pin = setPin(pin, 'yoyo_semav_success_patch_at', String(at));
  pin = setPin(pin, 'yoyo_semav_retargets', String(retarget.length));
  pin = setPin(pin, 'note', 'SEMAV_write_success_ExitProcess_inline;not_Thompson');
  pin = setPin(pin, 'doc', 'AUTO-SEMAV.md');
  fs.writeFileSync(pinPath, pin);

  console.log('SEMAV success_patch_at=' + at);
  console.log('SEMAV fail_retargets=' + retarget.length);
  console.log('SEMAV code_bytes=' + code.length);
  console.log('SEMAV peSize=' + layoutFor(code.length, data.length, 0x38000).peSize);
  console.log('SEMAV status=GREEN');
}

main();
