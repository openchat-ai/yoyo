//! LLM write contract gate — AST + generative + schema (replaces Node validate).
//! Lex / file_grammar runtime gates remain `gate_*.ty`. Exact_body grammar checked here.

use crate::ddc;
use crate::types::IsaError;
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fs;
use std::path::Path;

struct Op {
    op: String,
    args: Vec<String>,
}

struct Stmt {
    kind: String,
    text: String,
}

struct ValidateOut {
    errors: Vec<String>,
    canon: Option<String>,
    ast: Option<Value>,
    shape: Option<String>,
    stmt_count: usize,
}

fn is_hex2(t: &str) -> bool {
    t.len() == 2
        && t.chars()
            .all(|c| c.is_ascii_digit() || ('A'..='F').contains(&c))
}

fn lex_check(text: &str) -> (Vec<String>, Vec<String>) {
    let mut errors = Vec::new();
    let bytes = text.as_bytes();
    if text.starts_with('\u{feff}')
        || (bytes.len() >= 3 && bytes[0] == 0xef && bytes[1] == 0xbb && bytes[2] == 0xbf)
    {
        errors.push("forbid_bom".into());
    }
    if text.contains('\r') {
        errors.push("forbid_cr".into());
    }
    if text.contains('\t') {
        errors.push("forbid_tab".into());
    }
    if text.contains(';') {
        errors.push("forbid_comments".into());
    }
    let mut lines: Vec<String> = text.split('\n').map(|s| s.to_string()).collect();
    if lines.last().map(|s| s.is_empty()).unwrap_or(false) {
        lines.pop();
    }
    for (i, line) in lines.iter().enumerate() {
        if line.is_empty() {
            errors.push(format!("blank_line_{i}"));
        }
    }
    (errors, lines)
}

fn parse_strict(lines: &[String], contract: &Value) -> (Vec<Op>, Vec<String>) {
    let mut errors = Vec::new();
    let mut ops = Vec::new();
    if lines.is_empty() {
        errors.push("empty".into());
        return (ops, errors);
    }
    let entry = contract["entry_line"].as_str().unwrap_or("40 00");
    let indent = contract["body_indent"].as_str().unwrap_or("  ");
    if lines[0] != entry {
        errors.push("entry_line_mismatch".into());
    } else {
        ops.push(Op {
            op: "40".into(),
            args: vec!["00".into()],
        });
    }
    for (i, line) in lines.iter().enumerate().skip(1) {
        if !line.starts_with(indent) {
            errors.push(format!("indent_line_{i}"));
            continue;
        }
        if line.len() > indent.len() && line.as_bytes()[indent.len()] == b' ' {
            errors.push(format!("over_indent_line_{i}"));
        }
        let body = &line[indent.len()..];
        if body != body.trim() {
            errors.push(format!("trim_line_{i}"));
        }
        let trimmed = body.trim();
        if trimmed.contains("  ") {
            errors.push(format!("multi_space_line_{i}"));
        }
        let parts: Vec<&str> = trimmed.split(' ').filter(|p| !p.is_empty()).collect();
        for t in &parts {
            if !is_hex2(t) {
                errors.push(format!("token_not_upper_hex2_{t}_line_{i}"));
            }
        }
        if parts.is_empty() {
            errors.push(format!("empty_body_line_{i}"));
            continue;
        }
        ops.push(Op {
            op: parts[0].to_string(),
            args: parts[1..].iter().map(|s| (*s).to_string()).collect(),
        });
    }
    (ops, errors)
}

fn canonical(ops: &[Op], contract: &Value) -> String {
    let indent = contract["body_indent"].as_str().unwrap_or("  ");
    let entry = contract["entry_line"].as_str().unwrap_or("40 00");
    let mut lines = vec![entry.to_string()];
    for op in ops.iter().skip(1) {
        let mut parts = vec![op.op.clone()];
        parts.extend(op.args.iter().cloned());
        lines.push(format!("{indent}{}", parts.join(" ")));
    }
    lines.join("\n") + "\n"
}

