#!/usr/bin/env python3
"""Export place_stub .text as Capstone intel disasm (no llvm-objdump)."""
from __future__ import annotations

import struct
from pathlib import Path

from capstone import CS_ARCH_X86, CS_MODE_64, Cs

ROOT = Path(__file__).resolve().parents[1]
PE_PATH = ROOT / "scripts" / "fixtures" / "yoyo_place_stub.exe"
OUT = ROOT / "scripts" / "fixtures" / "audit-stub" / "STUB.disasm.txt"
STARTUP = 13
PLACE = 5876


def main() -> None:
    pe = PE_PATH.read_bytes()
    lfanew = struct.unpack_from("<I", pe, 0x3C)[0]
    opt = lfanew + 24
    entry_rva = struct.unpack_from("<I", pe, opt + 16)[0]
    image_base = struct.unpack_from("<Q", pe, opt + 24)[0]
    soh = struct.unpack_from("<H", pe, lfanew + 20)[0]
    nsec = struct.unpack_from("<H", pe, lfanew + 6)[0]
    sec = lfanew + 24 + soh
    rva = raw = None
    for i in range(nsec):
        s = sec + i * 40
        name = pe[s : s + 8].split(b"\0", 1)[0].decode("ascii", "replace")
        _vsize, sec_rva, _rawsz, sec_raw = struct.unpack_from("<IIII", pe, s + 8)
        if name.startswith(".text"):
            rva, raw = sec_rva, sec_raw
            break
    if rva is None or raw is None:
        raise SystemExit("HUMANAUDIT status=RED reason=no_text")
    need = STARTUP + PLACE
    code = pe[raw : raw + need]
    if len(code) < need:
        raise SystemExit("HUMANAUDIT status=RED reason=text_short")
    md = Cs(CS_ARCH_X86, CS_MODE_64)
    base = image_base + rva
    lines = [
        "; AUTO-HUMANAUDIT export — Capstone intel (no llvm-objdump)",
        f"; PE: {PE_PATH.relative_to(ROOT).as_posix()}",
        f"; image_base=0x{image_base:x}  .text_rva=0x{rva:x}  entry_va=0x{image_base + entry_rva:x}",
        f"; bytes={need} (= PE_startup {STARTUP} + code_place {PLACE})",
        "; ≠ Thompson",
        "",
    ]
    for insn in md.disasm(code, base):
        hexb = " ".join(f"{b:02x}" for b in insn.bytes)
        mark = ""
        if insn.address == image_base + entry_rva:
            mark = "  ; << entry"
        if insn.address == image_base + rva + STARTUP:
            mark = "  ; << code.bin place half starts (approx)"
        lines.append(
            f"{insn.address:016x}:  {hexb:<32}  {insn.mnemonic:<8} {insn.op_str}{mark}"
        )
    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(f"HUMANAUDIT disasm={OUT.relative_to(ROOT).as_posix()} insns={len(lines) - 6}")
    print("HUMANAUDIT status=GREEN")


if __name__ == "__main__":
    main()
