#!/usr/bin/env python3
"""
AUDITMAX annotated place_stub disasm — Capstone, no llvm-objdump.

Outputs:
  scripts/fixtures/audit-stub/STUB.disasm.txt
  scripts/fixtures/audit-stub/STUB.WALKTHROUGH.md
  scripts/fixtures/audit-stub/STUB.AI.md
  scripts/fixtures/audit-stub/STUB.CALLSEQ.pin  (write if missing; else compare)
"""
from __future__ import annotations

import re
import struct
from collections import Counter
from pathlib import Path

from capstone import CS_ARCH_X86, CS_MODE_64, Cs

ROOT = Path(__file__).resolve().parents[1]
PE_PATH = ROOT / "scripts" / "fixtures" / "yoyo_place_stub.exe"
OUT_DIR = ROOT / "scripts" / "fixtures" / "audit-stub"
OUT_DISASM = OUT_DIR / "STUB.disasm.txt"
OUT_WALK = OUT_DIR / "STUB.WALKTHROUGH.md"
OUT_AI = OUT_DIR / "STUB.AI.md"
OUT_CALLSEQ = OUT_DIR / "STUB.CALLSEQ.pin"
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

ALLOW_OPEN = {
    "input.tyb",
    "output.exe",
    "code.bin",
    "data.bin",
    "emit.bins",
    "yoyo_compiler.exe",
}

# Fail-closed: these substrings in PE ASCII = review RED (exfil / spawn / net).
BAN_SUBSTR = (
    "http://",
    "https://",
    "ftp://",
    "powershell",
    "cmd.exe",
    "CreateProcess",
    "LoadLibrary",
    "GetProcAddress",
    "WinExec",
    "URLDownload",
    "WSAStartup",
    "socket",
    "WinHttp",
    "InternetOpen",
    "RegSetValue",
    "WriteProcessMemory",
)


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


def imm_from_op(op: str) -> int | None:
    m = re.search(r"(?:^|,\s*)(0x[0-9a-fA-F]+|\d+)\s*$", op)
    if not m:
        return None
    t = m.group(1)
    return int(t, 16) if t.startswith("0x") else int(t)


def scan_ban(pe: bytes) -> list[str]:
    hits = []
    low = pe.lower()
    for s in BAN_SUBSTR:
        if s.encode("ascii").lower() in low:
            hits.append(s)
    return hits


def parse_callseq_pin(text: str) -> dict[str, str]:
    out = {}
    for line in text.splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        if "=" in line:
            k, v = line.split("=", 1)
            out[k.strip()] = v.strip()
    return out


