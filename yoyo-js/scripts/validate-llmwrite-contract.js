'use strict';
/**
 * AUTO-LLMWRITEX: aerospace fail-closed LLM write-surface checker.
 * Lexical + semantic + canonical round-trip + AST gold + contract pin + poison diversity.
 */
const fs = require('fs');
const path = require('path');
const crypto = require('crypto');

const root = path.join(__dirname, '..', '..');
const anyprog = path.join(root, 'scripts', 'fixtures', 'anyprog');
const contractPath = path.join(anyprog, 'model-contract.json');
const pinPath = path.join(anyprog, 'model-contract.sha256');
const HEX2 = /^[0-9A-F]{2}$/;

function loadContract() {
  return JSON.parse(fs.readFileSync(contractPath, 'utf8'));
}

function sha256file(p) {
  return crypto.createHash('sha256').update(fs.readFileSync(p)).digest('hex');
}

function lexCheck(text) {
  const errors = [];
  if (text.charCodeAt(0) === 0xfeff || (text.length >= 3 && text.charCodeAt(0) === 0xef)) {
    // BOM UTF-8 EF BB BF
    if (text.charCodeAt(0) === 0xfeff || (text.charCodeAt(0) === 0xef && text.charCodeAt(1) === 0xbb)) {
      errors.push('forbid_bom');
    }
  }
  if (text.includes('\r')) errors.push('forbid_cr');
  if (text.includes('\t')) errors.push('forbid_tab');
  if (text.includes(';')) errors.push('forbid_comments');
  const lines = text.split('\n');
  if (lines.length && lines[lines.length - 1] === '') lines.pop(); // allow final newline only
  for (let i = 0; i < lines.length; i++) {
    if (lines[i] === '') errors.push(`blank_line_${i}`);
  }
  return { errors, lines };
}

function parseStrict(lines, contract) {
  const errors = [];
  const ops = [];
  if (!lines.length) {
    errors.push('empty');
    return { ops, errors };
  }
  if (lines[0] !== contract.entry_line) {
    errors.push('entry_line_mismatch');
  } else {
    ops.push({ op: '40', args: ['00'] });
  }
  const indent = contract.body_indent;
  for (let i = 1; i < lines.length; i++) {
    const line = lines[i];
    if (!line.startsWith(indent)) {
      errors.push(`indent_line_${i}`);
      continue;
    }
    if (line.length > indent.length && line[indent.length] === ' ') {
      errors.push(`over_indent_line_${i}`);
    }
    const body = line.slice(indent.length);
    if (body !== body.trim() || /\s{2,}/.test(body)) {
      // single spaces only between tokens; no lead/trail in body
      if (body !== body.trim()) errors.push(`trim_line_${i}`);
      if (/\s{2,}/.test(body.trim())) errors.push(`multi_space_line_${i}`);
    }
    const parts = body.trim().split(' ');
    for (const t of parts) {
      if (!HEX2.test(t)) errors.push(`token_not_upper_hex2_${t}_line_${i}`);
    }
    if (!parts.length) {
      errors.push(`empty_body_line_${i}`);
      continue;
    }
    ops.push({ op: parts[0], args: parts.slice(1) });
  }
  return { ops, errors };
}

function canonical(ops, contract) {
  const lines = [contract.entry_line];
  for (let i = 1; i < ops.length; i++) {
    const o = ops[i];
    lines.push(contract.body_indent + [o.op, ...o.args].join(' '));
  }
  return lines.join('\n') + '\n';
}

function astOf(ops) {
  return {
    entry: '40 00',
    body: ops.slice(1).map((o) => [o.op, ...o.args].join(' ')),
  };
}

