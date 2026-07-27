#!/usr/bin/env node
/**
 * scripts/check-sugar.mjs — minimal deterministic `.tys` → Layer-S desugar gate.
 *
 * Scope (honest thin): yoyo/examples/hello.tys only, sugar profile `tys-hello-v1`.
 * Layer-S remains law — this gate proves invertibility for the hello sketch;
 * DDC / lock MUST NOT treat `.tys` as compare truth (Part N.8).
 *
 * Exit 0: hello desugars to hello.expected.ty (normalized) AND garbage probes fail-closed.
 * Exit ≠ 0: mismatch, parse error on hello, or garbage accepted.
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(__dirname, "..");
const SUGAR_PROFILE = "tys-hello-v1";
const HANDLER_ID_BASE = 0x10;

/** Thin mnemonic → Layer-M u8 (core subset needed by hello). JZ aliases JE. */
const MNEMONICS = Object.freeze({
  SET: { op: 0x30, arity: 2 },
  HANDLER: { op: 0x40, arity: 1 },
  CMP: { op: 0x65, arity: 2 },
  INC: { op: 0x66, arity: 1 },
  JMP: { op: 0x70, arity: 1 },
  JE: { op: 0x71, arity: 1 },
  JZ: { op: 0x71, arity: 1 }, // alias → JE
  RET: { op: 0xff, arity: 0 },
});

function fail(msg, code = 1) {
  console.error(`[FAIL] check-sugar.mjs: ${msg}`);
  process.exit(code);
}

/** Strip comments / blank lines; collapse whitespace for Layer-S compare. */
function normalizeTy(text) {
  const lines = [];
  for (const raw of text.split(/\r?\n/)) {
    const noComment = raw.replace(/;.*$/, "");
    const trimmed = noComment.trim();
    if (!trimmed) continue;
    lines.push(trimmed.replace(/\s+/g, " "));
  }
  return lines.join("\n");
}

function hexByte(n) {
  return n.toString(16).toUpperCase().padStart(2, "0");
}

/**
 * Deterministic desugar: mnemonics + named HANDLER labels + Part-8 slot names.
 * Handler labels → u8 ids from 0x10 in first-occurrence order.
 * Jump targets that name a HANDLER label resolve to that id.
 * Slot names (i/n/sum/…) pass through as Part-8 tokens (same as stock_gui.ty).
 */
export function desugarTys(source, { profile = SUGAR_PROFILE } = {}) {
  if (profile !== SUGAR_PROFILE) {
    throw new Error(`unsupported sugar profile: ${profile}`);
  }
  if (typeof source !== "string") {
    throw new Error("desugar input must be a string");
  }
  if (!source.trim()) {
    throw new Error("empty .tys input");
  }

  const labelToId = new Map();
  let nextHandlerId = HANDLER_ID_BASE;

  // Pass 1: bind HANDLER labels in source order (fail on duplicates / bad shape).
  const stmts = [];
  for (const raw of source.split(/\r?\n/)) {
    const line = raw.replace(/;.*$/, "").trim();
    if (!line) continue;

    const handlerDef = line.match(/^HANDLER\s+([A-Za-z_][A-Za-z0-9_]*)\s*:?\s*$/i);
    if (handlerDef) {
      const name = handlerDef[1];
      if (labelToId.has(name)) {
        throw new Error(`duplicate HANDLER label: ${name}`);
      }
      if (nextHandlerId > 0xff) {
        throw new Error("handler id space exhausted");
      }
      labelToId.set(name, nextHandlerId++);
      stmts.push({ kind: "handler", name });
      continue;
    }

    const parts = line.split(/\s+/);
    const mnem = parts[0].toUpperCase();
    if (mnem === "HANDLER") {
      throw new Error(`malformed HANDLER line: ${line}`);
    }
    if (!(mnem in MNEMONICS)) {
      throw new Error(`unknown mnemonic / NL-as-ISA refused: ${parts[0]}`);
    }
    const spec = MNEMONICS[mnem];
    const args = parts.slice(1);
    if (args.length !== spec.arity) {
      throw new Error(
        `${mnem} expects arity ${spec.arity}, got ${args.length}: ${line}`
      );
    }
    stmts.push({ kind: "op", mnem, op: spec.op, args });
  }

  if (stmts.length === 0) {
    throw new Error("no statements after parse");
  }
  if (![...stmts].some((s) => s.kind === "handler")) {
    throw new Error("desugar requires ≥1 HANDLER");
  }

  // Pass 2: emit Layer-S lines (hex opcode + resolved args).
  const out = [];
  for (const s of stmts) {
    if (s.kind === "handler") {
      out.push(`${hexByte(0x40)} ${hexByte(labelToId.get(s.name))}`);
      continue;
    }
    const resolved = s.args.map((a) => {
      if (labelToId.has(a)) return hexByte(labelToId.get(a));
      // immediates: decimal or 0x-hex → lowercase/decimal canonical as source token
      if (/^0x[0-9a-fA-F]+$/.test(a)) return String(parseInt(a, 16));
      if (/^\d+$/.test(a)) return a;
      // Part-8 named slot (or unresolved label → hard error)
      if (/^[A-Za-z_][A-Za-z0-9_]*$/.test(a)) {
        // Jump/call-like ops must resolve labels; slot names stay names.
        if (s.mnem === "JMP" || s.mnem === "JZ" || s.mnem === "JE") {
          throw new Error(`unresolved jump label: ${a}`);
        }
        return a;
      }
      throw new Error(`illegal operand: ${a}`);
    });
    const tokens = [hexByte(s.op), ...resolved];
    out.push(tokens.join(" "));
  }

  return out.join("\n") + "\n";
}

