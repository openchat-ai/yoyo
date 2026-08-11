fp = r"F:\yoyo\yoyo-rust\verifier\src\platform.rs"
with open(fp, "r", encoding="utf-8") as f:
    src = f.read()

# Debug: check indentation and patterns
import re
for i, line in enumerate(src.split("\n")):
    if "foreign_" in line and "fn foreign_" not in line and "foreign_" in line:
        # print repr
        print(f"L{i+1}: {repr(line)}")
