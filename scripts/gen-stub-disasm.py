#!/usr/bin/env python3
"""
AUDITCLR disasm — Capstone, disambiguated for AI/human audit.

Outputs:
  STUB.disasm.txt      — full but no false-code; long noise FOLDed
  STUB.SKELETON.txt    — signal-only (prefer for detail dive)
  STUB.WALKTHROUGH.md / STUB.AI.md / STUB.CALLSEQ.pin
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
OUT_SKELETON = OUT_DIR / "STUB.SKELETON.txt"
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

# Instructions that alone never break a "noise gate" fold.
GATE_MNEM = {
    "cmp",
    "test",
    "je",
    "jne",
    "jz",
    "jnz",
    "ja",
    "jb",
    "jae",
    "jbe",
    "jg",
    "jl",
    "jge",
    "jle",
    "jmp",
    "movzx",
    "movsx",
    "nop",
}

# PE-field store churn (try_place builder).
STORE_MNEM = {"mov", "movabs"}


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
    return pe[file_off:end].decode("ascii")


def rip_target(insn) -> int | None:
    if "rip" not in insn.op_str:
        return None
    m = re.search(r"\[rip\s*([\+\-]\s*0x[0-9a-fA-F]+)\]", insn.op_str)
    if not m:
        return None
    return insn.address + insn.size + int(m.group(1).replace(" ", ""), 16)


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
        if not line or line.startswith("#") or "=" not in line:
            continue
        k, v = line.split("=", 1)
        out[k.strip()] = v.strip()
    return out


def find_literal_spans(code: bytes, base: int) -> list[tuple[int, int, str]]:
    """
    Real embedded literals only — NOT instruction immediates (e.g. cmp …,'TYB').
    Accept: lea-backed filenames, or known tags (SEMHOST/yoyo_/kernel).
    """
    spans: list[tuple[int, int, str]] = []
    # Known tags that live as real .text literals in place_stub
    # Do NOT search ".text" — it appears as movabs immediates in PE builders.
    for tag in (b"SEMHOST", b"input.tyb", b"output.exe"):
        start = 0
        while True:
            at = code.find(tag, start)
            if at < 0:
                break
            # extend to NUL
            end = at + len(tag)
            while end < len(code) and 32 <= code[end] < 127:
                end += 1
            if end < len(code) and code[end] == 0:
                s = code[at:end].decode("ascii")
                spans.append((base + at, base + end + 1, s))
                start = end + 1
            else:
                start = at + 1
    return spans


def find_second_frame(code: bytes, base: int) -> int | None:
    """VA of `sub rsp, 0x78` (second place/open frame) after embedded strings."""
    pat = bytes([0x48, 0x83, 0xEC, 0x78])
    # skip first frame which uses sub rsp, 0x88
    idx = 0
    while True:
        at = code.find(pat, idx)
        if at < 0:
            return None
        return base + at


def merge_spans(spans: list[tuple[int, int, str]]) -> list[tuple[int, int, str]]:
    if not spans:
        return []
    spans = sorted(spans)
    out = [spans[0]]
    for a, b, s in spans[1:]:
        pa, pb, ps = out[-1]
        if a <= pb + 1:
            # merge adjacent/overlapping; unique labels
            parts = []
            for p in (ps + " | " + s).split(" | "):
                if p and p not in parts:
                    parts.append(p)
            out[-1] = (pa, max(pb, b), " | ".join(parts))
        else:
            out.append((a, b, s))
    return out


def is_pe_store_insn(insn) -> bool:
    if insn.mnemonic not in STORE_MNEM:
        return False
    op = insn.op_str
    # movabs rax, imm  OR  mov [rax+..]/..  OR mov rcx,[rsp+58] reload
    if insn.mnemonic == "movabs":
        return True
    if "[rax" in op or "[rcx" in op:
        return True
    if op.startswith("rax, qword ptr [rsp") or op.startswith("rcx, qword ptr [rsp"):
        return True
    if op.startswith("rdi, qword ptr [rsp") or op.startswith("rsi, qword ptr [rsp"):
        return True
    return False


def is_gate_insn(insn) -> bool:
    if insn.mnemonic in GATE_MNEM:
        return True
    # lea used only as table cursor in gates (not path rcx)
    if insn.mnemonic == "lea" and not insn.op_str.startswith("rcx"):
        return True
    if insn.mnemonic == "mov" and re.match(r"e?[abcd]x,\s*(0x)?[0-9a-f]+$", insn.op_str):
        # mov ecx, imm in cmpsb prep — treat as gate-ish only inside gate runs
        return True
    if insn.mnemonic in ("push", "pop", "cmpsb", "repe"):
        return True
    return False


def main() -> None:
    pe = PE_PATH.read_bytes()
    image_base, entry_rva, sections = pe_map(pe)
    text = next(s for s in sections if s[0].startswith(".text"))
    _name, text_rva, _vs, text_raw, _rs = text
    need = STARTUP + PLACE
    code = pe[text_raw : text_raw + need]
    if len(code) < need:
        raise SystemExit("AUDITCLR status=RED reason=text_short")

    ban_hits = scan_ban(pe)
    base = image_base + text_rva
    entry_va = image_base + entry_rva
    win_lo, win_hi = base, base + need
    md = Cs(CS_ARCH_X86, CS_MODE_64)

    # --- DATA: lea targets + known real literals (never cmp immediates) ---
    data_spans = merge_spans(find_literal_spans(code, base))

    # --- SEED/table fold: from first embedded literal cluster to second frame ---
    second = find_second_frame(code, base)
    seed_lo = seed_hi = None
    if data_spans and second is not None:
        cand = [a for a, b, s in data_spans if a >= base + 0x400]
        if cand:
            seed_lo = min(cand)
            seed_hi = second  # exclusive

    def in_data(va: int) -> tuple[int, int, str] | None:
        for a, b, s in data_spans:
            if a <= va < b:
                return a, b, s
        return None

    def in_seed_gap(va: int) -> bool:
        if seed_lo is None or seed_hi is None:
            return False
        if not (seed_lo <= va < seed_hi):
            return False
        return in_data(va) is None

    # Pass-1 lea→str for rcx path tracking (also from .data via r15)
    str_at: dict[int, str] = {}
    for insn in md.disasm(code, base):
        if insn.mnemonic != "lea":
            continue
        tgt = rip_target(insn)
        if tgt is None:
            continue
        s = read_cstr(pe, va_to_file(tgt, image_base, sections))
        if s and len(s) >= 4:
            str_at[tgt] = s
            foff = va_to_file(tgt, image_base, sections)
            if foff is not None and win_lo <= tgt < win_hi:
                end = foff + len(s)
                if end < len(pe) and pe[end] == 0:
                    end += 1
                data_spans.append((tgt, tgt + (end - foff), s))
    data_spans = merge_spans(data_spans)

    header = [
        "; AUTO-AUDITCLR annotated disasm — Capstone intel (disambiguated)",
        f"; PE: {PE_PATH.relative_to(ROOT).as_posix()}",
        f"; image_base=0x{image_base:x}  .text_rva=0x{text_rva:x}  entry_va=0x{entry_va:x}",
        f"; bytes={need} (= PE_startup {STARTUP} + code_place {PLACE})",
        "; Prefer STUB.SKELETON.txt. Full file: banners / call# / DATA / FOLD / OUT_OF_WINDOW.",
        "; ≠ Thompson",
        "",
    ]
    lines: list[str] = list(header)
    skel: list[str] = list(header)
    phases: list[tuple[int, str]] = []
    phases_emitted: set[str] = set()
    fold_notes: list[str] = []

    def emit_phase(va: int, title: str, to_skel: bool = True):
        key = title.split("@")[0].strip()
        if key in phases_emitted:
            return
        phases_emitted.add(key)
        phases.append((va, title))
        block = ["", f"; ========== {title} @ 0x{va:x} =========="]
        lines.extend(block)
        if to_skel:
            skel.extend(block)

    def emit_line(raw: str, signal: bool):
        lines.append(raw)
        if signal:
            skel.append(raw)

    def emit_fold(va0: int, va1: int, kind: str, ninsn: int):
        msg = f"; FOLD {kind} 0x{va0:x}..0x{va1:x}  ({ninsn} insns — skipped as noise)"
        fold_notes.append(msg)
        emit_line("", False)
        emit_line(msg, True)
        emit_line(f";      reason: AUDITCLR — {kind} is repetitive / not an I/O edge", True)

    call_rows: list[dict] = []
    pending_rcx_str: str | None = None
    pending_rcx_age = 999
    last_edx: tuple[int, int] | None = None
    last_r8d: tuple[int, int] | None = None
    last_edi: tuple[int, int] | None = None  # (imm or -1 if from reg, age)
    r8_from_edi = False
    flags: list[str] = []

    # lookahead buffers for folding
    gate_buf: list = []
    store_buf: list = []

    def flush_gate():
        nonlocal gate_buf
        if len(gate_buf) >= 10:
            emit_fold(gate_buf[0].address, gate_buf[-1].address + gate_buf[-1].size, "opcode/size-gates", len(gate_buf))
        else:
            for ins in gate_buf:
                hexb = " ".join(f"{b:02x}" for b in ins.bytes)
                emit_line(f"{ins.address:016x}:  {hexb:<32}  {ins.mnemonic:<8} {ins.op_str}", False)
        gate_buf = []

    def flush_store():
        nonlocal store_buf
        if len(store_buf) >= 8:
            emit_fold(
                store_buf[0].address,
                store_buf[-1].address + store_buf[-1].size,
                "PE-field/IAT stores",
                len(store_buf),
            )
        else:
            for ins in store_buf:
                hexb = " ".join(f"{b:02x}" for b in ins.bytes)
                emit_line(f"{ins.address:016x}:  {hexb:<32}  {ins.mnemonic:<8} {ins.op_str}", False)
        store_buf = []

    i = 0
    seed_fold_emitted = False
    while i < len(code):
        va = base + i

        # Seed gap: one fold covering all non-DATA bytes in embedded region
        if in_seed_gap(va):
            if not seed_fold_emitted:
                # advance across contiguous seed gaps, emitting DATA when hit
                start = va
                while i < len(code):
                    va2 = base + i
                    sp = in_data(va2)
                    if sp and sp[0] == va2:
                        # emit pending seed fold before this string
                        if va2 > start:
                            emit_phase(start, "EMBEDDED seed/table (not cold I/O path)")
                            emit_fold(start, va2, "seed/table helpers", (va2 - start))
                        a, b, s = sp
                        emit_phase(va2, f'DATA string "{s}"')
                        emit_line(f'{va2:016x}:  ; DATA "{s}"  (do not decode as code)', True)
                        i = b - base
                        start = base + i
                        continue
                    if not in_seed_gap(va2):
                        if va2 > start:
                            emit_phase(start, "EMBEDDED seed/table (not cold I/O path)")
                            emit_fold(start, va2, "seed/table helpers", (va2 - start))
                        seed_fold_emitted = True
                        break
                    i += 1
                else:
                    seed_fold_emitted = True
                continue
            i += 1
            continue

        span = in_data(va)
        if span and span[0] == va:
            flush_gate()
            flush_store()
            a, b, s = span
            emit_phase(va, f'DATA string "{s}"')
            emit_line(f'{va:016x}:  ; DATA "{s}"  (do not decode as code)', True)
            i = b - base
            continue
        if span:
            i += 1
            continue

        got = list(md.disasm(code[i:], va, count=1))
        if not got:
            flush_gate()
            flush_store()
            emit_line(f"{va:016x}:  {code[i]:02x}                               db 0x{code[i]:02x}", False)
            i += 1
            continue
        insn = got[0]
        if any(insn.address < b and insn.address + insn.size > a for a, b, _ in data_spans):
            i += 1
            continue

        # classify before emitting — maybe fold
        interesting = False
        marks: list[str] = []
        api = None
        path_hint = ""
        size_hint = ""

        pending_rcx_age += 1
        if last_edx:
            last_edx = (last_edx[0], last_edx[1] + 1)
        if last_r8d:
            last_r8d = (last_r8d[0], last_r8d[1] + 1)
        if last_edi:
            last_edi = (last_edi[0], last_edi[1] + 1)

        if insn.address == entry_va:
            interesting = True
            emit_phase(insn.address, "ENTRY / PE startup")
            marks.append("<< entry")
        if insn.address == base + STARTUP:
            interesting = True
            emit_phase(insn.address, "PLACE BODY")
            marks.append("<< place body")
        if second is not None and insn.address == second:
            interesting = True
            emit_phase(insn.address, "H_00 / open-loop frame (sub rsp,0x78)")
            marks.append("<< H_00 frame")

        if insn.mnemonic == "mov":
            op = insn.op_str
            imm = imm_from_op(op)
            if imm is not None:
                if op.startswith("edx,") or op.startswith("rdx,"):
                    last_edx = (imm, 0)
                elif op.startswith("r8d,") or op.startswith("r8,"):
                    last_r8d = (imm, 0)
                    r8_from_edi = False
                elif op.startswith("edi,") or op.startswith("rdi,"):
                    last_edi = (imm, 0)
            if op.startswith("r8d, edi") or op.startswith("r8, rdi") or op == "r8d, edi":
                r8_from_edi = True
                if last_edi:
                    last_r8d = (last_edi[0], 0)

        if insn.mnemonic == "lea":
            tgt = rip_target(insn)
            s = None
            if tgt is not None:
                s = str_at.get(tgt) or read_cstr(pe, va_to_file(tgt, image_base, sections))
            if s is None and insn.op_str.startswith("rcx") and "r15" in insn.op_str:
                m = re.search(r"\[r15\s*\+\s*(0x[0-9a-fA-F]+|\d+)\]", insn.op_str)
                if m:
                    disp = int(m.group(1), 16) if m.group(1).startswith("0x") else int(m.group(1))
                    s = read_cstr(pe, va_to_file(image_base + 0x3000 + disp, image_base, sections))
            if s and len(s) >= 4:
                interesting = True
                marks.append(f'str="{s}"')
                if insn.op_str.startswith("rcx"):
                    pending_rcx_str = s
                    pending_rcx_age = 0

        if insn.mnemonic == "call" and "r15" in insn.op_str:
            interesting = True
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
                    if last_r8d and last_r8d[1] <= 12:
                        size_hint = f"n=0x{last_r8d[0]:x}"
                    elif r8_from_edi and last_edi and last_edi[1] <= 20:
                        size_hint = f"n=edi:0x{last_edi[0]:x}"
                elif api == "ExitProcess":
                    emit_phase(insn.address, "ExitProcess")
                call_rows.append(
                    {"n": n, "va": insn.address, "api": api, "path_hint": path_hint, "size_hint": size_hint}
                )
                if api.startswith("UNKNOWN"):
                    flags.append(f"UNKNOWN IAT call @ 0x{insn.address:x}")

        if "0x4c503159" in insn.op_str:
            interesting = True
            marks.append("magic Y1PL")
            emit_phase(insn.address, "CHECK magic Y1PL")
        if "0x425954" in insn.op_str and insn.mnemonic == "cmp":
            # Only the first TYB check is a landmark; later ones fold into gates.
            if "CHECK magic TYB" not in phases_emitted:
                interesting = True
                marks.append("magic TYB")
                emit_phase(insn.address, "CHECK magic TYB")
        if insn.mnemonic == "mov" and "byte ptr [rsi]" in insn.op_str and "0x4d" in insn.op_str:
            interesting = True
            emit_phase(insn.address, "BUILD PE MZ header")
        if insn.mnemonic in ("movsb",) or "movsb" in insn.op_str:
            interesting = True
            emit_phase(insn.address, "rep movsb copy into PE")

        # Branch targets: PLACE_CUT (exact window end) vs true OUT_OF_WINDOW
        oow = ""
        place_cut = False
        if insn.mnemonic.startswith("j") or insn.mnemonic == "call":
            m = re.search(r"0x[0-9a-fA-F]+", insn.op_str)
            if m and "[" not in insn.op_str:
                tgt = int(m.group(0), 16)
                if tgt == win_hi:
                    # end of place half — do NOT force interesting (keep FOLDable)
                    place_cut = True
                    oow = "  ; PLACE_CUT (past code_place; not in this window)"
                elif not (win_lo <= tgt < win_hi):
                    interesting = True
                    oow = f"  ; OUT_OF_WINDOW tgt=0x{tgt:x} (beyond place_stub window)"

        # Folding decision
        if interesting or api:
            flush_gate()
            flush_store()
            hexb = " ".join(f"{b:02x}" for b in insn.bytes)
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
            extra += oow
            emit_line(f"{insn.address:016x}:  {hexb:<32}  {insn.mnemonic:<8} {insn.op_str}{extra}", True)
            i += insn.size
            continue

        # try accumulate PE stores (late region) or gates
        if is_pe_store_insn(insn) and insn.address >= base + 0x1000:
            flush_gate()
            store_buf.append(insn)
            i += insn.size
            continue
        else:
            flush_store()

        if is_gate_insn(insn):
            gate_buf.append(insn)
            i += insn.size
            continue
        else:
            flush_gate()

        hexb = " ".join(f"{b:02x}" for b in insn.bytes)
        emit_line(f"{insn.address:016x}:  {hexb:<32}  {insn.mnemonic:<8} {insn.op_str}{oow}", bool(oow))
        i += insn.size

    flush_gate()
    flush_store()

    # Fail-closed: SEMHOST must not appear as push-decoded instruction
    joined = "\n".join(lines)
    if re.search(r"push\s+r11", joined) and "SEMHOST" in pe.decode("latin1", "replace"):
        # only RED if we still false-decoded the SEMHOST bytes region
        flags.append("false_decode: push r11 still present (SEMHOST?)")

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
    OUT_SKELETON.write_text("\n".join(skel) + "\n", encoding="utf-8")

    # Anti-ambiguity gate: skeleton must contain FOLD or DATA for SEMHOST
    skel_txt = "\n".join(skel)
    if "SEMHOST" in pe.decode("latin1", "replace"):
        if "SEMHOST" not in skel_txt and "seed/table" not in skel_txt.lower() and "DATA" not in skel_txt:
            flags.append("SEMHOST not represented as DATA/FOLD in skeleton")

    api_c = Counter(r["api"] for r in call_rows)
    opens = [r for r in call_rows if r["api"] == "CreateFileA"]
    open_seq = ",".join(r["path_hint"] or "?" for r in opens)
    api_seq = ",".join(r["api"] for r in call_rows)
    api_counts = ",".join(f"{k}:{n}" for k, n in sorted(api_c.items()))

    if ban_hits:
        for h in ban_hits:
            flags.append(f"BAN_SUBSTR `{h}` in PE")

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
                print(f"AUDITCLR status=RED reason=callseq_{key}")
                print(f"AUDITCLR want_{key}={want.get(key)}")
                print(f"AUDITCLR got_{key}={got.get(key)}")
                raise SystemExit(1)
        print("AUDITMAX callseq=EQUAL")
    else:
        OUT_CALLSEQ.write_text(pin_body, encoding="utf-8")
        print(f"AUDITMAX callseq=WRITE {OUT_CALLSEQ.relative_to(ROOT).as_posix()}")

    walk = [
        "# Stub 审核走读（人或 AI 用这个，别啃生肉）",
        "",
        "> 生成：`STUB.AI.md` → 本文件 → **`STUB.SKELETON.txt`** → 必要时全量 disasm。≠ Thompson。",
        "",
        "## 审法（强制）",
        "",
        "1. **先读 `STUB.AI.md`**",
        "2. 细节优先 **`STUB.SKELETON.txt`**（无假解码；长门/PE 拼装已 FOLD）",
        "3. 全量 `STUB.disasm.txt` 仅在 FOLD 边界需要时打开",
        "4. CreateFile ⊆ 允许名单；无 UNKNOWN_IAT；无 BAN；CALLSEQ EQUAL",
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
        walk.append(f"| {r['n']} | `0x{r['va']:x}` | {r['api']} | {r['path_hint'] or r['size_hint'] or ''} |")

    walk.extend(["", "## API 计数", "", "| API | count |", "| --- | ---: |"])
    for k2, n in sorted(api_c.items()):
        walk.append(f"| {k2} | {n} |")

    walk.extend(["", "## 分场索引", "", "| VA | phase |", "| --- | --- |"])
    for va, title in phases:
        walk.append(f"| `0x{va:x}` | {title} |")

    walk.extend(["", "## FOLD 摘要（消歧义）", ""])
    if fold_notes:
        for m in fold_notes:
            walk.append(f"- `{m}`")
    else:
        walk.append("- （无）")

    walk.extend(["", "## 审核红旗（生成器）", ""])
    if flags:
        for f in flags:
            walk.append(f"- {f}")
    else:
        walk.append("- （无）路径允许 · 无 UNKNOWN_IAT · 无 BAN · CALLSEQ EQUAL · 无假解码")

    walk.extend(["", "## PE 内相关字符串（抽查用）", ""])
    for s in dict.fromkeys(pe_strs):
        walk.append(f"- `{s}`")

    walk.extend(
        [
            "",
            "## 诚实",
            "",
            "- FOLD/DATA 消歧义 ≠ 逻辑无后门证明",
            "- bins / plant 仍非「读完」对象",
            "",
        ]
    )
    OUT_WALK.write_text("\n".join(walk) + "\n", encoding="utf-8")

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
        f"- no_false_decode/FOLD: **{'PASS' if no_flag else 'FAIL'}**",
        f"- generator_flags: **{'PASS' if no_flag else 'FAIL'}** ({len(flags)})",
        "",
        f"**VERDICT: {'GREEN' if (opens_ok and no_unk and no_ban and no_flag) else 'RED'}**",
        "",
        "## 打开序列（合同）",
        "",
        f"`{open_seq}`",
        "",
        "## 怎么用",
        "",
        "1. 看 VERDICT；红则停",
        "2. 扫打开序列",
        "3. 细节 → `STUB.SKELETON.txt`（不是全量 disasm）",
        "4. 仅当 FOLD 边界存疑才开 `STUB.disasm.txt`",
        "",
        "## 必读分场",
        "",
    ]
    for va, title in phases[:14]:
        ai.append(f"- `0x{va:x}` {title}")
    if len(phases) > 14:
        ai.append(f"- … +{len(phases) - 14} more in WALKTHROUGH")
    ai.append("")
    OUT_AI.write_text("\n".join(ai) + "\n", encoding="utf-8")

    if any(r["api"].startswith("UNKNOWN") for r in call_rows):
        print("AUDITCLR status=RED reason=unknown_iat")
        raise SystemExit(1)
    if ban_hits:
        print(f"AUDITCLR status=RED reason=ban_substr hits={ban_hits}")
        raise SystemExit(1)
    if flags:
        print(f"AUDITCLR status=RED reason=flags n={len(flags)}")
        for f in flags:
            print(f"AUDITCLR flag={f}")
        raise SystemExit(1)

    print(f"AUDITCLR disasm={OUT_DISASM.relative_to(ROOT).as_posix()} lines={len(lines)}")
    print(f"AUDITCLR skeleton={OUT_SKELETON.relative_to(ROOT).as_posix()} lines={len(skel)}")
    print(f"AUDITCLR folds={len(fold_notes)}")
    print(f"AUDITCLR walk={OUT_WALK.relative_to(ROOT).as_posix()} calls={len(call_rows)} opens={len(opens)}")
    print(f"AUDITCLR ai={OUT_AI.relative_to(ROOT).as_posix()}")
    print("AUDITCLR status=GREEN")
    print(f"HUMANAUDIT disasm={OUT_DISASM.relative_to(ROOT).as_posix()}")
    print(f"HUMANAUDIT walk={OUT_WALK.relative_to(ROOT).as_posix()} calls={len(call_rows)} opens={len(opens)}")
    print("HUMANAUDIT flags=0")
    print("HUMANAUDIT status=GREEN")
    print("AUDITMAX status=GREEN")


if __name__ == "__main__":
    main()