def main() -> None:
    pe = PE_PATH.read_bytes()
    image_base, entry_rva, sections = pe_map(pe)
    text = next(s for s in sections if s[0].startswith(".text"))
    _name, text_rva, _vs, text_raw, _rs = text
    need = STARTUP + PLACE
    code = pe[text_raw : text_raw + need]
    if len(code) < need:
        raise SystemExit("AUDITMAX status=RED reason=text_short")

    ban_hits = scan_ban(pe)
    base = image_base + text_rva
    entry_va = image_base + entry_rva
    md = Cs(CS_ARCH_X86, CS_MODE_64)

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

    data_spans: list[tuple[int, int, str]] = []
    for va, s in sorted(str_at.items()):
        foff = va_to_file(va, image_base, sections)
        if foff is None:
            continue
        end_off = foff + len(s)
        if end_off < len(pe) and pe[end_off] == 0:
            end_off += 1
        end_va = va + (end_off - foff)
        if base <= va < base + need:
            data_spans.append((va, min(end_va, base + need), s))

    def in_data(va: int) -> tuple[int, int, str] | None:
        for a, b, s in data_spans:
            if a <= va < b:
                return a, b, s
        return None

    lines = [
        "; AUTO-AUDITMAX annotated disasm — Capstone intel",
        f"; PE: {PE_PATH.relative_to(ROOT).as_posix()}",
        f"; image_base=0x{image_base:x}  .text_rva=0x{text_rva:x}  entry_va=0x{entry_va:x}",
        f"; bytes={need} (= PE_startup {STARTUP} + code_place {PLACE})",
        "; Read `; ==========` / `; => API` / `; str=` / `; DATA` / `; call#` — not every insn",
        "; ≠ Thompson",
        "",
    ]
    phases: list[tuple[int, str]] = []
    phases_emitted: set[str] = set()

    def emit_phase(va: int, title: str):
        key = title.split("@")[0].strip()
        if key in phases_emitted:
            return
        phases_emitted.add(key)
        phases.append((va, title))
        lines.append("")
        lines.append(f"; ========== {title} @ 0x{va:x} ==========")

    call_rows: list[dict] = []
    pending_rcx_str: str | None = None
    pending_rcx_age = 999
    # recent immediates for size hints (age counted in insns)
    last_edx: tuple[int, int] | None = None  # (imm, age)
    last_r8d: tuple[int, int] | None = None
    flags: list[str] = []

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
        if any(insn.address < b and insn.address + insn.size > a for a, b, _ in data_spans):
            i += 1
            continue

        pending_rcx_age += 1
        if last_edx:
            last_edx = (last_edx[0], last_edx[1] + 1)
        if last_r8d:
            last_r8d = (last_r8d[0], last_r8d[1] + 1)

        hexb = " ".join(f"{b:02x}" for b in insn.bytes)
        marks: list[str] = []

        if insn.address == entry_va:
            emit_phase(insn.address, "ENTRY / PE startup")
            marks.append("<< entry")
        if insn.address == base + STARTUP:
            emit_phase(insn.address, "PLACE BODY")
            marks.append("<< place body")

        # Track size-ish immediates: mov edx,imm / mov r8d,imm / mov eax,imm before call
        if insn.mnemonic == "mov":
            op = insn.op_str
            imm = imm_from_op(op)
            if imm is not None:
                if op.startswith("edx,") or op.startswith("rdx,"):
                    last_edx = (imm, 0)
                elif op.startswith("r8d,") or op.startswith("r8,"):
                    last_r8d = (imm, 0)

        if insn.mnemonic == "lea":
            tgt = rip_target(insn)
            s = None
            if tgt is not None:
                s = str_at.get(tgt) or read_cstr(pe, va_to_file(tgt, image_base, sections))
            if s is None and insn.op_str.startswith("rcx") and "r15" in insn.op_str:
                m = re.search(r"\[r15\s*\+\s*(0x[0-9a-fA-F]+|\d+)\]", insn.op_str)
                if m:
                    disp = int(m.group(1), 16) if m.group(1).startswith("0x") else int(m.group(1))
                    data_va = image_base + 0x3000
                    s = read_cstr(pe, va_to_file(data_va + disp, image_base, sections))
            if s and len(s) >= 4:
                marks.append(f'str="{s}"')
                if insn.op_str.startswith("rcx"):
                    pending_rcx_str = s
                    pending_rcx_age = 0

        api = None
        path_hint = ""
        size_hint = ""
        if insn.mnemonic == "call" and "r15" in insn.op_str:
            if re.search(r"\[r15\]\s*$", insn.op_str) or insn.op_str.rstrip().endswith("[r15]"):
                api = IAT[0]
            else:
                m = re.search(r"\[r15\s*\+\s*(0x[0-9a-fA-F]+|\d+)\]", insn.op_str)
                if m:
                    off = int(m.group(1), 16) if m.group(1).startswith("0x") else int(m.group(1))
                    api = IAT.get(off, f"UNKNOWN_IAT+{off:#x}")
            if api:
                n = len(call_rows) + 1
                if api == "CreateFileA" and pending_rcx_age <= 12 and pending_rcx_str:
                    path_hint = pending_rcx_str
                    emit_phase(insn.address, f"CreateFileA → {path_hint}")
                    if path_hint not in ALLOW_OPEN:
                        flags.append(f"CreateFileA unusual path `{path_hint}` @ 0x{insn.address:x}")
                elif api == "VirtualAlloc":
                    emit_phase(insn.address, "VirtualAlloc")
                    if last_edx and last_edx[1] <= 8:
                        size_hint = f"size=0x{last_edx[0]:x}"
                elif api in ("ReadFile", "WriteFile"):
                    emit_phase(insn.address, api)
                    if last_r8d and last_r8d[1] <= 10:
                        size_hint = f"n=0x{last_r8d[0]:x}"
                    elif last_edx and last_edx[1] <= 10 and api == "WriteFile":
                        # sometimes size in r8; track edx as fallback noise
                        pass
                elif api == "ExitProcess":
                    emit_phase(insn.address, "ExitProcess")
                elif api == "CloseHandle":
                    pass
                call_rows.append(
                    {
                        "n": n,
                        "va": insn.address,
                        "api": api,
                        "path_hint": path_hint,
                        "size_hint": size_hint,
                    }
                )
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
            bits = [f"=> {api}", f"call#{len(call_rows)}"]
            if path_hint:
                bits.append(f'path="{path_hint}"')
            if size_hint:
                bits.append(size_hint)
            extra = "  ; " + " ".join(bits)
        elif marks:
            extra = "  ; " + " | ".join(marks)
        lines.append(f"{insn.address:016x}:  {hexb:<32}  {insn.mnemonic:<8} {op}{extra}")
        i += insn.size

    pe_strs: list[str] = []
    k = 0
    while k < len(pe):
        if 32 <= pe[k] < 127:
            j = k
            while j < len(pe) and 32 <= pe[j] < 127:
                j += 1
            if j - k >= 4:
                s = pe[k:j].decode("ascii")
                if re.search(
                    r"\.(bin|exe|tyb)|yoyo_|SEMHOST|kernel32|Virtual|Create|Read|Write|Close|Exit",
                    s,
                ):
                    pe_strs.append(s)
            k = j
        else:
            k += 1

    OUT_DIR.mkdir(parents=True, exist_ok=True)
    OUT_DISASM.write_text("\n".join(lines) + "\n", encoding="utf-8")

    api_c = Counter(r["api"] for r in call_rows)
    opens = [r for r in call_rows if r["api"] == "CreateFileA"]
    open_seq = ",".join(r["path_hint"] or "?" for r in opens)
    api_seq = ",".join(r["api"] for r in call_rows)
    api_counts = ",".join(f"{k}:{n}" for k, n in sorted(api_c.items()))

    if ban_hits:
        for h in ban_hits:
            flags.append(f"BAN_SUBSTR `{h}` in PE")

    # CALLSEQ pin
    pin_body = "\n".join(
        [
            "# AUTO-AUDITMAX call contract for place_stub (fail-closed).",
            "# ≠ Thompson.",
            f"opens={open_seq}",
            f"apis={api_seq}",
            f"api_counts={api_counts}",
            f"calls={len(call_rows)}",
            f"createfile={len(opens)}",
            "",
        ]
    )
    if OUT_CALLSEQ.exists():
        want = parse_callseq_pin(OUT_CALLSEQ.read_text(encoding="utf-8"))
        got = parse_callseq_pin(pin_body)
        for key in ("opens", "apis", "api_counts", "calls", "createfile"):
            if want.get(key) != got.get(key):
                flags.append(f"CALLSEQ drift `{key}` want={want.get(key)!r} got={got.get(key)!r}")
                print(f"AUDITMAX status=RED reason=callseq_{key}")
                print(f"AUDITMAX want_{key}={want.get(key)}")
                print(f"AUDITMAX got_{key}={got.get(key)}")
                raise SystemExit(1)
        print("AUDITMAX callseq=EQUAL")
    else:
        OUT_CALLSEQ.write_text(pin_body, encoding="utf-8")
        print(f"AUDITMAX callseq=WRITE {OUT_CALLSEQ.relative_to(ROOT).as_posix()}")

    walk = [
        "# Stub 审核走读（人或 AI 用这个，别啃生肉）",
        "",
        "> `python scripts/gen-stub-disasm.py` → 本文件 + `STUB.disasm.txt` + `STUB.AI.md`。≠ Thompson。",
        "",
        "## 审法（强制）",
        "",
        "1. **先读 `STUB.AI.md`（一页）**，再读本走读；反汇编只看 `; =>` / `; ==========` / `; str=` / `; DATA` / `; call#`",
        "2. CreateFile 路径必须落在允许名单；否则记红旗",
        "3. 禁止 `UNKNOWN_IAT`；禁止 BAN 串；`STUB.CALLSEQ.pin` 必须 EQUAL",
        "",
        "## 允许打开的路径",
        "",
        ", ".join(f"`{p}`" for p in sorted(ALLOW_OPEN)),
        "",
        "## CreateFileA 一览",
        "",
        "| # | VA | path | 允许? |",
        "| --- | --- | --- | --- |",
    ]
    for r in opens:
        p = r["path_hint"] or "(未解析)"
        ok = "YES" if r["path_hint"] in ALLOW_OPEN else ("?" if not r["path_hint"] else "FLAG")
        walk.append(f"| {r['n']} | `0x{r['va']:x}` | {p} | **{ok}** |")

    walk.extend(
        [
            "",
            "## 全部 API 时间线（合同）",
            "",
            "| # | VA | API | path/size |",
            "| --- | --- | --- | --- |",
        ]
    )
    for r in call_rows:
        extra = r["path_hint"] or r["size_hint"] or ""
        walk.append(f"| {r['n']} | `0x{r['va']:x}` | {r['api']} | {extra} |")

    walk.extend(
        [
            "",
            "## API 计数",
            "",
            "| API | count |",
            "| --- | ---: |",
        ]
    )
    for k2, n in sorted(api_c.items()):
        walk.append(f"| {k2} | {n} |")

    walk.extend(["", "## 分场索引", "", "| VA | phase |", "| --- | --- |"])
    for va, title in phases:
        walk.append(f"| `0x{va:x}` | {title} |")

    walk.extend(["", "## 审核红旗（生成器）", ""])
    if flags:
        for f in flags:
            walk.append(f"- {f}")
    else:
        walk.append("- （无）路径允许 · 无 UNKNOWN_IAT · 无 BAN 串 · CALLSEQ EQUAL")

    walk.extend(["", "## PE 内相关字符串（抽查用）", ""])
    for s in dict.fromkeys(pe_strs):
        walk.append(f"- `{s}`")

    walk.extend(
        [
            "",
            "## 诚实",
            "",
            "- 标注 + 合同让 AI/人**抽查面**到极致，不是证明无逻辑后门",
            "- bins / plant 仍非「读完」对象",
            "",
        ]
    )
    OUT_WALK.write_text("\n".join(walk) + "\n", encoding="utf-8")

    # One-page AI brief
    opens_ok = all(r["path_hint"] in ALLOW_OPEN for r in opens) and len(opens) > 0
    no_unk = not any(r["api"].startswith("UNKNOWN") for r in call_rows)
    no_ban = not ban_hits
    no_flag = not flags
    ai = [
        "# STUB.AI — 一页审（优先于反汇编）",
        "",
        "> 自动生成。逐项勾完即可交差。≠ Thompson。",
        "",
        "## 结论槽",
        "",
        f"- opens_all_ALLOW: **{'PASS' if opens_ok else 'FAIL'}** ({len(opens)} CreateFileA)",
        f"- no_UNKNOWN_IAT: **{'PASS' if no_unk else 'FAIL'}**",
        f"- no_BAN_substr: **{'PASS' if no_ban else 'FAIL'}**"
        + (f" hits={ban_hits}" if ban_hits else ""),
        f"- CALLSEQ_pin_EQUAL: **{'PASS' if no_flag or not any('CALLSEQ' in f for f in flags) else 'FAIL'}**",
        f"- generator_flags: **{'PASS' if no_flag else 'FAIL'}** ({len(flags)})",
        "",
        f"**VERDICT: {'GREEN' if (opens_ok and no_unk and no_ban and no_flag) else 'RED'}**",
        "",
        "## 打开序列（合同）",
        "",
        f"`{open_seq}`",
        "",
        "## 必读分场（跳读）",
        "",
    ]
    for va, title in phases[:12]:
        ai.append(f"- `0x{va:x}` {title}")
    if len(phases) > 12:
        ai.append(f"- … +{len(phases) - 12} more in WALKTHROUGH")
    ai.extend(
        [
            "",
            "## 怎么用",
            "",
            "1. 看 VERDICT；红则停",
            "2. 扫打开序列是否只含允许名单",
            "3. 需要细节才开 WALKTHROUGH / 带 `; call#` 的 disasm 行",
            "",
        ]
    )
    OUT_AI.write_text("\n".join(ai) + "\n", encoding="utf-8")

    if any(r["api"].startswith("UNKNOWN") for r in call_rows):
        print("AUDITMAX status=RED reason=unknown_iat")
        raise SystemExit(1)
    if ban_hits:
        print(f"AUDITMAX status=RED reason=ban_substr hits={ban_hits}")
        raise SystemExit(1)
    if flags:
        print(f"AUDITMAX status=RED reason=flags n={len(flags)}")
        for f in flags:
            print(f"AUDITMAX flag={f}")
        raise SystemExit(1)

    print(f"AUDITMAX disasm={OUT_DISASM.relative_to(ROOT).as_posix()}")
    print(f"AUDITMAX walk={OUT_WALK.relative_to(ROOT).as_posix()} calls={len(call_rows)} opens={len(opens)}")
    print(f"AUDITMAX ai={OUT_AI.relative_to(ROOT).as_posix()}")
    print(f"AUDITMAX flags={len(flags)}")
    print("AUDITMAX status=GREEN")
    # keep HUMANAUDIT tag for stage script
    print(f"HUMANAUDIT disasm={OUT_DISASM.relative_to(ROOT).as_posix()}")
    print(f"HUMANAUDIT walk={OUT_WALK.relative_to(ROOT).as_posix()} calls={len(call_rows)} opens={len(opens)}")
    print(f"HUMANAUDIT flags={len(flags)}")
    print("HUMANAUDIT status=GREEN")


if __name__ == "__main__":
    main()