function assertThrows(label, fn) {
  try {
    fn();
  } catch {
    return;
  }
  throw new Error(`fail-closed probe accepted garbage: ${label}`);
}

function runGarbageProbes() {
  assertThrows("empty", () => desugarTys(""));
  assertThrows("whitespace", () => desugarTys("   \n  ; only comment\n"));
  assertThrows("nl-as-isa", () =>
    desugarTys("please compile this natural language loop\n")
  );
  assertThrows("unknown-mnem", () =>
    desugarTys("HANDLER entry:\n  FOOBAR i 0\n")
  );
  assertThrows("bad-arity", () => desugarTys("HANDLER entry:\n  SET i\n"));
  assertThrows("no-handler", () => desugarTys("SET i 0\nRET\n"));
  assertThrows("unresolved-jmp", () =>
    desugarTys("HANDLER entry:\n  JMP nowhere\n")
  );
}

function main() {
  const tysPath = path.join(root, "yoyo", "examples", "hello.tys");
  const expectedPath = path.join(root, "yoyo", "examples", "hello.expected.ty");

  if (!fs.existsSync(tysPath)) {
    fail(`missing ${path.relative(root, tysPath)}`, 2);
  }
  if (!fs.existsSync(expectedPath)) {
    fail(`missing ${path.relative(root, expectedPath)}`, 2);
  }

  let got;
  try {
    got = desugarTys(fs.readFileSync(tysPath, "utf8"));
  } catch (e) {
    fail(`hello.tys desugar error: ${e.message}`);
  }

  const expected = fs.readFileSync(expectedPath, "utf8");
  const gotN = normalizeTy(got);
  const expN = normalizeTy(expected);
  if (gotN !== expN) {
    console.error("[FAIL] check-sugar.mjs: desugar ≠ hello.expected.ty");
    console.error("--- got ---");
    console.error(gotN);
    console.error("--- expected ---");
    console.error(expN);
    process.exit(1);
  }

  try {
    runGarbageProbes();
  } catch (e) {
    fail(e.message);
  }

  console.log(
    `[OK] check-sugar.mjs: hello.tys → Layer-S matches hello.expected.ty (${SUGAR_PROFILE}); garbage fail-closed.`
  );
  console.log(
    "  Note: DDC/lock still use Layer-S only — .tys is convenience, not compare truth."
  );
}

const isMain =
  process.argv[1] &&
  path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isMain) {
  main();
}
