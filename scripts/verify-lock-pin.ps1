# verify-lock-pin.ps1 - weekly Lock pin monitor (Decision #13 / PROMPT #18)
# Fail-closed when yoyo.ty drifts from yoyo/tests/yoyo.ty.lock (no auto-relock).
# Prefer release yoyo.exe (no cargo) when present - avoids build-dir races.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"

Write-Host "== Lock pin (node) =="
node scripts\verify-yoyo-ty.mjs
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }

Write-Host "== Lock pin (verifier test lock) =="
$code = 1
if (Test-Path $Yoyo) {
    Write-Host "using release yoyo.exe (no cargo)"
    & $Yoyo test lock
    $code = $LASTEXITCODE
    if ($null -eq $code) { $code = 0 }
} elseif ($SkipBuild) {
    Write-Host "verify-lock-pin: RED (yoyo.exe missing and -SkipBuild)"
    exit 1
} else {
    Write-Host "yoyo.exe missing; one serial cargo run --release -- test lock"
    Push-Location (Join-Path $Root "yoyo-rust\verifier")
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    try {
        cargo run --release -- test lock 2>&1 | Out-Host
        $code = $LASTEXITCODE
        if ($null -eq $code) { $code = 0 }
    } finally {
        $ErrorActionPreference = $prevEap
        Pop-Location
    }
}
if ($code -ne 0) { exit $code }

Write-Host "verify-lock-pin: PASS"
exit 0
