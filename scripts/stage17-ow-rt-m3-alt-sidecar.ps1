# stage17-ow-rt-m3-alt-sidecar.ps1 — S3.527 / M3 ALT_SIDECAR=in-dll trial
# place_cwd writes YOYO M1 in-DLL DLL (no Rust); H_00 + mini RET → exit 0.
# Production default remains Rust (env unset). OW-RT still CUT.
#
#   cd F:\yoyo
#   & .\scripts\stage17-ow-rt-m3-alt-sidecar.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Post-v1.0: OW-RT M3 ALT_SIDECAR=in-dll trial ==="

$Work = Join-Path $Root "scripts\_stage17-ow-rt-m3-alt-sidecar"
New-Item -ItemType Directory -Force -Path $Work | Out-Null

$prev = $env:YOYO_OW_RT_ALT_SIDECAR
$env:YOYO_OW_RT_ALT_SIDECAR = "in-dll"
try {
    Push-Location (Join-Path $Root "yoyo-rust")
    try {
        $prevEap = $ErrorActionPreference
        $ErrorActionPreference = "Continue"
        Write-Host "=== emit-rt-sidecar --m2-h00-subset under ALT=in-dll ==="
        # M2 helper places its own in-DLL DLL; pin proves place_cwd path via unit test.
        # Script: run H_00 subset (same contract) + print env mode.
        & cargo run -q -p verifier --bin emit-rt-sidecar --no-default-features --features full-backends -- --m2-h00-subset $Work
        $ec = $LASTEXITCODE
        $ErrorActionPreference = $prevEap
    } finally {
        Pop-Location
    }
} finally {
    if ($null -eq $prev) { Remove-Item Env:YOYO_OW_RT_ALT_SIDECAR -ErrorAction SilentlyContinue }
    else { $env:YOYO_OW_RT_ALT_SIDECAR = $prev }
}

$Sidecar = Join-Path $Work "yoyo_rt.dll"
$Out = Join-Path $Work "output.exe"
$hasOut = Test-Path $Out
$hasSidecar = Test-Path $Sidecar
Write-Host ("OW_RT_M3_ALT_SIDECAR exit={0} sidecar={1} output={2} env_trial=in-dll" -f $ec, $hasSidecar, $hasOut)

if ($ec -eq 0 -and $hasOut -and $hasSidecar) {
    Write-Host "OW_RT_M3_ALT_SIDECAR status=GREEN contract=mini_only production_default=RUST disposition=CUT"
    exit 0
}
Write-Host ("OW_RT_M3_ALT_SIDECAR status=RED reason=alt_in_dll_path exit={0}" -f $ec)
exit 1
