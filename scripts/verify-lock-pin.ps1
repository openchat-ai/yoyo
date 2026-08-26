# verify-lock-pin.ps1 — weekly Lock pin monitor (Decision #13 / PROMPT #18)
# Fail-closed when yoyo.ty drifts from yoyo/tests/yoyo.ty.lock (no auto-relock).
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "== Lock pin (node) =="
node scripts\verify-yoyo-ty.mjs
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "== Lock pin (verifier test lock) =="
Push-Location yoyo-rust\verifier
$prevEap = $ErrorActionPreference
$ErrorActionPreference = 'Continue'
try {
  cargo run -- test lock 2>&1 | Out-Host
  $code = $LASTEXITCODE
} finally {
  $ErrorActionPreference = $prevEap
}
Pop-Location
if ($code -ne 0) { exit $code }

Write-Host "verify-lock-pin: PASS"
