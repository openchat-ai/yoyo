# Emit rules (yoyo-js)

- All arithmetic/control/memory ops go through `encode-x64.js`
- Syscalls (0x20/0x50/0x51) resolved by platform backend at link time
- `OUTPUT_DATA_NEED = 0x38000` must match PE data VS (Phase 2 fix)
- Never embed IAT / PE magic / syscall numbers into `yoyo.ty`
