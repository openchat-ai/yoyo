# stage17-s2-runtime-effect-slim.ps1 — S2.6 / S3.103 acceptance: slim real I/O e2e
# (post-v1.0 Stage 17+ local track; NOT ROADMAP Stage 17)
#
# Runs TYB slim PE real-I/O e2e:
#   - host manual-map (S3.99–S3.102): ALLOC bind + LOAD + WRITE + roundtrip
#   - in-DLL recompile → manual-map (S3.104–S3.107): ALLOC/LOAD/WRITE/roundtrip
# StubPlatform / FlatBinary / H_00 self-test lane stays stub.
# Honest: OW-RT remains CUT; not CLOSED.
#
#   cd F:\yoyo
#   & .\scripts\stage17-s2-runtime-effect-slim.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Post-v1.0 S2.6/S3.108: slim + in-DLL real I/O e2e (fail-closed) ==="
Write-Host "note: StubPlatform stays stub; slim PE IAT is the real OS I/O lane"

Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo test -p verifier --lib s3_slim_pe_manual_map -- --nocapture --test-threads=1
    $cargoExit = $LASTEXITCODE
    if ($cargoExit -ne 0) {
        $ErrorActionPreference = $prevEap
        Write-Host "S2_RUNTIME_EFFECT_SLIM status=RED reason=host_slim_e2e_failed"
        exit 1
    }
    & cargo test -p verifier --lib e2e_runs -- --nocapture --test-threads=1
    $cargoExit = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($cargoExit -ne 0) {
        Write-Host "S2_RUNTIME_EFFECT_SLIM status=RED reason=in_dll_e2e_failed"
        exit 1
    }
} finally {
    Pop-Location
}

Write-Host "S2_RUNTIME_EFFECT_SLIM status=GREEN note=host+in_dll_slim_iat_e2e stubplatform=stub ow_rt=CUT"
exit 0
