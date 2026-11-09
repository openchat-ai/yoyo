# stage-truer8.ps1 — AUTO-TRUER8 E2E (H00 bake payload zeroed)
#
#   cd F:\yoyo
#   & .\scripts\stage-truer8.ps1
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-TRUER8 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER8 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER8 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER8 status=RED reason=sidecar_name_face"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-h00-bake-zero.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER8 status=RED reason=h00_bake"; exit 1 }

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER8 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER8 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER8 trust_root=place_stub"
Write-Host "TRUER8 h00_bake_payload=ZEROED"
Write-Host "TRUER8 h00_manual_map_stub=ABSENT"
Write-Host "TRUER8 sidecar_name_face=ABSENT"
Write-Host "TRUER8 data_bytes=414016"
Write-Host "TRUER8 doc=AUTO-TRUER8.md"
Write-Host "TRUER8 status=GREEN"
Write-Host "TRUER8 note=h00_bake_zeroed;slot_may_remain;not_Thompson"
exit 0
