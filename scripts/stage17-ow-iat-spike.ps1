# stage17-ow-iat-spike.ps1 — OW-IAT spike gate (post-v1.0 path 2)
#
# Proves in-process PE manual-map algorithm (pe_manual_map.rs) without claiming
# OW-IAT CLOSED. LoadLibraryA must remain on seed until H_00 stub wire-up + peer sync.
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Stage 17: OW-IAT spike (in-process PE mapper) ==="

Push-Location (Join-Path $Root "yoyo-rust")
try {
    & cargo test -p verifier pe_manual_map
    if ($LASTEXITCODE -ne 0) { throw "pe_manual_map tests failed" }
} finally {
    Pop-Location
}
Write-Host "OW_IAT_SPIKE pe_manual_map_tests=GREEN"

$peLink = Join-Path $Root "yoyo-rust\verifier\src\pe_link.rs"
$winH00 = Join-Path $Root "yoyo-rust\verifier\src\win32_selfhost.rs"
$spikeDoc = Join-Path $Root "SCOPE-CUT-v1.0-ow-iat-spike.md"
if (-not (Test-Path $spikeDoc)) { throw "missing $spikeDoc" }

$llPeLink = Select-String -Path $peLink -Pattern '"LoadLibraryA"' -Quiet
$llIat = Select-String -Path $winH00 -Pattern 'IAT_LOADLIBRARY' -Quiet
if ($llPeLink -or $llIat) {
    Write-Host "OW_IAT_SPIKE IAT_LoadLibraryA=PRESENT disposition=CUT (deeper shrink requires absent)"
} else {
    Write-Host "OW_IAT_SPIKE IAT_LoadLibraryA=ABSENT (deeper OW-IAT landed; PEB resolve still CUT)"
}

if (-not $SkipBuild) {
    $Yoyo = Join-Path $Root "yoyo-rust\target\release\yoyo.exe"
    if (-not (Test-Path $Yoyo)) {
        Push-Location (Join-Path $Root "yoyo-rust")
        try {
            & cargo build --release -p verifier
            if ($LASTEXITCODE -ne 0) { throw "verifier build failed" }
        } finally {
            Pop-Location
        }
    }
    $Ty = Join-Path $Root "yoyo\projects\yoyo.ty"
    $rustOut = Join-Path $Root "scripts\_stage17-ow-iat-spike\seed.exe"
    New-Item -ItemType Directory -Force -Path (Split-Path $rustOut) | Out-Null
    & $Yoyo link $Ty $rustOut
    if ($LASTEXITCODE -ne 0) { throw "seed link failed" }
    $bytes = [System.IO.File]::ReadAllBytes($rustOut)
    $ascii = [System.Text.Encoding]::ASCII.GetString($bytes)
    if ($ascii.Contains("LoadLibraryA")) {
        Write-Host "OW_IAT_SPIKE seed_pe ASCII_LoadLibraryA=PRESENT (IAT leak — fail deeper shrink)"
        throw "seed PE must not contain ASCII LoadLibraryA after deeper OW-IAT"
    } else {
        Write-Host "OW_IAT_SPIKE seed_pe ASCII_LoadLibraryA=ABSENT (expected — PEB resolve; manual-map not wired)"
    }
}

Write-Host "OW_IAT_SPIKE status=GREEN doc=SCOPE-CUT-v1.0-ow-iat-spike.md"
exit 0
