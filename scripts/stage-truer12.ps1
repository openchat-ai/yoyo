# stage-truer12.ps1 — AUTO-TRUER12 E2E (data assembled; prefab=code+seeds)
param()
$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "=== AUTO-TRUER12 E2E ==="

& node (Join-Path $Root "yoyo-js\scripts\verify-place-stub-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER12 status=RED reason=place_stub_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-bins-pin.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER12 status=RED reason=bins_pin"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-no-sidecar-face.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER12 status=RED reason=sidecar_name_face"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-h00-bake-zero.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER12 status=RED reason=h00_clean"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-prefab-bins-sot.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER12 status=RED reason=prefab_sot"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-yoyo-compact-code.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER12 status=RED reason=yoyo_compact_code"; exit 1 }

& node (Join-Path $Root "yoyo-js\scripts\verify-payload-assemble.js")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER12 status=RED reason=payload_assemble"; exit 1 }

$dataBytes = (Get-Item (Join-Path $Root "yoyo-js\src\platform\yoyo-compiler-payload\data.bin")).Length
if ($dataBytes -ne 42608) {
    Write-Host ("TRUER12 status=RED reason=data_bytes got={0} want=42608" -f $dataBytes)
    exit 1
}

& (Join-Path $Root "scripts\stage-truer5-semantic.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER12 status=RED reason=semantic"; exit 1 }

& (Join-Path $Root "scripts\stage-c1-native.ps1")
if ($LASTEXITCODE -ne 0) { Write-Host "TRUER12 status=RED reason=c1_native"; exit 1 }

Write-Host "TRUER12 trust_root=place_stub"
Write-Host "TRUER12 data_sot=assembled"
Write-Host "TRUER12 ret_pe_sot=seed_expand"
Write-Host "TRUER12 setret_sot=seed_bytes"
Write-Host "TRUER12 compact_sot=yoyo_ff_tyb"
Write-Host "TRUER12 prefab_sot=code_plus_seeds"
Write-Host "TRUER12 prefab_bytes=28908"
Write-Host "TRUER12 code_parse1_prefab_bytes=11105"
Write-Host "TRUER12 data_bytes=42608"
Write-Host "TRUER12 doc=AUTO-TRUER12.md"
Write-Host "TRUER12 status=GREEN"
Write-Host "TRUER12 note=data_assembled;parse1_plant_still_prefab;not_Thompson"
exit 0
