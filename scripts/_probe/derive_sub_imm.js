// Body-extend-002 derivation: independent compute of expected bytes
// for `0x61 0x50 0x03` (SUB S[0x50] -= 3) at H_2F.
// JS path: loadState(0x50,rax) + subImmRax(3) + storeState(0x50,rax) + 0xC3
const { encodeOp, loadState, storeState } = require('../../yoyo-js/src/platform/encode-x64');
const ls = loadState(0x50, 0, 0);     // slot 0x50*8=640 > 127 → disp32
const ss = storeState(0x50, 0, 0);     // disp32
// 0x61 = SUB slot,imm path. SUB imm8 = 48 83 e8 03 (REX.W + 83 /5 + imm8)
const out = [...ls, 0x48, 0x83, 0xE8, 0x03, ...ss, 0xC3];
console.log('JS stream:', Buffer.from(out).toString('hex'));
console.log('JS len:', out.length);
console.log('JS components:');
console.log('  load  S[0x50]:', Buffer.from(ls).toString('hex'));
console.log('  sub imm=3   :', '48 83 e8 03');
console.log('  store S[0x50]:', Buffer.from(ss).toString('hex'));
console.log('  ret         : c3');
