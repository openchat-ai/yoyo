#!/usr/bin/env node
/**
 * scripts/check-plans.mjs — D-plan gate (Part Deduce / Part N.3).
 *
 * Fail-closed validation of yoyo/plans/*.plan.md:
 * - required sections (Deduce.2)
 * - allowed step kinds only (Deduce.3)
 * - registered EMIT types only (Deduce.4)
 * - conclusions.mutation_class + FORBIDDEN rules (Deduce.8)
 * - ReplayRecord + real sha256 when claim_level=pinned (Deduce.5 / Deduce.8)
 * - stub / placeholder hashes MUST NOT green
 */
import { createHash } from "node:crypto";
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const plansDir = path.join(root, "yoyo", "plans");
const foundationsDir = path.join(root, "yoyo", "foundations");

const ALLOWED_KINDS = new Set([
  "READ_FACT",
  "READ_PROBE",
  "COMPARE",
  "SELECT",
  "EMIT",
]);

const EMIT_TYPES = new Set([
  "target_posture",
  "posture_weight",
  "mutation_class",
  "morph_id",
  "switch_path",
  "abort_reason",
  "journal_line",
]);

const MUTATION_CLASSES = new Set([
  "SAFE",
  "AUDITED",
  "DANGEROUS",
  "FORBIDDEN",
]);

const CLAIM_LEVELS = new Set(["pinned", "planned", "rejected"]);

const THEORY_PINS = new Set([
  "none",
  "posture-score-v0",
  "equal-scope-v0",
  "abort-thermal-v0",
]);

const STUB_HASH_RE =
  /^(<sha256>|sha256|TODO|TBD|stub|STUB|0{64}|f{64})$/i;

function sha256Hex(buf) {
  return createHash("sha256").update(buf).digest("hex");
}

function fail(msg) {
  console.error(`[FAIL] check-plans.mjs: ${msg}`);
  process.exit(1);
}

