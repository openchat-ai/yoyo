'use strict';
/** Phase 2 root-cause fix: data section floor must match finish()/PE template. */
const OUTPUT_DATA_NEED = 0x10000 + 0x8000 + 0x20000; // = 0x38000
const STATE_BUF_OFF = 0x8000;
const TEXT_RVA = 0x1000;
const IMAGE_BASE = 0x140000000n;

module.exports = { OUTPUT_DATA_NEED, STATE_BUF_OFF, TEXT_RVA, IMAGE_BASE };
