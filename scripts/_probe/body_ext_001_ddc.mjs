// Compile yoyo.ty via both peers and compare code bytes (NTP: stub)
import { execSync } from 'child_process';
import fs from 'fs';
import crypto from 'crypto';

const yoyoTy = 'f:/yoyo/yoyo/projects/yoyo.ty';

// JS stream
const jsOut = execSync(`node f:/yoyo/scripts/_probe/js-ty2text.mjs "${yoyoTy}"`, { encoding: 'buffer' });

// Rust stream (raw code, not PE) — use --target=stub which writes flat code
// We need only the code, not the PE wrapper. Build a probe .ty that wraps a
// single emit invocation — but simpler: use compile_one_handler is not
// applicable. Use yoyo link --target=stub and skip the startup blob.
//
// Looking at yoyo-rust/verifier/src/main.rs cmd_link: for Stub it writes
//   blob = startup_blob_baremetal() ++ code
// So we need to know the size of the startup blob. Looking at startup.rs.
const startupSize = 1;  // stub startup is 1 byte C3 (movable)
const rustBinPath = 'f:/yoyo/scripts/_probe/yoyoty_code.bin';
execSync(`cd f:/yoyo/yoyo-rust && cargo run -q -p verifier --bin yoyo -- link --target=stub "${yoyoTy}" "${rustBinPath}"`, { stdio: 'inherit' });
const rustBlob = fs.readFileSync(rustBinPath);
const rustOut = rustBlob.subarray(startupSize);

console.log(`JS code length:  ${jsOut.length} bytes`);
console.log(`Rust code length: ${rustOut.length} bytes`);

const jsHash = crypto.createHash('sha256').update(jsOut).digest('hex');
const rustHash = crypto.createHash('sha256').update(rustOut).digest('hex');
console.log(`JS sha256:  ${jsHash}`);
console.log(`Rust sha256: ${rustHash}`);

if (jsOut.equals(rustOut)) {
  console.log('JS <-> Rust code: EQUAL ✓');
  process.exit(0);
} else {
  console.log('JS <-> Rust code: DIFFER ✗');
  // Show first diff
  const min = Math.min(jsOut.length, rustOut.length);
  for (let i = 0; i < min; i++) {
    if (jsOut[i] !== rustOut[i]) {
      console.log(`First diff at offset ${i}: js=${jsOut[i].toString(16)} rust=${rustOut[i].toString(16)}`);
      break;
    }
  }
  process.exit(1);
}