fn classify_stmt(op: &str, args: &[String], contract: &Value) -> Option<&'static str> {
    let slot_size = contract["slot_size"].as_str().unwrap_or("51");
    let slot_buf = contract["slot_buffer"].as_str().unwrap_or("50");
    if op == "30" && args.first().map(|s| s.as_str()) == Some(slot_size) && args.get(1).map(|s| is_hex2(s)).unwrap_or(false)
    {
        return Some("SET_SIZE");
    }
    if op == "20" && args.first().map(|s| s.as_str()) == Some(slot_buf) && args.get(1).map(|s| is_hex2(s)).unwrap_or(false)
    {
        return Some("ALLOC_BUF");
    }
    if op == "50" && args.first().map(|s| s.as_str()) == Some(slot_buf) && args.get(1).map(|s| s.as_str()) == Some("00")
    {
        return Some("LOAD_IN");
    }
    if op == "51"
        && args.first().map(|s| s.as_str()) == Some(slot_buf)
        && args.get(1).map(|s| s.as_str()) == Some("02")
        && args.get(2).map(|s| s.as_str()) == Some(slot_size)
    {
        return Some("WRITE_OUT");
    }
    if op == "A0" {
        return Some("EXIT_BYTE");
    }
    None
}

fn exit_bytes(contract: &Value) -> Vec<String> {
    contract["exit_trampoline_bytes"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default()
}

fn trailing_exit_ok(ops: &[Op], exit: &[String]) -> bool {
    if ops.len() < 1 + exit.len() {
        return false;
    }
    let start = ops.len() - exit.len();
    for (i, want) in exit.iter().enumerate() {
        let op = &ops[start + i];
        if op.op != "A0" || op.args.first().map(|s| s.as_str()) != Some(want.as_str()) {
            return false;
        }
    }
    true
}

fn arity_of(contract: &Value, op: &str) -> Option<usize> {
    contract["allowed_opcodes"]
        .get(op)?
        .get("arity")?
        .as_u64()
        .map(|n| n as usize)
}

fn check_slots(op: &str, args: &[String], contract: &Value, errors: &mut Vec<String>) {
    let slot_buf = contract["slot_buffer"].as_str().unwrap_or("50");
    let slot_size = contract["slot_size"].as_str().unwrap_or("51");
    let load_ok: Vec<&str> = contract["str_load_allowed"]
        .as_array()
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();
    let write_ok: Vec<&str> = contract["str_write_allowed"]
        .as_array()
        .map(|a| a.iter().filter_map(|v| v.as_str()).collect())
        .unwrap_or_default();
    if op == "20" && args.first().map(|s| s.as_str()) != Some(slot_buf) {
        errors.push("alloc_slot_must_be_50".into());
    }
    if op == "30" && args.first().map(|s| s.as_str()) != Some(slot_size) {
        errors.push("set_slot_must_be_51".into());
    }
    if op == "50" {
        if args.first().map(|s| s.as_str()) != Some(slot_buf) {
            errors.push("load_slot_must_be_50".into());
        }
        if args.get(1).map(|s| !load_ok.contains(&s.as_str())).unwrap_or(true) {
            errors.push("load_str_not_allowed".into());
        }
    }
    if op == "51" {
        if args.first().map(|s| s.as_str()) != Some(slot_buf) {
            errors.push("write_slot_must_be_50".into());
        }
        if args.get(1).map(|s| !write_ok.contains(&s.as_str())).unwrap_or(true) {
            errors.push("write_str_not_allowed".into());
        }
        if args.get(2).map(|s| s.as_str()) != Some(slot_size) {
            errors.push("write_sz_must_be_51".into());
        }
    }
}

fn semantic_exact(ops: &[Op], contract: &Value) -> (Vec<String>, Vec<Stmt>) {
    let mut errors = Vec::new();
    let mut stmts = Vec::new();
    let exit = exit_bytes(contract);
    for (i, o) in ops.iter().enumerate() {
        if o.op == "FF" {
            errors.push("forbid_ret_ff".into());
            continue;
        }
        if i > 0 && o.op == "40" {
            errors.push(format!("extra_handler_at_{i}"));
        }
        match arity_of(contract, &o.op) {
            None => {
                errors.push(format!("opcode_not_allowed_{}", o.op));
                continue;
            }
            Some(a) if o.args.len() != a => {
                errors.push(format!(
                    "arity_{}_want_{}_got_{}",
                    o.op,
                    a,
                    o.args.len()
                ));
                continue;
            }
            _ => {}
        }
        check_slots(&o.op, &o.args, contract, &mut errors);
        if i == 0 {
            continue;
        }
        if o.op == "A0" {
            stmts.push(Stmt {
                kind: "RAW".into(),
                text: format!("A0 {}", o.args.first().map(|s| s.as_str()).unwrap_or("")),
            });
        } else {
            match classify_stmt(&o.op, &o.args, contract) {
                Some(k) if k != "EXIT_BYTE" => stmts.push(Stmt {
                    kind: k.into(),
                    text: {
                        let mut p = vec![o.op.clone()];
                        p.extend(o.args.iter().cloned());
                        p.join(" ")
                    },
                }),
                _ => errors.push(format!("stmt_not_in_grammar_{}", o.op)),
            }
        }
    }
    if !trailing_exit_ok(ops, &exit) {
        errors.push("exit_trampoline_mismatch".into());
    }
    (errors, stmts)
}

fn semantic_file(ops: &[Op], contract: &Value) -> (Vec<String>, Vec<Stmt>, usize) {
    let mut errors = Vec::new();
    let mut stmts = Vec::new();
    let exit = exit_bytes(contract);
    let mut raw: Vec<String> = Vec::new();
    let mut in_raw = false;
    let mut has_size = false;
    let mut has_buf = false;
    let mut write_count = 0usize;
    for (i, o) in ops.iter().enumerate() {
        if o.op == "FF" {
            errors.push("forbid_ret_ff".into());
            continue;
        }
        if i > 0 && o.op == "40" {
            errors.push(format!("extra_handler_at_{i}"));
        }
        match arity_of(contract, &o.op) {
            None => {
                errors.push(format!("opcode_not_allowed_{}", o.op));
                continue;
            }
            Some(a) if o.args.len() != a => {
                errors.push(format!(
                    "arity_{}_want_{}_got_{}",
                    o.op,
                    a,
                    o.args.len()
                ));
                continue;
            }
            _ => {}
        }
        if o.op == "A0" {
            in_raw = true;
            if let Some(b) = o.args.first() {
                raw.push(b.clone());
            }
            continue;
        }
        if in_raw {
            errors.push("raw_must_be_tail_only".into());
        }
        check_slots(&o.op, &o.args, contract, &mut errors);
        match classify_stmt(&o.op, &o.args, contract) {
            None if i > 0 => {
                errors.push(format!("stmt_not_in_grammar_{}", o.op));
                continue;
            }
            Some(k) if k != "EXIT_BYTE" => {
                if k == "WRITE_OUT" {
                    if !has_size {
                        errors.push("dataflow_write_without_size".into());
                    }
                    if !has_buf {
                        errors.push("dataflow_write_without_buf".into());
                    }
                    write_count += 1;
                }
                if k == "SET_SIZE" {
                    has_size = true;
                }
                if k == "ALLOC_BUF" || k == "LOAD_IN" {
                    has_buf = true;
                }
                let mut p = vec![o.op.clone()];
                p.extend(o.args.iter().cloned());
                stmts.push(Stmt {
                    kind: k.into(),
                    text: p.join(" "),
                });
            }
            _ => {}
        }
    }
    if raw.is_empty() {
        errors.push("missing_exit_trampoline".into());
    } else if raw != exit {
        errors.push("exit_trampoline_mismatch".into());
    }
    if contract["grammar"]["require_write"].as_bool().unwrap_or(false) && write_count < 1 {
        errors.push("grammar_require_write".into());
    }
    (errors, stmts, write_count)
}

fn multi_scale_ast(ops: &[Op], stmts: &[Stmt], contract: &Value, kind: &str) -> Value {
    let indent = contract["body_indent"].as_str().unwrap_or("  ");
    let entry = contract["entry_line"].as_str().unwrap_or("40 00");
    let mut lines = vec![json!(entry)];
    for op in ops.iter().skip(1) {
        let mut p = vec![op.op.clone()];
        p.extend(op.args.iter().cloned());
        lines.push(json!(format!("{indent}{}", p.join(" "))));
    }
    let exit = exit_bytes(contract);
    let body_stmts: Vec<&Stmt> = if kind == "exact_body" {
        let el = exit.len();
        if stmts.len() >= el {
            stmts[..stmts.len() - el].iter().collect()
        } else {
            stmts.iter().collect()
        }
    } else {
        stmts.iter().collect()
    };
    let stmt_strs: Vec<Value> = body_stmts
        .iter()
        .map(|s| json!(format!("{}:{}", s.kind, s.text)))
        .collect();
    let body: Vec<Value> = body_stmts.iter().map(|s| json!(s.text)).collect();
    let exit_v: Vec<Value> = exit.iter().map(|e| json!(e)).collect();
    json!({
        "lines": lines,
        "stmts": stmt_strs,
        "program": {
            "kind": kind,
            "entry": "40 00",
            "body": body,
            "exit": exit_v,
        }
    })
}

fn match_shape(ops: &[Op], contract: &Value) -> Option<String> {
    let mut body = Vec::new();
    for op in ops.iter().skip(1) {
        if op.op == "A0" {
            break;
        }
        let mut p = vec![op.op.clone()];
        p.extend(op.args.iter().cloned());
        body.push(p.join(" "));
    }
    let shapes = contract["shapes"].as_object()?;
    for (name, seq) in shapes {
        let Some(arr) = seq.as_array() else { continue };
        let want: Vec<&str> = arr.iter().filter_map(|v| v.as_str()).collect();
        if body.len() == want.len() && body.iter().zip(want.iter()).all(|(a, b)| a == b) {
            return Some(name.clone());
        }
    }
    None
}

fn program_kind(name: &str, contract: &Value) -> String {
    contract["program_kinds"]
        .get(name)
        .and_then(|v| v.as_str())
        .unwrap_or("file_grammar")
        .to_string()
}

fn validate_text(text: &str, contract: &Value, kind: &str) -> ValidateOut {
    let (mut errors, lines) = lex_check(text);
    let (ops, parse_errs) = parse_strict(&lines, contract);
    errors.extend(parse_errs);
    let (sem_errs, stmts, _) = if ops.is_empty() {
        (Vec::new(), Vec::new(), 0)
    } else if kind == "exact_body" {
        let (e, s) = semantic_exact(&ops, contract);
        (e, s, 0)
    } else {
        semantic_file(&ops, contract)
    };
    errors.extend(sem_errs);
    let mut canon = None;
    let mut ast = None;
    let mut shape = None;
    let stmt_count = stmts.len();
    if errors.is_empty() {
        let c = canonical(&ops, contract);
        if text != c {
            errors.push("canonical_mismatch".into());
        } else {
            canon = Some(c);
            if kind == "file_grammar" {
                shape = match_shape(&ops, contract);
            }
            ast = Some(multi_scale_ast(&ops, &stmts, contract, kind));
        }
    }
    ValidateOut {
        errors,
        canon,
        ast,
        shape,
        stmt_count,
    }
}

fn build_program(contract: &Value, stmts: &[String]) -> String {
    let mut ops = vec![Op {
        op: "40".into(),
        args: vec!["00".into()],
    }];
    for s in stmts {
        let parts: Vec<&str> = s.split(' ').collect();
        ops.push(Op {
            op: parts[0].to_string(),
            args: parts[1..].iter().map(|x| (*x).to_string()).collect(),
        });
    }
    for b in exit_bytes(contract) {
        ops.push(Op {
            op: "A0".into(),
            args: vec![b],
        });
    }
    canonical(&ops, contract)
}

fn generative(contract: &Value) -> Result<(), String> {
    let trials = contract["generative"]["trials"].as_u64().unwrap_or(32) as usize;
    let imms: Vec<String> = contract["generative"]["imm_choices"]
        .as_array()
        .map(|a| {
            a.iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_else(|| vec!["01".into(), "03".into(), "05".into(), "08".into()]);
    let max_extra = contract["generative"]["max_extra_sets"].as_u64().unwrap_or(2) as usize;
    let mut ok = 0usize;
    for t in 0..trials {
        let mode = t % 4;
        let mut stmts = Vec::new();
        let extra = t % (max_extra + 1);
        for e in 0..extra {
            stmts.push(format!("30 51 {}", imms[(t + e) % imms.len()]));
        }
        match mode {
            0 => {
                stmts.push(format!("30 51 {}", imms[t % imms.len()]));
                stmts.push("50 50 00".into());
                stmts.push("51 50 02 51".into());
            }
            1 => {
                stmts.push(format!("20 50 {}", imms[(t + 1) % imms.len()]));
                stmts.push(format!("30 51 {}", imms[t % imms.len()]));
                stmts.push("51 50 02 51".into());
            }
            2 => {
                stmts.push(format!("30 51 {}", imms[0]));
                stmts.push(format!("30 51 {}", imms[t % imms.len()]));
                stmts.push("50 50 00".into());
                stmts.push("51 50 02 51".into());
            }
            _ => {
                stmts.push(format!("20 50 {}", imms[t % imms.len()]));
                stmts.push(format!("30 51 {}", imms[(t + 2) % imms.len()]));
                stmts.push("50 50 00".into());
                stmts.push("51 50 02 51".into());
            }
        }
        let text = build_program(contract, &stmts);
        let r = validate_text(&text, contract, "file_grammar");
        if !r.errors.is_empty() {
            return Err(format!("gen={t} {}", r.errors.join("|")));
        }
        if r.canon.as_deref() != Some(text.as_str()) {
            return Err(format!("gen={t} gen_canon_mismatch"));
        }
        let r2 = validate_text(r.canon.as_ref().unwrap(), contract, "file_grammar");
        if !r2.errors.is_empty() || r2.canon != r.canon {
            return Err(format!("gen={t} gen_roundtrip"));
        }
        ok += 1;
    }
    println!("LLMSCALE generative trials={ok} status=GREEN");
    Ok(())
}

fn sha256_file(p: &Path) -> Result<String, IsaError> {
    let data = fs::read(p).map_err(|e| IsaError::IoError { msg: e.to_string() })?;
    Ok(ddc::sha256_hex(&data))
}

/// Run full LLMWRITE contract validation (AST + generative + schema). Prints status lines.
pub fn run(repo_root: &Path) -> Result<(), IsaError> {
    let anyprog = repo_root.join("scripts/fixtures/anyprog");
    let contract_path = anyprog.join("model-contract.json");
    let pin_path = anyprog.join("model-contract.sha256");
    let contract_raw = fs::read_to_string(&contract_path).map_err(|e| IsaError::IoError {
        msg: e.to_string(),
    })?;
    let contract: Value = serde_json::from_str(&contract_raw).map_err(|e| IsaError::IoError {
        msg: format!("model-contract.json: {e}"),
    })?;

    if contract["revision"].as_u64().unwrap_or(0) < 5 {
        println!("LLMSCALE status=RED reason=contract_not_v5");
        return Err(IsaError::IoError {
            msg: "contract_not_v5".into(),
        });
    }
    if contract.get("artifact_pins").is_none()
        || contract.get("drift").is_none()
        || contract.get("section_pins").is_none()
        || contract.get("program_kinds").is_none()
    {
        println!("LLMSCALE status=RED reason=pipe_schema_missing");
        return Err(IsaError::IoError {
            msg: "pipe_schema_missing".into(),
        });
    }
    if !anyprog.join("MODEL-CONTRACT.md").is_file() {
        println!("LLMSCALE status=RED reason=MODEL-CONTRACT_missing");
        return Err(IsaError::IoError {
            msg: "MODEL-CONTRACT_missing".into(),
        });
    }
    if !pin_path.is_file() {
        println!("LLMSCALE status=RED reason=contract_pin_missing");
        return Err(IsaError::IoError {
            msg: "contract_pin_missing".into(),
        });
    }
    let got_pin = sha256_file(&contract_path)?;
    let want_pin = fs::read_to_string(&pin_path)
        .map_err(|e| IsaError::IoError { msg: e.to_string() })?
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_lowercase();
    if got_pin != want_pin {
        println!("LLMSCALE status=RED reason=contract_pin_DIFF want={want_pin} got={got_pin}");
        return Err(IsaError::IoError {
            msg: "contract_pin_DIFF".into(),
        });
    }
    println!(
        "LLMSCALE contract_pin=EQUAL sha256_prefix={}",
        &got_pin[..16.min(got_pin.len())]
    );

    let required = [
        "cat",
        "head",
        "zero",
        "copy08",
        "resize",
        "hello_console",
        "cmdline",
        "tick",
        "thread",
        "socket",
    ];
    let cov = &contract["surface_coverage"];
    for t in &required {
        if cov.get(t).is_none() {
            println!("LLMSCALE status=RED reason=surface_coverage_missing tool={t}");
            return Err(IsaError::IoError {
                msg: format!("surface_coverage_missing {t}"),
            });
        }
    }
    let mut in_names: Vec<String> = cov
        .as_object()
        .map(|m| {
            m.iter()
                .filter(|(_, v)| v.as_str() == Some("in_contract"))
                .map(|(k, _)| k.clone())
                .collect()
        })
        .unwrap_or_default();
    in_names.sort();
    println!(
        "LLMSCALE surface_in={} classified={}",
        in_names.join(","),
        required.len()
    );

    for n in &in_names {
        let pin = contract["artifact_pins"][n].as_str().unwrap_or("");
        if pin.len() != 64 || !pin.chars().all(|c| c.is_ascii_hexdigit()) {
            println!("LLMSCALE status=RED reason=artifact_pin_bad sample={n}");
            return Err(IsaError::IoError {
                msg: format!("artifact_pin_bad {n}"),
            });
        }
        let text_pin = contract["section_pins"][n]["text_sha256"]
            .as_str()
            .unwrap_or("");
        if text_pin.len() != 64 {
            println!("LLMSCALE status=RED reason=section_pin_bad sample={n}");
            return Err(IsaError::IoError {
                msg: format!("section_pin_bad {n}"),
            });
        }
        if contract["program_kinds"].get(n).is_none() {
            println!("LLMSCALE status=RED reason=program_kind_missing sample={n}");
            return Err(IsaError::IoError {
                msg: format!("program_kind_missing {n}"),
            });
        }
    }
    if contract["drift"]["rebuild_times"].as_u64().unwrap_or(0) < 2 {
        println!("LLMSCALE status=RED reason=drift_rebuild_times");
        return Err(IsaError::IoError {
            msg: "drift_rebuild_times".into(),
        });
    }
    if contract["dual_emit"]["require_agree"].as_bool() != Some(true) {
        println!("LLMSCALE status=RED reason=dual_emit_missing");
        return Err(IsaError::IoError {
            msg: "dual_emit_missing".into(),
        });
    }

    let llm_rel = contract["samples_llm_dir"]
        .as_str()
        .unwrap_or("scripts/fixtures/anyprog/llm");
    let llm_dir = repo_root.join(llm_rel.replace('/', std::path::MAIN_SEPARATOR_STR));
    let mut goods = Vec::new();
    let mut poisons = Vec::new();
    for ent in fs::read_dir(&llm_dir).map_err(|e| IsaError::IoError { msg: e.to_string() })? {
        let ent = ent.map_err(|e| IsaError::IoError { msg: e.to_string() })?;
        let name = ent.file_name().to_string_lossy().to_string();
        if !name.ends_with(".ty") {
            continue;
        }
        if name.starts_with("_poison") {
            poisons.push(name);
        } else {
            goods.push(name);
        }
    }
    goods.sort();
    poisons.sort();
    if goods.len() < 9 {
        println!(
            "LLMSCALE status=RED reason=too_few_samples n={} want>=9",
            goods.len()
        );
        return Err(IsaError::IoError {
            msg: "too_few_samples".into(),
        });
    }

    let mut ok = true;
    for f in &goods {
        let name = f.trim_end_matches(".ty");
        let kind = program_kind(name, &contract);
        let path = llm_dir.join(f);
        let text = fs::read_to_string(&path).map_err(|e| IsaError::IoError {
            msg: e.to_string(),
        })?;
        let r = validate_text(&text, &contract, &kind);
        if !r.errors.is_empty() {
            println!(
                "LLMSCALE sample={name} status=RED reason={}",
                r.errors.join("|")
            );
            ok = false;
            continue;
        }
        let ast_path = llm_dir.join(format!("{name}.ast.json"));
        if !ast_path.is_file() {
            println!("LLMSCALE sample={name} status=RED reason=ast_missing");
            ok = false;
            continue;
        }
        let want_raw = fs::read_to_string(&ast_path).map_err(|e| IsaError::IoError {
            msg: e.to_string(),
        })?;
        let want: Value = serde_json::from_str(&want_raw).map_err(|e| IsaError::IoError {
            msg: format!("ast json {name}: {e}"),
        })?;
        let got = r.ast.as_ref().unwrap();
        if got != &want {
            println!("LLMSCALE sample={name} status=RED reason=ast_mismatch");
            ok = false;
            continue;
        }
        let again = validate_text(r.canon.as_ref().unwrap(), &contract, &kind);
        if !again.errors.is_empty() || again.canon != r.canon {
            println!("LLMSCALE sample={name} status=RED reason=roundtrip_fail");
            ok = false;
            continue;
        }
        println!(
            "LLMSCALE sample={name} kind={kind} stmts={} shape_alias={} status=GREEN",
            r.stmt_count,
            r.shape.as_deref().unwrap_or("-")
        );
    }

    let mut reasons: HashSet<String> = HashSet::new();
    for f in &poisons {
        let path = llm_dir.join(f);
        let text = fs::read_to_string(&path).map_err(|e| IsaError::IoError {
            msg: e.to_string(),
        })?;
        let r = validate_text(&text, &contract, "file_grammar");
        if r.errors.is_empty() {
            println!("LLMSCALE poison={f} status=RED reason=expected_fail_got_pass");
            ok = false;
            continue;
        }
        reasons.insert(r.errors[0].clone());
        println!(
            "LLMSCALE poison={f} status=RED_as_expected errs={}",
            r.errors[0]
        );
    }
    let min_p = contract["min_poison_distinct_reasons"]
        .as_u64()
        .unwrap_or(10) as usize;
    if reasons.len() < min_p {
        println!(
            "LLMSCALE status=RED reason=poison_diversity got={} want>={}",
            reasons.len(),
            min_p
        );
        return Err(IsaError::IoError {
            msg: "poison_diversity".into(),
        });
    }
    if let Err(e) = generative(&contract) {
        println!("LLMSCALE status=RED reason={e}");
        ok = false;
    }
    if !reasons.contains("dataflow_write_without_size")
        && !llm_dir.join("_poison_df_size.ty").is_file()
    {
        println!("LLMSCALE status=RED reason=missing_dataflow_poison");
        return Err(IsaError::IoError {
            msg: "missing_dataflow_poison".into(),
        });
    }

    if !ok {
        println!("LLMSCALE status=RED");
        return Err(IsaError::IoError {
            msg: "LLMSCALE RED".into(),
        });
    }
    println!(
        "LLMSCALE samples={} poisons={} distinct_reasons={}",
        goods.len(),
        poisons.len(),
        reasons.len()
    );
    println!("LLMSCALE status=GREEN");
    println!("LLMSCALE doc=AUTO-LLMWRITESCALE.md");
    println!("LLMPIPE schema=program_kinds+section_pins+dual_emit status=STRUCT_GREEN");
    println!("LLMWRITEYOYO authority=yoyo_llmwrite;exact_body_grammar=here;node_ABSENT");
    println!("LLMWRITEYOYO doc=AUTO-LLMWRITEYOYO.md");
    Ok(())
}
