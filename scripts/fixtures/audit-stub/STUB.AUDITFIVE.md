# STUB.AUDITFIVE — residual five

Gate: `scripts/stage-auditfive.ps1`

| Path | Status | Evidence |
| ---- | ------ | -------- |
| F1 collude_pins | HARD_BLOCK | pin/CI co-rewrite = Trusting Trust; not engineer-zeroable |
| F2 semantic_in_allow_io | CLOSED | plantpoison + y1plpin + binspoison poison→RED |
| F3 unwatched_allow_basenames | CLOSED | scan fixtures+payload for allow basenames vs pins |
| F4 os_loader_hardware | HARD_BLOCK | below A1 |
| F5 hash_collision | HARD_BLOCK | dual sha256+sha512 present; crypto break still HARD_BLOCK |

Pins: `STUB.DUALHASH.pin` · scripts: `verify-dualhash-pin.js`, `verify-live-scan-allow-pin.js`, `verify-auditfive-matrix.js`

≠ Thompson.
