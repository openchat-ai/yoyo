'use strict';
/**
 * AUTO-LLMWRITE: structural check — .ty matches model-contract.json (unambiguous LLM surface).
 */
const fs = require('fs');
const path = require('path');

const root = path.join(__dirname, '..', '..');
const contractPath = path.join(root, 'scripts', 'fixtures', 'anyprog', 'model-contract.json');

function loadContract() {
  return JSON.parse(fs.readFileSync(contractPath, 'utf8'));
}

function parseTy(text) {
  const ops = [];
  for (const raw of text.split(/\r?\n/)) {
    const line = raw.replace(/;.*$/, '').trim();
    if (!line) continue;
    const parts = line.split(/\s+/).filter(Boolean);
    const op = parts[0].toUpperCase();
    const args = parts.slice(1).map((x) => x.toUpperCase());
    ops.push({ op, args, raw: line });
  }
  return ops;
}

function validate(ops, contract) {
  const errors = [];
  if (!ops.length) {
    errors.push('empty');
    return errors;
  }
  if (ops[0].op !== '40' || ops[0].args.join(' ') !== '00') {
    errors.push('entry_must_be_40_00');
  }
  for (let i = 1; i < ops.length; i++) {
    if (ops[i].op === '40') errors.push(`extra_handler_at_${i}`);
  }

  const allowed = contract.allowed_opcodes;
  const exitBytes = contract.exit_trampoline_bytes.map((b) => b.toUpperCase());
  const rawBytes = [];
  let sawNonRawAfterRaw = false;
  let inRawTail = false;

  for (let i = 0; i < ops.length; i++) {
    const { op, args } = ops[i];
    if (op === 'FF' || (op === 'FF' && args.length === 0)) {
      errors.push('forbid_ret_ff');
      continue;
    }
    // lone FF as opcode token
    if (op === 'FF') {
      errors.push('forbid_ret_ff');
      continue;
    }
    const spec = allowed[op];
    if (!spec) {
      errors.push(`opcode_not_allowed_${op}`);
      continue;
    }
    if (args.length !== spec.arity) {
      errors.push(`arity_${op}_want_${spec.arity}_got_${args.length}`);
    }
    if (op === 'A0') {
      inRawTail = true;
      rawBytes.push(args[0]);
    } else if (inRawTail) {
      sawNonRawAfterRaw = true;
      errors.push('raw_must_be_tail_only');
    }
  }

  if (sawNonRawAfterRaw) {
    /* already pushed */
  }
  if (rawBytes.length === 0) {
    errors.push('missing_exit_trampoline');
  } else if (rawBytes.join(',') !== exitBytes.join(',')) {
    errors.push('exit_trampoline_mismatch');
  }

  // body before raw must not be empty for file samples — optional
  return errors;
}

function checkFile(filePath, contract, expectFail) {
  const ops = parseTy(fs.readFileSync(filePath, 'utf8'));
  // treat opcode FF written as "FF" with no args
  for (const o of ops) {
    if (o.op === 'FF') {
      /* handled */
    }
  }
  const errors = validate(ops, contract);
  // also catch bare FF line: parse as op FF
  const name = path.basename(filePath);
  if (expectFail) {
    if (errors.length === 0) {
      console.log(`LLMWRITE poison=${name} status=RED reason=expected_fail_got_pass`);
      return false;
    }
    console.log(`LLMWRITE poison=${name} status=RED_as_expected errs=${errors[0]}`);
    return true;
  }
  if (errors.length) {
    console.log(`LLMWRITE sample=${name} status=RED reason=${errors.join('|')}`);
    return false;
  }
  console.log(`LLMWRITE sample=${name} status=GREEN`);
  return true;
}

function main() {
  const contract = loadContract();
  const llmDir = path.join(root, contract.samples_llm_dir.replace(/\//g, path.sep));
  if (!fs.existsSync(path.join(root, 'scripts', 'fixtures', 'anyprog', 'MODEL-CONTRACT.md'))) {
    console.log('LLMWRITE status=RED reason=MODEL-CONTRACT_missing');
    process.exit(1);
  }
  if (!fs.existsSync(llmDir)) {
    console.log('LLMWRITE status=RED reason=llm_dir_missing');
    process.exit(1);
  }

  let ok = true;
  const files = fs.readdirSync(llmDir).filter((f) => f.endsWith('.ty'));
  const goods = files.filter((f) => !f.startsWith('_poison'));
  const poisons = files.filter((f) => f.startsWith('_poison'));
  if (goods.length < 3) {
    console.log(`LLMWRITE status=RED reason=too_few_samples n=${goods.length}`);
    process.exit(1);
  }
  for (const f of goods) {
    if (!checkFile(path.join(llmDir, f), contract, false)) ok = false;
  }
  for (const f of poisons) {
    if (!checkFile(path.join(llmDir, f), contract, true)) ok = false;
  }
  if (!poisons.length) {
    console.log('LLMWRITE status=RED reason=no_poison_fixtures');
    process.exit(1);
  }
  if (!ok) {
    console.log('LLMWRITE status=RED');
    process.exit(1);
  }
  console.log(`LLMWRITE samples=${goods.length} poisons=${poisons.length}`);
  console.log('LLMWRITE status=GREEN');
  console.log('LLMWRITE doc=AUTO-LLMWRITE.md');
  process.exit(0);
}

main();
