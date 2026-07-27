### §3.6 H_38 SET (0x30) CONTROL — PASS
fixture_arg: 30 50 00  (SET slot=0x50 imm=0x00, mirrors H_03 in yoyo.ty)
hand_derived: 48b8000000000000000049898780020000c3 sha256=196cd779c54c77017bbe3cf5b5220d01425bfeed4be21897754cab746f8ae7c0
js_actual:    48b8000000000000000049898780020000c3 sha256=196cd779c54c77017bbe3cf5b5220d01425bfeed4be21897754cab746f8ae7c0
rust_actual:  48b8000000000000000049898780020000c3 sha256=196cd779c54c77017bbe3cf5b5220d01425bfeed4be21897754cab746f8ae7c0
byte_equal: Y
byte_length: 18B
stop_if_fired: NONE
proc_calls: 10
note: control — no regression vs. existing 0x30 handler in yoyo.ty (H_03 slot=0x50/imm=0x00 emit path identical). Rust output stripped of 1-byte bare-metal 0xC3 startup prefix for byte-equal comparison.