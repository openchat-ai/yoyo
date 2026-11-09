# stage-truer10.ps1 — AUTO-TRUER10 E2E (FM excised · prefab=bins)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-TRUER10 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER10 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER10 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER10 status=RED reason=sidecar_name_face"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-h00-bake-zero.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER10 status=RED reason=h00_clean"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-prefab-bins-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER10 status=RED reason=prefab_sot"; exit 1 }

$dataBytes = (Get-Item (Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin")).Length
if ($dataBytes -ne 42608) {
    Write-Host ("TRUER10 status=RED reason=data_bytes got={0} want=42608" -f $dataBytes)
    exit 1
}

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER10 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER10 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER10 trust_root=place_stub"
Write-Host "TRUER10 prefab_sot=bins"
Write-Host "TRUER10 stub_role=derived_from_bins"
Write-Host "TRUER10 field_match_bake=EXCISED"
Write-Host "TRUER10 data_bytes=42608"
Write-Host "TRUER10 bins_total=58204"
Write-Host "TRUER10 doc=AUTO-TRUER10.md"
Write-Host "TRUER10 status=GREEN"
Write-Host "TRUER10 note=prefab_bins_only;not_empty_disk;not_Thompson"
exit 0