function semanticCheck(ops, contract) {
  const errors = [];
  const allowed = contract.allowed_opcodes;
  const exitBytes = contract.exit_trampoline_bytes;
  const rawBytes = [];
  let inRaw = false;

  for (let i = 0; i < ops.length; i++) {
    const { op, args } = ops[i];
    if (op === 'FF') {
      errors.push('forbid_ret_ff');
      continue;
    }
    if (i > 0 && op === '40') errors.push(`extra_handler_at_${i}`);
    const spec = allowed[op];
    if (!spec) {
      errors.push(`opcode_not_allowed_${op}`);
      continue;
    }
    if (args.length !== spec.arity) {
      errors.push(`arity_${op}_want_${spec.arity}_got_${args.length}`);
      continue;
    }

    if (op === 'A0') {
      inRaw = true;
      rawBytes.push(args[0]);
      continue;
    }
    if (inRaw) errors.push('raw_must_be_tail_only');

    if (op === '20') {
      if (args[0] !== contract.slot_buffer) errors.push('alloc_slot_must_be_50');
    }
    if (op === '30') {
      if (args[0] !== contract.slot_size) errors.push('set_slot_must_be_51');
    }
    if (op === '50') {
      if (args[0] !== contract.slot_buffer) errors.push('load_slot_must_be_50');
      if (!contract.str_load_allowed.includes(args[1])) errors.push('load_str_not_allowed');
    }
    if (op === '51') {
      if (args[0] !== contract.slot_buffer) errors.push('write_slot_must_be_50');
      if (!contract.str_write_allowed.includes(args[1])) errors.push('write_str_not_allowed');
      if (args[2] !== contract.slot_size) errors.push('write_sz_must_be_51');
    }
    for (const a of args) {
      if ((op === '20' || op === '30' || op === '50' || op === '51') && (a === '50' || a === '51')) {
        if (!contract.slots_allowed.includes(a) && (op === '20' || op === '30' || (op === '50' && args.indexOf(a) === 0) || (op === '51' && (args.indexOf(a) === 0 || args.indexOf(a) === 2)))) {
          /* covered above */
        }
      }
    }
  }

  if (!rawBytes.length) errors.push('missing_exit_trampoline');
  else if (rawBytes.join(',') !== exitBytes.join(',')) errors.push('exit_trampoline_mismatch');

  // shape: body before A0 must match a named shape if sample has shape name
  return errors;
}

function shapeBody(ops) {
  const body = [];
  for (let i = 1; i < ops.length; i++) {
    if (ops[i].op === 'A0') break;
    body.push([ops[i].op, ...ops[i].args].join(' '));
  }
  return body;
}

function matchShape(ops, contract) {
  const body = shapeBody(ops);
  for (const [name, seq] of Object.entries(contract.shapes || {})) {
    if (body.length === seq.length && body.every((b, i) => b === seq[i])) return name;
  }
  return null;
}

function validateText(text, contract) {
  const { errors: lexErrs, lines } = lexCheck(text);
  const { ops, errors: parseErrs } = parseStrict(lines, contract);
  const semErrs = ops.length ? semanticCheck(ops, contract) : [];
  const errors = [...lexErrs, ...parseErrs, ...semErrs];
  let canon = null;
  let ast = null;
  if (!errors.length) {
    canon = canonical(ops, contract);
    if (canon !== text && canon !== text.replace(/\r\n/g, '\n')) {
      // require exact canonical including trailing newline
      if (text !== canon) errors.push('canonical_mismatch');
    }
    ast = astOf(ops);
    if (!matchShape(ops, contract)) errors.push('shape_not_named');
  }
  return { errors, ops, canon, ast, shape: errors.includes('shape_not_named') ? null : matchShape(ops, contract) };
}

function checkGood(filePath, contract) {
  const name = path.basename(filePath, '.ty');
  const text = fs.readFileSync(filePath, 'utf8');
  const r = validateText(text, contract);
  if (r.errors.length) {
    console.log(`LLMWRITEX sample=${name} status=RED reason=${r.errors.join('|')}`);
    return { ok: false };
  }
  const astPath = path.join(path.dirname(filePath), `${name}.ast.json`);
  if (!fs.existsSync(astPath)) {
    console.log(`LLMWRITEX sample=${name} status=RED reason=ast_missing`);
    return { ok: false };
  }
  const wantAst = JSON.parse(fs.readFileSync(astPath, 'utf8'));
  const got = JSON.stringify(r.ast);
  const want = JSON.stringify(wantAst);
  if (got !== want) {
    console.log(`LLMWRITEX sample=${name} status=RED reason=ast_mismatch`);
    return { ok: false };
  }
  if (r.shape !== name) {
    console.log(`LLMWRITEX sample=${name} status=RED reason=shape_name_mismatch got=${r.shape}`);
    return { ok: false };
  }
  console.log(`LLMWRITEX sample=${name} shape=${r.shape} status=GREEN`);
  return { ok: true };
}

