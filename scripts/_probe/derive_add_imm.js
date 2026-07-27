// Body-extend-001 derivation: independent compute of expected bytes
// for `0x62 0x50 0x03` (ADD S[0x50] += 3) at H_2E.
// JS path: loadState(0x50,rax) + addImmRax(3) + storeState(0x50,rax) + 0xC3
const { encodeOp, loadState, storeState } = require('../../yoyo-js/src/platform/encode-x64');
const ls = loadState(0x50, 0, 0);     // slot 0x50*8=640 > 127 → disp32
const ar = encodeOp(0x62, [0x50, 3]).slice(loadState(0x50,0,0).length, -storeState(0x50,0,0).length);  // 0x62 = ADD slot,imm path
const ss = storeState(0x50, 0, 0);     // disp32
const out = [...ls, 0x48, 0x83, 0xC0, 0x03, ...ss, 0xC3];
console.log('JS stream:', Buffer.from(out).toString('hex'));
console.log('JS len:', out.length);
console.log('JS components:');
console.log('  load  S[0x50]:', Buffer.from(ls).toString('hex'));
console.log('  add imm=3   :', '48 83 c0 03');
console.log('  store S[0x50]:', Buffer.from(ss).toString('hex'));
console.log('  ret         : c3');
