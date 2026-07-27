'use strict';
/** ELF builder stub — Linux peer (Phase 4). Full ELF64 deferred; emits flat blob. */
function buildElf(code, data) {
  const buf = Buffer.concat([Buffer.from([0x7f, 0x45, 0x4c, 0x46]), code, data]);
  return buf;
}
module.exports = { buildElf };
