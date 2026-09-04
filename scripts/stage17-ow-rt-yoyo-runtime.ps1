# stage17-ow-rt-yoyo-runtime.ps1 — OW-RT YOYO-built runtime spike gate (post-v1.0 path 2)
#
# Gate F: YOYO-built read→compile→write effect (exit 0/1/2/3 + PE write).
# Still NOT OW-RT CLOSED — Rust sidecar yoyo_rt.dll remains production path.
#
# Script name stage17-* = post-v1.0 gate id (NOT ROADMAP Stage 17).
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Post-v1.0: OW-RT YOYO-built read-compile-write (pe_dll_link Gate F) ==="

$tyStub = Join-Path $Root "yoyo\tests\golden\ow_rt_yoyo_origin_exit2.ty"
if (-not (Test-Path $tyStub)) { throw "missing YOYO-origin stub $tyStub" }
$tyFx = Join-Path $Root "yoyo\tests\golden\selfhost_min_nop.ty"
if (-not (Test-Path $tyFx)) { throw "missing Gate F success fixture $tyFx" }

Push-Location (Join-Path $Root "yoyo-rust")
try {
    # cargo writes warnings to stderr; don't treat as terminating under Stop
    $prevEap = $ErrorActionPreference
    $ErrorActionPreference = "Continue"
    & cargo test -p verifier pe_dll_link
    $cargoExit = $LASTEXITCODE
    $ErrorActionPreference = $prevEap
    if ($cargoExit -ne 0) { throw "pe_dll_link tests failed" }
} finally {
    Pop-Location
}
Write-Host "OW_RT_SPIKE pe_dll_link_tests=GREEN"
Write-Host "OW_RT_SPIKE yoyo_origin_export=PRESENT stub=$tyStub"
Write-Host "OW_RT_SPIKE yoyo_built_effect=PRESENT fixture=$tyFx exits=0/1/2/3"

$spikeDoc = Join-Path $Root "SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
if (-not (Test-Path $spikeDoc)) { throw "missing $spikeDoc" }

$RuntimePreferred = Join-Path $Root "yoyo-rust\target\release-runtime\yoyo_runtime.dll"
$RuntimeCompat = Join-Path $Root "yoyo-rust\target\release\yoyo_runtime.dll"
$RuntimeDll = $null
if (Test-Path $RuntimePreferred) {
    $RuntimeDll = $RuntimePreferred
} elseif (Test-Path $RuntimeCompat) {
    $RuntimeDll = $RuntimeCompat
}

if (-not $RuntimeDll) {
    if ($SkipBuild) { throw "missing yoyo_runtime.dll (and -SkipBuild)" }
    Push-Location (Join-Path $Root "yoyo-rust")
    try {
        Write-Host "== build yoyo-runtime (release-runtime) =="
        & cargo build --profile release-runtime -p yoyo-runtime
        if ($LASTEXITCODE -ne 0) { throw "yoyo-runtime build failed" }
    } finally {
        Pop-Location
    }
    if (Test-Path $RuntimePreferred) {
        $RuntimeDll = $RuntimePreferred
    } elseif (Test-Path $RuntimeCompat) {
        $RuntimeDll = $RuntimeCompat
    } else {
        throw "yoyo_runtime.dll still missing after build"
    }
}

$dllBytes = (Get-Item $RuntimeDll).Length
$dllSha = (Get-FileHash -Algorithm SHA256 -Path $RuntimeDll).Hash.ToLowerInvariant().Substring(0, 16)
Write-Host "OW_RT_SPIKE rust_sidecar path=$RuntimeDll bytes=$dllBytes sha256_prefix=$dllSha"

# Honest: effect proven on YOYO seed/link path; production sidecar still Rust.
Write-Host "OW_RT_SPIKE yoyo_built=EFFECT rust_sidecar=PRESENT disposition=CUT"
Write-Host "OW_RT_SPIKE note=Gate_F_YOYO_built_effect_only; CLOSED requires production YOYO-built sidecar + no Rust yoyo_rt.dll host trust"
Write-Host "OW_RT_SPIKE status=GREEN doc=SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
exit 0
