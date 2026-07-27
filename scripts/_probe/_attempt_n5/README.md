# scripts/_probe/_attempt_n5 — N5 runtime canary (FAIL-CLOSED)

> Status: **EXPERIMENTAL · FAIL-CLOSED · pre-flight blocked**. See
> `docs/auxdocs/selfhost-attempt-N5-log.md` for the full report.

## What's here

- `_preflight.json` — record of the four pre-flight scans that
  established neither peer has a bytecode executor.
- (no fixtures, no runners, no byte streams — they would have been
  produced only if the pre-flight had a positive answer.)

## How to reproduce

The pre-flight is just four ripgrep / file-read steps; the JSON captures
their outputs. The log reproduces them inline.

The canary itself cannot be reproduced because the runner does not exist
in this repo. Attempting `cargo run -p verifier --bin yoyo -- run …`
prints `unknown command 'run'` (the CLI matrix is
`link|diff|hash|selftest|render|test`), and the JS peer has no
`vm.runInContext(emitBytes, sandbox)` path on its encode output.

## Verdict

- 0/3 canaries executed (per canary).
- Reason: no runtime. Fail-closed per the brief's "any … run error →
  STOP and report".
- D-1 unchanged: not resolved, not aggravated.
- See the linked log for the suggested next direction.
