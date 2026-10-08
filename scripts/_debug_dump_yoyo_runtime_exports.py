"""Dump export table of yoyo_runtime.dll directly using known export-dir file offset."""
import struct
from pathlib import Path

path = Path(r"F:\yoyo\yoyo-rust\target\release-runtime\yoyo_runtime.dll")
data = path.read_bytes()

def cstr(off):
    end = data.index(b"\0", off)
    return data[off:end].decode("utf-8", errors="replace")

# export dir directly at file offset 0x3a4e0 (already converted by dump dir)
exp_off = 0x3a4e0
print(f"file={path.name} size={len(data)} export_dir_at_file_off=0x{exp_off:x}")
export_name_rva = struct.unpack_from("<I", data, exp_off+4)[0]
num_funcs = struct.unpack_from("<I", data, exp_off+20)[0]
num_names = struct.unpack_from("<I", data, exp_off+24)[0]
addr_funcs_rva = struct.unpack_from("<I", data, exp_off+28)[0]
addr_names_rva = struct.unpack_from("<I", data, exp_off+32)[0]
addr_ordrva_rva = struct.unpack_from("<I", data, exp_off+36)[0]
print(f"export_name_rva=0x{export_name_rva:x} num_funcs={num_funcs} num_names={num_names}")
print(f"funcs_table_rva=0x{addr_funcs_rva:x} names_table_rva=0x{addr_names_rva:x} ordinals_rva=0x{addr_ordrva_rva:x}")

# We need to map these RVAs to file offsets. Do it by using the same section walk.
pe_off = struct.unpack_from("<I", data, 0x3C)[0]
opt_off = pe_off + 24
opt_size = struct.unpack_from("<H", data, pe_off+20)[0]
sec_off = opt_off + opt_size
sec_cnt = struct.unpack_from("<H", data, pe_off+6)[0]

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

addr_funcs = rva_to_off(addr_funcs_rva)
addr_names = rva_to_off(addr_names_rva)
addr_ord = rva_to_off(addr_ordrva_rva)
export_name_off = rva_to_off(export_name_rva)
print(f"-> file offsets: funcs=0x{addr_funcs:x} names=0x{addr_names:x} ords=0x{addr_ord:x} export_name=0x{export_name_off:x}")
print(f"export dll name = {cstr(export_name_off)}")
print(f"\n--- Named exports ({num_names}) ---")
for i in range(num_names):
    name_rva = struct.unpack_from("<I", data, addr_names + i*4)[0]
    name_off = rva_to_off(name_rva)
    ordi = struct.unpack_from("<H", data, addr_ord + i*2)[0]
    func_rva = struct.unpack_from("<I", data, addr_funcs + ordi*4)[0]
    print(f"  [{i}] ord={ordi} rva=0x{func_rva:x} name={cstr(name_off)}")
