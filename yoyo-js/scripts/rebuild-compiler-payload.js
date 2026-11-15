'use strict';
/**
 * RETIRED (AUTO-NO-PLANT-ESCAPE).
 * Always REFUSE. Operational regen: scripts/regen-payload-from-c1.ps1
 * No YOYO_ALLOW_NODE_PLANT escape.
 */
console.log('NODE_PLANT status=ABSENT');
console.log('NODE_PLANT reason=retired_no_escape');
console.log('NODE_PLANT hint=regen_via_c1: & .\\scripts\\regen-payload-from-c1.ps1');
console.log('NODE_PLANT escape=ABSENT');
console.log('NODE_PLANT doc=AUTO-NO-PLANT-ESCAPE.md');
process.exit(2);
