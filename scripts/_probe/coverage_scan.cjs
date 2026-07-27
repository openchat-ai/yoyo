// opcode coverage scan — parse yoyo.ty → (opcode, shape, handler) coverage
const fs = require('fs');
const src = fs.readFileSync('yoyo/projects/yoyo.ty', 'utf8');
const lines = src.split(/\n/);
const coverage = {};
let curHandler = '';

for (const ln of lines) {
  const t = ln.trim();
  if (!t || t.startsWith(';')) continue;
  const toks = t.split(/\s+/).filter(Boolean);
  if (toks.length === 0) continue;
  const op = toks[0].toUpperCase();
  if (!/^[0-9A-F]{1,2}$/.test(op)) continue;
  if (op === '40') {
    curHandler = 'H_' + (toks[1] || '?').toUpperCase();
    continue;
  }
  if (!coverage[op]) coverage[op] = { count: 0, handlers: new Set(), shapes: new Set(), examples: new Set() };
  coverage[op].count++;
  coverage[op].handlers.add(curHandler);
  const arg = toks.slice(1);
  let shape;
  if (op === 'FF') shape = 'ret(0)';
  else if (op === 'A0') shape = 'RAW_BYTE imm1';
  else if (op === 'A1') shape = 'RAW_BYTES imm×' + arg.length;
  else if (arg.length === 0) shape = '0-arg';
  else if (arg.length === 1) shape = '1-arg';
  else if (arg.length === 2) shape = '2-arg';
  else if (arg.length === 3) shape = '3-arg';
  else shape = 'varargs';
  coverage[op].shapes.add(shape);
  if (coverage[op].examples.size < 2) coverage[op].examples.add(op + ' ' + arg.join(' '));
}

const rows = Object.entries(coverage).sort((a, b) => Number('0x' + a[0]) - Number('0x' + b[0]));
console.log('OP | count | #handlers | shapes | example');
console.log('----|-------|-----------|--------|--------');
for (const [op, c] of rows) {
  const hn = [...c.handlers].slice(0, 3).join(',');
  const sh = [...c.shapes].join('/');
  const ex = [...c.examples].join(';');
  console.log(op + ' | ' + c.count + ' | ' + c.handlers.size + ' | ' + sh + ' | ' + ex);
}
console.log('\nTOTAL DISTINCT OPCODES COVERED: ' + rows.length);