function checkPoison(filePath, contract, reasonsSeen) {
  const name = path.basename(filePath);
  const text = fs.readFileSync(filePath, 'utf8');
  // poisons may violate lexical rules — use soft path: still run validateText
  const r = validateText(text, contract);
  if (!r.errors.length) {
    console.log(`LLMWRITEX poison=${name} status=RED reason=expected_fail_got_pass`);
    return false;
  }
  const reason = r.errors[0];
  reasonsSeen.add(reason);
  console.log(`LLMWRITEX poison=${name} status=RED_as_expected errs=${reason}`);
  return true;
}

function main() {
  const contract = loadContract();
  if (contract.revision !== 2) {
    console.log('LLMWRITEX status=RED reason=contract_not_v2');
    process.exit(1);
  }
  if (!fs.existsSync(path.join(anyprog, 'MODEL-CONTRACT.md'))) {
    console.log('LLMWRITEX status=RED reason=MODEL-CONTRACT_missing');
    process.exit(1);
  }
  if (!fs.existsSync(pinPath)) {
    console.log('LLMWRITEX status=RED reason=contract_pin_missing');
    process.exit(1);
  }
  const gotPin = sha256file(contractPath);
  const wantPin = fs.readFileSync(pinPath, 'utf8').trim().split(/\s+/)[0].toLowerCase();
  if (gotPin !== wantPin) {
    console.log(`LLMWRITEX status=RED reason=contract_pin_DIFF want=${wantPin} got=${gotPin}`);
    process.exit(1);
  }
  console.log(`LLMWRITEX contract_pin=EQUAL sha256_prefix=${gotPin.slice(0, 16)}`);

  // 全域：ANYPROG 已知工具必须在 surface_coverage 里显式 in/out（禁漏网）
  const cov = contract.surface_coverage || {};
  const requiredTools = [
    'cat',
    'head',
    'zero',
    'hello_console',
    'cmdline',
    'tick',
    'thread',
    'socket',
  ];
  for (const t of requiredTools) {
    if (!cov[t]) {
      console.log(`LLMWRITEX status=RED reason=surface_coverage_missing tool=${t}`);
      process.exit(1);
    }
  }
  const inNames = Object.entries(cov)
    .filter(([, v]) => v === 'in_contract')
    .map(([k]) => k)
    .sort();
  console.log(`LLMWRITEX surface_in=${inNames.join(',')} surface_classified=${requiredTools.length}`);

  const llmDir = path.join(root, contract.samples_llm_dir.replace(/\//g, path.sep));
  const files = fs.readdirSync(llmDir).filter((f) => f.endsWith('.ty'));
  const goods = files.filter((f) => !f.startsWith('_poison')).sort();
  const poisons = files.filter((f) => f.startsWith('_poison')).sort();
  if (goods.length < 3) {
    console.log(`LLMWRITEX status=RED reason=too_few_samples n=${goods.length}`);
    process.exit(1);
  }

  let ok = true;
  for (const f of goods) {
    if (!checkGood(path.join(llmDir, f), contract).ok) ok = false;
  }
  const reasons = new Set();
  for (const f of poisons) {
    if (!checkPoison(path.join(llmDir, f), contract, reasons)) ok = false;
  }
  const minP = contract.min_poison_distinct_reasons || 10;
  if (reasons.size < minP) {
    console.log(`LLMWRITEX status=RED reason=poison_diversity got=${reasons.size} want>=${minP}`);
    console.log(`LLMWRITEX reasons=${[...reasons].join(',')}`);
    process.exit(1);
  }
  if (!ok) {
    console.log('LLMWRITEX status=RED');
    process.exit(1);
  }
  console.log(`LLMWRITEX samples=${goods.length} poisons=${poisons.length} distinct_reasons=${reasons.size}`);
  console.log('LLMWRITEX status=GREEN');
  console.log('LLMWRITEX doc=AUTO-LLMWRITEX.md');
  process.exit(0);
}

main();
