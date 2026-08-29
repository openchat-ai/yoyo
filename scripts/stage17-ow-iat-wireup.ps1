# stage17-ow-iat-wireup.ps1 — OW-IAT wire-up WIP gate (post spike PR #7)
#
# Phase 1: file-read prelude emit (h00_manual_map_wireup.rs) — NOT wired into H_00 yet.
# Phase 2: manual-map x64 body + drop PEB LoadLibraryA call.
# Phase 3: JS/asm three-peer lockstep before merge.
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

$wired = Select-String -Path $winH00 -Pattern 'gen_h00_read_sidecar_prelude|h00_manual_map_wireup' -Quiet
if ($wired) {
    Write-Host "OW_IAT_WIREUP H_00_wired=YES — re-run stage15 for disposition"
} else {
    Write-Host "OW_IAT_WIREUP H_00_wired=NO (PEB LoadLibraryA resolve still live; honest CUT)"
}

Write-Host "OW_IAT_WIREUP status=WIP next=manual_map_x64_emit+three_peer_sync"
exit 0
