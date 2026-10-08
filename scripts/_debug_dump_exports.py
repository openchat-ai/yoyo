"""Check export dir [0] of each sidecar DLL; dump names if present."""
import struct
from pathlib import Path

def probe(data, label):
    print(f"=== {label} ({len(data)} bytes) ===")
    if data[:2] != b"MZ":
        print("  not MZ"); return
    pe_off = struct.unpack_from("<I", data, 0x3C)[0]
    if data[pe_off:pe_off+4] != b"PE\0\0":
        print(f"  no PE sig at 0x{pe_off:x}"); return
    opt_off = pe_off + 24
    magic = struct.unpack_from("<H", data, opt_off)[0]
    is64 = magic == 0x20B
    print(f"  is64={is64} pe_off=0x{pe_off:x} opt_off=0x{opt_off:x}")
    if is64:
        dd_base = opt_off + 112
    else:
        dd_base = opt_off + 96
    exp_rva = struct.unpack_from("<I", data, dd_base + 0)[0]
    exp_size = struct.unpack_from("<I", data, dd_base + 4)[0]
    print(f"  exp_dir[0]: rva=0x{exp_rva:x} size=0x{exp_size:x}")
    if exp_rva == 0:
        print("  (no export dir — H_00 loader would not find any exports)")
        return
    # section table
    sec_cnt = struct.unpack_from("<H", data, pe_off + 6)[0]
    opt_size = struct.unpack_from("<H", data, pe_off + 20)[0]
    sec_off = opt_off + opt_size
    secs = []
    for i in range(sec_cnt):
        s = sec_off + i*40
        vs = struct.unpack_from("<I", data, s+8)[0]
        vsize = struct.unpack_from("<I", data, s+0xC)[0]
        ro = struct.unpack_from("<I", data, s+0x14)[0]
        rs = struct.unpack_from("<I", data, s+0x18)[0]
        nm = data[s:s+8].rstrip(b"\0").decode("ascii", errors="replace")
        secs.append((nm, vs, vsize, ro, rs))
    def rva2off(rva):
        for nm,vs,vsize,ro,rs in secs:
            if vs <= rva < vs + max(vsize, rs):
                return ro + (rva - vs)
        return None
    eo = rva2off(exp_rva)
    if eo is None or eo + 40 > len(data):
        print(f"  exp rva 0x{exp_rva:x} -> off 0x{eo} (out of file?)"); return
    num_funcs = struct.unpack_from("<I", data, eo + 20)[0]
    num_names = struct.unpack_from("<I", data, eo + 24)[0]
    ert_rva = struct.unpack_from("<I", data, eo + 16)[0]
    ent_rva = struct.unpack_from("<I", data, eo + 20)[0]
    nrt_rva = struct.unpack_from("<I", data, eo + 24)[0]
    print(f"  num_funcs={num_funcs} num_names={num_names}")
    ert = rva2off(ert_rva); nrt = rva2off(nrt_rva); ent = rva2off(ent_rva)
    name_map = {}
    if nrt and num_names:
        for i in range(num_names):
            nr = struct.unpack_from("<I", data, nrt + i*4)[0]
            no = rva2off(nr)
            end = data.index(b"\0", no)
            nm = data[no:end].decode("utf-8", errors="replace")
            ord_idx = struct.unpack_from("<H", data, ent + i*2)[0]
            name_map[ord_idx] = nm
    for i in range(num_funcs):
        fr = struct.unpack_from("<I", data, ert + i*4)[0]
        nm = name_map.get(i, f"(no name)")
        print(f"    ord {i}: {nm:40s} rva=0x{fr:x}")

probe(Path(r"F:\yoyo\yoyo-rust\target\release-runtime\yoyo_runtime.dll").read_bytes(), "RUST release-runtime yoyo_runtime.dll")
print()
probe(Path(r"F:\yoyo\yoyo-rust\target\release\yoyo_runtime.dll").read_bytes(), "RUST release yoyo_runtime.dll")
print()
probe(Path(r"F:\yoyo\scripts\_stage17-ow-rt-in-dll-recompile\yoyo_rt.dll").read_bytes(), "YOYO in-DLL-recompile yoyo_rt.dll")