function stripComments(text) {
  return text
    .split(/\r?\n/)
    .map((line) => {
      if (/^\s*#/.test(line)) return "";
      // keep inline values; only drop full-line comments
      return line;
    })
    .join("\n");
}

/** Minimal YAML-ish parse for Deduce plan files (maps + numbered steps + lists). */
function parsePlan(text) {
  const body = stripComments(text);
  const lines = body.split(/\r?\n/);
  const root = {};
  const stack = [{ indent: -1, obj: root, key: null, kind: "map" }];

  function current() {
    return stack[stack.length - 1];
  }

  function ensureChildMap(parent, key) {
    if (!parent[key] || typeof parent[key] !== "object" || Array.isArray(parent[key])) {
      parent[key] = {};
    }
    return parent[key];
  }

  for (const raw of lines) {
    if (!raw.trim()) continue;
    const indent = raw.match(/^ */)[0].length;
    const line = raw.trim();

    while (stack.length > 1 && indent <= current().indent) stack.pop();
    const ctx = current();

    // numbered step: "1. KIND rest"
    const stepM = line.match(/^(\d+)\.\s+(\S+)(?:\s+(.*))?$/);
    if (stepM && ctx.key === "steps") {
      if (!Array.isArray(ctx.obj.steps)) ctx.obj.steps = [];
      ctx.obj.steps.push({
        n: Number(stepM[1]),
        kind: stepM[2],
        rest: (stepM[3] || "").trim(),
      });
      continue;
    }

    // list item
    if (line.startsWith("- ")) {
      const item = line.slice(2).trim();
      if (ctx.kind === "list") {
        ctx.list.push(parseScalar(item));
      } else if (ctx.key) {
        if (!Array.isArray(ctx.obj[ctx.key])) ctx.obj[ctx.key] = [];
        ctx.obj[ctx.key].push(parseScalar(item));
        stack.push({
          indent,
          obj: ctx.obj,
          key: ctx.key,
          kind: "list",
          list: ctx.obj[ctx.key],
        });
      }
      continue;
    }

    const kv = line.match(/^([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.*)$/);
    if (!kv) continue;
    const key = kv[1];
    const val = kv[2].trim();

    if (val === "") {
      const child = ensureChildMap(ctx.obj, key);
      // steps / commands / abort_if may become arrays
      if (key === "steps") {
        ctx.obj.steps = [];
        stack.push({ indent, obj: ctx.obj, key: "steps", kind: "steps" });
      } else if (key === "commands" || key === "abort_if" || key === "foundation_ids") {
        ctx.obj[key] = [];
        stack.push({
          indent,
          obj: ctx.obj,
          key,
          kind: "list",
          list: ctx.obj[key],
        });
      } else {
        stack.push({ indent, obj: child, key: null, kind: "map" });
      }
      continue;
    }

    // inline list [a, b]
    if (val.startsWith("[") && val.endsWith("]")) {
      ctx.obj[key] = val
        .slice(1, -1)
        .split(",")
        .map((s) => s.trim())
        .filter(Boolean)
        .map(parseScalar);
      continue;
    }

    ctx.obj[key] = parseScalar(val);
  }

  return root;
}

function parseScalar(s) {
  if (s === "true") return true;
  if (s === "false") return false;
  if (/^-?\d+(\.\d+)?$/.test(s)) return Number(s);
  return s;
}

function canonicalEmitPayload(conclusions) {
  if (!conclusions || typeof conclusions !== "object") return "";
  const keys = Object.keys(conclusions)
    .filter((k) => EMIT_TYPES.has(k))
    .sort();
  return keys.map((k) => `${k}=${String(conclusions[k])}`).join("\n") + (keys.length ? "\n" : "");
}

function listPlanFiles() {
  if (!fs.existsSync(plansDir)) return [];
  return fs
    .readdirSync(plansDir)
    .filter((n) => n.endsWith(".plan.md") || n.endsWith(".plan.yaml"))
    .map((n) => path.join(plansDir, n))
    .sort();
}

function foundationExists(id) {
  // accept id or id.fdn under yoyo/foundations/
  const candidates = [
    path.join(foundationsDir, `${id}.fdn`),
    path.join(foundationsDir, id),
  ];
  return candidates.some((p) => fs.existsSync(p));
}

function validatePlan(filePath) {
  const errors = [];
  const bytes = fs.readFileSync(filePath);
  const text = bytes.toString("utf8");
  const plan = parsePlan(text);
  const base = path.basename(filePath);

  // Deduce.1: in-tree sticks to one extension — warn as error if yaml appears alongside md policy
  if (filePath.endsWith(".plan.yaml")) {
    errors.push(`${base}: in-tree Deduce plans use .plan.md only (Deduce.1)`);
  }

  // Deduce.2 required sections
  for (const sec of [
    "claim_level",
    "inputs",
    "steps",
    "conclusions",
    "commands",
    "abort_if",
  ]) {
    if (plan[sec] === undefined || plan[sec] === null) {
      errors.push(`${base}: missing required section '${sec}' (Deduce.2)`);
    }
  }

  const claim = plan.claim_level;
  if (claim !== undefined && !CLAIM_LEVELS.has(String(claim))) {
    errors.push(
      `${base}: claim_level must be pinned|planned|rejected, got ${JSON.stringify(claim)}`
    );
  }

  const inputs = plan.inputs || {};
  if (!inputs.foundation_ids || !Array.isArray(inputs.foundation_ids) || inputs.foundation_ids.length === 0) {
    errors.push(`${base}: inputs.foundation_ids must be a non-empty list`);
  } else {
    for (const id of inputs.foundation_ids) {
      if (!foundationExists(String(id))) {
        errors.push(`${base}: foundation_ids pin missing on disk: ${id}`);
      }
    }
  }
  if (inputs.probe_id === undefined && inputs.probe_snapshot_id === undefined) {
    errors.push(`${base}: inputs must include probe_id (or probe snapshot id)`);
  }
  if (inputs.posture_id === undefined) {
    errors.push(`${base}: inputs.posture_id required`);
  }
  if (inputs.theory_pin !== undefined && !THEORY_PINS.has(String(inputs.theory_pin))) {
    errors.push(
      `${base}: theory_pin must be closed enum or omitted, got ${JSON.stringify(inputs.theory_pin)}`
    );
  }

  const steps = plan.steps;
  if (!Array.isArray(steps) || steps.length === 0) {
    errors.push(`${base}: steps must be a non-empty numbered list`);
  } else {
    for (let i = 0; i < steps.length; i++) {
      const s = steps[i];
      if (s.n !== i + 1) {
        errors.push(`${base}: steps must be numbered 1..N contiguously (saw ${s.n} at index ${i + 1})`);
      }
      if (!ALLOWED_KINDS.has(s.kind)) {
        errors.push(
          `${base}: forbidden or unknown step kind '${s.kind}' (Deduce.3 allows ${[...ALLOWED_KINDS].join("|")})`
        );
      }
      if (s.kind === "EMIT") {
        const em = s.rest.match(/^([A-Za-z_][A-Za-z0-9_]*)\s*=/);
        if (!em) {
          errors.push(`${base}: EMIT step ${s.n} must be 'EMIT <type>=...'`);
        } else if (!EMIT_TYPES.has(em[1])) {
          errors.push(
            `${base}: EMIT type '${em[1]}' not in Deduce.4 registered set`
          );
        }
      }
    }
  }

  const conclusions = plan.conclusions || {};
  const mc = conclusions.mutation_class;
  if (mc === undefined) {
    errors.push(`${base}: conclusions.mutation_class required`);
  } else if (!MUTATION_CLASSES.has(String(mc))) {
    errors.push(
      `${base}: conclusions.mutation_class must be SAFE|AUDITED|DANGEROUS|FORBIDDEN`
    );
  }

  const commands = plan.commands;
  if (!Array.isArray(commands)) {
    errors.push(`${base}: commands must be a list (may be empty when FORBIDDEN)`);
  }

  if (String(mc) === "FORBIDDEN") {
    if (Array.isArray(commands) && commands.length > 0) {
      errors.push(`${base}: FORBIDDEN ⇒ commands must be empty (Deduce.8)`);
    }
    const hasAbort =
      (conclusions.abort_reason !== undefined && conclusions.abort_reason !== "") ||
      (Array.isArray(steps) &&
        steps.some(
          (s) => s.kind === "EMIT" && /^abort_reason\s*=/.test(s.rest)
        ));
    if (!hasAbort) {
      errors.push(`${base}: FORBIDDEN ⇒ abort_reason must be set (Deduce.8)`);
    }
  }

  // Soft EQUAL guard: bare EQUAL claim in conclusions without re-prove token
  const conclText = JSON.stringify(conclusions);
  if (/\bEQUAL\b/.test(conclText) && !/\bre-?prove\b/i.test(text)) {
    errors.push(
      `${base}: conclusions mention EQUAL without re-prove marker (Deduce.8 / N.4)`
    );
  }

  // ReplayRecord when pinned
  if (String(claim) === "pinned") {
    const replay = plan.replay;
    if (!replay || typeof replay !== "object") {
      errors.push(
        `${base}: claim_level=pinned requires ReplayRecord (replay: block) before Relock`
      );
    } else {
      const planHash = String(replay.plan_hash || "");
      const emitHash = String(replay.emit_payload_hash || "");
      if (!planHash || STUB_HASH_RE.test(planHash) || !/^[0-9a-f]{64}$/i.test(planHash)) {
        errors.push(`${base}: replay.plan_hash must be real sha256 hex (stub hasher ⇒ D-plan red)`);
      } else {
        const actual = sha256Hex(bytes);
        if (planHash.toLowerCase() !== actual) {
          errors.push(
            `${base}: replay.plan_hash mismatch (file=${actual}, record=${planHash.toLowerCase()})`
          );
        }
      }
      if (!emitHash || STUB_HASH_RE.test(emitHash) || !/^[0-9a-f]{64}$/i.test(emitHash)) {
        errors.push(
          `${base}: replay.emit_payload_hash must be real sha256 hex (stub ⇒ red)`
        );
      } else {
        const canonical = canonicalEmitPayload(conclusions);
        const actualEmit = sha256Hex(Buffer.from(canonical, "utf8"));
        if (emitHash.toLowerCase() !== actualEmit) {
          errors.push(
            `${base}: replay.emit_payload_hash mismatch (canonical=${actualEmit}, record=${emitHash.toLowerCase()})`
          );
        }
      }
      const fids = replay.foundation_ids;
      if (!Array.isArray(fids) || fids.length === 0) {
        errors.push(`${base}: replay.foundation_ids required when pinned`);
      }
      if (replay.claim_level !== undefined && String(replay.claim_level) !== "pinned") {
        errors.push(`${base}: replay.claim_level must be pinned when plan is pinned`);
      }
    }
  }

  // Reject stub theater in any present replay block
  if (plan.replay && typeof plan.replay === "object") {
    for (const field of ["plan_hash", "emit_payload_hash"]) {
      const v = plan.replay[field];
      if (v !== undefined && STUB_HASH_RE.test(String(v))) {
        errors.push(`${base}: replay.${field} is a stub placeholder — MUST NOT green D-plan`);
      }
    }
  }

  return errors;
}

// --- main ---
const files = listPlanFiles();
if (files.length === 0) {
  fail(
    "no yoyo/plans/*.plan.md (or .plan.yaml) — empty plan set MUST NOT green D-plan.\n" +
      "  Spec: PROMPT-v3.md Part Deduce / Part N.3 gate D-plan."
  );
}

// Deduce.1: one extension in-tree
const exts = new Set(files.map((f) => (f.endsWith(".plan.yaml") ? ".plan.yaml" : ".plan.md")));
if (exts.size > 1) {
  fail("mixed .plan.md and .plan.yaml in yoyo/plans/ — Deduce.1 requires one extension");
}

const allErrors = [];
for (const f of files) {
  allErrors.push(...validatePlan(f));
}

if (allErrors.length > 0) {
  console.error(
    `[FAIL] check-plans.mjs: ${allErrors.length} error(s):\n  - ` +
      allErrors.join("\n  - ")
  );
  process.exit(1);
}

console.log(
  `[ok] check-plans.mjs: ${files.length} plan(s) under yoyo/plans/; ` +
    `kinds ⊆ Deduce.3; EMIT ⊆ Deduce.4; ReplayRecord rules enforced (pinned).`
);
process.exit(0);
