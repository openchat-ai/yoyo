# stage17-s2-runtime-effect.ps1 — S2.5 acceptance: runtime-effect stub firmware
# (post-v1.0 Stage 17+ local track; NOT ROADMAP Stage 17)
#
# Verifies golden TYB firmware (ALLOC/LOAD_FILE/WRITE_FILE stub form) compiles
# on host mini + full paths. Honest: stubs store size/str_idx only — **not**
# real OS I/O; OW-RT remains CUT. Do not claim CLOSED.
#
#   cd F:\yoyo
#   & .\scripts\stage17-s2-runtime-effect.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Post-v1.0 S2.5: runtime-effect stub firmware (fail-closed) ==="

$ty = Join-Path $Root "yoyo\tests\golden\s2_runtime_effect_stub.ty"
$tyb = Join-Path $Root "yoyo\tests\golden\s2_runtime_effect_stub.tyb"
if (-not (Test-Path $ty)) { throw "missing $ty" }
if (-not (Test-Path $tyb)) { throw "missing $tyb" }

$magic = [IO.File]::ReadAllBytes($tyb)[0..3]
$magicStr = [Text.Encoding]::ASCII.GetString($magic)
if ($magicStr -ne "TYB$([char]0)" -and -not ($magic[0] -eq 0x54 -and $magic[1] -eq 0x59 -and $magic[2] -eq 0x42 -and $magic[3] -eq 0x00)) {
    throw "S2_RUNTIME_EFFECT status=RED reason=bad_tyb_magic"
}

Push-Location (Join-Path $Root "yoyo-rust")
try {
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo test -p verifier --lib s2_runtime_effect -- --test-threads=1
    $cargoExit = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($cargoExit -ne 0) {
        Write-Host "S2_RUNTIME_EFFECT status=RED reason=cargo_test_failed"
        exit 1
    }
} finally {
    Pop-Location
}

Write-Host "S2_RUNTIME_EFFECT firmware_ty=$ty"
Write-Host "S2_RUNTIME_EFFECT firmware_tyb=$tyb"
Write-Host "S2_RUNTIME_EFFECT status=GREEN note=stub_only_not_real_io ow_rt=CUT"
exit 0
