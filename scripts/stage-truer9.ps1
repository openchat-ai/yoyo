# stage-truer9.ps1 — AUTO-TRUER9 E2E (H00 excised clean)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-TRUER9 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER9 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER9 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER9 status=RED reason=sidecar_name_face"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-h00-bake-zero.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER9 status=RED reason=h00_clean"; exit 1 }

$dataBytes = (Get-Item (Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin")).Length
if ($dataBytes -ne 180864) {
    Write-Host ("TRUER9 status=RED reason=data_bytes got={0} want=180864" -f $dataBytes)
    exit 1
}

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER9 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER9 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER9 trust_root=place_stub"
Write-Host "TRUER9 h00_markers=ABSENT"
Write-Host "TRUER9 h00_bake=EXCISED"
Write-Host "TRUER9 h00_manual_map_stub=ABSENT"
Write-Host "TRUER9 data_bytes=180864"
Write-Host "TRUER9 full_c1_pe_bytes=246272"
Write-Host "TRUER9 doc=AUTO-TRUER9.md"
Write-Host "TRUER9 status=GREEN"
Write-Host "TRUER9 note=h00_excised_clean;not_Thompson"
exit 0
