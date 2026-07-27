#!/usr/bin/env node
/**
 * scripts/check-cites.mjs — FACT cite_id resolution (v3.3.2).
 *
 * Rules (Part N.5.2):
 * - Unknown cite_id → fail
 * - Empty / missing bib MUST NOT green FACT ads
 * - MUST NOT require every MUST sentence to have a paper cite
 *
 * Scans PROMPT-v3.md for 【cite:ID】 / 【cite:ID;ID2】 tokens only.
 * Does not demand class tags on all historical MUST lines.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const bibPath = path.join(root, "yoyo", "bib", "fact.bib.yaml");
const promptPath = path.join(root, "PROMPT-v3.md");

function loadCiteIds(yamlText) {
  const ids = new Set();
  for (const line of yamlText.split(/\r?\n/)) {
    const m = line.match(/^\s*-?\s*cite_id:\s*(\S+)\s*$/);
    if (m) ids.add(m[1]);
  }
  return ids;
}

function extractCiteTokens(text) {
  const found = [];
  const re = /【cite:([^】]+)】/g;
  let m;
  while ((m = re.exec(text)) !== null) {
    const parts = m[1]
      .split(/[;,\s]+/)
      .map((s) => s.trim())
      .filter(Boolean);
    for (const id of parts) found.push(id);
  }
  return found;
}

if (!fs.existsSync(bibPath)) {
  console.error(
    "[FAIL] check-cites.mjs: missing yoyo/bib/fact.bib.yaml — empty bib MUST NOT green FACT ads.\n" +
      "  Spec: PROMPT-v3.md Part N.5 / Appendix Bib."
  );
  process.exit(2);
}

const bibText = fs.readFileSync(bibPath, "utf8");
const known = loadCiteIds(bibText);
if (known.size === 0) {
  console.error(
    "[FAIL] check-cites.mjs: fact.bib.yaml has zero cite_id entries — MUST NOT green FACT ads."
  );
  process.exit(2);
}

if (!fs.existsSync(promptPath)) {
  console.error("[FAIL] check-cites.mjs: PROMPT-v3.md not found.");
  process.exit(2);
}

const prompt = fs.readFileSync(promptPath, "utf8");
const used = extractCiteTokens(prompt);
const unknown = [...new Set(used.filter((id) => !known.has(id)))];

if (unknown.length > 0) {
  console.error(
    "[FAIL] check-cites.mjs: unknown cite_id(s):\n  - " +
      unknown.join("\n  - ") +
      "\n  Add real entries to yoyo/bib/fact.bib.yaml / Appendix Bib — do NOT invent DOIs."
  );
  process.exit(1);
}

console.log(
  `[ok] check-cites.mjs: ${known.size} bib ids; ${used.length} 【cite:】 token(s) resolved; no unknown ids.`
);
console.log(
  "  Note: does not require every MUST to carry a paper cite (Part N.5.2)."
);
process.exit(0);
