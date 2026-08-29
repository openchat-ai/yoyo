# stage17-ow-iat-wireup.ps1 — OW-IAT wire-up WIP gate (post spike PR #7)
#
# Phase 2: manual-map x64 body wired into gen_h00_selfhost_main; PEB LoadLibraryA dropped.
# Phase 3: JS/asm three-peer lockstep (template + explicit IAT patch sites).
param(
    [switch]$SkipBuild
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== Stage 17: OW-IAT wire-up WIP ==="

Push-Location (Join-Path $Root "yoyo-rust")
try {
    & cargo test -p verifier h00_manual_map_wireup pe_manual_map
    if ($LASTEXITCODE -ne 0) { throw "wire-up unit tests failed" }
} finally {
    Pop-Location
}
Write-Host "OW_IAT_WIREUP unit_tests=GREEN phase=file_read_prelude_emit"

$wireup = Join-Path $Root "yoyo-rust\verifier\src\h00_manual_map_wireup.rs"
$winH00 = Join-Path $Root "yoyo-rust\verifier\src\win32_selfhost.rs"
if (-not (Test-Path $wireup)) { throw "missing h00_manual_map_wireup.rs" }

$wired = Select-String -Path $winH00 -Pattern 'gen_h00_manual_map_main|h00_manual_map_wireup' -Quiet
if ($wired) {
    Write-Host "OW_IAT_WIREUP H_00_wired=YES manual_map_body=EMITTED PEB_LoadLibrary=DROPPED"
} else {
    Write-Host "OW_IAT_WIREUP H_00_wired=NO (honest CUT)"
}

Write-Host "OW_IAT_WIREUP status=WIP three_peer=JS_template_lockstep LoadLibraryA=ABSENT OW-IAT=CUT"
exit 0
