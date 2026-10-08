"""Dump yoyo_rt.dll imports from gen4.exe — count & list required exports."""
import struct, sys
from pathlib import Path

EXE = Path(r"F:\yoyo\scripts\_stage9-pure-m4\gen4.exe")
DLL_TARGET = "YOYO_RT.DLL"

data = EXE.read_bytes()
if data[:2] != b"MZ":
    print("not MZ"); sys.exit(1)
pe_off = struct.unpack_from("<I", data, 0x3C)[0]
if data[pe_off:pe_off+4] != b"PE\0\0":
    print("no PE"); sys.exit(1)
opt_off = pe_off + 24
magic = struct.unpack_from("<H", data, opt_off)[0]
is64 = magic == 0x20B
if is64:
    import_rva_off = opt_off + 112
    import_size_off = opt_off + 116
else:
    import_rva_off = opt_off + 88
    import_size_off = opt_off + 92
imp_rva = struct.unpack_from("<I", data, import_rva_off)[0]
imp_size = struct.unpack_from("<I", data, import_size_off)[0]

# section table
sec_cnt = struct.unpack_from("<H", data, pe_off + 6)[0]
opt_size = struct.unpack_from("<H", data, pe_off + 20)[0]
sec_off = opt_off + opt_size

def rva_to_off(rva):
    for i in range(sec_cnt):
        s = sec_off + i*40
        vs = struct.unpack_from("<I", data, s+8)[0]
        vsize = struct.unpack_from("<I", data, s+0xC)[0]
        raw_off = struct.unpack_from("<I", data, s+0x14)[0]
        raw_size = struct.unpack_from("<I", data, s+0x18)[0]
        if vs <= rva < vs + max(vsize, raw_size):
            return raw_off + (rva - vs)
    return None

def cstr(off):
    end = data.index(b"\0", off)
    return data[off:end].decode("utf-8", errors="replace")

print(f"is64={is64} sec_cnt={sec_cnt} imp_rva=0x{imp_rva:x} imp_size={imp_size}")
imp_off = rva_to_off(imp_rva)
if imp_off is None:
    print("import dir rva -> off failed"); sys.exit(1)

# IMAGE_IMPORT_DESCRIPTOR: 20 bytes, null terminator
imports = []
off = imp_off
i = 0
while True:
    name_rva = struct.unpack_from("<I", data, off + 12)[0]
    ilt_rva = struct.unpack_from("<I", data, off + 0)[0]
    thk_rva = struct.unpack_from("<I", data, off + 8)[0]
    if name_rva == 0 and ilt_rva == 0 and thk_rva == 0:
        break
    name_off = rva_to_off(name_rva)
    dll_name = cstr(name_off).upper()
    entries = []
    if ilt_rva:
        o = rva_to_off(ilt_rva)
        while True:
            if is64:
                val = struct.unpack_from("<Q", data, o)[0]
                hint_name_rva = val & 0x7FFFFFFF
                is_by_ord = bool(val & 0x80000000)
            else:
                val = struct.unpack_from("<I", data, o)[0]
                hint_name_rva = val & 0x7FFFFFFF
                is_by_ord = bool(val & 0x80000000)
            if hint_name_rva == 0:
                break
            if is_by_ord:
                entries.append((f"ord{hint_name_rva}", True))
            else:
                ho = rva_to_off(hint_name_rva)
                entries.append((cstr(ho + 2), False))  # +2 skip hint word
            o += 8 if is64 else 4
    imports.append((dll_name, entries))
    off += 20
    i += 1

print()
total_funcs = 0
for dll, entries in imports:
    print(f"  {dll:30s}  n_funcs={len(entries)}")
    total_funcs += len(entries)
    if dll == DLL_TARGET:
        print(f"  == {dll} exports required ({len(entries)}) ==")
        for name, by_ord in entries:
            tag = " [by_ord]" if by_ord else ""
            print(f"    - {name}{tag}")

print()
print(f"TOTAL_DLLS={len(imports)} TOTAL_FUNCS={total_funcs}")
for dll, entries in imports:
    if dll == DLL_TARGET:
        print(f"YOYO_RT_DLL_REQUIRED_EXPORTS={len(entries)}")
