"""Dump export table of any Windows PE."""
import struct, sys
from pathlib import Path

path = Path(sys.argv[1])
data = path.read_bytes()
if data[:2] != b"MZ":
    print("not MZ"); sys.exit(1)
pe_off = struct.unpack_from("<I", data, 0x3C)[0]
if data[pe_off:pe_off+4] != b"PE\0\0":
    print("no PE"); sys.exit(1)
opt_off = pe_off + 24
magic = struct.unpack_from("<H", data, opt_off)[0]
is64 = magic == 0x20B
exp_rva_off = opt_off + 112 if is64 else opt_off + 88  # data dir[0]
exp_rva = struct.unpack_from("<I", data, exp_rva_off)[0]
exp_size = struct.unpack_from("<I", data, exp_rva_off + 4)[0]
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

print(f"file={path.name} size={len(data)} is64={is64} export_dir_rva=0x{exp_rva:x} size={exp_size}")
if exp_rva == 0:
    print("(no export dir)"); sys.exit(0)
exp_off = rva_to_off(exp_rva)
num_funcs = struct.unpack_from("<I", data, exp_off + 20)[0]
num_names = struct.unpack_from("<I", data, exp_off + 24)[0]
addr_funcs = rva_to_off(struct.unpack_from("<I", data, exp_off + 28)[0])
addr_names = rva_to_off(struct.unpack_from("<I", data, exp_off + 32)[0])
addr_ordrva = rva_to_off(struct.unpack_from("<I", data, exp_off + 36)[0])
print(f"num_funcs={num_funcs} num_names={num_names}")
print("--- Named exports ---")
for i in range(num_names):
    name_rva = struct.unpack_from("<I", data, addr_names + i*4)[0]
    name_off = rva_to_off(name_rva)
    ordi = struct.unpack_from("<H", data, addr_ordrva + i*2)[0]
    func_rva = struct.unpack_from("<I", data, addr_funcs + ordi*4)[0]
    print(f"  [{i}] ord={ordi} rva=0x{func_rva:x} name={cstr(name_off)}")
