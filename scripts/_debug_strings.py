"""List every non-ASCII printable string in a file, filtering by length."""
import sys, re
from pathlib import Path

path = Path(sys.argv[1])
data = path.read_bytes()
min_len = int(sys.argv[2]) if len(sys.argv) > 2 else 6
pat = re.compile(rb"[\x20-\x7e]{%d,}" % min_len)
seen = set()
for m in pat.finditer(data):
    s = m.group()
    key = s.decode("ascii")
    if key in seen:
        continue
    seen.add(key)
    # keep only entries mentioning DLL-related or symbol-ish terms
    if any(k in key.lower() for k in ("yoyo", "compile", "runtime", "dll", "loader", "syscall", "getproc", "kernel32", "readfile", "exit", "load", "dll")):
        print(f"  off=0x{m.start():x}  {key}")
