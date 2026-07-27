// Compile the H_22 handler from yoyo.ty via both peers and compare code bytes
import { execSync } from 'child_process';
import fs from 'fs';
import crypto from 'crypto';

const yoyoTy = 'f:/yoyo/yoyo/projects/yoyo.ty';

// JS: write a fixture that contains H_22 + RET
const fixturePath = 'f:/yoyo/scripts/_probe/h22_only.ty';
fs.writeFileSync(fixturePath, `; H_22 probe (body-extend-001)
40 22
  62 50 03
  FF
`);

// JS stream via M0 (using js-ty2text)
const jsOut = execSync(`node f:/yoyo/scripts/_probe/js-ty2text.mjs "${fixturePath}"`, { encoding: 'buffer' });

// Rust: use existing compile_one_handler via... hmm, no CLI for that. Use full
// link then verify the H_22 chunk matches the expected pin.
const rustBinPath = 'f:/yoyo/scripts/_probe/h22_code.bin';
execSync(`cd f:/yoyo/yoyo-rust && cargo run -q -p verifier --bin yoyo -- link --target=stub "${fixturePath}" "${rustBinPath}"`, { stdio: 'inherit' });
const rustBlob = fs.readFileSync(rustBinPath);
const startupSize = 1;  // stub startup is 1 byte C3
const rustOut = rustBlob.subarray(startupSize);

console.log(`H_22 JS code length:  ${jsOut.length} bytes`);
console.log(`H_22 Rust code length: ${rustOut.length} bytes`);

const jsHex = jsOut.toString('hex');
const rustHex = rustOut.toString('hex');
console.log(`H_22 JS:  ${jsHex}`);
console.log(`H_22 Rust: ${rustHex}`);

const expected = '498b87800200004883c00349898780020000c3';
if (jsOut.equals(rustOut) && jsHex === expected) {
  console.log('H_22 JS <-> Rust: EQUAL, matches pin ✓');
  process.exit(0);
} else {
  console.log('H_22 JS <-> Rust: DIFFER ✗');
  process.exit(1);
}
