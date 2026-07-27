#!/usr/bin/env node
/**
 * scripts/check-foundations.mjs — F-foundations gate (Part F / N.3).
 *
 * Validates yoyo/foundations/*.fdn:
 *   - required fields (F.2)
 *   - filename matches id
 *   - k_refs ⊆ E.2 (K1..K7)
 *   - every maps_to resolves (CapabilityReport field / known CLI / existing path)
 *   - morph journal line foundation_pin=<id>
 *   - some *.morph (or plan) lists each pin in foundation_ids
 *
 * Fail-closed: missing pins, empty maps_to, unknown k_ref / field ⇒ exit ≠ 0.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");

const FD_DIR = path.join(root, "yoyo", "foundations");
const MORPH_DIR = path.join(root, "yoyo", "morph");
const JOURNAL_PATH = path.join(MORPH_DIR, "journal.log");
const PLANS_DIR = path.join(root, "yoyo", "plans");
const CAP_REPORT = path.join(root, "yoyo", "schemas", "CapabilityReport.md");

const E2_K_REFS = new Set(["K1", "K2", "K3", "K4", "K5", "K6", "K7"]);
const THEORY_PINS = new Set([
  "none",
  "posture-score-v0",
  "equal-scope-v0",
  "abort-thermal-v0",
]);
const POSTURE_HOOK_FIELDS = new Set([
  "debounce_ms",
  "L_enter",
  "L_exit",
  "min_dwell_ms",
  "transition_slo_ms",
  "score_formula_ref",
  "hot_switch_axes",
  "cold_switch_axes",
  "posture_id",
  "posture_weight",
]);
const KNOWN_CLI = new Set(["--posture=", "--morph="]);

const errors = [];

function fail(msg) {
  errors.push(msg);
}

function loadCapabilityFields(mdText) {
  const fields = new Set();
  for (const line of mdText.split(/\r?\n/)) {
    const m = line.match(/^\|\s*`([a-z][a-z0-9_]*)`\s*\|/);
    if (m) fields.add(m[1]);
  }
  return fields;
}

/** Minimal YAML-ish parse for Part F .fdn shape (no full YAML dependency). */
function parseFdn(text, fileLabel) {
  const lines = text.split(/\r?\n/);
  const doc = {
    id: null,
    k_refs: null,
    facts: [],
    posture_hooks: [],
    theory_pin: "none",
    forbidden_claims: [],
    _saw: {
      id: false,
      k_refs: false,
      facts: false,
      posture_hooks: false,
      theory_pin: false,
      forbidden_claims: false,
    },
  };

  let section = null; // facts | posture_hooks | forbidden_claims
  let current = null;

  function flushCurrent() {
    if (!current) return;
    if (section === "facts") doc.facts.push(current);
    else if (section === "posture_hooks") doc.posture_hooks.push(current);
    current = null;
  }

  for (let i = 0; i < lines.length; i++) {
    const raw = lines[i];
    const line = raw.replace(/\s+#.*$/, "").trimEnd();
    if (!line.trim()) continue;

    const top = line.match(
      /^(id|k_refs|facts|posture_hooks|theory_pin|forbidden_claims)\s*:\s*(.*)$/
    );
    if (top) {
      flushCurrent();
      const key = top[1];
      const rest = top[2].trim();
      doc._saw[key] = true;
      section = null;
      current = null;

      if (key === "id") {
        doc.id = rest || null;
      } else if (key === "k_refs") {
        doc.k_refs = parseList(rest);
      } else if (key === "theory_pin") {
        doc.theory_pin = rest || "none";
      } else if (key === "facts" || key === "posture_hooks") {
        section = key;
        if (rest && rest !== "[]") {
          fail(`${fileLabel}: unexpected inline ${key} value`);
        }
      } else if (key === "forbidden_claims") {
        section = "forbidden_claims";
        if (rest === "[]") {
          doc.forbidden_claims = [];
          section = null;
        } else if (rest.startsWith("[")) {
          doc.forbidden_claims = parseList(rest);
          section = null;
        }
      }
      continue;
    }

    if (section === "facts" || section === "posture_hooks") {
      const itemStart = line.match(/^\s*-\s+(\w+)\s*:\s*(.*)$/);
      if (itemStart) {
        flushCurrent();
        current = {};
        current[itemStart[1]] = unquote(itemStart[2].trim());
        continue;
      }
      const nested = line.match(/^\s+(\w+)\s*:\s*(.*)$/);
      if (nested && current) {
        current[nested[1]] = unquote(nested[2].trim());
        continue;
      }
      fail(`${fileLabel}:${i + 1}: unexpected line under ${section}: ${line.trim()}`);
      continue;
    }

    if (section === "forbidden_claims") {
      const item = line.match(/^\s*-\s+(.*)$/);
      if (item) {
        doc.forbidden_claims.push(unquote(item[1].trim()));
        continue;
      }
      fail(`${fileLabel}:${i + 1}: unexpected forbidden_claims line`);
      continue;
    }

    fail(`${fileLabel}:${i + 1}: unexpected top-level line: ${line.trim()}`);
  }
  flushCurrent();
  return doc;
}

function unquote(s) {
  if (
    (s.startsWith('"') && s.endsWith('"')) ||
    (s.startsWith("'") && s.endsWith("'"))
  ) {
    return s.slice(1, -1);
  }
  return s;
}

function parseList(s) {
  const t = s.trim();
  if (!t || t === "[]") return [];
  const inner = t.startsWith("[") && t.endsWith("]") ? t.slice(1, -1) : t;
  if (!inner.trim()) return [];
  return inner.split(",").map((p) => unquote(p.trim())).filter(Boolean);
}

function resolveMapsTo(mapsTo, capFields, fileLabel, factKey) {
  if (mapsTo == null || String(mapsTo).trim() === "") {
    fail(`${fileLabel}: fact "${factKey}" has empty maps_to`);
    return;
  }
  const m = String(mapsTo).trim();

  if (KNOWN_CLI.has(m)) return;

  const cap = m.match(/^CapabilityReport\.([a-z][a-z0-9_]*)$/);
  if (cap) {
    if (!capFields.has(cap[1])) {
      fail(
        `${fileLabel}: maps_to ${m} — unknown CapabilityReport field (see yoyo/schemas/CapabilityReport.md)`
      );
    }
    return;
  }

  // Repo-relative path must exist
  const asPath = path.join(root, m.replace(/^\.\//, ""));
  if (fs.existsSync(asPath)) return;

  // Posture / morph pin field names (spec surface)
  if (POSTURE_HOOK_FIELDS.has(m)) return;

  fail(
    `${fileLabel}: maps_to "${m}" does not resolve (not CapabilityReport.*, known CLI, posture field, or existing path)`
  );
}

function collectFoundationIdsFromText(text) {
  const ids = new Set();
  // foundation_ids: [a, b] or multiline list items
  const inline = text.match(/foundation_ids\s*:\s*\[([^\]]*)\]/g);
  if (inline) {
    for (const block of inline) {
      const inner = block.match(/\[([^\]]*)\]/);
      if (!inner) continue;
      for (const part of inner[1].split(",")) {
        const id = unquote(part.trim());
        if (id) ids.add(id);
      }
    }
  }
  const lines = text.split(/\r?\n/);
  let inList = false;
  for (const line of lines) {
    if (/^\s*foundation_ids\s*:\s*$/.test(line)) {
      inList = true;
      continue;
    }
    if (inList) {
      const item = line.match(/^\s*-\s+(\S+)\s*$/);
      if (item) {
        ids.add(unquote(item[1]));
        continue;
      }
      if (/^\S/.test(line) || /^\s+\w+\s*:/.test(line)) inList = false;
    }
  }
  return ids;
}

function walkFiles(dir, pred, out = []) {
  if (!fs.existsSync(dir)) return out;
  for (const ent of fs.readdirSync(dir, { withFileTypes: true })) {
    const p = path.join(dir, ent.name);
    if (ent.isDirectory()) walkFiles(p, pred, out);
    else if (pred(ent.name, p)) out.push(p);
  }
  return out;
}

// --- main ---

if (!fs.existsSync(FD_DIR)) {
  console.error(
    `[FAIL] check-foundations.mjs: missing ${path.relative(root, FD_DIR)}`
  );
  process.exit(2);
}

const fdnFiles = fs
  .readdirSync(FD_DIR)
  .filter((n) => n.endsWith(".fdn"))
  .map((n) => path.join(FD_DIR, n))
  .sort();

if (fdnFiles.length === 0) {
  console.error(
    "[FAIL] check-foundations.mjs: no yoyo/foundations/*.fdn pins — F-foundations stays RED."
  );
  process.exit(2);
}

if (!fs.existsSync(CAP_REPORT)) {
  console.error(
    "[FAIL] check-foundations.mjs: missing yoyo/schemas/CapabilityReport.md (needed for maps_to)."
  );
  process.exit(2);
}

const capFields = loadCapabilityFields(fs.readFileSync(CAP_REPORT, "utf8"));
if (capFields.size === 0) {
  console.error(
    "[FAIL] check-foundations.mjs: CapabilityReport.md has zero field rows."
  );
  process.exit(2);
}

const journalText = fs.existsSync(JOURNAL_PATH)
  ? fs.readFileSync(JOURNAL_PATH, "utf8")
  : "";

const refFiles = [
  ...walkFiles(MORPH_DIR, (n) => /\.morph(\.md)?$/i.test(n) || n.endsWith(".md")),
  ...walkFiles(PLANS_DIR, (n) => /\.plan\.(md|yaml|yml)$/i.test(n)),
];
const listedIds = new Set();
for (const f of refFiles) {
  const t = fs.readFileSync(f, "utf8");
  for (const id of collectFoundationIdsFromText(t)) listedIds.add(id);
}

const pinIds = [];

for (const fp of fdnFiles) {
  const base = path.basename(fp);
  const label = path.relative(root, fp).replace(/\\/g, "/");
  const text = fs.readFileSync(fp, "utf8").replace(/^\uFEFF/, "");
  const doc = parseFdn(text, label);

  if (!doc._saw.id || !doc.id) fail(`${label}: missing required field id`);
  if (!doc._saw.k_refs || doc.k_refs == null)
    fail(`${label}: missing required field k_refs`);
  if (!doc._saw.facts) fail(`${label}: missing required field facts`);
  if (!doc._saw.posture_hooks)
    fail(`${label}: missing required field posture_hooks (MAY be empty list)`);
  if (!doc._saw.forbidden_claims)
    fail(`${label}: missing required field forbidden_claims`);
  // theory_pin MAY omit (= none); if present must be closed enum

  if (doc.id) {
    pinIds.push(doc.id);
    const expectedName = `${doc.id}.fdn`;
    if (base !== expectedName) {
      fail(`${label}: filename must be "${expectedName}" (got "${base}")`);
    }
  }

  if (doc.k_refs) {
    for (const k of doc.k_refs) {
      if (!E2_K_REFS.has(k)) {
        fail(`${label}: k_refs contains "${k}" not in E.2 (K1..K7)`);
      }
    }
  }

  if (!doc.facts || doc.facts.length === 0) {
    fail(`${label}: facts MUST contain ≥1 entry`);
  } else {
    for (const fact of doc.facts) {
      if (!fact.key) fail(`${label}: fact missing key`);
      if (fact.value === undefined || fact.value === "")
        fail(`${label}: fact "${fact.key || "?"}" missing value`);
      if (!("maps_to" in fact))
        fail(`${label}: fact "${fact.key || "?"}" missing maps_to`);
      else resolveMapsTo(fact.maps_to, capFields, label, fact.key || "?");
    }
  }

  for (const hook of doc.posture_hooks || []) {
    if (!hook.field) fail(`${label}: posture_hooks entry missing field`);
    else if (!POSTURE_HOOK_FIELDS.has(hook.field)) {
      fail(
        `${label}: posture_hooks field "${hook.field}" not in E.19 pin surface`
      );
    }
    if (!hook.rule || !String(hook.rule).trim())
      fail(`${label}: posture_hooks field "${hook.field}" missing rule`);
  }

  if (doc.theory_pin != null && !THEORY_PINS.has(doc.theory_pin)) {
    fail(
      `${label}: theory_pin "${doc.theory_pin}" not in closed enum (none|posture-score-v0|equal-scope-v0|abort-thermal-v0)`
    );
  }

  if (doc.id) {
    const pinLine = `foundation_pin=${doc.id}`;
    if (!journalText.split(/\r?\n/).some((l) => l.trim() === pinLine || l.includes(pinLine))) {
      fail(
        `${label}: morph journal missing line "${pinLine}" (expected in yoyo/morph/journal.log)`
      );
    }
    if (!listedIds.has(doc.id)) {
      fail(
        `${label}: no *.morph / *.plan.* lists foundation_ids containing "${doc.id}"`
      );
    }
  }
}

if (errors.length > 0) {
  console.error(
    `[FAIL] check-foundations.mjs: ${errors.length} issue(s) — F-foundations RED.\n` +
      errors.map((e) => `  - ${e}`).join("\n")
  );
  process.exit(1);
}

console.log(
  `[ok] check-foundations.mjs: ${pinIds.length} pin(s) valid; maps_to resolved; k_refs ⊆ E.2; journal + foundation_ids refs ok.`
);
console.log(`  pins: ${pinIds.join(", ")}`);
process.exit(0);
