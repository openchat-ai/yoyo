# Repair body-extend-017 wiring after concurrent overwrite.
from __future__ import annotations

import hashlib
import importlib.util
import json
import re
from pathlib import Path

ROOT = Path(r"f:\yoyo")
spec = importlib.util.spec_from_file_location("fix", ROOT / "scripts" / "_body_extend_017_fix.py")
fix = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fix)

# --- golden.js ---
GOLDEN_JS = ROOT / "yoyo-js" / "scripts" / "golden.js"
text = GOLDEN_JS.read_text(encoding="utf-8")
m = re.search(r"function checkLDBDST51\(\) \{[\s\S]*?\n\}\n", text)
if not m:
    raise SystemExit("LDBDST51 missing")
main = text.find("function main()")
if main < 0:
    raise SystemExit("main missing")
text = text[: m.end()] + fix.GOLDEN_017 + text[main:]
text2, n = re.subn(
    r"checkLDBDST51\(\),.*?checkJMP\(\)",
    "checkLDBDST51(), checkINCH51(), checkDECH51(), checkADDIMMH51(), checkCMPH52(), "
    "checkADDV5052(), checkGET5150(), checkSET12345678(), checkLDBDST52(), checkJMP()",
    text,
    count=1,
)
if n != 1:
    raise SystemExit(f"cases fail n={n}")
text = text2
text = re.sub(
    r"golden: \$\{cases\.length\} case\(s\) ok[^\"]*",
    "golden: ${cases.length} case(s) ok (016+017 batch-11 reflectors)",
    text,
    count=1,
)
GOLDEN_JS.write_bytes(text.encode("utf-8"))
print("golden.js repaired")

# --- self_test.rs ---
SELF = ROOT / "yoyo-rust" / "verifier" / "src" / "self_test.rs"
text = SELF.read_text(encoding="utf-8")
text2, n = re.subn(
    r"    ldb_dst51_slot_check\(\)\?;\n(?:    [a-z0-9_]+_slot_check\(\)\?;\n)+    Ok\(\(\)\)",
    "    ldb_dst51_slot_check()?;\n"
    "    inc_h51_slot_check()?;\n"
    "    dec_h51_slot_check()?;\n"
    "    addimm_h51_slot_check()?;\n"
    "    cmp_h52_slot_check()?;\n"
    "    addv_5052_slot_check()?;\n"
    "    get_5150_slot_check()?;\n"
    "    set_12345678_slot_check()?;\n"
    "    ldb_dst52_slot_check()?;\n"
    "    Ok(())",
    text,
    count=1,
)
if n != 1:
    raise SystemExit(f"self call fail n={n}")
text = text2
m = re.search(r"fn ldb_dst51_slot_check\(\)[\s\S]*?\n\}\n", text)
if not m:
    raise SystemExit("ldb_dst51 fn missing")
cfg = text.find("#[cfg(test)]", m.end())
if cfg < 0:
    raise SystemExit("cfg missing")
text = text[: m.end()] + fix.SELF_TEST_017 + text[cfg:]
SELF.write_bytes(text.encode("utf-8"))
print("self_test.rs repaired")

fix.wire_main()
fix.write_fixtures()
fix.append_ty()

sha = hashlib.sha256((ROOT / "yoyo/projects/yoyo.ty").read_bytes()).hexdigest()
lock = json.loads((ROOT / "yoyo/tests/yoyo.ty.lock").read_text(encoding="utf-8"))
print("ty", sha)
print("lock", lock["sha256"])
print("match", sha == lock["sha256"])
if sha != lock["sha256"]:
    fix.relock()
