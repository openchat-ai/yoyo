'use strict';
/**
 * AUTO-LLMWRITEDRIFT / SCALE / WRITEX: unambiguous surface + artifact pin schema.
 * Grammar+dataflow authority; shapes aliases; generative round-trip; v4 pins.
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
  if (
    text.charCodeAt(0) === 0xfeff ||
    (text.length >= 3 && text.charCodeAt(0) === 0xef && text.charCodeAt(1) === 0xbb && text.charCodeAt(2) === 0xbf)
  ) {
    errors.push('forbid_bom');
  }
  if (text.includes('\r')) errors.push('forbid_cr');
  if (text.includes('\t')) errors.push('forbid_tab');
  if (text.includes(';')) errors.push('forbid_comments');
  const lines = text.split('\n');
  if (lines.length && lines[lines.length - 1] === '') lines.pop();
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
  if (lines[0] !== contract.entry_line) errors.push('entry_line_mismatch');
  else ops.push({ op: '40', args: ['00'] });

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
    if (body !== body.trim()) errors.push(`trim_line_${i}`);
    if (/\s{2,}/.test(body.trim())) errors.push(`multi_space_line_${i}`);
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
    lines.push(contract.body_indent + [ops[i].op, ...ops[i].args].join(' '));
  }
  return lines.join('\n') + '\n';
}

function classifyStmt(op, args, contract) {
  if (op === '30' && args[0] === contract.slot_size && HEX2.test(args[1] || '')) return 'SET_SIZE';
  if (op === '20' && args[0] === contract.slot_buffer && HEX2.test(args[1] || '')) return 'ALLOC_BUF';
  if (op === '50' && args[0] === contract.slot_buffer && args[1] === '00') return 'LOAD_IN';
  if (op === '51' && args[0] === contract.slot_buffer && args[1] === '02' && args[2] === contract.slot_size) {
    return 'WRITE_OUT';
  }
  if (op === 'A0') return 'EXIT_BYTE';
  return null;
}

function semanticAndGrammar(ops, contract) {
  const errors = [];
  const allowed = contract.allowed_opcodes;
  const exitBytes = contract.exit_trampoline_bytes;
  const rawBytes = [];
  let inRaw = false;
  let has_size = false;
  let has_buf = false;
  let writeCount = 0;
  const stmts = [];

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

    // slot / str checks (fail-closed)
    if (op === '20' && args[0] !== contract.slot_buffer) errors.push('alloc_slot_must_be_50');
    if (op === '30' && args[0] !== contract.slot_size) errors.push('set_slot_must_be_51');
    if (op === '50') {
      if (args[0] !== contract.slot_buffer) errors.push('load_slot_must_be_50');
      if (!contract.str_load_allowed.includes(args[1])) errors.push('load_str_not_allowed');
    }
    if (op === '51') {
      if (args[0] !== contract.slot_buffer) errors.push('write_slot_must_be_50');
      if (!contract.str_write_allowed.includes(args[1])) errors.push('write_str_not_allowed');
      if (args[2] !== contract.slot_size) errors.push('write_sz_must_be_51');
    }

    const kind = classifyStmt(op, args, contract);
    if (i > 0 && !kind) {
      errors.push(`stmt_not_in_grammar_${op}`);
      continue;
    }
    if (kind && kind !== 'EXIT_BYTE') {
      if (kind === 'WRITE_OUT') {
        if (!has_size) errors.push('dataflow_write_without_size');
        if (!has_buf) errors.push('dataflow_write_without_buf');
        writeCount += 1;
      }
      if (kind === 'SET_SIZE') has_size = true;
      if (kind === 'ALLOC_BUF' || kind === 'LOAD_IN') has_buf = true;
      stmts.push({ kind, text: [op, ...args].join(' ') });
    }
  }

  if (!rawBytes.length) errors.push('missing_exit_trampoline');
  else if (rawBytes.join(',') !== exitBytes.join(',')) errors.push('exit_trampoline_mismatch');

  if (contract.grammar && contract.grammar.require_write && writeCount < 1) {
    errors.push('grammar_require_write');
  }

  return { errors, stmts, writeCount };
}

function multiScaleAst(ops, stmts, contract) {
  const lines = [contract.entry_line];
  for (let i = 1; i < ops.length; i++) {
    lines.push(contract.body_indent + [ops[i].op, ...ops[i].args].join(' '));
  }
  const exit = [];
  for (let i = 1; i < ops.length; i++) {
    if (ops[i].op === 'A0') exit.push(ops[i].args[0]);
  }
  return {
    lines,
    stmts: stmts.map((s) => s.kind + ':' + s.text),
    program: {
      entry: '40 00',
      body: stmts.map((s) => s.text),
      exit,
    },
  };
}

function matchShapeAlias(ops, contract) {
  const body = [];
  for (let i = 1; i < ops.length; i++) {
    if (ops[i].op === 'A0') break;
    body.push([ops[i].op, ...ops[i].args].join(' '));
  }
  for (const [name, seq] of Object.entries(contract.shapes || {})) {
    if (body.length === seq.length && body.every((b, i) => b === seq[i])) return name;
  }
  return null;
}

function validateText(text, contract) {
  const { errors: lexErrs, lines } = lexCheck(text);
  const { ops, errors: parseErrs } = parseStrict(lines, contract);
  const g = ops.length ? semanticAndGrammar(ops, contract) : { errors: [], stmts: [] };
  const errors = [...lexErrs, ...parseErrs, ...g.errors];
  let canon = null;
  let ast = null;
  let shape = null;
  if (!errors.length) {
    canon = canonical(ops, contract);
    if (text !== canon) errors.push('canonical_mismatch');
    shape = matchShapeAlias(ops, contract);
    ast = multiScaleAst(ops, g.stmts, contract);
  }
  return { errors, ops, canon, ast, shape, stmts: g.stmts };
}

function checkGood(filePath, contract) {
  const name = path.basename(filePath, '.ty');
  const text = fs.readFileSync(filePath, 'utf8');
  const r = validateText(text, contract);
  if (r.errors.length) {
    console.log(`LLMSCALE sample=${name} status=RED reason=${r.errors.join('|')}`);
    return false;
  }
  const astPath = path.join(path.dirname(filePath), `${name}.ast.json`);
  if (!fs.existsSync(astPath)) {
    console.log(`LLMSCALE sample=${name} status=RED reason=ast_missing`);
    return false;
  }
  const want = JSON.stringify(JSON.parse(fs.readFileSync(astPath, 'utf8')));
  const got = JSON.stringify(r.ast);
  if (got !== want) {
    console.log(`LLMSCALE sample=${name} status=RED reason=ast_mismatch`);
    return false;
  }
  // round-trip idempotent
  const again = validateText(r.canon, contract);
  if (again.errors.length || again.canon !== r.canon) {
    console.log(`LLMSCALE sample=${name} status=RED reason=roundtrip_fail`);
    return false;
  }
  console.log(`LLMSCALE sample=${name} stmts=${r.stmts.length} shape_alias=${r.shape || '-'} status=GREEN`);
  return true;
}

function checkPoison(filePath, contract, reasonsSeen) {
  const name = path.basename(filePath);
  const text = fs.readFileSync(filePath, 'utf8');
  const r = validateText(text, contract);
  if (!r.errors.length) {
    console.log(`LLMSCALE poison=${name} status=RED reason=expected_fail_got_pass`);
    return false;
  }
  reasonsSeen.add(r.errors[0]);
  console.log(`LLMSCALE poison=${name} status=RED_as_expected errs=${r.errors[0]}`);
  return true;
}

function imm2(n) {
  return n.toString(16).toUpperCase().padStart(2, '0');
}

function buildProgram(contract, stmts) {
  const ops = [{ op: '40', args: ['00'] }];
  for (const s of stmts) {
    const p = s.split(' ');
    ops.push({ op: p[0], args: p.slice(1) });
  }
  for (const b of contract.exit_trampoline_bytes) {
    ops.push({ op: 'A0', args: [b] });
  }
  return canonical(ops, contract);
}

function generativeRoundTrip(contract) {
  const gen = contract.generative || { trials: 32, imm_choices: ['01', '03', '05', '08'], max_extra_sets: 2 };
  const imms = gen.imm_choices;
  let ok = 0;
  for (let t = 0; t < gen.trials; t++) {
    const mode = t % 4;
    const stmts = [];
    const extra = t % ((gen.max_extra_sets || 2) + 1);
    for (let e = 0; e < extra; e++) {
      stmts.push(`30 51 ${imms[(t + e) % imms.length]}`);
    }
    if (mode === 0) {
      // load path
      stmts.push(`30 51 ${imms[t % imms.length]}`);
      stmts.push('50 50 00');
      stmts.push('51 50 02 51');
    } else if (mode === 1) {
      // alloc path
      stmts.push(`20 50 ${imms[(t + 1) % imms.length]}`);
      stmts.push(`30 51 ${imms[t % imms.length]}`);
      stmts.push('51 50 02 51');
    } else if (mode === 2) {
      // resize then load
      stmts.push(`30 51 ${imms[0]}`);
      stmts.push(`30 51 ${imms[t % imms.length]}`);
      stmts.push('50 50 00');
      stmts.push('51 50 02 51');
    } else {
      // alloc then load overwrites buf
      stmts.push(`20 50 ${imms[t % imms.length]}`);
      stmts.push(`30 51 ${imms[(t + 2) % imms.length]}`);
      stmts.push('50 50 00');
      stmts.push('51 50 02 51');
    }
    const text = buildProgram(contract, stmts);
    const r = validateText(text, contract);
    if (r.errors.length) {
      console.log(`LLMSCALE gen=${t} status=RED reason=${r.errors.join('|')}`);
      return false;
    }
    if (r.canon !== text) {
      console.log(`LLMSCALE gen=${t} status=RED reason=gen_canon_mismatch`);
      return false;
    }
    const r2 = validateText(r.canon, contract);
    if (r2.errors.length || r2.canon !== r.canon) {
      console.log(`LLMSCALE gen=${t} status=RED reason=gen_roundtrip`);
      return false;
    }
    ok += 1;
  }
  console.log(`LLMSCALE generative trials=${ok} status=GREEN`);
  return true;
}

function main() {
  const contract = loadContract();
  if (contract.revision < 4) {
    console.log('LLMSCALE status=RED reason=contract_not_v4');
    process.exit(1);
  }
  if (!contract.artifact_pins || !contract.drift) {
    console.log('LLMSCALE status=RED reason=drift_schema_missing');
    process.exit(1);
  }
  if (!fs.existsSync(path.join(anyprog, 'MODEL-CONTRACT.md'))) {
    console.log('LLMSCALE status=RED reason=MODEL-CONTRACT_missing');
    process.exit(1);
  }
  if (!fs.existsSync(pinPath)) {
    console.log('LLMSCALE status=RED reason=contract_pin_missing');
    process.exit(1);
  }
  const gotPin = sha256file(contractPath);
  const wantPin = fs.readFileSync(pinPath, 'utf8').trim().split(/\s+/)[0].toLowerCase();
  if (gotPin !== wantPin) {
    console.log(`LLMSCALE status=RED reason=contract_pin_DIFF want=${wantPin} got=${gotPin}`);
    process.exit(1);
  }
  console.log(`LLMSCALE contract_pin=EQUAL sha256_prefix=${gotPin.slice(0, 16)}`);

  const cov = contract.surface_coverage || {};
  const requiredTools = [
    'cat', 'head', 'zero', 'copy08', 'resize',
    'hello_console', 'cmdline', 'tick', 'thread', 'socket',
  ];
  for (const t of requiredTools) {
    if (!cov[t]) {
      console.log(`LLMSCALE status=RED reason=surface_coverage_missing tool=${t}`);
      process.exit(1);
    }
  }
  const inNames = Object.entries(cov).filter(([, v]) => v === 'in_contract').map(([k]) => k).sort();
  console.log(`LLMSCALE surface_in=${inNames.join(',')} classified=${requiredTools.length}`);
  for (const n of inNames) {
    const pin = contract.artifact_pins[n];
    if (!pin || !/^[0-9a-f]{64}$/.test(String(pin))) {
      console.log(`LLMSCALE status=RED reason=artifact_pin_bad sample=${n}`);
      process.exit(1);
    }
  }
  if (!contract.drift.rebuild_times || contract.drift.rebuild_times < 2) {
    console.log('LLMSCALE status=RED reason=drift_rebuild_times');
    process.exit(1);
  }

  const llmDir = path.join(root, contract.samples_llm_dir.replace(/\//g, path.sep));
  const files = fs.readdirSync(llmDir).filter((f) => f.endsWith('.ty'));
  const goods = files.filter((f) => !f.startsWith('_poison')).sort();
  const poisons = files.filter((f) => f.startsWith('_poison')).sort();
  if (goods.length < 5) {
    console.log(`LLMSCALE status=RED reason=too_few_samples n=${goods.length} want>=5`);
    process.exit(1);
  }

  let ok = true;
  for (const f of goods) {
    if (!checkGood(path.join(llmDir, f), contract)) ok = false;
  }
  const reasons = new Set();
  for (const f of poisons) {
    if (!checkPoison(path.join(llmDir, f), contract, reasons)) ok = false;
  }
  const minP = contract.min_poison_distinct_reasons || 10;
  if (reasons.size < minP) {
    console.log(`LLMSCALE status=RED reason=poison_diversity got=${reasons.size} want>=${minP}`);
    process.exit(1);
  }
  if (!generativeRoundTrip(contract)) ok = false;

  // dataflow poisons must exist
  if (!reasons.has('dataflow_write_without_size') && !fs.existsSync(path.join(llmDir, '_poison_df_size.ty'))) {
    console.log('LLMSCALE status=RED reason=missing_dataflow_poison');
    process.exit(1);
  }

  if (!ok) {
    console.log('LLMSCALE status=RED');
    process.exit(1);
  }
  console.log(`LLMSCALE samples=${goods.length} poisons=${poisons.length} distinct_reasons=${reasons.size}`);
  console.log('LLMSCALE status=GREEN');
  console.log('LLMSCALE doc=AUTO-LLMWRITESCALE.md');
  console.log('LLMDRIFT schema=artifact_pins+drift status=STRUCT_GREEN');
  console.log('LLMDRIFT doc=AUTO-LLMWRITEDRIFT.md');
  process.exit(0);
}

main();
