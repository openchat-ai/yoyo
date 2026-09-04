# stage17-ow-rt-yoyo-runtime.ps1 — OW-RT YOYO-built runtime spike gate (post-v1.0 path 2)
#
# Gate E: YOYO-origin stub fills pe_dll_link export body (fixed exit-2).
# Still NOT OW-RT CLOSED — Rust sidecar yoyo_rt.dll remains production path.
#
# Script name stage17-* = post-v1.0 gate id (NOT ROADMAP Stage 17).
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Post-v1.0: OW-RT YOYO-origin export stub (pe_dll_link Gate E) ==="

$tyStub = Join-Path $Root "yoyo\tests\golden\ow_rt_yoyo_origin_exit2.ty"
if (-not (Test-Path $tyStub)) { throw "missing YOYO-origin stub $tyStub" }

Push-Location (Join-Path $Root "yoyo-rust")
try {
    & cargo test -p verifier pe_dll_link
    if ($LASTEXITCODE -ne 0) { throw "pe_dll_link tests failed" }
} finally {
    Pop-Location
}
Write-Host "OW_RT_SPIKE pe_dll_link_tests=GREEN"
Write-Host "OW_RT_SPIKE yoyo_origin_export=PRESENT stub=$tyStub"

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

# Honest: production sidecar is still Rust-built; YOYO-origin is export probe only.
Write-Host "OW_RT_SPIKE yoyo_built=ABSENT rust_sidecar=PRESENT disposition=CUT"
Write-Host "OW_RT_SPIKE note=Gate_E_YOYO_origin_export_only; CLOSED requires YOYO-built sidecar + no Rust yoyo_rt.dll host trust"
Write-Host "OW_RT_SPIKE status=GREEN doc=SCOPE-CUT-v1.0-ow-rt-yoyo-runtime.md"
exit 0
