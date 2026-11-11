#!/usr/bin/env python3
"""
Annotated place_stub disasm for human/AI audit (Capstone, no llvm-objdump).

Outputs:
  scripts/fixtures/audit-stub/STUB.disasm.txt
  scripts/fixtures/audit-stub/STUB.WALKTHROUGH.md
"""
from __future__ import annotations

import re
import struct
from pathlib import Path

from capstone import CS_ARCH_X86, CS_MODE_64, Cs

ROOT = Path(__file__).resolve().parents[1]
PE_PATH = ROOT / "scripts" / "fixtures" / "yoyo_place_stub.exe"
OUT_DIR = ROOT / "scripts" / "fixtures" / "audit-stub"
OUT_DISASM = OUT_DIR / "STUB.disasm.txt"
OUT_WALK = OUT_DIR / "STUB.WALKTHROUGH.md"
STARTUP = 13
PLACE = 5876

IAT = {
    0: "VirtualAlloc",
    8: "CreateFileA",
    0x10: "ReadFile",
    0x18: "WriteFile",
    0x20: "CloseHandle",
    0x28: "ExitProcess",
}

# Paths we expect stub may open (contract). Others = review flag.
ALLOW_OPEN = {
    "input.tyb",
    "output.exe",
    "code.bin",
    "data.bin",
    "emit.bins",
    "yoyo_compiler.exe",
}


def pe_map(pe: bytes):
    lfanew = struct.unpack_from("<I", pe, 0x3C)[0]
    opt = lfanew + 24
    entry_rva = struct.unpack_from("<I", pe, opt + 16)[0]
    image_base = struct.unpack_from("<Q", pe, opt + 24)[0]
    soh = struct.unpack_from("<H", pe, lfanew + 20)[0]
    nsec = struct.unpack_from("<H", pe, lfanew + 6)[0]
    sec = lfanew + 24 + soh
    sections = []
    for i in range(nsec):
        s = sec + i * 40
        name = pe[s : s + 8].split(b"\0", 1)[0].decode("ascii", "replace")
        vsize, rva, rawsz, raw = struct.unpack_from("<IIII", pe, s + 8)
        sections.append((name, rva, vsize, raw, rawsz))
    return image_base, entry_rva, sections


def va_to_file(va: int, image_base: int, sections) -> int | None:
    rva = va - image_base
    for _name, srva, vsize, raw, rawsz in sections:
        if srva <= rva < srva + max(vsize, rawsz):
            off = raw + (rva - srva)
            if raw <= off < raw + rawsz:
                return off
    return None


def read_cstr(pe: bytes, file_off: int | None) -> str | None:
    if file_off is None or not (0 <= file_off < len(pe)):
        return None
    if not (32 <= pe[file_off] < 127):
        return None
    end = file_off
    while end < len(pe) and 32 <= pe[end] < 127:
        end += 1
    if end - file_off < 4:
        return None
    if end < len(pe) and pe[end] != 0:
        # still accept if looks like filename
        s = pe[file_off:end].decode("ascii")
        if "." not in s and not s.endswith(".exe") and not s.endswith(".bin") and not s.endswith(".tyb"):
            return None
    return pe[file_off:end].decode("ascii")


def rip_target(insn) -> int | None:
    if "rip" not in insn.op_str:
        return None
    m = re.search(r"\[rip\s*([\+\-]\s*0x[0-9a-fA-F]+)\]", insn.op_str)
    if not m:
        return None
    disp = int(m.group(1).replace(" ", ""), 16)
    return insn.address + insn.size + disp


