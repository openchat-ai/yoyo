import fs from 'fs';
import path from 'path';

const ROOT = 'f:/yoyo';
const dash = '\u2014';
const p = path.join(ROOT, 'yoyo-rust/verifier/src/main.rs');
let c = fs.readFileSync(p, 'utf8');
c = c.replace(/\u20AC\u017D\?/g, dash);
// also fix common 3-byte mojibake for emdash when file was mis-decoded
c = c.split('鈥?').join(dash);

const s = c.indexOf('    match check_selfhost_min_set_large(&root) {');
const e = c.indexOf('    match check_selfhost_min_jmp(&root) {');
if (s < 0 || e < 0) {
  console.error('arms markers', s, e);
  process.exit(1);
}

const names = [
  'orv_h52',
  'subv_h52',
  'imul_swap',
  'imul_h52',
  'cmp_swap',
  'get_h52',
  'set_deadbeef',
  'ldb_dst51',
];
const labels = [
  'ORV-H52',
  'SUBV-H52',
  'IMUL-SWAP',
  'IMUL-H52',
  'CMP-SWAP',
  'GET-H52',
  'SET-DEADBEEF',
  'LDB-DST51',
];

let arms =
  '    match check_selfhost_min_set_large(&root) {\n' +
  `        Ok(detail) => println!("G-SM-SET-LARGE PASS ${dash} {detail}"),\n` +
  '        Err(e) => {\n' +
  '            failed += 1;\n' +
  `            eprintln!("G-SM-SET-LARGE FAIL ${dash} {e}");\n` +
  '        }\n' +
  '    }\n' +
  '    total += 1;\n';

for (let i = 0; i < names.length; i++) {
  arms +=
    `    match check_selfhost_min_${names[i]}(&root) {\n` +
    `        Ok(detail) => println!("G-SM-${labels[i]} PASS ${dash} {detail}"),\n` +
    '        Err(e) => {\n' +
    '            failed += 1;\n' +
    `            eprintln!("G-SM-${labels[i]} FAIL ${dash} {e}");\n` +
    '        }\n' +
    '    }\n' +
    '    total += 1;\n';
}

c = c.slice(0, s) + arms + c.slice(e);

const setLargeOk = c.indexOf('via opcode set-large+FF');
if (setLargeOk < 0) {
  console.error('set-large ok missing');
  process.exit(1);
}
const after = c.indexOf('\n}\n\n', setLargeOk) + 4;
const jmpDoc = c.indexOf('/// W-SM control flow: Rust-only golden for H_19', after);
if (jmpDoc < 0) {
  console.error('jmp doc missing');
  process.exit(1);
}
const checkFns = fs.readFileSync(
  path.join(ROOT, 'docs/auxdocs/_body_extend_016_main_fns.rs.txt'),
  'utf8'
);
c = c.slice(0, after) + checkFns + '\n' + c.slice(jmpDoc);

c = c.replace(
  /G-SM-SET-LARGE \+ .*? \+ G-SM-JMP \+ G-SM-CALL/,
  'G-SM-SET-LARGE + G-SM-ORV-H52 + G-SM-SUBV-H52 + G-SM-IMUL-SWAP + G-SM-IMUL-H52 + G-SM-CMP-SWAP + G-SM-GET-H52 + G-SM-SET-DEADBEEF + G-SM-LDB-DST51 + G-SM-JMP + G-SM-CALL'
);
c = c.replace(
  /G-SM-SET-LARGE, .*?, G-SM-JMP, G-SM-CALL/,
  'G-SM-SET-LARGE, G-SM-ORV-H52, G-SM-SUBV-H52, G-SM-IMUL-SWAP, G-SM-IMUL-H52, G-SM-CMP-SWAP, G-SM-GET-H52, G-SM-SET-DEADBEEF, G-SM-LDB-DST51, G-SM-JMP, G-SM-CALL'
);

fs.writeFileSync(p, c);
console.log('main.rs fixed');
console.log('ldb arm', c.includes('check_selfhost_min_ldb_dst51(&root)'));
console.log('inc arm', c.includes('check_selfhost_min_inc_h51(&root)'));
console.log('get_h52_50 fn', c.includes('fn check_selfhost_min_get_h52_50'));
console.log('imul_h52 fn', c.includes('fn check_selfhost_min_imul_h52'));
console.log('set_deadbeef @0x42', /fn check_selfhost_min_set_deadbeef[\s\S]{0,600}0x42/.test(c));
console.log('orv @0x3C', /fn check_selfhost_min_orv_h52[\s\S]{0,600}0x3C/.test(c));