def main() -> None:
    pe = PE_PATH.read_bytes()
    image_base, entry_rva, sections = pe_map(pe)
    text = next(s for s in sections if s[0].startswith(".text"))
    _name, text_rva, _vs, text_raw, _rs = text
    need = STARTUP + PLACE
    code = pe[text_raw : text_raw + need]
    if len(code) < need:
        raise SystemExit("HUMANAUDIT status=RED reason=text_short")

    base = image_base + text_rva
    entry_va = image_base + entry_rva
    md = Cs(CS_ARCH_X86, CS_MODE_64)

    # Pass 1: disassemble fully; collect lea→string targets (only real cstrings)
    insns = list(md.disasm(code, base))
    str_at: dict[int, str] = {}
    for insn in insns:
        if insn.mnemonic != "lea":
            continue
        tgt = rip_target(insn)
        if tgt is None:
            continue
        s = read_cstr(pe, va_to_file(tgt, image_base, sections))
        if s and len(s) >= 4 and re.search(r"[A-Za-z].*\.|bin|tyb|exe|dll|SEMHOST|yoyo_", s):
            str_at[tgt] = s

    # Expand string spans (NUL-terminated) for DATA emission
    data_spans: list[tuple[int, int, str]] = []
    for va, s in sorted(str_at.items()):
        foff = va_to_file(va, image_base, sections)
        if foff is None:
            continue
        end_off = foff + len(s)
        if end_off < len(pe) and pe[end_off] == 0:
            end_off += 1
        end_va = va + (end_off - foff)
        # only mark if inside our disasm window
        if base <= va < base + need:
            data_spans.append((va, min(end_va, base + need), s))

    def in_data(va: int) -> tuple[int, int, str] | None:
        for a, b, s in data_spans:
            if a <= va < b:
                return a, b, s
        return None

    lines = [
        "; AUTO-HUMANAUDIT annotated disasm — Capstone intel",
        f"; PE: {PE_PATH.relative_to(ROOT).as_posix()}",
        f"; image_base=0x{image_base:x}  .text_rva=0x{text_rva:x}  entry_va=0x{entry_va:x}",
        f"; bytes={need} (= PE_startup {STARTUP} + code_place {PLACE})",
        "; Read banners `; ==========` and `; => API` / `; str=` — not every insn",
        "; ≠ Thompson",
        "",
    ]
    phases_emitted: set[str] = set()

    def emit_phase(va: int, title: str):
        key = title.split("@")[0].strip()
        if key in phases_emitted:
            return
        phases_emitted.add(key)
        lines.append("")
        lines.append(f"; ========== {title} @ 0x{va:x} ==========")

    call_rows = []
    pending_rcx_str: str | None = None
    pending_rcx_age = 999
    flags = []  # review flags for walkthrough

    i = 0
    while i < len(code):
        va = base + i
        span = in_data(va)
        if span and span[0] == va:
            a, b, s = span
            emit_phase(va, f'DATA string "{s}"')
            lines.append(f'{va:016x}:  ; DATA "{s}"  (do not decode as code)')
            i = b - base
            continue
        if span:
            i += 1
            continue

        chunk = code[i:]
        got = list(md.disasm(chunk, va, count=1))
        if not got:
            lines.append(f"{va:016x}:  {code[i]:02x}                               db 0x{code[i]:02x}")
            i += 1
            continue
        insn = got[0]
        # don't decode into a string span
        if any(insn.address < b and insn.address + insn.size > a for a, b, _ in data_spans):
            # step one byte to resync
            i += 1
            continue

        pending_rcx_age += 1
        hexb = " ".join(f"{b:02x}" for b in insn.bytes)
        marks: list[str] = []

        if insn.address == entry_va:
            emit_phase(insn.address, "ENTRY / PE startup")
            marks.append("<< entry")
        if insn.address == base + STARTUP:
            emit_phase(insn.address, "PLACE BODY")
            marks.append("<< place body")

        if insn.mnemonic == "lea":
            tgt = rip_target(insn)
            s = None
            if tgt is not None:
                s = str_at.get(tgt) or read_cstr(pe, va_to_file(tgt, image_base, sections))
            # lea rcx, [r15 + disp] → string in .data (r15=.data base @ entry)
            if s is None and insn.op_str.startswith("rcx") and "r15" in insn.op_str:
                m = re.search(r"\[r15\s*\+\s*(0x[0-9a-fA-F]+|\d+)\]", insn.op_str)
                if m:
                    disp = int(m.group(1), 16) if m.group(1).startswith("0x") else int(m.group(1))
                    # entry: lea r15,[rip+…] → .data VA = image_base + 0x3000 for this stub
                    data_va = image_base + 0x3000
                    s = read_cstr(pe, va_to_file(data_va + disp, image_base, sections))
            if s and len(s) >= 4:
                marks.append(f'str="{s}"')
                if insn.op_str.startswith("rcx"):
                    pending_rcx_str = s
                    pending_rcx_age = 0

        api = None
        if insn.mnemonic == "call" and "r15" in insn.op_str:
            if re.search(r"\[r15\]\s*$", insn.op_str) or insn.op_str.rstrip().endswith("[r15]"):
                api = IAT[0]
            else:
                m = re.search(r"\[r15\s*\+\s*(0x[0-9a-fA-F]+|\d+)\]", insn.op_str)
                if m:
                    off = int(m.group(1), 16) if m.group(1).startswith("0x") else int(m.group(1))
                    api = IAT.get(off, f"UNKNOWN_IAT+{off:#x}")
            if api:
                path_hint = ""
                if api == "CreateFileA" and pending_rcx_age <= 12 and pending_rcx_str:
                    path_hint = pending_rcx_str
                    emit_phase(insn.address, f"CreateFileA → {path_hint}")
                    if path_hint not in ALLOW_OPEN:
                        flags.append(f"CreateFileA unusual path `{path_hint}` @ 0x{insn.address:x}")
                elif api == "VirtualAlloc":
                    emit_phase(insn.address, "VirtualAlloc")
                elif api == "WriteFile":
                    emit_phase(insn.address, "WriteFile")
                elif api == "ExitProcess":
                    emit_phase(insn.address, "ExitProcess")
                call_rows.append({"va": insn.address, "api": api, "path_hint": path_hint})
                if api.startswith("UNKNOWN"):
                    flags.append(f"UNKNOWN IAT call @ 0x{insn.address:x}")

        if "0x4c503159" in insn.op_str:
            marks.append("magic Y1PL")
            emit_phase(insn.address, "CHECK magic Y1PL")
        if "0x425954" in insn.op_str:
            marks.append("magic TYB")
            emit_phase(insn.address, "CHECK magic TYB")
        if insn.mnemonic == "mov" and "byte ptr [rsi]" in insn.op_str and "0x4d" in insn.op_str:
            emit_phase(insn.address, "BUILD PE MZ header")
        if "movsb" in insn.op_str or insn.mnemonic == "movsb":
            emit_phase(insn.address, "rep movsb copy into PE")

        op = insn.op_str
        extra = ""
        if api:
            op = f"{insn.op_str}"
            extra = f"  ; => {api}" + (f' path="{path_hint}"' if path_hint else "")
        elif marks:
            extra = "  ; " + " | ".join(marks)
        lines.append(f"{insn.address:016x}:  {hexb:<32}  {insn.mnemonic:<8} {op}{extra}")
        i += insn.size

    # Also list PE-wide interesting strings for audit
    pe_strs = []
    k = 0
    while k < len(pe):
        if 32 <= pe[k] < 127:
            j = k
            while j < len(pe) and 32 <= pe[j] < 127:
                j += 1
            if j - k >= 4:
                s = pe[k:j].decode("ascii")
                if re.search(r"\.(bin|exe|tyb)|yoyo_|SEMHOST|kernel32|Virtual|Create|Read|Write|Close|Exit", s):
                    pe_strs.append(s)
            k = j
        else:
            k += 1

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    OUT_DISASM.write_text("\n".join(lines) + "\n", encoding="utf-8")

    # Summarize APIs
    from collections import Counter

    api_c = Counter(r["api"] for r in call_rows)
    opens = [r for r in call_rows if r["api"] == "CreateFileA"]

    walk = [
        "# Stub 审核走读（人或 AI 用这个，别啃生肉）",
        "",
        "> `python scripts/gen-stub-disasm.py` → 本文件 + `STUB.disasm.txt`。≠ Thompson。",
        "",
        "## 审法（强制）",
        "",
        "1. **先读本走读**，再只打开反汇编里带 `; =>` / `; ==========` / `; str=` / `; DATA` 的行",
        "2. CreateFile 路径必须落在允许名单；否则记红旗",
        "3. 禁止出现 `UNKNOWN_IAT`；所有 call 应已标成 6 API 之一",
        "",
        "## 允许打开的路径",
        "",
        ", ".join(f"`{p}`" for p in sorted(ALLOW_OPEN)),
        "",
        "## CreateFileA 一览",
        "",
        "| VA | path | 允许? |",
        "| --- | --- | --- |",
    ]
    for r in opens:
        p = r["path_hint"] or "(未解析到 lea rcx 字符串 — 可能路径在寄存器/表里)"
        ok = "YES" if r["path_hint"] in ALLOW_OPEN else ("?" if not r["path_hint"] else "FLAG")
        walk.append(f"| `0x{r['va']:x}` | {p} | **{ok}** |")

    walk.extend(
        [
            "",
            "## 全部 API 计数",
            "",
            "| API | count |",
            "| --- | ---: |",
        ]
    )
    for k, n in sorted(api_c.items()):
        walk.append(f"| {k} | {n} |")

    walk.extend(["", "## 审核红旗（生成器）", ""])
    if flags:
        for f in flags:
            walk.append(f"- {f}")
    else:
        walk.append("- （无）路径均在允许名单或未解析；无 UNKNOWN_IAT")

    walk.extend(
        [
            "",
            "## PE 内相关字符串（抽查用）",
            "",
        ]
    )
    for s in dict.fromkeys(pe_strs):
        walk.append(f"- `{s}`")

    walk.extend(
        [
            "",
            "## 分场",
            "",
            "在 `STUB.disasm.txt` 搜索 `==========` 跳读：ENTRY → 开 input.tyb → Y1PL/TYB → "
            "VirtualAlloc/建 PE → 写 output.exe → ExitProcess → 后续 bins/place 变体。",
            "",
            "## 诚实",
            "",
            "- 标注让 AI/人**抽查面**变小，不是证明无逻辑后门",
            "- 未解析到路径的 CreateFile 需结合前后 `lea`/`mov` 人工看一眼",
            "",
        ]
    )
    OUT_WALK.write_text("\n".join(walk) + "\n", encoding="utf-8")

    if any(r["api"].startswith("UNKNOWN") for r in call_rows):
        print("HUMANAUDIT status=RED reason=unknown_iat")
        raise SystemExit(1)

    print(f"HUMANAUDIT disasm={OUT_DISASM.relative_to(ROOT).as_posix()}")
    print(f"HUMANAUDIT walk={OUT_WALK.relative_to(ROOT).as_posix()} calls={len(call_rows)} opens={len(opens)}")
    print(f"HUMANAUDIT flags={len(flags)}")
    print("HUMANAUDIT status=GREEN")


if __name__ == "__main__":
    main()
